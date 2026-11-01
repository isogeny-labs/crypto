//! Exact classifier for `research/dreg_high_arity_20261101`.
//!
//! Usage:
//! `dreg_high_arity_analyze DRAW_DIR OUTPUT_ROOT [--require LOGICAL_NAME]`
//!
//! A required logical run exits 0 only for terminal valid evidence, 75 while
//! retryable/missing below the four-attempt cap, 76 for terminal unmeasured
//! evidence (including a deterministic incomplete status-zero row), and 65
//! for malformed, contradictory, or otherwise invalid evidence.  Without
//! `--require`, the same precedence is applied across the complete campaign.
//!
//! This program interprets no clocks. It validates the committed draw prefix,
//! the historical identity replay, six satisfiable controls, and the twelve
//! registered UNSAT scans. Missing, interrupted, capped, or incomplete work is
//! `unmeasured`; a contradictory completed record is `invalid_record`. Only a
//! complete saturated degree-four scan can be `above_four`.

use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

const MAX_DRAWS: u64 = 4096;

#[derive(Clone, Copy)]
struct Cell {
    m: u64,
    n: u64,
    ell: u64,
}

impl Cell {
    const fn new(m: u64, n: u64, ell: u64) -> Self {
        Self { m, n, ell }
    }

    fn id(self) -> String {
        format!("m{}-n{}-l{}", self.m, self.n, self.ell)
    }

    fn source_cell(self) -> String {
        format!("n{}l{}m{}", self.n, self.ell, self.m)
    }

    fn n_vars(self) -> u64 {
        (self.m - 2) * self.n + self.m * self.ell
    }

    fn draw_file(self, draw_dir: &Path) -> PathBuf {
        draw_dir.join(format!("{}.jsonl", self.id()))
    }
}

const CELLS: [Cell; 3] = [
    Cell::new(5, 11, 2),
    Cell::new(5, 13, 3),
    Cell::new(6, 11, 2),
];

#[derive(Clone)]
struct DrawRecord {
    draw: u64,
    solutions: u64,
    outcome: Value,
}

#[derive(Serialize)]
struct PanelSummary {
    cell: String,
    state: String,
    path: String,
    records: usize,
    unsat_draws: Vec<u64>,
    satisfiable_draws: Vec<u64>,
    errors: Vec<String>,
}

struct Panel {
    summary: PanelSummary,
    unsat: Vec<DrawRecord>,
    sat: Vec<DrawRecord>,
}

#[derive(Clone, Serialize)]
struct AttemptSummary {
    attempt: String,
    exit_status: Option<i32>,
    state: String,
    reason: String,
    output: String,
    status: String,
    metadata: String,
}

#[derive(Clone, Serialize)]
struct ProcessRow {
    run_id: String,
    kind: String,
    cell: String,
    draw: Option<u64>,
    classification: String,
    reason: String,
    selected_attempt: Option<String>,
    attempts: Vec<AttemptSummary>,
}

#[derive(Clone, Serialize)]
struct CellSummary {
    cell: String,
    classification: String,
    closure_by_four: usize,
    above_four: usize,
    unmeasured: usize,
    invalid: usize,
    controls_passed: usize,
}

#[derive(Serialize)]
struct PackageSummary {
    package_id: String,
    role: String,
    classification: String,
    reason: String,
    subprocesses: Vec<String>,
}

struct FrozenContext {
    protocol_path: PathBuf,
    binary_manifest_path: PathBuf,
    protocol_sha256: String,
    source_commit: String,
    ladder_sha256: String,
    closure_sha256: String,
    analyzer_sha256: String,
    container_image_digest: String,
    rustc_version: String,
}

#[derive(Serialize)]
struct ProvenanceSummary {
    state: String,
    protocol: String,
    binary_manifest: String,
    reason: String,
}

fn field_u64(v: &Value, key: &str) -> Option<u64> {
    v.get(key).and_then(Value::as_u64)
}

fn field_bool(v: &Value, key: &str) -> Option<bool> {
    v.get(key).and_then(Value::as_bool)
}

fn field_i64(v: &Value, key: &str) -> Option<i64> {
    v.get(key).and_then(Value::as_i64)
}

fn parse_json_lines(path: &Path) -> Result<Vec<Value>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    text.lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(i, line)| {
            serde_json::from_str(line).map_err(|e| format!("{}:{}: {e}", path.display(), i + 1))
        })
        .collect()
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut digest = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        if n == 0 {
            break;
        }
        digest.update(&buffer[..n]);
    }
    let digest = digest.finalize();
    Ok(format!("{digest:x}"))
}

fn is_lower_hex(value: &str, len: usize) -> bool {
    value.len() == len
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn study_file(draw_dir: &Path, name: &str) -> PathBuf {
    let beside_draws = draw_dir.parent().map(|p| p.join(name)).unwrap_or_default();
    if beside_draws.is_file() {
        return beside_draws;
    }
    PathBuf::from("research/dreg_high_arity_20261101").join(name)
}

fn string_at<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, String> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {pointer}"))
}

