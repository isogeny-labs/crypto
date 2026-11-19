//! Native fail-closed authority checker for R17 flat JSONL records.
//!
//! This tool deliberately has no external dependencies.  It validates the
//! small-division accounting contract and exposes a caller-supplied rational
//! sensitivity weight.  It does not rank arms or interpret elapsed time.

use std::collections::BTreeMap;
use std::env;
use std::fs;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Value {
    String(String),
    Bool(bool),
    Number(String),
    Null,
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            bytes: text.as_bytes(),
            pos: 0,
        }
    }

    fn error<T>(&self, message: impl AsRef<str>) -> Result<T, String> {
        Err(format!("byte {}: {}", self.pos + 1, message.as_ref()))
    }

    fn skip_ws(&mut self) {
        while self
            .bytes
            .get(self.pos)
            .is_some_and(|byte| byte.is_ascii_whitespace())
        {
            self.pos += 1;
        }
    }

    fn take(&mut self, expected: u8) -> Result<(), String> {
        self.skip_ws();
        if self.bytes.get(self.pos) != Some(&expected) {
            return self.error(format!("expected '{}'", expected as char));
        }
        self.pos += 1;
        Ok(())
    }

    fn parse_string(&mut self) -> Result<String, String> {
        self.skip_ws();
        if self.bytes.get(self.pos) != Some(&b'\"') {
            return self.error("expected JSON string");
        }
        self.pos += 1;
        let mut out = String::new();
        while let Some(&byte) = self.bytes.get(self.pos) {
            self.pos += 1;
            match byte {
                b'\"' => return Ok(out),
                b'\\' => {
                    let escaped = *self
                        .bytes
                        .get(self.pos)
                        .ok_or_else(|| "unterminated JSON escape".to_owned())?;
                    self.pos += 1;
                    match escaped {
                        b'\"' => out.push('\"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{0008}'),
                        b'f' => out.push('\u{000c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let end = self.pos.checked_add(4).ok_or("unicode escape overflow")?;
                            let digits = self
                                .bytes
                                .get(self.pos..end)
                                .ok_or_else(|| "short unicode escape".to_owned())?;
                            let digits = std::str::from_utf8(digits)
                                .map_err(|_| "non-ASCII unicode escape".to_owned())?;
                            let code = u16::from_str_radix(digits, 16)
                                .map_err(|_| "invalid unicode escape".to_owned())?;
                            let character = char::from_u32(code as u32)
                                .ok_or_else(|| "invalid unicode scalar".to_owned())?;
                            out.push(character);
                            self.pos = end;
                        }
                        _ => return self.error("unknown JSON escape"),
                    }
                }
                0x00..=0x1f => return self.error("control byte in JSON string"),
                0x20..=0x7f => out.push(byte as char),
                _ => return self.error("non-ASCII JSON string is unsupported"),
            }
        }
        self.error("unterminated JSON string")
    }

    fn parse_number(&mut self) -> Result<String, String> {
        self.skip_ws();
        let start = self.pos;
        if self.bytes.get(self.pos) == Some(&b'-') {
            self.pos += 1;
        }
        match self.bytes.get(self.pos) {
            Some(b'0') => self.pos += 1,
            Some(b'1'..=b'9') => {
                self.pos += 1;
                while self.bytes.get(self.pos).is_some_and(u8::is_ascii_digit) {
                    self.pos += 1;
                }
            }
            _ => return self.error("invalid JSON number"),
        }
        if self.bytes.get(self.pos) == Some(&b'.') {
            self.pos += 1;
            let fraction_start = self.pos;
            while self.bytes.get(self.pos).is_some_and(u8::is_ascii_digit) {
                self.pos += 1;
            }
            if self.pos == fraction_start {
                return self.error("empty JSON fractional part");
            }
        }
        if matches!(self.bytes.get(self.pos), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.bytes.get(self.pos), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            let exponent_start = self.pos;
            while self.bytes.get(self.pos).is_some_and(u8::is_ascii_digit) {
                self.pos += 1;
            }
            if self.pos == exponent_start {
                return self.error("empty JSON exponent");
            }
        }
        std::str::from_utf8(&self.bytes[start..self.pos])
            .map(str::to_owned)
            .map_err(|_| "number is not UTF-8".to_owned())
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        self.skip_ws();
        match self.bytes.get(self.pos) {
            Some(b'\"') => self.parse_string().map(Value::String),
            Some(b't') if self.bytes.get(self.pos..self.pos + 4) == Some(b"true") => {
                self.pos += 4;
                Ok(Value::Bool(true))
            }
            Some(b'f') if self.bytes.get(self.pos..self.pos + 5) == Some(b"false") => {
                self.pos += 5;
                Ok(Value::Bool(false))
            }
            Some(b'n') if self.bytes.get(self.pos..self.pos + 4) == Some(b"null") => {
                self.pos += 4;
                Ok(Value::Null)
            }
            Some(b'-' | b'0'..=b'9') => self.parse_number().map(Value::Number),
            Some(b'{' | b'[') => self.error("nested JSON is forbidden; records must be flat"),
            _ => self.error("invalid JSON value"),
        }
    }

    fn parse_object(mut self) -> Result<BTreeMap<String, Value>, String> {
        self.take(b'{')?;
        let mut fields = BTreeMap::new();
        self.skip_ws();
        if self.bytes.get(self.pos) == Some(&b'}') {
            self.pos += 1;
        } else {
            loop {
                let key = self.parse_string()?;
                self.take(b':')?;
                let value = self.parse_value()?;
                if fields.insert(key.clone(), value).is_some() {
                    return self.error(format!("duplicate key {key:?}"));
                }
                self.skip_ws();
                match self.bytes.get(self.pos) {
                    Some(b',') => self.pos += 1,
                    Some(b'}') => {
                        self.pos += 1;
                        break;
                    }
                    _ => return self.error("expected ',' or '}'"),
                }
            }
        }
        self.skip_ws();
        if self.pos != self.bytes.len() {
            return self.error("trailing content after JSON object");
        }
        Ok(fields)
    }
}

