//! Versioned, replayable factor-base populations, with exact finite domains.
//!
//! GGMP, sections 3--5, explains why coordinate-domain size, subgroup
//! membership, Frobenius columns and solver cost are separate quantities:
//! https://pure-oai.bham.ac.uk/ws/portalfiles/portal/104974339/On_Index_Calculus_Algorithms_for_Subfield_Curves.pdf
//! In particular, projecting a point changes its x coordinate. Literal
//! subspace policies here intersect with <G>; they never cofactor-project.
use super::curves::{self, Arithmetic, CyclicMembership, WPoint};
use super::types::{CaseConfig, FactorBasePolicy};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashMap, HashSet};

const POINT_CAP: usize = 4096;
const DOMAIN_CAP: usize = 61 * (1 << 12);

/// This stream does not depend on the target, key index, or generator index.
/// Hashing avoids disclosing the target-generation seed through invertible
/// seed mixing. The derived seed is restricted construction provenance.
pub(super) fn sampling_seed(config: &CaseConfig) -> u64 {
    let bytes = serde_json::to_vec(&json!([
        "ml-dataset-factor-base-control-v1",
        config.seed,
        config.curve_seed,
        config.family,
        config.n,
        config.twist
    ]))
    .expect("integer/string serialization");
    let digest = super::digest(&bytes);
    u64::from_str_radix(&digest[..16], 16).expect("SHA256 hexadecimal")
}

#[derive(Default)]
struct Counters {
    sampling_draws: u64,
    domain_xor_calls: u64,
    domain_frobenius_steps: u64,
    candidate_coordinates: u64,
    lifted_points: u64,
    projection_candidates: u64,
    subgroup_checks: u64,
    membership_baby_add_calls: u64,
    membership_giant_add_calls: u64,
    membership_scalar_replays: u64,
    rejected_identity: u64,
    rejected_wrong_subgroup: u64,
    rejected_duplicate: u64,
    rejected_point_cap: u64,
}
impl Counters {
    fn json(&self) -> Value {
        json!({"sampling_draws":self.sampling_draws,"domain_xor_calls":self.domain_xor_calls,
            "domain_frobenius_steps":self.domain_frobenius_steps,
            "candidate_coordinates":self.candidate_coordinates,"lifted_points":self.lifted_points,
            "projection_candidates":self.projection_candidates,"subgroup_checks":self.subgroup_checks,
            "membership_baby_add_calls":self.membership_baby_add_calls,
            "membership_giant_add_calls":self.membership_giant_add_calls,
            "membership_scalar_replays":self.membership_scalar_replays,
            "rejected_identity":self.rejected_identity,"rejected_wrong_subgroup":self.rejected_wrong_subgroup,
            "rejected_duplicate":self.rejected_duplicate,"rejected_point_cap":self.rejected_point_cap,
            "scope":"named native construction operation calls, not a complete cost total; excludes field-operation and hardware-instruction counts; symmetry witnesses are outside these counters; isolated factor-base wall time is not measured"})
    }
}

fn span(basis: &[u64]) -> Vec<u64> {
    let mut values = vec![0];
    for &b in basis {
        let old = values.len();
        for i in 0..old {
            values.push(values[i] ^ b);
        }
    }
    values.sort_unstable();
    values
}

