//Random doubts
// pda seeds and bump seeds are same ?
// try changing initializer position in process_undelegate function

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

use ephemeral_rollups_sdk::cpi::{
    delegate_account, undelegate_account, DelegateAccounts, DelegateConfig,
};
use ephemeral_rollups_sdk::ephem::{commit_accounts, commit_and_undelegate_accounts};

use crate::{instructions::ProgramInstruction, states::Counter};

// program's entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // Unpack instruction discriminator and data
    // The instruction data is a byte array that contains the instruction discriminator
    // and the data for the instruction. The instruction discriminator is a unique identifier
    // for the instruction, and it is used to determine which instruction to execute.
    let instruction = ProgramInstruction::unpack(_instruction_data)?;

    match instruction {
        // 0: InitializeCounter
        ProgramInstruction::InitializeCounter => {
            msg!("Instruction: InitializeCounter");
            process_initialize_counter(program_id, accounts)
        }

        //1: IncreaseCounter
        ProgramInstruction::IncreaseCounter { increase_by } => {
            msg!("Instruction: InitializeCounter");
            process_increase_counter(program_id, accounts, increase_by)
        }

        //2: Delegate
        ProgramInstruction::Delegate => {
            msg!("Instruction:: Delegate");
            process_delegate(program_id, accounts)
        }

        //3: CommitAndUndelegate
        ProgramInstruction::CommitAndUndelegate => {
            msg!("Instruction: CommitAndUndelegate");
            process_commit_and_undelegate(program_id, accounts)
        }

        //4: Commit
        ProgramInstruction::Commit => {
            msg!("Instruction: Commit");
            process_commit(program_id, accounts)
        }

        //5: Undelegate
        ProgramInstruction::Undelegate { pda_seeds } => {
            msg!("Instruction: Undelegate");
            process_undelegate(program_id, accounts, pda_seeds)
        }
    }
}

pub fn process_initialize_counter(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    //Iterate Accounts
    let accounts_iter = &mut accounts.iter();
    let initializer_account = next_account_info(accounts_iter)?;
    let counter_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    //checking the client is sending correct PDA
    let (counter_pda, bump_seed) = Pubkey::find_program_address(
        &[b"counter_account", initializer_account.key.as_ref()],
        program_id,
    );
    if counter_pda != *counter_account.key {
        msg!("Invalid PDA");
        return Err(ProgramError::InvalidArgument);
    }

    // create acc if PDA doesn't exist
    // if exist update count to 0
    let borrowed_lamports = counter_account.try_borrow_lamports().unwrap();
    if *borrowed_lamports == &mut 0 {
        let rent = Rent::get()?;
        let rent_lamport = rent.minimum_balance(Counter::USIZE);
        msg!(
            "InitializingCounter account {} with {} lamports ",
            counter_pda,
            rent_lamport
        );

        drop(borrowed_lamports);
        invoke_signed(
            &system_instruction::create_account(
                initializer_account.key,
                counter_account.key,
                rent_lamport,
                Counter::USIZE.try_into().unwrap(),
                program_id,
            ),
            &[
                initializer_account.clone(),
                counter_account.clone(),
                system_program.clone(),
            ],
            &[&[
                b"counter_account",
                initializer_account.key.as_ref(),
                &[bump_seed],
            ]],
        )?;
        msg!(
            "counter acc {} created by {} and {}",
            counter_pda,
            counter_account.owner,
            initializer_account.key
        )
    }

    let mut counter_data = Counter::try_from_slice(&counter_account.data.borrow())?;
    msg!("Setting Count to 0");
    counter_data.count = 0;
    counter_data.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;

    Ok(())
}

pub fn process_increase_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    increase_by: u64,
) -> ProgramResult {
    //Iterate Accounts
    let accounts_iter = &mut accounts.iter();
    let initializer_account = next_account_info(accounts_iter)?;
    let counter_account = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;

    //checking the client is sending correct PDA
    let (counter_pda, _bump_seed) = Pubkey::find_program_address(
        &[b"counter_account", initializer_account.key.as_ref()],
        program_id,
    );
    if counter_pda != *counter_account.key {
        msg!("Invalid PDA");
        return Err(ProgramError::InvalidArgument);
    }

    // increse the count value
    let mut counter_data = Counter::try_from_slice(&counter_account.data.borrow())?;
    msg!("Increasing the count by {}", increase_by);
    counter_data.count += increase_by;
    counter_data.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;

    Ok(())
}

//Delegation
//State accounts are delegated to the Ephemeral Rollup via the Delegation Program, specifying parameters like lifetime and update frequency.
pub fn process_delegate(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let pda_to_delegate = next_account_info(account_info_iter)?;
    let owner_program = next_account_info(account_info_iter)?;
    let delegation_buffer = next_account_info(account_info_iter)?;
    let delegation_record = next_account_info(account_info_iter)?;
    let delegation_metadata = next_account_info(account_info_iter)?;
    let delegation_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Prepare counter pda seeds
    let seed_1 = b"counter_account";
    let seed_2 = initializer.key.as_ref();
    let pda_seeds: &[&[u8]] = &[seed_1, seed_2];

    let delegate_accounts = DelegateAccounts {
        payer: initializer,
        pda: pda_to_delegate,
        owner_program,
        buffer: delegation_buffer,
        delegation_record,
        delegation_metadata,
        delegation_program,
        system_program,
    };

    let delegate_config = DelegateConfig {
        commit_frequency_ms: 30_000,
        validator: None,
    };
    delegate_account(delegate_accounts, pda_seeds, delegate_config)?;
    Ok(())
}

//// Undelegates counter on the Base Layer (called on Base Layer through validator CPI)
pub fn process_undelegate(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    pda_seed: Vec<Vec<u8>>,
) -> ProgramResult {
    //Iterate Accounts
    let accounts_iter = &mut accounts.iter();
    let delegate_pda = next_account_info(accounts_iter)?;
    let delegation_buffer = next_account_info(accounts_iter)?;
    let initializer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    //CPI call
    let _ = undelegate_account(
        delegate_pda,
        program_id,
        delegation_buffer,
        initializer,
        system_program,
        pda_seed,
    );
    Ok(())
}

//Schedules sync of counter from ER to Base Layer, and undelegates counter on ER (called on ER)
pub fn process_commit_and_undelegate(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let counter_account = next_account_info(account_info_iter)?;
    let magic_program = next_account_info(account_info_iter)?;
    let magic_context = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        msg!("Initializer {} should be the signer", initializer.key);
        return Err(ProgramError::MissingRequiredSignature);
    }

    commit_and_undelegate_accounts(
        initializer,
        vec![counter_account],
        magic_context,
        magic_program,
    )?;

    Ok(())
}

// Schedules sync of counter from ER to Base Layer (called on ER)
pub fn process_commit(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    //Iterate Accounts
    let accounts_iter = &mut accounts.iter();
    let initializer = next_account_info(accounts_iter)?;
    let counter_account = next_account_info(accounts_iter)?;
    let magic_program = next_account_info(accounts_iter)?;
    let magic_context = next_account_info(accounts_iter)?;

    if !initializer.is_signer {
        msg!("Initializer {} should be the signer", initializer.key);
        return Err(ProgramError::MissingRequiredSignature);
    }

    commit_accounts(
        initializer,
        vec![counter_account],
        magic_context,
        magic_program,
    )?;

    Ok(())
}
