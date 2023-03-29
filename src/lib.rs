use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    program::{invoke_signed},
    borsh::try_from_slice_unchecked,
};
use std::convert::TryInto;

// define the enum for instruction data
pub enum MovieInstruction {
    AddMovieReview {
        title: String,
        rating: u8,
        description: String,
    }
}

// struct used to determine the parameters that define what needs to be saved in accounts.

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovieAccountState {
    pub is_initialized: bool,
    pub rating: u8,
    pub title: String,
    pub description: String,
}

// create an impl off of MovieInstruction that parses the u8 instruction datatype
impl MovieInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        //split the first byte of the data
        let (&varient, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)
            .unwrap();
        // `try_from_slice` is one of the implementations from the BorshDeserialization trait
        let payload = MovieReviewPayload::try_from_slice(rest).unwrap();
        // match the first byte and return the movie review sturct
        Ok(match varient {
            0 => Self::AddMovieReview {
                description: payload.description,
                title: payload.title,
                rating: payload.rating,
            },
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}

// function to log and add it to the blockchain accounts the incoming movie data
pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    // get account iterator
    let account_info_iter = &mut accounts.iter();
    //get  accounts
    let initializer = next_account_info(account_info_iter).unwrap();
    let pda_account = next_account_info(account_info_iter).unwrap();
    let system_program = next_account_info(account_info_iter).unwrap();

    //derive PDA and check it matches the client
    let (pda, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(), title.as_bytes()
        .as_ref()], program_id);
    // calculate the space and based-rent
    let account_len = 1 + 1 + (4 + title.len()) + (4 + description.len());
    // calculate the rent required
    let rent = Rent::get().unwrap();
    let rent_lamports = rent.minimum_balance(account_len);

    //create account
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[initializer.clone(), pda_account.clone(), system_program.clone()],
        &[&[initializer.key.as_ref(), title.as_bytes().as_ref(), &[bump_seed]]],
    ).expect("Invoking a new PDA failed.");
    msg!("PDA created: {}", pda);
    msg!("Unpacking the state account");

    let mut account_data = try_from_slice_unchecked::<MovieAccountState>
        (&pda_account.data.borrow()).unwrap();
    account_data.title = (&title).to_string();
    account_data.rating = rating;
    account_data.description = (&description).to_string();
    account_data.is_initialized = true;

    // serialize the account
    msg!("Serilalizing accounts");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..]).unwrap();
    msg!("state account serialized");

    // msg!("Adding the movie to blockchain");
    msg!("Title: {}",title);
    msg!("Description {}", description);
    Ok(())
}

// here is the review payload
#[derive(BorshDeserialize)]
struct MovieReviewPayload {
    title: String,
    rating: u8,
    description: String,
}

// hers is the entrypoint!
entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MovieInstruction::unpack(instruction_data)
        .unwrap();
    // matching the outgoing instruction variant
    match instruction {
        MovieInstruction::AddMovieReview { description, title, rating } => {
            add_movie_review(program_id, accounts, title, rating, description)
        }
    }
}
