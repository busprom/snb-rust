use solana_program_test::tokio;
use solana_sdk::{
  signature::{Keypair, Signer},
  pubkey::Pubkey
};

#[tokio::test]
async fn arr_test() {

  let mint_1 = Keypair::new(); // н
  let mint_2 = Keypair::new(); // ю
  let mint_3 = Keypair::new(); // я


  let mut collect: Vec<Pubkey> = vec![
    mint_1.pubkey(),
    mint_2.pubkey(),
    mint_1.pubkey(),
    mint_3.pubkey()
  ];

  let mut tokens: Vec<Pubkey> = vec![
    mint_1.pubkey(),
    mint_3.pubkey(),
    mint_1.pubkey(),
    mint_2.pubkey()
  ];

  collect.sort_by_key(|k| k.to_string());
  tokens.sort_by_key(|k| k.to_string());

  println!("{:#?}", (collect.clone(), tokens.clone()));

  if collect != tokens {
    println!("{:#?}", "noooooooo");
  }
  
  if collect == tokens {
    println!("{:#?}", "yeeeeessss");
  }
  
}