//! Actual Curve25519 target construction using the existing counted arithmetic.
//! Coordinates are short Weierstrass coordinates, not Montgomery u coordinates.
use super::canonical::canonical;
use super::exact_sampling::derive_below;
use super::exact_scalar::ExactScalar;
use crate::cryptanalysis::curve25519_wide::{Curve25519Wide, WidePoint, A25519, L25519, P25519};
use crate::cryptanalysis::fp_backend::PrimeField;
use crate::cryptanalysis::ic_boundary::GroupOps;
use crate::hash::sha256::sha256;
use num_bigint::BigUint;

pub const PLANTED_LAW: &str = "curve25519_scalar_rejection_sha256_v1";
pub const PUBLIC_LAW: &str = "curve25519_point_rejection_sha256_v1";
const HASH_ATTEMPTS: u64 = 128;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetSample {
    pub coordinates: [String; 2],
    pub point_counter: u64,
    pub hash_counter: u64,
}

fn ensure_curve(curve: &Curve25519Wide) -> Result<(), String> {
    let expected = Curve25519Wide::new();
    if curve.field.modulus() != expected.field.modulus()
        || curve.a != expected.a
        || curve.b != expected.b
        || curve.r != expected.r
        || curve.cofactor != expected.cofactor
        || curve.generator != expected.generator
    {
        return Err("target law requires canonical Curve25519 and its recorded generator".into());
    }
    Ok(())
}

fn parts(seed: u64, index: u64, point_counter: u64) -> Vec<String> {
    [
        P25519.to_string(),
        A25519.to_string(),
        L25519.to_string(),
        seed.to_string(),
        index.to_string(),
        point_counter.to_string(),
    ]
    .into()
}

pub fn encode(curve: &Curve25519Wide, point: WidePoint) -> Option<[String; 2]> {
    point.map(|(x, y)| {
        [
            format!("0x{:x}", curve.field.to_biguint(x)),
            format!("0x{:x}", curve.field.to_biguint(y)),
        ]
    })
}

/// Reject noncanonical field values before the backend can reduce them.
/// Only nonidentity points in the selected prime-order subgroup are accepted.
pub fn decode(curve: &Curve25519Wide, coordinates: &[String; 2]) -> Result<WidePoint, String> {
    decode_counted(curve, coordinates, &mut GroupOps::default())
}

/// Same checked decoder with a retained group/field ledger. Parsing, field
/// conversion and canonical-curve construction are not group operations.
pub fn decode_counted(
    curve: &Curve25519Wide,
    coordinates: &[String; 2],
    ops: &mut GroupOps,
) -> Result<WidePoint, String> {
    ensure_curve(curve)?;
    let read = |text: &str| -> Result<_, String> {
        let digits = text
            .strip_prefix("0x")
            .ok_or("coordinate needs a 0x prefix")?;
        if digits.is_empty() || digits.len() > 64 || !digits.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err("coordinate must contain 1..64 hex digits".into());
        }
        let value = BigUint::parse_bytes(digits.as_bytes(), 16).ok_or("invalid coordinate")?;
        if &value >= curve.field.modulus() {
            return Err("coordinate is outside the field".into());
        }
        Ok(curve.field.from_biguint(&value))
    };
    let point = Some((read(&coordinates[0])?, read(&coordinates[1])?));
    // y^2 == x^3 + a*x + b: two multiplications and two squares.
    ops.field_ops.muls += 2;
    ops.field_ops.sqrs += 2;
    if !curve.on_curve(point) {
        return Err("point is not on Curve25519".into());
    }
    if curve.mul_biguint(ops, point, &curve.r).is_some() {
        return Err("point is outside the prime-order subgroup".into());
    }
    Ok(point)
}

pub fn mul_generator(curve: &Curve25519Wide, scalar: ExactScalar) -> Option<[String; 2]> {
    ensure_curve(curve).ok()?;
    let reduced = scalar.to_biguint() % &curve.r;
    encode(
        curve,
        curve.mul_biguint(&mut GroupOps::default(), curve.generator, &reduced),
    )
}

pub fn planted(
    curve: &Curve25519Wide,
    seed: u64,
    index: u64,
) -> Result<(TargetSample, ExactScalar), String> {
    ensure_curve(curve)?;
    let bound = ExactScalar::from_biguint(&(&curve.r - 1u8)).ok_or("order exceeds 256 bits")?;
    let (draw, hash_counter) =
        derive_below(PLANTED_LAW, &parts(seed, index, 0), bound, HASH_ATTEMPTS)?;
    let scalar = ExactScalar::from_biguint(&(draw.to_biguint() + 1u8)).ok_or("scalar overflow")?;
    let coordinates = mul_generator(curve, scalar).ok_or("planted scalar produced identity")?;
    Ok((
        TargetSample {
            coordinates,
            point_counter: 0,
            hash_counter,
        },
        scalar,
    ))
}

