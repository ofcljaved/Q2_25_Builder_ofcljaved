use anchor_lang::prelude::*;

use crate::{Poll, ANCHOR_DISCRIMINATOR_SIZE};

#[derive(Accounts)]
#[instruction(poll_id:u64)]
pub struct InitPoll<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer = creator,
        space = ANCHOR_DISCRIMINATOR_SIZE + Poll::INIT_SPACE,
        seeds =[b"poll",poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,
}

impl<'info> InitPoll<'info> {
    pub fn init_poll(
        &mut self,
        poll_id: u64,
        description: String,
        start_time: u64,
        end_time: u64,
        bump: &InitPollBumps,
    ) -> Result<()> {
        self.poll.set_inner(Poll {
            poll_id,
            description,
            start_time,
            end_time,
            bump: bump.poll,
            candidate_amount: 0,
        });
        Ok(())
    }
}
