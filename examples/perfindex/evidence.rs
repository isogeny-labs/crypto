//! `replay` and `bundle`: recomputing a recorded run from its raw rows, and
//! packaging one that replays for a reviewer.

use crate::compare::{make_schedule, summarize_from_plan, Plan};
use crate::summarize::markdown;
use crate::util::{read_json, read_jsonl, sha256_file, write_json};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::Path;

/// The fields of `compare.json` that follow from the raw rows by the formula
/// (the others describe the host and the cost, which a replay cannot know).
const STABLE_KEYS: [&str; 8] = [
    "valid",
    "performance_index",
    "performance_index_ci95",
    "areas",
    "kernels",
    "rounds",
    "aa_noise_median_pct",
    "exclude_contended",
];

/// Equality that tolerates the last digits of a float, nothing else.
pub fn close(x: &Value, y: &Value, tol: f64) -> bool {
    match (x, y) {
        (Value::Number(a), Value::Number(b)) => match (a.as_f64(), b.as_f64()) {
            (Some(p), Some(q)) => p == q || (p - q).abs() <= tol.max(tol * p.abs().max(q.abs())),
            _ => a == b,
        },
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(k, v)| b.get(k).is_some_and(|w| close(v, w, tol)))
        }
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(p, q)| close(p, q, tol))
        }
        _ => x == y,
    }
}

/// Recomputes `compare.json` from `plan.json` and `raw.jsonl`.  What it
/// checks is the statistics: that the reported numbers follow from the raw
/// rows by the documented formula, and that every planned invocation is there
/// exactly once.  It does not re-time anything.
pub fn replay_dir(dir: &Path) -> Result<(bool, Vec<String>), String> {
    let plan: Plan = serde_json::from_value(read_json(&dir.join("plan.json"))?)
        .map_err(|e| format!("plan.json: {e}"))?;
    let raw = read_jsonl(&dir.join("raw.jsonl"))?;
    let told = read_json(&dir.join("compare.json"))?;
    let mut again = summarize_from_plan(&plan, &raw);
    if raw.iter().any(|r| r.get("row_problems").is_some()) {
        again.valid = false;
        again.performance_index = None;
        again.performance_index_ci95 = None;
    }
    let again = serde_json::to_value(&again).map_err(|e| e.to_string())?;
    let mut diffs: Vec<String> = Vec::new();
    for key in STABLE_KEYS {
        let (a, b) = (
            told.get(key).unwrap_or(&Value::Null),
            again.get(key).unwrap_or(&Value::Null),
        );
        if !close(a, b, 1e-9) {
            diffs.push(key.to_string());
        }
    }
    let ids: Vec<String> = plan.kernels.iter().map(|k| k.id.clone()).collect();
    let planned: BTreeSet<(usize, String, String)> =
        make_schedule(&ids, plan.rounds, plan.aa, plan.seed)
            .into_iter()
            .map(|i| (i.round, i.kernel, i.arm.name().to_string()))
            .collect();
    let got: Vec<(usize, String, String)> = raw
        .iter()
        .filter(|r| r.get("type").and_then(Value::as_str) == Some("invocation"))
        .map(|r| {
            (
                r["round"].as_u64().unwrap_or(u64::MAX) as usize,
                r["kernel"].as_str().unwrap_or("").to_string(),
                r["arm"].as_str().unwrap_or("").to_string(),
            )
        })
        .collect();
    let got_set: BTreeSet<_> = got.iter().cloned().collect();
    if got.len() != got_set.len() {
        diffs.push("raw.jsonl repeats an invocation".into());
    }
    let stray = got_set.difference(&planned).count();
    if stray > 0 {
        diffs.push(format!(
            "raw.jsonl holds {stray} invocation(s) outside the plan"
        ));
    }
    let stopped = told.get("stopped_by_budget").and_then(Value::as_bool) == Some(true);
    let missing = planned.difference(&got_set).count();
    if !stopped && missing > 0 {
        diffs.push(format!(
            "raw.jsonl is missing {missing} planned invocation(s)"
        ));
    }
    Ok((diffs.is_empty(), diffs))
}

