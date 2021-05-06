rustup target add wasm32-unknown-unknown

cargo install wasm-gc

cargo build --target wasm32-unknown-unknown --release

wasm-gc target/wasm32-unknown-unknown/release/crypto_gcid.wasm
