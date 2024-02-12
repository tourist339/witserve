rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
wasm-tools component new ./target/wasm32-unknown-unknown/debug/wit_server.wasm \
    -o fib.wasm --adapt ./wasi_snapshot_preview1.wasm