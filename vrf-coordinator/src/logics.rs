use core::convert::TryInto;

use alloc::{borrow::ToOwned, vec, vec::Vec};
use casper_contract::{
    contract_api::runtime::{self, revert},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{Bytes, FromBytes},
    CLValue, HashAddr, Key, U128, U256,
};
use common::{
    constants::{MAX_CONSUMERS, MAX_NUM_WORDS, MAX_REQUEST_CONFIRMATIONS},
    data_types::{Config, FeeConfig, Proof, RequestCommitment, Subscription, SubscriptionConfig},
    erc20_helpers,
    error::Error,
    helpers::{
        self, current_block_timestamp, get_immediate_caller_key, get_self_key, null_key,
        to_vec_string,
    },
    interfaces::{call_raw_fulfill_random_words, get_block_hash},
    lock::{lock_contract, unlock_contract, when_not_locked},
    owner::only_owner,
};

use crate::{
    checks::only_sub_owner,
    events::{self, ConfigSet},
    store::{
        self, read_block_hash_store, read_config, read_consumer, read_current_sub_id,
        read_fee_config, read_payment_token, read_proving_key, read_proving_key_hashes,
        read_request_commitment, read_subscription, read_subscription_config, read_total_balance,
        read_withdrawable_token, save_config, save_consumer, save_current_sub_id, save_fee_config,
        save_proving_key, save_proving_key_hashes, save_request_commitment, save_subscription,
        save_subscription_config, save_total_balance, save_withdrawable_token,
    },
    vrf,
};

pub fn initialize(payment_token: Key, block_hash_store: Key, price_feed: Key) {
    store::initialize(payment_token, block_hash_store, price_feed);
}

/**
 * @notice Registers a proving key to an oracle.
 * @param oracle address of the oracle
 * @param publicProvingKey key that oracle can use to submit vrf fulfillments
 */
#[no_mangle]
pub extern "C" fn register_proving_key() {
    only_owner();
    let (oracle, public_proving_key): (Key, Bytes) =
        helpers::get_named_args_2(to_vec_string(&["oracle", "public_proving_key"]));
    let kh = _hash_of_key(&public_proving_key.to_vec());
    if read_proving_key(&kh) != null_key() {
        revert(Error::ProvingKeyAlreadyRegistered);
    }
    save_proving_key(&kh, &oracle);
    let mut current_proving_key_hashes = read_proving_key_hashes();
    current_proving_key_hashes.push(kh);
    save_proving_key_hashes(current_proving_key_hashes);
    casper_event_standard::emit(events::ProvingKeyRegistered::new(hex::encode(kh), oracle));
}

#[no_mangle]
pub extern "C" fn deregister_proving_key() {
    only_owner();
    let public_proving_key: Bytes = runtime::get_named_arg("public_proving_key");
    let kh = _hash_of_key(&public_proving_key.to_vec());
    let oracle = read_proving_key(&kh);
    if oracle == null_key() {
        revert(Error::NoSuchProvingKey);
    }
    save_proving_key(&kh, &null_key());
    let mut current_proving_key_hashes = read_proving_key_hashes();
    let count = current_proving_key_hashes.len();
    for i in 0..count {
        if current_proving_key_hashes[i] == kh {
            let last = current_proving_key_hashes[current_proving_key_hashes.len() - 1];
            current_proving_key_hashes[i] = last;
            current_proving_key_hashes.pop();
            save_proving_key_hashes(current_proving_key_hashes);
            break;
        }
    }
    casper_event_standard::emit(events::ProvingKeyDeregistered::new(hex::encode(kh), oracle));
}

/**
 * @notice Sets the configuration of the vrfv2 coordinator
 * @param minimumRequestConfirmations global min for request confirmations
 * @param maxGasLimit global max for request gas limit
 * @param stalenessSeconds if the eth/link feed is more stale then this, use the fallback price
 * @param gasAfterPaymentCalculation gas used in doing accounting after completing the gas measurement
 * @param feeConfig fee tier configuration
 */
