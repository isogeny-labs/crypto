use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const PROTOCOL_STATES: [&str; 6] = [
    "blocked",
    "executed-pass",
    "executed-reject",
    "implemented-unmeasured",
    "queued-after-dependency",
    "screened",
];

#[derive(Debug)]
enum Json {
    Null,
    Bool,
    Number(i64),
    String(String),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
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

    fn parse(mut self) -> Result<Json, String> {
        let value = self.value()?;
        self.ws();
        if self.pos != self.bytes.len() {
            return Err(format!("trailing JSON at byte {}", self.pos));
        }
        Ok(value)
    }

    fn ws(&mut self) {
        while self
            .bytes
            .get(self.pos)
            .is_some_and(|b| matches!(b, b' ' | b'\n' | b'\r' | b'\t'))
        {
            self.pos += 1;
        }
    }

    fn value(&mut self) -> Result<Json, String> {
        self.ws();
        match self.bytes.get(self.pos).copied() {
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => self.string().map(Json::String),
            Some(b'-' | b'0'..=b'9') => self.number(),
            Some(b't') => self.literal(b"true", Json::Bool),
            Some(b'f') => self.literal(b"false", Json::Bool),
            Some(b'n') => self.literal(b"null", Json::Null),
            Some(other) => Err(format!(
                "unexpected JSON byte {:?} at {}",
                other as char, self.pos
            )),
            None => Err("unexpected end of JSON".into()),
        }
    }

    fn literal(&mut self, expected: &[u8], value: Json) -> Result<Json, String> {
        if self.bytes.get(self.pos..self.pos + expected.len()) == Some(expected) {
            self.pos += expected.len();
            Ok(value)
        } else {
            Err(format!("invalid literal at byte {}", self.pos))
        }
    }

    fn object(&mut self) -> Result<Json, String> {
        self.pos += 1;
        let mut out = BTreeMap::new();
        loop {
            self.ws();
            if self.bytes.get(self.pos) == Some(&b'}') {
                self.pos += 1;
                return Ok(Json::Object(out));
            }
            let key = self.string()?;
            self.ws();
            if self.bytes.get(self.pos) != Some(&b':') {
                return Err(format!("expected colon at byte {}", self.pos));
            }
            self.pos += 1;
            let value = self.value()?;
            if out.insert(key.clone(), value).is_some() {
                return Err(format!("duplicate JSON key {key:?}"));
            }
            self.ws();
            match self.bytes.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(Json::Object(out));
                }
                _ => return Err(format!("expected comma or object end at byte {}", self.pos)),
            }
        }
    }

    fn array(&mut self) -> Result<Json, String> {
        self.pos += 1;
        let mut out = Vec::new();
        loop {
            self.ws();
            if self.bytes.get(self.pos) == Some(&b']') {
                self.pos += 1;
                return Ok(Json::Array(out));
            }
            out.push(self.value()?);
            self.ws();
            match self.bytes.get(self.pos) {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    return Ok(Json::Array(out));
                }
                _ => return Err(format!("expected comma or array end at byte {}", self.pos)),
            }
        }
    }

    fn string(&mut self) -> Result<String, String> {
        self.ws();
        if self.bytes.get(self.pos) != Some(&b'"') {
            return Err(format!("expected string at byte {}", self.pos));
        }
        self.pos += 1;
        let mut out = String::new();
        while let Some(byte) = self.bytes.get(self.pos).copied() {
            self.pos += 1;
            match byte {
                b'"' => return Ok(out),
                b'\\' => {
                    let escaped = self
                        .bytes
                        .get(self.pos)
                        .copied()
                        .ok_or_else(|| "truncated string escape".to_string())?;
                    self.pos += 1;
                    match escaped {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{0008}'),
                        b'f' => out.push('\u{000c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let digits = self
                                .bytes
                                .get(self.pos..self.pos + 4)
                                .ok_or_else(|| "truncated unicode escape".to_string())?;
                            let digits = std::str::from_utf8(digits)
                                .map_err(|_| "non-UTF8 unicode escape".to_string())?;
                            let code = u32::from_str_radix(digits, 16)
                                .map_err(|_| "invalid unicode escape".to_string())?;
                            out.push(
                                char::from_u32(code)
                                    .ok_or_else(|| "invalid unicode scalar".to_string())?,
                            );
                            self.pos += 4;
                        }
                        _ => return Err(format!("invalid escape at byte {}", self.pos - 1)),
                    }
                }
                0x00..=0x1f => return Err("control byte in JSON string".into()),
                0x20..=0x7f => out.push(byte as char),
                _ => {
                    let start = self.pos - 1;
                    let rest = std::str::from_utf8(&self.bytes[start..])
                        .map_err(|_| "invalid UTF-8 in JSON string".to_string())?;
                    let ch = rest
                        .chars()
                        .next()
                        .ok_or_else(|| "truncated UTF-8".to_string())?;
                    out.push(ch);
                    self.pos = start + ch.len_utf8();
                }
            }
        }
        Err("unterminated JSON string".into())
    }

    fn number(&mut self) -> Result<Json, String> {
        let start = self.pos;
        if self.bytes.get(self.pos) == Some(&b'-') {
            self.pos += 1;
        }
        while self.bytes.get(self.pos).is_some_and(|b| b.is_ascii_digit()) {
            self.pos += 1;
        }
        let text = std::str::from_utf8(&self.bytes[start..self.pos])
            .map_err(|_| "non-UTF8 number".to_string())?;
        let n = text
            .parse::<i64>()
            .map_err(|_| format!("invalid integer {text:?}"))?;
        Ok(Json::Number(n))
    }
}

