//! `ic sweep-trial`: one configuration, one scored, classified record.
//!
//! A trial is the unit the sweeper optimises over.  It runs one pipeline
//! configuration through `ic bench` (so the instance, the planted
//! logarithm, the verification and the matched rho are the framework's, not
//! a second copy) and reduces the report to a record whose score is a
//! function of **operation counts alone**.
//!
//! # The score
//!
//! AGENTS.md §5a prices every arm's native work at a weight `w` and ranks
//! two arms only where the order holds across the band.  For a record with
//! `G` charged GAE, `C` the GAE a pinned calibration put on native work,
//! `n_C` the operations behind `C` and `U` the count behind its unpriced
//! counters, `priced_gae(w) = G − C + w·(U + n_C)`.  `G − C` is the group
//! operations (the native conversions are exactly what `C` removes), and
//! `U + n_C` is every native operation, so
//!
//! ```text
//! priced_gae(w) = (adds + doubles) + w · (native operations)
//! ```
//!
//! and no calibration ratio, host timing or solver wall pricing enters.  Two
//! runs of the same configuration therefore score identically (checked: a
//! debug and a release build, one host), which is the property an optimiser
//! needs.  It has not been checked across hosts.  `native operations` are the counters
//! `price_phase` converts (`PRICED_NATIVE`), every `*_uncharged` counter,
//! and the solver's own `ops` in its own unit, the same set
//! `ecbench`'s `ic.pipeline` method lists as priced or unpriced.  Any other
//! native counter is reported under `other_native` and not scored, so a
//! counter that deserves a price is visible rather than silently free.
//!
//! The solver's operations are in the solver's own unit, so a trial's score
//! is comparable with a baseline's only where the units agree: see
//! [`unit_family`].
//!
//! # Failure classes
//!
//! A trial that does not produce a verified answer is never scored.  Each
//! ends in exactly one class, and none is "no solution":
//!
//! | class | meaning |
//! |:--|:--|
//! | `ok` | every repeat recovered the planted logarithm |
//! | `exhausted` | a repeat hit `max_trials` or a solver budget without recovering it |
//! | `invalid` | a repeat recovered the **wrong** logarithm (a defect, never a result) |
//! | `refused` | the instance or model cannot represent the configuration |
//! | `invalid_config` | the configuration is malformed |
//! | `crashed` | the run panicked or produced no rows |

use std::path::PathBuf;
use std::{collections::BTreeMap, panic::AssertUnwindSafe};

use clap::{Args, Parser};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::bench::{self, BenchArgs};

pub const SCHEMA: &str = "ic.sweep-trial/v1";

/// The band of AGENTS.md §5a: GAE charged per native operation.
pub const BAND_WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];

/// The native counters `price_phase` converts, i.e. native work a pinned
/// calibration may price (`ecbench::methods::PRICED_NATIVE`).
const PRICED_NATIVE: &[&str] = &[
    "sqrt_solves",
    "as_solves",
    "s4_pairs",
    "lookups",
    "row_ops",
    "legendre_symbols",
    "inversions",
    "frobenius_maps",
    "canonicalisations",
    "target_guard_probes",
];

/// The unit a solver's operations are counted in, normalised the way the
/// repository's own pricing does it (`ic_framework::is_word_xor_op_unit`):
/// `word XORs` with or without a qualifier is one family, priced by the one
/// pinned ratio.  Any other unit stands for itself.
///
/// Two solvers' operations can be compared only inside one family.  The
/// framework measured a Buchberger monomial operation at about 50 ns against
/// 0.4 ns for a word XOR, and a SAT conflict is neither, so a ratio of
/// `S` across units says which engine counts coarsely, not which is cheaper.
/// An optimiser would find the coarsest unit, which is "changing the unit"
/// (AGENTS.md §6).
pub fn unit_family(op_unit: &str) -> String {
    if op_unit == "word XORs" || op_unit.starts_with("word XORs (") {
        "word XORs".into()
    } else {
        op_unit.to_string()
    }
}

