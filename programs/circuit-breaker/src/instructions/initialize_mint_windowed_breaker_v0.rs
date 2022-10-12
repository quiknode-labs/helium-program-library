use anchor_lang::prelude::*;
use anchor_spl::token::{
  set_authority, spl_token::instruction::AuthorityType, Mint, SetAuthority, Token,
};

use crate::{MintWindowedCircuitBreakerV0, WindowV0, WindowedCircuitBreakerConfigV0};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeMintWindowedBreakerArgsV0 {
  authority: Pubkey,
  config: WindowedCircuitBreakerConfigV0,
}

#[derive(Accounts)]
pub struct InitializeMintWindowedBreakerV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    init,
    payer = payer,
    space = 60 + std::mem::size_of::<MintWindowedCircuitBreakerV0>(),
    seeds = ["mint_windowed_breaker".as_bytes(), mint.key().as_ref()],
    bump
  )]
  pub circuit_breaker: Box<Account<'info, MintWindowedCircuitBreakerV0>>,
  #[account(mut)]
  pub mint: Box<Account<'info, Mint>>,
  pub mint_authority: Signer<'info>,

  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
  pub clock: Sysvar<'info, Clock>,
}

pub fn handler(
  ctx: Context<InitializeMintWindowedBreakerV0>,
  args: InitializeMintWindowedBreakerArgsV0,
) -> Result<()> {
  ctx
    .accounts
    .circuit_breaker
    .set_inner(MintWindowedCircuitBreakerV0 {
      mint: ctx.accounts.mint.key(),
      authority: args.authority,
      mint_authority: ctx.accounts.mint.mint_authority.unwrap(),
      config: args.config,
      last_window: WindowV0 {
        last_aggregated_value: 0,
        last_unix_timestamp: ctx.accounts.clock.unix_timestamp,
      },
      bump_seed: ctx.bumps["circuit_breaker"],
    });

  set_authority(
    CpiContext::new(
      ctx.accounts.token_program.to_account_info(),
      SetAuthority {
        account_or_mint: ctx.accounts.mint.to_account_info(),
        current_authority: ctx.accounts.mint_authority.to_account_info(),
      },
    ),
    AuthorityType::MintTokens,
    Some(ctx.accounts.circuit_breaker.key()),
  )?;

  Ok(())
}
