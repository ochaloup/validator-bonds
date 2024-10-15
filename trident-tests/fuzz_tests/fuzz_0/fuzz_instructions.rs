pub mod validator_bonds_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        InitConfig(InitConfig),
        ConfigureConfig(ConfigureConfig),
        InitBond(InitBond),
        ConfigureBond(ConfigureBond),
        ConfigureBondWithMint(ConfigureBondWithMint),
        MintBond(MintBond),
        FundBond(FundBond),
        InitWithdrawRequest(InitWithdrawRequest),
        CancelWithdrawRequest(CancelWithdrawRequest),
        ClaimWithdrawRequest(ClaimWithdrawRequest),
        InitSettlement(InitSettlement),
        UpsizeSettlementClaims(UpsizeSettlementClaims),
        CancelSettlement(CancelSettlement),
        FundSettlement(FundSettlement),
        MergeStake(MergeStake),
        ResetStake(ResetStake),
        WithdrawStake(WithdrawStake),
        EmergencyPause(EmergencyPause),
        EmergencyResume(EmergencyResume),
        CloseSettlementV2(CloseSettlementV2),
        ClaimSettlementV2(ClaimSettlementV2),
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitConfig {
        pub accounts: InitConfigAccounts,
        pub data: InitConfigData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitConfigAccounts {
        pub config: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitConfigData {
        pub init_config_args: validator_bonds::instructions::config::init_config::InitConfigArgs,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigureConfig {
        pub accounts: ConfigureConfigAccounts,
        pub data: ConfigureConfigData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigureConfigAccounts {
        pub config: AccountId,
        pub admin_authority: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigureConfigData {
        pub configure_config_args:
            validator_bonds::instructions::config::configure_config::ConfigureConfigArgs,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitBond {
        pub accounts: InitBondAccounts,
        pub data: InitBondData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitBondAccounts {
        pub config: AccountId,
        pub vote_account: AccountId,
        pub validator_identity: AccountId,
        pub bond: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitBondData {
        pub init_bond_args: validator_bonds::instructions::bond::init_bond::InitBondArgs,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigureBond {
        pub accounts: ConfigureBondAccounts,
        pub data: ConfigureBondData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigureBondAccounts {
        pub config: AccountId,
        pub bond: AccountId,
        pub authority: AccountId,
        pub vote_account: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigureBondData {
        pub configure_bond_args:
            validator_bonds::instructions::bond::configure_bond::ConfigureBondArgs,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigureBondWithMint {
        pub accounts: ConfigureBondWithMintAccounts,
        pub data: ConfigureBondWithMintData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigureBondWithMintAccounts {
        pub config: AccountId,
        pub bond: AccountId,
        pub mint: AccountId,
        pub vote_account: AccountId,
        pub token_account: AccountId,
        pub token_authority: AccountId,
        pub token_program: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ConfigureBondWithMintData { pub args : validator_bonds :: instructions :: bond :: configure_bond_with_mint :: ConfigureBondWithMintArgs }
    #[derive(Arbitrary, Debug)]
    pub struct MintBond {
        pub accounts: MintBondAccounts,
        pub data: MintBondData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MintBondAccounts {
        pub config: AccountId,
        pub bond: AccountId,
        pub mint: AccountId,
        pub validator_identity: AccountId,
        pub validator_identity_token_account: AccountId,
        pub vote_account: AccountId,
        pub metadata: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
        pub token_program: AccountId,
        pub associated_token_program: AccountId,
        pub metadata_program: AccountId,
        pub rent: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MintBondData {}
    #[derive(Arbitrary, Debug)]
    pub struct FundBond {
        pub accounts: FundBondAccounts,
        pub data: FundBondData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct FundBondAccounts {
        pub config: AccountId,
        pub bond: AccountId,
        pub bonds_withdrawer_authority: AccountId,
        pub stake_account: AccountId,
        pub stake_authority: AccountId,
        pub clock: AccountId,
        pub stake_history: AccountId,
        pub stake_program: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct FundBondData {}
    #[derive(Arbitrary, Debug)]
    pub struct InitWithdrawRequest {
        pub accounts: InitWithdrawRequestAccounts,
        pub data: InitWithdrawRequestData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitWithdrawRequestAccounts {
        pub config: AccountId,
        pub bond: AccountId,
        pub vote_account: AccountId,
        pub authority: AccountId,
        pub withdraw_request: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitWithdrawRequestData {
        pub create_withdraw_request_args:
            validator_bonds::instructions::withdraw::init_withdraw_request::InitWithdrawRequestArgs,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CancelWithdrawRequest {
        pub accounts: CancelWithdrawRequestAccounts,
        pub data: CancelWithdrawRequestData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CancelWithdrawRequestAccounts {
        pub config: AccountId,
        pub bond: AccountId,
        pub vote_account: AccountId,
        pub authority: AccountId,
        pub withdraw_request: AccountId,
        pub rent_collector: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CancelWithdrawRequestData {}
    #[derive(Arbitrary, Debug)]
    pub struct ClaimWithdrawRequest {
        pub accounts: ClaimWithdrawRequestAccounts,
        pub data: ClaimWithdrawRequestData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ClaimWithdrawRequestAccounts {
        pub config: AccountId,
        pub bond: AccountId,
        pub vote_account: AccountId,
        pub authority: AccountId,
        pub withdraw_request: AccountId,
        pub bonds_withdrawer_authority: AccountId,
        pub stake_account: AccountId,
        pub withdrawer: AccountId,
        pub split_stake_account: AccountId,
        pub split_stake_rent_payer: AccountId,
        pub stake_program: AccountId,
        pub system_program: AccountId,
        pub stake_history: AccountId,
        pub clock: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ClaimWithdrawRequestData {}
    #[derive(Arbitrary, Debug)]
    pub struct InitSettlement {
        pub accounts: InitSettlementAccounts,
        pub data: InitSettlementData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitSettlementAccounts {
        pub config: AccountId,
        pub bond: AccountId,
        pub settlement: AccountId,
        pub settlement_claims: AccountId,
        pub operator_authority: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitSettlementData {
        pub init_settlement_args:
            validator_bonds::instructions::settlement::init_settlement::InitSettlementArgs,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpsizeSettlementClaims {
        pub accounts: UpsizeSettlementClaimsAccounts,
        pub data: UpsizeSettlementClaimsData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpsizeSettlementClaimsAccounts {
        pub settlement_claims: AccountId,
        pub rent_payer: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct UpsizeSettlementClaimsData {}
    #[derive(Arbitrary, Debug)]
    pub struct CancelSettlement {
        pub accounts: CancelSettlementAccounts,
        pub data: CancelSettlementData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CancelSettlementAccounts {
        pub config: AccountId,
        pub bond: AccountId,
        pub settlement: AccountId,
        pub settlement_claims: AccountId,
        pub authority: AccountId,
        pub bonds_withdrawer_authority: AccountId,
        pub rent_collector: AccountId,
        pub split_rent_collector: AccountId,
        pub split_rent_refund_account: AccountId,
        pub clock: AccountId,
        pub stake_program: AccountId,
        pub stake_history: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CancelSettlementData {}
    #[derive(Arbitrary, Debug)]
    pub struct FundSettlement {
        pub accounts: FundSettlementAccounts,
        pub data: FundSettlementData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct FundSettlementAccounts {
        pub config: AccountId,
        pub bond: AccountId,
        pub vote_account: AccountId,
        pub settlement: AccountId,
        pub operator_authority: AccountId,
        pub stake_account: AccountId,
        pub settlement_staker_authority: AccountId,
        pub bonds_withdrawer_authority: AccountId,
        pub split_stake_account: AccountId,
        pub split_stake_rent_payer: AccountId,
        pub system_program: AccountId,
        pub stake_history: AccountId,
        pub clock: AccountId,
        pub rent: AccountId,
        pub stake_program: AccountId,
        pub stake_config: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct FundSettlementData {}
    #[derive(Arbitrary, Debug)]
    pub struct MergeStake {
        pub accounts: MergeStakeAccounts,
        pub data: MergeStakeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MergeStakeAccounts {
        pub config: AccountId,
        pub source_stake: AccountId,
        pub destination_stake: AccountId,
        pub staker_authority: AccountId,
        pub stake_history: AccountId,
        pub clock: AccountId,
        pub stake_program: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct MergeStakeData {
        pub merge_args: validator_bonds::instructions::stake::merge_stake::MergeStakeArgs,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ResetStake {
        pub accounts: ResetStakeAccounts,
        pub data: ResetStakeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ResetStakeAccounts {
        pub config: AccountId,
        pub bond: AccountId,
        pub settlement: AccountId,
        pub stake_account: AccountId,
        pub bonds_withdrawer_authority: AccountId,
        pub vote_account: AccountId,
        pub stake_history: AccountId,
        pub stake_config: AccountId,
        pub clock: AccountId,
        pub stake_program: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ResetStakeData {}
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawStake {
        pub accounts: WithdrawStakeAccounts,
        pub data: WithdrawStakeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawStakeAccounts {
        pub config: AccountId,
        pub operator_authority: AccountId,
        pub settlement: AccountId,
        pub stake_account: AccountId,
        pub bonds_withdrawer_authority: AccountId,
        pub withdraw_to: AccountId,
        pub stake_history: AccountId,
        pub clock: AccountId,
        pub stake_program: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawStakeData {}
    #[derive(Arbitrary, Debug)]
    pub struct EmergencyPause {
        pub accounts: EmergencyPauseAccounts,
        pub data: EmergencyPauseData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct EmergencyPauseAccounts {
        pub config: AccountId,
        pub pause_authority: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct EmergencyPauseData {}
    #[derive(Arbitrary, Debug)]
    pub struct EmergencyResume {
        pub accounts: EmergencyResumeAccounts,
        pub data: EmergencyResumeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct EmergencyResumeAccounts {
        pub config: AccountId,
        pub pause_authority: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct EmergencyResumeData {}
    #[derive(Arbitrary, Debug)]
    pub struct CloseSettlementV2 {
        pub accounts: CloseSettlementV2Accounts,
        pub data: CloseSettlementV2Data,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CloseSettlementV2Accounts {
        pub config: AccountId,
        pub bond: AccountId,
        pub settlement: AccountId,
        pub settlement_claims: AccountId,
        pub bonds_withdrawer_authority: AccountId,
        pub rent_collector: AccountId,
        pub split_rent_collector: AccountId,
        pub split_rent_refund_account: AccountId,
        pub clock: AccountId,
        pub stake_program: AccountId,
        pub stake_history: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CloseSettlementV2Data {}
    #[derive(Arbitrary, Debug)]
    pub struct ClaimSettlementV2 {
        pub accounts: ClaimSettlementV2Accounts,
        pub data: ClaimSettlementV2Data,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ClaimSettlementV2Accounts {
        pub config: AccountId,
        pub bond: AccountId,
        pub settlement: AccountId,
        pub settlement_claims: AccountId,
        pub stake_account_from: AccountId,
        pub stake_account_to: AccountId,
        pub bonds_withdrawer_authority: AccountId,
        pub stake_history: AccountId,
        pub clock: AccountId,
        pub stake_program: AccountId,
        pub event_authority: AccountId,
        pub program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct ClaimSettlementV2Data {
        pub claim_settlement_args:
            validator_bonds::instructions::settlement::claim_settlement::ClaimSettlementV2Args,
    }
    impl<'info> IxOps<'info> for InitConfig {
        type IxData = validator_bonds::instruction::InitConfig;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitConfigSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::InitConfig {
                init_config_args: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::InitConfig {
                config: todo!(),
                rent_payer: todo!(),
                system_program: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ConfigureConfig {
        type IxData = validator_bonds::instruction::ConfigureConfig;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ConfigureConfigSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::ConfigureConfig {
                configure_config_args: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::ConfigureConfig {
                config: todo!(),
                admin_authority: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for InitBond {
        type IxData = validator_bonds::instruction::InitBond;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitBondSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::InitBond {
                init_bond_args: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::InitBond {
                config: todo!(),
                vote_account: todo!(),
                validator_identity: todo!(),
                bond: todo!(),
                rent_payer: todo!(),
                system_program: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ConfigureBond {
        type IxData = validator_bonds::instruction::ConfigureBond;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ConfigureBondSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::ConfigureBond {
                configure_bond_args: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::ConfigureBond {
                config: todo!(),
                bond: todo!(),
                authority: todo!(),
                vote_account: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ConfigureBondWithMint {
        type IxData = validator_bonds::instruction::ConfigureBondWithMint;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ConfigureBondWithMintSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::ConfigureBondWithMint { args: todo!() };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::ConfigureBondWithMint {
                config: todo!(),
                bond: todo!(),
                mint: todo!(),
                vote_account: todo!(),
                token_account: todo!(),
                token_authority: todo!(),
                token_program: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for MintBond {
        type IxData = validator_bonds::instruction::MintBond;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = MintBondSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::MintBond {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::MintBond {
                config: todo!(),
                bond: todo!(),
                mint: todo!(),
                validator_identity: todo!(),
                validator_identity_token_account: todo!(),
                vote_account: todo!(),
                metadata: todo!(),
                rent_payer: todo!(),
                system_program: todo!(),
                token_program: todo!(),
                associated_token_program: todo!(),
                metadata_program: todo!(),
                rent: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for FundBond {
        type IxData = validator_bonds::instruction::FundBond;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = FundBondSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::FundBond {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::FundBond {
                config: todo!(),
                bond: todo!(),
                bonds_withdrawer_authority: todo!(),
                stake_account: todo!(),
                stake_authority: todo!(),
                clock: todo!(),
                stake_history: todo!(),
                stake_program: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for InitWithdrawRequest {
        type IxData = validator_bonds::instruction::InitWithdrawRequest;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitWithdrawRequestSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::InitWithdrawRequest {
                create_withdraw_request_args: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::InitWithdrawRequest {
                config: todo!(),
                bond: todo!(),
                vote_account: todo!(),
                authority: todo!(),
                withdraw_request: todo!(),
                rent_payer: todo!(),
                system_program: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for CancelWithdrawRequest {
        type IxData = validator_bonds::instruction::CancelWithdrawRequest;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CancelWithdrawRequestSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::CancelWithdrawRequest {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::CancelWithdrawRequest {
                config: todo!(),
                bond: todo!(),
                vote_account: todo!(),
                authority: todo!(),
                withdraw_request: todo!(),
                rent_collector: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ClaimWithdrawRequest {
        type IxData = validator_bonds::instruction::ClaimWithdrawRequest;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ClaimWithdrawRequestSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::ClaimWithdrawRequest {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::ClaimWithdrawRequest {
                config: todo!(),
                bond: todo!(),
                vote_account: todo!(),
                authority: todo!(),
                withdraw_request: todo!(),
                bonds_withdrawer_authority: todo!(),
                stake_account: todo!(),
                withdrawer: todo!(),
                split_stake_account: todo!(),
                split_stake_rent_payer: todo!(),
                stake_program: todo!(),
                system_program: todo!(),
                stake_history: todo!(),
                clock: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for InitSettlement {
        type IxData = validator_bonds::instruction::InitSettlement;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitSettlementSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::InitSettlement {
                init_settlement_args: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::InitSettlement {
                config: todo!(),
                bond: todo!(),
                settlement: todo!(),
                settlement_claims: todo!(),
                operator_authority: todo!(),
                rent_payer: todo!(),
                system_program: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for UpsizeSettlementClaims {
        type IxData = validator_bonds::instruction::UpsizeSettlementClaims;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = UpsizeSettlementClaimsSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::UpsizeSettlementClaims {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::UpsizeSettlementClaims {
                settlement_claims: todo!(),
                rent_payer: todo!(),
                system_program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for CancelSettlement {
        type IxData = validator_bonds::instruction::CancelSettlement;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CancelSettlementSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::CancelSettlement {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::CancelSettlement {
                config: todo!(),
                bond: todo!(),
                settlement: todo!(),
                settlement_claims: todo!(),
                authority: todo!(),
                bonds_withdrawer_authority: todo!(),
                rent_collector: todo!(),
                split_rent_collector: todo!(),
                split_rent_refund_account: todo!(),
                clock: todo!(),
                stake_program: todo!(),
                stake_history: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for FundSettlement {
        type IxData = validator_bonds::instruction::FundSettlement;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = FundSettlementSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::FundSettlement {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::FundSettlement {
                config: todo!(),
                bond: todo!(),
                vote_account: todo!(),
                settlement: todo!(),
                operator_authority: todo!(),
                stake_account: todo!(),
                settlement_staker_authority: todo!(),
                bonds_withdrawer_authority: todo!(),
                split_stake_account: todo!(),
                split_stake_rent_payer: todo!(),
                system_program: todo!(),
                stake_history: todo!(),
                clock: todo!(),
                rent: todo!(),
                stake_program: todo!(),
                stake_config: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for MergeStake {
        type IxData = validator_bonds::instruction::MergeStake;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = MergeStakeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::MergeStake {
                merge_args: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::MergeStake {
                config: todo!(),
                source_stake: todo!(),
                destination_stake: todo!(),
                staker_authority: todo!(),
                stake_history: todo!(),
                clock: todo!(),
                stake_program: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ResetStake {
        type IxData = validator_bonds::instruction::ResetStake;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ResetStakeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::ResetStake {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::ResetStake {
                config: todo!(),
                bond: todo!(),
                settlement: todo!(),
                stake_account: todo!(),
                bonds_withdrawer_authority: todo!(),
                vote_account: todo!(),
                stake_history: todo!(),
                stake_config: todo!(),
                clock: todo!(),
                stake_program: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for WithdrawStake {
        type IxData = validator_bonds::instruction::WithdrawStake;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = WithdrawStakeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::WithdrawStake {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::WithdrawStake {
                config: todo!(),
                operator_authority: todo!(),
                settlement: todo!(),
                stake_account: todo!(),
                bonds_withdrawer_authority: todo!(),
                withdraw_to: todo!(),
                stake_history: todo!(),
                clock: todo!(),
                stake_program: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for EmergencyPause {
        type IxData = validator_bonds::instruction::EmergencyPause;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = EmergencyPauseSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::EmergencyPause {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::EmergencyPauseResume {
                config: todo!(),
                pause_authority: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for EmergencyResume {
        type IxData = validator_bonds::instruction::EmergencyResume;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = EmergencyResumeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::EmergencyResume {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::EmergencyPauseResume {
                config: todo!(),
                pause_authority: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for CloseSettlementV2 {
        type IxData = validator_bonds::instruction::CloseSettlementV2;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CloseSettlementV2Snapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::CloseSettlementV2 {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::CloseSettlementV2 {
                config: todo!(),
                bond: todo!(),
                settlement: todo!(),
                settlement_claims: todo!(),
                bonds_withdrawer_authority: todo!(),
                rent_collector: todo!(),
                split_rent_collector: todo!(),
                split_rent_refund_account: todo!(),
                clock: todo!(),
                stake_program: todo!(),
                stake_history: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for ClaimSettlementV2 {
        type IxData = validator_bonds::instruction::ClaimSettlementV2;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = ClaimSettlementV2Snapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = validator_bonds::instruction::ClaimSettlementV2 {
                claim_settlement_args: todo!(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = validator_bonds::accounts::ClaimSettlementV2 {
                config: todo!(),
                bond: todo!(),
                settlement: todo!(),
                settlement_claims: todo!(),
                stake_account_from: todo!(),
                stake_account_to: todo!(),
                bonds_withdrawer_authority: todo!(),
                stake_history: todo!(),
                clock: todo!(),
                stake_program: todo!(),
                event_authority: todo!(),
                program: todo!(),
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        admin_authority: AccountsStorage<todo!()>,
        associated_token_program: AccountsStorage<todo!()>,
        authority: AccountsStorage<todo!()>,
        bond: AccountsStorage<todo!()>,
        bonds_withdrawer_authority: AccountsStorage<todo!()>,
        clock: AccountsStorage<todo!()>,
        config: AccountsStorage<todo!()>,
        destination_stake: AccountsStorage<todo!()>,
        event_authority: AccountsStorage<todo!()>,
        metadata: AccountsStorage<todo!()>,
        metadata_program: AccountsStorage<todo!()>,
        mint: AccountsStorage<todo!()>,
        operator_authority: AccountsStorage<todo!()>,
        pause_authority: AccountsStorage<todo!()>,
        program: AccountsStorage<todo!()>,
        rent: AccountsStorage<todo!()>,
        rent_collector: AccountsStorage<todo!()>,
        rent_payer: AccountsStorage<todo!()>,
        settlement: AccountsStorage<todo!()>,
        settlement_claims: AccountsStorage<todo!()>,
        settlement_staker_authority: AccountsStorage<todo!()>,
        source_stake: AccountsStorage<todo!()>,
        split_rent_collector: AccountsStorage<todo!()>,
        split_rent_refund_account: AccountsStorage<todo!()>,
        split_stake_account: AccountsStorage<todo!()>,
        split_stake_rent_payer: AccountsStorage<todo!()>,
        stake_account: AccountsStorage<todo!()>,
        stake_account_from: AccountsStorage<todo!()>,
        stake_account_to: AccountsStorage<todo!()>,
        stake_authority: AccountsStorage<todo!()>,
        stake_config: AccountsStorage<todo!()>,
        stake_history: AccountsStorage<todo!()>,
        stake_program: AccountsStorage<todo!()>,
        staker_authority: AccountsStorage<todo!()>,
        system_program: AccountsStorage<todo!()>,
        token_account: AccountsStorage<todo!()>,
        token_authority: AccountsStorage<todo!()>,
        token_program: AccountsStorage<todo!()>,
        validator_identity: AccountsStorage<todo!()>,
        validator_identity_token_account: AccountsStorage<todo!()>,
        vote_account: AccountsStorage<todo!()>,
        withdraw_request: AccountsStorage<todo!()>,
        withdraw_to: AccountsStorage<todo!()>,
        withdrawer: AccountsStorage<todo!()>,
    }
}
