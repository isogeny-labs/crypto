//! Timed baseline/candidate A/B driver for the frozen 84-input panel.
//!
//! The driver executes two prebuilt `hashed-ecdl` arm binaries over a prepared
//! panel. It never solves anything itself. Each execution is one child process
//! (`ARM solve PROBLEM NEW_RECEIPT`), timed from spawn to exit, followed by a
//! separately timed `ARM verify RECEIPT` replay. The driver then validates the
//! receipt against the manifest without trusting the arm: strict JSON, echoed
//! input, full cardinality and the returned scalar through the tiny exhaustive
//! oracle (no library curve code), the manifest's expected scalar, and finally
//! the driver's own library certificate replay. Every failure, rejection,
//! timeout and deadline-skipped execution is retained as a row.
//!
//! Choices where the frozen protocol is silent are documented in
//! docs/hashed-ecdl-panel.md ("Timed A/B driver") and repeated in the
//! `choices` object of every analysis.

use super::TinyCurve;
use hashed_ecdl_tensor::curve::{Curve, Point as NativePoint};
use hashed_ecdl_tensor::reduction::{self, Certificate};
use hashed_ecdl_tensor::{BigUint, Limits, Meter};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const ROW_SCHEMA: &str = "hashed-ecdl-panel.run-row/v1";
pub const ANALYSIS_SCHEMA: &str = "hashed-ecdl-panel.analysis/v1";
const RECEIPT_SCHEMA: &str = "hashed-ecdl-tensor/v1";
const PANEL_SCHEMA: &str = "hashed-ecdl-panel/v1";
const RECEIPT_LIMIT: u64 = 16 * 1024 * 1024;
/// Frozen protocol minimums (PROTOCOL.md "Measurement and execution").
pub const MIN_AA_ROUNDS: usize = 5;
pub const MIN_AB_ROUNDS: usize = 5;
pub const PANEL_CASES: usize = 84;
pub const PROTOCOL_REPETITIONS: usize = 3;
pub const PROTOCOL_DEADLINE_SECS: u64 = 1800;

// ---------------------------------------------------------------- records

pub struct ManifestCase {
    pub id: String,
    pub split: String,
    pub problem: String,
    pub expected_scalar: String,
    pub full_cardinality: u64,
}
json_record!(
    ManifestCase,
    required {
        id,
        split,
        problem,
        expected_scalar,
        full_cardinality
    },
    optional {}
);

pub struct Manifest {
    pub schema: String,
    pub status: String,
    pub protocol: String,
    pub baseline: String,
    pub paired_baseline: String,
    pub development_inputs: u64,
    pub holdout_inputs: u64,
    pub repetitions: u64,
    pub required_aa_rounds: u64,
    pub required_ab_rounds: u64,
    pub execution: String,
    pub measurements: Value,
    pub cases: Vec<ManifestCase>,
}
json_record!(
    Manifest,
    required {
        schema,
        status,
        protocol,
        baseline,
        paired_baseline,
        development_inputs,
        holdout_inputs,
        repetitions,
        required_aa_rounds,
        required_ab_rounds,
        execution,
        measurements,
        cases
    },
    optional {}
);

/// The union of the baseline and candidate runners' problem fields. The
/// baseline omits `prime_certificates`; the candidate echoes it as null.
#[derive(PartialEq, Debug)]
pub struct PanelProblem {
    pub p: String,
    pub a: String,
    pub b: String,
    pub r: String,
    pub cofactor: String,
    pub generator: [String; 2],
    pub target: Option<[String; 2]>,
    pub slots: Option<usize>,
    pub threshold: Option<String>,
    pub hash_prime: Option<String>,
    pub trials: Option<usize>,
    pub columns: Option<usize>,
    pub steps: Option<u64>,
    pub vector: Option<usize>,
    pub bits: Option<usize>,
    pub fixture_seed: Option<u64>,
    pub prime_certificates: Option<Value>,
}
json_record!(
    PanelProblem,
    required {
        p,
        a,
        b,
        r,
        cofactor,
        generator,
        target
    },
    optional {
        slots,
        threshold,
        hash_prime,
        trials,
        columns,
        steps,
        vector,
        bits,
        fixture_seed,
        prime_certificates
    }
);

pub struct Receipt {
    pub schema: String,
    pub problem: PanelProblem,
    pub certificate: Certificate,
    pub diagnostics: Value,
}
json_record!(
    Receipt,
    required {
        schema,
        problem,
        certificate,
        diagnostics
    },
    optional {}
);

// ---------------------------------------------------------------- SHA-256

