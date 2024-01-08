// use borsh::de::BorshDeserialize;
use solana_sdk::{
	signature::{Keypair, Signer},
	transaction::Transaction,
	pubkey::Pubkey
};
use solana_program_test::{BanksClientError, tokio};
use solana_program::system_instruction;
use crate::one::Env;
use nft::{
	PREFIX, id,
	instruction::SolInstruction,
	types::metadata::{
		Metadata,	Uses, Data, Collection, Creator
	}
};


#[tokio::test]
async fn metadata_test() -> Result<(), BanksClientError> {
	
	let mint = Keypair::new();
	let token = Keypair::new();

	let mut context = Env::new().await;
	
	let rent = context.ctx.banks_client.get_rent().await.unwrap();

	// create_mint(context, &self.mint, &context.payer.pubkey(), None).await?;
	let mut tx = Transaction::new_signed_with_payer(
		&[
			system_instruction::create_account(
				&context.payer.pubkey(),
				&mint.pubkey(),
				rent.minimum_balance(82),
				82 as u64,
				&spl_token::id(),
			),
			spl_token::instruction::initialize_mint(
				&spl_token::id(),
				&mint.pubkey(),
				&context.payer.pubkey(),
				None,
				0,
			)
			.unwrap(),
		],
		Some(&context.payer.pubkey()),
		&[&context.payer, &mint],
		context.ctx.last_blockhash,
	);
	context.ctx.banks_client.process_transaction(tx).await?;

	// create_token_account(&self.token, &self.mint.pubkey(), &context.payer.pubkey())
	tx = Transaction::new_signed_with_payer(
		&[
			system_instruction::create_account(
				&context.payer.pubkey(),
				&token.pubkey(),
				rent.minimum_balance(165),
				165 as u64,
				&spl_token::id(),
			),
			spl_token::instruction::initialize_account(
				&spl_token::id(),
				&token.pubkey(),
				&mint.pubkey(),
				&context.payer.pubkey(),
			)
			.unwrap(),
		],
		Some(&context.payer.pubkey()),
		&[&context.payer, &token],
		context.ctx.last_blockhash,
	);
  context.ctx.banks_client.process_transaction(tx).await?;

	// mint_tokens(mint: &Pubkey, account: &Pubkey, amount: u64, owner: &Pubkey)
	tx = Transaction::new_signed_with_payer(
		&[
			spl_token::instruction::mint_to(
			&spl_token::id(), 
			&mint.pubkey(), &token.pubkey(), &context.payer.pubkey(), &[], 1)
			.unwrap(),
		],
		Some(&context.payer.pubkey()),
		&[&context.payer],
		context.ctx.last_blockhash,
	);
	context.ctx.banks_client.process_transaction(tx).await?;

	let mint_pubkey = mint.pubkey();
	let program_id = id();

	let metadata_seeds = &[PREFIX.as_bytes(), program_id.as_ref(), mint_pubkey.as_ref()];
  let (pubkey, _) = Pubkey::find_program_address(metadata_seeds, &id());

	let init = Metadata {
		key: 1,
    update_authority: context.payer.pubkey(),
    mint: mint.pubkey(),
    data: Data {
			name: "Test name".to_string(),
			symbol: "Test symbol".to_string(),
			uri: "Test uri".to_string(),
			seller_fee_basis_points: 3,
			creators: Some(vec! [Creator {
				address: context.payer.pubkey(),
				verified: true,
				share: 1,
			}])
		},
    primary_sale_happened: false,
    is_mutable: false,
    edition_nonce: Some(4),
    token_standard: Some(1),
    collection: Some(Collection {
			verified: true,
    	key: mint.pubkey(),
		}),
    uses: Some(Uses {
			use_method: 1,
    	remaining: 1,
    	total: 5,
		})
	};

	let tx = Transaction::new_signed_with_payer(
		&[SolInstruction::get_mint(
			pubkey,
			mint.pubkey(),
			context.payer.pubkey(),
			init
		)],
		Some(&context.payer.pubkey()),
		&[&context.payer],
		context.ctx.last_blockhash,
	);
	context.ctx.banks_client.process_transaction(tx).await?;

	let acc = context.ctx.banks_client.get_account(pubkey).await.unwrap().unwrap();
	println!("existing_mint_authority - {:#?}", acc);

	let res = Metadata::slice(acc.data.as_slice());
	println!("existing_mint_authority METADATA - {:#?}", res);

	Ok(())
}