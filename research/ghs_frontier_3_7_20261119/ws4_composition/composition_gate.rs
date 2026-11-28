//! Native command generator and fail-closed record gate for GHS rho composition.
//!
//! This tool does not run a benchmark. `plan` emits the prospectively frozen
//! invocations. `validate` hashes the supplied binary and checks one combined
//! correctness/count record. `selftest` runs a corruption matrix against the
//! exact q=223 source-bound capsule.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

const CAPSULE_SOURCE: &str = "95297ce1d098e124e72d45effc07fe8c640de82b";
const Q_VALUES: [u64; 3] = [4_051, 16_363, 65_479];
const DEVELOPMENT_SEEDS: [u64; 3] = [101, 102, 103];
const HOLDOUT_SEEDS: [u64; 3] = [1, 2, 3];
const WIDTHS: [usize; 5] = [1, 4, 8, 16, 32];
const SCHEDULE: [[&str; 4]; 5] = [
    ["A0", "A1", "B0", "B1"],
    ["B1", "B0", "A1", "A0"],
    ["A1", "B1", "A0", "B0"],
    ["B0", "A0", "B1", "A1"],
    ["A0", "B0", "A1", "B1"],
];
const PHASES: [&str; 3] = ["setup", "walk", "verify"];
const FIELD: [&str; 9] = [
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
const NATIVE_PHASES: [&str; 2] = ["setup_native", "walk_native"];
const NATIVE: [&str; 11] = [
    "rng",
    "hash",
    "canon",
    "lookahead_test",
    "dp",
    "table",
    "cycle_cmp",
    "scalar_add",
    "scalar_mul",
    "scalar_inv",
    "band_ops",
];
const TRAJECTORY: [&str; 15] = [
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
];

#[derive(Clone, Debug, PartialEq, Eq)]
enum Scalar {
    Null,
    Bool(bool),
    Number(String),
    String(String),
}

struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self { input, pos: 0 }
    }

    fn ws(&mut self) {
        while self
            .input
            .get(self.pos)
            .is_some_and(|b| b.is_ascii_whitespace())
        {
            self.pos += 1;
        }
    }

    fn take(&mut self, expected: u8) -> Result<(), String> {
        self.ws();
        match self.input.get(self.pos) {
            Some(actual) if *actual == expected => {
                self.pos += 1;
                Ok(())
            }
            Some(actual) => Err(format!(
                "byte {}: expected {:?}, got {:?}",
                self.pos, expected as char, *actual as char
            )),
            None => Err(format!("byte {}: expected byte, got EOF", self.pos)),
        }
    }

    fn string(&mut self) -> Result<String, String> {
        self.take(b'"')?;
        let mut out = String::new();
        loop {
            let byte = *self
                .input
                .get(self.pos)
                .ok_or_else(|| "unterminated string".to_string())?;
            self.pos += 1;
            match byte {
                b'"' => return Ok(out),
                b'\\' => {
                    let escaped = *self
                        .input
                        .get(self.pos)
                        .ok_or_else(|| "unterminated escape".to_string())?;
                    self.pos += 1;
                    out.push(match escaped {
                        b'"' => '"',
                        b'\\' => '\\',
                        b'/' => '/',
                        b'b' => '\u{0008}',
                        b'f' => '\u{000c}',
                        b'n' => '\n',
                        b'r' => '\r',
                        b't' => '\t',
                        _ => return Err(format!("unsupported escape at byte {}", self.pos - 1)),
                    });
                }
                0x00..=0x1f => return Err("control byte in string".to_string()),
                0x20..=0x7f => out.push(byte as char),
                _ => return Err("non-ASCII string outside record schema".to_string()),
            }
        }
    }

    fn literal(&mut self, text: &[u8], value: Scalar) -> Result<Scalar, String> {
        if self.input.get(self.pos..self.pos + text.len()) == Some(text) {
            self.pos += text.len();
            Ok(value)
        } else {
            Err(format!("invalid literal at byte {}", self.pos))
        }
    }

    fn number(&mut self) -> Result<Scalar, String> {
        let start = self.pos;
        if self.input.get(self.pos) == Some(&b'-') {
            self.pos += 1;
        }
        match self.input.get(self.pos) {
            Some(b'0') => {
                self.pos += 1;
                if self.input.get(self.pos).is_some_and(u8::is_ascii_digit) {
                    return Err(format!("leading zero at byte {start}"));
                }
            }
            Some(b'1'..=b'9') => {
                self.pos += 1;
                while self.input.get(self.pos).is_some_and(u8::is_ascii_digit) {
                    self.pos += 1;
                }
            }
            _ => return Err(format!("invalid number at byte {start}")),
        }
        if self.input.get(self.pos) == Some(&b'.') {
            self.pos += 1;
            let digits = self.pos;
            while self.input.get(self.pos).is_some_and(u8::is_ascii_digit) {
                self.pos += 1;
            }
            if self.pos == digits {
                return Err(format!("empty fractional part at byte {start}"));
            }
        }
        if matches!(self.input.get(self.pos), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.input.get(self.pos), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            let digits = self.pos;
            while self.input.get(self.pos).is_some_and(u8::is_ascii_digit) {
                self.pos += 1;
            }
            if self.pos == digits {
                return Err(format!("empty exponent at byte {start}"));
            }
        }
        let raw = std::str::from_utf8(&self.input[start..self.pos])
            .map_err(|error| error.to_string())?
            .to_string();
        let parsed = raw
            .parse::<f64>()
            .map_err(|error| format!("invalid number {raw}: {error}"))?;
        if !parsed.is_finite() {
            return Err(format!("non-finite number {raw}"));
        }
        Ok(Scalar::Number(raw))
    }

    fn scalar(&mut self) -> Result<Scalar, String> {
        self.ws();
        match self.input.get(self.pos) {
            Some(b'n') => self.literal(b"null", Scalar::Null),
            Some(b't') => self.literal(b"true", Scalar::Bool(true)),
            Some(b'f') => self.literal(b"false", Scalar::Bool(false)),
            Some(b'"') => self.string().map(Scalar::String),
            Some(b'-' | b'0'..=b'9') => self.number(),
            Some(b'{' | b'[') => Err("nested JSON is outside flat record schema".to_string()),
            Some(byte) => Err(format!(
                "unexpected {:?} at byte {}",
                *byte as char, self.pos
            )),
            None => Err("unexpected EOF".to_string()),
        }
    }

    fn object(&mut self) -> Result<BTreeMap<String, Scalar>, String> {
        self.take(b'{')?;
        let mut values = BTreeMap::new();
        self.ws();
        if self.input.get(self.pos) == Some(&b'}') {
            self.pos += 1;
            return Ok(values);
        }
        loop {
            let key = self.string()?;
            self.take(b':')?;
            let value = self.scalar()?;
            if values.insert(key.clone(), value).is_some() {
                return Err(format!("duplicate JSON key {key}"));
            }
            self.ws();
            match self.input.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    break;
                }
                _ => return Err(format!("expected comma or }} at byte {}", self.pos)),
            }
        }
        self.ws();
        if self.pos != self.input.len() {
            return Err(format!("trailing data at byte {}", self.pos));
        }
        Ok(values)
    }
}