fn require_string<'a>(row: &'a BTreeMap<String, Value>, key: &str) -> Result<&'a str, String> {
    match row.get(key) {
        Some(Value::String(value)) => Ok(value),
        Some(_) => Err(format!("{key} is not a string")),
        None => Err(format!("missing {key}")),
    }
}

fn require_bool(row: &BTreeMap<String, Value>, key: &str) -> Result<bool, String> {
    match row.get(key) {
        Some(Value::Bool(value)) => Ok(*value),
        Some(_) => Err(format!("{key} is not a Boolean")),
        None => Err(format!("missing {key}")),
    }
}

fn optional_bool(row: &BTreeMap<String, Value>, key: &str) -> Result<Option<bool>, String> {
    match row.get(key) {
        Some(Value::Bool(value)) => Ok(Some(*value)),
        Some(_) => Err(format!("{key} is not a Boolean")),
        None => Ok(None),
    }
}

fn require_u64(row: &BTreeMap<String, Value>, key: &str) -> Result<u64, String> {
    match row.get(key) {
        Some(Value::Number(value)) => value
            .parse::<u64>()
            .map_err(|_| format!("{key} is not a canonical nonnegative integer")),
        Some(_) => Err(format!("{key} is not numeric")),
        None => Err(format!("missing {key}")),
    }
}

#[derive(Clone, Copy, Debug)]
struct Rational {
    numerator: u64,
    denominator: u64,
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn parse_rational(text: &str) -> Result<Rational, String> {
    let (numerator, denominator) = text
        .split_once('/')
        .ok_or_else(|| "small-division weight must be an explicit NUM/DEN rational".to_owned())?;
    if denominator.contains('/') {
        return Err("small-division weight has more than one slash".to_owned());
    }
    let mut numerator = numerator
        .parse::<u64>()
        .map_err(|_| "small-division numerator is not a nonnegative integer".to_owned())?;
    let mut denominator = denominator
        .parse::<u64>()
        .map_err(|_| "small-division denominator is not a positive integer".to_owned())?;
    if denominator == 0 {
        return Err("small-division denominator is zero".to_owned());
    }
    let divisor = gcd(numerator, denominator);
    numerator /= divisor;
    denominator /= divisor;
    Ok(Rational {
        numerator,
        denominator,
    })
}

#[derive(Clone, Debug)]
struct Phase {
    name: &'static str,
    fq_mul: u64,
    fq_inv: u64,
    fq_add: u64,
    fq_small_div: u64,
    f3_mul: u64,
    f3_sq: u64,
    f3_inv: u64,
    batch_inv: u64,
    group_add: u64,
}

impl Phase {
    fn parse(row: &BTreeMap<String, Value>, name: &'static str) -> Result<Self, String> {
        let field = |suffix: &str| require_u64(row, &format!("{name}.{suffix}"));
        Ok(Self {
            name,
            fq_mul: field("fq_mul")?,
            fq_inv: field("fq_inv")?,
            fq_add: field("fq_add")?,
            fq_small_div: field("fq_small_div")?,
            f3_mul: field("f3_mul")?,
            f3_sq: field("f3_sq")?,
            f3_inv: field("f3_inv")?,
            batch_inv: field("batch_inv")?,
            group_add: field("group_add")?,
        })
    }

    fn all_zero(&self) -> bool {
        self.fq_mul == 0
            && self.fq_inv == 0
            && self.fq_add == 0
            && self.fq_small_div == 0
            && self.f3_mul == 0
            && self.f3_sq == 0
            && self.f3_inv == 0
            && self.batch_inv == 0
            && self.group_add == 0
    }