fn object<'a>(value: &'a Json, context: &str) -> Result<&'a BTreeMap<String, Json>, String> {
    if let Json::Object(value) = value {
        Ok(value)
    } else {
        Err(format!("{context} must be an object"))
    }
}

fn string<'a>(
    obj: &'a BTreeMap<String, Json>,
    key: &str,
    context: &str,
) -> Result<&'a str, String> {
    match obj.get(key) {
        Some(Json::String(value)) if !value.trim().is_empty() => Ok(value),
        _ => Err(format!("{context}.{key} must be a nonempty string")),
    }
}

fn array<'a>(
    obj: &'a BTreeMap<String, Json>,
    key: &str,
    context: &str,
) -> Result<&'a [Json], String> {
    match obj.get(key) {
        Some(Json::Array(value)) => Ok(value),
        _ => Err(format!("{context}.{key} must be an array")),
    }
}

fn string_array(
    obj: &BTreeMap<String, Json>,
    key: &str,
    context: &str,
) -> Result<Vec<String>, String> {
    array(obj, key, context)?
        .iter()
        .enumerate()
        .map(|(index, item)| match item {
            Json::String(value) if !value.trim().is_empty() => Ok(value.clone()),
            _ => Err(format!(
                "{context}.{key}[{index}] must be a nonempty string"
            )),
        })
        .collect()
}

fn nonempty_array<'a>(
    obj: &'a BTreeMap<String, Json>,
    key: &str,
    context: &str,
) -> Result<&'a [Json], String> {
    let values = array(obj, key, context)?;
    if values.is_empty() {
        Err(format!("{context}.{key} must be nonempty"))
    } else {
        Ok(values)
    }
}

fn range_endpoint(value: &Json, context: &str) -> Result<usize, String> {
    match value {
        Json::Number(n) if (0..=99).contains(n) => Ok(*n as usize),
        Json::String(id) if id.len() == 3 && id.starts_with('R') => id[1..]
            .parse::<usize>()
            .ok()
            .filter(|n| *n <= 99)
            .ok_or_else(|| format!("{context} has invalid lane endpoint {id:?}")),
        _ => Err(format!("{context} must be an integer 0..99 or R00..R99")),
    }
}

