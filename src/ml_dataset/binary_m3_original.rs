//! Shared original mathematical authentication only; no ordinary-binary dispatch.
//! Caller must authenticate source bytes and pre-admit existing setup resources.

#[derive(Clone, Copy)]
pub(super) enum Profile {
    Koblitz,
    #[allow(dead_code)] // next adapter is not activated by this extraction
    OrdinaryBinary,
}
impl Profile {
    fn family(self) -> &'static str {
        match self {
            Self::Koblitz => "koblitz",
            Self::OrdinaryBinary => "binary",
        }
    }
    fn accepts_b(self, b: Option<u64>) -> bool {
        match self {
            Self::Koblitz => b == Some(1),
            Self::OrdinaryBinary => b.is_some_and(|b| b != 0),
        }
    }
}

// Preserve the old setup envelope: at most 255 coordinate candidates, each
// with at most two lifts. V2 units may nest coordinates but cannot expand it.
fn bounded_candidates(
    config: &super::super::types::CaseConfig,
    public: &serde_json::Value,
    base: &serde_json::Value,
) -> Result<(), String> {
    use super::super::types::FactorBasePolicy;
    let error = || "Koblitz two-code setup candidate replay cap exceeded".to_owned();
    let rows = base["construction_candidates"]
        .as_array()
        .ok_or_else(error)?;
    if rows.len() > 255 {
        return Err(error());
    }
    let check_coordinate = |row: &serde_json::Value| -> Result<(), String> {
        if row["lifts"].as_array().is_none_or(|lifts| lifts.len() > 2) {
            return Err(error());
        }
        Ok(())
    };
    match base["schema"].as_str() {
        Some("ml-ecdlp-factor-base/v1") => {
            for row in rows {
                check_coordinate(row)?;
            }
        }
        Some("ml-ecdlp-factor-base/v2") => {
            let policy = config.factor_base_policy.as_ref().ok_or_else(error)?;
            policy.validate(config.n)?;
            // Replay builds candidate units before consuming them. Bound that
            // allocation/work as well as the retained consumed prefix.
            let expansion = match policy {
                FactorBasePolicy::Legacy {} => config
                    .factor_base_size
                    .checked_mul(32)
                    .and_then(|v| v.checked_add(128))
                    .and_then(|v| v.checked_sub(1)),
                FactorBasePolicy::SampledXProjectedControl { candidate_limit } => {
                    usize::try_from(*candidate_limit).ok()
                }
                FactorBasePolicy::BinarySubspaceIntersection { basis, .. } => {
                    1usize.checked_shl(basis.len() as u32)
                }
                FactorBasePolicy::BinaryFrobeniusUnionIntersection { basis, .. } => {
                    let k = public["symmetry"]["frobenius_power"]
                        .as_u64()
                        .ok_or_else(error)?;
                    if k == 0 || k > config.n as u64 || config.n as u64 % k != 0 {
                        return Err(error());
                    }
                    1usize
                        .checked_shl(basis.len() as u32)
                        .and_then(|v| v.checked_mul(config.n as usize / k as usize))
                }
            }
            .ok_or_else(error)?;
            if expansion > 255 {
                return Err(error());
            }
            let mut total = 0usize;
            for unit in rows {
                let coordinates = unit["coordinates"].as_array().ok_or_else(error)?;
                total = total.checked_add(coordinates.len()).ok_or_else(error)?;
                if total > 255 {
                    return Err(error());
                }
                for row in coordinates {
                    check_coordinate(row)?;
                }
            }
        }
        _ => return Err(error()),
    }
    Ok(())
}

/// Canonical original mathematical inputs for the initial two-code matrix
/// profile. Setup admission/accounting and persistence must precede runtime use.
pub(super) struct MathInput {
    pub curve: Box<crate::binary_ecc::BinaryCurve>,
    pub target: (u64, u64),
    pub codes: [u64; 2],
    pub signed: [(u64, u64); 4],
}

