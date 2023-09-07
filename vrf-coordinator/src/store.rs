use alloc::{vec, vec::Vec};
use casper_contract::contract_api::{runtime, storage};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{CLValue, HashAddr, Key, U128, U256};
use common::error::Error;
use common::{
    data_types::{Config, FeeConfig, Subscription, SubscriptionConfig},
    get_set, get_set_dict, get_set_nested_dict, get_set_no_set, helpers,
};

pub fn initialize(payment_token: Key, block_hash_store: Key, price_feed: Key) {
    save_payment_token(payment_token);
    save_price_feed(price_feed);
    save_block_hash_store(block_hash_store);
    storage::new_dictionary("consumers").unwrap_or_revert_with(Error::FailedToCreateDictionary);
    storage::new_dictionary("subscription_configs")
        .unwrap_or_revert_with(Error::FailedToCreateDictionary);
    storage::new_dictionary("subscriptions").unwrap_or_revert_with(Error::FailedToCreateDictionary);
    save_current_sub_id(0);
    save_total_balance(0.into());
    storage::new_dictionary("proving_keys").unwrap_or_revert_with(Error::FailedToCreateDictionary);
    save_proving_key_hashes(vec![]);
    storage::new_dictionary("withdrawable_tokens")
        .unwrap_or_revert_with(Error::FailedToCreateDictionary);
    storage::new_dictionary("request_commitments")
        .unwrap_or_revert_with(Error::FailedToCreateDictionary);
    save_config(Default::default());
    save_fee_config(Default::default());
}

get_set!(
    payment_token,
    "payment_token",
    Key,
    helpers::null_key(),
    save_payment_token,
    read_payment_token,
    get_payment_token,
    set_payment_token
);
get_set!(
    price_feed,
    "price_feed",
    Key,
    helpers::null_key(),
    save_price_feed,
    read_price_feed,
    get_price_feed,
    set_price_feed
);
get_set!(
    block_hash_store,
    "block_hash_store",
    Key,
    helpers::null_key(),
    save_block_hash_store,
    read_block_hash_store,
    get_block_hash_store,
    set_block_hash_store
);

get_set_nested_dict!(
    "consumers",
    "consumer",
    "sub_id",
    Key,
    u64,
    u64,
    0,
    save_consumer,
    read_consumer,
    get_consumer,
    set_consumer
);

get_set_dict!(
    "subscription_configs",
    "sub_id",
    u64,
    SubscriptionConfig,
    SubscriptionConfig::default(),
    save_subscription_config,
    read_subscription_config,
    get_subscription_config,
    set_subscription_config
);

get_set_dict!(
    "subscriptions",
    "sub_id",
    u64,
    Subscription,
    Subscription::default(),
    save_subscription,
    read_subscription,
    get_subscription,
    set_subscription
);

get_set_no_set!(
    current_sub_id,
    "current_sub_id",
    u64,
    0,
    save_current_sub_id,
    read_current_sub_id,
    get_current_sub_id
);

get_set_no_set!(
    total_balance,
    "total_balance",
    U128,
    U128::zero(),
    save_total_balance,
    read_total_balance,
    get_total_balance
);

get_set_dict!(
    "proving_keys",
    "key_hash",
    HashAddr,
    Key,
    helpers::null_key(),
    save_proving_key,
    read_proving_key,
    get_proving_key,
    set_proving_key
);

get_set_no_set!(
    proving_key_hashes,
    "proving_key_hashes",
    Vec<HashAddr>,
    vec![],
    save_proving_key_hashes,
    read_proving_key_hashes,
    get_proving_key_hashes
);

get_set_dict!(
    "withdrawable_tokens",
    "oracle",
    Key,
    U128,
    U128::zero(),
    save_withdrawable_token,
    read_withdrawable_token,
    get_withdrawable_token,
    set_withdrawable_token
);

get_set_dict!(
    "request_commitments",
    "request_id",
    U256,
    HashAddr,
    HashAddr::default(),
    save_request_commitment,
    read_request_commitment,
    get_request_commitment,
    set_request_commitment
);

get_set_no_set!(
    config,
    "config",
    Config,
    Config::default(),
    save_config,
    read_config,
    get_config
);

get_set_no_set!(
    fee_config,
    "fee_config",
    FeeConfig,
    FeeConfig::default(),
    save_fee_config,
    read_fee_config,
    get_fee_config
);
