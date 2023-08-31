#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// use std::collections::BTreeMap;

extern crate alloc;
use alloc::string::String;
use casper_event_standard::Event;
use casper_event_standard::Schemas;
use common::helpers::current_block_timestamp;

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SetHash {
    height: u64,
    hash: String,
    timestamp: u64,
}
impl SetHash {
    pub fn new(height: u64, hash: String) -> Self {
        Self {
            height,
            hash,
            timestamp: current_block_timestamp(),
        }
    }
}

pub fn init_events() {
    let schemas = Schemas::new().with::<SetHash>();
    casper_event_standard::init(schemas);
}
