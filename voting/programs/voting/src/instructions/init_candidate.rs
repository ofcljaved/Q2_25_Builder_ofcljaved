use anchor_lang::prelude::*;

use crate::{Candidate, Poll, ANCHOR_DISCRIMINATOR_SIZE};

#[derive(Accounts)]
#[instruction(poll_id:u64, name:String)]
pub struct InitCandidate<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds =[b"poll",poll_id.to_le_bytes().as_ref()],
        bump=poll.bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        init,
        payer = creator,
        space = ANCHOR_DISCRIMINATOR_SIZE + Candidate::INIT_SPACE,
        seeds =[b"candidate",poll_id.to_le_bytes().as_ref(), name.as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>,
}

impl<'info> InitCandidate<'info> {
    pub fn init_candidate(
        &mut self,
        _poll_id: u64,
        name: String,
        bump: &InitCandidateBumps,
    ) -> Result<()> {
        self.candidate.set_inner(Candidate {
            name,
            vote_count: 0,
            bump: bump.candidate,
        });
        self.poll.candidate_amount += 1;
        Ok(())
    }
}
