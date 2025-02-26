use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Staking {
  pub start: u64,
  pub end: u64,
  pub owner: Pubkey,
  pub mint: Pubkey
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Admin {
  pub snb_transfer: u64,
  pub update: Option<Staking>
}