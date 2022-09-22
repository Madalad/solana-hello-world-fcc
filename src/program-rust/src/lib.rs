use solana_program::{
    entrypoint,
    msg,
    pubkey::Pubkey,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError
};
use borsh::{
    BorshSerialize,
    BorshDeserialize
};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize)] // be able to serialize and deserialize the data in the account
pub struct GreetingAccount {
    pub counter: u32
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    
    msg!("Hello World");
    
    let mut accounts_iter = accounts.iter(); // create iterator
    if let Some(account) = accounts_iter.next() { // safely access next element
        if account.owner != program_id {
            msg!("Account info does not match program id");
            //Err(ProgramError::IncorrectProgramId) // this raises error for some reason
            return Err(ProgramError::IncorrectProgramId);
        }
        // deserialize the data in the account variable:
        let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
        greeting_account.counter += 1;
        let acc_data = &mut account.data.borrow_mut()[..];
        // serialize account data in my program
        greeting_account.serialize(&mut acc_data.as_mut())?;
        msg!("{}", greeting_account.counter);
        Ok(())
    } else { // ran out of account keys
        msg!("Else!");
        Err(ProgramError::NotEnoughAccountKeys)
    }

}