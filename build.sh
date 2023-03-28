#!/usr/bin/sh

cd wai-sample-plugin && cargo build --target=wasm32-unknown-unknown && cd .. && \
cd wai-sample-host && cargo run && cd .. && \
echo All done
