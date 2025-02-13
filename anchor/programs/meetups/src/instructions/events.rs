use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode, EventEntry, EventStatusType, HostProfile, IdentityProfile,
    ANCHOR_DISCRIMINATOR_SIZE,
};

// Steps:
// 1. Create an EventEntry (with YYYY MM DD)
// 2. Update an EventEntry
// 3. Open (for registration) an EventEntry
// 4. ...

// ---
// 1. Create an EventEntry (with YYYY MM DD)
// ---

#[derive(Accounts)]
#[instruction(event_manager_id: Pubkey, year: u16, month: u8, day: u8)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    funder: Signer<'info>,

    #[account(
        seeds = [b"identity", funder.key().as_ref()],
        bump,
        constraint = identity_profile.owner == funder.key(), // do we need this?
    )]
    identity_profile: Account<'info, IdentityProfile>,

    #[account(
        init_if_needed,
        payer = funder,
        space = ANCHOR_DISCRIMINATOR_SIZE + HostProfile::INIT_SPACE,
        seeds = [b"host", event_manager_id.as_ref(), identity_profile.key().as_ref()],
        bump,
    )]
    host_profile: Account<'info, HostProfile>,

    #[account(
        init,
        payer = funder,
        space = ANCHOR_DISCRIMINATOR_SIZE + EventEntry::INIT_SPACE,
        seeds = [
            b"event",
            event_manager_id.as_ref(),
            host_profile.key().as_ref(),
            year.to_le_bytes().as_ref(),
            month.to_le_bytes().as_ref(),
            day.to_le_bytes().as_ref(),
        ],
        bump
    )]
    event: Account<'info, EventEntry>,

    system_program: Program<'info, System>,
}

pub fn handle_create_event(
    ctx: Context<CreateEvent>,
    _event_manager_id: Pubkey,
    year: u16,
    month: u8,
    day: u8,
    name: String,
) -> Result<()> {
    let event = &mut ctx.accounts.event;
    event.status = EventStatusType::Pending;
    event.host = ctx.accounts.host_profile.key();
    event.name = name;
    event.year = year;
    event.month = month;
    event.day = day;

    Ok(())
}

// ---
// 2. Update an EventEntry
// ---

#[derive(Accounts)]
#[instruction(event_manager_id: Pubkey, year: u16, month: u8, day: u8)]
pub struct UpdateEvent<'info> {
    signer: Signer<'info>,

    #[account(
        seeds = [b"identity", signer.key().as_ref()],
        bump,
        constraint = identity_profile.owner == signer.key(), // do we need this?
    )]
    identity_profile: Account<'info, IdentityProfile>,

    #[account(
        seeds = [b"host", event_manager_id.as_ref(), identity_profile.key().as_ref()],
        bump,
        constraint = host_profile.key() == event.host, // do we need this?
    )]
    host_profile: Account<'info, HostProfile>,

    #[account(
        mut,
        seeds = [
            b"event",
            event_manager_id.as_ref(),
            host_profile.key().as_ref(),
            year.to_le_bytes().as_ref(),
            month.to_le_bytes().as_ref(),
            day.to_le_bytes().as_ref(),
        ],
        bump,
        constraint = event.status == EventStatusType::Pending,
    )]
    event: Account<'info, EventEntry>,
}

pub fn handle_update_event(
    ctx: Context<UpdateEvent>,
    _event_manager_id: Pubkey,
    _year: u16,
    _month: u8,
    _day: u8,
    name: String,
    location: String,
    mappable_address: String,
    start_time_at: u64,
    end_time_at: u64,
) -> Result<()> {
    let event = &mut ctx.accounts.event;
    require!(
        event.status == EventStatusType::Pending,
        ErrorCode::EventNotPending
    );

    event.name = name;
    event.location = location;
    event.mappable_address = mappable_address;
    event.start_time_at = start_time_at;
    event.end_time_at = end_time_at;

    Ok(())
}

// ---
// 3. Open (for registration) an EventEntry
// ---

#[derive(Accounts)]
#[instruction(event_manager_id: Pubkey, year: u16, month: u8, day: u8)]
pub struct UpdateEventStatus<'info> {
    signer: Signer<'info>,

    #[account(
        seeds = [b"identity", signer.key().as_ref()],
        bump,
        constraint = identity_profile.owner == signer.key(), // do we need this?
    )]
    identity_profile: Account<'info, IdentityProfile>,

    #[account(
        seeds = [b"host", event_manager_id.as_ref(), identity_profile.key().as_ref()],
        bump,
        constraint = host_profile.key() == event.host, // do we need this?
    )]
    host_profile: Account<'info, HostProfile>,

    #[account(
        mut,
        seeds = [
            b"event",
            event_manager_id.as_ref(),
            host_profile.key().as_ref(),
            year.to_le_bytes().as_ref(),
            month.to_le_bytes().as_ref(),
            day.to_le_bytes().as_ref(),
        ],
        bump,
        constraint = event.status == EventStatusType::Pending || event.status == EventStatusType::Open,
    )]
    event: Account<'info, EventEntry>,
}

pub fn handle_open_event(
    ctx: Context<UpdateEventStatus>,
    _event_manager_id: Pubkey,
    _year: u16,
    _month: u8,
    _day: u8,
) -> Result<()> {
    let event = &mut ctx.accounts.event;
    require!(
        event.status == EventStatusType::Pending,
        ErrorCode::EventNotPending
    );

    event.status = EventStatusType::Open;

    Ok(())
}

pub fn handle_close_event(
    ctx: Context<UpdateEventStatus>,
    _event_manager_id: Pubkey,
    _year: u16,
    _month: u8,
    _day: u8,
) -> Result<()> {
    let event = &mut ctx.accounts.event;
    require!(
        event.status == EventStatusType::Open,
        ErrorCode::EventNotOpen
    );

    event.status = EventStatusType::Closed;

    Ok(())
}
