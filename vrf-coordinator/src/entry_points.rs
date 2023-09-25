use alloc::{boxed::Box, string::String, vec, vec::Vec};
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
        String::from("get_payment_token"),
        vec![],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("set_payment_token"),
        vec![Parameter::new("payment_token", CLType::Key)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_price_feed"),
        vec![],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("set_price_feed"),
        vec![Parameter::new("price_feed", CLType::Key)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_block_hash_store"),
        vec![],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("set_block_hash_store"),
        vec![Parameter::new("block_hash_store", CLType::Key)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_consumer"),
        vec![
            Parameter::new("consumer", CLType::Key),
            Parameter::new("sub_id", CLType::U64),
        ],
        CLType::U64,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_subscription_config"),
        vec![Parameter::new("sub_id", CLType::U64)],
        CLType::Any,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_subscription"),
        vec![Parameter::new("sub_id", CLType::U64)],
        CLType::Any,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_subscription_view"),
        vec![Parameter::new("sub_id", CLType::U64)],
        CLType::Any,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_current_sub_id"),
        vec![],
        CLType::U64,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_total_balance"),
        vec![],
        CLType::U128,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_proving_key"),
        vec![],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_proving_key_hashes"),
        vec![],
        CLType::Any,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_withdrawable_token"),
        vec![Parameter::new("oracle", CLType::Key)],
        CLType::U128,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_request_commitment"),
        vec![Parameter::new("request_id", CLType::U256)],
        CLType::Any,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_config"),
        vec![],
        CLType::Any,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_fee_config"),
        vec![],
        CLType::Any,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("register_proving_key"),
        vec![
            Parameter::new("oracle", CLType::Key),
            Parameter::new("public_proving_key", CLType::List(Box::new(CLType::U8))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("deregister_proving_key"),
        vec![Parameter::new(
            "public_proving_key",
            CLType::List(Box::new(CLType::U8)),
        )],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("set_config"),
        vec![
            Parameter::new("minimum_request_confirmations", CLType::U64),
            Parameter::new("max_gas_limit", CLType::U128),
            Parameter::new("staleness_seconds", CLType::U64),
            Parameter::new("gas_after_payment_calculation", CLType::U128),
            Parameter::new("fee_config_bytes", CLType::List(Box::new(CLType::U8))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("owner_cancel_subscription"),
        vec![Parameter::new("sub_id", CLType::U64)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("recover_funds"),
        vec![Parameter::new("to", CLType::Key)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_fee_tier"),
        vec![Parameter::new("req_count", CLType::U64)],
        CLType::U64,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_request_config"),
        vec![],
        CLType::Any,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("get_commitment"),
        vec![Parameter::new("request_id", CLType::U256)],
        CLType::Any,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("request_random_words"),
        vec![
            Parameter::new("key_hash", CLType::Key),
            Parameter::new("sub_id", CLType::U64),
            Parameter::new("request_confirmations", CLType::U64),
            Parameter::new("callback_gas_limit", CLType::U128),
            Parameter::new("num_words", CLType::U64),
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("fulfill_random_words"),
        vec![
            Parameter::new("proof", CLType::List(Box::new(CLType::U8))),
            Parameter::new("rc", CLType::List(Box::new(CLType::U8))),
        ],
        CLType::U128,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("hash_of_key"),
        vec![
            Parameter::new("public_proving_key", CLType::List(Box::new(CLType::U8))),
        ],
        CLType::ByteArray(32),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("deposit_token"),
        vec![
            Parameter::new("amount", CLType::U128),
            Parameter::new("sub_id", CLType::U64),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("create_subscription"),
        vec![],
        CLType::U64,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("request_subscription_owner_transfer"),
        vec![
            Parameter::new("new_owner", CLType::Key),
            Parameter::new("sub_id", CLType::U64),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("accept_subscription_owner_transfer"),
        vec![Parameter::new("sub_id", CLType::U64)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("oracle_withdraw"),
        vec![
            Parameter::new("amount", CLType::U128),
            Parameter::new("recipient", CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("remove_consumer"),
        vec![
            Parameter::new("sub_id", CLType::U64),
            Parameter::new("consumer", CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("add_consumer"),
        vec![
            Parameter::new("sub_id", CLType::U64),
            Parameter::new("consumer", CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("cancel_subscription"),
        vec![
            Parameter::new("sub_id", CLType::U64),
            Parameter::new("to", CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        String::from("init"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