pub fn run_replay(dir: &Path) -> Result<i32, String> {
    let (ok, diffs) = replay_dir(dir)?;
    if ok {
        println!(
            "replay MATCH: {}/compare.json follows from plan.json and raw.jsonl",
            dir.display()
        );
        Ok(0)
    } else {
        eprintln!("replay MISMATCH in: {diffs:?}");
        Ok(1)
    }
}

pub fn run_bundle(dir: &Path, dest: &Path, max_mb: f64) -> Result<i32, String> {
    let (ok, diffs) = replay_dir(dir)?;
    if !ok {
        eprintln!("refused: the run does not replay ({diffs:?})");
        return Ok(1);
    }
    if dest.exists() {
        eprintln!("refused: {} exists", dest.display());
        return Ok(2);
    }
    let names = [
        "plan.json",
        "raw.jsonl",
        "compare.json",
        "compare.md",
        "cost.json",
    ];
    let size: u64 = names
        .iter()
        .filter_map(|n| std::fs::metadata(dir.join(n)).ok())
        .map(|m| m.len())
        .sum();
    if size as f64 > max_mb * 1e6 {
        eprintln!(
            "refused: {:.1} MB exceeds --max-mb {max_mb}",
            size as f64 / 1e6
        );
        return Ok(2);
    }
    std::fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    let mut sums = String::new();
    for n in names {
        if dir.join(n).exists() {
            std::fs::copy(dir.join(n), dest.join(n)).map_err(|e| e.to_string())?;
            sums.push_str(&format!(
                "{}  {n}\n",
                sha256_file(&dest.join(n)).map_err(|e| e.to_string())?
            ));
        }
    }
    std::fs::write(dest.join("SHA256SUMS"), sums).map_err(|e| e.to_string())?;
    let plan = read_json(&dir.join("plan.json"))?;
    let told = read_json(&dir.join("compare.json"))?;
    let short = |v: &Value, k: &str| {
        v.get(k)
            .and_then(Value::as_str)
            .map_or("?".to_string(), |s| s[..s.len().min(16)].to_string())
    };
    let rev = |side: &str| {
        plan[side]
            .get("rev")
            .and_then(Value::as_str)
            .unwrap_or("?")
            .to_string()
    };
    let pi = told.get("performance_index").and_then(Value::as_f64);
    let mut readme = vec![
        format!(
            "# perfbench evidence: {}",
            dest.file_name()
                .map_or(String::new(), |n| n.to_string_lossy().into_owned())
        ),
        String::new(),
        format!(
            "* base      `{}` (binary `{}`)",
            rev("base"),
            short(&plan["base"], "binary_sha256")
        ),
        format!(
            "* candidate `{}` (binary `{}`)",
            rev("cand"),
            short(&plan["cand"], "binary_sha256")
        ),
        format!(
            "* threads {}, {} planned rounds, A/A {}, seed {}, schedule `{}`",
            plan["threads"],
            plan["rounds"],
            if plan["aa"].as_bool() == Some(true) {
                "on"
            } else {
                "OFF"
            },
            plan["seed"],
            plan["schedule_hash"].as_str().unwrap_or("?")
        ),
        format!(
            "* performance index: {}; qualified: {}; valid: {}",
            pi.map_or("refused".to_string(), |p| format!("{p:.3}x")),
            told["qualified"],
            told["valid"]
        ),
        String::new(),
        "Warnings recorded with the run:".into(),
    ];
    let warns: Vec<String> = told["warnings"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(|w| format!("* {w}"))
                .collect()
        })
        .unwrap_or_default();
    if warns.is_empty() {
        readme.push("* none".into());
    } else {
        readme.extend(warns);
    }
    readme.extend([
        String::new(),
        "Check that the numbers follow from the raw rows (this re-times nothing):".into(),
        String::new(),
        "```bash".into(),
        "sha256sum -c SHA256SUMS".into(),
        "cargo run --release --example perfindex -- replay .".into(),
        "```".into(),
        String::new(),
        "`raw.jsonl` holds every harness row exactly as the harness printed it, with the machine's".into(),
        "load and pressure sampled at each round's start and end.  Wall-time rows are evidence only".into(),
        "when uncontended (AGENTS.md section 10); `compare.json` says how many rounds were left out.".into(),
        "These are host-specific observations, not a claim about another host class.".into(),
    ]);
    std::fs::write(dest.join("README.md"), readme.join("\n") + "\n").map_err(|e| e.to_string())?;
    println!(
        "bundle written: {} ({:.2} MB), replay MATCH",
        dest.display(),
        size as f64 / 1e6
    );
    Ok(0)
}

