use casper_types::{ContractPackageHash, Key};

pub fn get_contract_package_hash_key(contract_name: String) -> String {
    format!("{}_package_hash", contract_name)
}

pub fn get_contract_package_hash_key_cep18(contract_name: String) -> String {
    format!("cep18_contract_package_{}", contract_name)
}

pub fn key_to_contract_package_hash(k: Key) -> ContractPackageHash {
    k.into_hash().unwrap().into()
}
