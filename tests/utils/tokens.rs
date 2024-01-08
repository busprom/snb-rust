use solana_sdk::{
  signature::{Keypair, Signer},
  transaction::Transaction,
};
use solana_program_test::{
  BanksClientError,
  ProgramTestContext
};
use solana_program::system_instruction;


pub async fn init_mint ( 
  ctx: &mut ProgramTestContext,
  mint: &Keypair,
  payer: &Keypair
) -> Result<(), BanksClientError> {
	let rent = ctx.banks_client.get_rent().await.unwrap();
  let tx = Transaction::new_signed_with_payer(
		&[
			system_instruction::create_account(
				&payer.pubkey(),
				&mint.pubkey(),
				rent.minimum_balance(82),
				82 as u64,
				&spl_token::id(),
			),
			spl_token::instruction::initialize_mint(
				&spl_token::id(),   // token_program_id
				&mint.pubkey(),     // mint_pubkey
				&payer.pubkey(),    // mint_authority_pubkey
				None,               // freeze_authority_pubkey
				0                   // decimals
			)
			.unwrap(),
		],
		Some(&payer.pubkey()),
		&[payer, mint],
		ctx.last_blockhash,
	);
	ctx.banks_client.process_transaction(tx).await
}

pub async fn create_account (
	ctx: &mut ProgramTestContext,
  payer: &Keypair,
	token_account: &Keypair
) -> Result<(), BanksClientError> {
	let rent = ctx.banks_client.get_rent().await.unwrap();
  let tx = Transaction::new_signed_with_payer(
		&[
			system_instruction::create_account(
				&payer.pubkey(),
				&token_account.pubkey(),
				rent.minimum_balance(165),
				165 as u64,
				&spl_token::id(),
			)
		],
		Some(&payer.pubkey()),
		&[payer, token_account],
		ctx.last_blockhash,
	);
  ctx.banks_client.process_transaction(tx).await
}

pub async fn get_token_account (
  ctx: &mut ProgramTestContext,
  mint: &Keypair,
  payer: &Keypair,
	token_account: &Keypair
) -> Result<(), BanksClientError> {
  let rent = ctx.banks_client.get_rent().await.unwrap();
  let tx = Transaction::new_signed_with_payer(
		&[
			system_instruction::create_account(
				&payer.pubkey(),
				&token_account.pubkey(),
				rent.minimum_balance(165),
				165 as u64,
				&spl_token::id(),
			),
			spl_token::instruction::initialize_account(
				&spl_token::id(),          // token_program_id
				&token_account.pubkey(),   // account_pubkey
				&mint.pubkey(),            // mint_pubkey
				&payer.pubkey()            // owner_pubkey
			)
			.unwrap(),
		],
		Some(&payer.pubkey()),
		&[payer, token_account],
		ctx.last_blockhash,
	);
  ctx.banks_client.process_transaction(tx).await
}

pub async fn mint_token (
	ctx: &mut ProgramTestContext,
  mint: &Keypair,
  payer: &Keypair,
	token_account: &Keypair
) -> Result<(), BanksClientError> {
	let tx = Transaction::new_signed_with_payer(
		&[
			spl_token::instruction::mint_to(
				&spl_token::id(),         // token_program_id
				&mint.pubkey(),           // mint_pubkey
				&token_account.pubkey(),  // account_pubkey
				&payer.pubkey(),          // owner_pubkey
				&[],                      // signer_pubkeys
				1                         // amount
			).unwrap(),
		],
		Some(&payer.pubkey()),
		&[payer],
		ctx.last_blockhash,
	);
	ctx.banks_client.process_transaction(tx).await
}

pub async fn transfer_token (
	ctx: &mut ProgramTestContext,
  from_token_account: &Keypair,
  to_token_account: &Keypair,
	payer: &Keypair
) -> Result<(), BanksClientError> {
	let tx = Transaction::new_signed_with_payer(
		&[
			spl_token::instruction::transfer(
				&spl_token::id(),               // token_program_id
				&from_token_account.pubkey(),   // source_pubkey
				&to_token_account.pubkey(),     // destination_pubkey
				&payer.pubkey(),                // authority_pubkey
				&[&payer.pubkey()],             // signer_pubkeys
				1,                              // amount
			).unwrap()
		],
		Some(&payer.pubkey()),
		&[payer],
		ctx.last_blockhash,
	);
	ctx.banks_client.process_transaction(tx).await
}

pub async fn create_token (
	ctx: &mut ProgramTestContext,
  mint: &Keypair,
  payer: &Keypair,
	token_account: &Keypair
) -> Result<(), BanksClientError> {
	init_mint(ctx, mint, payer).await?;
	get_token_account(ctx, mint, payer, token_account).await?;
	mint_token(ctx, mint, payer, token_account).await
}

pub async fn create_token_with_mint (
	ctx: &mut ProgramTestContext,
	payer: &Keypair,
	mint: &Keypair,
	token_account: &Keypair
) -> Transaction {
	let rent = ctx.banks_client.get_rent().await.unwrap();
	Transaction::new_signed_with_payer(
		&[
			system_instruction::create_account(
				&payer.pubkey(),
				&mint.pubkey(),
				rent.minimum_balance(82),
				82 as u64,
				&spl_token::id(),
			),
			spl_token::instruction::initialize_mint(
				&spl_token::id(),
				&mint.pubkey(),
				&payer.pubkey(),
				None,
				0
			).unwrap(),
			system_instruction::create_account(
				&payer.pubkey(),
				&token_account.pubkey(),
				rent.minimum_balance(165),
				165 as u64,
				&spl_token::id(),
			),
			spl_token::instruction::initialize_account(
				&spl_token::id(),          // token_program_id
				&token_account.pubkey(),   // account_pubkey
				&mint.pubkey(),            // mint_pubkey
				&payer.pubkey()            // owner_pubkey
			).unwrap(),
			spl_token::instruction::mint_to(
				&spl_token::id(),         // token_program_id
				&mint.pubkey(),           // mint_pubkey
				&token_account.pubkey(),  // account_pubkey
				&payer.pubkey(),          // owner_pubkey
				&[],                      // signer_pubkeys
				1                         // amount
			).unwrap(),
		],
		Some(&payer.pubkey()),
		&[payer, token_account, mint],
		ctx.last_blockhash,
	)
}