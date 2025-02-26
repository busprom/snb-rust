use borsh::from_slice;
use solana_program::{
	pubkey::Pubkey, msg,
	entrypoint::ProgramResult,
	account_info::AccountInfo,
  program_error::ProgramError,
};
use crate::{
  types::staking::Staking,
  token::metaplex_transfer::process_metaplex_transfer,
};

pub fn process_unstake<'a>(
  program_id: &Pubkey,
  owner: &AccountInfo<'a>,
  stake_account: &AccountInfo<'a>,
  mint: &AccountInfo<'a>,
  from_token_accaunt: &AccountInfo<'a>,
  to_token_accaunt: &AccountInfo<'a>,
  token_program: &AccountInfo<'a>,
  rent_program: &AccountInfo<'a>,
  spl_token_program: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  metadata_program: &AccountInfo<'a>,
  metadata_account: &AccountInfo<'a>,
  edition_account: &AccountInfo<'a>,
  from_record_accaunt: &AccountInfo<'a>,
  to_record_accaunt: &AccountInfo<'a>,
  token_auth_rules_programm: &AccountInfo<'a>,
  token_auth_rules_acc: &AccountInfo<'a>
) -> ProgramResult {
  msg!("SNB Unstake");
  if !owner.is_signer { return Err(ProgramError::InvalidAccountData); }

  let stake = from_slice::<Staking>(&stake_account.data.borrow())?;
  if stake.owner != *owner.key { return Err(ProgramError::InvalidAccountData); }

  let (calc_stake, raffle_seed) = Pubkey::find_program_address(
    &[owner.key.as_ref(), program_id.as_ref(), mint.key.as_ref()], &program_id
  );
  if calc_stake != *stake_account.key { return Err(ProgramError::InvalidAccountData); }
  let stake_signer_seeds = &[owner.key.as_ref(), program_id.as_ref(), mint.key.as_ref(), &[raffle_seed]];

  process_metaplex_transfer(
    &owner,
    &from_token_accaunt,
    &stake_account ,
    &to_token_accaunt,
    &owner,
    &mint,
    &metadata_account,
    &edition_account,
    &from_record_accaunt,
    &to_record_accaunt,
    &system_program,
    &rent_program,
    &token_program,
    &spl_token_program,
    &token_auth_rules_programm,
    &token_auth_rules_acc,
    &metadata_program,
    true,
    stake_signer_seeds
  )?;

  let dest_starting_lamports = owner.lamports();
    **owner.lamports.borrow_mut() = dest_starting_lamports.checked_add(stake_account.lamports()).unwrap();
    **stake_account.lamports.borrow_mut() = 0;

  Ok(())
}