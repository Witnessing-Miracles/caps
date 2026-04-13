use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::errors::PayrollError;
use crate::states::Organization;

// Main handler function for funding treasury
pub fn fund_treasury(ctx: Context<FundTreasuryCtx>, amount: u64) -> Result<()> {
    // Validate: amount must be more than 0
    require!(amount > 0, PayrollError::InvalidAmount);

    // Prepare CPI (Cross-Program Invocation) to system program
    // We're calling system program's transfer instruction
    let cpi_accounts = system_program::Transfer {
        // Transfer From: the authority's wallet
        from: ctx.accounts.authority.to_account_info(),
        // Transfer To: the organization's PDA account
        to: ctx.accounts.org.to_account_info(),
    };

    // Create CPI context with system program
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        cpi_accounts
    );

    // Update the organization's treasury balance
    ctx.accounts.org.treasury += amount;

    // Execute the transfer
    system_program::transfer(cpi_ctx, amount)?;

    // Log the funding event
    msg!("Treasury funded by {} lamports.", amount);
    Ok(())
}

// Context struct: defines all accounts needed for this instruction
#[derive(Accounts)]
pub struct FundTreasuryCtx<'info> {
    // The organization account receiving funds
    // has_one = authority: ensures authority signer is the org owner
    // seeds: ensures this is the correct organization PDA
    #[account(
        mut,
        has_one = authority @ PayrollError::Unauthorized,
        seeds = [b"org", authority.key().as_ref(), org.name.as_bytes()],
        bump = org.bump
    )]
    pub org: Account<'info, Organization>,

    // The account sending funds (must be transaction signer)
    #[account(mut)]
    pub authority: Signer<'info>,

    // System program (needed for transfer)
    pub system_program: Program<'info, System>
}
