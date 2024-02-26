use crate::error::ErrorCode;
use crate::events::settlement::InitSettlementEvent;
use crate::state::bond::Bond;
use crate::state::config::Config;
use crate::state::settlement::{find_settlement_staker_authority, Bumps, Settlement};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program::ID as system_program_id;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitSettlementArgs {
    /// merkle root for this settlement, multiple settlements can be created with the same merkle root,
    /// settlements will be distinguished by the vote_account
    pub merkle_root: [u8; 32],
    /// maximal number of lamports that can be claimed from this settlement
    pub max_total_claim: u64,
    /// maximal number of merkle tree nodes that can be claimed from this settlement
    pub max_merkle_nodes: u64,
    /// collects the rent exempt from the settlement account when closed
    pub rent_collector: Pubkey,
    /// epoch that the settlement is created for
    pub epoch: u64,
}

/// Creates settlement account for the bond, only operator authority can create the account
#[derive(Accounts)]
#[instruction(params: InitSettlementArgs)]
pub struct InitSettlement<'info> {
    #[account(
        has_one = operator_authority @ ErrorCode::InvalidOperatorAuthority,
    )]
    config: Account<'info, Config>,

    #[account(
        has_one = config @ ErrorCode::ConfigAccountMismatch,
        seeds = [
            b"bond_account",
            config.key().as_ref(),
            bond.vote_account.as_ref()
        ],
        bump = bond.bump,
    )]
    bond: Account<'info, Bond>,

    #[account(
        init,
        payer = rent_payer,
        space = 8 + std::mem::size_of::<Settlement>(),
        seeds = [
            b"settlement_account",
            bond.key().as_ref(),
            params.merkle_root.as_ref(),
            params.epoch.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    settlement: Account<'info, Settlement>,

    /// operator signer authority that is allowed to create the settlement account
    operator_authority: Signer<'info>,

    /// rent exempt payer of account creation
    #[account(
        mut,
        owner = system_program_id,
    )]
    rent_payer: Signer<'info>,

    system_program: Program<'info, System>,
}

impl<'info> InitSettlement<'info> {
    pub fn process(
        &mut self,
        InitSettlementArgs {
            merkle_root,
            rent_collector,
            max_total_claim,
            max_merkle_nodes,
            epoch,
        }: InitSettlementArgs,
        settlement_bump: u8,
    ) -> Result<()> {
        require!(!self.config.paused, ErrorCode::ProgramIsPaused);

        if max_total_claim == 0 || max_merkle_nodes == 0 {
            return Err(error!(ErrorCode::EmptySettlementMerkleTree).with_values((
                "max_total_claim, max_merkle_nodes",
                format!("{}, {}", max_total_claim, max_merkle_nodes),
            )));
        }

        let (authority, authority_bump) = find_settlement_staker_authority(&self.settlement.key());
        self.settlement.set_inner(Settlement {
            bond: self.bond.key(),
            staker_authority: authority,
            merkle_root,
            max_total_claim,
            max_merkle_nodes,
            lamports_funded: 0,
            lamports_claimed: 0,
            merkle_nodes_claimed: 0,
            epoch_created_for: epoch,
            rent_collector,
            split_rent_collector: None,
            split_rent_amount: 0,
            bumps: Bumps {
                pda: settlement_bump,
                staker_authority: authority_bump,
            },
            reserved: [0; 99],
        });
        emit!(InitSettlementEvent {
            bond: self.settlement.bond,
            vote_account: self.bond.vote_account,
            staker_authority: self.settlement.staker_authority,
            merkle_root: self.settlement.merkle_root,
            max_total_claim: self.settlement.max_total_claim,
            max_merkle_nodes: self.settlement.max_merkle_nodes,
            epoch_created_for: self.settlement.epoch_created_for,
            rent_collector: self.settlement.rent_collector,
            bumps: self.settlement.bumps.clone(),
        });

        Ok(())
    }
}