/// Lift uniform field abscissas with an independently derived sign, clear the
/// cofactor, and reject identity. This never constructs or retains a logarithm.
pub fn public(
    curve: &Curve25519Wide,
    seed: u64,
    index: u64,
    max_points: u64,
) -> Result<TargetSample, String> {
    ensure_curve(curve)?;
    let bound = ExactScalar::from_biguint(curve.field.modulus()).ok_or("field exceeds 256 bits")?;
    for point_counter in 0..max_points {
        let inputs = parts(seed, index, point_counter);
        let (draw, hash_counter) = derive_below(PUBLIC_LAW, &inputs, bound, HASH_ATTEMPTS)?;
        let x = curve.field.from_biguint(&draw.to_biguint());
        let Some(mut y) = curve.field.sqrt(curve.rhs(x)) else {
            continue;
        };
        let negative = curve.field.neg(y);
        if curve.field.to_biguint(negative) < curve.field.to_biguint(y) {
            y = negative;
        }
        let sign_input =
            serde_json::json!({"domain":"curve25519_point_sign_sha256_v1","parts":inputs});
        if sha256(canonical(&sign_input)?.as_bytes())[0] & 1 == 1 {
            y = curve.field.neg(y);
        }
        let cleared = curve.mul_u64(&mut GroupOps::default(), Some((x, y)), curve.cofactor);
        if let Some(coordinates) = encode(curve, cleared) {
            return Ok(TargetSample {
                coordinates,
                point_counter,
                hash_counter,
            });
        }
    }
    Err(format!(
        "no nonidentity public target in {max_points} point attempts"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn target_laws_reject_a_mutated_curve_or_generator() {
        let original = Curve25519Wide::new();
        for variant in 0..3 {
            let mut curve = original.clone();
            match variant {
                0 => curve.r = BigUint::from(0u8),
                1 => curve.cofactor = 1,
                _ => curve.generator = curve.neg(curve.generator),
            }
            assert!(planted(&curve, 0, 0).is_err());
            assert!(public(&curve, 0, 0, 2).is_err());
            assert!(decode(&curve, &encode(&original, original.generator).unwrap()).is_err());
            assert!(mul_generator(&curve, 1u64.into()).is_none());
        }
    }
    #[test]
    fn actual_curve_full_width_scalar_roundtrip() {
        let c = Curve25519Wide::new();
        let scalar = ExactScalar::from_biguint(&(&c.r - 1u8)).unwrap();
        assert!(scalar.checked_u128().is_none());
        let encoded = mul_generator(&c, scalar).unwrap();
        assert_eq!(decode(&c, &encoded).unwrap(), c.neg(c.generator));
        assert!(mul_generator(&c, ExactScalar::from_biguint(&c.r).unwrap()).is_none());
        assert_eq!(
            encode(&c, c.generator).unwrap(),
            mul_generator(&c, 1u64.into()).unwrap()
        );
    }
    #[test]
    fn malformed_out_of_field_and_torsion_points_are_rejected() {
        let c = Curve25519Wide::new();
        let good = encode(&c, c.generator).unwrap();
        for x in [
            "".into(),
            "0x".into(),
            "-1".into(),
            "0xgg".into(),
            format!("0x{}", "0".repeat(65)),
            format!("0x{:x}", c.field.modulus()),
        ] {
            assert!(decode(&c, &[x, good[1].clone()]).is_err());
        }
        assert!(decode(&c, &["0x0".into(), "0x0".into()]).is_err());
        let shift = c.field.mul(
            c.field.from_u64(A25519),
            c.field.inv(c.field.from_u64(3)).unwrap(),
        );
        let torsion = Some((shift, c.field.zero()));
        assert!(c.on_curve(torsion));
        assert!(decode(&c, &encode(&c, torsion).unwrap())
            .unwrap_err()
            .contains("subgroup"));
    }
    #[test]
    fn planted_targets_replay_with_full_width_scalars() {
        let c = Curve25519Wide::new();
        for index in 0..4 {
            let (sample, scalar) = planted(&c, 20261105, index).unwrap();
            assert!(scalar.checked_u128().is_none());
            assert!(scalar.to_biguint() < c.r);
            assert_eq!(
                (sample.clone(), scalar),
                planted(&c, 20261105, index).unwrap()
            );
            assert_eq!(sample.coordinates, mul_generator(&c, scalar).unwrap());
            assert!(decode(&c, &sample.coordinates).unwrap().is_some());
        }
    }
    #[test]
    fn public_targets_replay_and_have_prime_subgroup_membership() {
        let c = Curve25519Wide::new();
        let mut points = std::collections::BTreeSet::new();
        for index in 0..4 {
            let sample = public(&c, 20261105, index, 100).unwrap();
            assert_eq!(sample, public(&c, 20261105, index, 100).unwrap());
            assert!(decode(&c, &sample.coordinates).unwrap().is_some());
            assert!(points.insert(sample.coordinates));
        }
        assert!(public(&c, 20261105, 0, 0).is_err());
    }
}
