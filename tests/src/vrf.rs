use casper_types::{
    account::AccountHash,
    bytesrepr::{Bytes, ToBytes},
    runtime_args, HashAddr, Key, RuntimeArgs, SECP256K1_TAG, U128, U256,
};
use common::{
    data_types::{Config, FeeConfig, SubscriptionView},
    helpers::null_key,
};
use test_env::env::TestEnv;

use crate::utils::{self, key_to_contract_package_hash};

pub struct VRFFixture {
    pub test_env: TestEnv,
    pub vrf: Key,
    pub payment_token: Key,
    pub block_hash_store: Key,
    pub owner: AccountHash,
    pub sub_owner: AccountHash,
    pub sub_owner_address: Key,
    pub consumer: AccountHash,
    pub random: AccountHash,
    pub random_address: Key,
    pub oracle: AccountHash,
    pub config: Config,
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

    pub fn add_consumer(&mut self, caller: AccountHash, sub_id: u64, consumer: Key) {
        self.test_env.call_contract(
            Some(caller),
            key_to_contract_package_hash(self.vrf),
            "add_consumer",
            runtime_args! {
                "sub_id" => sub_id,
                "consumer" => consumer
            },
            true,
        );
    }

    pub fn remove_consumer(&mut self, caller: AccountHash, sub_id: u64, consumer: Key) {
        self.test_env.call_contract(
            Some(caller),
            key_to_contract_package_hash(self.vrf),
            "remove_consumer",
            runtime_args! {
                "sub_id" => sub_id,
                "consumer" => consumer
            },
            true,
        );
    }

    pub fn cancel_subscription(&mut self, caller: AccountHash, sub_id: u64, to: Key) {
        self.test_env.call_contract(
            Some(caller),
            self.vrf.into_hash().unwrap().into(),
            "cancel_subscription",
            runtime_args! {
                "sub_id" => sub_id,
                "to" => to
            },
            true,
        );
    }
    pub fn get_subscription(&mut self, sub_id: u64) -> SubscriptionView {
        self.test_env.call_view_function(
            self.vrf,
            "get_subscription_view",
            runtime_args! {
                "sub_id" => sub_id,
            },
        )
    }

    pub fn deposit_token(&mut self, caller: AccountHash, sub_id: u64, amount: U128) {
        self.test_env
            .approve(self.payment_token, caller, self.vrf, U256::MAX);
        self.test_env.call_contract(
            Some(caller),
            self.vrf.into_hash().unwrap().into(),
            "deposit_token",
            runtime_args! {
                "sub_id" => sub_id,
                "amount" => amount
            },
            true,
        );
    }

    pub fn balance_of(&mut self, token: Key, addr: Key) -> U128 {
        let b: U256 = self.test_env.call_view_function(
            token,
            "balance_of",
            runtime_args! {
                "address" => addr,
            },
        );
        b.as_u128().into()
    }

    pub fn recover_funds(&mut self, caller: AccountHash, to: Key) {
        self.test_env.call_contract(
            Some(caller),
            self.vrf.into_hash().unwrap().into(),
            "recover_funds",
            runtime_args! {
                "to" => to,
            },
            true,
        );
    }

    pub fn request_random_words(
        &mut self,
        caller: AccountHash,
        key_hash: HashAddr,
        sub_id: u64,
        request_confirmations: u64,
        callback_gas_limit: U128,
        num_words: u64,
    ) {
        self.test_env.call_contract(
            Some(caller),
            self.vrf.into_hash().unwrap().into(),
            "request_random_words",
            runtime_args! {
                "key_hash" => key_hash,
                "sub_id" => sub_id,
                "request_confirmations" => request_confirmations,
                "callback_gas_limit" => callback_gas_limit,
                "num_words" => num_words
            },
            true,
        );
    }

    pub fn register_proving_key(&mut self, caller: AccountHash, oracle: Key, test_key: Bytes) {
        self.test_env.call_contract(
            Some(caller),
            self.vrf.into_hash().unwrap().into(),
            "register_proving_key",
            runtime_args! {
                "oracle" => oracle,
                "public_proving_key" => test_key,
            },
            true,
        );
    }

