use anchor_lang::prelude::*;

// Import all modules we'll create
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("A9fnM3skS5kbECt2isFV2EGvS4D7hnsMjnqi2YBwaeed");

#[program]
pub mod payroll_program {
    use super::*;

    // Instruction 1: Create an organization
    // Parameters: organization name
    // Returns: Success or error
    pub fn create_org(ctx: Context<CreateOrgCtx>, name: String) -> Result<()> {
        instructions::create_org(ctx, name)
    }

    // Instruction 2: Add a worker to an organization
    // Parameters: worker's salary (in lamports)
    // Returns: Success or error
    pub fn add_worker(ctx: Context<AddWorkerCtx>, salary: u64) -> Result<()> {
        instructions::add_worker(ctx, salary)
    }

    // Instruction 3: Deposit funds into organization's treasury
    // Parameters: amount to deposit (in lamports)
    // Returns: Success or error
    pub fn fund_treasury(ctx: Context<FundTreasuryCtx>, amount: u64) -> Result<()> {
        instructions::fund_treasury(ctx, amount)
    }

    // Instruction 4: Process payroll for all workers in a batch
    // Parameters: cycle timestamp (when this payroll cycle started)
    // Uses remaining_accounts for worker PDAs and wallets
    // Returns: Success or error
    pub fn process_payroll<'info>(
        ctx: Context<'_, '_, 'info, 'info, ProcessPayrollCtx<'info>>,
        cycle_timestamp: u64,
    ) -> Result<()> {
        instructions::process_payroll(ctx, cycle_timestamp)
    }

    // Instruction 5: Withdraw funds from organization's treasury
    // Parameters: amount to withdraw (in lamports)
    // Returns: Success or error
    pub fn withdraw(ctx: Context<WithdrawCtx>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }
}