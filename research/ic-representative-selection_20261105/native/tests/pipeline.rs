//! End to end on the committed v2 fixture class: transport, job specs,
//! synthetic records with a planted 5% B effect, and the frozen analysis.

use std::fs;
use std::path::PathBuf;

use repsel::{analyse, ascend, specs};
use serde_json::{json, Value};

fn tmp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("repsel-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(&d).unwrap();
    d
}

fn record(session: &str, w: &Value, arm: &str, target: u64, round: u64, total: f64) -> String {
    json!({"record_id": "x", "session_id": session, "arm": arm, "round": round, "warmup": round == 0,
           "workload": {"target_index": target, "planted": 1000 + target,
                        "curve_spec": {"p": w["p"], "a": w["a"], "b": w["b"], "gx": w["gx"], "gy": w["gy"], "r": w["r"]}},
           "outcome": {"status": "verified", "matches_target": true, "matches_planted": true, "recovered": (1000 + target).to_string()},
           "cost": {"total_gae": total, "unpriced": ["lookups_uncharged"]},
           "counters": {"lookups_uncharged": 500, "relations_found": 33.0, "targets_tried": if arm == "ic-mitm-plus1" { 2800.0 } else { 3000.0 }},
           "phases": [], "detail": {"calibration_pinned_units": []}}).to_string()
}

#[test]
fn fixture_class_runs_through_specs_and_analysis() {
    let class = tmp("class");
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata/class-l2-s1001");
    for f in ["curves.jsonl", "spec.json"] {
        fs::copy(src.join(f), class.join(f)).unwrap();
    }
    let rows = ascend::write_transport(&class).unwrap();
    assert_eq!(rows.len(), 12);
    let design: Value = serde_json::from_str(
        &fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../design.json"))
            .unwrap(),
    )
    .unwrap();
    let mut design = design;
    design["tiers"]["24"]["group_min"] = json!(2);
    let out = tmp("specs");
    let jobs = specs::pre_selection(&class, &design, "24", "t24-fx", false, &out).unwrap();
    assert!(jobs.iter().any(|j| j.contains("\tb\t")));
    // Synthetic sessions: IC 77 S with ±10% noise, B planted at 0.95; rho and bsgs depend on the target only.
    let mut recs = String::new();
    let mut labels = Vec::new();
    let mut n = 0u64;
    for e in fs::read_dir(&out).unwrap() {
        let p = e.unwrap().path();
        let name = p.file_name().unwrap().to_str().unwrap().to_string();
        if !name.ends_with(".labels.jsonl") {
            continue;
        }
        let spec: Value = serde_json::from_str(
            &fs::read_to_string(out.join(name.replace(".labels.jsonl", ".json"))).unwrap(),
        )
        .unwrap();
        assert!(serde_json::to_string(&spec).unwrap().len() < specs::MAX_SPEC_BYTES);
        let text = fs::read_to_string(&p).unwrap();
        labels.push(text.clone());
        for line in text.lines() {
            let l: Value = serde_json::from_str(line).unwrap();
            let arm = l["labels"][0]["arm"].as_str().unwrap();
            let factor = if arm == "B" || arm == "ctlB" {
                0.95
            } else {
                1.0
            };
            for t in 0..spec["workloads"]["targets_per_curve"].as_u64().unwrap() {
                for round in 0..6 {
                    for a in spec["arms"].as_array().unwrap() {
                        let name = a["name"].as_str().unwrap();
                        n += 1;
                        let noise = 0.9 + 0.2 * ((n * 2654435761) % 1000) as f64 / 1000.0;
                        let total = match name {
                            "rho-neg" | "rho-neg-aa" => 1400.0 + t as f64 * 10.0 + round as f64,
                            "bsgs-neg" => 1700.0 + t as f64,
                            _ => 77.0 * 1147.86 * factor * noise,
                        };
                        recs += &record(
                            l["job"].as_str().unwrap(),
                            &l["workload"],
                            name,
                            t,
                            round,
                            total,
                        );
                        recs.push('\n');
                    }
                }
            }
        }
    }
    let lab = analyse::read_labels(&labels).unwrap();
    let (obs, unlabelled) = analyse::observations(&[recs.clone()], &lab).unwrap();
    assert!(unlabelled.is_empty());
    let cmp = |x: &str, y: &str, xm: &str, ym: &str, metric: &str| {
        analyse::compare(
            &obs,
            &json!({"id": format!("{x}-{y}"), "tier": "24", "metric": metric, "alternative": "less",
            "x": {"arm": x, "method": xm}, "y": {"arm": y, "method": ym}}),
            200,
            999,
            1,
        )
        .unwrap()
    };
    let hb = cmp("B", "A", "ic-mitm", "ic-mitm", "S");
    let t0 = hb["by_weight"][0]["theta"].as_f64().unwrap();
    assert!((0.92..0.98).contains(&t0), "theta {t0}");
    // Transport is charged: the ratio rises with the weight.
    assert!(hb["by_weight"][2]["theta"].as_f64().unwrap() > t0);
    let aa = cmp("Aaa", "A", "ic-mitm", "ic-mitm", "S");
    let ci = &aa["by_weight"][0]["ci95"];
    assert!(ci[0].as_f64().unwrap() <= 1.0 && ci[1].as_f64().unwrap() >= 1.0);
    let y = cmp("ctlA", "ctlA", "ic-mitm-plus1", "ic-mitm", "yield");
    assert!(y["by_weight"][0]["theta"].as_f64().unwrap() > 1.05);
    let runs: Vec<analyse::Run> = recs
        .lines()
        .map(|l| analyse::parse_run(l).unwrap())
        .collect();
    assert_eq!(analyse::check_determinism(&runs)["pass"], json!(true));
    assert_eq!(analyse::check_transport(&obs, "24")["pass"], json!(true));
}
