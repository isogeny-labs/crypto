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
    let estimate = p["sample_size"]["preflight_runtime"]["estimated_total_seconds"].as_f64();
    check(
        estimate.is_some() || p["freeze"]["status"] != "frozen",
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
