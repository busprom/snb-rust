use solana_program::program_error::ProgramError as BorshError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    pubkey::Pubkey,
    account_info::AccountInfo
};


#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub enum CollectionDetails {
    V1 { size: u64 },
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Data {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Uses {
    pub use_method: UseMethod,
    pub remaining: u64,
    pub total: u64
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Edition {
    pub instruction: u8,
    pub max_supply: Option<u64>
} 

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Update {
    pub instruction: u8,
    pub data: Option<Data>,
    pub update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct CollectionInfo {
    pub owner: Pubkey,
    pub collection: Pubkey
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct MetadataMetaplex {
    pub key: Key,
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub data: DataMetaplex,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<TokenStandard>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
    pub collection_details: Option<CollectionDetails>,
    pub programmable_config: Option<ProgrammableConfig>,
}


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum TokenStandard {
    NonFungible,             // This is a master edition
    FungibleAsset,           // A token with metadata that can also have attrributes
    Fungible,                // A token with simple metadata
    NonFungibleEdition,      // This is a limited edition
    ProgrammableNonFungible, // NonFungible with programmable configuration
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ProgrammableConfig {
    V1 {
        /// Programmable authorization rules.
        #[cfg_attr(
            feature = "serde-feature",
            serde(
                deserialize_with = "deser_option_pubkey",
                serialize_with = "ser_option_pubkey"
            )
        )]
        rule_set: Option<Pubkey>,
    },
}


#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Key {
    Uninitialized,
    EditionV1,
    MasterEditionV1,
    ReservationListV1,
    MetadataV1,
    ReservationListV2,
    MasterEditionV2,
    EditionMarker,
    UseAuthorityRecord,
    CollectionAuthorityRecord,
    TokenOwnedEscrow,
    TokenRecord,
    MetadataDelegate,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct DataMetaplex {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>
}

impl MetadataMetaplex {
    fn safe_deserialize(mut data: &[u8]) -> Result<Self, BorshError> {
        let result = Self::deserialize(&mut data)?;
        Ok(result)
    }

    pub fn from_account_info(a: &AccountInfo) -> Result<Self, BorshError> where {
        let data = &a.data.borrow_mut();
        let ua = Self::safe_deserialize(data)?;
        Ok(ua)
    }
}


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct User {
    pub payer: Pubkey,
    pub step: u8
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct Metadata {
    pub instruction: u8,
    pub data: Data,
    pub is_mutable: bool,
    pub collection_details: Option<CollectionDetails>
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateMetadataArgs {
    pub metadata: Metadata,
    pub info: Details
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct Details {
    pub collection: Option<Pubkey>,
    pub kind: String
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct TransferToken {
    pub instruction: u8,
    pub args: Args
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum Args {
    V1 {
        amount: u64,
        authorization_data: Option<u8>
    }
}