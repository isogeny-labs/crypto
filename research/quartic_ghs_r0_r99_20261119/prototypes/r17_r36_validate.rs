//! Independent native validator for the composed R17/R36 q=223 record.
//!
//! This validator was written from the frozen record contract and expected
//! deltas. It does not call the producer implementation, its jq validator, or
//! either producer-side Rust oracle. Development clocks are required to be
//! present for evidence retention but are never parsed or compared.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::process;

type Row = BTreeMap<String, String>;

struct Cursor<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            pos: 0,
        }
    }

    fn skip_ws(&mut self) {
        while self
            .input
            .get(self.pos)
            .is_some_and(|b| b.is_ascii_whitespace())
        {
            self.pos += 1;
        }
    }

    fn take(&mut self, expected: u8) -> Result<(), String> {
        self.skip_ws();
        match self.input.get(self.pos) {
            Some(got) if *got == expected => {
                self.pos += 1;
                Ok(())
            }
            Some(got) => Err(format!(
                "byte {}: expected {:?}, got {:?}",
                self.pos, expected as char, *got as char
            )),
            None => Err(format!(
                "byte {}: expected {:?}, got EOF",
                self.pos, expected as char
            )),
        }
    }

    fn string(&mut self) -> Result<String, String> {
        self.take(b'"')?;
        let mut output = String::new();
        loop {
            let byte = *self
                .input
                .get(self.pos)
                .ok_or_else(|| "unterminated JSON string".to_string())?;
            self.pos += 1;
            match byte {
                b'"' => return Ok(output),
                b'\\' => {
                    let escaped = *self
                        .input
                        .get(self.pos)
                        .ok_or_else(|| "unterminated JSON escape".to_string())?;
                    self.pos += 1;
                    let decoded = match escaped {
                        b'"' => '"',
                        b'\\' => '\\',
                        b'/' => '/',
                        b'b' => '\u{0008}',
                        b'f' => '\u{000c}',
                        b'n' => '\n',
                        b'r' => '\r',
                        b't' => '\t',
                        b'u' => {
                            return Err("unicode escapes are outside this flat schema".to_string())
                        }
                        _ => return Err(format!("invalid JSON escape {escaped}")),
                    };
                    output.push(decoded);
                }
                0x00..=0x1f => return Err("control byte in JSON string".to_string()),
                _ if byte.is_ascii() => output.push(byte as char),
                _ => return Err("non-ASCII JSON string is outside this schema".to_string()),
            }
        }
    }

    fn atom(&mut self) -> Result<String, String> {
        self.skip_ws();
        if self.input.get(self.pos) == Some(&b'"') {
            return self.string();
        }
        let start = self.pos;
        while let Some(byte) = self.input.get(self.pos) {
            if matches!(*byte, b',' | b'}') || byte.is_ascii_whitespace() {
                break;
            }
            if matches!(*byte, b'{' | b'[' | b']' | b':') {
                return Err(format!("nested/non-flat value at byte {}", self.pos));
            }
            self.pos += 1;
        }
        if self.pos == start {
            return Err(format!("empty JSON atom at byte {start}"));
        }
        let atom = std::str::from_utf8(&self.input[start..self.pos])
            .map_err(|e| format!("non-UTF8 atom: {e}"))?
            .to_string();
        if !matches!(atom.as_str(), "true" | "false" | "null") {
            let value = atom
                .parse::<f64>()
                .map_err(|e| format!("invalid numeric atom {atom}: {e}"))?;
            if !value.is_finite() {
                return Err(format!("non-finite numeric atom {atom}"));
            }
        }
        Ok(atom)
    }
}

