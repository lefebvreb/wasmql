# WasmQL

WasmQL (Web Assembly Query Layer) is a Rust and JavaScript library for 
offloading your backend's serialization to the frontend and organize your
API in a more RPC-oriented way, that integrates well with both Rust and JavaScript.

## Features

* Faster serialization/deserialization on the backend.
* Smaller messages on the wire: more throughput.
* Cleaner API design and implementation on the backend.
* Build utilities in the [`wasmql-build`](https://crates.io/crates/wasmql-build) crate.

## How

A project using WasmQL is structured in the following way:

* A Rust crate defining the WasmQL "codec": basically a Rust trait that is implemented on your backend and can be called remotely from the frontend.
* A Rust backend, using any framework, with an endpoint exposing the codec.
* A JavaScript/TypeScript frontend, using any technologies, that consumes the codec.

## Examples

* [This directory](https://github.com/L-Benjamin/wasmql/tree/main/examples/hyper-html-js) contains a minimal example of using WasmQL with [`hyper`](https://crates.io/crates/hyper) for the backend, and HTML/JavaScript for the frontend.
* [This one](https://github.com/L-Benjamin/wasmql/tree/main/examples/hyper-html-js) contains a little more involved example with [`poem`](https://crates.io/crates/poem) for the backend and [`React`](https://react.dev/) for the frontend.
