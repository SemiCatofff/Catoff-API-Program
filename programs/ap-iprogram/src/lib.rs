use anchor_lang::prelude::*;

declare_id!("CbkPd8y8rgVaAgfGo6TmiPXd5JGViVbbZW6WBCs8aD9V");

#[program]
pub mod ap_iprogram {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
