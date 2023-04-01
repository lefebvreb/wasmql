use std::collections::BTreeMap;
use std::io;
use std::sync::{Arc, Mutex};

use codec::TodoCodec;
use poem::endpoint::StaticFilesEndpoint;
use poem::http::StatusCode;
use poem::listener::TcpListener;
use poem::web::Data;
use poem::{post, EndpointExt, Error, Route, Server};

pub type Items = BTreeMap<i32, codec::Item>;

struct MyCodec<'a> {
    items: &'a mut Items,
}

impl TodoCodec for MyCodec<'_> {
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

#[poem::handler]
async fn wasmql(items: Data<&Arc<Mutex<Items>>>, bytes: Vec<u8>) -> poem::Result<Vec<u8>> {
    let mut items = items.lock().expect("mutex was poisoned");

    let codec = MyCodec { items: &mut items };

    codec
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
        )
        .data(Arc::new(Mutex::new(Items::default())));

    println!("Server started at http://127.0.0.1:8080");

    Server::new(listener).run(app).await?;

    Ok(())
}
