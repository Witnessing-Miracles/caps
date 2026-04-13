use anchor_lang::prelude::*;
use crate::errors::PayrollError;
use crate::states::Organization;

// Main handler function or withdrawing from treasury
pub fn withdraw(ctx: Context<WithdrawCtx>, amount: u64) -> Result<()> {
    // Validate: amount must be greater than 0
    require!(amount > 0, PayrollError::InvalidAmount);

    // Validate: treasury must have sufficient funds
    require!(
        ctx.accounts.org.treasury >= amount,
        PayrollError::InsufficientFunds
    );

    // Transfer lamports directly by manipulating account balances
    // We do this manually (not using system_program::transfer) because
    // the organization account contains program data, which would cause
    // conflicts with system program's transfer logic

    // Update the organization's treasury balance
    ctx.accounts.org.treasury -= amount;

    // Decrease organization's lamport balance by amount
    **ctx.accounts.org.to_account_info().try_borrow_mut_lamports()? -= amount;

    // Increase authority's lamport balance by amount
    **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += amount;

    // Log the withdrawal
    msg!("Withdrawn {} lamports from treasury.", amount);
    Ok(())
}

// Context struct: defines all accounts needed for this instruction
#[derive(Accounts)]
pub struct WithdrawCtx<'info> {
    // The organization account (funds being withdrawn from here)
    // has_one = authority: ensures only the org owner can withdraw
    // seeds: ensures this is the correct organization PDA
    #[account(
        mut,
        has_one = authority @ PayrollError::Unauthorized,
        seeds = [b"org", authority.key().as_ref(), org.name.as_bytes()],
        bump = org.bump
    )]
    pub org: Account<'info, Organization>,

    // The transaction signer (authority) receiving the withdrawal
    #[account(mut)]
    pub authority: Signer<'info>,

    // System program (included for consistency)
    pub system_program: Program<'info, System>,
}