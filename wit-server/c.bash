cargo build --target wasm32-unknown-unknown
wasm-tools component new ./target/wasm32-unknown-unknown/debug/wit_server.wasm \
    -o my-component.wasm --adapt ./wasi_snapshot_preview1.wasm