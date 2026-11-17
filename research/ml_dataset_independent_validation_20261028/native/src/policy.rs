//! Independent structural bindings, without construction or cost certification.
use anyhow::{ensure, Context, Result};
use serde_json::{json, Map, Value};

use crate::common::digest;

pub(crate) fn recipe(value: &Value, n: u64) -> Result<Value> {
    ensure!((3..=61).contains(&n), "invalid policy field size");
    let object = value.as_object().context("policy must be an object")?;
    let kind = value["kind"].as_str().context("missing policy kind")?;
    let fields: &[&str] = match kind {
        "legacy" => &["kind"],
        "sampled_x_projected_control" => &["kind", "candidate_limit"],
        "binary_subspace_intersection" | "binary_frobenius_union_intersection" => {
            &["kind", "basis", "candidate_limit"]
        }
        _ => anyhow::bail!("unsupported policy kind"),
    };
    ensure!(
        object.len() == fields.len() && fields.iter().all(|key| object.contains_key(*key)),
        "unknown/missing policy fields"
    );
    if kind != "legacy" {
        ensure!(
            (1..=65_536).contains(&integer(&value["candidate_limit"], "candidate limit")?),
            "candidate limit outside bounds"
        );
    }
    if let Some(basis) = value.get("basis") {
        let basis = basis.as_array().context("policy basis must be an array")?;
        ensure!(
            !basis.is_empty() && basis.len() <= 12 && basis.len() <= n as usize,
            "invalid policy basis dimension"
        );
        let mut pivots = [0u64; 61];
        for entry in basis {
            let mut row = integer(entry, "basis coordinate")?;
            ensure!(
                row > 0 && row < (1u64 << n),
                "basis coordinate outside field"
            );
            while row != 0 {
                let pivot = (63 - row.leading_zeros()) as usize;
                if pivots[pivot] == 0 {
                    pivots[pivot] = row;
                    break;
                }
                row ^= pivots[pivot];
            }
            ensure!(row != 0, "dependent policy basis");
        }
    }
    // Producer enum serialization orders kind, basis (if present), then limit.
    // Incoming JSON key order must not change the canonical plan digest.
    let mut canonical = Map::new();
    for key in fields {
        canonical.insert((*key).into(), value[*key].clone());
    }
    Ok(Value::Object(canonical))
}

fn integer(value: &Value, label: &str) -> Result<u64> {
    value.as_u64().with_context(|| format!("invalid {label}"))
}

fn decimal(value: &Value, label: &str) -> Result<u64> {
    let text = value.as_str().with_context(|| format!("invalid {label}"))?;
    ensure!(
        !text.is_empty()
            && text.len() <= 20
            && text.bytes().all(|b| b.is_ascii_digit())
            && (text == "0" || !text.starts_with('0')),
        "noncanonical {label}"
    );
    text.parse().with_context(|| format!("oversized {label}"))
}

pub(crate) fn sampling_seed(config: &Value) -> Result<u64> {
    for key in ["seed", "curve_seed", "n"] {
        integer(&config[key], key)?;
    }
    ensure!(
        config["family"].is_string() && config["twist"].is_boolean(),
        "invalid seed context"
    );
    let bytes = serde_json::to_vec(&json!([
        "ml-dataset-factor-base-control-v1",
        config["seed"],
        config["curve_seed"],
        config["family"],
        config["n"],
        config["twist"]
    ]))?;
    Ok(u64::from_str_radix(&digest(&bytes)[..16], 16)?)
}

