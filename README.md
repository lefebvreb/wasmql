# WasmQL

WasmQL stands for Wasm Query Layer

# TODO

* Finish
* Tests
* Optimize serialization to chunk data output
* Typescript bindings
* Serde i128/u128 to bigints ?
* Rename "api" macro to "codec"
* Better modules

cargo rustc --config "dependencies.wasmql.features=['frontend']" --config "dependencies.wasmql.default-features=false" --crate-type cdylib --release --target wasm32-unknown-unknown
