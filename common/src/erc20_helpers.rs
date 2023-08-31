use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, Key, RuntimeArgs, U128, U256};
pub fn get_total_supply(contract: Key) -> U128 {
    let total_supply: U256 = runtime::call_versioned_contract(
        contract.into_hash().unwrap().into(),
        None,
        "total_supply",
        runtime_args! {},
    );
    total_supply.as_u128().into()
}

pub fn get_balance(token: Key, user: Key) -> U128 {
    let b: U256 = runtime::call_versioned_contract(
        token.into_hash().unwrap().into(),
        Some(1u32),
        "balance_of",
        runtime_args! {
            "address" => user
        },
    );
    U128::from(b.as_u128())
}

pub fn get_decimals(token: Key) -> u8 {
    let d: u8 = runtime::call_versioned_contract(
        token.into_hash().unwrap().into(),
        None,
        "decimals",
        runtime_args! {},
    );
    d
}

pub fn transfer(token: Key, recipient: Key, amount: U128) {
    let _: () = runtime::call_versioned_contract(
        token.into_hash().unwrap().into(),
        None,
        "transfer",
        runtime_args! {
            "recipient" => recipient,
            "amount" => U256::from(amount.as_u128())
        },
    );
}

pub fn transfer_from(token: Key, from: Key, recipient: Key, amount: U128) {
    let _: () = runtime::call_versioned_contract(
        token.into_hash().unwrap().into(),
        None,
        "transfer_from",
        runtime_args! {
            "owner" => from,
            "recipient" => recipient,
            "amount" => U256::from(amount.as_u128())
        },
    );
}

pub fn approve(token: Key, spender: Key, amount: U128) {
    let _: () = runtime::call_versioned_contract(
        token.into_hash().unwrap().into(),
        None,
        "approve",
        runtime_args! {
            "spender" => spender,
            "amount" => U256::from(amount.as_u128())
        },
    );
}
