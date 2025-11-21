.PHONY: contract_test contract_build contract_accounts contract_testnet

contract_test:
	cargo test

# --------- CONTRACT BUILD/TEST/DEPLOY --------- #

contract_build:
	stellar contract build
	@ls -l target/wasm32v1-none/release/*.wasm

contract_accounts:
	./accounts.sh

contract_testnet:
	./run.sh