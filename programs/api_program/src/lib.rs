mod instructions;
mod state;

use anchor_lang::prelude::*;

declare_id!("CbkPd8y8rgVaAgfGo6TmiPXd5JGViVbbZW6WBCs8aD9V");

#[program]
pub mod p2p_challenge_platform {
    use super::*;
    pub use instructions::*;
}
