use std::io;

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Result};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Server started at http://127.0.0.1:8080");

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(respond))
                .await
            {
                println!("Failed to serve connection: {err}");
            }
        });
    }
}

async fn respond(req: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
    todo!()
}
