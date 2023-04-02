use std::io;

use codec::TestCodec;
use poem::endpoint::StaticFilesEndpoint;
use poem::http::StatusCode;
use poem::listener::TcpListener;
use poem::{post, Error, Route, Server};

struct TestCodecImpl;

impl TestCodec for TestCodecImpl {
    fn echo(self, data: codec::EchoData) -> codec::EchoData {
        assert_eq!(data, *codec::DATA);
        data
    }
}

#[poem::handler]
async fn wasmql(bytes: Vec<u8>) -> poem::Result<Vec<u8>> {
    TestCodecImpl
        .dispatch(&bytes)
        .map_err(|_| Error::from_status(StatusCode::BAD_REQUEST))
}

#[tokio::main]
pub async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080");

    let app = Route::new()
        .at("/wasmql", post(wasmql))
        .nest(
            "/",
            StaticFilesEndpoint::new("dist").index_file("index.html"),
        );

    println!("Server started at http://127.0.0.1:8080");

    Server::new(listener).run(app).await?;

    Ok(())
}