fn parse_record(line: &str) -> Result<BTreeMap<String, Scalar>, String> {
    Parser::new(line.as_bytes()).object()
}

fn get<'a>(map: &'a BTreeMap<String, Scalar>, key: &str) -> Result<&'a Scalar, String> {
    map.get(key).ok_or_else(|| format!("missing key {key}"))
}

fn text<'a>(map: &'a BTreeMap<String, Scalar>, key: &str) -> Result<&'a str, String> {
    match get(map, key)? {
        Scalar::String(value) => Ok(value),
        _ => Err(format!("{key}: expected string")),
    }
}

fn boolean(map: &BTreeMap<String, Scalar>, key: &str) -> Result<bool, String> {
    match get(map, key)? {
        Scalar::Bool(value) => Ok(*value),
        _ => Err(format!("{key}: expected boolean")),
    }
}

fn integer(map: &BTreeMap<String, Scalar>, key: &str) -> Result<u64, String> {
    match get(map, key)? {
        Scalar::Number(raw) if raw.bytes().all(|byte| byte.is_ascii_digit()) => raw
            .parse()
            .map_err(|error| format!("{key}: invalid u64 {raw}: {error}")),
        _ => Err(format!("{key}: expected canonical unsigned integer")),
    }
}

fn nonnegative_finite(map: &BTreeMap<String, Scalar>, key: &str) -> Result<(), String> {
    match get(map, key)? {
        Scalar::Number(raw) => {
            let value: f64 = raw
                .parse()
                .map_err(|error| format!("{key}: invalid f64: {error}"))?;
            if value.is_finite() && value >= 0.0 {
                Ok(())
            } else {
                Err(format!("{key}: negative or non-finite"))
            }
        }
        _ => Err(format!("{key}: expected number")),
    }
}

