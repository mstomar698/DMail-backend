use crate::error::MailError::InvalidInstruction;
use crate::state::Mail;
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

// This will declare endpoint InitAccount
#[derive(Debug, PartialEq)]
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

//Tests here
#[cfg(test)]
mod test {
    use super::*;
    use borsh::BorshSerialize;
    use solana_program::{borsh::get_instance_packed_len, pubkey::Pubkey};

    #[test]
    fn test_init_endpoint() {
        let data: Vec<u8> = vec![0];

        let mail_instruction = MailInstruction::unpack(&data).unwrap();

        assert_eq!(mail_instruction, MailInstruction::InitAccount);
    }

    #[test]
    fn test_send_endpoint() {
        let test_mail = Mail {
            id: String::from("00000000-0000-0000-0000-000000000000"),
            from_address: Pubkey::default().to_string(),
            to_address: Pubkey::default().to_string(),
            subject: String::from("Hey Mike"),
            body: String::from("Body text with some characters"),
            sent_date: String::from("9/29/2021, 3:58:02 PM"),
        };

        let mut data: Vec<u8> = vec![1; get_instance_packed_len(&test_mail).unwrap() + 1];

        test_mail.serialize(&mut &mut data[1..]).unwrap();

        let mail_instruction = MailInstruction::unpack(&data).unwrap();

        assert_eq!(
            mail_instruction,
            MailInstruction::SendMail {
                mail: test_mail.clone()
            }
        );

        match mail_instruction {
            MailInstruction::SendMail { mail } => {
                assert_eq!(mail.from_address, test_mail.from_address);
                assert_eq!(mail.to_address, test_mail.to_address);
                assert_eq!(mail.subject, test_mail.subject);
                assert_eq!(mail.body, test_mail.body);
            }
            MailInstruction::InitAccount => (),
        }
    }
}
