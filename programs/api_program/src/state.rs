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

#[account]
pub struct WithdrawalRequest {
    pub requestor: Pubkey, // User requesting the withdrawal
    pub amount: u64,       // Amount of SOL to withdraw, calculated based on credits burned
    pub processed: bool,   // Whether the request has been processed by the off-chain service
}
