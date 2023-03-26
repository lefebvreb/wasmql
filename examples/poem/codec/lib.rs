#![no_std]

use wasmql::prelude::*;

#[wasmql::data]
#[derive(Debug, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub done: bool,
}

#[wasmql::data]
#[derive(Debug)]
pub struct CreateItem {
    pub name: String,
    pub desc: String,
}

#[wasmql::codec]
pub trait TodoCodec {
    fn items(self) -> Vec<Item>; 

    fn create_item(self, name: String, desc: String) -> Item;

    fn mark_done(self, id: i32);
}
