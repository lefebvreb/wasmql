use std::collections::BTreeMap;
use std::io;
use std::path::Path;
use std::sync::Mutex;

use codec::TodoCodec;
use http_body_util::{Full, BodyExt};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Result, Method, StatusCode};
use tokio::fs;
use tokio::net::TcpListener;

type Items = BTreeMap<i32, codec::Item>;

static ITEMS: Mutex<Items> = Mutex::new(BTreeMap::new());

struct TodoCodecImpl<'a> {
    items: &'a mut Items,
}

impl TodoCodec for TodoCodecImpl<'_> {
    fn create(self, name: String, desc: String) -> codec::Item {
        let id = self.items.last_key_value().map(|(&id, _)| id).unwrap_or_default() + 1;
        let item = codec::Item { id, name, desc, done: false };
        self.items.insert(id, item.clone());
        item
    }

    fn remove(self, id: i32) {
        self.items.remove(&id);
    }

    fn items(self) -> Vec<codec::Item> {
        self.items.values().cloned().collect()
    }

    fn set_done(self, id: i32, done: bool) {
        if let Some(item) = self.items.get_mut(&id) {
            item.done = done;
        }
    }
}

/// Returns an http error, with the given status code and message.
fn http_error(status: StatusCode, msg: &'static str) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .body(Full::new(msg.into()))
        .unwrap()
}

/// Handles our wasmql endpoint.
async fn wasmql(req: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
    // Collect body into bytes.
    let body = req.collect()
        .await?
        .to_bytes();

    let mut items_guard = ITEMS.lock().unwrap();

    let codec = TodoCodecImpl {
        items: &mut items_guard,
    };

    // Dispatch wasmql call and build a response.
    Ok(codec
        .dispatch(&body)
        .map(|res| Response::new(Full::new(res.into())))
        .unwrap_or_else(|_| http_error(StatusCode::BAD_REQUEST, "wasmql codec error")))
}

/// (Tries to) read a file with the given name and build an HTTP response from its content.
async fn file_send<P: AsRef<Path>>(filename: P) -> Response<Full<Bytes>> {
    // Get full name.
    let path = Path::new("dist")
        .join(filename);

    // Guess mime type from extension.
    let mime = match path.extension().and_then(|ext| ext.to_str()) {
        Some("js") => "text/javascript",
        Some("html") => "text/html",
        Some("wasm") => "application/wasm",
        _ => "",
    };
    
    // Fetch file and build response.
    fs::read(path)
        .await
        .map(|bytes| {
            let mut res = Response::new(Full::new(bytes.into()));
            res.headers_mut().insert("Content-Type", mime.parse().unwrap());
            res
        })
        .unwrap_or_else(|_| http_error(StatusCode::NOT_FOUND, "file not found"))
}

/// Handles an HTTP request.
async fn respond(req: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
    Ok(match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => file_send("index.html").await,
        (&Method::GET, path) => file_send(&path[1..]).await,
        (&Method::POST, "/wasmql") => wasmql(req).await?,
        _ => http_error(StatusCode::BAD_REQUEST, "endpoint not supported"),
    })
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Start listener.
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server started at http://127.0.0.1:8080");

    loop {
        // For each incoming stream, spawn a new task to handle it.
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