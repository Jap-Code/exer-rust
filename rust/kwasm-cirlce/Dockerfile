FROM scratch

COPY circle/target/wasm32-wasip1/release/circle.wasm /module.wasm

ENTRYPOINT ["/module.wasm"]