/// The unit family of a record's solver, `None` when no solver ran.
pub fn solver_unit(record: &TrialRecord) -> Option<String> {
    record
        .context
        .pointer("/solver/op_unit")
        .and_then(Value::as_str)
        .map(unit_family)
}

/// Where each phase's cost sits in a `RunReport`.
const PHASE_PATHS: &[&[&str]] = &[
    &["factor_base", "cost"],
    &["decomposition", "setup"],
    &["decomposition", "cost"],
    &["linear_algebra", "cost"],
    &["verify"],
];

#[derive(Args, Clone)]
pub struct TrialArgs {
    /// The trial configuration, as a JSON file.
    #[arg(long, conflicts_with = "config_json")]
    pub config: Option<PathBuf>,
    /// The trial configuration, inline.
    #[arg(long)]
    pub config_json: Option<String>,
    /// Safety net: stop and report `exhausted` after this many seconds.  A
    /// run that hits it is classed by the host's speed, so prefer the
    /// configuration's counted `max_trials`; 0 means no limit.
    #[arg(long, default_value_t = 0)]
    pub timeout_seconds: u64,
    /// Safety net: stop and report `exhausted` once this process's peak
    /// resident memory passes this many MiB; 0 means no limit.
    #[arg(long, default_value_t = 0)]
    pub max_rss_mb: u64,
}

/// What a trial runs.  Closed on purpose: a misspelt field is an error, not
/// a silently ignored knob.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TrialConfig {
    /// `prime`, `char2` or `koblitz`.
    pub regime: String,
    /// Subgroup bits (`prime`) or field degree (`char2`, `koblitz`).
    pub degree: u32,
    /// `prime` only: `generic`, `j0`, `j1728`, `d7`, `d8`.
    #[serde(default)]
    pub family: Option<String>,
    /// `koblitz` only: 0 for `K_0`, 1 for `K_1`.
    #[serde(default)]
    pub koblitz_a: Option<u8>,
    /// `char2` only.
    #[serde(default)]
    pub max_cofactor: Option<u64>,
    pub factor_base: String,
    pub oracle: String,
    #[serde(default)]
    pub solver: Option<String>,
    #[serde(default = "default_targets")]
    pub targets: String,
    #[serde(default = "default_linalg")]
    pub linalg: String,
    /// Counted trials before a run gives up.  A count, never a time.
    #[serde(default = "default_max_trials")]
    pub max_trials: u64,
    #[serde(default = "default_seed")]
    pub seed: u64,
    #[serde(default = "default_repeats")]
    pub repeats: usize,
    /// Counted rho runs for the context column (0 leaves it empty).
    #[serde(default = "default_rho_runs")]
    pub rho_runs: usize,
}

fn default_targets() -> String {
    "walk".into()
}
fn default_linalg() -> String {
    "incremental-gauss".into()
}
fn default_max_trials() -> u64 {
    2_000_000
}
fn default_seed() -> u64 {
    0x1C_B0_0B_DA_7A
}
fn default_repeats() -> usize {
    1
}
fn default_rho_runs() -> usize {
    8
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Class {
    Ok,
    Exhausted,
    Invalid,
    Refused,
    InvalidConfig,
    Crashed,
}

impl Class {
    pub fn as_str(self) -> &'static str {
        match self {
            Class::Ok => "ok",
            Class::Exhausted => "exhausted",
            Class::Invalid => "invalid",
            Class::Refused => "refused",
            Class::InvalidConfig => "invalid_config",
            Class::Crashed => "crashed",
        }
    }
}

/// Band-priced `S` of a trial: the mean over its verified repeats.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Band {
    pub weights: Vec<f64>,
    /// `priced_gae(w) / sqrt(r)` at each weight.
    pub s: Vec<f64>,
    /// Group additions plus doublings (the `w`-independent part).
    pub group_gae: f64,
    /// Native operations priced at `w`: priced counters, `*_uncharged`
    /// counters and the solver's own operations.
    pub native_ops: f64,
    /// Of `native_ops`, the solver's operations (in the solver's unit).
    pub solver_ops: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrialRecord {
    pub schema: String,
    pub class: Class,
    pub detail: String,
    pub config: TrialConfig,
    pub instance: Option<String>,
    pub log2_r: Option<f64>,
    pub repeats_run: usize,
    pub repeats_verified: usize,
    /// Present only for class `ok`.
    pub band: Option<Band>,
    /// Everything read off the report that is not the score.
    pub context: Value,
}

