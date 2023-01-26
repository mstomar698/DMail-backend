use solana_program::program_error::ProgramError;
use thiserror::Error;

// It uses fmt::Dislpay to display error
#[derive(Error, Debug, Copy, Clone)]
pub enum MailError {
    /// Invalid Instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not Writable
    #[error("Not Writable")]
    NotWritable,
}

// For returning error as ProgramError
impl From<MailError> for ProgramError {
    fn from(e: MailError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

