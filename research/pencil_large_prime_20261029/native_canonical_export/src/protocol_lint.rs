//! Canonical semantic rules; unsupported seed/blinding/power paths fail closed.
use super::*;
fn text(v: &V) -> bool {
    v.as_str().is_some_and(|s| !s.trim().is_empty())
}
fn decisive(v: &V) -> bool {
    matches!(v.as_str(), Some("one_factor" | "factorial" | "absolute"))
}
pub fn lint(p: &V) -> Result<()> {
    let aa = arr(&p["arms"])?;
    let arms: BTreeMap<_, _> = aa
        .iter()
        .map(|a| Ok((s(&a["id"])?, a)))
        .collect::<Result<_>>()?;
    let comps = arr(&p["variable_ledger"]["comparisons"])?;
    let cm: BTreeMap<_, _> = comps
        .iter()
        .map(|a| Ok((s(&a["id"])?, a)))
        .collect::<Result<_>>()?;
    let metrics: Vec<_> = arr(&p["metrics"]["primary"])?
        .iter()
        .chain(arr(&p["metrics"]["secondary"])?)
        .collect();
    for values in [
        aa.iter().collect::<Vec<_>>(),
        arr(&p["claims"])?.iter().collect(),
        comps.iter().collect(),
        metrics.clone(),
    ] {
        let mut seen = BTreeSet::new();
        for a in values {
            check(seen.insert(s(&a["id"])?), "duplicate protocol identity")?;
        }
    }
    let names: BTreeSet<_> = arr(&p["variable_ledger"]["factors"])?
        .iter()
        .chain(arr(&p["variable_ledger"]["independent"])?)
        .map(|a| s(&a["name"]))
        .collect::<Result<_>>()?;
    for c in comps {
        for a in arr(&c["arms"])? {
            check(arms.contains_key(s(a)?), "comparison unknown arm")?;
        }
        for d in arr(&c["differs_in"])? {
            check(names.contains(s(d)?), "comparison unknown factor")?;
        }
        match c["design"].as_str() {
            Some("one_factor") => {
                check(arr(&c["differs_in"])?.len() == 1, "one-factor comparison")?
            }
            Some("aa") => check(arr(&c["arms"])?.len() >= 2, "AA needs two arms")?,
            Some("confounded") => check(text(&c["reason"]), "confounded reason")?,
            Some("absolute") => check(text(&c["threshold"]), "absolute threshold")?,
            _ => {}
        }
    }
    let ctl = &p["controls"];
    if ctl["aa"]["present"] == true {
        check(
            cm.get(s(&ctl["aa"]["comparison"])?)
                .is_some_and(|c| c["design"] == "aa"),
            "invalid AA comparison",
        )?;
    } else {
        check(
            comps.iter().any(|c| c["design"] == "aa") || text(&ctl["aa"]["not_applicable_reason"]),
            "absent AA reason",
        )?;
    }
    for (k, role, field) in [
        ("positive", "control_positive", "expected_detection"),
        ("negative", "control_negative", "null_population"),
    ] {
        let c = &ctl[k];
        if c["present"] == true {
            check(
                arms.get(s(&c["arm"])?).is_some_and(|a| a["role"] == role),
                "control role mismatch",
            )?;
            check(
                if k == "positive" {
                    text(&c[field])
                } else {
                    !c[field].is_null()
                },
                "control expectation absent",
            )?;
        } else {
            check(text(&c["not_applicable_reason"]), "absent control reason")?;
        }
    }
    for c in arr(&p["claims"])? {
        if c["class"] == "comparative" {
            check(
                !arr(&c["decided_by"])?.is_empty(),
                "comparative claim no decision",
            )?;
        }
        for id in arr(&c["decided_by"])? {
            let comp = cm.get(s(id)?).ok_or("claim unknown comparison")?;
            if c["class"] == "comparative" {
                check(decisive(&comp["design"]), "nondecisive claim comparison")?;
            }
        }
    }
    for h in p["hypotheses"].as_array().into_iter().flatten() {
        for pr in arr(&h["predictions"])? {
            check(
                metrics.iter().any(|m| m["id"] == pr["metric"]),
                "unknown prediction metric",
            )?;
            let role = pr["role"].as_str().unwrap_or("decisive");
            if pr["provenance"]["sample_status"] == "in_sample" {
                check(role == "calibration", "in-sample decisive prediction")?;
            }
            if pr["provenance"]["sample_status"] == "partially_in_sample" && role == "decisive" {
                check(
                    text(&pr["provenance"]["decisive_scope"]),
                    "partial sample scope absent",
                )?;
            }
        }
    }
    for id in arr(&p["decision_rule"]["comparison_ids"])? {
        check(
            cm.get(s(id)?).is_some_and(|c| decisive(&c["design"])),
            "invalid decision comparison",
        )?;
    }
    let power = &p["sample_size"]["power"];
    check(
        !(p["decision_rule"]["multiple_comparison_correction"] == "none"
            && n(&power["tests_in_family"])? > 1),
        "multiple-comparison correction absent",
    )?;
    check(
        power["method"] != "rank_test",
        "rank-test protocol unsupported by this study adapter; cannot skip power rule",
    )?;
    let rz = &p["randomization"];
    if rz["arm_order"] == "fixed" {
        check(
            text(&rz["arm_order_reason"]),
            "fixed arm order reason absent",
        )?;
    }
    check(
        rz["seed_derivation"]["method"] == "harness_drawn" && text(&rz["seed_derivation"]["note"]),
        "only documented harness-drawn seeds supported",
    )?;
    check(
        rz["blinding"]["analysis_labels_masked"] == false,
        "masked analysis unsupported",
    )?;
    let timed = arr(&p["metrics"]["primary"])?
        .iter()
        .any(|m| m["kind"] == "timed");
    if timed {
        check(
            text(&p["measurement"]["timed_primary_justification"]),
            "timed justification absent",
        )?;
        check(
            matches!(
                p["measurement"]["isolation_required"].as_str(),
                Some("L2" | "L3" | "tierB" | "tierC")
            ),
            "timed primary requires L2",
        )?;
    }
    for m in metrics {
        if m["kind"] == "derived" {
            check(text(&m["formula"]), "derived metric formula absent")?;
        }
    }
    let ck = &p["execution"]["checkpoint"];
    if ck["policy"] != "none" {
        check(
            n(&ck["interval_seconds"]).is_ok_and(|n| n > 0)
                && n(&ck["grace_seconds"]).is_ok_and(|n| n > 0)
                && ck["restore_validation"] != "none",
            "invalid checkpoint contract",
        )?;
    }
    check(
        !(p["execution"]["measurement_class"] == "measurement" && ck["policy"] == "criu"),
        "CRIU computation not measurement",
    )?;
    let characterization = unknown_runtime_gate(p)?;
    let estimate = p["sample_size"]["preflight_runtime"]["estimated_total_seconds"].as_f64();
    check(
        estimate.is_some() || characterization || p["freeze"]["status"] != "frozen",
        "frozen protocol preflight estimate absent",
    )?;
    if let (Some(timeout), Some(estimate)) = (p["execution"]["timeout_seconds"].as_f64(), estimate)
    {
        let count = n(&p["sample_size"]["instances"])?
            .checked_mul(n(&p["sample_size"]["replicates"])?)
            .ok_or("sample overflow")?
            .max(1);
        check(
            timeout == 0. || estimate == 0. || timeout >= estimate / count as f64,
            "timeout below preflight per sample",
        )?;
    }
    for a in aa {
        if let Some(same) = a["same_instrument_as"].as_str() {
            let b = arms.get(same).ok_or("unknown same instrument")?;
            for key in ["commit", "binary_sha256", "threads"] {
                check(
                    a["instrument"][key] == b["instrument"][key],
                    "same instrument mismatch",
                )?;
            }
        }
    }
    Ok(())
}

