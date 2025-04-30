use anchor_lang::prelude::*;

use crate::{Candidate, Poll};

#[derive(Accounts)]
#[instruction(poll_id:u64, name:String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds =[b"poll",poll_id.to_le_bytes().as_ref()],
        bump=poll.bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        mut,
        seeds =[b"candidate",poll_id.to_le_bytes().as_ref(), name.as_ref()],
        bump = candidate.bump
    )]
    pub candidate: Account<'info, Candidate>,
}

impl<'info> Vote<'info> {
    pub fn vote(&mut self, _poll_id: u64, _name: String) -> Result<()> {
        self.candidate.vote_count += 1;
        Ok(())
    }
}
