use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    #[max_len(100)]
    pub description: String,
    pub start_time: u64,
    pub end_time: u64,
    pub candidate_amount: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(50)]
    pub name: String,
    pub vote_count: u64,
    pub bump: u8,
}