/// FIPS 180-4 SHA-256, kept in-tree so the crate's dependency set is unchanged.
pub fn sha256(data: &[u8]) -> [u8; 32] {
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
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut message = data.to_vec();
    let bit_length = (data.len() as u64).wrapping_mul(8);
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_length.to_be_bytes());
    for block in message.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (i, word) in block.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let mut v = h;
        for i in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7]
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v = [
                t1.wrapping_add(t2),
                v[0],
                v[1],
                v[2],
                v[3].wrapping_add(t1),
                v[4],
                v[5],
                v[6],
            ];
        }
        for (state, value) in h.iter_mut().zip(v) {
            *state = state.wrapping_add(value);
        }
    }
    let mut out = [0u8; 32];
    for (i, word) in h.iter().enumerate() {
        out[4 * i..4 * i + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

pub fn sha256_hex(data: &[u8]) -> String {
    sha256(data).iter().map(|b| format!("{b:02x}")).collect()
}

fn file_sha256(path: &Path) -> Result<String> {
    Ok(sha256_hex(&fs::read(path)?))
}

/// Parse `sha256sum` output: `<64 hex>  <relative path>` per line.
pub fn parse_sha256_list(text: &str) -> Result<BTreeMap<String, String>> {
    let mut map = BTreeMap::new();
    for (number, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let (hash, path) = line
            .split_once("  ")
            .ok_or_else(|| format!("hash list line {}: expected `HASH  PATH`", number + 1))?;
        let path = path.strip_prefix("./").unwrap_or(path);
        if hash.len() != 64 || !hash.bytes().all(|c| matches!(c, b'0'..=b'9' | b'a'..=b'f')) {
            return Err(format!("hash list line {}: malformed SHA-256", number + 1).into());
        }
        if map.insert(path.to_string(), hash.to_string()).is_some() {
            return Err(format!("hash list: duplicate path {path}").into());
        }
    }
    Ok(map)
}

/// `sha256sum`-format list of a prepared panel: manifest.json and every
/// problems/*.json, sorted bytewise by path. Other files are not inputs.
pub fn hash_list(panel: &Path) -> Result<String> {
    let mut paths = vec![String::from("manifest.json")];
    for entry in fs::read_dir(panel.join("problems"))? {
        let name = entry?
            .file_name()
            .into_string()
            .map_err(|_| "non-UTF-8 name")?;
        paths.push(format!("problems/{name}"));
    }
    paths.sort();
    let mut out = String::new();
    for path in paths {
        out.push_str(&format!("{}  {path}\n", file_sha256(&panel.join(&path))?));
    }
    Ok(out)
}

// ---------------------------------------------------------------- schedule

/// SplitMix64: a tiny, fully specified generator so a schedule is replayable
/// from its seed alone, independent of any library RNG.
pub struct SplitMix64(pub u64);
impl SplitMix64 {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        // Rejection sampling avoids modulo bias.
        let n = n as u64;
        let zone = u64::MAX - u64::MAX % n;
        loop {
            let x = self.next();
            if x < zone {
                return (x % n) as usize;
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RoundKind {
    Warmup,
    AA,
    AB,
}
impl RoundKind {
    pub fn label(self) -> &'static str {
        match self {
            RoundKind::Warmup => "warmup",
            RoundKind::AA => "aa",
            RoundKind::AB => "ab",
        }
    }
}

/// `pair_member` 0 is the reference copy (baseline in A/B, first baseline copy
/// in A/A); member 1 is the compared copy (candidate in A/B). Execution order
/// inside a pair is `slot`, which the Latin alternation decouples from member.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Execution {
    pub kind: RoundKind,
    pub round: usize,
    pub position: usize,
    pub slot: usize,
    pub pair_member: usize,
    pub case: usize,
}
impl Execution {
    pub fn arm(&self) -> &'static str {
        if self.pair_member == 1 && self.kind != RoundKind::AA {
            "candidate"
        } else {
            "baseline"
        }
    }
}

/// Seeded interleaved Latin schedule.
///
/// * Round kinds alternate A/A and A/B; the seed picks which comes first.
/// * A seeded base permutation of the cases is cyclically rotated by a
///   different offset each round (rows of a cyclic Latin square), so each case
///   occupies a different position in every round while rounds <= cases.
/// * Within a pair, which member runs first alternates with position and round
///   (a 2x2 Latin square), so member order is balanced within every round and
///   for every case across consecutive rounds.
/// * One warmup pair (both arms, on the base permutation's first case) runs
///   before round 0 and is excluded from analysis but retained.
pub fn schedule(cases: usize, aa_rounds: usize, ab_rounds: usize, seed: u64) -> Vec<Execution> {
    let mut rng = SplitMix64(seed);
    let mut base: Vec<usize> = (0..cases).collect();
    for i in (1..cases).rev() {
        let j = rng.below(i + 1);
        base.swap(i, j);
    }
    let aa_first = rng.next() & 1 == 0;
    let member_phase = (rng.next() & 1) as usize;
    let mut kinds = Vec::new();
    for i in 0..aa_rounds.max(ab_rounds) {
        let (first, second) = if aa_first {
            (RoundKind::AA, RoundKind::AB)
        } else {
            (RoundKind::AB, RoundKind::AA)
        };
        for kind in [first, second] {
            let wanted = if kind == RoundKind::AA {
                aa_rounds
            } else {
                ab_rounds
            };
            if i < wanted {
                kinds.push(kind);
            }
        }
    }
    let mut out = Vec::new();
    if cases == 0 {
        return out;
    }
    for slot in 0..2 {
        out.push(Execution {
            kind: RoundKind::Warmup,
            round: 0,
            position: 0,
            slot,
            pair_member: slot,
            case: base[0],
        });
    }
    let total = kinds.len().max(1);
    let stride = (cases / total).max(1);
    for (round, kind) in kinds.into_iter().enumerate() {
        let offset = (round * stride) % cases;
        for position in 0..cases {
            let case = base[(position + offset) % cases];
            let first_member = (position + round + member_phase) % 2;
            for slot in 0..2 {
                out.push(Execution {
                    kind,
                    round,
                    position,
                    slot,
                    pair_member: (first_member + slot) % 2,
                    case,
                });
            }
        }
    }
    out
}

// ---------------------------------------------------------------- validation

fn parse_point(pair: &Option<[String; 2]>) -> Result<Option<(u64, u64)>> {
    pair.as_ref()
        .map(|[x, y]| Ok((parse_u64(x)?, parse_u64(y)?)))
        .transpose()
}

fn parse_u64(s: &str) -> Result<u64> {
    if s.is_empty() || !s.bytes().all(|c| c.is_ascii_digit()) || (s.len() > 1 && s.starts_with('0'))
    {
        return Err(format!("not a canonical unsigned decimal: {s:?}").into());
    }
    Ok(s.parse()?)
}

fn decimal(s: &str) -> Result<BigUint> {
    parse_u64(s).map(BigUint::from)
}

/// Every check is recorded; `validated` requires all of them.
#[derive(Default, Debug)]
pub struct Checks {
    pub receipt_present: bool,
    pub strict_json: bool,
    pub schema: bool,
    pub echoed_problem: bool,
    pub oracle_full_cardinality: bool,
    pub scalar_in_range: bool,
    pub oracle_scalar_maps_to_target: bool,
    pub manifest_expected_scalar: bool,
    pub diagnostic_scalar_consistent: bool,
    pub library_certificate_replay: bool,
}
impl Checks {
    pub fn all(&self) -> bool {
        self.receipt_present
            && self.strict_json
            && self.schema
            && self.echoed_problem
            && self.oracle_full_cardinality
            && self.scalar_in_range
            && self.oracle_scalar_maps_to_target
            && self.manifest_expected_scalar
            && self.diagnostic_scalar_consistent
            && self.library_certificate_replay
    }
    fn json(&self) -> Value {
        json!({
            "receipt_present": self.receipt_present, "strict_json": self.strict_json,
            "schema": self.schema, "echoed_problem": self.echoed_problem,
            "oracle_full_cardinality": self.oracle_full_cardinality,
            "scalar_in_range": self.scalar_in_range,
            "oracle_scalar_maps_to_target": self.oracle_scalar_maps_to_target,
            "manifest_expected_scalar": self.manifest_expected_scalar,
            "diagnostic_scalar_consistent": self.diagnostic_scalar_consistent,
            "library_certificate_replay": self.library_certificate_replay,
        })
    }
}

pub struct Validation {
    pub checks: Checks,
    pub reason: Option<String>,
    pub scalar: Option<String>,
    pub certificate_sha256: Option<String>,
    pub diagnostics: Value,
}

/// Validate one receipt against its input problem and manifest entry. The
/// order mirrors trust: structure, echo, the independent oracle, the
/// manifest's expected answer, then the library replay last.
pub fn validate_receipt(
    receipt_bytes: Option<&[u8]>,
    problem: &PanelProblem,
    case: &ManifestCase,
) -> Validation {
    let mut checks = Checks::default();
    let mut result = Validation {
        checks: Checks::default(),
        reason: None,
        scalar: None,
        certificate_sha256: None,
        diagnostics: Value::Null,
    };
    let fail = |mut result: Validation, checks: Checks, reason: String| {
        result.checks = checks;
        result.reason = Some(reason);
        result
    };
    let Some(bytes) = receipt_bytes else {
        return fail(result, checks, "missing receipt".into());
    };
    checks.receipt_present = true;
    let receipt: Receipt = match serde_json::from_slice(bytes) {
        Ok(receipt) => receipt,
        Err(error) => return fail(result, checks, format!("receipt JSON: {error}")),
    };
    checks.strict_json = true;
    result.diagnostics = receipt.diagnostics.clone();
    result.certificate_sha256 = serde_json::to_vec(&receipt.certificate)
        .ok()
        .map(|b| sha256_hex(&b));
    if receipt.schema != RECEIPT_SCHEMA {
        return fail(
            result,
            checks,
            format!("receipt schema {:?}", receipt.schema),
        );
    }
    checks.schema = true;
    if &receipt.problem != problem {
        return fail(result, checks, "receipt problem differs from input".into());
    }
    checks.echoed_problem = true;
    let scalar = receipt.certificate.scalar.clone();
    result.scalar = Some(scalar.to_string());
    let tiny = (|| -> Result<(TinyCurve, Vec<Option<(u64, u64)>>, Option<(u64, u64)>)> {
        if parse_u64(&problem.cofactor)? != 1 {
            return Err("panel requires cofactor one".into());
        }
        let generator = parse_point(&Some(problem.generator.clone()))?;
        let curve = TinyCurve {
            p: parse_u64(&problem.p)?,
            a: parse_u64(&problem.a)?,
            b: parse_u64(&problem.b)?,
            r: parse_u64(&problem.r)?,
            generator,
        };
        let orbit = curve.validate()?;
        Ok((curve, orbit, parse_point(&problem.target)?))
    })();
    let (curve, orbit, target) = match tiny {
        Ok(v) => v,
        Err(error) => return fail(result, checks, format!("tiny oracle: {error}")),
    };
    if curve.r != case.full_cardinality {
        return fail(result, checks, "manifest full cardinality differs".into());
    }
    checks.oracle_full_cardinality = true;
    let small = match u64::try_from(&scalar) {
        Ok(s) if s < curve.r => s,
        _ => return fail(result, checks, "scalar outside [0, r)".into()),
    };
    checks.scalar_in_range = true;
    if orbit[small as usize] != target {
        return fail(
            result,
            checks,
            "oracle: scalar*G differs from target".into(),
        );
    }
    checks.oracle_scalar_maps_to_target = true;
    if case.expected_scalar != small.to_string() {
        return fail(result, checks, "scalar differs from manifest".into());
    }
    checks.manifest_expected_scalar = true;
    match receipt.diagnostics.get("scalar_decimal") {
        None => {}
        Some(Value::String(s)) if *s == small.to_string() => {}
        Some(_) => {
            return fail(
                result,
                checks,
                "diagnostic scalar differs from certificate".into(),
            )
        }
    }
    checks.diagnostic_scalar_consistent = true;
    let replay = (|| -> Result<()> {
        let defaults = Limits::default();
        let mut meter = Meter::new(Limits {
            steps: problem.steps.unwrap_or(defaults.steps),
            vector: problem.vector.unwrap_or(defaults.vector),
            bits: problem.bits.unwrap_or(defaults.bits),
        });
        let native = |p: Option<(u64, u64)>| match p {
            None => NativePoint::Infinity,
            Some((x, y)) => NativePoint::Affine {
                x: x.into(),
                y: y.into(),
            },
        };
        let library = Curve::with_cofactor(
            decimal(&problem.p)?,
            decimal(&problem.a)?,
            decimal(&problem.b)?,
            decimal(&problem.r)?,
            decimal(&problem.cofactor)?,
            native(curve.generator),
            &mut meter,
        )?;
        reduction::verify(&library, &native(target), &receipt.certificate, &mut meter)?;
        Ok(())
    })();
    if let Err(error) = replay {
        return fail(
            result,
            checks,
            format!("library certificate replay: {error}"),
        );
    }
    checks.library_certificate_replay = true;
    result.checks = checks;
    result
}

// ---------------------------------------------------------------- execution

#[derive(Debug)]
pub struct Outcome {
    pub status: &'static str,
    pub exit_code: Option<i32>,
    pub wall_ns: Option<u128>,
    pub detail: Option<String>,
}

/// Run one child with a wall limit. Stdout/stderr go to files, never pipes,
/// so a chatty child cannot block. Timing spans spawn to reaped exit.
pub fn run_child(
    program: &Path,
    args: &[&Path],
    stdout: &Path,
    stderr: &Path,
    limit: Duration,
) -> Outcome {
    let open = |p: &Path| OpenOptions::new().write(true).create_new(true).open(p);
    let (out, err) = match (open(stdout), open(stderr)) {
        (Ok(o), Ok(e)) => (o, e),
        (Err(error), _) | (_, Err(error)) => {
            return Outcome {
                status: "spawn_error",
                exit_code: None,
                wall_ns: None,
                detail: Some(format!("log file: {error}")),
            }
        }
    };
    let start = Instant::now();
    let mut child = match Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(err)
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            return Outcome {
                status: "spawn_error",
                exit_code: None,
                wall_ns: None,
                detail: Some(error.to_string()),
            }
        }
    };
    let mut pause = Duration::from_micros(50);
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let wall = start.elapsed().as_nanos();
                return Outcome {
                    status: if status.success() {
                        "exited_ok"
                    } else {
                        "failed"
                    },
                    exit_code: status.code(),
                    wall_ns: Some(wall),
                    detail: if status.success() {
                        None
                    } else {
                        Some(format!("exit status {status}"))
                    },
                };
            }
            Ok(None) if start.elapsed() >= limit => {
                let _ = child.kill();
                let _ = child.wait();
                return Outcome {
                    status: "timeout",
                    exit_code: None,
                    wall_ns: Some(start.elapsed().as_nanos()),
                    detail: Some(format!("killed after {} ms limit", limit.as_millis())),
                };
            }
            Ok(None) => {
                std::thread::sleep(pause);
                pause = (pause * 2).min(Duration::from_millis(5));
            }
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Outcome {
                    status: "spawn_error",
                    exit_code: None,
                    wall_ns: None,
                    detail: Some(format!("wait: {error}")),
                };
            }
        }
    }
}

