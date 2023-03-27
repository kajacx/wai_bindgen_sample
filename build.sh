#!/usr/bin/sh

cd wai-sample-plugin && cargo build --target=wasm32-unknown-unknown && cd .. && \
echo All done
