//! Independent source/Cantor/rank/Krylov/full-affine/elliptic replay for trial46.
#![allow(clippy::duplicate_mod)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_pipeline_check.rs"]
mod check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_collection_check.rs"]
mod collection_check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder_driver.rs"]
#[allow(clippy::type_complexity)] // Unchanged baseline graph key.
mod driver;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder.rs"]
#[allow(clippy::too_many_arguments)] // Existing independent equation verifier.
mod remainder;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_math_check.rs"]
mod sparse_math_check;
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path};
const MODES: [&str; 9] = [
    "ic",
    "extension-full-smooth",
    "extension-remainders",
    "extension-sparse-full",
    "extension-sparse-remainders",
    "rho-plain-1",
    "rho-negation-1",
    "rho-plain-16",
    "rho-negation-16",
];
fn registered_plan(p: &Value) -> bool {
    p["schema"] == "cover.extension-sparse.runpod-validation/v1"
        && p["backend"] == "runpod_user_authorized"
        && p["p"] == 1009
        && p["seed"] == 389
        && p["m"] == 9
        && p["small_percent"] == 100
        && p["b_count"] == 1018081
        && p["source_cap"] == 65536
        && p["collection_batch_size"] == 1024
        && p["max_descent_residuals"] == 8192
        && p["max_accepted_descents"] == 32
        && p["mode_order"] == json!(MODES)
        && p["rounds"] == 1
        && p["warmup_rounds"] == 0
        && p["automatic_retries"] == 0
        && p["fixture_timeout_seconds"] == 60
        && p["worker_timeout_seconds"] == 600
        && p["timeout_seconds"] == 4500
        && p["qualifier_timeout_seconds"] == 900
        && p["runpod_lifetime_seconds"] == 7200
        && p["memory_gib"] == 8
        && p["sparse_rank_options"]
            == json!({"surplus":8,"failed_checkpoint_stride":16,"max_checkpoints":8,"max_preconditioners":3})
        && p["kernel_seed_xor"] == "0x5A464B45524E454C"
        && p["allow_comparative_claims"] == false
        && p["complete_total"].is_null()
        && p["rho_parity"] == false
        && p["source_revision"]
            .as_str()
            .is_some_and(|s| s.len() == 40 && s.bytes().all(|b| b.is_ascii_hexdigit()))
        && p["runpod_allocation"]["compute_type"] == "CPU"
        && p["runpod_allocation"]["vcpuCount"] == 4
        && p["runpod_allocation"]["memoryInGb"] == 8
        && p["runpod_allocation"]["lifetime_seconds"] == 7200
        && p["runpod_allocation"]["costPerHr"]
            .as_f64()
            .is_some_and(|x| x > 0.0 && x <= 0.40)
}
fn control_replay(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let old = Path::new(&args[1]);
    let kernel = Path::new(&args[2]);
    let public: Value = serde_json::from_slice(&fs::read(old.join("public.json"))?)?;
    let fixture: Value = serde_json::from_slice(&fs::read(old.join("fixture.json"))?)?;
    let input_sha = driver::digest(&fs::read(old.join("public.json"))?);
    let answer = fixture["expected_scalar"].as_u64().ok_or("answer")?;
    let mut proofs = Vec::new();
    for mode in ["extension-full-smooth", "extension-remainders"] {
        let w: Value = serde_json::from_slice(&fs::read(old.join(format!("{mode}.raw.json")))?)?;
        if w["public"] != public {
            return Err("control public binding".into());
        }
        let c = check::collection(&w, &input_sha)?;
        let q = collection_check::qualify(&c, 389, &input_sha)?;
        proofs.push(check::pipeline(&w, &c, &q, answer, 8192, 32)?);
    }
    let input: Value = serde_json::from_slice(&fs::read(kernel.join("input.json"))?)?;
    let scalar: Value =
        serde_json::from_slice(&fs::read(kernel.join("randomized-lanczos.raw.json"))?)?;
    for i in 0..2 {
        let m = &input["corpora"][i]["matrix"];
        let r = &scalar["corpora"][i];
        let cp = json!({"rows":m["rows"].as_array().ok_or("rows")?.len(),"result":r["result"],"native":r});
        let q = sparse_math_check::qualify_checkpoint(m, &cp, 508)?;
        if q["verified_kernel"] != true {
            return Err("control kernel".into());
        }
        for pointer in [
            "/native/foreign_thread_hooks",
            "/result/attempts/0/steps/0/diagonal",
            "/native/primitives/modr_inv",
        ] {
            let mut bad = cp.clone();
            *bad.pointer_mut(pointer).ok_or("control pointer")? = json!(999);
            if sparse_math_check::qualify_checkpoint(m, &bad, 508).is_ok() {
                return Err(format!("corruption accepted {pointer}").into());
            }
        }
        if sparse_math_check::qualify_checkpoint(m, &cp, 507).is_ok() {
            return Err("substituted rank accepted".into());
        }
    }
    let mut out = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[3])?;
    serde_json::to_writer_pretty(
        &mut out,
        &json!({"schema":"cover.extension-sparse.recorded-control/v1","source_fixture_sha256":driver::digest(&fs::read(old.join("fixture.json"))?),"baseline_pipeline_proofs":proofs,"recorded_kernel_checks":2,"corruption_controls":8,"scientific_measurement":false}),
    )?;
    out.write_all(b"\n")?;
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.first().map(String::as_str) == Some("--control-replay") && a.len() == 4 {
        return control_replay(&a);
    }
    if a.len() != 2 {
        return Err("usage: CHECK RUN NEW_SUMMARY".into());
    }
    let dir = Path::new(&a[0]);
    let read = |name: &str| -> Result<Value, Box<dyn std::error::Error>> {
        Ok(serde_json::from_slice(&fs::read(dir.join(name))?)?)
    };
    let plan = read("runtime-plan.json")?;
    let fixture = read("fixture.json")?;
    let public = read("public.json")?;
    let source = plan["source_revision"].as_str().ok_or("source")?;
    let answer = fixture["expected_scalar"].as_u64().ok_or("answer")?;
    let l = public["l"].as_u64().ok_or("l")?;
    if !registered_plan(&plan)
        || plan["seed"] != 389
        || public["p"] != 1009
        || l != 263807419614139553
        || plan["mode_order"] != json!(MODES)
        || plan["rounds"] != 1
        || plan["warmup_rounds"] != 0
        || plan["automatic_retries"] != 0
        || plan["complete_total"] != Value::Null
        || plan["rho_parity"] != false
        || fixture["public"] != public
        || fixture["seed"] != 389
        || fixture["source_revision"] != source
        || fixture["workers_receive"] != "public only"
        || source.len() != 40
        || answer == 0
        || answer >= l
    {
        return Err("registration/fixture".into());
    }
    let input_sha = driver::digest(&fs::read(dir.join("public.json"))?);
    if plan["public_sha256"] != input_sha {
        return Err("registered public input bytes".into());
    }
    let mut workers = Vec::new();
    let mut proofs = Vec::new();
    let mut records = Vec::new();
    for (i, mode) in MODES.iter().enumerate() {
        if fs::read_to_string(dir.join(format!("{mode}.exit-code.txt")))?.trim() != "0" {
            return Err(format!("failed worker {mode}").into());
        }
        let w = read(&format!("{mode}.raw.json"))?;
        if w["schema"] != "cover.two-large-prime.dlp-worker/v1"
            || w["source_revision"] != source
            || w["public"] != public
            || w["seed"] != 389
            || w["solver_answer_placeholder"] != 0
            || w["actual_scalar"] != answer
            || w["verified"] != true
            || w["capped"] != false
            || w["method"] != if i < 5 { *mode } else { "rho" }
        {
            return Err(format!("worker identity {mode}").into());
        }
        if i == 0 {
            proofs.push(json!({"mode":mode,"actual_scalar":answer,"accounting":check::full::counter_check(&w,1009,l)?}));
        } else if i < 5 {
            let c = check::collection(&w, &input_sha)?;
            let cp = if i < 3 {
                collection_check::qualify(&c, 389, &input_sha)?
            } else {
                collection_check::qualify_sparse(&c, 389, &input_sha, &w["work"]["collection"])?
            };
            proofs.push(check::pipeline(&w, &c, &cp, answer, 8192, 32)?);
            records.push(c);
        } else {
            check::rho_identity(
                &w,
                i == 6 || i == 8,
                if i < 7 { 1 } else { 16 },
                1009,
                l,
                389,
            )?;
            proofs.push(json!({"mode":mode,"actual_scalar":answer,"accounting":check::rho::account(&w,true,1009,l)?.0}));
        }
        workers.push(w);
    }
    let baseline_pair = check::paired_pipeline(&workers[1], &workers[2])?;
    let mut paired = Vec::new();
    for (old, new) in [(1usize, 3usize), (2, 4)] {
        let x = &workers[old];
        let y = &workers[new];
        for key in [
            "descents",
            "affine_rows",
            "projected_rows",
            "descent_residuals",
            "descent_stream_advances",
            "descent_timeouts",
            "descent_incomplete",
            "invalid_descents",
            "last_rank",
            "actual_scalar",
        ] {
            if x["work"][key] != y["work"][key] {
                return Err(format!("paired descent endpoint {key}").into());
            }
        }
        if x["work"]["initial_basis"]["kernel"] != y["work"]["initial_basis"]["kernel"] {
            return Err("paired normalized kernel".into());
        }
        let a = &records[old - 1];
        let b = &records[new - 1];
        let mut common = 0;
        for (u, v) in a.checkpoints.iter().zip(&b.checkpoints) {
            if u.b_count != v.b_count || u.lines != v.lines || u.cohort_sha256 != v.cohort_sha256 {
                return Err("paired cohort fingerprint".into());
            }
            common += 1;
        }
        for (u, v) in a.sources.iter().zip(&b.sources) {
            if serde_json::to_value(u)? != serde_json::to_value(v)? {
                return Err("paired exact source prefix".into());
            }
        }
        for (u, v) in a.projections.iter().zip(&b.projections) {
            if u.terms != v.terms || u.witness != v.witness {
                return Err("paired projection witness prefix".into());
            }
        }
        paired.push(json!({"baseline":MODES[old],"candidate":MODES[new],"common_batch_fingerprints":common,"all_common_sources_and_witnesses_equal":true,"kernel_and_all_affine_endpoints_equal":true}));
    }
    let flow: Vec<Value> = fs::read_to_string(dir.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    if flow.len() != 20 {
        return Err("fixture/nine-arm flow cardinality".into());
    }
    for (i, mode) in std::iter::once("fixture").chain(MODES).enumerate() {
        let kind = if i == 0 { "fixture" } else { "execution" };
        if flow[2 * i]["mode"] != mode
            || flow[2 * i + 1]["mode"] != mode
            || flow[2 * i]["record_type"] != format!("{kind}_start")
            || flow[2 * i + 1]["record_type"] != kind
            || flow[2 * i + 1]["exit_code"] != 0
            || flow[2 * i]["source_revision"] != source
            || flow[2 * i + 1]["source_revision"] != source
        {
            return Err("complete planned execution flow".into());
        }
    }
    let rows:Vec<_>=MODES.iter().zip(&workers).zip(proofs).map(|((mode,w),proof)|json!({"mode":mode,"proof":proof,"inner_wall_ns":w["inner_wall_ns"],"observed_primitives":w["work_native"]["observed_primitives"],"b_count":w["work"]["collection"]["counts"]["b_count"],"sources":w["work"]["collection"]["sources"].as_array().map(Vec::len),"projections":w["work"]["collection"]["projections"].as_array().map(Vec::len),"descent_residuals":w["work"]["descent_residuals"]})).collect();
    let summary = json!({"schema":"cover.extension-sparse.qualified/v1","source_revision":source,"public":public,"seed":389,"actual_scalar":answer,"outcomes":rows,"paired":paired,"baseline_pair":baseline_pair,"planned":9,"completed":9,"excluded":0,"automatic_retries":0,"complete_cost":false,"complete_total":null,"ratios":null,"rho_parity":false,"independent_signed_verdict":false});
    let mut out = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[1])?;
    serde_json::to_writer_pretty(&mut out, &summary)?;
    out.write_all(b"\n")?;
    Ok(())
}

#[cfg(test)]
mod registration_controls {
    use super::*;
    #[test]
    fn fixed_source_allocation_and_attempt_plan_are_enforced() {
        let mut p: Value = serde_json::from_str(include_str!(
            "../research/extension_ic_graph_20261115/extension-sparse-plan.json"
        ))
        .unwrap();
        p["source_revision"] = json!("a".repeat(40));
        p["runpod_allocation"] = json!({"compute_type":"CPU","vcpuCount":4,"memoryInGb":8,"lifetime_seconds":7200,"costPerHr":0.12});
        assert!(registered_plan(&p));
        for path in [
            "/seed",
            "/rounds",
            "/automatic_retries",
            "/max_descent_residuals",
            "/worker_timeout_seconds",
            "/sparse_rank_options/surplus",
            "/sparse_rank_options/max_preconditioners",
            "/runpod_allocation/lifetime_seconds",
            "/runpod_allocation/costPerHr",
        ] {
            let mut bad = p.clone();
            *bad.pointer_mut(path).unwrap() = json!(999);
            assert!(!registered_plan(&bad), "{path}");
        }
        let mut bad = p;
        bad["mode_order"] = json!(["ic"]);
        assert!(!registered_plan(&bad));
    }
}
