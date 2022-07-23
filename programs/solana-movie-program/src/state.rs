use anchor_lang::{prelude::*, solana_program};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
};

#[account]
pub struct MovieAccountState {
    pub is_initialized: bool,
    pub rating: u8,
    pub title: String,
    pub description: String
}

impl Sealed for MovieAccountState {}

impl IsInitialized for MovieAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}