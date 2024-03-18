// Manually pre-defined compute units cap. Used to be defined on CLI execution.
// Manual definition puts the cap a over-funded (~twice what was taken in tests).
// Its not automatically calculated but easy to be implemented.
// ---
// Purpose:
// Having the defined limit of compute units make possible to tip the priority fee with bigger amount and pay less.
// https://www.helius.dev/blog/priority-fees-understanding-solanas-transaction-fee-mechanics

export const INIT_CONFIG_LIMIT_UNITS = 26_000
export const CONFIGURE_CONFIG_LIMIT_UNITS = 10_000
export const EMERGENCY_LIMIT_UNITS = 8_000
export const INIT_BOND_LIMIT_UNITS = 45_000
export const CONFIGURE_BOND_LIMIT_UNITS = 20_000
export const CONFIGURE_BOND_MINT_LIMIT_UNITS = 60_000
export const MINT_BOND_LIMIT_UNITS = 200_000
export const FUND_BOND_LIMIT_UNITS = 140_000
export const INIT_WITHDRAW_REQUEST_LIMIT_UNITS = 40_000
export const CANCEL_WITHDRAW_REQUEST_LIMIT_UNITS = 24_000
export const MERGE_STAKE_LIMIT_UNITS = 120_000
// this is the limit for the claim withdraw request + merge as it is within the same CLI command
export const CLAIM_WITHDRAW_REQUEST_LIMIT_UNITS = 600_000
