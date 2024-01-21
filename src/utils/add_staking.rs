use borsh::BorshSerialize;
use solana_program::{
	pubkey::Pubkey, msg,
	entrypoint::ProgramResult,
	account_info::AccountInfo,
  program::invoke_signed,
  sysvar::{Sysvar, rent::Rent, clock::Clock},
  system_instruction,
};
use crate::{
  types::{
    staking::Staking,
    metadata::MetadataMetaplex
  },
  error::NftError,
  token::metaplex_transfer::process_metaplex_transfer,
  FOUNDER_ID
};

pub fn process_add_staking<'a>(
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
  record_rent_program: &AccountInfo<'a>,
  metadata_program: &AccountInfo<'a>,
  metadata_account: &AccountInfo<'a>,
  edition_account: &AccountInfo<'a>,
  from_record_accaunt: &AccountInfo<'a>,
  to_record_accaunt: &AccountInfo<'a>,
  token_auth_rules_programm: &AccountInfo<'a>,
  token_auth_rules_acc: &AccountInfo<'a>,
  mut data: Staking
) -> ProgramResult {
  msg!("Add SNB");
  if !owner.is_signer { return Err(NftError::WrongOwnerNFR.into()); }

  let meta = MetadataMetaplex::from_account_info(&metadata_account)?;
  if meta.data.creators.unwrap()[1].address.to_string() != FOUNDER_ID { return Err(NftError::AdminRequired.into()); }
  if meta.mint != *mint.key { return Err(NftError::AdminRequired.into()); }

  let (calc_stake, raffle_seed) = Pubkey::find_program_address(
    &[owner.key.as_ref(), program_id.as_ref(), mint.key.as_ref()], &program_id
  );
  if calc_stake != *stake_account.key { return Err(NftError::WrongSettingsPDA.into()); }
  let stake_signer_seeds = &[owner.key.as_ref(), program_id.as_ref(), mint.key.as_ref(), &[raffle_seed]];

  process_metaplex_transfer(
    &owner,
    &from_token_accaunt,
    &owner,
    &to_token_accaunt,
    &stake_account,
    &mint,
    &metadata_account,
    &edition_account,
    &from_record_accaunt,
    &to_record_accaunt,
    &system_program,
    &record_rent_program,
    &token_program,
    &spl_token_program,
    &token_auth_rules_programm,
    &token_auth_rules_acc,
    &metadata_program,
    false,
    stake_signer_seeds
  )?;

  let cl = Clock::get().unwrap();
  data.start = cl.unix_timestamp as u64;
  data.owner = *owner.key;
  data.mint = *mint.key;
  if data.end < 30 {
    data.end = 30;
  }
  data.end = data.start + (data.end * 86400);

  msg!("Create Staking account");
  let rent = &Rent::from_account_info(&rent_program)?;
  let space = data.try_to_vec()?.len();
  let lamports = rent.minimum_balance(space);
  invoke_signed(
    &system_instruction::create_account(
      owner.key,
      stake_account.key,
      lamports,
      space as u64,
      &program_id
    ),
    &[owner.clone(), stake_account.clone(), system_program.clone()],
    &[stake_signer_seeds],
  )?;

  let _ = data.serialize(&mut &mut stake_account.data.borrow_mut()[..]);

  Ok(())
}