/// Validate public inputs using the existing independent curve/factor-base
/// replay. Never consult private_witness or solver-produced input artifacts.
/// The caller authenticates case provenance; this checks mathematical content.
pub(super) fn authenticate(
    profile: Profile,
    config: &super::super::types::CaseConfig,
    case: &super::super::types::CaseData,
) -> Result<MathInput, String> {
    use super::{curves, domain, Element, Group};
    use num_traits::ToPrimitive;
    use serde_json::json;
    if config.family != profile.family()
        || config.m != 3
        || !((23..=31).contains(&config.n) || (47..=61).contains(&config.n))
    {
        return Err("outside Koblitz m3 field/arity profile".into());
    }
    let public = &case.public_instance;
    if public["family"] != config.family
        || public["n"] != json!(config.n)
        || public["m"] != json!(config.m)
        || public["twist"] != json!(config.twist)
        || case.factor_base["requested_size"] != json!(config.factor_base_size)
    {
        return Err("Koblitz original public inputs differ from configuration".into());
    }
    let points = case.factor_base["points"]
        .as_array()
        .ok_or("missing original factor base")?;
    if points.len() > 4 || points.is_empty() {
        return Err("Koblitz two-code profile requires at most four original signed points".into());
    }
    bounded_candidates(config, public, &case.factor_base)?;
    // Full mathematical checks include actual twist, certified order,
    // irreducibility, subgroup checks and factor-base construction replay.
    // Their setup cost is not represented by native S4 reservation counters.
    curves::verify_public(public)?;
    curves::verify_factor_base_config(config, public, &case.factor_base)?;
    let group = Group::read(case)?;
    let Group::Binary(curve) = &group else {
        return Err("Koblitz profile requires actual binary curve".into());
    };
    if curve.m != config.n || !profile.accepts_b(curve.b.to_biguint().to_u64()) {
        return Err(match profile {
            Profile::Koblitz => "Koblitz S4 profile requires actual b=1 and original field degree",
            Profile::OrdinaryBinary => {
                "ordinary-binary resultant requires actual nonzero b and original field degree"
            }
        }
        .into());
    }
    let word = |p: &Element| -> Result<(u64, u64), String> {
        match p {
            Element::Binary(crate::binary_ecc::BinaryPoint::Affine { x, y }) => Ok((
                x.to_biguint().to_u64().ok_or("x exceeds u64")?,
                y.to_biguint().to_u64().ok_or("y exceeds u64")?,
            )),
            _ => Err("Koblitz original point must be affine".into()),
        }
    };
    let target = word(&group.point(&public["target"])?)?;
    let points = domain(&group, case)?;
    if points.len() != 4 {
        return Err(
            "Koblitz matrix profile requires two distinct x codes, not two signed points".into(),
        );
    }
    let mut signed = [(0, 0); 4];
    for (out, point) in signed.iter_mut().zip(&points) {
        *out = word(point)?;
    }
    signed.sort_unstable();
    if signed.windows(2).any(|p| p[0] == p[1])
        || signed[0].0 != signed[1].0
        || signed[2].0 != signed[3].0
        || signed[0].0 == signed[2].0
        || signed[0].1 ^ signed[0].0 != signed[1].1
        || signed[2].1 ^ signed[2].0 != signed[3].1
    {
        return Err("noncanonical original signed two-code domain".into());
    }
    let codes = [signed[0].0, signed[2].0];
    let Group::Binary(mut curve) = group else {
        unreachable!("validated binary group")
    };
    curve.irreducible.low_terms.sort_unstable();
    Ok(MathInput {
        curve,
        target,
        codes,
        signed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml_dataset::{
        curves,
        types::{CaseConfig, FactorBasePolicy},
    };
    use serde_json::{json, Value};
    fn config(family: &str, twist: bool) -> CaseConfig {
        CaseConfig {
            family: family.into(),
            n: 23,
            m: 3,
            twist,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 4,
            factor_base_policy: None,
            degree_bound: 3,
            work_limit: 16384,
            solver: "f4".into(),
        }
    }
    #[test]
    fn closed_profiles_keep_actual_b_and_reject_cross_family() {
        assert!(Profile::OrdinaryBinary.accepts_b(Some(1)));
        assert!(Profile::OrdinaryBinary.accepts_b(Some(2)));
        assert!(!Profile::OrdinaryBinary.accepts_b(Some(0)));
        assert!(!Profile::Koblitz.accepts_b(Some(2)));
        for twist in [false, true] {
            let cfg = config("binary", twist);
            let mut case = curves::generate(&cfg).unwrap();
            case.private_witness = Value::Null;
            let checked = authenticate(Profile::OrdinaryBinary, &cfg, &case).unwrap();
            use num_traits::ToPrimitive;
            assert_ne!(checked.curve.b.to_biguint().to_u64().unwrap(), 1);
            assert_eq!(
                checked.curve.b.to_biguint().to_string(),
                case.public_instance["curve"]["b"].as_str().unwrap()
            );
            assert_eq!(
                checked.curve.a.to_biguint().to_string(),
                case.public_instance["curve"]["a"].as_str().unwrap()
            );
            assert!(authenticate(Profile::Koblitz, &cfg, &case).is_err());
            for mutation in 0..4 {
                let mut bad = case.clone();
                match mutation {
                    0 => bad.public_instance["curve"]["b"] = json!("0"),
                    1 => bad.public_instance["twist"] = json!(!twist),
                    2 => bad.factor_base["points"][0] = bad.factor_base["points"][1].clone(),
                    _ => bad.public_instance["field"]["irreducible_low_terms"] = json!([0]),
                }
                assert!(authenticate(Profile::OrdinaryBinary, &cfg, &bad).is_err());
            }
        }
    }
    #[test]
    fn explicit_legacy_preserves_koblitz_math_and_policy_seed_binding() {
        for twist in [false, true] {
            let legacy = config("koblitz", twist);
            let old_case = curves::generate(&legacy).unwrap();
            let old = authenticate(Profile::Koblitz, &legacy, &old_case).unwrap();
            let mut explicit = legacy.clone();
            explicit.factor_base_policy = Some(FactorBasePolicy::Legacy {});
            let mut case = curves::generate(&explicit).unwrap();
            case.private_witness = Value::Null;
            let new = authenticate(Profile::Koblitz, &explicit, &case).unwrap();
            assert_eq!(old.target, new.target);
            assert_eq!(old.codes, new.codes);
            assert_eq!(old.signed, new.signed);
            assert_eq!(
                old.curve.irreducible.low_terms,
                new.curve.irreducible.low_terms
            );
            assert_eq!(old.curve.a, new.curve.a);
            assert_eq!(old.curve.b, new.curve.b);
            assert!(authenticate(Profile::Koblitz, &legacy, &case).is_err());
            explicit.seed += 1;
            assert!(authenticate(Profile::Koblitz, &explicit, &case).is_err());
        }
    }
    #[test]
    fn real_v2_recipes_replay_before_exact_domain_admission() {
        for twist in [false, true] {
            for policy in [
                FactorBasePolicy::SampledXProjectedControl {
                    candidate_limit: 128,
                },
                FactorBasePolicy::BinarySubspaceIntersection {
                    basis: vec![1, 2],
                    candidate_limit: 128,
                },
                FactorBasePolicy::BinaryFrobeniusUnionIntersection {
                    basis: vec![1, 2],
                    candidate_limit: 128,
                },
            ] {
                let mut cfg = config("koblitz", twist);
                cfg.factor_base_policy = Some(policy);
                let case = curves::generate(&cfg).unwrap();
                curves::verify_factor_base_config(&cfg, &case.public_instance, &case.factor_base)
                    .unwrap();
                bounded_candidates(&cfg, &case.public_instance, &case.factor_base).unwrap();
                let points = case.factor_base["points"].as_array().unwrap();
                let checked = authenticate(Profile::Koblitz, &cfg, &case);
                if points.len() == 4 {
                    let checked = checked.unwrap();
                    assert!(checked.codes[0] < checked.codes[1]);
                } else {
                    // Preserve the fixed natural domain; never truncate or resample
                    // a policy intersection to manufacture a two-code profile.
                    assert!(checked.is_err());
                }
            }
        }
    }
    #[test]
    fn every_v2_recipe_has_bounded_nested_candidate_admission() {
        let mut cfg = config("koblitz", false);
        let public = json!({"symmetry":{"frobenius_power":1}});
        let coordinate = json!({"x":"1","lifts":[{},{}]});
        let base = json!({"schema":"ml-ecdlp-factor-base/v2","construction_candidates":[{"coordinates":[coordinate.clone()]}]});
        for policy in [
            FactorBasePolicy::Legacy {},
            FactorBasePolicy::SampledXProjectedControl {
                candidate_limit: 128,
            },
            FactorBasePolicy::BinarySubspaceIntersection {
                basis: vec![1, 2, 4],
                candidate_limit: 128,
            },
            FactorBasePolicy::BinaryFrobeniusUnionIntersection {
                basis: vec![1, 2, 4],
                candidate_limit: 128,
            },
        ] {
            cfg.factor_base_policy = Some(policy);
            bounded_candidates(&cfg, &public, &base).unwrap();
            for mutation in 0..4 {
                let mut bad = base.clone();
                match mutation {
                    0 => bad["construction_candidates"][0]["coordinates"] = Value::Null,
                    1 => {
                        bad["construction_candidates"][0]["coordinates"][0]["lifts"] =
                            json!([{}, {}, {}])
                    }
                    2 => {
                        bad["construction_candidates"] = json!(vec![json!({"coordinates":[]}); 256])
                    }
                    _ => {
                        bad["construction_candidates"] = json!([
                        {"coordinates":vec![coordinate.clone();128]},
                        {"coordinates":vec![coordinate.clone();128]}])
                    }
                }
                assert!(bounded_candidates(&cfg, &public, &bad).is_err());
            }
        }
        // A small saved prefix must not conceal unbounded recipe replay work.
        for policy in [
            FactorBasePolicy::SampledXProjectedControl {
                candidate_limit: 256,
            },
            FactorBasePolicy::BinarySubspaceIntersection {
                basis: vec![1, 2, 4, 8, 16, 32, 64, 128],
                candidate_limit: 1,
            },
            FactorBasePolicy::BinaryFrobeniusUnionIntersection {
                basis: vec![1, 2, 4, 8],
                candidate_limit: 1,
            },
        ] {
            cfg.factor_base_policy = Some(policy);
            assert!(bounded_candidates(&cfg, &public, &base).is_err());
        }
    }
}
