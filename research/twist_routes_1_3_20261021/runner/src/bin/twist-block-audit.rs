//! Independent aggregation of E26 raw receipts; no new targets or timings.
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn number(v: &Value, key: &str) -> u64 {
    v[key].as_u64().expect("required recorded count")
}
fn leaf(root: &Path, id: &str, suffix: &str) -> PathBuf {
    root.join("cells").join(format!("{id}.{suffix}"))
}
fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 5 {
        return Err("usage: audit INPUT_CENSUS MATCHED_INPUTS PANEL FRESH_OUTPUT".into());
    }
    let census = Path::new(&args[1]);
    let matched = Path::new(&args[2]);
    let panel = Path::new(&args[3]);
    let out = Path::new(&args[4]);
    if out.exists() {
        return Err("fresh audit output required".into());
    }
    fs::create_dir_all(out)?;
    let workloads = read(census.join("WORKLOADS.json"))?;
    let proofs = read(matched.join("MATCHED_CONTROL_PROOFS.json"))?;
    let rows = read(panel.join("PANEL.json"))?;
    let primary = read(panel.join("ASSESSMENT.json"))?;
    assert_eq!(workloads.as_array().unwrap().len(), 90);
    assert_eq!(proofs.as_array().unwrap().len(), 90);
    assert_eq!(rows.as_array().unwrap().len(), 631);
    for p in proofs.as_array().unwrap() {
        assert_eq!(
            p["dropping_zero_anti_block_recovers_same_polynomial_coefficients"],
            true
        );
    }
    let execution = "research/twist_routes_1_3_20261021/execution-v2/runs";
    let gate = read(format!(
        "{execution}/block-cas-adjudication-20261025-v1/ADJUDICATION.json"
    ))?;
    assert_eq!(gate["accepted"], true);
    let transport = &gate["transport_metadata"];
    let prefix = gate["log_prefix"].as_str().unwrap();
    assert_eq!(transport["stdout_sha256"], sha(format!("{prefix}.stdout"))?);
    assert_eq!(transport["stderr_sha256"], sha(format!("{prefix}.stderr"))?);
    assert_eq!(transport["script_sha256"], sha(census.join("verify.gp"))?);
    let stdout = fs::read_to_string(format!("{prefix}.stdout"))?;
    let marker = stdout
        .lines()
        .find(|s| s.starts_with("ALL_BLOCK_REPLAY_OK "))
        .ok_or("missing replay counts")?
        .split_whitespace()
        .collect::<Vec<_>>();
    let witness_checks = marker[1].parse::<u64>()?;
    let negative_checks = marker[2].parse::<u64>()?;
    let expected_witnesses = workloads
        .as_array()
        .unwrap()
        .iter()
        .map(|w| number(w, "canonical_relation_tuples"))
        .sum::<u64>();
    assert_eq!(witness_checks, expected_witnesses);
    assert!(negative_checks > 0);
    let mut totals = BTreeMap::<String, (u64, u64, u64, u64, u64)>::new();
    let mut input_cases = BTreeMap::new();
    let mut traces = vec![];
    for row in rows.as_array().unwrap() {
        let arm = row["arm"].as_str().unwrap();
        let workload = row["workload"].as_str().unwrap();
        let mut sum = 0u64;
        let mut complete = true;
        let item = totals.entry(arm.into()).or_default();
        item.0 += 1;
        for branch in row["branches"].as_array().unwrap() {
            let Some(id) = branch["job"].as_str() else {
                complete = false;
                item.4 += 1;
                continue;
            };
            let input = leaf(panel, id, "input.json");
            assert_eq!(branch["input_sha256"], sha(&input)?);
            let job = read(input)?;
            if arm == "direct-aa" || arm == "direct-chain" {
                input_cases.insert((workload.to_string(), arm.to_string()), job["case"].clone());
            }
            let native_path = leaf(panel, id, "native.json");
            if let Ok(result) = read(&native_path) {
                assert_eq!(branch["result_sha256"], sha(&native_path)?);
                let ops = number(&result, "field_multiplications_row_reduction");
                sum += ops;
                let step_sum = result["step_log"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|s| number(s, "row_reduction_multiplications"))
                    .sum::<u64>();
                traces.push(json!({"job":id,"recorded_row_multiplications":ops,"step_log_multiplications":step_sum,"total_covers_logged_steps":ops>=step_sum}));
            }
            if branch["native_complete_claim"] != true {
                complete = false;
                continue;
            }
            item.2 += 1;
            let cert = &branch["certificate"];
            if cert["accepted"] != true {
                complete = false;
                continue;
            }
            let script = leaf(panel, id, "certificate.sing");
            let raw = fs::read_to_string(leaf(panel, id, "certificate.stdout"))?;
            let err = fs::read_to_string(leaf(panel, id, "certificate.stderr"))?;
            assert_eq!(cert["input_sha256"], sha(script)?);
            assert_eq!(
                cert["stdout_sha256"],
                sha(leaf(panel, id, "certificate.stdout"))?
            );
            assert_eq!(
                cert["stderr_sha256"],
                sha(leaf(panel, id, "certificate.stderr"))?
            );
            assert!(err.trim().is_empty());
            assert!(raw.lines().any(|s| s == "CERT=1"));
            assert!(raw.contains("E26_CERTIFICATE_DONE"));
            assert!(!raw.contains('?'));
            let dim = raw
                .lines()
                .find_map(|s| s.strip_prefix("VDIM="))
                .ok_or("missing dimension")?
                .parse::<u64>()?;
            assert_eq!(dim, number(&job, "expected"));
            assert_eq!(dim, number(cert, "observed_vdim"));
            if cert["msolve_basis_independently_certified"] == true {
                assert_eq!(cert["msolve_q0_exit_code"], 0);
                assert!(raw.lines().any(|s| s == "MSCERT=1"));
                assert_eq!(
                    cert["msolve_output_sha256"],
                    sha(leaf(panel, id, "certificate.msolve.gb"))?
                );
                item.3 += 1;
            }
        }
        assert_eq!(sum, number(row, "native_row_reduction_multiplications"));
        assert_eq!(row["all_branches_independently_certified"], complete);
        if complete {
            item.1 += 1;
        }
    }
    let by_arm=totals.iter().map(|(arm,(requested,certified,finished,external,unstarted))|json!({"arm":arm,"requested_cells":requested,"certified_cells":certified,"incomplete_cells":requested-certified,"native_finished_branches":finished,"msolve_independently_certified_branches":external,"branches_not_started":unstarted})).collect::<Vec<_>>();
    for a in &by_arm {
        let p = primary["by_arm"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["arm"] == a["arm"])
            .unwrap();
        assert_eq!(p["requested_cells"], a["requested_cells"]);
        assert_eq!(p["certified_cells"], a["certified_cells"]);
    }
    let aa = rows
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["arm"] == "direct-aa")
        .unwrap();
    let id = aa["workload"].as_str().unwrap();
    let direct = rows
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["workload"] == id && r["arm"] == "direct-chain")
        .unwrap();
    assert_eq!(
        input_cases[&(id.into(), "direct-aa".into())],
        input_cases[&(id.into(), "direct-chain".into())]
    );
    let aa_equal = aa["all_branches_independently_certified"] == true
        && direct["all_branches_independently_certified"] == true
        && aa["native_row_reduction_multiplications"]
            == direct["native_row_reduction_multiplications"];
    assert_eq!(primary["same_input_AA_equal_certified_counts"], aa_equal);
    let mut frobenius_control = vec![];
    let mut comparisons = vec![];
    for w in workloads.as_array().unwrap() {
        let id = w["workload"].as_str().unwrap();
        let local = rows
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["workload"] == id)
            .collect::<Vec<_>>();
        let ordinary = local
            .iter()
            .find(|r| r["arm"] == "ordinary-prefix-order")
            .unwrap();
        let frob = local
            .iter()
            .find(|r| r["arm"] == "frobenius-block")
            .unwrap();
        let both = ordinary["all_branches_independently_certified"] == true
            && frob["all_branches_independently_certified"] == true;
        frobenius_control.push(json!({"workload":id,"paired_complete":both,"ordinary_row_multiplications":ordinary["native_row_reduction_multiplications"],"frobenius_row_multiplications":frob["native_row_reduction_multiplications"],"equal_count":both&&ordinary["native_row_reduction_multiplications"]==frob["native_row_reduction_multiplications"]}));
        let baseline = local
            .iter()
            .filter(|r| {
                [
                    "direct-chain",
                    "ordinary-prefix-order",
                    "factored-control",
                    "bits-control",
                ]
                .contains(&r["arm"].as_str().unwrap())
                    && r["all_branches_independently_certified"] == true
            })
            .min_by_key(|r| number(r, "native_row_reduction_multiplications"));
        for arm in [
            "direct-chain",
            "frobenius-block",
            "hybrid-eight",
            "first-pair-invariant",
        ] {
            let row = local.iter().find(|r| r["arm"] == arm).unwrap();
            let paired = baseline.is_some() && row["all_branches_independently_certified"] == true;
            let count = number(row, "native_row_reduction_multiplications");
            let base = baseline.map(|r| number(r, "native_row_reduction_multiplications"));
            let ratio = if paired && base != Some(0) {
                base.map(|n| count as f64 / n as f64)
            } else {
                None
            };
            let p = primary["comparisons"]
                .as_array()
                .unwrap()
                .iter()
                .find(|p| p["workload"] == id && p["arm"] == arm)
                .unwrap();
            assert_eq!(p["paired_complete_affine_certificates"], paired);
            assert_eq!(p["candidate_row_reduction_multiplications"], count);
            assert_eq!(p["baseline_row_reduction_multiplications"], json!(base));
            comparisons.push(json!({"workload":id,"p":w["p"],"B":w["B"],"arity":w["arity"],"arm":arm,"paired_complete_affine_certificates":paired,"candidate_row_multiplications":count,"best_ordinary_row_multiplications":base,"candidate_to_ordinary_ratio":ratio,"all_other_phase_costs_missing":true}));
        }
    }
    let summary = json!({"schema":"twist.block-independent-audit/v1","status":"raw-receipts-and-aggregates-verified","workloads":90,"requested_cells":631,"by_arm":by_arm,"matched_order_coefficient_checks":90,"independent_PARI_signed_witness_checks":witness_checks,"independent_PARI_wrong_sign_checks":negative_checks,"AA_identical_polynomial_inputs":true,"AA_equal_certified_row_multiplications":aa_equal,"counter_trace_entries_audited":traces.len(),"trace_coverage_failures":traces.iter().filter(|r|r["total_covers_logged_steps"]!=true).count(),"frobenius_vs_exact_ordinary_order":frobenius_control,"comparisons":comparisons,"primary_assessment_sha256":sha(panel.join("ASSESSMENT.json"))?,"panel_sha256":sha(panel.join("PANEL.json"))?,"counts_are_row_reduction_multiplications_only":true,"complete_solver_or_DLP_cost":false,"speedup_or_novelty_established":false,"final_E28_targets_opened":false});
    fs::write(
        out.join("AUDIT.json"),
        serde_json::to_string_pretty(&summary)? + "\n",
    )?;
    fs::write(
        out.join("COUNTER_TRACE.json"),
        serde_json::to_string_pretty(&traces)? + "\n",
    )?;
    println!(
        "{}",
        json!({"status":"verified","workloads":90,"cells":631,"AA_equal_certified_counts":aa_equal})
    );
    Ok(())
}
