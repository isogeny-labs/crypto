//! Native admission and exact paired-bootstrap analysis for clean CI samples.
use serde_json::{json, Value};
use std::{fs, path::Path};
fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    if v.len() % 2 == 0 {
        (v[v.len() / 2 - 1] + v[v.len() / 2]) / 2.0
    } else {
        v[v.len() / 2]
    }
}
fn analyze(root: &Path) -> Value {
    let mut output = Vec::new();
    for arm in ["matrix-f4", "matrix-f5"] {
        let mut aa = Vec::new();
        let mut ab = Vec::new();
        let mut raw = Vec::new();
        let mut fingerprint = None;
        let mut missing = Vec::new();
        for phase in ["aa", "ab"] {
            for round in 0..5 {
                let mut times = Vec::new();
                for side in ["a", "b"] {
                    let mut selected = None;
                    for attempt in 0..3 {
                        let name = format!("{arm}-{phase}-{round}-{side}-{attempt}");
                        let Ok(text) =
                            fs::read_to_string(root.join(format!("{name}.isolation.jsonl")))
                        else {
                            continue;
                        };
                        let Some(line) = text.lines().last() else {
                            continue;
                        };
                        let Ok(record) = serde_json::from_str::<Value>(line) else {
                            continue;
                        };
                        if record["run"]["exit_status"] != 0
                            || record["run"]["contended"] != false
                            || record["left_on_reserved"]["user_threads"]
                                .as_array()
                                .is_none_or(|a| !a.is_empty())
                        {
                            continue;
                        }
                        let Ok(text) = fs::read_to_string(root.join(format!("{name}.jsonl")))
                        else {
                            continue;
                        };
                        let lines = text.lines().collect::<Vec<_>>();
                        if lines.len() != 1 {
                            continue;
                        }
                        let row: Value = serde_json::from_str(lines[0]).unwrap();
                        assert_eq!(row["verified"], true);
                        assert_eq!(row["arm"], arm);
                        assert_eq!(row["n_vars"], 22);
                        assert_eq!(row["degree"], 4);
                        assert_eq!(
                            row["input_digest"],
                            "a0a37969916d31483a539b73a8af9517307abecedc3c96643200f807786445a5"
                        );
                        assert_eq!(row["table_write_once"], phase == "ab" && side == "b");
                        let fp = json!([
                            row["input_digest"],
                            row["output_digest"],
                            row["row_space"],
                            row["word_ops"],
                            row["rank"]
                        ]);
                        if let Some(old) = &fingerprint {
                            assert_eq!(*old, fp);
                        } else {
                            fingerprint = Some(fp);
                        }
                        let ns = row["elapsed_ns"].as_u64().unwrap() as f64;
                        assert!(ns > 0.0);
                        selected = Some(ns);
                        raw.push(json!({"name":name,"elapsed_ns":ns,"isolation":record}));
                        break;
                    }
                    if let Some(ns) = selected {
                        times.push(ns);
                    } else {
                        missing.push(format!("{phase}-{round}-{side}"));
                    }
                }
                if times.len() == 2 {
                    if phase == "aa" {
                        aa.push(times[0] / times[1]);
                    } else {
                        ab.push(times[0] / times[1]);
                    }
                }
            }
        }
        if !missing.is_empty() {
            output.push(json!({"arm":arm,"decision":"inconclusive","missing":missing,"raw":raw}));
            continue;
        }
        assert_eq!(aa.len(), 5);
        assert_eq!(ab.len(), 5);
        let noise = median(&aa.iter().map(|r| (r - 1.0).abs()).collect::<Vec<_>>());
        let mut boot = Vec::with_capacity(3125);
        for mut code in 0..3125 {
            let mut sample = Vec::new();
            for _ in 0..5 {
                sample.push(ab[code % 5]);
                code /= 5;
            }
            boot.push(median(&sample));
        }
        boot.sort_by(f64::total_cmp);
        let lo = boot[78];
        let hi = boot[3046];
        let ratio = median(&ab);
        let threshold = 0.02_f64.max(2.0 * noise);
        let decision = if lo > 1.0 && ratio - 1.0 > threshold {
            "runtime_improvement"
        } else if hi < 1.0 && 1.0 - ratio > threshold {
            "runtime_regression"
        } else {
            "neutral"
        };
        output.push(json!({"arm":arm,"decision":decision,"paired_median_ratio":ratio,"ci95":[lo,hi],"aa_median_absolute_noise":noise,"effect_threshold":threshold,"aa_ratios":aa,"ab_ratios":ab,"raw":raw}));
    }
    json!({"schema":"boolean-runtime-qualification/v1","bootstrap":"all 3125 five-sample resamplings; percentile indices 78 and 3046","results":output})
}
fn main() {
    let root = std::env::args().nth(1).expect("runtime RECORDS_DIRECTORY");
    println!(
        "{}",
        serde_json::to_string_pretty(&analyze(Path::new(&root))).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(tag: &str, contended: bool) -> std::path::PathBuf {
        let root =
            std::env::temp_dir().join(format!("boolean-runtime-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        for arm in ["matrix-f4", "matrix-f5"] {
            for phase in ["aa", "ab"] {
                for round in 0..5 {
                    for side in ["a", "b"] {
                        let on = phase == "ab" && side == "b";
                        let name = format!("{arm}-{phase}-{round}-{side}-0");
                        let row = json!({"arm":arm,"verified":true,"n_vars":22,"degree":4,"input_digest":"a0a37969916d31483a539b73a8af9517307abecedc3c96643200f807786445a5",
                "output_digest":"same output","row_space":123,"word_ops":456,"rank":5289,"table_write_once":on,"elapsed_ns":if on{7000}else{10000}});
                        fs::write(root.join(format!("{name}.jsonl")), row.to_string()).unwrap();
                        fs::write(root.join(format!("{name}.isolation.jsonl")),json!({"run":{"exit_status":0,"contended":contended},"left_on_reserved":{"user_threads":[]}}).to_string()).unwrap();
                    }
                }
            }
        }
        root
    }
    #[test]
    fn admits_only_qualified_complete_matching_pairs() {
        let root = fixture("admit", false);
        let out = analyze(&root);
        for r in out["results"].as_array().unwrap() {
            assert_eq!(r["decision"], "runtime_improvement");
            assert_eq!(r["raw"].as_array().unwrap().len(), 20);
        }
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn rejects_contended_and_missing_samples() {
        let root = fixture("contention", true);
        let out = analyze(&root);
        for r in out["results"].as_array().unwrap() {
            assert_eq!(r["decision"], "inconclusive");
        }
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn refuses_changed_outputs() {
        let root = fixture("mismatch", false);
        let path = root.join("matrix-f4-ab-0-b-0.jsonl");
        let mut row: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        row["output_digest"] = json!("wrong");
        fs::write(path, row.to_string()).unwrap();
        assert!(std::panic::catch_unwind(|| analyze(&root)).is_err());
        fs::remove_dir_all(root).unwrap();
    }
}