/// Thresholds `reanalyze` may change.  The raw rows never change.
#[derive(clap::Args, Debug)]
pub struct ReanalyzeArgs {
    /// Pool contended rounds instead of leaving them out
    #[arg(long)]
    pub keep_contended: bool,
    /// Involuntary-switch limit per second of sampling (default: unchanged)
    #[arg(long)]
    pub max_invol_per_s: Option<f64>,
    /// Drop the involuntary-switch limit
    #[arg(long)]
    pub no_invol_limit: bool,
    #[arg(long)]
    pub min_cpu_ratio: Option<f64>,
    #[arg(long)]
    pub max_steal: Option<f64>,
    /// Why the thresholds were changed (stored in plan.json)
    #[arg(long, default_value = "")]
    pub note: String,
}

/// Apply different contention thresholds to a recorded run.  `raw.jsonl` is
/// copied byte for byte; the new `plan.json` says which run it came from (by
/// hash) and what changed, and `compare.json` follows from the two, so the
/// result passes `replay`.  The old run stays as it was.
pub fn run_reanalyze(dir: &Path, out: &Path, a: &ReanalyzeArgs) -> Result<i32, String> {
    if out.exists() {
        eprintln!("refused: {} exists", out.display());
        return Ok(2);
    }
    let mut plan: Plan = serde_json::from_value(read_json(&dir.join("plan.json"))?)
        .map_err(|e| format!("plan.json: {e}"))?;
    let raw = read_jsonl(&dir.join("raw.jsonl"))?;
    let told = read_json(&dir.join("compare.json"))?;
    let before = json!({
        "exclude_contended": plan.exclude_contended,
        "min_cpu_ratio": plan.min_cpu_ratio,
        "max_invol_per_s": plan.max_invol_per_s,
        "max_steal": plan.max_steal,
    });
    if a.keep_contended {
        plan.exclude_contended = false;
    }
    if a.no_invol_limit {
        plan.max_invol_per_s = None;
    }
    if let Some(v) = a.max_invol_per_s {
        plan.max_invol_per_s = Some(v);
    }
    if let Some(v) = a.min_cpu_ratio {
        plan.min_cpu_ratio = v;
    }
    if let Some(v) = a.max_steal {
        plan.max_steal = v;
    }
    let after = json!({
        "exclude_contended": plan.exclude_contended,
        "min_cpu_ratio": plan.min_cpu_ratio,
        "max_invol_per_s": plan.max_invol_per_s,
        "max_steal": plan.max_steal,
    });
    plan.reanalysis = Some(json!({
        "from_plan_sha256": sha256_file(&dir.join("plan.json")).map_err(|e| e.to_string())?,
        "from_raw_sha256": sha256_file(&dir.join("raw.jsonl")).map_err(|e| e.to_string())?,
        "from_compare_sha256": sha256_file(&dir.join("compare.json")).map_err(|e| e.to_string())?,
        "thresholds_before": before,
        "thresholds_after": after,
        "note": a.note,
    }));
    let mut summary = summarize_from_plan(&plan, &raw);
    if raw.iter().any(|r| r.get("row_problems").is_some()) {
        summary.valid = false;
        summary.performance_index = None;
        summary.performance_index_ci95 = None;
        summary.qualified = false;
    }
    if told.get("stopped_by_budget").and_then(Value::as_bool) == Some(true)
        || !plan.provenance_problems.is_empty()
        || !plan.only_base.is_empty()
        || !plan.only_cand.is_empty()
    {
        summary.qualified = false;
    }
    for key in [
        "schema",
        "mode",
        "unit",
        "base",
        "cand",
        "threads",
        "samples_per_invocation",
        "schedule_hash",
        "stopped_by_budget",
        "host",
        "cost",
    ] {
        if let Some(v) = told.get(key) {
            summary.extra.insert(key.into(), v.clone());
        }
    }
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    std::fs::copy(dir.join("raw.jsonl"), out.join("raw.jsonl")).map_err(|e| e.to_string())?;
    if dir.join("cost.json").exists() {
        std::fs::copy(dir.join("cost.json"), out.join("cost.json")).map_err(|e| e.to_string())?;
    }
    write_json(
        &out.join("plan.json"),
        &serde_json::to_value(&plan).map_err(|e| e.to_string())?,
    )?;
    write_json(
        &out.join("compare.json"),
        &serde_json::to_value(&summary).map_err(|e| e.to_string())?,
    )?;
    let md = markdown(&summary, "perfbench wall-time comparison (reanalysis)");
    std::fs::write(out.join("compare.md"), &md).map_err(|e| e.to_string())?;
    print!("{md}");
    eprintln!(
        "reanalysis written to {}; thresholds {before} -> {after}",
        out.display()
    );
    Ok(if summary.valid { 0 } else { 1 })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn close_tolerates_last_digits_only() {
        assert!(close(&json!(1.0), &json!(1.0 + 1e-12), 1e-9));
        assert!(!close(&json!(1.0), &json!(1.001), 1e-9));
        assert!(close(
            &json!({"a": [1, 2.0]}),
            &json!({"a": [1.0, 2]}),
            1e-9
        ));
        assert!(!close(&json!({"a": 1}), &json!({"a": 1, "b": 2}), 1e-9));
        assert!(!close(&json!(null), &json!(0), 1e-9));
        assert!(close(&json!(null), &json!(null), 1e-9));
        assert!(!close(&json!("x"), &json!("y"), 1e-9));
    }

    // ---- end to end, against fake benchmark binaries (shell scripts) ----

    use crate::compare::{run_compare, CompareArgs};
    use crate::util::tests_support::tempdir;
    use clap::Parser;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;

    #[derive(Parser)]
    struct Wrap {
        #[command(flatten)]
        args: CompareArgs,
    }

    /// A stand-in for perfbench: `list` prints two kernels, `run` prints one
    /// row whose median is `ax_ns` for `a/x` and one millisecond otherwise.
    fn fake_bench(dir: &Path, name: &str, ax_ns: u64) -> PathBuf {
        let path = dir.join(name);
        let script = format!(
            r#"#!/bin/sh
# {name} {ax_ns}
case "$1" in
  list)
    echo '{{"id":"a/x","area":"a"}}'
    echo '{{"id":"b/y","area":"b"}}'
    ;;
  run)
    id="$3"
    case "$id" in a/x) ns={ax_ns};; *) ns=1000000;; esac
    echo "{{\"id\":\"$id\",\"area\":\"${{id%%/*}}\",\"fingerprint\":\"00000000000000aa\",\"median_ns\":$ns,\"min_ns\":$ns,\"samples_ns\":[$ns],\"median_cpu_ns\":$ns,\"threads\":${{RAYON_NUM_THREADS:-1}}}}"
    ;;
