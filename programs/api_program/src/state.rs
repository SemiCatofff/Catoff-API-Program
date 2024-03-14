use anchor_lang::prelude::*;

#[account]
pub struct EscrowAccount {
    pub sol_balance: u64, // Tracks the SOL balance if you need to track this separately.
    pub usdt_balance: u64, // Placeholder for USDT balance, actual tracking will be through the token account.
    pub authority: Pubkey, // Authority who can initiate withdrawals.
}
