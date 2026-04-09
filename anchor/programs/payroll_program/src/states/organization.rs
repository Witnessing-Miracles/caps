use anchor_lang::prelude::*;

// #[account] macro: tells Anchor this is an on-chain account
// #[derive(InitSpace)] macro: automatically calculates space for serialization
#[account]
#[derive(InitSpace)]
pub struct Organization {
    // Define owner of this Organization
    pub authority: Pubkey,

    // Defina name of this Organization (max 100 characters)
    // #[max_len(100)] limits string length for space calculation
    #[max_len(100)]
    pub name: String,

    // Total SOL in the organization's treasury (in lamports)
    pub treasury: u64,

    // Number of workers currently registered
    pub workers_count: u64,

    // Unix timestamp when organization creadted
    pub created_at: i64,

    // Bump seed for PDA
    pub bump: u8,
}

impl Organization {
    // Maximum allowed length for organization's name
    pub const MAX_NAME_LEN: usize  = 100;

    // Total space this account will occupy on-chain (in bytes)
    // Calculated as: 32(pubkey) + 4(String length prefix) + 100(100 bytes for String contents)
    // + 8 + 8 + 8 + 1 = 161 bytes
    pub const INIT_SPACE: usize = 32    // authority (Pubkey)
        + 4 + 100                       // name (String with max 100 chars)
        + 8                             // treasury (u64)
        + 8                             // workers_count (u64)
        + 8                             // created_at (i64)
        + 1;                            // bump (u8)
}