/// Applies to source-cell audit and standalone positive-certificate replay.
/// No saved membership bits, point construction or counters become proofs.
pub(crate) fn bind_base(public: &Value, base: &Value, config: &Value) -> Result<()> {
    let Some(selected) = config.get("factor_base_policy") else {
        ensure!(
            base["schema"] == "ml-ecdlp-factor-base/v1",
            "factor-base schema differs from absent policy"
        );
        return Ok(());
    };
    ensure!(
        base["schema"] == "ml-ecdlp-factor-base/v2",
        "factor-base schema differs from explicit policy"
    );
    let n = integer(&config["n"], "field size")?;
    let selected = recipe(selected, n)?;
    ensure!(
        selected == recipe(&base["policy"], n)?,
        "factor-base recipe differs from frozen config"
    );
    let kind = selected["kind"].as_str().expect("validated kind");
    ensure!(base["rule"] == kind, "factor-base rule differs from recipe");
    for key in ["family", "n", "m", "twist"] {
        ensure!(
            public[key] == config[key],
            "policy public/config {key} mismatch"
        );
    }
    ensure!(
        base["requested_size"] == config["factor_base_size"],
        "factor-base size provenance"
    );
    let requested = integer(&base["requested_size"], "requested base size")?;
    ensure!(
        (1..=4096).contains(&requested),
        "requested base size outside bounds"
    );
    let points = base["points"]
        .as_array()
        .context("missing factor-base points")?;
    let count = points.len() as u64;
    ensure!(
        count <= 4096 && base["point_cap"] == 4096 && base["actual_size"] == count,
        "factor-base point count/cap mismatch"
    );
    let context = base["construction_context"]
        .as_object()
        .context("missing construction context")?;
    let context_fields = ["sampling_seed", "sampling_algorithm", "seed_access"];
    ensure!(
        context.len() == context_fields.len()
            && context_fields.iter().all(|k| context.contains_key(*k)),
        "unknown/missing construction context fields"
    );
    ensure!(
        decimal(&context["sampling_seed"], "sampling seed")? == sampling_seed(config)?,
        "factor-base control seed differs from frozen config"
    );
    ensure!(
        context["sampling_algorithm"]
            == if kind == "sampled_x_projected_control" {
                "rand-StdRng-gen_range-with-replacement-v1"
            } else {
                "not-used-by-this-policy"
            },
        "unexpected policy sampling algorithm"
    );
    ensure!(
        context["seed_access"].is_string(),
        "missing seed access label"
    );
    let finite = selected.get("basis").is_some();
    ensure!(
        base["subgroup_treatment"]
            == if finite {
                "literal-coordinate-domain-intersection"
            } else {
                "prime-primary-projected-image"
            },
        "policy subgroup treatment mismatch"
    );
    let complete = base["coordinate_domain_complete"]
        .as_bool()
        .context("missing coordinate domain status")?;
    let counters = base["construction_counters"]
        .as_object()
        .context("missing construction counters")?;
    let candidates = integer(
        &base["construction_counters"]["candidate_coordinates"],
        "candidate coordinates",
    )?;
    for (key, value) in counters {
        if key == "scope" {
            ensure!(value.is_string(), "invalid counter scope");
        } else {
            integer(value, "construction counter")?;
        }
    }
    if kind != "legacy" {
        ensure!(
            candidates <= integer(&selected["candidate_limit"], "candidate limit")?,
            "candidate count exceeds frozen limit"
        );
    }
    if finite && config["family"] == "prime" {
        ensure!(
            base["construction_status"] == "unsupported",
            "binary policy on prime field must be unsupported"
        );
    }
    match base["construction_status"].as_str() {
        Some("unsupported") => {
            ensure!(
                finite
                    && (config["family"] == "prime"
                        || kind == "binary_frobenius_union_intersection"),
                "unsupported status outside policy domain"
            );
            ensure!(
                count == 0
                    && candidates == 0
                    && !complete
                    && base["declared_coordinate_count"].is_null(),
                "unsupported policy has constructed domain"
            );
            ensure!(
                base["unsupported_reason"]
                    .as_str()
                    .is_some_and(|s| !s.is_empty()),
                "unsupported policy lacks reason"
            );
        }
        Some("complete" | "capped") => {
            ensure!(
                base["unsupported_reason"].is_null(),
                "supported policy has unsupported reason"
            );
            if finite {
                let declared = integer(
                    &base["declared_coordinate_count"],
                    "declared coordinate count",
                )?;
                let span = 1u64 << selected["basis"].as_array().expect("validated basis").len();
                ensure!(
                    (span..=n * span).contains(&declared) && candidates <= declared,
                    "invalid declared coordinate count"
                );
                if kind == "binary_subspace_intersection" {
                    ensure!(
                        declared == span,
                        "subspace coordinate count differs from dimension"
                    );
                }
                ensure!(
                    complete == (base["construction_status"] == "complete"),
                    "literal policy completion status mismatch"
                );
                if complete {
                    ensure!(
                        candidates == declared,
                        "complete domain lacks candidate coverage"
                    );
                }
            } else {
                ensure!(
                    !complete && base["declared_coordinate_count"].is_null(),
                    "projected policy claims finite coordinate exhaustion"
                );
                let r = decimal(
                    &public["curve"]["subgroup_order"],
                    "declared subgroup order",
                )?;
                let fulfilled = count >= requested || r.checked_sub(1) == Some(count);
                ensure!(
                    fulfilled == (base["construction_status"] == "complete"),
                    "projected policy completion count mismatch"
                );
            }
        }
        _ => anyhow::bail!("unknown construction status"),
    }
    let mut frobenius_closed = true;
    for key in ["negation_action", "frobenius_action"] {
        let action = base[key].as_array().context("missing factor-base action")?;
        ensure!(
            action.len() == points.len(),
            "factor-base action length mismatch"
        );
        for index in action {
            if index.is_null() {
                ensure!(key == "frobenius_action", "negation action must be closed");
                frobenius_closed = false;
            } else {
                ensure!(
                    integer(index, "factor-base action")? < count,
                    "factor-base action outside domain"
                );
            }
        }
    }
    ensure!(
        base["negation_closed"] == true && base["frobenius_closed"] == frobenius_closed,
        "action closure metadata mismatch"
    );
    ensure!(
        kind != "binary_frobenius_union_intersection" || frobenius_closed,
        "Frobenius union action must be closed"
    );
    ensure!(
        integer(&base["frobenius_power"], "Frobenius power")? > 0,
        "zero Frobenius power"
    );
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn fixture() -> (Value, Value, Value) {
        let config = json!({"family":"prime","n":5,"m":2,"twist":false,
            "seed":1,"curve_seed":0,"generator_index":0,"key_index":0,
            "solver":"enumerate","degree_bound":3,"work_limit":100,
            "factor_base_size":2,"factor_base_policy":{"kind":"legacy"}});
        let public = json!({"schema":"ml-ecdlp-instance/v1","family":"prime","n":5,"m":2,"twist":false,
            "field":{"characteristic":"17","modulus":"17","degree":1,"basis":"canonical-residue"},
            "curve":{"model":"short-weierstrass","a":"2","b":"2","subgroup_order":"19"},
            "generator":{"x":"5","y":"1"},"target":{"x":"6","y":"3"}});
        let base = json!({"schema":"ml-ecdlp-factor-base/v2","rule":"legacy","policy":{"kind":"legacy"},
            "requested_size":2,"actual_size":2,"point_cap":4096,"points":[{"x":"5","y":"1"},{"x":"5","y":"16"}],
            "construction_context":{"sampling_seed":sampling_seed(&config).unwrap().to_string(),
                "sampling_algorithm":"not-used-by-this-policy","seed_access":"restricted"},
            "construction_counters":{"candidate_coordinates":1,"scope":"producer reported"},
            "subgroup_treatment":"prime-primary-projected-image","coordinate_domain_complete":false,
            "declared_coordinate_count":null,"construction_status":"complete","unsupported_reason":null,
            "negation_action":[1,0],"frobenius_action":[0,1],"negation_closed":true,
            "frobenius_closed":true,"frobenius_power":1});
        (public, base, config)
    }

    #[test]
    fn recipes_are_strict_bounded_and_canonical() {
        for good in [
            json!({"kind":"legacy"}),
            json!({"kind":"sampled_x_projected_control","candidate_limit":32}),
            json!({"kind":"binary_subspace_intersection","basis":[1,2],"candidate_limit":4}),
            json!({"kind":"binary_frobenius_union_intersection","basis":[1,2],"candidate_limit":64}),
        ] {
            assert_eq!(recipe(&good, 5).unwrap(), good);
            let mut bad = good.clone();
            bad["secret"] = json!(7);
            assert!(recipe(&bad, 5).is_err());
        }
        for limit in [json!(0), json!(65_537), json!(true), Value::Null] {
            assert!(recipe(
                &json!({"kind":"sampled_x_projected_control","candidate_limit":limit}),
                5
            )
            .is_err());
        }
        for basis in [
            json!([]),
            json!([0]),
            json!([1, 1]),
            json!([1, 2, 3]),
            json!([32]),
            json!([true]),
        ] {
            assert!(recipe(
                &json!({"kind":"binary_subspace_intersection","basis":basis,"candidate_limit":32}),
                5
            )
            .is_err());
        }
    }

    #[test]
    fn policy_binding_rejects_substitutions_and_false_status_metadata() {
        let (public, base, config) = fixture();
        bind_base(&public, &base, &config).unwrap();
        for (pointer, value) in [
            (
                "/policy",
                json!({"kind":"sampled_x_projected_control","candidate_limit":32}),
            ),
            ("/construction_context/sampling_seed", json!("0")),
            ("/construction_context/sampling_algorithm", json!("unknown")),
            ("/rule", json!("sampled_x_projected_control")),
            ("/actual_size", json!(3)),
            ("/point_cap", json!(8192)),
            ("/construction_status", json!("capped")),
            ("/coordinate_domain_complete", json!(true)),
            ("/construction_counters/candidate_coordinates", Value::Null),
            ("/negation_action", json!([1, 2])),
            ("/frobenius_closed", json!(false)),
        ] {
            let mut bad = base.clone();
            *bad.pointer_mut(pointer).unwrap() = value;
            assert!(
                bind_base(&public, &bad, &config).is_err(),
                "accepted {pointer}"
            );
        }
        let mut unversioned = config.clone();
        unversioned
            .as_object_mut()
            .unwrap()
            .remove("factor_base_policy");
        assert!(bind_base(&public, &base, &unversioned).is_err());
        let mut null_policy = config;
        null_policy["factor_base_policy"] = Value::Null;
        assert!(bind_base(&public, &base, &null_policy).is_err());
    }

    #[test]
    fn literal_completion_is_domain_coverage_not_requested_cardinality() {
        let (mut public, mut base, mut config) = fixture();
        public["family"] = json!("binary");
        config["family"] = json!("binary");
        let recipe =
            json!({"kind":"binary_subspace_intersection","basis":[1,2],"candidate_limit":4});
        config["factor_base_policy"] = recipe.clone();
        base["policy"] = recipe;
        base["rule"] = json!("binary_subspace_intersection");
        base["subgroup_treatment"] = json!("literal-coordinate-domain-intersection");
        base["coordinate_domain_complete"] = json!(true);
        base["declared_coordinate_count"] = json!(4);
        base["construction_counters"]["candidate_coordinates"] = json!(4);
        base["construction_context"]["sampling_seed"] =
            json!(sampling_seed(&config).unwrap().to_string());
        base["points"] = json!([]);
        base["actual_size"] = json!(0);
        base["negation_action"] = json!([]);
        base["frobenius_action"] = json!([]);
        bind_base(&public, &base, &config).unwrap();
        let mut incomplete = base.clone();
        incomplete["construction_counters"]["candidate_coordinates"] = json!(3);
        assert!(bind_base(&public, &incomplete, &config).is_err());
        incomplete["construction_status"] = json!("capped");
        incomplete["coordinate_domain_complete"] = json!(false);
        bind_base(&public, &incomplete, &config).unwrap();
        public["family"] = json!("prime");
        config["family"] = json!("prime");
        base["construction_context"]["sampling_seed"] =
            json!(sampling_seed(&config).unwrap().to_string());
        assert!(bind_base(&public, &base, &config).is_err());
        base["construction_status"] = json!("unsupported");
        base["coordinate_domain_complete"] = json!(false);
        base["declared_coordinate_count"] = Value::Null;
        base["construction_counters"]["candidate_coordinates"] = json!(0);
        base["unsupported_reason"] = json!("binary recipe on prime field");
        bind_base(&public, &base, &config).unwrap();
    }
}
