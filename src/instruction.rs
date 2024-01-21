use borsh::{BorshDeserialize, BorshSerialize};
use crate::{
	types::{
		staking::{Staking, Admin}
	}
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SolInstruction {
	CreateTokenAcc,
	AddStaking {data: Staking},
	Unstake,
	Claim,
	Admin {data: Admin}
}