// ── Configuration → `ic bench` arguments ────────────────────────────

#[derive(Parser)]
struct BenchWrapper {
    #[command(flatten)]
    bench: BenchArgs,
}

impl TrialConfig {
    pub fn load(args: &TrialArgs) -> Result<TrialConfig, String> {
        let raw = match (&args.config, &args.config_json) {
            (Some(path), None) => std::fs::read_to_string(path)
                .map_err(|e| format!("cannot read {}: {e}", path.display()))?,
            (None, Some(inline)) => inline.clone(),
            _ => return Err("give exactly one of --config and --config-json".into()),
        };
        serde_json::from_str(&raw).map_err(|e| format!("trial configuration is not valid: {e}"))
    }

    fn argv(&self) -> Result<Vec<String>, String> {
        let mut v: Vec<String> = vec!["ic-bench".into()];
        let mut push = |flag: &str, value: String| {
            v.push(flag.into());
            v.push(value);
        };
        match self.regime.as_str() {
            "prime" => {
                push("--bits", self.degree.to_string());
                if let Some(f) = &self.family {
                    push("--family", f.clone());
                }
            }
            "char2" => {
                push("--char2-degree", self.degree.to_string());
                if let Some(c) = self.max_cofactor {
                    push("--max-cofactor", c.to_string());
                }
            }
            "koblitz" => {
                push("--koblitz-degree", self.degree.to_string());
                if let Some(a) = self.koblitz_a {
                    push("--koblitz-a", a.to_string());
                }
            }
            other => {
                return Err(format!(
                    "unknown regime `{other}`; expected prime, char2 or koblitz"
                ))
            }
        }
        push("--factor-base", self.factor_base.clone());
        push("--oracle", self.oracle.clone());
        if let Some(s) = &self.solver {
            push("--solver", s.clone());
        }
        push("--targets", self.targets.clone());
        push("--linalg", self.linalg.clone());
        push("--max-trials", self.max_trials.to_string());
        push("--seed", self.seed.to_string());
        push("--repeats", self.repeats.max(1).to_string());
        push("--rho-runs", self.rho_runs.to_string());
        // A wall-clock budget makes a run depend on the host.  Counts only.
        push("--solver-budget-seconds", "0".into());
        Ok(v)
    }

    pub fn bench_args(&self) -> Result<BenchArgs, String> {
        let argv = self.argv()?;
        BenchWrapper::try_parse_from(argv)
            .map(|w| w.bench)
            .map_err(|e| e.to_string())
    }
}

// ── Report → score ──────────────────────────────────────────────────

fn at<'a>(v: &'a Value, path: &[&str]) -> Option<&'a Value> {
    path.iter().try_fold(v, |cur, key| cur.get(*key))
}

fn u64_at(v: &Value, path: &[&str]) -> u64 {
    at(v, path).and_then(Value::as_u64).unwrap_or(0)
}

/// One verified row's operation counts.
#[derive(Debug, PartialEq)]
struct RowCost {
    r: f64,
    group_gae: u64,
    native_ops: u64,
    solver_ops: u64,
    other_native: BTreeMap<String, u64>,
}

