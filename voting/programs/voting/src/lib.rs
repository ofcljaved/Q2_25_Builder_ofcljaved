#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Dxc1yvZEVFWsaxiYDECdD77r7hnBoWXtokFhzXTEGqJv");

#[program]
pub mod voting {
    use super::*;

    pub fn init_poll(
        ctx: Context<InitPoll>,
        poll_id: u64,
        description: String,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {
        ctx.accounts
            .init_poll(poll_id, description, start_time, end_time, &ctx.bumps)
    }

    pub fn init_candidate(ctx: Context<InitCandidate>, poll_id: u64, name: String) -> Result<()> {
        ctx.accounts.init_candidate(poll_id, name, &ctx.bumps)
    }

    pub fn vote(ctx: Context<Vote>, poll_id: u64, name: String) -> Result<()> {
        ctx.accounts.vote(poll_id, name)
    }
}
