use anchor_client::{DynSigner, Program};
use anyhow::anyhow;
use clap::Parser;
use log::{debug, info};
use settlement_pipelines::anchor::add_instruction_to_builder;
use settlement_pipelines::arguments::{
    init_from_opts, InitializedGlobalOpts, PriorityFeePolicyOpts, TipPolicyOpts,
};
use settlement_pipelines::arguments::{load_keypair, GlobalOpts};
use settlement_pipelines::cli_result::{CliError, CliResult};
use settlement_pipelines::executor::execute_parallel;
use settlement_pipelines::init::{get_executor, init_log};
use settlement_pipelines::json_data::{
    load_json, load_json_with_on_chain, CombinedMerkleTreeSettlementCollections,
};
use settlement_pipelines::reporting::{with_reporting, PrintReportable, ReportHandler};
use settlement_pipelines::settlement_data::SettlementRecord;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_program;
use solana_transaction_builder::TransactionBuilder;
use solana_transaction_executor::{PriorityFeePolicy, TransactionExecutor};
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use validator_bonds::instructions::InitSettlementArgs;
use validator_bonds::state::settlement::find_settlement_claims_address;
use validator_bonds::ID as validator_bonds_id;
use validator_bonds_common::constants::find_event_authority;
use validator_bonds_common::settlements::get_settlements_for_pubkeys;
use validator_bonds_common::utils::get_account_infos_for_pubkeys;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    /// Merkle tree collection file path generated by protected-event-distribution as of --output-merkle-tree-collection
    #[arg(short = 'm', long)]
    input_merkle_tree_collection: PathBuf,

    /// Settlement collection file path generated by protected-event-distribution as of --output-settlement-collection
    #[arg(short = 's', long)]
    input_settlement_collection: PathBuf,

    /// forcing epoch, overriding from the settlement collection
    #[arg(long)]
    epoch: Option<u64>,

    #[clap(flatten)]
    priority_fee_policy_opts: PriorityFeePolicyOpts,

    #[clap(flatten)]
    tip_policy_opts: TipPolicyOpts,

    /// keypair payer for rent of accounts, if not provided, fee payer keypair is used
    #[arg(long)]
    rent_payer: Option<String>,
}

#[tokio::main]
async fn main() -> CliResult {
    let mut reporting = InitSettlementReport::report_handler();
    let result = real_main(&mut reporting).await;
    with_reporting::<InitSettlementReport>(&reporting, result).await
}

async fn real_main(reporting: &mut ReportHandler<InitSettlementReport>) -> anyhow::Result<()> {
    let args: Args = Args::parse();
    init_log(&args.global_opts);

    let InitializedGlobalOpts {
        fee_payer,
        operator_authority,
        priority_fee_policy,
        tip_policy,
        rpc_client,
        program,
    } = init_from_opts(
        &args.global_opts,
        &args.priority_fee_policy_opts,
        &args.tip_policy_opts,
    )?;

    let rent_payer = if let Some(rent_payer) = args.rent_payer.clone() {
        load_keypair("--rent-payer", &rent_payer)?
    } else {
        fee_payer.clone()
    };

    let config_address = args.global_opts.config;
    info!(
        "Loading merkle tree at: '{:?}', validator-bonds config: {}",
        args.input_merkle_tree_collection, config_address
    );

    let mut json_data = load_json(&[
        args.input_merkle_tree_collection,
        args.input_settlement_collection,
    ])?;

    // Load on-chain data for Settlement accounts that we need to create
    let mut settlement_records = load_on_chain_data(
        rpc_client.clone(),
        &mut json_data,
        &config_address,
        args.epoch,
    )
    .await?;

    let epoch = args
        .epoch
        .map_or_else(|| settlement_records.first().unwrap().epoch, |v| v);
    reporting.reportable.init(epoch, &settlement_records);

    let transaction_executor = get_executor(rpc_client.clone(), tip_policy);
    init_settlements(
        &program,
        rpc_client.clone(),
        transaction_executor.clone(),
        &mut settlement_records,
        &config_address,
        fee_payer.clone(),
        operator_authority.clone(),
        rent_payer.clone(),
        &priority_fee_policy,
        reporting,
    )
    .await?;

    upsize_settlements(
        &program,
        rpc_client.clone(),
        transaction_executor.clone(),
        &settlement_records,
        fee_payer.clone(),
        rent_payer.clone(),
        &priority_fee_policy,
        reporting,
    )
    .await?;

    Ok(())
}

