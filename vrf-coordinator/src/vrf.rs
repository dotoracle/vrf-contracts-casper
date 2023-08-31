use core::ops::{Add, Shr};

use alloc::{vec, vec::Vec};
use casper_contract::contract_api::runtime;
use casper_types::{
    bytesrepr::{Bytes, FromBytes},
    HashAddr, Key, U256,
};
use common::{
    error::Error,
    helpers::{add_mod, mulmod, null_key},
};

fn field_size() -> U256 {
    U256::from_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16,
    )
    .unwrap()
}

fn group_order() -> U256 {
    U256::from_str_radix(
        "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
        16,
    )
    .unwrap()
}

use common::{
    data_types::Proof,
    helpers::{self, require},
};

use crate::signature::verify_signature;

fn y_squred(x: &U256) -> U256 {
    let x_cubed = helpers::mulmod(*x, mulmod(*x, *x, field_size()), field_size());
    add_mod(x_cubed, 7.into(), field_size())
}

fn is_on_secp256k1_curve_(bs: &Bytes) -> bool {
    let p: Vec<U256> = helpers::decode_1(bs);
    is_on_secp256k1_curve(&p)
}

fn is_on_secp256k1_curve(p: &Vec<U256>) -> bool {
    require(p.len() == 2, Error::InvalidProvingKeyLength);

    require(p[0].lt(&field_size()), Error::InvalidXCordinate);
    require(p[1].lt(&field_size()), Error::InvalidXCordinate);
    y_squred(&p[0]) == mulmod(p[1], p[1], field_size())
}

fn verify_linear_combination_with_generator(
    _c: U256,
    _p: &[U256],
    _s: U256,
    lc_witness: Key,
) -> bool {
    // TODO: do the real verification
    require(lc_witness != null_key(), Error::BadWitness);
    true
}

fn field_hash(b: &Bytes) -> U256 {
    let mut ret = U256::from_bytes(&runtime::blake2b(b)).unwrap().0;
    while ret.ge(&field_size()) {
        ret = U256::from_bytes(&runtime::blake2b(helpers::encode_1(&ret)))
            .unwrap()
            .0;
    }
    ret
}

fn big_mod_exp(base: U256, exponent: U256) -> U256 {
    let mut ret = U256::one();
    let mut base = base % field_size();
    let mut exponent = exponent;
    while exponent.gt(&0.into()) {
        if exponent % 2 == U256::one() {
            ret = mulmod(ret, base, field_size());
        }
        exponent = exponent.shr(1);
        base = mulmod(base, base, field_size());
    }
    ret
}

fn sqrt_power() -> U256 {
    field_size().add(1).shr(2)
}

fn hash_to_curve_hash_prefix() -> U256 {
    U256::from(1)
}

fn scalar_from_curve_points_hash_prefix() -> U256 {
    U256::from(2)
}

fn vrf_random_output_hash_prefix() -> U256 {
    U256::from(3)
}

fn square_root(x: U256) -> U256 {
    big_mod_exp(x, sqrt_power())
}

fn new_candidate_secp256k1_point(b: &Bytes) -> Vec<U256> {
    let mut p: Vec<U256> = vec![field_hash(b), square_root(y_squred(&field_hash(b)))];
    if p[1] % U256::from(2) == U256::one() {
        p[1] = field_size() - p[1];
    }
    p
}

fn hash_to_curve(pk: &Vec<U256>, input: U256) -> Vec<U256> {
    let mut rv = new_candidate_secp256k1_point(&Bytes::from(helpers::encode_3(
        &hash_to_curve_hash_prefix(),
        pk,
        &input,
    )));
    while !is_on_secp256k1_curve(&rv) {
        rv = new_candidate_secp256k1_point(&Bytes::from(helpers::encode_1(&rv)));
    }
    rv
}

fn ecmul_verify(multiplic_and: &[U256], scalar: U256, product: &[U256]) -> bool {
    require(scalar != U256::zero(), Error::ScalarZero);
    let x = multiplic_and[0];
    let _v: u8 = if multiplic_and[1] % U256::from(2) == U256::zero() {
        27
    } else {
        28
    };
    let scalar_times_x = mulmod(scalar, x, group_order());
    verify_signature(
        &Bytes::from(helpers::encode_1(&Vec::from(product))),
        HashAddr::default(),
        &Bytes::from(helpers::encode_2(&x, &scalar_times_x)),
    );

    true
}

fn projective_mul(x1: U256, z1: U256, x2: U256, z2: U256) -> (U256, U256) {
    (mulmod(x1, x2, field_size()), mulmod(z1, z2, field_size()))
}

