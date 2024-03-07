use crate::state::GlobalState;
use anchor_lang::prelude::*;

// pub fn exchange_sol_for_credits(ctx: Context<ExchangeSolForCredits>, amount: u64) -> Result<()> {
//     // Logic for SOL to credits exchange
//     Ok(())
// }

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