fn validate_target(lane: &BTreeMap<String, Json>, context: &str) -> Result<(), String> {
    match lane.get("target") {
        Some(Json::String(value)) if !value.trim().is_empty() => Ok(()),
        Some(Json::Array(values)) if !values.is_empty() => {
            for (index, value) in values.iter().enumerate() {
                match value {
                    Json::String(value) if !value.trim().is_empty() => {}
                    _ => {
                        return Err(format!(
                            "{context}.target[{index}] must be a nonempty string"
                        ))
                    }
                }
            }
            Ok(())
        }
        _ => Err(format!(
            "{context}.target must be a nonempty string or string array"
        )),
    }
}

fn resolve_evidence(path: &str) -> Option<PathBuf> {
    let direct = PathBuf::from(path);
    if direct.exists() {
        return Some(direct);
    }
    let campaign = Path::new("research/quartic_ghs_r0_r99_20261119").join(path);
    campaign.exists().then_some(campaign)
}

fn heading_slug(heading: &str) -> String {
    let mut out = String::new();
    for ch in heading.trim().chars().flat_map(char::to_lowercase) {
        if ch.is_alphanumeric() || ch == '-' || ch == '_' {
            out.push(ch);
        } else if ch.is_whitespace() {
            out.push('-');
        }
    }
    out
}

fn evidence_exists(reference: &str) -> bool {
    let (path, anchor) = reference
        .split_once('#')
        .map_or((reference, None), |(path, anchor)| (path, Some(anchor)));
    let Some(path) = resolve_evidence(path) else {
        return false;
    };
    let Some(anchor) = anchor else {
        return true;
    };
    let Ok(text) = fs::read_to_string(path) else {
        return false;
    };
    text.lines().any(|line| {
        let hashes = line.bytes().take_while(|byte| *byte == b'#').count();
        (1..=6).contains(&hashes)
            && line.as_bytes().get(hashes) == Some(&b' ')
            && heading_slug(&line[hashes + 1..]) == anchor
    })
}

fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))
}

fn require_markers(name: &str, text: &str, markers: &[&str]) -> Result<(), String> {
    let missing: Vec<_> = markers
        .iter()
        .copied()
        .filter(|marker| !text.contains(marker))
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("{name} lacks required markers {missing:?}"))
    }
}

fn section<'a>(text: &'a str, start: &str, end: &str) -> Result<&'a str, String> {
    let start_at = text
        .find(start)
        .ok_or_else(|| format!("missing section start {start:?}"))?;
    let tail = &text[start_at..];
    let end_at = tail
        .find(end)
        .ok_or_else(|| format!("missing section end {end:?}"))?;
    Ok(&tail[..end_at])
}

fn quoted_count(text: &str) -> usize {
    let mut count = 0;
    let mut quoted = false;
    let mut escaped = false;
    for byte in text.bytes() {
        if quoted && escaped {
            escaped = false;
        } else if quoted && byte == b'\\' {
            escaped = true;
        } else if byte == b'"' {
            quoted = !quoted;
            if quoted {
                count += 1;
            }
        }
    }
    count
}

