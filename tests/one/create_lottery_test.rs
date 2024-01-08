use solana_sdk::{
  signature::{Keypair, Signer},
	transaction::Transaction
};
use solana_program_test::{BanksClientError, tokio};
use solana_program::pubkey::Pubkey;
use crate::one::Env;
use crate::utils::tokens::create_token_with_mint;
use nft::{
  types::lottery::{Lottery, Lot},
  LOTTERY_SEED, id, instruction::SolInstruction
};


#[tokio::test]
async fn lottery_test() -> Result<(), BanksClientError> {

	let mut context = Env::new().await;

  let mut tx: Vec<Transaction> = Vec::new();
  let mut lots: Vec<Lot> =  Vec::new();

  let mut accs: Vec<Pubkey> = Vec::new();
  let mut mints: Vec<Pubkey> = Vec::new();

	for i in 0..5 {

    let mint = Keypair::new();
    mints.push(mint.pubkey());

	  let token_account = Keypair::new();
    accs.push(token_account.pubkey());

    lots.push(
      Lot {
        address: mint.pubkey(),
        amount: 1000000+(i*10000),
        status: 0
      }
    );

    tx.push(
      create_token_with_mint(
        &mut context.ctx,
        &context.payer,
        &mint,
        &token_account
      ).await
    );

  }

  for el in tx.clone() {
    context.ctx.banks_client.process_transaction(el).await?;
  }

  let (lottery, _) = Pubkey::find_program_address(
    &[LOTTERY_SEED.as_bytes(), id().as_ref(), context.payer.pubkey().as_ref()],
    &id(),
  );

  context.ctx.banks_client.process_transaction(
    Transaction::new_signed_with_payer(
      &[SolInstruction::create_lottery(
        context.payer.pubkey(),
        lottery,
        Lottery { lots }
      )],
      Some(&context.payer.pubkey()),
      &[&context.payer],
      context.ctx.last_blockhash,
    )
  ).await?;

  let mut i = 0;
  for _ in &accs {
    context.ctx.banks_client.process_transaction(
      Transaction::new_signed_with_payer(
        &[SolInstruction::check_lottery(
          context.payer.pubkey(),
          mints[i],
          accs[i],
          lottery
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.ctx.last_blockhash,
      )
    ).await?;
    i += 1;
  }
  


  // let res_storage = context.ctx.banks_client.get_account(lottery).await.unwrap().unwrap();
  // let res = Lottery::try_from_slice(res_storage.data.as_slice()).unwrap();
  // println!("lottery account - {:#?}", res.lots[9]);

  // assert_eq!(res.lots[9].2, 1);

  // println!("mints account - {:#?}", mints);

	Ok(())
}