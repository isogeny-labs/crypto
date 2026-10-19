//! Native receipt admission and paired statistics; no solver is run here.
#[path = "../f4_f5_boolean_runtime_20261019/analyze.rs"]
#[allow(dead_code)]
mod common;
use common::{comparison, median, select, spread, stable, Sample};
use serde_json::{json, Value};
use std::{fs, path::Path};

fn checked(sample: &Sample, arm: &str, n: u32, seed: u64, random: u32, policy: Option<&str>) {
    let row = &sample.row;
    assert_eq!(row["arm"], arm);
    assert_eq!(row["n"], n);
    assert_eq!(row["seed"], seed);
    assert_eq!(row["random"], random == 1);
    assert_eq!(row["table_write_once"], true);
    assert_eq!(row["vector_tables"], true);
    if let Some(policy) = policy {
        assert_eq!(row["matrix_runtime"], policy);
    }
    if row["stats"].is_object() {
        assert_eq!(row["stats"]["exhausted"], false);
        assert_eq!(row["stats"]["unsupported"], false);
    }
}
fn profiles(rows: &[Value]) -> Value {
    let mut out = serde_json::Map::new();
    for name in [
        "calls",
        "build_ns",
        "reduce_ns",
        "readback_ns",
        "rows",
        "cols",
        "word_ops",
        "criterion_word_ops",
    ] {
        let values: Vec<_> = rows
            .iter()
            .filter_map(|r| r["profile"][name].as_u64().map(|v| v as f64))
            .collect();
        if !values.is_empty() {
            out.insert(
                name.into(),
                json!({"median":median(&values),"median_ms":name.ends_with("_ns").then(||median(&values)/1e6),"samples":values}),
            );
        }
    }
    for name in [
        "reductions",
        "splits",
        "tail_enumerations",
        "tail_assignments_tested",
    ] {
        let values: Vec<_> = rows
            .iter()
            .filter_map(|r| r["stats"][name].as_u64().map(|v| v as f64))
            .collect();
        if !values.is_empty() {
            out.insert(
                name.into(),
                json!({"median":median(&values),"samples":values}),
            );
        }
    }
    Value::Object(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn profiles_keep_count_units_separate_from_time() {
        let result = profiles(&[
            json!({"profile":{"calls":7,"build_ns":2_000_000},"stats":{"tail_assignments_tested":16}}),
            json!({"profile":{"calls":9,"build_ns":4_000_000},"stats":{"tail_assignments_tested":32}}),
        ]);
        assert_eq!(result["calls"]["median"], 8.0);
        assert_eq!(result["build_ns"]["median_ms"], 3.0);
        assert!(result["calls"]["median_ms"].is_null());
        assert_eq!(result["tail_assignments_tested"]["median"], 24.0);
    }
}
fn main() {
    let root = std::env::args()
        .nth(1)
        .expect("analyze-native NATIVE_OUTPUT");
    let root = Path::new(&root);
    let adaptive = root.join("ADAPTIVE_PROTOCOL.md").exists();
    let prepared_policy = if adaptive {
        "adaptive-prepared"
    } else {
        "prepared"
    };
    let tail_policy = if adaptive {
        "adaptive-quadratic-tail"
    } else {
        "quadratic-tail"
    };
    assert_eq!(
        fs::read_to_string(root.join("native-phase.status"))
            .unwrap()
            .trim(),
        "complete"
    );
    let mut rejected = Vec::new();
    let mut changes = Vec::new();
    for n in [17, 23] {
        for (seed, random) in [(230, 0), (230, 1), (231, 0), (232, 0)] {
            for arm in ["solve-f4", "solve-f5", "decomp-f4", "decomp-f5"] {
                let mut aa = Vec::new();
                let mut before = Vec::new();
                let mut prepared = Vec::new();
                let mut tail = Vec::new();
                let mut prepared_ratios = Vec::new();
                let mut tail_ratios = Vec::new();
                let mut incremental = Vec::new();
                let mut old_rows = Vec::new();
                let mut prepared_rows = Vec::new();
                let mut tail_rows = Vec::new();
                let mut sources = Vec::new();
                for round in 0..5 {
                    let case = format!("{arm}-n{n}-s{seed}-r{random}-{round}");
                    if let (Some(a), Some(b)) = (
                        select(root, &format!("controls/{case}-a"), &mut rejected),
                        select(root, &format!("controls/{case}-b"), &mut rejected),
                    ) {
                        assert_eq!(stable(&a.row), stable(&b.row));
                        aa.push(a.cold / b.cold);
                    }
                    let a = select(root, &format!("paired/{case}-a"), &mut rejected);
                    let b = select(root, &format!("paired/{case}-prepared"), &mut rejected);
                    let c = select(root, &format!("paired/{case}-tail"), &mut rejected);
                    if let (Some(a), Some(b), Some(c)) = (a, b, c) {
                        checked(&a, arm, n, seed, random, None);
                        checked(&b, arm, n, seed, random, Some(prepared_policy));
                        checked(&c, arm, n, seed, random, Some(tail_policy));
                        assert_eq!(stable(&a.row), stable(&b.row));
                        assert_eq!(stable(&a.row), stable(&c.row));
                        before.push(a.cold);
                        prepared.push(b.cold);
                        tail.push(c.cold);
                        prepared_ratios.push(a.cold / b.cold);
                        tail_ratios.push(a.cold / c.cold);
                        incremental.push(b.cold / c.cold);
                        old_rows.push(a.row);
                        prepared_rows.push(b.row);
                        tail_rows.push(c.row);
                        sources.push(json!([a.name, b.name, c.name]));
                    }
                }
                let noise = if aa.len() == 5 {
                    median(&aa.iter().map(|x| (x - 1.0).abs()).collect::<Vec<_>>())
                } else {
                    f64::INFINITY
                };
                changes.push(json!({"arm":arm,"n":n,"seed":seed,"random":random,"before":spread(&before),"prepared":spread(&prepared),"candidate":spread(&tail),
                    "prepared_comparison":comparison(&prepared_ratios,noise),"comparison":comparison(&tail_ratios,noise),"tail_increment":comparison(&incremental,noise),
                    "aa_ratios":aa,"before_profile":profiles(&old_rows),"prepared_profile":profiles(&prepared_rows),"candidate_profile":profiles(&tail_rows),"sources":sources}));
            }
        }
    }
    let mut controls = Vec::new();
    for n in [17, 23] {
        for arm in ["matrix-f4", "matrix-f5"] {
            let mut aa = Vec::new();
            let mut ratios = Vec::new();
            let mut before = Vec::new();
            let mut after = Vec::new();
            for round in 0..5 {
                let case = format!("{arm}-n{n}-{round}");
                if let (Some(a), Some(b)) = (
                    select(root, &format!("four-worker/{case}-aa"), &mut rejected),
                    select(root, &format!("four-worker/{case}-ab"), &mut rejected),
                ) {
                    assert_eq!(stable(&a.row), stable(&b.row));
                    aa.push(a.cold / b.cold);
                }
                if let (Some(a), Some(b)) = (
                    select(root, &format!("four-worker/{case}-a"), &mut rejected),
                    select(root, &format!("four-worker/{case}-b"), &mut rejected),
                ) {
                    assert_eq!(stable(&a.row), stable(&b.row));
                    for sample in [&a, &b] {
                        assert!(
                            fs::read_to_string(root.join(format!("{}.command", sample.name)))
                                .unwrap()
                                .contains("RAYON_NUM_THREADS=4")
                        );
                    }
                    ratios.push(a.cold / b.cold);
                    before.push(a.cold);
                    after.push(b.cold);
                }
            }
            let noise = if aa.len() == 5 {
                median(&aa.iter().map(|x| (x - 1.0).abs()).collect::<Vec<_>>())
            } else {
                f64::INFINITY
            };
            controls.push(json!({"arm":arm,"n":n,"before":spread(&before),"after":spread(&after),"comparison":comparison(&ratios,noise),"aa_ratios":aa}));
        }
    }
    let mut costs = Vec::new();
    for n in [17, 23] {
        for random in [0, 1] {
            for (boundary, arms) in [
                (
                    "complete_roots",
                    vec![
                        "roots-gray",
                        "roots-wide",
                        "solve-f4",
                        "solve-f5",
                        "solve-inherited",
                        "generalized-native",
                        "generalized-typed",
                        "roots-moebius",
                        "roots-monica",
                        "generic-gb",
                    ],
                ),
                (
                    "decomposition",
                    vec![
                        "decomp-f4",
                        "decomp-f5",
                        "decomp-f6",
                        "decomp-f6-pairs",
                        "decomp-generalized",
                        "decomp-generalized-typed",
                        "rootfind-mitm",
                        "rootfind-table",
                    ],
                ),
            ] {
                let mut fingerprint = None;
                for arm in arms {
                    let mut cold = Vec::new();
                    let mut query = Vec::new();
                    let mut sources = Vec::new();
                    let mut rows = Vec::new();
                    for round in 0..5 {
                        if let Some(sample) = select(
                            root,
                            &format!("comparison/{arm}-n{n}-r{random}-{round}"),
                            &mut rejected,
                        ) {
                            checked(&sample, arm, n, 230, random, Some(tail_policy));
                            let fp = stable(&sample.row);
                            if let Some(previous) = &fingerprint {
                                assert_eq!(previous, &fp);
                            } else {
                                fingerprint = Some(fp);
                            }
                            cold.push(sample.cold);
                            query.push(sample.query);
                            sources.push(sample.name);
                            rows.push(sample.row);
                        }
                    }
                    costs.push(json!({"boundary":boundary,"arm":arm,"n":n,"random":random,"cold":spread(&cold),"query":spread(&query),"profile":profiles(&rows),"sources":sources}));
                }
            }
        }
    }
    let mut table_controls = Vec::new();
    if root.join("table-four-worker").is_dir() {
        for n in [17, 23] {
            let (mut aa, mut ratios, mut before, mut after) =
                (Vec::new(), Vec::new(), Vec::new(), Vec::new());
            for round in 0..5 {
                let prefix = format!("table-four-worker/rootfind-table-n{n}-{round}");
                if let (Some(a), Some(b)) = (
                    select(root, &format!("{prefix}-aa"), &mut rejected),
                    select(root, &format!("{prefix}-ab"), &mut rejected),
                ) {
                    assert_eq!(stable(&a.row), stable(&b.row));
                    aa.push(a.cold / b.cold);
                }
                if let (Some(a), Some(b)) = (
                    select(root, &format!("{prefix}-a"), &mut rejected),
                    select(root, &format!("{prefix}-b"), &mut rejected),
                ) {
                    assert_eq!(stable(&a.row), stable(&b.row));
                    for s in [&a, &b] {
                        assert!(fs::read_to_string(root.join(format!("{}.command", s.name)))
                            .unwrap()
                            .contains("RAYON_NUM_THREADS=4"));
                    }
                    ratios.push(a.cold / b.cold);
                    before.push(a.cold);
                    after.push(b.cold);
                }
            }
            let noise = if aa.len() == 5 {
                median(&aa.iter().map(|r| (r - 1.0).abs()).collect::<Vec<_>>())
            } else {
                f64::INFINITY
            };
            table_controls.push(json!({"arm":"rootfind-table","n":n,"before":spread(&before),"after":spread(&after),"aa_ratios":aa,"comparison":comparison(&ratios,noise)}));
        }
    }
    let neutral = controls.iter().all(|row| {
        row["comparison"]["decision"] == "neutral"
            && row["before"]["n"] == 5
            && row["after"]["n"] == 5
    });
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"boolean-recursive-native/v1","baseline":"a78cc8906 algorithm with previous three GF2 switches enabled","changes":changes,"four_worker_controls":controls,"all_four_worker_controls_neutral":neutral,"full_table_four_worker_controls":table_controls,"costs":costs,"rejected_attempts":rejected,
        "host":{"cpu":fs::read_to_string(root.join("cpu.txt")).unwrap(),"compiler":fs::read_to_string(root.join("compiler.txt")).unwrap(),"cpus":fs::read_to_string(root.join("reserved-cpus.txt")).unwrap(),"binary_sha256":fs::read_to_string(root.join("binaries.sha256")).unwrap(),"source_sha256":fs::read_to_string(root.join("source.sha256")).unwrap()},
        "scope":"One-worker native calls, exact original outputs, fresh eighteen-arm table with latest policies enabled; cold Semaev setup charged. Separate ecbench sessions measure full DLP recovery."})).unwrap());
}
