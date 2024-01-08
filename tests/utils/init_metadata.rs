use nft::types::metadata::{Data, CreateMetadataArgs, Metadata};

pub fn init_metadata() -> CreateMetadataArgs {
	CreateMetadataArgs {
		metadata: Metadata {
			instruction: 0,
			data: Data {
				name: "Test name".to_string(),
				symbol: "Test symbol".to_string(),
				uri: "Test uri".to_string(),
				seller_fee_basis_points: 3,
				creators: None
			},
			is_mutable: 0,
		},
		kind: "nft_box".to_string(),
		metaplex: 1
	}
}