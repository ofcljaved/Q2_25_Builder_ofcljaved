#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("v7DNSqZrkKQrdv5Mg4beh7PiCxWJhagcXFgNFiY9Cnv");
const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favourite {
    use super::*;

    pub fn set_favourite(
        ctx: Context<SetFavourite>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.set_favourite(number, color, hobbies);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetFavourite<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        init_if_needed,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR_SIZE+ Favourite::INIT_SPACE,
        seeds = [b"favourite", owner.key().as_ref()],
        bump
    )]
    pub favourite: Account<'info, Favourite>,
}

impl<'info> SetFavourite<'info> {
    pub fn set_favourite(
        &mut self,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        let user = self.owner.key();
        msg!(
            "User {user} 's favourite number: {number}, color: {color} and hobbies are {hobbies:?}",
        );
        self.favourite.set_inner(Favourite {
            number,
            color,
            hobbies,
        });
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favourite {
    pub number: u64,
    #[max_len(50)]
    pub color: String,
    #[max_len(4, 50)]
    pub hobbies: Vec<String>,
}
