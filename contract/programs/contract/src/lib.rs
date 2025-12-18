use anchor_lang::prelude::*;

mod state;
use state::*;
mod constants;
use constants::*;
mod instructions;
use instructions::*;

declare_id!("GSP7LL75gEw5EetxyAEjMiy4A9yij5oHEVjuGbjg18yu");

#[program]
pub mod contract {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }
}
