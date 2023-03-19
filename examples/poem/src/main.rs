use std::io::Result;

use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;
use poem::{post, Route, Server};

#[poem::handler]
async fn wasmql() {}

#[tokio::main]
pub async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080");

    let app = Route::new()
        .at("/wasmql", post(wasmql))
        .nest("/", StaticFilesEndpoint::new("dist").index_file("index.html"));

    println!("Server started at http://127.0.0.1:8080");

    Server::new(listener).run(app).await?;

    Ok(())
}
