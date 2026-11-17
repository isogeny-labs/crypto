//! Read-only crosscheck of the frozen performance-gains page against Git blobs.
//! No benchmark is run, no measurement is produced, and no file is modified.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

type AuditResult<T> = Result<T, String>;

fn require(ok: bool, message: impl Into<String>) -> AuditResult<()> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn git_blob(root: &Path, path: &str) -> AuditResult<Vec<u8>> {
    let out = Command::new("git")
        .current_dir(root)
        .args(["show", &format!("HEAD:{path}")])
        .output()
        .map_err(|e| format!("git show {path}: {e}"))?;
    require(out.status.success(), format!("git show failed for {path}"))?;
    Ok(out.stdout)
}

fn parsed(raw: &[u8], label: &str) -> AuditResult<Value> {
    serde_json::from_slice(raw).map_err(|e| format!("invalid JSON in {label}: {e}"))
}

fn number(row: &Value, key: &str) -> AuditResult<f64> {
    row.get(key)
        .and_then(Value::as_f64)
        .ok_or_else(|| format!("missing numeric {key} in {row}"))
}

fn integer(row: &Value, key: &str) -> AuditResult<i64> {
    row.get(key)
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("missing integer {key} in {row}"))
}

fn string<'a>(row: &'a Value, key: &str) -> AuditResult<&'a str> {
    row.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {key} in {row}"))
}

fn list<'a>(row: &'a Value, key: &str) -> AuditResult<&'a Vec<Value>> {
    row.get(key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing list {key} in {row}"))
}

fn near(actual: f64, expected: f64, label: &str) -> AuditResult<()> {
    require(
        actual.is_finite()
            && expected.is_finite()
            && (actual - expected).abs() <= 1e-11 * actual.abs().max(expected.abs()).max(1.0),
        format!("{label}: computed {actual}, published {expected}"),
    )
}

fn check_pair_rows(pairs: &[Value], label: &str) -> AuditResult<()> {
    for (i, row) in pairs.iter().enumerate() {
        let baseline = number(row, "baseline_s")?;
        let candidate = number(row, "candidate_s")?;
        require(
            baseline > 0.0 && candidate > 0.0,
            format!("{label}[{i}]: nonpositive time"),
        )?;
        near(
            candidate / baseline,
            number(row, "ratio")?,
            &format!("{label}[{i}] ratio"),
        )?;
    }
    Ok(())
}

fn check_aggregates(aggregates: &[Value], pairs: &[Value], local: bool) -> AuditResult<()> {
    for row in aggregates {
        let metric = string(row, "metric")?;
        let variant = string(row, "variant")?;
        let kind = if local {
            Some(string(row, "kind")?)
        } else {
            None
        };
        let selected: Vec<&Value> = pairs
            .iter()
            .filter(|p| {
                p["metric"] == metric
                    && p["variant"] == variant
                    && kind.is_none_or(|k| p["kind"] == k)
            })
            .collect();
        require(
            !selected.is_empty(),
            format!("empty aggregate {metric}/{variant}"),
        )?;
        if local {
            require(
                integer(row, "pairs")? as usize == selected.len(),
                format!("pair count mismatch for {metric}/{variant}"),
            )?;
        }
        let sum: f64 = selected
            .iter()
            .map(|p| number(p, "candidate_s"))
            .collect::<AuditResult<Vec<_>>>()?
            .iter()
            .sum();
        near(
            sum,
            number(row, "sum_s")?,
            &format!("{metric}/{variant} sum"),
        )?;
        let mut clusters: BTreeMap<(i64, i64), Vec<f64>> = BTreeMap::new();
        for p in selected {
            clusters
                .entry((integer(p, "p")?, integer(p, "seed")?))
                .or_default()
                .push(number(p, "ratio")?.ln());
        }
        let mean_log = clusters
            .values()
            .map(|logs| logs.iter().sum::<f64>() / logs.len() as f64)
            .sum::<f64>()
            / clusters.len() as f64;
        let ratio = mean_log.exp();
        near(
            ratio,
            number(row, "ratio")?,
            &format!("{metric}/{variant} cluster ratio"),
        )?;
        let interval = list(row, "paired_95")?;
        require(
            interval.len() == 2,
            format!("bad interval for {metric}/{variant}"),
        )?;
        let lo = interval[0]
            .as_f64()
            .ok_or("nonnumeric interval lower bound")?;
        let hi = interval[1]
            .as_f64()
            .ok_or("nonnumeric interval upper bound")?;
        require(
            lo.is_finite() && hi.is_finite() && lo <= ratio && ratio <= hi,
            format!("interval excludes point estimate for {metric}/{variant}"),
        )?;
    }
    Ok(())
}

fn check_count(root: &Path, run_dir: &str, variant: &str, raw_lines: &[Value]) -> AuditResult<i64> {
    let name = format!("micro-p67-seed11-r0-{variant}.json");
    let row = raw_lines
        .iter()
        .find(|r| r["result"] == name)
        .ok_or_else(|| format!("missing raw run row {name}"))?;
    require(
        integer(row, "returncode")? == 0,
        format!("failed raw run {name}"),
    )?;
    let bytes = git_blob(root, &format!("{run_dir}/{name}"))?;
    let actual_hash = format!("{:x}", Sha256::digest(&bytes));
    require(
        actual_hash == string(row, "result_sha256")?,
        format!("result digest mismatch: {name}"),
    )?;
    let result = parsed(&bytes, &name)?;
    let denominator = integer(&result, "targets")? * integer(&result, "evaluation_repetitions")?;
    require(
        denominator > 0 && integer(&result, "specialization_fp_muls")? % denominator == 0,
        format!("nonintegral per-call count in {name}"),
    )?;
    Ok(integer(&result, "specialization_fp_muls")? / denominator)
}

