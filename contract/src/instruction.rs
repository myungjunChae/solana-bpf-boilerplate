use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::CustomError::InvalidInstruction;

pub enum CustomInstruction {
    FirstInstruction { data: u64 },
    SecondInstruction { data: u64 },
    // add more instruction what you want
}

//function of enum
impl CustomInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        //check instruction type
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        //unpack the rest data for each instruction
        return match tag {
            0 => Ok(Self::FirstInstruction {
                data: Self::unpack_data(rest)?,
            }),
            1 => Ok(Self::SecondInstruction {
                data: Self::unpack_data(rest)?,
            }),
            _ => Err(InvalidInstruction.into()),
        };
    }

    fn unpack_data(input: &[u8]) -> Result<u64, ProgramError> {
        let data = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;

        return Ok(data);
    }
}
