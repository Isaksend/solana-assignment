use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

// Declare the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Log a message to the blockchain
    msg!("Hello, world!");
    
    // Print program and account information
    msg!("Program ID: {}", program_id);
    msg!("Number of accounts: {}", accounts.len());
    
    // Process the instruction data if any
    if !instruction_data.is_empty() {
        msg!("Instruction data: {:?}", instruction_data);
    }
    
    Ok(())
}pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
