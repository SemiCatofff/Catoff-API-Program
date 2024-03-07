use anchor_lang::prelude::*;

#[account]
pub struct Challenge {
    pub escrow_account: Pubkey,
    pub is_active: bool,
}

#[account]
pub struct Participant {
    pub user: Pubkey,
    pub challenge_id: Pubkey,
    pub entry_fee_paid: u64,
}

#[account]
pub struct GlobalState {
    pub admin: Pubkey,
    pub sol_to_credit_exchange_rate: u64,
}
