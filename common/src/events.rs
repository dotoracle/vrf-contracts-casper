use casper_event_standard::Event;
use casper_types::{HashAddr, U256, U128, Key};

use crate::helpers::current_block_timestamp;

#[derive(Event, Debug, PartialEq, Eq)]
pub struct RandomWordsRequested {
    pub key_hash: HashAddr,
    pub request_id: U256,
    pub pre_seed: U256,
    pub sub_id: u64,
    pub minimum_request_confirmations: u64,
    pub callback_cas_limit: U128,
    pub num_words: u64,
    pub sender: Key,
    pub timestamp: u64,
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