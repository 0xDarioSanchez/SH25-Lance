.PHONY: run build test

contract_test:
	cargo test

# --------- CONTRACT BUILD/TEST/DEPLOY --------- #

contract_build:
	stellar contract build
	@ls -l target/wasm32v1-none/release/*.wasm