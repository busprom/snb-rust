use solana_program::{
	account_info::AccountInfo,
  program_pack::Pack
};
use spl_token::state::Account;

pub fn process_check_count<'a>(
  mint_account: &AccountInfo<'a>
) -> bool {
  let spl_token_account_data = mint_account.try_borrow_data().unwrap();
  let spl_token_account = Account::unpack(&spl_token_account_data).unwrap();
  if spl_token_account.amount == 1 { return false; }
  else { return true; }
}