fn coordinate_units(
    c: &Arithmetic,
    basis: &[u64],
    frobenius: bool,
    k: u32,
    counters: &mut Counters,
) -> Result<Vec<Vec<u64>>, String> {
    let seed = span(basis);
    counters.domain_xor_calls += seed.len() as u64 - 1;
    if !frobenius {
        return Ok(seed.into_iter().map(|x| vec![x]).collect());
    }
    let Arithmetic::Binary { fast, .. } = c else {
        return Err("binary coordinate policy on prime field".into());
    };
    let action_order = fast.n / k;
    let mut seen = BTreeSet::new();
    let mut units = Vec::new();
    for x in seed {
        if seen.contains(&x) {
            continue;
        }
        let mut orbit = Vec::new();
        let mut next = x;
        for _ in 0..action_order {
            orbit.push(next);
            next = fast.gf.sqr_k(next, k);
            counters.domain_frobenius_steps += 1;
            if next == x {
                break;
            }
        }
        if next != x {
            return Err("coordinate Frobenius orbit failed to close".into());
        }
        orbit.sort_unstable();
        for &v in &orbit {
            seen.insert(v);
        }
        if seen.len() > DOMAIN_CAP {
            return Err("coordinate domain exceeds fixed allocation bound".into());
        }
        units.push(orbit);
    }
    units.sort_by_key(|unit| unit[0]);
    Ok(units)
}

struct Membership<'a> {
    c: &'a Arithmetic,
    g: WPoint,
    r: u64,
    diagnostic: Option<CyclicMembership>,
    reference: bool,
}
impl Membership<'_> {
    fn check(&self, p: WPoint, counters: &mut Counters) -> Result<(bool, Option<u64>), String> {
        counters.subgroup_checks += 1;
        if p.is_none() {
            return Ok((false, None));
        }
        if !self.c.on_curve(p) {
            return Err("factor-base candidate off curve".into());
        }
        let killed = if self.reference {
            self.c.reference_mul(p, self.r)
        } else {
            self.c.mul(p, self.r)
        };
        if killed.is_some() {
            return Ok((false, None));
        }
        if let Some(test) = &self.diagnostic {
            let (log, additions) = test.logarithm_with_reference(self.c, p, self.reference);
            counters.membership_giant_add_calls += additions;
            if let Some(log) = log {
                counters.membership_scalar_replays += 1;
                // Independently bind the diagnostic logarithm to the actual G.
                if self.c.reference_mul(self.g, log) != p {
                    return Err("cyclic membership scalar replay failed".into());
                }
            }
            Ok((log.is_some(), log))
        } else {
            Ok((true, None))
        }
    }
}