#[no_mangle]
pub extern "C" fn set_config() {
    only_owner();
    let (
        minimum_request_confirmations,
        max_gas_limit,
        staleness_seconds,
        gas_after_payment_calculation,
        fee_config_bytes,
    ) = helpers::get_named_args_5::<u64, U128, u64, U128, Bytes>(to_vec_string(&[
        "minimum_request_confirmations",
        "max_gas_limit",
        "staleness_seconds",
        "gas_after_payment_calculation",
        "fee_config_bytes",
    ]));
    if minimum_request_confirmations > MAX_REQUEST_CONFIRMATIONS {
        revert(Error::InvalidRequestConfirmations);
    }

    let fee_config = FeeConfig::from_bytes(&fee_config_bytes)
        .unwrap_or_revert_with(Error::FailedToDecodeInputBytes)
        .0;
    save_config(Config {
        minimum_request_confirmations,
        max_gas_limit,
        staleness_seconds,
        gas_after_payment_calculation,
    });
    save_fee_config(fee_config.clone());
    casper_event_standard::emit(ConfigSet::new(
        minimum_request_confirmations,
        max_gas_limit,
        staleness_seconds,
        gas_after_payment_calculation,
        fee_config,
    ));
}

/**
 * @notice Owner cancel subscription, sends remaining link directly to the subscription owner.
 * @param subId subscription id
 * @dev notably can be called even if there are pending requests, outstanding ones may fail onchain
 */
#[no_mangle]
pub extern "C" fn owner_cancel_subscription() {
    only_owner();
    let sub_id: u64 = runtime::get_named_arg("sub_id");
    let subscription_config = read_subscription_config(&sub_id);
    if subscription_config.owner == null_key() {
        revert(Error::InvalidSubscription);
    }
    _cancel_subscription_helper(sub_id, subscription_config.owner);
}

#[no_mangle]
pub extern "C" fn recover_funds() {
    only_owner();
    let to: Key = runtime::get_named_arg("to");
    let payment_token = read_payment_token();
    let external_balance = erc20_helpers::get_balance(payment_token, get_self_key());
    let internal_balance = read_total_balance();
    if internal_balance > external_balance {
        revert(Error::BalanceInvariantViolated);
    }
    if internal_balance < external_balance {
        let amount = external_balance - internal_balance;
        erc20_helpers::transfer(payment_token, to, amount);
        casper_event_standard::emit(events::FundsRecovered::new(to, amount));
    }
}

#[no_mangle]
pub extern "C" fn get_fee_tier() {
    let req_count: u64 = runtime::get_named_arg("req_count");
    runtime::ret(CLValue::from_t(_get_fee_tier(req_count)).unwrap_or_revert())
}

#[no_mangle]
pub extern "C" fn hash_of_key() {
    let public_proving_key: Bytes = runtime::get_named_arg("public_proving_key");
    runtime::ret(CLValue::from_t(_hash_of_key(&public_proving_key.to_vec())).unwrap_or_revert())
}

#[no_mangle]
pub extern "C" fn get_request_config() {
    let config = read_config();
    runtime::ret(
        CLValue::from_t((
            config.minimum_request_confirmations,
            config.max_gas_limit,
            read_proving_key_hashes(),
        ))
        .unwrap_or_revert(),
    )
}

#[no_mangle]
pub extern "C" fn get_commitment() {
    let request_id: U256 = runtime::get_named_arg("request_id");
    runtime::ret(CLValue::from_t(read_request_commitment(&request_id)).unwrap_or_revert())
}

#[no_mangle]
pub extern "C" fn request_random_words() {
    when_not_locked();
    lock_contract();
    let caller = get_immediate_caller_key();
    let (key_hash, sub_id, request_confirmations, callback_gas_limit, num_words) =
        helpers::get_named_args_5::<HashAddr, u64, u64, U128, u64>(to_vec_string(&[
            "key_hash",
            "sub_id",
            "request_confirmations",
            "callback_gas_limit",
            "num_words",
        ]));
    let subscription_config = read_subscription_config(&sub_id);
    if subscription_config.owner == null_key() {
        revert(Error::InvalidSubscription);
    }
    let current_nonce = read_consumer(&caller, &sub_id);
    if current_nonce == 0 {
        revert(Error::InvalidConsumer);
    }
    let config = read_config();
    if request_confirmations < config.minimum_request_confirmations
        || request_confirmations > MAX_REQUEST_CONFIRMATIONS
    {
        revert(Error::InvalidRequestConfirmations);
    }
    if callback_gas_limit > config.max_gas_limit {
        revert(Error::GasLimitTooBig);
    }

    if num_words > MAX_NUM_WORDS {
        revert(Error::NumWordsTooBig);
    }
    let nonce = current_nonce + 1;
    let (request_id, pre_seed) = _compute_request_id(key_hash, caller, sub_id, nonce);
    // save block timestamp instead of block height as there is no way to get block height from contract
    save_request_commitment(
        &request_id,
        &runtime::blake2b(helpers::encode_6(
            &request_id,
            &current_block_timestamp(),
            &sub_id,
            &callback_gas_limit,
            &num_words,
            &caller,
        )),
    );
    casper_event_standard::emit(events::RandomWordsRequested::new(
        key_hash,
        request_id,
        pre_seed,
        sub_id,
        request_confirmations,
        callback_gas_limit,
        num_words,
        caller,
    ));
    save_consumer(&caller, &sub_id, &nonce);

    unlock_contract();

    runtime::ret(CLValue::from_t(request_id).unwrap_or_revert())
}

