//! New registration/pair wrapper around unchanged trial46 mathematical checkers.
#![allow(clippy::duplicate_mod)]
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_pipeline_check.rs"]
mod check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_collection_check.rs"]
mod collection_check;
#[allow(dead_code, clippy::type_complexity)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder_driver.rs"]
mod driver;
#[allow(dead_code, clippy::too_many_arguments)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder.rs"]
mod remainder;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_math_check.rs"]
mod sparse_math_check;
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const ROLES: [&str; 2] = ["base-horner", "forward-differences"];
fn geometry(w: &Value) -> Value {
    let mut g = json!({});
    let c = &w["work"]["collection"];
    for k in [
        "base",
        "h",
        "sources",
        "projections",
        "checkpoints",
        "counts",
        "rank_target",
        "stop",
    ] {
        g[k] = c[k].clone();
    }
    g["rank_checks"] = Value::Array(
        c["rank_checks"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|r| json!({"b_count":r["b_count"],"rows":r["rows"],"result":r["result"]}))
            .collect(),
    );
    for k in [
        "initial_basis",
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
        g[k] = w["work"][k].clone();
    }
    g
}
fn plan_ok(p: &Value) -> bool {
    p["schema"] == "cover.table-roots.runpod-validation/v1"
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
        && p["mode_order"] == json!(ROLES)
        && p["rounds"] == 1
        && p["warmup_rounds"] == 0
        && p["automatic_retries"] == 0
        && p["fixture_timeout_seconds"] == 60
        && p["worker_timeout_seconds"] == 600
        && p["timeout_seconds"] == 1500
        && p["qualifier_timeout_seconds"] == 900
        && p["runpod_lifetime_seconds"] == 4500
        && p["memory_gib"] == 8
        && p["complete_total"].is_null()
        && p["rho_parity"] == false
        && p["allow_comparative_claims"] == false
        && p["public_sha256"] == "b3ae4f93974456bdd6a772caf7daa0208d64c17f7341f70ee4187d327fd6462e"
        && p["baseline_binding_sha256"]
            == "f69ab075011fc9dbb891cf2de1fbaba22b6b3e08fbf34df0d2a20f9f2b093167"
        && p["sparse_rank_options"]
            == json!({"surplus":8,"failed_checkpoint_stride":16,"max_checkpoints":8,"max_preconditioners":3})
        && p["source_revision"]
            .as_str()
            .is_some_and(|s| s.len() == 40 && s.bytes().all(|c| c.is_ascii_hexdigit()))
        && p["runpod_allocation"]["compute_type"] == "CPU"
        && p["runpod_allocation"]["vcpuCount"] == 4
        && p["runpod_allocation"]["memoryInGb"] == 8
        && p["runpod_allocation"]["lifetime_seconds"] == 4500
        && p["runpod_allocation"]["costPerHr"]
            .as_f64()
            .is_some_and(|x| x > 0.0 && x <= 0.40)
}
fn pair(a: &Value, b: &Value) -> Result<()> {
    if a["work"]["collection"]["root_finder"] != Value::Null
        || b["work"]["collection"]["root_finder"] != "forward-differences"
        || geometry(a) != geometry(b)
    {
        return Err("root policy or exact paired geometry".into());
    }
    Ok(())
}
fn write_new(path: &Path, v: &Value) -> Result<()> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    serde_json::to_writer_pretty(&mut f, v)?;
    f.write_all(b"\n")?;
    Ok(())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() == 3 && a[0] == "--control-replay" {
        let dir = Path::new(&a[1]);
        let bytes = fs::read(dir.join("extension-sparse-remainders.raw.json"))?;
        let w: Value = serde_json::from_slice(&bytes)?;
        if w["source_revision"] != "ed5ca5ab8a4f41f89165a40e7efefc21aa76a399" {
            return Err("recorded46source".into());
        }
        let public_sha = driver::digest(&fs::read(dir.join("public.json"))?);
        let answer = w["actual_scalar"].as_u64().ok_or("answer")?;
        let c = check::collection(&w, &public_sha)?;
        let cp = collection_check::qualify_sparse(&c, 389, &public_sha, &w["work"]["collection"])?;
        let proof = check::pipeline(&w, &c, &cp, answer, 8192, 32)?;
        let mut labelled = w.clone();
        labelled["work"]["collection"]["root_finder"] = json!("forward-differences");
        pair(&w, &labelled)?;
        for path in [
            "/work/collection/counts/refused",
            "/work/collection/sources/0/s",
            "/work/collection/rank_checks/0/result/rank_certificate",
            "/work/descent_residuals",
        ] {
            let mut bad = labelled.clone();
            *bad.pointer_mut(path).ok_or("control pointer")? = json!(999);
            if pair(&w, &bad).is_ok() {
                return Err(format!("paired corruption accepted {path}").into());
            }
        }
        return write_new(
            Path::new(&a[2]),
            &json!({"schema":"cover.table-roots.recorded-control/v1","recorded_worker_sha256":driver::digest(&bytes),"recorded46_pipeline_proof":proof,"geometry_sha256":driver::digest(&serde_json::to_vec(&geometry(&w))?),"new_pair_corruption_controls":4,"scientific_worker_invoked":false,"candidate_label_control_is_simulation":true}),
        );
    }
    if a.len() == 3 && a[0] == "--bind-baseline" {
        let bytes = fs::read(&a[1])?;
        let w: Value = serde_json::from_slice(&bytes)?;
        if w["source_revision"] != "ed5ca5ab8a4f41f89165a40e7efefc21aa76a399"
            || w["method"] != "extension-sparse-remainders"
            || w["verified"] != true
            || w["capped"] != false
        {
            return Err("original46baseline".into());
        }
        return write_new(
            Path::new(&a[2]),
            &json!({"schema":"cover.table-roots.baseline-binding/v1","original_file":a[1],"original_sha256":driver::digest(&bytes),"source_revision":w["source_revision"],"geometry_sha256":driver::digest(&serde_json::to_vec(&geometry(&w))?),"observed_primitives":w["work_native"]["observed_primitives"],"scientific_worker_invoked":false}),
        );
    }
    if a.len() != 2 {
        return Err("usage: CHECK RUN NEW_SUMMARY | --bind-baseline ORIGINAL46 NEW_BINDING".into());
    }
    let dir = Path::new(&a[0]);
    let read =
        |name: &str| -> Result<Value> { Ok(serde_json::from_slice(&fs::read(dir.join(name))?)?) };
    let p = read("runtime-plan.json")?;
    let public = read("public.json")?;
    let fixture = read("fixture.json")?;
    let binding = read("baseline-binding.json")?;
    let source = p["source_revision"].as_str().ok_or("source")?;
    let answer = fixture["expected_scalar"].as_u64().ok_or("answer")?;
    let public_sha = driver::digest(&fs::read(dir.join("public.json"))?);
    if !plan_ok(&p)
        || fixture["public"] != public
        || fixture["seed"] != 389
        || fixture["source_revision"] != source
        || fixture["workers_receive"] != "public only"
        || public["p"] != 1009
        || public["l"] != 263807419614139553u64
        || answer == 0
        || answer >= 263807419614139553
        || p["public_sha256"] != public_sha
        || p["baseline_binding_sha256"]
            != driver::digest(&fs::read(dir.join("baseline-binding.json"))?)
        || binding["source_revision"] != "ed5ca5ab8a4f41f89165a40e7efefc21aa76a399"
    {
        return Err("registered plan/public/fixture/baseline binding".into());
    }
    let mut workers = Vec::new();
    let mut outcomes = Vec::new();
    for role in ROLES {
        if fs::read_to_string(dir.join(format!("{role}.exit-code.txt")))?.trim() != "0" {
            return Err(format!("failed worker {role}").into());
        }
        let w = read(&format!("{role}.raw.json"))?;
        if w["schema"] != "cover.two-large-prime.dlp-worker/v1"
            || w["source_revision"] != source
            || w["method"] != "extension-sparse-remainders"
            || w["seed"] != 389
            || w["public"] != public
            || w["solver_answer_placeholder"] != 0
            || w["actual_scalar"] != answer
            || w["verified"] != true
            || w["capped"] != false
        {
            return Err("complete worker identity".into());
        }
        let c = check::collection(&w, &public_sha)?;
        let cp = collection_check::qualify_sparse(&c, 389, &public_sha, &w["work"]["collection"])?;
        let proof = check::pipeline(&w, &c, &cp, answer, 8192, 32)?;
        outcomes.push(json!({"mode":role,"proof":proof,"inner_wall_ns":w["inner_wall_ns"],"observed_primitives":w["work_native"]["observed_primitives"],"b_count":c.counts.b_count,"sources":c.sources.len(),"projections":c.projections.len(),"descent_residuals":w["work"]["descent_residuals"]}));
        workers.push(w);
    }
    pair(&workers[0], &workers[1])?;
    if binding["geometry_sha256"] != driver::digest(&serde_json::to_vec(&geometry(&workers[0]))?)
        || binding["observed_primitives"] != workers[0]["work_native"]["observed_primitives"]
    {
        return Err("unchanged default46geometry/primitive regression".into());
    }
    let flow: Vec<Value> = fs::read_to_string(dir.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    if flow.len() != 6 {
        return Err("fixture/two-arm flow".into());
    }
    for (i, role) in std::iter::once("fixture").chain(ROLES).enumerate() {
        let kind = if i == 0 { "fixture" } else { "execution" };
        if flow[2 * i]["mode"] != role
            || flow[2 * i + 1]["mode"] != role
            || flow[2 * i]["record_type"] != format!("{kind}_start")
            || flow[2 * i + 1]["record_type"] != kind
            || flow[2 * i + 1]["exit_code"] != 0
            || flow[2 * i]["source_revision"] != source
            || flow[2 * i + 1]["source_revision"] != source
        {
            return Err("exact complete flow".into());
        }
    }
    write_new(
        Path::new(&a[1]),
        &json!({"schema":"cover.table-roots.qualified/v1","source_revision":source,"public":public,"seed":389,"actual_scalar":answer,"outcomes":outcomes,"paired_geometry_equal":true,"default46geometry_and_primitives_equal":true,"planned":2,"completed":2,"excluded":0,"automatic_retries":0,"complete_cost":false,"complete_total":null,"ratios":null,"rho_parity":false,"independent_signed_verdict":false}),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_registration_refuses_changed_caps_or_attempts() {
        let mut p: Value = serde_json::from_str(include_str!(
            "../research/extension_ic_graph_20261115/table-roots-plan.json"
        ))
        .unwrap();
        p["source_revision"] = json!("a".repeat(40));
        p["runpod_allocation"] = json!({"compute_type":"CPU","vcpuCount":4,"memoryInGb":8,"lifetime_seconds":4500,"costPerHr":0.12});
        assert!(plan_ok(&p));
        for path in [
            "/seed",
            "/rounds",
            "/automatic_retries",
            "/source_cap",
            "/worker_timeout_seconds",
            "/runpod_lifetime_seconds",
            "/sparse_rank_options/surplus",
            "/baseline_binding_sha256",
            "/runpod_allocation/costPerHr",
        ] {
            let mut bad = p.clone();
            *bad.pointer_mut(path).unwrap() = json!(999);
            assert!(!plan_ok(&bad), "{path}");
        }
    }
    #[test]
    fn pair_refuses_changed_policy_or_retained_failure_geometry() {
        let a = json!({"work":{"collection":{},"actual_scalar":1}});
        let mut b = a.clone();
        b["work"]["collection"]["root_finder"] = json!("forward-differences");
        assert!(pair(&a, &b).is_ok());
        b["work"]["actual_scalar"] = json!(2);
        assert!(pair(&a, &b).is_err());
        b["work"]["actual_scalar"] = json!(1);
        b["work"]["collection"]["root_finder"] = json!("unchecked");
        assert!(pair(&a, &b).is_err());
    }
}
