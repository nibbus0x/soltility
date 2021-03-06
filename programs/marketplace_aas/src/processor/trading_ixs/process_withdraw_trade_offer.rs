use {
  anchor_lang::prelude::*,
  crate::context::trading_context::WithdrawTradeOffer,
  crate::error::*,
  crate::utils::*,
  spl_token::{
    instruction::{
      close_account,
    },
    ID as TOKEN_PROGRAM_ID,
  },
  solana_program::{
    program::{invoke_signed},
  },
  anchor_spl::token::TokenAccount,
};

pub fn process<'a, 'b, 'c, 'info>(
  ctx: Context<'a, 'b, 'c, 'info, WithdrawTradeOffer<'info>>,
) -> Result<()> {
  let escrow_account = &mut ctx.accounts.escrow_account;
  let offerer = &mut ctx.accounts.offerer;
  let escrow_bump = *ctx.bumps.get("escrow_account").ok_or(CustomError::MissingBump)?;

  if *offerer.key != escrow_account.offerer {
    return Err(error!(CustomError::InvalidOfferer));
  }

  if ctx.remaining_accounts.len() != escrow_account.tokens_offering.len() * 3 {
    return Err(error!(CustomError::InvalidRemainingAccounts));
  }

  let mut i = 0;
  for offering in escrow_account.tokens_offering.clone() {
    let mint_info = ctx.remaining_accounts.get(i).ok_or(CustomError::InvalidRemainingAccounts)?;
    let offerer_token_account_info = ctx.remaining_accounts.get(i+1).ok_or(CustomError::InvalidRemainingAccounts)?;
    let escrow_token_account_info = ctx.remaining_accounts.get(i+2).ok_or(CustomError::InvalidRemainingAccounts)?;

    if *mint_info.key != offering.mint.ok_or(CustomError::MissingOfferingMint)? {
      return Err(error!(CustomError::InvalidRemainingAccounts));
    }

    let offerer_token_account_data = TokenAccount::try_deserialize(&mut &offerer_token_account_info.try_borrow_data()?[..])?;

    if offerer_token_account_data.owner != *offerer.key || offerer_token_account_data.mint != *mint_info.key {
      return Err(error!(CustomError::InvalidRemainingAccounts));
    }

    let (escrow_token_account, _bump) = Pubkey::find_program_address(
      &[
        b"token-account".as_ref(),
        escrow_account.key().as_ref(),
        mint_info.key.as_ref(),
      ],
      ctx.program_id,
    );

    if *escrow_token_account_info.key != escrow_token_account {
      return Err(error!(CustomError::InvalidRemainingAccounts));
    }

    let transfer_ix = spl_token::instruction::transfer(
      &TOKEN_PROGRAM_ID,
      &escrow_token_account,
      offerer_token_account_info.key,
      &escrow_account.key(),
      &[],
      offering.amount // Must be 1 for nfts. For tokens can be anything > 0
    )?;

    invoke_signed(
      &transfer_ix,
      &[
        offerer_token_account_info.clone(),
        escrow_token_account_info.clone(),
        escrow_account.to_account_info(),
      ],
      &[
        &[
          b"trade-escrow".as_ref(),
          ctx.accounts.global_state.key().as_ref(),
          offerer.key.as_ref(),
          &escrow_account.nonce.to_le_bytes(),
          &[escrow_bump],
        ]
      ]
    )?;

    let close_ix = close_account(
      &TOKEN_PROGRAM_ID,
      &escrow_token_account,
      offerer.key,
      &escrow_account.key(),
      &[]
    )?;

    invoke_signed(
      &close_ix,
      &[
        escrow_token_account_info.clone(),
        offerer.to_account_info(),
        escrow_account.to_account_info(),
      ],
      &[
        &[
          b"trade-escrow".as_ref(),
          ctx.accounts.global_state.key().as_ref(),
          offerer.key.as_ref(),
          &escrow_account.nonce.to_le_bytes(),
          &[escrow_bump],
        ]
      ]
    )?;

    i += 3;
  }

  match escrow_account.lamports_offering {
    Some(amount) => {
      transfer_from_program_owned_account(
        &mut escrow_account.to_account_info(),
        &mut offerer.to_account_info(),
        amount,
      )?;
    },
    None => {},
  };

  ctx.accounts.global_state.pending_offers -= 1;

  Ok(())
}