#[no_mangle]
pub extern "C" fn fulfill_random_words() {
    when_not_locked();
    lock_contract();
    let proof: Bytes = runtime::get_named_arg("proof");
    let rc: Bytes = runtime::get_named_arg("rc");

    let proof = Proof::from_bytes(proof.as_slice()).unwrap().0;
    let rc = RequestCommitment::from_bytes(rc.as_slice()).unwrap().0;

    let (key_hash, request_id, randomness) = _get_randomness_from_proof(&proof, &rc);

    let mut random_words: Vec<U256> = vec![];
    for i in 0..rc.num_words {
        random_words.push(
            U256::from_bytes(&runtime::blake2b(helpers::encode_2(&randomness, &i)))
                .unwrap()
                .0,
        );
    }

    save_request_commitment(&request_id, &Default::default());

    let success = true;
    call_raw_fulfill_random_words(rc.sender, request_id, &random_words);

    // update after consumer call
    let mut subscription = read_subscription(&rc.sub_id);
    subscription.req_count += 1;

    // handle payment
    let payment = _calculate_payment_amount();

    if subscription.balance < payment {
        revert(Error::InsufficientBalance);
    }

    subscription.balance -= payment;
    let proving_key = read_proving_key(&key_hash);
    let mut withdrawnable_token = read_withdrawable_token(&proving_key);
    withdrawnable_token += payment;
    save_withdrawable_token(&proving_key, &withdrawnable_token);
    save_subscription(&rc.sub_id, &subscription);

    casper_event_standard::emit(events::RandomWordsFulfilled::new(
        request_id, randomness, payment, success,
    ));

    unlock_contract();

    runtime::ret(CLValue::from_t(payment).unwrap_or_revert())
}

#[no_mangle]
pub extern "C" fn deposit_token() {
    when_not_locked();
    lock_contract();

    let amount: U128 = runtime::get_named_arg("amount");
    let sub_id: u64 = runtime::get_named_arg("sub_id");
    let caller = helpers::get_immediate_caller_key();

    let subscription_config = read_subscription_config(&sub_id);
    if subscription_config.owner == null_key() {
        revert(Error::InvalidSubscription);
    }
    let mut subscription = read_subscription(&sub_id);
    let old_balance = subscription.balance;
    erc20_helpers::transfer_from(
        read_payment_token(),
        caller,
        helpers::get_self_key(),
        amount,
    );
    subscription.balance += amount;
    save_subscription(&sub_id, &subscription);
    save_total_balance(read_total_balance() + amount);

    casper_event_standard::emit(events::SubscriptionFunded::new(
        sub_id,
        old_balance,
        subscription.balance,
    ));
    unlock_contract();
}

#[no_mangle]
pub extern "C" fn create_subscription() {
    when_not_locked();
    lock_contract();

    let mut sub_id = read_current_sub_id();
    sub_id += 1;
    save_current_sub_id(sub_id);

    let consumers: Vec<Key> = vec![];

    save_subscription(
        &sub_id,
        &Subscription {
            balance: 0.into(),
            req_count: 0,
        },
    );

    save_subscription_config(
        &sub_id,
        &SubscriptionConfig {
            owner: helpers::get_immediate_caller_key(),
            requested_owner: null_key(),
            consumers,
        },
    );

    casper_event_standard::emit(events::SubscriptionCreated::new(
        sub_id,
        helpers::get_immediate_caller_key(),
    ));

    unlock_contract();

    runtime::ret(CLValue::from_t(sub_id).unwrap_or_revert())
}