fn validate_fragment(
    path: &Path,
    seen: &mut BTreeSet<usize>,
    state_counts: &mut BTreeMap<String, usize>,
    status_counts: &mut BTreeMap<String, usize>,
    evidence_references: &mut usize,
) -> Result<(usize, usize, usize), String> {
    let root = Parser::new(&read(path)?).parse()?;
    let root = object(&root, &path.display().to_string())?;
    if string(root, "schema", "root")? != "ghs-r0-r99-range/v1" {
        return Err(format!("{} has wrong schema", path.display()));
    }
    if string(root, "claim_scope", "root")? != "producer-side deterministic screens only" {
        return Err(format!("{} has wrong claim_scope", path.display()));
    }
    let range = array(root, "range", "root")?;
    if range.len() != 2 {
        return Err(format!(
            "{} range must contain two integers",
            path.display()
        ));
    }
    let first = range_endpoint(&range[0], &format!("{} range start", path.display()))?;
    let last = range_endpoint(&range[1], &format!("{} range end", path.display()))?;
    if last < first {
        return Err(format!("{} range end precedes its start", path.display()));
    }
    let lanes = nonempty_array(root, "lanes", "root")?;
    if lanes.len() != last - first + 1 {
        return Err(format!(
            "{} range R{first:02}-R{last:02} needs {} lanes, found {}",
            path.display(),
            last - first + 1,
            lanes.len()
        ));
    }
    for (offset, lane) in lanes.iter().enumerate() {
        let expected_number = first + offset;
        let context = format!("{} lane {offset}", path.display());
        let lane = object(lane, &context)?;
        let id = string(lane, "id", &context)?;
        let expected_id = format!("R{expected_number:02}");
        if id != expected_id {
            return Err(format!("{context}.id is {id:?}, expected {expected_id:?}"));
        }
        if !seen.insert(expected_number) {
            return Err(format!("duplicate global lane {expected_id}"));
        }
        for field in [
            "title",
            "mechanism",
            "deterministic_falsifier",
            "measurement_gate",
        ] {
            string(lane, field, &context)?;
        }
        validate_target(lane, &context)?;
        string_array(lane, "prerequisites", &context)?;
        let evidence = string_array(lane, "evidence", &context)?;
        if evidence.is_empty() {
            return Err(format!(
                "{context}.evidence must contain at least one reference"
            ));
        }
        for evidence_path in evidence {
            *evidence_references += 1;
            if !evidence_exists(&evidence_path) {
                return Err(format!(
                    "{context} evidence reference is unresolved: {evidence_path}"
                ));
            }
        }
        let state = string(lane, "state", &context)?;
        let Some(state_count) = state_counts.get_mut(state) else {
            return Err(format!(
                "{context}.state must be exactly one of {:?}, found {state:?}",
                PROTOCOL_STATES
            ));
        };
        *state_count += 1;
        let status = string(lane, "status", &context)?;
        *status_counts.entry(status.to_string()).or_default() += 1;
    }
    Ok((first, last, lanes.len()))
}

