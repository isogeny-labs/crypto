//! Bounded two-summand relation collection on the actual Curve25519 subgroup.
//! The caller provides public factor-base points; no logarithm vector is accepted.
//! This is the generic pair-search baseline, with complete relation admission.
use crate::cryptanalysis::curve25519_wide::{
    Curve25519Wide, WideMatrixWork, WidePoint, WideRelationSystem,
};
use crate::cryptanalysis::ecbench::exact_sampling::derive_below;
use crate::cryptanalysis::ecbench::exact_scalar::ExactScalar;
use crate::cryptanalysis::fp_backend::PrimeField;
use crate::cryptanalysis::ic_boundary::GroupOps;
use num_bigint::BigUint;
use std::collections::HashMap;

/// A public factor base: G followed by cofactor-cleared lifts of x=0,1,...
/// Distinct modulo sign. No hidden scalar is used to generate a lift.
#[derive(Clone, Debug)]
pub struct FactorBase {
    pub points: Vec<WidePoint>,
    pub complete: bool,
    pub coordinates_tried: u64,
    pub sqrt_calls: u64,
    pub ops: GroupOps,
}

pub fn build_factor_base(
    curve: &Curve25519Wide,
    size: usize,
    max_scan: u64,
) -> Result<FactorBase, String> {
    if !(1..=128).contains(&size) || max_scan > 1_000_000 {
        return Err("factor base requires 1..=128 points and at most1000000 x coordinates".into());
    }
    let mut points = vec![curve.generator];
    let mut signed = std::collections::HashSet::new();
    signed.insert(curve.generator);
    signed.insert(curve.neg(curve.generator));
    let mut ops = GroupOps::default();
    let mut tried = 0;
    while points.len() < size && tried < max_scan {
        let x = curve.field.from_u64(tried);
        tried += 1;
        let rhs = curve.rhs(x);
        // rhs(x) has two field multiplications and one squaring. Root calls
        // are retained separately; their internal work is not inferred.
        ops.field_ops.muls += 2;
        ops.field_ops.sqrs += 1;
        if let Some(y) = curve.field.sqrt(rhs) {
            let point = curve.mul_u64(&mut ops, Some((x, y)), curve.cofactor);
            if point.is_some() && !signed.contains(&point) {
                signed.insert(point);
                signed.insert(curve.neg(point));
                points.push(point);
            }
        }
    }
    Ok(FactorBase {
        complete: points.len() == size,
        points,
        coordinates_tried: tried,
        sqrt_calls: tried,
        ops,
    })
}

#[derive(Clone, Debug)]
pub struct Outcome {
    pub recovered: Option<BigUint>,
    pub rank: usize,
    pub trials: u64,
    pub relations: u64,
    pub lookups: u64,
    pub scalar_hash_draws: u64,
    pub setup: GroupOps,
    pub search: GroupOps,
    pub admission: GroupOps,
    pub recovery: GroupOps,
    pub matrix: WideMatrixWork,
}

/// Scan signed factor-base summands. Hash collisions use full point equality.
/// Duplicate representations are harmless; only one valid decomposition is needed.
fn decompose(
    curve: &Curve25519Wide,
    base: &[WidePoint],
    signed: &HashMap<WidePoint, (usize, bool)>,
    point: WidePoint,
    ops: &mut GroupOps,
    lookups: &mut u64,
) -> Option<Vec<BigUint>> {
    for (i, &p) in base.iter().enumerate() {
        for negative in [false, true] {
            let left = if negative { curve.neg(p) } else { p };
            let remainder = curve.add(ops, point, curve.neg(left));
            *lookups += 1;
            if let Some(&(j, right_negative)) = signed.get(&remainder) {
                let mut row = vec![BigUint::from(0u8); base.len()];
                for (column, minus) in [(i, negative), (j, right_negative)] {
                    if minus {
                        row[column] = (&row[column] + &curve.r - 1u8) % &curve.r;
                    } else {
                        row[column] = (&row[column] + 1u8) % &curve.r;
                    }
                }
                return Some(row);
            }
        }
    }
    None
}

