//! Program instruction processor

extern crate solana_program;

use std::borrow::Borrow;

use crate::{instruction::RPSInstruction, state::game::Game};

use self::solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    log::{sol_log_compute_units, sol_log_params, sol_log_slice},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    config::program,
    program_pack::Pack
};

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let instruction = RPSInstruction::unpack(instruction_data)?;

    match instruction {
        RPSInstruction::Init { withdraw_fee_authority } => {
            msg!("Instruction: Init");
            todo!()
        }
        RPSInstruction::PlaceChallenge { hash } => {
            msg!("Instruction: Place Challenge");
            todo!()
        }
        RPSInstruction::Crank => {
            msg!("Instruction: Crank");
            todo!()
        }
        RPSInstruction::WithdrawFees => {
            msg!("Instruction: Withdraw fees");
            todo!()
        }
    }

    Ok(())
}

pub fn process_crank(
    program_id: &Pubkey,
    accounts: &[AccountInfo]
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let game_account_info = next_account_info(account_info_iter)?;

    if game_account_info.owner != program_id {
        msg!("Game account provided is not owned by the program");
        return Err(ProgramError::IllegalOwner);
    }

    let game_state = Game::unpack(&game_account_info.data.as_ref().borrow())?;

    if game_state.player_1.
}