fn read_bounded(path: &Path) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    let file = File::open(path).ok()?;
    file.take(RECEIPT_LIMIT + 1).read_to_end(&mut bytes).ok()?;
    (bytes.len() as u64 <= RECEIPT_LIMIT).then_some(bytes)
}

pub struct RunOptions {
    pub panel: PathBuf,
    pub inputs_sha256: PathBuf,
    pub baseline: PathBuf,
    pub candidate: PathBuf,
    pub out: PathBuf,
    pub seed: u64,
    pub aa_rounds: usize,
    pub ab_rounds: usize,
    pub deadline: Duration,
    pub case_limit: Duration,
    pub cases: Option<Vec<String>>,
    pub allow_incomplete_panel: bool,
    pub label: String,
}

pub struct LoadedPanel {
    pub manifest: Manifest,
    pub manifest_sha256: String,
    pub problems: Vec<(PanelProblem, String, PathBuf)>,
}

/// Load the manifest and problems, refusing any byte that is not pinned in
/// the frozen hash list.
pub fn load_panel(panel: &Path, hashes: &Path) -> Result<LoadedPanel> {
    let pinned = parse_sha256_list(&fs::read_to_string(hashes)?)?;
    let manifest_bytes = fs::read(panel.join("manifest.json"))?;
    let manifest_sha256 = sha256_hex(&manifest_bytes);
    if pinned.get("manifest.json") != Some(&manifest_sha256) {
        return Err("manifest.json is not pinned by the input hash list".into());
    }
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes)?;
    if manifest.schema != PANEL_SCHEMA || manifest.status != "prepared" {
        return Err("manifest schema/status is not a prepared hashed-ecdl-panel/v1".into());
    }
    let mut ids = BTreeSet::new();
    let mut problems = Vec::new();
    for case in &manifest.cases {
        if !ids.insert(case.id.clone()) {
            return Err(format!("duplicate case id {}", case.id).into());
        }
        let expected_path = format!("problems/{}.json", case.id);
        if case.problem != expected_path
            || !case
                .id
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'-')
        {
            return Err(format!("case {} has an unexpected problem path", case.id).into());
        }
        if !matches!(case.split.as_str(), "development" | "holdout") {
            return Err(format!("case {} has unknown split", case.id).into());
        }
        let path = panel.join(&case.problem);
        let bytes = fs::read(&path)?;
        let hash = sha256_hex(&bytes);
        if pinned.get(&case.problem) != Some(&hash) {
            return Err(format!("{} is not pinned by the input hash list", case.problem).into());
        }
        let problem: PanelProblem = serde_json::from_slice(&bytes)?;
        problems.push((problem, hash, path));
    }
    Ok(LoadedPanel {
        manifest,
        manifest_sha256,
        problems,
    })
}

