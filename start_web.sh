
#!/bin/bash
set -eu

cargo build --release -p app --lib --target wasm32-unknown-unknown

wasm-bindgen target/wasm32-unknown-unknown/release/app.wasm \
--out-dir docs --no-modules --no-typescript

cd docs
basic-http-server --addr 127.0.0.1:3000 .