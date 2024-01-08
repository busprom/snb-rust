use solana_program::{
  msg, program::{invoke_signed},
  account_info::AccountInfo,
  entrypoint::ProgramResult
};
use crate::{
  token::{
    create_token_account::process_create_token_account,
    check_count::process_check_count
  }
};

pub fn process_transfer_token_seed<'a>(
  payer: &AccountInfo<'a>,
  new_owner: &AccountInfo<'a>,
  mint: &AccountInfo<'a>,
  from_token_accaunt: &AccountInfo<'a>,
  to_token_accaunt: &AccountInfo<'a>,
  nft_raffle_account: &AccountInfo<'a>,
  profit_id: &AccountInfo<'a>,
  token_program: &AccountInfo<'a>,
  rent_program: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  spl_token_program: &AccountInfo<'a>,
  amount: u64,
  signer_seeds: &[&[u8]]
) -> ProgramResult {
  msg!("Transfer token");
  
  if to_token_accaunt.data_is_empty() {
    process_create_token_account(
      payer,
      new_owner,
      mint,
      to_token_accaunt,
      token_program,
      rent_program,
      system_program,
      spl_token_program
    )?;
  }
  
  invoke_signed(
    &spl_token::instruction::transfer(
      token_program.key,
      from_token_accaunt.key,
      to_token_accaunt.key,
      nft_raffle_account.key,
      &[nft_raffle_account.key],
      amount
    ).unwrap(),
    &[token_program.clone(), from_token_accaunt.clone(), to_token_accaunt.clone(), nft_raffle_account.clone()],
    &[signer_seeds],
  )?;

  if process_check_count(from_token_accaunt) == false {
    invoke_signed(// закрываем vault аккаунт
      &spl_token::instruction::close_account(
        token_program.key,
        from_token_accaunt.key,
        profit_id.key,
        nft_raffle_account.key,
        &[nft_raffle_account.key]
      ).unwrap(),
      &[from_token_accaunt.clone(), profit_id.clone(), nft_raffle_account.clone()],
      &[signer_seeds],
    )?;
  }

  Ok(())
}