fn absolute(path: &Path) -> Result<PathBuf> {
    Ok(fs::canonicalize(path)?)
}

/// Execute the schedule; rows stream to `rows.jsonl` as they finish, so a
/// killed driver still leaves every completed row.
pub fn run(options: &RunOptions) -> Result<Value> {
    let panel = load_panel(&options.panel, &options.inputs_sha256)?;
    let selected: Vec<usize> = match &options.cases {
        None => (0..panel.manifest.cases.len()).collect(),
        Some(ids) => ids
            .iter()
            .map(|id| {
                panel
                    .manifest
                    .cases
                    .iter()
                    .position(|c| &c.id == id)
                    .ok_or_else(|| format!("unknown case {id}").into())
            })
            .collect::<Result<_>>()?,
    };
    let protocol_complete = options.cases.is_none()
        && panel.manifest.cases.len() == PANEL_CASES
        && options.aa_rounds >= MIN_AA_ROUNDS
        && options.ab_rounds >= MIN_AB_ROUNDS.max(PROTOCOL_REPETITIONS)
        && options.deadline.as_secs() <= PROTOCOL_DEADLINE_SECS;
    if !protocol_complete && !options.allow_incomplete_panel {
        return Err("schedule is below the frozen protocol (84 cases, >=5 A/A, >=5 A/B, <=1800 s deadline); pass --allow-incomplete-panel for a labelled diagnostic".into());
    }
    let baseline = absolute(&options.baseline)?;
    let candidate = absolute(&options.candidate)?;
    let baseline_sha = file_sha256(&baseline)?;
    let candidate_sha = file_sha256(&candidate)?;
    fs::create_dir(&options.out)?;
    for sub in ["receipts", "logs"] {
        fs::create_dir(options.out.join(sub))?;
    }
    let plan = schedule(
        selected.len(),
        options.aa_rounds,
        options.ab_rounds,
        options.seed,
    );
    let mut rows_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(options.out.join("rows.jsonl"))?;
    let started = Instant::now();
    let mut rows = Vec::new();
    for (index, execution) in plan.iter().enumerate() {
        let case_index = selected[execution.case];
        let case = &panel.manifest.cases[case_index];
        let (problem, problem_sha, problem_path) = &panel.problems[case_index];
        let (binary, binary_sha) = if execution.arm() == "candidate" {
            (&candidate, &candidate_sha)
        } else {
            (&baseline, &baseline_sha)
        };
        let stem = format!(
            "{index:05}-{}-r{}-p{:02}-s{}-{}",
            execution.kind.label(),
            execution.round,
            execution.position,
            execution.slot,
            case.id
        );
        let receipt_path = options.out.join("receipts").join(format!("{stem}.json"));
        let remaining = options.deadline.saturating_sub(started.elapsed());
        let mut row = json!({
            "schema": ROW_SCHEMA, "label": options.label, "row": index,
            "kind": execution.kind.label(), "round": execution.round,
            "position": execution.position, "slot": execution.slot,
            "pair_member": execution.pair_member, "arm": execution.arm(),
            "binary_sha256": binary_sha, "case_id": case.id, "split": case.split,
            "problem_sha256": problem_sha, "command": "solve",
            "status": Value::Null, "reason": Value::Null, "exit_code": Value::Null,
            "solve_wall_ns": Value::Null, "limit_ms": Value::Null,
            "replay_status": Value::Null, "replay_wall_ns": Value::Null,
            "checks": Checks::default().json(), "scalar": Value::Null,
            "certificate_sha256": Value::Null, "meter_diagnostics": Value::Null,
            "phase_ns": phase_ns(None),
            "unpriced_common_operations": Value::Null, "s_over_rho": Value::Null,
        });
        if remaining.is_zero() {
            row["status"] = json!("not_run_deadline");
            row["reason"] = json!("global deadline elapsed before this execution");
        } else {
            let limit = options.case_limit.min(remaining);
            row["limit_ms"] = json!(limit.as_millis() as u64);
            let logs = options.out.join("logs");
            let outcome = run_child(
                binary,
                &[Path::new("solve"), problem_path, &receipt_path],
                &logs.join(format!("{stem}.solve.out")),
                &logs.join(format!("{stem}.solve.err")),
                limit,
            );
            row["exit_code"] = json!(outcome.exit_code);
            row["solve_wall_ns"] = json!(outcome.wall_ns.map(|n| n as u64));
            let bytes = read_bounded(&receipt_path);
            let validation = validate_receipt(bytes.as_deref(), problem, case);
            row["checks"] = validation.checks.json();
            row["scalar"] = json!(validation.scalar);
            row["certificate_sha256"] = json!(validation.certificate_sha256);
            row["meter_diagnostics"] = validation.diagnostics;
            match outcome.status {
                "exited_ok" if validation.checks.all() => {
                    row["status"] = json!("validated");
                }
                "exited_ok" => {
                    row["status"] = json!("rejected");
                    row["reason"] = json!(validation.reason);
                }
                "timeout" if options.case_limit > remaining => {
                    row["status"] = json!("timeout_deadline");
                    row["reason"] = json!(outcome.detail);
                }
                status => {
                    row["status"] = json!(status);
                    row["reason"] = json!(outcome.detail);
                }
            }
            if bytes.is_some() && outcome.status == "exited_ok" {
                let remaining = options.deadline.saturating_sub(started.elapsed());
                if remaining.is_zero() {
                    row["replay_status"] = json!("not_run_deadline");
                } else {
                    let replay = run_child(
                        binary,
                        &[Path::new("verify"), &receipt_path],
                        &logs.join(format!("{stem}.verify.out")),
                        &logs.join(format!("{stem}.verify.err")),
                        options.case_limit.min(remaining),
                    );
                    row["replay_status"] = json!(match replay.status {
                        "exited_ok" => "verified",
                        other => other,
                    });
                    row["replay_wall_ns"] = json!(replay.wall_ns.map(|n| n as u64));
                    row["phase_ns"] = phase_ns(replay.wall_ns);
                    if replay.status != "exited_ok" && row["status"] == "validated" {
                        // The arm's own replay disagreeing is a failure too.
                        row["status"] = json!("rejected");
                        row["reason"] = json!("arm replay did not verify its receipt");
                    }
                }
            }
        }
        writeln!(rows_file, "{}", serde_json::to_string(&row)?)?;
        rows_file.sync_data()?;
        rows.push(row);
    }
    // Binaries must not change under the driver.
    let binaries_unchanged =
        file_sha256(&baseline)? == baseline_sha && file_sha256(&candidate)? == candidate_sha;
    let mut analysis = analyze(&rows);
    analysis["label"] = json!(options.label);
    analysis["protocol_complete"] = json!(protocol_complete);
    analysis["evidence_class"] = json!(if protocol_complete {
        "protocol_schedule_requires_external_L2_admission"
    } else {
        "diagnostic_incomplete_panel"
    });
    analysis["inputs"] = json!({
        "manifest_sha256": panel.manifest_sha256,
        "inputs_sha256_file_sha256": file_sha256(&options.inputs_sha256)?,
        "selected_cases": selected.len(),
    });
    analysis["arms"] = json!({
        "baseline_sha256": baseline_sha, "candidate_sha256": candidate_sha,
        "unchanged_after_run": binaries_unchanged,
    });
    analysis["schedule"] = json!({
        "seed": options.seed.to_string(), "aa_rounds": options.aa_rounds,
        "ab_rounds": options.ab_rounds, "deadline_ms": options.deadline.as_millis() as u64,
        "case_limit_ms": options.case_limit.as_millis() as u64,
        "executions": plan.len(), "driver_elapsed_ns": started.elapsed().as_nanos() as u64,
    });
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(options.out.join("analysis.json"))?;
    serde_json::to_writer_pretty(&mut file, &analysis)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    if !binaries_unchanged {
        return Err("an arm binary changed during the run; analysis retained but invalid".into());
    }
    Ok(analysis)
}

