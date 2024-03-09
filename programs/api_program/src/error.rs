use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The challenge is currently inactive.")]
    ChallengeInactive,
}
