#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// use std::collections::BTreeMap;

extern crate alloc;
use alloc::string::String;
use casper_event_standard::Event;
use casper_event_standard::Schemas;
use casper_types::bytesrepr::Bytes;
use casper_types::bytesrepr::ToBytes;
use casper_types::HashAddr;
use casper_types::Key;
use casper_types::U128;
use casper_types::U256;
use common::data_types::FeeConfig;
use common::helpers::current_block_timestamp;

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SubscriptionCreated {
    sub_id: u64,
    owner: Key,
    timestamp: u64,
}
impl SubscriptionCreated {
    pub fn new(sub_id: u64, owner: Key) -> Self {
        Self {
            sub_id,
            owner,
            timestamp: current_block_timestamp(),
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SubscriptionFunded {
    sub_id: u64,
    old_balance: U128,
    new_balance: U128,
    timestamp: u64,
}
impl SubscriptionFunded {
    pub fn new(sub_id: u64, old_balance: U128, new_balance: U128) -> Self {
        Self {
            sub_id,
            old_balance,
            new_balance,
            timestamp: current_block_timestamp(),
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SubscriptionConsumerAdded {
    sub_id: u64,
    consumer: Key,
    timestamp: u64,
}
impl SubscriptionConsumerAdded {
    pub fn new(sub_id: u64, consumer: Key) -> Self {
        Self {
            sub_id,
            consumer,
            timestamp: current_block_timestamp(),
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SubscriptionConsumerRemoved {
    sub_id: u64,
    consumer: Key,
    timestamp: u64,
}
impl SubscriptionConsumerRemoved {
    pub fn new(sub_id: u64, consumer: Key) -> Self {
        Self {
            sub_id,
            consumer,
            timestamp: current_block_timestamp(),
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SubscriptionCanceled {
    sub_id: u64,
    to: Key,
    amount: U128,
    timestamp: u64,
}
impl SubscriptionCanceled {
    pub fn new(sub_id: u64, to: Key, amount: U128) -> Self {
        Self {
            sub_id,
            to,
            amount,
            timestamp: current_block_timestamp(),
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SubscriptionOwnerTransferRequested {
    sub_id: u64,
    from: Key,
    to: Key,
    timestamp: u64,
}
impl SubscriptionOwnerTransferRequested {
    pub fn new(sub_id: u64, from: Key, to: Key) -> Self {
        Self {
            sub_id,
            to,
            from,
            timestamp: current_block_timestamp(),
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SubscriptionOwnerTransferred {
    sub_id: u64,
    from: Key,
    to: Key,
    timestamp: u64,
}
impl SubscriptionOwnerTransferred {
    pub fn new(sub_id: u64, from: Key, to: Key) -> Self {
        Self {
            sub_id,
            to,
            from,
            timestamp: current_block_timestamp(),
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ProvingKeyRegistered {
    key_hash: String,
    oracle: Key,
    timestamp: u64,
}
impl ProvingKeyRegistered {
    pub fn new(key_hash: String, oracle: Key) -> Self {
        Self {
            key_hash,
            oracle,
            timestamp: current_block_timestamp(),
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ProvingKeyDeregistered {
    key_hash: String,
    oracle: Key,
    timestamp: u64,
}
impl ProvingKeyDeregistered {
    pub fn new(key_hash: String, oracle: Key) -> Self {
        Self {
            key_hash,
            oracle,
            timestamp: current_block_timestamp(),
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct RandomWordsRequested {
    key_hash: HashAddr,
    request_id: U256,
    pre_seed: U256,
    sub_id: u64,
    minimum_request_confirmations: u64,
    callback_cas_limit: U128,
    num_words: u64,
    sender: Key,
    timestamp: u64,
}
#[allow(clippy::too_many_arguments)]
impl RandomWordsRequested {
    pub fn new(
        key_hash: HashAddr,
        request_id: U256,
        pre_seed: U256,
        sub_id: u64,
        minimum_request_confirmations: u64,
        callback_cas_limit: U128,
        num_words: u64,
        sender: Key,
    ) -> Self {
        Self {
            key_hash,
            request_id,
            pre_seed,
            sub_id,
            minimum_request_confirmations,
            callback_cas_limit,
            num_words,
            sender,
            timestamp: current_block_timestamp(),
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct RandomWordsFulfilled {
    request_id: U256,
    output_seed: U256,
    payment: U128,
    success: bool,
    timestamp: u64,
}
impl RandomWordsFulfilled {
    pub fn new(request_id: U256, output_seed: U256, payment: U128, success: bool) -> Self {
        Self {
            request_id,
            output_seed,
            payment,
            success,
            timestamp: current_block_timestamp(),
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct ConfigSet {
    minimum_request_confirmations: u64,
    max_gas_limit: U128,
    stateless_seconds: u64,
    gas_after_payment_calculation: U128,
    fee_config: Bytes,
    timestamp: u64,
}
impl ConfigSet {
    pub fn new(
        minimum_request_confirmations: u64,
        max_gas_limit: U128,
        stateless_seconds: u64,
        gas_after_payment_calculation: U128,
        fee_config: FeeConfig,
    ) -> Self {
        Self {
            minimum_request_confirmations,
            max_gas_limit,
            stateless_seconds,
            gas_after_payment_calculation,
            fee_config: fee_config.to_bytes().unwrap().into_bytes().unwrap().into(),
            timestamp: current_block_timestamp(),
        }
    }
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct FundsRecovered {
    to: Key,
    amount: U128,
    timestamp: u64,
}
impl FundsRecovered {
    pub fn new(to: Key, amount: U128) -> Self {
        Self {
            to,
            amount,
            timestamp: current_block_timestamp(),
        }
    }
}

pub fn init_events() {
    let schemas = Schemas::new()
        .with::<SubscriptionCreated>()
        .with::<SubscriptionConsumerAdded>()
        .with::<SubscriptionConsumerRemoved>()
        .with::<SubscriptionCanceled>()
        .with::<SubscriptionOwnerTransferRequested>()
        .with::<SubscriptionOwnerTransferred>()
        .with::<ProvingKeyRegistered>()
        .with::<ProvingKeyDeregistered>()
        .with::<RandomWordsRequested>()
        .with::<RandomWordsFulfilled>()
        .with::<ConfigSet>()
        .with::<FundsRecovered>()
        .with::<SubscriptionFunded>();
    casper_event_standard::init(schemas);
}
