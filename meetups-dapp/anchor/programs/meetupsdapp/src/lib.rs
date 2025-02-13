#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod meetupsdapp {
    use super::*;

  pub fn close(_ctx: Context<CloseMeetupsdapp>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.meetupsdapp.count = ctx.accounts.meetupsdapp.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.meetupsdapp.count = ctx.accounts.meetupsdapp.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeMeetupsdapp>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.meetupsdapp.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeMeetupsdapp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Meetupsdapp::INIT_SPACE,
  payer = payer
  )]
  pub meetupsdapp: Account<'info, Meetupsdapp>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseMeetupsdapp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub meetupsdapp: Account<'info, Meetupsdapp>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub meetupsdapp: Account<'info, Meetupsdapp>,
}

#[account]
#[derive(InitSpace)]
pub struct Meetupsdapp {
  count: u8,
}