fn row_cost(row: &Value) -> Result<RowCost, String> {
    let r = row
        .get("r")
        .and_then(Value::as_f64)
        .filter(|r| *r > 1.0)
        .ok_or("a report row has no subgroup order r")?;
    let mut group_gae = 0u64;
    let mut priced = 0u64;
    let mut other_native = BTreeMap::new();
    for path in PHASE_PATHS {
        let Some(phase) = at(row, path) else { continue };
        group_gae +=
            u64_at(phase, &["group_ops", "adds"]) + u64_at(phase, &["group_ops", "doubles"]);
        if let Some(native) = phase.get("native").and_then(Value::as_object) {
            for (name, count) in native {
                let count = count.as_u64().unwrap_or(0);
                if PRICED_NATIVE.contains(&name.as_str()) || name.ends_with("_uncharged") {
                    priced += count;
                } else if !name.starts_with("solver_") && count > 0 {
                    // The solver's operations are taken from its own block
                    // below; `solver_calls` and friends are bookkeeping.
                    *other_native.entry(name.clone()).or_insert(0) += count;
                }
            }
        }
    }
    let solver_ops = at(row, &["decomposition", "solver"])
        .filter(|s| s.is_object())
        .map_or(0, |s| u64_at(s, &["ops"]));
    Ok(RowCost {
        r,
        group_gae,
        native_ops: priced + solver_ops,
        solver_ops,
        other_native,
    })
}

/// The band-priced `S` of verified rows, averaged.  Only called on rows that
/// all verified.
fn band_of(rows: &[&Value]) -> Result<(Band, BTreeMap<String, u64>), String> {
    let costs = rows
        .iter()
        .map(|r| row_cost(r))
        .collect::<Result<Vec<_>, _>>()?;
    let n = costs.len() as f64;
    let mut s = vec![0.0; BAND_WEIGHTS.len()];
    let (mut group, mut native, mut solver) = (0.0, 0.0, 0.0);
    let mut other = BTreeMap::new();
    for c in &costs {
        let sqrt_r = c.r.sqrt();
        for (slot, w) in s.iter_mut().zip(BAND_WEIGHTS) {
            *slot += (c.group_gae as f64 + w * c.native_ops as f64) / sqrt_r / n;
        }
        group += c.group_gae as f64 / n;
        native += c.native_ops as f64 / n;
        solver += c.solver_ops as f64 / n;
        for (k, v) in &c.other_native {
            *other.entry(k.clone()).or_insert(0) += v;
        }
    }
    Ok((
        Band {
            weights: BAND_WEIGHTS.to_vec(),
            s,
            group_gae: group,
            native_ops: native,
            solver_ops: solver,
        },
        other,
    ))
}

/// Substrings of a `bench` error that name a malformed configuration rather
/// than an instance or model that cannot represent it.
const MALFORMED: &[&str] = &[
    "stray token",
    "unknown ",
    "needs a ",
    "needs --",
    "malformed",
    "not valid",
    "invalid ",
    "expected ",
];

fn error_class(message: &str) -> Class {
    if MALFORMED.iter().any(|m| message.contains(m)) {
        Class::InvalidConfig
    } else {
        Class::Refused
    }
}

fn mean(xs: impl Iterator<Item = f64>) -> Option<f64> {
    let (sum, n) = xs.fold((0.0, 0usize), |(s, n), x| (s + x, n + 1));
    (n > 0).then(|| sum / n as f64)
}

