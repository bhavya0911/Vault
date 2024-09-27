use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = ANCHOR_DISC + U8_L * 2;
}