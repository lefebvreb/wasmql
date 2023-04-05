# Poem + HTML + JS

This directory contains a minimal "hello world"-type example of using WasmQL.

* Backend: [Poem](https://crates.io/crates/poem) (Rust).
* Frontend: HTML and JavaScript.

## Structure

The structure of this project is very simple:

* `codec/` contains our WasmQL codec definition, compiled to wasm for the browser and implemented in the backend.
* `dist/` contains our static resources, exposed to the frontend:
    * `index.html`, our HTML document.
    * (generated) `wasmql.min.js`, the minified wasmql js library, that is provided when the server is built.
    * (generated) `codec.wasm`, the codec wasm binary, that is compiled when the server is built.
* `src/` contains the code for our backend server, using the high-level Poem HTTP framework.
* `build.rs` contains instructions for building the codec into a wasm binary and exporting the minified js library.

## Running

You need to have `cargo` installed with the additional `wasm32-unknown-unknown` rustc toolchain available.
With these on your system, simply place yourself at the root of this directory and run:

```sh
cargo run --release
```

This prints an url that you can follow to the web page.
