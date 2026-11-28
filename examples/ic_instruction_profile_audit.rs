//! Native validation of frozen child outputs and bounded Callgrind intervals.
use crypto_lib::cryptanalysis::ecbench::callgrind;
use crypto_lib::cryptanalysis::ic_boundary::{
    koblitz_instance, BinaryGroup, CountedGroup, GroupOps,
};
use crypto_lib::cryptanalysis::koblitz_fast::FastPoint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;

fn read(p: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap()
}
fn coord(v: &Value) -> u64 {
    u64::from_str_radix(v.as_str().unwrap().trim_start_matches("0x"), 16).unwrap()
}
fn same_price(a: &Value, b: &Value) -> bool {
    match (a.as_f64(), b.as_f64()) {
        (Some(a), Some(b)) => (a - b).abs() <= 1e-12 * (1.0 + a.abs().max(b.abs())),
        _ => a == b,
    }
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 2, "STUDY PROFILE_DIR");
    let study = Path::new(&args[0]);
    let dir = Path::new(&args[1]);
    let raw = std::fs::read_to_string(
        study.join("pipeline-runs/structured-n13-20261025-01/records.jsonl"),
    )
    .unwrap();
    let records: Vec<Value> = raw
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(records.len(), 10);
    let inst = koblitz_instance(0, 13).unwrap();
    let group = BinaryGroup(&inst.fast);
    let mut rows = vec![];
    for r in records {
        let seq = r["seq"].as_u64().unwrap();
        let p = dir.join(format!("seq-{seq}"));
        let status = read(&p.join("status.json"));
        if status["exit_code"] != 0 {
            rows.push(json!({"seq":seq,"arm":r["arm"],"status":"profile_failed_or_censored","process":status}));
            continue;
        }
        let child_bytes = std::fs::read(p.join("child.json")).unwrap();
        let child: Value = serde_json::from_slice(&child_bytes).unwrap();
        assert!(child["error"].is_null());
        assert_eq!(child["workload_id"], r["workload"]["workload_id"]);
        assert_eq!(child["method_id"], r["method"]["method_id"]);
        let report = &child["report"];
        let k = report["recovered"].as_u64().unwrap();
        assert_eq!(k.to_string(), r["outcome"]["recovered"].as_str().unwrap());
        assert_eq!(r["workload"]["curve"]["r"], inst.r);
        assert_eq!(r["workload"]["curve"]["group_order"], inst.group_order);
        assert_eq!(r["workload"]["curve"]["field_degree"], 13);
        let generator = &r["workload"]["curve"]["generator"];
        let g = FastPoint {
            x: coord(&generator[0]),
            y: coord(&generator[1]),
            infinity: false,
        };
        let recovered = group.mul(&mut GroupOps::default(), g, k);
        let target = &r["workload"]["target"];
        assert_eq!(
            recovered,
            FastPoint {
                x: coord(&target[0]),
                y: coord(&target[1]),
                infinity: false
            }
        );
        assert_eq!(report["counters"], r["counters"]);
        assert_eq!(report["factor_base"], r["factor_base"]);
        assert_eq!(report["unpriced"], r["cost"]["unpriced"]);
        assert_eq!(report["deterministic"], r["cost"]["deterministic"]);
        assert!(same_price(&report["total_gae"], &r["cost"]["total_gae"]));
        assert_eq!(report["solver"]["ops"], r["solver"]["ops"]);
        let phases = report["phases"].as_array().unwrap();
        let expected = r["phases"].as_array().unwrap();
        assert_eq!(phases.len(), expected.len());
        for (a, b) in phases.iter().zip(expected) {
            for field in ["name", "adds", "doubles", "scalar_mults", "native"] {
                assert_eq!(a[field], b[field], "profile seq {seq}, phase field {field}");
            }
            assert!(same_price(&a["gae"], &b["gae"]));
        }
        let parser = read(&p.join("parser-status.json"));
        let solve = if parser["solve_exit_code"] == 0 {
            let recorded = read(&p.join("solve-ir.json"));
            let rebuilt =
                serde_json::to_value(callgrind::solve_ir(&p.join("callgrind.out")).unwrap())
                    .unwrap();
            for key in ["solve_ir", "prework_ir", "solve_parts"] {
                assert_eq!(rebuilt[key], recorded[key]);
            }
            let parts = |v: &Value| {
                v["parts"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|p| (p["trigger"].clone(), p["ir"].clone()))
                    .collect::<Vec<_>>()
            };
            assert_eq!(parts(&rebuilt), parts(&recorded));
            rebuilt
        } else {
            Value::Null
        };
        let online = if parser["online_exit_code"] == 0 {
            let recorded = read(&p.join("online-ir.json"));
            let rebuilt =
                serde_json::to_value(callgrind::online_ir(&p.join("callgrind.out")).unwrap())
                    .unwrap();
            for key in ["solve_ir", "pre_online_ir", "online_ir", "post_online_ir"] {
                assert_eq!(rebuilt[key], recorded[key]);
            }
            rebuilt
        } else {
            Value::Null
        };
        let mut attribution = BTreeMap::<String, u64>::new();
        if !solve.is_null() {
            let mut inside = false;
            for part in solve["parts"].as_array().unwrap() {
                let marker = part["trigger"].as_str().unwrap();
                if marker == "ecbench_before_solve" {
                    inside = true;
                    continue;
                }
                if inside {
                    let n = attribution.entry(marker.to_string()).or_default();
                    *n = n.checked_add(part["ir"].as_u64().unwrap()).unwrap();
                    if marker == "ecbench_solve" {
                        inside = false;
                        break;
                    }
                }
            }
            assert!(!inside);
            assert_eq!(
                attribution.values().sum::<u64>(),
                solve["solve_ir"].as_u64().unwrap()
            );
        }
        if !online.is_null() {
            assert_eq!(online["solve_ir"], solve["solve_ir"]);
            let sum = ["pre_online_ir", "online_ir", "post_online_ir"]
                .into_iter()
                .map(|key| online[key].as_u64().unwrap())
                .sum::<u64>();
            assert_eq!(sum, solve["solve_ir"].as_u64().unwrap());
        }
        rows.push(json!({"seq":seq,"arm":r["arm"],"workload_id":child["workload_id"],
            "status":"scalar_and_integer_native_counts_equal","child_sha256":format!("{:x}",Sha256::digest(&child_bytes)),
            "phase_gae_exact":phases.iter().zip(expected).all(|(a,b)|a["gae"]==b["gae"]),
            "solve_ir":solve["solve_ir"],"solve_parts":solve["solve_parts"],
            "phase_ending_marker_ir":attribution,
            "online_ir":online["online_ir"],"pre_online_ir":online["pre_online_ir"],"post_online_ir":online["post_online_ir"],
            "online_parser_exit":parser["online_exit_code"],"solver_words":report["solver"]["ops"]}));
    }
    let rho: BTreeMap<_, _> = rows
        .iter()
        .filter(|r| r["arm"] == "rho-strong")
        .map(|r| {
            (
                r["workload_id"].as_str().unwrap().to_string(),
                r["solve_ir"].as_u64(),
            )
        })
        .collect();
    for row in &mut rows {
        let reference = row["workload_id"]
            .as_str()
            .and_then(|k| rho.get(k).copied().flatten());
        row["solve_ir_over_same_target_rho"] = match (row["solve_ir"].as_u64(), reference) {
            (Some(a), Some(b)) => json!(a as f64 / b as f64),
            _ => Value::Null,
        };
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"ic-instruction-profile-audit/v1",
        "status":"native_outputs_checked","rows":rows,"unit":"callgrind.Ir under Valgrind allocator",
        "price_relative_tolerance":1e-12,
        "scope":"Implementation instruction attribution only; native allocator, physical throughput and complete native-price accounting remain unmeasured."})).unwrap());
}
