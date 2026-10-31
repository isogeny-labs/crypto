//! Native admission and comparison; rendering consumes this JSON unchanged.
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};

pub(crate) fn median(v: &[f64]) -> f64 {
    assert!(!v.is_empty());
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    if v.len() % 2 == 0 {
        (v[v.len() / 2 - 1] + v[v.len() / 2]) / 2.0
    } else {
        v[v.len() / 2]
    }
}
pub(crate) fn spread(v: &[f64]) -> Value {
    if v.is_empty() {
        return json!({"n":0});
    }
    json!({"n":v.len(),"median_ns":median(v),"min_ns":v.iter().copied().fold(f64::INFINITY,f64::min),
        "max_ns":v.iter().copied().fold(0.0,f64::max),"median_ms":median(v)/1e6,
        "min_ms":v.iter().copied().fold(f64::INFINITY,f64::min)/1e6,"max_ms":v.iter().copied().fold(0.0,f64::max)/1e6,"samples_ns":v})
}
pub(crate) fn comparison(ratios: &[f64], noise: f64) -> Value {
    if ratios.len() != 5 || !noise.is_finite() {
        return json!({"decision":"inconclusive","paired_samples":ratios.len()});
    }
    let mut boot = Vec::with_capacity(3125);
    for mut code in 0..3125 {
        let mut sample = [0.0; 5];
        for v in &mut sample {
            *v = ratios[code % 5];
            code /= 5;
        }
        boot.push(median(&sample));
    }
    boot.sort_by(f64::total_cmp);
    let ratio = median(ratios);
    let lo = boot[78];
    let hi = boot[3046];
    let threshold = 0.02_f64.max(2.0 * noise);
    let decision = if lo > 1.0 && ratio - 1.0 > threshold {
        "faster"
    } else if hi < 1.0 && 1.0 - ratio > threshold {
        "slower"
    } else {
        "neutral"
    };
    json!({"decision":decision,"reference_over_candidate":ratio,"ci95":[lo,hi],"ratios":ratios,
        "effect_threshold":threshold,"aa_absolute_noise":noise,"paired_samples":5})
}
pub(crate) fn stable(row: &Value) -> Value {
    let arm = row["arm"].as_str().unwrap();
    if arm.starts_with("matrix-") {
        json!([
            row["input_digest"],
            row["output_digest"],
            row["row_space"],
            row["rank"],
            row["word_ops"]
        ])
    } else if arm.starts_with("decomp-") || arm.starts_with("rootfind-") {
        json!([row["input_digest"], row["target_x"], row["hit"]])
    } else {
        json!([row["input_digest"], row["roots"]])
    }
}
#[derive(Clone)]
pub(crate) struct Sample {
    pub(crate) row: Value,
    pub(crate) query: f64,
    pub(crate) cold: f64,
    pub(crate) name: String,
}
pub(crate) fn select(root: &Path, prefix: &str, rejections: &mut Vec<Value>) -> Option<Sample> {
    for attempt in 0..3 {
        let name = format!("{prefix}-{attempt}");
        let read = |extension: &str| fs::read_to_string(root.join(format!("{name}.{extension}")));
        let exit = read("exit").unwrap_or_default();
        let iso = read("isolation.jsonl").ok().and_then(|t| {
            t.lines()
                .last()
                .and_then(|s| serde_json::from_str::<Value>(s).ok())
        });
        let reason = if exit.trim() != "0" {
            Some(format!("exit {}", exit.trim()))
        } else if let Some(i) = &iso {
            if i["run"]["exit_status"] != 0 || i["run"]["contended"] != false {
                Some("failed or contended isolation".into())
            } else if i["left_on_reserved"]["user_threads"]
                .as_array()
                .is_none_or(|a| !a.is_empty())
            {
                Some("residual user threads or missing reservation".into())
            } else {
                None
            }
        } else {
            Some("missing isolation receipt".into())
        };
        if let Some(reason) = reason {
            rejections.push(json!({"name":name,"reason":reason}));
            continue;
        }
        let text = read("jsonl").expect("successful run needs output");
        let lines = text.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 1, "one fixed target per process: {name}");
        let row: Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(row["verified"], true, "{name}");
        let query = row["elapsed_ns"].as_u64().expect("elapsed ns") as f64;
        assert!(query > 0.0);
        let arm = row["arm"].as_str().unwrap();
        if let Some(roots) = row["roots"].as_array() {
            let roots = roots
                .iter()
                .map(|v| v.as_u64().unwrap())
                .collect::<Vec<_>>();
            assert!(
                roots.windows(2).all(|w| w[0] < w[1]),
                "unique sorted roots: {name}"
            );
        }
        if arm.starts_with("decomp-") {
            assert_eq!(row["stats"]["exhausted"], false);
            assert_eq!(row["stats"]["unsupported"], false);
        }
        let setup = if arm.starts_with("rootfind-") {
            row["pair_setup_ns"].as_u64().expect("cold setup") as f64
        } else {
            0.0
        };
        return Some(Sample {
            row,
            query,
            cold: query + setup,
            name,
        });
    }
    None
}
fn noise(
    root: &Path,
    directory: &str,
    arm: &str,
    n: u32,
    random: u8,
    rejections: &mut Vec<Value>,
) -> (f64, Value) {
    let mut ratios = Vec::new();
    let mut fingerprint = None;
    for round in 0..5 {
        let prefix = format!("{directory}/{arm}-n{n}-r{random}-{round}");
        let a = select(root, &format!("{prefix}-a"), rejections);
        let b = select(root, &format!("{prefix}-b"), rejections);
        if let (Some(a), Some(b)) = (a, b) {
            let fp = stable(&a.row);
            assert_eq!(fp, stable(&b.row));
            if let Some(old) = &fingerprint {
                assert_eq!(*old, fp)
            } else {
                fingerprint = Some(fp)
            }
            ratios.push(a.cold / b.cold);
        }
    }
    let noise = if ratios.len() == 5 {
        median(&ratios.iter().map(|r| (r - 1.0).abs()).collect::<Vec<_>>())
    } else {
        f64::INFINITY
    };
    (
        noise,
        json!({"arm":arm,"n":n,"random":random,"ratios":ratios,"median_absolute_noise":if noise.is_finite(){Some(noise)}else{None}}),
    )
}
fn analyze(root: &Path) -> Value {
    let policy_run = root.join("policy-phase.status").exists();
    let final_run = root.join("final-phase.status").exists() || policy_run;
    let mut rejected = Vec::new();
    let mut matrices = Vec::new();
    let mut controls = Vec::new();
    for arm in if final_run {
        vec![]
    } else {
        vec!["matrix-f4", "matrix-f5"]
    } {
        let mut aa = Vec::new();
        for round in 0..5 {
            let a = select(root, &format!("matrix/{arm}-aa-{round}-a"), &mut rejected);
            let b = select(root, &format!("matrix/{arm}-aa-{round}-b"), &mut rejected);
            if let (Some(a), Some(b)) = (a, b) {
                assert_eq!(stable(&a.row), stable(&b.row));
                aa.push(a.query / b.query);
            }
        }
        let matrix_noise = if aa.len() == 5 {
            median(&aa.iter().map(|r| (r - 1.0).abs()).collect::<Vec<_>>())
        } else {
            f64::INFINITY
        };
        for (n, seed) in [(23, 230), (17, 230), (23, 231), (23, 232)] {
            let mut reference = Vec::new();
            let mut candidate = Vec::new();
            let mut ratios = Vec::new();
            let mut sources = Vec::new();
            let mut profiles = Vec::new();
            let mut fp = None;
            for round in 0..5 {
                let base = if n == 23 && seed == 230 {
                    format!("matrix/{arm}-ab-{round}")
                } else {
                    format!("matrix/{arm}-n{n}-s{seed}-{round}")
                };
                let a = select(root, &format!("{base}-a"), &mut rejected);
                let b = select(root, &format!("{base}-b"), &mut rejected);
                if let (Some(a), Some(b)) = (a, b) {
                    assert_eq!(a.row["table_write_once"], false);
                    assert_eq!(b.row["table_write_once"], true);
                    assert_eq!(a.row["n"], n);
                    assert_eq!(a.row["seed"], seed);
                    assert_eq!(stable(&a.row), stable(&b.row));
                    let now = stable(&a.row);
                    if let Some(old) = &fp {
                        assert_eq!(*old, now)
                    } else {
                        fp = Some(now)
                    }
                    reference.push(a.query);
                    candidate.push(b.query);
                    ratios.push(a.query / b.query);
                    profiles.push(json!({"a":a.row["profile"],"b":b.row["profile"]}));
                    sources.push(json!([a.name, b.name]));
                }
            }
            matrices.push(json!({"arm":arm,"n":n,"seed":seed,"reference":spread(&reference),"candidate":spread(&candidate),
                "comparison":comparison(&ratios,matrix_noise),"aa_ratios":aa,"sources":sources,"phase_profiles":profiles}));
        }
    }
    let mut costs = Vec::new();
    for n in [17u32, 23] {
        for random in [0u8, 1] {
            for (boundary, reference, control, arms) in [
                (
                    "complete_roots",
                    "solve-f4",
                    if final_run { "solve-f4" } else { "roots-gray" },
                    vec![
                        "solve-f4",
                        "solve-f5",
                        "solve-inherited",
                        "generic-gb",
                        "roots-gray",
                        "roots-moebius",
                        "roots-monica",
                        "generalized-native",
                    ],
                ),
                (
                    "decomposition",
                    "decomp-f4",
                    "decomp-f6",
                    vec![
                        "decomp-f4",
                        "decomp-f5",
                        "decomp-f6",
                        "decomp-f6-pairs",
                        "decomp-generalized",
                        "rootfind-mitm",
                        "rootfind-table",
                    ],
                ),
            ] {
                let (aa_noise, aa) = noise(
                    root,
                    if final_run {
                        "reference-controls"
                    } else {
                        "controls"
                    },
                    control,
                    n,
                    random,
                    &mut rejected,
                );
                controls.push(aa);
                let mut rows = BTreeMap::<&str, Vec<Option<Sample>>>::new();
                let mut fingerprint = None;
                let mut arms = arms;
                if final_run && boundary == "complete_roots" {
                    arms.push("roots-wide");
                }
                if policy_run {
                    arms.push(if boundary == "complete_roots" {
                        "generalized-typed"
                    } else {
                        "decomp-generalized-typed"
                    });
                }
                for arm in arms {
                    let mut samples = Vec::new();
                    for round in 0..5 {
                        let sample = select(
                            root,
                            &format!("comparison/{arm}-n{n}-r{random}-{round}"),
                            &mut rejected,
                        );
                        if let Some(s) = &sample {
                            assert_eq!(s.row["arm"], arm);
                            assert_eq!(s.row["n"], n);
                            assert_eq!(s.row["random"], random == 1);
                            assert_eq!(s.row["table_write_once"], true);
                            let fp = stable(&s.row);
                            if let Some(old) = &fingerprint {
                                assert_eq!(*old, fp, "{arm} n{n} r{random}")
                            } else {
                                fingerprint = Some(fp)
                            }
                        }
                        samples.push(sample);
                    }
                    rows.insert(arm, samples);
                }
                for (&arm, samples) in &rows {
                    let query = samples
                        .iter()
                        .flatten()
                        .map(|s| s.query)
                        .collect::<Vec<_>>();
                    let cold = samples.iter().flatten().map(|s| s.cold).collect::<Vec<_>>();
                    let ratios = samples
                        .iter()
                        .zip(&rows[reference])
                        .filter_map(|(s, r)| {
                            s.as_ref().zip(r.as_ref()).map(|(s, r)| r.cold / s.cold)
                        })
                        .collect::<Vec<_>>();
                    let f6_ratios = if boundary == "decomposition" {
                        samples
                            .iter()
                            .zip(&rows["decomp-f6"])
                            .filter_map(|(s, r)| {
                                s.as_ref().zip(r.as_ref()).map(|(s, r)| r.cold / s.cold)
                            })
                            .collect::<Vec<_>>()
                    } else {
                        Vec::new()
                    };
                    let mut phases = serde_json::Map::new();
                    for key in ["build_ns", "reduce_ns", "readback_ns"] {
                        let v = samples
                            .iter()
                            .flatten()
                            .map(|s| s.row["profile"][key].as_u64().unwrap_or(0) as f64)
                            .collect::<Vec<_>>();
                        phases.insert(key.into(), spread(&v));
                    }
                    costs.push(json!({"boundary":boundary,"arm":arm,"n":n,"random":random,"query":spread(&query),"cold":spread(&cold),
                    "reference":reference,"comparison":comparison(&ratios,aa_noise),"sources":samples.iter().flatten().map(|s|&s.name).collect::<Vec<_>>(),
                    "verified_output":samples.iter().flatten().next().map(|s| json!({"input_digest":s.row["input_digest"],"roots":s.row["roots"],"hit":s.row["hit"],"target_x":s.row["target_x"]})),
                    "f6_over_candidate":comparison(&f6_ratios,aa_noise),"phase_medians":phases,
                    "phase_profiles":samples.iter().flatten().map(|s|&s.row["profile"]).collect::<Vec<_>>()}));
                }
            }
        }
    }
    // Admission refusals are retained as failures, never accepted timings.
    for arm in if final_run {
        vec![]
    } else {
        vec!["matrix-f4", "matrix-f5"]
    } {
        for state in 0..2 {
            let sample = select(
                root,
                &format!("matrix/{arm}-n19-state{state}"),
                &mut rejected,
            );
            assert!(
                sample.is_none(),
                "n19 admission changed; review protocol before ranking"
            );
        }
    }
    let mut batching = Vec::new();
    if final_run && !policy_run {
        for n in [17u32, 23] {
            for random in [0u8, 1] {
                let (aa_noise, aa) =
                    noise(root, "controls", "rootfind-mitm", n, random, &mut rejected);
                controls.push(aa);
                let (mut before, mut after, mut ratios, mut sources) =
                    (Vec::new(), Vec::new(), Vec::new(), Vec::new());
                let (mut before_query, mut after_query) = (Vec::new(), Vec::new());
                for round in 0..5 {
                    let prefix = format!("paired/rootfind-mitm-n{n}-r{random}-{round}");
                    let a = select(root, &format!("{prefix}-a"), &mut rejected);
                    let b = select(root, &format!("{prefix}-b"), &mut rejected);
                    if let (Some(a), Some(b)) = (a, b) {
                        assert_eq!(stable(&a.row), stable(&b.row));
                        assert_eq!(a.row["indices"], b.row["indices"]);
                        assert_eq!(a.row["setup_group_ops"], b.row["setup_group_ops"]);
                        for key in ["pair_table_entries", "abscissae"] {
                            assert_eq!(a.row["setup_counts"][key], b.row["setup_counts"][key]);
                        }
                        assert_eq!(b.row["setup_counts"]["pair_table_batch_setup"], 1);
                        before.push(a.cold);
                        after.push(b.cold);
                        before_query.push(a.query);
                        after_query.push(b.query);
                        ratios.push(a.cold / b.cold);
                        sources.push(json!([a.name, b.name]));
                    }
                }
                batching.push(json!({"n":n,"random":random,"before":spread(&before),"after":spread(&after),"before_query":spread(&before_query),"after_query":spread(&after_query),"comparison":comparison(&ratios,aa_noise),"sources":sources}));
            }
        }
    }
    let cpu = fs::read_to_string(root.join("cpu.txt")).unwrap_or_default();
    let cpu_model = cpu
        .lines()
        .find_map(|line| line.trim().strip_prefix("Model name:").map(str::trim));
    let host = json!({"cpu_model":cpu_model,"cpu_manifest":cpu,"compiler":fs::read_to_string(root.join("compiler.txt")).ok(),
        "reserved_cpus":fs::read_to_string(root.join("reserved-cpus.txt")).ok(),"memory_manifest":fs::read_to_string(root.join("memory.txt")).ok(),
        "binary_sha256":fs::read_to_string(root.join("binaries.sha256")).ok(),"source_sha256":fs::read_to_string(root.join("source.sha256")).ok()});
    json!({"schema":"boolean-native-comparison/v1","baseline":if policy_run {"frozen-iteration1 for policy pairs; candidate-only18-arm costs"}else{"1340a6176701b3f541a36112a02a3d4a90d1075a"},"final_run":final_run,"policy_run":policy_run,"batching":batching,"host":host,
        "matrices":matrices,"costs":costs,"controls":controls,"rejected_attempts":rejected,
        "bootstrap":"all3125 resamplings of five paired ratios; percentiles78/3046",
        "scope":"one worker, dedicated x86-64 VM, original matched public fixtures; cold oracle setup charged; no full IC/rho ratio"})
}
fn main() {
    let root = std::env::args()
        .nth(1)
        .expect("analyze NATIVE_OUTPUT_DIRECTORY");
    println!(
        "{}",
        serde_json::to_string_pretty(&analyze(Path::new(&root))).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paired_decisions_respect_noise_and_missing_data() {
        assert_eq!(comparison(&[1.3; 5], 0.01)["decision"], "faster");
        assert_eq!(comparison(&[0.7; 5], 0.01)["decision"], "slower");
        assert_eq!(comparison(&[1.03; 5], 0.04)["decision"], "neutral");
        assert_eq!(comparison(&[1.3; 4], 0.01)["decision"], "inconclusive");
        assert_eq!(
            comparison(&[1.3; 5], f64::INFINITY)["decision"],
            "inconclusive"
        );
    }
    #[test]
    fn admission_checks_residual_threads_and_first_qualified_retry() {
        let root =
            std::env::temp_dir().join(format!("boolean-native-admission-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let mut rejected = Vec::new();
        for attempt in 0..3 {
            fs::write(root.join(format!("sample-{attempt}.exit")), "0").unwrap();
            fs::write(
                root.join(format!("sample-{attempt}.jsonl")),
                json!({"arm":"roots-gray","verified":true,"elapsed_ns":123,"roots":[1,3]})
                    .to_string(),
            )
            .unwrap();
            fs::write(root.join(format!("sample-{attempt}.isolation.jsonl")),json!({"run":{"exit_status":0,"contended":false},"left_on_reserved":{"user_threads":if attempt==0{json!([42])}else{json!([])}}}).to_string()).unwrap();
        }
        let s = select(&root, "sample", &mut rejected).unwrap();
        assert_eq!(s.name, "sample-1");
        assert_eq!(rejected.len(), 1);
        fs::remove_dir_all(root).unwrap();
    }
}
