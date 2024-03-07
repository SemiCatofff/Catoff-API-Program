use anchor_lang::prelude::*;

#[account]
pub struct Challenge {
    pub creator: Pubkey,
    pub escrow_account: Pubkey,
}

#[account]
pub struct Participant {
    pub user: Pubkey,
    pub challenge_id: Pubkey,
}

#[account]
pub struct GlobalState {
    pub admin: Pubkey,
    pub sol_to_credit_exchange_rate: u64,
}