/// Reduce an `ic bench` report (the JSON `bench::run` returns) to a record.
pub fn record_from_report(config: &TrialConfig, report: &Value) -> TrialRecord {
    let rows: Vec<&Value> = report
        .get("rows")
        .and_then(Value::as_array)
        .map(|a| a.iter().collect())
        .unwrap_or_default();
    let skipped = report
        .get("configurations_skipped")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut record = TrialRecord {
        schema: SCHEMA.into(),
        class: Class::Crashed,
        detail: String::new(),
        config: config.clone(),
        instance: report
            .get("instance")
            .and_then(Value::as_str)
            .map(str::to_string),
        log2_r: rows
            .first()
            .and_then(|r| r.get("log2_r"))
            .and_then(Value::as_f64),
        repeats_run: rows.len(),
        repeats_verified: 0,
        band: None,
        context: Value::Null,
    };
    if rows.is_empty() {
        match skipped.first() {
            Some(s) => {
                let why = s["why"].as_str().unwrap_or("not run").to_string();
                record.class = error_class(&why);
                record.detail = why;
            }
            None => record.detail = "the run produced no rows".into(),
        }
        return record;
    }
    let verified: Vec<&Value> = rows
        .iter()
        .copied()
        .filter(|r| r.get("verified") == Some(&Value::Bool(true)))
        .collect();
    record.repeats_verified = verified.len();
    // A row that is not verified but did recover something recovered the
    // wrong logarithm: that is a defect, whatever else the run did.
    let wrong = rows.iter().any(|r| {
        r.get("verified") != Some(&Value::Bool(true))
            && r.get("exhausted") == Some(&Value::Bool(false))
    });
    let mut other = BTreeMap::new();
    if wrong {
        record.class = Class::Invalid;
        record.detail = "a repeat recovered a logarithm that is not the planted one".into();
    } else if verified.len() < rows.len() {
        record.class = Class::Exhausted;
        record.detail = format!(
            "{} of {} repeats recovered the logarithm within max_trials",
            verified.len(),
            rows.len()
        );
    } else {
        match band_of(&verified) {
            Ok((band, o)) => {
                record.class = Class::Ok;
                record.band = Some(band);
                other = o;
            }
            Err(e) => {
                record.class = Class::Crashed;
                record.detail = e;
            }
        }
    }
    let solver = verified
        .iter()
        .find_map(|r| r.pointer("/decomposition/solver").filter(|s| s.is_object()));
    let band_s0 = record.band.as_ref().and_then(|b| b.s.first().copied());
    let rho_s = report
        .pointer("/rho_reference/mean_s")
        .and_then(Value::as_f64)
        .filter(|_| report.pointer("/rho_reference/all_verified") == Some(&Value::Bool(true)));
    record.context = json!({
        "regime": report.get("regime"),
        "group_order": rows[0].get("group_order"),
        "columns": mean(rows.iter().filter_map(|r| r.pointer("/factor_base/columns").and_then(Value::as_f64))),
        "signed_points": mean(rows.iter().filter_map(|r| r.pointer("/factor_base/signed_points").and_then(Value::as_f64))),
        "hit_rate": mean(rows.iter().filter_map(|r| r.pointer("/decomposition/hit_rate").and_then(Value::as_f64))),
        "targets_tried": mean(rows.iter().filter_map(|r| r.pointer("/decomposition/targets_tried").and_then(Value::as_f64))),
        "label": rows[0].get("label"),
        "solver": solver.map(|s| json!({
            "name": s.get("name"),
            "op_unit": s.get("op_unit"),
            "calls": s.get("calls"),
            "priced_by": s.get("priced_by"),
            "budget_exceeded": s.get("budget_exceeded"),
            "solving_degree_mean": s.get("solving_degree_mean"),
            "semi_regular_degree": s.get("semi_regular_degree"),
        })),
        "other_native_unscored": other,
        // Informational only: the framework's own S is host-calibrated and
        // is NOT what the sweeper scores.
        "framework_s_mean": mean(verified.iter().filter_map(|r| r.get("s").and_then(Value::as_f64))),
        "rho_mean_s": rho_s,
        // Group operations on both sides, native work on neither: a ratio of
        // two partial counts, not a bound on either method's cost.
        "w0_group_ops_over_rho_group_ops": band_s0.zip(rho_s).map(|(s, rho)| s / rho),
        "calibration_pinned_units": report.pointer("/calibration/pinned_units"),
        "software_note": "score is a function of operation counts; wall time is not used",
    });
    record
}

/// Run one configuration and reduce it to a record.  Never panics and never
/// returns without a class.
pub fn run_trial(config: &TrialConfig) -> TrialRecord {
    let failed = |class: Class, detail: String| TrialRecord {
        schema: SCHEMA.into(),
        class,
        detail,
        config: config.clone(),
        instance: None,
        log2_r: None,
        repeats_run: 0,
        repeats_verified: 0,
        band: None,
        context: Value::Null,
    };
    let args = match config.bench_args() {
        Ok(a) => a,
        Err(e) => return failed(Class::InvalidConfig, e),
    };
    let outcome = std::panic::catch_unwind(AssertUnwindSafe(|| bench::run(args, true)));
    match outcome {
        Ok(Ok(report)) => record_from_report(config, &report),
        Ok(Err(message)) => failed(error_class(&message), message),
        Err(payload) => {
            let message = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "panic with a non-string payload".into());
            failed(Class::Crashed, format!("panicked: {message}"))
        }
    }
}

