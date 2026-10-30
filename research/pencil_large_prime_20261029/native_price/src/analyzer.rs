//! Source-bound finite algorithm-cost mapper. Coverage is an independently
//! authored evidence record, never inferred from a successful solve or flag.
use crate::{
    grid,
    importer::{self, Expected},
    Fraction,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs};
pub const POLICY: &str = "pencil-algorithm-cost/atomic-scalar-disjoint-v1";
pub const CATEGORIES: [&str; 9] = [
    "field_arithmetic",
    "scalar_arithmetic",
    "outer_and_constructor_validation",
    "setup_transfer",
    "collection_graph_elimination",
    "hash_rng",
    "source_verification",
    "metadata_clone",
    "calibration",
];
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
pub fn mapping_sha256() -> String {
    digest(include_bytes!("analyzer.rs"))
}
pub fn mapping_contract() -> Value {
    json!({
     "schema":"pencil-algorithm-mapping/v1","policy":POLICY,"mapping_sha256":mapping_sha256(),
     "coverage_categories":CATEGORIES,"grid":{"square_twice":[0,1,2],"inversion":[1,10,100,1000],"native_tenths":[0,1,10]},
     "normalization":"atomic scalar inversions; base inverse-expanded multiplies subtracted using bitlength(p-2)+popcount(p-2)",
     "geometry":"use solver_including_cold_setup + additional_source_verification segments; never add total again; calibration separate and included once in combined ratio",
     "metadata":"one native unit per copied Value node; key/string/family bytes supplemental",
     "rng":"candidate symbolic.semantic_counts.walk_label_calls plus cold_setup; reference rng_draws_uncharged, one atomic generation each, internal mix and reduction included",
     "hash":"reference rho_observed_hash_calls_v1 plus partition_hashes_uncharged; hash_probes_uncharged excluded as overlapping summary",
     "diagnostic_exclusions":["map_calls","sparse_accumulations","partition_terms","payload_terms","parent_hops","primality_checks","constructor_prime_calls","scalar_constructor_prime_calls","product_reductions","rho_observed_hash_bytes_v1"],
     "separate_units":["report formatting and serialization","allocation bytes and calls","RSS","wall and CPU time","parent verification and harness","infrastructure and retries"],
     "admission":"exact record expectations plus independent source-bound nine-category review; matching protocol/source/report digests, binary IDs and Rust1.99.1; missing or failed pair keeps all36 prices null",
     "aggregation":"pair rows only; cohort aggregation must sum exact prices across the entire prescribed matched cohort before dividing, never average ratios or use survivors"
    })
}
fn text<'a>(v: &'a Value, p: &str) -> Result<&'a str, String> {
    v.pointer(p)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {p}"))
}
fn n(v: &Value, p: &str) -> Result<u128, String> {
    v.pointer(p)
        .and_then(Value::as_number)
        .ok_or_else(|| format!("missing numeric {p}"))?
        .to_string()
        .parse()
        .map_err(|_| format!("invalid unsigned integer {p}"))
}
fn plus(a: u128, b: u128) -> Result<u128, String> {
    a.checked_add(b).ok_or("counter overflow".into())
}
fn minus(a: u128, b: u128) -> Result<u128, String> {
    a.checked_sub(b).ok_or("inverse expansion underflow".into())
}
fn times(a: u128, b: u128) -> Result<u128, String> {
    a.checked_mul(b).ok_or("counter overflow".into())
}
fn sum(v: &Value, prefix: &str, keys: &[&str]) -> Result<u128, String> {
    keys.iter()
        .try_fold(0, |a, k| plus(a, n(v, &format!("{prefix}/{k}"))?))
}
fn expected(v: &Value) -> Result<Expected, String> {
    let session = match v.get("session") {
        Some(Value::String(value)) => Some(value.clone()),
        Some(Value::Null) => None,
        _ => return Err("missing string or null /session".into()),
    };
    Ok(Expected {
        session,
        binary_sha256: text(v, "/binary_sha256")?.into(),
        seq: n(v, "/seq")?.try_into().map_err(|_| "seq overflow")?,
        round: n(v, "/round")?.try_into().map_err(|_| "round overflow")?,
        warmup: v["warmup"].as_bool().ok_or("warmup missing")?,
        arm: text(v, "/arm")?.into(),
        role: text(v, "/role")?.into(),
        seed: text(v, "/seed")?.into(),
        workload: v.get("workload").ok_or("workload missing")?.clone(),
        method: v.get("method").ok_or("method missing")?.clone(),
    })
}
fn review(v: &Value, binaries: &[&str], protocol: &str) -> Result<(), String> {
    if text(v, "/schema")? != "pencil-algorithm-coverage-review/v1"
        || text(v, "/status")? != "reviewed_complete"
        || text(v, "/policy")? != POLICY
        || text(v, "/mapping_sha256")? != mapping_sha256()
        || text(v, "/protocol_sha256")? != protocol
    {
        return Err("coverage review identity/status mismatch".into());
    }
    if text(v, "/reviewer_identity")?.is_empty()
        || text(v, "/reviewer_identity")? == text(v, "/producer_identity")?
    {
        return Err("independent reviewer required".into());
    }
    if text(v, "/rust_toolchain")? != "1.99.1" {
        return Err("unregistered Rust toolchain".into());
    }
    let protocol_bytes = fs::read(text(v, "/protocol_path")?).map_err(|e| e.to_string())?;
    if digest(&protocol_bytes) != protocol {
        return Err("protocol digest mismatch".into());
    }
    let approved = v["binary_sha256"]
        .as_array()
        .ok_or("review binary identities missing")?;
    for b in binaries {
        if !approved.iter().any(|x| x.as_str() == Some(b)) {
            return Err("binary not covered by review".into());
        }
    }
    let mut hashes = BTreeSet::new();
    for f in v["source_files"]
        .as_array()
        .ok_or("review source files missing")?
    {
        let bytes = fs::read(text(f, "/path")?).map_err(|e| e.to_string())?;
        let h = digest(&bytes);
        if h != text(f, "/sha256")? {
            return Err("review source digest mismatch".into());
        }
        hashes.insert(h);
    }
    if hashes.is_empty() {
        return Err("review sources empty".into());
    }
    let report = fs::read(text(v, "/independent_report/path")?).map_err(|e| e.to_string())?;
    if digest(&report) != text(v, "/independent_report/sha256")? {
        return Err("independent report digest mismatch".into());
    }
    let rows = v["categories"]
        .as_array()
        .ok_or("review categories missing")?;
    if rows.len() != CATEGORIES.len() {
        return Err("coverage category count mismatch".into());
    }
    let mut seen = BTreeSet::new();
    for row in rows {
        let category = text(row, "/category")?;
        if !CATEGORIES.contains(&category) || !seen.insert(category) {
            return Err("unknown/duplicate coverage category".into());
        }
        if text(row, "/disposition")? != "covered"
            || !row["uncovered_sites"]
                .as_array()
                .ok_or("uncovered sites missing")?
                .is_empty()
        {
            return Err("uncovered algorithm sites".into());
        }
        let sites = row["source_sites"]
            .as_array()
            .ok_or("source sites missing")?;
        if sites.is_empty() {
            return Err("empty category source evidence".into());
        }
        for site in sites {
            if !hashes.contains(text(site, "/sha256")?)
                || n(site, "/line_start")? == 0
                || n(site, "/line_end")? < n(site, "/line_start")?
                || text(site, "/primitive")?.is_empty()
            {
                return Err("invalid source evidence".into());
            }
        }
    }
    Ok(())
}
#[derive(Clone, Copy, Debug)]
struct Priced {
    m: u128,
    s: u128,
    i: u128,
    logical: u128,
}
fn arithmetic(v: &Value, prefix: &str) -> Result<Priced, String> {
    Ok(Priced {
        m: n(v, &format!("{prefix}/m"))?,
        s: n(v, &format!("{prefix}/s"))?,
        i: n(v, &format!("{prefix}/i"))?,
        logical: sum(v, prefix, &["field_adds", "bit_ops"])?,
    })
}
impl Priced {
    fn add(self, b: Self) -> Result<Self, String> {
        Ok(Self {
            m: plus(self.m, b.m)?,
            s: plus(self.s, b.s)?,
            i: plus(self.i, b.i)?,
            logical: plus(self.logical, b.logical)?,
        })
    }
}
fn map(v: &Value, candidate: bool, scalar: u128, p: u64) -> Result<(Priced, Priced), String> {
    let mut out = if candidate {
        arithmetic(v, "/detail/candidate/counts")?
            .add(arithmetic(v, "/detail/additional_source_verification")?)?
    } else {
        Priced {
            m: n(v, "/counters/field_multiplications")?,
            s: n(v, "/counters/field_squarings")?,
            i: n(v, "/counters/field_inversions")?,
            logical: sum(
                v,
                "/counters",
                &[
                    "field_additions_uncharged",
                    "linear_bit_operations_uncharged",
                ],
            )?,
        }
    };
    out.logical = plus(out.logical, scalar)?;
    if candidate {
        let a = "/detail/candidate/arithmetic";
        let inv = n(v, &format!("{a}/base_inversions"))?;
        let e = p.checked_sub(2).ok_or("invalid modulus")?;
        let expanded = times(u128::from(64 - e.leading_zeros() + e.count_ones()), inv)?;
        out.m = plus(out.m, minus(n(v, &format!("{a}/base_muls"))?, expanded)?)?;
        out.i = plus(out.i, inv)?;
        out.logical = plus(out.logical, n(v, &format!("{a}/base_adds"))?)?;
        for prefix in [
            "/detail/candidate/symbolic/semantic_counts",
            "/detail/cold_setup/semantic_counts",
        ] {
            out.logical = plus(
                out.logical,
                sum(
                    v,
                    prefix,
                    &[
                        "modular_reductions",
                        "map_key_comparisons",
                        "map_entry_visits",
                        "map_cloned_entries",
                        "walk_label_calls",
                        "collection_entry_visits",
                        "collection_elements_copied",
                        "collection_elements_initialized",
                        "collection_comparisons",
                    ],
                )?,
            )?;
        }
        // Atomic label generation includes its internal mixing and final reduction,
        // as inherited RNG draws do. Never infer this field from walk totals.

        for domain in ["base_modulus", "scalar_modulus"] {
            if v.pointer(&format!(
                "/detail/cold_setup/outer_primality/observations/{domain}/prime"
            ))
            .and_then(Value::as_bool)
                != Some(true)
            {
                return Err("outer primality not verified".into());
            }
            let prefix = format!("/detail/cold_setup/outer_primality/observations/{domain}/counts");
            let m = n(v, &format!("{prefix}/modular_muls"))?;
            let s = n(v, &format!("{prefix}/modular_squares"))?;
            if domain == "base_modulus" {
                out.m = plus(out.m, m)?;
                out.s = plus(out.s, s)?;
            } else {
                out.logical = plus(out.logical, plus(m, s)?)?;
            }
            out.logical = plus(
                out.logical,
                n(v, &format!("{prefix}/standalone_modular_reductions"))?,
            )?;
        }
    } else {
        if n(v, "/counters/rho_hash_policy_rust_default_v1")? != 1 {
            return Err("unregistered rho hash policy".into());
        }
        out.logical = plus(
            out.logical,
            sum(
                v,
                "/counters",
                &[
                    "rho_semantic_key_comparisons_v1",
                    "rho_semantic_entry_visits_v1",
                    "rho_semantic_elements_copied_v1",
                    "rho_semantic_elements_initialized_v1",
                    "rho_observed_hash_calls_v1",
                    "partition_hashes_uncharged",
                    "rng_draws_uncharged",
                ],
            )?,
        )?;
    }
    // Geometry calibration is separated below, rather than summing whole+segments.
    let g = "/detail/adapter_accounting/geometry/segments";
    for segment in [
        "solver_including_cold_setup",
        "additional_source_verification",
    ] {
        let prefix = format!("{g}/{segment}");
        out = out.add(geometry(v, &prefix)?)?;
    }
    let calibration_prefix = "/detail/adapter_accounting/source_unit_calibration_arithmetic";
    let unit_prefix = if candidate {
        "/detail/source_addition"
    } else {
        "/detail/arithmetic_price/source_addition"
    };
    for key in ["m", "s", "i", "field_adds", "bit_ops"] {
        let prep = n(v, &format!("{calibration_prefix}/preparation/{key}"))?;
        let addition = n(v, &format!("{calibration_prefix}/addition/{key}"))?;
        let total = n(v, &format!("{calibration_prefix}/total/{key}"))?;
        if plus(prep, addition)? != total || addition != n(v, &format!("{unit_prefix}/{key}"))? {
            return Err("calibration partition or source unit mismatch".into());
        }
    }
    let calibration = arithmetic(
        v,
        "/detail/adapter_accounting/source_unit_calibration_arithmetic/total",
    )?
    .add(geometry(v, &format!("{g}/source_unit_calibration"))?)?;
    Ok((out, calibration))
}
fn geometry(v: &Value, p: &str) -> Result<Priced, String> {
    Ok(Priced {
        m: n(v, &format!("{p}/constructor_modular_muls"))?,
        s: n(v, &format!("{p}/constructor_modular_squares"))?,
        i: 0,
        logical: sum(
            v,
            p,
            &[
                "entry_visits",
                "elements_copied",
                "elements_initialized",
                "key_comparisons",
                "constructor_standalone_reductions",
                "scalar_constructor_modular_muls",
                "scalar_constructor_modular_squares",
                "scalar_constructor_standalone_reductions",
                "metadata_value_nodes_copied",
            ],
        )?,
    })
}
fn price(c: Priced, unit: Priced, cell: crate::Cell) -> Result<Fraction, String> {
    let weighted = |x: Priced| -> Result<u128, String> {
        plus(
            plus(times(2, x.m)?, times(cell.square_twice, x.s)?)?,
            times(times(2, cell.inversion)?, x.i)?,
        )
    };
    let unit = weighted(unit)?;
    Fraction::new(
        plus(
            times(10, weighted(c)?)?,
            times(times(cell.native_tenths, c.logical)?, unit)?,
        )?,
        times(10, unit)?,
    )
    .map_err(|e| format!("{e:?}"))
}
fn frac(x: Fraction) -> Value {
    json!({"numerator":x.numerator().to_string(),"denominator":x.denominator().to_string()})
}
pub fn analyze(bundle: &Value) -> Result<Value, String> {
    if text(bundle, "/schema")? != "pencil-price-pair-input/v1" {
        return Err("unknown input schema".into());
    }
    let c = &bundle["candidate"];
    let q = &bundle["reference"];
    let ce = expected(&c["expected"])?;
    let qe = expected(&q["expected"])?;
    if ce.session != qe.session
        || ce.binary_sha256 != qe.binary_sha256
        || ce.seed != qe.seed
        || ce.workload != qe.workload
        || ce.round != qe.round
        || ce.warmup != qe.warmup
        || ce.role != "candidate"
        || qe.role != "reference"
    {
        return Err("unmatched pair identity".into());
    }
    let ci = if c["record"].is_null() {
        None
    } else {
        Some(importer::import(&c["record"], &ce)?)
    };
    let qi = if q["record"].is_null() {
        None
    } else {
        Some(importer::import(&q["record"], &qe)?)
    };
    let p: u64 = n(bundle, "/p")?.try_into().map_err(|_| "p overflow")?;
    let r: u64 = n(bundle, "/r")?.try_into().map_err(|_| "r overflow")?;
    if p <= 2 || r <= 2 {
        return Err("invalid expected modulus".into());
    }
    let mut blockers = Vec::new();
    for (record, prefix) in [
        (&c["record"], "/detail/candidate"),
        (&q["record"], "/detail/engine"),
    ] {
        for (key, expected) in [("p", p), ("r", r)] {
            let path = format!("{prefix}/{key}");
            if record.pointer(&path).is_none_or(Value::is_null) {
                blockers.push(format!("missing observed modulus {path}"));
            } else if n(record, &path)? != u128::from(expected) {
                return Err("modulus identity mismatch".into());
            }
        }
    }
    if !ci.as_ref().is_some_and(|i| i.verified)
        || !qi.as_ref().is_some_and(|i| i.verified)
        || ce.warmup
    {
        blockers.push("pair_not_verified_measured".to_string());
    }
    let protocol = text(bundle, "/protocol_sha256")?;
    if let Err(e) = review(
        &bundle["coverage_review"],
        &[&ce.binary_sha256, &qe.binary_sha256],
        protocol,
    ) {
        blockers.push(e);
    }
    let mapped = (|| {
        let a = map(
            &c["record"],
            true,
            ci.as_ref()
                .and_then(|i| i.normalized_scalar_events)
                .ok_or("candidate scalar incomplete")?,
            p,
        )?;
        let b = map(
            &q["record"],
            false,
            qi.as_ref()
                .and_then(|i| i.normalized_scalar_events)
                .ok_or("reference scalar incomplete")?,
            p,
        )?;
        let u = arithmetic(&c["record"], "/detail/source_addition")?;
        let v = arithmetic(&q["record"], "/detail/arithmetic_price/source_addition")?;
        if (u.m, u.s, u.i, u.logical) != (v.m, v.s, v.i, v.logical) {
            return Err("matched source unit mismatch".into());
        }
        Ok::<_, String>((a, b, u))
    })();
    if let Err(e) = &mapped {
        blockers.push(e.clone());
    }
    let mut rows = vec![];
    for cell in grid() {
        let mut row = json!({"square_twice":cell.square_twice,"inversion":cell.inversion,"native_tenths":cell.native_tenths,"candidate":null,"reference":null,"ratio":null,"candidate_calibration":null,"reference_calibration":null,"candidate_including_calibration":null,"reference_including_calibration":null,"ratio_including_calibration":null});
        if blockers.is_empty() {
            let ((a, ac), (b, bc), u) = mapped.as_ref().map_err(Clone::clone)?;
            let cp = price(*a, *u, cell)?;
            let rp = price(*b, *u, cell)?;
            let cc = price(*ac, *u, cell)?;
            let rc = price(*bc, *u, cell)?;
            let ct = cp.plus(cc).map_err(|e| format!("{e:?}"))?;
            let rt = rp.plus(rc).map_err(|e| format!("{e:?}"))?;
            row["candidate"] = frac(cp);
            row["reference"] = frac(rp);
            row["ratio"] = frac(cp.divided_by(rp).map_err(|e| format!("{e:?}"))?);
            row["candidate_calibration"] = frac(cc);
            row["reference_calibration"] = frac(rc);
            row["candidate_including_calibration"] = frac(ct);
            row["reference_including_calibration"] = frac(rt);
            row["ratio_including_calibration"] =
                frac(ct.divided_by(rt).map_err(|e| format!("{e:?}"))?);
        }
        rows.push(row);
    }
    Ok(
        json!({"schema":"pencil-price-pair-output/v1","policy":POLICY,"mapping_sha256":mapping_sha256(),"protocol_sha256":protocol,"candidate_record_id":ci.as_ref().map(|i|&i.record_id),"reference_record_id":qi.as_ref().map(|i|&i.record_id),"coverage_blockers":blockers,"cells":rows,"raw_input":bundle,"resource_scope":"Report serialization, allocator, process/harness/infrastructure resources retained in raw_input; no GAE conversion. Algorithm calibration retained separately and also included in explicit combined ratio."}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn zero_geometry() -> Value {
        let mut out = serde_json::Map::new();
        for k in [
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
            out.insert(k.into(), json!(0));
        }
        Value::Object(out)
    }
    fn field(m: u64) -> Value {
        json!({"m":m,"s":0,"i":0,"field_adds":0,"bit_ops":0})
    }
    fn pair_fixture() -> Value {
        let geom = zero_geometry();
        let segments = json!({"solver_including_cold_setup":geom,"additional_source_verification":geom,"source_unit_calibration":geom,"report_assembly":geom});
        let adapter = json!({"schema":"pencil-adapter-accounting/v1-draft","geometry":{"schema":"engine-geometry-partition/v1-draft","partition_sum_verified":true,"total":geom,"segments":segments},"source_unit_calibration_arithmetic":{"preparation":field(2),"addition":field(1),"total":field(3)}});
        let mut semantic = serde_json::Map::new();
        for k in [
            "scalar_adds",
            "walk_label_calls",
            "scalar_muls",
            "scalar_negations",
            "scalar_inversions",
            "modular_reductions",
            "map_key_comparisons",
            "map_entry_visits",
            "map_cloned_entries",
            "collection_entry_visits",
            "collection_elements_copied",
            "collection_elements_initialized",
            "collection_comparisons",
        ] {
            semantic.insert(k.into(), json!(0));
        }
        let observation = json!({"modulus":101,"prime":true,"counts":{"modular_muls":0,"modular_squares":0,"standalone_modular_reductions":0}});
        let detail = json!({"candidate":{"p":101,"r":101,"counts":field(10),"arithmetic":{"base_adds":0,"base_muls":0,"base_inversions":0,"scalar_adds":0,"scalar_muls":0,"scalar_inversions":0},"symbolic":{"semantic_counts":semantic},"walk_label_calls":0},"cold_setup":{"semantic_counts":semantic,"outer_primality":{"schema":"pencil-outer-primality/v1","observations":{"base_modulus":observation,"scalar_modulus":observation}}},"additional_source_verification":field(0),"source_addition":field(1),"adapter_accounting":adapter});
        let mut counters = serde_json::Map::new();
        for key in [
            "field_multiplications",
            "field_squarings",
            "field_inversions",
            "field_additions_uncharged",
            "linear_bit_operations_uncharged",
            "scalar_adds_uncharged",
            "scalar_muls_uncharged",
            "scalar_inversions_uncharged",
            "rho_semantic_key_comparisons_v1",
            "rho_semantic_entry_visits_v1",
            "rho_semantic_elements_copied_v1",
            "rho_semantic_elements_initialized_v1",
            "rho_observed_hash_calls_v1",
            "partition_hashes_uncharged",
            "rng_draws_uncharged",
        ] {
            counters.insert(key.into(), json!(0));
        }
        counters.insert("field_multiplications".into(), json!(5));
        counters.insert("rho_hash_policy_rust_default_v1".into(), json!(1));
        let mut bundle = json!({"schema":"pencil-price-pair-input/v1","p":101,"r":101,"protocol_sha256":"a".repeat(64),"coverage_review":null});
        for (arm, role, method) in [
            (
                "candidate",
                "candidate",
                "ic.three_leads_genus2_pencil_symbolic_v2",
            ),
            ("reference", "reference", "rho.three_leads_negation_v1"),
        ] {
            let e = json!({"session":"software-fixture-not-experiment","binary_sha256":"b".repeat(64),"seq":1,"round":1,"warmup":false,"arm":arm,"role":role,"seed":"1","workload":{"workload_id":"synthetic-analyzer-test"},"method":{"id":method,"params":{}}});
            let rec = json!({"schema":"ecbench.record/v1","record_id":arm,"session_id":e["session"],"binary_sha256":e["binary_sha256"],"seq":1,"round":1,"warmup":false,"arm":arm,"role":role,"algorithm_seed":"1","workload":e["workload"],"method":e["method"],"outcome":{"status":"verified","matches_target":true,"matches_planted":true,"error":null},"detail":if arm=="candidate"{detail.clone()}else{json!({"engine":{"p":101,"r":101},"arithmetic_price":{"source_addition":field(1)},"adapter_accounting":adapter})},"counters":counters});
            bundle[arm] = json!({"record":rec,"expected":e});
        }
        bundle
    }
    #[test]
    fn fixed_map_hand_arithmetic_and_calibration() {
        let b = pair_fixture();
        let (a, cal) = map(&b["candidate"]["record"], true, 0, 101).unwrap();
        assert_eq!(a.m, 10);
        assert_eq!(cal.m, 3);
        for cell in grid() {
            assert_eq!(
                price(
                    a,
                    Priced {
                        m: 1,
                        s: 0,
                        i: 0,
                        logical: 0
                    },
                    cell
                )
                .unwrap(),
                Fraction::new(10, 1).unwrap()
            );
        }
        let mut missing = b["candidate"]["record"].clone();
        missing["detail"]["candidate"]["symbolic"]["semantic_counts"]["walk_label_calls"] =
            Value::Null;
        assert!(map(&missing, true, 0, 101)
            .unwrap_err()
            .contains("walk_label_calls"));
    }
    #[test]
    fn whole_pair_without_review_stays_null() {
        let b = pair_fixture();
        let out = analyze(&b).unwrap();
        assert_eq!(out["cells"].as_array().unwrap().len(), 36);
        assert!(!out["coverage_blockers"].as_array().unwrap().is_empty());
        for row in out["cells"].as_array().unwrap() {
            assert_eq!(row["ratio"], Value::Null);
            assert_eq!(row["ratio_including_calibration"], Value::Null);
        }
        let mut wrong = b;
        wrong["reference"]["record"]["workload"]["workload_id"] = json!("other");
        assert!(analyze(&wrong).is_err());
    }
    #[test]
    fn atomic_normalization_preserves_overflow_and_missing() {
        assert!(price(
            Priced {
                m: u128::MAX,
                s: 0,
                i: 0,
                logical: 0
            },
            Priced {
                m: 1,
                s: 0,
                i: 0,
                logical: 0
            },
            grid()[0]
        )
        .is_err());
        let mut b = pair_fixture();
        b["candidate"]["record"]["detail"]["candidate"]["arithmetic"]["base_inversions"] = json!(1);
        assert!(map(&b["candidate"]["record"], true, 0, 101)
            .unwrap_err()
            .contains("underflow"));
    }
    #[test]
    fn boolean_coverage_cannot_admit_price() {
        let mut b = pair_fixture();
        b["coverage_review"] = json!({"complete":true});
        let out = analyze(&b).unwrap();
        assert!(out["cells"]
            .as_array()
            .unwrap()
            .iter()
            .all(|r| r["ratio"].is_null()));
    }
    #[test]
    fn synthetic_review_fixture_exercises_price_path_and_digest_rejection() {
        let dir = std::env::temp_dir().join(format!(
            "pencil-analyzer-software-fixture-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&dir).unwrap();
        let source = dir.join("source.txt");
        let report = dir.join("review.txt");
        let protocol = dir.join("protocol.txt");
        fs::write(&source, b"synthetic software fixture source\n").unwrap();
        fs::write(
            &report,
            b"synthetic software fixture review, not scientific evidence\n",
        )
        .unwrap();
        fs::write(&protocol, b"synthetic software fixture protocol\n").unwrap();
        let sh = digest(&fs::read(&source).unwrap());
        let ph = digest(&fs::read(&protocol).unwrap());
        let categories:Vec<_>=CATEGORIES.iter().map(|category|json!({"category":category,"disposition":"covered","uncovered_sites":[],"source_sites":[{"sha256":sh,"line_start":1,"line_end":1,"primitive":"software fixture"}]})).collect();
        let mut b = pair_fixture();
        b["protocol_sha256"] = json!(ph);
        b["coverage_review"] = json!({"schema":"pencil-algorithm-coverage-review/v1","status":"reviewed_complete","policy":POLICY,"mapping_sha256":mapping_sha256(),"protocol_sha256":ph,"protocol_path":protocol,"rust_toolchain":"1.99.1","producer_identity":"software-fixture-producer","reviewer_identity":"software-fixture-reviewer","binary_sha256":["b".repeat(64)],"source_files":[{"path":source,"sha256":sh}],"independent_report":{"path":report,"sha256":digest(&fs::read(&report).unwrap())},"categories":categories});
        let out = analyze(&b).unwrap();
        assert_eq!(out["coverage_blockers"], json!([]));
        for row in out["cells"].as_array().unwrap() {
            assert_eq!(row["ratio"], json!({"numerator":"2","denominator":"1"}));
            assert_eq!(
                row["ratio_including_calibration"],
                json!({"numerator":"13","denominator":"8"})
            );
        }
        fs::write(&source, b"changed").unwrap();
        assert!(analyze(&b).unwrap()["cells"]
            .as_array()
            .unwrap()
            .iter()
            .all(|r| r["ratio"].is_null()));
        fs::remove_dir_all(&dir).unwrap();
    }
    #[test]
    fn paired_session_binary_and_seed_are_enforced() {
        for (expected_key, record_key) in [
            ("session", "session_id"),
            ("binary_sha256", "binary_sha256"),
            ("seed", "algorithm_seed"),
        ] {
            let mut b = pair_fixture();
            let changed = if expected_key == "binary_sha256" {
                "c".repeat(64)
            } else {
                "2".into()
            };
            b["reference"]["expected"][expected_key] = json!(changed);
            b["reference"]["record"][record_key] = json!(changed);
            assert!(analyze(&b).unwrap_err().contains("unmatched pair identity"));
        }
    }
    #[test]
    fn missing_and_capped_details_retain_null_rows() {
        for missing in [false, true] {
            let mut b = pair_fixture();
            if missing {
                b["candidate"]["record"] = Value::Null;
            } else {
                b["candidate"]["record"]["outcome"] = json!({"status":"exhausted","matches_target":false,"matches_planted":false,"error":null});
                b["candidate"]["record"]["detail"] = Value::Null;
            }
            let out = analyze(&b).unwrap();
            assert_eq!(out["cells"].as_array().unwrap().len(), 36);
            assert_eq!(out["raw_input"], b);
            assert!(out["cells"]
                .as_array()
                .unwrap()
                .iter()
                .all(|x| x["ratio"].is_null() && x["ratio_including_calibration"].is_null()));
        }
        let mut contradiction = pair_fixture();
        contradiction["candidate"]["record"]["outcome"]["status"] = json!("exhausted");
        contradiction["candidate"]["record"]["detail"]["candidate"]["p"] = json!(103);
        assert!(analyze(&contradiction).unwrap_err().contains("modulus"));
    }
    #[test]
    fn calibration_tampering_and_missing_components_are_refused() {
        let mut b = pair_fixture();
        b["candidate"]["record"]["detail"]["adapter_accounting"]
            ["source_unit_calibration_arithmetic"]["total"]["m"] = json!(100);
        assert!(analyze(&b)
            .unwrap_err()
            .contains("calibration partition mismatch"));
        let mut b = pair_fixture();
        b["candidate"]["record"]["detail"]["adapter_accounting"]
            ["source_unit_calibration_arithmetic"]["addition"]["m"] = json!(2);
        b["candidate"]["record"]["detail"]["adapter_accounting"]
            ["source_unit_calibration_arithmetic"]["total"]["m"] = json!(4);
        assert!(analyze(&b)
            .unwrap_err()
            .contains("calibration source unit mismatch"));
        let mut b = pair_fixture();
        b["candidate"]["record"]["detail"]["adapter_accounting"]
            ["source_unit_calibration_arithmetic"]["preparation"]["m"] = Value::Null;
        assert!(map(&b["candidate"]["record"], true, 0, 101)
            .unwrap_err()
            .contains("preparation/m"));
    }
    #[test]
    fn scalar_constructor_work_uses_native_not_base_field_weights() {
        let mut g = zero_geometry();
        g["scalar_constructor_prime_calls"] = json!(1);
        g["scalar_constructor_modular_muls"] = json!(3);
        g["scalar_constructor_modular_squares"] = json!(4);
        g["scalar_constructor_standalone_reductions"] = json!(2);
        let v = json!({"geometry":g});
        let counts = geometry(&v, "/geometry").unwrap();
        assert_eq!((counts.m, counts.s, counts.i, counts.logical), (0, 0, 0, 9));
        for square_twice in [0, 1, 2] {
            let c = crate::Cell {
                square_twice,
                inversion: 1000,
                native_tenths: 1,
            };
            assert_eq!(
                price(
                    counts,
                    Priced {
                        m: 100,
                        s: 0,
                        i: 0,
                        logical: 0
                    },
                    c
                )
                .unwrap(),
                Fraction::new(9, 10).unwrap()
            );
        }
    }
}
