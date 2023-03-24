use std::io;
use std::sync::{Arc, Mutex};

use codec::TodoCodec;
use poem::endpoint::StaticFilesEndpoint;
use poem::http::StatusCode;
use poem::listener::TcpListener;
use poem::web::Data;
use poem::{post, Route, Server, Error, EndpointExt};

pub type Items = Vec<(i32, codec::Item)>;

struct MyCodec<'a> {
    items: &'a mut Items,
}

impl TodoCodec for MyCodec<'_> {
    fn items(self) -> Vec<codec::Item>  {
        self.items.iter().map(|(_, item)| item).cloned().collect()
    }

    fn create_item(self, name: String, desc: String) -> codec::Item {
        let item = codec::Item {
            id: self.items.len() as i32,
            name,
            desc,
            done: false,
        };

        self.items.push((item.id, item.clone()));

        item
    }

    fn mark_done(self, id: i32) {
        self.items.iter_mut()
            .find(|(i, _)| *i == id)
            .map(|(_, item)| item.done = true);
    }
}

#[poem::handler]
async fn wasmql(items: Data<&Arc<Mutex<Items>>>, bytes: Vec<u8>) -> poem::Result<Vec<u8>> {
    let mut guard = items.lock().expect("mutex was poisoned");

    let codec = MyCodec {
        items: &mut guard,
    };

    codec.__dispatch(&bytes)
        .map_err(|_| Error::from_status(StatusCode::BAD_REQUEST))
}

#[tokio::main]
pub async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080");

    let app = Route::new()
        .at("/wasmql", post(wasmql))
        .nest("/", StaticFilesEndpoint::new("dist").index_file("index.html"))
        .data(Arc::new(Mutex::new(Items::default())));

    println!("Server started at http://127.0.0.1:8080");

    Server::new(listener).run(app).await?;

    Ok(())
}
