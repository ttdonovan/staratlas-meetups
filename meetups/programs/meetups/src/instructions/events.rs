use anchor_lang::prelude::*;

use crate::{EventEntry, EventsManager, HostProfile, IdentityProfile, ANCHOR_DISCRIMINATOR_SIZE};

#[derive(Accounts)]
#[instruction(event_manager_id: Pubkey, name: String)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"manager", event_manager_id.as_ref()],
        bump,
    )]
    pub events_manager: Account<'info, EventsManager>,

    #[account(
        seeds = [b"identity", signer.key().as_ref()],
        bump,
        constraint = identity_profile.owner == signer.key(),
    )]
    identity_profile: Account<'info, IdentityProfile>,

    #[account(
        init_if_needed,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR_SIZE + HostProfile::INIT_SPACE,
        seeds = [b"host", events_manager.key().as_ref(), identity_profile.key().as_ref()],
        bump,
    )]
    host_profile: Account<'info, HostProfile>,

    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR_SIZE + EventEntry::INIT_SPACE,
        seeds = [
            b"event",
            events_manager.key().as_ref(),
            host_profile.key().as_ref(),
        ],
        bump
    )]
    event: Account<'info, EventEntry>,

    system_program: Program<'info, System>,
}

pub fn handle_create_event(ctx: Context<CreateEvent>, name: String) -> Result<()> {
    let event = &mut ctx.accounts.event;
    event.host = ctx.accounts.host_profile.key();
    event.name = name;

    Ok(())
}