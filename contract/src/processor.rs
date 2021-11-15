use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_program,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{error::CustomError, instruction::CustomInstruction, state::AccountData};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CustomInstruction::unpack(instruction_data)?;

        match instruction {
            CustomInstruction::FirstInstruction { data } => {
                msg!("Instruction: First");
                Self::do_what_first_instruction(accounts, data, program_id)
            }
            CustomInstruction::SecondInstruction { data } => {
                msg!("Instruction: Second");
                Self::do_what_second_instruction(accounts, data, program_id)
            }
        }
    }

    fn do_what_first_instruction(
        account_info_list: &[AccountInfo],
        data: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut account_info_list.iter();

        let wallet_account_info = next_account_info(account_info_iter)?;
        if !wallet_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let custom_account_info = next_account_info(account_info_iter)?;
        if *custom_account_info.owner != *program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        //check having enough SOL balance which can exempt to pay
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        //if not error
        if !rent.is_exempt(
            custom_account_info.lamports(),
            custom_account_info.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        //get data from account (needed `is_writable = true` option)
        let mut custom_account_data =
            AccountData::unpack_unchecked(&custom_account_info.try_borrow_data()?)?;
        if custom_account_data.is_initialized {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        custom_account_data.init(
            true,
            *wallet_account_info.key,
            *custom_account_info.key,
            data,
        );

        AccountData::pack(
            custom_account_data,
            &mut custom_account_info.try_borrow_mut_data()?,
        )?;

        return Ok(());
    }

    fn do_what_second_instruction(
        accounts: &[AccountInfo],
        data: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();
        return Ok(());
    }
}
