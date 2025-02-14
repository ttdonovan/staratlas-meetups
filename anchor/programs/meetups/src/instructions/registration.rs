use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{shared, AttendeeProfile, EventEntry, ANCHOR_DISCRIMINATOR_SIZE};

#[derive(Accounts)]
pub struct RegisterEvent<'info> {
    #[account(mut)]
    funder: Signer<'info>,

    #[account()]
    event: Account<'info, EventEntry>,

    #[account(
        init,
        payer = funder,
        space = ANCHOR_DISCRIMINATOR_SIZE + AttendeeProfile::INIT_SPACE,
        seeds = [b"attendee", event.key().as_ref(), funder.key().as_ref()],
        bump,
    )]
    attendee: Account<'info, AttendeeProfile>,

    #[account(mint::token_program = token_program)]
    attendee_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = attendee_token_mint,
        associated_token::authority = funder,
        associated_token::token_program = token_program
    )]
    attendee_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = funder,
        associated_token::mint = attendee_token_mint,
        associated_token::authority = attendee, // FIXME: attendee or event?
        associated_token::token_program = token_program,
    )]
    vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handle_event_registration(ctx: Context<RegisterEvent>, entry_amount: u64) -> Result<()> {
    // FIXME: add checks for the required amount...
    shared::transfer_tokens(
        &ctx.accounts.attendee_token_account,
        &ctx.accounts.vault,
        &entry_amount,
        &ctx.accounts.attendee_token_mint,
        &ctx.accounts.funder,
        &ctx.accounts.token_program,
    )?;

    let attendee = &ctx.accounts.attendee;
    msg!("Attendee({:?}) - event registration.", attendee.key());

    Ok(())
}
