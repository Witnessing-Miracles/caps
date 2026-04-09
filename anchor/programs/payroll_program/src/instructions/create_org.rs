use anchor_lang::prelude::*;
use crate::states::Organization;
use crate::errors::PayrollError;

// Main handler function for creating an organization
pub fn create_org(ctx: Context<CreateOrgCtx>, name: String) -> Result<()> {
    // Validate: Organization's name not exceed maximum length
    require!(
        name.len() <= Organization::MAX_NAME_LEN,
        PayrollError::InvalidName
    );

    // Get mutable reference to the organization account
    let org = &mut ctx.accounts.org;

    // Set the authority (owner) to the transaction signer
    org.authority = ctx.accounts.authority.key();

    // Store the organization name
    org.name = name.clone();

    // Initialize treasury to 0
    org.treasury = 0;

    // Initialize workers count to 0
    org.workers_count = 0;

    // Set creation timestamp to current block time
    org.created_at = Clock::get()?.unix_timestamp;

    // Store the bump seed (provided by Anchor)
    org.bump = ctx.bumps.org;

    // Log message for debugging and user feedback
    msg!("Organization '{}' created.", name);

    Ok(())
}

// Context struct: defines all accounts needed for this instruction
pub struct CreateOrgCtx<'info> {
    /**
     * The organization account being created (This is a decoration, providing more background info for account)
     * init: if this account not exist, create it
     * payer: who pays for this account's creation? (the authority)
     * space: allocates space according to Organization::INIT_SPACE, one thing should be noticed:
       The first 8 bytes are the discriminator for the Anchor account. Each Anchor account needs these 8 bytes
       to identify the account type. They must be included, otherwise there will be insufficient storage space
       and an error will occur.
     * seeds: derives PDA from authority pubkey and organization name
     * bump: stores the bump value for reproducible PDA generation
     */
    #[account(
        init,
        payer = authority,
        space = 8 + Organization::INIT_SPACE,
        seeds = [b"org", authority.key().as_ref()],
        bump
    )]

    pub org: Account<'info, Organization>,

    // The transaction signer (must be the authority)
    // mut: allows modification of account (needed to deduct rent)
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
