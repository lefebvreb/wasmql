#[wasmql::data]
pub struct Item {
    id: u128,
    name: String,
    data: String,
    done: bool,
}

#[wasmql::data]
pub struct CreateItem {
    name: String,
    data: String,
}

#[wasmql::api]
pub trait TodoApi {
    fn items(self, _request: ()) -> Vec<Item>;

    fn create_item(self, item: CreateItem) -> Item;

    fn mark_done(self, id: u128) -> Result<(), String>;
}