/// Explicit, bounded unknown-runtime gate; never a runtime estimate or launch approval.
/// Bindings use absolute paths, like existing native package bindings.
pub(crate) fn unknown_runtime_gate(p: &V) -> Result<bool> {
    let Some(g) = p["extensions"].get("unknown_runtime_characterization_gate") else {
        return Ok(false);
    };
    let ext = &p["extensions"];
    let upper = &ext["upper_gate"];
    for (key, existing) in [
        ("resource_authorization", &ext["resource_authorization"]),
        ("execution_request", &upper["execution_request"]),
        (
            "operational_review",
            &ext["continuation_envelope"]["operational_guard"]["independent_review"],
        ),
    ] {
        check(
            g[key] == *existing,
            "unknown runtime shadow binding mismatch",
        )?;
    }
    check(
        g["prepared_spec"]["path"] == upper["spec"]
            && g["prepared_spec"]["sha256"] == upper["spec_sha256"],
        "unknown runtime shadow spec mismatch",
    )?;
    let model = &ext["finite_cost_model"];
    check(
        model["policy"] == "pencil-algorithm-cost/atomic-scalar-disjoint-v1"
            && model["mapping_sha256"]
                == "450efc42a1d73d1cfcc6d99425da9c36a129f1c62c45763aaf6823add003587d"
            && model["square_per_multiply"] == json!([0, 0.5, 1])
            && model["inversion_per_multiply"] == json!([1, 10, 100, 1000])
            && model["native_GAE_per_event"] == json!([0, 0.1, 1])
            && model["qualified_ratios_available"] == false,
        "unknown runtime finite model mismatch",
    )?;
    let replacement = &ext["replacement_authorization"];
    check(
        replacement["prior_run_id"] == "pencil-p186379-20261030-b1"
            && replacement["prior_failure"] == "original UTC deadline expired"
            && replacement["prior_observed_rows"] == 2
            && replacement["prior_warmup_rows"] == 2
            && replacement["prior_measured_rows"] == 0
            && replacement["prior_missing_rows"] == 16,
        "unknown runtime prior failure lineage",
    )?;
    let known = &ext["known_data"]["replacement_context"];
    check(
        known["prior_run"] == "pencil-p186379-20261030-b1"
            && known["observations_known_at_design"] == true
            && known["records_enrolled"] == false
            && known["terminal_state"]["sha256"]
                == "d3985784784f82244779f0db72bb7e996a7947f571e11fa23701bd856d9580be",
        "unknown runtime known-data lineage",
    )?;
    bound(&known["terminal_state"])?;
    check(
        arr(&p["amendments"])?.iter().any(|a| {
            a["id"] == "A-known-prior-b1-prospective-replacement-20261030"
                && a["filed_before_affected_data"] == true
        }),
        "unknown runtime prospective amendment absent",
    )?;
    check(
        replacement["count"] == 1
            && replacement["prospective_only"] == true
            && replacement["prior_rows_enrolled_in_replacement"] == false
            && replacement["early_rejection_instrument_integrated"] == false
            && replacement["automatic_further_scientific_retry"] == false
            && replacement["prior_costs_preserved"] == true,
        "unknown runtime replacement semantics",
    )?;
    check(
        upper["enrolled_in_cohort"] == false
            && upper["shares_continuation_budget"] == true
            && upper["requires_independent_nonwall_replay"] == true
            && upper["admission_conditions"] == g["remaining_cohort_blocked_until"]
            && ext["parent_research_scope"]["upper_gate_rows_reused_in_cohort"] == false
            && ext["scientific_admission"] == false
            && ext["launch_authorized"] == false,
        "unknown runtime gate admission barriers",
    )?;
    check(
        p["execution"]["retries"] == json!({"infrastructure_only":true,"max_attempts":2}),
        "unknown runtime retry envelope",
    )?;
    check(p["decision_rule"]["comparison_ids"] == json!(["C-method"])
        && p["decision_rule"]["statistic"] == "All36 exact ratios of complete matched price sums across5measured pairs, each stream one pair. Complete denominator required. Separate algorithm-only, calibration-only and combined prices; no superiority test or qualified clock ratio.", "unknown runtime finite decision rule")?;
    check(
        g["schema"] == "pencil-bounded-unknown-runtime-gate/v1",
        "unknown runtime gate schema",
    )?;
    check(
        p["sample_size"]["preflight_runtime"]["estimated_total_seconds"].is_null(),
        "characterization estimate must remain unknown",
    )?;
    check(
        g["runtime_estimate_status"] == "unknown"
            && g["resource_caps_are_not_runtime_estimates"] == true,
        "unknown runtime declaration",
    )?;
    let scope = &p["extensions"]["analysis_scope"];
    check(
        scope["kind"] == "gate-p186379-target0/v1"
            && scope["expected_workloads"] == 1
            && scope["expected_executions"] == 18
            && scope["expected_measured_pairs"] == 5
            && scope["prepared_plan_sha256"]
                == "481d795e6208cfb3ca648c463de2d7b232adfb7d28bebc341afa4f58e119dfcf"
            && scope["charged_separately_from_full_cohort"] == true,
        "unknown runtime requires exact bounded upper gate",
    )?;
    check(
        p["sample_size"]["instances"] == 1
            && p["sample_size"]["replicates"] == 5
            && p["sample_size"]["warmups"] == 1,
        "unknown runtime gate sample geometry",
    )?;
    let arms = arr(&p["arms"])?;
    let names: BTreeSet<_> = arms.iter().map(|a| s(&a["id"])).collect::<Result<_>>()?;
    check(
        arms.len() == 3 && names == BTreeSet::from(["pencil", "rho", "rho_AA"]),
        "unknown runtime gate arms",
    )?;
    let primary = arr(&p["metrics"]["primary"])?;
    check(
        primary.len() == 1
            && primary[0]["id"] == "finite_algorithm_gae"
            && primary[0]["kind"] == "derived"
            && primary[0]["unit"] == "GAE"
            && p["extensions"]["finite_cost_model"]["cells"] == 36,
        "unknown runtime permits finite counted primary only",
    )?;
    check(
        p["extensions"]["timing_status"]["export_wall_valid"] == false
            && p["extensions"]["timing_status"]["this_continuation_clock_qualification"] == false
            && p["extensions"]["canonical_packaging"]["wall_valid"] == false,
        "unknown runtime clocks must remain unqualified",
    )?;
    check(
        g["remaining_cohort_blocked_until"]
            == json!([
                "complete_gate",
                "all_15_measured_independent_nonwall_replays",
                "independent_review",
                "separate_prospective_cohort_admission"
            ])
            && p["extensions"]["parent_research_scope"]["later_cohort_execution_authorized"]
                == false,
        "unknown runtime remainder block",
    )?;
    let env = &p["extensions"]["continuation_envelope"];
    let cap = n(&env["cumulative_active_worker_seconds"])?;
    let timeout = n(&env["seconds_per_execution"])?;
    check(
        cap > 0
            && timeout > 0
            && timeout <= cap
            && n(&env["memory_bytes"])? > 0
            && n(&env["coefficients_per_execution"])? > 0
            && env["max_active_payloads"] == 1,
        "unknown runtime positive finite caps",
    )?;
    check(
        p["execution"]["timeout_seconds"] == env["seconds_per_execution"]
            && p["execution"]["placement"]["memory_limit_bytes"] == env["memory_bytes"],
        "unknown runtime execution cap mismatch",
    )?;
    let utc = |v: &V| -> Result<i64> {
        chrono::DateTime::parse_from_rfc3339(s(v)?)
            .map(|t| t.timestamp())
            .map_err(|e| e.to_string())
    };
    let start = utc(&env["authorization_start_utc"])?;
    let end = utc(&env["absolute_deadline_utc"])?;
    check(
        end > start && u64::try_from(end - start).map_err(|e| e.to_string())? <= cap,
        "unknown runtime absolute budget span",
    )?;
    let authorization = load(&bound(&g["resource_authorization"])?)?;
    for key in [
        "scientific_replacement_admission",
        "remaining_cohort_admitted",
        "timing_design_approved",
        "launch_ready",
    ] {
        check(
            authorization[key] == false,
            "unknown runtime authorization must remain resource only",
        )?;
    }
    check(
        authorization["schema"] == "pencil-continuation-resource-authorization/v1"
            && authorization["cumulative_active_worker_seconds"]
                == env["cumulative_active_worker_seconds"]
            && authorization["absolute_deadline_utc"] == env["absolute_deadline_utc"]
            && authorization["recorded_utc"] == env["authorization_start_utc"],
        "unknown runtime authorization mismatch",
    )?;
    for key in [
        "max_active_payloads",
        "coefficients_per_execution",
        "memory_bytes",
        "seconds_per_execution",
    ] {
        check(
            authorization["limits"][key] == env[key],
            "unknown runtime authorized cap mismatch",
        )?;
    }
    let review = load(&bound(&g["operational_review"])?)?;
    check(
        review["schema"] == "pencil-six-hour-operational-source-review/v1"
            && review["status"] == "accepted_conditional_resource_guard_only"
            && review["resource_guard_scope_accepted"] == true
            && review["no_hard_realtime_claim"] == true,
        "unknown runtime reviewed conditional guard absent",
    )?;
    let guard = &review["guard"];
    check(
        guard["C0"].as_i64() == Some(start) && guard["D"].as_i64() == Some(end),
        "unknown runtime guard epoch mismatch",
    )?;
    let internal = guard["G"].as_i64().ok_or("guard G absent")?;
    let reserve = guard["R"].as_i64().ok_or("guard reserve absent")?;
    check(
        internal > start
            && internal < end
            && reserve > 0
            && i128::from(internal) * 2 - i128::from(start) + i128::from(reserve)
                <= i128::from(end),
        "unknown runtime guard inequality",
    )?;
    let request = load(&bound(&g["execution_request"])?)?;
    check(
        request["checkpoint_interval_seconds"] == 300,
        "unknown runtime checkpoint envelope",
    )?;
    for arm in arms {
        let id = s(&arm["id"])?;
        let role = match id {
            "pencil" => "candidate",
            "rho" => "reference",
            _ => "control_aa",
        };
        check(
            arm["role"] == role
                && (if id == "rho" {
                    arm["same_instrument_as"].is_null()
                } else {
                    arm["same_instrument_as"] == "rho"
                }),
            "unknown runtime arm role or pairing mismatch",
        )?;
        let reference = arms
            .iter()
            .find(|a| a["id"] == "rho")
            .ok_or("missing rho")?;
        check(
            arm["instrument"] == reference["instrument"]
                && arm["instrument"]["flags"]
                    == json!({"RUSTFLAGS":"-C target-cpu=x86-64","CARGO_BUILD_JOBS":"2"})
                && arm["instrument"]["env"]
                    == json!({"RAYON_NUM_THREADS":"1","OMP_NUM_THREADS":"1"}),
            "unknown runtime instrument flags mismatch",
        )?;
        check(
            arm["instrument"]["commit"] == "543a54b4b1c3be7052872af3ffe393a44c69a3c3"
                && arm["instrument"]["image_digest"]
                    == "sha256:e82333a5e77a03270335589848783e1d0cf55e42c7c2f5f35732ee8ba54523fd"
                && arm["instrument"]["threads"] == 1,
            "unknown runtime arm instrument mismatch",
        )?;
        let expected = match s(&arm["id"])? {
            "pencil" => {
                json!({"method":"ic.three_leads_genus2_pencil_symbolic_v2","small_denominator":8,"matrix_coefficient_cap":25000000})
            }
            "rho" => {
                json!({"method":"rho.three_leads_negation_v1","trial_cap":"resolved exact archived per-workload value"})
            }
            "rho_AA" => json!({"method":"rho.three_leads_negation_v1","trial_cap":"same as rho"}),
            _ => return Err("unknown runtime arm".into()),
        };
        check(
            arm["parameters"] == expected,
            "unknown runtime arm parameters mismatch",
        )?;
    }
    check(request["runtime_image_uri"] == "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:e82333a5e77a03270335589848783e1d0cf55e42c7c2f5f35732ee8ba54523fd"
        && request["build"]["image_uri"] == "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:13f27597322fced9230635cedc5a4e228daf2fb951f77652edbbcab17e674c1b", "unknown runtime image mismatch")?;
    check(
        request["schema"] == "isolab.eks.resumable/v1"
            && request["deadline_unix_seconds"].as_i64() == Some(internal),
        "unknown runtime request guard mismatch",
    )?;
    let spec = load(&bound(&g["prepared_spec"])?)?;
    check(
        g["prepared_spec"]["sha256"]
            == "d4ec180d5a2cf65617a5821b565ec0d4952435a77669da65ca77852d2723c32c"
            && request["spec"] == spec,
        "unknown runtime exact prepared spec mismatch",
    )?;
    check(
        request["memory"] == "8Gi"
            && env["memory_bytes"] == 8589934592u64
            && env["coefficients_per_execution"] == 25000000
            && timeout == 3600
            && request["max_restores"] == 1,
        "unknown runtime fixed execution envelope",
    )?;
    check(
        request["build"]["ecbench_sha256"]
            == "a1c8ca89748a22b8a07ea00fd3983061be0d8197d5cae49fca883dcf5dd50eab"
            && request["build"]["source_revision"] == "543a54b4b1c3be7052872af3ffe393a44c69a3c3"
            && arms
                .iter()
                .all(|a| a["instrument"]["binary_sha256"] == request["build"]["ecbench_sha256"]),
        "unknown runtime fixed instrument mismatch",
    )?;
    Ok(true)
}
