//! Native deterministic control-plane validator for GHS rho lanes R00-R16.
//!
//! This deliberately reads only the flat JSONL records emitted by the
//! standalone GHS comparator.  It checks mathematical/control invariants and
//! never reads or interprets development clocks.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::process;

type Row = BTreeMap<String, String>;

fn parse_flat_object(line: &str) -> Result<Row, String> {
    let body = line
        .trim()
        .strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))
        .ok_or_else(|| "record is not a JSON object".to_string())?;
    let mut fields = Vec::new();
    let mut start = 0usize;
    let mut quoted = false;
    let mut escaped = false;
    for (i, ch) in body.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' if quoted => escaped = true,
            '"' => quoted = !quoted,
            ',' if !quoted => {
                fields.push(&body[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    if quoted || escaped {
        return Err("unterminated JSON string".to_string());
    }
    fields.push(&body[start..]);

    let mut row = Row::new();
    for field in fields {
        let (raw_key, raw_value) = field
            .split_once(':')
            .ok_or_else(|| format!("field lacks colon: {field}"))?;
        let key = raw_key
            .trim()
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .ok_or_else(|| format!("key is not quoted: {raw_key}"))?;
        let raw_value = raw_value.trim();
        let value = if raw_value.starts_with('"') {
            raw_value
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .ok_or_else(|| format!("unterminated value for key {key}"))?
                .to_string()
        } else {
            raw_value.to_string()
        };
        if row.insert(key.to_string(), value).is_some() {
            return Err(format!("duplicate key {key}"));
        }
    }
    Ok(row)
}

fn get<'a>(row: &'a Row, key: &str) -> Result<&'a str, String> {
    row.get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("missing key {key}"))
}

fn parse_u64(row: &Row, key: &str) -> Result<u64, String> {
    get(row, key)?
        .parse::<u64>()
        .map_err(|e| format!("invalid u64 at {key}: {e}"))
}

fn rho<'a>(rows: &'a [Row], arm: &str) -> Result<&'a Row, String> {
    let found: Vec<&Row> = rows
        .iter()
        .filter(|row| row.get("rec").map(String::as_str) == Some("rho"))
        .filter(|row| row.get("arm").map(String::as_str) == Some(arm))
        .collect();
    if found.len() != 1 {
        return Err(format!(
            "expected one rho row for {arm}, got {}",
            found.len()
        ));
    }
    Ok(found[0])
}

fn field_zero(row: &Row) -> Result<bool, String> {
    for (key, value) in row {
        let Some((phase, counter)) = key.split_once('.') else {
            continue;
        };
        let field_counter = matches!(phase, "setup" | "walk" | "verify")
            && (counter.starts_with("fq_")
                || counter.starts_with("f3_")
                || matches!(counter, "batch_inv" | "group_add"));
        if field_counter && value.parse::<u64>().map_err(|e| format!("{key}: {e}"))? != 0 {
            return Ok(false);
        }
    }
    Ok(true)
}

const EXECUTION_FIELDS: &[&str] = &[
    "batch",
    "setup_mode",
    "q",
    "seed",
    "i",
    "target_kind",
    "ell",
    "k",
    "k_found",
    "ok",
    "steps",
    "dps",
    "restarts",
    "lookahead",
    "cycles",
    "cycle_adds",
    "setup.reciprocal_div",
    "setup_native.rng",
    "setup_native.hash",
    "setup_native.canon",
    "setup_native.scalar_add",
    "setup_native.band_ops",
    "walk_native.rng",
    "walk_native.hash",
    "walk_native.canon",
    "walk_native.lookahead_test",
    "walk_native.dp",
    "walk_native.table",
    "walk_native.cycle_cmp",
    "walk_native.scalar_add",
    "walk_native.scalar_mul",
    "walk_native.scalar_inv",
    "walk_native.band_ops",
];

fn same_execution(a: &Row, b: &Row) -> Result<(), String> {
    for key in EXECUTION_FIELDS {
        if get(a, key)? != get(b, key)? {
            return Err(format!("execution mismatch at {key}"));
        }
    }
    Ok(())
}

fn require_count_vector(row: &Row, keys: &[&str], expected: &[u64]) -> Result<(), String> {
    if keys.len() != expected.len() {
        return Err("internal count-vector length mismatch".to_string());
    }
    for (key, want) in keys.iter().zip(expected) {
        let got = parse_u64(row, key)?;
        if got != *want {
            return Err(format!("{key}: expected {want}, got {got}"));
        }
    }
    Ok(())
}

