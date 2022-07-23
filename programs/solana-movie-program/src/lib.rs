pub mod processor;
pub mod movie_instruction;
pub mod error;
pub mod state;

use anchor_lang::prelude::*;
use processor::*;

declare_id!("2SogeA4hASCYGTSQqoSqKy8cZ3bnka5N5U9Ewkswkyf5");

#[program]
pub mod solana_movie_program {
    use super::*;

    pub fn add_or_update_review(
        ctx: Context<AddOrUpdateReview>,
        variant: u8,
        title: String,
        rating: u8,
        description: String,
    ) -> Result<()> {
        process_instruction(ctx, variant, title, rating, description)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
