//! Copied independent full-pipeline qualifier. Scientific execution uses Runpod.
#![allow(clippy::duplicate_mod)]
#[path = "../research/extension_ic_graph_20261115/extension_pipeline_check.rs"]
mod check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_pipeline_collection_check.rs"]
mod collection_check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder_driver.rs"]
mod driver;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder.rs"]
mod remainder;
use serde_json::{json, Value};
use std::{
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    path::Path,
};
const MODES: [&str; 7] = [
    "ic",
    "extension-full-smooth",
    "extension-remainders",
    "rho-plain-1",
    "rho-negation-1",
    "rho-plain-16",
    "rho-negation-16",
];
fn registered_plan(plan: &Value) -> bool {
    let source = plan["source_revision"].as_str().unwrap_or("");
    plan["schema"] == "cover.extension-pipeline.runpod-validation/v1"
        && plan["backend"] == "runpod_user_authorized"
        && plan["runpod_compute_type"] == "CPU"
        && plan["p"] == 1009
        && plan["seed"] == 389
        && plan["m"] == 9
        && plan["small_percent"] == 100
        && plan["b_count"] == 1018081
        && plan["source_cap"] == 65536
        && plan["collection_batch_size"] == 1024
        && plan["max_descent_residuals"] == 8192
        && plan["max_accepted_descents"] == 32
        && plan["mode_order"] == json!(MODES)
        && plan["rounds"] == 1
        && plan["warmup_rounds"] == 0
        && plan["automatic_retries"] == 0
        && plan["fixture_timeout_seconds"] == 60
        && plan["worker_timeout_seconds"] == 600
        && plan["timeout_seconds"] == 4500
        && plan["qualifier_timeout_seconds"] == 900
        && plan["runpod_lifetime_seconds"] == 7200
        && plan["memory_gib"] == 8
        && plan["allow_comparative_claims"] == false
        && plan["complete_total"].is_null()
        && plan["rho_parity"] == false
        && source.len() == 40
        && source.bytes().all(|x| x.is_ascii_hexdigit())
        && plan["runpod_allocation"]["compute_type"] == "CPU"
        && plan["runpod_allocation"]["vcpuCount"] == 4
        && plan["runpod_allocation"]["memoryInGb"] == 8
        && plan["runpod_allocation"]["lifetime_seconds"] == 7200
        && plan["runpod_allocation"]["costPerHr"]
            .as_f64()
            .is_some_and(|v| v > 0.0 && v <= 0.40)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: CHECKER RUN_DIR NEW_SUMMARY".into());
    }
    let dir = Path::new(&args[0]);
    let read = |name: &str| -> Result<Value, Box<dyn std::error::Error>> {
        Ok(serde_json::from_slice(&fs::read(dir.join(name))?)?)
    };
    let plan = read("runtime-plan.json")?;
    let fixture = read("fixture.json")?;
    let public = read("public.json")?;
    let p = public["p"].as_u64().ok_or("p")?;
    let l = public["l"].as_u64().ok_or("l")?;
    let seed = plan["seed"].as_u64().ok_or("seed")?;
    let answer = fixture["expected_scalar"]
        .as_u64()
        .ok_or("held-out answer")?;
    if !registered_plan(&plan)
        || seed != 389
        || p != 1009
        || plan["mode_order"] != json!(MODES)
        || fixture["public"] != public
        || fixture["source_revision"] != plan["source_revision"]
        || fixture["seed"] != seed
        || fixture["workers_receive"] != "public only"
        || answer == 0
        || answer >= l
    {
        return Err("protocol/fixture identity".into());
    }
    let workers: Vec<Value> = MODES
        .iter()
        .map(|mode| read(&format!("{mode}.raw.json")))
        .collect::<Result<_, _>>()?;
    for (i, (mode, w)) in MODES.iter().zip(&workers).enumerate() {
        let method = if i < 3 { *mode } else { "rho" };
        if fs::read_to_string(dir.join(format!("{mode}.exit-code.txt")))?.trim() != "0"
            || w["schema"] != "cover.two-large-prime.dlp-worker/v1"
            || w["source_revision"] != plan["source_revision"]
            || w["public"] != public
            || w["seed"] != seed
            || w["method"] != method
            || w["solver_answer_placeholder"] != 0
            || w["actual_scalar"] != answer
            || w["verified"] != true
            || w["capped"] != false
        {
            return Err(format!("worker identity or failure: {mode}").into());
        }
    }
    let input_sha = driver::digest(&fs::read(dir.join("public.json"))?);
    let a = check::collection(&workers[1], &input_sha)?;
    let b = check::collection(&workers[2], &input_sha)?;
    let pair = collection_check::pair(&a, &b, seed, &input_sha)?;
    let mut outcomes = vec![
        json!({"mode":"ic","actual_scalar":answer,"accounting":check::full::counter_check(&workers[0],p,l)?}),
    ];
    for (i, c) in [&a, &b].into_iter().enumerate() {
        outcomes.push(check::pipeline(
            &workers[i + 1],
            c,
            &pair["outcomes"][i],
            answer,
            plan["max_descent_residuals"]
                .as_u64()
                .ok_or("descent cap")?,
            plan["max_accepted_descents"]
                .as_u64()
                .ok_or("accepted cap")? as usize,
        )?);
    }
    for (i, w) in workers.iter().enumerate().skip(3) {
        check::rho_identity(w, i == 4 || i == 6, if i < 5 { 1 } else { 16 }, p, l, seed)?;
        outcomes.push(json!({"mode":MODES[i],"actual_scalar":answer,"accounting":check::rho::account(w,true,p,l)?.0}));
    }
    let conservation = check::paired_pipeline(&workers[1], &workers[2])?;
    let flow: Vec<Value> = fs::read_to_string(dir.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    if flow.len() != 16 {
        return Err("fixture/seven-arm flow cardinality".into());
    }
    for (i, mode) in std::iter::once("fixture").chain(MODES).enumerate() {
        let kind = if i == 0 { "fixture" } else { "execution" };
        if flow[2 * i]["mode"] != mode
            || flow[2 * i + 1]["mode"] != mode
            || flow[2 * i]["record_type"] != format!("{kind}_start")
            || flow[2 * i + 1]["record_type"] != kind
            || flow[2 * i + 1]["exit_code"] != 0
            || flow[2 * i]["source_revision"] != plan["source_revision"]
            || flow[2 * i + 1]["source_revision"] != plan["source_revision"]
        {
            return Err("complete planned execution flow".into());
        }
    }
    let summary = json!({"schema":"cover.extension-pipeline.analysis/v1","source_revision":plan["source_revision"],"p":p,"l":l,"seed":seed,"actual_scalar":answer,"outcomes":outcomes,"collection_pair":pair,"paired_pipeline":conservation,"flow":{"planned":7,"attempted":7,"completed":7,"excluded":0,"automatic_retries":0},"complete_cost":false,"total_bitops":null,"cost_ratios":null,"comparative_eligibility":false,"rho_parity":false,"independent_signed_verdict":false});
    let export = json!({"schema":"experiment.export/v1","canonical_experiment_export_schema_available":false,"canonical_schema_conformance_claimed":false,"protocol":plan,"fixture":fixture,"workers":workers,"analysis":summary,"flow":flow});
    for (path, value) in [
        (Path::new(&args[1]).to_path_buf(), &summary),
        (dir.join("raw-export.json"), &export),
    ] {
        let mut out = BufWriter::new(OpenOptions::new().write(true).create_new(true).open(path)?);
        serde_json::to_writer_pretty(&mut out, value)?;
        out.flush()?;
    }
    println!("Independent full pipeline, every source/Cantor/witness/kernel/target/counter and rho checks passed.");
    Ok(())
}

#[cfg(test)]
mod protocol_tests {
    use super::*;
    #[test]
    fn frozen_protocol_and_runtime_allocation_are_bound() {
        let mut plan: Value = serde_json::from_str(include_str!(
            "../research/extension_ic_graph_20261115/extension-pipeline-plan.json"
        ))
        .unwrap();
        plan["source_revision"] = json!("a".repeat(40));
        plan["runpod_allocation"] = json!({"compute_type":"CPU","vcpuCount":4,"memoryInGb":8,"lifetime_seconds":7200,"costPerHr":0.12});
        assert!(registered_plan(&plan));
        for (field, value) in [
            ("worker_timeout_seconds", json!(601)),
            ("max_descent_residuals", json!(8193)),
            ("rounds", json!(2)),
            ("automatic_retries", json!(1)),
            ("source_revision", json!("bad")),
            ("small_percent", json!(85)),
        ] {
            let mut bad = plan.clone();
            bad[field] = value;
            assert!(!registered_plan(&bad));
        }
        let mut bad = plan;
        bad["runpod_allocation"]["costPerHr"] = json!(0.41);
        assert!(!registered_plan(&bad));
    }
}
