pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("C14GzDxp9S1UfZk1BR1BPwHK1erFoPWkvjjFRtyf4B7L");

#[program]
pub mod meetups {
    use super::*;

    pub fn init_event_manager(
        ctx: Context<InitEventsManager>,
        atlas_mint: Pubkey,
        polis_mint: Pubkey,
        usdc_mint: Pubkey,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id,);

        admin::handle_init_events_manager(ctx, atlas_mint, polis_mint, usdc_mint)
    }

    pub fn update_events_manager_vault_fees(
        ctx: Context<UpdateEventsManagerVaultFees>,
        dao_vault_fee: u8,
        dev_vault_fee: u8,
        ops_vault_fee: u8,
        host_profile_fee: u8,
    ) -> Result<()> {
        admin::handle_update_events_manager_vault_fees(
            ctx,
            dao_vault_fee,
            dev_vault_fee,
            ops_vault_fee,
            host_profile_fee,
        )
    }

    pub fn update_events_manager_vault_owners(
        ctx: Context<UpdateEventsManagerVaultOwners>,
        dao_vault_owner: Pubkey,
        dev_vault_owner: Pubkey,
        ops_vault_owner: Pubkey,
    ) -> Result<()> {
        admin::handle_update_events_manager_vault_owners(
            ctx,
            dao_vault_owner,
            dev_vault_owner,
            ops_vault_owner,
        )
    }

    pub fn init_identity_profile(ctx: Context<InitIdentityProfile>, name: String) -> Result<()> {
        profile::handle_init_identity_profile(ctx, name)
    }

    pub fn create_event(
        ctx: Context<CreateEvent>,
        event_manager_id: Pubkey,
        year: u16,
        month: u8,
        day: u8,
        name: String,
    ) -> Result<()> {
        events::handle_create_event(ctx, event_manager_id, year, month, day, name)
    }

    pub fn update_event(
        ctx: Context<UpdateEvent>,
        event_manager_id: Pubkey,
        year: u16,
        month: u8,
        day: u8,
        name: String,
        location: String,
        mappable_address: String,
        start_time_at: u64,
        end_time_at: u64,
        entry_token_mint: Pubkey,
        entry_token_amount: u64,
    ) -> Result<()> {
        events::handle_update_event(
            ctx,
            event_manager_id,
            year,
            month,
            day,
            name,
            location,
            mappable_address,
            start_time_at,
            end_time_at,
            entry_token_mint,
            entry_token_amount,
        )
    }

    pub fn open_event(
        ctx: Context<UpdateEventStatus>,
        event_manager_id: Pubkey,
        year: u16,
        month: u8,
        day: u8,
    ) -> Result<()> {
        events::handle_open_event(ctx, event_manager_id, year, month, day)
    }

    pub fn close_event(
        ctx: Context<UpdateEventStatus>,
        event_manager_id: Pubkey,
        year: u16,
        month: u8,
        day: u8,
    ) -> Result<()> {
        events::handle_close_event(ctx, event_manager_id, year, month, day)
    }

    pub fn event_registration(ctx: Context<RegisterEvent>, entry_amount: u64) -> Result<()> {
        registration::handle_event_registration(ctx, entry_amount)
    }
}
