//! Structural and provenance checks for the actual native analyzer output.
use super::*;
fn slot_identity(slot: &V, disjoint: bool) -> Result<(String, u64)> {
    Ok((
        s(&slot[if disjoint { "plan" } else { "session_id" }])?.to_owned(),
        n(&slot["seq"])?,
    ))
}
pub(super) fn decimal(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit())
}
pub(super) fn history(ev: &V, p: &V) -> Result<()> {
    let attempts = n(&ev["attempts"])?;
    let resumed = n(&ev["resumed"])?;
    check(
        attempts >= 1 && attempts <= n(&p["execution"]["retries"]["max_attempts"])?,
        "attempts exceed protocol retry admission",
    )?;
    let ck = &ev["checkpoint"];
    let policy = &p["execution"]["checkpoint"]["policy"];
    if resumed > 0 {
        check(
            policy != "none" && ck["policy"] == *policy,
            "restored history incompatible with measurement checkpoint policy",
        )?;
        let limit = p["execution"]
            .get("max_restores")
            .or_else(|| p["execution"]["checkpoint"].get("max_restores"));
        // Separate computation settings cannot authorize restores in measurement epochs.
        let limit = if p["execution"]["measurement_class"] != "measurement"
            && p["extensions"]["active_execution_epoch"] == "computation"
        {
            limit.or_else(|| p["extensions"]["computation_epoch"].get("max_restores"))
        } else {
            limit
        };
        check(
            resumed <= n(limit.ok_or("restored history lacks applicable max_restores")?)?,
            "restores exceed applicable cap",
        )?;
        let written = n(&ck["generations_written"])?;
        let restored = n(&ck["restored_from_generation"])?;
        check(
            written > 0 && restored <= written,
            "restored generation exceeds committed generations",
        )?;
    } else if ck.is_object() {
        check(
            ck["restored_from_generation"].is_null(),
            "zero restores with restored generation",
        )?;
        check(
            ck["policy"] == *policy,
            "checkpoint policy differs from protocol",
        )?;
    }
    Ok(())
}
pub(super) fn plan(plan: &V, p: &V) -> Result<()> {
    let arms = arr(&plan["arms"])?;
    let protocol_arms = arr(&p["arms"])?;
    check(
        arms.len() == protocol_arms.len(),
        "plan arm inventory differs",
    )?;
    let mut ids = BTreeSet::new();
    for a in arms {
        let id = s(&a["name"])?;
        check(ids.insert(id), "duplicate planned arm")?;
        check(
            protocol_arms.iter().any(|x| x["id"] == a["name"]),
            "plan unknown arm",
        )?;
    }
    let mut cells = BTreeMap::<(u64, u64, bool), BTreeSet<u64>>::new();
    for e in arr(&plan["executions"])? {
        let wi = n(&e["workload"])?;
        let ai = n(&e["arm"])?;
        check(
            wi < (arr(&plan["workloads"])?.len() as u64) && ai < (arms.len() as u64),
            "plan index range",
        )?;
        let warm = e["warmup"].as_bool().ok_or("plan warmup boolean")?;
        check(
            cells
                .entry((wi, ai, warm))
                .or_default()
                .insert(n(&e["round"])?),
            "duplicate planned cell",
        )?;
    }
    for wi in 0..arr(&plan["workloads"])?.len() as u64 {
        for ai in 0..arms.len() as u64 {
            for (warm, field) in [(false, "replicates"), (true, "warmups")] {
                let count = cells.get(&(wi, ai, warm)).map_or(0, BTreeSet::len) as u64;
                check(
                    count == n(&p["sample_size"][field])?,
                    "plan lacks prescribed measured/warmup cells",
                )?;
                if ai > 0 {
                    check(
                        cells.get(&(wi, ai, warm)) == cells.get(&(wi, 0, warm)),
                        "plan round identities differ across arms",
                    )?;
                }
            }
        }
    }
    Ok(())
}
pub(super) fn provenance(req: &V, p: &V, files: &mut BTreeMap<PathBuf, V>) -> Result<()> {
    let prov = &req["analysis_provenance"];
    let pins = p["extensions"]["analysis_support_sha256"]
        .as_object()
        .ok_or("analysis source pins absent")?;
    let sources = prov["sources"]
        .as_object()
        .ok_or("analysis provenance source map required")?;
    check(
        pins.keys().collect::<BTreeSet<_>>() == sources.keys().collect::<BTreeSet<_>>(),
        "analysis provenance source set differs",
    )?;
    for (name, pin) in pins {
        check(
            sources[name]["sha256"] == *pin,
            "analysis provenance source pin mismatch",
        )?;
        attach_bound(files, &sources[name], "analysis")?;
    }
    let build = &p["extensions"]["analysis_build"];
    check(
        prov["build_receipt"]["sha256"] == build["receipt_sha256"],
        "analysis build receipt pin mismatch",
    )?;
    let receipt = load(&attach_bound(files, &prov["build_receipt"], "receipt")?)?;
    check(
        receipt["schema"] == "pencil-native-analysis-build/v1"
            && receipt["build_exit_code"] == 0
            && receipt["source_files_match_commit"] == true,
        "invalid native analysis build receipt",
    )?;
    check(
        receipt["source_revision"] == build["source_commit"]
            && receipt["mapping_sha256"] == p["extensions"]["finite_cost_model"]["mapping_sha256"],
        "analysis build source/mapping identity mismatch",
    )?;
    let mut receipt_sources = BTreeMap::new();
    for src in arr(&receipt["sources"])? {
        let path = s(&src["path"])?;
        let names: Vec<_> = pins
            .keys()
            .filter(|k| path == k.as_str() || path.ends_with(&format!("/{k}")))
            .collect();
        check(
            names.len() == 1,
            "build receipt unknown or ambiguous source",
        )?;
        check(
            receipt_sources.insert(names[0], &src["sha256"]).is_none(),
            "duplicate build source",
        )?;
        check(
            src["sha256"] == pins[names[0]],
            "build source digest differs from protocol",
        )?;
    }
    for name in pins
        .keys()
        .filter(|k| k.starts_with("native_price/") || k.starts_with("paired_export/"))
    {
        check(
            receipt_sources.contains_key(name),
            "analysis build omits source",
        )?;
    }
    for name in ["analyze_pair", "analyze_cohort"] {
        let b = &prov["binaries"][name];
        let pinned = &build["binaries"][name];
        check(
            b["sha256"] == pinned["sha256"]
                && b["sha256"]
                    == p["extensions"]["finite_cost_model"]["analyzer_binaries_sha256"][name],
            "analyzer binary pin mismatch",
        )?;
        let path = attach_bound(files, b, "analysis")?;
        let size = fs::metadata(path).map_err(|e| e.to_string())?.len();
        check(
            size == n(&pinned["bytes"])? && size == n(&b["bytes"])?,
            "analyzer binary size mismatch",
        )?;
        let entries: Vec<_> = arr(&receipt["binaries"])?
            .iter()
            .filter(|r| r["name"] == name)
            .collect();
        check(
            entries.len() == 1
                && entries[0]["sha256"] == b["sha256"]
                && entries[0]["bytes"] == b["bytes"],
            "analyzer binary differs from build receipt",
        )?;
    }
    Ok(())
}
fn fraction(v: &V) -> Result<()> {
    if v.is_null() {
        return Ok(());
    }
    let num = s(&v["numerator"])?;
    let den = s(&v["denominator"])?;
    check(
        decimal(num) && decimal(den) && den.bytes().any(|b| b != b'0'),
        "invalid exact nonnegative fraction",
    )
}
fn grid(cells: &V, expected: u64, aggregate: bool) -> Result<()> {
    let rows = arr(cells)?;
    check(rows.len() == 36, "native cost grid must contain36 cells")?;
    let mut i = 0;
    for sq in [0, 1, 2] {
        for inv in [1, 10, 100, 1000] {
            for logical in [0, 1, 10] {
                let c = &rows[i];
                i += 1;
                check(
                    c["square_twice"] == sq
                        && c["inversion"] == inv
                        && c["native_tenths"] == logical,
                    "native cost grid identity mismatch",
                )?;
                if aggregate {
                    let admitted = n(&c["admitted_pairs"])?;
                    check(
                        n(&c["expected_pairs"])? == expected
                            && admitted <= expected
                            && n(&c["missing_or_ineligible_pairs"])? == expected - admitted
                            && c["complete"] == json!(expected > 0 && admitted == expected),
                        "native aggregate count mismatch",
                    )?;
                    check(
                        c.get("qualified_ratio") == Some(&V::Null),
                        "native aggregate qualification must be explicit null",
                    )?;
                    for field in [
                        "candidate_sum",
                        "reference_sum",
                        "algorithm_ratio_of_sums",
                        "candidate_sum_including_calibration",
                        "reference_sum_including_calibration",
                        "ratio_of_sums_including_calibration",
                    ] {
                        let v = c.get(field).ok_or("native aggregate cost field missing")?;
                        fraction(v)?;
                        check(
                            v.is_null() == (c["complete"] == false),
                            "native aggregate complete/null mismatch",
                        )?;
                    }
                    for field in [
                        "candidate",
                        "reference",
                        "candidate_including_calibration",
                        "reference_including_calibration",
                    ] {
                        let v = c["partial_admitted_sums_diagnostic_only"]
                            .get(field)
                            .ok_or("partial diagnostic sum missing")?;
                        check(!v.is_null(), "partial diagnostic sum null")?;
                        fraction(v)?;
                    }
                } else {
                    let mut nulls = 0;
                    for field in [
                        "candidate",
                        "reference",
                        "candidate_including_calibration",
                        "reference_including_calibration",
                    ] {
                        let v = c.get(field).ok_or("native pair cost field missing")?;
                        fraction(v)?;
                        nulls += usize::from(v.is_null());
                    }
                    check(nulls == 0 || nulls == 4, "partial pair prices invalid")?;
                    for field in ["ratio", "ratio_including_calibration"] {
                        let v = c.get(field).ok_or("pair ratio field missing")?;
                        fraction(v)?;
                        check(v.is_null() == (nulls == 4), "pair ratio null mismatch")?;
                    }
                }
            }
        }
    }
    Ok(())
}
pub(super) fn analysis(
    native: &V,
    p: &V,
    slots: &[V],
    runs: &[V],
    diagnostics: &[V],
    receipt: &V,
) -> Result<()> {
    for name in ["mapping", "input_manifest", "outcome_counts"] {
        check(
            native[name].is_object(),
            "native output required object missing",
        )?;
    }
    for name in [
        "aggregate_cells",
        "streams",
        "pair_rows",
        "missing_executions",
        "target_controls",
        "plan_observations",
    ] {
        arr(&native[name])?;
    }
    for name in ["integrity", "authenticity", "AA_rule", "resource_policy"] {
        s(&native[name])?;
    }
    for name in [
        "all_expected_records_present",
        "all_target_cold_AA_passed",
        "all_target_online_AA_passed",
    ] {
        check(
            native[name].is_boolean(),
            "native output required boolean missing",
        )?;
    }
    check(
        native.get("qualified_ratios") == Some(&V::Null),
        "native qualified_ratios must be explicit null",
    )?;
    let workload_ids: BTreeSet<_> = slots.iter().map(|s| s["workload_id"].as_str()).collect();
    let nw = workload_ids.len() as u64;
    let measured = slots.iter().filter(|s| s["warmup"] == false).count() as u64;
    let warm = slots.len() as u64 - measured;
    let observed_warm = runs.iter().filter(|s| s["warmup"] == true).count() as u64;
    check(
        n(&native["expected_workloads"])? == nw
            && n(&native["expected_measured_pairs"])? == nw * 5
            && n(&native["expected_warmup_executions"])? == warm
            && n(&native["observed_warmup_executions"])? == observed_warm,
        "native workload/warmup dimensions differ",
    )?;
    for (k, v) in [
        ("targets", nw),
        ("planned_executions", slots.len() as u64),
        ("measured_executions", measured),
        ("warmup_executions", warm),
    ] {
        check(n(&receipt[k])? == v, "prepared receipt dimensions differ")?;
    }
    check(
        n(&p["sample_size"]["replicates"])? == 5 && n(&p["sample_size"]["warmups"])? == 1,
        "native analyzer requires five streams and one warmup",
    )?;
    check(
        native["all_expected_records_present"] == json!(runs.len() == slots.len()),
        "native missing flag differs",
    )?;
    let mut outcomes = BTreeMap::<String, u64>::new();
    for (r, d) in runs.iter().zip(diagnostics) {
        let status = d["raw_outcome"]["status"]
            .as_str()
            .unwrap_or("missing_status");
        *outcomes
            .entry(format!("{}:{status}", s(&r["arm"])?))
            .or_default() += 1;
    }
    check(
        native["outcome_counts"] == json!(outcomes),
        "native outcome counts differ",
    )?;
    let run_ids: BTreeSet<_> = runs
        .iter()
        .map(|r| s(&r["run_id"]))
        .collect::<Result<_>>()?;
    let mut present = BTreeSet::new();
    let disjoint = disjoint::selected(p);
    let identity = |slot: &V| slot_identity(slot, disjoint);
    for slot in slots {
        if runs.iter().any(|r| {
            r["arm"] == slot["arm"]
                && r["instance_id"] == slot["workload_id"]
                && r["replicate"] == slot["round"]
                && r["warmup"] == slot["warmup"]
        }) {
            present.insert(identity(slot)?);
        }
    }
    let mut misses = BTreeSet::new();
    for m in arr(&native["missing_executions"])? {
        let key = identity(m)?;
        check(misses.insert(key.clone()), "duplicate native missing slot")?;
        check(
            slots.iter().any(|s| {
                identity(s).ok().as_ref() == Some(&key) && s["session_id"] == m["session_id"]
            }) && !present.contains(&key),
            "native missing slot differs from records",
        )?;
    }
    check(
        misses.len() + present.len() == slots.len(),
        "native missing inventory incomplete",
    )?;
    grid(&native["aggregate_cells"], nw * 5, true)?;
    let streams = arr(&native["streams"])?;
    check(streams.len() == 5, "native streams missing")?;
    for (i, st) in streams.iter().enumerate() {
        check(st["stream_index"] == json!(i), "native stream identity")?;
        grid(&st["cells"], nw, true)?;
    }
    let pairs = arr(&native["pair_rows"])?;
    check(pairs.len() as u64 == nw * 5, "native pair rows incomplete")?;
    let mut pair_ids = BTreeSet::new();
    for row in pairs {
        let stream = n(&row["stream_index"])?;
        check(
            stream < 5 && pair_ids.insert((s(&row["workload_id"])?, stream)),
            "native pair identity duplicate/out of range",
        )?;
        let cs = slots
            .iter()
            .find(|s| {
                s["session_id"] == row["session_id"]
                    && (!disjoint || s["plan"] == row["plan"])
                    && s["seq"] == row["candidate_seq"]
                    && s["workload_id"] == row["workload_id"]
                    && s["round"] == row["round"]
                    && s["warmup"] == false
                    && s["arm"] == "pencil"
            })
            .ok_or("native candidate pair differs from plan")?;
        check(
            slots.iter().any(|s| {
                s["session_id"] == cs["session_id"]
                    && (!disjoint || s["plan"] == cs["plan"])
                    && s["seq"] == row["reference_seq"]
                    && s["workload_id"] == cs["workload_id"]
                    && s["round"] == cs["round"]
                    && s["warmup"] == false
                    && s["arm"] == "rho"
            }),
            "native reference pair differs from plan",
        )?;
        for name in ["candidate_record_id", "reference_record_id"] {
            let id = row.get(name).ok_or("pair record identity missing")?;
            if !id.is_null() {
                check(run_ids.contains(s(id)?), "native pair unknown record")?;
            }
        }
        arr(&row["coverage_blockers"])?;
        grid(&row["cells"], 1, false)?;
    }
    Ok(())
}

#[cfg(test)]
mod disjoint_identity_tests {
    use super::*;
    #[test]
    fn missing_future_sessions_are_distinguished_by_plan_without_invention() {
        let a = json!({"plan":"cohort20","session_id":null,"seq":0});
        let b = json!({"plan":"cohort24","session_id":null,"seq":0});
        assert_ne!(
            slot_identity(&a, true).unwrap(),
            slot_identity(&b, true).unwrap()
        );
        assert!(slot_identity(&a, false).is_err());
        let old = json!({"session_id":"retained-session","seq":42});
        assert_eq!(
            slot_identity(&old, false).unwrap(),
            ("retained-session".into(), 42)
        );
    }
}
