use crate::state::GlobalState;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

// pub fn participate_in_challenge(
//     ctx: Context<ParticipateInChallenge>,
//     challenge_id: Pubkey,
// ) -> Result<()> {
//     // Logic for challenge participation
//     Ok(())
// }

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
