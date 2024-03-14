use crate::state::EscrowAccount;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use solana_program::system_instruction;

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub depositor: Signer<'info>,
}

#[derive(Accounts)]
pub struct DepositUsdt<'info> {
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub depositor: Signer<'info>,
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>, // To track USDT balance if needed.
    pub token_program: Program<'info, Token>,
}