/// The arm binaries expose no internal phase timers. Only the separately
/// spawned replay is a disjoint phase; the rest stay null rather than guessed.
fn phase_ns(replay: Option<u128>) -> Value {
    json!({
        "accounting": "arm_binaries_expose_no_phase_timers; solve_wall_ns covers setup through extraction",
        "setup": null, "proof_validation": null, "enumeration": null, "encoding": null,
        "relation_attempts": null, "witness_search": null, "matrix": null,
        "replay": replay.map(|n| n as u64),
    })
}

// ---------------------------------------------------------------- analysis

/// Two-sided 0.975 Student-t quantile (Cornish-Fisher expansion about the
/// normal quantile, absolute error below 1e-4 for df > 30; table at or below 30).
pub fn t975(df: usize) -> Option<f64> {
    // Standard t-table values (4 decimals) for small df.
    const SMALL: [f64; 30] = [
        12.7062, 4.3027, 3.1824, 2.7764, 2.5706, 2.4469, 2.3646, 2.3060, 2.2622, 2.2281, 2.2010,
        2.1788, 2.1604, 2.1448, 2.1314, 2.1199, 2.1098, 2.1009, 2.0930, 2.0860, 2.0796, 2.0739,
        2.0687, 2.0639, 2.0595, 2.0555, 2.0518, 2.0484, 2.0452, 2.0423,
    ];
    match df {
        0 => None,
        1..=30 => Some(SMALL[df - 1]),
        _ => {
            let z: f64 = 1.959_963_984_540_054;
            let n = df as f64;
            let z3 = z.powi(3);
            let z5 = z.powi(5);
            let z7 = z.powi(7);
            Some(
                z + (z3 + z) / (4.0 * n)
                    + (5.0 * z5 + 16.0 * z3 + 3.0 * z) / (96.0 * n * n)
                    + (3.0 * z7 + 19.0 * z5 + 17.0 * z3 - 15.0 * z) / (384.0 * n.powi(3)),
            )
        }
    }
}