/// At most `max_trials` relation probes; each uses at most 2*base.len()
/// full-point lookups. Trial zero uses Q; later probes use independent named
/// rejection-sampled a in [0,r) and b in [1,r), then form aG+bQ.
/// The only initial logarithm admitted is log_G(G)=1, when G is in the base.
pub fn collect(
    curve: &Curve25519Wide,
    base: &[WidePoint],
    target: WidePoint,
    seed: u64,
    max_trials: u64,
) -> Result<Outcome, String> {
    if base.is_empty() || base.len() > 128 || max_trials > 1_000_000 {
        return Err(
            "pair collector requires 1..=128 base points and at most 1000000 trials".into(),
        );
    }
    let mut setup = GroupOps::default();
    let mut system = WideRelationSystem::new(curve, base.to_vec(), target, &mut setup)?;
    let mut signed = HashMap::new();
    for (i, &point) in base.iter().enumerate() {
        signed.entry(point).or_insert((i, false));
        signed.entry(curve.neg(point)).or_insert((i, true));
    }
    let mut admission = GroupOps::default();
    if let Some(i) = base.iter().position(|p| *p == curve.generator) {
        let mut row = vec![BigUint::from(0u8); base.len()];
        row[i] = BigUint::from(1u8);
        system.admit(
            &row,
            &BigUint::from(1u8),
            &BigUint::from(0u8),
            &mut admission,
        )?;
    }
    let r = ExactScalar::from_biguint(&curve.r).ok_or("order exceeds 256 bits")?;
    let rm1 = ExactScalar::from_biguint(&(&curve.r - 1u8)).ok_or("order exceeds 256 bits")?;
    let mut search = GroupOps::default();
    let mut recovery = GroupOps::default();
    let mut recovered = system.recover(&mut recovery);
    let (mut trials, mut relations, mut lookups, mut scalar_hash_draws) = (0, 0, 0, 0);
    while recovered.is_none() && trials < max_trials {
        let (a, b, point) = if trials == 0 {
            (BigUint::from(0u8), BigUint::from(1u8), target)
        } else {
            let seed_part = seed.to_string();
            let trial_part = trials.to_string();
            let parts = [seed_part, trial_part];
            let (a, ca) = derive_below("curve25519_pair_relation_a_v1", &parts, r, 128)?;
            let (b, cb) = derive_below("curve25519_pair_relation_b_v1", &parts, rm1, 128)?;
            scalar_hash_draws += ca + cb + 2;
            let (a, b) = (a.to_biguint(), b.to_biguint() + 1u8);
            let ag = curve.mul_biguint(&mut search, curve.generator, &a);
            let bq = curve.mul_biguint(&mut search, target, &b);
            let point = curve.add(&mut search, ag, bq);
            (a, b, point)
        };
        trials += 1;
        if let Some(row) = decompose(curve, base, &signed, point, &mut search, &mut lookups) {
            relations += 1;
            if system.admit(&row, &a, &b, &mut admission)? {
                recovered = system.recover(&mut recovery);
            }
        }
    }
    Ok(Outcome {
        recovered,
        rank: system.rank(),
        trials,
        relations,
        lookups,
        scalar_hash_draws,
        setup,
        search,
        admission,
        recovery,
        matrix: system.matrix_work(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lifted_factor_base_is_bounded_reproducible_and_in_the_subgroup() {
        let curve = Curve25519Wide::new();
        let a = build_factor_base(&curve, 8, 128).unwrap();
        let b = build_factor_base(&curve, 8, 128).unwrap();
        assert!(a.complete);
        assert_eq!(a.points, b.points);
        assert_eq!(a.ops, b.ops);
        assert_eq!(a.sqrt_calls, a.coordinates_tried);
        assert!(a.coordinates_tried <= 128);
        assert_eq!(a.points[0], curve.generator);
        for (i, &point) in a.points.iter().enumerate() {
            assert!(point.is_some());
            assert!(curve.on_curve(point));
            assert!(curve
                .mul_biguint(&mut GroupOps::default(), point, &curve.r)
                .is_none());
            for &other in &a.points[..i] {
                assert!(point != other && point != curve.neg(other));
            }
        }
        assert!(!build_factor_base(&curve, 2, 0).unwrap().complete);
        assert!(build_factor_base(&curve, 1, 0).unwrap().complete);
        assert!(build_factor_base(&curve, 129, 128).is_err());
    }

    #[test]
    fn public_input_recovery_includes_high_bit_scalar_and_repeated_summands() {
        let curve = Curve25519Wide::new();
        for negative in [false, true] {
            let twice = curve.double(&mut GroupOps::default(), curve.generator);
            let target = if negative { curve.neg(twice) } else { twice };
            let outcome = collect(&curve, &[curve.generator], target, 7, 1).unwrap();
            let expected = if negative {
                &curve.r - 2u8
            } else {
                BigUint::from(2u8)
            };
            assert_eq!(outcome.recovered, Some(expected));
            assert_eq!((outcome.trials, outcome.relations, outcome.rank), (1, 1, 2));
            assert!(outcome.setup.scalar_mults > 0);
            assert!(outcome.admission.scalar_mults > 0);
            assert!(outcome.recovery.scalar_mults > 0);
            assert!(outcome.matrix.inversions > 0);
        }
    }
    #[test]
    fn bounded_failure_replays_without_using_the_target_log() {
        let curve = Curve25519Wide::new();
        let target = curve.mul_biguint(
            &mut GroupOps::default(),
            curve.generator,
            &((BigUint::from(1u8) << 200usize) + 19u8),
        );
        let a = collect(&curve, &[curve.generator], target, 9, 4).unwrap();
        let b = collect(&curve, &[curve.generator], target, 9, 4).unwrap();
        assert!(a.recovered.is_none());
        assert_eq!((a.trials, a.relations, a.rank), (4, 0, 1));
        assert_eq!(a.search, b.search);
        assert_eq!(a.matrix, b.matrix);
        assert_eq!(a.lookups, 8);
        assert_eq!(a.scalar_hash_draws, b.scalar_hash_draws);
        assert!(a.scalar_hash_draws >= 6);
        let zero = collect(&curve, &[curve.generator], target, 9, 0).unwrap();
        assert_eq!(zero.trials, 0);
        assert!(zero.recovered.is_none());
    }
    #[test]
    fn rejects_invalid_public_inputs_and_resource_caps() {
        let curve = Curve25519Wide::new();
        assert!(collect(&curve, &[], curve.generator, 0, 1).is_err());
        assert!(collect(&curve, &[None], curve.generator, 0, 1).is_err());
        assert!(collect(&curve, &[curve.generator], None, 0, 1).is_err());
        assert!(collect(&curve, &[curve.generator], curve.generator, 0, 1_000_001).is_err());
    }
}
