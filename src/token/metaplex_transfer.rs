use borsh::BorshSerialize;
use solana_program::{
  account_info::AccountInfo,
  instruction::{AccountMeta, Instruction},
  entrypoint::ProgramResult, 
  program::{invoke, invoke_signed}
};
use crate::{
  types::metadata::{TransferToken, Args}
};

pub fn process_metaplex_transfer<'a>(
  fee_payer: &AccountInfo<'a>,
  from_token_accaunt: &AccountInfo<'a>,
  owner: &AccountInfo<'a>,
  to_token_accaunt: &AccountInfo<'a>,
  new_owner: &AccountInfo<'a>,
  mint: &AccountInfo<'a>,
  metadata_account: &AccountInfo<'a>,
  edition_account: &AccountInfo<'a>,
  from_record_accaunt: &AccountInfo<'a>,
  to_record_accaunt: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  rent_program: &AccountInfo<'a>,
  token_program: &AccountInfo<'a>,
	spl_token_program: &AccountInfo<'a>,
  token_auth_rules_programm: &AccountInfo<'a>,
  token_auth_rules_acc: &AccountInfo<'a>,
  metadata_program: &AccountInfo<'a>,
  seed: bool,
  signer_seeds: &[&[u8]]
) -> ProgramResult {

  let transfer = TransferToken { instruction: 49, args: Args::V1 { amount: 1, authorization_data: None} };

  let accounts = vec![
    AccountMeta::new(*from_token_accaunt.key, false),
    AccountMeta::new(*owner.key, true),
    AccountMeta::new(*to_token_accaunt.key, false),
    AccountMeta::new(*new_owner.key, false),
    AccountMeta::new(*mint.key, false),
    AccountMeta::new(*metadata_account.key, false),
    AccountMeta::new(*edition_account.key, false),
    AccountMeta::new(*from_record_accaunt.key, false),
    AccountMeta::new(*to_record_accaunt.key, false),
    AccountMeta::new(*owner.key, true),
    AccountMeta::new(*fee_payer.key, true),
    AccountMeta::new_readonly(*system_program.key, false),
    AccountMeta::new_readonly(*rent_program.key, false),
    AccountMeta::new_readonly(*token_program.key, false),
    AccountMeta::new_readonly(*spl_token_program.key, false),
    AccountMeta::new_readonly(*token_auth_rules_programm.key, false),
    AccountMeta::new_readonly(*token_auth_rules_acc.key, false)
  ];
  let sign = [
    from_token_accaunt.clone(),
    owner.clone(),
    to_token_accaunt.clone(),
    new_owner.clone(),
    mint.clone(),
    metadata_account.clone(),
    edition_account.clone(),
    from_record_accaunt.clone(),
    to_record_accaunt.clone(),
    owner.clone(),
    fee_payer.clone(),
    system_program.clone(),
    rent_program.clone(),
    token_program.clone(),
    spl_token_program.clone(),
    token_auth_rules_programm.clone(),
    token_auth_rules_acc.clone()
  ];

  if seed == true {
    invoke_signed(
      &Instruction {
        program_id: *metadata_program.key,
        data: TransferToken::try_to_vec(&transfer).unwrap(),
        accounts
      },
      &sign,
      &[signer_seeds],
    )?;
  }
  else {
    invoke(
      &Instruction {
        program_id: *metadata_program.key,
        data: TransferToken::try_to_vec(&transfer).unwrap(),
        accounts
      },
      &sign
    )?;
  }

  Ok(())
}