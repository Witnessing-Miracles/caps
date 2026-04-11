use crate::errors::PayrollError;
use crate::states::{Organization, Worker};
use anchor_lang::prelude::*;

// Main handler function for processing payroll
pub fn process_payroll<'info>(
    ctx: Context<'_, '_, 'info, 'info, ProcessPayrollCtx<'info>>,
    cycle_timestamp: u64,
) -> Result<()> {
    // Get the number of workers from organization
    let num_workers = ctx.accounts.org.workers_count as usize;

    // Each worker needs 2 accounts: their PDA and their wallet
    // Calculate expected number of remaining accounts
    let num_expected_accounts = num_workers * 2;

    // Validate: correct number of worker accounts provided
    require!(
        ctx.remaining_accounts.len() == num_expected_accounts,
        PayrollError::MissingWorkerAccount
    );

    // Cache
}

// Context struct: defines required accounts for this instruction
#[derive(Accounts)]
pub struct ProcessPayrollCtx<'info> {
    // The organization account (treasury being debited)
    // has_one = authority: only organization owner can process payroll
    // seeds: ensures this is the correct organization PDA
    #[account(
        mut,
        has_one = authority @ PayrollError::Unauthorized,
        seeds = [b"org", authority.key().as_ref(), org.name.as_bytes()],
        bump = org.bump
    )]
    pub org: Account<'info, Organization>,

    // The transaction signer (must be organization authority)
    #[account(mut)]
    pub authority: Signer<'info>,

    // System program
    pub system_program: Program<'info, System>,

    // NOTE: Worker PDAs and wallet accounts are passed as remaining_accounts
    // They must be in alternating order: [worker_pda_1, wallet_1, worker_pda_2, wallet_2, ...]
    // This keeps the transaction flexible for any number of workers
}