esac
"#
        );
        std::fs::write(&path, script).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    struct Fixture {
        dir: PathBuf,
        base: PathBuf,
        cand: PathBuf,
        weights: PathBuf,
    }

    fn fixture(tag: &str) -> Fixture {
        let dir = tempdir(tag);
        let weights = dir.join("weights.json");
        std::fs::write(&weights, r#"{"a": 1.0, "b": 1.0}"#).unwrap();
        Fixture {
            base: fake_bench(&dir, "base", 1_000_000),
            cand: fake_bench(&dir, "cand", 500_000),
            weights,
            dir,
        }
    }

    fn args(f: &Fixture, extra: &[&str]) -> CompareArgs {
        // steal is a property of the machine the test runs on, so switch it off
        let mut v = vec![
            "x",
            "--rounds",
            "3",
            "--samples",
            "1",
            "--max-steal",
            "1000000",
            "--allow-provenance-mismatch",
            "--weights",
            f.weights.to_str().unwrap(),
        ];
        v.extend_from_slice(extra);
        Wrap::parse_from(v).args
    }

    fn invocations(dir: &Path) -> usize {
        read_jsonl(&dir.join("raw.jsonl"))
            .unwrap()
            .iter()
            .filter(|r| r.get("type").and_then(Value::as_str) == Some("invocation"))
            .count()
    }

    #[test]
    fn a_run_replays_and_an_edited_row_does_not() {
        let f = fixture("replay");
        let out = f.dir.join("run");
        assert_eq!(
            run_compare(&f.base, &f.cand, &out, &args(&f, &[])).unwrap(),
            0
        );
        assert_eq!(invocations(&out), 2 * 3 * 3); // kernels x rounds x (A, B, A2)
        let told = read_json(&out.join("compare.json")).unwrap();
        // a/x is twice as fast and b/y unchanged: the index is the geometric mean, sqrt(2)
        let pi = told["performance_index"].as_f64().unwrap();
        assert!((pi - 2f64.sqrt()).abs() < 1e-9, "{pi}");
        assert_eq!(told["kernels"][0]["verdict"], "faster");
        assert_eq!(told["kernels"][1]["verdict"], "neutral");
        assert_eq!(replay_dir(&out).unwrap(), (true, vec![]));

        let raw = std::fs::read_to_string(out.join("raw.jsonl")).unwrap();
        let edited = raw.replacen("\"median_ns\":500000", "\"median_ns\":500001", 1);
        assert_ne!(raw, edited);
        std::fs::write(out.join("raw.jsonl"), &edited).unwrap();
        let (ok, diffs) = replay_dir(&out).unwrap();
        assert!(!ok && !diffs.is_empty());
        // and a bundle of the edited run is refused
        assert_eq!(run_bundle(&out, &f.dir.join("bundle"), 25.0).unwrap(), 1);
        assert!(!f.dir.join("bundle").exists());
    }

    #[test]
    fn a_missing_or_repeated_invocation_fails_the_replay() {
        let f = fixture("complete");
        let out = f.dir.join("run");
        run_compare(&f.base, &f.cand, &out, &args(&f, &[])).unwrap();
        let raw = std::fs::read_to_string(out.join("raw.jsonl")).unwrap();
        let lines: Vec<&str> = raw.lines().collect();
        let k = lines
            .iter()
            .position(|l| l.contains("\"type\":\"invocation\""))
            .unwrap();
        let dropped: Vec<&str> = lines
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != k)
            .map(|(_, l)| *l)
            .collect();
        std::fs::write(out.join("raw.jsonl"), dropped.join("\n") + "\n").unwrap();
        let (ok, diffs) = replay_dir(&out).unwrap();
        assert!(!ok);
        assert!(diffs.iter().any(|d| d.contains("missing")), "{diffs:?}");
        let doubled = [lines.clone(), vec![lines[k]]].concat();
        std::fs::write(out.join("raw.jsonl"), doubled.join("\n") + "\n").unwrap();
        let (ok, diffs) = replay_dir(&out).unwrap();
        assert!(!ok);
        assert!(diffs.iter().any(|d| d.contains("repeats")), "{diffs:?}");
    }

    #[test]
    fn bundle_copies_a_replaying_run_with_checksums() {
        let f = fixture("bundle");
        let out = f.dir.join("run");
        run_compare(&f.base, &f.cand, &out, &args(&f, &[])).unwrap();
        let dest = f.dir.join("bundle");
        assert_eq!(run_bundle(&out, &dest, 25.0).unwrap(), 0);
        for n in [
            "plan.json",
            "raw.jsonl",
            "compare.json",
            "compare.md",
            "cost.json",
            "SHA256SUMS",
            "README.md",
        ] {
            assert!(dest.join(n).exists(), "{n}");
        }
        let sums = std::fs::read_to_string(dest.join("SHA256SUMS")).unwrap();
        let want = sha256_file(&dest.join("raw.jsonl")).unwrap();
        assert!(sums.contains(&format!("{want}  raw.jsonl")), "{sums}");
        assert_eq!(replay_dir(&dest).unwrap(), (true, vec![]));
        // an existing destination is never overwritten, and a size limit refuses
        assert_eq!(run_bundle(&out, &dest, 25.0).unwrap(), 2);
        assert_eq!(run_bundle(&out, &f.dir.join("small"), 0.000001).unwrap(), 2);
        assert!(!f.dir.join("small").exists());
    }

    #[test]
    fn a_budget_stop_keeps_what_was_measured_and_resume_finishes_the_same_plan() {
        let f = fixture("resume");
        let out = f.dir.join("run");
        run_compare(
            &f.base,
            &f.cand,
            &out,
            &args(&f, &["--budget-seconds", "0.0001"]),
        )
        .unwrap();
        assert_eq!(
            invocations(&out),
            2 * 3,
            "the first round always runs, then the budget stops it"
        );
        let told = read_json(&out.join("compare.json")).unwrap();
        assert_eq!(told["stopped_by_budget"], true);
        assert_eq!(told["qualified"], false);
        assert_eq!(
            replay_dir(&out).unwrap(),
            (true, vec![]),
            "a budget stop is a replayable partial run"
        );
        // without --resume an existing directory is refused and left alone
        assert_eq!(
            run_compare(&f.base, &f.cand, &out, &args(&f, &[])).unwrap(),
            2
        );
        assert_eq!(invocations(&out), 2 * 3);
        // resume completes the same plan
        run_compare(&f.base, &f.cand, &out, &args(&f, &["--resume"])).unwrap();
        assert_eq!(invocations(&out), 2 * 3 * 3);
        assert_eq!(
            read_json(&out.join("compare.json")).unwrap()["stopped_by_budget"],
            false
        );
        assert_eq!(replay_dir(&out).unwrap(), (true, vec![]));
        // resuming against different binaries is refused
        let other = fake_bench(&f.dir, "other", 400_000);
        assert_eq!(
            run_compare(&f.base, &other, &out, &args(&f, &["--resume"])).unwrap(),
            2
        );
    }

    #[test]
    fn reanalysis_keeps_the_rows_names_its_origin_and_replays() {
        let f = fixture("reanalyze");
        let out = f.dir.join("run");
        run_compare(&f.base, &f.cand, &out, &args(&f, &[])).unwrap();
        let again = f.dir.join("again");
        let a = ReanalyzeArgs {
            keep_contended: true,
            max_invol_per_s: Some(50.0),
            no_invol_limit: false,
            min_cpu_ratio: None,
            max_steal: None,
            note: "test".into(),
        };
        assert_eq!(run_reanalyze(&out, &again, &a).unwrap(), 0);
        assert_eq!(
            std::fs::read(out.join("raw.jsonl")).unwrap(),
            std::fs::read(again.join("raw.jsonl")).unwrap(),
            "the raw rows are copied byte for byte"
        );
        let plan = read_json(&again.join("plan.json")).unwrap();
        let r = &plan["reanalysis"];
        assert_eq!(
            r["from_plan_sha256"],
            sha256_file(&out.join("plan.json")).unwrap()
        );
        assert_eq!(
            r["from_raw_sha256"],
            sha256_file(&out.join("raw.jsonl")).unwrap()
        );
        assert_eq!(r["thresholds_before"]["exclude_contended"], true);
        assert_eq!(r["thresholds_after"]["exclude_contended"], false);
        assert_eq!(r["thresholds_after"]["max_invol_per_s"], 50.0);
        assert_eq!(plan["exclude_contended"], false);
        assert_eq!(replay_dir(&again).unwrap(), (true, vec![]));
        // the original is untouched, and an existing destination is refused
        assert_eq!(
            read_json(&out.join("plan.json")).unwrap().get("reanalysis"),
            None
        );
        assert_eq!(run_reanalyze(&out, &again, &a).unwrap(), 2);
    }
}
