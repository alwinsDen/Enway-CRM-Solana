use borsh::{BorshDeserialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
};

// define the enum for instruction data
pub enum MovieInstruction {
    AddMovieReview {
        title: String,
        rating: u8,
        description: String,
    }
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
    msg!("Adding the movie to blockchain");
    msg!("Title: {}",title);
    msg!("Rating: {}", rating);
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
        MovieInstruction::AddMovieReview {description,title,rating}=> {
            add_movie_review(program_id,accounts,title,rating,description)
        }
    }
}


// ==========CREATING HELLO WORLD WITH SOLANA - Part 1============
// use solana_program::{
//     account_info::AccountInfo,
//     entrypoint,
//     entrypoint::ProgramResult,
//     pubkey::Pubkey,
//     msg
// };
// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8]
// )-> ProgramResult {
//     msg!("Hello world! {:?}",program_id);
//     return Ok(());
// }
// entrypoint!(process_instruction);