    pub fn deregister_proving_key(&mut self, caller: AccountHash, test_key: Bytes) {
        self.test_env.call_contract(
            Some(caller),
            self.vrf.into_hash().unwrap().into(),
            "deregister_proving_key",
            runtime_args! {
                "public_proving_key" => test_key,
            },
            true,
        );
    }

    pub fn fulfill_random_words(&mut self, caller: AccountHash, proof: Bytes, rc: Bytes) {
        self.test_env.call_contract(
            Some(caller),
            self.vrf.into_hash().unwrap().into(),
            "fulfill_random_words",
            runtime_args! {
                "proof" => proof,
                "rc" => rc,
            },
            true,
        );
    }

    pub fn hash_of_key(&mut self, test_key: Bytes) -> HashAddr {
        self.test_env.call_view_function(
            self.vrf,
            "hash_of_key",
            runtime_args! {
                "public_proving_key" => test_key.clone()
            },
        )
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

#[cfg(test)]
mod test_cancel_subscription {
    use casper_types::U256;

    use super::{setup, VRFFixture};

    fn before_each() -> (VRFFixture, u64) {
        let mut fixture = setup();
        let sub_id = fixture.create_subscription(&[]);
        (fixture, sub_id)
    }

    #[test]
    #[should_panic = "User(10013)"]
    fn test_subscription_must_exist() {
        let (mut fixture, _) = before_each();
        fixture.cancel_subscription(fixture.sub_owner, 1203123123, fixture.sub_owner_address);
    }

    #[test]
    #[should_panic = "User(10016)"]
    fn test_must_be_owner() {
        let (mut fixture, sub_id) = before_each();
        fixture.cancel_subscription(fixture.random, sub_id, fixture.sub_owner_address);
    }

    #[test]
    #[should_panic = "User(10013)"]
    fn test_on_cancel() {
        let (mut fixture, sub_id) = before_each();
        fixture.test_env.approve(
            fixture.payment_token,
            fixture.sub_owner,
            fixture.vrf,
            U256::MAX,
        );
        fixture.deposit_token(fixture.sub_owner, sub_id, 1000.into());
        fixture.cancel_subscription(fixture.sub_owner, sub_id, fixture.random_address);
        let random_balance = fixture.balance_of(fixture.payment_token, fixture.random_address);
        assert!(random_balance.to_string() == "1000000000000001000");
        fixture.get_subscription(sub_id);
    }
}

#[cfg(test)]
mod test_recover_funds {

    use super::{setup, VRFFixture};

    fn before_each() -> (VRFFixture, u64) {
        let mut fixture = setup();
        let sub_id = fixture.create_subscription(&[]);
        (fixture, sub_id)
    }

    #[test]
    #[should_panic = "User(10002)"]
    fn test_only_owner_can_recover() {
        let (mut fixture, _) = before_each();
        fixture.recover_funds(fixture.sub_owner, fixture.random_address);
    }

    // #[test]
    // fn test_owner_can_recover_payment_token() {
    //     let (mut fixture, _) = before_each();
    //     assert!(fixture.balance_of(fixture.payment_token, fixture.random_address) == U128::zero());
    // }
}

#[cfg(test)]
mod test_request_random_words {
    use casper_types::{
        bytesrepr::{Bytes, ToBytes},
        runtime_args, HashAddr, Key, RuntimeArgs, U128, U256,
    };

    use super::{setup, VRFFixture};

    fn before_each() -> (VRFFixture, u64, HashAddr) {
        let mut fixture = setup();
        let sub_id = fixture.create_subscription(&[Key::from(fixture.consumer)]);
        let test_key = vec![U256::one(), U256::from(2)];
        let test_key_bytes = Bytes::from(test_key.to_bytes().unwrap());
        let kh: HashAddr = fixture.test_env.call_view_function(
            fixture.vrf,
            "hash_of_key",
            runtime_args! {
                "public_proving_key" => test_key_bytes
            },
        );
        (fixture, sub_id, kh)
    }

    #[test]
    #[should_panic = "User(10013)"]
    fn test_invalid_sub_id() {
        let (mut fixture, _, kh) = before_each();
        fixture.request_random_words(fixture.random, kh, 12301928312, 1, 1000.into(), 1);
    }

    #[test]
    #[should_panic = "User(10012)"]
    fn test_invalid_consumer() {
        let (mut fixture, sub_id, kh) = before_each();
        fixture.request_random_words(fixture.random, kh, sub_id, 1, 1000.into(), 1);
    }

    #[test]
    #[should_panic = "User(10020)"]
    fn test_invalid_req_confs() {
        let (mut fixture, sub_id, kh) = before_each();
        fixture.request_random_words(fixture.consumer, kh, sub_id, 0, 1000.into(), 1);
    }

    #[test]
    #[should_panic = "User(10021)"]
    fn test_gas_limit_too_high() {
        let (mut fixture, sub_id, kh) = before_each();
        fixture.deposit_token(
            fixture.sub_owner,
            sub_id,
            U128::from(1000000000000000000_u128),
        );
        fixture.request_random_words(
            fixture.consumer,
            kh,
            sub_id,
            1,
            100_000_000_001_u128.into(),
            1,
        );
    }

    #[test]
    #[should_panic = "User(10012)"]
    fn test_add_remove_consumer_invariant() {
        let (mut fixture, sub_id, kh) = before_each();
        fixture.deposit_token(
            fixture.sub_owner,
            sub_id,
            U128::from(1000000000000000000_u128),
        );
        fixture.add_consumer(fixture.sub_owner, sub_id, fixture.random_address);
        fixture.remove_consumer(fixture.sub_owner, sub_id, fixture.random_address);
        fixture.request_random_words(fixture.random, kh, sub_id, 1, 1000.into(), 1);
    }

    #[test]
    #[should_panic = "User(10012)"]
    fn test_cancel_add_subscription_invariant() {
        let (mut fixture, mut sub_id, kh) = before_each();
        fixture.deposit_token(
            fixture.sub_owner,
            sub_id,
            U128::from(1000000000000000000_u128),
        );
        fixture.cancel_subscription(fixture.sub_owner, sub_id, fixture.random_address);
        sub_id = fixture.create_subscription(&[]);
        fixture.request_random_words(fixture.random, kh, sub_id, 1, 1000.into(), 1);
    }
}

#[cfg(test)]
mod test_key_registration {
    use casper_types::{
        bytesrepr::{Bytes, ToBytes},
        runtime_args, HashAddr, RuntimeArgs, U256,
    };

    use super::{setup, VRFFixture};

    fn before_each() -> (VRFFixture, Bytes, HashAddr) {
        let mut fixture = setup();
        let test_key = vec![U256::one(), U256::from(2)];
        let test_key_bytes = Bytes::from(test_key.to_bytes().unwrap());
        let kh: HashAddr = fixture.test_env.call_view_function(
            fixture.vrf,
            "hash_of_key",
            runtime_args! {
                "public_proving_key" => test_key_bytes.clone()
            },
        );
        (fixture, test_key_bytes, kh)
    }

    #[test]
    #[should_panic = "User(10023)"]
    fn test_cannot_reregister_key() {
        let (mut fixture, test_key, _) = before_each();
        fixture.register_proving_key(fixture.owner, fixture.sub_owner_address, test_key.clone());
        fixture.register_proving_key(fixture.owner, fixture.sub_owner_address, test_key.clone());
    }

    #[test]
    #[should_panic = "User(10024)"]
    fn test_cannot_deregister_unregistered_key() {
        let (mut fixture, test_key, _) = before_each();
        fixture.deregister_proving_key(fixture.owner, test_key.clone());
    }

    #[test]
    fn test_can_reregister_after_deregister() {
        let (mut fixture, test_key, _) = before_each();
        fixture.register_proving_key(fixture.owner, fixture.sub_owner_address, test_key.clone());
        fixture.deregister_proving_key(fixture.owner, test_key.clone());
        fixture.register_proving_key(fixture.owner, fixture.random_address, test_key.clone());
    }
}

#[cfg(test)]
mod test_fulfill_random_words {
    use casper_types::{
        bytesrepr::{Bytes, ToBytes},
        Key, U256,
    };
    use common::{
        data_types::{Proof, RequestCommitment},
        events::RandomWordsRequested,
    };

    use super::{setup, VRFFixture};

    fn before_each() -> (VRFFixture, Bytes) {
        let mut fixture = setup();
        let test_key = vec![U256::one(), U256::from(2)];
        let test_key_bytes = Bytes::from(test_key.to_bytes().unwrap());
        fixture.register_proving_key(
            fixture.owner,
            fixture.sub_owner_address,
            test_key_bytes.clone(),
        );
        (fixture, test_key_bytes)
    }

    #[test]
    #[should_panic = "User(10024)"]
    fn test_unregistered_key_should_fail() {
        let (mut fixture, _) = before_each();
        let proof = Proof {
            pk: vec![U256::one(), U256::from(3)].to_bytes().unwrap().into(),
            gamma: vec![U256::one(), U256::from(2)].to_bytes().unwrap().into(),
            c: U256::from(1),
            s: U256::from(1),
            seed: U256::from(1),
            u_witness: fixture.random_address,
            c_gamma_witness: vec![U256::one(), U256::from(2)].to_bytes().unwrap().into(),
            s_hash_witness: vec![U256::one(), U256::from(2)].to_bytes().unwrap().into(),
            z_inv: U256::from(1),
        };
        let proof = proof.to_bytes().unwrap().into();
        let rc = RequestCommitment {
            block_num: 1,
            sub_id: 1,
            callback_gas_limit: 3.into(),
            num_words: 4,
            sender: fixture.random_address,
        };

        let rc = rc.to_bytes().unwrap().into();
        fixture.fulfill_random_words(fixture.oracle, proof, rc);
    }

    #[test]
    #[should_panic = "User(10027)"]
    fn test_no_corresponding_request() {
        let (mut fixture, _) = before_each();
        let proof = Proof {
            pk: vec![U256::one(), U256::from(2)].to_bytes().unwrap().into(),
            gamma: vec![U256::one(), U256::from(2)].to_bytes().unwrap().into(),
            c: U256::from(1),
            s: U256::from(1),
            seed: U256::from(1),
            u_witness: fixture.random_address,
            c_gamma_witness: vec![U256::one(), U256::from(2)].to_bytes().unwrap().into(),
            s_hash_witness: vec![U256::one(), U256::from(2)].to_bytes().unwrap().into(),
            z_inv: U256::from(1),
        };
        let proof = proof.to_bytes().unwrap().into();
        let rc = RequestCommitment {
            block_num: 1,
            sub_id: 1,
            callback_gas_limit: 3.into(),
            num_words: 4,
            sender: fixture.random_address,
        };

        let rc = rc.to_bytes().unwrap().into();
        fixture.fulfill_random_words(fixture.oracle, proof, rc);
    }

    #[test]
    #[should_panic = "User(10028)"]
    fn test_incorrect_commitment_wrong_blocknum() {
        let (mut fixture, test_key) = before_each();
        let sub_id = fixture.create_subscription(&[Key::from(fixture.consumer)]);
        fixture.deposit_token(fixture.sub_owner, sub_id, 1000000000000000000_u128.into());
        let kh = fixture.hash_of_key(test_key);
        let event_length = fixture.test_env.get_event_length(fixture.vrf);
        fixture.request_random_words(fixture.consumer, kh, sub_id, 1, 1000.into(), 1);

        let req_recipt: RandomWordsRequested = fixture
            .test_env
            .get_event(fixture.vrf, event_length as usize)
            .unwrap();

        let proof = Proof {
            pk: vec![U256::one(), U256::from(2)].to_bytes().unwrap().into(),
            gamma: vec![U256::one(), U256::from(2)].to_bytes().unwrap().into(),
            c: U256::from(1),
            s: U256::from(1),
            seed: req_recipt.pre_seed,
            u_witness: fixture.random_address,
            c_gamma_witness: vec![U256::one(), U256::from(2)].to_bytes().unwrap().into(),
            s_hash_witness: vec![U256::one(), U256::from(2)].to_bytes().unwrap().into(),
            z_inv: U256::from(1),
        };
        let proof: Bytes = proof.to_bytes().unwrap().into();
        let rc = RequestCommitment {
            block_num: req_recipt.timestamp + 1,    //wrong block number
            sub_id,
            callback_gas_limit: 1000.into(),
            num_words: 1,
            sender: fixture.consumer.into(),
        };

        let rc = rc.to_bytes().unwrap().into();
        fixture.fulfill_random_words(fixture.oracle, proof, rc);
    }
}
