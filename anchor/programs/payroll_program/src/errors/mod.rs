use anchor_lang::prelude::*;

// #[error_code] macro: creates an error enum for the program
#[error_code]
pub enum PayrollError {
    // Error when caller is not authorized to perform action 
    #[msg("Unauthorized access")]
    Unauthorized,

    // Error when organization's name is too long (exceeds MAX_NAME_LEN)
    #[msg("Invalid organization name length")]
    InvalidName,

    // Error when salary amount is 0 or invalid
    #[msg("Invalid salary amount")]
    InvalidSalary,

    // Error when amount parameter is 0 or invalid
    #[msg("Invalid amount")]
    InvalidAmount,

    // Error when treasury doesn't have enough funds
    #[msg("Insufficient funds in treasury")]
    InsufficientFunds,

    // Error when worker accounts are missing in remaining_accounts
    #[msg("Missing worker account in remaining accounts")]
    MissingWorkerAccount,

    // Error when provided worker PDA doesn't match expected PDA
    #[msg("Invalid worker PDA")]
    InvalidWorkerPDA,

    // Error when worker's wallet pubkey is invalid
    #[msg("Invalid worker wallet pubkey")]
    InvalidWorkerWallet,
}