impl FrozenContext {
    fn load(draw_dir: &Path) -> Result<Self, String> {
        let protocol_path = study_file(draw_dir, "protocol.json");
        let binary_manifest_path = study_file(draw_dir, "binary.sha256");
        let protocol_text = fs::read_to_string(&protocol_path)
            .map_err(|e| format!("{}: {e}", protocol_path.display()))?;
        let protocol: Value = serde_json::from_str(&protocol_text)
            .map_err(|e| format!("{}: {e}", protocol_path.display()))?;
        if string_at(&protocol, "/freeze/status")? != "frozen" {
            return Err("protocol freeze status is not frozen".into());
        }
        if protocol
            .pointer("/freeze/filed_before_affected_data")
            .and_then(Value::as_bool)
            != Some(true)
        {
            return Err("protocol was not filed before affected data".into());
        }
        let protocol_sha256 = string_at(&protocol, "/freeze/protocol_sha256")?.to_string();
        let source_commit = string_at(&protocol, "/freeze/commit")?.to_string();
        if !is_lower_hex(&protocol_sha256, 64) || !is_lower_hex(&source_commit, 40) {
            return Err("protocol freeze hashes are malformed".into());
        }
        let mut body = protocol.clone();
        body.as_object_mut()
            .ok_or_else(|| "protocol is not an object".to_string())?
            .remove("freeze");
        let canonical = serde_json::to_vec(&body).map_err(|e| e.to_string())?;
        if sha256_bytes(&canonical) != protocol_sha256 {
            return Err("protocol body does not match its frozen SHA-256".into());
        }

        let arms = protocol
            .get("arms")
            .and_then(Value::as_array)
            .ok_or_else(|| "protocol arms are missing".to_string())?;
        if arms.len() != 5 {
            return Err(format!("protocol has {} arms, expected five", arms.len()));
        }
        let first = &arms[0];
        let closure_sha256 = string_at(first, "/instrument/binary_sha256")?.to_string();
        let ladder_sha256 = string_at(first, "/instrument/flags/ladder_binary_sha256")?.to_string();
        let analyzer_sha256 =
            string_at(first, "/instrument/flags/analyzer_binary_sha256")?.to_string();
        let container_image_digest =
            string_at(first, "/instrument/flags/container_image_digest")?.to_string();
        let rustc_version = string_at(first, "/instrument/flags/rustc_version")?.to_string();
        for hash in [&closure_sha256, &ladder_sha256, &analyzer_sha256] {
            if !is_lower_hex(hash, 64) {
                return Err("protocol contains a malformed binary SHA-256".into());
            }
        }
        let expected_env = json!({
            "F4_F2_MAX_ROWS": "50000000",
            "F4_F2_MAX_COLS": "50000000",
            "RAYON_NUM_THREADS": "4",
            "DREG_CKPT_SECONDS": "300"
        });
        for arm in arms {
            if string_at(arm, "/instrument/commit")? != source_commit
                || string_at(arm, "/instrument/binary_sha256")? != closure_sha256
                || string_at(arm, "/instrument/flags/ladder_binary_sha256")? != ladder_sha256
                || string_at(arm, "/instrument/flags/analyzer_binary_sha256")? != analyzer_sha256
                || string_at(arm, "/instrument/flags/container_image_digest")?
                    != container_image_digest
                || string_at(arm, "/instrument/flags/rustc_version")? != rustc_version
            {
                return Err("protocol arms disagree on frozen instrument identity".into());
            }
            if arm.pointer("/instrument/env") != Some(&expected_env)
                || arm.pointer("/instrument/threads").and_then(Value::as_u64) != Some(4)
                || string_at(arm, "/instrument/flags/address_space_bytes")? != "32000000000"
            {
                return Err("protocol arm has unexpected environment or limits".into());
            }
            let expected_cols = if arm.get("role").and_then(Value::as_str) == Some("reference") {
                "155000"
            } else {
                "400000"
            };
            if string_at(arm, "/instrument/flags/max_cols")? != expected_cols {
                return Err("protocol arm has an unexpected column cap".into());
            }
        }
        if protocol
            .pointer("/execution/timeout_seconds")
            .and_then(Value::as_u64)
            != Some(86_400)
            || protocol
                .pointer("/execution/checkpoint/grace_seconds")
                .and_then(Value::as_u64)
                != Some(60)
        {
            return Err("protocol execution timeout/grace differs from the frozen limits".into());
        }

        let manifest_text = fs::read_to_string(&binary_manifest_path)
            .map_err(|e| format!("{}: {e}", binary_manifest_path.display()))?;
        let mut manifest = BTreeMap::new();
        for (line_no, line) in manifest_text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields.len() != 2 || !is_lower_hex(fields[0], 64) {
                return Err(format!(
                    "{}:{}: malformed binary manifest row",
                    binary_manifest_path.display(),
                    line_no + 1
                ));
            }
            if manifest.insert(fields[1], fields[0]).is_some() {
                return Err(format!("duplicate binary manifest label {}", fields[1]));
            }
        }
        for (label, expected) in [
            ("dreg_ladder", ladder_sha256.as_str()),
            ("dreg_closure", closure_sha256.as_str()),
            ("dreg_high_arity_analyze", analyzer_sha256.as_str()),
        ] {
            if manifest.get(label).copied() != Some(expected) {
                return Err(format!("binary manifest mismatch for {label}"));
            }
        }
        let current_exe = env::current_exe().map_err(|e| format!("current executable: {e}"))?;
        if sha256_file(&current_exe)? != analyzer_sha256 {
            return Err("running analyzer binary differs from the frozen manifest".into());
        }

