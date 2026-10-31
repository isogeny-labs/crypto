//! Native, fail-closed validation of complete-call Callgrind records.
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};

fn visit(path: &Path, files: &mut Vec<std::path::PathBuf>) {
    for e in fs::read_dir(path).unwrap().flatten() {
        if e.file_type().unwrap().is_dir() {
            visit(&e.path(), files)
        } else {
            files.push(e.path());
        }
    }
}
fn stable(row: &Value) -> Value {
    match row["arm"].as_str().unwrap() {
        "matrix-f4" => json!([
            row["input_digest"],
            row["output_digest"],
            row["row_space"],
            row["word_ops"],
            row["rank"],
            row["profile"]["rows"],
            row["profile"]["cols"]
        ]),
        "matrix-f5" => json!([
            row["input_digest"],
            row["output_digest"],
            row["row_space"],
            row["word_ops"],
            row["rank"],
            row["profile"]["counts"]
        ]),
        arm if arm.starts_with("solve-")
            || arm.starts_with("roots-")
            || arm == "generic-gb"
            || arm == "generalized-native" =>
        {
            json!([row["input_digest"], row["roots"]])
        }
        _ => json!([row["input_digest"], row["target_x"], row["hit"]]),
    }
}
fn main() {
    let root = std::env::args().nth(1).expect("analyze RECORDS_DIR");
    let mut files = Vec::new();
    visit(Path::new(&root), &mut files);
    files.sort();
    let mut profiles = BTreeMap::<(String, String), u64>::new();
    let mut setup = BTreeMap::<String, u64>::new();
    let mut sources = Vec::new();
    for path in &files {
        if !path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .contains(".callgrind")
        {
            continue;
        }
        let data = fs::read_to_string(path).unwrap();
        let Some(label) = data
            .lines()
            .find_map(|s| s.strip_prefix("desc: Trigger: Client Request: "))
        else {
            continue;
        };
        assert!(
            !data.lines().any(|s| s.starts_with("thread:")),
            "requires merged worker threads"
        );
        let label_value: Value = serde_json::from_str(label).unwrap();
        let events = data
            .lines()
            .find_map(|s| s.strip_prefix("events: "))
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();
        let ir = events.iter().position(|s| *s == "Ir").unwrap();
        let totals = data
            .lines()
            .find_map(|s| s.strip_prefix("totals: "))
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(events.len(), totals.len());
        assert!(totals[ir] > 0);
        let dir = path.parent().unwrap().to_string_lossy().to_string();
        if label_value[0] == "setup" {
            assert!(setup.insert(dir, totals[ir]).is_none());
        } else {
            assert!(
                profiles
                    .insert((dir, label.to_owned()), totals[ir])
                    .is_none(),
                "duplicate profile"
            );
        }
        sources.push(json!({"archive_member":path.strip_prefix(&root).unwrap(),"blake3":blake3::hash(data.as_bytes()).to_hex().to_string(),"ir":totals[ir],"label":label_value}));
    }
    let mut controls = BTreeMap::<String, Value>::new();
    let mut roots = BTreeMap::<String, Value>::new();
    let mut verdicts = BTreeMap::<String, Value>::new();
    let mut spaces = BTreeMap::<String, Value>::new();
    let mut groups = BTreeMap::<String, Vec<u64>>::new();
    let mut cold_groups = BTreeMap::<String, Vec<u64>>::new();
    let mut samples = Vec::new();
    let mut failures = Vec::new();
    for path in &files {
        if path.file_name().unwrap() != "run.jsonl" {
            continue;
        }
        let dir = path.parent().unwrap();
        let exit = fs::read_to_string(dir.join("exit.txt")).unwrap();
        if exit.trim() != "0" {
            failures.push(json!({"path":dir,"exit":exit.trim()}));
            continue;
        }
        let data = fs::read_to_string(path).unwrap();
        let mut rows = 0;
        for line in data.lines() {
            let row: Value = serde_json::from_str(line).unwrap();
            assert_eq!(row["verified"], true);
            rows += 1;
            let label = json!([
                row["arm"],
                row["n"],
                row["seed"],
                row["target"],
                row["repeat"]
            ])
            .to_string();
            let profile = profiles
                .get(&(dir.to_string_lossy().to_string(), label))
                .copied();
            let fixture = json!([
                row["n"],
                row["seed"],
                row["random"],
                row["target"],
                row["input_digest"]
            ])
            .to_string();
            let key = json!([row["arm"], fixture]).to_string();
            if let Some(old) = controls.insert(key, stable(&row)) {
                assert_eq!(
                    old,
                    stable(&row),
                    "before/after output or accounting changed"
                );
            }
            let target = if row["roots"].is_array() {
                Some((&mut roots, row["roots"].clone()))
            } else if row["hit"].is_boolean() {
                Some((&mut verdicts, row["hit"].clone()))
            } else {
                Some((&mut spaces, row["row_space"].clone()))
            };
            if let Some((map, value)) = target {
                if let Some(old) = map.insert(fixture.clone(), value.clone()) {
                    assert_eq!(old, value, "algorithms disagree");
                }
            }
            let summary_key = json!([
                row["arm"],
                row["n"],
                row["seed"],
                row["random"],
                row["table_write_once"],
                if dir
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("pair-")
                {
                    "paired"
                } else if dir
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("aa-")
                {
                    "aa"
                } else if dir
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("screen-")
                {
                    "screen"
                } else if dir
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("holdout-")
                {
                    "holdout"
                } else if dir
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("compare-")
                    || dir
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .starts_with("generalized-")
                {
                    "comparison"
                } else {
                    "original"
                }
            ])
            .to_string();
            if let Some(ir) = profile {
                groups.entry(summary_key.clone()).or_default().push(ir);
                cold_groups.entry(summary_key).or_default().push(
                    ir + setup
                        .get(&dir.to_string_lossy().to_string())
                        .copied()
                        .unwrap_or(0),
                );
            }
            samples.push(json!({"archive_member":path.strip_prefix(&root).unwrap(),"row":row,"ir":profile,"setup_ir":setup.get(&dir.to_string_lossy().to_string())}));
        }
        assert!(rows > 0, "successful run has no rows");
    }
    let mut summaries = Vec::new();
    for (key, mut values) in groups {
        let first = values[0];
        values.sort_unstable();
        summaries.push(json!({"key":serde_json::from_str::<Value>(&key).unwrap(),"n":values.len(),
            "first_ir":first,"median_ir":values[values.len()/2],"min_ir":values[0],"max_ir":values.last(),"samples_ir":values,
            "spread_percent":100.0*(values.last().unwrap()-values[0])as f64/values[values.len()/2]as f64}));
    }
    let mut comparisons = Vec::new();
    for candidate in &summaries {
        let key = &candidate["key"];
        if key[4] != true || (key[5] != "paired" && key[5] != "holdout") {
            continue;
        }
        if let Some(reference) = summaries.iter().find(|r| {
            r["key"][4] == false
                && r["key"][5] == key[5]
                && r["key"].as_array().unwrap()[..4] == key.as_array().unwrap()[..4]
        }) {
            let a = reference["median_ir"].as_u64().unwrap() as f64;
            let b = candidate["median_ir"].as_u64().unwrap() as f64;
            comparisons.push(json!({"case":&key.as_array().unwrap()[..4],"reference_median_ir":a,
                "candidate_median_ir":b,"reference_over_candidate":a/b,"reduction_percent":100.0*(a-b)/a,
                "reference_samples":reference["n"],"candidate_samples":candidate["n"]}));
        }
    }
    let mut cold_summaries = Vec::new();
    for (key, mut values) in cold_groups {
        values.sort_unstable();
        cold_summaries.push(json!({"key":serde_json::from_str::<Value>(&key).unwrap(),"n":values.len(),"median_ir":values[values.len()/2],"samples_ir":values}));
    }
    let mut root_comparisons = Vec::new();
    for generalized in &summaries {
        let k = &generalized["key"];
        if k[0] != "generalized-native" || k[5] != "comparison" {
            continue;
        }
        for reference in &summaries {
            let r = &reference["key"];
            if ![
                "solve-f4",
                "solve-f5",
                "solve-inherited",
                "roots-gray",
                "roots-moebius",
                "roots-monica",
                "generic-gb",
            ]
            .contains(&r[0].as_str().unwrap_or(""))
                || r[1] != k[1]
                || r[2] != k[2]
                || r[3] != k[3]
                || r[5] != "comparison"
            {
                continue;
            }
            root_comparisons.push(json!({"n":k[1],"random":k[3],"reference":r[0],"reference_ir":reference["median_ir"],"generalized_ir":generalized["median_ir"],
                "reference_over_generalized":reference["median_ir"].as_u64().unwrap()as f64/generalized["median_ir"].as_u64().unwrap()as f64}));
        }
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"boolean-perf-instructions/v1",
        "runtime_eligible":false,"simulated_isa":"Valgrind AVX2; not native AVX-512", "summaries":summaries,
        "samples":samples,"profile_sources":sources,"failures":failures,"comparisons":comparisons,"root_comparisons":root_comparisons,"cold_summaries":cold_summaries,
        "control_cases_checked":controls.len(),"root_fixtures_checked":roots.len(),
        "decomposition_fixtures_checked":verdicts.len(),"matrix_fixtures_checked":spaces.len()})).unwrap());
}
