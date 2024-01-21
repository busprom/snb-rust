use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
	pubkey::Pubkey, msg,
	entrypoint::ProgramResult,
	account_info::AccountInfo,
  sysvar::{Sysvar, clock::Clock},
};
use crate::{
  ADMIN_ID,
  types::staking::Staking,
  error::NftError,
  token::transfer_token_seed::process_transfer_token_seed
};

pub fn process_claim<'a>(
  program_id: &Pubkey,
  owner: &AccountInfo<'a>,
  stake_account: &AccountInfo<'a>,
  snb_token: &AccountInfo<'a>,
  pool_account: &AccountInfo<'a>,
  from_token_accaunt: &AccountInfo<'a>,
  to_token_accaunt: &AccountInfo<'a>,
  token_program: &AccountInfo<'a>,
  rent_program: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  spl_token_program: &AccountInfo<'a>
) -> ProgramResult {
  msg!("Send NFT");
  if !owner.is_signer { return Err(NftError::WrongOwnerNFR.into()); }

  let mut stake = Staking::try_from_slice(&stake_account.data.borrow())?;
  if stake.owner != *owner.key { return Err(NftError::WrongOwnerNFR.into()); }

  let cl = Clock::get().unwrap();
  let current = cl.unix_timestamp as u64;
  let days = (((stake.start - current) / 86400) as f32).floor() as u64;
  if days < 1 { return Err(NftError::WrongOwnerNFR.into()); }

  let (calc_pool, pool_seed) = Pubkey::find_program_address(
    &[ADMIN_ID.as_ref(), program_id.as_ref(), snb_token.key.as_ref()], &program_id
  );
  if calc_pool != *pool_account.key { return Err(NftError::WrongSettingsPDA.into()); }
  let pool_signer_seeds = &[ADMIN_ID.as_ref(), program_id.as_ref(), snb_token.key.as_ref(), &[pool_seed]];

  process_transfer_token_seed(
    owner,
    owner,
    snb_token,
    from_token_accaunt,
    to_token_accaunt,
    pool_account,
    owner,
    token_program,
    rent_program,
    system_program,
    spl_token_program,
    days * 115 * 1000000000,
    pool_signer_seeds
  )?;

  stake.start = stake.start - (days * 86400);
  let _ = stake.serialize(&mut &mut stake_account.data.borrow_mut()[..]);
  
  Ok(())
}