/// Load on-chain data for Settlement accounts that we need to create
async fn load_on_chain_data(
    rpc_client: Arc<RpcClient>,
    json_data: &mut [CombinedMerkleTreeSettlementCollections],
    config_address: &Pubkey,
    epoch: Option<u64>,
) -> Result<Vec<SettlementRecord>, CliError> {
    let settlement_records_per_epoch =
        load_json_with_on_chain(rpc_client, json_data, config_address, epoch).await?;
    if settlement_records_per_epoch.keys().len() != 1 {
        return Err(CliError::Processing(anyhow!(
            "Loading Settlements from JSON data expected one epoch data but got {}",
            settlement_records_per_epoch.keys().len()
        )));
    }
    if let Some(settlement_records) = settlement_records_per_epoch.into_iter().next() {
        Ok(settlement_records.1)
    } else {
        Err(CliError::Processing(anyhow!(
            "Loading Settlements from JSON data failed to get Settlement records"
        )))
    }
}

#[allow(clippy::too_many_arguments)]
async fn init_settlements(
    program: &Program<Arc<DynSigner>>,
    rpc_client: Arc<RpcClient>,
    transaction_executor: Arc<TransactionExecutor>,
    settlement_records: &mut Vec<SettlementRecord>,
    config_address: &Pubkey,
    fee_payer: Arc<Keypair>,
    operator_authority: Arc<Keypair>,
    rent_payer: Arc<Keypair>,
    priority_fee_policy: &PriorityFeePolicy,
    reporting: &mut ReportHandler<InitSettlementReport>,
) -> anyhow::Result<()> {
    let mut transaction_builder = TransactionBuilder::limited(fee_payer.clone());
    transaction_builder.add_signer_checked(&operator_authority);
    transaction_builder.add_signer_checked(&rent_payer);

    for record in settlement_records {
        if record.bond_account.is_none() {
            // the existence of the Bond is required for any init Settlement, when not exists, we skip the init
            reporting.add_error_string(format!(
                "Cannot find bond account {} for vote account {}, funder {}, claim amount {} SOLs (settlement to init: {})",
                record.bond_address,
                record.vote_account_address,
                record.funder,
                lamports_to_sol(record.max_total_claim_sum),
                record.settlement_address,
            ));
            continue;
        }
        if record.max_total_claim == 0 || record.max_total_claim_sum == 0 {
            reporting.add_error_string(format!(
                "Cannot init Settlement with 0 at max_total_claim({}) or max_total_claim_sum({} SOLs), vote account {}, funder {} (settlement to init: {})",
                record.max_total_claim,
                lamports_to_sol(record.max_total_claim_sum),
                record.vote_account_address,
                record.funder,
                record.settlement_address,
            ));
            continue;
        }

        if record.settlement_account.is_some() {
            debug!(
                "Settlement account {} already exists, skipping initialization",
                record.settlement_address
            );
        } else {
            let req = program
                .request()
                .accounts(validator_bonds::accounts::InitSettlement {
                    config: *config_address,
                    bond: record.bond_address,
                    operator_authority: operator_authority.pubkey(),
                    system_program: system_program::ID,
                    rent_payer: rent_payer.pubkey(),
                    program: validator_bonds_id,
                    settlement: record.settlement_address,
                    settlement_claims: find_settlement_claims_address(&record.settlement_address).0,
                    event_authority: find_event_authority().0,
                })
                .args(validator_bonds::instruction::InitSettlement {
                    init_settlement_args: InitSettlementArgs {
                        merkle_root: record.merkle_root,
                        rent_collector: rent_payer.pubkey(),
                        max_total_claim: record.max_total_claim_sum,
                        max_merkle_nodes: record.max_total_claim,
                        epoch: record.epoch,
                    },
                });
            add_instruction_to_builder(
                &mut transaction_builder,
                &req,
                format!(
                    "InitSettlement: {} (vote account {})",
                    record.settlement_address, record.vote_account_address
                ),
            )?;
            reporting.reportable.add_created_settlement(record);
        }
    }

    let (tx_count, ix_count) = execute_parallel(
        rpc_client.clone(),
        transaction_executor.clone(),
        &mut transaction_builder,
        priority_fee_policy,
    )
    .await
    .map_err(CliError::retry_able)?;
    info!(
        "InitSettlement [{}]: txes {tx_count}/ixes {ix_count} executed successfully",
        reporting.reportable.list_created_settlements()
    );
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn upsize_settlements(
    program: &Program<Arc<DynSigner>>,
    rpc_client: Arc<RpcClient>,
    transaction_executor: Arc<TransactionExecutor>,
    settlement_records: &[SettlementRecord],
    fee_payer: Arc<Keypair>,
    rent_payer: Arc<Keypair>,
    priority_fee_policy: &PriorityFeePolicy,
    reporting: &mut ReportHandler<InitSettlementReport>,
) -> anyhow::Result<()> {
    let mut transaction_builder = TransactionBuilder::limited(fee_payer.clone());
    transaction_builder.add_signer_checked(&rent_payer);

    // re-loading settlements data from the chain on provided settlement addresses
    let reloaded_settlements_records = get_settlements_for_pubkeys(
        rpc_client.clone(),
        &settlement_records
            .iter()
            .map(|s| s.settlement_address)
            .collect::<Vec<Pubkey>>(),
    )
    .await
    .map_err(CliError::retry_able)?
    .into_iter()
    .filter(|(settlement_address, info)| {
        if info.is_none() {
            reporting.add_error_string(format!(
                "[UpsizeElements] Settlement account {} does not exist on-chain",
                settlement_address
            ));
            false
        } else {
            true
        }
    })
    .map(|(settlement_address, settlement)| (settlement_address, settlement.unwrap()))
    .collect::<Vec<(Pubkey, validator_bonds::state::settlement::Settlement)>>();
    let settlement_claims_pubkeys = reloaded_settlements_records
        .iter()
        .map(|(settlement_address, _)| find_settlement_claims_address(settlement_address).0)
        .collect::<Vec<Pubkey>>();
    let settlement_claims_infos =
        get_account_infos_for_pubkeys(rpc_client.clone(), &settlement_claims_pubkeys).await?;

    for (
        (settlement_claims_address, settlement_claims_info),
        (settlement_address, settlement_data),
    ) in settlement_claims_infos
        .into_iter()
        .zip(reloaded_settlements_records.iter())
    {
        let claims = if let Some(settlement_claims) = settlement_claims_info {
            settlement_claims
        } else {
            reporting.add_error_string(format!(
                "CRITICAL [upsize_settlements]: No SettlementClaims account {} for an existing Settlement {}",
                settlement_claims_address, settlement_address
            ));
            continue;
        };
        let required_size = validator_bonds::state::settlement_claims::account_size(
            settlement_data.max_merkle_nodes,
        );
        let number_of_upsize_calls = required_size
            .saturating_sub(claims.data.len())
            .div_ceil(validator_bonds::state::settlement_claims::MAX_PERMITTED_DATA_INCREASE);
        debug!(
            "[upsize_settlements] settlement {} (settlement claims: {}), max merkle nodes: {}, requires {} bytes, current size: {} bytes, upsize calls: {}",
            settlement_address,
            settlement_claims_address,
            settlement_data.max_merkle_nodes,
            required_size,
            claims.data.len(),
            number_of_upsize_calls
        );
        let vote_account = if let Some(settlement_record) = settlement_records
            .iter()
            .find(|s| s.settlement_address == *settlement_address)
        {
            settlement_record.vote_account_address
        } else {
            reporting.add_error_string(format!(
                "CRITICAL [upsize_settlements]: No vote account found for Settlement {}",
                settlement_address
            ));
            continue;
        };
        for _ in 0..number_of_upsize_calls {
            let req = program
                .request()
                .accounts(validator_bonds::accounts::UpsizeSettlementClaims {
                    settlement_claims: settlement_claims_address,
                    system_program: system_program::ID,
                    rent_payer: rent_payer.pubkey(),
                })
                .args(validator_bonds::instruction::UpsizeSettlementClaims {});
            add_instruction_to_builder(
                &mut transaction_builder,
                &req,
                format!(
                    "UpsizeSettlementClaims: {} (settlement: {}, vote account {})",
                    settlement_claims_address, settlement_address, vote_account,
                ),
            )?;
            reporting
                .reportable
                .add_upsized_settlement(*settlement_address);
        }
    }

    let (tx_count, ix_count) = execute_parallel(
        rpc_client.clone(),
        transaction_executor.clone(),
        &mut transaction_builder,
        priority_fee_policy,
    )
    .await
    .map_err(CliError::retry_able)?;
    info!(
        "Upsize Settlement Claims [{}]: txes {tx_count}/ixes {ix_count} executed successfully",
        reporting.reportable.list_created_settlements()
    );
    Ok(())
}

struct InitSettlementReport {
    json_settlements_count: u64,
    json_settlements_max_claim_sum: u64,
    json_max_merkle_nodes_sum: u64,
    // settlement_address, vote_account_address
    created_settlements: HashSet<SettlementRecord>,
    upsized_settlements: HashMap<Pubkey, u32>,
    epoch: u64,
}

impl PrintReportable for InitSettlementReport {
    fn get_report(&self) -> Pin<Box<dyn Future<Output = Vec<String>> + '_>> {
        Box::pin(async {
            vec![
                format!(
                    "InitSettlement (epoch: {}, sum merkle nodes: {}, sum claim amounts: {} SOLs): created {}/{} settlements",
                    self.epoch,
                    self.json_max_merkle_nodes_sum,
                    lamports_to_sol(self.json_settlements_max_claim_sum),
                    self.created_settlements.len(),
                    self.json_settlements_count
                ),
                format!(
                    "  UpsizeSettlementClaims: upsized settlements {}/{}",
                    self.upsized_settlements.len(),
                    self.json_settlements_count
                ),
            ]
        })
    }
}

