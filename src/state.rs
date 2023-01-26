use borsh::{BorshDeserialize, BorshSerialize};

// Declaring mail struct|| model to represent mail object.
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct Mail {
    pub id: String,
    pub from_address: String,
    pub to_address: String,
    pub subject: String,
    pub body: String,
    pub sent_date: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct MailAccount {
    pub inbox: Vec<Mail>,
    pub sent: Vec<Mail>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DataLength {
    pub length: u32,
}
