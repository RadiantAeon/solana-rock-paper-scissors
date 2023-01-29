extern crate solana_program;

use std::convert::TryInto;

use self::solana_program::{
    msg,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES}
};

#[derive(Clone, Debug, PartialEq)]
pub enum RPSInstruction {
    Init {
        withdraw_fee_authority: Pubkey,
    },
    PlaceChallenge {
        hash: [u8; 32]
    },
    Crank,
    WithdrawFees,
}

impl RPSInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => {
                let (withdraw_fee_authority, _rest) = Self::unpack_pubkey(rest)?;
                Self::Init { withdraw_fee_authority }
            }
            1 => {
                let (hash, _rest) = Self::unpack_hash(rest)?;
                Self::PlaceChallenge { hash }
            }
            2 => {
                Self::Crank {}
            }
            3 => {
                Self::WithdrawFees {}
            }
            4_u8..=u8::MAX => todo!()
        })
    }

    fn unpack_hash(input: &[u8]) -> Result<([u8;32], &[u8]), ProgramError> {
        if input.len() < 32 {
            msg!("Hash cannot be unpacked");
            return Err(ProgramError::InvalidInstructionData);
        }
        let (hash, rest) = input.split_at(32);
        Ok((hash.try_into().unwrap(), rest))
    }

    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() < PUBKEY_BYTES {
            msg!("Pubkey cannot be unpacked");
            return Err(ProgramError::InvalidInstructionData);
        }
        let (key, rest) = input.split_at(PUBKEY_BYTES);
        let pk = Pubkey::new(key);
        Ok((pk, rest))
    }
}