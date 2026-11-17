//! Independently copied Runpod qualifier for the recorded-state component.
#![allow(clippy::duplicate_mod)]
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/anchored_descent.rs"]
mod anchored;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/anchored_input.rs"]
mod anchored_input;
#[path = "../research/extension_ic_graph_20261115/anchored_check.rs"]
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
fn registered_plan(plan: &Value) -> bool {
    let expected: Value = serde_json::from_str(include_str!(
        "../research/extension_ic_graph_20261115/anchored-descent-plan.json"
    ))
    .expect("compiled fixed plan");
    let Some(mut fixed) = plan.as_object().cloned() else {
        return false;
    };
    for key in [
        "source_revision",
        "binary_sha256",
        "analyzer_sha256",
        "pinned_cpu",
        "runpod_allocation",
    ] {
        fixed.remove(key);
    }
    let hex = |key: &str, len: usize| {
        plan[key]
            .as_str()
            .is_some_and(|s| s.len() == len && s.bytes().all(|c| c.is_ascii_hexdigit()))
    };
    let allocation = &plan["runpod_allocation"];
    Value::Object(fixed) == expected
        && hex("source_revision", 40)
        && hex("binary_sha256", 64)
        && hex("analyzer_sha256", 64)
        && plan["pinned_cpu"]
            .as_str()
            .is_some_and(|s| s.parse::<u32>().is_ok())
        && allocation["compute_type"] == "CPU"
        && allocation["vcpuCount"] == 4
        && allocation["memoryInGb"] == 8
        && allocation["lifetime_seconds"] == 5400
        && allocation["costPerHr"]
            .as_f64()
            .is_some_and(|v| v > 0.0 && v <= 0.40)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: CHECKER RUN_DIR NEW_SUMMARY".into());
    }
    let dir = Path::new(&args[0]);
    let read = |n: &str| -> Result<Value, Box<dyn std::error::Error>> {
        Ok(serde_json::from_slice(&fs::read(dir.join(n))?)?)
    };
    let plan = read("runtime-plan.json")?;
    if !registered_plan(&plan) {
        return Err("frozen plan registration".into());
    }
    let source = plan["source_revision"].as_str().ok_or("source revision")?;
    let bytes = fs::read(dir.join("input.json"))?;
    let input = anchored_input::parse(&bytes)?;
    if plan["input_sha256"] != driver::digest(&bytes) {
        return Err("frozen input hash".into());
    }
    let homogeneous = anchored_input::check_homogeneous(&input)?;
    let mut raws = Vec::new();
    let mut proofs = Vec::new();
    for mode in ["original-nagao", "full-only", "anchored"] {
        if fs::read_to_string(dir.join(format!("{mode}.exit-code.txt")))?.trim() != "0" {
            return Err(format!("worker failed: {mode}").into());
        }
        let raw = read(&format!("{mode}.raw.json"))?;
        if raw["mode"] != mode || raw["input_validation"] != homogeneous {
            return Err("worker mode/input admission".into());
        }
        proofs.push(check::qualify(&input, &bytes, &raw, source)?);
        raws.push(raw);
    }
    let paired = check::paired(&raws[1], &raws[2], &raws[0])?;
    let summary = json!({"schema":"cover.anchored-descent.qualified/v1","source_revision":source,"input_sha256":driver::digest(&bytes),"homogeneous_input_replay":homogeneous,"workers":proofs,"paired":paired,"rounds":1,"warmup_rounds":0,"retries":0,"excluded_runs":0,"signed_pcc_admission":false,"complete_cost":false,"complete_total":null,"ratios":null,"collection_cost_included":false,"fresh_dlp_instance":false});
    let mut out = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args[1])?,
    );
    serde_json::to_writer_pretty(&mut out, &summary)?;
    out.write_all(b"\n")?;
    out.flush()?;
    println!("all three recorded-state arms replayed; incomplete costs retained");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Value {
        let mut p: Value = serde_json::from_str(include_str!(
            "../research/extension_ic_graph_20261115/anchored-descent-plan.json"
        ))
        .unwrap();
        p["source_revision"] = json!("1".repeat(40));
        p["binary_sha256"] = json!("2".repeat(64));
        p["analyzer_sha256"] = json!("3".repeat(64));
        p["pinned_cpu"] = json!("0");
        p["runpod_allocation"] = json!({"compute_type":"CPU","vcpuCount":4,"memoryInGb":8,"lifetime_seconds":5400,"costPerHr":0.12});
        p
    }
    #[test]
    fn compiled_protocol_binds_every_fixed_field_and_resource_bounds() {
        let p = fixture();
        assert!(registered_plan(&p));
        for (key, value) in [
            ("seed", json!(402)),
            ("rounds", json!(2)),
            ("max_residuals", json!(8193)),
            ("input_sha256", json!("0".repeat(64))),
            ("collection_cost_included", json!(true)),
            ("timeout_seconds", json!(9999)),
        ] {
            let mut bad = p.clone();
            bad[key] = value;
            assert!(!registered_plan(&bad), "{key}");
        }
        let mut bad = p.clone();
        bad["runpod_allocation"]["costPerHr"] = json!(0.41);
        assert!(!registered_plan(&bad));
        let mut bad = p.clone();
        bad["unregistered_retry"] = json!(true);
        assert!(!registered_plan(&bad));
    }
}
