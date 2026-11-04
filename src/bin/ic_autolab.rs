//! `ic_autolab`: the index-calculus improvement loop for one curve family,
//! first the Curve25519 miniatures (`research/curve25519_autolab_20261104`).
//!
//! The loop is the owner's success criterion made mechanical (AGENTS.md,
//! "Success criterion: iterative improvement of the index-calculus
//! baseline"): a named incumbent per track and size, a frozen protocol before
//! data, a development screen on fresh public targets, one selected challenger
//! confirmed on a fresh holdout against the incumbent, an A/A control and the
//! matched rho reference, the §5a pricing band, and a ratchet that makes a
//! verified improvement the new incumbent and records its delta.
//!
//! Every measurement is an `ecbench` session (native, verified, replayable).
//! This binary never computes an ECDLP result itself: it writes specs, calls
//! `ecbench run / verify / band / compare`, reads their JSON back, classifies
//! under the frozen rule, and keeps the ledger.  Candidate generation ports
//! the autolab's `propose` / `recombine` / portfolio rules
//! (`research/ic_candidate_tournament_20260915/AUTOLAB.md`) onto ecbench arms.
//!
//! One round, one directory `round-NNN/` under the loop's home:
//!
//! ```text
//! propose ─► freeze ─► run train ─► select ─► run holdout ─► assess ─► ratchet ─► report
//!   panel     PROTOCOL    sessions/   selection   sessions/     RESULT     ledger     README
//!   .json     specs/      train-k*    .json       holdout-k*    .json      .json      .svg
//! ```
//!
//! `card` emits the same train or holdout stage as a frozen-command task card
//! for the director/executor relay (AGENTS.md "Model tiers"); `run` is the
//! explicitly requested local route (counted units only, no wall-time claim).

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

const LEDGER_SCHEMA: &str = "ic_autolab.ledger/v1";
const PANEL_SCHEMA: &str = "ic_autolab.panel/v1";
const RESULT_SCHEMA: &str = "ic_autolab.result/v1";
const FAMILIES_SCHEMA: &str = "ic_autolab.families/v1";
/// Target seeds of round `r` start here; `freeze` moves on past any seed the
/// ledger already lists as exposed.
const SEED_BASE: u64 = 2_026_110_400;

