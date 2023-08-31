use alloc::vec::Vec;
use casper_types::{bytesrepr::Bytes, Key, U128, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::helpers;

#[derive(Serialize, Deserialize, Clone, CLTyped, ToBytes, FromBytes, Default)]
pub struct Subscription {
    pub balance: U128,
    pub req_count: u64,
}

#[derive(Serialize, Deserialize, Clone, CLTyped, ToBytes, FromBytes)]
pub struct SubscriptionConfig {
    pub owner: Key,
    pub requested_owner: Key,
    pub consumers: Vec<Key>,
}

impl Default for SubscriptionConfig {
    fn default() -> Self {
        Self {
            owner: helpers::null_key(),
            requested_owner: helpers::null_key(),
            consumers: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, CLTyped, ToBytes, FromBytes)]
pub struct RequestCommitment {
    pub block_num: u64,
    pub sub_id: u64,
    pub callback_gas_limit: U128,
    pub num_words: u32,
    pub sender: Key,
}

impl Default for RequestCommitment {
    fn default() -> Self {
        Self {
            sender: helpers::null_key(),
            block_num: 0,
            sub_id: 0,
            callback_gas_limit: 0.into(),
            num_words: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, CLTyped, ToBytes, FromBytes, Default)]
pub struct Config {
    pub minimum_request_confirmations: u64,
    pub max_gas_limit: U128,
    pub staleness_seconds: u64,
    pub gas_after_payment_calculation: U128,
}

#[derive(Serialize, Deserialize, Clone, CLTyped, ToBytes, FromBytes, Default)]
pub struct FeeConfig {
    pub fulfillment_flat_fee_link_ppm_tier1: u64,
    pub fulfillment_flat_fee_link_ppm_tier2: u64,
    pub fulfillment_flat_fee_link_ppm_tier3: u64,
    pub fulfillment_flat_fee_link_ppm_tier4: u64,
    pub fulfillment_flat_fee_link_ppm_tier5: u64,
    pub reqs_for_tier2: u32,
    pub reqs_for_tier3: u32,
    pub reqs_for_tier4: u32,
    pub reqs_for_tier5: u32,
}

#[derive(Serialize, Deserialize, Clone, CLTyped, ToBytes, FromBytes)]
pub struct Proof {
    pub pk: Bytes,
    pub gamma: Bytes,
    pub c: U256,
    pub s: U256,
    pub seed: U256,
    pub u_witness: Key,
    pub c_gamma_witness: Bytes,
    pub s_hash_witness: Bytes,
    pub z_inv: U256,
}
