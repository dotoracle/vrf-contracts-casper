use crate::constants::*;
use crate::error::Error;
use crate::helpers::{self, *};
use crate::owner::*;
use alloc::{string::String, vec, vec::*};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{CLType, CLValue, EntryPoint, EntryPointAccess, EntryPointType, Parameter};
#[no_mangle]
pub extern "C" fn get_paused() {
    runtime::ret(CLValue::from_t(get_paused_internal()).unwrap_or_revert());
}

pub fn get_paused_internal() -> bool {
    let paused: bool = helpers::get_key(PAUSED).unwrap();
    paused
}

#[no_mangle]
pub extern "C" fn set_paused() {
    only_owner();
    let paused: bool = runtime::get_named_arg(PAUSED);
    helpers::set_key(PAUSED, paused);
}

pub fn when_not_paused() {
    require(!get_paused_internal(), Error::ContractPaused);
}

pub fn init() {
    runtime::put_key(PAUSED, storage::new_uref(false).into());
}

pub fn entry_points() -> Vec<EntryPoint> {
    vec![EntryPoint::new(
        String::from("set_paused"),
        vec![Parameter::new(PAUSED, CLType::Bool)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )]
}
