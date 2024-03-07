use crate::state::GlobalState;
use crate::state::{Challenge, Participant};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

use crate::error::ErrorCode;

// pub fn payout_to_winners(ctx: Context<PayoutToWinners>, challenge_id: Pubkey) -> Result<()> {
//     Ok(())
// }

// pub fn withdraw_credits(ctx: Context<WithdrawCredits>, amount: u64) -> Result<()> {
//     Ok(())
// }

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + 8 + 32, seeds = [b"global_state".as_ref()], bump)]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, sol_to_credit_exchange_rate: u64) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    global_state.admin = ctx.accounts.admin.key();
    global_state.sol_to_credit_exchange_rate = sol_to_credit_exchange_rate;

    Ok(())
}

#[derive(Accounts)]
pub struct ExchangeSolForCredits<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_credit_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub credit_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn exchange_sol_for_credits(ctx: Context<ExchangeSolForCredits>, amount: u64) -> Result<()> {
    let sol_to_credit_exchange_rate = 100;
    let credits_to_mint = amount * sol_to_credit_exchange_rate;

    let cpi_accounts = MintTo {
        mint: ctx.accounts.credit_mint.to_account_info(),
        to: ctx.accounts.user_credit_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, credits_to_mint)?;

    Ok(())
}

#[derive(Accounts)]
pub struct ParticipateInChallenge<'info> {
    #[account(mut)]
    pub participant: Signer<'info>,
    #[account(mut)]
    pub participant_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub challenge: Account<'info, Challenge>,
    #[account(mut)]
    pub participant_record: Account<'info, Participant>,
    pub token_program: Program<'info, Token>,
}

pub fn participate_in_challenge(ctx: Context<ParticipateInChallenge>, amount: u64) -> Result<()> {
    require!(
        ctx.accounts.challenge.is_active,
        ErrorCode::ChallengeInactive
    );

    let cpi_accounts = Transfer {
        from: ctx.accounts.participant_account.to_account_info(),
        to: ctx.accounts.escrow_account.to_account_info(),
        authority: ctx.accounts.participant.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    ctx.accounts.participant_record.entry_fee_paid = amount;

    Ok(())
}

#[derive(Accounts)]
pub struct PayoutToWinners<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub escrow_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn payout_to_winners(ctx: Context<PayoutToWinners>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.escrow_account.to_account_info(),
        to: ctx.accounts.winner_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;
    Ok(())
}
