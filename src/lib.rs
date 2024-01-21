pub mod utils;
pub mod processor;
pub mod error;
pub mod instruction;
pub mod types;
pub mod token;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;



pub const FOUNDER_ID: &str = "BJMcUQNyA1a3u1xicZte8ARgQGwkm5G6xQsZ5kxVj5J";
pub const ADMIN_ID: &str = "Euy2YtCb7sQvFQu3ohS1eqe72g6yRqqEu1eZVwg9oqUG";
pub const SNB_SEED: &str = "TPT";

