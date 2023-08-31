use casper_contract::contract_api::runtime;
use casper_types::bytesrepr::Bytes;
use common::helpers;
use k256::ecdsa;
use k256::ecdsa::signature::Verifier;
#[cfg(feature = "ecdsa")]
pub(crate) fn verify_signature(pubkey_bytes: &Bytes, digest: [u8; 32], signature: &Bytes) {
    use common::error::Error;
    use k256::ecdsa::VerifyingKey;
    let sig = helpers::set_size_64(signature);
    let owner_pubkey = VerifyingKey::from_sec1_bytes(pubkey_bytes.as_slice()).unwrap();
    let (r_sig, s_sig) = sig.split_at(32);
    let r = helpers::set_size_32(r_sig);
    let s = helpers::set_size_32(s_sig);
    let secp256k1_sig = ecdsa::Signature::from_scalars(r, s)
        .unwrap_or_else(|_e| runtime::revert(Error::ErrorGettingSignature));
    let valid_sig = (owner_pubkey as VerifyingKey)
        .verify(&digest, &secp256k1_sig)
        .is_ok();

    if !valid_sig {
        runtime::revert(Error::InvalidSignature);
    }
}
