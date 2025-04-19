use crate::processor;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

//entrypoint point of program
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "process_instruction: Program {} is executed with {} account(s) and the following data={:?}",
        program_id,
        accounts.len(),
        _instruction_data
    );
    processor::process_instruction(program_id, accounts, _instruction_data)?;
    Ok(())
}
