use crate::error::MailError::NotWritable;
use crate::instruction::MailInstruction;
use crate::state::{DataLength, Mail, MailAccount};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::borsh::get_instance_packed_len;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};
use std::convert::TryFrom;

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = MailInstruction::unpack(instruction_data)?;

        match instruction {
            MailInstruction::InitAccount => {
                msg!("Instruction: InitAccount");
                Self::process_init_account(&accounts[0], program_id)
            }

            MailInstruction::SendMail { mail } => {
                msg!("Instruction: SendMail");
                Self::process_send_mail(accounts, &mail, program_id)
            }
        }
    }

    fn process_init_account(account: &AccountInfo, program_id: &Pubkey) -> ProgramResult {
        if !account.is_writable {
            return Err(NotWritable.into());
        }

        if account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let welcome_mail = Mail {
            id: String::from("00000000-0000-0000-0000-000000000000"),
            from_address: program_id.to_string(),
            to_address: account.key.to_string(),
            subject: String::from("Welcome to mail-dApp"),
            body: String::from("here body section will come!!"),
            sent_date: String::from("9/29/2021, 3:58:02 PM"),
        };

        let mail_account = MailAccount {
            inbox: vec![welcome_mail],
            sent: Vec::new(),
        };

        let data_length = DataLength {
            length: u32::try_from(get_instance_packed_len(&mail_account)?).unwrap(),
        };

        let offset: usize = 4;
        data_length.serialize(&mut &mut account.data.borrow_mut()[..offset])?;
        mail_account.serialize(&mut &mut account.data.borrow_mut()[offset..])?;
        Ok(())
    }

    fn process_send_mail(
        accounts: &[AccountInfo],
        mail: &Mail,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let sender_account = &accounts[0];

        if !sender_account.is_writable {
            return Err(NotWritable.into());
        }

        if sender_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let receiver_account = &accounts[1];

        if !receiver_account.is_writable {
            return Err(NotWritable.into());
        }

        if receiver_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let offset: usize = 4;

        let data_length = DataLength::try_from_slice(&sender_account.data.borrow()[..offset])?;

        let mut sender_data;
        if data_length.length > 0 {
            let length =
                usize::try_from(data_length.length + u32::try_from(offset).unwrap()).unwrap();
            sender_data =
                MailAccount::try_from_slice(&sender_account.data.borrow()[offset..length])?;
        } else {
            sender_data = MailAccount {
                inbox: Vec::new(),
                sent: Vec::new(),
            };
        }

        sender_data.sent.push(mail.clone());
        let data_length = DataLength {
            length: u32::try_from(get_instance_packed_len(&sender_data)?).unwrap(),
        };

        data_length.serialize(&mut &mut sender_account.data.borrow_mut()[..offset])?;
        sender_data.serialize(&mut &mut sender_account.data.borrow_mut()[offset..])?;

        let data_length = DataLength::try_from_slice(&receiver_account.data.borrow()[..offset])?;

        let mut reciever_data;
        if data_length.length > 0 {
            let length =
                usize::try_from(data_length.length + u32::try_from(offset).unwrap()).unwrap();
            reciever_data =
                MailAccount::try_from_slice(&receiver_account.data.borrow()[offset..length])?;
        } else {
            reciever_data = MailAccount {
                inbox: Vec::new(),
                sent: Vec::new(),
            }
        }
        reciever_data.inbox.push(mail.clone());

        let data_length = DataLength {
            length: u32::try_from(get_instance_packed_len(&reciever_data)?).unwrap(),
        };
        data_length.serialize(&mut &mut receiver_account.data.borrow_mut()[..offset])?;
        reciever_data.serialize(&mut &mut receiver_account.data.borrow_mut()[offset..])?;

        Ok(())
    }
}

//Test Here
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_init_account() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = [0; 1000];

        let account = AccountInfo::new(
            &key,
            true,
            true,
            &mut lamports,
            &mut data,
            &program_id,
            false,
            Epoch::default(),
        );

        Processor::process_init_account(&account, &program_id).unwrap();

        let data_length = DataLength::try_from_slice(&account.data.borrow()[..4]).unwrap();
        let mail_account = MailAccount::try_from_slice(
            &account.data.borrow()[4..usize::try_from(data_length.length + 4).unwrap()],
        )
        .unwrap();

        assert_eq!(mail_account.inbox[0].subject, "Welcome to SolMail");
    }

    #[test]
    fn test_send_mail() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut sender_data = [0; 1000];

        let sender_account = AccountInfo::new(
            &key,
            true,
            true,
            &mut lamports,
            &mut sender_data,
            &program_id,
            false,
            Epoch::default(),
        );

        let mut receiver_data = [0; 1000];
        let mut lamports = 0;

        let receiver_account = AccountInfo::new(
            &key,
            true,
            true,
            &mut lamports,
            &mut receiver_data,
            &program_id,
            false,
            Epoch::default(),
        );

        let accounts = vec![sender_account.clone(), receiver_account.clone()];

        let mail = Mail {
      id: String::from("00000000-0000-0000-0000-000000000000"),
      from_address: sender_account.key.to_string(),
      to_address: receiver_account.key.to_string(),
      subject: String::from("Hey Mike!!!"),
      body: String::from(
        "Lorem, ipsum dolor sit amet consectetur adipisicing elit. Quos ut labore, debitis assumenda, dolorem nulla facere soluta exercitationem excepturi provident ipsam reprehenderit repellat quisquam corrupti commodi fugiat iusto quae voluptates!"
      ),
      sent_date: String::from("9/29/2021, 3:58:02 PM")
    };

        Processor::process_send_mail(&accounts, &mail, &program_id).unwrap();

        let data_length = DataLength::try_from_slice(&sender_account.data.borrow()[..4]).unwrap();
        let mail_account = MailAccount::try_from_slice(
            &sender_account.data.borrow()[4..usize::try_from(data_length.length + 4).unwrap()],
        )
        .unwrap();

        assert_eq!(mail_account.sent[0].subject, "Hey Mike!!!");

        let data_length = DataLength::try_from_slice(&receiver_account.data.borrow()[..4]).unwrap();
        let mail_account = MailAccount::try_from_slice(
            &receiver_account.data.borrow()[4..usize::try_from(data_length.length + 4).unwrap()],
        )
        .unwrap();

        assert_eq!(mail_account.inbox[0].subject, "Hey Mike!!!");
    }
}