#[no_mangle]
pub extern "C" fn request_subscription_owner_transfer() {
    let sub_id: u64 = runtime::get_named_arg("sub_id");
    let new_owner: Key = runtime::get_named_arg("new_owner");
    only_sub_owner(sub_id);
    when_not_locked();
    lock_contract();

    let mut sub_config = read_subscription_config(&sub_id);
    if sub_config.requested_owner != new_owner {
        sub_config.requested_owner = new_owner;
        casper_event_standard::emit(events::SubscriptionOwnerTransferRequested::new(
            sub_id,
            helpers::get_immediate_caller_key(),
            new_owner,
        ));
    }

    unlock_contract();
}

#[no_mangle]
pub extern "C" fn accept_subscription_owner_transfer() {
    let sub_id: u64 = runtime::get_named_arg("sub_id");
    let caller = helpers::get_immediate_caller_key();

    let mut sub_coinfig = read_subscription_config(&sub_id);
    if sub_coinfig.owner == null_key() {
        revert(Error::InvalidSubscription);
    }

    if sub_coinfig.requested_owner != caller {
        revert(Error::MustBeRequestedOwner);
    }

    let old_owner = sub_coinfig.owner;
    sub_coinfig.owner = caller;
    sub_coinfig.requested_owner = null_key();

    casper_event_standard::emit(events::SubscriptionOwnerTransferred::new(
        sub_id, old_owner, caller,
    ));
}

#[no_mangle]
pub extern "C" fn oracle_withdraw() {
    when_not_locked();
    lock_contract();

    let amount: U128 = runtime::get_named_arg("amount");
    let recipient: Key = runtime::get_named_arg("recipient");

    let caller = helpers::get_immediate_caller_key();
    let mut withdrawnable_token = read_withdrawable_token(&caller);
    if withdrawnable_token < amount {
        revert(Error::InsufficientBalance);
    }

    withdrawnable_token -= amount;
    save_total_balance(read_total_balance() - amount);

    erc20_helpers::transfer(read_payment_token(), recipient, amount);

    unlock_contract();
}

#[no_mangle]
pub extern "C" fn remove_consumer() {
    let sub_id: u64 = runtime::get_named_arg("sub_id");
    let consumer: Key = runtime::get_named_arg("consumer");

    only_sub_owner(sub_id);
    when_not_locked();
    lock_contract();

    if pending_request_exists(sub_id) {
        revert(Error::PendingRequestExists);
    }

    if read_consumer(&consumer, &sub_id) == 0 {
        revert(Error::InvalidConsumer);
    }

    let mut sub_config = read_subscription_config(&sub_id);
    let consumers: &mut Vec<Key> = &mut sub_config.consumers;

    for i in 0..consumers.len() {
        if consumers[i] == consumer {
            consumers.swap_remove(i);
            break;
        }
    }

    save_subscription_config(&sub_id, &sub_config);
    save_consumer(&consumer, &sub_id, &0);
    casper_event_standard::emit(events::SubscriptionConsumerRemoved::new(sub_id, consumer));

    unlock_contract();
}

#[no_mangle]
pub extern "C" fn add_consumer() {
    let sub_id: u64 = runtime::get_named_arg("sub_id");
    let consumer: Key = runtime::get_named_arg("consumer");

    only_sub_owner(sub_id);
    when_not_locked();
    lock_contract();

    let mut sub_config = read_subscription_config(&sub_id);
    let consumers: &mut Vec<Key> = &mut sub_config.consumers;

    if consumers.len() == MAX_CONSUMERS as usize {
        revert(Error::TooManyConsumers);
    }

    save_consumer(&consumer, &sub_id, &1);
    consumers.push(consumer);
    save_subscription_config(&sub_id, &sub_config);

    casper_event_standard::emit(events::SubscriptionConsumerAdded::new(sub_id, consumer));

    unlock_contract();
}

#[no_mangle]
pub extern "C" fn cancel_subscription() {
    let sub_id: u64 = runtime::get_named_arg("sub_id");
    let to: Key = runtime::get_named_arg("to");
    only_sub_owner(sub_id);
    when_not_locked();
    lock_contract();

    if pending_request_exists(sub_id) {
        revert(Error::PendingRequestExists);
    }

    _cancel_subscription_helper(sub_id, to);

    unlock_contract();
}

