use ark_bn254::G1Projective;
use ark_bn254::G2Projective;
use ark_ff::BigInteger256;

use crate::contract::registry_coordinator::{
    registry_coordinator, G1Point as RegistryG1Point, G2Point as RegistryG2Point,
};
use eigensdk_crypto_bls::attestation::G1Point;
use eigensdk_crypto_bn254::utils::biginteger256_to_u256;
use eigensdk_crypto_bn254::utils::u256_to_bigint256;

pub fn convert_bn254_to_ark(g1_point: registry_coordinator::G1Point) -> G1Point {
    G1Point::new(u256_to_bigint256(g1_point.x), u256_to_bigint256(g1_point.y))
}

pub fn convert_to_bn254_g1_point(g1: G1Projective) -> RegistryG1Point {
    let x: BigInteger256 = g1.x.into();
    let y: BigInteger256 = g1.y.into();

    RegistryG1Point {
        x: biginteger256_to_u256(x),
        y: biginteger256_to_u256(y),
    }
}

pub fn convert_to_bn254_g2_point(g2: G2Projective) -> RegistryG2Point {
    let x_0: BigInteger256 = g2.x.c0.into();
    let x_1: BigInteger256 = g2.x.c1.into();
    let y_0: BigInteger256 = g2.y.c0.into();
    let y_1: BigInteger256 = g2.y.c1.into();

    RegistryG2Point {
        x: [biginteger256_to_u256(x_0), biginteger256_to_u256(x_1)],
        y: [biginteger256_to_u256(y_0), biginteger256_to_u256(y_1)],
    }
}
