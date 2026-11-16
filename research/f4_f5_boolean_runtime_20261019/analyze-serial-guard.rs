//! Incremental same-binary one-worker dispatch; four-worker negative controls.
#[path = "analyze.rs"]
#[allow(dead_code)]
mod common;
use common::{comparison, median, select, spread, stable, Sample};
use serde_json::json;
use std::{fs, path::Path};

fn equal(a: &Sample, b: &Sample) {
    assert_eq!(stable(&a.row), stable(&b.row));
    assert_eq!(a.row["profile"]["counts"], b.row["profile"]["counts"]);
    for key in [
        "word_ops",
        "rows",
        "cols",
        "calls",
        "oversize",
        "rows_pruned",
        "criterion_word_ops",
        "specialise_word_ops",
    ] {
        assert_eq!(a.row["profile"][key], b.row["profile"][key], "{key}");
    }
}
fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("analyze-serial-guard OUTPUT_DIRECTORY");
    let root = Path::new(&arg);
    assert_eq!(
        fs::read_to_string(root.join("serial-guard-phase.status"))
            .unwrap()
            .trim(),
        "complete"
    );
    let mut rejected = Vec::new();
    let mut matrices = Vec::new();
    for workers in [1, 4] {
        for arm in ["matrix-f4", "matrix-f5"] {
            for (n, seed) in [(17, 230), (23, 230), (23, 231), (23, 232)] {
                if workers == 4 && seed != 230 {
                    continue;
                }
                let case = format!("{arm}-n{n}-s{seed}-w{workers}");
                let mut aa = Vec::new();
                let mut before = Vec::new();
                let mut after = Vec::new();
                let mut ratios = Vec::new();
                let mut sources = Vec::new();
                let mut fingerprint = None;
                let mut legacy_values = Vec::new();
                let mut direct = Vec::new();
                let mut legacy_sources = Vec::new();
                for round in 0..5 {
                    let a = select(root, &format!("controls/{case}-{round}-a"), &mut rejected);
                    let b = select(root, &format!("controls/{case}-{round}-b"), &mut rejected);
                    if let (Some(a), Some(b)) = (a, b) {
                        equal(&a, &b);
                        aa.push(a.query / b.query);
                    }
                    let a = select(root, &format!("paired/{case}-{round}-a"), &mut rejected);
                    let b = select(root, &format!("paired/{case}-{round}-b"), &mut rejected);
                    if let (Some(a), Some(b)) = (a, b) {
                        equal(&a, &b);
                        let fp = stable(&a.row);
                        if let Some(old) = &fingerprint {
                            assert_eq!(*old, fp)
                        } else {
                            fingerprint = Some(fp)
                        }
                        for (sample, state) in [(&a, 0), (&b, 1)] {
                            let command =
                                fs::read_to_string(root.join(format!("{}.command", sample.name)))
                                    .unwrap();
                            assert!(command.contains(&format!("KIC_GF2_SERIAL_CLEAR={state}")));
                            assert!(command.contains(&format!("RAYON_NUM_THREADS={workers}")));
                            assert_eq!(sample.row["table_write_once"], true);
                            assert_eq!(sample.row["vector_tables"], true);
                        }
                        if workers == 1 {
                            if let Some(legacy) =
                                select(root, &format!("legacy/{case}-{round}"), &mut rejected)
                            {
                                equal(&legacy, &b);
                                assert_eq!(legacy.row["table_write_once"], false);
                                assert_eq!(legacy.row["vector_tables"], false);
                                legacy_values.push(legacy.query);
                                direct.push(legacy.query / b.query);
                                legacy_sources.push(json!([legacy.name, b.name.clone()]));
                            }
                        }
                        before.push(a.query);
                        after.push(b.query);
                        ratios.push(a.query / b.query);
                        sources.push(json!([a.name, b.name]));
                    }
                }
                let noise = if aa.len() == 5 {
                    median(&aa.iter().map(|r| (r - 1.0).abs()).collect::<Vec<_>>())
                } else {
                    f64::INFINITY
                };
                matrices.push(json!({"arm":arm,"n":n,"seed":seed,"workers":workers,"before":spread(&before),"after":spread(&after),
                    "comparison":comparison(&ratios,noise),"legacy":spread(&legacy_values),"legacy_over_candidate":comparison(&direct,noise),"legacy_sources":legacy_sources,"aa_ratios":aa,"sources":sources,"verified_output":fingerprint,
                    "interpretation":if workers==4 {"unchanged-path negative control; no algorithmic gain inferred"}else{"incremental one-worker matrix dispatch"}}));
            }
        }
    }
    let controls = matrices
        .iter()
        .filter(|r| r["workers"] == 4)
        .collect::<Vec<_>>();
    let all_controls_neutral = controls.len() == 4
        && controls.iter().all(|r| {
            r["comparison"]["decision"] == "neutral"
                && r["before"]["n"] == 5
                && r["after"]["n"] == 5
        });
    let cpu = fs::read_to_string(root.join("cpu.txt")).unwrap();
    let model = cpu
        .lines()
        .find_map(|l| l.trim().strip_prefix("Model name:").map(str::trim));
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"boolean-serial-guard/v1","matrices":matrices,"all_four_worker_controls_neutral":all_controls_neutral,"rejected_attempts":rejected,
        "host":{"cpu_model":model,"cpu_manifest":cpu,"compiler":fs::read_to_string(root.join("compiler.txt")).unwrap(),"reserved_cpus":fs::read_to_string(root.join("reserved-cpus.txt")).unwrap(),
        "binary_sha256":fs::read_to_string(root.join("binaries.sha256")).unwrap(),"source_sha256":fs::read_to_string(root.join("source.sha256")).unwrap()},
        "scope":"Explicit-one-worker guard confirmation. Incremental pairs keep other switches fixed; direct legacy ablation measured separately in the same binary. One-worker matrix calls only; four-worker unchanged-path controls. No complete-root or decomposition speed claim."})).unwrap());
}
