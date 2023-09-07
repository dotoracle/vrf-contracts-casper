#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;

use alloc::{string::String, vec};
use casper_contract::{
    self,
    contract_api::{runtime, storage},
};
use casper_types::bytesrepr::FromBytes;
use casper_types::{
    bytesrepr::ToBytes, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key,
    RuntimeArgs,
};
use common::{data_types::Config, helpers};
const TEST_SESSION: &str = "test_session";

fn store_result_with_name<T: CLTyped + ToBytes>(result: T, name: String) {
    match runtime::get_key(&name) {
        Some(Key::URef(uref)) => storage::write(uref, result),
        Some(_) => unreachable!(),
        None => {
            let new_uref = storage::new_uref(result);
            runtime::put_key(&name, new_uref.into());
        }
    }
}

#[no_mangle]
extern "C" fn get_any_data() {
    let package_hash: Key = runtime::get_named_arg("contract_package_hash");
    let func: String = runtime::get_named_arg("func");
    let return_type_name: String = runtime::get_named_arg("return_type_name");
    let args: String = runtime::get_named_arg("args");
    let args = hex::decode(args).unwrap();
    let args = RuntimeArgs::from_bytes(&args).unwrap().0;
    helpers::log_msg("calling function");
    helpers::log_msg(&func);
    if return_type_name == "Config" {
        let b: Config = runtime::call_versioned_contract(
            package_hash.into_hash().unwrap().into(),
            None,
            &func,
            args,
        );

        store_result_with_name(b, return_type_name);
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();

    let get_any_data = EntryPoint::new(
        String::from("get_any_data"),
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    entry_points.add_entry_point(get_any_data);

    let (_contract_hash, _version) = storage::new_contract(
        entry_points,
        None,
        Some(format!("{}_package_hash", TEST_SESSION)),
        None,
    );
}
