rustup target add wasm32-unknown-unknown
cargo component build --release --target wasm32-unknown-unknown
#wasm-tools component new ./target/wasm32-unknown-unknown/release/example.wasm \
#    -o my-component.wasm --adapt ./wasi_snapshot_preview1.wasm