/// Construct a v2 base. Private diagnostic logarithms are returned separately.
/// `reference=true` reconstructs using the generic curve group operations.
pub(super) fn build(
    public: &Value,
    policy: &FactorBasePolicy,
    requested: usize,
    seed: u64,
    reference: bool,
) -> Result<(Value, Vec<Value>), String> {
    let n: u32 = public["n"]
        .as_u64()
        .and_then(|n| n.try_into().ok())
        .ok_or("invalid field size")?;
    policy.validate(n)?;
    if !(1..=POINT_CAP).contains(&requested) {
        return Err("requested factor base outside 1..4096".into());
    }
    let c = curves::arithmetic(public)?;
    let r = curves::number(&public["curve"]["subgroup_order"], "subgroup order")?;
    let order = curves::number(&public["curve"]["group_order"], "group order")?;
    let h = curves::number(&public["curve"]["cofactor"], "cofactor")?;
    if r <= 2 || order == 0 || order % r != 0 || order / r != h {
        return Err("invalid factor-base subgroup arithmetic".into());
    }
    let generator = curves::point_words(&public["generator"])?;
    let k: u32 = public["symmetry"]["frobenius_power"]
        .as_u64()
        .and_then(|v| v.try_into().ok())
        .ok_or("invalid Frobenius power")?;
    let expected_k = match &c {
        Arithmetic::Prime(_) => 1,
        Arithmetic::Binary { fast, b } => (1..=n)
            .find(|&d| {
                n.is_multiple_of(d)
                    && fast.gf.sqr_k(fast.a, d) == fast.a
                    && fast.gf.sqr_k(*b, d) == *b
            })
            .ok_or("coefficient field missing")?,
    };
    if k != expected_k || public["symmetry"]["coefficient_field_degree"].as_u64() != Some(k as u64)
    {
        return Err("factor-base Frobenius does not match coefficient field".into());
    }
    let binary_policy = matches!(
        policy,
        FactorBasePolicy::BinarySubspaceIntersection { .. }
            | FactorBasePolicy::BinaryFrobeniusUnionIntersection { .. }
    );
    let union = matches!(
        policy,
        FactorBasePolicy::BinaryFrobeniusUnionIntersection { .. }
    );
    let projected = !binary_policy;
    let mut counters = Counters::default();
    // r^2 | #E bounds r <= sqrt(#E), so this diagnostic has a bounded
    // baby table even at n=61; public-order verification precedes replay.
    let diagnostic = if h.is_multiple_of(r) {
        if r > 1u64 << 32 {
            return Err("cyclic membership diagnostic exceeds word-field bound".into());
        }
        Some(CyclicMembership::new_with_reference(
            &c, generator, r, reference,
        ))
    } else {
        None
    };
    counters.membership_baby_add_calls = diagnostic.as_ref().map_or(0, |d| d.stride);
    let membership = Membership {
        c: &c,
        g: generator,
        r,
        diagnostic,
        reference,
    };
    let mut reason = None;
    let unsupported = if binary_policy && matches!(&c, Arithmetic::Prime(_)) {
        reason = Some("binary coordinate policy is not defined over a prime field");
        true
    } else if union
        && !membership
            .check(c.frobenius(generator, k), &mut counters)?
            .0
    {
        reason = Some("model Frobenius does not preserve the selected cyclic subgroup");
        true
    } else {
        false
    };

    let (candidate_limit, units, finite_domain) = match policy {
        FactorBasePolicy::Legacy {} => {
            let end = (requested as u64 * 32 + 128).min(c.q());
            (
                end - 1,
                (1..end).map(|x| vec![x]).collect::<Vec<_>>(),
                false,
            )
        }
        FactorBasePolicy::SampledXProjectedControl { candidate_limit } => {
            let mut rng = StdRng::seed_from_u64(seed);
            counters.sampling_draws = *candidate_limit;
            // Sampling is with replacement; duplicate draws are visible and
            // charged. Each draw is uniform on all field coordinates.
            (
                *candidate_limit,
                (0..*candidate_limit)
                    .map(|_| vec![rng.gen_range(0..c.q())])
                    .collect(),
                false,
            )
        }
        FactorBasePolicy::BinarySubspaceIntersection {
            basis,
            candidate_limit,
        }
        | FactorBasePolicy::BinaryFrobeniusUnionIntersection {
            basis,
            candidate_limit,
        } => (
            *candidate_limit,
            if unsupported {
                Vec::new()
            } else {
                coordinate_units(&c, basis, union, k, &mut counters)?
            },
            true,
        ),
    };
    let total_coordinates: usize = units.iter().map(Vec::len).sum();
    let mut points = Vec::<WPoint>::new();
    let mut seen = HashSet::new();
    let mut candidates = Vec::new();
    let mut private_logs = Vec::new();
    let mut exhausted_units = !unsupported;
    let mut hit_point_cap = false;
    let mut fulfilled = false;
    if !unsupported {
        for (unit_index, xs) in units.iter().enumerate() {
            if counters.candidate_coordinates + xs.len() as u64 > candidate_limit {
                exhausted_units = false;
                break;
            }
            let mut pending = Vec::<(WPoint, Option<u64>)>::new();
            let mut pending_seen = HashSet::new();
            let mut rows = Vec::new();
            for &x in xs {
                counters.candidate_coordinates += 1;
                let mut lifts = Vec::new();
                for input in c.lift(x) {
                    counters.lifted_points += 1;
                    let (point, multiplier) = if projected {
                        counters.projection_candidates += 1;
                        curves::prime_projection(&c, input, order, r, reference)
                    } else {
                        (input, 1)
                    };
                    let (member, log) = membership.check(point, &mut counters)?;
                    let rejection = if point.is_none() {
                        counters.rejected_identity += 1;
                        Some("identity")
                    } else if !member {
                        counters.rejected_wrong_subgroup += 1;
                        Some("wrong_cyclic_subgroup")
                    } else if seen.contains(&point) || !pending_seen.insert(point) {
                        counters.rejected_duplicate += 1;
                        Some("duplicate")
                    } else {
                        pending.push((point, log));
                        None
                    };
                    lifts.push(json!({"lifted":curves::point_json(input),"point":curves::point_json(point),
                        "projection_multiplier":multiplier.to_string(),"cyclic_subgroup_member":member,
                        "accepted":rejection.is_none(),"rejection":rejection}));
                }
                rows.push(json!({"x":x.to_string(),"lifts":lifts}));
            }
            // Atomic sign/Frobenius units: retain all points of a processed
            // unit or none, including when a resource bound stops construction.
            let admitted = points.len() + pending.len() <= POINT_CAP;
            if admitted {
                for (p, log) in pending {
                    seen.insert(p);
                    points.push(p);
                    private_logs.push(log.map(|x| x.to_string()));
                }
            } else {
                counters.rejected_point_cap += pending.len() as u64;
                for row in &mut rows {
                    for lift in row["lifts"].as_array_mut().expect("built array") {
                        if lift["accepted"] == true {
                            lift["accepted"] = json!(false);
                            lift["rejection"] = json!("point_cap");
                        }
                    }
                }
                hit_point_cap = true;
                exhausted_units = false;
            }
            candidates.push(json!({"unit":unit_index,"admitted":admitted,"coordinates":rows}));
            if !admitted {
                break;
            }
            if !finite_domain && (points.len() >= requested || points.len() as u64 == r - 1) {
                fulfilled = true;
                exhausted_units = unit_index + 1 == units.len();
                break;
            }
        }
    }
    if finite_domain && counters.candidate_coordinates < total_coordinates as u64 {
        exhausted_units = false;
    }
    let domain_complete = finite_domain && exhausted_units && !hit_point_cap && !unsupported;
    let status = if unsupported {
        "unsupported"
    } else if domain_complete || fulfilled {
        "complete"
    } else {
        "capped"
    };
    let lookup: HashMap<_, _> = points
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();
    let negation: Vec<_> = points
        .iter()
        .map(|&p| lookup.get(&c.neg(p)).copied())
        .collect();
    let frobenius: Vec<_> = points
        .iter()
        .map(|&p| lookup.get(&c.frobenius(p, k)).copied())
        .collect();
    if negation.iter().any(Option::is_none) || (union && frobenius.iter().any(Option::is_none)) {
        return Err("construction violated its atomic symmetry units".into());
    }
    let action_order = if matches!(&c, Arithmetic::Prime(_)) {
        1
    } else {
        n / k
    };
    let mut orbits = Vec::new();
    for &p in &points {
        let mut orbit = Vec::new();
        let mut next = p;
        for _ in 0..action_order {
            if !c.on_curve(next) {
                return Err("Frobenius left the model".into());
            }
            orbit.push(curves::point_json(next));
            next = c.frobenius(next, k);
            if next == p {
                break;
            }
        }
        if next != p {
            return Err("point Frobenius orbit did not close".into());
        }
        orbits.push(json!({"stabilizer_size":action_order as usize / orbit.len(),"points":orbit,"closed":true}));
    }
    Ok((
        json!({"schema":"ml-ecdlp-factor-base/v2","rule":policy.name(),"policy":policy,
        "points":points.iter().copied().map(curves::point_json).collect::<Vec<_>>(),
        "requested_size":requested,"actual_size":points.len(),"point_cap":POINT_CAP,
        "size_policy":if finite_domain {"complete declared coordinate domain intersected with cyclic subgroup, or explicit capped prefix of whole symmetry units; requested_size is not a truncation rule"} else {"stop after whole sign unit reaches requested_size or subgroup cardinality; otherwise capped"},
        "subgroup_treatment":if projected {"prime-primary-projected-image"} else {"literal-coordinate-domain-intersection"},
        "negation_closed":true,"negation_action":negation,"frobenius_action":frobenius,
        "frobenius_power":k,"frobenius_closed":frobenius.iter().all(Option::is_some),"orbits":orbits,
        "construction_status":status,"unsupported_reason":reason,
        "coordinate_domain_complete":domain_complete,
        "declared_coordinate_count":if finite_domain && !unsupported {Some(total_coordinates)} else {None},
        "construction_candidates":candidates,"construction_counters":counters.json(),
        "construction_context":{"sampling_seed":seed.to_string(),"sampling_algorithm":if matches!(policy,FactorBasePolicy::SampledXProjectedControl{..}) {"rand-StdRng-gen_range-with-replacement-v1"} else {"not-used-by-this-policy"},"seed_access":"restricted provenance; not a model feature"},
        "cyclic_membership_method":if membership.diagnostic.is_some(){"bounded-bsgs-diagnostic"}else{"unique-prime-subgroup"},
        "known_log_features":false}),
        private_logs.into_iter().map(|v| json!(v)).collect(),
    ))
}

