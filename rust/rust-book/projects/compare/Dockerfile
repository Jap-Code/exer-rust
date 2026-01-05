FROM opengauss/wasmtime:0.1.0

COPY target/wasm32-wasip1/release/compare.wasm /app/compare.wasm

ENTRYPOINT ["wasmtime", "app/compare.wasm"]