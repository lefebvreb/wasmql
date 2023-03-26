# Poem TODO App

This directory contains a minimal example on building a simple TODO app using WasmQL, the [Poem](https://crates.io/crates/poem) backend framework and HTML, CSS and JS for the frontend.

## Structure

The structure of this project is pretty standard and straightforward:

* `codec/` contains your custom WasmQL codec crate.
* `dist/` contains the static files served by the server:
    * `index.html` is the entry point of the frontend, instantiating the WasmQL codec and consuming the API it exposes.
    * `codec.wasm` and `wasmql.min.js` files are generated when building the server and are used to instantiate the WasmQL codec.
    * `styles.css` contains some CSS styling for the page.
* `src/` contains the source code for the backend server that:
    * Exposes the endpoint `/wasmql` that exposes the API defined in `codec/`.
    * Serves the static files in `dist/`.
* `build.rs` contains some instructions for generating the `dist/codec.wasm` and `dist/wasmql.min.js` files.
