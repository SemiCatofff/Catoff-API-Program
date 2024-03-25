mod error;
mod instructions;
mod state;

use anchor_lang::prelude::*;

declare_id!("79VSBMCvvTKNwZSocidYCTxDEDqy39ge9fdzWiBcP5WV");

#[program]
pub mod api_program {
    use super::*;
    pub use instructions::*;
}
