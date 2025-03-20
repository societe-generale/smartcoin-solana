use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Redemptions {
    pub bump: u8, // 1
    pub max_size: u64, // 8
    pub redemption: Vec<Redemption>, // 4 + x * REDEMPTION_SIZE
}

#[derive(Default, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Redemption {
    pub amount: u64, // 8
    pub sender: Pubkey, // 32
    pub recipient: Pubkey, // 32
    pub end_date: i64, // 8
    pub done: bool, // 1
}

impl Redemptions {
    pub const REDEMPTION_SIZE: usize = 8 + 32 + 32 + 8 + 1;
    pub const INIT_SIZE: usize =  8 + 1 + 8 + 4 + Self::REDEMPTION_SIZE;
    pub fn space(max_size: u64) -> usize {
        8 + 1 + 8 + 4 + ((max_size + 1) as usize * Self::REDEMPTION_SIZE)
    }
}