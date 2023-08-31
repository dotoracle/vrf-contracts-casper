use alloc::{borrow::ToOwned, string::String, vec::Vec};
use casper_contract::contract_api::runtime::call_versioned_contract;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};

pub fn set_block_hash(contract_package: Key, height: u64, hash: String) {
    call_versioned_contract::<()>(
        contract_package.into_hash().unwrap().into(),
        None,
        "set_block_hash",
        runtime_args! {
            "height" =>  height,
            "hash" => hash
        },
    );
}

pub fn get_block_hash(contract_package: Key, height: u64) -> String {
    call_versioned_contract::<String>(
        contract_package.into_hash().unwrap().into(),
        None,
        "get_block_hash",
        runtime_args! {
            "height" =>  height,
        },
    )
}

pub fn call_raw_fulfill_random_words(consumer: Key, request_id: U256, random_words: &Vec<U256>) {
    call_versioned_contract::<()>(
        consumer.into_hash().unwrap().into(),
        None,
        "raw_fulfill_random_words",
        runtime_args! {
            "request_id" => request_id,
            "random_words" => random_words.to_owned()
        },
    );
}
