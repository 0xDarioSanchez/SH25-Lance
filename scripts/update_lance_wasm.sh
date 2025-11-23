#!/usr/bin/env bash
set -euo pipefail

# Helper script to build and optimize lance-protocol wasm and place
# the optimized file where governor-dao's contractimport expects it.

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
CONTRACT_DIR="$ROOT_DIR/contracts/lance-protocol"
BUILD_WASM="$CONTRACT_DIR/target/wasm32-unknown-unknown/release/lance_protocol.wasm"
OUT_DIR="$ROOT_DIR/target/wasm32v1-none/release"
OUT_WASM="$OUT_DIR/lance_protocol.optimized.wasm"

echo "Root: $ROOT_DIR"

echo "1) Building lance-protocol (release, wasm32-unknown-unknown)..."
cargo build --release --target wasm32-unknown-unknown --manifest-path "$CONTRACT_DIR/Cargo.toml"

if [ ! -f "$BUILD_WASM" ]; then
  echo "ERROR: built wasm not found at: $BUILD_WASM"
  echo "Make sure the build succeeded and the crate name matches 'lance_protocol'."
  exit 1
fi

if ! command -v stellar >/dev/null 2>&1; then
  echo "ERROR: 'stellar' CLI not found in PATH."
  echo "Install it (example): cargo install --locked stellar-cli"
  echo "Or add the cargo bin dir to PATH: export PATH=\"$HOME/.cargo/bin:$PATH\""
  exit 1
fi

mkdir -p "$OUT_DIR"

echo "2) Optimizing wasm with 'stellar contract optimize'..."
stellar contract optimize \
  --wasm "$BUILD_WASM" \
  --output "$OUT_WASM"

if [ -f "$OUT_WASM" ]; then
  echo "Success: optimized wasm written to: $OUT_WASM"
else
  echo "ERROR: optimization failed, output not found: $OUT_WASM"
  exit 1
fi

echo "Done. You can now run 'cargo test' or rebuild governor-dao."
