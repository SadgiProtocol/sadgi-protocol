build:
	cargo build --target wasm32-unknown-unknown --release

test:
	cargo test

deploy: build
	stellar contract deploy \
		--wasm target/wasm32-unknown-unknown/release/starter_soroban.wasm \
		--source alice \
		--network testnet
