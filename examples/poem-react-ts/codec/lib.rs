#![no_std]

use wasmql::prelude::*;

#[wasmql::data]
#[derive(Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub done: bool,
}

#[wasmql::data]
pub struct CreateItem {
    pub name: String,
    pub desc: String,
}

#[wasmql::codec]
pub trait TodoCodec {
    fn create(self, name: String, desc: String) -> Item;

    fn remove(self, id: i32);

    fn items(self) -> Vec<Item>;

    fn set_done(self, id: i32, done: bool);
}