fn parse_row(line: &str) -> Result<Row, String> {
    let mut cursor = Cursor::new(line);
    let mut row = Row::new();
    cursor.take(b'{')?;
    cursor.skip_ws();
    if cursor.input.get(cursor.pos) == Some(&b'}') {
        cursor.pos += 1;
    } else {
        loop {
            let key = cursor.string()?;
            cursor.take(b':')?;
            let value = cursor.atom()?;
            if row.insert(key.clone(), value).is_some() {
                return Err(format!("duplicate JSON key {key}"));
            }
            cursor.skip_ws();
            match cursor.input.get(cursor.pos) {
                Some(b',') => cursor.pos += 1,
                Some(b'}') => {
                    cursor.pos += 1;
                    break;
                }
                Some(byte) => {
                    return Err(format!(
                        "byte {}: expected comma or closing brace, got {:?}",
                        cursor.pos, *byte as char
                    ));
                }
                None => return Err("unterminated JSON object".to_string()),
            }
        }
    }
    cursor.skip_ws();
    if cursor.pos != cursor.input.len() {
        return Err(format!("trailing bytes at {}", cursor.pos));
    }
    Ok(row)
}

fn value<'a>(row: &'a Row, key: &str) -> Result<&'a str, String> {
    row.get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("missing required key {key}"))
}

fn number(row: &Row, key: &str) -> Result<u64, String> {
    value(row, key)?
        .parse::<u64>()
        .map_err(|e| format!("key {key} is not a canonical u64: {e}"))
}

fn require_text(row: &Row, key: &str, expected: &str) -> Result<(), String> {
    let got = value(row, key)?;
    if got != expected {
        return Err(format!("{key}: expected {expected}, got {got}"));
    }
    Ok(())
}

fn require_number(row: &Row, key: &str, expected: u64) -> Result<(), String> {
    let got = number(row, key)?;
    if got != expected {
        return Err(format!("{key}: expected {expected}, got {got}"));
    }
    Ok(())
}

fn equal_keys(left: &Row, right: &Row, keys: &[&str], label: &str) -> Result<(), String> {
    for key in keys {
        let a = value(left, key)?;
        let b = value(right, key)?;
        if a != b {
            return Err(format!("{label}: {key} differs: {a} != {b}"));
        }
    }
    Ok(())
}

const ARMS: &[&str] = &[
    "fast-b8-mul5-walker-table",
    "strong-b8",
    "strong-b8-mul5",
    "strong-b8-mul5-walker-table",
    "strong-b8-walker-table",
];

const PHASES: &[&str] = &["setup", "walk", "verify"];
const FIELD_COUNTERS: &[&str] = &[
    "fq_mul",
    "fq_inv",
    "fq_add",
    "fq_small_div",
    "f3_mul",
    "f3_sq",
    "f3_inv",
    "batch_inv",
    "group_add",
];
const TRAJECTORY: &[&str] = &[
    "batch",
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
];
const NATIVE: &[&str] = &[
    "setup_native.rng",
    "setup_native.hash",
    "setup_native.canon",
    "setup_native.lookahead_test",
    "setup_native.dp",
    "setup_native.table",
    "setup_native.cycle_cmp",
    "setup_native.scalar_add",
    "setup_native.scalar_mul",
    "setup_native.scalar_inv",
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

fn field_key(phase: &str, counter: &str) -> String {
    format!("{phase}.{counter}")
}

fn field_total(row: &Row, counter: &str) -> Result<u64, String> {
    PHASES.iter().try_fold(0u64, |total, phase| {
        total
            .checked_add(number(row, &field_key(phase, counter))?)
            .ok_or_else(|| format!("overflow summing {counter}"))
    })
}

fn require_all_fields(row: &Row) -> Result<(), String> {
    for phase in PHASES {
        for counter in FIELD_COUNTERS {
            number(row, &field_key(phase, counter))?;
        }
    }
    for key in TRAJECTORY.iter().chain(NATIVE) {
        value(row, key)?;
    }
    value(row, "wall_s")?;
    Ok(())
}

fn mul5_relation(reference: &Row, candidate: &Row, label: &str) -> Result<(), String> {
    for phase in PHASES {
        for counter in [
            "fq_inv",
            "f3_mul",
            "f3_sq",
            "f3_inv",
            "batch_inv",
            "group_add",
        ] {
            let key = field_key(phase, counter);
            if number(reference, &key)? != number(candidate, &key)? {
                return Err(format!("{label}: invariant counter {key} changed"));
            }
        }
        let f3_mul = number(reference, &field_key(phase, "f3_mul"))?;
        let ref_mul = number(reference, &field_key(phase, "fq_mul"))?;
        let candidate_mul = number(candidate, &field_key(phase, "fq_mul"))?;
        if ref_mul.checked_sub(candidate_mul) != Some(f3_mul) {
            return Err(format!("{label}: {phase} fq_mul delta is not -f3_mul"));
        }
        require_number(reference, &field_key(phase, "fq_small_div"), 0)?;
        let expected_small_div = f3_mul
            .checked_mul(4)
            .ok_or_else(|| format!("{label}: small-div expectation overflow"))?;
        require_number(
            candidate,
            &field_key(phase, "fq_small_div"),
            expected_small_div,
        )?;
    }
    Ok(())
}

fn exact_setup(row: &Row, expected: &[u64; 9], label: &str) -> Result<(), String> {
    for (counter, want) in FIELD_COUNTERS.iter().zip(expected) {
        let key = field_key("setup", counter);
        let got = number(row, &key)?;
        if got != *want {
            return Err(format!("{label}: {key}: expected {want}, got {got}"));
        }
    }
    Ok(())
}

fn find_rho<'a>(rows: &'a [Row], arm: &str) -> Result<&'a Row, String> {
    let mut matches = rows.iter().filter(|row| {
        row.get("rec").map(String::as_str) == Some("rho")
            && row.get("arm").map(String::as_str) == Some(arm)
    });
    let first = matches
        .next()
        .ok_or_else(|| format!("required arm {arm} is absent"))?;
    if matches.next().is_some() {
        return Err(format!("required arm {arm} is duplicated"));
    }
    Ok(first)
}

