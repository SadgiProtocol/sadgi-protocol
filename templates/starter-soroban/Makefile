build:
	cargo build --target wasm32v1-none --release

test:
	cargo test

deploy: build
	stellar contract deploy \
		--wasm target/wasm32v1-none/release/starter_soroban.wasm \
		--source alice \
		--network testnet
