#[wasmql::data]
pub struct Item {
    id: u128,
    name: String,
    done: bool,
}

#[wasmql::api]
pub trait TodoApi {
    fn items() -> Vec<Item>;

    fn create_item(item_name: String) -> Item;

    fn mark_done(id: u128) -> Result<(), String>;
}
