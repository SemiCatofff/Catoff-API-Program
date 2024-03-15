use anchor_lang::prelude::*;

#[account]
pub struct EscrowAccount {
    pub sol_balance: u64, // Tracks the SOL balance if you need to track this separately.
    pub usdt_balance: u64, // Placeholder for USDT balance, actual tracking will be through the token account.
    pub authority: Pubkey, // Authority who can initiate withdrawals.
}
#[event]
pub struct DepositEvent {
    pub from: Pubkey,
    pub amount: u64,
    pub currency: String,
}

#[event]
pub struct WithdrawEvent {
    pub to: Pubkey,
    pub amount: u64,
    pub currency: String,
}