/// Paired analysis. Unit of inference is the case: the per-case mean of
/// ln(member1/member0) over rounds where both members validated, then a
/// Student-t 95% interval over cases. Failed, rejected, timed-out and
/// deadline-skipped executions exclude that pair from the interval but are
/// counted by status; they are never dropped from rows.jsonl.
pub fn analyze(rows: &[Value]) -> Value {
    let mut result = json!({
        "schema": ANALYSIS_SCHEMA,
        "choices": {
            "unit_of_inference": "case (mean of per-round paired log ratios)",
            "effect": "ln(wall member1 / wall member0); member1 = candidate in A/B, second baseline copy in A/A",
            "interval": "two-sided 95% Student-t over cases",
            "pair_exclusion": "pair enters the interval only if both members validated",
            "warmup": "one pair before round 0, retained, excluded",
        },
        "runtime_claim_admissible": null,
        "runtime_claim_note": "Requires an externally admitted matched L2 host; this driver cannot attest isolation. Local runs are diagnostics.",
        "unpriced_common_operations": null, "s_over_rho": null,
    });
    for kind in [RoundKind::AA, RoundKind::AB] {
        let mut status_counts: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
        let mut pairs: BTreeMap<(u64, String), [Option<&Value>; 2]> = BTreeMap::new();
        for row in rows.iter().filter(|r| r["kind"] == kind.label()) {
            let arm_key = format!("member{}", row["pair_member"]);
            *status_counts
                .entry(arm_key)
                .or_default()
                .entry(row["status"].as_str().unwrap_or("unknown").to_string())
                .or_default() += 1;
            let key = (
                row["round"].as_u64().unwrap_or(u64::MAX),
                row["case_id"].as_str().unwrap_or("").to_string(),
            );
            let member = row["pair_member"].as_u64().unwrap_or(0).min(1) as usize;
            pairs.entry(key).or_insert([None, None])[member] = Some(row);
        }
        let mut per_case: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        let (mut complete, mut excluded, mut identical, mut compared) = (0u64, 0u64, 0u64, 0u64);
        for ((_, case), [m0, m1]) in &pairs {
            let ok = |r: &Option<&Value>| {
                r.filter(|r| r["status"] == "validated")
                    .and_then(|r| r["solve_wall_ns"].as_u64())
                    .filter(|&n| n > 0)
            };
            if let (Some(a), Some(b)) = (m0, m1) {
                if a["certificate_sha256"].is_string() && b["certificate_sha256"].is_string() {
                    compared += 1;
                    if a["certificate_sha256"] == b["certificate_sha256"] {
                        identical += 1;
                    }
                }
            }
            match (ok(m0), ok(m1)) {
                (Some(a), Some(b)) => {
                    complete += 1;
                    per_case
                        .entry(case.clone())
                        .or_default()
                        .push((b as f64 / a as f64).ln());
                }
                _ => excluded += 1,
            }
        }
        let means: Vec<f64> = per_case
            .values()
            .map(|v| v.iter().sum::<f64>() / v.len() as f64)
            .collect();
        let n = means.len();
        let (mean, low, high) = if n >= 2 {
            let mean = means.iter().sum::<f64>() / n as f64;
            let var = means.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
            let half = t975(n - 1).unwrap() * (var / n as f64).sqrt();
            (Some(mean), Some(mean - half), Some(mean + half))
        } else {
            (means.first().copied(), None, None)
        };
        let finite = |x: Option<f64>| x.filter(|v| v.is_finite());
        result[kind.label()] = json!({
            "pairs": pairs.len() as u64, "complete_pairs": complete, "excluded_pairs": excluded,
            "cases_in_interval": n as u64, "status_counts": status_counts,
            "certificate_pairs_compared": compared, "certificate_pairs_identical": identical,
            "mean_log_ratio": finite(mean), "ci95_log_ratio": [finite(low), finite(high)],
            "ratio_point": finite(mean.map(f64::exp)),
            "ci95_ratio": [finite(low.map(f64::exp)), finite(high.map(f64::exp))],
            "interval_excludes_no_change": match (low, high) {
                (Some(l), Some(h)) => json!(l > 0.0 || h < 0.0), _ => Value::Null },
        });
    }
    result
}