/// This process's peak resident memory in MiB.  `ru_maxrss` is bytes on
/// macOS and KiB elsewhere.
fn peak_rss_mb() -> u64 {
    // SAFETY: `rusage` is plain old data and `getrusage` only writes to it.
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut usage) } != 0 {
        return 0;
    }
    let raw = usage.ru_maxrss.max(0) as u64;
    if cfg!(target_os = "macos") {
        raw / (1024 * 1024)
    } else {
        raw / 1024
    }
}

fn envelope(record: &TrialRecord) -> Value {
    json!({
        "schema_version": 1,
        "operation": "sweep-trial",
        "status": "complete",
        "trial": record,
    })
}

/// Stops a runaway trial from inside, so the limits hold on every host
/// (`ps` is not always available and macOS does not enforce `RLIMIT_AS`).
/// The record it prints is the same shape the normal path prints, class
/// `exhausted`: a budget was hit, which is not "no solution" and not a score.
fn arm_watchdog(config: &TrialConfig, timeout_seconds: u64, max_rss_mb: u64) {
    if timeout_seconds == 0 && max_rss_mb == 0 {
        return;
    }
    let config = config.clone();
    let started = std::time::Instant::now();
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let why = if timeout_seconds > 0 && started.elapsed().as_secs() >= timeout_seconds {
            Some(format!("wall-clock limit of {timeout_seconds} s reached"))
        } else if max_rss_mb > 0 && peak_rss_mb() >= max_rss_mb {
            Some(format!("peak memory passed {max_rss_mb} MiB"))
        } else {
            None
        };
        if let Some(detail) = why {
            let record = TrialRecord {
                schema: SCHEMA.into(),
                class: Class::Exhausted,
                detail,
                config,
                instance: None,
                log2_r: None,
                repeats_run: 0,
                repeats_verified: 0,
                band: None,
                context: json!({"watchdog": true}),
            };
            println!("{}", envelope(&record));
            std::process::exit(0);
        }
    });
}

