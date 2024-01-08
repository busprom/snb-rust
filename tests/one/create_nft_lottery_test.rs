use borsh::de::BorshDeserialize;
use crate::one::_Env;
use solana_program_test::{BanksClientError, tokio};
use solana_sdk::{
  signature::{Keypair, Signer},
  transaction::Transaction
};
use nft::{
	instruction::SolInstruction,
  types::nft_lottery::{NftLottery},
  id, LOTTERY_NFT
};
use solana_program::{
	pubkey::Pubkey,
};
use crate::utils::{
  init_nft_lottery::create_init_lottery,
  init_metadata::init_metadata
};
// use solana_sdk::program_pack::Pack;



#[tokio::test]
async fn create_token_account() -> Result<(), BanksClientError> {
  let mut i = _Env::_new().await;
  
  let (storage, _) = Pubkey::find_program_address(
    &[LOTTERY_NFT.as_bytes(), id().as_ref(), i.payer.pubkey().as_ref()],
    &id(),
  );

  let mut data = create_init_lottery();

  for _ in 0..14 {
    let new_mint = Keypair::new();
    let new_mint_account = Keypair::new();
    data.nfts.push(new_mint.pubkey());
  }

  i.ctx.banks_client.process_transaction(
    Transaction::new_signed_with_payer(
      &[SolInstruction::create_nft_lottery(
        i.payer.pubkey(),
        storage,
        data
      )],
      Some(&i.payer.pubkey()),
      &[&i.payer],
      i.ctx.last_blockhash,
    )
  ).await?;

  let res_storage = i.ctx.banks_client.get_account(storage).await.unwrap().unwrap();
  let res = NftLottery::try_from_slice(res_storage.data.as_slice()).unwrap();

  for el in res.nfts {
    i.ctx.banks_client.process_transaction(
      Transaction::new_signed_with_payer(
        &[SolInstruction::nft_lottery_get_token(
          i.payer.pubkey(), el, el, el, el, el, el,
          storage, el, el,
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

  let el = Keypair::new().pubkey();

  i.ctx.banks_client.process_transaction(
    Transaction::new_signed_with_payer(
      &[SolInstruction::nft_lottery_check(
        i.payer.pubkey(), 
        res.lots[0].win[0].collect[0],
        res.lots[0].win[0].collect[1],
        res.lots[0].win[0].collect[2],
        res.lots[0].win[0].collect[3],
        res.lots[0].win[0].collect[4],
        res.lots[0].win[0].collect[5],
        res.lots[0].win[0].collect[6],
        res.lots[0].win[0].collect[7],
        res.lots[0].win[0].collect[8],
        res.lots[0].win[0].collect[9],
        el, el, el, el, el, el, el, el, el, el,
        storage, el
      )],
      Some(&i.payer.pubkey()),
      &[&i.payer],
      i.ctx.last_blockhash,
    )
  ).await?;

  // let res_storage = i.ctx.banks_client.get_account(storage).await.unwrap().unwrap();
  // let res = NftLottery::try_from_slice(res_storage.data.as_slice()).unwrap();
  // println!("{:#?}", res);

  Ok(())
}