// ---------------------------------------------------------------- CLI

fn value<'a>(args: &'a [String], i: &mut usize, flag: &str) -> Result<&'a str> {
    *i += 1;
    args.get(*i)
        .map(String::as_str)
        .ok_or_else(|| format!("{flag} needs a value").into())
}

pub const USAGE: &str = "usage: hashed-ecdl-panel run --panel DIR --inputs-sha256 FILE --baseline BIN --candidate BIN --out NEW_DIR --seed U64 --case-limit-secs S [--deadline-secs 1800] [--aa-rounds 5] [--ab-rounds 5] [--cases ID,ID] [--label TEXT] [--allow-incomplete-panel]\n       hashed-ecdl-panel analyze ROWS.jsonl\n       hashed-ecdl-panel hash PANEL_DIR";

pub fn cli_run(args: &[String]) -> Result<()> {
    let mut panel = None;
    let mut hashes = None;
    let mut baseline = None;
    let mut candidate = None;
    let mut out = None;
    let mut seed = None;
    let mut case_limit = None;
    let mut deadline = PROTOCOL_DEADLINE_SECS;
    let mut aa = MIN_AA_ROUNDS;
    let mut ab = MIN_AB_ROUNDS;
    let mut cases = None;
    let mut label = String::from("unlabelled");
    let mut allow = false;
    let mut i = 0;
    while i < args.len() {
        let flag = args[i].as_str();
        match flag {
            "--panel" => panel = Some(PathBuf::from(value(args, &mut i, flag)?)),
            "--inputs-sha256" => hashes = Some(PathBuf::from(value(args, &mut i, flag)?)),
            "--baseline" => baseline = Some(PathBuf::from(value(args, &mut i, flag)?)),
            "--candidate" => candidate = Some(PathBuf::from(value(args, &mut i, flag)?)),
            "--out" => out = Some(PathBuf::from(value(args, &mut i, flag)?)),
            "--seed" => seed = Some(value(args, &mut i, flag)?.parse::<u64>()?),
            "--case-limit-secs" => case_limit = Some(value(args, &mut i, flag)?.parse::<f64>()?),
            "--deadline-secs" => deadline = value(args, &mut i, flag)?.parse()?,
            "--aa-rounds" => aa = value(args, &mut i, flag)?.parse()?,
            "--ab-rounds" => ab = value(args, &mut i, flag)?.parse()?,
            "--cases" => {
                cases = Some(
                    value(args, &mut i, flag)?
                        .split(',')
                        .map(str::to_string)
                        .collect(),
                )
            }
            "--label" => label = value(args, &mut i, flag)?.to_string(),
            "--allow-incomplete-panel" => allow = true,
            _ => return Err(format!("unknown argument {flag}\n{USAGE}").into()),
        }
        i += 1;
    }
    let missing = |name: &str| format!("missing {name}\n{USAGE}");
    let case_limit = case_limit.ok_or_else(|| missing("--case-limit-secs"))?;
    if !(case_limit.is_finite() && case_limit > 0.0) {
        return Err("--case-limit-secs must be positive".into());
    }
    let options = RunOptions {
        panel: panel.ok_or_else(|| missing("--panel"))?,
        inputs_sha256: hashes.ok_or_else(|| missing("--inputs-sha256"))?,
        baseline: baseline.ok_or_else(|| missing("--baseline"))?,
        candidate: candidate.ok_or_else(|| missing("--candidate"))?,
        out: out.ok_or_else(|| missing("--out"))?,
        seed: seed.ok_or_else(|| missing("--seed"))?,
        aa_rounds: aa,
        ab_rounds: ab,
        deadline: Duration::from_secs(deadline),
        case_limit: Duration::from_secs_f64(case_limit),
        cases,
        allow_incomplete_panel: allow,
        label,
    };
    let analysis = run(&options)?;
    println!("{}", serde_json::to_string(&analysis)?);
    Ok(())
}

