//! Golden tests against the committed evidence: every committed measurement,
//! re-scored against its committed calibration, must reproduce the committed
//! record (all values, exactly), and the committed roofline SVGs must come out
//! byte for byte.  The committed files were written by the tool's first
//! (Python) version; this is the proof that the port changed no number.
use optlevel::cmd::{record_errors, score_documents, ScoreArgs};
use optlevel::plot;
use optlevel::util::{load_json, read_json};
use serde_json::Value;
use std::path::{Path, PathBuf};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn files(dir: &Path, suffix: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).unwrap_or_else(|e| panic!("{}: {e} (the golden tests need the committed evidence directories)", d.display())).flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.to_string_lossy().ends_with(suffix) {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}
fn evidence_dirs() -> [PathBuf; 2] {
    [repo().join("research/optlevel_20261117"), repo().join("research/hky_pod_runs_20261117/results")]
}

/// Differences between two JSON values; numbers are compared as f64 (exactly).
fn diff(a: &Value, b: &Value, path: &str, out: &mut Vec<String>) {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            if x.as_f64() != y.as_f64() {
                out.push(format!("{path}: {x} != {y}"));
            }
        }
        (Value::Object(x), Value::Object(y)) => {
            for k in x.keys().chain(y.keys().filter(|k| !x.contains_key(*k))) {
                match (x.get(k), y.get(k)) {
                    (Some(u), Some(v)) => diff(u, v, &format!("{path}.{k}"), out),
                    (u, v) => out.push(format!("{path}.{k}: {} vs {}", u.is_some(), v.is_some())),
                }
            }
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() => {
            for (i, (u, v)) in x.iter().zip(y).enumerate() {
                diff(u, v, &format!("{path}[{i}]"), out);
            }
        }
        _ => {
            if a != b {
                out.push(format!("{path}: {a} != {b}"));
            }
        }
    }
}

#[test]
fn committed_records_validate() {
    let mut n = 0;
    for dir in evidence_dirs() {
        for p in files(&dir, ".record.json") {
            assert_eq!(record_errors(&read_json(&p).unwrap()), Vec::<String>::new(), "{}", p.display());
            n += 1;
        }
    }
    assert!(n >= 17, "only {n} committed records found");
}

#[test]
fn rescoring_committed_measurements_reproduces_committed_records() {
    let mut calibrations: Vec<Value> = Vec::new();
    for dir in evidence_dirs() {
        for p in files(&dir, ".json") {
            let d = load_json(&p).unwrap();
            if d["schema"] == "optlevel.calibration/v1" {
                calibrations.push(d);
            }
        }
    }
    let (mut checked, mut attainments) = (0, 0);
    for dir in evidence_dirs() {
        for rec_path in files(&dir, ".record.json") {
            let stem = rec_path.to_string_lossy().trim_end_matches(".record.json").to_string();
            let Some(meas_path) = [".measurement.json", ".measure.json"].iter().map(|s| PathBuf::from(format!("{stem}{s}"))).find(|p| p.exists()) else {
                continue; // two pod records were committed without their measurement files
            };
            let want = read_json(&rec_path).unwrap();
            let calib = calibrations.iter().find(|c| c["_sha256"] == want["calibration"]["sha256"])
                .unwrap_or_else(|| panic!("{}: no committed calibration has sha256 {}", rec_path.display(), want["calibration"]["sha256"]));
            // what the scoring command line supplied, taken from the committed record
            let text = |v: &Value| v.as_str().map(String::from);
            let args = ScoreArgs {
                vector_share: want["diagnosis"]["vector_share_static"].as_f64(),
                repository: text(&want["kernel"]["repository"]), path: text(&want["kernel"]["path"]), registry_id: text(&want["kernel"]["registry_id"]),
                counted_ops: want["algorithmic"]["counted_ops"].as_f64(), best_known_ops: want["algorithmic"]["best_known_ops"].as_f64(),
                best_known_source: text(&want["algorithmic"]["source"]),
                ..Default::default()
            };
            let mut got = score_documents(calib, &load_json(&meas_path).unwrap(), &args).unwrap_or_else(|e| panic!("{}: {}", rec_path.display(), e.msg));
            // not derived from the measurement: the tool version, and the control's expectation text
            got["tool_version"] = want["tool_version"].clone();
            if let Some(e) = want["kernel"].get("expectation") {
                got["kernel"]["expectation"] = e.clone();
            }
            let mut d = Vec::new();
            diff(&want, &got, "$", &mut d);
            assert!(d.is_empty(), "{}:\n{}", rec_path.display(), d.join("\n"));
            for point in ["single", "full"] {
                if let Some(a) = want["attainment"].get(point) {
                    assert_eq!(a["value"].as_f64().unwrap().to_bits(), got["attainment"][point]["value"].as_f64().unwrap().to_bits());
                    attainments += 1;
                }
            }
            checked += 1;
        }
    }
    assert!(checked >= 15, "only {checked} committed records could be re-scored");
    assert!(attainments >= 27, "only {attainments} attainment values compared");
}

#[test]
fn committed_control_rooflines_are_reproduced_byte_for_byte() {
    for run in ["controls-run1", "controls-run2"] {
        let dir = repo().join("research/optlevel_20261117").join(run);
        let calib = load_json(&dir.join("calibration.json")).unwrap();
        let recs: Vec<Value> = ["positive-simd-incache", "scalar-incache", "serial-dependency", "stream-daxpy", "pointer-chase", "aa-repeat"]
            .iter().map(|c| read_json(&dir.join(format!("{c}.record.json"))).unwrap()).collect();
        let svg = plot::roofline_svg(&calib, &recs, "optlevel controls").unwrap();
        let want = std::fs::read_to_string(dir.join("controls-roofline.svg")).unwrap();
        assert!(svg == want, "{run}: controls-roofline.svg differs from the committed file");
    }
}

#[test]
fn committed_controls_table_is_reproduced() {
    for run in ["controls-run1", "controls-run2"] {
        let dir = repo().join("research/optlevel_20261117").join(run);
        let recs: Vec<Value> = ["positive-simd-incache", "scalar-incache", "serial-dependency", "stream-daxpy", "pointer-chase", "aa-repeat"]
            .iter().map(|c| read_json(&dir.join(format!("{c}.record.json"))).unwrap()).collect();
        let want = std::fs::read_to_string(dir.join("controls.md")).unwrap();
        assert_eq!(format!("{}\n", optlevel::cmd::controls_table(&recs)), want, "{run}");
    }
}
