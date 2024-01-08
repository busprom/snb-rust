#![cfg(feature = "test-bpf")]

use nft::{
	instruction::SolInstruction,
  types::sell::Sell
};
use crate::one::CreateStorage;
use crate::utils::tokens::get_token_account;
use borsh::de::BorshDeserialize;
use solana_sdk::{
	signature::{Signer, Keypair},
	transaction::Transaction,
  program_pack::Pack
};
use solana_program_test::{BanksClientError, tokio};
use spl_token::state::Account;


// #[tokio::test]
async fn _sale_test() -> Result<(), BanksClientError> {
  // прямая продажа auction: 0 //////////////////////////////////////////////////////////////
  let mut i = CreateStorage::new(0).await.unwrap();
  let res_storage = i.context.ctx.banks_client.get_account(i.storage).await.unwrap().unwrap();
  let res = Sell::try_from_slice(res_storage.data.as_slice()).unwrap();
  i.context.ctx.banks_client.process_transaction(
    Transaction::new_signed_with_payer(
      &[SolInstruction::withdrawal(
        i.mint.pubkey(),
        res.seller,
        i.context._buyer.pubkey(),
        i.buyer_account,
        i.vault,
        i.storage,
        i.context.payer.pubkey(), // aplicants account
        i.profit_id
      )],
      Some(&i.context._buyer.pubkey()),
      &[&i.context._buyer],
      i.context.ctx.last_blockhash,
    )
  ).await?;

  let buyer_token_account = i.context.ctx.banks_client.get_account(i.buyer_account).await.unwrap().unwrap();
  let spl_token_account_data = Account::unpack(&buyer_token_account.data).unwrap();
  // аккаунт принадлежит покупателю и на нем 1 токен
  assert_eq!(spl_token_account_data.owner, i.context._buyer.pubkey());
  assert_eq!(spl_token_account_data.amount, 1);
  assert_eq!(spl_token_account_data.mint, i.mint.pubkey());

  // владелец токена ctx.payer - покупатель ctx._buyer, проверка перечислений
  let token_owner_wallet = i.context.ctx.banks_client.get_account(i.context.payer.pubkey()).await.unwrap().unwrap();
  let buyer_wallet = i.context.ctx.banks_client.get_account(i.context._buyer.pubkey()).await.unwrap().unwrap();
  assert_eq!((token_owner_wallet.lamports - 2_000_000_000) > buyer_wallet.lamports, true);
  // Тест закрытия токен аккаунта
  let token_account = i.context.ctx.banks_client.get_account(i.token_account.pubkey()).await.unwrap();
  assert_eq!(token_account, None);

  let profit_id = i.context.ctx.banks_client.get_account(i.profit_id).await.unwrap().unwrap();
  assert_eq!(profit_id.lamports > 7962240, true);

  Ok(())
}

// #[tokio::test]
async fn _sale_owner_test() -> Result<(), BanksClientError> {
  // продажа через auction ////////////////////////////////////////////////////////
  // вывод токена вдадельцем, ставок еще не было
  let mut i = CreateStorage::new(1).await.unwrap();
  let res_storage = i.context.ctx.banks_client.get_account(i.storage).await.unwrap().unwrap();
  let res = Sell::try_from_slice(res_storage.data.as_slice()).unwrap();

  let owner_toket_account = Keypair::new();
  get_token_account(// создаем токен аккаунт для владельца токена
    &mut i.context.ctx,
    &i.mint,
    &i.context.payer,
    &owner_toket_account
  ).await?;

  i.context.ctx.banks_client.process_transaction(
    Transaction::new_signed_with_payer(
      &[SolInstruction::withdrawal(
        i.mint.pubkey(),
        res.seller,
        i.context.payer.pubkey(),
        owner_toket_account.pubkey(),
        i.vault,
        i.storage,
        res.applicant, // aplicants account
        i.profit_id
      )],
      Some(&i.context.payer.pubkey()),
      &[&i.context.payer],
      i.context.ctx.last_blockhash,
    )
  ).await?;

  let buyer_token_account = i.context.ctx.banks_client.get_account(owner_toket_account.pubkey()).await.unwrap().unwrap();
  let spl_token_account_data = Account::unpack(&buyer_token_account.data).unwrap();
  assert_eq!(spl_token_account_data.owner, i.context.payer.pubkey());
  assert_eq!(spl_token_account_data.mint, i.mint.pubkey());
  assert_eq!(spl_token_account_data.amount, 1);

  let profit_id = i.context.ctx.banks_client.get_account(i.profit_id).await.unwrap().unwrap();
  assert_eq!(profit_id.lamports > 7962240, true);

  Ok(())
}