// internal
fn _hash_of_key(b: &Vec<u8>) -> [u8; 32] {
    runtime::blake2b(b)
}

fn _cancel_subscription_helper(sub_id: u64, to: Key) {
    let sub_config = read_subscription_config(&sub_id);
    let sub = read_subscription(&sub_id);
    let balance = sub.balance;
    let consumer_len = sub_config.consumers.len();
    for i in 0..consumer_len {
        save_consumer(&sub_config.consumers[i], &sub_id, &Default::default());
    }
    save_subscription_config(&sub_id, &Default::default());
    save_subscription(&sub_id, &Default::default());
    let total_balance = read_total_balance();
    save_total_balance(total_balance - balance);
    erc20_helpers::transfer(read_payment_token(), to, balance);
    casper_event_standard::emit(events::SubscriptionCanceled::new(sub_id, to, balance));
}

fn _compute_request_id(kh: HashAddr, sender: Key, sub_id: u64, nonce: u64) -> (U256, U256) {
    let pre_seed = runtime::blake2b(helpers::encode_4(&kh, &sender, &sub_id, &nonce));
    (
        U256::from_bytes(&runtime::blake2b(helpers::encode_2(&kh, &pre_seed)))
            .unwrap()
            .0,
        U256::from_bytes(&pre_seed).unwrap().0,
    )
}

fn _get_randomness_from_proof(proof: &Proof, rc: &RequestCommitment) -> (HashAddr, U256, U256) {
    let key_hash = _hash_of_key(&proof.pk.to_vec());
    let oracle = read_proving_key(&key_hash);
    if oracle == null_key() {
        revert(Error::NoSuchProvingKey);
    }
    let request_id = U256::from_bytes(&runtime::blake2b(helpers::encode_2(&key_hash, &proof.seed)))
        .unwrap()
        .0;
    let commitment = read_request_commitment(&request_id);
    if commitment == HashAddr::default() {
        revert(Error::NoCorrespondingRequest);
    }

    if commitment
        != runtime::blake2b(helpers::encode_6(
            &request_id,
            &rc.block_num,
            &rc.sub_id,
            &rc.callback_gas_limit,
            &rc.num_words,
            &rc.sender,
        ))
    {
        revert(Error::IncorrectCommitment);
    }

    let block_hash = get_block_hash(read_block_hash_store(), rc.block_num);
    if block_hash.is_empty() {
        revert(Error::BlockhashNotInStore);
    }

    let block_hash = hex::decode(block_hash).unwrap();
    let block_hash: HashAddr = block_hash.try_into().unwrap();
    let actual_seed = U256::from_bytes(&runtime::blake2b(helpers::encode_2(
        &proof.seed,
        &block_hash,
    )))
    .unwrap()
    .0;

    let randomness = vrf::random_value_from_vrf_proof(proof, actual_seed);
    (key_hash, request_id, randomness)
}

fn _get_fee_tier(req_count: u64) -> u64 {
    let req_count = req_count as u32;
    let fc = read_fee_config();
    if req_count <= fc.reqs_for_tier2 {
        fc.fulfillment_flat_fee_link_ppm_tier1
    } else if fc.reqs_for_tier2 < req_count && req_count <= fc.reqs_for_tier3 {
        fc.fulfillment_flat_fee_link_ppm_tier2
    } else if fc.reqs_for_tier3 < req_count && req_count <= fc.reqs_for_tier4 {
        fc.fulfillment_flat_fee_link_ppm_tier3
    } else if fc.reqs_for_tier4 < req_count && req_count <= fc.reqs_for_tier4 {
        fc.fulfillment_flat_fee_link_ppm_tier4
    } else {
        fc.fulfillment_flat_fee_link_ppm_tier5
    }
}

fn _calculate_payment_amount() -> U128 {
    // TODO: compute payment amount
    0.into()
}

pub fn pending_request_exists(sub_id: u64) -> bool {
    let sub_config = read_subscription_config(&sub_id);
    let proving_key_hashes = read_proving_key_hashes();
    for i in 0..sub_config.consumers.len() {
        for item in &proving_key_hashes {
            let (req_id, _) = _compute_request_id(
                item.to_owned(),
                sub_config.consumers[i],
                sub_id,
                read_consumer(&sub_config.consumers[i], &sub_id),
            );

            if read_request_commitment(&req_id) != HashAddr::default() {
                return true;
            }
        }
    }
    false
}
