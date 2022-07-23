use anchor_lang::prelude::*;

#[error_code]
pub enum ReviewError{
  #[msg("Account not initialized yet")]
  UninitializedAccount,
  
  #[msg("PDA derived does not equal PDA passed in")]
  InvalidPDA,
  
  #[msg("Input data exceeds max length")]
  InvalidDataLength,
  
  #[msg("Rating greater than 5 or less than 1")]
  InvalidRating,
}