// #[tokio::test]
async fn _auction_bet() -> Result<(), BanksClientError> {
  // продажа через auction ////////////////////////////////////////////////////////
  let mut i = CreateStorage::new(1).await.unwrap();
  let arr: [Keypair; 6] = [i.context._buyer, i.context._buyer1, i.context._buyer2, i.context._buyer3, i.context._buyer4, i.context._buyer5];

  for el in &arr {
    let owner_toket_account = Keypair::new();
    let res_storage = i.context.ctx.banks_client.get_account(i.storage).await.unwrap().unwrap();
    let res = Sell::try_from_slice(res_storage.data.as_slice()).unwrap();
    // println!("storage settings - {:#?}", res);
    i.context.ctx.banks_client.process_transaction(
      Transaction::new_signed_with_payer(
        &[SolInstruction::withdrawal(
          i.mint.pubkey(),
          res.seller,
          el.pubkey(),
          owner_toket_account.pubkey(),
          i.vault,
          i.storage,
          res.applicant, // aplicants account
          i.profit_id
        )],
        Some(&el.pubkey()),
        &[el],
        i.context.ctx.last_blockhash,
      )
    ).await?;
  }

  for el in &arr {
    let buyer_account = i.context.ctx.banks_client.get_account(el.pubkey()).await.unwrap().unwrap();
    println!("buyer_account - {:#?}", (el.pubkey(), buyer_account.lamports));
  }

  // отмена текущей ставки
  let res_storage = i.context.ctx.banks_client.get_account(i.storage).await.unwrap().unwrap();
  let res = Sell::try_from_slice(res_storage.data.as_slice()).unwrap();
  let owner_toket_account = Keypair::new();
  i.context.ctx.banks_client.process_transaction(
    Transaction::new_signed_with_payer(
      &[SolInstruction::withdrawal(
        i.mint.pubkey(),
        res.seller,
        arr[5].pubkey(),
        owner_toket_account.pubkey(),
        i.vault,
        i.storage,
        res.applicant, // aplicants account
        i.profit_id
      )],
      Some(&arr[5].pubkey()),
      &[&arr[5]],
      i.context.ctx.last_blockhash,
    )
  ).await?;

  let buyer_account = i.context.ctx.banks_client.get_account(arr[5].pubkey()).await.unwrap().unwrap();
  assert_eq!(buyer_account.lamports > 4000000000, true);

  let res_storage = i.context.ctx.banks_client.get_account(i.storage).await.unwrap().unwrap();
  let res = Sell::try_from_slice(res_storage.data.as_slice()).unwrap();
  assert_eq!(res.price, res.start_price);

  Ok(())
}

#[tokio::test]
async fn _transfer_to_winer() -> Result<(), BanksClientError> {
  // продажа через auction ////////////////////////////////////////////////////////
  let mut i = CreateStorage::new(1).await.unwrap();
  let arr: [Keypair; 6] = [i.context._buyer, i.context._buyer1, i.context._buyer2, i.context._buyer3, i.context._buyer4, i.context._buyer5];

  for el in &arr {
    let owner_toket_account = Keypair::new();
    let res_storage = i.context.ctx.banks_client.get_account(i.storage).await.unwrap().unwrap();
    let res = Sell::try_from_slice(res_storage.data.as_slice()).unwrap();
    println!("storage settings - {:#?}", res);
    i.context.ctx.banks_client.process_transaction(
      Transaction::new_signed_with_payer(
        &[SolInstruction::withdrawal(
          i.mint.pubkey(),
          res.seller,
          el.pubkey(),
          owner_toket_account.pubkey(),
          i.vault,
          i.storage,
          res.applicant, // aplicants account
          i.profit_id
        )],
        Some(&el.pubkey()),
        &[el],
        i.context.ctx.last_blockhash,
      )
    ).await?;
  }

  let res_storage = i.context.ctx.banks_client.get_account(i.storage).await.unwrap().unwrap();
  let res = Sell::try_from_slice(res_storage.data.as_slice()).unwrap();

  let owner_toket_account = Keypair::new();
  get_token_account(// создаем токен аккаунт для владельца токена
    &mut i.context.ctx,
    &i.mint,
    &arr[5],
    &owner_toket_account
  ).await?;

  i.context.ctx.banks_client.process_transaction(
    Transaction::new_signed_with_payer(
      &[SolInstruction::withdrawal(
        i.mint.pubkey(),
        res.seller,
        i.context.payer.pubkey(),
        owner_toket_account.pubkey(), //account for transfer token
        i.vault,
        i.storage,
        res.applicant, // aplicants account
        i.profit_id
      )],
      Some(&i.context.payer.pubkey()),
      &[&i.context.payer],
      i.context.ctx.last_blockhash,
    )
  ).await?;

  let buyer_account = i.context.ctx.banks_client.get_account(res.seller).await.unwrap().unwrap();
  assert_eq!(buyer_account.lamports > 7000000000, true);

  let buyer_token_account = i.context.ctx.banks_client.get_account(owner_toket_account.pubkey()).await.unwrap().unwrap();
  let spl_token_account_data = Account::unpack(&buyer_token_account.data).unwrap();
  assert_eq!(spl_token_account_data.amount, 1);
  assert_eq!(spl_token_account_data.owner, arr[5].pubkey());

  println!("seller account - {:#?}", (arr[5].pubkey(), spl_token_account_data));
  
  Ok(())
}