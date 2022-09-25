use anchor_lang::{prelude::*, solana_program::program_pack::IsInitialized};
use crate::movie_instruction::MovieInstruction;
use crate::state::MovieAccountState;
use crate::error::ReviewError;

#[derive(Accounts)]
#[instruction(variant: u8, title: String, rating: u8, description: String)]
pub struct AddOrUpdateReview<'info> {
  #[account(mut)]
  pub initializer: Signer<'info>,
  #[account(
      init_if_needed, 
      seeds = [initializer.key.as_ref(), title.as_bytes().as_ref()],
      bump,
      payer = initializer,
      space = 1000,
      constraint = 1 + 1 + (4 + title.len()) + (4 + description.len()) <= 1000
        @ ReviewError::InvalidDataLength
  )]
  pub pda_account: Account<'info, MovieAccountState>,
  pub system_program: Program<'info, System>
}

pub fn process_instruction(
  ctx: Context<AddOrUpdateReview>,
  variant: u8,
  title: String,
  rating: u8,
  description: String,
) -> Result<()> {
  let instruction = MovieInstruction::unpack(variant, title, rating, description)?;
  match instruction {
    MovieInstruction::AddMovieReview { title, rating, description } => {
      add_movie_review(ctx, title, rating, description)
    },
    MovieInstruction::UpdateMovieReview { title, rating, description } => {
      update_movie_review(ctx, rating, description)
    }
  }
}

pub fn add_movie_review(
  ctx: Context<AddOrUpdateReview>,
  title: String,
  rating: u8,
  description: String
) -> Result<()> {
  msg!("Adding movie review...");
  msg!("Title: {}", title);
  msg!("Rating: {}", rating);
  msg!("Description: {}", description);

  if rating > 5 || rating < 1 {
      msg!("Rating cannot be higher than 5");
      return Err(ReviewError::InvalidRating.into())
  }

  msg!("checking if movie account is already initialized");
  if ctx.accounts.pda_account.is_initialized() {
      msg!("Account already initialized");
      return Err(ProgramError::AccountAlreadyInitialized.into());
  }

  ctx.accounts.pda_account.title = title;
  ctx.accounts.pda_account.rating = rating;
  ctx.accounts.pda_account.description = description;
  ctx.accounts.pda_account.is_initialized = true;

  msg!("PDA created: {}", ctx.accounts.pda_account.key());

  Ok(())
}

pub fn update_movie_review(
  ctx: Context<AddOrUpdateReview>,
  rating: u8,
  description: String
) -> Result<()> {    

  // check if movie account is initialized, if not return ReviewError::UninitializedAccount

  // check if rating is more than 5 or less than 1, if so return ReviewError::InvalidRating

  // set pda_account's rating to given rating

  // set pda_account's rating to given rating

  Ok(())
}