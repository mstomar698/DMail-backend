use crate::error::MailError::InvalidInstruction;
use solana_program::program_error::ProgramError;

// This will declare endpoint InitAccount
#[derive(Debug)]
pub enum MailInstruction {
    /// Creating account here.
    /// * `[writable]` AccountInfo of the created Account
    InitAccount,
}

// To call Error when initiating account from InitAccount enum
impl MailInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitAccount,
            _ => return Err(InvalidInstruction.into()),
        })
    }
}
