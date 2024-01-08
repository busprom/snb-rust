use borsh::de::BorshDeserialize;
use crate::one::_Env;
use solana_program_test::{BanksClientError, tokio};
use solana_sdk::{
  signature::{Keypair, Signer},
  transaction::Transaction
};
use nft::{
	instruction::SolInstruction,
  types::{
    nft_lottery::{NftLottery, NftStorage, Lot}
  },
  id, LOTTERY_NFT
};
use solana_program::{
	pubkey::Pubkey,
};
use crate::utils::init_metadata::init_metadata;
use crate::utils::tokens::create_token;
// use solana_sdk::program_pack::Pack;



#[tokio::test]
async fn create_token_account() -> Result<(), BanksClientError> {
  let mut i = _Env::_new().await;
  let (storage, _) = Pubkey::find_program_address(
    &[LOTTERY_NFT.as_bytes(), id().as_ref(), i.payer.pubkey().as_ref()],
    &id(),
  );

  let pic1 = Pubkey::new_unique();
  let pic2 = Pubkey::new_unique();
  let pic3 = Pubkey::new_unique();
  let pic4 = Pubkey::new_unique();
  let pic5 = Pubkey::new_unique();
  let pic6 = Pubkey::new_unique();
  let pic7 = Pubkey::new_unique();
  let pic8 = Pubkey::new_unique();
  let pic9 = Pubkey::new_unique();
  let pic10 = Pubkey::new_unique();

  i.ctx.banks_client.process_transaction(
    Transaction::new_signed_with_payer(
      &[SolInstruction::nft_collection_create(
        i.payer.pubkey(),
        storage,
        NftStorage {
          name: "Test Lottery".to_string(),
          owner: i.payer.pubkey(),
          pictures: vec![pic1, pic2, pic3, pic4, pic5, pic6, pic7, pic8, pic9, pic10],
          url: String::from("https://cryptomore.me/meta/")
        }
      )],
      Some(&i.payer.pubkey()),
      &[&i.payer],
      i.ctx.last_blockhash,
    )
  ).await?;

  
  for j in 1..2 {
    i.ctx.banks_client.process_transaction(
      Transaction::new_signed_with_payer(
        &[SolInstruction::nft_lot_add(
          i.payer.pubkey(),
          storage,
          Lot {
            collect: vec![0, 1, 2, 3, 4, 4, 4, 7, 8, 9],
            qty: 1,
            wins: 0,
            bonus: 1000000000
          }
        )],
        Some(&i.payer.pubkey()),
        &[&i.payer],
        i.ctx.last_blockhash,
      )
    ).await?;
  }

  for _ in 0..5 {// mint tokens
    let mint = Keypair::new();
    let mint_account = Keypair::new();
    let (mint_storage, _) = Pubkey::find_program_address(
      &[LOTTERY_NFT.as_bytes(), id().as_ref(), mint.pubkey().as_ref()],
      &id(),
    );

    create_token(&mut i.ctx, &mint, &i.payer, &mint_account).await?;

    i.ctx.banks_client.process_transaction(// минтим токены на лот
      Transaction::new_signed_with_payer(
        &[SolInstruction::create_one_token(
          i.payer.pubkey(),
          mint.pubkey(),
          mint_account.pubkey(),
          mint.pubkey(),         //metaaccount
          mint_account.pubkey(), //metaprog
          mint.pubkey(),         //profitID
          storage,
          mint_storage,
          mint.pubkey(),         //ATA program
          init_metadata()
        )],
        Some(&i.payer.pubkey()),
        &[&i.payer, &mint],
        i.ctx.last_blockhash,
      )
    ).await?;

    let new_mint = Keypair::new();
    let new_mint_account = Keypair::new();
    let (new_mint_storage, _) = Pubkey::find_program_address(
      &[LOTTERY_NFT.as_bytes(), id().as_ref(), new_mint.pubkey().as_ref()],
      &id(),
    );

    i.ctx.banks_client.process_transaction(// минтим токены на лот
      Transaction::new_signed_with_payer(
        &[SolInstruction::nft_lottery_get_token(
          i.payer.pubkey(),
          mint.pubkey(),
          mint_account.pubkey(),
          mint_storage,
          new_mint.pubkey(),
          new_mint_account.pubkey(),
          new_mint_storage,
          mint.pubkey(),         //metaaccount
          mint_account.pubkey(), //metaprog
          storage,
          mint.pubkey(),         //profitID
          mint.pubkey(),         //ATA program
          init_metadata()
        )],
        Some(&i.payer.pubkey()),
        &[&i.payer],
        i.ctx.last_blockhash,
      )
    ).await?;

  }




  let res_storage = i.ctx.banks_client.get_account(storage).await.unwrap().unwrap();
  let res = NftLottery::try_from_slice(res_storage.data.as_slice()).unwrap();
  // println!("{:#?}", res_storage);
  println!("{:#?}", res);
  

  Ok(())
}