/// The `ic sweep-trial` command.
pub fn run(args: TrialArgs) -> Result<Value, String> {
    let config = TrialConfig::load(&args)?;
    arm_watchdog(&config, args.timeout_seconds, args.max_rss_mb);
    let record = run_trial(&config);
    Ok(envelope(&record))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn phase(adds: u64, doubles: u64, native: Value) -> Value {
        json!({"wall_ns": 7, "gae": 1234.5, "group_ops": {"adds": adds, "doubles": doubles, "scalar_mults": 3}, "native": native})
    }

    /// A hand-made `RunReport` row: every figure chosen so the expected band
    /// can be written down by hand.
    fn row(verified: bool, exhausted: bool) -> Value {
        json!({
            "instance": "toy", "log2_r": 8.0, "r": 256, "group_order": 512,
            "verified": verified, "exhausted": exhausted,
            "s": 99.0, "total_gae": 9999.0,
            "factor_base": {"columns": 10, "signed_points": 40,
                "cost": phase(100, 20, json!({"sqrt_solves": 5, "abscissae_scanned": 777}))},
            "decomposition": {"hit_rate": 0.25, "targets_tried": 40,
                "setup": phase(10, 0, json!({"lookups": 100})),
                "cost": phase(50, 30, json!({"solver_ops": 1000, "solver_calls": 4, "x_uncharged": 9, "trials": 40})),
                "solver": {"name": "f4", "ops": 1000, "op_unit": "word XORs", "calls": 4, "priced_by": "measured"}},
            "linear_algebra": {"cost": phase(0, 0, json!({"row_ops": 20}))},
            "verify": phase(2, 1, json!({})),
        })
    }

    fn config() -> TrialConfig {
        serde_json::from_value(json!({
            "regime": "koblitz", "degree": 13, "koblitz_a": 0,
            "factor_base": "binary-subspace:dimension=4", "oracle": "descent-algebraic:m=2",
            "solver": "f4-f2"
        }))
        .unwrap()
    }

    fn report(rows: Vec<Value>) -> Value {
        json!({"instance": "toy", "regime": "koblitz", "rows": rows, "configurations_skipped": [],
               "rho_reference": {"mean_s": 4.0, "all_verified": true}, "calibration": {"pinned_units": ["ns_per_lookup"]}})
    }

    /// The score is group operations plus `w` times native operations, and
    /// nothing from the report's own (host-calibrated) GAE or S.
    #[test]
    fn the_band_is_group_ops_plus_w_times_native_ops() {
        let rec = record_from_report(&config(), &report(vec![row(true, false)]));
        assert_eq!(rec.class, Class::Ok, "{}", rec.detail);
        let band = rec.band.unwrap();
        // group: (100+20) + (10+0) + (50+30) + (0+0) + (2+1) = 213
        assert_eq!(band.group_gae, 213.0);
        // native: sqrt 5 + lookups 100 + row_ops 20 + x_uncharged 9
        //         + solver ops 1000 (solver_ops/solver_calls are not re-counted)
        assert_eq!(band.native_ops, 5.0 + 100.0 + 20.0 + 9.0 + 1000.0);
        assert_eq!(band.solver_ops, 1000.0);
        let sqrt_r = 16.0;
        assert_eq!(band.s[0], 213.0 / sqrt_r);
        assert_eq!(band.s[1], (213.0 + 0.1 * 1134.0) / sqrt_r);
        assert_eq!(band.s[2], (213.0 + 1134.0) / sqrt_r);
        // Counters nobody prices are visible, not free and not scored.
        let other = &rec.context["other_native_unscored"];
        assert_eq!(other["abscissae_scanned"], 777);
        assert_eq!(other["trials"], 40);
        assert!(other.get("solver_ops").is_none());
    }

    #[test]
    fn host_timing_in_the_report_cannot_move_the_score() {
        let mut a = row(true, false);
        let mut b = row(true, false);
        a["s"] = json!(1.0);
        b["s"] = json!(1.0e9);
        b["factor_base"]["cost"]["gae"] = json!(1.0e12);
        b["factor_base"]["cost"]["wall_ns"] = json!(123456789u64);
        b["decomposition"]["solver"]["priced_by"] = json!("unpriced");
        let ra = record_from_report(&config(), &report(vec![a]));
        let rb = record_from_report(&config(), &report(vec![b]));
        assert_eq!(ra.band, rb.band);
    }

    #[test]
    fn repeats_are_averaged() {
        let mut heavy = row(true, false);
        heavy["decomposition"]["solver"]["ops"] = json!(3000);
        let rec = record_from_report(&config(), &report(vec![row(true, false), heavy]));
        assert_eq!(rec.class, Class::Ok);
        assert_eq!(rec.band.unwrap().solver_ops, 2000.0);
        assert_eq!(rec.repeats_verified, 2);
    }

    #[test]
    fn an_unverified_repeat_is_never_scored() {
        let exhausted =
            record_from_report(&config(), &report(vec![row(true, false), row(false, true)]));
        assert_eq!(exhausted.class, Class::Exhausted);
        assert!(exhausted.band.is_none());
        // Recovered the wrong logarithm: a defect, not "gave up".
        let wrong = record_from_report(&config(), &report(vec![row(false, false)]));
        assert_eq!(wrong.class, Class::Invalid);
        assert!(wrong.band.is_none());
    }

    #[test]
    fn skipped_and_empty_reports_get_a_class() {
        let skipped = json!({"rows": [], "configurations_skipped": [
            {"configuration": "x", "why": "oracle `foo` is not available here"}]});
        let rec = record_from_report(&config(), &skipped);
        assert_eq!(rec.class, Class::Refused);
        assert!(rec.detail.contains("not available"));
        let malformed = json!({"rows": [], "configurations_skipped": [
            {"configuration": "x", "why": "stray token `2`"}]});
        assert_eq!(
            record_from_report(&config(), &malformed).class,
            Class::InvalidConfig
        );
        let empty = json!({"rows": [], "configurations_skipped": []});
        assert_eq!(record_from_report(&config(), &empty).class, Class::Crashed);
    }

    #[test]
    fn units_are_compared_by_the_repositorys_family_rule() {
        // The F4 family and Gray-code FES are one priced unit, as in
        // `ic_framework::is_word_xor_op_unit`.
        for u in [
            "word XORs",
            "word XORs (elimination only)",
            "word XORs (elimination, specialisation and linear elimination only)",
            "word XORs (Gray-code steps)",
        ] {
            assert_eq!(unit_family(u), "word XORs", "{u}");
        }
        // Everything else is its own unit, and none is a word XOR.
        for u in [
            "monomial operations",
            "conflicts",
            "word operations (partial: extraction, transform and filter; per-point solves in solve_row_ops)",
        ] {
            assert_eq!(unit_family(u), u);
            assert_ne!(unit_family(u), "word XORs");
        }
    }

    #[test]
    fn unknown_config_fields_are_rejected() {
        let bad = r#"{"regime":"prime","degree":14,"factor_base":"a","oracle":"b","solvr":"x"}"#;
        assert!(serde_json::from_str::<TrialConfig>(bad).is_err());
    }

    #[test]
    fn a_wall_clock_budget_is_never_passed_through() {
        let argv = config().argv().unwrap();
        let i = argv
            .iter()
            .position(|a| a == "--solver-budget-seconds")
            .unwrap();
        assert_eq!(argv[i + 1], "0");
        let args = config().bench_args().unwrap();
        assert_eq!(args.solver_budget_seconds, 0);
        assert_eq!(args.koblitz_degree, Some(13));
        assert_eq!(args.koblitz_a, Some(0));
    }

    #[test]
    fn an_unknown_regime_is_a_malformed_configuration() {
        let mut c = config();
        c.regime = "ternary".into();
        let rec = run_trial(&c);
        assert_eq!(rec.class, Class::InvalidConfig);
    }

    /// The real pipeline, end to end, on a prime curve small enough to run
    /// in a test.  The band must be ordered in `w`, verified, and identical
    /// on a second run: the property the optimiser relies on.
    #[test]
    fn a_real_prime_trial_is_verified_and_reproducible() {
        let cfg: TrialConfig = serde_json::from_value(json!({
            "regime": "prime", "degree": 14,
            "factor_base": "prime-abscissa:size=24",
            "oracle": "mitm:m=2,negation_folded=1",
            "rho_runs": 2, "repeats": 2
        }))
        .unwrap();
        let a = run_trial(&cfg);
        assert_eq!(a.class, Class::Ok, "{}", a.detail);
        let band = a.band.clone().unwrap();
        assert!(band.s[0] > 0.0 && band.s[0] <= band.s[1] && band.s[1] <= band.s[2]);
        let b = run_trial(&cfg);
        assert_eq!(
            a.band, b.band,
            "the same configuration must score identically"
        );
    }

    /// The algebraic track on a Koblitz curve: an F4-family solver whose
    /// work is priced in its own unit must still land in the band.
    #[test]
    fn a_real_algebraic_trial_counts_the_solver() {
        let cfg: TrialConfig = serde_json::from_value(json!({
            "regime": "koblitz", "degree": 13, "koblitz_a": 0,
            "factor_base": "binary-subspace:dimension=4",
            "oracle": "descent-algebraic:m=2", "solver": "inherited-f4",
            "rho_runs": 2
        }))
        .unwrap();
        let rec = run_trial(&cfg);
        assert_eq!(
            rec.class,
            Class::Ok,
            "{}: {}",
            rec.class.as_str(),
            rec.detail
        );
        let band = rec.band.unwrap();
        assert!(
            band.solver_ops > 0.0,
            "an algebraic oracle's solver ops are native work"
        );
        assert!(
            band.s[2] > band.s[0],
            "native work must separate w = 1 from w = 0"
        );
    }
}
