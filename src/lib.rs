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
use solana_program::program_pack::{IsInitialized, Sealed};
use thiserror::Error;

// define the enum for instruction data
pub enum OpporunityDef {
    AddNewOppotunity {
        //adding some common salesforce keys and vals
        title: String,
        amount: u32,
        stage: String,
        account_name: String,
        delivered: bool,
        probability: u8,
    },
    UpdateOpportunity {
        title: String,
        amount: u32,
        stage: String,
        account_name: String,
        delivered: bool,
        probability: u8,
    },
}

// struct used to determine the parameters that define what needs to be saved in accounts.
#[derive(BorshSerialize, BorshDeserialize)]
pub struct OpporDataState {
    pub is_initialized: bool,
    pub title: String,
    pub amount: u32,
    pub stage: String,
    pub account_name: String,
    pub delivered: bool,
    pub probability: u8,
}

// impl for AccountState
impl Sealed for OpporDataState {}

//sealed is Solana's implementation of Sized rust tract.
impl IsInitialized for OpporDataState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

// create an impl off of OpporunityDef that parses the u8 instruction datatype
impl OpporunityDef {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        //split the first byte of the data
        let (&varient, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)
            .unwrap();
        msg!("{:?}",varient);
        // `try_from_slice` is one of the implementations from the BorshDeserialization trait
        let payload = OpporDataPayload::try_from_slice(rest).unwrap();
        // match the first byte and return the opportunity data sturct
        Ok(match varient {
            0 => Self::AddNewOppotunity {
                account_name: payload.account_name,
                title: payload.title,
                probability: payload.probability,
                stage: payload.stage,
                delivered: payload.delivered,
                amount: payload.amount,
            },
            1 => Self::UpdateOpportunity {
                account_name: payload.account_name,
                title: payload.title,
                probability: payload.probability,
                stage: payload.stage,
                delivered: payload.delivered,
                amount: payload.amount,
            },
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}

// function to log and add it to the blockchain accounts the incoming opportunity data
pub fn add_new_oppor(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    account_name: String,
    amount: u32,
    delivered: bool,
    probability: u8,
    stage: String,
    title: String,
) -> ProgramResult {
    // get account iterator
    let account_info_iter = &mut accounts.iter();
    //get  accounts
    let initializer = next_account_info(account_info_iter).unwrap();
    let pda_account = next_account_info(account_info_iter).unwrap();
    let system_program = next_account_info(account_info_iter).unwrap();

    //verify transaction signer
    if !initializer.is_signer {
        msg!("Missing required signature!");
        return Err(ProgramError::MissingRequiredSignature);
    }

    //derive PDA and check it matches the client
    let (pda, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(), title.as_bytes()
        .as_ref()], program_id);

    //pda verification
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA creation");
        return Err(ReviewError::InvalidPDA.into());
    }

    // calculate the size of the incoming data
    let request_len = 1 + (4 + account_name.len()) + 4 + 1 + 1 + (4 + stage.len()) + (4 + title.len
    ());
    if request_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(ReviewError::InvalidDataLength.into());
    }

    //maximum allowed data size
    let account_len = 1000;
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
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone()
        ],
        &[
            &[
                initializer.key.as_ref(),
                title.as_bytes().as_ref(),
                &[bump_seed]
            ]
        ],
    ).expect("Invoking a new PDA failed.");
    msg!("PDA created: {}", pda);
    msg!("Unpacking the state account");

    let mut account_data = try_from_slice_unchecked::<OpporDataState>
        (&pda_account.data.borrow()).unwrap();

    //checking is the account is already initialized
    if account_data.is_initialized() {
        msg!("The account is already initilized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_data.account_name = (&account_name).to_string();
    account_data.probability = probability;
    account_data.stage = (&stage).to_string();
    account_data.delivered = delivered;
    account_data.amount = amount;
    account_data.title = (&title).to_string();
    account_data.is_initialized = true;

    // serialize the account
    msg!("Serilalizing accounts");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..]).unwrap();
    msg!("state account serialized");

    // msg!("Adding the opporunity to blockchain");
    msg!("Title: {}",title);
    msg!("Amount added: {}", amount);
    Ok(())
}

fn update_oppor_data(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    account_name: String,
    amount: u32,
    delivered: bool,
    probability: u8,
    stage: String,
    title: String,
) -> ProgramResult {
    msg!("Updating the exising review");
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter).unwrap();
    let pda_account = next_account_info(account_info_iter).unwrap();

    // unpack the incoming data
    let mut account_data = try_from_slice_unchecked::<OpporDataState>
        (&pda_account.data.borrow()).unwrap();
    let (pda, seeds) = Pubkey::find_program_address(
        &[
            initializer.key.as_ref(),
            account_data.title.as_bytes()
        ],
        program_id);
    //check for size
    let update_len: usize = 1 + (4 + account_name.len()) + 4 + 1 + 1 + (4 + stage.len()) + (4 + title.len
    ());
    account_data.account_name = (&account_name).to_string();
    account_data.probability = probability;
    account_data.stage = (&stage).to_string();
    account_data.delivered = delivered;
    account_data.amount = amount;
    account_data.title = title;
    // now save the data
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..]).unwrap();
    msg!("The data has been updated!!");
    Ok(())
}

// here is the review payload
#[derive(BorshDeserialize)]
struct OpporDataPayload {
    pub title: String,
    pub account_name: String,
    pub amount: u32,
    pub delivered: bool,
    pub probability: u8,
    pub stage: String,
}

// hers is the entrypoint!
entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = OpporunityDef::unpack(instruction_data)
        .unwrap();
    // matching the outgoing instruction variant
    match instruction {
        OpporunityDef::AddNewOppotunity {
            account_name,
            amount,
            delivered,
            probability,
            stage,
            title
        } => {
            add_new_oppor(program_id,
                          accounts,
                          account_name,
                          amount,
                          delivered,
                          probability,
                          stage,
                          title,
            )
        }
        OpporunityDef::UpdateOpportunity { account_name, amount, delivered, probability, stage, title }
        => {
            update_oppor_data(program_id,
                              accounts,
                              account_name,
                              amount,
                              delivered,
                              probability,
                              stage,
                              title,
            )
        }
    }
}

//custom error section
#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("Account not initialized yet")]
    UninitializedAccount,

    #[error("PDA derived does not equal PDA passed in")]
    InvalidPDA,

    #[error("Input data exceeds max length")]
    InvalidDataLength,

    #[error("Rating greater than 5 or less than 1")]
    InvalidRating,
}

//adding support for custom error in Solana
impl From<ReviewError> for ProgramError {
    fn from(value: ReviewError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