fn validate(rows: &[Row]) -> Result<(), String> {
    let instances: Vec<&Row> = rows
        .iter()
        .filter(|row| row.get("rec").map(String::as_str) == Some("instance"))
        .collect();
    if instances.len() != 1 {
        return Err(format!(
            "expected one instance row, got {}",
            instances.len()
        ));
    }
    if get(instances[0], "source_revision")? != "d85752a9a0" {
        return Err("source revision binding changed".to_string());
    }

    let rho_rows: Vec<&Row> = rows
        .iter()
        .filter(|row| row.get("rec").map(String::as_str) == Some("rho"))
        .collect();
    if rho_rows.len() != 12 {
        return Err(format!("expected 12 rho rows, got {}", rho_rows.len()));
    }
    for row in &rho_rows {
        if get(row, "ok")? != "true" || parse_u64(row, "k")? != parse_u64(row, "k_found")? {
            return Err(format!("known-answer failure for {}", get(row, "arm")?));
        }
        if parse_u64(row, "setup.reciprocal_div")? != 1 {
            return Err(format!(
                "reciprocal setup authority failure for {}",
                get(row, "arm")?
            ));
        }
    }

    for width in [1u64, 4, 8, 16, 32] {
        let counted = rho(rows, &format!("strong-b{width}"))?;
        let fast = rho(rows, &format!("fast-b{width}"))?;
        if get(counted, "count_authoritative")? != "true" {
            return Err(format!("strong-b{width} is not authoritative"));
        }
        if get(fast, "count_authoritative")? != "false" {
            return Err(format!("fast-b{width} incorrectly claims authority"));
        }
        if !field_zero(fast)? {
            return Err(format!("fast-b{width} emitted a nonzero field counter"));
        }
        same_execution(counted, fast).map_err(|e| format!("width {width}: {e}"))?;
    }

    let strong_b8 = rho(rows, "strong-b8")?;
    const COUNT_KEYS: &[&str] = &[
        "setup.fq_mul",
        "setup.fq_inv",
        "setup.fq_add",
        "setup.f3_mul",
        "setup.f3_sq",
        "setup.f3_inv",
        "setup.batch_inv",
        "setup.group_add",
        "walk.fq_mul",
        "walk.fq_inv",
        "walk.fq_add",
        "walk.f3_mul",
        "walk.f3_sq",
        "walk.f3_inv",
        "walk.batch_inv",
        "walk.group_add",
        "verify.fq_mul",
        "verify.fq_inv",
        "verify.fq_add",
        "verify.f3_mul",
        "verify.f3_sq",
        "verify.f3_inv",
        "verify.batch_inv",
        "verify.group_add",
    ];
    require_count_vector(
        strong_b8,
        COUNT_KEYS,
        &[
            56_484, 511, 204_142, 6_437, 2_346, 511, 33, 1_455, 52_861, 241, 192_668, 7_034, 1_553,
            241, 231, 1_551, 1_090, 30, 3_466, 80, 50, 30, 0, 31,
        ],
    )?;

    let legacy = rho(rows, "strong-b8-legacy-setup")?;
    let fast_legacy = rho(rows, "fast-b8-legacy-setup")?;
    if get(legacy, "count_authoritative")? != "true"
        || get(fast_legacy, "count_authoritative")? != "false"
        || !field_zero(fast_legacy)?
    {
        return Err("legacy authority or zero-field-count invariant failed".to_string());
    }
    same_execution(legacy, fast_legacy).map_err(|e| format!("legacy: {e}"))?;
    require_count_vector(
        legacy,
        &[
            "setup.fq_mul",
            "setup.fq_inv",
            "setup.fq_add",
            "setup.f3_mul",
            "setup.f3_sq",
            "setup.f3_inv",
            "setup.batch_inv",
            "setup.group_add",
        ],
        &[85_131, 2_348, 271_096, 6_245, 3_897, 2_348, 0, 2_429],
    )?;

    println!("r00-r16 deterministic control PASS");
    println!("source_revision=d85752a9a0");
    println!("rho_rows=12 known_answers=12 paired_widths=5 legacy_pairs=1");
    println!("fast_zero_field_rows=6 strong_b8_count_vector=exact");
    Ok(())
}

fn read_rows(path: &str) -> Result<Vec<Row>, String> {
    let input = fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    input
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(i, line)| parse_flat_object(line).map_err(|e| format!("line {}: {e}", i + 1)))
        .collect::<Result<Vec<_>, _>>()
}

fn tamper_selftest(rows: &[Row]) -> Result<(), String> {
    let mut trajectory_tamper = rows.to_vec();
    let fast_b8 = trajectory_tamper
        .iter_mut()
        .find(|row| row.get("arm").map(String::as_str) == Some("fast-b8"))
        .ok_or_else(|| "selftest fixture lacks fast-b8".to_string())?;
    fast_b8.insert("steps".to_string(), "1488".to_string());
    if validate(&trajectory_tamper).is_ok() {
        return Err("trajectory tamper was accepted".to_string());
    }

    let mut authority_tamper = rows.to_vec();
    let fast_b4 = authority_tamper
        .iter_mut()
        .find(|row| row.get("arm").map(String::as_str) == Some("fast-b4"))
        .ok_or_else(|| "selftest fixture lacks fast-b4".to_string())?;
    fast_b4.insert("count_authoritative".to_string(), "true".to_string());
    if validate(&authority_tamper).is_ok() {
        return Err("authority tamper was accepted".to_string());
    }

    let mut counter_tamper = rows.to_vec();
    let fast_b1 = counter_tamper
        .iter_mut()
        .find(|row| row.get("arm").map(String::as_str) == Some("fast-b1"))
        .ok_or_else(|| "selftest fixture lacks fast-b1".to_string())?;
    fast_b1.insert("walk.fq_mul".to_string(), "1".to_string());
    if validate(&counter_tamper).is_ok() {
        return Err("counter tamper was accepted".to_string());
    }
    println!("tamper_rejections=3/3");
    Ok(())
}

fn run() -> Result<(), String> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "r00_r16".to_string());
    let first = args
        .next()
        .ok_or_else(|| format!("usage: {program} [--selftest] Q223_JSONL"))?;
    let (selftest, path) = if first == "--selftest" {
        let path = args
            .next()
            .ok_or_else(|| format!("usage: {program} [--selftest] Q223_JSONL"))?;
        (true, path)
    } else {
        (false, first)
    };
    if args.next().is_some() {
        return Err(format!("usage: {program} [--selftest] Q223_JSONL"));
    }
    let rows = read_rows(&path)?;
    validate(&rows)?;
    if selftest {
        tamper_selftest(&rows)?;
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("r00-r16 deterministic control FAIL: {error}");
        process::exit(1);
    }
}
