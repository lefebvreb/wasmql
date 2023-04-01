# Poem + React + TS

This directory contains a complete example of using WasmQL to create a simple TODO webapp.

* Backend: [poem](https://crates.io/crates/poem) (Rust).
* Frontend: [React](https://react.dev/), CSS, TypeScript and [Vite](https://vitejs.dev/).

## Render

<img src="/doc/poem-react-ts.jpg" width="350" title="HTML render of the TODO app">

## Structure

<!-- The structure of this project is pretty standard and straightforward:

* `codec/` contains your custom WasmQL codec crate.
* `dist/` contains the static files served by the server:
    * `index.html` is the entry point of the frontend, instantiating the WasmQL codec and consuming the API it exposes.
    * `codec.wasm` and `wasmql.min.js` files are generated when building the server and are used to instantiate the WasmQL codec.
    * `styles.css` contains some CSS styling for the page.
* `src/` contains the source code for the backend server that:
    * Exposes the endpoint `/wasmql` that exposes the API defined in `codec/`.
    * Serves the static files in `dist/`.
* `build.rs` contains some instructions for generating the `dist/codec.wasm` and `dist/wasmql.min.js` files. -->

## Running

You need to have `cargo` with the additional `wasm32-unknown-unknown` toolchain installed. You also need [npm](https://www.npmjs.com/) to resolved the frontend dependencies.

With all these on your system, simply place yourself at the root of this directory and run:

```sh
npm install
```

This will install all the required dependencies for the frontend page, then do:

```sh
cargo run --release
```

This will compile your backend server, your codec to wasm and run the server, printing the URL that will take you to it.
