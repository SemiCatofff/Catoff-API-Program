use crate::state::EscrowAccount;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
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

pub fn deposit_usdt(ctx: Context<DepositUsdt>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.depositor_token_account.to_account_info(),
        to: ctx.accounts.escrow_token_account.to_account_info(),
        authority: ctx.accounts.depositor.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;
    let escrow_account = &mut ctx.accounts.escrow_account;
    escrow_account.usdt_balance += amount;

    Ok(())
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64, currency: String) -> Result<()> {
    require!(
        ctx.accounts.escrow_account.authority == *ctx.accounts.authority.key,
        ErrorCode::Unauthorized
    );

    match currency.as_str() {
        "SOL" => {
            // Ensure the escrow account has enough SOL to cover the withdrawal.
            if **ctx
                .accounts
                .escrow_account
                .to_account_info()
                .lamports
                .borrow()
                < amount
            {
                return Err(ErrorCode::InsufficientFunds.into());
            }

            // Transfer SOL to the recipient
            **ctx
                .accounts
                .escrow_account
                .to_account_info()
                .try_borrow_mut_lamports()? -= amount;
            **ctx.accounts.to_account.try_borrow_mut_lamports()? += amount;
        }
        "USDT" => {
            // Transfer USDT using the SPL Token program
            let cpi_accounts = Transfer {
                from: ctx.accounts.escrow_token_account.to_account_info(),
                to: ctx.accounts.to_account.to_account_info(),
                authority: ctx.accounts.escrow_account.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, amount)?;
        }
        _ => return Err(ErrorCode::UnsupportedCurrency.into()),
    }
    Ok(())
}