#[derive(Parser)]
#[command(
    name = "ic_autolab",
    version,
    about = "The index-calculus improvement loop (propose, freeze, run, select, assess, ratchet, report) on ecbench"
)]
struct Cli {
    /// The loop's home directory (ledger.json, families.json, round-NNN/).
    #[arg(long, default_value = "research/curve25519_autolab_20261104")]
    home: PathBuf,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Create `ledger.json` from a seed ledger (the named incumbents with
    /// their provenance). Refuses to overwrite.
    Init {
        #[arg(long)]
        from: PathBuf,
    },
    /// Print the ledger: incumbents per track and size, distance to rho,
    /// rounds and exposures.
    Status,
    /// Write the candidate panel of a round from the incumbents, the family
    /// table and the trial history (neighbourhood, alternatives,
    /// exploration slots; never a configuration already tried at that size).
    Propose {
        #[arg(long)]
        round: u64,
        #[arg(long, default_value = "generic")]
        track: String,
        /// Sizes k (bits of p), comma separated.
        #[arg(long, default_value = "20,24")]
        sizes: String,
        /// Candidate arms per size, incumbent and reference excluded.
        #[arg(long, default_value_t = 6)]
        max_candidates: usize,
        /// Of those, slots for families not yet tried at the size.
        #[arg(long, default_value_t = 2)]
        exploration: usize,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Freeze the round: PROTOCOL.md, train specs and MANIFEST.json, from a
    /// panel. Refuses an existing round directory. With `--ecbench`, every
    /// spec is validated with `ecbench plan` before anything is written.
    Freeze {
        #[arg(long)]
        round: u64,
        #[arg(long)]
        panel: PathBuf,
        #[arg(long, default_value_t = 16)]
        train_targets: u64,
        #[arg(long, default_value_t = 1)]
        train_rounds: u64,
        #[arg(long, default_value_t = 32)]
        holdout_targets: u64,
        #[arg(long, default_value_t = 2)]
        holdout_rounds: u64,
        #[arg(long, default_value_t = 1800)]
        timeout_seconds: u64,
        /// `local` (explicitly requested local route, counted units) or
        /// `eks` (the default lab route, through a task card).
        #[arg(long, default_value = "local")]
        route: String,
        #[arg(long)]
        ecbench: Option<PathBuf>,
    },
    /// Run one stage's sessions locally (`ecbench run`), audit each
    /// (`ecbench verify --replay`), and write the §5a band. Each session
    /// directory must not exist. A failed run is recorded, not retried.
    Run {
        #[arg(long)]
        round: u64,
        /// train or holdout.
        #[arg(long)]
        stage: String,
        #[arg(long, default_value = "target/release/ecbench")]
        ecbench: PathBuf,
        #[arg(long, default_value = "auto")]
        cpus: String,
        #[arg(long)]
        allow_busy: bool,
        #[arg(long, default_value_t = 8)]
        replay: usize,
    },
    /// Emit a frozen-command task card for one stage (the director/executor
    /// relay); `python3 tools/task_card.py lint` must accept it.
    Card {
        #[arg(long)]
        round: u64,
        #[arg(long)]
        stage: String,
        #[arg(long, default_value = "target/release/ecbench")]
        ecbench: PathBuf,
        #[arg(long, default_value = "auto")]
        cpus: String,
        #[arg(long)]
        allow_busy: bool,
        #[arg(long, default_value_t = 8)]
        replay: usize,
        /// The card's ordinal in `CARD-<experiment>-<date>-<n>`.
        #[arg(long, default_value_t = 1)]
        n: u64,
    },
    /// Read the train bands, select one challenger per size (lowest S at
    /// w = 0 among verified candidates, in sample, disclosed), write the
    /// holdout specs and append the selection amendment to PROTOCOL.md.
    Select {
        #[arg(long)]
        round: u64,
        #[arg(long)]
        ecbench: Option<PathBuf>,
    },
    /// Compare sel against inc on every holdout session (paired ratio with
    /// its interval, the band order at every weight, the A/A control) and
    /// classify each size under the frozen rule into RESULT.json.
    Assess {
        #[arg(long)]
        round: u64,
        #[arg(long, default_value = "target/release/ecbench")]
        ecbench: PathBuf,
    },
    /// Apply RESULT.json to the ledger: an improvement becomes the new
    /// incumbent with its delta; every arm joins the trial history; every
    /// target seed joins the exposures. Once per round.
    Ratchet {
        #[arg(long)]
        round: u64,
    },
    /// Write the round's README.md and SVG graphs from the recorded values,
    /// and refresh the loop's ratchet graph.
    Report {
        #[arg(long)]
        round: u64,
    },
    /// Propose, freeze, run train, select, run holdout, assess, ratchet and
    /// report, for `--rounds` consecutive rounds on the local route.
    Loop {
        #[arg(long, default_value_t = 1)]
        rounds: u64,
        #[arg(long, default_value = "generic")]
        track: String,
        #[arg(long, default_value = "20,24")]
        sizes: String,
        #[arg(long, default_value_t = 6)]
        max_candidates: usize,
        #[arg(long, default_value_t = 2)]
        exploration: usize,
        #[arg(long, default_value_t = 16)]
        train_targets: u64,
        #[arg(long, default_value_t = 1)]
        train_rounds: u64,
        #[arg(long, default_value_t = 32)]
        holdout_targets: u64,
        #[arg(long, default_value_t = 2)]
        holdout_rounds: u64,
        #[arg(long, default_value_t = 1800)]
        timeout_seconds: u64,
        #[arg(long, default_value = "target/release/ecbench")]
        ecbench: PathBuf,
        #[arg(long, default_value = "auto")]
        cpus: String,
        #[arg(long)]
        allow_busy: bool,
        #[arg(long, default_value_t = 8)]
        replay: usize,
    },
}

// ---------------------------------------------------------------------------
// Ledger and families
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct ArmConfig {
    name: String,
    family: String,
    n: u64,
    factor_base: String,
    oracle: String,
}

impl ArmConfig {
    fn key(&self) -> (String, String) {
        (self.factor_base.clone(), self.oracle.clone())
    }
    fn method(&self) -> Value {
        json!({"id": "ic.pipeline", "params": {"factor_base": self.factor_base, "oracle": self.oracle}})
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Incumbent {
    arm: ArmConfig,
    /// Mean S at w = 0, 0.1, 1 GAE per unpriced native operation.
    s: [f64; 3],
    rho_s: [f64; 3],
    evidence: Value,
    since_round: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SizeInfo {
    k: u32,
    r: u64,
    log2_r: f64,
    n_a: u64,
    curve: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Trial {
    round: u64,
    stage: String,
    size: u32,
    track: String,
    arm: ArmConfig,
    s: Option<[f64; 3]>,
    status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Exposure {
    size: u32,
    target_seed: u64,
    targets: u64,
    source: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RoundEntry {
    round: u64,
    dir: String,
    track: String,
    sizes: Vec<u32>,
    status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RatchetEvent {
    round: u64,
    size: u32,
    track: String,
    from: ArmConfig,
    to: ArmConfig,
    s_from: [f64; 3],
    s_to: [f64; 3],
    ratio: f64,
    ci95: [f64; 2],
    session: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Ledger {
    schema: String,
    family: String,
    created: String,
    updated: String,
    #[serde(default)]
    notes: Vec<String>,
    sizes: BTreeMap<String, SizeInfo>,
    /// track -> k -> incumbent.
    tracks: BTreeMap<String, BTreeMap<String, Incumbent>>,
    #[serde(default)]
    exposures: Vec<Exposure>,
    #[serde(default)]
    trials: Vec<Trial>,
    #[serde(default)]
    rounds: Vec<RoundEntry>,
    #[serde(default)]
    history: Vec<RatchetEvent>,
}

impl Ledger {
    fn size(&self, k: u32) -> Result<&SizeInfo, String> {
        self.sizes
            .get(&k.to_string())
            .ok_or_else(|| format!("ledger has no size k = {k}"))
    }
    fn incumbent(&self, track: &str, k: u32) -> Result<&Incumbent, String> {
        self.tracks
            .get(track)
            .and_then(|t| t.get(&k.to_string()))
            .ok_or_else(|| format!("ledger has no incumbent for track {track}, k = {k}"))
    }
    fn tried(&self, track: &str, k: u32) -> BTreeSet<(String, String)> {
        let mut set: BTreeSet<(String, String)> = self
            .trials
            .iter()
            .filter(|t| t.track == track && t.size == k)
            .map(|t| t.arm.key())
            .collect();
        if let Ok(inc) = self.incumbent(track, k) {
            set.insert(inc.arm.key());
        }
        set
    }
    fn exposed_seeds(&self) -> BTreeSet<u64> {
        self.exposures.iter().map(|e| e.target_seed).collect()
    }
}

#[derive(Deserialize, Clone, Debug)]
struct Family {
    id: String,
    oracle: String,
    default_f: f64,
    #[serde(default)]
    #[allow(dead_code)]
    note: String,
    #[serde(default = "default_true")]
    enabled: bool,
    /// A family's own floor on N (a k-sum table needs more base points
    /// than summands); the track's `n_min` applies when absent.
    #[serde(default)]
    n_min: Option<u64>,
}

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Clone, Debug)]
struct TrackFamilies {
    factor_base: String,
    scan_factor: u64,
    n_grid: Vec<f64>,
    n_min: u64,
    n_max: u64,
    families: Vec<Family>,
}

impl TrackFamilies {
    fn arm(&self, fam: &Family, n: u64) -> ArmConfig {
        let n = n.clamp(fam.n_min.unwrap_or(self.n_min).max(self.n_min), self.n_max);
        ArmConfig {
            name: format!("{}-n{n}", fam.id),
            family: fam.id.clone(),
            n,
            factor_base: self
                .factor_base
                .replace("{n}", &n.to_string())
                .replace("{scan}", &(self.scan_factor * n).to_string()),
            oracle: fam.oracle.clone(),
        }
    }
    fn family(&self, id: &str) -> Option<&Family> {
        self.families.iter().find(|f| f.id == id)
    }
}

#[derive(Deserialize, Debug)]
struct Families {
    schema: String,
    tracks: BTreeMap<String, TrackFamilies>,
}

// ---------------------------------------------------------------------------
// Small utilities
// ---------------------------------------------------------------------------

fn read_json(path: &Path) -> Result<Value, String> {
    let text =
        std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", path.display()))
}

fn write_json(path: &Path, v: &Value) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("mkdir {}: {e}", dir.display()))?;
    }
    let text = serde_json::to_string_pretty(v).map_err(|e| e.to_string())? + "\n";
    std::fs::write(path, text).map_err(|e| format!("write {}: {e}", path.display()))
}

fn write_new(path: &Path, text: &str) -> Result<(), String> {
    if path.exists() {
        return Err(format!(
            "{} exists; a frozen file is never rewritten",
            path.display()
        ));
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("mkdir {}: {e}", dir.display()))?;
    }
    std::fs::write(path, text).map_err(|e| format!("write {}: {e}", path.display()))
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    Ok(hex(&Sha256::digest(&bytes)))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// UTC time as `YYYY-MM-DDTHH:MM:SSZ` from the system clock (no chrono).
fn now_utc() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

/// Howard Hinnant's days-to-civil conversion.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn date_compact() -> String {
    now_utc()[..10].replace('-', "")
}

fn git_head() -> Option<String> {
    let out = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Run a command, tee its output to `log`, return (exit code, stdout).
fn run_logged(argv: &[String], log: Option<&Path>) -> Result<(i32, String), String> {
    eprintln!("$ {}", argv.join(" "));
    let out = Command::new(&argv[0])
        .args(&argv[1..])
        .output()
        .map_err(|e| format!("spawn {}: {e}", argv[0]))?;
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    if let Some(log) = log {
        if let Some(dir) = log.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        std::fs::write(
            log,
            format!(
                "$ {}\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}\n--- exit {:?} ---\n",
                argv.join(" "),
                out.status.code()
            ),
        )
        .map_err(|e| e.to_string())?;
    }
    Ok((out.status.code().unwrap_or(-1), stdout))
}

fn parse_sizes(s: &str) -> Result<Vec<u32>, String> {
    s.split(',')
        .map(|t| {
            t.trim()
                .parse::<u32>()
                .map_err(|e| format!("size {t}: {e}"))
        })
        .collect()
}

fn round_dir(home: &Path, round: u64) -> PathBuf {
    home.join(format!("round-{round:03}"))
}

fn load_ledger(home: &Path) -> Result<Ledger, String> {
    let v = read_json(&home.join("ledger.json"))?;
    let l: Ledger = serde_json::from_value(v).map_err(|e| format!("ledger.json: {e}"))?;
    if l.schema != LEDGER_SCHEMA {
        return Err(format!(
            "ledger.json schema {} is not {LEDGER_SCHEMA}",
            l.schema
        ));
    }
    Ok(l)
}

fn save_ledger(home: &Path, l: &mut Ledger) -> Result<(), String> {
    l.updated = now_utc();
    write_json(
        &home.join("ledger.json"),
        &serde_json::to_value(&*l).map_err(|e| e.to_string())?,
    )
}

fn load_families(home: &Path) -> Result<Families, String> {
    let v = read_json(&home.join("families.json"))?;
    let f: Families = serde_json::from_value(v).map_err(|e| format!("families.json: {e}"))?;
    if f.schema != FAMILIES_SCHEMA {
        return Err(format!(
            "families.json schema {} is not {FAMILIES_SCHEMA}",
            f.schema
        ));
    }
    Ok(f)
}

fn s3(v: &Value) -> Option<[f64; 3]> {
    let a = v.as_array()?;
    Some([
        a.first()?.as_f64()?,
        a.get(1)?.as_f64()?,
        a.get(2)?.as_f64()?,
    ])
}

fn fmt_s(s: &[f64; 3]) -> String {
    format!("{:.3} / {:.3} / {:.3}", s[0], s[1], s[2])
}

fn distance(s: &[f64; 3], rho: &[f64; 3]) -> [f64; 3] {
    [s[0] / rho[0], s[1] / rho[1], s[2] / rho[2]]
}

// ---------------------------------------------------------------------------
// propose
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Proposal {
    arm: ArmConfig,
    reason: String,
}

/// Least-squares fit of ln N on log2 r over the sizes (other than `k`) where
/// `family` is the incumbent: `(intercept, slope, points)`, when there are at
/// least two such sizes.
fn family_fit(ledger: &Ledger, track: &str, family: &str, k: u32) -> Option<(f64, f64, usize)> {
    let pts: Vec<(f64, f64)> = ledger
        .tracks
        .get(track)?
        .iter()
        .filter_map(|(k2, inc)| {
            let k2: u32 = k2.parse().ok()?;
            (k2 != k && inc.arm.family == family)
                .then(|| Some((ledger.size(k2).ok()?.log2_r, (inc.arm.n as f64).ln())))?
        })
        .collect();
    if pts.len() < 2 {
        return None;
    }
    let n = pts.len() as f64;
    let mx = pts.iter().map(|p| p.0).sum::<f64>() / n;
    let my = pts.iter().map(|p| p.1).sum::<f64>() / n;
    let sxx: f64 = pts.iter().map(|p| (p.0 - mx).powi(2)).sum();
    if sxx == 0.0 {
        return None;
    }
    let b = pts.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum::<f64>() / sxx;
    Some((my - b * mx, b, pts.len()))
}

/// Candidate arms for one size. Order of priority, each skipping anything
/// already tried at this size (the ledger's trials and the incumbent):
///
/// 1. the incumbent family's base-size neighbourhood, nearest factors first;
/// 2. cross-size transfer: the incumbent family of every other size (nearest
///    in log2 r first), when it differs from this size's, at N extrapolated
///    along the family's own incumbents when it holds two or more sizes,
///    else at the same N / N_a, and its grid neighbours (a winner at
///    one size is a hypothesis at the next; the knob is scale-relative, as in
///    `ic sweep`);
/// 3. refinement of alternatives: families already tried at this size whose
///    best verified S(0) is within 1.5x of the incumbent get their best base
///    size moved by the two nearest grid factors (the Pareto alternatives
///    the autolab keeps so that one lucky family does not close the search);
/// 4. exploration slots: families never tried at this size, those with the
///    fewest trials anywhere first, at `default_f * N_a`.
fn propose_size(
    ledger: &Ledger,
    tf: &TrackFamilies,
    track: &str,
    k: u32,
    max_candidates: usize,
    exploration: usize,
) -> Result<Vec<Proposal>, String> {
    let size = ledger.size(k)?;
    let inc = ledger.incumbent(track, k)?;
    let tried = ledger.tried(track, k);
    let mut taken: BTreeSet<(String, String)> = tried.clone();
    let mut out: Vec<Proposal> = Vec::new();
    let mut push = |arm: ArmConfig, reason: String, out: &mut Vec<Proposal>| {
        if taken.insert(arm.key()) {
            out.push(Proposal { arm, reason });
        }
    };
    let mut grid: Vec<f64> = tf.n_grid.clone();
    grid.sort_by(|a, b| (a.ln().abs()).partial_cmp(&b.ln().abs()).unwrap());

    // 1. neighbourhood of the incumbent.
    if let Some(fam) = tf.family(&inc.arm.family) {
        if fam.enabled {
            for f in &grid {
                let n = ((inc.arm.n as f64) * f).round() as u64;
                let arm = tf.arm(fam, n);
                push(
                    arm,
                    format!("incumbent neighbourhood: N = {f} x {}", inc.arm.n),
                    &mut out,
                );
            }
        }
    }

    // 2. cross-size transfer of other sizes' incumbent families.
    if let Some(sizes) = ledger.tracks.get(track) {
        let mut others: Vec<(f64, u32, &Incumbent)> = sizes
            .iter()
            .filter_map(|(k2, inc2)| {
                let k2: u32 = k2.parse().ok()?;
                let s2 = ledger.size(k2).ok()?;
                (k2 != k && inc2.arm.family != inc.arm.family)
                    .then(|| ((s2.log2_r - size.log2_r).abs(), k2, inc2))
            })
            .collect();
        others.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(a.1.cmp(&b.1)));
        for (_, k2, inc2) in others {
            let Some(fam) = tf.family(&inc2.arm.family).filter(|f| f.enabled) else {
                continue;
            };
            let n_a2 = ledger.size(k2)?.n_a as f64;
            let frac = inc2.arm.n as f64 / n_a2;
            // With the family incumbent at two or more sizes, extrapolate N
            // along its own incumbents (least squares of ln N on log2 r);
            // otherwise hold N / N_a fixed.
            let (n0, why) = match family_fit(ledger, track, &inc2.arm.family, k) {
                Some((a, b, pts)) => (
                    (a + b * size.log2_r).exp().round().max(1.0) as u64,
                    format!(
                        "cross-size transfer: {} is incumbent at {pts} sizes; ln N fitted on log2 r (slope {b:.3}) and extrapolated",
                        inc2.arm.family
                    ),
                ),
                None => (
                    (frac * size.n_a as f64).round().max(1.0) as u64,
                    format!(
                        "cross-size transfer: {} is the k = {k2} incumbent at N / N_a = {frac:.3}",
                        inc2.arm.name
                    ),
                ),
            };
            push(tf.arm(fam, n0), why, &mut out);
            for f in &grid {
                let n = ((n0 as f64) * f).round() as u64;
                push(
                    tf.arm(fam, n),
                    format!(
                        "cross-size transfer neighbourhood: N = {f} x {n0} ({} from k = {k2})",
                        inc2.arm.family
                    ),
                    &mut out,
                );
            }
        }
    }

    // 3. alternatives: best verified trial per other family at this size.
    let mut best: BTreeMap<String, (f64, u64)> = BTreeMap::new();
    for t in ledger
        .trials
        .iter()
        .filter(|t| t.track == track && t.size == k)
    {
        if let Some(s) = t.s {
            let e = best
                .entry(t.arm.family.clone())
                .or_insert((f64::INFINITY, t.arm.n));
            if s[0] < e.0 {
                *e = (s[0], t.arm.n);
            }
        }
    }
    let mut alts: Vec<(String, f64, u64)> = best
        .iter()
        .filter(|(fam, (s0, _))| *fam != &inc.arm.family && *s0 <= 1.5 * inc.s[0])
        .map(|(fam, (s0, n))| (fam.clone(), *s0, *n))
        .collect();
    alts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    for (fam_id, s0, n_best) in alts {
        if let Some(fam) = tf.family(&fam_id) {
            if !fam.enabled {
                continue;
            }
            for f in grid.iter().take(2) {
                let n = ((n_best as f64) * f).round() as u64;
                push(
                    tf.arm(fam, n),
                    format!("alternative {fam_id} (best S(0) {s0:.3} at N = {n_best}): N = {f} x {n_best}"),
                    &mut out,
                );
            }
        }
    }

    // 4. exploration: untried families, least-tried anywhere first.
    let mut untried: Vec<(usize, usize, &Family)> =
        tf.families
            .iter()
            .enumerate()
            .filter(|(_, fam)| fam.enabled)
            .filter(|(_, fam)| {
                // Untried: no verified trial of the family at this size (an
                // exhausted or failed exploration does not close the family;
                // the dedup on (factor base, oracle) keeps the same N from
                // being proposed twice).
                !ledger.trials.iter().any(|t| {
                    t.track == track && t.size == k && t.arm.family == fam.id && t.s.is_some()
                }) && inc.arm.family != fam.id
            })
            .map(|(i, fam)| {
                let anywhere = ledger
                    .trials
                    .iter()
                    .filter(|t| t.track == track && t.arm.family == fam.id)
                    .count();
                (anywhere, i, fam)
            })
            .collect();
    untried.sort_by_key(|(anywhere, i, _)| (*anywhere, *i));
    let mut explored = 0usize;
    for (_, _, fam) in untried {
        if explored >= exploration {
            break;
        }
        let n = ((size.n_a as f64) * fam.default_f).round() as u64;
        let before = out.len();
        push(
            tf.arm(fam, n),
            format!(
                "exploration: family {} untried at k = {k}, N = {} x N_a",
                fam.id, fam.default_f
            ),
            &mut out,
        );
        if out.len() > before {
            explored += 1;
        }
    }

    // Budget: keep every exploration slot, fill the rest in priority order.
    if out.len() > max_candidates {
        let (expl, rest): (Vec<Proposal>, Vec<Proposal>) = out
            .into_iter()
            .partition(|p| p.reason.starts_with("exploration"));
        let keep = max_candidates.saturating_sub(expl.len());
        out = rest.into_iter().take(keep).chain(expl).collect();
    }
    Ok(out)
}

fn cmd_propose(
    home: &Path,
    round: u64,
    track: &str,
    sizes: &[u32],
    max_candidates: usize,
    exploration: usize,
    out: Option<PathBuf>,
) -> Result<Value, String> {
    let ledger = load_ledger(home)?;
    let fams = load_families(home)?;
    let tf = fams
        .tracks
        .get(track)
        .ok_or_else(|| format!("families.json has no track {track}"))?;
    if ledger.rounds.iter().any(|r| r.round == round) {
        return Err(format!("round {round} is already in the ledger"));
    }
    let mut per_size = serde_json::Map::new();
    for &k in sizes {
        let inc = ledger.incumbent(track, k)?;
        let props = propose_size(&ledger, tf, track, k, max_candidates, exploration)?;
        if props.is_empty() {
            return Err(format!("k = {k}: nothing left to propose (every family and neighbour tried); extend families.json"));
        }
        per_size.insert(
            k.to_string(),
            json!({
                "incumbent": inc.arm,
                "incumbent_s": inc.s,
                "candidates": props,
                "tried_before": ledger.tried(track, k).len(),
            }),
        );
    }
    let panel = json!({
        "schema": PANEL_SCHEMA,
        "round": round,
        "track": track,
        "created": now_utc(),
        "rules": {
            "max_candidates": max_candidates,
            "exploration_slots": exploration,
            "n_grid": tf.n_grid,
            "families_enabled": tf.families.iter().filter(|f| f.enabled).map(|f| f.id.clone()).collect::<Vec<_>>(),
        },
        "sizes": per_size,
    });
    let path = out.unwrap_or_else(|| home.join(format!("panel-round-{round:03}.json")));
    write_json(&path, &panel)?;
    println!("panel written: {}", path.display());
    Ok(panel)
}

// ---------------------------------------------------------------------------
// freeze
// ---------------------------------------------------------------------------

fn round_seeds(round: u64, k: u32, exposed: &BTreeSet<u64>) -> (u64, u64) {
    let mut base = SEED_BASE + round * 1000;
    loop {
        let train = base + k as u64;
        let holdout = base + 500 + k as u64;
        if !exposed.contains(&train) && !exposed.contains(&holdout) {
            return (train, holdout);
        }
        base += 100_000;
    }
}

#[allow(clippy::too_many_arguments)]
fn train_spec(
    home: &Path,
    round: u64,
    track: &str,
    size: &SizeInfo,
    inc: &ArmConfig,
    cands: &[ArmConfig],
    target_seed: u64,
    targets: u64,
    rounds: u64,
    timeout: u64,
) -> Value {
    let mut arms = vec![
        json!({"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}),
        json!({"name": "inc", "role": "baseline", "method": inc.method()}),
    ];
    for c in cands {
        arms.push(json!({"name": c.name, "role": "candidate", "method": c.method()}));
    }
    json!({
        "schema": "ecbench.spec/v1",
        "label": format!("ic_autolab round {round} train, {track} track, k = {}", size.k),
        "description": format!("{}/round-{round:03}/PROTOCOL.md (frozen before data), train screen. Public synthetic Curve25519-family miniature and targets; no security claim. In-sample selection; promotion_eligible=false.", home.display()),
        "workloads": {"curves": [size.curve], "targets_per_curve": targets, "target_seed": target_seed, "target_kind": "public"},
        "arms": arms,
        "measurement": {"rounds": rounds, "warmup": 1, "order": if rounds % 2 == 1 { "alternate_measured" } else { "alternate" }, "seed": target_seed + 7, "isolation_required": "L0", "timeout_seconds": timeout}
    })
}

#[allow(clippy::too_many_arguments)]
fn holdout_spec(
    home: &Path,
    round: u64,
    track: &str,
    size: &SizeInfo,
    inc: &ArmConfig,
    sel: &ArmConfig,
    target_seed: u64,
    targets: u64,
    rounds: u64,
    timeout: u64,
) -> Value {
    json!({
        "schema": "ecbench.spec/v1",
        "label": format!("ic_autolab round {round} holdout, {track} track, k = {}", size.k),
        "description": format!("{}/round-{round:03}/PROTOCOL.md (frozen before data), fresh holdout: sel = {} ({} + {}). Public synthetic Curve25519-family miniature and targets; no security claim.", home.display(), sel.name, sel.factor_base, sel.oracle),
        "workloads": {"curves": [size.curve], "targets_per_curve": targets, "target_seed": target_seed, "target_kind": "public"},
        "arms": [
            {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
            {"name": "inc", "role": "baseline", "method": inc.method()},
            {"name": "sel", "role": "candidate", "method": sel.method()},
            {"name": "inc-aa", "role": "control", "method": inc.method()},
            {"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}}
        ],
        "measurement": {"rounds": rounds, "warmup": 1, "order": if rounds % 2 == 1 { "alternate_measured" } else { "alternate" }, "seed": target_seed + 7, "isolation_required": "L0", "timeout_seconds": timeout}
    })
}

struct FreezeOpts {
    train_targets: u64,
    train_rounds: u64,
    holdout_targets: u64,
    holdout_rounds: u64,
    timeout: u64,
    route: String,
    ecbench: Option<PathBuf>,
}

fn protocol_text(
    home: &Path,
    round: u64,
    track: &str,
    ledger: &Ledger,
    panel: &Value,
    seeds: &BTreeMap<u32, (u64, u64)>,
    opts: &FreezeOpts,
    spec_hashes: &BTreeMap<u32, String>,
) -> String {
    let mut t = String::new();
    t += &format!("# ic_autolab round {round}: protocol ({track} track)\n\n");
    t += &format!(
        "Frozen {} before any data of this round. This file is not edited after that; \
         the selection step appends a dated amendment (after train, before holdout), \
         and nothing else is added.\n\n",
        now_utc()
    );
    t += "Loop home: `";
    t += &home.display().to_string();
    t += "` (`ledger.json` names the incumbents; `families.json` the candidate families). \
          Success criterion: AGENTS.md, \"Success criterion: iterative improvement of the \
          index-calculus baseline\"; tracks: AGENTS.md, \"Tracks of index calculus\". \
          The gap to matched rho is reported as the distance remaining, never as a gate.\n\n";
    t += "## Question\n\nDoes any candidate configuration of the ";
    t += track;
    t += " track lower the counted end-to-end cost S (GAE / sqrt r, every phase, every \
          solve verified) below the incumbent on the same fresh public targets of the \
          Curve25519-family miniature at each size below, with the order holding across \
          the AGENTS.md section 5a pricing band (w = 0, 0.1, 1 GAE per unpriced native operation)?\n\n";
    t += "## Incumbents and candidates\n\n";
    t += "Factor base and oracle strings are ecbench `ic.pipeline` parameters. Every \
          candidate is a configuration never tried at its size in this loop (ledger trials).\n\n";
    for (k, (train_seed, holdout_seed)) in seeds {
        let inc = ledger.incumbent(track, *k).expect("checked");
        let size = ledger.size(*k).expect("checked");
        t += &format!(
            "### k = {k} (p = {}, r = {}, log2 r = {:.2}, N_a = {})\n\n",
            size.curve["p"], size.r, size.log2_r, size.n_a
        );
        t += &format!(
            "- `inc` (baseline): `{}` + `{}` (family `{}`, N = {}); S = {} since round {}.\n",
            inc.arm.factor_base,
            inc.arm.oracle,
            inc.arm.family,
            inc.arm.n,
            fmt_s(&inc.s),
            inc.since_round
        );
        t += "- `rho-neg` (reference): `rho.negation`, the matched rho.\n";
        let cands = panel["sizes"][k.to_string()]["candidates"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        for c in &cands {
            t += &format!(
                "- `{}` (candidate): `{}` + `{}`; {}.\n",
                c["arm"]["name"].as_str().unwrap_or(""),
                c["arm"]["factor_base"].as_str().unwrap_or(""),
                c["arm"]["oracle"].as_str().unwrap_or(""),
                c["reason"].as_str().unwrap_or("")
            );
        }
        t += &format!(
            "- Train spec `specs/train-k{k}.json` (sha256 `{}`): `target_seed` {train_seed}, \
             {} public targets x {} measured round(s), 1 warm-up, timeout {} s per run. \
             Holdout `target_seed` {holdout_seed} (reserved now, spec written at selection).\n\n",
            spec_hashes.get(k).cloned().unwrap_or_default(),
            opts.train_targets,
            opts.train_rounds,
            opts.timeout
        );
    }
    t += "## Train (development screen)\n\n";
    t += &format!(
        "Per size: `rho-neg`, `inc` and every candidate on {} fresh public targets, {} measured \
         round(s) (an odd count cannot be balanced under either alternating order; ecbench warns, \
         accepted). Audit: `ecbench verify --replay` on every session; a failed audit voids the \
         session. The screen is diagnostic and in sample: `promotion_eligible=false`. A timed-out, \
         failed or unverified arm is reported and is not selectable.\n\n",
        opts.train_targets, opts.train_rounds
    );
    t += "## Selection (disclosed in-sample rule)\n\n";
    t += "Per size, the challenger `sel` is the candidate with the lowest mean S at w = 0 among arms \
          whose every run verified. The best candidate at w = 1 is recorded as the Pareto alternative \
          for the next round's panel. If no candidate has mean S at w = 0 below `inc` on the train \
          targets, the size has no challenger and ends the round as `no_challenger` (no holdout is run).\n\n";
    t += "## Holdout (confirmation)\n\n";
    t += &format!(
        "Per size with a challenger: `rho-neg`, `inc`, `sel`, `inc-aa` (A/A copy of `inc`) and \
         `rho-neg-aa`, on {} fresh public targets x {} measured rounds, 1 warm-up, order alternate, \
         the reserved holdout `target_seed`. Audit as in train.\n\n",
        opts.holdout_targets, opts.holdout_rounds
    );
    t += "## Decision rule\n\n";
    t += "Per size, from the holdout session:\n\n\
          - **improvement**: `ecbench compare --a inc --b sel` (paired, linearised-t interval over \
            workloads) has `ops.status = ok`, is admissible, and its 95% interval lies below 1 at w = 0; \
            AND `ecbench band --ranks` orders `sel < inc` on that curve (the order holds at w = 0, 0.1 \
            and 1 alike). A/A: `inc-aa / inc` must contain 1 (deterministic counts give exactly 1).\n\
          - **regression**: the interval lies above 1 at w = 0, or the band orders `inc < sel`.\n\
          - **null**: anything else (interval spans 1, or the band is unresolved).\n\
          - Relabelling (a headline count falls while total cost rises) is caught by the band; \
            accounting changes are not improvements. Distance to rho is `mean S(sel) / mean S(rho-neg)` \
            at each weight, reported as the distance remaining.\n\n";
    t += "## Ratchet\n\n";
    t += "An improvement makes `sel` the incumbent of its track and size in `ledger.json`, with the \
          paired ratio and interval as its delta, the session id, spec sha256 and audit receipt sha256 \
          as its evidence, and `since_round` = this round. Every train and holdout arm joins the trial \
          history (so it is never proposed again at that size); every target seed joins the exposures. \
          A null or regression leaves the incumbent in place and is recorded as such.\n\n";
    t += "## Route and scope\n\n";
    t += &format!(
        "Route: `{}`. Counted GAE, deterministic and host-independent; no wall-time claim. \
         Isolation level earned is recorded per session and does not change the counts. This is a \
         development loop: no round reaches the IC leaderboard or `benchmarks/README.md` without the \
         validation gate (replay certificate on a second host class, validator report, matched rho \
         with the band; `docs/VALIDATION_GATE.md`).\n\n",
        opts.route
    );
    t += &format!(
        "Exposures excluded: {} target seeds already used on this family (ledger exposures); the \
         seeds above are new.\n",
        ledger.exposures.len()
    );
    t
}

fn cmd_freeze(home: &Path, round: u64, panel_path: &Path, opts: FreezeOpts) -> Result<(), String> {
    let mut ledger = load_ledger(home)?;
    let panel = read_json(panel_path)?;
    if panel["schema"] != PANEL_SCHEMA || panel["round"].as_u64() != Some(round) {
        return Err(format!(
            "{} is not a panel for round {round}",
            panel_path.display()
        ));
    }
    let track = panel["track"].as_str().ok_or("panel.track")?.to_string();
    let dir = round_dir(home, round);
    if dir.exists() {
        return Err(format!("{} exists; a round is frozen once", dir.display()));
    }
    if ledger.rounds.iter().any(|r| r.round == round) {
        return Err(format!("round {round} is already in the ledger"));
    }
    let exposed = ledger.exposed_seeds();
    let mut sizes: Vec<u32> = panel["sizes"]
        .as_object()
        .ok_or("panel.sizes")?
        .keys()
        .map(|k| k.parse::<u32>().map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;
    sizes.sort();
    let mut seeds = BTreeMap::new();
    let mut specs: BTreeMap<u32, Value> = BTreeMap::new();
    for &k in &sizes {
        let (ts, hs) = round_seeds(round, k, &exposed);
        seeds.insert(k, (ts, hs));
        let inc: ArmConfig =
            serde_json::from_value(panel["sizes"][k.to_string()]["incumbent"].clone())
                .map_err(|e| format!("panel incumbent k={k}: {e}"))?;
        let cands: Vec<ArmConfig> = panel["sizes"][k.to_string()]["candidates"]
            .as_array()
            .ok_or("panel candidates")?
            .iter()
            .map(|c| serde_json::from_value(c["arm"].clone()).map_err(|e| e.to_string()))
            .collect::<Result<_, _>>()?;
        if inc != ledger.incumbent(&track, k)?.arm {
            return Err(format!(
                "panel incumbent at k = {k} differs from the ledger; re-propose"
            ));
        }
        let size = ledger.size(k)?;
        specs.insert(
            k,
            train_spec(
                home,
                round,
                &track,
                size,
                &inc,
                &cands,
                ts,
                opts.train_targets,
                opts.train_rounds,
                opts.timeout,
            ),
        );
    }
    // Validate every spec with ecbench plan before writing the round.
    let tmp = home.join(format!(".freeze-{round:03}.tmp"));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).map_err(|e| e.to_string())?;
    if let Some(ecbench) = &opts.ecbench {
        for (k, spec) in &specs {
            let p = tmp.join(format!("train-k{k}.json"));
            write_json(&p, spec)?;
            let (code, out) = run_logged(
                &[
                    ecbench.display().to_string(),
                    "plan".into(),
                    "--spec".into(),
                    p.display().to_string(),
                ],
                None,
            )?;
            if code != 0 {
                let _ = std::fs::remove_dir_all(&tmp);
                return Err(format!(
                    "ecbench plan rejected train-k{k}.json (exit {code}):\n{out}"
                ));
            }
        }
    }
    let _ = std::fs::remove_dir_all(&tmp);
    // Write the round.
    let mut hashes = BTreeMap::new();
    for (k, spec) in &specs {
        let p = dir.join("specs").join(format!("train-k{k}.json"));
        write_new(
            &p,
            &(serde_json::to_string_pretty(spec).map_err(|e| e.to_string())? + "\n"),
        )?;
        hashes.insert(*k, sha256_file(&p)?);
    }
    write_new(
        &dir.join("panel.json"),
        &(serde_json::to_string_pretty(&panel).map_err(|e| e.to_string())? + "\n"),
    )?;
    let protocol = protocol_text(home, round, &track, &ledger, &panel, &seeds, &opts, &hashes);
    write_new(&dir.join("PROTOCOL.md"), &protocol)?;
    let manifest = json!({
        "schema": "ic_autolab.manifest/v1",
        "round": round,
        "track": track,
        "frozen": now_utc(),
        "git_head": git_head(),
        "route": opts.route,
        "sizes": sizes,
        "seeds": seeds.iter().map(|(k, (t, h))| (k.to_string(), json!({"train": t, "holdout": h}))).collect::<serde_json::Map<_, _>>(),
        "train": {"targets": opts.train_targets, "rounds": opts.train_rounds},
        "holdout": {"targets": opts.holdout_targets, "rounds": opts.holdout_rounds},
        "timeout_seconds": opts.timeout,
        "spec_sha256": hashes.iter().map(|(k, h)| (format!("train-k{k}"), json!(h))).collect::<serde_json::Map<_, _>>(),
        "protocol_sha256": sha256_file(&dir.join("PROTOCOL.md"))?,
        "panel_sha256": sha256_file(&dir.join("panel.json"))?,
    });
    write_new(
        &dir.join("MANIFEST.json"),
        &(serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())? + "\n"),
    )?;
    ledger.rounds.push(RoundEntry {
        round,
        dir: format!("round-{round:03}"),
        track,
        sizes,
        status: "frozen".into(),
    });
    save_ledger(home, &mut ledger)?;
    println!("round {round} frozen: {}", dir.display());
    Ok(())
}

// ---------------------------------------------------------------------------
// run (local route) and card (relay route)
// ---------------------------------------------------------------------------

struct RunOpts {
    ecbench: PathBuf,
    cpus: String,
    allow_busy: bool,
    replay: usize,
}

fn stage_specs(dir: &Path, stage: &str) -> Result<Vec<(u32, PathBuf)>, String> {
    let mut v = Vec::new();
    for entry in std::fs::read_dir(dir.join("specs")).map_err(|e| format!("specs: {e}"))? {
        let p = entry.map_err(|e| e.to_string())?.path();
        let name = p
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        if let Some(rest) = name.strip_prefix(&format!("{stage}-k")) {
            if let Some(k) = rest.strip_suffix(".json") {
                v.push((k.parse::<u32>().map_err(|e| e.to_string())?, p));
            }
        }
    }
    v.sort();
    if v.is_empty() {
        return Err(format!(
            "no {stage} specs in {}",
            dir.join("specs").display()
        ));
    }
    Ok(v)
}

fn run_argv(opts: &RunOpts, spec: &Path, out: &Path) -> Vec<String> {
    let mut a = vec![
        opts.ecbench.display().to_string(),
        "run".into(),
        "--wait".into(),
        "--spec".into(),
        spec.display().to_string(),
        "--out".into(),
        out.display().to_string(),
        "--cpus".into(),
        opts.cpus.clone(),
    ];
    if opts.allow_busy {
        a.push("--allow-busy".into());
    }
    a
}

fn verify_argv(opts: &RunOpts, out: &Path, receipt: &Path) -> Vec<String> {
    vec![
        opts.ecbench.display().to_string(),
        "verify".into(),
        "--dir".into(),
        out.display().to_string(),
        "--replay".into(),
        opts.replay.to_string(),
        "--out".into(),
        receipt.display().to_string(),
    ]
}

fn band_argv(ecbench: &Path, out: &Path, json: &Path) -> Vec<String> {
    vec![
        ecbench.display().to_string(),
        "band".into(),
        "--dir".into(),
        out.display().to_string(),
        "--json".into(),
        json.display().to_string(),
        "--ranks".into(),
    ]
}

fn cmd_run(home: &Path, round: u64, stage: &str, opts: &RunOpts) -> Result<Value, String> {
    let dir = round_dir(home, round);
    if !dir.join("MANIFEST.json").exists() {
        return Err(format!("round {round} is not frozen"));
    }
    let specs = stage_specs(&dir, stage)?;
    let record_path = dir.join("analysis").join(format!("run-{stage}.json"));
    if record_path.exists() {
        return Err(format!(
            "{} exists; a stage runs once",
            record_path.display()
        ));
    }
    for sub in ["analysis", "logs", "sessions"] {
        std::fs::create_dir_all(dir.join(sub)).map_err(|e| format!("mkdir {sub}: {e}"))?;
    }
    let mut rows = Vec::new();
    for (k, spec) in &specs {
        let out = dir.join("sessions").join(format!("{stage}-k{k}"));
        let log = dir.join("logs").join(format!("{stage}-k{k}.run.log"));
        // A session directory is never rerun. One that exists (a stage
        // interrupted after `ecbench run`) is adopted as it is: only the
        // audit and the band, which read it, are redone.
        let adopted = out.exists();
        let run_code = if adopted {
            eprintln!("adopting existing session {}", out.display());
            if out.join("session.json").exists() {
                0
            } else {
                -1
            }
        } else {
            run_logged(&run_argv(opts, spec, &out), Some(&log))?.0
        };
        let receipt = dir
            .join("analysis")
            .join(format!("verify-{stage}-k{k}.json"));
        let mut verify_code = None;
        let mut band_code = None;
        let band_json = dir.join("analysis").join(format!("band-{stage}-k{k}.json"));
        if out.join("session.json").exists() {
            let (c, _) = run_logged(
                &verify_argv(opts, &out, &receipt),
                Some(&dir.join("logs").join(format!("{stage}-k{k}.verify.log"))),
            )?;
            verify_code = Some(c);
            let (c, text) = run_logged(&band_argv(&opts.ecbench, &out, &band_json), None)?;
            band_code = Some(c);
            std::fs::write(
                dir.join("analysis").join(format!("band-{stage}-k{k}.txt")),
                text,
            )
            .map_err(|e| e.to_string())?;
        }
        let session_id = read_json(&out.join("session.json"))
            .ok()
            .and_then(|s| s["session_id"].as_str().map(String::from));
        rows.push(json!({
            "size": k,
            "spec": spec.display().to_string(),
            "spec_sha256": sha256_file(spec)?,
            "session_dir": out.display().to_string(),
            "session_id": session_id,
            "run_exit": run_code,
            "adopted_existing_session": adopted,
            "verify_exit": verify_code,
            "verify_receipt": receipt.display().to_string(),
            "verify_receipt_sha256": sha256_file(&receipt).ok(),
            "band_exit": band_code,
            "band": band_json.display().to_string(),
            "ok": run_code == 0 && verify_code == Some(0) && band_code == Some(0),
        }));
    }
    let record = json!({
        "schema": "ic_autolab.run/v1",
        "round": round,
        "stage": stage,
        "route": "local",
        "ecbench": opts.ecbench.display().to_string(),
        "ecbench_sha256": sha256_file(&opts.ecbench)?,
        "cpus": opts.cpus,
        "allow_busy": opts.allow_busy,
        "replay": opts.replay,
        "finished": now_utc(),
        "sessions": rows,
    });
    write_json(&record_path, &record)?;
    let bad: Vec<_> = record["sessions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["ok"] != true)
        .map(|r| r["size"].clone())
        .collect();
    if bad.is_empty() {
        println!("{stage}: every session ran, verified and banded");
    } else {
        println!("{stage}: sessions with a failure (recorded, not retried): sizes {bad:?}");
    }
    Ok(record)
}

/// A YAML flow sequence of double-quoted strings.
fn yaml_argv(argv: &[String]) -> String {
    let quoted: Vec<String> = argv
        .iter()
        .map(|a| format!("\"{}\"", a.replace('"', "\\\"")))
        .collect();
    format!("[{}]", quoted.join(", "))
}

fn cmd_card(
    home: &Path,
    round: u64,
    stage: &str,
    opts: &RunOpts,
    n: u64,
) -> Result<PathBuf, String> {
    let dir = round_dir(home, round);
    let manifest = read_json(&dir.join("MANIFEST.json"))?;
    let specs = stage_specs(&dir, stage)?;
    let head = git_head().ok_or("git HEAD unavailable")?;
    let experiment = home
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("home has no name")?
        .to_string();
    let card_id = format!(
        "CARD-{}-{}-{n}",
        experiment.replace('_', "-"),
        date_compact()
    );
    let cards = dir.join("cards");
    let card_path = cards.join(format!("{card_id}.yaml"));
    if card_path.exists() {
        return Err(format!(
            "{} exists; a card is write-once (use --n)",
            card_path.display()
        ));
    }
    let timeout = manifest["timeout_seconds"].as_u64().unwrap_or(1800);
    let mut binds = vec![
        (
            dir.join("PROTOCOL.md"),
            sha256_file(&dir.join("PROTOCOL.md"))?,
        ),
        (opts.ecbench.clone(), sha256_file(&opts.ecbench)?),
    ];
    let mut must_not_exist = Vec::new();
    let mut commands = String::new();
    let mut total_timeout = 0u64;
    for (k, spec) in &specs {
        binds.push((spec.clone(), sha256_file(spec)?));
        let out = dir.join("sessions").join(format!("{stage}-k{k}"));
        must_not_exist.push(out.display().to_string());
        let spec_v = read_json(spec)?;
        let execs = spec_v["workloads"]["targets_per_curve"]
            .as_u64()
            .unwrap_or(16)
            * spec_v["arms"]
                .as_array()
                .map(|a| a.len() as u64)
                .unwrap_or(2)
            * (spec_v["measurement"]["rounds"].as_u64().unwrap_or(1)
                + spec_v["measurement"]["warmup"].as_u64().unwrap_or(1));
        let run_timeout = execs * timeout + 600;
        total_timeout += run_timeout;
        let receipt = dir
            .join("analysis")
            .join(format!("verify-{stage}-k{k}.json"));
        let band_json = dir.join("analysis").join(format!("band-{stage}-k{k}.json"));
        commands += &format!(
            "  - id: run-{stage}-k{k}\n    argv: {}\n    timeout_seconds: {run_timeout}\n    success_exit_codes: [0]\n    stdout: {}\n    stderr: {}\n    outputs: [{}]\n",
            yaml_argv(&run_argv(opts, spec, &out)),
            cards.join(format!("{card_id}.run-{stage}-k{k}.stdout")).display(),
            cards.join(format!("{card_id}.run-{stage}-k{k}.stderr")).display(),
            out.join("session.json").display()
        );
        commands += &format!(
            "  - id: verify-{stage}-k{k}\n    argv: {}\n    timeout_seconds: {run_timeout}\n    success_exit_codes: [0]\n    stdout: {}\n    stderr: {}\n    outputs: [{}]\n",
            yaml_argv(&verify_argv(opts, &out, &receipt)),
            cards.join(format!("{card_id}.verify-{stage}-k{k}.stdout")).display(),
            cards.join(format!("{card_id}.verify-{stage}-k{k}.stderr")).display(),
            receipt.display()
        );
        commands += &format!(
            "  - id: band-{stage}-k{k}\n    argv: {}\n    timeout_seconds: 600\n    success_exit_codes: [0]\n    stdout: {}\n    stderr: {}\n    outputs: [{}]\n",
            yaml_argv(&band_argv(&opts.ecbench, &out, &band_json)),
            dir.join("analysis").join(format!("band-{stage}-k{k}.txt")).display(),
            cards.join(format!("{card_id}.band-{stage}-k{k}.stderr")).display(),
            band_json.display()
        );
    }
    let binds_yaml: String = binds
        .iter()
        .map(|(p, h)| format!("  - {{path: {}, sha256: {h}}}\n", p.display()))
        .collect();
    let card = format!(
        "schema: crypto.autoresearch.mechanical-task-card.v1\n\
card_id: {card_id}\n\
experiment_id: {experiment}\n\
author_role: experiment-director\n\
executor_role: executor-mechanical\n\
kind: frozen_command\n\
commit: {head}\n\
receipt: {}\n\
binds:\n{binds_yaml}\
must_not_exist: {}\n\
commands:\n{commands}\
bus_address: executor-mechanical-{}\n\
background: {}\n\
write_scope: [{}]\n\
stop_when:\n\
  - Any command exits nonzero or reaches its timeout; retain the session directory and the logs as evidence.\n",
        cards.join(format!("{card_id}.receipt.json")).display(),
        yaml_argv(&must_not_exist),
        experiment.replace('_', "-"),
        total_timeout > 600,
        dir.display()
    );
    write_new(&card_path, &card)?;
    println!("card written: {}", card_path.display());
    Ok(card_path)
}

// ---------------------------------------------------------------------------
// select
// ---------------------------------------------------------------------------

/// Mean S per arm (w = 0, 0.1, 1) and run/verified counts, from a band file.
fn band_rows(band: &Value) -> BTreeMap<String, ([f64; 3], u64, u64)> {
    let mut m = BTreeMap::new();
    for row in band["rows"].as_array().cloned().unwrap_or_default() {
        if let (Some(arm), Some(s)) = (row["arm"].as_str(), s3(&row["mean_s"])) {
            m.insert(
                arm.to_string(),
                (
                    s,
                    row["runs"].as_u64().unwrap_or(0),
                    row["verified"].as_u64().unwrap_or(0),
                ),
            );
        }
    }
    m
}

fn band_verdict(band: &Value, a: &str, b: &str) -> Option<String> {
    band["ranks"].as_array()?.iter().find_map(|r| {
        let (ra, rb) = (r["a"].as_str()?, r["b"].as_str()?);
        if (ra == a && rb == b) || (ra == b && rb == a) {
            r["verdict"].as_str().map(String::from)
        } else {
            None
        }
    })
}

/// The selection rule of the protocol, as a pure function of the train band.
fn select_from_band(
    band: &Value,
    cands: &[ArmConfig],
) -> (
    Option<ArmConfig>,
    Option<ArmConfig>,
    Vec<Value>,
    Option<[f64; 3]>,
) {
    let rows = band_rows(band);
    let inc_s = rows.get("inc").map(|r| r.0);
    let mut table = Vec::new();
    let mut best0: Option<(f64, ArmConfig)> = None;
    let mut best2: Option<(f64, ArmConfig)> = None;
    for c in cands {
        let status;
        let s;
        match rows.get(&c.name) {
            Some((sv, runs, verified)) if runs == verified && *runs > 0 => {
                status = "verified";
                s = Some(*sv);
                if best0.as_ref().map(|b| sv[0] < b.0).unwrap_or(true) {
                    best0 = Some((sv[0], c.clone()));
                }
                if best2.as_ref().map(|b| sv[2] < b.0).unwrap_or(true) {
                    best2 = Some((sv[2], c.clone()));
                }
            }
            Some((sv, runs, verified)) => {
                status = "not every run verified";
                s = Some(*sv);
                let _ = (runs, verified);
            }
            None => {
                status = "no verified runs";
                s = None;
            }
        }
        table.push(json!({"arm": c, "s": s, "status": status}));
    }
    let sel = match (&best0, inc_s) {
        (Some((s0, arm)), Some(inc)) if *s0 < inc[0] => Some(arm.clone()),
        _ => None,
    };
    (sel, best2.map(|b| b.1), table, inc_s)
}

fn cmd_select(home: &Path, round: u64, ecbench: Option<PathBuf>) -> Result<Value, String> {
    let ledger = load_ledger(home)?;
    let dir = round_dir(home, round);
    let manifest = read_json(&dir.join("MANIFEST.json"))?;
    let panel = read_json(&dir.join("panel.json"))?;
    let track = manifest["track"]
        .as_str()
        .ok_or("manifest.track")?
        .to_string();
    let sel_path = dir.join("selection.json");
    if sel_path.exists() {
        return Err(format!(
            "{} exists; selection happens once",
            sel_path.display()
        ));
    }
    let run = read_json(&dir.join("analysis").join("run-train.json"))
        .map_err(|e| format!("train has not run: {e}"))?;
    let mut per_size = serde_json::Map::new();
    let mut amendment = format!(
        "\n## Amendment: selection ({}, after train, before holdout)\n\n",
        now_utc()
    );
    let mut holdout_specs: Vec<(u32, Value)> = Vec::new();
    for row in run["sessions"].as_array().ok_or("run.sessions")? {
        let k = row["size"].as_u64().ok_or("size")? as u32;
        let cands: Vec<ArmConfig> = panel["sizes"][k.to_string()]["candidates"]
            .as_array()
            .ok_or("panel candidates")?
            .iter()
            .map(|c| serde_json::from_value(c["arm"].clone()).map_err(|e| e.to_string()))
            .collect::<Result<_, _>>()?;
        if row["ok"] != true {
            per_size.insert(k.to_string(), json!({"status": "train_failed", "run": row}));
            amendment += &format!("- k = {k}: the train session did not run, verify and band cleanly (see `analysis/run-train.json`); no challenger.\n");
            continue;
        }
        let band = read_json(Path::new(row["band"].as_str().ok_or("band")?))?;
        let (sel, alt_w1, table, inc_s) = select_from_band(&band, &cands);
        let rho_s = band_rows(&band).get("rho-neg").map(|r| r.0);
        match &sel {
            Some(arm) => {
                let inc = ledger.incumbent(&track, k)?;
                let (_, hs) = {
                    let s = &manifest["seeds"][k.to_string()];
                    (
                        s["train"].as_u64().unwrap_or(0),
                        s["holdout"].as_u64().ok_or("holdout seed")?,
                    )
                };
                let size = ledger.size(k)?;
                let spec = holdout_spec(
                    home,
                    round,
                    &track,
                    size,
                    &inc.arm,
                    arm,
                    hs,
                    manifest["holdout"]["targets"].as_u64().unwrap_or(32),
                    manifest["holdout"]["rounds"].as_u64().unwrap_or(2),
                    manifest["timeout_seconds"].as_u64().unwrap_or(1800),
                );
                holdout_specs.push((k, spec));
                amendment += &format!(
                    "- k = {k}: selected `{}` (`{}` + `{}`), train mean S = {} against `inc` {}; Pareto alternative at w = 1: `{}`. Holdout spec `specs/holdout-k{k}.json`, `target_seed` {hs}.\n",
                    arm.name,
                    arm.factor_base,
                    arm.oracle,
                    table.iter().find(|t| t["arm"]["name"] == arm.name).and_then(|t| s3(&t["s"])).map(|s| fmt_s(&s)).unwrap_or_default(),
                    inc_s.map(|s| fmt_s(&s)).unwrap_or_default(),
                    alt_w1.as_ref().map(|a| a.name.clone()).unwrap_or_default()
                );
            }
            None => {
                amendment += &format!(
                    "- k = {k}: no candidate below `inc` ({}) at w = 0 on the train targets; no challenger, no holdout.\n",
                    inc_s.map(|s| fmt_s(&s)).unwrap_or_default()
                );
            }
        }
        per_size.insert(
            k.to_string(),
            json!({
                "status": if sel.is_some() { "challenger" } else { "no_challenger" },
                "sel": sel,
                "alt_w1": alt_w1,
                "inc_s": inc_s,
                "rho_s": rho_s,
                "candidates": table,
                "session_id": row["session_id"],
            }),
        );
    }
    // Validate holdout specs before writing anything.
    if let Some(ecbench) = &ecbench {
        let tmp = dir.join(".select.tmp");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).map_err(|e| e.to_string())?;
        for (k, spec) in &holdout_specs {
            let p = tmp.join(format!("holdout-k{k}.json"));
            write_json(&p, spec)?;
            let (code, out) = run_logged(
                &[
                    ecbench.display().to_string(),
                    "plan".into(),
                    "--spec".into(),
                    p.display().to_string(),
                ],
                None,
            )?;
            if code != 0 {
                let _ = std::fs::remove_dir_all(&tmp);
                return Err(format!(
                    "ecbench plan rejected holdout-k{k}.json (exit {code}):\n{out}"
                ));
            }
        }
        let _ = std::fs::remove_dir_all(&tmp);
    }
    let mut hashes = serde_json::Map::new();
    for (k, spec) in &holdout_specs {
        let p = dir.join("specs").join(format!("holdout-k{k}.json"));
        write_new(
            &p,
            &(serde_json::to_string_pretty(spec).map_err(|e| e.to_string())? + "\n"),
        )?;
        hashes.insert(format!("holdout-k{k}"), json!(sha256_file(&p)?));
    }
    amendment += &format!(
        "\nHoldout spec sha256: {}\n",
        serde_json::to_string(&hashes).unwrap_or_default()
    );
    let mut protocol =
        std::fs::read_to_string(dir.join("PROTOCOL.md")).map_err(|e| e.to_string())?;
    protocol += &amendment;
    std::fs::write(dir.join("PROTOCOL.md"), protocol).map_err(|e| e.to_string())?;
    let selection = json!({
        "schema": "ic_autolab.selection/v1",
        "round": round,
        "track": track,
        "selected": now_utc(),
        "rule": "lowest mean S at w = 0 among candidates whose every run verified; a challenger only if below inc at w = 0",
        "sizes": per_size,
        "holdout_spec_sha256": hashes,
    });
    write_json(&sel_path, &selection)?;
    println!("selection written: {}", sel_path.display());
    Ok(selection)
}

// ---------------------------------------------------------------------------
// assess
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Class {
    Improvement,
    Regression,
    Null,
    NoChallenger,
    Inadmissible,
}

/// The decision rule, as a pure function of the compare and band outputs.
fn classify(compare_w0: &Value, band: &Value, aa: Option<&Value>) -> (Class, Vec<String>) {
    let mut why = Vec::new();
    let ops = &compare_w0["ops"];
    let status = ops["status"].as_str().unwrap_or("");
    let admissible = compare_w0["admissible"].as_bool().unwrap_or(false);
    let ratio = ops["ratio_b_over_a"].as_f64();
    let ci = ops["ci95"]
        .as_array()
        .and_then(|a| Some([a.first()?.as_f64()?, a.get(1)?.as_f64()?]));
    let verdict = band_verdict(band, "inc", "sel").unwrap_or_else(|| "missing".into());
    why.push(format!(
        "compare ops.status = {status}, admissible = {admissible}"
    ));
    if let (Some(r), Some(c)) = (ratio, ci) {
        why.push(format!(
            "sel / inc = {r:.4} [{:.3}, {:.3}] at w = 0",
            c[0], c[1]
        ));
    }
    why.push(format!("band: {verdict}"));
    if let Some(aa) = aa {
        if let Some(c) = aa["ops"]["ci95"]
            .as_array()
            .and_then(|a| Some([a.first()?.as_f64()?, a.get(1)?.as_f64()?]))
        {
            let ok = c[0] <= 1.0 && 1.0 <= c[1];
            why.push(format!(
                "A/A inc-aa / inc interval [{:.3}, {:.3}] {} 1",
                c[0],
                c[1],
                if ok { "contains" } else { "excludes" }
            ));
            if !ok {
                return (Class::Inadmissible, why);
            }
        }
    }
    if status != "ok" || !admissible {
        return (Class::Inadmissible, why);
    }
    let Some(ci) = ci else {
        return (Class::Inadmissible, why);
    };
    if ci[1] < 1.0 && verdict == "sel < inc" {
        (Class::Improvement, why)
    } else if ci[0] > 1.0 || verdict == "inc < sel" {
        (Class::Regression, why)
    } else {
        (Class::Null, why)
    }
}

fn compare(
    ecbench: &Path,
    dir: &Path,
    a: &str,
    b: &str,
    weight: Option<f64>,
) -> Result<Value, String> {
    let mut argv = vec![
        ecbench.display().to_string(),
        "compare".into(),
        "--dir".into(),
        dir.display().to_string(),
        "--a".into(),
        a.into(),
        "--b".into(),
        b.into(),
        "--save".into(),
        "--json".into(),
    ];
    if let Some(w) = weight {
        argv.push("--weight".into());
        argv.push(w.to_string());
    }
    let (code, out) = run_logged(&argv, None)?;
    if code != 0 {
        return Err(format!("ecbench compare {a} {b} exit {code}:\n{out}"));
    }
    serde_json::from_str(&out).map_err(|e| format!("compare json: {e}"))
}

fn cmd_assess(home: &Path, round: u64, ecbench: &Path) -> Result<Value, String> {
    let ledger = load_ledger(home)?;
    let dir = round_dir(home, round);
    let manifest = read_json(&dir.join("MANIFEST.json"))?;
    let track = manifest["track"]
        .as_str()
        .ok_or("manifest.track")?
        .to_string();
    let result_path = dir.join("RESULT.json");
    if result_path.exists() {
        return Err(format!(
            "{} exists; a round is assessed once",
            result_path.display()
        ));
    }
    let selection =
        read_json(&dir.join("selection.json")).map_err(|e| format!("select first: {e}"))?;
    let run_holdout = read_json(&dir.join("analysis").join("run-holdout.json")).ok();
    let mut sizes = serde_json::Map::new();
    for (k_s, sel_row) in selection["sizes"].as_object().ok_or("selection.sizes")? {
        let k: u32 = k_s.parse().map_err(|e| format!("{e}"))?;
        let inc = ledger.incumbent(&track, k)?;
        if sel_row["status"] != "challenger" {
            sizes.insert(
                k_s.clone(),
                json!({"class": Class::NoChallenger, "why": [sel_row["status"]], "incumbent": inc.arm, "incumbent_s": inc.s}),
            );
            continue;
        }
        let Some(run) = run_holdout
            .as_ref()
            .and_then(|r| r["sessions"].as_array())
            .and_then(|rows| {
                rows.iter()
                    .find(|r| r["size"].as_u64() == Some(k as u64))
                    .cloned()
            })
        else {
            return Err(format!("k = {k}: holdout has not run"));
        };
        let sel: ArmConfig =
            serde_json::from_value(sel_row["sel"].clone()).map_err(|e| e.to_string())?;
        if run["ok"] != true {
            sizes.insert(
                k_s.clone(),
                json!({"class": Class::Inadmissible, "why": ["holdout session did not run, verify and band cleanly"], "run": run, "sel": sel, "incumbent": inc.arm}),
            );
            continue;
        }
        let sdir = PathBuf::from(run["session_dir"].as_str().ok_or("session_dir")?);
        let c0 = compare(ecbench, &sdir, "inc", "sel", None)?;
        let c01 = compare(ecbench, &sdir, "inc", "sel", Some(0.1))?;
        let c1 = compare(ecbench, &sdir, "inc", "sel", Some(1.0))?;
        let aa = compare(ecbench, &sdir, "inc", "inc-aa", None)?;
        let band = read_json(Path::new(run["band"].as_str().ok_or("band")?))?;
        let (class, why) = classify(&c0, &band, Some(&aa));
        let rows = band_rows(&band);
        let s_sel = rows.get("sel").map(|r| r.0);
        let s_inc = rows.get("inc").map(|r| r.0);
        let s_rho = rows.get("rho-neg").map(|r| r.0);
        let session = read_json(&sdir.join("session.json"))?;
        sizes.insert(
            k_s.clone(),
            json!({
                "class": class,
                "why": why,
                "sel": sel,
                "incumbent": inc.arm,
                "ratio": {
                    "w0": {"ratio": c0["ops"]["ratio_b_over_a"], "ci95": c0["ops"]["ci95"], "status": c0["ops"]["status"], "admissible": c0["admissible"]},
                    "w0.1": {"ratio": c01["ops"]["ratio_b_over_a"], "ci95": c01["ops"]["ci95"]},
                    "w1": {"ratio": c1["ops"]["ratio_b_over_a"], "ci95": c1["ops"]["ci95"]},
                    "aa": {"ratio": aa["ops"]["ratio_b_over_a"], "ci95": aa["ops"]["ci95"]},
                },
                "band_verdict": band_verdict(&band, "inc", "sel"),
                "band_rho": band_verdict(&band, "rho-neg", "sel"),
                "s": {"sel": s_sel, "inc": s_inc, "rho_neg": s_rho},
                "distance_to_rho": {
                    "sel": s_sel.zip(s_rho).map(|(a, b)| distance(&a, &b)),
                    "inc": s_inc.zip(s_rho).map(|(a, b)| distance(&a, &b)),
                },
                "evidence": {
                    "session_id": session["session_id"],
                    "session_dir": run["session_dir"],
                    "spec_sha256": run["spec_sha256"],
                    "binary_sha256": session["binary_sha256"],
                    "verify_receipt": run["verify_receipt"],
                    "verify_receipt_sha256": run["verify_receipt_sha256"],
                    "verify_exit": run["verify_exit"],
                    "compare_verdict": c0["verdict"],
                    "levels": session["status_counts"],
                },
            }),
        );
    }
    let result = json!({
        "schema": RESULT_SCHEMA,
        "round": round,
        "track": track,
        "assessed": now_utc(),
        "rule": "improvement = paired sel/inc 95% interval below 1 at w = 0 and band order sel < inc at every weight; regression = interval above 1 or band inc < sel; else null",
        "sizes": sizes,
    });
    write_json(&result_path, &result)?;
    for (k, v) in result["sizes"].as_object().unwrap() {
        println!("k = {k}: {}", v["class"]);
    }
    Ok(result)
}

// ---------------------------------------------------------------------------
// ratchet
// ---------------------------------------------------------------------------

fn cmd_ratchet(home: &Path, round: u64) -> Result<(), String> {
    let mut ledger = load_ledger(home)?;
    let dir = round_dir(home, round);
    let manifest = read_json(&dir.join("MANIFEST.json"))?;
    let result = read_json(&dir.join("RESULT.json")).map_err(|e| format!("assess first: {e}"))?;
    let selection = read_json(&dir.join("selection.json"))?;
    let panel = read_json(&dir.join("panel.json"))?;
    let track = manifest["track"]
        .as_str()
        .ok_or("manifest.track")?
        .to_string();
    let entry = ledger
        .rounds
        .iter()
        .find(|r| r.round == round)
        .ok_or_else(|| format!("round {round} is not in the ledger"))?;
    if entry.status == "ratcheted" {
        return Err(format!("round {round} is already ratcheted"));
    }
    let run_train = read_json(&dir.join("analysis").join("run-train.json"))?;
    let source = format!("round-{round:03}");
    // Trials: every train arm with its band S, and the holdout sel.
    for row in run_train["sessions"].as_array().ok_or("run.sessions")? {
        let k = row["size"].as_u64().ok_or("size")? as u32;
        let cands = panel["sizes"][k.to_string()]["candidates"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        let band = row["band"]
            .as_str()
            .and_then(|p| read_json(Path::new(p)).ok());
        let rows = band.as_ref().map(band_rows).unwrap_or_default();
        for c in cands {
            let arm: ArmConfig =
                serde_json::from_value(c["arm"].clone()).map_err(|e| e.to_string())?;
            let (s, status) = match rows.get(&arm.name) {
                Some((s, runs, verified)) if runs == verified => (Some(*s), "verified".to_string()),
                Some((s, runs, verified)) => (Some(*s), format!("{verified}/{runs} verified")),
                None => (
                    None,
                    if row["ok"] == true {
                        "no verified runs".into()
                    } else {
                        "session failed".into()
                    },
                ),
            };
            ledger.trials.push(Trial {
                round,
                stage: "train".into(),
                size: k,
                track: track.clone(),
                arm,
                s,
                status,
            });
        }
        let seed = manifest["seeds"][k.to_string()]["train"]
            .as_u64()
            .unwrap_or(0);
        ledger.exposures.push(Exposure {
            size: k,
            target_seed: seed,
            targets: manifest["train"]["targets"].as_u64().unwrap_or(0),
            source: format!("{source}/train"),
        });
    }
    let mut summary = Vec::new();
    for (k_s, r) in result["sizes"].as_object().ok_or("result.sizes")? {
        let k: u32 = k_s.parse().map_err(|e| format!("{e}"))?;
        let class: Class = serde_json::from_value(r["class"].clone()).map_err(|e| e.to_string())?;
        if selection["sizes"][k_s]["status"] == "challenger" {
            let seed = manifest["seeds"][k_s]["holdout"].as_u64().unwrap_or(0);
            ledger.exposures.push(Exposure {
                size: k,
                target_seed: seed,
                targets: manifest["holdout"]["targets"].as_u64().unwrap_or(0),
                source: format!("{source}/holdout"),
            });
            if let Ok(sel) = serde_json::from_value::<ArmConfig>(r["sel"].clone()) {
                ledger.trials.push(Trial {
                    round,
                    stage: "holdout".into(),
                    size: k,
                    track: track.clone(),
                    arm: sel,
                    s: s3(&r["s"]["sel"]),
                    status: format!("{:?}", class).to_lowercase(),
                });
            }
        }
        if class == Class::Improvement {
            let sel: ArmConfig =
                serde_json::from_value(r["sel"].clone()).map_err(|e| e.to_string())?;
            let s_to = s3(&r["s"]["sel"]).ok_or("sel S")?;
            let rho = s3(&r["s"]["rho_neg"]).ok_or("rho S")?;
            let ratio = r["ratio"]["w0"]["ratio"].as_f64().ok_or("ratio")?;
            let ci = r["ratio"]["w0"]["ci95"]
                .as_array()
                .and_then(|a| Some([a.first()?.as_f64()?, a.get(1)?.as_f64()?]))
                .ok_or("ci")?;
            let session = r["evidence"]["session_id"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let old = ledger.incumbent(&track, k)?.clone();
            ledger.history.push(RatchetEvent {
                round,
                size: k,
                track: track.clone(),
                from: old.arm.clone(),
                to: sel.clone(),
                s_from: old.s,
                s_to,
                ratio,
                ci95: ci,
                session: session.clone(),
            });
            let evidence = json!({
                "round": round,
                "dir": source,
                "session_id": session,
                "spec_sha256": r["evidence"]["spec_sha256"],
                "verify_receipt_sha256": r["evidence"]["verify_receipt_sha256"],
                "binary_sha256": r["evidence"]["binary_sha256"],
                "paired_ratio_w0": ratio,
                "ci95_w0": ci,
                "band_verdict": r["band_verdict"],
                "route": manifest["route"],
                "validation_gate": "none (development loop)",
            });
            ledger.tracks.get_mut(&track).unwrap().insert(
                k_s.clone(),
                Incumbent {
                    arm: sel.clone(),
                    s: s_to,
                    rho_s: rho,
                    evidence,
                    since_round: round,
                },
            );
            summary.push(format!(
                "k = {k}: improvement, {} -> {} (sel/inc {ratio:.3} [{:.3}, {:.3}])",
                old.arm.name, sel.name, ci[0], ci[1]
            ));
        } else {
            summary.push(format!("k = {k}: {:?}, incumbent kept", class));
        }
    }
    if let Some(e) = ledger.rounds.iter_mut().find(|r| r.round == round) {
        e.status = "ratcheted".into();
    }
    save_ledger(home, &mut ledger)?;
    for s in summary {
        println!("{s}");
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// report
// ---------------------------------------------------------------------------

fn svg_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Grouped bars of mean S at w = 0 per arm, one panel per size, with the
/// incumbent as a horizontal line. Recorded values only.
fn round_svg(round: u64, panels: &[(u32, String, Vec<(String, [f64; 3], bool)>)]) -> String {
    let (pw, ph, ml, mt) = (420.0, 260.0, 50.0, 40.0);
    let w = ml + pw * panels.len() as f64 + 30.0;
    let h = mt + ph + 70.0;
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}' font-family='Helvetica,Arial,sans-serif' font-size='11'>\n<rect width='100%' height='100%' fill='white'/>\n<text x='{ml}' y='20' font-size='14' font-weight='bold'>ic_autolab round {round}: mean S at w = 0 per arm (ecbench counted GAE / sqrt r; every run verified)</text>\n");
    for (i, (k, session, arms)) in panels.iter().enumerate() {
        let x0 = ml + pw * i as f64;
        let max = arms.iter().map(|a| a.1[0]).fold(1.0f64, f64::max) * 1.15;
        s += &format!(
            "<text x='{}' y='{}' font-size='12' font-weight='bold'>k = {k}  ({})</text>\n",
            x0,
            mt - 6.0,
            svg_escape(session)
        );
        for t in 0..=4 {
            let v = max * t as f64 / 4.0;
            let y = mt + ph - ph * t as f64 / 4.0;
            s += &format!("<line x1='{x0}' x2='{}' y1='{y:.1}' y2='{y:.1}' stroke='#ddd'/><text x='{}' y='{:.1}' text-anchor='end' font-size='9'>{v:.1}</text>\n", x0 + pw - 20.0, x0 - 4.0, y + 3.0);
        }
        let bw = (pw - 30.0) / arms.len().max(1) as f64;
        for (j, (name, sv, ok)) in arms.iter().enumerate() {
            let bh = ph * sv[0] / max;
            let x = x0 + 5.0 + bw * j as f64;
            let col = if name == "inc" {
                "#444"
            } else if name == "rho-neg" {
                "#2a7"
            } else if *ok {
                "#36c"
            } else {
                "#c33"
            };
            s += &format!(
                "<rect x='{x:.1}' y='{:.1}' width='{:.1}' height='{bh:.1}' fill='{col}'/>\n",
                mt + ph - bh,
                bw * 0.8
            );
            s += &format!(
                "<text x='{:.1}' y='{:.1}' text-anchor='middle' font-size='9'>{:.2}</text>\n",
                x + bw * 0.4,
                mt + ph - bh - 3.0,
                sv[0]
            );
            s += &format!("<text transform='translate({:.1},{:.1}) rotate(-45)' text-anchor='end' font-size='9'>{}</text>\n", x + bw * 0.4, mt + ph + 12.0, svg_escape(name));
        }
        if let Some(inc) = arms.iter().find(|a| a.0 == "inc") {
            let y = mt + ph - ph * inc.1[0] / max;
            s += &format!("<line x1='{x0}' x2='{}' y1='{y:.1}' y2='{y:.1}' stroke='#444' stroke-dasharray='4 3'/>\n", x0 + pw - 20.0);
        }
    }
    s += &format!("<text x='{ml}' y='{}' font-size='10' fill='#666'>grey: incumbent; green: matched rho; blue: candidate (every run verified); red: candidate with an unverified or failed run. Bars are recorded band means; the dashed line is the incumbent.</text>\n</svg>\n", h - 8.0);
    s
}

/// The ratchet across rounds: incumbent S at w = 0 per size, by round.
fn ratchet_svg(ledger: &Ledger, track: &str) -> String {
    let (w, h, ml, mt, mb) = (720.0, 320.0, 60.0, 40.0, 50.0);
    let mut series: BTreeMap<u32, Vec<(u64, f64)>> = BTreeMap::new();
    if let Some(t) = ledger.tracks.get(track) {
        for (k, inc) in t {
            let k: u32 = k.parse().unwrap_or(0);
            let mut pts = vec![];
            // Seeded value at round 0 reconstructed from history, then each ratchet.
            let mut events: Vec<&RatchetEvent> = ledger
                .history
                .iter()
                .filter(|e| e.track == track && e.size == k)
                .collect();
            events.sort_by_key(|e| e.round);
            if let Some(first) = events.first() {
                pts.push((0, first.s_from[0]));
                for e in &events {
                    pts.push((e.round, e.s_to[0]));
                }
            } else {
                pts.push((inc.since_round, inc.s[0]));
            }
            let last_round = ledger.rounds.iter().map(|r| r.round).max().unwrap_or(0);
            if let Some(l) = pts.last().cloned() {
                if l.0 < last_round {
                    pts.push((last_round, l.1));
                }
            }
            series.insert(k, pts);
        }
    }
    let max_r = series
        .values()
        .flatten()
        .map(|p| p.0)
        .max()
        .unwrap_or(1)
        .max(1) as f64;
    let max_s = series.values().flatten().map(|p| p.1).fold(1.0, f64::max) * 1.1;
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}' font-family='Helvetica,Arial,sans-serif' font-size='11'>\n<rect width='100%' height='100%' fill='white'/>\n<text x='{ml}' y='20' font-size='14' font-weight='bold'>Ratchet: incumbent mean S at w = 0 by round ({track} track, Curve25519 miniatures)</text>\n");
    let colours = ["#36c", "#c63", "#393", "#939", "#099"];
    for t in 0..=4 {
        let v = max_s * t as f64 / 4.0;
        let y = mt + (h - mt - mb) * (1.0 - t as f64 / 4.0);
        s += &format!("<line x1='{ml}' x2='{}' y1='{y:.1}' y2='{y:.1}' stroke='#ddd'/><text x='{}' y='{:.1}' text-anchor='end' font-size='9'>{v:.1}</text>\n", w - 120.0, ml - 4.0, y + 3.0);
    }
    for (i, (k, pts)) in series.iter().enumerate() {
        let col = colours[i % colours.len()];
        let path: Vec<String> = pts
            .iter()
            .map(|(r, v)| {
                let x = ml + (w - 120.0 - ml) * (*r as f64) / max_r;
                let y = mt + (h - mt - mb) * (1.0 - v / max_s);
                format!("{x:.1},{y:.1}")
            })
            .collect();
        s += &format!(
            "<polyline fill='none' stroke='{col}' stroke-width='2' points='{}'/>\n",
            path.join(" ")
        );
        for p in &path {
            let (x, y) = p.split_once(',').unwrap();
            s += &format!("<circle cx='{x}' cy='{y}' r='3' fill='{col}'/>\n");
        }
        s += &format!(
            "<text x='{}' y='{}' fill='{col}'>k = {k}</text>\n",
            w - 110.0,
            mt + 14.0 * i as f64
        );
    }
    for r in 0..=(max_r as u64) {
        let x = ml + (w - 120.0 - ml) * (r as f64) / max_r;
        s += &format!(
            "<text x='{x:.1}' y='{}' text-anchor='middle' font-size='9'>{r}</text>\n",
            h - mb + 14.0
        );
    }
    s += &format!(
        "<text x='{}' y='{}' text-anchor='middle'>round (0 = the seeded incumbent)</text>\n",
        (ml + w - 120.0) / 2.0,
        h - 14.0
    );
    s += &format!("<text x='{ml}' y='{}' font-size='10' fill='#666'>Recorded ledger values; a step down is a ratcheted improvement (paired interval below 1 at w = 0, band order at every weight).</text>\n</svg>\n", h - 2.0);
    s
}

/// The newest pcc claim whose statement names this round
/// (`ic_autolab round N ...`), read from `pcc/claims/` under the working
/// directory (the repository root).
fn find_claim(round: u64) -> Option<String> {
    let prefix = format!("ic_autolab round {round} ");
    let mut found: Vec<(String, String)> = std::fs::read_dir("pcc/claims")
        .ok()?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let v = read_json(&e.path()).ok()?;
            let statement = v["statement"].as_str()?;
            let id = v["id"].as_str().or_else(|| v["claim_id"].as_str())?;
            statement.starts_with(&prefix).then(|| {
                (
                    v["created_at"].as_str().unwrap_or("").to_string(),
                    id.to_string(),
                )
            })
        })
        .collect();
    found.sort();
    found.pop().map(|(_, id)| id)
}

fn cmd_report(home: &Path, round: u64) -> Result<(), String> {
    let ledger = load_ledger(home)?;
    let dir = round_dir(home, round);
    let manifest = read_json(&dir.join("MANIFEST.json"))?;
    let panel = read_json(&dir.join("panel.json"))?;
    let track = manifest["track"]
        .as_str()
        .ok_or("manifest.track")?
        .to_string();
    let selection = read_json(&dir.join("selection.json")).ok();
    let result = read_json(&dir.join("RESULT.json")).ok();
    let run_train = read_json(&dir.join("analysis").join("run-train.json")).ok();
    let run_holdout = read_json(&dir.join("analysis").join("run-holdout.json")).ok();
    let mut md = format!("# ic_autolab round {round} ({track} track): report\n\n");
    md += &format!(
        "Protocol: [PROTOCOL.md](PROTOCOL.md), frozen {} at git `{}`; route `{}`. \
         Counted GAE only (deterministic, host independent); no wall-time claim; development loop, no validation-gate row. \
         Loop home: [`../`](../README.md); ledger: [`../ledger.json`](../ledger.json).\n\n",
        manifest["frozen"].as_str().unwrap_or(""),
        manifest["git_head"].as_str().unwrap_or("unknown"),
        manifest["route"].as_str().unwrap_or("")
    );
    // The round's claim record (AGENTS.md "Sabotage is assumed"): cited by id
    // with an explicit unverified tag; `pcc standing` is the only source for
    // a stronger word.
    match find_claim(round) {
        Some(id) => {
            md += &format!(
                "Claim record: `{id}` (`python3 tools/pcc/pcc.py standing {id}`). Until a signed replay and recompute exist, this round's outcome is an unverified assertion [pcc:unverified].\n\n"
            )
        }
        None => {
            md += "Claim record: none yet. Record one with `python3 tools/pcc/pcc.py claim new` before citing this round; until then its outcome is an unverified assertion [pcc:unverified].\n\n"
        }
    }
    // Outcome first.
    md += "## Outcome\n\n";
    if let Some(res) = &result {
        md += "| k | class | sel | sel / inc at w = 0 [95%] | w = 0.1 | w = 1 | band | S sel (w = 0 / 0.1 / 1) | S inc | S rho-neg | distance sel / rho |\n|--:|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|\n";
        for (k, r) in res["sizes"].as_object().unwrap() {
            if r["class"] == "no_challenger" {
                // The train screen's best candidate, from selection.json.
                let sel_k = selection
                    .as_ref()
                    .map(|s| s["sizes"][k].clone())
                    .unwrap_or(Value::Null);
                let best = sel_k["candidates"]
                    .as_array()
                    .map(|c| {
                        c.iter()
                            .filter(|t| t["status"] == "verified")
                            .filter_map(|t| {
                                s3(&t["s"]).map(|s| {
                                    (s, t["arm"]["name"].as_str().unwrap_or("").to_string())
                                })
                            })
                            .min_by(|a, b| a.0[0].partial_cmp(&b.0[0]).unwrap())
                    })
                    .unwrap_or(None);
                let inc_s = s3(&sel_k["inc_s"]);
                let rho_s = s3(&sel_k["rho_s"]);
                md += &format!(
                    "| {k} | **no_challenger** | none (best candidate `{}`, train S {}) | - | - | - | - | - | {} | {} | inc: {} |\n",
                    best.as_ref().map(|b| b.1.clone()).unwrap_or("-".into()),
                    best.as_ref().map(|b| fmt_s(&b.0)).unwrap_or("-".into()),
                    inc_s.map(|s| fmt_s(&s)).unwrap_or("-".into()),
                    rho_s.map(|s| fmt_s(&s)).unwrap_or("-".into()),
                    inc_s.zip(rho_s).map(|(a, b)| { let d = distance(&a, &b); format!("{:.1}x / {:.1}x / {:.1}x", d[0], d[1], d[2]) }).unwrap_or("-".into()),
                );
                continue;
            }
            let f = |w: &str| -> String {
                let x = &r["ratio"][w];
                match (x["ratio"].as_f64(), x["ci95"].as_array()) {
                    (Some(v), Some(c)) => format!(
                        "{v:.3} [{:.3}, {:.3}]",
                        c[0].as_f64().unwrap_or(0.0),
                        c[1].as_f64().unwrap_or(0.0)
                    ),
                    _ => "-".into(),
                }
            };
            md += &format!(
                "| {k} | **{}** | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                r["class"].as_str().unwrap_or(""),
                r["sel"]["name"].as_str().unwrap_or("-"),
                f("w0"),
                f("w0.1"),
                f("w1"),
                r["band_verdict"].as_str().unwrap_or("-"),
                s3(&r["s"]["sel"]).map(|s| fmt_s(&s)).unwrap_or("-".into()),
                s3(&r["s"]["inc"]).map(|s| fmt_s(&s)).unwrap_or("-".into()),
                s3(&r["s"]["rho_neg"])
                    .map(|s| fmt_s(&s))
                    .unwrap_or("-".into()),
                s3(&r["distance_to_rho"]["sel"])
                    .map(|s| format!("{:.1}x / {:.1}x / {:.1}x", s[0], s[1], s[2]))
                    .unwrap_or("-".into()),
            );
        }
        md += "\nClasses follow the protocol's decision rule. Distance to rho is the distance remaining, not a gate. The reasons per size:\n\n";
        for (k, r) in res["sizes"].as_object().unwrap() {
            let why: Vec<String> = r["why"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .map(|w| w.as_str().unwrap_or("").to_string())
                        .collect()
                })
                .unwrap_or_default();
            md += &format!(
                "- k = {k}: {}\n",
                if why.is_empty() {
                    "no candidate below the incumbent at w = 0 on the train targets; the incumbent is kept and the round's arms join the trial history".to_string()
                } else {
                    why.join("; ")
                }
            );
        }
        md += "\n";
    } else {
        md += "Not assessed yet.\n\n";
    }
    // Train table and graph.
    md += "## Train (development screen, in sample)\n\n";
    let mut panels = Vec::new();
    if let Some(rt) = &run_train {
        for row in rt["sessions"].as_array().unwrap() {
            let k = row["size"].as_u64().unwrap_or(0) as u32;
            let session = row["session_id"]
                .as_str()
                .unwrap_or("(no session)")
                .to_string();
            md += &format!(
                "### k = {k}: session `{session}` (run exit {}, verify exit {}, receipt sha256 `{}`)\n\n",
                row["run_exit"],
                row["verify_exit"],
                row["verify_receipt_sha256"].as_str().unwrap_or("-")
            );
            md += "| arm | family | N | factor base | oracle | runs | verified | S (w = 0 / 0.1 / 1) | reason proposed |\n|:--|:--|--:|:--|:--|--:|--:|:--|:--|\n";
            let band = row["band"]
                .as_str()
                .and_then(|p| read_json(Path::new(p)).ok());
            let rows = band.as_ref().map(band_rows).unwrap_or_default();
            let mut arms_for_svg = Vec::new();
            let inc: Option<ArmConfig> =
                serde_json::from_value(panel["sizes"][k.to_string()]["incumbent"].clone()).ok();
            let mut listed: Vec<(String, Option<ArmConfig>, String)> =
                vec![("rho-neg".into(), None, "matched reference".into())];
            if let Some(i) = &inc {
                listed.push(("inc".into(), Some(i.clone()), "incumbent".into()));
            }
            for c in panel["sizes"][k.to_string()]["candidates"]
                .as_array()
                .cloned()
                .unwrap_or_default()
            {
                let arm: ArmConfig =
                    serde_json::from_value(c["arm"].clone()).map_err(|e| e.to_string())?;
                listed.push((
                    arm.name.clone(),
                    Some(arm),
                    c["reason"].as_str().unwrap_or("").to_string(),
                ));
            }
            for (name, arm, reason) in listed {
                let (s, runs, verified) = rows.get(&name).cloned().unwrap_or(([f64::NAN; 3], 0, 0));
                md += &format!(
                    "| `{name}` | {} | {} | `{}` | `{}` | {runs} | {verified} | {} | {} |\n",
                    arm.as_ref()
                        .map(|a| a.family.clone())
                        .unwrap_or("rho".into()),
                    arm.as_ref().map(|a| a.n.to_string()).unwrap_or("-".into()),
                    arm.as_ref()
                        .map(|a| a.factor_base.clone())
                        .unwrap_or("-".into()),
                    arm.as_ref()
                        .map(|a| a.oracle.clone())
                        .unwrap_or("rho.negation".into()),
                    if runs > 0 {
                        fmt_s(&s)
                    } else {
                        "no verified runs".into()
                    },
                    reason
                );
                if runs > 0 {
                    arms_for_svg.push((name, s, runs == verified));
                }
            }
            md += "\n";
            panels.push((k, session, arms_for_svg));
        }
        let svg = round_svg(round, &panels);
        std::fs::write(dir.join("round.svg"), svg).map_err(|e| e.to_string())?;
        md += "![train screen](round.svg)\n\n";
    } else {
        md += "Train has not run.\n\n";
    }
    if let Some(sel) = &selection {
        md += "## Selection\n\n";
        for (k, r) in sel["sizes"].as_object().unwrap() {
            md += &format!(
                "- k = {k}: {} {}{}\n",
                r["status"].as_str().unwrap_or(""),
                r["sel"]["name"]
                    .as_str()
                    .map(|n| format!("`{n}`"))
                    .unwrap_or_default(),
                r["alt_w1"]["name"]
                    .as_str()
                    .map(|n| format!(" (Pareto alternative at w = 1: `{n}`)"))
                    .unwrap_or_default()
            );
        }
        md += "\n";
    }
    if let Some(rh) = &run_holdout {
        md += "## Holdout sessions\n\n| k | session | run exit | verify exit | receipt sha256 | levels |\n|--:|:--|--:|--:|:--|:--|\n";
        for row in rh["sessions"].as_array().unwrap() {
            let levels = row["session_dir"]
                .as_str()
                .and_then(|d| read_json(&Path::new(d).join("session.json")).ok())
                .map(|s| s["status_counts"].to_string())
                .unwrap_or_default();
            md += &format!(
                "| {} | `{}` | {} | {} | `{}` | {} |\n",
                row["size"],
                row["session_id"].as_str().unwrap_or("-"),
                row["run_exit"],
                row["verify_exit"],
                row["verify_receipt_sha256"].as_str().unwrap_or("-"),
                levels
            );
        }
        md += "\n";
    }
    md += "## Ledger after this round\n\n| k | incumbent | since round | S (w = 0 / 0.1 / 1) | distance to rho |\n|--:|:--|--:|:--|:--|\n";
    if let Some(t) = ledger.tracks.get(&track) {
        for (k, inc) in t {
            let d = distance(&inc.s, &inc.rho_s);
            md += &format!(
                "| {k} | `{}` (`{}` + `{}`) | {} | {} | {:.1}x / {:.1}x / {:.1}x |\n",
                inc.arm.name,
                inc.arm.factor_base,
                inc.arm.oracle,
                inc.since_round,
                fmt_s(&inc.s),
                d[0],
                d[1],
                d[2]
            );
        }
    }
    md += "\n![ratchet](../ratchet.svg)\n\n";
    md += "## Evidence and deviations\n\n";
    md += &format!(
        "- ecbench binary sha256: `{}`; `--cpus {}`, allow_busy = {}; `verify --replay {}` per session; audit receipts in `analysis/`; sessions in `sessions/` (records in the artifact store when large).\n",
        run_train.as_ref().and_then(|r| r["ecbench_sha256"].as_str()).unwrap_or("-"),
        run_train.as_ref().and_then(|r| r["cpus"].as_str()).unwrap_or("-"),
        run_train.as_ref().map(|r| r["allow_busy"].to_string()).unwrap_or("-".into()),
        run_train.as_ref().map(|r| r["replay"].to_string()).unwrap_or("-".into())
    );
    md += "- Route: the local route is an explicitly requested development screen, not the Iso Lab EKS default; counts are deterministic and replayed, wall time is not claimed. No row of this round enters the IC leaderboard or `benchmarks/README.md` (validation gate).\n";
    md += "- Train selection is in sample (disclosed); the holdout is fresh (new `target_seed`, excluded exposures). Failed, timed-out and regressing arms are retained above.\n";
    std::fs::write(dir.join("README.md"), md).map_err(|e| e.to_string())?;
    std::fs::write(home.join("ratchet.svg"), ratchet_svg(&ledger, &track))
        .map_err(|e| e.to_string())?;
    println!("report written: {}", dir.join("README.md").display());
    Ok(())
}

// ---------------------------------------------------------------------------
// status, init, loop
// ---------------------------------------------------------------------------

fn cmd_status(home: &Path) -> Result<(), String> {
    let l = load_ledger(home)?;
    println!(
        "{} ({}), created {}, updated {}",
        l.family, l.schema, l.created, l.updated
    );
    for (track, sizes) in &l.tracks {
        println!("track {track}:");
        println!(
            "  {:>3} | {:<14} | {:<5} | {:<28} | {:<10}",
            "k", "incumbent", "round", "S (w=0 / 0.1 / 1)", "to rho"
        );
        for (k, inc) in sizes {
            let d = distance(&inc.s, &inc.rho_s);
            println!(
                "  {k:>3} | {:<14} | {:<5} | {:<28} | {:.1}x / {:.1}x / {:.1}x",
                inc.arm.name,
                inc.since_round,
                fmt_s(&inc.s),
                d[0],
                d[1],
                d[2]
            );
        }
    }
    println!(
        "rounds: {}",
        l.rounds
            .iter()
            .map(|r| format!("{} {}", r.round, r.status))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "trials: {}, exposures: {}, ratchets: {}",
        l.trials.len(),
        l.exposures.len(),
        l.history.len()
    );
    Ok(())
}

fn cmd_init(home: &Path, from: &Path) -> Result<(), String> {
    let target = home.join("ledger.json");
    if target.exists() {
        return Err(format!("{} exists", target.display()));
    }
    let v = read_json(from)?;
    let mut l: Ledger =
        serde_json::from_value(v).map_err(|e| format!("{}: {e}", from.display()))?;
    if l.schema != LEDGER_SCHEMA {
        return Err(format!("seed schema {} is not {LEDGER_SCHEMA}", l.schema));
    }
    for (track, sizes) in &l.tracks {
        for k in sizes.keys() {
            if !l.sizes.contains_key(k) {
                return Err(format!(
                    "track {track} names k = {k}, which sizes does not define"
                ));
            }
        }
    }
    let now = now_utc();
    if l.created.is_empty() {
        l.created = now.clone();
    }
    l.updated = now;
    write_json(
        &target,
        &serde_json::to_value(&l).map_err(|e| e.to_string())?,
    )?;
    println!("ledger written: {}", target.display());
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn cmd_loop(
    home: &Path,
    rounds: u64,
    track: &str,
    sizes: &[u32],
    max_candidates: usize,
    exploration: usize,
    freeze: FreezeOpts,
    run: RunOpts,
) -> Result<(), String> {
    let start = load_ledger(home)?
        .rounds
        .iter()
        .map(|r| r.round)
        .max()
        .unwrap_or(0)
        + 1;
    for round in start..start + rounds {
        println!("=== round {round} ===");
        let panel = home.join(format!("panel-round-{round:03}.json"));
        cmd_propose(
            home,
            round,
            track,
            sizes,
            max_candidates,
            exploration,
            Some(panel.clone()),
        )?;
        cmd_freeze(
            home,
            round,
            &panel,
            FreezeOpts {
                train_targets: freeze.train_targets,
                train_rounds: freeze.train_rounds,
                holdout_targets: freeze.holdout_targets,
                holdout_rounds: freeze.holdout_rounds,
                timeout: freeze.timeout,
                route: freeze.route.clone(),
                ecbench: Some(run.ecbench.clone()),
            },
        )?;
        let _ = std::fs::remove_file(&panel);
        cmd_run(home, round, "train", &run)?;
        let selection = cmd_select(home, round, Some(run.ecbench.clone()))?;
        let any_challenger = selection["sizes"]
            .as_object()
            .map(|m| m.values().any(|v| v["status"] == "challenger"))
            .unwrap_or(false);
        if any_challenger {
            cmd_run(home, round, "holdout", &run)?;
        }
        cmd_assess(home, round, &run.ecbench)?;
        cmd_ratchet(home, round)?;
        cmd_report(home, round)?;
        cmd_status(home)?;
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let home = cli.home.clone();
    let r = match cli.cmd {
        Cmd::Init { from } => cmd_init(&home, &from),
        Cmd::Status => cmd_status(&home),
        Cmd::Propose {
            round,
            track,
            sizes,
            max_candidates,
            exploration,
            out,
        } => parse_sizes(&sizes)
            .and_then(|s| cmd_propose(&home, round, &track, &s, max_candidates, exploration, out))
            .map(|_| ()),
        Cmd::Freeze {
            round,
            panel,
            train_targets,
            train_rounds,
            holdout_targets,
            holdout_rounds,
            timeout_seconds,
            route,
            ecbench,
        } => cmd_freeze(
            &home,
            round,
            &panel,
            FreezeOpts {
                train_targets,
                train_rounds,
                holdout_targets,
                holdout_rounds,
                timeout: timeout_seconds,
                route,
                ecbench,
            },
        ),
        Cmd::Run {
            round,
            stage,
            ecbench,
            cpus,
            allow_busy,
            replay,
        } => cmd_run(
            &home,
            round,
            &stage,
            &RunOpts {
                ecbench,
                cpus,
                allow_busy,
                replay,
            },
        )
        .map(|_| ()),
        Cmd::Card {
            round,
            stage,
            ecbench,
            cpus,
            allow_busy,
            replay,
            n,
        } => cmd_card(
            &home,
            round,
            &stage,
            &RunOpts {
                ecbench,
                cpus,
                allow_busy,
                replay,
            },
            n,
        )
        .map(|_| ()),
        Cmd::Select { round, ecbench } => cmd_select(&home, round, ecbench).map(|_| ()),
        Cmd::Assess { round, ecbench } => cmd_assess(&home, round, &ecbench).map(|_| ()),
        Cmd::Ratchet { round } => cmd_ratchet(&home, round),
        Cmd::Report { round } => cmd_report(&home, round),
        Cmd::Loop {
            rounds,
            track,
            sizes,
            max_candidates,
            exploration,
            train_targets,
            train_rounds,
            holdout_targets,
            holdout_rounds,
            timeout_seconds,
            ecbench,
            cpus,
            allow_busy,
            replay,
        } => parse_sizes(&sizes).and_then(|s| {
            cmd_loop(
                &home,
                rounds,
                &track,
                &s,
                max_candidates,
                exploration,
                FreezeOpts {
                    train_targets,
                    train_rounds,
                    holdout_targets,
                    holdout_rounds,
                    timeout: timeout_seconds,
                    route: "local".into(),
                    ecbench: Some(ecbench.clone()),
                },
                RunOpts {
                    ecbench,
                    cpus,
                    allow_busy,
                    replay,
                },
            )
        }),
    };
    if let Err(e) = r {
        eprintln!("ic_autolab: {e}");
        std::process::exit(1);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn families() -> TrackFamilies {
        serde_json::from_value(json!({
            "factor_base": "prime-projected-abscissa:size={n},scan_cap={scan}",
            "scan_factor": 16, "n_grid": [0.5, 0.7, 1.4, 2.0], "n_min": 4, "n_max": 2047,
            "families": [
                {"id": "mitm2", "oracle": "mitm:negation_folded=1", "default_f": 1.0},
                {"id": "ksum3", "oracle": "ksum:m=3", "default_f": 0.35},
                {"id": "ksum4", "oracle": "ksum:m=4", "default_f": 0.15},
                {"id": "sub2", "oracle": "subtract", "default_f": 1.0, "enabled": false}
            ]
        }))
        .unwrap()
    }

    fn ledger() -> Ledger {
        let tf = families();
        let inc = tf.arm(tf.family("ksum3").unwrap(), 11);
        let mut tracks = BTreeMap::new();
        let mut g = BTreeMap::new();
        g.insert(
            "20".to_string(),
            Incumbent {
                arm: inc,
                s: [8.987, 9.71, 16.217],
                rho_s: [1.578, 1.693, 2.724],
                evidence: json!({}),
                since_round: 0,
            },
        );
        tracks.insert("generic".to_string(), g);
        let mut sizes = BTreeMap::new();
        sizes.insert(
            "20".to_string(),
            SizeInfo {
                k: 20,
                r: 131203,
                log2_r: 17.0,
                n_a: 32,
                curve: json!({"kind": "prime_explicit", "p": 1048573}),
            },
        );
        Ledger {
            schema: LEDGER_SCHEMA.into(),
            family: "curve25519".into(),
            created: String::new(),
            updated: String::new(),
            notes: vec![],
            sizes,
            tracks,
            exposures: vec![Exposure {
                size: 20,
                target_seed: SEED_BASE + 1000 + 20,
                targets: 16,
                source: "test".into(),
            }],
            trials: vec![],
            rounds: vec![],
            history: vec![],
        }
    }

    #[test]
    fn arm_strings_follow_the_family_template() {
        let tf = families();
        let a = tf.arm(tf.family("ksum3").unwrap(), 11);
        assert_eq!(
            a.factor_base,
            "prime-projected-abscissa:size=11,scan_cap=176"
        );
        assert_eq!(a.oracle, "ksum:m=3");
        assert_eq!(a.name, "ksum3-n11");
        assert_eq!(
            tf.arm(tf.family("ksum3").unwrap(), 1).n,
            4,
            "clamped to n_min"
        );
    }

    #[test]
    fn propose_neighbourhood_then_exploration_never_repeats_a_trial() {
        let mut l = ledger();
        let tf = families();
        let p = propose_size(&l, &tf, "generic", 20, 6, 2).unwrap();
        let names: Vec<_> = p.iter().map(|x| x.arm.name.as_str()).collect();
        // nearest factors first (|ln 1.4| < |ln 0.7|), then 0.5, 2.0; two exploration slots.
        assert_eq!(
            names,
            [
                "ksum3-n15",
                "ksum3-n8",
                "ksum3-n6",
                "ksum3-n22",
                "mitm2-n32",
                "ksum4-n5"
            ]
        );
        assert!(
            !names.contains(&"sub2-n32"),
            "disabled family is never proposed"
        );
        // Record them as trials: the next proposal must not repeat any.
        for x in &p {
            l.trials.push(Trial {
                round: 1,
                stage: "train".into(),
                size: 20,
                track: "generic".into(),
                arm: x.arm.clone(),
                s: Some([20.0, 21.0, 30.0]),
                status: "verified".into(),
            });
        }
        let q = propose_size(&l, &tf, "generic", 20, 6, 2).unwrap();
        for x in &q {
            assert!(
                !p.iter().any(|y| y.arm.key() == x.arm.key()),
                "{} repeated",
                x.arm.name
            );
        }
        assert!(
            q.is_empty()
                || q.iter().all(|x| x.arm.family == "ksum3"
                    || x.arm.family == "mitm2"
                    || x.arm.family == "ksum4")
        );
    }

    #[test]
    fn propose_refines_alternatives_within_one_and_a_half_of_the_incumbent() {
        let mut l = ledger();
        let tf = families();
        let alt = tf.arm(tf.family("mitm2").unwrap(), 32);
        l.trials.push(Trial {
            round: 1,
            stage: "train".into(),
            size: 20,
            track: "generic".into(),
            arm: alt,
            s: Some([11.9, 12.7, 20.6]),
            status: "verified".into(),
        });
        let p = propose_size(&l, &tf, "generic", 20, 8, 1).unwrap();
        let names: Vec<_> = p.iter().map(|x| x.arm.name.as_str()).collect();
        assert!(
            names.contains(&"mitm2-n22") && names.contains(&"mitm2-n45"),
            "{names:?}"
        );
        let far = tf.arm(tf.family("ksum4").unwrap(), 5);
        l.trials.push(Trial {
            round: 1,
            stage: "train".into(),
            size: 20,
            track: "generic".into(),
            arm: far,
            s: Some([99.0, 99.0, 99.0]),
            status: "verified".into(),
        });
        let p = propose_size(&l, &tf, "generic", 20, 8, 1).unwrap();
        assert!(
            !p.iter().any(|x| x.arm.family == "ksum4"),
            "an alternative 11x worse is not refined"
        );
    }

    #[test]
    fn an_exhausted_exploration_does_not_close_the_family() {
        let mut l = ledger();
        let tf = families();
        let dead = tf.arm(tf.family("ksum4").unwrap(), 5);
        l.trials.push(Trial {
            round: 1,
            stage: "train".into(),
            size: 20,
            track: "generic".into(),
            arm: dead,
            s: None,
            status: "no verified runs".into(),
        });
        let p = propose_size(&l, &tf, "generic", 20, 8, 2).unwrap();
        // ksum4 at N = 5 is in the history; the family is still untried (no verified S),
        // but the same N would repeat, so nothing is proposed for it at this N.
        assert!(!p.iter().any(|x| x.arm.name == "ksum4-n5"));
        let mut tf2 = tf.clone();
        tf2.families
            .iter_mut()
            .find(|f| f.id == "ksum4")
            .unwrap()
            .n_min = Some(6);
        let p = propose_size(&l, &tf2, "generic", 20, 8, 2).unwrap();
        assert!(
            p.iter().any(|x| x.arm.name == "ksum4-n6"),
            "with a family floor the family is explored again at a new N: {:?}",
            p.iter().map(|x| x.arm.name.clone()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn another_sizes_incumbent_family_transfers_at_the_same_relative_n() {
        let mut l = ledger();
        let tf = families();
        l.sizes.insert(
            "24".into(),
            SizeInfo {
                k: 24,
                r: 2097349,
                log2_r: 21.0,
                n_a: 81,
                curve: json!({}),
            },
        );
        let win = tf.arm(tf.family("ksum4").unwrap(), 12);
        l.tracks.get_mut("generic").unwrap().insert(
            "24".into(),
            Incumbent {
                arm: win,
                s: [7.0, 7.5, 12.3],
                rho_s: [1.2, 1.3, 2.2],
                evidence: json!({}),
                since_round: 2,
            },
        );
        // ksum4 at N = 5 was tried at k = 20 (verified, far worse).
        l.trials.push(Trial {
            round: 2,
            stage: "train".into(),
            size: 20,
            track: "generic".into(),
            arm: tf.arm(tf.family("ksum4").unwrap(), 5),
            s: Some([34.0, 40.0, 92.0]),
            status: "verified".into(),
        });
        let p = propose_size(&l, &tf, "generic", 20, 8, 2).unwrap();
        let names: Vec<_> = p.iter().map(|x| x.arm.name.as_str()).collect();
        // N0 = round(12/81 * 32) = 5 (tried); neighbours 1.4 x 5 = 7, 2 x 5 = 10;
        // 0.7 and 0.5 x 5 fall below the track floor 4 and clamp to it.
        assert!(
            names.contains(&"ksum4-n7") && names.contains(&"ksum4-n10"),
            "{names:?}"
        );
        assert!(
            !names.contains(&"ksum4-n5"),
            "a tried configuration is never repeated"
        );
        let i = names.iter().position(|n| *n == "ksum4-n7").unwrap();
        let j = names.iter().position(|n| n.starts_with("mitm2")).unwrap();
        assert!(i < j, "transfer comes before exploration: {names:?}");
    }

    #[test]
    fn a_family_incumbent_at_two_sizes_extrapolates_n_by_its_own_fit() {
        let mut l = ledger();
        let tf = families();
        for (k, r, l2, na, n) in [
            (24u32, 2097349u64, 21.0, 81u64, 12u64),
            (28, 33557809, 25.0, 203, 21),
        ] {
            l.sizes.insert(
                k.to_string(),
                SizeInfo {
                    k,
                    r,
                    log2_r: l2,
                    n_a: na,
                    curve: json!({}),
                },
            );
            l.tracks.get_mut("generic").unwrap().insert(
                k.to_string(),
                Incumbent {
                    arm: tf.arm(tf.family("ksum4").unwrap(), n),
                    s: [1.0; 3],
                    rho_s: [1.0; 3],
                    evidence: json!({}),
                    since_round: 2,
                },
            );
        }
        l.sizes.insert(
            "32".into(),
            SizeInfo {
                k: 32,
                r: 536884639,
                log2_r: 29.0,
                n_a: 512,
                curve: json!({}),
            },
        );
        l.tracks.get_mut("generic").unwrap().insert(
            "32".into(),
            Incumbent {
                arm: tf.arm(tf.family("mitm2").unwrap(), 512),
                s: [24.0; 3],
                rho_s: [1.0; 3],
                evidence: json!({}),
                since_round: 0,
            },
        );
        let (a, b, pts) = family_fit(&l, "generic", "ksum4", 32).unwrap();
        assert_eq!(pts, 2);
        let n32 = (a + b * 29.0).exp().round() as u64;
        assert_eq!(n32, 37, "12 at 21 bits, 21 at 25 bits -> 37 at 29 bits");
        let p = propose_size(&l, &tf, "generic", 32, 12, 0).unwrap();
        assert!(
            p.iter().any(|x| x.arm.name == "ksum4-n37"),
            "{:?}",
            p.iter().map(|x| x.arm.name.clone()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn exploration_slots_survive_the_budget_cut() {
        let l = ledger();
        let tf = families();
        let p = propose_size(&l, &tf, "generic", 20, 3, 2).unwrap();
        let names: Vec<_> = p.iter().map(|x| x.arm.name.as_str()).collect();
        assert_eq!(names, ["ksum3-n15", "mitm2-n32", "ksum4-n5"]);
    }

    #[test]
    fn seeds_skip_exposed_values_and_differ_between_stages() {
        let l = ledger();
        let (t, h) = round_seeds(1, 20, &l.exposed_seeds());
        assert_ne!(t, SEED_BASE + 1000 + 20, "the exposed seed is skipped");
        assert_ne!(t, h);
        let (t2, _) = round_seeds(2, 20, &l.exposed_seeds());
        assert_eq!(t2, SEED_BASE + 2000 + 20);
    }

    #[test]
    fn train_spec_names_every_arm_and_holdout_adds_the_controls() {
        let l = ledger();
        let tf = families();
        let size = l.size(20).unwrap();
        let inc = l.incumbent("generic", 20).unwrap().arm.clone();
        let c = tf.arm(tf.family("ksum4").unwrap(), 5);
        let s = train_spec(
            Path::new("home"),
            1,
            "generic",
            size,
            &inc,
            std::slice::from_ref(&c),
            7,
            16,
            1,
            1800,
        );
        let names: Vec<_> = s["arms"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| a["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, ["rho-neg", "inc", "ksum4-n5"]);
        assert_eq!(s["measurement"]["order"], "alternate_measured");
        assert_eq!(s["workloads"]["target_seed"], 7);
        let h = holdout_spec(
            Path::new("home"),
            1,
            "generic",
            size,
            &inc,
            &c,
            9,
            32,
            2,
            1800,
        );
        let names: Vec<_> = h["arms"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| a["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, ["rho-neg", "inc", "sel", "inc-aa", "rho-neg-aa"]);
        assert_eq!(
            h["arms"][3]["method"], h["arms"][1]["method"],
            "A/A copies the incumbent exactly"
        );
        assert_eq!(h["measurement"]["order"], "alternate");
    }

    fn band(rows: &[(&str, [f64; 3], u64, u64)], verdict: &str) -> Value {
        json!({
            "schema": "ecbench.band/v1",
            "rows": rows.iter().map(|(a, s, r, v)| json!({"curve": "c", "arm": a, "mean_s": s, "runs": r, "verified": v})).collect::<Vec<_>>(),
            "ranks": [{"curve": "c", "a": "inc", "b": "sel", "verdict": verdict}]
        })
    }

    #[test]
    fn selection_takes_the_lowest_verified_candidate_below_the_incumbent() {
        let tf = families();
        let a = tf.arm(tf.family("ksum3").unwrap(), 8);
        let b = tf.arm(tf.family("ksum4").unwrap(), 5);
        let c = tf.arm(tf.family("mitm2").unwrap(), 32);
        let bd = band(
            &[
                ("inc", [9.0, 9.7, 16.0], 16, 16),
                ("ksum3-n8", [8.0, 9.0, 20.0], 16, 16),
                ("ksum4-n5", [7.0, 7.5, 12.0], 16, 15),
                ("mitm2-n32", [8.5, 9.0, 13.0], 16, 16),
            ],
            "",
        );
        let (sel, alt, table, inc) = select_from_band(&bd, &[a, b, c]);
        assert_eq!(
            sel.unwrap().name,
            "ksum3-n8",
            "ksum4 has an unverified run and is not selectable"
        );
        assert_eq!(alt.unwrap().name, "mitm2-n32");
        assert_eq!(inc.unwrap()[0], 9.0);
        assert_eq!(table[1]["status"], "not every run verified");
        let bd = band(
            &[
                ("inc", [7.0, 7.5, 12.0], 16, 16),
                ("ksum3-n8", [8.0, 9.0, 20.0], 16, 16),
            ],
            "",
        );
        let (sel, _, _, _) = select_from_band(&bd, &[tf.arm(tf.family("ksum3").unwrap(), 8)]);
        assert!(
            sel.is_none(),
            "no challenger when nothing beats the incumbent in train"
        );
    }

    fn cmp(status: &str, admissible: bool, ratio: f64, ci: [f64; 2]) -> Value {
        json!({"ops": {"status": status, "ratio_b_over_a": ratio, "ci95": ci}, "admissible": admissible})
    }

    #[test]
    fn classification_follows_the_frozen_rule() {
        let aa = cmp("ok", true, 1.0, [1.0, 1.0]);
        let (c, _) = classify(
            &cmp("ok", true, 0.76, [0.72, 0.80]),
            &band(&[], "sel < inc"),
            Some(&aa),
        );
        assert_eq!(c, Class::Improvement);
        let (c, _) = classify(
            &cmp("ok", true, 0.76, [0.72, 0.80]),
            &band(&[], "unresolved inside the band"),
            Some(&aa),
        );
        assert_eq!(c, Class::Null, "band must order the pair at every weight");
        let (c, _) = classify(
            &cmp("ok", true, 0.99, [0.94, 1.05]),
            &band(&[], "sel < inc"),
            Some(&aa),
        );
        assert_eq!(c, Class::Null);
        let (c, _) = classify(
            &cmp("ok", true, 1.06, [1.03, 1.09]),
            &band(&[], "unresolved inside the band"),
            Some(&aa),
        );
        assert_eq!(c, Class::Regression);
        let (c, _) = classify(
            &cmp("ok", true, 0.76, [0.72, 0.80]),
            &band(&[], "inc < sel"),
            Some(&aa),
        );
        assert_eq!(
            c,
            Class::Regression,
            "a headline fall with a band reversal is relabelling, not an improvement"
        );
        let (c, _) = classify(
            &cmp("incomplete", false, 0.5, [0.4, 0.6]),
            &band(&[], "sel < inc"),
            Some(&aa),
        );
        assert_eq!(c, Class::Inadmissible);
        let bad_aa = cmp("ok", true, 1.2, [1.1, 1.3]);
        let (c, _) = classify(
            &cmp("ok", true, 0.76, [0.72, 0.80]),
            &band(&[], "sel < inc"),
            Some(&bad_aa),
        );
        assert_eq!(
            c,
            Class::Inadmissible,
            "an A/A interval excluding 1 voids the session"
        );
    }

    #[test]
    fn civil_dates_are_right() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(20761), (2026, 11, 4));
    }
}
