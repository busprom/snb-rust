// use borsh::de::BorshDeserialize;
use solana_sdk::{
	signature::{Keypair, Signer},
	transaction::Transaction
};
use solana_program_test::{BanksClientError, tokio};
use crate::{
	one::_Env,
	utils::init_metadata::init_metadata
};
use nft::{
	METADATA_PROGRAM_ID,
	instruction::SolInstruction
};
use spl_token::state::Account;
use solana_sdk::program_pack::Pack;
use solana_program::pubkey::Pubkey;


#[tokio::test]
async fn metadata_test() -> Result<(), BanksClientError> {
	let mut ctx = _Env::_new().await;

	let mint = Keypair::new();
	let mint_acc = Keypair::new();

	ctx.ctx.banks_client.process_transaction(
		Transaction::new_signed_with_payer(
			&[SolInstruction::create_one_token(
				ctx.payer.pubkey(),
				mint.pubkey(),
				mint_acc.pubkey(),
				mint_acc.pubkey(),
				mint_acc.pubkey(),
				mint_acc.pubkey(),
				init_metadata()
			)],
			Some(&ctx.payer.pubkey()),
			&vec![&ctx.payer, &mint, &mint_acc],
			ctx.ctx.last_blockhash,
		)
	).await?;

	// ctx.ctx.banks_client.process_transaction(
	// 	Transaction::new_signed_with_payer(
	// 		&[SolInstruction::get_mint(
	// 			ctx.payer.pubkey(),
	// 			mint_1.pubkey(),
	// 			acc_1.pubkey(),
	// 			init_metadata()
	// 		)],
	// 		Some(&ctx.payer.pubkey()),
	// 		&vec![&ctx.payer, &mint_1, &acc_1, &mint_2, &acc_2, 
	// 			&mint_3, &acc_3, &mint_4, &acc_4, &mint_5, &acc_5, 
	// 			&mint_6, &acc_6, &mint_7, &acc_7, &mint_8, &acc_8,
	// 			&mint_9, &acc_9, &mint_10, &acc_10
	// 		],
	// 		ctx.ctx.last_blockhash,
	// 	)
	// ).await?;

	// let mut ctx = _Env::_new().await;

	// let mint_1= Keypair::new(); let acc_1= Keypair::new();
	// let mint_2= Keypair::new(); let acc_2= Keypair::new();
	// let mint_3= Keypair::new(); let acc_3= Keypair::new();
	// let mint_4= Keypair::new(); let acc_4= Keypair::new();
	// let mint_5= Keypair::new(); let acc_5= Keypair::new();
	// let mint_6= Keypair::new(); let acc_6= Keypair::new();
	// let mint_7= Keypair::new(); let acc_7= Keypair::new();
	// let mint_8= Keypair::new(); let acc_8= Keypair::new();
	// let mint_9= Keypair::new(); let acc_9= Keypair::new();
	// let mint_10= Keypair::new(); let acc_10= Keypair::new();

	// ctx.ctx.banks_client.process_transaction(
	// 	Transaction::new_signed_with_payer(
	// 		&[SolInstruction::get_mint(
	// 			ctx.payer.pubkey(),
	// 			mint_1.pubkey(), acc_1.pubkey(),
	// 			mint_2.pubkey(), acc_2.pubkey(),
	// 			mint_3.pubkey(), acc_3.pubkey(),
	// 			mint_4.pubkey(), acc_4.pubkey(),
	// 			mint_5.pubkey(), acc_5.pubkey(),
	// 			mint_6.pubkey(), acc_6.pubkey(),
	// 			mint_7.pubkey(), acc_7.pubkey(),
	// 			mint_8.pubkey(), acc_8.pubkey(),
	// 			mint_9.pubkey(), acc_9.pubkey(),
	// 			mint_10.pubkey(), acc_10.pubkey(),
	// 			init_metadata()
	// 		)],
	// 		Some(&ctx.payer.pubkey()),
	// 		&vec![&ctx.payer, &mint_1, &acc_1, &mint_2, &acc_2, 
	// 			&mint_3, &acc_3, &mint_4, &acc_4, &mint_5, &acc_5, 
	// 			&mint_6, &acc_6, &mint_7, &acc_7, &mint_8, &acc_8,
	// 			&mint_9, &acc_9, &mint_10, &acc_10
	// 		],
	// 		ctx.ctx.last_blockhash,
	// 	)
	// ).await?;

	// let buyer_token_account = ctx.ctx.banks_client.get_account(acc_5.pubkey()).await.unwrap().unwrap();
  // let spl_token_account_data = Account::unpack(&buyer_token_account.data).unwrap();
	// assert_eq!(spl_token_account_data.owner, ctx.payer.pubkey());

	// let buyer_token_account = ctx.ctx.banks_client.get_account(acc_10.pubkey()).await.unwrap().unwrap();
  // let spl_token_account_data = Account::unpack(&buyer_token_account.data).unwrap();
	// assert_eq!(spl_token_account_data.owner, ctx.payer.pubkey());
	// println!("{:#?}", ctx.payer.pubkey());

	Ok(())
}