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

## Minimal example

Let's try building a small fullstack application using simple HTML and the [Poem](https://crates.io/crates/poem) HTTP framework for Rust. This example is available for view/download [here](https://github.com/L-Benjamin/wasmql/tree/main/examples/poem-html-js).

First, let's create our server:

```sh
cargo new wasmql-example
cd wasmql-example
```

Let's add the required dependencies to our main `Cargo.toml`:

```toml
# wasmql-example/Cargo.toml
[package]
name = "wasmql-example"
version = "0.1.0"
edition = "2021"

[worspace]
members = ["my-codec"] # Our codec crate (yet to be created)

[dependencies]
my-codec = { path = "my-codec" }
poem = { version = "1", features = ["static-files"] }
tokio = { version = "1", features = ["rt-multi-thread"] }

[build-dependencies]
wasmql-build = "*"
```

Next, let's create our WasmQL codec, that will define our API:

```sh
cargo new --lib my-codec
cd my-codec
```

Let's also configure this project's `Cargo.toml`:

```toml
# wasmql-example/my-codec/Cargo.toml
[package]
name = "my-codec"
version = "0.1.0"
edition = "2021"

[dependencies]
wasmql = "*"
```

Now here is the code of our codec, in `my-codec/src/lib.rs`:

```
// wasmql-example/my-codec/src/lib.rs
#![no_std] // Greatly reduce .wasm binary size

use wasmql::prelude::*; // Use String

#[wasmql::codec] // Our WasmQL codec API
pub trait MyCodec {
    fn greet(self, name: String) -> String;
}
```

Next, get back to the root of our project, and put the code of our server in `src/main.rs`:

```
// wasmql-example/src/main.rs
use std::io;

use my_codec::MyCodec;
use poem::endpoint::StaticFilesEndpoint;
use poem::http::StatusCode;
use poem::listener::TcpListener;
use poem::{post, Error, Route, Server};

struct MyCodecImpl; // Type implementing our WasmQL Codec

impl MyCodec for MyCodecImpl {
    fn greet(self, name: String) -> String {
        format!("Hello, {name}!")
    }
}

#[poem::handler]
async fn wasmql(bytes: Vec<u8>) -> poem::Result<Vec<u8>> {
    MyCodecImpl
        .dispatch(&bytes)
        .map_err(|_| Error::from_status(StatusCode::BAD_REQUEST))
}

#[tokio::main]
pub async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080");
    println!("Server started at http://127.0.0.1:8080");

    let app = Route::new()
        .at("/wasmql", post(wasmql))
        .nest("/", StaticFilesEndpoint::new("dist").index_file("index.html"));

    Server::new(listener).run(app).await
}
```

Almost done! Let's get back to the root of our project and create the HTML document that will be served to our client:

```sh
mkdir dist
cd dist
touch index.html
```

And in `dist/index.html`:

```html
<!-- wasmql-example/dist/index.html -->
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>WasmQL Demo</title>
    </head>
    <body>
        <label for="name">Your name:</label>
        <input id="name" value="John Doe">
        <h1 id="greeting">Hello, John Doe!</h1>

        <script type="module">
            import wasmql from "/wasmql.min.js";

            // Initialize our codec
            let codec = await wasmql({
                endpoint: "/wasmql",
                wasm: "/codec.wasm",
            });

            document.getElementById("name").oninput = async (e) => {
                // Call our codec's greet method and update document.
                let greeting = await codec.greet(e.target.value);
                document.getElementById("greeting").innerHTML = greeting;
            };
        </script>
    </body>
</html>
```

Finally, get back to our project's root one last time, and create a file named `build.rs` with the following code in it:

```
// wasmql-example/build.rs
use std::io::Result;

fn main() -> Result<()> {
    wasmql_build::export_js_library("dist/wasmql.min.js", true)?;
    wasmql_build::compile_wasm_codec("my-codec", "dist/codec.wasm")?;
    Ok(())
}
```

And we're done! Here is the layout of our project:

```txt
.
├── .gitignore
├── build.rs
├── Cargo.toml
├── my-codec
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── dist
│   └── index.html
└── src
    └── main.rs
```

To run our example, simply do:

```sh
cargo run
```

And click on the link!

## More Examples

* [This one](https://github.com/L-Benjamin/wasmql/tree/main/examples/hyper-react-ts) contains a little more involved example, using [Hyper](https://crates.io/crates/hyper) for the backend and [`React`](https://react.dev/) for the frontend to build a complete TODO web app.
