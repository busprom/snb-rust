use borsh::BorshSerialize;
use solana_program::{
	pubkey::Pubkey, msg,
	entrypoint::ProgramResult,
	account_info::AccountInfo,
  program_error::ProgramError,
};
use crate::{
  ADMIN_ID,
  types::staking::Admin,
  token::transfer_token::process_transfer_token
};

pub fn process_admin<'a>(
  program_id: &Pubkey,
  owner: &AccountInfo<'a>,
  pool_account: &AccountInfo<'a>,
  snb_token: &AccountInfo<'a>,
  from_token_accaunt: &AccountInfo<'a>,
  to_token_accaunt: &AccountInfo<'a>,
  update_stakung: &AccountInfo<'a>,
  token_program: &AccountInfo<'a>,
  rent_program: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  spl_token_program: &AccountInfo<'a>,
  data: Admin
) -> ProgramResult {
  msg!("Send NFT");
  if !owner.is_signer { return Err(ProgramError::InvalidAccountData); }
  if *owner.key.to_string() != *ADMIN_ID { return Err(ProgramError::InvalidAccountData); }

  let (calc_pool, _) = Pubkey::find_program_address(
    &[owner.key.as_ref(), program_id.as_ref(), snb_token.key.as_ref()], &program_id
  );
  if calc_pool != *pool_account.key { return Err(ProgramError::InvalidAccountData); }

  process_transfer_token(
    owner,
    snb_token,
    pool_account,
    from_token_accaunt,
    to_token_accaunt,
    token_program,
    rent_program,
    system_program,
    spl_token_program,
    data.snb_transfer * 1000000000
  )?;

  if data.update != None {
    let _ = data.update.unwrap().serialize(&mut &mut update_stakung.data.borrow_mut()[..]);
  }

  Ok(())
}