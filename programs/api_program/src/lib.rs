mod error;
mod instructions;
mod state;

use anchor_lang::prelude::*;

declare_id!("CbkPd8y8rgVaAgfGo6TmiPXd5JGViVbbZW6WBCs8aD9V");

#[program]
pub mod api_program {
    use super::*;
    pub use instructions::*;
}
