use crate::processor::Processor;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

// This will serve as entrypoint for users
entrypoint!(process_instruction);
fn process_instruction(
    // Public key of program that solaan assigns
    program_id: &Pubkey,
    // the user details that solana will keep
    accounts: &[AccountInfo],
    // It contains all the data of the user
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)
}