fn projective_sub(x1: U256, z1: U256, x2: U256, z2: U256) -> (U256, U256) {
    let num1 = mulmod(z2, x1, field_size());
    let num2 = mulmod(field_size() - x2, z1, field_size());
    (
        add_mod(num1, num2, field_size()),
        mulmod(z1, z2, field_size()),
    )
}
fn projective_ec_add(px: U256, py: U256, qx: U256, qy: U256) -> (U256, U256, U256) {
    let (z1, z2) = (U256::one(), U256::one());
    let lx = add_mod(qy, field_size() - py, field_size());
    let lz = add_mod(qx, field_size() - px, field_size());

    let (mut sx, mut dx) = projective_mul(lx, lz, lx, lz);
    (sx, dx) = projective_sub(sx, dx, px, z1);
    (sx, dx) = projective_sub(sx, dx, qx, z2);

    let (mut sy, mut dy) = projective_sub(px, z1, sx, dx);
    (sy, dy) = projective_mul(sy, dy, lx, lz);
    (sy, dy) = projective_sub(sy, dy, py, z1);

    let sz = if dx != dy {
        sx = mulmod(sx, dy, field_size());
        sy = mulmod(sy, dx, field_size());
        mulmod(dx, dy, field_size())
    } else {
        dx
    };
    (sx, sy, sz)
}

fn affine_ec_add(p1: &[U256], p2: &[U256], inv_z: U256) -> Vec<U256> {
    let (x, y, z) = projective_ec_add(p1[0], p1[1], p2[0], p2[1]);
    require(
        mulmod(z, inv_z, field_size()) == U256::one(),
        Error::InvZMustBeInverseOfZ,
    );
    vec![
        mulmod(x, inv_z, field_size()),
        mulmod(y, inv_z, field_size()),
    ]
}

fn linear_combination(
    c: U256,
    p1: &[U256],
    cp1_witness: &[U256],
    s: U256,
    p2: &[U256],
    sp2_witness: &[U256],
    z_inv: U256,
) -> Vec<U256> {
    require(
        cp1_witness[0] % field_size() != sp2_witness[0] % field_size(),
        Error::PointsSumMustBeDistinct,
    );
    require(ecmul_verify(p1, c, cp1_witness), Error::FirstMulCheckFailed);
    require(
        ecmul_verify(p2, s, sp2_witness),
        Error::SecondMulCheckFailed,
    );

    affine_ec_add(cp1_witness, sp2_witness, z_inv)
}

fn scalar_from_curve_points(
    hash: &Vec<U256>,
    pk: &Vec<U256>,
    gamma: &Vec<U256>,
    u_witness: Key,
    v: &Vec<U256>,
) -> U256 {
    U256::from_bytes(&runtime::blake2b(helpers::encode_6(
        &scalar_from_curve_points_hash_prefix(),
        hash,
        pk,
        gamma,
        v,
        &u_witness,
    )))
    .unwrap()
    .0
}

#[allow(clippy::too_many_arguments)]
pub fn verify_vrf_proof(
    pk: &Bytes,
    gamma: &Bytes,
    c: U256,
    s: U256,
    seed: U256,
    u_witness: Key,
    c_gamma_witness: &Bytes,
    s_hash_witness: &Bytes,
    z_inv: U256,
) {
    let pk: Vec<U256> = helpers::decode_1(pk);
    require(is_on_secp256k1_curve(&pk), Error::KeyNotOnCurve);
    require(is_on_secp256k1_curve_(gamma), Error::KeyNotOnCurve);
    require(
        is_on_secp256k1_curve_(c_gamma_witness),
        Error::KeyNotOnCurve,
    );
    require(is_on_secp256k1_curve_(s_hash_witness), Error::KeyNotOnCurve);

    require(
        verify_linear_combination_with_generator(c, &pk, s, u_witness),
        Error::BadLinearCombinationWithGenerator,
    );

    let hash = hash_to_curve(&pk, seed);
    let gamma: Vec<U256> = helpers::decode_1(gamma);
    let c_gamma_witness: Vec<U256> = helpers::decode_1(c_gamma_witness);
    let s_hash_witness: Vec<U256> = helpers::decode_1(s_hash_witness);
    let v = linear_combination(
        c,
        &gamma,
        &c_gamma_witness,
        s,
        &hash,
        &s_hash_witness,
        z_inv,
    );

    let derived_c = scalar_from_curve_points(&hash, &pk, &gamma, u_witness, &v);
    require(c == derived_c, Error::InvalidProof);
}

pub fn random_value_from_vrf_proof(proof: &Proof, seed: U256) -> U256 {
    verify_vrf_proof(
        &proof.pk,
        &proof.gamma,
        proof.c,
        proof.s,
        seed,
        proof.u_witness,
        &proof.c_gamma_witness,
        &proof.s_hash_witness,
        proof.z_inv,
    );

    U256::from_bytes(&runtime::blake2b(helpers::encode_2(
        &vrf_random_output_hash_prefix(),
        &proof.gamma,
    )))
    .unwrap()
    .0
}