        Ok(Self {
            protocol_path,
            binary_manifest_path,
            protocol_sha256,
            source_commit,
            ladder_sha256,
            closure_sha256,
            analyzer_sha256,
            container_image_digest,
            rustc_version,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_attempt_meta(
        &self,
        meta_path: &Path,
        logical: &str,
        attempt: &str,
        expected_input: &Path,
        expected_draw: u64,
        expected_mode: &str,
        expected_max_cols: u64,
        _output_root: &Path,
    ) -> Result<(), String> {
        let text =
            fs::read_to_string(meta_path).map_err(|e| format!("{}: {e}", meta_path.display()))?;
        let meta: Value =
            serde_json::from_str(&text).map_err(|e| format!("{}: {e}", meta_path.display()))?;
        if !meta.is_object() {
            return Err("attempt metadata is not one JSON object".into());
        }
        let expect_string = |pointer: &str, expected: &str| -> Result<(), String> {
            let got = meta.pointer(pointer).and_then(Value::as_str);
            if got == Some(expected) {
                Ok(())
            } else {
                Err(format!(
                    "metadata {pointer} is {got:?}, expected {expected:?}"
                ))
            }
        };
        expect_string("/schema", "dreg.high-arity-attempt/v1")?;
        expect_string("/logical_name", logical)?;
        expect_string("/attempt", attempt)?;
        expect_string("/protocol_sha256", &self.protocol_sha256)?;
        expect_string("/source_commit", &self.source_commit)?;
        expect_string("/binaries/ladder", &self.ladder_sha256)?;
        expect_string("/binaries/closure", &self.closure_sha256)?;
        expect_string("/binaries/analyzer", &self.analyzer_sha256)?;
        expect_string(
            "/build/container_image_digest",
            &self.container_image_digest,
        )?;
        expect_string("/build/rustc_version", &self.rustc_version)?;
        expect_string("/mode", expected_mode)?;
        if meta.get("draw").and_then(Value::as_u64) != Some(expected_draw) {
            return Err("metadata draw differs from the logical draw".into());
        }
        if meta
            .get("parent_environment_cleared")
            .and_then(Value::as_bool)
            != Some(true)
        {
            return Err("metadata does not attest a cleared parent environment".into());
        }
        let expected_env = json!({
            "PATH": "/usr/bin:/bin",
            "F4_F2_MAX_ROWS": "50000000",
            "F4_F2_MAX_COLS": "50000000",
            "RAYON_NUM_THREADS": "4",
            "DREG_CKPT_SECONDS": "300"
        });
        if meta.get("sanitized_env") != Some(&expected_env) {
            return Err("metadata sanitized environment differs from the frozen allowlist".into());
        }
        let expected_limits = json!({
            "address_space_bytes": 32_000_000_000u64,
            "timeout_seconds": 86_400,
            "term_grace_seconds": 60,
            "max_cols": expected_max_cols,
            "degree_to": 4
        });
        if meta.get("limits") != Some(&expected_limits) {
            return Err("metadata limits differ from the registered limits".into());
        }

        let input_path_text = string_at(&meta, "/input/path")?;
        let input_path = Path::new(input_path_text);
        if !input_path.is_absolute() || input_path.file_name() != expected_input.file_name() {
            return Err("metadata input path differs from the registered input".into());
        }
        let input_sha = sha256_file(expected_input)?;
        expect_string("/input/sha256", &input_sha)?;

        let checkpoint_text = string_at(&meta, "/checkpoint_dir")?.to_string();
        let checkpoint = Path::new(&checkpoint_text);
        if !checkpoint.is_absolute()
            || checkpoint.file_name().and_then(|x| x.to_str()) != Some(logical)
            || checkpoint
                .parent()
                .and_then(Path::file_name)
                .and_then(|x| x.to_str())
                != Some("checkpoints")
        {
            return Err(
                "metadata checkpoint_dir is not an absolute checkpoints/<logical> path".into(),
            );
        }
        let satisfiable = expected_mode == "sat";
        let mut argv = Vec::new();
        if satisfiable {
            argv.push(Value::String("--satisfiable".into()));
        }
        for value in [
            "--to".to_string(),
            "4".to_string(),
            "--max-cols".to_string(),
            expected_max_cols.to_string(),
            "--draw".to_string(),
            expected_draw.to_string(),
            "--checkpoint-dir".to_string(),
            checkpoint_text,
            input_path_text.to_string(),
        ] {
            argv.push(Value::String(value));
        }
        if meta.get("argv").and_then(Value::as_array) != Some(&argv) {
            return Err("metadata argv differs from the registered invocation".into());
        }
        Ok(())
    }
}

fn validate_draw_outcome(v: &Value, solutions: u64) -> Result<(), String> {
    let outcome = v
        .get("outcome")
        .and_then(Value::as_object)
        .ok_or_else(|| "outcome is not an object".to_string())?;
    match outcome.get("kind").and_then(Value::as_str) {
        Some("satisfiable") if solutions > 0 && outcome.len() == 1 => Ok(()),
        Some("satisfiable") => {
            Err("satisfiable outcome must have a positive solution count and no extra fields".into())
        }
        Some("at_least")
            if solutions == 0
                && outcome.get("degree").and_then(Value::as_u64) == Some(4)
                && outcome.len() == 2 =>
        {
            Ok(())
        }
        Some("resolved")
            if solutions == 0
                && outcome
                    .get("degree")
                    .and_then(Value::as_u64)
                    .is_some_and(|d| (1..=3).contains(&d))
                && outcome.get("refuted").and_then(Value::as_bool).is_some()
                && outcome.len() == 3 =>
        {
            Ok(())
        }
        Some("caps_hit")
            if solutions == 0
                && outcome.len() == 2
                && match outcome.get("built") {
                    Some(Value::Null) => true,
                    Some(built) => built
                        .as_u64()
                        .is_some_and(|degree| degree <= 2),
                    None => false,
                } =>
        {
            Ok(())
        }
        Some(kind) => Err(format!(
            "outcome kind {kind:?} is inconsistent with the exact solution count or registered degree-3 draw scan"
        )),
        None => Err("outcome.kind is missing".into()),
    }
}

fn draw_panel(cell: Cell, draw_dir: &Path) -> Panel {
    let path = cell.draw_file(draw_dir);
    let mut errors = Vec::new();
    let values = match parse_json_lines(&path) {
        Ok(v) => v,
        Err(e) => {
            let state = if path.is_file() {
                "invalid"
            } else {
                "unmeasured"
            };
            return Panel {
                summary: PanelSummary {
                    cell: cell.id(),
                    state: state.into(),
                    path: path.display().to_string(),
                    records: 0,
                    unsat_draws: Vec::new(),
                    satisfiable_draws: Vec::new(),
                    errors: vec![e],
                },
                unsat: Vec::new(),
                sat: Vec::new(),
            };
        }
    };

    let mut records = Vec::new();
    let mut seen = BTreeSet::new();
    let mut previous = None;
    let mut first_terminal = None;
    let mut unsat_seen = 0usize;
    let mut sat_seen = 0usize;

    for (i, v) in values.iter().enumerate() {
        let Some(draw) = field_u64(v, "draw") else {
            errors.push(format!("record {i} has no integer draw"));
            continue;
        };
        if draw >= MAX_DRAWS {
            errors.push(format!("draw {draw} is outside 0..{}", MAX_DRAWS - 1));
        }
        if !seen.insert(draw) {
            errors.push(format!("duplicate draw {draw}"));
        }
        if previous.map_or(draw != 0, |p| draw != p + 1) {
            errors.push(format!("draw {draw} does not continue the exact prefix"));
        }
        previous = Some(draw);

        for (key, got, want) in [
            ("m", field_u64(v, "m"), cell.m),
            ("n", field_u64(v, "n"), cell.n),
            ("ell", field_u64(v, "ell"), cell.ell),
            ("n_vars", field_u64(v, "n_vars"), cell.n_vars()),
        ] {
            if got != Some(want) {
                errors.push(format!("draw {draw}: {key} is {got:?}, expected {want}"));
            }
        }
        if v.get("cell").and_then(Value::as_str) != Some(cell.source_cell().as_str()) {
            errors.push(format!("draw {draw}: wrong cell label"));
        }
        if field_u64(v, "d_max") != Some(3) {
            errors.push(format!("draw {draw}: d_max is not the registered value 3"));
        }
        if field_u64(v, "n_eqs") != Some((cell.m - 1) * cell.n) {
            errors.push(format!("draw {draw}: wrong equation count"));
        }
        if field_i64(v, "surplus") != Some(cell.n as i64 - (cell.m * cell.ell) as i64) {
            errors.push(format!("draw {draw}: wrong surplus"));
        }
        let field_bound = 1u64 << cell.n;
        match v.get("v_basis").and_then(Value::as_array) {
            Some(basis)
                if basis.len() == cell.ell as usize
                    && basis
                        .iter()
                        .all(|x| x.as_u64().is_some_and(|x| x < field_bound)) => {}
            _ => errors.push(format!("draw {draw}: malformed subspace basis")),
        }
        if field_u64(v, "x_r").is_none_or(|x| x >= field_bound) {
            errors.push(format!("draw {draw}: malformed target abscissa"));
        }

        let Some(solutions) = field_u64(v, "solutions") else {
            errors.push(format!("draw {draw}: missing exact solution count"));
            continue;
        };
        if let Err(e) = validate_draw_outcome(v, solutions) {
            errors.push(format!("draw {draw}: {e}"));
        }
        if v.get("secs").and_then(Value::as_f64).is_none() {
            errors.push(format!("draw {draw}: missing diagnostic seconds field"));
        }
        match (solutions, v.get("ffd")) {
            (_, Some(Value::Null)) => {}
            (0, Some(ffd)) if ffd.as_u64().is_some_and(|d| (1..=3).contains(&d)) => {}
            _ => errors.push(format!("draw {draw}: malformed first-fall degree")),
        }
        if solutions == 0 {
            unsat_seen += 1;
        } else {
            sat_seen += 1;
        }
        if first_terminal.is_none() && unsat_seen >= 4 && sat_seen >= 2 {
            first_terminal = Some(i);
        }
        records.push(DrawRecord {
            draw,
            solutions,
            outcome: v.get("outcome").cloned().unwrap_or(Value::Null),
        });
    }

    if first_terminal != values.len().checked_sub(1) {
        errors.push(match first_terminal {
            Some(i) => format!("stream continued past the first quota-complete record {i}"),
            None => "stream ended before four UNSAT and two SAT records".into(),
        });
    }

    let unsat: Vec<_> = records
        .iter()
        .filter(|r| r.solutions == 0)
        .take(4)
        .cloned()
        .collect();
    let sat: Vec<_> = records
        .iter()
        .filter(|r| r.solutions > 0)
        .take(2)
        .cloned()
        .collect();
    let state = if errors.is_empty() && unsat.len() == 4 && sat.len() == 2 {
        "ready"
    } else if errors.iter().any(|e| {
        !e.contains("ended before") && !e.contains("No such file") && !e.contains("not found")
    }) {
        "invalid"
    } else {
        "unmeasured"
    };
    Panel {
        summary: PanelSummary {
            cell: cell.id(),
            state: state.into(),
            path: path.display().to_string(),
            records: values.len(),
            unsat_draws: unsat.iter().map(|r| r.draw).collect(),
            satisfiable_draws: sat.iter().map(|r| r.draw).collect(),
            errors,
        },
        unsat,
        sat,
    }
}

enum CompletedRecord {
    Value(Value, String, Vec<AttemptSummary>),
    Unmeasured(String, Vec<AttemptSummary>),
    Invalid(String, Vec<AttemptSummary>),
}

#[allow(clippy::too_many_arguments)]
fn completed_record(
    output_root: &Path,
    logical: &str,
    frozen: &Result<FrozenContext, String>,
    expected_input: &Path,
    expected_draw: u64,
    expected_mode: &str,
    expected_max_cols: u64,
) -> CompletedRecord {
    let attempts_dir = output_root.join("runs").join(logical).join("attempts");
    let entries = match fs::read_dir(&attempts_dir) {
        Ok(entries) => entries,
        Err(e) => {
            return CompletedRecord::Unmeasured(
                format!("{}: {e}", attempts_dir.display()),
                Vec::new(),
            );
        }
    };
    let mut dirs = Vec::new();
    let mut layout_errors = Vec::new();
    for entry in entries {
        match entry {
            Ok(e) if e.path().is_dir() => dirs.push(e),
            Ok(e) => layout_errors.push(format!(
                "unexpected non-directory entry {}",
                e.path().display()
            )),
            Err(e) => {
                return CompletedRecord::Invalid(
                    format!("cannot enumerate attempt: {e}"),
                    Vec::new(),
                );
            }
        }
    }
    dirs.sort_by_key(|e| e.file_name());

    let mut summaries = Vec::new();
    let mut successes = Vec::new();
    let mut terminal_zero_attempt: Option<String> = None;
    let mut hard_invalid = layout_errors;
    if dirs.len() > 4 {
        hard_invalid.push(format!(
            "{} attempts exist, exceeding the registered maximum four",
            dirs.len()
        ));
    }
    for (position, entry) in dirs.into_iter().enumerate() {
        let attempt = entry.file_name().to_string_lossy().to_string();
        let dir = entry.path();
        let out = dir.join("out.jsonl");
        let status = dir.join("status");
        let metadata = dir.join("meta.json");
        let empty_allocation = fs::read_dir(&dir)
            .map(|mut entries| entries.next().is_none())
            .unwrap_or(false);
        let expected_attempt = format!("{:04}", position + 1);
        let name_valid = attempt.len() == 4
            && attempt.bytes().all(|b| b.is_ascii_digit())
            && attempt == expected_attempt;
        if !name_valid {
            hard_invalid.push(format!(
                "attempt directory {attempt:?} is not the contiguous registered name {expected_attempt}"
            ));
        }
        if let Some(previous) = &terminal_zero_attempt {
            hard_invalid.push(format!(
                "attempt {attempt} follows terminal status-zero attempt {previous}"
            ));
        }
        let meta_error = if empty_allocation {
            None
        } else {
            match frozen {
                Ok(context) => context
                    .validate_attempt_meta(
                        &metadata,
                        logical,
                        &attempt,
                        expected_input,
                        expected_draw,
                        expected_mode,
                        expected_max_cols,
                        output_root,
                    )
                    .err(),
                Err(e) => Some(format!("frozen provenance is unavailable: {e}")),
            }
        };
        if let Some(e) = &meta_error {
            hard_invalid.push(format!("attempt {attempt}: {e}"));
        }
        let status_text = fs::read_to_string(&status);
        let rc = status_text
            .as_ref()
            .ok()
            .and_then(|s| s.trim().parse::<i32>().ok());
        if rc == Some(0) && terminal_zero_attempt.is_none() {
            terminal_zero_attempt = Some(attempt.clone());
        }
        let (mut state, mut reason) = match (&status_text, rc) {
            (Err(e), _) => ("not_completed", format!("missing status: {e}")),
            (Ok(_), None) => {
                hard_invalid.push(format!("attempt {attempt}: malformed status"));
                ("invalid", "malformed status".into())
            }
            (Ok(_), Some(rc)) if rc != 0 => ("not_completed", format!("process exit status {rc}")),
            (Ok(_), Some(0)) => match parse_json_lines(&out) {
                Ok(mut rows) if rows.len() == 1 => {
                    successes.push((attempt.clone(), rows.remove(0)));
                    (
                        "candidate_complete",
                        "status zero with exactly one JSON row".into(),
                    )
                }
                Ok(rows) if rows.is_empty() => (
                    "incomplete",
                    "status zero but output has no final JSON row".into(),
                ),
                Ok(rows) => {
                    let why = format!(
                        "status zero but output has {} JSON rows, expected one",
                        rows.len()
                    );
                    hard_invalid.push(format!("attempt {attempt}: {why}"));
                    ("invalid", why)
                }
                Err(e) if out.is_file() => {
                    let why = format!("status zero with invalid output: {e}");
                    hard_invalid.push(format!("attempt {attempt}: {why}"));
                    ("invalid", why)
                }
                Err(e) => (
                    "incomplete",
                    format!("status zero but output is missing: {e}"),
                ),
            },
            (Ok(_), Some(rc)) => ("not_completed", format!("process exit status {rc}")),
        };
        if let Some(e) = meta_error {
            state = "invalid";
            reason = format!("invalid metadata: {e}; {reason}");
        }
        summaries.push(AttemptSummary {
            attempt,
            exit_status: rc,
            state: state.into(),
            reason,
            output: out.display().to_string(),
            status: status.display().to_string(),
            metadata: metadata.display().to_string(),
        });
    }
    if !hard_invalid.is_empty() {
        return CompletedRecord::Invalid(hard_invalid.join("; "), summaries);
    }
    match successes.len() {
        0 => CompletedRecord::Unmeasured("no complete status-zero attempt".into(), summaries),
        1 => {
            let (attempt, value) = successes.remove(0);
            CompletedRecord::Value(value, attempt, summaries)
        }
        n => CompletedRecord::Invalid(
            format!("{n} successful attempts exist; expected exactly one"),
            summaries,
        ),
    }
}

fn common_errors(v: &Value, cell: Cell, draw: &DrawRecord, expected_input: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    if !v.is_object() {
        return vec!["record is not a JSON object".into()];
    }
    for (key, got, want) in [
        ("m", field_u64(v, "m"), cell.m),
        ("n", field_u64(v, "n"), cell.n),
        ("ell", field_u64(v, "ell"), cell.ell),
        ("n_vars", field_u64(v, "n_vars"), cell.n_vars()),
        ("draw", field_u64(v, "draw"), draw.draw),
    ] {
        if got != Some(want) {
            errors.push(format!("{key} is {got:?}, expected {want}"));
        }
    }
    if v.get("cell").and_then(Value::as_str) != Some(cell.source_cell().as_str()) {
        errors.push("cell label differs from the committed draw".into());
    }
    if v.get("committed_outcome") != Some(&draw.outcome) {
        errors.push("committed_outcome differs from the draw file".into());
    }
    match v.get("source").and_then(Value::as_str) {
        Some(source) => {
            let source = Path::new(source);
            if !source.is_absolute() || source.file_name() != expected_input.file_name() {
                errors.push("source does not identify the committed draw input".into());
            }
        }
        None => errors.push("source is missing or is not a string".into()),
    }
    if v.get("checkpoint_segments")
        .is_some_and(|x| x.as_u64().is_none())
    {
        errors.push("checkpoint_segments is not an integer".into());
    }
    if v.get("engine").is_some() {
        errors.push("sparse/alternate engine output is not registered".into());
    }
    errors
}

fn scan_rows(v: &Value) -> Result<Vec<&Value>, String> {
    let scan = v
        .get("scan")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing scan array".to_string())?;
    let mut expected = 3u64;
    for step in scan {
        if expected > 4 {
            return Err("scan extends past the registered degree four".into());
        }
        if field_u64(step, "degree") != Some(expected) {
            return Err(format!(
                "scan degree is {:?}, expected {expected}",
                field_u64(step, "degree")
            ));
        }
        for key in ["xl_refuted", "closure_refuted", "saturated"] {
            if field_bool(step, key).is_none() {
                return Err(format!("degree {expected} has no Boolean {key}"));
            }
        }
        for key in [
            "xl_rank",
            "closure_rank",
            "columns",
            "closure_products",
            "closure_word_ops",
        ] {
            if field_u64(step, key).is_none() {
                return Err(format!("degree {expected} has no integer {key}"));
            }
        }
        if step.get("secs").and_then(Value::as_f64).is_none() {
            return Err(format!(
                "degree {expected} has no numeric diagnostic seconds field"
            ));
        }
        expected += 1;
    }
    let implied_measured_to = scan.last().map_or(2, |s| field_u64(s, "degree").unwrap());
    if field_u64(v, "measured_to") != Some(implied_measured_to) {
        return Err(format!(
            "measured_to is {:?}, but the scan ends at {implied_measured_to}",
            field_u64(v, "measured_to")
        ));
    }
    Ok(scan.iter().collect())
}

fn validate_scan_degrees(v: &Value, scan: &[&Value]) -> Result<(), String> {
    let first_xl = scan
        .iter()
        .find(|s| field_bool(s, "xl_refuted") == Some(true))
        .and_then(|s| field_u64(s, "degree"));
    let first_closure = scan
        .iter()
        .find(|s| field_bool(s, "closure_refuted") == Some(true))
        .and_then(|s| field_u64(s, "degree"));
    let json_degree = |key: &str| match v.get(key) {
        Some(Value::Null) => Ok(None),
        Some(value) => value
            .as_u64()
            .map(Some)
            .ok_or_else(|| format!("{key} is neither null nor an integer")),
        None => Err(format!("missing {key}")),
    };
    if json_degree("xl_degree")? != first_xl {
        return Err("xl_degree disagrees with the scan".into());
    }
    if json_degree("closure_degree")? != first_closure {
        return Err("closure_degree disagrees with the scan".into());
    }
    if scan.iter().any(|s| {
        field_bool(s, "xl_refuted") == Some(true) && field_bool(s, "closure_refuted") != Some(true)
    }) {
        return Err("an XL refutation is absent from the closure row".into());
    }
    Ok(())
}

fn process_row(
    run_id: String,
    kind: &str,
    cell: Cell,
    draw: Option<&DrawRecord>,
    classification: &str,
    reason: String,
    selected_attempt: Option<String>,
    attempts: Vec<AttemptSummary>,
) -> ProcessRow {
    ProcessRow {
        run_id,
        kind: kind.into(),
        cell: cell.id(),
        draw: draw.map(|d| d.draw),
        classification: classification.into(),
        reason,
        selected_attempt,
        attempts,
    }
}

fn classify_unsat(
    output_root: &Path,
    frozen: &Result<FrozenContext, String>,
    draw_dir: &Path,
    cell: Cell,
    slot: usize,
    draw: Option<&DrawRecord>,
) -> ProcessRow {
    let run_id = draw.map_or_else(
        || format!("unsat-{}-slot{slot}", cell.id()),
        |d| format!("unsat-{}-d{}", cell.id(), d.draw),
    );
    let Some(draw) = draw else {
        return process_row(
            run_id,
            "unsat",
            cell,
            None,
            "unmeasured",
            "draw panel did not supply this registered slot".into(),
            None,
            Vec::new(),
        );
    };
    match completed_record(
        output_root,
        &run_id,
        frozen,
        &cell.draw_file(draw_dir),
        draw.draw,
        "unsat",
        400_000,
    ) {
        CompletedRecord::Unmeasured(reason, attempts) => process_row(
            run_id,
            "unsat",
            cell,
            Some(draw),
            "unmeasured",
            reason,
            None,
            attempts,
        ),
        CompletedRecord::Invalid(reason, attempts) => process_row(
            run_id,
            "unsat",
            cell,
            Some(draw),
            "invalid_record",
            reason,
            None,
            attempts,
        ),
        CompletedRecord::Value(v, attempt, attempts) => {
            let input = cell.draw_file(draw_dir);
            let mut errors = common_errors(&v, cell, draw, &input);
            if field_u64(&v, "max_cols") != Some(400_000) {
                errors.push("max_cols is not the registered value 400000".into());
            }
            let specialised = match v.get("fixed") {
                None => false,
                Some(Value::Array(fixed)) => !fixed.is_empty(),
                Some(_) => true,
            };
            if specialised {
                errors.push("UNSAT replay carries nonempty or malformed specialisation".into());
            }
            if v.get("solutions").is_some() || v.get("refuted_satisfiable").is_some() {
                errors.push("UNSAT record carries satisfiable-control fields".into());
            }
            let scan = match scan_rows(&v) {
                Ok(s) => s,
                Err(e) => {
                    errors.push(e);
                    Vec::new()
                }
            };
            if let Err(e) = validate_scan_degrees(&v, &scan) {
                errors.push(e);
            }
            let measured_to = field_u64(&v, "measured_to");
            let closure = v.get("closure_degree");
            let (class, reason) = if !errors.is_empty() {
                ("invalid_record", errors.join("; "))
            } else if let Some(d) = closure.and_then(Value::as_u64) {
                let matching = scan.last().is_some_and(|s| {
                    field_u64(s, "degree") == Some(d)
                        && field_bool(s, "closure_refuted") == Some(true)
                });
                let complete = scan
                    .iter()
                    .all(|s| field_bool(s, "saturated") == Some(true));
                if (3..=4).contains(&d)
                    && matching
                    && measured_to == Some(d)
                    && scan.len() == (d - 2) as usize
                    && complete
                {
                    (
                        "closure_by_four",
                        format!("complete closure refutation at degree {d}"),
                    )
                } else {
                    (
                        "invalid_record",
                        format!("closure_degree={d} is inconsistent with scan/measured_to"),
                    )
                }
            } else if closure == Some(&Value::Null) && measured_to == Some(4) {
                let d4 = scan.iter().find(|s| field_u64(s, "degree") == Some(4));
                let any_refuted = scan
                    .iter()
                    .any(|s| field_bool(s, "closure_refuted") == Some(true));
                if scan.len() == 2
                    && scan
                        .iter()
                        .all(|s| field_bool(s, "saturated") == Some(true))
                    && d4.and_then(|s| field_bool(s, "saturated")) == Some(true)
                    && !any_refuted
                {
                    (
                        "above_four",
                        "complete saturated degree-four scan did not refute".into(),
                    )
                } else {
                    (
                        "unmeasured",
                        "degree-four row is present but incomplete or unsaturated".into(),
                    )
                }
            } else if measured_to.unwrap_or(0) < 4 {
                (
                    "unmeasured",
                    format!(
                        "scan completed only through degree {}",
                        measured_to.unwrap_or(0)
                    ),
                )
            } else {
                (
                    "invalid_record",
                    "missing or out-of-range closure classification".into(),
                )
            };
            let selected = (class != "invalid_record").then_some(attempt);
            process_row(
                run_id,
                "unsat",
                cell,
                Some(draw),
                class,
                reason,
                selected,
                attempts,
            )
        }
    }
}

fn classify_control(
    output_root: &Path,
    frozen: &Result<FrozenContext, String>,
    draw_dir: &Path,
    cell: Cell,
    slot: usize,
    draw: Option<&DrawRecord>,
) -> ProcessRow {
    let run_id = draw.map_or_else(
        || format!("control-{}-slot{slot}", cell.id()),
        |d| format!("control-{}-d{}", cell.id(), d.draw),
    );
    let Some(draw) = draw else {
        return process_row(
            run_id,
            "satisfiable_control",
            cell,
            None,
            "unmeasured",
            "draw panel did not supply this registered control".into(),
            None,
            Vec::new(),
        );
    };
    match completed_record(
        output_root,
        &run_id,
        frozen,
        &cell.draw_file(draw_dir),
        draw.draw,
        "sat",
        400_000,
    ) {
        CompletedRecord::Unmeasured(reason, attempts) => process_row(
            run_id,
            "satisfiable_control",
            cell,
            Some(draw),
            "unmeasured",
            reason,
            None,
            attempts,
        ),
        CompletedRecord::Invalid(reason, attempts) => process_row(
            run_id,
            "satisfiable_control",
            cell,
            Some(draw),
            "invalid_record",
            reason,
            None,
            attempts,
        ),
        CompletedRecord::Value(v, attempt, attempts) => {
            let input = cell.draw_file(draw_dir);
            let mut errors = common_errors(&v, cell, draw, &input);
            if field_u64(&v, "max_cols") != Some(400_000) {
                errors.push("max_cols is not the registered value 400000".into());
            }
            for key in ["fixed", "xl_degree", "closure_degree"] {
                if v.get(key).is_some() {
                    errors.push(format!("control unexpectedly carries {key}"));
                }
            }
            let scan = match scan_rows(&v) {
                Ok(s) => s,
                Err(e) => {
                    errors.push(e);
                    Vec::new()
                }
            };
            if field_u64(&v, "solutions") != Some(draw.solutions) || draw.solutions == 0 {
                errors.push("positive solution count differs from committed draw".into());
            }
            let refuting_scan = scan.iter().any(|s| {
                field_bool(s, "xl_refuted") == Some(true)
                    || field_bool(s, "closure_refuted") == Some(true)
            });
            if refuting_scan || field_bool(&v, "refuted_satisfiable") == Some(true) {
                errors.push("satisfiable control was refuted".into());
            } else if field_bool(&v, "refuted_satisfiable") != Some(false) {
                errors.push("refuted_satisfiable is missing or malformed".into());
            }
            let complete = field_u64(&v, "measured_to") == Some(4)
                && scan.len() == 2
                && scan
                    .iter()
                    .all(|s| field_bool(s, "saturated") == Some(true));
            let (class, reason) = if !errors.is_empty() {
                ("invalid_record", errors.join("; "))
            } else if complete {
                (
                    "passed",
                    "complete saturated degrees three and four did not refute".into(),
                )
            } else {
                (
                    "unmeasured",
                    "control scan did not completely saturate degrees three and four".into(),
                )
            };
            let selected = (class != "invalid_record").then_some(attempt);
            process_row(
                run_id,
                "satisfiable_control",
                cell,
                Some(draw),
                class,
                reason,
                selected,
                attempts,
            )
        }
    }
}

fn normalize_identity(mut v: Value) -> Value {
    if let Some(map) = v.as_object_mut() {
        map.remove("source");
        map.remove("checkpoint_segments");
        if let Some(scan) = map.get_mut("scan").and_then(Value::as_array_mut) {
            for step in scan {
                if let Some(step) = step.as_object_mut() {
                    step.remove("secs");
                }
            }
        }
    }
    v
}

fn classify_identity(
    output_root: &Path,
    frozen: &Result<FrozenContext, String>,
    expected_input: &Path,
    expected_path: &Path,
) -> ProcessRow {
    let run_id = "identity-m4-n12-l5-d24".to_string();
    let cell = Cell::new(4, 12, 5);
    match completed_record(
        output_root,
        &run_id,
        frozen,
        expected_input,
        24,
        "unsat",
        155_000,
    ) {
        CompletedRecord::Unmeasured(reason, attempts) => process_row(
            run_id,
            "identity",
            cell,
            None,
            "unmeasured",
            reason,
            None,
            attempts,
        ),
        CompletedRecord::Invalid(reason, attempts) => process_row(
            run_id,
            "identity",
            cell,
            None,
            "invalid_record",
            reason,
            None,
            attempts,
        ),
        CompletedRecord::Value(got, attempt, attempts) => {
            let expected = parse_json_lines(expected_path);
            let (class, reason) = match expected {
                Ok(v)
                    if v.len() == 1
                        && normalize_identity(v[0].clone()) == normalize_identity(got) =>
                {
                    ("passed", "all historical algebraic fields match".into())
                }
                Ok(v) if v.len() != 1 => (
                    "invalid_record",
                    format!(
                        "historical identity file has {} rows, expected one",
                        v.len()
                    ),
                ),
                Ok(_) => (
                    "invalid_instrument",
                    "historical algebraic identity replay differs".into(),
                ),
                Err(e) if expected_path.is_file() => (
                    "invalid_record",
                    format!("historical identity row is malformed: {e}"),
                ),
                Err(e) => (
                    "unmeasured",
                    format!("cannot read historical identity: {e}"),
                ),
            };
            let selected = (class == "passed").then_some(attempt);
            process_row(
                run_id, "identity", cell, None, class, reason, selected, attempts,
            )
        }
    }
}

fn apply_panel_gate(row: &mut ProcessRow, panel: &PanelSummary) {
    if panel.state == "ready" {
        return;
    }
    row.classification = if panel.state == "invalid" {
        "invalid_record"
    } else {
        "unmeasured"
    }
    .into();
    let detail = if panel.errors.is_empty() {
        format!("draw panel is {}", panel.state)
    } else {
        format!("draw panel is {}: {}", panel.state, panel.errors.join("; "))
    };
    row.reason = detail;
    row.selected_attempt = None;
}

fn cell_classification(
    cell: Cell,
    panel_state: &str,
    identity: &ProcessRow,
    controls: &[ProcessRow],
    unsat: &[ProcessRow],
) -> CellSummary {
    let count = |name: &str| unsat.iter().filter(|r| r.classification == name).count();
    let closure = count("closure_by_four");
    let above = count("above_four");
    let unmeasured = count("unmeasured");
    let invalid = count("invalid_record");
    let controls_passed = controls
        .iter()
        .filter(|r| r.classification == "passed")
        .count();
    let controls_invalid = controls
        .iter()
        .any(|r| r.classification == "invalid_record" || r.classification == "invalid_instrument");
    let controls_unmeasured = controls.iter().any(|r| r.classification == "unmeasured");
    let classification = if panel_state == "invalid"
        || identity.classification == "invalid_record"
        || identity.classification == "invalid_instrument"
        || controls_invalid
        || invalid > 0
    {
        "invalid_instrument"
    } else if panel_state != "ready"
        || identity.classification != "passed"
        || controls_unmeasured
        || unsat.len() != 4
        || controls.len() != 6
        || controls_passed != 6
    {
        "unmeasured"
    } else if closure + above == 4 && closure > 0 && above > 0 {
        "draw_dependent"
    } else if above > 0 {
        "not_universally_four_at_cell"
    } else if unmeasured > 0 {
        "unmeasured"
    } else if closure == 4 {
        "bounded_four_at_cell"
    } else {
        "invalid_instrument"
    };
    CellSummary {
        cell: cell.id(),
        classification: classification.into(),
        closure_by_four: closure,
        above_four: above,
        unmeasured,
        invalid,
        controls_passed,
    }
}

fn historical_expected(draw_dir: &Path) -> PathBuf {
    let repo_relative =
        PathBuf::from("research/dreg_closure_driver_20261016/runs/fresh-m4-12-5.d24.jsonl");
    if repo_relative.is_file() {
        return repo_relative;
    }
    draw_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| Path::new("research"))
        .join("dreg_closure_driver_20261016/runs/fresh-m4-12-5.d24.jsonl")
}

fn historical_input(expected_path: &Path) -> PathBuf {
    expected_path
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| Path::new("research/dreg_closure_driver_20261016"))
        .join("draws/m4-12-5.jsonl")
}

