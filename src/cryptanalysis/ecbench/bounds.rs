//! Bounds: what a method costs, as a sealed record another session can
//! cite, re-derive and challenge.
//!
//! A *bound* takes one arm of one or more sessions and states, in the
//! harness's unit, the cost of that method on that problem: the constant
//! at the declared exponent (`S = gae / √r`, and the dimensionless ratio
//! to the curve's generic floor `√(π / 2A)`), the exponent fitted across
//! sizes (`gae = C · r^α`, least squares in log–log space, as
//! `ca_bench complexity` fits it in aburan28/cryptanalysis), both with
//! two-stage bootstrap intervals, the share and exponent of every phase,
//! the memory the method stored and the work it counted but did not
//! charge, and the sessions and audit receipts it was read from.
//!
//! A bound is scoped by its *domain* — problem, curve family, target
//! kind, unit, tier, resource envelope — and compares with nothing outside
//! it.  Its id is the SHA-256 of its own bytes (`seal_document`), so a
//! changed bound is a new bound, two sessions fitting the same sessions
//! write the same file, and `bound check` re-derives a committed bound
//! from the sessions it names.  `docs/bounds/README.md` is the protocol;
//! [`frontier`](super::frontier) ranks bounds and
//! [`challenge`](super::challenge) replaces them.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cryptanalysis::ecbench::audit::AuditReport;
use crate::cryptanalysis::ecbench::canonical::{compat_u128, sha256_hex, short_id};
use crate::cryptanalysis::ecbench::methods::expected_s;
use crate::cryptanalysis::ecbench::record::{floor_s, Record};
use crate::cryptanalysis::ecbench::runner::{read_records, read_session, PlanDoc};
use crate::cryptanalysis::ecbench::stats::{bootstrap_ci, cluster_bootstrap_ci, mean};

pub const BOUND_SCHEMA: &str = "ecbench.bound/v1";
pub const BOUND_PREFIX: &str = "ECBND1h";
pub const DOMAIN_PREFIX: &str = "ECDOM1";
/// The problem every ecbench workload poses: one cold target, one
/// subgroup, no precomputation.
pub const PROBLEM: &str = "ecdlp.single_target";
/// Bootstrap resamples and seed, fixed so a bound re-derives bit for bit.
pub const RESAMPLES: usize = 2000;
pub const SEED: u64 = 20261005;
/// A scaling claim (an exponent) needs this many sizes (AGENTS.md §5).
pub const MIN_SIZES_FOR_SCALING: usize = 4;
/// The fit procedure a new bound is written under.  Version 1 is the
/// procedure the first committed records were fitted with; a record that
/// does not name a version is version 1 and re-derives under it.  Version 2
/// (`docs/bounds/README.md` §3a) keeps the same figures but refuses what
/// version 1 let through: degenerate exponent resamples, intervals that
/// exclude their estimate, duplicated draws, runs dropped by a filter,
/// sessions that are not complete or not audited, and a resample count
/// other than [`RESAMPLES`].
pub const FIT_VERSION: u32 = 2;
pub const LEGACY_FIT_VERSION: u32 = 1;
/// A resample is a valid exponent resample only when it holds this many
/// distinct sizes (version 2).
pub const MIN_DISTINCT_SIZES_RESAMPLE: usize = 3;
/// Tail flags: a run this many times the median ratio to the floor, or a
/// single run holding this share of the total (with at least
/// [`TAIL_MIN_RUNS_FOR_SHARE`] runs), marks the mean as dominated by its tail.
pub const TAIL_MAX_OVER_MEDIAN: f64 = 10.0;
pub const TAIL_TOP_SHARE: f64 = 0.25;
pub const TAIL_MIN_RUNS_FOR_SHARE: u64 = 8;
/// The pricing band of AGENTS.md §5a, in GAE per unpriced operation.
pub const BAND_WEIGHTS: [f64; 3] = super::band::DEFAULT_WEIGHTS;
/// The path of an [`AuditRef`] a challenge verdict made of its own session.
pub const VERDICT_AUDIT_PREFIX: &str = "verdict:";

// ── Documents sealed by their bytes ────────────────────────────────

/// Seal a pretty-printed JSON document: `field` is written empty, the
/// text is hashed, and the id (`prefix` + 12 hex) is written in.  Returns
/// the id and the text.  The seal covers bytes, not a parsed value, for
/// the reason `Record::seal` gives: a float parsed and re-serialised can
/// differ in a last digit from what was written.
pub fn seal_document<T: Serialize>(
    doc: &T,
    field: &str,
    prefix: &str,
) -> Result<(String, String), String> {
    let mut v = serde_json::to_value(doc).map_err(|e| e.to_string())?;
    let obj = v
        .as_object_mut()
        .ok_or("a sealed document is a JSON object")?;
    if !obj.contains_key(field) {
        return Err(format!("document has no `{field}` field to seal"));
    }
    obj.insert(field.to_string(), Value::String(String::new()));
    // Key order: the struct's own order is lost in a `Value`; the text is
    // what is hashed, so whatever order it has is the order that counts.
    let blank = serde_json::to_string_pretty(&v).map_err(|e| e.to_string())? + "\n";
    let needle = format!("\"{field}\": \"\"");
    if blank.matches(&needle).count() != 1 {
        return Err(format!(
            "the `{field}` field must appear exactly once in the document"
        ));
    }
    let id = format!("{prefix}{}", &sha256_hex(blank.as_bytes())[..12]);
    let text = blank.replacen(&needle, &format!("\"{field}\": \"{id}\""), 1);
    Ok((id, text))
}

/// Whether `text` hashes to the id it carries in `field`; the id itself.
pub fn check_document_seal(text: &str, field: &str, prefix: &str) -> Result<String, String> {
    let v: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let id = v
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("no `{field}`"))?
        .to_string();
    if !id.starts_with(prefix) {
        return Err(format!("`{field}` {id} does not start with {prefix}"));
    }
    let filled = format!("\"{field}\": \"{id}\"");
    if text.matches(&filled).count() != 1 {
        return Err(format!("`{field}` must appear exactly once as written"));
    }
    let blank = text.replacen(&filled, &format!("\"{field}\": \"\""), 1);
    let expect = format!("{prefix}{}", &sha256_hex(blank.as_bytes())[..12]);
    if expect != id {
        return Err(format!(
            "seal mismatch: bytes hash to {expect}, document says {id}"
        ));
    }
    Ok(id)
}

/// Read and parse a sealed document, checking its seal first.
pub fn read_sealed<T: DeserializeOwned>(
    path: &Path,
    field: &str,
    prefix: &str,
) -> Result<T, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    check_document_seal(&text, field, prefix).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

// ── The domain ─────────────────────────────────────────────────────

/// The resource envelope a bound was measured in.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Envelope {
    pub targets: u32,
    /// `none`: every run starts cold.
    pub precomputation: String,
    pub threads: u32,
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            targets: 1,
            precomputation: "none".into(),
            threads: 1,
        }
    }
}

/// What a bound is about.  Bounds compare only inside one domain.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Domain {
    pub problem: String,
    /// `prime`, `koblitz` or `binary`: the automorphism group, and so the
    /// floor, differs.
    pub family: String,
    /// `planted` or `public`.
    pub target_kind: String,
    pub unit: String,
    /// `toy`, `medium` or `crypto`, from the field size.
    pub tier: String,
    pub envelope: Envelope,
}

impl Domain {
    /// `ECDOM1h` + 12 hex of the canonical domain.
    pub fn id(&self) -> Result<String, String> {
        let v = serde_json::to_value(self).map_err(|e| e.to_string())?;
        Ok(short_id(DOMAIN_PREFIX, &v)?.0)
    }
}

/// The claim tier a field size contributes to (crypto-autoresearcher,
/// `docs/claims-and-verification.md`): `toy` up to 32 bits, `medium` up
/// to 96, `crypto` above.
pub fn tier_of_bits(field_bits: u32) -> &'static str {
    if field_bits <= 32 {
        "toy"
    } else if field_bits <= 96 {
        "medium"
    } else {
        "crypto"
    }
}

/// The field size of a record's curve: bits of `p`, or the degree over
/// `F_2`.
pub fn field_size(r: &Record) -> Option<u32> {
    let c = &r.workload.curve;
    if c.family == "prime" {
        c.field_bits
    } else {
        c.field_degree
    }
}

// ── The record ─────────────────────────────────────────────────────

/// A statistic with its interval.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Estimate {
    pub value: f64,
    pub ci95: Option<(f64, f64)>,
    /// `cluster` (sizes, then runs), `within` (runs of one size) or `none`.
    pub ci_method: String,
}

/// The method a bound is about: the configuration (`ECM1h…`), never the
/// code, whose hash is in the provenance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodRef {
    pub id: String,
    pub params: BTreeMap<String, String>,
    pub method_id: String,
    pub family: String,
    pub entry: String,
}

/// One curve size.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SizeRow {
    pub slug: String,
    pub log2_r: f64,
    #[serde(with = "compat_u128")]
    pub r: u128,
    pub field_bits: u32,
    pub automorphisms_available: u32,
    pub floor_s: f64,
    pub workloads: u64,
    /// Measured runs of the arm on this curve, and those that verified.
    pub runs: u64,
    pub verified: u64,
    pub mean_gae: Option<f64>,
    pub mean_s: Option<f64>,
    pub s_ci95: Option<(f64, f64)>,
    pub ratio_to_floor: Option<f64>,
    /// `expected_s` of the method on this curve, and its ratio to the
    /// floor, where the registry derives one.
    pub declared_s: Option<f64>,
    pub declared_ratio_to_floor: Option<f64>,
    pub memory_entries_per_sqrt_r: Option<f64>,
    pub uncharged_per_sqrt_r: Option<f64>,
    pub levels: BTreeMap<String, u64>,
    /// Fit version 2: the other axes on this size alone, each with a
    /// two-stage interval over the size's workloads, so two bounds can be
    /// compared on the sizes they share.  Keys are `memory`, `uncharged`
    /// and the field-operation axes the size's runs all counted.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub axes: BTreeMap<String, SizeAxis>,
    /// Fit version 2: the band-priced `S` on this size at each weight of
    /// [`BAND_WEIGHTS`] (AGENTS.md §5a), where every verified run can be
    /// priced.  Empty for a bound fitted at a weight.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub band: Vec<BandPoint>,
}

/// One axis on one size.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SizeAxis {
    pub value: f64,
    pub ci95: Option<(f64, f64)>,
}

