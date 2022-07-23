use anchor_lang::{prelude::*};

pub enum MovieInstruction {
  AddMovieReview {
    title: String,
    rating: u8,
    description: String
  },
  UpdateMovieReview {
    title: String,
    rating: u8,
    description: String
  }
}

impl MovieInstruction {
  pub fn unpack(variant: u8, title: String, rating: u8, description: String) -> Result<Self> {
        Ok(match variant {
            0 => Self::AddMovieReview {
                title: title,
                rating: rating,
                description: description },
            1 => Self::UpdateMovieReview {
                title: title,
                rating: rating,
                description: description },
            _ => return Err(ProgramError::InvalidInstructionData.into())
        })
    }
}