fn row_exit_code(row: &ProcessRow) -> i32 {
    match row.classification.as_str() {
        "passed" | "closure_by_four" | "above_four" => 0,
        "unmeasured" => {
            let has_status_zero = row.attempts.iter().any(|a| a.exit_status == Some(0));
            if has_status_zero || row.attempts.len() >= 4 {
                76
            } else {
                75
            }
        }
        _ => 65,
    }
}

fn main() {
    let args: Vec<_> = env::args_os().collect();
    let required = match args.len() {
        3 => None,
        5 if args[3].to_str() == Some("--require") => Some(args[4].to_string_lossy().to_string()),
        _ => {
            eprintln!(
                "usage: {} DRAW_DIR OUTPUT_ROOT [--require LOGICAL_NAME]",
                args[0].to_string_lossy()
            );
            std::process::exit(2);
        }
    };
    if required.as_deref().is_some_and(str::is_empty) {
        eprintln!("--require needs a nonempty logical name");
        std::process::exit(2);
    }
    let draw_dir = PathBuf::from(&args[1]);
    let output_root = PathBuf::from(&args[2]);
    let expected = historical_expected(&draw_dir);
    let expected_input = historical_input(&expected);
    let frozen = FrozenContext::load(&draw_dir);
    let provenance = match &frozen {
        Ok(context) => ProvenanceSummary {
            state: "ready".into(),
            protocol: context.protocol_path.display().to_string(),
            binary_manifest: context.binary_manifest_path.display().to_string(),
            reason: "frozen protocol and binary manifest agree".into(),
        },
        Err(reason) => ProvenanceSummary {
            state: "unavailable".into(),
            protocol: study_file(&draw_dir, "protocol.json").display().to_string(),
            binary_manifest: study_file(&draw_dir, "binary.sha256").display().to_string(),
            reason: reason.clone(),
        },
    };

    let panels: Vec<_> = CELLS.iter().map(|&c| draw_panel(c, &draw_dir)).collect();
    let identity = classify_identity(&output_root, &frozen, &expected_input, &expected);
    let mut control_sets = Vec::new();
    let mut unsat_sets = Vec::new();
    for (&cell, panel) in CELLS.iter().zip(&panels) {
        let mut controls: Vec<_> = (0..2)
            .map(|slot| {
                classify_control(
                    &output_root,
                    &frozen,
                    &draw_dir,
                    cell,
                    slot,
                    panel.sat.get(slot),
                )
            })
            .collect();
        let mut unsat: Vec<_> = (0..4)
            .map(|slot| {
                classify_unsat(
                    &output_root,
                    &frozen,
                    &draw_dir,
                    cell,
                    slot,
                    panel.unsat.get(slot),
                )
            })
            .collect();
        for row in controls.iter_mut().chain(unsat.iter_mut()) {
            apply_panel_gate(row, &panel.summary);
        }
        control_sets.push(controls);
        unsat_sets.push(unsat);
    }
    let all_controls: Vec<_> = control_sets.iter().flatten().cloned().collect();
    let cells: Vec<_> = CELLS
        .iter()
        .zip(&panels)
        .zip(&unsat_sets)
        .map(|((&cell, panel), unsat)| {
            cell_classification(cell, &panel.summary.state, &identity, &all_controls, unsat)
        })
        .collect();

    let mut rows = vec![identity.clone()];
    for (controls, unsat) in control_sets.iter().zip(&unsat_sets) {
        rows.extend(controls.iter().cloned());
        rows.extend(unsat.iter().cloned());
    }

    assert_eq!(
        rows.len(),
        19,
        "the protocol has exactly nineteen subprocess rows"
    );
    let controls_invalid = all_controls.iter().any(|r| row_exit_code(r) == 65);
    let controls_complete =
        all_controls.len() == 6 && all_controls.iter().all(|r| r.classification == "passed");
    let sat_package_class = if controls_invalid {
        "invalid"
    } else if controls_complete {
        "passed"
    } else {
        "unmeasured"
    };
    let sat_package_reason = match sat_package_class {
        "passed" => "all six complete saturated controls remained unrefuted".to_string(),
        "invalid" => "at least one satisfiable control is invalid or refuted".to_string(),
        _ => "at least one satisfiable control is missing or incomplete".to_string(),
    };
    let mut packages = vec![
        PackageSummary {
            package_id: "identity-replay".into(),
            role: "reference".into(),
            classification: identity.classification.clone(),
            reason: identity.reason.clone(),
            subprocesses: vec![identity.run_id.clone()],
        },
        PackageSummary {
            package_id: "satisfiable-controls".into(),
            role: "control_positive".into(),
            classification: sat_package_class.into(),
            reason: sat_package_reason,
            subprocesses: all_controls.iter().map(|r| r.run_id.clone()).collect(),
        },
    ];
    for (cell, unsat) in cells.iter().zip(&unsat_sets) {
        packages.push(PackageSummary {
            package_id: cell.cell.clone(),
            role: "candidate".into(),
            classification: cell.classification.clone(),
            reason: format!(
                "{} closure-by-four, {} above-four, {} unmeasured, {} invalid; {}/6 shared controls passed",
                cell.closure_by_four,
                cell.above_four,
                cell.unmeasured,
                cell.invalid,
                cell.controls_passed
            ),
            subprocesses: unsat.iter().map(|r| r.run_id.clone()).collect(),
        });
    }
    assert_eq!(
        packages.len(),
        5,
        "the protocol has five composite packages"
    );

    let global = if cells
        .iter()
        .any(|c| c.classification == "invalid_instrument")
        || identity.classification == "invalid_record"
        || identity.classification == "invalid_instrument"
        || sat_package_class == "invalid"
    {
        "invalid_instrument"
    } else if identity.classification != "passed" || sat_package_class != "passed" {
        "unmeasured"
    } else if cells.iter().any(|c| {
        c.classification == "not_universally_four_at_cell" || c.classification == "draw_dependent"
    }) {
        "not_universally_four_in_registered_panel"
    } else if cells.iter().any(|c| c.classification == "unmeasured") {
        "unmeasured"
    } else if cells
        .iter()
        .all(|c| c.classification == "bounded_four_at_cell")
    {
        "bounded_four_across_registered_cells"
    } else {
        "invalid_instrument"
    };
    let summary = json!({
        "schema": "dreg_high_arity.summary/v1",
        "draw_dir": draw_dir,
        "output_root": output_root,
        "historical_identity": expected,
        "frozen_provenance": provenance,
        "global_classification": global,
        "draw_panels": panels.iter().map(|p| &p.summary).collect::<Vec<_>>(),
        "rows": &rows,
        "cells": &cells,
        "packages": &packages,
        "timing_interpreted": false
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&summary).expect("summary serializes")
    );

    let exit_code = if let Some(logical) = required {
        match rows.iter().find(|row| row.run_id == logical) {
            Some(row) => row_exit_code(row),
            None => {
                eprintln!("unknown or unavailable registered logical name: {logical}");
                65
            }
        }
    } else {
        let codes: Vec<_> = rows.iter().map(row_exit_code).collect();
        if codes.contains(&65) {
            65
        } else if codes.contains(&75) {
            75
        } else if codes.contains(&76) {
            76
        } else {
            0
        }
    };
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_normalization_removes_only_non_algebraic_fields() {
        let a = json!({"source":"a", "checkpoint_segments":1, "scan":[{"degree":4,"secs":1.0,"rank":9}]});
        let b = json!({"source":"b", "checkpoint_segments":7, "scan":[{"degree":4,"secs":99.0,"rank":9}]});
        assert_eq!(normalize_identity(a), normalize_identity(b));
    }

    fn scan_step(degree: u64, saturated: bool, xl: bool, closure: bool) -> Value {
        json!({
            "degree": degree,
            "xl_refuted": xl,
            "closure_refuted": closure,
            "saturated": saturated,
            "xl_rank": 1,
            "closure_rank": 2,
            "columns": 3,
            "closure_products": 4,
            "closure_word_ops": 5,
            "secs": 0.25
        })
    }

    #[test]
    fn scan_degree_validation_is_contiguous() {
        let valid = json!({
            "measured_to": 4,
            "scan": [scan_step(3, true, false, false), scan_step(4, true, false, false)]
        });
        assert!(scan_rows(&valid).is_ok());
        let skipped = json!({"measured_to":4,"scan":[scan_step(4, true, false, false)]});
        assert!(scan_rows(&skipped).is_err());
        let inconsistent = json!({"measured_to":4,"scan":[scan_step(3, true, false, false)]});
        assert!(scan_rows(&inconsistent).is_err());
    }

    #[test]
    fn draw_outcomes_are_restricted_to_the_registered_generator() {
        assert!(validate_draw_outcome(&json!({"outcome":{"kind":"satisfiable"}}), 1).is_ok());
        assert!(
            validate_draw_outcome(&json!({"outcome":{"kind":"at_least","degree":4}}), 0).is_ok()
        );
        assert!(
            validate_draw_outcome(&json!({"outcome":{"kind":"caps_hit","built":2}}), 0).is_ok()
        );
        assert!(
            validate_draw_outcome(&json!({"outcome":{"kind":"caps_hit","built":4}}), 0).is_err()
        );
        assert!(validate_draw_outcome(&json!({"outcome":{"kind":"mystery"}}), 0).is_err());
        assert!(validate_draw_outcome(&json!({"outcome":{"kind":"satisfiable"}}), 0).is_err());
    }

    fn test_row(classification: &str) -> ProcessRow {
        ProcessRow {
            run_id: classification.into(),
            kind: "test".into(),
            cell: "m5-n11-l2".into(),
            draw: Some(0),
            classification: classification.into(),
            reason: "test".into(),
            selected_attempt: None,
            attempts: Vec::new(),
        }
    }

    #[test]
    fn completed_above_four_is_not_hidden_by_unmeasured_draws() {
        let identity = test_row("passed");
        let controls = (0..6).map(|_| test_row("passed")).collect::<Vec<_>>();
        let unsat = vec![
            test_row("above_four"),
            test_row("unmeasured"),
            test_row("unmeasured"),
            test_row("unmeasured"),
        ];
        let cell = cell_classification(Cell::new(5, 11, 2), "ready", &identity, &controls, &unsat);
        assert_eq!(cell.classification, "not_universally_four_at_cell");
    }

    #[test]
    fn cell_formula_matches_registered_sizes() {
        assert_eq!(Cell::new(5, 11, 2).n_vars(), 43);
        assert_eq!(Cell::new(5, 13, 3).n_vars(), 54);
        assert_eq!(Cell::new(6, 11, 2).n_vars(), 56);
    }
}