/// Mean `S` on one size with unpriced work charged at `weight` GAE per
/// native operation; `s` is `None` when some verified run names an
/// unpriced counter without a count (it is a lower bound above `w = 0`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BandPoint {
    pub weight: f64,
    pub s: Option<f64>,
    pub ci95: Option<(f64, f64)>,
}

/// How far the mean is carried by a few runs (fit version 2), over the
/// ratios to the floor the `ops` figure averages.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tail {
    pub runs: u64,
    pub median_ratio: f64,
    pub max_ratio: f64,
    /// Largest ratio over the median.
    pub max_over_median: f64,
    /// The largest run's share of the sum of all runs.
    pub top_run_share: f64,
    pub top_run: String,
    /// `max_over_median` or `top_run_share` crossed its threshold: the
    /// mean is a statement about the tail, and its interval is wide for
    /// that reason.
    pub large: bool,
}

/// `gae = C · r^α`, least squares on every verified run's `(ln r, ln gae)`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fit {
    pub size_parameter: String,
    pub model: String,
    pub points: u64,
    pub sizes: u64,
    pub alpha: Option<f64>,
    pub alpha_ci95: Option<(f64, f64)>,
    pub log2_c: Option<f64>,
    pub r_squared: Option<f64>,
    pub ci_method: String,
    /// `0.5` for every method the registry derives an expectation for.
    pub declared_alpha: Option<f64>,
    /// The declared exponent lies inside the fitted interval.
    pub alpha_agrees_with_declared: Option<bool>,
    /// At least [`MIN_SIZES_FOR_SCALING`] sizes: the exponent is a result,
    /// not a description.
    pub scaling_claim: bool,
}

/// The constant at the declared exponent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Constant {
    /// Mean `S = gae / √r` over verified runs.
    pub s: Estimate,
    /// Mean `S / √(π / 2A)`: comparable across curve families.
    pub ratio_to_floor: Estimate,
    /// The method's derived ratio to the floor when it is the same on
    /// every size (rho.negation: 1; bsgs.textbook: 1.5 / √(π/4)).
    pub declared_ratio_to_floor: Option<f64>,
}

/// One cost axis.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    pub statistic: String,
    pub source: String,
    pub lower_is_better: bool,
    /// Every verified run reported it.  Unknown is not zero.
    pub known: bool,
    pub value: Option<f64>,
    pub ci95: Option<(f64, f64)>,
}

