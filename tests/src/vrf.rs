use casper_types::{
    account::AccountHash,
    bytesrepr::{Bytes, ToBytes},
    runtime_args, Key, RuntimeArgs, SECP256K1_TAG, U128, U256,
};
use common::{
    data_types::{Config, FeeConfig, SubscriptionView},
    helpers::null_key,
};
use test_env::env::TestEnv;

use crate::utils::{self, key_to_contract_package_hash};

pub struct VRFFixture {
    test_env: TestEnv,
    vrf: Key,
    payment_token: Key,
    block_hash_store: Key,
    owner: AccountHash,
    sub_owner: AccountHash,
    sub_owner_address: Key,
    consumer: AccountHash,
    random: AccountHash,
    random_address: Key,
    oracle: AccountHash,
    config: Config,
}

impl VRFFixture {
    #[allow(clippy::too_many_arguments)]
    pub fn set_config(
        &mut self,
        caller: Option<AccountHash>,
        minimum_request_confirmations: u64,
        max_gas_limit: U128,
        staleness_seconds: u64,
        gas_after_payment_calculation: U128,
        fee_config_bytes: Bytes,
        success: bool,
    ) {
        self.test_env.call_contract(
            caller,
            utils::key_to_contract_package_hash(self.vrf),
            "set_config",
            runtime_args! {
                "minimum_request_confirmations" => minimum_request_confirmations,
                "max_gas_limit" => max_gas_limit,
                "staleness_seconds" => staleness_seconds,
                "gas_after_payment_calculation" => gas_after_payment_calculation,
                "fee_config_bytes" => fee_config_bytes
            },
            success,
        );
    }

    pub fn get_config(&mut self) -> Config {
        self.test_env
            .call_view_function(self.vrf, "get_config", runtime_args! {})
    }

    pub fn get_fee_config(&mut self) -> FeeConfig {
        self.test_env
            .call_view_function(self.vrf, "get_fee_config", runtime_args! {})
    }

    pub fn get_block_hash_store(&mut self) -> Key {
        self.test_env
            .call_view_function(self.vrf, "get_block_hash_store", runtime_args! {})
    }

    pub fn create_subscription(&mut self, consumers: &[Key]) -> u64 {
        let mut sub_id: u64 =
            self.test_env
                .call_view_function(self.vrf, "get_current_sub_id", runtime_args! {});
        sub_id += 1;
        self.test_env.call_contract(
            Some(self.sub_owner),
            key_to_contract_package_hash(self.vrf),
            "create_subscription",
            runtime_args! {},
            true,
        );

        for consumer in consumers {
            self.test_env.call_contract(
                Some(self.sub_owner),
                key_to_contract_package_hash(self.vrf),
                "add_consumer",
                runtime_args! {
                    "sub_id" => sub_id,
                    "consumer" => *consumer
                },
                true,
            );
        }
        sub_id
    }

    pub fn get_subscription(&mut self, sub_id: u64) -> SubscriptionView {
        self.test_env.call_view_function(self.vrf, "get_subscription_view", runtime_args! {
            "sub_id" => sub_id,
        })
    }
}