fn validate(rows: &[Row]) -> Result<(), String> {
    if rows.len() != 7 {
        return Err(format!("expected exactly 7 records, got {}", rows.len()));
    }
    let record_types: BTreeSet<&str> = rows
        .iter()
        .map(|row| value(row, "rec"))
        .collect::<Result<_, _>>()?;
    if record_types != BTreeSet::from(["calibration", "instance", "rho"]) {
        return Err(format!("unexpected record types: {record_types:?}"));
    }
    let instances: Vec<&Row> = rows
        .iter()
        .filter(|row| row.get("rec").map(String::as_str) == Some("instance"))
        .collect();
    let calibrations: Vec<&Row> = rows
        .iter()
        .filter(|row| row.get("rec").map(String::as_str) == Some("calibration"))
        .collect();
    if instances.len() != 1 || calibrations.len() != 1 {
        return Err(format!(
            "expected one instance and calibration, got {}/{}",
            instances.len(),
            calibrations.len()
        ));
    }
    let instance = instances[0];
    require_number(instance, "q", 223)?;
    require_number(instance, "seed", 1)?;
    require_number(instance, "ell", 2_773_073)?;
    require_number(instance, "order", 11_092_292)?;
    require_number(instance, "tries", 91)?;
    require_number(instance, "field_c", 3)?;
    require_number(instance, "ic_target", 1_765_657)?;
    require_text(instance, "target_policy", "ic-repeat")?;
    require_text(instance, "source_revision", "worktree-r17-r36")?;

    let calibration = calibrations[0];
    require_number(calibration, "q", 223)?;
    require_number(calibration, "seed", 1)?;
    require_number(calibration, "triples", 600)?;
    require_text(calibration, "isolation", "development")?;
    for retained_only in ["r_inv", "r_lmul", "mul_ns_median", "mul_ns_min"] {
        value(calibration, retained_only)?;
    }

    let actual_arms: BTreeSet<&str> = rows
        .iter()
        .filter(|row| row.get("rec").map(String::as_str) == Some("rho"))
        .map(|row| value(row, "arm"))
        .collect::<Result<_, _>>()?;
    let required_arms: BTreeSet<&str> = ARMS.iter().copied().collect();
    if actual_arms != required_arms {
        return Err(format!(
            "rho arm set differs: expected {required_arms:?}, got {actual_arms:?}"
        ));
    }

    let base = find_rho(rows, "strong-b8")?;
    let mul5 = find_rho(rows, "strong-b8-mul5")?;
    let walker = find_rho(rows, "strong-b8-walker-table")?;
    let combined = find_rho(rows, "strong-b8-mul5-walker-table")?;
    let fast = find_rho(rows, "fast-b8-mul5-walker-table")?;
    for row in [base, mul5, walker, combined, fast] {
        require_all_fields(row)?;
        require_number(row, "batch", 8)?;
        require_number(row, "q", 223)?;
        require_number(row, "seed", 1)?;
        require_number(row, "i", 0)?;
        require_text(row, "target_kind", "ic")?;
        require_number(row, "ell", 2_773_073)?;
        require_number(row, "k", 1_765_657)?;
        require_number(row, "k_found", 1_765_657)?;
        require_text(row, "ok", "true")?;
        equal_keys(base, row, TRAJECTORY, "trajectory")?;
        equal_keys(base, row, NATIVE, "native counts")?;
    }

    require_text(base, "setup_mode", "batched-window2-jumps")?;
    require_text(base, "field_kernel", "karatsuba6")?;
    require_text(base, "count_authoritative", "true")?;
    require_text(mul5, "setup_mode", "batched-window2-jumps")?;
    require_text(mul5, "field_kernel", "evaluation5")?;
    require_text(mul5, "count_authoritative", "true")?;
    require_text(walker, "setup_mode", "batched-window2-jumps-walkers")?;
    require_text(walker, "field_kernel", "karatsuba6")?;
    require_text(walker, "count_authoritative", "true")?;
    require_text(combined, "setup_mode", "batched-window2-jumps-walkers")?;
    require_text(combined, "field_kernel", "evaluation5")?;
    require_text(combined, "count_authoritative", "true")?;
    require_text(fast, "setup_mode", "batched-window2-jumps-walkers")?;
    require_text(fast, "field_kernel", "evaluation5")?;
    require_text(fast, "count_authoritative", "false")?;

    mul5_relation(base, mul5, "R17 standalone")?;
    require_number(base, "setup.fq_mul", 56_484)?;
    exact_setup(
        walker,
        &[48_433, 288, 178_576, 0, 5_847, 1_979, 288, 258, 1_224],
        "R36",
    )?;
    exact_setup(
        base,
        &[56_484, 511, 204_142, 0, 6_437, 2_346, 511, 33, 1_455],
        "R36 baseline",
    )?;
    for phase in ["walk", "verify"] {
        for counter in FIELD_COUNTERS {
            let key = field_key(phase, counter);
            if number(base, &key)? != number(walker, &key)? {
                return Err(format!("R36 changed non-setup counter {key}"));
            }
        }
    }

    mul5_relation(walker, combined, "R17+R36 composition")?;
    for (row, expected, label) in [
        (base, [110_435, 782, 400_276, 0], "baseline"),
        (mul5, [96_884, 782, 670_668, 54_204], "mul5"),
        (combined, [89_423, 559, 633_370, 51_844], "combined"),
    ] {
        for (counter, want) in ["fq_mul", "fq_inv", "fq_add", "fq_small_div"]
            .iter()
            .zip(expected)
        {
            let got = field_total(row, counter)?;
            if got != want {
                return Err(format!(
                    "{label} total {counter}: expected {want}, got {got}"
                ));
            }
        }
    }

    for phase in PHASES {
        for counter in FIELD_COUNTERS {
            require_number(fast, &field_key(phase, counter), 0)?;
        }
    }
    equal_keys(combined, fast, TRAJECTORY, "counted/fast trajectory")?;
    equal_keys(combined, fast, NATIVE, "counted/fast native counts")?;

    println!("R17-R36 independent record validation PASS");
    println!("records=7 rho_arms=5 recovered_scalar=1765657");
    println!("trajectory_pairs=4 native_count_pairs=4");
    println!("r17_phase_relations=6 r36_exact_setup_vectors=2");
    println!("counted_fast_zero_field_counters=27");
    println!("timing_fields=retained_not_interpreted");
    Ok(())
}

