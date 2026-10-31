//! Exact projections from current ecbench records. No fabricated coverage or zero
//! filling. Plan/source/seal authenticity must additionally be independently reviewed.
use crate::{InverseModel, Scalar};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Expected {
    /// Prospective, unexecuted plans deliberately have no session identity.
    /// A record can only be imported after a concrete session is supplied.
    pub session: Option<String>,
    pub binary_sha256: String,
    pub seq: u64,
    pub round: u64,
    pub warmup: bool,
    pub arm: String,
    pub role: String,
    pub seed: String,
    pub workload: Value,
    pub method: Value,
}
#[derive(Debug)]
pub struct Imported {
    pub record_id: String,
    pub verified: bool,
    pub raw: Value,
    pub observations: BTreeMap<String, Option<u128>>,
    pub normalized_scalar_events: Option<u128>,
    /// Always absent for the current producer: it declares incomplete cost.
    pub full_price: Option<crate::Fraction>,
    pub coverage_blockers: Vec<String>,
}
fn text<'a>(v: &'a Value, p: &str) -> Result<&'a str, String> {
    v.pointer(p)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {p}"))
}
fn observation(v: &Value, p: &str) -> Result<Option<u128>, String> {
    match v.pointer(p) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(n)) => n
            .to_string()
            .parse::<u128>()
            .map(Some)
            .map_err(|_| format!("nonnegative integer required at {p}")),
        Some(_) => Err(format!("numeric observation required at {p}")),
    }
}
fn sum(v: &Value, pointers: &[&str]) -> Result<Option<u128>, String> {
    let mut total = 0u128;
    for p in pointers {
        let Some(n) = observation(v, p)? else {
            return Ok(None);
        };
        total = total.checked_add(n).ok_or("counter overflow")?;
    }
    Ok(Some(total))
}
pub fn import(record: &Value, expected: &Expected) -> Result<Imported, String> {
    if text(record, "/schema")? != "ecbench.record/v1" {
        return Err("unsupported schema".into());
    }
    if expected.binary_sha256.len() != 64
        || !expected
            .binary_sha256
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    {
        return Err("expected binary SHA256 is not canonical hex".into());
    }
    if expected
        .seed
        .parse::<u64>()
        .map(|x| x.to_string())
        .ok()
        .as_deref()
        != Some(&expected.seed)
    {
        return Err("expected seed not canonical decimal u64".into());
    }
    let session = expected
        .session
        .as_deref()
        .ok_or("record present for a plan without a session identity")?;
    for (p, want) in [
        ("/session_id", session),
        ("/binary_sha256", expected.binary_sha256.as_str()),
        ("/arm", &expected.arm),
        ("/role", &expected.role),
        ("/algorithm_seed", &expected.seed),
    ] {
        if text(record, p)? != want {
            return Err(format!("identity mismatch {p}"));
        }
    }
    for (p, want) in [("/seq", expected.seq), ("/round", expected.round)] {
        if record.pointer(p).and_then(Value::as_u64) != Some(want) {
            return Err(format!("identity mismatch {p}"));
        }
    }
    if record.pointer("/warmup").and_then(Value::as_bool) != Some(expected.warmup)
        || record.get("workload") != Some(&expected.workload)
        || record.get("method") != Some(&expected.method)
    {
        return Err("plan workload/method/warmup mismatch".into());
    }
    let id = text(record, "/record_id")?.to_owned();
    let method = text(record, "/method/id")?;
    let candidate = method == "ic.three_leads_genus2_pencil_symbolic_v2";
    if !candidate && method != "rho.three_leads_negation_v1" {
        return Err("unsupported method".into());
    }
    let verified = text(record, "/outcome/status")? == "verified"
        && record.pointer("/outcome/matches_target") == Some(&Value::Bool(true))
        && record.pointer("/outcome/matches_planted") == Some(&Value::Bool(true))
        && record.pointer("/outcome/error") == Some(&Value::Null);
    let mut observations = BTreeMap::new();
    let mut blockers = vec![
        "source_bound_complete_semantic_coverage_not_exported".into(),
        "raw_record_seal_plan_and_source_authenticity_require_independent_review".into(),
    ];
    if !verified {
        blockers.push("not_independently_verified_known_answer".into())
    }
    if expected.warmup {
        blockers.push("warmup_excluded_from_measured_prices".into())
    }
    let scalar = if candidate {
        for key in ["m", "s", "i", "field_adds", "bit_ops"] {
            let p = format!("/detail/candidate/counts/{key}");
            observations.insert(p.clone(), observation(record, &p)?);
            let p = format!("/detail/additional_source_verification/{key}");
            observations.insert(p.clone(), observation(record, &p)?);
            let p = format!("/detail/source_addition/{key}");
            observations.insert(p.clone(), observation(record, &p)?);
        }
        for key in [
            "base_adds",
            "base_muls",
            "base_inversions",
            "scalar_adds",
            "scalar_muls",
            "scalar_inversions",
        ] {
            let p = format!("/detail/candidate/arithmetic/{key}");
            observations.insert(p.clone(), observation(record, &p)?);
        }
        for key in [
            "scalar_adds",
            "scalar_muls",
            "scalar_negations",
            "scalar_inversions",
            "modular_reductions",
            "map_calls",
            "map_key_comparisons",
            "map_entry_visits",
            "map_cloned_entries",
            "parent_hops",
            "payload_terms",
            "partition_terms",
            "walk_label_calls",
            "collection_entry_visits",
            "collection_elements_copied",
            "collection_elements_initialized",
            "collection_comparisons",
        ] {
            for prefix in [
                "/detail/candidate/symbolic/semantic_counts",
                "/detail/cold_setup/semantic_counts",
            ] {
                let p = format!("{prefix}/{key}");
                observations.insert(p.clone(), observation(record, &p)?);
            }
        }
        // Exact landed outer-validation ledger, separate from already counted
        // symbolic constructor primality. Do not fill skipped calls with zero.
        if let Some(outer) = record.pointer("/detail/cold_setup/outer_primality") {
            if text(outer, "/schema")? != "pencil-outer-primality/v1" {
                return Err("unknown outer primality schema".into());
            }
            for domain in ["base_modulus", "scalar_modulus"] {
                let prefix = format!("/detail/cold_setup/outer_primality/observations/{domain}");
                match record.pointer(&prefix) {
                    Some(Value::Null) => {
                        blockers.push(format!("outer_primality_{domain}_not_invoked"))
                    }
                    None => blockers.push(format!("outer_primality_{domain}_observation_missing")),
                    Some(o) => {
                        if o.get("prime") != Some(&Value::Bool(true)) {
                            blockers.push(format!("outer_primality_{domain}_not_prime"));
                        }
                        let modulus = observation(o, "/modulus")?;
                        let expected_modulus = observation(
                            record,
                            if domain == "base_modulus" {
                                "/detail/candidate/p"
                            } else {
                                "/detail/candidate/r"
                            },
                        )?;
                        match (modulus, expected_modulus) {
                            (Some(a), Some(b)) if a != b => {
                                return Err("outer primality modulus mismatch".into())
                            }
                            (Some(_), Some(_)) => {}
                            _ => blockers
                                .push(format!("outer_primality_{domain}_modulus_binding_missing")),
                        }
                        for key in [
                            "modular_muls",
                            "modular_squares",
                            "product_reductions",
                            "standalone_modular_reductions",
                            "divisibility_checks",
                            "small_divisor_iterations",
                            "witness_iterations",
                            "exponent_iterations",
                            "witness_square_iterations",
                        ] {
                            let pointer = format!("{prefix}/counts/{key}");
                            observations.insert(pointer.clone(), observation(record, &pointer)?);
                        }
                    }
                }
            }
        } else {
            blockers.push("outer_primality_ledger_missing".into())
        }
        // Missing or future provenance is retained raw, not treated as free work.
        blockers.push("outer_validation_and_between_phase_coverage_not_certified".into());
        blockers.push("sieve_vector_adapter_reference_event_mapping_not_complete".into());
        if record
            .pointer("/detail/cost_status")
            .and_then(Value::as_str)
            != Some("complete")
        {
            blockers.push("producer_declares_incomplete_cost".into())
        }
        let a = sum(
            record,
            &[
                "/detail/candidate/arithmetic/scalar_adds",
                "/detail/candidate/symbolic/semantic_counts/scalar_adds",
                "/detail/cold_setup/semantic_counts/scalar_adds",
            ],
        )?;
        let m = sum(
            record,
            &[
                "/detail/candidate/arithmetic/scalar_muls",
                "/detail/candidate/symbolic/semantic_counts/scalar_muls",
                "/detail/cold_setup/semantic_counts/scalar_muls",
            ],
        )?;
        let i = sum(
            record,
            &[
                "/detail/candidate/arithmetic/scalar_inversions",
                "/detail/candidate/symbolic/semantic_counts/scalar_inversions",
                "/detail/cold_setup/semantic_counts/scalar_inversions",
            ],
        )?;
        let n = sum(
            record,
            &[
                "/detail/candidate/symbolic/semantic_counts/scalar_negations",
                "/detail/cold_setup/semantic_counts/scalar_negations",
            ],
        )?;
        let r = observation(record, "/detail/candidate/r")?;
        match (a, m, i, n, r) {
            (Some(adds), Some(muls), Some(inversions), Some(negations), Some(r)) => Some(
                Scalar {
                    adds,
                    muls,
                    inversions,
                    negations,
                    model: InverseModel::ExpandedPencil,
                }
                .atomic_events(u64::try_from(r).map_err(|_| "modulus exceeds u64")?)
                .map_err(|e| format!("scalar ledger {e:?}"))?,
            ),
            _ => None,
        }
    } else {
        for key in [
            "field_multiplications",
            "field_squarings",
            "field_inversions",
            "field_additions_uncharged",
            "linear_bit_operations_uncharged",
            "source_add_multiplications",
            "source_add_squarings",
            "source_add_inversions",
            "scalar_adds_uncharged",
            "scalar_muls_uncharged",
            "scalar_inversions_uncharged",
            "rho_semantic_key_comparisons_v1",
            "rho_semantic_entry_visits_v1",
            "rho_semantic_elements_copied_v1",
            "rho_semantic_elements_initialized_v1",
            "rho_observed_hash_calls_v1",
            "rho_observed_hash_bytes_v1",
            "partition_hashes_uncharged",
            "rng_draws_uncharged",
            "hash_probes_uncharged",
            "canonicalisations_uncharged",
        ] {
            let p = format!("/counters/{key}");
            observations.insert(p.clone(), observation(record, &p)?);
        }
        let a = observation(record, "/counters/scalar_adds_uncharged")?;
        let m = observation(record, "/counters/scalar_muls_uncharged")?;
        let i = observation(record, "/counters/scalar_inversions_uncharged")?;
        blockers.push("reference_disjoint_logical_coverage_not_certified".into());
        match (a, m, i) {
            (Some(a), Some(m), Some(i)) => Some(
                a.checked_add(m)
                    .and_then(|x| x.checked_add(i))
                    .ok_or("scalar counter overflow")?,
            ),
            _ => None,
        }
    };
    if let Some(accounting) = record.pointer("/detail/adapter_accounting") {
        if text(accounting, "/schema")? != "pencil-adapter-accounting/v1-draft" {
            return Err("unknown adapter accounting schema".into());
        }
        if text(accounting, "/geometry/schema")? != "engine-geometry-partition/v1-draft" {
            return Err("unknown engine geometry schema".into());
        }
    }
    for segment in [
        "total",
        "segments/solver_including_cold_setup",
        "segments/additional_source_verification",
        "segments/source_unit_calibration",
        "segments/report_assembly",
    ] {
        for key in [
            "entry_visits",
            "elements_copied",
            "elements_initialized",
            "key_comparisons",
            "constructor_prime_calls",
            "constructor_modular_muls",
            "constructor_modular_squares",
            "constructor_standalone_reductions",
            "scalar_constructor_prime_calls",
            "scalar_constructor_modular_muls",
            "scalar_constructor_modular_squares",
            "scalar_constructor_standalone_reductions",
            "metadata_value_nodes_copied",
            "metadata_string_bytes_copied",
            "metadata_key_bytes_copied",
            "metadata_family_bytes_copied",
        ] {
            let p = format!("/detail/adapter_accounting/geometry/{segment}/{key}");
            observations.insert(p.clone(), observation(record, &p)?);
        }
    }
    for segment in ["preparation", "addition", "total"] {
        for key in ["m", "s", "i", "field_adds", "bit_ops"] {
            let p = format!(
                "/detail/adapter_accounting/source_unit_calibration_arithmetic/{segment}/{key}"
            );
            observations.insert(p.clone(), observation(record, &p)?);
        }
    }
    if record.pointer("/detail/adapter_accounting").is_some() {
        if record
            .pointer("/detail/adapter_accounting/geometry/partition_sum_verified")
            .and_then(Value::as_bool)
            != Some(true)
        {
            return Err("adapter geometry partition not asserted".into());
        }
        for key in [
            "entry_visits",
            "elements_copied",
            "elements_initialized",
            "key_comparisons",
            "constructor_prime_calls",
            "constructor_modular_muls",
            "constructor_modular_squares",
            "constructor_standalone_reductions",
            "scalar_constructor_prime_calls",
            "scalar_constructor_modular_muls",
            "scalar_constructor_modular_squares",
            "scalar_constructor_standalone_reductions",
            "metadata_value_nodes_copied",
            "metadata_string_bytes_copied",
            "metadata_key_bytes_copied",
            "metadata_family_bytes_copied",
        ] {
            let prefix = "/detail/adapter_accounting/geometry";
            let total = observation(record, &format!("{prefix}/total/{key}"))?
                .ok_or("missing geometry total")?;
            let mut sum = 0u128;
            for segment in [
                "solver_including_cold_setup",
                "additional_source_verification",
                "source_unit_calibration",
                "report_assembly",
            ] {
                sum = sum
                    .checked_add(
                        observation(record, &format!("{prefix}/segments/{segment}/{key}"))?
                            .ok_or("missing geometry segment")?,
                    )
                    .ok_or("geometry partition overflow")?;
            }
            if sum != total {
                return Err("geometry partition mismatch".into());
            }
        }
    }
    // Check any complete calibration evidence; absence remains absent and the
    // executable mapper requires all components before admitting a price.
    let unit_prefix = if candidate {
        "/detail/source_addition"
    } else {
        "/detail/arithmetic_price/source_addition"
    };
    for key in ["m", "s", "i", "field_adds", "bit_ops"] {
        let prefix = "/detail/adapter_accounting/source_unit_calibration_arithmetic";
        let prep = observation(record, &format!("{prefix}/preparation/{key}"))?;
        let addition = observation(record, &format!("{prefix}/addition/{key}"))?;
        let total = observation(record, &format!("{prefix}/total/{key}"))?;
        if let (Some(a), Some(b), Some(c)) = (prep, addition, total) {
            if a.checked_add(b).ok_or("calibration counter overflow")? != c {
                return Err("calibration partition mismatch".into());
            }
        }
        if let (Some(a), Some(b)) = (
            addition,
            observation(record, &format!("{unit_prefix}/{key}"))?,
        ) {
            if a != b {
                return Err("calibration source unit mismatch".into());
            }
        }
    }
    if scalar.is_none() {
        blockers.push("scalar_observations_missing".into())
    }
    Ok(Imported {
        record_id: id,
        verified,
        raw: record.clone(),
        observations,
        normalized_scalar_events: scalar,
        full_price: None,
        coverage_blockers: blockers,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn fixture() -> (Value, Expected) {
        let e = Expected {
            session: Some("s".into()),
            binary_sha256: "a".repeat(64),
            seq: 1,
            round: 1,
            warmup: false,
            arm: "rho".into(),
            role: "reference".into(),
            seed: "18446744073709551615".into(),
            workload: json!({"workload_id":"W","source":"fixture"}),
            method: json!({"id":"rho.three_leads_negation_v1","params":{"trial_cap":"3"}}),
        };
        let v = json!({"schema":"ecbench.record/v1","record_id":"projection-not-sealed","session_id":e.session,"binary_sha256":e.binary_sha256,"seq":e.seq,"round":e.round,"warmup":e.warmup,"arm":e.arm,"role":e.role,"algorithm_seed":e.seed,"workload":e.workload,"method":e.method,"outcome":{"status":"verified","matches_target":true,"matches_planted":true,"error":null},"counters":{"scalar_adds_uncharged":2,"scalar_muls_uncharged":3,"scalar_inversions_uncharged":1}});
        (v, e)
    }
    #[test]
    fn exact_identity_and_unknown_coverage() {
        let (v, e) = fixture();
        let i = import(&v, &e).unwrap();
        assert_eq!(i.normalized_scalar_events, Some(6));
        assert_eq!(i.observations["/counters/field_multiplications"], None);
        assert_eq!(i.full_price, None);
        let mut bad = v.clone();
        bad["workload"]["source"] = json!("changed");
        assert!(import(&bad, &e).is_err());
        let mut bad = v;
        bad["algorithm_seed"] = json!("18446744073709551614");
        assert!(import(&bad, &e).is_err());
    }
    #[test]
    fn missing_scalar_is_not_zero_and_negative_rejected() {
        let (mut v, e) = fixture();
        v["counters"]
            .as_object_mut()
            .unwrap()
            .remove("scalar_muls_uncharged");
        assert_eq!(import(&v, &e).unwrap().normalized_scalar_events, None);
        v["counters"]["scalar_muls_uncharged"] = json!(-1);
        assert!(import(&v, &e).is_err());
    }
    #[test]
    fn failure_and_warmup_retained() {
        let (mut v, mut e) = fixture();
        v["outcome"]["status"] = json!("exhausted");
        v["warmup"] = json!(true);
        e.warmup = true;
        let i = import(&v, &e).unwrap();
        assert!(!i.verified);
        assert!(i
            .coverage_blockers
            .contains(&"warmup_excluded_from_measured_prices".into()));
        assert_eq!(i.raw, v);
        assert_eq!(i.full_price, None);
    }
    #[test]
    fn candidate_expansion_normalizes_actual_paths_and_refuses_underflow() {
        let (mut v, mut e) = fixture();
        e.arm = "pencil".into();
        e.role = "candidate".into();
        e.method = json!({"id":"ic.three_leads_genus2_pencil_symbolic_v2","params":{"small_denominator":"8","matrix_coefficient_cap":"25000000"}});
        v["arm"] = json!(e.arm);
        v["role"] = json!(e.role);
        v["method"] = e.method.clone();
        // r=101: inverse exponent99 has7bits+4ones=11 expanded multiplies.
        v["detail"] = json!({"cost_status":"incomplete","candidate":{"r":101,"arithmetic":{"scalar_adds":2,"scalar_muls":14,"scalar_inversions":1},"symbolic":{"semantic_counts":{"scalar_adds":1,"scalar_muls":0,"scalar_inversions":0,"scalar_negations":1}}}});
        assert_eq!(import(&v, &e).unwrap().normalized_scalar_events, None);
        v["detail"]["cold_setup"] = json!({"semantic_counts":{"scalar_adds":0,"scalar_muls":0,"scalar_negations":0,"scalar_inversions":0}});
        let i = import(&v, &e).unwrap();
        assert_eq!(i.normalized_scalar_events, Some(8));
        assert_eq!(i.full_price, None);
        assert_eq!(i.observations["/detail/candidate/counts/m"], None);
        assert_eq!(
            i.observations["/detail/cold_setup/semantic_counts/collection_entry_visits"],
            None
        );
        v["detail"]["cold_setup"]["semantic_counts"]["collection_entry_visits"] = json!(17);
        v["detail"]["candidate"]["symbolic"]["semantic_counts"]["collection_elements_copied"] =
            json!(23);
        let observed = import(&v, &e).unwrap();
        assert_eq!(
            observed.observations["/detail/cold_setup/semantic_counts/collection_entry_visits"],
            Some(17)
        );
        assert_eq!(
            observed.observations
                ["/detail/candidate/symbolic/semantic_counts/collection_elements_copied"],
            Some(23)
        );
        assert_eq!(observed.full_price, None);
        v["detail"]["candidate"]["arithmetic"]["scalar_muls"] = json!(10);
        assert!(import(&v, &e).is_err());
    }
    #[test]
    fn outer_primality_binds_domain_and_preserves_missing_events() {
        let (mut v, mut e) = fixture();
        e.arm = "pencil".into();
        e.role = "candidate".into();
        e.method = json!({"id":"ic.three_leads_genus2_pencil_symbolic_v2"});
        v["arm"] = json!(e.arm);
        v["role"] = json!(e.role);
        v["method"] = e.method.clone();
        v["detail"] = json!({"candidate":{"p":107,"r":101},"cold_setup":{"outer_primality":{"schema":"pencil-outer-primality/v1","observations":{"base_modulus":{"modulus":107,"prime":true,"counts":{"modular_muls":7}},"scalar_modulus":null}}}});
        let i = import(&v, &e).unwrap();
        assert_eq!(i.observations["/detail/cold_setup/outer_primality/observations/base_modulus/counts/modular_muls"],Some(7));
        assert_eq!(i.observations["/detail/cold_setup/outer_primality/observations/base_modulus/counts/modular_squares"],None);
        assert!(i
            .coverage_blockers
            .contains(&"outer_primality_scalar_modulus_not_invoked".into()));
        assert_eq!(i.full_price, None);
        v["detail"]["cold_setup"]["outer_primality"]["observations"]["base_modulus"]["modulus"] =
            json!(101);
        assert!(import(&v, &e).is_err());
    }
    #[test]
    fn adapter_partition_is_recomputed_and_missing_is_not_zero() {
        let (mut v, e) = fixture();
        let mut zero = serde_json::Map::new();
        for key in [
            "entry_visits",
            "elements_copied",
            "elements_initialized",
            "key_comparisons",
            "constructor_prime_calls",
            "constructor_modular_muls",
            "constructor_modular_squares",
            "constructor_standalone_reductions",
            "scalar_constructor_prime_calls",
            "scalar_constructor_modular_muls",
            "scalar_constructor_modular_squares",
            "scalar_constructor_standalone_reductions",
            "metadata_value_nodes_copied",
            "metadata_string_bytes_copied",
            "metadata_key_bytes_copied",
            "metadata_family_bytes_copied",
        ] {
            zero.insert(key.into(), json!(0));
        }
        let mut segments = serde_json::Map::new();
        for name in [
            "solver_including_cold_setup",
            "additional_source_verification",
            "source_unit_calibration",
            "report_assembly",
        ] {
            segments.insert(name.into(), Value::Object(zero.clone()));
        }
        v["detail"] = json!({"adapter_accounting":{"schema":"pencil-adapter-accounting/v1-draft","geometry":{"schema":"engine-geometry-partition/v1-draft","total":zero,"segments":segments,"partition_sum_verified":true}}});
        assert_eq!(import(&v, &e).unwrap().full_price, None);
        v["detail"]["adapter_accounting"]["geometry"]["total"]["entry_visits"] = json!(1);
        assert!(import(&v, &e).unwrap_err().contains("partition mismatch"));
        v["detail"]["adapter_accounting"]["geometry"]["total"]["entry_visits"] = Value::Null;
        assert!(import(&v, &e)
            .unwrap_err()
            .contains("missing geometry total"));
    }
}
