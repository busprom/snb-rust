use nft::{
	instruction::SolInstruction, id, STORAGE,
  types::sell::Sell
};
use crate::utils::tokens::{
	create_account,
	get_token_account,
	create_token
};
use solana_sdk::{
  signature::{Keypair, Signer},
  transaction::Transaction
};
use solana_program::{system_instruction};
use solana_program_test::{
  processor, ProgramTestContext, ProgramTest
};
use nft::entrypoint::process_instruction;
use solana_program_test::{BanksClientError};
use solana_program::pubkey::Pubkey;



pub struct _Env {
	ctx: ProgramTestContext,
	payer: Keypair,
	_buyer: Keypair,
	_buyer1: Keypair,
	_buyer2: Keypair,
	_buyer3: Keypair,
	_buyer4: Keypair,
	_buyer5: Keypair
}

impl _Env {
	async fn _new() -> Self {
		let program_test = ProgramTest::new("nft", id(), processor!(process_instruction));
		let mut ctx = program_test.start_with_context().await;
		let payer = Keypair::new();
		let _buyer = Keypair::new();
		let _buyer1 = Keypair::new();
		let _buyer2 = Keypair::new();
		let _buyer3 = Keypair::new();
		let _buyer4 = Keypair::new();
		let _buyer5 = Keypair::new();
		let arr = [&payer, &_buyer, &_buyer1, &_buyer2, &_buyer3, &_buyer4, &_buyer5];

		for el in arr {
			ctx.banks_client.process_transaction(Transaction::new_signed_with_payer(
				&[
					system_instruction::transfer(
						&ctx.payer.pubkey(),
						&el.pubkey(),
						50_000_000_000,
					),
				],
				Some(&ctx.payer.pubkey()),
				&[&ctx.payer],
				ctx.last_blockhash,
			)).await.unwrap();
		}

		_Env { ctx, payer, _buyer, _buyer1, _buyer2, _buyer3, _buyer4, _buyer5 }
	}
}

pub struct _CreateStorage {
  mint: Keypair,
	mint_1: Keypair,
  token_account: Keypair,
  vault: Pubkey,
  storage: Pubkey,
  buyer_account: Pubkey,
  profit_id: Pubkey,
  context: _Env
}

impl _CreateStorage {
  async fn _new(auction: u8) -> Result<Self, BanksClientError> {
		let mint = Keypair::new();

		let token_account = Keypair::new();

		let (vault, _) = Pubkey::find_program_address(
			&[STORAGE.as_bytes(), id().as_ref(), mint.pubkey().as_ref()],
			&id(),
		);

		let (storage, _) = Pubkey::find_program_address(
			&[STORAGE.as_bytes(), id().as_ref(), vault.as_ref()],
			&id(),
		);

		let mut context = _Env::_new().await;

		create_token(
			&mut context.ctx,
			&mint,
			&context.payer,
			&token_account
		).await?;

		let mint_1 = Keypair::new();
		let token_account_1 = Keypair::new();

		create_token(
			&mut context.ctx,
			&mint_1,
			&context._buyer,
			&token_account_1
		).await?;

		context.ctx.banks_client.process_transaction(
			Transaction::new_signed_with_payer(
				&[SolInstruction::create_storage(
					context.payer.pubkey(),
					token_account.pubkey(),
					mint.pubkey(),
					vault,
					storage,
					Sell {
						price: 2_000_000_000,
						start_price: 2_000_000_000,
						bets: 0,
						auction,
						seller: context.payer.pubkey(),
						vault,
						applicant: context.payer.pubkey()
					}
				)],
				Some(&context.payer.pubkey()),
				&[&context.payer],
				context.ctx.last_blockhash,
			)
		).await?;

		let buyer_account = Keypair::new();
		get_token_account(
			&mut context.ctx,
			&mint,
			&context._buyer,
			&buyer_account
		).await?;

		let profit_id = Keypair::new();
		create_account(
			&mut context.ctx,
			&context._buyer,
			&profit_id
		).await?;
		
		Ok(_CreateStorage {
			mint: mint,
			token_account,
			vault,
			storage,
			buyer_account: buyer_account.pubkey(),
			profit_id: profit_id.pubkey(),
			context,
			mint_1
		})
	}
}



// mod metadata_test;
// mod create_sell_test;
// mod create_lottery_test;
// mod create_nft_lottery_test;
// mod create_tokens_test;
// mod arr_val;

// mod nft_lottery_test;
// mod random;