start:
	cargo watch -x 'test-bpf -- --show-output'

key:
	solana-keygen new --outfile netw/program.json --force

update:
	cargo build-bpf
	solana program deploy target/deploy/nft.so --program-id netw/program.json --keypair netw/admin.json --upgrade-authority netw/admin.json