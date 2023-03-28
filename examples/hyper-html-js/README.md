# Hyper - HTML - JS

This directory contains a minimal "hello world"-type example of using WasmQL.

* Backend: [hyper](https://crates.io/crates/hyper) (Rust).
* Frontend: HTML and JavaScript.

## Structure

The structure of this project is very simple:

* `codec/` contains our WasmQL definition, compiled to wasm for the browser and implemented in the backend.
* `dist/` contains our static resources, exposes to the frontend:
    * `index.html`, our HTML page.
    * `wasmql.min.js`, the minified wasmql js library, that is provided when the server is built.
    * `codec.wasm`, the codec wasm binary, that is compiled when the server is built.
* `src/` contains the code for our backend server, using the low-level hyper HTTP framework.
* `build.rs` contains instructions for building the codec into a wasm binary and exporting the minified js library.