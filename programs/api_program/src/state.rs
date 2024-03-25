
use anchor_lang::prelude::*;

#[account]
pub struct EscrowAccount {
    pub sol_balance: u64,
    pub usdc_balance: u64,
    pub authority: Pubkey,
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