/// Replay all public construction decisions. Neither saved acceptance bits nor
/// diagnostic logarithms drive reconstruction; generic group arithmetic checks
/// the equation and subgroup tests, and every metadata field is compared.
pub(super) fn verify(public: &Value, base: &Value) -> Result<(), String> {
    if base["schema"] != "ml-ecdlp-factor-base/v2" {
        return Err("invalid policy base schema".into());
    }
    curves::verify_public(public)?;
    let policy: FactorBasePolicy =
        serde_json::from_value(base["policy"].clone()).map_err(|e| e.to_string())?;
    let requested: usize = base["requested_size"]
        .as_u64()
        .and_then(|n| n.try_into().ok())
        .ok_or("invalid requested size")?;
    let seed = curves::number(
        &base["construction_context"]["sampling_seed"],
        "policy sampling seed",
    )?;
    let (expected, _) = build(public, &policy, requested, seed, true)?;
    if expected != *base {
        return Err("factor-base policy construction or metadata replay mismatch".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn config(family: &str, n: u32, twist: bool) -> CaseConfig {
        CaseConfig {
            family: family.into(),
            n,
            m: 2,
            twist,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 8,
            factor_base_policy: None,
            degree_bound: 3,
            work_limit: 1000,
            solver: "enumerate".into(),
        }
    }
    fn point_set(base: &Value) -> HashSet<WPoint> {
        base["points"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| curves::point_words(v).unwrap())
            .collect()
    }
    fn with_policy(
        mut cfg: CaseConfig,
        policy: FactorBasePolicy,
    ) -> (CaseConfig, super::super::types::CaseData) {
        cfg.factor_base_policy = Some(policy);
        let case = curves::generate(&cfg).unwrap();
        (cfg, case)
    }
    #[test]
    fn explicit_legacy_preserves_actual_domain_on_every_family_and_twist() {
        for family in ["koblitz", "binary", "prime"] {
            for twist in [false, true] {
                let cfg = config(family, 11, twist);
                let legacy = curves::generate(&cfg).unwrap();
                let (_, explicit) = with_policy(cfg, FactorBasePolicy::Legacy {});
                assert_eq!(legacy.public_instance, explicit.public_instance);
                assert_eq!(legacy.private_witness, explicit.private_witness);
                for field in [
                    "points",
                    "negation_action",
                    "frobenius_action",
                    "frobenius_power",
                    "orbits",
                ] {
                    assert_eq!(
                        legacy.factor_base[field], explicit.factor_base[field],
                        "{family}/{twist}/{field}"
                    );
                }
                assert_eq!(legacy.factor_base["schema"], "ml-ecdlp-factor-base/v1");
                assert_eq!(explicit.factor_base["schema"], "ml-ecdlp-factor-base/v2");
            }
        }
    }
    #[test]
    fn policies_preserve_curve_generator_target_and_scalar() {
        for family in ["koblitz", "binary", "prime"] {
            for twist in [false, true] {
                let cfg = config(family, 11, twist);
                let legacy = curves::generate(&cfg).unwrap();
                for policy in [
                    FactorBasePolicy::SampledXProjectedControl {
                        candidate_limit: 64,
                    },
                    FactorBasePolicy::BinarySubspaceIntersection {
                        basis: vec![1, 2, 4],
                        candidate_limit: 8,
                    },
                    FactorBasePolicy::BinaryFrobeniusUnionIntersection {
                        basis: vec![1, 2],
                        candidate_limit: 128,
                    },
                ] {
                    let (_, candidate) = with_policy(cfg.clone(), policy);
                    assert_eq!(legacy.public_instance, candidate.public_instance);
                    assert_eq!(
                        legacy.private_witness["scalar"],
                        candidate.private_witness["scalar"]
                    );
                    assert_eq!(
                        legacy.private_witness["generator_multiplier"],
                        candidate.private_witness["generator_multiplier"]
                    );
                }
            }
        }
    }
    #[test]
    fn sampled_control_is_independent_of_key_and_generator_indices() {
        let cfg = config("prime", 11, false);
        let (cfg, first) = with_policy(
            cfg,
            FactorBasePolicy::SampledXProjectedControl {
                candidate_limit: 64,
            },
        );
        let mut changed = cfg.clone();
        changed.key_index = 2;
        changed.generator_index = 2;
        assert_eq!(sampling_seed(&cfg), sampling_seed(&changed));
        let second = curves::generate(&changed).unwrap();
        assert_eq!(first.factor_base, second.factor_base);
        assert_eq!(
            first.factor_base["construction_counters"]["sampling_draws"],
            64
        );
        assert_ne!(
            first.public_instance["generator"],
            second.public_instance["generator"]
        );
    }
    #[test]
    fn complete_subspace_matches_exhaustive_subgroup_intersection() {
        for twist in [false, true] {
            let (_, case) = with_policy(
                config("koblitz", 5, twist),
                FactorBasePolicy::BinarySubspaceIntersection {
                    basis: vec![1, 2, 4],
                    candidate_limit: 8,
                },
            );
            let c = curves::arithmetic(&case.public_instance).unwrap();
            let g = curves::point_words(&case.public_instance["generator"]).unwrap();
            let r = curves::number(&case.public_instance["curve"]["subgroup_order"], "r").unwrap();
            let expected: HashSet<_> = (1..r)
                .map(|i| c.reference_mul(g, i))
                .filter(|p| p.is_some_and(|(x, _)| x < 8))
                .collect();
            assert_eq!(point_set(&case.factor_base), expected);
            assert_eq!(case.factor_base["coordinate_domain_complete"], true);
            assert_eq!(case.factor_base["construction_status"], "complete");
            assert_eq!(
                case.factor_base["construction_counters"]["projection_candidates"],
                0
            );
            assert_eq!(
                case.factor_base["construction_counters"]["candidate_coordinates"],
                8
            );
        }
    }
    #[test]
    fn full_coordinate_space_returns_exact_nonzero_cyclic_subgroup() {
        for twist in [false, true] {
            let (_, case) = with_policy(
                config("koblitz", 5, twist),
                FactorBasePolicy::BinarySubspaceIntersection {
                    basis: vec![1, 2, 4, 8, 16],
                    candidate_limit: 32,
                },
            );
            let c = curves::arithmetic(&case.public_instance).unwrap();
            let g = curves::point_words(&case.public_instance["generator"]).unwrap();
            let r = curves::number(&case.public_instance["curve"]["subgroup_order"], "r").unwrap();
            assert_eq!(
                point_set(&case.factor_base),
                (1..r).map(|i| c.reference_mul(g, i)).collect()
            );
            assert_eq!(case.factor_base["actual_size"], r - 1);
        }
    }
    #[test]
    fn repeated_prime_uses_actual_cyclic_subgroup_and_private_logs() {
        let (_, case) = with_policy(
            config("koblitz", 11, true),
            FactorBasePolicy::BinarySubspaceIntersection {
                basis: vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
                candidate_limit: 2048,
            },
        );
        let c = curves::arithmetic(&case.public_instance).unwrap();
        let g = curves::point_words(&case.public_instance["generator"]).unwrap();
        let r = curves::number(&case.public_instance["curve"]["subgroup_order"], "r").unwrap();
        assert_eq!(r, 23);
        assert_eq!(
            point_set(&case.factor_base),
            (1..r).map(|i| c.reference_mul(g, i)).collect()
        );
        assert_eq!(
            case.factor_base["cyclic_membership_method"],
            "bounded-bsgs-diagnostic"
        );
        assert!(
            case.factor_base["construction_counters"]["membership_baby_add_calls"]
                .as_u64()
                .unwrap()
                > 0
        );
        assert!(case.private_witness["factor_base_membership_logs"]
            .as_array()
            .unwrap()
            .iter()
            .all(Value::is_string));
        assert!(case.factor_base.get("membership_logs").is_none());
    }
    #[test]
    fn union_matches_exhaustive_subgroup_coordinate_predicate() {
        for twist in [false, true] {
            let (_, case) = with_policy(
                config("koblitz", 5, twist),
                FactorBasePolicy::BinaryFrobeniusUnionIntersection {
                    basis: vec![2],
                    candidate_limit: 32,
                },
            );
            let c = curves::arithmetic(&case.public_instance).unwrap();
            let Arithmetic::Binary { fast, .. } = &c else {
                panic!("binary")
            };
            let mut xs = HashSet::from([0]);
            let mut x = 2;
            for _ in 0..5 {
                xs.insert(x);
                x = fast.gf.sqr(x);
            }
            let g = curves::point_words(&case.public_instance["generator"]).unwrap();
            let r = curves::number(&case.public_instance["curve"]["subgroup_order"], "r").unwrap();
            let expected: HashSet<_> = (1..r)
                .map(|i| c.reference_mul(g, i))
                .filter(|p| p.is_some_and(|(x, _)| xs.contains(&x)))
                .collect();
            assert_eq!(point_set(&case.factor_base), expected);
            assert_eq!(case.factor_base["frobenius_closed"], true);
            assert_eq!(case.factor_base["coordinate_domain_complete"], true);
        }
    }
    #[test]
    fn union_caps_preserve_whole_orbits() {
        let (_, case) = with_policy(
            config("koblitz", 5, false),
            FactorBasePolicy::BinaryFrobeniusUnionIntersection {
                basis: vec![2],
                candidate_limit: 2,
            },
        );
        assert_eq!(case.factor_base["construction_status"], "capped");
        assert_eq!(case.factor_base["coordinate_domain_complete"], false);
        assert_eq!(
            case.factor_base["construction_counters"]["candidate_coordinates"],
            1
        );
        assert_eq!(case.factor_base["frobenius_closed"], true);
    }
    #[test]
    fn literal_subspace_cap_never_claims_complete_domain() {
        let (_, case) = with_policy(
            config("koblitz", 5, false),
            FactorBasePolicy::BinarySubspaceIntersection {
                basis: vec![1, 2, 4],
                candidate_limit: 2,
            },
        );
        assert_eq!(case.factor_base["construction_status"], "capped");
        assert_eq!(case.factor_base["coordinate_domain_complete"], false);
        assert_eq!(case.factor_base["declared_coordinate_count"], 8);
        assert_eq!(
            case.factor_base["construction_counters"]["candidate_coordinates"],
            2
        );
    }
    #[test]
    fn literal_domain_is_not_truncated_to_requested_point_count() {
        let mut cfg = config("koblitz", 5, false);
        cfg.factor_base_size = 1;
        let (_, case) = with_policy(
            cfg,
            FactorBasePolicy::BinarySubspaceIntersection {
                basis: vec![1, 2, 4, 8, 16],
                candidate_limit: 32,
            },
        );
        assert!(case.factor_base["actual_size"].as_u64().unwrap() > 1);
        assert_eq!(case.factor_base["coordinate_domain_complete"], true);
    }
    #[test]
    fn binary_policies_on_prime_are_explicitly_unsupported() {
        for twist in [false, true] {
            for policy in [
                FactorBasePolicy::BinarySubspaceIntersection {
                    basis: vec![1, 2],
                    candidate_limit: 4,
                },
                FactorBasePolicy::BinaryFrobeniusUnionIntersection {
                    basis: vec![1, 2],
                    candidate_limit: 4,
                },
            ] {
                let (_, case) = with_policy(config("prime", 11, twist), policy);
                assert_eq!(case.factor_base["construction_status"], "unsupported");
                assert_eq!(case.factor_base["points"], json!([]));
                assert_eq!(case.factor_base["coordinate_domain_complete"], false);
                assert!(case.factor_base["unsupported_reason"].is_string());
            }
        }
    }
    #[test]
    fn even_twist_uses_its_model_preserving_frobenius() {
        let (_, case) = with_policy(
            config("koblitz", 12, true),
            FactorBasePolicy::BinaryFrobeniusUnionIntersection {
                basis: vec![1, 2],
                candidate_limit: 64,
            },
        );
        let k = case.factor_base["frobenius_power"].as_u64().unwrap();
        assert!(k > 1);
        assert_eq!(
            case.public_instance["symmetry"]["coefficient_field_degree"],
            k
        );
        if case.factor_base["construction_status"] != "unsupported" {
            assert_eq!(case.factor_base["frobenius_closed"], true);
        }
    }
    #[test]
    fn reference_replay_rejects_candidate_policy_counter_and_status_tampering() {
        let (_, case) = with_policy(
            config("prime", 11, false),
            FactorBasePolicy::SampledXProjectedControl {
                candidate_limit: 64,
            },
        );
        for (pointer, value) in [
            ("/construction_status", json!("capped")),
            ("/construction_counters/candidate_coordinates", json!(999)),
            ("/construction_candidates/0/coordinates/0/x", json!("000")),
            ("/policy/candidate_limit", json!(63)),
            ("/negation_closed", json!(false)),
            ("/coordinate_domain_complete", json!(true)),
            ("/actual_size", json!(999)),
        ] {
            let mut bad = case.factor_base.clone();
            *bad.pointer_mut(pointer).unwrap() = value;
            assert!(verify(&case.public_instance, &bad).is_err(), "{pointer}");
        }
        let mut bad = case.factor_base.clone();
        bad["unexpected"] = json!(true);
        assert!(verify(&case.public_instance, &bad).is_err());
    }
    #[test]
    fn config_binding_rejects_valid_but_wrong_policy_seed_or_version() {
        let (cfg, case) = with_policy(
            config("prime", 11, false),
            FactorBasePolicy::SampledXProjectedControl {
                candidate_limit: 64,
            },
        );
        let mut different = cfg.clone();
        different.seed += 1;
        assert!(curves::verify_factor_base_config(
            &different,
            &case.public_instance,
            &case.factor_base
        )
        .is_err());
        different = cfg.clone();
        different.factor_base_policy = None;
        assert!(curves::verify_factor_base_config(
            &different,
            &case.public_instance,
            &case.factor_base
        )
        .is_err());
        different = cfg.clone();
        different.m = 3;
        assert!(curves::verify_factor_base_config(
            &different,
            &case.public_instance,
            &case.factor_base
        )
        .is_err());
        different = cfg;
        different.factor_base_policy = Some(FactorBasePolicy::Legacy {});
        assert!(curves::verify_factor_base_config(
            &different,
            &case.public_instance,
            &case.factor_base
        )
        .is_err());
    }
}