fn lower_hex(value: &str, width: usize) -> bool {
    value.len() == width
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn sha256(input: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let bit_len = (input.len() as u64).wrapping_mul(8);
    let mut padded = input.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    let mut state = [
        0x6a09e667u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    for chunk in padded.chunks_exact(64) {
        let mut words = [0u32; 64];
        for (index, bytes) in chunk.chunks_exact(4).enumerate() {
            words[index] = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        }
        for index in 16..64 {
            let s0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let s1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(s0)
                .wrapping_add(words[index - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for index in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let t1 = h
                .wrapping_add(s1)
                .wrapping_add((e & f) ^ ((!e) & g))
                .wrapping_add(K[index])
                .wrapping_add(words[index]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let t2 = s0.wrapping_add((a & b) ^ (a & c) ^ (b & c));
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (slot, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }
    state.iter().map(|word| format!("{word:08x}")).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Panel {
    Capsule8,
    Composition14,
}

impl Panel {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "capsule8" => Ok(Self::Capsule8),
            "composition14" => Ok(Self::Composition14),
            _ => Err(format!("unknown panel {value}")),
        }
    }
}

fn primary_arms(width: usize, panel: Panel) -> Vec<String> {
    let prefix = |kind: &str, suffix: &str| format!("{kind}-b{width}{suffix}");
    match panel {
        Panel::Capsule8 => vec![
            prefix("strong", ""),
            prefix("strong", "-mul6"),
            prefix("strong", "-mul5"),
            prefix("strong", "-walker-table"),
            prefix("strong", "-mul6-walker-table"),
            prefix("strong", "-mul5-walker-table"),
            prefix("fast", "-mul5-walker-table"),
            prefix("bare", "-mul5-walker-table"),
        ],
        Panel::Composition14 => vec![
            prefix("strong", ""),
            prefix("strong", "-mul6"),
            prefix("strong", "-mul5"),
            prefix("strong", "-walker-table"),
            prefix("strong", "-mul6-walker-table"),
            prefix("strong", "-mul5-walker-table"),
            prefix("fast", ""),
            prefix("fast", "-mul5"),
            prefix("fast", "-walker-table"),
            prefix("fast", "-mul5-walker-table"),
            prefix("bare", ""),
            prefix("bare", "-mul5"),
            prefix("bare", "-walker-table"),
            prefix("bare", "-mul5-walker-table"),
        ],
    }
}

fn counter(map: &BTreeMap<String, Scalar>, phase: &str, name: &str) -> Result<u64, String> {
    integer(map, &format!("{phase}.{name}"))
}

fn same_fields(
    left: &BTreeMap<String, Scalar>,
    right: &BTreeMap<String, Scalar>,
    keys: &[&str],
    label: &str,
) -> Result<(), String> {
    for key in keys {
        if get(left, key)? != get(right, key)? {
            return Err(format!("{label}: key {key} differs"));
        }
    }
    Ok(())
}

fn same_counter_phase(
    left: &BTreeMap<String, Scalar>,
    right: &BTreeMap<String, Scalar>,
    phase: &str,
    names: &[&str],
    label: &str,
) -> Result<(), String> {
    for name in names {
        if counter(left, phase, name)? != counter(right, phase, name)? {
            return Err(format!("{label}: {phase}.{name} differs"));
        }
    }
    Ok(())
}

fn authority(arm: &str) -> Result<(bool, bool, bool), String> {
    if arm.starts_with("strong-") {
        Ok((true, true, true))
    } else if arm.starts_with("fast-") {
        Ok((false, false, true))
    } else if arm.starts_with("bare-") {
        Ok((false, false, false))
    } else {
        Err(format!("unknown arm family {arm}"))
    }
}

fn validate_authority(arm: &str, record: &BTreeMap<String, Scalar>) -> Result<(), String> {
    let (all, field, native) = authority(arm)?;
    if boolean(record, "count_authoritative")? != all
        || boolean(record, "field_count_authoritative")? != field
        || boolean(record, "native_count_authoritative")? != native
    {
        return Err(format!("{arm}: authority booleans differ"));
    }
    let mut field_sum = 0u64;
    for phase in PHASES {
        for name in FIELD {
            field_sum = field_sum
                .checked_add(counter(record, phase, name)?)
                .ok_or_else(|| format!("{arm}: field counter sum overflow"))?;
        }
    }
    let mut native_sum = 0u64;
    for phase in NATIVE_PHASES {
        for name in NATIVE {
            native_sum = native_sum
                .checked_add(counter(record, phase, name)?)
                .ok_or_else(|| format!("{arm}: native counter sum overflow"))?;
        }
    }
    if field && field_sum == 0 {
        return Err(format!("{arm}: counted field positive control is zero"));
    }
    if !field && field_sum != 0 {
        return Err(format!(
            "{arm}: non-authoritative field counters are nonzero"
        ));
    }
    if native && native_sum == 0 {
        return Err(format!("{arm}: counted native positive control is zero"));
    }
    if !native && native_sum != 0 {
        return Err(format!(
            "{arm}: non-authoritative native counters are nonzero"
        ));
    }
    Ok(())
}

fn mul5_identity(
    base: &BTreeMap<String, Scalar>,
    candidate: &BTreeMap<String, Scalar>,
    label: &str,
) -> Result<(), String> {
    for phase in PHASES {
        let f3_mul = counter(base, phase, "f3_mul")?;
        if counter(candidate, phase, "f3_mul")? != f3_mul {
            return Err(format!("{label}: {phase}.f3_mul differs"));
        }
        let expected_mul = counter(base, phase, "fq_mul")?
            .checked_sub(f3_mul)
            .ok_or_else(|| format!("{label}: {phase}.fq_mul underflow"))?;
        if counter(candidate, phase, "fq_mul")? != expected_mul {
            return Err(format!("{label}: {phase}.fq_mul is not base-f3_mul"));
        }
        let expected_div = counter(base, phase, "fq_small_div")?
            .checked_add(4 * f3_mul)
            .ok_or_else(|| format!("{label}: {phase}.fq_small_div overflow"))?;
        if counter(candidate, phase, "fq_small_div")? != expected_div {
            return Err(format!(
                "{label}: {phase}.fq_small_div is not base+4*f3_mul"
            ));
        }
        for name in FIELD {
            if !matches!(name, "fq_mul" | "fq_add" | "fq_small_div")
                && counter(base, phase, name)? != counter(candidate, phase, name)?
            {
                return Err(format!("{label}: {phase}.{name} unexpectedly differs"));
            }
        }
    }
    Ok(())
}

fn validate_records(
    record_path: &Path,
    binary_path: &Path,
    expected_binary_sha: &str,
    expected_source: &str,
    q: u64,
    seed: u64,
    width: usize,
    panel: Panel,
) -> Result<(), String> {
    if !lower_hex(expected_binary_sha, 64) {
        return Err("expected binary SHA-256 is not lowercase 64-hex".to_string());
    }
    if !lower_hex(expected_source, 40) {
        return Err("expected source revision is not lowercase 40-hex".to_string());
    }
    let binary = fs::read(binary_path)
        .map_err(|error| format!("read binary {}: {error}", binary_path.display()))?;
    let actual_binary_sha = sha256(&binary);
    if actual_binary_sha != expected_binary_sha {
        return Err(format!(
            "binary SHA-256 differs: expected {expected_binary_sha}, got {actual_binary_sha}"
        ));
    }

    let bytes = fs::read(record_path)
        .map_err(|error| format!("read records {}: {error}", record_path.display()))?;
    let contents = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    if !contents.ends_with('\n') {
        return Err("record file must end in newline".to_string());
    }
    let mut parsed = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        if line.is_empty() {
            return Err(format!("empty record line {}", index + 1));
        }
        parsed.push(parse_record(line).map_err(|error| format!("line {}: {error}", index + 1))?);
    }
    let expected_arms = primary_arms(width, panel);
    if parsed.len() != expected_arms.len() + 2 {
        return Err(format!(
            "expected {} records, got {}",
            expected_arms.len() + 2,
            parsed.len()
        ));
    }
    if text(&parsed[0], "rec")? != "instance" || text(&parsed[1], "rec")? != "calibration" {
        return Err("first records must be instance then calibration".to_string());
    }
    if integer(&parsed[0], "q")? != q || integer(&parsed[0], "seed")? != seed {
        return Err("instance q/seed differs".to_string());
    }
    if text(&parsed[0], "source_revision")? != expected_source {
        return Err("instance source revision differs".to_string());
    }
    if !matches!(text(&parsed[0], "target_policy")?, "ic-repeat" | "sequence") {
        return Err("unsupported target policy".to_string());
    }
    let ell = integer(&parsed[0], "ell")?;
    if ell == 0 || integer(&parsed[1], "q")? != q || integer(&parsed[1], "seed")? != seed {
        return Err("calibration identity differs".to_string());
    }
    if integer(&parsed[1], "triples")? == 0 {
        return Err("calibration has zero triples".to_string());
    }
    for key in ["r_inv", "r_lmul", "mul_ns_median", "mul_ns_min"] {
        nonnegative_finite(&parsed[1], key)?;
    }
    let _isolation = text(&parsed[1], "isolation")?;

    let mut by_arm = BTreeMap::new();
    let mut actual_order = Vec::new();
    for record in parsed.iter().skip(2) {
        if text(record, "rec")? != "rho" {
            return Err("non-rho record after calibration".to_string());
        }
        let arm = text(record, "arm")?.to_string();
        if by_arm.insert(arm.clone(), record).is_some() {
            return Err(format!("duplicate arm {arm}"));
        }
        actual_order.push(arm.clone());
        if integer(record, "q")? != q
            || integer(record, "seed")? != seed
            || integer(record, "batch")? != width as u64
            || integer(record, "i")? != 0
            || integer(record, "ell")? != ell
        {
            return Err(format!("{arm}: identity differs"));
        }
        if text(record, "target_kind")? != "ic" || !boolean(record, "ok")? {
            return Err(format!("{arm}: target kind or ok differs"));
        }
        if integer(record, "k_found")? != integer(record, "k")? {
            return Err(format!("{arm}: recovered scalar differs"));
        }
        nonnegative_finite(record, "wall_s")?;
        let expected_setup = if arm.ends_with("-walker-table") {
            "batched-window2-jumps-walkers"
        } else {
            "batched-window2-jumps"
        };
        if text(record, "setup_mode")? != expected_setup {
            return Err(format!("{arm}: setup mode differs"));
        }
        let expected_kernel = if arm.contains("-mul5") {
            "evaluation5"
        } else {
            "karatsuba6"
        };
        if text(record, "field_kernel")? != expected_kernel {
            return Err(format!("{arm}: field kernel differs"));
        }
        validate_authority(&arm, record)?;
    }
    if actual_order != expected_arms {
        return Err(format!(
            "arm order/census differs: expected {expected_arms:?}, got {actual_order:?}"
        ));
    }
    let baseline = by_arm
        .get(&format!("strong-b{width}"))
        .ok_or_else(|| "missing baseline".to_string())?;
    for record in by_arm.values() {
        same_fields(baseline, record, &TRAJECTORY, "trajectory")?;
    }

    let base_alias = by_arm
        .get(&format!("strong-b{width}-mul6"))
        .ok_or_else(|| "missing baseline mul6 alias".to_string())?;
    for phase in PHASES {
        same_counter_phase(baseline, base_alias, phase, &FIELD, "baseline alias")?;
    }
    for phase in NATIVE_PHASES {
        same_counter_phase(baseline, base_alias, phase, &NATIVE, "baseline alias")?;
    }
    let mul5 = by_arm
        .get(&format!("strong-b{width}-mul5"))
        .ok_or_else(|| "missing R17 arm".to_string())?;
    mul5_identity(baseline, mul5, "R17")?;

    let walker = by_arm
        .get(&format!("strong-b{width}-walker-table"))
        .ok_or_else(|| "missing R36 arm".to_string())?;
    let walker_alias = by_arm
        .get(&format!("strong-b{width}-mul6-walker-table"))
        .ok_or_else(|| "missing R36 mul6 alias".to_string())?;
    for phase in PHASES {
        same_counter_phase(walker, walker_alias, phase, &FIELD, "walker alias")?;
    }
    for phase in NATIVE_PHASES {
        same_counter_phase(walker, walker_alias, phase, &NATIVE, "walker alias")?;
    }
    let combined = by_arm
        .get(&format!("strong-b{width}-mul5-walker-table"))
        .ok_or_else(|| "missing composed arm".to_string())?;
    mul5_identity(walker, combined, "R17+R36")?;
    for phase in ["walk", "verify"] {
        same_counter_phase(baseline, walker, phase, &FIELD, "R36 baseline")?;
        same_counter_phase(mul5, combined, phase, &FIELD, "R36 mul5")?;
    }
    for phase in NATIVE_PHASES {
        same_counter_phase(baseline, walker, phase, &NATIVE, "R36 native")?;
        same_counter_phase(mul5, combined, phase, &NATIVE, "R36 mul5 native")?;
    }

    let mechanisms = [
        ("", ""),
        ("-mul5", "-mul5"),
        ("-walker-table", "-walker-table"),
        ("-mul5-walker-table", "-mul5-walker-table"),
    ];
    for (strong_suffix, mode_suffix) in mechanisms {
        let strong_name = format!("strong-b{width}{strong_suffix}");
        if let Some(fast) = by_arm.get(&format!("fast-b{width}{mode_suffix}")) {
            let strong = by_arm
                .get(&strong_name)
                .ok_or_else(|| format!("missing counted peer for fast {mode_suffix}"))?;
            for phase in NATIVE_PHASES {
                same_counter_phase(strong, fast, phase, &NATIVE, "fast native parity")?;
            }
        }
        if let Some(bare) = by_arm.get(&format!("bare-b{width}{mode_suffix}")) {
            same_fields(
                by_arm
                    .get(&strong_name)
                    .ok_or_else(|| format!("missing counted peer for bare {mode_suffix}"))?,
                bare,
                &TRAJECTORY,
                "bare trajectory parity",
            )?;
        }
    }

    Ok(())
}

fn arm_list(width: usize, family: &str) -> String {
    primary_arms(width, Panel::Composition14)
        .into_iter()
        .filter(|arm| arm.starts_with(family))
        .collect::<Vec<_>>()
        .join(",")
}

fn validate_plan_binding(source: &str, binary_sha: &str) -> Result<(), String> {
    if !lower_hex(source, 40) {
        return Err("source revision must be lowercase 40-hex".to_string());
    }
    if !lower_hex(binary_sha, 64) {
        return Err("binary SHA-256 must be lowercase 64-hex".to_string());
    }
    Ok(())
}

fn command(q: u64, seed: u64, policy: &str, arms: &str) -> String {
    format!(
        "GHS_CALIB_TRIPLES=600 GHS_ISOLATION=isolab-l2 \"$GHS_BINARY\" run {q} {seed} 0 1 {policy} {arms}"
    )
}

fn candidate_arm(candidate: &str) -> Result<&'static str, String> {
    match candidate {
        "R17" => Ok("bare-b8-mul5"),
        "R36" => Ok("bare-b8-walker-table"),
        "R17+R36" => Ok("bare-b8-mul5-walker-table"),
        _ => Err(format!("unknown candidate {candidate}")),
    }
}

fn emit_plan(
    mode: &str,
    source: &str,
    binary_sha: &str,
    selected: Option<&str>,
) -> Result<(), String> {
    validate_plan_binding(source, binary_sha)?;
    println!("schema\tghs-rho-composition-plan/v1");
    println!("source_revision\t{source}");
    println!("binary_sha256\t{binary_sha}");
    println!("mode\t{mode}");
    println!("record_id\tq\tseed\tcandidate\tblock\tlabel\tarm\tmeasured\tcommand");
    if mode == "correctness" {
        if selected.is_some() {
            return Err("correctness mode takes no selected candidate".to_string());
        }
        for width in WIDTHS {
            let arms = primary_arms(width, Panel::Composition14).join(",");
            println!(
                "q223-w{width}\t223\t1\tall\tpre\tcomposition14\tall\tfalse\tGHS_CALIB_TRIPLES=1 GHS_ISOLATION=correctness-only \"$GHS_BINARY\" run 223 1 0 1 ic-repeat {arms}"
            );
        }
        return Ok(());
    }
    let seeds: &[u64] = match mode {
        "development" => {
            if selected.is_some() {
                return Err("development mode takes no selected candidate".to_string());
            }
            &DEVELOPMENT_SEEDS
        }
        "holdout" => {
            if selected.is_none() {
                return Err("holdout mode requires one selected candidate".to_string());
            }
            &HOLDOUT_SEEDS
        }
        _ => return Err(format!("unknown plan mode {mode}")),
    };
    let target_policy = if mode == "holdout" {
        "ic-repeat"
    } else {
        "sequence"
    };
    let candidates: Vec<&str> = if let Some(candidate) = selected {
        candidate_arm(candidate)?;
        vec![candidate]
    } else {
        vec!["R17", "R36", "R17+R36"]
    };
    for q in Q_VALUES {
        for seed in seeds {
            let precheck_arms = primary_arms(8, Panel::Composition14).join(",");
            println!(
                "q{q}-s{seed}-precheck\t{q}\t{seed}\tall\tpre\tcomposition14\tall\tfalse\t{}",
                command(q, *seed, target_policy, &precheck_arms)
            );
            println!(
                "q{q}-s{seed}-counted\t{q}\t{seed}\tall\tpre\tcounted\tstrong-*\tfalse\t{}",
                command(q, *seed, target_policy, &arm_list(8, "strong-"))
            );
            println!(
                "q{q}-s{seed}-native\t{q}\t{seed}\tall\tpre\tnative\tfast-*\tfalse\t{}",
                command(q, *seed, target_policy, &arm_list(8, "fast-"))
            );
            for candidate in &candidates {
                let b_arm = candidate_arm(candidate)?;
                for (label, arm) in [("warmup-A", "bare-b8"), ("warmup-B", b_arm)] {
                    println!(
                        "q{q}-s{seed}-{candidate}-{label}\t{q}\t{seed}\t{candidate}\twarmup\t{label}\t{arm}\tfalse\t{}",
                        command(q, *seed, target_policy, arm)
                    );
                }
                for (block, labels) in SCHEDULE.iter().enumerate() {
                    for label in labels {
                        let arm = if label.starts_with('A') {
                            "bare-b8"
                        } else {
                            b_arm
                        };
                        println!(
                            "q{q}-s{seed}-{candidate}-b{block}-{label}\t{q}\t{seed}\t{candidate}\t{block}\t{label}\t{arm}\ttrue\t{}",
                            command(q, *seed, target_policy, arm)
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

fn temp_path(label: &str) -> PathBuf {
    env::temp_dir().join(format!(
        "ghs-composition-gate-{}-{label}.jsonl",
        process::id()
    ))
}

fn replace_once(input: &str, from: &str, to: &str) -> Result<String, String> {
    let count = input.matches(from).count();
    if count != 1 {
        return Err(format!("mutation marker {from:?} occurs {count} times"));
    }
    Ok(input.replacen(from, to, 1))
}

fn replace_first(input: &str, from: &str, to: &str) -> Result<String, String> {
    if !input.contains(from) {
        return Err(format!("mutation marker {from:?} is absent"));
    }
    Ok(input.replacen(from, to, 1))
}

fn json_string(value: &str) -> String {
    let mut out = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_ascii() && !c.is_ascii_control() => out.push(c),
            _ => panic!("selftest record serializer received non-ASCII/control text"),
        }
    }
    out.push('"');
    out
}

fn serialize_record(record: &BTreeMap<String, Scalar>) -> String {
    let fields = record
        .iter()
        .map(|(key, value)| {
            let value = match value {
                Scalar::Null => "null".to_string(),
                Scalar::Bool(value) => value.to_string(),
                Scalar::Number(value) => value.clone(),
                Scalar::String(value) => json_string(value),
            };
            format!("{}:{value}", json_string(key))
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("{{{fields}}}")
}

fn synthesize_authority_record(
    source: &BTreeMap<String, Scalar>,
    arm: &str,
    field_authority: bool,
    native_authority: bool,
) -> BTreeMap<String, Scalar> {
    let mut record = source.clone();
    record.insert("arm".to_string(), Scalar::String(arm.to_string()));
    record.insert(
        "count_authoritative".to_string(),
        Scalar::Bool(field_authority && native_authority),
    );
    record.insert(
        "field_count_authoritative".to_string(),
        Scalar::Bool(field_authority),
    );
    record.insert(
        "native_count_authoritative".to_string(),
        Scalar::Bool(native_authority),
    );
    if !field_authority {
        for phase in PHASES {
            for name in FIELD {
                record.insert(format!("{phase}.{name}"), Scalar::Number("0".to_string()));
            }
        }
    }
    if !native_authority {
        for phase in NATIVE_PHASES {
            for name in NATIVE {
                record.insert(format!("{phase}.{name}"), Scalar::Number("0".to_string()));
            }
        }
    }
    record
}

fn selftest(fixture: &Path) -> Result<(), String> {
    let executable = env::current_exe().map_err(|error| error.to_string())?;
    let executable_sha = sha256(&fs::read(&executable).map_err(|error| error.to_string())?);
    validate_records(
        fixture,
        &executable,
        &executable_sha,
        CAPSULE_SOURCE,
        223,
        1,
        8,
        Panel::Capsule8,
    )?;
    let fixture_text = fs::read_to_string(fixture).map_err(|error| error.to_string())?;
    let fixture_records = fixture_text
        .lines()
        .map(parse_record)
        .collect::<Result<Vec<_>, _>>()?;
    let fixture_by_arm: BTreeMap<String, &BTreeMap<String, Scalar>> = fixture_records
        .iter()
        .skip(2)
        .map(|record| Ok((text(record, "arm")?.to_string(), record)))
        .collect::<Result<_, String>>()?;
    let mut composition_records = fixture_records[..2].to_vec();
    for arm in primary_arms(8, Panel::Composition14) {
        if let Some(record) = fixture_by_arm.get(&arm) {
            composition_records.push((*record).clone());
            continue;
        }
        let (source_arm, field_authority, native_authority) = if arm.starts_with("fast-") {
            (arm.replacen("fast-", "strong-", 1), false, true)
        } else if arm.starts_with("bare-") {
            (arm.replacen("bare-", "strong-", 1), false, false)
        } else {
            return Err(format!(
                "selftest cannot synthesize missing counted arm {arm}"
            ));
        };
        let source = fixture_by_arm
            .get(&source_arm)
            .ok_or_else(|| format!("selftest missing source arm {source_arm}"))?;
        composition_records.push(synthesize_authority_record(
            source,
            &arm,
            field_authority,
            native_authority,
        ));
    }
    let composition_path = temp_path("composition14-valid");
    let composition_text = composition_records
        .iter()
        .map(serialize_record)
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    fs::write(&composition_path, composition_text).map_err(|error| error.to_string())?;
    let composition_result = validate_records(
        &composition_path,
        &executable,
        &executable_sha,
        CAPSULE_SOURCE,
        223,
        1,
        8,
        Panel::Composition14,
    );
    let _ = fs::remove_file(&composition_path);
    composition_result?;

    let wrong_source = validate_records(
        fixture,
        &executable,
        &executable_sha,
        "0000000000000000000000000000000000000000",
        223,
        1,
        8,
        Panel::Capsule8,
    );
    if wrong_source.is_ok() {
        return Err("wrong-source corruption was accepted".to_string());
    }
    let wrong_binary = validate_records(
        fixture,
        &executable,
        &"0".repeat(64),
        CAPSULE_SOURCE,
        223,
        1,
        8,
        Panel::Capsule8,
    );
    if wrong_binary.is_ok() {
        return Err("wrong-binary corruption was accepted".to_string());
    }
    let mutations = [
        (
            "arm",
            replace_once(
                &fixture_text,
                "\"arm\":\"strong-b8\"",
                "\"arm\":\"strong-b9\"",
            )?,
        ),
        (
            "trajectory",
            replace_first(&fixture_text, "\"steps\":1487", "\"steps\":1488")?,
        ),
        (
            "authority",
            replace_first(
                &fixture_text,
                "\"field_count_authoritative\":true",
                "\"field_count_authoritative\":false",
            )?,
        ),
    ];
    for (label, mutation) in mutations {
        let path = temp_path(label);
        fs::write(&path, mutation).map_err(|error| error.to_string())?;
        let result = validate_records(
            &path,
            &executable,
            &executable_sha,
            CAPSULE_SOURCE,
            223,
            1,
            8,
            Panel::Capsule8,
        );
        let _ = fs::remove_file(&path);
        if result.is_ok() {
            return Err(format!("{label} corruption was accepted"));
        }
    }
    println!("composition gate selftest PASS");
    println!("valid_fixtures=2 corruptions_rejected=5");
    println!("timing_observations=0 performance_claims=0");
    Ok(())
}

fn usage() -> ! {
    eprintln!(
        "usage:\n  composition_gate selftest <capsule.jsonl>\n  composition_gate validate <records.jsonl> <binary> <binary_sha256> <source_revision> <q> <seed> <width> <capsule8|composition14>\n  composition_gate plan correctness <source_revision> <binary_sha256>\n  composition_gate plan development <source_revision> <binary_sha256>\n  composition_gate plan holdout <source_revision> <binary_sha256> <R17|R36|R17+R36>"
    );
    process::exit(2)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = match args.as_slice() {
        [_, command, fixture] if command == "selftest" => selftest(Path::new(fixture)),
        [_, command, records, binary, binary_sha, source, q, seed, width, panel]
            if command == "validate" =>
        {
            (|| {
                let q = q.parse().map_err(|error| format!("invalid q: {error}"))?;
                let seed = seed
                    .parse()
                    .map_err(|error| format!("invalid seed: {error}"))?;
                let width = width
                    .parse()
                    .map_err(|error| format!("invalid width: {error}"))?;
                validate_records(
                    Path::new(records),
                    Path::new(binary),
                    binary_sha,
                    source,
                    q,
                    seed,
                    width,
                    Panel::parse(panel)?,
                )
            })()
        }
        [_, command, mode, source, binary_sha] if command == "plan" => {
            emit_plan(mode, source, binary_sha, None)
        }
        [_, command, mode, source, binary_sha, selected] if command == "plan" => {
            emit_plan(mode, source, binary_sha, Some(selected))
        }
        _ => usage(),
    };
    if let Err(error) = result {
        eprintln!("FAIL: {error}");
        process::exit(1);
    }
}
