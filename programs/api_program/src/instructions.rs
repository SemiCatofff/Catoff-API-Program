use crate::error::ErrorCode;
use crate::state::{DepositEvent, EscrowAccount, WithdrawEvent};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(init, payer = owner, space = 8 + 8 + 8 + 32)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositUsdc<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(mut)]
    pub depositor_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, signer)]
    pub escrow_account: Account<'info, EscrowAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>, // For USDC withdrawals.
    #[account(signer)]
    pub authority: Signer<'info>,
    /// CHECK: The `to_account` is a generic account that can be either a SOL account or an SPL Token account.
    /// Safety is ensured by runtime checks depending on the withdrawal currency type.
    #[account(mut)]
    pub to_account: AccountInfo<'info>, // For SOL withdrawals, this needs to be a SystemAccount.
    pub token_program: Program<'info, Token>,
}

pub fn initialize_escrow(ctx: Context<InitializeEscrow>) -> Result<()> {
    let escrow_account = &mut ctx.accounts.escrow_account;
    escrow_account.sol_balance = 0;
    escrow_account.usdc_balance = 0;
    escrow_account.authority = ctx.accounts.owner.key();

    Ok(())
}

const MIN_LAMPORT_DEPOSIT: u64 = 1_000_000;

pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
    require!(amount >= MIN_LAMPORT_DEPOSIT, ErrorCode::InvalidSolAmount);

    let escrow_account = &mut ctx.accounts.escrow_account;
    escrow_account.sol_balance += amount;

    let sol_transfer = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.depositor.key(),
        &ctx.accounts.escrow_account.key(),
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &sol_transfer,
        &[
            ctx.accounts.depositor.to_account_info().clone(),
            ctx.accounts.escrow_account.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;

    emit!(DepositEvent {
        from: ctx.accounts.depositor.key(),
        amount,
        currency: "SOL".to_string(),
    });

    Ok(())
}

pub fn deposit_usdc(ctx: Context<DepositUsdc>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.depositor_token_account.to_account_info(),
        to: ctx.accounts.escrow_token_account.to_account_info(),
        authority: ctx.accounts.depositor.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;
    let escrow_account = &mut ctx.accounts.escrow_account;
    escrow_account.usdc_balance += amount;

    emit!(DepositEvent {
        from: ctx.accounts.depositor.key(),
        amount,
        currency: "USDC".to_string(),
    });

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

            let escrow_account = &mut ctx.accounts.escrow_account;
            escrow_account.sol_balance -= amount;

            // Emit the withdraw event for SOL
            emit!(WithdrawEvent {
                to: ctx.accounts.to_account.key(),
                amount,
                currency: "SOL".to_string(),
            });
        }
        "USDC" => {
            // Transfer USDC using the SPL Token program
            let cpi_accounts = Transfer {
                from: ctx.accounts.escrow_token_account.to_account_info(),
                to: ctx.accounts.to_account.to_account_info(),
                authority: ctx.accounts.escrow_account.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, amount)?;

            let escrow_account = &mut ctx.accounts.escrow_account;
            escrow_account.usdc_balance -= amount;

            // Emit the withdraw event for USDC
            emit!(WithdrawEvent {
                to: ctx.accounts.to_account.key(),
                amount,
                currency: "USDC".to_string(),
            });
        }
        _ => return Err(ErrorCode::UnsupportedCurrency.into()),
    }
    Ok(())
}