impl InitSettlementReport {
    fn report_handler() -> ReportHandler<Self> {
        let init_settlement_report = Self {
            created_settlements: vec![],
            upsized_settlements: HashMap::new(),
            json_settlements_count: 0,
            json_settlements_max_claim_sum: 0,
            json_max_merkle_nodes_sum: 0,
            epoch: 0,
        };
        ReportHandler::new(init_settlement_report)
    }

    fn init(&mut self, epoch: u64, json_settlements: &[SettlementRecord]) {
        self.json_settlements_count = json_settlements.len() as u64;
        self.json_settlements_max_claim_sum =
            json_settlements.iter().map(|s| s.max_total_claim_sum).sum();
        self.json_max_merkle_nodes_sum = json_settlements.iter().map(|s| s.max_total_claim).sum();
        self.epoch = epoch;
    }

    fn add_created_settlement(&mut self, settlement_record: &SettlementRecord) {
        self.created_settlements.insert(settlement_record.clone());
    }

    fn add_upsized_settlement(&mut self, settlement_address: Pubkey) {
        *self
            .upsized_settlements
            .entry(settlement_address)
            .or_insert(0_u32) += 1_u32;
    }

    fn list_created_settlements(&self) -> String {
        self.created_settlements
            .iter()
            .map(|(s, v)| format!("{}/{}", s, v))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
