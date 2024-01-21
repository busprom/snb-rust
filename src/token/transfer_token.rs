use solana_program::{
  program::invoke,
  account_info::AccountInfo,
  entrypoint::ProgramResult
};
use crate::token::create_token_account::process_create_token_account;


pub fn process_transfer_token<'a>(
  payer: &AccountInfo<'a>,
  mint: &AccountInfo<'a>,
  new_owner: &AccountInfo<'a>,
  token_account: &AccountInfo<'a>,
  vault: &AccountInfo<'a>,
  token_program: &AccountInfo<'a>,
  rent_info: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  spl_token_program: &AccountInfo<'a>,
  
  amount: u64
) -> ProgramResult {

  if vault.data_is_empty() {
    process_create_token_account(
      payer,
      new_owner,
      mint,
      vault,
      token_program,
      rent_info,
      system_program,
      spl_token_program
    )?;
  }

  invoke(
    &spl_token::instruction::transfer(
      token_program.key,
      token_account.key,
      vault.key,
      payer.key,
      &[payer.key],
      amount,
    ).unwrap(), 
    &[token_account.clone(), vault.clone(), payer.clone()]
  )?;
  
  Ok(())
}