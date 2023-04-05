use std::io;

use codec::MyCodec;
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
