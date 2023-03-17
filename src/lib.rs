use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg
};
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
)-> ProgramResult {
    msg!("Hello world! {:?}",program_id);
    return Ok(());
}
entrypoint!(process_instruction);
