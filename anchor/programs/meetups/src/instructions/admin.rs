use anchor_lang::prelude::*;

use crate::{
    EventsManager, EventsManagerState, Mints, VaultFeeInfo, VaultOwnerInfo,
    ANCHOR_DISCRIMINATOR_SIZE,
};

#[derive(Accounts)]
pub struct InitEventsManager<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR_SIZE + EventsManager::INIT_SPACE,
        seeds = [b"manager", authority.key().as_ref()],
        bump,
    )]
    pub event_manager: Account<'info, EventsManager>,
    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR_SIZE + EventsManagerState::INIT_SPACE,
        seeds = [b"state", authority.key().as_ref()],
        bump,
    )]
    event_manager_state: Account<'info, EventsManagerState>,
    // todo!(add vaults here...)
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateEventsManagerVaultFees<'info> {
    authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    event_manager_state: Account<'info, EventsManagerState>,
}

#[derive(Accounts)]
pub struct UpdateEventsManagerVaultOwners<'info> {
    authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    event_manager_state: Account<'info, EventsManagerState>,
}

pub fn handle_init_events_manager(
    ctx: Context<InitEventsManager>,
    atlas_mint: Pubkey,
    polis_mint: Pubkey,
    usdc_mint: Pubkey,
) -> Result<()> {
    let events_manager = &mut ctx.accounts.event_manager;

    events_manager.mints = Mints {
        atlas: atlas_mint,
        polis: polis_mint,
        usdc: usdc_mint,
    };

    ctx.accounts
        .event_manager_state
        .set_inner(EventsManagerState {
            authority: *ctx.accounts.authority.key,
            events_manager: events_manager.key(),
            vault_fee_info: VaultFeeInfo {
                dao_vault_fee: 0,
                dev_vault_fee: 0,
                ops_vault_fee: 0,
                host_profile_fee: 0,
            },
            vault_owner_info: VaultOwnerInfo {
                dao_vault_owner: *ctx.accounts.authority.key,
                dev_vault_owner: *ctx.accounts.authority.key,
                ops_vault_owner: *ctx.accounts.authority.key,
            },
            bump: 0,
        });

    Ok(())
}

pub fn handle_update_events_manager_vault_fees(
    ctx: Context<UpdateEventsManagerVaultFees>,
    dao_vault_fee: u8,
    dev_vault_fee: u8,
    ops_vault_fee: u8,
    host_profile_fee: u8,
) -> Result<()> {
    let total_fees = dao_vault_fee + dev_vault_fee + ops_vault_fee + host_profile_fee;
    assert_eq!(total_fees, 100, "Total fees must equal 100");

    let event_manager_state = &mut ctx.accounts.event_manager_state;

    event_manager_state.vault_fee_info = VaultFeeInfo {
        dao_vault_fee,
        dev_vault_fee,
        ops_vault_fee,
        host_profile_fee,
    };

    Ok(())
}

pub fn handle_update_events_manager_vault_owners(
    ctx: Context<UpdateEventsManagerVaultOwners>,
    dao_vault_owner: Pubkey,
    dev_vault_owner: Pubkey,
    ops_vault_owner: Pubkey,
) -> Result<()> {
    let event_manager_state = &mut ctx.accounts.event_manager_state;

    event_manager_state.vault_owner_info = VaultOwnerInfo {
        dao_vault_owner,
        dev_vault_owner,
        ops_vault_owner,
    };

    Ok(())
}
