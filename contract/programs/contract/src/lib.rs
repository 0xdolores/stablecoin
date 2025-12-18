use anchor_lang::prelude::*;

use state::*;
mod state;
use constants::*;
mod constants;
use instructions::*;
mod instructions;

declare_id!("GSP7LL75gEw5EetxyAEjMiy4A9yij5oHEVjuGbjg18yu");

#[program]
pub mod contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