    fn mul_plus_small_div_numerator(&self, weight: Rational) -> Result<u128, String> {
        let mul = (self.fq_mul as u128)
            .checked_mul(weight.denominator as u128)
            .ok_or_else(|| "weighted multiplication count overflow".to_owned())?;
        let division = (self.fq_small_div as u128)
            .checked_mul(weight.numerator as u128)
            .ok_or_else(|| "weighted small-division count overflow".to_owned())?;
        mul.checked_add(division)
            .ok_or_else(|| "weighted phase count overflow".to_owned())
    }
}

#[derive(Clone, Debug)]
struct RhoRow {
    line: usize,
    arm: String,
    kernel: String,
    phases: [Phase; 3],
}

fn validate_rho(row: &BTreeMap<String, Value>, line: usize) -> Result<RhoRow, String> {
    let arm = require_string(row, "arm")?.to_owned();
    let kernel = require_string(row, "field_kernel")?.to_owned();
    let authoritative = require_bool(row, "count_authoritative")?;
    let counted = if arm.starts_with("strong-") {
        true
    } else if arm.starts_with("fast-") || arm.starts_with("bare-") {
        false
    } else {
        return Err(format!("line {line}: unknown rho arm class {arm:?}"));
    };
    if authoritative != counted {
        return Err(format!(
            "line {line}: arm {arm:?} has count_authoritative={authoritative}, expected {counted}"
        ));
    }
    if let Some(field_authoritative) = optional_bool(row, "field_count_authoritative")? {
        if field_authoritative != counted {
            return Err(format!(
                "line {line}: arm {arm:?} has field_count_authoritative={field_authoritative}, expected {counted}"
            ));
        }
    }

    let phases = [
        Phase::parse(row, "setup")?,
        Phase::parse(row, "walk")?,
        Phase::parse(row, "verify")?,
    ];
    if !counted && phases.iter().any(|phase| !phase.all_zero()) {
        return Err(format!(
            "line {line}: non-authoritative arm {arm:?} contains nonzero field counts"
        ));
    }
    for phase in &phases {
        match kernel.as_str() {
            "karatsuba6" if phase.fq_small_div != 0 => {
                return Err(format!(
                    "line {line}: karatsuba6 {}.fq_small_div must be zero, got {}",
                    phase.name, phase.fq_small_div
                ));
            }
            "karatsuba6" => {}
            "evaluation5" => {
                let expected = phase
                    .f3_mul
                    .checked_mul(4)
                    .ok_or_else(|| format!("line {line}: {} f3_mul overflow", phase.name))?;
                if phase.fq_small_div != expected {
                    return Err(format!(
                        "line {line}: evaluation5 {}.fq_small_div={} but 4*f3_mul={expected}",
                        phase.name, phase.fq_small_div
                    ));
                }
            }
            _ => return Err(format!("line {line}: unknown field_kernel {kernel:?}")),
        }
    }
    Ok(RhoRow {
        line,
        arm,
        kernel,
        phases,
    })
}

fn analyze(text: &str) -> Result<Vec<RhoRow>, String> {
    let mut rho_rows = Vec::new();
    for (index, raw) in text.lines().enumerate() {
        let line = index + 1;
        if raw.trim().is_empty() {
            continue;
        }
        let row = Parser::new(raw)
            .parse_object()
            .map_err(|error| format!("line {line}: {error}"))?;
        if require_string(&row, "rec").map_err(|error| format!("line {line}: {error}"))? == "rho" {
            rho_rows.push(validate_rho(&row, line)?);
        }
    }
    if rho_rows.is_empty() {
        return Err("JSONL contains no rho rows".to_owned());
    }
    Ok(rho_rows)
}

fn print_report(rows: &[RhoRow], weight: Rational) -> Result<(), String> {
    println!(
        "validated_rho_rows={} explicit_small_div_weight={}/{}",
        rows.len(),
        weight.numerator,
        weight.denominator
    );
    for row in rows {
        for phase in &row.phases {
            println!(
                "line={} arm={} kernel={} phase={} fq_mul={} fq_inv={} fq_add={} fq_small_div={} f3_mul={} f3_sq={} f3_inv={} batch_inv={} group_add={} mul_plus_small_div={}/{}",
                row.line,
                row.arm,
                row.kernel,
                phase.name,
                phase.fq_mul,
                phase.fq_inv,
                phase.fq_add,
                phase.fq_small_div,
                phase.f3_mul,
                phase.f3_sq,
                phase.f3_inv,
                phase.batch_inv,
                phase.group_add,
                phase.mul_plus_small_div_numerator(weight)?,
                weight.denominator
            );
        }
    }
    Ok(())
}

fn replace_once(text: &str, old: &str, new: &str) -> Result<String, String> {
    let position = text
        .find(old)
        .ok_or_else(|| format!("selftest fixture lacks {old:?}"))?;
    let mut out = String::with_capacity(text.len() + new.len().saturating_sub(old.len()));
    out.push_str(&text[..position]);
    out.push_str(new);
    out.push_str(&text[position + old.len()..]);
    Ok(out)
}

fn expect_rejected(label: &str, text: String) -> Result<(), String> {
    match analyze(&text) {
        Ok(_) => Err(format!("corruption {label} was accepted")),
        Err(error) => {
            println!("corruption={label} rejected={error}");
            Ok(())
        }
    }
}

fn selftest(path: &str) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|error| format!("read {path}: {error}"))?;
    let rows = analyze(&text)?;
    if rows.len() != 5 {
        return Err(format!(
            "real fixture has {} rho rows, expected 5",
            rows.len()
        ));
    }
    println!("real_fixture_pass rows={}", rows.len());
    print_report(&rows, parse_rational("1/2")?)?;

    expect_rejected(
        "missing-field-kernel",
        replace_once(&text, "\"field_kernel\":\"karatsuba6\",", "")?,
    )?;
    expect_rejected(
        "missing-small-div",
        replace_once(&text, "\"setup.fq_small_div\":0,", "")?,
    )?;
    expect_rejected(
        "malformed-small-div",
        replace_once(
            &text,
            "\"setup.fq_small_div\":0,",
            "\"setup.fq_small_div\":\"0\",",
        )?,
    )?;
    expect_rejected(
        "unknown-kernel",
        replace_once(
            &text,
            "\"field_kernel\":\"karatsuba6\"",
            "\"field_kernel\":\"unknown\"",
        )?,
    )?;
    expect_rejected(
        "karatsuba-nonzero-small-div",
        replace_once(
            &text,
            "\"setup.fq_small_div\":0,",
            "\"setup.fq_small_div\":1,",
        )?,
    )?;
    expect_rejected(
        "evaluation5-wrong-small-div",
        replace_once(
            &text,
            "\"setup.fq_small_div\":25748,",
            "\"setup.fq_small_div\":25749,",
        )?,
    )?;
    expect_rejected(
        "counted-authority-false",
        replace_once(
            &text,
            "\"count_authoritative\":true",
            "\"count_authoritative\":false",
        )?,
    )?;
    expect_rejected(
        "fast-authority-true",
        replace_once(
            &text,
            "\"count_authoritative\":false",
            "\"count_authoritative\":true",
        )?,
    )?;
    expect_rejected(
        "fast-nonzero-field-count",
        replace_once(&text, "\"setup.fq_mul\":0,", "\"setup.fq_mul\":1,")?,
    )?;
    expect_rejected(
        "unknown-arm-class",
        replace_once(&text, "\"arm\":\"strong-b8\"", "\"arm\":\"mystery-b8\"")?,
    )?;
    expect_rejected(
        "phase-cross-substitution",
        replace_once(
            &text,
            "\"setup.fq_small_div\":25748,",
            "\"setup.fq_small_div\":28136,",
        )?,
    )?;
    expect_rejected(
        "duplicate-key",
        replace_once(
            &text,
            "\"field_kernel\":\"karatsuba6\",",
            "\"field_kernel\":\"karatsuba6\",\"field_kernel\":\"karatsuba6\",",
        )?,
    )?;

    for bad in ["", "1", "1/0", "-1/2", "1/-2", "1.5/2", "1/2/3"] {
        if parse_rational(bad).is_ok() {
            return Err(format!("invalid rational {bad:?} was accepted"));
        }
        println!("invalid_weight={bad:?} rejected");
    }
    let explicit_zero = parse_rational("0/7")?;
    if explicit_zero.numerator != 0 || explicit_zero.denominator != 1 {
        return Err("explicit zero rational did not canonicalize to 0/1".to_owned());
    }
    println!("explicit_zero_weight=0/1 accepted_as_explicit_sensitivity");
    println!("SELFTEST OK");
    Ok(())
}

fn usage(program: &str) -> String {
    format!(
        "usage: {program} check <small-div-NUM/DEN> <flat.jsonl> | {program} --selftest <flat.jsonl>"
    )
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, flag, path] if flag == "--selftest" => selftest(path),
        [_, command, weight, path] if command == "check" => {
            let weight = parse_rational(weight)?;
            let text = fs::read_to_string(path).map_err(|error| format!("read {path}: {error}"))?;
            let rows = analyze(&text)?;
            print_report(&rows, weight)
        }
        _ => Err(usage(
            args.first()
                .map(String::as_str)
                .unwrap_or("r17_native_analyzer"),
        )),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("r17-native-analyzer: {error}");
        std::process::exit(2);
    }
}