pub fn setup() -> VRFFixture {
    let owner = test_env::env::generate_random_account(SECP256K1_TAG);
    let sub_owner = test_env::env::generate_random_account(SECP256K1_TAG);
    let sub_owner_address = Key::from(sub_owner);
    let consumer = test_env::env::generate_random_account(SECP256K1_TAG);
    let random = test_env::env::generate_random_account(SECP256K1_TAG);
    let random_address = Key::from(random);
    let oracle = test_env::env::generate_random_account(SECP256K1_TAG);
    let mut test_env = TestEnv::new(&[owner, sub_owner, consumer, random, oracle], 0);

    test_env.deploy_contract(
        Some(owner),
        "cep18.wasm",
        runtime_args! {
            "name" => "T0",
            "symbol" => "T0",
            "decimals" => 18u8,
            "total_supply" =>
            U256::from(10_000_000_000_000_000_000_000_000_000_000_u128),
            "minter_list" => vec![Key::from(owner)],
            "admin_list" => vec![Key::from(owner)],
            "enable_mint_burn" => 1u8,
            "contract_name" => "T0",
        },
    );

    let payment_token = test_env.get_contract_package_hash(
        owner,
        &utils::get_contract_package_hash_key_cep18("T0".to_owned()),
    );

    test_env.deploy_contract(
        Some(owner),
        "block-hash-store.wasm",
        runtime_args! {
            "contract_name" => "block-hash-store"
        },
    );
    let block_hash_store = test_env.get_contract_package_hash(
        owner,
        &utils::get_contract_package_hash_key("block-hash-store".to_owned()),
    );

    let price_feed = null_key();

    test_env.deploy_contract(
        Some(owner),
        "vrf-coordinator.wasm",
        runtime_args! {
            "contract_name" => "vrf",
            "price_feed" => price_feed,
            "block_hash_store" => block_hash_store,
            "payment_token" => payment_token
        },
    );

    let vrf = test_env.get_contract_package_hash(
        owner,
        &utils::get_contract_package_hash_key("vrf".to_owned()),
    );

    test_env.transfer(
        payment_token,
        owner,
        sub_owner_address,
        U256::from(1000000000000000000_u128),
    );
    test_env.transfer(
        payment_token,
        owner,
        random_address,
        U256::from(1000000000000000000_u128),
    );

    let config = Config {
        minimum_request_confirmations: 1,
        max_gas_limit: U128::from(100_000_000_000_u128),
        staleness_seconds: 86400,
        gas_after_payment_calculation: U128::from(100_000_000_000_u128),
    };

    let mut fixture = VRFFixture {
        test_env,
        vrf,
        payment_token,
        block_hash_store,
        owner,
        sub_owner,
        sub_owner_address,
        consumer,
        random,
        random_address,
        oracle,
        config,
    };

    fixture.set_config(
        Some(owner),
        config.minimum_request_confirmations,
        config.max_gas_limit,
        config.staleness_seconds,
        config.gas_after_payment_calculation,
        Bytes::from(FeeConfig::default().to_bytes().unwrap()),
        true,
    );

    fixture
}

#[cfg(test)]
mod test_set_config {
    use casper_types::bytesrepr::{Bytes, ToBytes};
    use common::data_types::FeeConfig;

    use super::setup;

    #[test]
    fn test_only_owner_can_set() {
        let mut fixture = setup();
        fixture.set_config(
            Some(fixture.sub_owner),
            fixture.config.minimum_request_confirmations,
            fixture.config.max_gas_limit,
            fixture.config.staleness_seconds,
            fixture.config.gas_after_payment_calculation,
            Bytes::from(FeeConfig::default().to_bytes().unwrap()),
            false,
        );

        let c = fixture.get_config();
        assert!(c.max_gas_limit == fixture.config.max_gas_limit);
        assert!(c.minimum_request_confirmations == fixture.config.minimum_request_confirmations);
        assert!(c.staleness_seconds == fixture.config.staleness_seconds);
    }

    #[test]
    #[should_panic = "User(10020)"]
    fn test_max_req_confs() {
        let mut fixture = setup();
        fixture.set_config(
            Some(fixture.owner),
            201,
            fixture.config.max_gas_limit,
            fixture.config.staleness_seconds,
            fixture.config.gas_after_payment_calculation,
            Bytes::from(FeeConfig::default().to_bytes().unwrap()),
            true,
        );
    }
}


#[cfg(test)]
mod test_create_subscription {
    use super::setup;

    #[test]
    fn test_can_create_subscription() {
        let mut fixture = setup();
        let sub_id = fixture.create_subscription(&[]);
        assert!(sub_id == 1);
        let subscription = fixture.get_subscription(sub_id);
        assert!(subscription.balance.as_u128() == 0);
        assert!(subscription.owner == fixture.sub_owner_address);
    }
}