//! `fingerprints`: every kernel once at each thread count, against the
//! recorded fingerprints in `docs/perf/fingerprints.json`.

use crate::compare::{list_kernels, run_json};
use crate::host::host_manifest;
use crate::util::{read_json, sha256_file, write_json};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(clap::Args, Clone, Debug)]
pub struct FingerprintArgs {
    #[arg(long)]
    pub bin: PathBuf,
    /// Thread counts to run each kernel at.
    #[arg(long, default_value = "1,2")]
    pub threads: String,
    #[arg(long)]
    pub golden: Option<PathBuf>,
    /// Rewrite the golden file from this binary.
    #[arg(long)]
    pub update: bool,
    #[arg(long)]
    pub full: bool,
    #[arg(long)]
    pub filter: Vec<String>,
}

pub type Found = BTreeMap<String, BTreeMap<usize, String>>;

/// One run of every kernel at each thread count, fingerprints only.
pub fn collect(
    binary: &Path,
    threads: &[usize],
    full: bool,
    filters: &[String],
) -> Result<Found, String> {
    let kernels = list_kernels(binary, full, filters)?;
    let mut out: Found = kernels
        .iter()
        .map(|k| (k.id.clone(), BTreeMap::new()))
        .collect();
    for &t in threads {
        for k in &kernels {
            let mut cmd = Command::new(binary);
            cmd.args([
                "run",
                "--filter",
                &k.id,
                "--exact",
                "--samples",
                "1",
                "--min-samples",
                "1",
                "--warmup-seconds",
                "0",
                "--max-seconds",
                "0",
            ])
            .env("RAYON_NUM_THREADS", t.to_string());
            let rows = run_json(cmd, None)?;
            let fp = rows
                .first()
                .and_then(|r| r.get("fingerprint"))
                .and_then(Value::as_str)
                .ok_or_else(|| format!("{}: no fingerprint", k.id))?;
            out.get_mut(&k.id).unwrap().insert(t, fp.to_string());
        }
    }
    Ok(out)
}

/// (kernels with one fingerprint at every thread count, kernels without: per thread count)
pub fn split_thread_dependence(
    found: &Found,
) -> (
    BTreeMap<String, String>,
    BTreeMap<String, BTreeMap<String, String>>,
) {
    let (mut stable, mut dependent) = (BTreeMap::new(), BTreeMap::new());
    for (kid, by_t) in found {
        let distinct: std::collections::BTreeSet<&String> = by_t.values().collect();
        if distinct.len() > 1 {
            dependent.insert(
                kid.clone(),
                by_t.iter()
                    .map(|(t, fp)| (t.to_string(), fp.clone()))
                    .collect(),
            );
        } else if let Some(fp) = by_t.values().next() {
            stable.insert(kid.clone(), fp.clone());
        }
    }
    (stable, dependent)
}

/// What is wrong with the fingerprints found, against the recorded ones.
///
/// A kernel's fingerprint covers its counted units as well as its answer, and
/// a unit such as the multiply-add count can legitimately differ with the
/// thread count (a parallel elimination splits its blocks differently).  Those
/// kernels are listed under `thread_dependent` in the golden file, per thread
/// count; any other kernel must agree across thread counts, and a new
/// dependence is a problem until someone has looked at it and recorded it.
pub fn problems(found: &Found, golden: Option<&Value>) -> Vec<String> {
    let (stable, dependent) = split_thread_dependence(found);
    let obj = |k: &str| {
        golden
            .and_then(|g| g.get(k))
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default()
    };
    let (gold_stable, gold_dep) = (obj("fingerprints"), obj("thread_dependent"));
    let mut bad = Vec::new();
    for (kid, by_t) in &dependent {
        if golden.is_none() {
            bad.push(format!(
                "{kid}: fingerprint depends on the thread count {by_t:?}"
            ));
        } else if let Some(rec) = gold_dep.get(kid) {
            for (t, fp) in by_t {
                match rec.get(t).and_then(Value::as_str) {
                    None => bad.push(format!("{kid}: no recorded fingerprint at {t} thread(s)")),
                    Some(want) if want != fp => {
                        bad.push(format!("{kid}: {fp} != recorded {want} at {t} thread(s) (the kernel computes something else)"))
                    }
                    _ => {}
                }
            }
        } else {
            bad.push(format!("{kid}: fingerprint depends on the thread count {by_t:?} and is not recorded as such"));
        }
    }
    if golden.is_some() {
        for (kid, fp) in &stable {
            if gold_dep.contains_key(kid) {
                bad.push(format!("{kid}: recorded as thread-dependent but now the same at every thread count: update the record"));
            } else if let Some(want) = gold_stable.get(kid).and_then(Value::as_str) {
                if want != fp {
                    bad.push(format!(
                        "{kid}: {fp} != golden {want} (the kernel computes something else)"
                    ));
                }
            } else {
                bad.push(format!(
                    "{kid}: not in the golden file (a new kernel: record it)"
                ));
            }
        }
        for kid in gold_stable.keys().chain(gold_dep.keys()) {
            if !found.contains_key(kid) {
                bad.push(format!(
                    "{kid}: in the golden file but not in this binary (removed or renamed)"
                ));
            }
        }
    }
    bad.sort();
    bad
}