pub fn cli_analyze(args: &[String]) -> Result<()> {
    let [path] = args else {
        return Err(USAGE.into());
    };
    let rows = fs::read_to_string(path)?
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(serde_json::from_str)
        .collect::<std::result::Result<Vec<Value>, _>>()?;
    if rows.iter().any(|r| r["schema"] != ROW_SCHEMA) {
        return Err("rows file contains a row with an unexpected schema".into());
    }
    println!("{}", serde_json::to_string_pretty(&analyze(&rows))?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_matches_fips_vectors() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
        assert_eq!(
            sha256_hex(&vec![b'a'; 1_000_000]),
            "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
        );
    }

    #[test]
    fn hash_list_rejects_duplicates_and_malformed_lines() {
        let h = "a".repeat(64);
        let ok =
            parse_sha256_list(&format!("{h}  manifest.json\n{h}  ./problems/x.json\n")).unwrap();
        assert_eq!(ok.len(), 2);
        assert!(ok.contains_key("problems/x.json"));
        assert!(parse_sha256_list(&format!("{h}  a\n{h}  a\n")).is_err());
        assert!(parse_sha256_list("abc  a\n").is_err());
        assert!(parse_sha256_list(&format!("{} a\n", h)).is_err());
    }

    #[test]
    fn schedule_is_seeded_latin_and_balanced() {
        let plan = schedule(84, 5, 5, 20261031);
        assert_eq!(plan, schedule(84, 5, 5, 20261031));
        assert_ne!(plan, schedule(84, 5, 5, 20261032));
        assert_eq!(plan.len(), 2 + 10 * 84 * 2);
        let rounds: Vec<_> = plan
            .iter()
            .filter(|e| e.kind != RoundKind::Warmup)
            .collect();
        let kinds: Vec<_> = rounds
            .chunks(168)
            .map(|c| {
                assert!(c.iter().all(|e| e.kind == c[0].kind));
                c[0].kind
            })
            .collect();
        assert_eq!(kinds.iter().filter(|&&k| k == RoundKind::AA).count(), 5);
        assert!(kinds.windows(2).all(|w| w[0] != w[1]), "interleaved");
        let mut positions: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        for (round, chunk) in rounds.chunks(168).enumerate() {
            let cases: BTreeSet<_> = chunk.iter().map(|e| e.case).collect();
            assert_eq!(cases.len(), 84, "every case once per round");
            let member1_first = chunk
                .iter()
                .filter(|e| e.slot == 0 && e.pair_member == 1)
                .count();
            assert_eq!(member1_first, 42, "member order balanced in round {round}");
            for pair in chunk.chunks(2) {
                assert_eq!(pair[0].case, pair[1].case);
                assert_ne!(pair[0].pair_member, pair[1].pair_member);
                assert!(positions
                    .entry(pair[0].case)
                    .or_default()
                    .insert(pair[0].position));
            }
        }
        // Each case occupies a distinct position in each of the ten rounds.
        assert!(positions.values().all(|p| p.len() == 10));
        // Member order alternates for a case between consecutive rounds of
        // the same parity class, so no case always runs the candidate first.
        for case in 0..84 {
            let firsts: BTreeSet<_> = rounds
                .iter()
                .filter(|e| e.case == case && e.slot == 0)
                .map(|e| e.pair_member)
                .collect();
            assert_eq!(firsts.len(), 2, "case {case}");
        }
        assert_eq!(plan[0].arm(), "baseline");
        assert_eq!(plan[1].arm(), "candidate");
        assert!(rounds
            .iter()
            .filter(|e| e.kind == RoundKind::AA)
            .all(|e| e.arm() == "baseline"));
    }

    #[test]
    fn t_quantile_is_close_to_tables() {
        for (df, expected) in [
            (4, 2.776_445),
            (10, 2.228_139),
            (30, 2.042_272),
            (83, 1.988_959),
        ] {
            assert!((t975(df).unwrap() - expected).abs() < 2e-4, "df {df}");
        }
        assert!(t975(0).is_none());
    }

    fn row(kind: &str, round: u64, case: &str, member: u64, status: &str, ns: u64) -> Value {
        json!({"schema": ROW_SCHEMA, "kind": kind, "round": round, "case_id": case,
            "pair_member": member, "status": status, "solve_wall_ns": ns,
            "certificate_sha256": "c"})
    }

    #[test]
    fn analysis_identity_gives_zero_and_retains_failures() {
        let mut rows = Vec::new();
        for round in 0..5 {
            for (i, case) in ["a", "b", "c", "d"].iter().enumerate() {
                let ns = 1000 + 37 * i as u64 + round;
                rows.push(row("aa", round, case, 0, "validated", ns));
                rows.push(row("aa", round, case, 1, "validated", ns));
            }
        }
        rows.push(row("ab", 1, "a", 0, "validated", 100));
        rows.push(row("ab", 1, "a", 1, "timeout", 900));
        rows.push(row("ab", 1, "b", 0, "not_run_deadline", 0));
        let a = analyze(&rows);
        assert_eq!(a["aa"]["mean_log_ratio"], 0.0);
        assert_eq!(a["aa"]["ci95_log_ratio"], json!([0.0, 0.0]));
        assert_eq!(a["aa"]["certificate_pairs_identical"], 20);
        assert_eq!(a["aa"]["interval_excludes_no_change"], false);
        assert_eq!(a["ab"]["complete_pairs"], 0);
        assert_eq!(a["ab"]["excluded_pairs"], 2);
        assert_eq!(a["ab"]["status_counts"]["member1"]["timeout"], 1);
        assert_eq!(a["ab"]["status_counts"]["member0"]["not_run_deadline"], 1);
        assert!(a["ab"]["mean_log_ratio"].is_null());
        assert!(a["s_over_rho"].is_null());
    }

    #[test]
    fn analysis_detects_a_uniform_shift() {
        let mut rows = Vec::new();
        for round in 0..5 {
            for (i, case) in ["a", "b", "c", "d", "e"].iter().enumerate() {
                let base = 10_000 + 1000 * i as u64 + 7 * round;
                rows.push(row("ab", round, case, 0, "validated", base));
                rows.push(row("ab", round, case, 1, "validated", base / 2 + round));
            }
        }
        let a = analyze(&rows);
        assert_eq!(a["ab"]["interval_excludes_no_change"], true);
        let point = a["ab"]["ratio_point"].as_f64().unwrap();
        assert!((point - 0.5).abs() < 0.01, "{point}");
    }
}
