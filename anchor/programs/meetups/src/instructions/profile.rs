use anchor_lang::prelude::*;

use crate::{IdentityProfile, ANCHOR_DISCRIMINATOR_SIZE};

#[derive(Accounts)]
pub struct InitIdentityProfile<'info> {
    #[account(mut)]
    funder: Signer<'info>,
    #[account(
        init,
        payer = funder,
        space = ANCHOR_DISCRIMINATOR_SIZE + IdentityProfile::INIT_SPACE,
        seeds = [b"identity", funder.key().as_ref()],
        bump,
    )]
    identity_profile: Account<'info, IdentityProfile>,
    system_program: Program<'info, System>,
}

pub fn handle_init_identity_profile(ctx: Context<InitIdentityProfile>, name: String) -> Result<()> {
    let identity_profile = &mut ctx.accounts.identity_profile;
    identity_profile.owner = *ctx.accounts.funder.key;
    identity_profile.name = name;

    Ok(())
}