pub fn run_fingerprints(a: &FingerprintArgs) -> Result<i32, String> {
    let threads: Vec<usize> = a
        .threads
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().map_err(|_| format!("bad thread count {s:?}")))
        .collect::<Result<_, _>>()?;
    if a.update && !a.filter.is_empty() {
        eprintln!("refused: --update needs the whole registry, not a --filter subset");
        return Ok(2);
    }
    let found = collect(&a.bin, &threads, a.full, &a.filter)?;
    let existing = a
        .golden
        .as_deref()
        .filter(|p| p.exists())
        .map(read_json)
        .transpose()?;
    if a.update {
        let path = a.golden.as_deref().ok_or("--update needs --golden")?;
        let (stable, dependent) = split_thread_dependence(&found);
        for (kid, by_t) in &dependent {
            eprintln!("thread-dependent: {kid} {by_t:?}");
        }
        let notes: serde_json::Map<String, Value> = existing
            .as_ref()
            .and_then(|g| g.get("notes"))
            .and_then(Value::as_object)
            .map(|m| {
                m.iter()
                    .filter(|(k, _)| dependent.contains_key(*k))
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            })
            .unwrap_or_default();
        let doc = json!({
            "comment": "Expected perfbench fingerprints (docs/perf/PERFORMANCE_INDEX.md). A kernel's fingerprint changes only when it \
                        computes something else: that is a new kernel ID, not an edit. `fingerprints` holds kernels that agree across \
                        thread counts; `thread_dependent` holds those that do not, per thread count, with the reason in `notes`. \
                        Regenerate with `cargo run --release --example perfindex -- fingerprints --bin BIN --update --golden \
                        docs/perf/fingerprints.json`.",
            "schema": 2,
            "tier": if a.full { "full" } else { "quick" },
            "threads_checked": threads,
            "recorded_with": {"host": host_manifest(true), "binary_sha256": sha256_file(&a.bin).ok()},
            "fingerprints": stable,
            "thread_dependent": dependent,
            "notes": notes,
        });
        write_json(path, &doc)?;
        println!(
            "wrote {}: {} thread-independent, {} thread-dependent kernels",
            path.display(),
            stable.len(),
            dependent.len()
        );
        return Ok(0);
    }
    let bad = problems(&found, existing.as_ref());
    for b in &bad {
        eprintln!("{b}");
    }
    println!(
        "{} kernels, threads {threads:?}, golden {}: {}",
        found.len(),
        if existing.is_some() { "yes" } else { "no" },
        if bad.is_empty() {
            "OK".to_string()
        } else {
            format!("{} problem(s)", bad.len())
        }
    );
    Ok(if bad.is_empty() { 0 } else { 1 })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn found(items: &[(&str, &[(usize, &str)])]) -> Found {
        items
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    v.iter().map(|(t, f)| (*t, f.to_string())).collect(),
                )
            })
            .collect()
    }

    #[test]
    fn problems_name_drift_missing_and_unrecorded_dependence() {
        let f = found(&[
            ("a", &[(1, "aa"), (2, "aa")]),
            ("b", &[(1, "bb"), (2, "cc")]),
            ("c", &[(1, "dd"), (2, "dd")]),
        ]);
        let golden =
            json!({"fingerprints": {"a": "aa", "c": "00", "gone": "ee"}, "thread_dependent": {}});
        let text = problems(&f, Some(&golden)).join("\n");
        assert!(
            text.contains("b: fingerprint depends on the thread count"),
            "{text}"
        );
        assert!(text.contains("not recorded as such"));
        assert!(text.contains("c: dd != golden 00"));
        assert!(text.contains("gone: in the golden file but not in this binary"));
        assert!(!text.contains("a:"));
        assert!(problems(&found(&[("a", &[(1, "aa")])]), None).is_empty());
        assert!(problems(
            &found(&[("n", &[(1, "aa")])]),
            Some(&json!({"fingerprints": {}}))
        )
        .join("")
        .contains("not in the golden file"));
    }

    #[test]
    fn a_recorded_thread_dependence_passes_and_is_pinned() {
        let golden = json!({"fingerprints": {}, "thread_dependent": {"t": {"1": "aa", "2": "bb"}}});
        assert!(problems(&found(&[("t", &[(1, "aa"), (2, "bb")])]), Some(&golden)).is_empty());
        let drift = problems(&found(&[("t", &[(1, "aa"), (2, "cc")])]), Some(&golden));
        assert!(
            drift
                .iter()
                .any(|b| b.contains("cc != recorded bb at 2 thread")),
            "{drift:?}"
        );
        let unrecorded = problems(&found(&[("t", &[(1, "aa"), (4, "dd")])]), Some(&golden));
        assert!(
            unrecorded
                .iter()
                .any(|b| b.contains("no recorded fingerprint at 4")),
            "{unrecorded:?}"
        );
        let stale = problems(&found(&[("t", &[(1, "aa"), (2, "aa")])]), Some(&golden));
        assert!(
            stale.iter().any(|b| b.contains("update the record")),
            "{stale:?}"
        );
    }

    #[test]
    fn split_separates_the_two_kinds() {
        let (stable, dep) = split_thread_dependence(&found(&[
            ("a", &[(1, "x"), (2, "x")]),
            ("b", &[(1, "x"), (2, "y")]),
        ]));
        assert_eq!(stable.get("a").map(String::as_str), Some("x"));
        assert_eq!(dep["b"]["1"], "x");
        assert_eq!(dep["b"]["2"], "y");
        assert!(!stable.contains_key("b"));
    }
}