/// One phase of the method: its share of the total and its own exponent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StageRow {
    pub name: String,
    pub mean_gae: f64,
    /// `Σ phase gae / Σ total gae` over verified runs.
    pub share_of_gae: f64,
    pub sizes_with_work: u64,
    pub alpha: Option<f64>,
    pub alpha_ci95: Option<(f64, f64)>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionRef {
    pub dir: String,
    pub session_id: String,
    pub spec_id: String,
    pub status: String,
    pub arm: String,
    pub role: String,
    pub binary_sha256: Option<String>,
    pub git_commit: Option<String>,
    pub env_class_id: String,
    pub records_sha256: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditRef {
    pub path: String,
    pub sha256: String,
    pub session_id: String,
    pub ok: bool,
    pub replays: u64,
    pub replays_reproduced: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Provenance {
    pub sessions: Vec<SessionRef>,
    pub audits: Vec<AuditRef>,
    pub records: u64,
    pub verified: u64,
    pub statuses: BTreeMap<String, u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Admissibility {
    /// `admissible`, or `inadmissible` (a measured run did not verify).
    pub status: String,
    /// Some work was counted and not priced: every total is a floor.
    pub bounded: bool,
    pub unpriced: Vec<String>,
    pub deterministic: bool,
    pub reasons: Vec<String>,
}

/// How the bound was fitted, so `bound check` can fit it again.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FitOptions {
    pub tier: Option<String>,
    /// Curve slugs kept; empty keeps every curve of the arm.
    pub curves: Vec<String>,
    pub resamples: usize,
    pub seed: u64,
    /// Fit the band-priced cost at this weight (GAE per native operation,
    /// AGENTS.md §5a, [`super::band`]) instead of the charged cost.  The
    /// domain's unit then names the weight, so the bound never shares an
    /// id with a charged-cost bound.  Omitted from the record when unset, so
    /// a bound fitted without it keeps the id it had before the field existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    /// Accept an arm whose parameters change from size to size under a
    /// declared rule (for example a factor-base size chosen per size on a
    /// train panel).  Every session must run the same method id with the
    /// same parameter names; parameters whose values differ are recorded
    /// as `per size` and the rule text becomes part of the bound's method
    /// identity, so the bound never shares an id with a fixed-parameter one.
    /// Omitted from the record when unset (see `weight`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_rule: Option<String>,
    /// The fit procedure ([`FIT_VERSION`]).  A record that predates the
    /// field is version 1 and is not written with it, so its seal stands.
    #[serde(default = "legacy_fit_version", skip_serializing_if = "is_legacy_fit")]
    pub fit_version: u32,
}

fn legacy_fit_version() -> u32 {
    LEGACY_FIT_VERSION
}

fn is_legacy_fit(v: &u32) -> bool {
    *v == LEGACY_FIT_VERSION
}

impl Default for FitOptions {
    fn default() -> Self {
        Self {
            tier: None,
            curves: vec![],
            resamples: RESAMPLES,
            seed: SEED,
            weight: None,
            param_rule: None,
            fit_version: FIT_VERSION,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bound {
    pub schema: String,
    /// `ECBND1h` + 12 hex of this document with the field empty.
    pub bound_id: String,
    pub label: String,
    pub domain: Domain,
    pub domain_id: String,
    pub method: MethodRef,
    /// `exponent` (a scaling claim) or `constant`.
    pub level: String,
    pub sizes: Vec<SizeRow>,
    pub fit: Fit,
    pub constant: Constant,
    pub dimensions: BTreeMap<String, Dimension>,
    pub stages: Vec<StageRow>,
    pub provenance: Provenance,
    pub admissibility: Admissibility,
    pub fit_options: FitOptions,
    /// Bounds this one was measured against and beat or traded with, set
    /// by the verdict that produced it.
    pub improves_on: Vec<String>,
    pub verdict_id: Option<String>,
    pub notes: String,
    /// Fit version 2: tail diagnostics of the `ops` figure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tail: Option<Tail>,
}

impl Bound {
    /// Seal: compute `bound_id` and return the text to write.
    pub fn seal(&mut self) -> Result<String, String> {
        self.bound_id = String::new();
        let (id, text) = seal_document(self, "bound_id", BOUND_PREFIX)?;
        self.bound_id = id;
        Ok(text)
    }

    pub fn read(path: &Path) -> Result<Self, String> {
        let b: Bound = read_sealed(path, "bound_id", BOUND_PREFIX)?;
        if b.schema != BOUND_SCHEMA {
            return Err(format!(
                "{}: schema `{}`, expected {BOUND_SCHEMA}",
                path.display(),
                b.schema
            ));
        }
        Ok(b)
    }

    /// The ops axis: mean ratio to the floor.
    pub fn ops(&self) -> Option<&Dimension> {
        self.dimensions.get("ops")
    }

    pub fn admissible(&self) -> bool {
        self.admissibility.status == "admissible"
    }

    /// The fit procedure this bound was written under.
    pub fn fit_version(&self) -> u32 {
        self.fit_options.fit_version
    }

    /// Why this bound may not be ranked on a frontier even though it is
    /// admissible (README §4): fewer than [`MIN_SIZES_FOR_SCALING`] sizes,
    /// or a fit restricted to some curves or one tier of a session.  Empty
    /// when it may be ranked.  Version 1 bounds are legacy and unranked.
    pub fn frontier_ineligible(&self) -> Vec<String> {
        let mut why = Vec::new();
        if self.fit_version() < FIT_VERSION {
            why.push(format!(
                "historical: fit version {}, kept as published and re-derivable; ranking uses fit-version-2 bounds (its -v2 twin where one was fitted)",
                self.fit_version()
            ));
        }
        let n = self.sizes.iter().filter(|s| s.verified > 0).count();
        if n < MIN_SIZES_FOR_SCALING {
            why.push(format!(
                "{n} size(s): a frontier entry needs at least {MIN_SIZES_FOR_SCALING}"
            ));
        }
        if !self.fit_options.curves.is_empty() {
            why.push("fitted on a subset of the session's curves (--curve)".into());
        }
        if self.fit_options.tier.is_some() {
            why.push("fitted on one tier of a session (--tier), not its full curve set".into());
        }
        why
    }
}

// ── Fitting ────────────────────────────────────────────────────────

/// Least squares of `y = alpha · x + b` over `(x, y)`: `(alpha, b, R²)`.
/// `None` below two distinct `x`.
pub fn ols(points: &[(f64, f64)]) -> Option<(f64, f64, f64)> {
    let n = points.len() as f64;
    if points.len() < 2 {
        return None;
    }
    let (mut sx, mut sy, mut sxx, mut sxy) = (0.0, 0.0, 0.0, 0.0);
    for (x, y) in points {
        sx += x;
        sy += y;
        sxx += x * x;
        sxy += x * y;
    }
    let denom = n * sxx - sx * sx;
    if denom.abs() < 1e-12 {
        return None;
    }
    let alpha = (n * sxy - sx * sy) / denom;
    let b = (sy - alpha * sx) / n;
    let ybar = sy / n;
    let (mut ss_res, mut ss_tot) = (0.0, 0.0);
    for (x, y) in points {
        let yh = alpha * x + b;
        ss_res += (y - yh) * (y - yh);
        ss_tot += (y - ybar) * (y - ybar);
    }
    let r2 = if ss_tot > 0.0 {
        1.0 - ss_res / ss_tot
    } else {
        1.0
    };
    Some((alpha, b, r2))
}

/// `gae = C · r^α` over strata of `(ln r, ln gae)` points, one stratum per
/// size: the slope, its two-stage interval (sizes, then runs), the
/// intercept as `log2 C`, and R².
pub fn power_law(
    strata: &[Vec<(f64, f64)>],
    resamples: usize,
    seed: u64,
) -> (
    Option<f64>,
    Option<(f64, f64)>,
    Option<f64>,
    Option<f64>,
    String,
) {
    let all: Vec<(f64, f64)> = strata.iter().flatten().copied().collect();
    let Some((alpha, b, r2)) = ols(&all) else {
        return (None, None, None, None, "none".into());
    };
    let slope = |s: &[Vec<(f64, f64)>]| {
        let pts: Vec<(f64, f64)> = s.iter().flatten().copied().collect();
        ols(&pts).map(|(a, _, _)| a)
    };
    let nonempty: Vec<Vec<(f64, f64)>> = strata.iter().filter(|s| !s.is_empty()).cloned().collect();
    let (ci, method) = if nonempty.len() >= 2 {
        (
            cluster_bootstrap_ci(&nonempty, resamples, seed, slope),
            "cluster".to_string(),
        )
    } else {
        (None, "none".to_string())
    };
    (
        Some(alpha),
        ci,
        Some(b / std::f64::consts::LN_2),
        Some(r2),
        method,
    )
}

/// Number of distinct `x` values, two being equal when they differ by no
/// more than a relative `1e-9` (an absolute tolerance on `ln r` or on any
/// scale-dependent sum cannot tell a degenerate resample from a real one).
pub fn distinct_x(points: &[(f64, f64)]) -> usize {
    let mut xs: Vec<f64> = points.iter().map(|p| p.0).collect();
    xs.sort_by(f64::total_cmp);
    let mut n = 0;
    let mut last: Option<f64> = None;
    for x in xs {
        match last {
            Some(l) if (x - l).abs() <= 1e-9 * l.abs().max(1.0) => {}
            _ => {
                n += 1;
                last = Some(x);
            }
        }
    }
    n
}

/// [`ols`] for fit version 2: centred sums, and `None` below two distinct
/// `x` by a relative tolerance ([`distinct_x`]) instead of an absolute
/// threshold on `n·Σx² − (Σx)²`.
pub fn ols_v2(points: &[(f64, f64)]) -> Option<(f64, f64, f64)> {
    if points.len() < 2 || distinct_x(points) < 2 {
        return None;
    }
    let n = points.len() as f64;
    let xbar = points.iter().map(|p| p.0).sum::<f64>() / n;
    let ybar = points.iter().map(|p| p.1).sum::<f64>() / n;
    let (mut sxx, mut sxy, mut syy) = (0.0, 0.0, 0.0);
    for (x, y) in points {
        sxx += (x - xbar) * (x - xbar);
        sxy += (x - xbar) * (y - ybar);
        syy += (y - ybar) * (y - ybar);
    }
    if sxx <= 0.0 || !sxx.is_finite() {
        return None;
    }
    let alpha = sxy / sxx;
    let b = ybar - alpha * xbar;
    let r2 = if syy > 0.0 {
        1.0 - (syy - alpha * sxy) / syy
    } else {
        1.0
    };
    Some((alpha, b, r2))
}

/// Percentile-bootstrap values of `stat` over strata that draw until
/// `resamples` draws have been *accepted* by `accept` (the rejecting
/// bootstrap of fit version 2: a draw `accept` refuses is discarded and
/// drawn again, never counted).  `clusters` also resamples the strata.
/// `None` when a stratum is empty or acceptance is too rare to fill
/// `resamples` in `50 · resamples` draws.
pub fn bootstrap_values<T: Clone>(
    strata: &[Vec<T>],
    resamples: usize,
    seed: u64,
    clusters: bool,
    accept: impl Fn(&[Vec<T>]) -> bool,
    stat: impl Fn(&[Vec<T>]) -> Option<f64>,
) -> Option<Vec<f64>> {
    if strata.is_empty() || strata.iter().any(|s| s.is_empty()) || resamples == 0 {
        return None;
    }
    let mut rng = crate::cryptanalysis::ecbench::stats::Resampler::new(seed);
    let mut values = Vec::with_capacity(resamples);
    let mut draws = 0usize;
    while values.len() < resamples && draws < resamples.saturating_mul(50) {
        draws += 1;
        let picked: Vec<&Vec<T>> = if clusters {
            (0..strata.len())
                .map(|_| &strata[rng.below(strata.len())])
                .collect()
        } else {
            strata.iter().collect()
        };
        let draw: Vec<Vec<T>> = picked
            .iter()
            .map(|s| {
                (0..s.len())
                    .map(|_| s[rng.below(s.len())].clone())
                    .collect()
            })
            .collect();
        if !accept(&draw) {
            continue;
        }
        if let Some(v) = stat(&draw) {
            if v.is_finite() {
                values.push(v);
            }
        }
    }
    (values.len() == resamples).then_some(values)
}

/// The `(2.5 %, 97.5 %)` interval of [`bootstrap_values`].
pub fn bootstrap_interval(values: &[f64]) -> Option<(f64, f64)> {
    use crate::cryptanalysis::ecbench::stats::quantile;
    Some((quantile(values, 0.025)?, quantile(values, 0.975)?))
}

/// [`power_law`] for fit version 2.  The slope needs two distinct sizes;
/// its interval needs [`MIN_SIZES_FOR_SCALING`] and is the percentile
/// interval of resamples that hold at least [`MIN_DISTINCT_SIZES_RESAMPLE`]
/// distinct sizes (the others are discarded and redrawn); and an interval
/// that excludes the slope it brackets is refused.
pub fn power_law_v2(
    strata: &[Vec<(f64, f64)>],
    resamples: usize,
    seed: u64,
) -> (
    Option<f64>,
    Option<(f64, f64)>,
    Option<f64>,
    Option<f64>,
    String,
) {
    let all: Vec<(f64, f64)> = strata.iter().flatten().copied().collect();
    let Some((alpha, b, r2)) = ols_v2(&all) else {
        return (None, None, None, None, "none".into());
    };
    let nonempty: Vec<Vec<(f64, f64)>> = strata.iter().filter(|s| !s.is_empty()).cloned().collect();
    let slope = |s: &[Vec<(f64, f64)>]| {
        let pts: Vec<(f64, f64)> = s.iter().flatten().copied().collect();
        ols_v2(&pts).map(|(a, _, _)| a)
    };
    let enough = |s: &[Vec<(f64, f64)>]| {
        let pts: Vec<(f64, f64)> = s.iter().flatten().copied().collect();
        distinct_x(&pts) >= MIN_DISTINCT_SIZES_RESAMPLE
    };
    let ci = if nonempty.len() >= MIN_SIZES_FOR_SCALING {
        bootstrap_values(&nonempty, resamples, seed, true, enough, slope)
            .and_then(|v| bootstrap_interval(&v))
            .filter(|(lo, hi)| *lo <= alpha && alpha <= *hi)
    } else {
        None
    };
    let method = if ci.is_some() { "cluster" } else { "none" };
    (
        Some(alpha),
        ci,
        Some(b / std::f64::consts::LN_2),
        Some(r2),
        method.into(),
    )
}

/// The `ci_method` of an interval fit version 2 refused because it did not
/// contain the estimate it belongs to.
pub const CI_REFUSED: &str = "refused: interval excludes its estimate";

/// Mean over strata with its two-stage interval (within-stratum when there
/// is one stratum), under fit `version`.  From version 2 an interval that does
/// not contain its own point estimate is dropped and named
/// ([`CI_REFUSED`]): such an interval is a bootstrap that failed (one
/// resample, a degenerate stratum), and `frontier::order` must not read it
/// as perfect precision.
fn estimate_v(strata: &[Vec<f64>], resamples: usize, seed: u64, version: u32) -> Option<Estimate> {
    let all: Vec<f64> = strata.iter().flatten().copied().collect();
    let value = mean(&all)?;
    let stat = |s: &[Vec<f64>]| mean(&s.concat());
    let nonempty: Vec<Vec<f64>> = strata.iter().filter(|s| !s.is_empty()).cloned().collect();
    let (mut ci95, mut ci_method) = if nonempty.len() >= 2 {
        (
            cluster_bootstrap_ci(&nonempty, resamples, seed, stat),
            "cluster".to_string(),
        )
    } else {
        (
            bootstrap_ci(&nonempty, resamples, seed, stat),
            "within".to_string(),
        )
    };
    if version >= 2 {
        if let Some((lo, hi)) = ci95 {
            if !(lo <= value && value <= hi) {
                ci95 = None;
                ci_method = CI_REFUSED.to_string();
            }
        }
    }
    Some(Estimate {
        value,
        ci95,
        ci_method,
    })
}

/// The counters a method reports its stored table through, in the order
/// they are read: BSGS, the kangaroo and the claw count `inserts_uncharged`;
/// the strong Koblitz rho counts `table_inserts_uncharged` and reports
/// `table_entries`; the tuned rho walks store one distinguished point per
/// walk (`distinguished_points`).
pub const MEMORY_COUNTERS: &[&str] = &[
    "inserts_uncharged",
    "table_inserts_uncharged",
    "table_entries",
    "distinguished_points",
];

/// Table entries a run stored, as a multiple of `√r`, from the first of
/// [`MEMORY_COUNTERS`] the record carries.  `None` when it carries none:
/// unknown, not zero.
pub fn memory_entries(r: &Record) -> Option<f64> {
    let entries = MEMORY_COUNTERS
        .iter()
        .find_map(|k| r.counters.get(*k))
        .copied()?;
    Some(entries as f64 / (r.workload.curve.r as f64).sqrt())
}

/// Every counter the unit does not charge, summed, per `√r`.
pub fn uncharged_work(r: &Record) -> f64 {
    // Top-level `*_uncharged` counters only.  An index-calculus record keeps
    // most of its unpriced counts in its phases' native maps, so this axis
    // undercounts IC unpriced work; it is kept as it is because committed
    // bound ids depend on it.  Use `ecbench band` / `bound fit --weight`
    // (super::band, AGENTS.md §5a) for a complete count.
    let total: u64 = r
        .counters
        .iter()
        .filter(|(k, _)| k.ends_with("_uncharged"))
        .map(|(_, v)| *v)
        .sum();
    total as f64 / (r.workload.curve.r as f64).sqrt()
}

/// The primitive-level axes: modular multiplications, squarings and
/// inversions behind the group operations, per `√r`, read from a record's
/// `field_ops` block.  A bound carries them only when every verified run
/// of the arm reports the block (prime-field curves through the generic
/// walks and tables); a verdict reports them always and lets them decide
/// only when the challenge names them.
pub const FIELD_AXES: &[&str] = &["field_muls", "field_sqrs", "field_invs"];

/// What a field axis counts, for the record's `statistic` and `source`.
pub fn field_axis_describes(axis: &str) -> Option<(&'static str, &'static str)> {
    match axis {
        "field_muls" => Some(("field multiplications", "field_ops.muls")),
        "field_sqrs" => Some(("field squarings", "field_ops.sqrs")),
        "field_invs" => Some(("field inversions", "field_ops.invs")),
        _ => None,
    }
}

/// One field axis of a run as a multiple of `√r`, or `None` when the
/// record carries no field-operation block (unknown, not zero) or `axis`
/// is not one of [`FIELD_AXES`].
pub fn field_axis(r: &Record, axis: &str) -> Option<f64> {
    let f = r.field_ops.as_ref()?;
    let count = match axis {
        "field_muls" => f.muls,
        "field_sqrs" => f.sqrs,
        "field_invs" => f.invs,
        _ => return None,
    };
    Some(count as f64 / (r.workload.curve.r as f64).sqrt())
}

/// What `bound fit` reads besides the sessions.
pub struct FitInputs<'a> {
    pub dirs: &'a [PathBuf],
    pub arm: &'a str,
    pub audits: &'a [PathBuf],
    pub label: Option<&'a str>,
    pub notes: &'a str,
    /// Paths in the record are written relative to this root.
    pub root: &'a Path,
    pub options: FitOptions,
    /// Audits already in hand rather than in a receipt file (a challenge
    /// verdict audits its own session); cited as given.
    pub audit_refs: &'a [AuditRef],
}

fn relative(root: &Path, p: &Path) -> String {
    let abs = std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    let root = std::fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
    abs.strip_prefix(&root)
        .map(|r| r.display().to_string())
        .unwrap_or_else(|_| p.display().to_string())
}

fn sha_of_file(p: &Path) -> Result<String, String> {
    let bytes = std::fs::read(p).map_err(|e| format!("{}: {e}", p.display()))?;
    Ok(sha256_hex(&bytes))
}

/// Fit the bound of `arm` from the sessions in `dirs`.
pub fn bound_from_sessions(inputs: &FitInputs) -> Result<Bound, String> {
    if inputs.dirs.is_empty() {
        return Err("a bound needs at least one session".into());
    }
    let opts = &inputs.options;
    let v2 = opts.fit_version >= 2;
    if opts.fit_version == 0 || opts.fit_version > FIT_VERSION {
        return Err(format!(
            "fit version {} is not one this binary knows (1 to {FIT_VERSION})",
            opts.fit_version
        ));
    }
    let mut sessions: Vec<SessionRef> = Vec::new();
    let mut recs: Vec<Record> = Vec::new();
    let mut method: Option<MethodRef> = None;
    // Fit version 2: the same draw must not enter twice.  A session read
    // twice, the same records under two names, and two sessions of one
    // spec (their shared seeds make identical draws) all narrow an interval
    // without adding information.
    let mut seen_draws: BTreeMap<(String, String), String> = BTreeMap::new();
    for dir in inputs.dirs {
        let s = read_session(dir)?;
        if v2 {
            if let Some(prev) = sessions.iter().find(|x| x.session_id == s.session_id) {
                return Err(format!(
                    "session {} ({}) is listed twice (also as {}); a session counts once",
                    s.session_id,
                    dir.display(),
                    prev.dir
                ));
            }
        }
        let plan: PlanDoc = serde_json::from_str(
            &std::fs::read_to_string(dir.join("plan.json"))
                .map_err(|e| format!("{}: {e}", dir.join("plan.json").display()))?,
        )
        .map_err(|e| format!("plan.json: {e}"))?;
        let arm = plan
            .arms
            .iter()
            .find(|a| a.name == inputs.arm)
            .ok_or_else(|| {
                format!(
                    "session {} has no arm `{}` (arms: {})",
                    s.session_id,
                    inputs.arm,
                    plan.arms
                        .iter()
                        .map(|a| a.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })?;
        let m = MethodRef {
            id: arm.method.id.clone(),
            params: arm.method.params.clone(),
            method_id: arm.method.method_id.clone(),
            family: arm.method.family.clone(),
            entry: arm.method.entry.clone(),
        };
        match (&mut method, &inputs.options.param_rule) {
            (None, _) => method = Some(m),
            (Some(prev), Some(rule)) if prev.method_id != m.method_id => {
                let same_keys = prev.params.keys().eq(m.params.keys());
                if prev.id != m.id || !same_keys {
                    return Err(format!(
                        "arm `{}` runs {} in one session and {} in another; a parameter rule \
                         covers one method id with one set of parameter names",
                        inputs.arm, prev.id, m.id
                    ));
                }
                for (k, v) in m.params.iter() {
                    if prev.params.get(k) != Some(v) {
                        prev.params.insert(k.clone(), "per size".into());
                    }
                }
                prev.method_id = format!("{} under rule: {rule}", prev.id);
            }
            (Some(prev), None) if prev.method_id != m.method_id => {
                return Err(format!(
                    "arm `{}` is {} in one session and {} in another; a bound is about one method \
                     (pass --param-rule to fit a size-dependent parameter rule)",
                    inputs.arm, prev.method_id, m.method_id
                ))
            }
            _ => {}
        }
        let role = format!("{:?}", arm.role).to_lowercase();
        let records_sha256 = sha_of_file(&dir.join("records.jsonl"))?;
        if v2 {
            if let Some(prev) = sessions.iter().find(|x| x.records_sha256 == records_sha256) {
                return Err(format!(
                    "{} holds the same records as {} (records_sha256 {}); a copy counts once",
                    dir.display(),
                    prev.dir,
                    records_sha256
                ));
            }
        }
        sessions.push(SessionRef {
            dir: relative(inputs.root, dir),
            session_id: s.session_id.clone(),
            spec_id: s.spec_id.clone(),
            status: s.status.clone(),
            arm: inputs.arm.to_string(),
            role,
            binary_sha256: s.binary_sha256.clone(),
            git_commit: s.git_commit.clone(),
            env_class_id: s.env_class_id.clone(),
            records_sha256,
        });
        let mine: Vec<Record> = read_records(dir)?
            .into_iter()
            .filter(|r| r.arm == inputs.arm && !r.warmup)
            .collect();
        if v2 {
            for r in &mine {
                let key = (r.workload.workload_id.clone(), r.algorithm_seed.clone());
                if let Some(other) = seen_draws.get(&key) {
                    if other != &s.session_id {
                        return Err(format!(
                            "sessions {other} and {} both ran workload {} with algorithm seed {}: \
                             the same draw twice; fit one of them",
                            s.session_id, key.0, key.1
                        ));
                    }
                } else {
                    seen_draws.insert(key, s.session_id.clone());
                }
            }
        }
        recs.extend(mine);
    }
    let method = method.expect("at least one session");

    // Runs that did not verify are counted before any filter: a filter
    // that drops them would make the rest "wholly verified" (version 2).
    let unverified_before_filter = recs.iter().filter(|r| !r.counts()).count();
    let runs_before_filter = recs.len();

    // Filters: named curves, then tier.
    if !opts.curves.is_empty() {
        recs.retain(|r| opts.curves.contains(&r.workload.curve.slug));
    }
    if let Some(t) = &opts.tier {
        recs.retain(|r| field_size(r).map(tier_of_bits) == Some(t.as_str()));
    }
    let dropped_unverified = unverified_before_filter - recs.iter().filter(|r| !r.counts()).count();
    if recs.is_empty() {
        return Err(format!(
            "no measured runs of arm `{}` after filtering",
            inputs.arm
        ));
    }

    // Band-priced or charged cost per record (AGENTS.md §5a).
    let mut priced: BTreeMap<String, (Option<f64>, Option<f64>)> = BTreeMap::new();
    for r in &recs {
        let v = match opts.weight {
            None => (r.cost.total_gae, r.cost.s),
            Some(w) => (super::band::priced_gae(r, w)?, super::band::priced_s(r, w)?),
        };
        priced.insert(r.record_id.clone(), v);
    }
    let gae_of = |r: &Record| priced[&r.record_id].0;
    let s_of = |r: &Record| priced[&r.record_id].1;

    // One domain.
    let families: BTreeSet<&str> = recs
        .iter()
        .map(|r| r.workload.curve.family.as_str())
        .collect();
    if families.len() != 1 {
        return Err(format!(
            "the arm ran on {} curve families ({}); a bound is about one, so pass --curve to pick",
            families.len(),
            families.into_iter().collect::<Vec<_>>().join(", ")
        ));
    }
    let kinds: BTreeSet<String> = recs
        .iter()
        .map(|r| format!("{:?}", r.workload.kind()).to_lowercase())
        .collect();
    if kinds.len() != 1 {
        return Err(
            "the arm ran on planted and public targets; a bound is about one target kind".into(),
        );
    }
    let units: BTreeSet<String> = recs
        .iter()
        .map(|r| match opts.weight {
            None => r.cost.unit.clone(),
            Some(w) => format!("{}+band(w={w})", r.cost.unit),
        })
        .collect();
    if units.len() != 1 {
        return Err("the runs report different units".into());
    }
    let mut tiers: BTreeSet<&str> = BTreeSet::new();
    for r in &recs {
        let bits = field_size(r)
            .ok_or_else(|| format!("{}: curve facts carry no field size", r.record_id))?;
        tiers.insert(tier_of_bits(bits));
    }
    if tiers.len() != 1 {
        return Err(format!(
            "the sizes span tiers {}; tiers are not fungible, so fit one bound per tier with --tier",
            tiers.into_iter().collect::<Vec<_>>().join(" and ")
        ));
    }
    let domain = Domain {
        problem: PROBLEM.into(),
        family: families.into_iter().next().unwrap().to_string(),
        target_kind: kinds.into_iter().next().unwrap(),
        unit: units.into_iter().next().unwrap(),
        tier: tiers.into_iter().next().unwrap().to_string(),
        envelope: Envelope::default(),
    };
    let domain_id = domain.id()?;

    // Sizes.
    let mut by_slug: BTreeMap<String, Vec<&Record>> = BTreeMap::new();
    for r in &recs {
        by_slug
            .entry(r.workload.curve.slug.clone())
            .or_default()
            .push(r);
    }
    let mut sizes: Vec<SizeRow> = Vec::new();
    // Intervals fit version 2 refused for excluding their estimate.
    let mut refused: Vec<String> = Vec::new();
    for (slug, rs) in &by_slug {
        let any = rs[0];
        let ok: Vec<&&Record> = rs.iter().filter(|r| r.counts()).collect();
        let a = any.workload.curve.automorphisms_available;
        let floor = floor_s(a);
        let mut by_w: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
        for r in &ok {
            if let Some(s) = s_of(r) {
                by_w.entry(r.workload.workload_id.as_str())
                    .or_default()
                    .push(s);
            }
        }
        let strata: Vec<Vec<f64>> = by_w.into_values().collect();
        let s_est = estimate_v(&strata, opts.resamples, opts.seed, opts.fit_version);
        if s_est.as_ref().is_some_and(|e| e.ci_method == CI_REFUSED) {
            refused.push(format!("S on {slug}"));
        }
        let (size_axes, size_band) = if v2 {
            size_axes_and_band(&ok, opts, &mut refused, slug)
        } else {
            (BTreeMap::new(), Vec::new())
        };
        let mut levels = BTreeMap::new();
        for r in rs {
            *levels
                .entry(r.isolation.level.name().to_string())
                .or_insert(0) += 1;
        }
        let gaes: Vec<f64> = ok.iter().filter_map(|r| gae_of(r)).collect();
        let mems: Vec<f64> = ok.iter().filter_map(|r| memory_entries(r)).collect();
        let unch: Vec<f64> = ok.iter().map(|r| uncharged_work(r)).collect();
        let declared = expected_s(&method.id, a);
        sizes.push(SizeRow {
            slug: slug.clone(),
            log2_r: (any.workload.curve.r as f64).log2(),
            r: any.workload.curve.r,
            field_bits: field_size(any).unwrap_or(0),
            automorphisms_available: a,
            floor_s: floor,
            workloads: rs
                .iter()
                .map(|r| r.workload.workload_id.as_str())
                .collect::<BTreeSet<_>>()
                .len() as u64,
            runs: rs.len() as u64,
            verified: ok.len() as u64,
            mean_gae: mean(&gaes),
            mean_s: s_est.as_ref().map(|e| e.value),
            s_ci95: s_est.as_ref().and_then(|e| e.ci95),
            ratio_to_floor: s_est.as_ref().map(|e| e.value / floor),
            declared_s: declared,
            declared_ratio_to_floor: declared.map(|d| d / floor),
            memory_entries_per_sqrt_r: if mems.len() == ok.len() {
                mean(&mems)
            } else {
                None
            },
            uncharged_per_sqrt_r: mean(&unch),
            levels,
            axes: size_axes,
            band: size_band,
        });
    }
    sizes.sort_by_key(|a| a.r);

    // The fit, over verified runs, one stratum per size.
    let verified: Vec<&Record> = recs.iter().filter(|r| r.counts()).collect();
    let mut strata: Vec<Vec<(f64, f64)>> = Vec::new();
    for row in &sizes {
        let ln_r = (row.r as f64).ln();
        strata.push(
            verified
                .iter()
                .filter(|r| r.workload.curve.slug == row.slug)
                .filter_map(|r| gae_of(r))
                .filter(|g| *g > 0.0)
                .map(|g| (ln_r, g.ln()))
                .collect(),
        );
    }
    // The strata of the constant and the other axes: one per size, which
    // is the design when there are several.  A bound on one size has no
    // sizes to resample, and treating its runs as independent would count
    // the rounds of one target as many targets; from version 2 its strata
    // are the workloads.
    let groups: Vec<Vec<&Record>> = if v2 && sizes.len() == 1 {
        let mut by_w: BTreeMap<&str, Vec<&Record>> = BTreeMap::new();
        for r in &verified {
            by_w.entry(r.workload.workload_id.as_str())
                .or_default()
                .push(r);
        }
        by_w.into_values().collect()
    } else {
        sizes
            .iter()
            .map(|row| {
                verified
                    .iter()
                    .filter(|r| r.workload.curve.slug == row.slug)
                    .copied()
                    .collect()
            })
            .collect()
    };
    let mut s_strata: Vec<Vec<f64>> = Vec::new();
    let mut f_strata: Vec<Vec<f64>> = Vec::new();
    let mut m_strata: Vec<Vec<f64>> = Vec::new();
    let mut u_strata: Vec<Vec<f64>> = Vec::new();
    let mut memory_known = true;
    for mine in &groups {
        s_strata.push(mine.iter().filter_map(|r| s_of(r)).collect());
        f_strata.push(
            mine.iter()
                .filter_map(|r| {
                    s_of(r).map(|s| s / floor_s(r.workload.curve.automorphisms_available))
                })
                .collect(),
        );
        let mems: Vec<f64> = mine.iter().filter_map(|r| memory_entries(r)).collect();
        if mems.len() != mine.len() {
            memory_known = false;
        }
        m_strata.push(mems);
        u_strata.push(mine.iter().map(|r| uncharged_work(r)).collect());
    }
    let n_sizes = sizes.iter().filter(|s| s.verified > 0).count();
    let (alpha, alpha_ci, log2_c, r2, ci_method) = if v2 {
        power_law_v2(&strata, opts.resamples, opts.seed)
    } else {
        power_law(&strata, opts.resamples, opts.seed)
    };
    let declared_alpha = expected_s(&method.id, sizes[0].automorphisms_available).map(|_| 0.5);
    let scaling_claim =
        n_sizes >= MIN_SIZES_FOR_SCALING && alpha.is_some() && (!v2 || alpha_ci.is_some());
    let fit = Fit {
        size_parameter: "r".into(),
        model: "gae = C * r^alpha".into(),
        points: strata.iter().map(|s| s.len() as u64).sum(),
        sizes: n_sizes as u64,
        alpha,
        alpha_ci95: alpha_ci,
        log2_c,
        r_squared: r2,
        ci_method,
        declared_alpha,
        alpha_agrees_with_declared: match (declared_alpha, alpha_ci) {
            (Some(d), Some((lo, hi))) => Some(lo <= d && d <= hi),
            _ => None,
        },
        scaling_claim,
    };
    let none = || Estimate {
        value: f64::NAN,
        ci95: None,
        ci_method: "none".into(),
    };
    let est = |strata: &[Vec<f64>], salt: u64| {
        estimate_v(strata, opts.resamples, opts.seed ^ salt, opts.fit_version)
    };
    let s_est = est(&s_strata, 0x5).unwrap_or_else(none);
    let f_est = est(&f_strata, 0xF).unwrap_or_else(none);
    for (name, e) in [("S", &s_est), ("ops", &f_est)] {
        if e.ci_method == CI_REFUSED {
            refused.push(format!("{name} (the constant)"));
        }
    }
    let declared_ratios: Vec<f64> = sizes
        .iter()
        .filter_map(|s| s.declared_ratio_to_floor)
        .collect();
    let declared_ratio = if declared_ratios.len() == sizes.len()
        && declared_ratios
            .iter()
            .all(|d| (d - declared_ratios[0]).abs() < 1e-9)
    {
        declared_ratios.first().copied()
    } else {
        None
    };
    let constant = Constant {
        s: s_est,
        ratio_to_floor: f_est.clone(),
        declared_ratio_to_floor: declared_ratio,
    };

    let mut dimensions = BTreeMap::new();
    dimensions.insert(
        "ops".to_string(),
        Dimension {
            statistic: "mean S / sqrt(pi / 2A) over verified runs".into(),
            source: "cost.total_gae, workload.curve.r, automorphisms_available".into(),
            lower_is_better: true,
            known: f_est.value.is_finite(),
            value: f_est.value.is_finite().then_some(f_est.value),
            ci95: f_est.ci95,
        },
    );
    let m_est = if memory_known {
        est(&m_strata, 0x3)
    } else {
        None
    };
    if m_est.as_ref().is_some_and(|e| e.ci_method == CI_REFUSED) {
        refused.push("memory".into());
    }
    dimensions.insert(
        "memory".to_string(),
        Dimension {
            statistic: "mean table entries / sqrt(r) over verified runs".into(),
            source: format!("first of counters.{}", MEMORY_COUNTERS.join(", counters.")),
            lower_is_better: true,
            known: m_est.is_some(),
            value: m_est.as_ref().map(|e| e.value),
            ci95: m_est.as_ref().and_then(|e| e.ci95),
        },
    );
    let u_est = est(&u_strata, 0x7);
    if u_est.as_ref().is_some_and(|e| e.ci_method == CI_REFUSED) {
        refused.push("uncharged".into());
    }
    dimensions.insert(
        "uncharged".to_string(),
        Dimension {
            statistic: "mean sum of *_uncharged counters / sqrt(r) over verified runs".into(),
            source: "counters.*_uncharged".into(),
            lower_is_better: true,
            known: u_est.is_some(),
            value: u_est.as_ref().map(|e| e.value),
            ci95: u_est.as_ref().and_then(|e| e.ci95),
        },
    );
    // The field-operation axes, with the same two-stage interval, only
    // when there are verified runs and every one of them carries the
    // block.  Otherwise nothing is added — not an unknown axis — so a
    // bound fitted from records written before the block existed is
    // byte for byte the bound it was.
    for (i, axis) in FIELD_AXES.iter().enumerate() {
        let mut strata: Vec<Vec<f64>> = Vec::new();
        let mut known = !verified.is_empty();
        for mine in &groups {
            let vals: Vec<f64> = mine.iter().filter_map(|r| field_axis(r, axis)).collect();
            if vals.len() != mine.len() {
                known = false;
            }
            strata.push(vals);
        }
        if !known {
            continue;
        }
        let (what, source) = field_axis_describes(axis).expect("a field axis");
        let est = est(&strata, 0x11 + 2 * i as u64);
        if est.as_ref().is_some_and(|e| e.ci_method == CI_REFUSED) {
            refused.push((*axis).to_string());
        }
        dimensions.insert(
            axis.to_string(),
            Dimension {
                statistic: format!("mean {what} / sqrt(r) over verified runs"),
                source: format!("{source}, workload.curve.r"),
                lower_is_better: true,
                known: est.is_some(),
                value: est.as_ref().map(|e| e.value),
                ci95: est.as_ref().and_then(|e| e.ci95),
            },
        );
    }

    // Stages.
    let mut names: Vec<String> = Vec::new();
    for r in &verified {
        for p in &r.phases {
            if !names.contains(&p.name) {
                names.push(p.name.clone());
            }
        }
    }
    let total_gae: f64 = verified.iter().filter_map(|r| r.cost.total_gae).sum();
    let mut stages = Vec::new();
    for name in names {
        let mut sum = 0.0;
        let mut st: Vec<Vec<(f64, f64)>> = Vec::new();
        for row in &sizes {
            let mut pts = Vec::new();
            for r in verified
                .iter()
                .filter(|r| r.workload.curve.slug == row.slug)
            {
                for p in r.phases.iter().filter(|p| p.name == name) {
                    sum += p.gae;
                    if p.gae > 0.0 {
                        pts.push(((row.r as f64).ln(), p.gae.ln()));
                    }
                }
            }
            st.push(pts);
        }
        let with_work = st.iter().filter(|s| !s.is_empty()).count();
        let (a, ci) = if with_work >= MIN_SIZES_FOR_SCALING {
            let (a, ci, _, _, _) = if v2 {
                power_law_v2(&st, opts.resamples, opts.seed ^ 0x51)
            } else {
                power_law(&st, opts.resamples, opts.seed ^ 0x51)
            };
            (a, ci)
        } else {
            (None, None)
        };
        stages.push(StageRow {
            name,
            mean_gae: if verified.is_empty() {
                0.0
            } else {
                sum / verified.len() as f64
            },
            share_of_gae: if total_gae > 0.0 {
                sum / total_gae
            } else {
                0.0
            },
            sizes_with_work: with_work as u64,
            alpha: a,
            alpha_ci95: ci,
        });
    }

    // Provenance and admissibility.
    let mut audits = Vec::new();
    let session_ids: BTreeSet<&str> = sessions.iter().map(|s| s.session_id.as_str()).collect();
    for p in inputs.audits {
        let bytes = std::fs::read(p).map_err(|e| format!("{}: {e}", p.display()))?;
        let rep: AuditReport =
            serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", p.display()))?;
        if !session_ids.contains(rep.session_id.as_str()) {
            return Err(format!(
                "{} audits session {}, which this bound does not read",
                p.display(),
                rep.session_id
            ));
        }
        audits.push(AuditRef {
            path: relative(inputs.root, p),
            sha256: sha256_hex(&bytes),
            session_id: rep.session_id.clone(),
            ok: rep.ok,
            replays: rep.replays.len() as u64,
            replays_reproduced: rep.replays.iter().filter(|r| r.reproduced).count() as u64,
        });
    }
    for a in inputs.audit_refs {
        if !session_ids.contains(a.session_id.as_str()) {
            return Err(format!(
                "audit {} is of session {}, which this bound does not read",
                a.path, a.session_id
            ));
        }
        audits.push(a.clone());
    }
    let mut statuses = BTreeMap::new();
    for r in &recs {
        *statuses.entry(r.outcome.status.clone()).or_insert(0u64) += 1;
    }
    let mut reasons = Vec::new();
    let unverified = recs.len() - verified.len();
    if unverified > 0 {
        reasons.push(format!(
            "{unverified} of {} measured runs did not verify; only a wholly verified arm is admissible",
            recs.len()
        ));
    }
    if !scaling_claim {
        reasons.push(format!(
            "{n_sizes} size(s): below {MIN_SIZES_FOR_SCALING}, so the exponent is descriptive and this is a constant bound"
        ));
    }
    for s in &sessions {
        if s.status != "complete" {
            reasons.push(format!("session {} is {}", s.session_id, s.status));
        }
    }
    if audits.iter().any(|a| !a.ok) {
        reasons.push("an audit receipt reports problems".into());
    }
    let bounded = recs.iter().any(|r| r.cost.lower_bound);
    let mut unpriced: Vec<String> = recs.iter().flat_map(|r| r.cost.unpriced.clone()).collect();
    unpriced.sort();
    unpriced.dedup();
    let mut admissible = unverified == 0 && audits.iter().all(|a| a.ok);
    if v2 {
        // The refusals of fit version 2 (README §3a).  Each is a reason and
        // each makes the bound inadmissible; none changes a figure.
        if dropped_unverified > 0 {
            admissible = false;
            reasons.push(format!(
                "{dropped_unverified} of {runs_before_filter} measured runs did not verify and a filter (--curve/--tier) dropped them; failed runs are counted before any filter"
            ));
        }
        for s in &sessions {
            if s.status != "complete" {
                admissible = false;
            }
            if !audits.iter().any(|a| a.session_id == s.session_id && a.ok) {
                admissible = false;
                reasons.push(format!(
                    "session {} has no passing audit receipt",
                    s.session_id
                ));
            }
        }
        if opts.resamples != RESAMPLES {
            admissible = false;
            reasons.push(format!(
                "{} bootstrap resamples; an admissible bound uses the default {RESAMPLES} (seed {} recorded)",
                opts.resamples, opts.seed
            ));
        }
        for what in &refused {
            admissible = false;
            reasons.push(format!(
                "the interval of {what} excluded its own estimate and was refused"
            ));
        }
        if n_sizes >= MIN_SIZES_FOR_SCALING && alpha.is_some() && alpha_ci.is_none() {
            admissible = false;
            reasons.push(
                "the exponent has no interval: too few valid resamples (at least three distinct sizes each) or the interval excluded the slope"
                    .into(),
            );
        }
    }
    let admissibility = Admissibility {
        status: if admissible {
            "admissible"
        } else {
            "inadmissible"
        }
        .into(),
        bounded,
        unpriced,
        deterministic: recs.iter().all(|r| r.cost.deterministic),
        reasons,
    };
    let label = inputs.label.map(String::from).unwrap_or_else(|| {
        format!(
            "{} on {} {} curves, {} tier",
            method.id, n_sizes, domain.family, domain.tier
        )
    });
    let tail = v2.then(|| {
        let mut ratios: Vec<(f64, &str)> = verified
            .iter()
            .filter_map(|r| {
                s_of(r).map(|s| {
                    (
                        s / floor_s(r.workload.curve.automorphisms_available),
                        r.record_id.as_str(),
                    )
                })
            })
            .collect();
        tail_of(&mut ratios)
    });
    let mut b = Bound {
        schema: BOUND_SCHEMA.into(),
        bound_id: String::new(),
        label,
        domain,
        domain_id,
        method,
        level: if scaling_claim {
            "exponent"
        } else {
            "constant"
        }
        .into(),
        sizes,
        fit,
        constant,
        dimensions,
        stages,
        provenance: Provenance {
            sessions,
            audits,
            records: recs.len() as u64,
            verified: verified.len() as u64,
            statuses,
        },
        admissibility,
        fit_options: opts.clone(),
        improves_on: vec![],
        verdict_id: None,
        notes: inputs.notes.to_string(),
        tail: tail.flatten(),
    };
    b.seal()?;
    Ok(b)
}

/// Median, maximum and top-run share of the `ops` ratios of a bound's
/// verified runs, with the flag of [`TAIL_MAX_OVER_MEDIAN`] and
/// [`TAIL_TOP_SHARE`].  `None` without a run.
pub fn tail_of(ratios: &mut [(f64, &str)]) -> Option<Tail> {
    if ratios.is_empty() {
        return None;
    }
    ratios.sort_by(|a, b| a.0.total_cmp(&b.0));
    let vals: Vec<f64> = ratios.iter().map(|r| r.0).collect();
    let median = crate::cryptanalysis::ecbench::stats::median(&vals)?;
    let (max, top) = ratios.last().map(|r| (r.0, r.1.to_string()))?;
    let total: f64 = vals.iter().sum();
    let runs = vals.len() as u64;
    let max_over_median = if median > 0.0 {
        max / median
    } else {
        f64::INFINITY
    };
    let share = if total > 0.0 { max / total } else { 0.0 };
    Some(Tail {
        runs,
        median_ratio: median,
        max_ratio: max,
        max_over_median: if max_over_median.is_finite() {
            max_over_median
        } else {
            0.0
        },
        top_run_share: share,
        top_run: top,
        large: max_over_median >= TAIL_MAX_OVER_MEDIAN
            || (runs >= TAIL_MIN_RUNS_FOR_SHARE && share >= TAIL_TOP_SHARE),
    })
}

/// The per-size axes and band of fit version 2, each with a two-stage
/// interval over the size's workloads.
fn size_axes_and_band(
    ok: &[&&Record],
    opts: &FitOptions,
    refused: &mut Vec<String>,
    slug: &str,
) -> (BTreeMap<String, SizeAxis>, Vec<BandPoint>) {
    // Strata for `f`, one per workload; `None` when some run lacks it.
    let strata_of = |f: &dyn Fn(&Record) -> Option<f64>| -> Option<Vec<Vec<f64>>> {
        if ok.is_empty() {
            return None;
        }
        let mut by_w: BTreeMap<&str, Vec<f64>> = BTreeMap::new();
        for r in ok {
            by_w.entry(r.workload.workload_id.as_str())
                .or_default()
                .push(f(r)?);
        }
        Some(by_w.into_values().collect())
    };
    let mut axes = BTreeMap::new();
    let mut put = |name: &str, f: &dyn Fn(&Record) -> Option<f64>, salt: u64| {
        if let Some(st) = strata_of(f) {
            if let Some(e) = estimate_v(&st, opts.resamples, opts.seed ^ salt, opts.fit_version) {
                if e.ci_method == CI_REFUSED {
                    refused.push(format!("{name} on {slug}"));
                }
                axes.insert(
                    name.to_string(),
                    SizeAxis {
                        value: e.value,
                        ci95: e.ci95,
                    },
                );
            }
        }
    };
    put("memory", &memory_entries, 0x103);
    put("uncharged", &|r| Some(uncharged_work(r)), 0x107);
    for (i, axis) in FIELD_AXES.iter().enumerate() {
        put(axis, &|r| field_axis(r, axis), 0x111 + 2 * i as u64);
    }
    let mut band = Vec::new();
    if opts.weight.is_none() {
        for (i, w) in BAND_WEIGHTS.iter().enumerate() {
            let point =
                strata_of(&|r| super::band::priced_s(r, *w).ok().flatten()).and_then(|st| {
                    estimate_v(
                        &st,
                        opts.resamples,
                        opts.seed ^ (0x121 + i as u64),
                        opts.fit_version,
                    )
                });
            if point.as_ref().is_some_and(|e| e.ci_method == CI_REFUSED) {
                refused.push(format!("S at w={w} on {slug}"));
            }
            band.push(BandPoint {
                weight: *w,
                s: point.as_ref().map(|e| e.value),
                ci95: point.as_ref().and_then(|e| e.ci95),
            });
        }
    }
    (axes, band)
}

/// Re-derive a committed bound from the sessions it names and require the
/// same id: the bound's analogue of a replay.  Returns the re-derived
/// bound.
pub fn check(path: &Path, root: &Path) -> Result<Bound, String> {
    let recorded = Bound::read(path)?;
    let dirs: Vec<PathBuf> = recorded
        .provenance
        .sessions
        .iter()
        .map(|s| root.join(&s.dir))
        .collect();
    // An audit a challenge verdict made of its own session has no receipt
    // file (its path starts with `verdict:`); the verdict's seal binds it.
    let audits: Vec<PathBuf> = recorded
        .provenance
        .audits
        .iter()
        .filter(|a| !a.path.starts_with(VERDICT_AUDIT_PREFIX))
        .map(|a| root.join(&a.path))
        .collect();
    let audit_refs: Vec<AuditRef> = recorded
        .provenance
        .audits
        .iter()
        .filter(|a| a.path.starts_with(VERDICT_AUDIT_PREFIX))
        .cloned()
        .collect();
    let arm = recorded
        .provenance
        .sessions
        .first()
        .map(|s| s.arm.clone())
        .ok_or("the bound names no session")?;
    let mut fresh = bound_from_sessions(&FitInputs {
        dirs: &dirs,
        arm: &arm,
        audits: &audits,
        label: Some(&recorded.label),
        notes: &recorded.notes,
        root,
        options: recorded.fit_options.clone(),
        audit_refs: &audit_refs,
    })?;
    // What a verdict wrote onto the bound is not re-derivable from the
    // sessions; carry it over, then compare the whole document.
    fresh.improves_on = recorded.improves_on.clone();
    fresh.verdict_id = recorded.verdict_id.clone();
    fresh.seal()?;
    if fresh.bound_id != recorded.bound_id {
        // Compare each section as the JSON the seal hashes, not as parsed
        // structs: a float read back from a record need not equal the one
        // that was written bit for bit, so struct equality can name sections
        // that did not change and miss the one that did.
        fn json<T: Serialize>(v: &T) -> String {
            serde_json::to_string(v).unwrap_or_default()
        }
        let raw: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?,
        )
        .map_err(|e| format!("{}: {e}", path.display()))?;
        let fresh_v: serde_json::Value = serde_json::from_str(&json(&fresh)).unwrap_or_default();
        let mut diffs = Vec::new();
        for key in [
            "sizes",
            "fit",
            "constant",
            "dimensions",
            "provenance",
            "admissibility",
            "fit_options",
            "method",
            "domain",
        ] {
            if raw.get(key) != fresh_v.get(key) {
                diffs.push(key);
            }
        }
        return Err(format!(
            "{}: re-derived bound is {}, recorded {}; differs in: {}",
            path.display(),
            fresh.bound_id,
            recorded.bound_id,
            if diffs.is_empty() {
                "other fields".to_string()
            } else {
                diffs.join(", ")
            }
        ));
    }
    Ok(fresh)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ols_recovers_an_exact_power_law() {
        // cost = 3 · r^0.5 at four sizes.
        let pts: Vec<(f64, f64)> = [16.0f64, 18.0, 20.0, 22.0]
            .iter()
            .map(|b| {
                let r = 2f64.powf(*b);
                (r.ln(), (3.0 * r.sqrt()).ln())
            })
            .collect();
        let (alpha, b, r2) = ols(&pts).unwrap();
        assert!((alpha - 0.5).abs() < 1e-9, "{alpha}");
        assert!((b.exp() - 3.0).abs() < 1e-9);
        assert!((r2 - 1.0).abs() < 1e-12);
        assert!(ols(&pts[..1]).is_none());
        assert!(ols(&[(1.0, 1.0), (1.0, 2.0)]).is_none());
    }

    #[test]
    fn power_law_interval_brackets_the_slope_and_needs_two_sizes() {
        let mut strata = Vec::new();
        for (i, b) in [16.0f64, 18.0, 20.0, 22.0].iter().enumerate() {
            let r = 2f64.powf(*b);
            // Spread the runs around the law, deterministically.
            strata.push(
                (0..8)
                    .map(|k| {
                        let noise = 1.0 + 0.3 * (((k * 7 + i * 3) % 5) as f64 - 2.0) / 2.0;
                        (r.ln(), (1.25 * r.sqrt() * noise).ln())
                    })
                    .collect::<Vec<_>>(),
            );
        }
        let (alpha, ci, log2_c, r2, method) = power_law(&strata, 500, 1);
        let a = alpha.unwrap();
        assert!((a - 0.5).abs() < 0.05, "{a}");
        let (lo, hi) = ci.unwrap();
        assert!(lo <= a && a <= hi);
        assert_eq!(method, "cluster");
        assert!(log2_c.unwrap().is_finite());
        assert!(r2.unwrap() > 0.9);
        let (_, ci1, _, _, m1) = power_law(&strata[..1], 500, 1);
        assert!(ci1.is_none());
        assert_eq!(m1, "none");
    }

    #[test]
    fn tiers_follow_the_field_size() {
        assert_eq!(tier_of_bits(13), "toy");
        assert_eq!(tier_of_bits(32), "toy");
        assert_eq!(tier_of_bits(33), "medium");
        assert_eq!(tier_of_bits(96), "medium");
        assert_eq!(tier_of_bits(131), "crypto");
    }

    #[test]
    fn the_domain_id_ignores_nothing_and_is_stable() {
        let d = Domain {
            problem: PROBLEM.into(),
            family: "prime".into(),
            target_kind: "planted".into(),
            unit: "ecbench.gae".into(),
            tier: "toy".into(),
            envelope: Envelope::default(),
        };
        let id = d.id().unwrap();
        assert!(id.starts_with("ECDOM1h"), "{id}");
        let mut k = d.clone();
        k.family = "koblitz".into();
        assert_ne!(k.id().unwrap(), id);
    }

    #[derive(Serialize, Deserialize)]
    struct Doc {
        schema: String,
        doc_id: String,
        value: f64,
        inner: BTreeMap<String, String>,
    }

    #[test]
    fn a_sealed_document_checks_and_a_changed_byte_does_not() {
        let mut inner = BTreeMap::new();
        inner.insert(
            "doc_id".to_string(),
            "nested keys of the same name are fine".to_string(),
        );
        let d = Doc {
            schema: "t/v1".into(),
            doc_id: String::new(),
            value: 0.8862269254527579,
            inner,
        };
        let (id, text) = seal_document(&d, "doc_id", "T1h").unwrap();
        assert!(id.starts_with("T1h") && id.len() == 3 + 12);
        assert_eq!(check_document_seal(&text, "doc_id", "T1h").unwrap(), id);
        let forged = text.replace("0.8862269254527579", "0.5862269254527579");
        assert!(check_document_seal(&forged, "doc_id", "T1h").is_err());
        // The nested `doc_id` is not the sealed field: the top-level one is
        // the first occurrence in the pretty text because keys sort.
        assert!(text.find("\"doc_id\": \"T1h").unwrap() < text.find("nested keys").unwrap());
    }
    // ── Fit version 2 ──────────────────────────────────────────────

    fn repo() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    const CANDIDATES: &str = "research/ecbench_all_candidates_20261003";

    /// A scratch copy of a committed session (never the evidence itself),
    /// with `edit` applied to its `session.json` text and `records_tail`
    /// appended to its records.
    fn scratch_copy(
        src: &Path,
        name: &str,
        edit: impl Fn(String) -> String,
        tail: &str,
    ) -> PathBuf {
        let dst =
            std::env::temp_dir().join(format!("ecbench-bounds-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dst);
        std::fs::create_dir_all(&dst).unwrap();
        for f in ["plan.json", "spec.json", "host.json"] {
            if src.join(f).exists() {
                std::fs::copy(src.join(f), dst.join(f)).unwrap();
            }
        }
        let sj = std::fs::read_to_string(src.join("session.json")).unwrap();
        std::fs::write(dst.join("session.json"), edit(sj)).unwrap();
        let mut rec = std::fs::read_to_string(src.join("records.jsonl")).unwrap();
        rec.push_str(tail);
        std::fs::write(dst.join("records.jsonl"), rec).unwrap();
        dst
    }

    fn fit(
        dirs: &[PathBuf],
        arm: &str,
        audits: &[PathBuf],
        options: FitOptions,
    ) -> Result<Bound, String> {
        bound_from_sessions(&FitInputs {
            dirs,
            arm,
            audits,
            label: None,
            notes: "",
            root: &repo(),
            options,
            audit_refs: &[],
        })
    }

    fn v1() -> FitOptions {
        FitOptions {
            fit_version: 1,
            ..Default::default()
        }
    }

    #[test]
    fn a_degenerate_resample_has_no_slope_by_a_relative_tolerance() {
        // Every point on one size: the slope is rounding noise, not a value.
        let x = (2f64.powi(40)).ln();
        let pts: Vec<(f64, f64)> = (0..40).map(|i| (x, 10.0 + i as f64 * 0.37)).collect();
        assert_eq!(distinct_x(&pts), 1);
        assert!(ols_v2(&pts).is_none());
        // Sizes closer than a relative 1e-9 are one size.
        let near = [(x, 1.0), (x * (1.0 + 1e-12), 2.0)];
        assert_eq!(distinct_x(&near), 1);
        assert!(ols_v2(&near).is_none());
        let two = [(x, 1.0), (x + 1.0, 3.0)];
        assert!((ols_v2(&two).unwrap().0 - 2.0).abs() < 1e-9);
    }

    fn strata_of_sizes(bits: &[f64]) -> Vec<Vec<(f64, f64)>> {
        bits.iter()
            .enumerate()
            .map(|(i, b)| {
                let r = 2f64.powf(*b);
                (0..8)
                    .map(|k| {
                        let noise = 1.0 + 0.3 * (((k * 7 + i * 3) % 5) as f64 - 2.0) / 2.0;
                        (r.ln(), (1.25 * r.sqrt() * noise).ln())
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn the_exponent_has_no_interval_below_four_sizes_and_no_slope_below_two() {
        let four = strata_of_sizes(&[16.0, 18.0, 20.0, 22.0]);
        let (a, ci, _, _, m) = power_law_v2(&four, 400, 1);
        let a = a.unwrap();
        let (lo, hi) = ci.unwrap();
        assert!(
            lo <= a && a <= hi && (a - 0.5).abs() < 0.1,
            "{a} [{lo}, {hi}]"
        );
        assert_eq!(m, "cluster");
        // Version 1 would have given three sizes an interval.
        let three = strata_of_sizes(&[16.0, 18.0, 20.0]);
        assert!(power_law(&three, 400, 1).1.is_some());
        let (a3, ci3, _, _, m3) = power_law_v2(&three, 400, 1);
        assert!(a3.is_some() && ci3.is_none() && m3 == "none");
        let (a1, ci1, _, _, _) = power_law_v2(&four[..1], 400, 1);
        assert!(a1.is_none() && ci1.is_none());
    }

    #[test]
    fn the_rejecting_bootstrap_draws_until_it_has_the_resamples_it_was_asked_for() {
        let strata = strata_of_sizes(&[16.0, 18.0, 20.0, 22.0]);
        let enough = |s: &[Vec<(f64, f64)>]| {
            let p: Vec<(f64, f64)> = s.iter().flatten().copied().collect();
            distinct_x(&p) >= MIN_DISTINCT_SIZES_RESAMPLE
        };
        let slope = |s: &[Vec<(f64, f64)>]| {
            let p: Vec<(f64, f64)> = s.iter().flatten().copied().collect();
            ols_v2(&p).map(|x| x.0)
        };
        let v = bootstrap_values(&strata, 300, 9, true, enough, slope).unwrap();
        assert_eq!(v.len(), 300);
        // An acceptance that never holds cannot fill the request.
        assert!(bootstrap_values(&strata, 50, 9, true, |_| false, slope).is_none());
    }

    #[test]
    fn an_interval_that_excludes_its_estimate_is_refused_from_version_2() {
        // One resample is a point that is not the mean: version 1 keeps it
        // as an "interval", version 2 refuses it.
        let strata = vec![vec![1.0, 2.0, 9.0], vec![3.0, 4.0, 5.0]];
        let old = estimate_v(&strata, 1, 3, 1).unwrap();
        let new = estimate_v(&strata, 1, 3, 2).unwrap();
        let (lo, hi) = old.ci95.unwrap();
        assert!(old.value < lo || old.value > hi, "{old:?}");
        assert!(new.ci95.is_none() && new.ci_method == CI_REFUSED, "{new:?}");
        // A sound interval is kept.
        let ok = estimate_v(&strata, 500, 3, 2).unwrap();
        let (lo, hi) = ok.ci95.unwrap();
        assert!(lo <= ok.value && ok.value <= hi);
    }

    #[test]
    fn the_tail_flags_a_mean_carried_by_one_run() {
        let mut flat: Vec<(f64, &str)> = (0..20).map(|i| (1.0 + i as f64 * 0.01, "r")).collect();
        let t = tail_of(&mut flat).unwrap();
        assert!(!t.large && t.max_over_median < 1.3, "{t:?}");
        let mut heavy: Vec<(f64, &str)> = (0..20).map(|_| (1.0, "r")).collect();
        heavy.push((60.0, "lucky-unlucky"));
        let t = tail_of(&mut heavy).unwrap();
        assert!(
            t.large && t.top_run == "lucky-unlucky" && t.top_run_share > 0.7,
            "{t:?}"
        );
        assert!(tail_of(&mut []).is_none());
    }

    #[test]
    fn a_session_counted_twice_is_refused_and_a_copy_of_its_draws_too() {
        let src = repo().join(CANDIDATES).join("sessions/prime");
        // The same directory twice (version 1 counted it twice).
        let dup = fit(&[src.clone(), src.clone()], "rho-neg", &[], v1());
        assert!(dup.is_ok(), "{dup:?}");
        let e = fit(
            &[src.clone(), src.clone()],
            "rho-neg",
            &[],
            FitOptions::default(),
        )
        .unwrap_err();
        assert!(e.contains("listed twice"), "{e}");
        // The same draws under another session id and other bytes: one
        // workload and algorithm seed, twice.
        let copy = scratch_copy(
            &src,
            "draws",
            |t| t.replace("ECBS1haf997766feb9", "ECBS1hdeadbeef0000"),
            "\n",
        );
        let e = fit(
            &[src.clone(), copy.clone()],
            "rho-neg",
            &[],
            FitOptions::default(),
        )
        .unwrap_err();
        assert!(e.contains("the same draw twice"), "{e}");
        // The same bytes under another name.
        let same_bytes = scratch_copy(
            &src,
            "bytes",
            |t| t.replace("ECBS1haf997766feb9", "ECBS1hdeadbeef0001"),
            "",
        );
        let e = fit(
            &[src.clone(), same_bytes.clone()],
            "rho-neg",
            &[],
            FitOptions::default(),
        )
        .unwrap_err();
        assert!(e.contains("same records"), "{e}");
        let _ = std::fs::remove_dir_all(copy);
        let _ = std::fs::remove_dir_all(same_bytes);
    }

    #[test]
    fn admissibility_needs_a_complete_session_an_audit_and_the_default_resamples() {
        let src = repo().join(CANDIDATES).join("sessions/prime");
        let audit = repo().join(CANDIDATES).join("audit-prime.json");
        let good = fit(
            std::slice::from_ref(&src),
            "rho-neg",
            std::slice::from_ref(&audit),
            FitOptions::default(),
        )
        .unwrap();
        assert!(good.admissible(), "{:?}", good.admissibility.reasons);
        assert_eq!(good.fit_version(), FIT_VERSION);
        assert!(good.tail.is_some());
        // Zero audits: version 1 admitted it; version 2 does not.
        assert!(fit(std::slice::from_ref(&src), "rho-neg", &[], v1())
            .unwrap()
            .admissible());
        let none = fit(
            std::slice::from_ref(&src),
            "rho-neg",
            &[],
            FitOptions::default(),
        )
        .unwrap();
        assert!(!none.admissible());
        assert!(none
            .admissibility
            .reasons
            .iter()
            .any(|r| r.contains("no passing audit")));
        // A free resample count is recorded and is not admissible.
        let few = fit(
            std::slice::from_ref(&src),
            "rho-neg",
            std::slice::from_ref(&audit),
            FitOptions {
                resamples: 100,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(!few.admissible());
        assert_eq!(few.fit_options.resamples, 100);
        // An interrupted session: version 1 admitted it.
        let interrupted = scratch_copy(
            &src,
            "interrupted",
            |t| t.replace("\"status\": \"complete\"", "\"status\": \"interrupted\""),
            "",
        );
        let aud_ok = || {
            // The audit names the original session id; the copy keeps it.
            vec![audit.clone()]
        };
        let old = fit(
            std::slice::from_ref(&interrupted),
            "rho-neg",
            &aud_ok(),
            v1(),
        )
        .unwrap();
        assert!(
            old.admissible(),
            "version 1 hole: {:?}",
            old.admissibility.reasons
        );
        let new = fit(
            std::slice::from_ref(&interrupted),
            "rho-neg",
            &aud_ok(),
            FitOptions::default(),
        )
        .unwrap();
        assert!(!new.admissible());
        assert!(new
            .admissibility
            .reasons
            .iter()
            .any(|r| r.contains("interrupted")));
        let _ = std::fs::remove_dir_all(interrupted);
    }

    #[test]
    fn a_filter_that_drops_failed_runs_cannot_make_a_bound_admissible() {
        // `ic-frob-m2` errors on two of the four curves.  Keeping only the
        // two that verified leaves a "wholly verified" arm.
        let src = repo().join(CANDIDATES).join("sessions/koblitz");
        let audit = repo().join(CANDIDATES).join("audit-koblitz.json");
        let keep = vec![
            "icv1-f2m17-tm101-00378d4e".to_string(),
            "icv1-f2m23-t5197-69e76b73".to_string(),
        ];
        let opts = |version| FitOptions {
            curves: keep.clone(),
            fit_version: version,
            ..Default::default()
        };
        let old = fit(
            std::slice::from_ref(&src),
            "ic-frob-m2",
            std::slice::from_ref(&audit),
            opts(1),
        )
        .unwrap();
        assert!(
            old.admissible(),
            "version 1 hole: {:?}",
            old.admissibility.reasons
        );
        let new = fit(
            std::slice::from_ref(&src),
            "ic-frob-m2",
            std::slice::from_ref(&audit),
            opts(2),
        )
        .unwrap();
        assert!(!new.admissible());
        assert!(new
            .admissibility
            .reasons
            .iter()
            .any(|r| r.contains("before any filter")));
        // And a filtered bound is not a frontier entry.
        assert!(!new.frontier_ineligible().is_empty());
    }

    #[test]
    fn a_bound_on_one_size_is_stratified_by_workload() {
        let src = repo().join(CANDIDATES).join("sessions/prime");
        let audit = repo().join(CANDIDATES).join("audit-prime.json");
        let slug = {
            let recs = read_records(&src).unwrap();
            recs[0].workload.curve.slug.clone()
        };
        let o = |version| FitOptions {
            curves: vec![slug.clone()],
            fit_version: version,
            ..Default::default()
        };
        let old = fit(
            std::slice::from_ref(&src),
            "rho-neg",
            std::slice::from_ref(&audit),
            o(1),
        )
        .unwrap();
        let new = fit(
            std::slice::from_ref(&src),
            "rho-neg",
            std::slice::from_ref(&audit),
            o(2),
        )
        .unwrap();
        assert_eq!(old.constant.ratio_to_floor.ci_method, "within");
        assert_eq!(new.constant.ratio_to_floor.ci_method, "cluster");
        // Same point estimate, a different (workload-clustered) interval.
        // (summed in another order: equal to rounding)
        assert!(
            (old.constant.ratio_to_floor.value - new.constant.ratio_to_floor.value).abs() < 1e-12
        );
        assert!(new.fit.alpha.is_none() && new.fit.alpha_ci95.is_none());
    }

    #[test]
    fn version_2_sizes_carry_the_axes_and_the_pricing_band() {
        let src = repo().join(CANDIDATES).join("sessions/prime");
        let audit = repo().join(CANDIDATES).join("audit-prime.json");
        let b = fit(
            std::slice::from_ref(&src),
            "bsgs-neg",
            &[audit],
            FitOptions::default(),
        )
        .unwrap();
        for s in &b.sizes {
            assert!(s.axes.contains_key("memory") && s.axes.contains_key("uncharged"));
            assert_eq!(s.band.len(), BAND_WEIGHTS.len());
            // Priced work only adds: S(w) is non-decreasing in w.
            let v: Vec<f64> = s.band.iter().filter_map(|p| p.s).collect();
            assert!(v.windows(2).all(|w| w[0] <= w[1] + 1e-12), "{v:?}");
        }
    }

    #[test]
    fn committed_version_1_bounds_re_derive_under_their_recorded_version() {
        let rec =
            repo().join("docs/bounds/records/challenge-prime-toy-reference-e1-bsgs-negation.json");
        let b = check(&rec, &repo()).unwrap();
        assert_eq!(b.fit_version(), 1);
        assert!(
            b.tail.is_none()
                && b.sizes
                    .iter()
                    .all(|s| s.axes.is_empty() && s.band.is_empty())
        );
    }
}
