use alloc::{string::String, vec, vec::Vec};
use casper_types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter};

use common::{owner, timestamp_testing};

fn add_entry_points(entry_points: &mut EntryPoints, list: &Vec<EntryPoint>) {
    for e in list {
        entry_points.add_entry_point(e.clone());
    }
}

pub(crate) fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    add_entry_points(&mut entry_points, &timestamp_testing::entry_points());
    add_entry_points(&mut entry_points, &owner::entry_points());

    // view functions
    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_block_hash"),
        vec![Parameter::new("height", CLType::U64)],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("set_block_hash"),
        vec![
            Parameter::new("height", CLType::U64),
            Parameter::new("hash", CLType::String),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("init"),
        vec![
            Parameter::new("contract_hash", CLType::Key),
            Parameter::new("contract_package_hash", CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}