fn run() -> Result<(), String> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let expect_all = if args.first().is_some_and(|arg| arg == "--expect-all") {
        args.remove(0);
        true
    } else {
        false
    };
    let separator = args
        .iter()
        .position(|arg| arg == "--")
        .ok_or_else(|| "usage: r84_99 [--expect-all] REGISTRY... -- OPT SOLVERS README BACKEND VALIDATION BENCHREC".to_string())?;
    let source_args = args.split_off(separator + 1);
    args.pop();
    if args.is_empty() || source_args.len() != 6 {
        return Err("usage: r84_99 [--expect-all] REGISTRY... -- OPT SOLVERS README BACKEND VALIDATION BENCHREC".into());
    }

    let mut seen = BTreeSet::new();
    let mut state_counts: BTreeMap<String, usize> = PROTOCOL_STATES
        .iter()
        .map(|state| ((*state).to_string(), 0))
        .collect();
    let mut status_counts = BTreeMap::new();
    let mut evidence_references = 0;
    let mut spans = Vec::new();
    let mut lanes = 0;
    for registry in &args {
        let span = validate_fragment(
            Path::new(registry),
            &mut seen,
            &mut state_counts,
            &mut status_counts,
            &mut evidence_references,
        )?;
        lanes += span.2;
        spans.push(span);
    }
    if expect_all {
        let expected: BTreeSet<_> = (0..=99).collect();
        if seen != expected {
            let missing: Vec<_> = expected.difference(&seen).collect();
            let extra: Vec<_> = seen.difference(&expected).collect();
            return Err(format!(
                "R0-R99 coverage mismatch: missing={missing:?}, extra={extra:?}"
            ));
        }
    }

    let source_paths: Vec<PathBuf> = source_args.iter().map(PathBuf::from).collect();
    let optimization = read(&source_paths[0])?;
    let solvers = read(&source_paths[1])?;
    let readme = read(&source_paths[2])?;
    let backend = read(&source_paths[3])?;
    let validation = read(&source_paths[4])?;
    let benchrec = read(&source_paths[5])?;

    require_markers(
        "optimization inventory",
        &optimization,
        &[
            "INVENTORY_SCHEMA",
            "coverage_rule",
            "promotion_rule",
            "PDP3_ENGINES",
            "PKM_TOWER_ENGINES",
            "require_all_development_coverage",
            "missing_methods",
            "missing_nested",
        ],
    )?;
    require_markers(
        "solver registry",
        &solvers,
        &[
            "f4-f2",
            "matrix-f4",
            "matrix-f5",
            "inherited-f4",
            "sat-cdcl",
            "fes-f2",
            "fes-f2-gpu",
            "fes-f2-wide",
            "exhaustive",
            "m4gb-f2",
            "wdsat_oracle",
            "Never guess `Unsatisfiable`",
        ],
    )?;
    require_markers(
        "ecbench contract",
        &readme,
        &[
            "cold_end_to_end",
            "--replay-all",
            "matrix-f5",
            "f6-ic",
            "rootfind-mitm",
            "rootfind-pairs",
            "Gray enumeration",
        ],
    )?;
    require_markers(
        "backend contract",
        &backend,
        &[
            "Magma/F5",
            "msolve",
            "FGb",
            "FLINT/NTL/Magma",
            "same-target A/A",
        ],
    )?;
    require_markers(
        "validation gate",
        &validation,
        &[
            "replay_certificate",
            "validator_report",
            "rho_reference",
            "VALIDATION.json",
        ],
    )?;
    require_markers(
        "benchmark records",
        &benchrec,
        &[
            "bench.record/v1",
            "wall_s",
            "cpu_s",
            "VALIDATION.json",
            "invalid",
            "censored",
            "S3",
        ],
    )?;

    let policies = section(&optimization, "const POLICIES", "];\n\nfn parameter_role")?
        .matches("policy!(")
        .count();
    let solver_registry = section(&solvers, "pub fn solver_registry()", "/// Look one up.")?;
    let solver_count = solver_registry.matches("Box::new(").count();
    let pdp3_count = quoted_count(section(
        &optimization,
        "const PDP3_ENGINES",
        "];\n\nconst PKM_TOWER_ENGINES",
    )?);
    let pkm_count = quoted_count(section(
        &optimization,
        "const PKM_TOWER_ENGINES",
        "];\n\nfn nested_algorithms",
    )?);
    if policies == 0 || solver_count == 0 || pdp3_count == 0 || pkm_count == 0 {
        return Err("source inventory unexpectedly empty".into());
    }

    println!(
        "registry fragments={} lanes={} spans={:?} expect_all={}",
        args.len(),
        lanes,
        spans,
        expect_all
    );
    println!(
        "source policy_declarations={} solver_registry_entries={} pdp3_engines={} pkm_tower_engines={}",
        policies, solver_count, pdp3_count, pkm_count
    );
    println!(
        "schema=ghs-r0-r99-range/v1 claim_scope=producer-side_deterministic_screens_only required_fields=PASS state_valid={} status_nonempty={} evidence_nonempty={}",
        state_counts.values().sum::<usize>(),
        status_counts.values().sum::<usize>(),
        lanes
    );
    println!(
        "state protocol_values={} counts={:?}",
        state_counts.len(),
        state_counts
    );
    println!(
        "status detail_values={} nonempty={} preserved=PASS",
        status_counts.len(),
        status_counts.values().sum::<usize>()
    );
    println!(
        "evidence references={} nonempty_lanes={} dangling=0 path_anchor_resolution=PASS",
        evidence_references, lanes
    );
    println!("coverage markers: PASS");
    println!("R0-R99 range registry: PASS");
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("R0-R99 range registry: FAIL: {error}");
        std::process::exit(1);
    }
}