fn audit(root: &Path, data_path: &Path, html_path: &Path) -> AuditResult<Value> {
    let board = parsed(
        &fs::read(data_path).map_err(|e| e.to_string())?,
        "dashboard data",
    )?;
    let hashes = board["sourceHashes"]
        .as_object()
        .ok_or("missing sourceHashes")?;
    let paths = [
        "research/summation_polynomials_20260914/run-01/comparison.json",
        "research/summation_polynomials_20260914/confirmation-01/comparison.json",
        "research/gaudry_allocation_20260914/run-01/comparison.json",
        "gpu/macaulay/benchmarks/pipeline-20260914/run-01/summary.json",
    ];
    require(hashes.len() == paths.len(), "unexpected source hash set")?;
    let mut source = Vec::new();
    for path in paths {
        let bytes = git_blob(root, path)?;
        let actual = format!("{:x}", Sha256::digest(&bytes));
        require(
            hashes.get(path).and_then(Value::as_str) == Some(actual.as_str()),
            format!("source digest mismatch: {path}"),
        )?;
        source.push(parsed(&bytes, path)?);
    }
    for (board_key, source_idx, source_key) in [
        ("local", 0, "aggregates"),
        ("localPairs", 0, "pairs"),
        ("confirmation", 1, "aggregates"),
        ("confirmationPairs", 1, "pairs"),
        ("history", 2, "aggregates"),
        ("gpu", 3, "rows"),
    ] {
        require(
            board[board_key] == source[source_idx][source_key],
            format!("dashboard {board_key} differs from frozen source"),
        )?;
    }
    for (idx, expected_runs) in [(0, 342), (1, 140)] {
        require(
            source[idx]["complete"] == true
                && list(&source[idx], "failures")?.is_empty()
                && integer(&source[idx], "runs")? == expected_runs,
            format!("frozen comparison {idx} is incomplete"),
        )?;
    }
    require(
        source[2]["complete"] == true && list(&source[2], "failures")?.is_empty(),
        "allocation comparison is incomplete",
    )?;
    require(
        source[3]["complete"] == true && integer(&source[3], "failed_runs")? == 0,
        "GPU summary is incomplete",
    )?;
    let local_pairs = list(&board, "localPairs")?;
    let confirmation_pairs = list(&board, "confirmationPairs")?;
    require(
        local_pairs.len() == 486 && confirmation_pairs.len() == 280,
        "unexpected pair populations",
    )?;
    check_pair_rows(local_pairs, "localPairs")?;
    check_pair_rows(confirmation_pairs, "confirmationPairs")?;
    check_aggregates(list(&board, "local")?, local_pairs, true)?;
    check_aggregates(list(&board, "confirmation")?, confirmation_pairs, false)?;

    let html = fs::read_to_string(html_path).map_err(|e| e.to_string())?;
    let embedded = html
        .lines()
        .find_map(|line| {
            line.strip_prefix("const D=")
                .and_then(|s| s.strip_suffix(';'))
        })
        .ok_or("missing embedded dashboard data")?;
    require(
        parsed(embedded.as_bytes(), "embedded HTML data")? == board,
        "HTML embedded data differs from data.json",
    )?;

    let run_dir = "research/summation_polynomials_20260914/run-01";
    let raw = git_blob(root, &format!("{run_dir}/raw.jsonl"))?;
    let raw_text = std::str::from_utf8(&raw).map_err(|e| e.to_string())?;
    let raw_lines = raw_text
        .lines()
        .map(|line| parsed(line.as_bytes(), "raw.jsonl"))
        .collect::<AuditResult<Vec<_>>>()?;
    let reference = check_count(root, run_dir, "reference", &raw_lines)?;
    let horner = check_count(root, run_dir, "horner", &raw_lines)?;
    require(
        board["counts"] == json!([reference, horner]),
        "arithmetic counts differ",
    )?;
    let source_rows = [
        "local",
        "localPairs",
        "confirmation",
        "confirmationPairs",
        "history",
        "gpu",
    ]
    .iter()
    .map(|key| list(&board, key).map(Vec::len))
    .collect::<AuditResult<Vec<_>>>()?
    .iter()
    .sum::<usize>();
    require(source_rows == 804, "unexpected source-row population")?;
    let intervals = list(&board, "local")?.len() + list(&board, "confirmation")?.len();
    require(intervals == 32, "unexpected interval population")?;
    Ok(json!({
        "status": "pass", "source_hashes_verified": paths.len(),
        "rows_exact_from_sources": source_rows,
        "pair_ratios_recomputed": local_pairs.len() + confirmation_pairs.len(),
        "cluster_ratios_recomputed": intervals,
        "source_intervals_present": intervals,
        "frozen_runs": {"ablation": 342, "confirmation": 140},
        "fp_multiplications_per_specialization": {"reference": reference, "horner": horner},
        "html_embedded_data_exact": true,
        "scope": "frozen 2026-09-14 performance-gains page; no new measurements"
    }))
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (data, html) = match args.as_slice() {
        [] => (
            root.join("docs/performance-gains/data.json"),
            root.join("docs/performance-gains.html"),
        ),
        [data, html] => (PathBuf::from(data), PathBuf::from(html)),
        _ => {
            eprintln!("usage: performance_gains_audit [DATA_JSON HTML]");
            std::process::exit(2);
        }
    };
    match audit(&root, &data, &html) {
        Ok(result) => println!(
            "{}",
            serde_json::to_string_pretty(&result).expect("JSON output")
        ),
        Err(error) => {
            eprintln!("FAIL: {error}");
            std::process::exit(1);
        }
    }
}
