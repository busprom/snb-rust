use borsh::{BorshDeserialize, BorshSerialize};
use crate::{
	types::{
		staking::{Staking}
	}
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SolInstruction {
	AddStaking {data: Staking},
	Unstake,
	Claim
}