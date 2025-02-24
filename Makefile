run:
	cargo watch -x 'test-sbf -- --show-output'

deploy:
	cargo build-sbf -- --ignore-rust-version
	solana program deploy target/deploy/nft.so --program-id netw/program.json --keypair netw/admin.json --upgrade-authority netw/admin.json