fn mutate_arm<'a>(rows: &'a mut [Row], arm: &str) -> Result<&'a mut Row, String> {
    rows.iter_mut()
        .find(|row| row.get("arm").map(String::as_str) == Some(arm))
        .ok_or_else(|| format!("selftest cannot find {arm}"))
}

fn require_rejected(rows: &[Row], label: &str) -> Result<(), String> {
    if validate(rows).is_ok() {
        return Err(format!("tamper selftest accepted {label}"));
    }
    Ok(())
}

fn tamper_selftest(rows: &[Row]) -> Result<(), String> {
    let mut wrong_scalar = rows.to_vec();
    mutate_arm(&mut wrong_scalar, "strong-b8-mul5")?
        .insert("k_found".to_string(), "1765658".to_string());
    require_rejected(&wrong_scalar, "wrong recovered scalar")?;

    let mut changed_trajectory = rows.to_vec();
    mutate_arm(&mut changed_trajectory, "strong-b8-walker-table")?
        .insert("steps".to_string(), "1488".to_string());
    require_rejected(&changed_trajectory, "changed trajectory")?;

    let mut wrong_delta = rows.to_vec();
    mutate_arm(&mut wrong_delta, "strong-b8-mul5-walker-table")?
        .insert("setup.fq_mul".to_string(), "42587".to_string());
    require_rejected(&wrong_delta, "wrong exact delta")?;

    let mut fast_counter = rows.to_vec();
    mutate_arm(&mut fast_counter, "fast-b8-mul5-walker-table")?
        .insert("walk.fq_mul".to_string(), "1".to_string());
    require_rejected(&fast_counter, "nonzero fast counter")?;

    let mut missing_key = rows.to_vec();
    mutate_arm(&mut missing_key, "strong-b8")?.remove("walk_native.hash");
    require_rejected(&missing_key, "missing native counter")?;

    let mut extra_arm = rows.to_vec();
    let mut cloned = find_rho(rows, "strong-b8")?.clone();
    cloned.insert("arm".to_string(), "strong-b8-unknown".to_string());
    extra_arm.push(cloned);
    require_rejected(&extra_arm, "unexpected arm")?;

    let mut source_mismatch = rows.to_vec();
    source_mismatch
        .iter_mut()
        .find(|row| row.get("rec").map(String::as_str) == Some("instance"))
        .ok_or_else(|| "selftest instance absent".to_string())?
        .insert("source_revision".to_string(), "other".to_string());
    require_rejected(&source_mismatch, "source revision mismatch")?;

    if parse_row(r#"{"rec":"rho","rec":"instance"}"#).is_ok() {
        return Err("tamper selftest accepted duplicate JSON key".to_string());
    }

    println!("tamper_rejections=8/8");
    Ok(())
}

fn read_rows(path: &str) -> Result<Vec<Row>, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("{path}: {e}"))?
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| parse_row(line).map_err(|e| format!("line {}: {e}", index + 1)))
        .collect()
}

fn run() -> Result<(), String> {
    let mut args = env::args();
    let program = args
        .next()
        .unwrap_or_else(|| "r17_r36_validate".to_string());
    let first = args
        .next()
        .ok_or_else(|| format!("usage: {program} [--selftest] RECORDS.jsonl"))?;
    let (selftest, path) = if first == "--selftest" {
        (
            true,
            args.next()
                .ok_or_else(|| format!("usage: {program} [--selftest] RECORDS.jsonl"))?,
        )
    } else {
        (false, first)
    };
    if args.next().is_some() {
        return Err(format!("usage: {program} [--selftest] RECORDS.jsonl"));
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
        eprintln!("R17-R36 independent record validation FAIL: {error}");
        process::exit(1);
    }
}
