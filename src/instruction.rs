use crate::error::MailError::InvalidInstruction;
use crate::state::Mail;
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

// This will declare endpoint InitAccount
#[derive(Debug)]
pub enum MailInstruction {
    /// Creating account here.
    /// * `[writable]` AccountInfo of the created Account
    InitAccount,
    /// Sending mail to another account
    /// * `[writable]` AccountInfo of sender and reciever
    SendMail { mail: Mail },
}

// To call Error when initiating account from InitAccount enum
impl MailInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitAccount,
            1 => Self::SendMail {
                mail: Mail::try_from_slice(&rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}

