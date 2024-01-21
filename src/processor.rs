use borsh::BorshDeserialize;
use solana_program::{
	pubkey::Pubkey,
	entrypoint::ProgramResult,
	account_info::{next_account_info, AccountInfo}
};
use crate::{
	utils::{
		add_staking::process_add_staking,
		unstake::process_unstake,
		claim::process_claim
	},
	token::create_token_account::process_create_token_account,
	instruction::SolInstruction
};
pub struct Processor;

impl Processor {
	pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
		let instruction = SolInstruction::try_from_slice(input)?;
		match instruction {
			SolInstruction::AddStaking {data} => {
				let account_info_iter = &mut accounts.iter();
				let owner = next_account_info(account_info_iter)?;
				let stake_account = next_account_info(account_info_iter)?;
				let mint = next_account_info(account_info_iter)?;
				let from_token_accaunt = next_account_info(account_info_iter)?;
				let to_token_accaunt = next_account_info(account_info_iter)?;
				let token_program = next_account_info(account_info_iter)?;
				let rent_program = next_account_info(account_info_iter)?;
				let spl_token_program = next_account_info(account_info_iter)?;
				let system_program = next_account_info(account_info_iter)?;

				let record_rent_program = next_account_info(account_info_iter)?;
				let metadata_program = next_account_info(account_info_iter)?;
				let metadata_account = next_account_info(account_info_iter)?;
				let edition_account = next_account_info(account_info_iter)?;
				let from_record_accaunt = next_account_info(account_info_iter)?;
				let to_record_accaunt = next_account_info(account_info_iter)?;
				let token_auth_rules_programm = next_account_info(account_info_iter)?;
				let token_auth_rules_acc = next_account_info(account_info_iter)?;
				process_add_staking(
					program_id,
					owner,
					stake_account,
					mint,
					from_token_accaunt,
					to_token_accaunt,
					token_program,
					rent_program,
					spl_token_program,
					system_program,
					record_rent_program,
					metadata_program,
					metadata_account,
					edition_account,
					from_record_accaunt,
					to_record_accaunt,
					token_auth_rules_programm,
					token_auth_rules_acc,
					data
				)
			},
			SolInstruction::Unstake => {
				let account_info_iter = &mut accounts.iter();
				let owner = next_account_info(account_info_iter)?;
				let stake_account = next_account_info(account_info_iter)?;
				let mint = next_account_info(account_info_iter)?;
				let from_token_accaunt = next_account_info(account_info_iter)?;
				let to_token_accaunt = next_account_info(account_info_iter)?;
				let token_program = next_account_info(account_info_iter)?;
				let rent_program = next_account_info(account_info_iter)?;
				let spl_token_program = next_account_info(account_info_iter)?;
				let system_program = next_account_info(account_info_iter)?;

				let metadata_program = next_account_info(account_info_iter)?;
				let metadata_account = next_account_info(account_info_iter)?;
				let edition_account = next_account_info(account_info_iter)?;
				let from_record_accaunt = next_account_info(account_info_iter)?;
				let to_record_accaunt = next_account_info(account_info_iter)?;
				let token_auth_rules_programm = next_account_info(account_info_iter)?;
				let token_auth_rules_acc = next_account_info(account_info_iter)?;

				process_unstake(
					program_id,
					owner,
					stake_account,
					mint,
					from_token_accaunt,
					to_token_accaunt,
					token_program,
					rent_program,
					spl_token_program,
					system_program,
					metadata_program,
					metadata_account,
					edition_account,
					from_record_accaunt,
					to_record_accaunt,
					token_auth_rules_programm,
					token_auth_rules_acc
				)
			},
			SolInstruction::Claim => {
				let account_info_iter = &mut accounts.iter();
				let owner = next_account_info(account_info_iter)?;
				let stake_account = next_account_info(account_info_iter)?;
				let snb_token = next_account_info(account_info_iter)?;
				let pool_account = next_account_info(account_info_iter)?;
				let from_token_accaunt = next_account_info(account_info_iter)?;
				let to_token_accaunt = next_account_info(account_info_iter)?;
				let token_program = next_account_info(account_info_iter)?;
				let rent_program = next_account_info(account_info_iter)?;
				let system_program = next_account_info(account_info_iter)?;
				let spl_token_program = next_account_info(account_info_iter)?;

				process_claim(
					program_id,
					owner,
					stake_account,
					snb_token,
					pool_account,
					from_token_accaunt,
					to_token_accaunt,
					token_program,
					rent_program,
					system_program,
					spl_token_program
				)
			},
			SolInstruction::CreateTokenAcc => {
				let account_info_iter = &mut accounts.iter();
				let payer = next_account_info(account_info_iter)?;
				let new_owner = next_account_info(account_info_iter)?;
				let mint = next_account_info(account_info_iter)?;
				let mint_account = next_account_info(account_info_iter)?;
				let token_program = next_account_info(account_info_iter)?;
				let rent_program = next_account_info(account_info_iter)?;
				let system_program = next_account_info(account_info_iter)?;
				let spl_token_program = next_account_info(account_info_iter)?;
				process_create_token_account(
					payer,
					new_owner,
					mint,
					mint_account,
					token_program,
					rent_program,
					system_program,
					spl_token_program
				)
			},
		}
	}
}