extern crate solana_program;
extern crate arrayref;

use self::arrayref::{array_mut_ref, mut_array_refs, array_ref, array_refs};
use self::solana_program::{
    msg,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
    program_pack::{Pack, IsInitialized, Sealed}
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Game {
    // who created this account(who gets the rent back)
    pub creator: Pubkey,
    // how much does each player bet
    pub size: u64,
    // what token
    pub token_mint: Pubkey,
    // player 1
    pub player_1: Player,
    // player 2
    pub player_2: Player,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Player {
    pub pubkey: Pubkey,
    pub hash: [u8; 32],
    // moves should only be added after both hashes are in
    pub rps_move: RPSMove
}


#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(u8)]
pub enum RPSMove {
    #[default]
    None = 0,
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl RPSMove {
    pub fn is_none(&self) -> bool {
        self == &Self::None
    }

    pub fn from_u8(int: u8) -> Self {
        match int {
            0 => Self::None,
            1 => Self::Rock,
            2 => Self::Paper,
            3 => Self::Scissors
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[repr(u8)]
pub enum Winner {
    #[default]
    Neither,
    Player1,
    Player2,
}

impl Game {
    pub fn winner(&self) -> Result<Winner, ProgramError> {
        if self.player_1.rps_move.is_none() || self.player_2.rps_move.is_none() {
            // @TODO - customize errors
            msg!("Not all players have submitted their move yet");
            return Err(ProgramError::InvalidAccountData)
        }

        let player_1_move = self.player_1.rps_move;
        let player_2_move = self.player_2.rps_move;

        // :) i love me if/else ladder
        match player_1_move {
            RPSMove::Rock => {
                match player_2_move {
                    RPSMove::Rock => Ok(Winner::Neither),
                    RPSMove::Paper => Ok(Winner::Player2),
                    RPSMove::Scissors => Ok(Winner::Player1),
                    RPSMove::None => unimplemented!(),
                }
            },
            RPSMove::Paper => {
                match player_2_move {
                    RPSMove::Rock => Ok(Winner::Player1),
                    RPSMove::Paper => Ok(Winner::Neither),
                    RPSMove::Scissors => Ok(Winner::Player2),
                    RPSMove::None => unimplemented!(),
                }
            },
            RPSMove::Scissors => {
                match player_2_move {
                    RPSMove::Rock => Ok(Winner::Player2),
                    RPSMove::Paper => Ok(Winner::Player1),
                    RPSMove::Scissors => Ok(Winner::Neither),
                    RPSMove::None => unimplemented!(),
                }
            },
            RPSMove::None => unimplemented!(),
        }
    }
}

impl Sealed for Game {}
impl IsInitialized for Game {
    fn is_initialized(&self) -> bool {
        // @TODO - is this legit? jdjkf
        self.token_mint != Pubkey::default()
    }
}

const GAME_LEN: usize = 202;
impl Pack for Game {
    const LEN: usize = GAME_LEN;

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, GAME_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            creator,
            size,
            token_mint,
            player_1,
            player_2,
        ) = mut_array_refs![
            output,
            PUBKEY_BYTES,
            8,
            PUBKEY_BYTES,
            PLAYER_LEN,
            PLAYER_LEN
        ];

        creator.copy_from_slice(self.creator.as_ref());
        *size = self.size.to_le_bytes();
        token_mint.copy_from_slice(self.token_mint.as_ref());
        self.player_1.pack_into_slice(player_1);
        self.player_2.pack_into_slice(player_2);
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, GAME_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            creator,
            size,
            token_mint,
            player_1,
            player_2,
        ) = array_refs![
            input,
            PUBKEY_BYTES,
            8,
            PUBKEY_BYTES,
            PLAYER_LEN,
            PLAYER_LEN
        ];

        Ok(Self {
            creator: Pubkey::new_from_array(*creator),
            size: u64::from_le_bytes(*size),
            token_mint: Pubkey::new_from_array(*token_mint),
            player_1: Player::unpack(player_1)?,
            player_2: Player::unpack(player_2)?,
        })
    }
}

impl Sealed for Player {}
impl IsInitialized for Player {
    fn is_initialized(&self) -> bool {
        self.pubkey != Pubkey::default()
    }
}

const PLAYER_LEN: usize = 65;
impl Pack for Player {
    const LEN: usize = PLAYER_LEN;

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, PLAYER_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            pubkey,
            hash,
            rps_move
        ) = mut_array_refs![
            output,
            PUBKEY_BYTES,
            32,
            1
        ];

        pubkey.copy_from_slice(self.pubkey.as_ref());
        *hash = self.hash;
        *rps_move = (self.rps_move as u8).to_le_bytes();
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, PLAYER_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            pubkey,
            hash,
            rps_move,
        ) = array_refs![
            input,
            PUBKEY_BYTES,
            32,
            1
        ];

        Ok(Self {
            pubkey: Pubkey::new_from_array(*pubkey),
            hash: *hash,
            rps_move: RPSMove::from_u8(u8::from_le_bytes(*rps_move)),
        })
    }
}