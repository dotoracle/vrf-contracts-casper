use alloc::string::{String, ToString};
use casper_contract::contract_api::{runtime, storage};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::CLValue;
use common::error::Error;
use common::helpers;

use crate::events::SetHash;

pub fn initialize() {
    storage::new_dictionary("block_hashes").unwrap_or_revert_with(Error::FailedToCreateDictionary);
}

// SETTER & GETTER
#[no_mangle]
pub extern "C" fn get_block_hash() {
    let height: u64 = runtime::get_named_arg("height");
    runtime::ret(CLValue::from_t(read_block_hash(height)).unwrap_or_revert())
}

#[no_mangle]
pub extern "C" fn set_block_hash() {
    common::owner::only_owner();
    let height: u64 = runtime::get_named_arg("height");
    let hash: String = runtime::get_named_arg("hash");
    save_block_hash(height, hash.clone());
    casper_event_standard::emit(SetHash::new(height, hash));
}

pub fn read_block_hash(height: u64) -> String {
    helpers::get_dictionary_value_from_key("block_hashes", &height.to_string()).unwrap_or_default()
}

pub fn save_block_hash(height: u64, hash: String) {
    helpers::write_dictionary_value_from_key("block_hashes", &height.to_string(), hash);
}
