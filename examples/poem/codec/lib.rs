#![no_std]

use wasmql::prelude::*;

#[wasmql::data]
pub struct Item {
    id: u128,
    name: String,
    desc: String,
    done: bool,
}

#[wasmql::data]
pub struct CreateItem {
    name: String,
    desc: String,
}

#[wasmql::api]
pub trait TodoApi {
    fn items(self) -> Vec<Item>;

    fn create_item(self, name: String, data: String) -> Item;

    fn mark_done(self, id: u128);
}
