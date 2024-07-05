use crate::bn254::errors::BLSError;
use ark_bn254::{Bn254, Fq, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::pairing::Pairing;
use ark_ec::{pairing::PairingOutput, AffineRepr, CurveGroup};
use ark_ff::{BigInteger256, Field, PrimeField};
use ark_std::One;
use num_bigint::BigUint;
use sha2::{Digest, Sha256};

fn pairing(u: G2Affine, v: G1Affine) -> PairingOutput<Bn254> {
    Bn254::pairing(v, u)
}

fn hash_to_curve(digest: &[u8]) -> G1Affine {
    let one = Fq::one();
    let three = Fq::from(3u64);

    let mut hasher = Sha256::new();
    hasher.update(digest);
    let hashed_result = hasher.finalize();

    // Convert digest to a big integer and then to a field element
    let mut x = {
        let big_int = BigUint::from_bytes_be(&hashed_result);
        let mut bytes = [0u8; 32];
        big_int
            .to_bytes_be()
            .iter()
            .rev()
            .enumerate()
            .for_each(|(i, &b)| bytes[i] = b);
        Fq::from_le_bytes_mod_order(&bytes)
    };

    loop {
        // y = x^3 + 3
        let mut y = x;
        y.square_in_place();
        y *= x;
        y += three;

        // Check if y is a quadratic residue (i.e., has a square root in the field)
        if let Some(y) = y.sqrt() {
            return G1Projective::new(x, y, Fq::one()).into_affine();
        } else {
            // x = x + 1
            x += one;
        }
    }
}

pub fn sign(sk: Fr, message: &[u8]) -> Result<G1Affine, BLSError> {
    let q = hash_to_curve(message);

    let sk_int: BigInteger256 = sk.into();
    let r = q.mul_bigint(sk_int);

    if !r.into_affine().is_on_curve() || !r.into_affine().is_in_correct_subgroup_assuming_on_curve()
    {
        return Err(BLSError::SignatureNotInSubgroup);
    }

    Ok(r.into_affine())
}

pub fn verify(public_key: G2Affine, message: &[u8], signature: G1Affine) -> bool {
    if !signature.is_in_correct_subgroup_assuming_on_curve() || !signature.is_on_curve() {
        return false;
    }

    let q = hash_to_curve(message);
    let c1 = pairing(public_key, q);
    let c2 = pairing(G2Affine::generator(), signature);
    c1 == c2
}

pub fn aggregate_signatures(signatures: &[G1Affine]) -> Result<G1Affine, BLSError> {
    if signatures.is_empty() {
        return Err(BLSError::SignatureListEmpty);
    }

    let signatures_in_projective: Vec<G1Projective> = signatures
        .iter()
        .map(|sig| {
            let proj = G1Projective::from(*sig);
            if !sig.is_on_curve() || !sig.is_in_correct_subgroup_assuming_on_curve() {
                return Err(BLSError::SignatureNotInSubgroup);
            }
            Ok(proj)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut aggregated = signatures_in_projective[0];
    for sig in &signatures[1..] {
        aggregated += sig;
    }
    Ok(aggregated.into_affine())
}

pub fn aggregate_public_keys(public_keys: &[G2Affine]) -> Result<G2Affine, BLSError> {
    if public_keys.is_empty() {
        return Err(BLSError::PublicKeyListEmpty);
    }

    let public_keys_in_projective: Vec<G2Projective> = public_keys
        .iter()
        .map(|pk| {
            let proj = G2Projective::from(*pk);
            if !pk.is_on_curve() || !pk.is_in_correct_subgroup_assuming_on_curve() {
                return Err(BLSError::PublicKeyNotInSubgroup);
            }
            Ok(proj)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut aggregated = public_keys_in_projective[0];
    for pk in &public_keys_in_projective[1..] {
        aggregated += *pk;
    }

    Ok(aggregated.into_affine())
}

pub mod errors {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub enum BLSError {
        SignatureNotInSubgroup,
        SignatureListEmpty,
        PublicKeyNotInSubgroup,
        PublicKeyListEmpty,
    }

    impl Error for BLSError {}

    impl fmt::Display for BLSError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                BLSError::SignatureNotInSubgroup => write!(f, "Signature not in subgroup"),
                BLSError::PublicKeyNotInSubgroup => write!(f, "Public key not in subgroup"),
                BLSError::SignatureListEmpty => write!(f, "Signature array is empty"),
                BLSError::PublicKeyListEmpty => write!(f, "The public key list is empty"),
            }
        }
    }
}
