use casper_contract::contract_api::runtime::revert;
use common::{
    error::Error,
    helpers::{self, null_key},
};

use crate::store::read_subscription_config;

pub fn only_sub_owner(sub_id: u64) {
    let owner = read_subscription_config(&sub_id).owner;
    if owner == null_key() {
        revert(Error::InvalidSubscription);
    }

    if helpers::get_immediate_caller_key() != owner {
        revert(Error::MustBeSubOwner);
    }
}
