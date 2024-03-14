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

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>, // For USDT withdrawals.
    #[account(signer)]
    pub authority: Signer<'info>,
    pub to_account: AccountInfo<'info>, // For SOL withdrawals, this needs to be a SystemAccount.
    pub token_program: Program<'info, Token>,
}

pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
    let escrow_account = &mut ctx.accounts.escrow_account;
    escrow_account.sol_balance += amount; // Update your internal tracking of SOL balance.
    Ok(())
}
