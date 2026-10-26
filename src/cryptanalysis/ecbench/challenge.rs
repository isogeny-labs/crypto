//! Challenges and verdicts: how a frontier moves.
//!
//! A frontier compares bounds that were measured apart, so it can only
//! leave ties standing.  A *challenge* fixes what a candidate must do to
//! replace an incumbent: the domain, the incumbent method, at least four
//! curve sizes, how many targets and rounds, and the acceptance rule.  From
//! a challenge and a candidate, `spec_for` writes the one spec both run
//! in — incumbent, candidate and an A/A control of the incumbent,
//! interleaved, paired on the same workloads and seeds — with the target
//! and algorithm seeds derived from the challenge's nonce and an *epoch*,
//! so nobody can tune to the targets and anyone can rebuild the spec.
//!
//! The *verdict* reads the session back: it audits it (replays included),
//! pairs the arms with `compare`, fits both bounds, and decides on each
//! axis whether the candidate is clearly better (the paired interval
//! excludes 1 from below), clearly worse, or indistinguishable.  Pareto
//! language follows: `advances` (better somewhere, worse nowhere), `trade`
//! (better somewhere, worse somewhere), `matches`, `regresses`; or
//! `inadmissible`, when the session is not the challenge's spec, did not
//! audit clean, has too few sizes or runs, or a measured run did not
//! verify.  An advance names the level it moved: `exponent` when the
//! fitted α intervals are disjoint across four or more sizes, `constant`
//! otherwise when `ops` moved, and `primitive` when `ops` did not move but
//! a field-operation axis (`field_muls`, `field_sqrs`, `field_invs`) did.
//! Counted-but-unpriced work is reported beside the result and decides only
//! when the challenge says so; the field-operation axes are carried when a
//! run of either arm counted them or the challenge names one, and decide
//! only when named, so a verdict over a session that counted nothing is the
//! verdict it always was.
//!
//! An admissible `advances` or `trade` verdict yields the candidate's
//! bound with `improves_on` set, and the frontier is rebuilt from the
//! records.  Nothing here edits a committed record.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::cryptanalysis::ecbench::audit::{audit_with, AuditReport, REPLAY_ALL};
use crate::cryptanalysis::ecbench::bounds::{
    bootstrap_values, bound_from_sessions, field_axis, field_size, memory_entries, read_sealed,
    seal_document, tier_of_bits, uncharged_work, AuditRef, Bound, Domain, Estimate, FitInputs,
    FitOptions, BAND_WEIGHTS, FIELD_AXES, MIN_SIZES_FOR_SCALING, RESAMPLES, VERDICT_AUDIT_PREFIX,
};
use crate::cryptanalysis::ecbench::canonical::{derive_u64, sha256_hex};
use crate::cryptanalysis::ecbench::compare::compare;
use crate::cryptanalysis::ecbench::frontier::{axis_names, is_axis, load_bounds};
use crate::cryptanalysis::ecbench::methods::MethodSpec;
use crate::cryptanalysis::ecbench::record::Record;
use crate::cryptanalysis::ecbench::runner::{read_records, read_session, PlanDoc};
use crate::cryptanalysis::ecbench::spec::{
    plan, ArmSpec, Level, Measurement, Order, Role, Spec, WorkloadPlan, SPEC_SCHEMA,
};
use crate::cryptanalysis::ecbench::stats::{bootstrap_ci, cluster_bootstrap_ci, mean};
use crate::cryptanalysis::ecbench::workload::{CurveSpec, TargetKind};

pub const CHALLENGE_SCHEMA: &str = "ecbench.challenge/v1";
pub const CHALLENGE_PREFIX: &str = "ECCH1h";
pub const VERDICT_SCHEMA: &str = "ecbench.verdict/v1";
pub const VERDICT_PREFIX: &str = "ECVD1h";
/// The verdict procedure a new verdict is written under (README §6).
/// Version 1 is the procedure of the first two committed verdicts: it
/// re-derives them and nothing else.  Version 2 adds the epoch commitment,
/// the recorded bootstrap settings, the incumbent drift check as a
/// condition, Holm's correction over the deciding axes, the pricing band
/// for bounded comparisons, and the other refusals of fit version 2.
pub const VERDICT_PROCEDURE: u32 = 2;
pub const COMMITMENT_SCHEMA: &str = "ecbench.commitment/v1";
pub const COMMITMENT_PREFIX: &str = "ECCM1h";
const EPOCH_LAW: &str = "ecbench.challenge.epoch";
/// The family-wise error rate of the deciding axes (Holm).
pub const HOLM_ALPHA: f64 = 0.05;
pub const INCUMBENT_ARM: &str = "incumbent";
pub const CANDIDATE_ARM: &str = "candidate";
pub const CONTROL_ARM: &str = "incumbent-aa";
const TARGET_SEED_LAW: &str = "ecbench.challenge.target_seed";
const ALGORITHM_SEED_LAW: &str = "ecbench.challenge.algorithm_seed";

fn default_axes() -> Vec<String> {
    vec!["ops".into(), "memory".into()]
}
fn default_min_sizes() -> usize {
    MIN_SIZES_FOR_SCALING
}
fn default_min_runs() -> u64 {
    8
}
fn default_true() -> bool {
    true
}
fn default_tolerance() -> f64 {
    0.05
}

/// What a candidate must do.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Acceptance {
    /// Axes dominance reads: `ops`, `memory`, `uncharged`, and the
    /// primitive-level `field_muls`, `field_sqrs`, `field_invs`.
    #[serde(default = "default_axes")]
    pub axes: Vec<String>,
    /// Sizes the session must cover for an exponent to be read.
    #[serde(default = "default_min_sizes")]
    pub min_sizes: usize,
    /// Verified measured runs of each arm on each size (targets × rounds).
    #[serde(default = "default_min_runs")]
    pub min_runs_per_size: u64,
    /// The session must audit clean …
    #[serde(default = "default_true")]
    pub require_audit: bool,
    /// … with every measured deterministic run replayed.
    #[serde(default = "default_true")]
    pub require_replay_all: bool,
    /// Above `1 + tolerance` the candidate's unpriced work per `√r`,
    /// relative to the incumbent's, is reported as an accounting shift.
    #[serde(default = "default_tolerance")]
    pub uncharged_tolerance: f64,
}

impl Default for Acceptance {
    fn default() -> Self {
        Self {
            axes: default_axes(),
            min_sizes: default_min_sizes(),
            min_runs_per_size: default_min_runs(),
            require_audit: true,
            require_replay_all: true,
            uncharged_tolerance: default_tolerance(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChallengeWorkloads {
    pub curves: Vec<CurveSpec>,
    pub targets_per_curve: u64,
    #[serde(default)]
    pub target_kind: TargetKind,
    /// Mixed with the epoch into the target and algorithm seeds.
    pub nonce: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChallengeMeasurement {
    pub rounds: u32,
    pub warmup: u32,
    pub isolation_required: Level,
    pub timeout_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Incumbent {
    /// The committed bound the incumbent holds, when there is one.
    pub bound_id: Option<String>,
    pub method: MethodSpec,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Challenge {
    pub schema: String,
    pub challenge_id: String,
    pub label: String,
    #[serde(default)]
    pub description: String,
    pub domain: Domain,
    pub domain_id: String,
    pub incumbent: Incumbent,
    pub workloads: ChallengeWorkloads,
    pub measurement: ChallengeMeasurement,
    #[serde(default)]
    pub acceptance: Acceptance,
}

impl Challenge {
    pub fn seal(&mut self) -> Result<String, String> {
        self.challenge_id = String::new();
        let (id, text) = seal_document(self, "challenge_id", CHALLENGE_PREFIX)?;
        self.challenge_id = id;
        Ok(text)
    }

    pub fn read(path: &Path) -> Result<Self, String> {
        let c: Challenge = read_sealed(path, "challenge_id", CHALLENGE_PREFIX)?;
        if c.schema != CHALLENGE_SCHEMA {
            return Err(format!(
                "{}: schema `{}`, expected {CHALLENGE_SCHEMA}",
                path.display(),
                c.schema
            ));
        }
        if c.domain.id()? != c.domain_id {
            return Err(format!(
                "{}: domain_id does not hash from the domain",
                path.display()
            ));
        }
        Ok(c)
    }
}

/// The spec a candidate runs against a challenge in `epoch`.  Everything
/// but the candidate's method is fixed by the challenge, and the seeds by
/// the challenge's nonce and the epoch.
pub fn spec_for(ch: &Challenge, candidate: &MethodSpec, epoch: u64) -> Spec {
    let w = &ch.workloads;
    let m = &ch.measurement;
    Spec {
        schema: SPEC_SCHEMA.into(),
        label: format!(
            "challenge {} epoch {epoch}: {} against {}",
            ch.challenge_id, candidate.id, ch.incumbent.method.id
        ),
        description: format!(
            "Generated by `ecbench challenge spec` from {}: the incumbent, the candidate and an A/A control of the incumbent, interleaved on the challenge's curves with seeds derived from its nonce and epoch {epoch}. Acceptance: {}.",
            ch.challenge_id,
            serde_json::to_string(&ch.acceptance).unwrap_or_default()
        ),
        workloads: WorkloadPlan {
            curves: w.curves.clone(),
            targets_per_curve: w.targets_per_curve,
            target_seed: derive_u64(TARGET_SEED_LAW, &[w.nonce, epoch]),
            target_kind: w.target_kind,
        },
        arms: vec![
            ArmSpec {
                name: INCUMBENT_ARM.into(),
                role: Role::Reference,
                method: ch.incumbent.method.clone(),
            },
            ArmSpec {
                name: CANDIDATE_ARM.into(),
                role: Role::Candidate,
                method: candidate.clone(),
            },
            ArmSpec {
                name: CONTROL_ARM.into(),
                role: Role::Control,
                method: ch.incumbent.method.clone(),
            },
        ],
        measurement: Measurement {
            rounds: m.rounds,
            warmup: m.warmup,
            order: Order::Alternate,
            seed: derive_u64(ALGORITHM_SEED_LAW, &[w.nonce, epoch]),
            isolation_required: m.isolation_required,
            timeout_seconds: m.timeout_seconds,
        },
    }
}

// ── The epoch commitment (procedure 2) ─────────────────────────────

/// A candidate's commitment to a challenge, made *before* the challenge
/// spec exists: the candidate's method, named and sealed.  The epoch is
/// not the candidate's to choose: it is derived from the challenge's
/// nonce, this commitment's id and a *beacon*, a public value that did not
/// exist when the commitment was written (the git commit that first
/// contains the commitment file, a randomness-beacon round), so a candidate
/// cannot try epochs until the targets suit it.  A commitment can be
/// grinded by varying `note`; every commitment a candidate ever made for
/// a challenge therefore belongs in the repository, and the verdict names
/// the one it used.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Commitment {
    pub schema: String,
    pub commitment_id: String,
    pub challenge_id: String,
    pub candidate: MethodSpec,
    /// Milliseconds since the Unix epoch when the commitment was written.
    pub committed_unix_ms: u64,
    #[serde(default)]
    pub note: String,
}

impl Commitment {
    pub fn seal(&mut self) -> Result<String, String> {
        self.commitment_id = String::new();
        let (id, text) = seal_document(self, "commitment_id", COMMITMENT_PREFIX)?;
        self.commitment_id = id;
        Ok(text)
    }

    pub fn read(path: &Path) -> Result<Self, String> {
        let c: Commitment = read_sealed(path, "commitment_id", COMMITMENT_PREFIX)?;
        if c.schema != COMMITMENT_SCHEMA {
            return Err(format!(
                "{}: schema `{}`, expected {COMMITMENT_SCHEMA}",
                path.display(),
                c.schema
            ));
        }
        Ok(c)
    }
}

/// Write a candidate's commitment to a challenge at `now_ms`.
pub fn commit(
    ch: &Challenge,
    candidate: &MethodSpec,
    note: &str,
    now_ms: u64,
) -> Result<(Commitment, String), String> {
    let mut c = Commitment {
        schema: COMMITMENT_SCHEMA.into(),
        commitment_id: String::new(),
        challenge_id: ch.challenge_id.clone(),
        candidate: candidate.clone(),
        committed_unix_ms: now_ms,
        note: note.into(),
    };
    let text = c.seal()?;
    Ok((c, text))
}

/// A beacon is a public value of at least 16 lowercase hex digits.
pub fn check_beacon(beacon: &str) -> Result<(), String> {
    if beacon.len() < 16 || !beacon.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')) {
        return Err(format!(
            "the beacon `{beacon}` must be at least 16 lowercase hex digits (a git commit id made after the commitment, or a randomness-beacon value)"
        ));
    }
    Ok(())
}

/// The epoch a commitment and a beacon give a challenge.  The commitment
/// must be of this challenge.
pub fn epoch_from(ch: &Challenge, c: &Commitment, beacon: &str) -> Result<u64, String> {
    check_beacon(beacon)?;
    if c.challenge_id != ch.challenge_id {
        return Err(format!(
            "the commitment is to challenge {}, not {}",
            c.challenge_id, ch.challenge_id
        ));
    }
    let hex = |h: &str| u64::from_str_radix(&h[..h.len().min(16)], 16).map_err(|e| e.to_string());
    let cid = c
        .commitment_id
        .strip_prefix(COMMITMENT_PREFIX)
        .ok_or("a commitment id starts with ECCM1h")?;
    let b = sha256_hex(beacon.as_bytes());
    Ok(derive_u64(
        EPOCH_LAW,
        &[ch.workloads.nonce, hex(cid)?, hex(&b)?],
    ))
}

/// The spec a committed candidate runs: [`spec_for`] at the derived
/// epoch, with the commitment and beacon in its description.
pub fn spec_for_committed(
    ch: &Challenge,
    candidate: &MethodSpec,
    c: &Commitment,
    beacon: &str,
) -> Result<Spec, String> {
    if &c.candidate != candidate {
        return Err(format!(
            "the commitment names {} {:?}, not the candidate {} {:?}",
            c.candidate.id, c.candidate.params, candidate.id, candidate.params
        ));
    }
    let epoch = epoch_from(ch, c, beacon)?;
    let mut spec = spec_for(ch, candidate, epoch);
    spec.description = format!(
        "{} Epoch {epoch} derived from commitment {} and beacon {beacon} (procedure 2).",
        spec.description, c.commitment_id
    );
    Ok(spec)
}

/// Every refusal a challenge can earn before anything runs: its curves
/// must build, lie in its domain's family and tier, and number at least
/// `acceptance.min_sizes`; the incumbent must resolve.
pub fn validate(ch: &Challenge) -> Result<(), String> {
    if ch.workloads.curves.len() < ch.acceptance.min_sizes {
        return Err(format!(
            "the challenge names {} curve(s) and requires {} sizes",
            ch.workloads.curves.len(),
            ch.acceptance.min_sizes
        ));
    }
    for a in &ch.acceptance.axes {
        if !is_axis(a) {
            return Err(format!(
                "unknown acceptance axis `{a}`; axes are {}",
                axis_names()
            ));
        }
    }
    if !ch.acceptance.axes.iter().any(|a| a == "ops") {
        return Err("acceptance axes must include ops".into());
    }
    let p = plan(spec_for(ch, &ch.incumbent.method, 0))?;
    let kind = format!("{:?}", ch.workloads.target_kind).to_lowercase();
    if kind != ch.domain.target_kind {
        return Err(format!(
            "target kind {kind} is not the domain's {}",
            ch.domain.target_kind
        ));
    }
    for w in &p.workloads {
        if w.curve.family != ch.domain.family {
            return Err(format!(
                "{} is a {} curve; the domain is {}",
                w.curve.slug, w.curve.family, ch.domain.family
            ));
        }
        let bits = if w.curve.family == "prime" {
            w.curve.field_bits
        } else {
            w.curve.field_degree
        }
        .ok_or_else(|| format!("{}: no field size", w.curve.slug))?;
        if tier_of_bits(bits) != ch.domain.tier {
            return Err(format!(
                "{} is {} tier ({bits} bits); the domain is {}",
                w.curve.slug,
                tier_of_bits(bits),
                ch.domain.tier
            ));
        }
    }
    Ok(())
}

// ── The verdict ────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionSummary {
    pub dir: String,
    pub session_id: String,
    pub spec_id: String,
    /// The spec id the challenge yields for this epoch and candidate.
    pub expected_spec_id: String,
    pub spec_matches: bool,
    pub status: String,
    pub binary_sha256: Option<String>,
    pub git_commit: Option<String>,
    pub env_class_id: String,
    pub levels: BTreeMap<String, u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditSummary {
    pub ok: bool,
    pub problems: Vec<String>,
    pub records: u64,
    pub verified_records: u64,
    pub replays: u64,
    pub replays_reproduced: u64,
    pub replay_all: bool,
    /// SHA-256 of every session file, as the audit read them.  The receipt
    /// itself carries a timestamp, so the verdict binds the session bytes
    /// and stays re-derivable.
    pub files: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmInfo {
    pub arm: String,
    pub method_id: String,
    pub method: String,
    pub params: BTreeMap<String, String>,
    pub bound_id: Option<String>,
}

fn is_one(v: &u32) -> bool {
    *v == 1
}
fn one() -> u32 {
    1
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AxisVerdict {
    pub axis: String,
    pub known: bool,
    pub incumbent_mean: Option<f64>,
    pub candidate_mean: Option<f64>,
    /// `Σ candidate / Σ incumbent` over matched pairs.
    pub ratio_candidate_over_incumbent: Option<f64>,
    pub ci95: Option<(f64, f64)>,
    pub ci_method: String,
    pub pairs: u64,
    /// `better`, `worse`, `indistinguishable` or `unknown`.
    pub verdict: String,
    /// Read by the outcome, or reported only.
    pub decides: bool,
    /// Procedure 2: the bootstrap two-sided p-value of the paired ratio
    /// against 1, the Holm threshold it was held to, and the verdict the
    /// interval alone gave before the correction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holm_threshold: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unadjusted_verdict: Option<String>,
}

/// Holm's step-down correction over the deciding axes (procedure 2).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Multiplicity {
    pub method: String,
    pub alpha: f64,
    pub family: Vec<String>,
}

/// The commitment and beacon the epoch was derived from (procedure 2).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentRef {
    pub commitment_id: String,
    pub committed_unix_ms: u64,
    pub beacon: String,
}

/// The `ops` axis of a bounded comparison across the pricing band
/// (AGENTS.md §5a): unpriced work charged at each weight.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BandVerdict {
    pub weights: Vec<f64>,
    pub ratio_candidate_over_incumbent: Vec<Option<f64>>,
    pub ci95: Vec<Option<(f64, f64)>>,
    pub verdicts: Vec<String>,
    /// The price per unpriced operation at which the candidate's and the
    /// incumbent's mean cost cross, when they do at a non-negative price.
    pub break_even_price: Option<f64>,
    /// The same verdict at every weight; `null` when a weight cannot be
    /// priced.
    pub holds_across_band: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurveVerdict {
    pub slug: String,
    pub log2_r: f64,
    pub pairs: u64,
    pub ratio_candidate_over_incumbent: Option<f64>,
    pub ci95: Option<(f64, f64)>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FitSummary {
    pub alpha: Option<f64>,
    pub alpha_ci95: Option<(f64, f64)>,
    pub scaling_claim: bool,
    pub sizes: u64,
    pub ratio_to_floor: Estimate,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fits {
    pub incumbent: FitSummary,
    pub candidate: FitSummary,
    /// The candidate's α interval lies wholly below the incumbent's, both
    /// over enough sizes; `null` when either interval is missing.
    pub exponent_moved: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StageDelta {
    pub name: String,
    pub incumbent_share: f64,
    pub candidate_share: f64,
    /// `Σ candidate phase gae / Σ incumbent phase gae` over matched pairs.
    pub ratio_candidate_over_incumbent: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Accounting {
    pub incumbent_unpriced: Vec<String>,
    pub candidate_unpriced: Vec<String>,
    /// Either arm left work unpriced: every total is a floor.
    pub bounded: bool,
    pub uncharged_ratio: Option<f64>,
    pub tolerance: f64,
    /// The candidate counts more unpriced work per `√r` than the
    /// incumbent by more than the tolerance.
    pub uncharged_shift: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Control {
    pub arm: String,
    pub ratio: Option<f64>,
    pub ci95: Option<(f64, f64)>,
    /// Procedure 2: what the control is.  The incumbent against itself
    /// with shared seeds is identical by construction, so it reads
    /// `1.000 [1.000, 1.000]` whatever the noise: a determinism check of
    /// the pairing, not a noise gate for `ops`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Drift {
    pub recorded_bound_id: String,
    pub recorded_ops: Option<f64>,
    pub recorded_ci95: Option<(f64, f64)>,
    pub fresh_ops: Option<f64>,
    pub fresh_ci95: Option<(f64, f64)>,
    /// The fresh figure lies outside the recorded interval or vice versa.
    pub outside: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Verdict {
    pub schema: String,
    pub verdict_id: String,
    pub challenge_id: String,
    pub epoch: u64,
    pub domain_id: String,
    pub session: SessionSummary,
    pub audit: AuditSummary,
    pub incumbent: ArmInfo,
    pub candidate: ArmInfo,
    pub control: Option<Control>,
    pub acceptance: Acceptance,
    pub axes: Vec<AxisVerdict>,
    pub per_curve: Vec<CurveVerdict>,
    pub fits: Fits,
    pub stages: Vec<StageDelta>,
    pub accounting: Accounting,
    pub incumbent_drift: Option<Drift>,
    /// `advances`, `trade`, `matches`, `regresses` or `inadmissible`.
    pub outcome: String,
    /// The deciding axes the candidate is clearly better on, and worse on.
    pub advances_on: Vec<String>,
    pub regresses_on: Vec<String>,
    /// `exponent` or `constant` when the advance includes `ops`;
    /// `primitive` when it does not and includes a field-operation axis
    /// (fewer multiplications, squarings or inversions per group operation
    /// at the same count of group operations); `null` otherwise: a level
    /// is a statement about operations, and memory alone names none.
    pub level_moved: Option<String>,
    pub reasons: Vec<String>,
    pub statement: String,
    /// Procedure 2 fields; a version-1 verdict carries none of them and is
    /// not written with them.
    #[serde(default = "one", skip_serializing_if = "is_one")]
    pub procedure: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resamples: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commitment: Option<CommitmentRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiplicity: Option<Multiplicity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub band: Option<BandVerdict>,
}

impl Verdict {
    pub fn seal(&mut self) -> Result<String, String> {
        self.verdict_id = String::new();
        let (id, text) = seal_document(self, "verdict_id", VERDICT_PREFIX)?;
        self.verdict_id = id;
        Ok(text)
    }

    pub fn read(path: &Path) -> Result<Self, String> {
        read_sealed(path, "verdict_id", VERDICT_PREFIX)
    }
}

/// `Σ b / Σ a` over matched `(workload, round)` pairs of `f` with its
/// two-stage interval.  `None` for the ratio when either arm lacks `f` on
/// some verified run (unknown is not zero).
fn paired_axis(
    recs: &[Record],
    a_arm: &str,
    b_arm: &str,
    f: impl Fn(&Record) -> Option<f64>,
    resamples: usize,
    seed: u64,
) -> (
    bool,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<(f64, f64)>,
    String,
    u64,
) {
    let measured = |arm: &str| -> Vec<&Record> {
        recs.iter().filter(|r| r.arm == arm && r.counts()).collect()
    };
    let (ma, mb) = (measured(a_arm), measured(b_arm));
    let va: Vec<Option<f64>> = ma.iter().map(|r| f(r)).collect();
    let vb: Vec<Option<f64>> = mb.iter().map(|r| f(r)).collect();
    let known = va.iter().all(Option::is_some) && vb.iter().all(Option::is_some);
    let mean_a = mean(&va.iter().flatten().copied().collect::<Vec<_>>());
    let mean_b = mean(&vb.iter().flatten().copied().collect::<Vec<_>>());
    if !known {
        return (false, mean_a, mean_b, None, None, "none".into(), 0);
    }
    let key = |r: &Record| (r.workload.workload_id.clone(), r.round);
    let bmap: BTreeMap<(String, u32), f64> = mb
        .iter()
        .zip(vb.iter())
        .map(|(r, v)| (key(r), v.unwrap_or(0.0)))
        .collect();
    let mut strata: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
    for (r, v) in ma.iter().zip(va.iter()) {
        if let Some(y) = bmap.get(&key(r)) {
            strata
                .entry(r.workload.workload_id.clone())
                .or_default()
                .push((v.unwrap_or(0.0), *y));
        }
    }
    let strata: Vec<Vec<(f64, f64)>> = strata.into_values().filter(|v| !v.is_empty()).collect();
    let pairs: u64 = strata.iter().map(|s| s.len() as u64).sum();
    let stat = |s: &[Vec<(f64, f64)>]| {
        let (mut sa, mut sb) = (0.0, 0.0);
        for (x, y) in s.iter().flatten() {
            sa += x;
            sb += y;
        }
        (sa > 0.0).then(|| sb / sa)
    };
    let ratio = stat(&strata);
    let (ci, method) = if strata.len() >= 2 {
        (
            cluster_bootstrap_ci(&strata, resamples, seed, stat),
            "cluster",
        )
    } else {
        (bootstrap_ci(&strata, resamples, seed, stat), "within")
    };
    (true, mean_a, mean_b, ratio, ci, method.into(), pairs)
}

/// `better`, `worse`, `indistinguishable` or `unknown` from the paired
/// ratio's interval against 1.  With both means known but no ratio (the
/// incumbent's total is zero), the means decide: more than nothing is
/// worse, nothing against nothing is indistinguishable.
fn axis_verdict(
    ratio: Option<f64>,
    ci: Option<(f64, f64)>,
    known: bool,
    means: (Option<f64>, Option<f64>),
) -> &'static str {
    if !known {
        return "unknown";
    }
    if ratio.is_none() {
        return match means {
            (Some(a), Some(b)) if a == 0.0 && b > 0.0 => "worse",
            (Some(a), Some(b)) if a == 0.0 && b == 0.0 => "indistinguishable",
            _ => "unknown",
        };
    }
    match ci {
        Some((_, hi)) if hi < 1.0 => "better",
        Some((lo, _)) if lo > 1.0 => "worse",
        Some(_) => "indistinguishable",
        None => "unknown",
    }
}

/// The level an advance moved (README §2).  `exponent` or `constant`
/// when `ops` is among the axes advanced on, by whether the fitted `α`
/// intervals moved apart; `primitive` when `ops` is not but a
/// field-operation axis is — the group-operation count held and the
/// field work behind it fell; `None` for any other outcome or an advance
/// on memory alone.
pub fn level_of(
    outcome: &str,
    advances_on: &[String],
    exponent_moved: Option<bool>,
) -> Option<&'static str> {
    if outcome != "advances" {
        return None;
    }
    if advances_on.iter().any(|a| a == "ops") {
        return Some(if exponent_moved == Some(true) {
            "exponent"
        } else {
            "constant"
        });
    }
    if advances_on.iter().any(|a| FIELD_AXES.contains(&a.as_str())) {
        return Some("primitive");
    }
    None
}

/// Pareto outcome over the deciding axes.
pub fn outcome_of(axes: &[AxisVerdict]) -> &'static str {
    let deciding: Vec<&AxisVerdict> = axes.iter().filter(|a| a.decides).collect();
    let better = deciding.iter().any(|a| a.verdict == "better");
    let worse = deciding.iter().any(|a| a.verdict == "worse");
    match (better, worse) {
        (true, false) => "advances",
        (true, true) => "trade",
        (false, true) => "regresses",
        (false, false) => "matches",
    }
}

/// Matched `(workload, round)` pairs of `f` by workload, or `None` when
/// either arm lacks `f` on some verified run (unknown is not zero).
fn paired_strata(
    recs: &[Record],
    a_arm: &str,
    b_arm: &str,
    f: impl Fn(&Record) -> Option<f64>,
) -> Option<Vec<Vec<(f64, f64)>>> {
    let measured = |arm: &str| -> Vec<&Record> {
        recs.iter().filter(|r| r.arm == arm && r.counts()).collect()
    };
    let key = |r: &Record| (r.workload.workload_id.clone(), r.round);
    let mut b: BTreeMap<(String, u32), f64> = BTreeMap::new();
    for r in measured(b_arm) {
        b.insert(key(r), f(r)?);
    }
    let mut strata: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
    for r in measured(a_arm) {
        let x = f(r)?;
        if let Some(y) = b.get(&key(r)) {
            strata
                .entry(r.workload.workload_id.clone())
                .or_default()
                .push((x, *y));
        }
    }
    Some(strata.into_values().filter(|v| !v.is_empty()).collect())
}

fn ratio_of_pairs(s: &[Vec<(f64, f64)>]) -> Option<f64> {
    let (mut sa, mut sb) = (0.0, 0.0);
    for (x, y) in s.iter().flatten() {
        sa += x;
        sb += y;
    }
    (sa > 0.0).then(|| sb / sa)
}

/// The two-sided bootstrap p-value of the paired ratio `Σ b / Σ a` against
/// 1, from the same two-stage resampling as the interval: twice the smaller
/// tail, each counted with the add-one correction.  `None` when `f` is
/// unknown on some run or the bootstrap cannot be drawn.
pub fn paired_p(
    recs: &[Record],
    a_arm: &str,
    b_arm: &str,
    f: impl Fn(&Record) -> Option<f64>,
    resamples: usize,
    seed: u64,
) -> Option<f64> {
    let strata = paired_strata(recs, a_arm, b_arm, f)?;
    if strata.is_empty() {
        return None;
    }
    let values = bootstrap_values(
        &strata,
        resamples,
        seed,
        strata.len() >= 2,
        |_| true,
        ratio_of_pairs,
    )?;
    Some(two_sided_p(&values))
}

/// Twice the smaller tail of the bootstrap ratios about 1, each tail
/// counted with the add-one correction (so `p` is never 0), at most 1.
pub fn two_sided_p(values: &[f64]) -> f64 {
    let n = values.len() as f64;
    let ge = values.iter().filter(|v| **v >= 1.0).count() as f64;
    let le = values.iter().filter(|v| **v <= 1.0).count() as f64;
    (2.0 * ((ge + 1.0) / (n + 1.0)).min((le + 1.0) / (n + 1.0))).min(1.0)
}

/// [`axis_verdict`] for procedure 2: a paired ratio that its own interval
/// excludes is a failed bootstrap and reads `unknown`.
pub fn axis_verdict_v2(
    ratio: Option<f64>,
    ci: Option<(f64, f64)>,
    known: bool,
    means: (Option<f64>, Option<f64>),
) -> &'static str {
    if let (Some(r), Some((lo, hi))) = (ratio, ci) {
        if !(lo <= r && r <= hi) {
            return "unknown";
        }
    }
    axis_verdict(ratio, ci, known, means)
}

/// Holm's step-down procedure over the deciding axes that carry a
/// p-value: sorted ascending, the `i`-th (from 0) of `m` is rejected only
/// while every smaller one was and its p is at most `alpha / (m - i)`.  An
/// axis whose interval said `better` or `worse` and that Holm does not
/// reject reads `indistinguishable`; its interval-only verdict is kept in
/// `unadjusted_verdict`.  The outcome is then [`outcome_of`] as always.
pub fn apply_holm(axes: &mut [AxisVerdict], alpha: f64) -> Multiplicity {
    let mut fam: Vec<(usize, f64)> = axes
        .iter()
        .enumerate()
        .filter(|(_, a)| {
            a.decides
                && a.p_value.is_some()
                && matches!(a.verdict.as_str(), "better" | "worse" | "indistinguishable")
        })
        .map(|(i, a)| (i, a.p_value.unwrap_or(1.0)))
        .collect();
    fam.sort_by(|x, y| x.1.total_cmp(&y.1).then(x.0.cmp(&y.0)));
    let m = fam.len();
    let mut still = true;
    for (rank, (i, p)) in fam.iter().enumerate() {
        let threshold = alpha / (m - rank) as f64;
        let a = &mut axes[*i];
        a.holm_threshold = Some(threshold);
        a.unadjusted_verdict = Some(a.verdict.clone());
        if still && *p <= threshold {
            continue;
        }
        still = false;
        if a.verdict == "better" || a.verdict == "worse" {
            a.verdict = "indistinguishable".into();
        }
    }
    Multiplicity {
        method: "holm".into(),
        alpha,
        family: fam.iter().map(|(i, _)| axes[*i].axis.clone()).collect(),
    }
}

pub struct VerdictInputs<'a> {
    pub challenge: &'a Challenge,
    pub dir: &'a Path,
    pub epoch: u64,
    pub candidate_arm: &'a str,
    pub incumbent_arm: &'a str,
    pub control_arm: Option<&'a str>,
    /// Runs to replay in the audit; `REPLAY_ALL` for every one.
    pub replay: usize,
    /// Committed bound records, to read the incumbent's recorded figure.
    pub bounds: Option<&'a Path>,
    pub root: &'a Path,
    pub resamples: usize,
    pub seed: u64,
    /// 1 re-derives a committed version-1 verdict; 2 is [`VERDICT_PROCEDURE`].
    pub procedure: u32,
    /// Procedure 2: the candidate's commitment and the beacon the epoch
    /// derives from; `epoch` must be what they give.
    pub commitment: Option<&'a Commitment>,
    pub beacon: Option<&'a str>,
}

pub struct VerdictOutput {
    pub verdict: Verdict,
    pub verdict_text: String,
    pub audit: AuditReport,
    pub audit_text: String,
    /// The candidate's bound, sealed, on an admissible advance or trade.
    pub candidate_bound: Option<Bound>,
    pub candidate_bound_text: Option<String>,
}

fn fit_summary(b: &Bound) -> FitSummary {
    FitSummary {
        alpha: b.fit.alpha,
        alpha_ci95: b.fit.alpha_ci95,
        scaling_claim: b.fit.scaling_claim,
        sizes: b.fit.sizes,
        ratio_to_floor: b.constant.ratio_to_floor.clone(),
    }
}

fn relative(root: &Path, p: &Path) -> String {
    let abs = std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    let root = std::fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
    abs.strip_prefix(&root)
        .map(|r| r.display().to_string())
        .unwrap_or_else(|_| p.display().to_string())
}

/// Judge a session against a challenge.
pub fn verdict(inputs: &VerdictInputs) -> Result<VerdictOutput, String> {
    let ch = inputs.challenge;
    validate(ch)?;
    if inputs.procedure == 0 || inputs.procedure > VERDICT_PROCEDURE {
        return Err(format!(
            "verdict procedure {} is not known (1 to {VERDICT_PROCEDURE})",
            inputs.procedure
        ));
    }
    let v2 = inputs.procedure >= 2;
    let dir = inputs.dir;
    let session = read_session(dir)?;
    let plan_doc: PlanDoc = serde_json::from_str(
        &std::fs::read_to_string(dir.join("plan.json"))
            .map_err(|e| format!("{}: {e}", dir.join("plan.json").display()))?,
    )
    .map_err(|e| format!("plan.json: {e}"))?;
    let spec_text = std::fs::read_to_string(dir.join("spec.json"))
        .map_err(|e| format!("{}: {e}", dir.join("spec.json").display()))?;
    let spec = Spec::from_json(&spec_text)?;
    let recs = read_records(dir)?;
    let mut reasons: Vec<String> = Vec::new();

    let arm_info = |name: &str| -> Result<ArmInfo, String> {
        let a = plan_doc
            .arms
            .iter()
            .find(|a| a.name == name)
            .ok_or_else(|| format!("the session has no arm `{name}`"))?;
        Ok(ArmInfo {
            arm: name.into(),
            method_id: a.method.method_id.clone(),
            method: a.method.id.clone(),
            params: a.method.params.clone(),
            bound_id: None,
        })
    };
    let mut incumbent = arm_info(inputs.incumbent_arm)?;
    incumbent.bound_id = ch.incumbent.bound_id.clone();
    let candidate = arm_info(inputs.candidate_arm)?;

    // The session must be the challenge's spec for this epoch, with the
    // candidate's method exactly as the spec wrote it.
    let written = spec
        .arms
        .iter()
        .find(|a| a.name == inputs.candidate_arm)
        .map(|a| a.method.clone())
        .ok_or_else(|| format!("spec.json has no arm `{}`", inputs.candidate_arm))?;
    let expected = if v2 {
        // The epoch is derived from the candidate's commitment and a
        // public beacon, never chosen.
        let (c, beacon) = match (inputs.commitment, inputs.beacon) {
            (Some(c), Some(b)) => (c, b),
            _ => {
                return Err(
                    "procedure 2 derives the epoch from a commitment: pass --commitment and --beacon (or --procedure 1 to re-derive a version-1 verdict)"
                        .into(),
                )
            }
        };
        let derived = epoch_from(ch, c, beacon)?;
        if derived != inputs.epoch {
            return Err(format!(
                "epoch {} is not the epoch {derived} that commitment {} and beacon {beacon} give",
                inputs.epoch, c.commitment_id
            ));
        }
        plan(spec_for_committed(ch, &written, c, beacon)?)?
    } else {
        plan(spec_for(ch, &written, inputs.epoch))?
    };
    let spec_matches = expected.spec_id == session.spec_id;
    if !spec_matches {
        reasons.push(format!(
            "the session ran spec {}, not the challenge's spec {} for epoch {} and candidate {}",
            session.spec_id, expected.spec_id, inputs.epoch, written.id
        ));
    }
    if v2 {
        if let Some(c) = inputs.commitment {
            if c.committed_unix_ms == 0 {
                reasons.push("the commitment carries no timestamp".into());
            }
        }
        if inputs.resamples != RESAMPLES {
            reasons.push(format!(
                "{} bootstrap resamples; an admissible verdict uses the default {RESAMPLES} (the seed, {}, is recorded)",
                inputs.resamples, inputs.seed
            ));
        }
        if inputs.bounds.is_none() || ch.incumbent.bound_id.is_none() {
            reasons.push(
                "the incumbent drift check was not run: an admissible verdict needs --bounds and a challenge that names the incumbent's committed bound"
                    .into(),
            );
        }
    }
    let mut levels = BTreeMap::new();
    for r in recs.iter().filter(|r| !r.warmup) {
        *levels
            .entry(r.isolation.level.name().to_string())
            .or_insert(0u64) += 1;
    }
    let session_summary = SessionSummary {
        dir: relative(inputs.root, dir),
        session_id: session.session_id.clone(),
        spec_id: session.spec_id.clone(),
        expected_spec_id: expected.spec_id.clone(),
        spec_matches,
        status: session.status.clone(),
        binary_sha256: session.binary_sha256.clone(),
        git_commit: session.git_commit.clone(),
        env_class_id: session.env_class_id.clone(),
        levels,
    };
    if session.status != "complete" {
        reasons.push(format!("the session is {}", session.status));
    }

    // Audit, replays included.
    let audit = audit_with(dir, inputs.replay, false)?;
    let audit_text = serde_json::to_string_pretty(&audit).map_err(|e| e.to_string())? + "\n";
    let replay_all = inputs.replay == REPLAY_ALL;
    let audit_summary = AuditSummary {
        ok: audit.ok,
        problems: audit.problems.clone(),
        records: audit.records,
        verified_records: audit.verified_records,
        replays: audit.replays.len() as u64,
        replays_reproduced: audit.replays.iter().filter(|r| r.reproduced).count() as u64,
        replay_all,
        files: audit.files.clone(),
    };
    if ch.acceptance.require_audit && !audit.ok {
        reasons.push(format!(
            "the audit found {} problem(s)",
            audit.problems.len()
        ));
    }
    if ch.acceptance.require_replay_all && !replay_all {
        reasons.push("acceptance requires every measured run replayed (--replay-all)".into());
    }
    if audit_summary.replays_reproduced < audit_summary.replays {
        reasons.push(format!(
            "{} of {} replays did not reproduce",
            audit_summary.replays - audit_summary.replays_reproduced,
            audit_summary.replays
        ));
    }

    // Paired operations, from `compare`.
    let cmp = compare(
        dir,
        inputs.incumbent_arm,
        None,
        inputs.candidate_arm,
        inputs.resamples,
        inputs.seed,
    )?;
    if cmp.ops.status != "ok" {
        reasons.push(format!(
            "the comparison is {}: {}",
            cmp.ops.status,
            match cmp.ops.status.as_str() {
                "incomplete" => "a measured run did not verify",
                "partial" => "some runs have no partner",
                _ => "no pairs",
            }
        ));
    }

    // Bounds of both arms from this session.
    let fit_opts = FitOptions {
        resamples: inputs.resamples,
        seed: inputs.seed,
        fit_version: if v2 {
            crate::cryptanalysis::ecbench::bounds::FIT_VERSION
        } else {
            1
        },
        ..Default::default()
    };
    let dirs = [dir.to_path_buf()];
    // The verdict audited the session itself; the bounds cite that audit
    // by the session bytes it read (the receipt carries a timestamp).
    let own_audit: Vec<AuditRef> = if v2 {
        vec![AuditRef {
            path: format!("{VERDICT_AUDIT_PREFIX}{}", session.session_id),
            sha256: sha256_hex(
                serde_json::to_string(&audit.files)
                    .map_err(|e| e.to_string())?
                    .as_bytes(),
            ),
            session_id: session.session_id.clone(),
            ok: audit.ok,
            replays: audit_summary.replays,
            replays_reproduced: audit_summary.replays_reproduced,
        }]
    } else {
        vec![]
    };
    let fit = |arm: &str, label: String| {
        bound_from_sessions(&FitInputs {
            dirs: &dirs,
            arm,
            audits: &[],
            label: Some(&label),
            notes: "",
            root: inputs.root,
            options: fit_opts.clone(),
            audit_refs: &own_audit,
        })
    };
    let inc_bound = fit(
        inputs.incumbent_arm,
        format!(
            "{} as incumbent in challenge {} epoch {}",
            incumbent.method, ch.challenge_id, inputs.epoch
        ),
    )?;
    let cand_bound = fit(
        inputs.candidate_arm,
        format!(
            "{} against {} in challenge {} epoch {}",
            candidate.method, incumbent.method, ch.challenge_id, inputs.epoch
        ),
    )?;
    if cand_bound.domain_id != ch.domain_id {
        reasons.push(format!(
            "the session's domain {} is not the challenge's {}",
            cand_bound.domain_id, ch.domain_id
        ));
    }
    let sizes = cand_bound.fit.sizes as usize;
    if sizes < ch.acceptance.min_sizes {
        reasons.push(format!(
            "{sizes} size(s) verified; the challenge requires {}",
            ch.acceptance.min_sizes
        ));
    }
    for b in [&inc_bound, &cand_bound] {
        for s in &b.sizes {
            if s.verified < ch.acceptance.min_runs_per_size {
                reasons.push(format!(
                    "{} has {} verified runs on {}; the challenge requires {}",
                    b.method.id, s.verified, s.slug, ch.acceptance.min_runs_per_size
                ));
            }
        }
    }
    // A tier is a property of the field size; the domain carried it, and
    // the fit refused mixed tiers, so one check per size suffices.
    for r in recs.iter().filter(|r| !r.warmup) {
        if let Some(bits) = field_size(r) {
            if tier_of_bits(bits) != ch.domain.tier {
                reasons.push(format!(
                    "{} is {} tier; the challenge is {}",
                    r.workload.curve.slug,
                    tier_of_bits(bits),
                    ch.domain.tier
                ));
                break;
            }
        }
    }

    // Axes.
    let av = if v2 { axis_verdict_v2 } else { axis_verdict };
    let decides = |name: &str| ch.acceptance.axes.iter().any(|a| a == name);
    let mut axes = vec![AxisVerdict {
        axis: "ops".into(),
        known: cmp.ops.ratio_b_over_a.is_some(),
        incumbent_mean: cmp.a.mean_ratio_to_floor,
        candidate_mean: cmp.b.mean_ratio_to_floor,
        ratio_candidate_over_incumbent: cmp.ops.ratio_b_over_a,
        ci95: cmp.ops.ci95,
        ci_method: cmp.ops.ci_method.clone(),
        pairs: cmp.ops.pairs,
        verdict: av(
            cmp.ops.ratio_b_over_a,
            cmp.ops.ci95,
            cmp.ops.ratio_b_over_a.is_some(),
            (cmp.a.mean_ratio_to_floor, cmp.b.mean_ratio_to_floor),
        )
        .into(),
        decides: true,
        p_value: None,
        holm_threshold: None,
        unadjusted_verdict: None,
    }];
    // Memory and unpriced work as before.  The field-operation axes ride
    // on a verdict only when a measured run of either arm counted them or
    // the challenge names one: a session that counted nothing under a
    // challenge that asked nothing gives the verdict it always gave, so a
    // committed verdict re-derives byte for byte.  Named but uncounted, an
    // axis is `unknown` and decides nothing (unknown is not zero).  Their
    // seeds are distinct from the first two axes' (`seed ^ name.len()`: 6
    // and 9), which stay as they were.
    let mut reads: Vec<(&str, Box<dyn Fn(&Record) -> Option<f64>>, u64)> = vec![
        ("memory", Box::new(memory_entries), inputs.seed ^ 6),
        (
            "uncharged",
            Box::new(|r: &Record| Some(uncharged_work(r))),
            inputs.seed ^ 9,
        ),
    ];
    let field_counted = recs.iter().any(|r| {
        !r.warmup
            && (r.arm == inputs.incumbent_arm || r.arm == inputs.candidate_arm)
            && r.field_ops.is_some()
    });
    let field_named = ch
        .acceptance
        .axes
        .iter()
        .any(|a| FIELD_AXES.contains(&a.as_str()));
    if field_counted || field_named {
        for (i, name) in FIELD_AXES.iter().enumerate() {
            reads.push((
                name,
                Box::new(move |r: &Record| field_axis(r, name)),
                inputs.seed ^ (11 + i as u64),
            ));
        }
    }
    for (name, f, seed) in reads {
        let (known, ma, mb, ratio, ci, method, pairs) = paired_axis(
            &recs,
            inputs.incumbent_arm,
            inputs.candidate_arm,
            |r| f(r),
            inputs.resamples,
            seed,
        );
        axes.push(AxisVerdict {
            axis: name.into(),
            known,
            incumbent_mean: ma,
            candidate_mean: mb,
            ratio_candidate_over_incumbent: ratio,
            ci95: ci,
            ci_method: method,
            pairs,
            verdict: av(ratio, ci, known, (ma, mb)).into(),
            decides: decides(name),
            p_value: None,
            holm_threshold: None,
            unadjusted_verdict: None,
        });
    }

    // Procedure 2: p-values for the deciding axes, the pricing band for a
    // bounded comparison, then Holm's correction over the family.
    let bounded_pair = inc_bound.admissibility.bounded || cand_bound.admissibility.bounded;
    let mut multiplicity: Option<Multiplicity> = None;
    let mut band_verdict: Option<BandVerdict> = None;
    if v2 {
        let p_of = |f: &dyn Fn(&Record) -> Option<f64>, salt: u64| {
            paired_p(
                &recs,
                inputs.incumbent_arm,
                inputs.candidate_arm,
                f,
                inputs.resamples,
                inputs.seed ^ salt,
            )
        };
        if bounded_pair {
            let mut ratios = Vec::new();
            let mut cis = Vec::new();
            let mut verdicts = Vec::new();
            let mut means = Vec::new();
            let mut ps = Vec::new();
            let mut all_known = true;
            for (i, w) in BAND_WEIGHTS.iter().enumerate() {
                let f = move |r: &Record| {
                    crate::cryptanalysis::ecbench::band::priced_s(r, *w)
                        .ok()
                        .flatten()
                };
                let (known, ma, mb, ratio, ci, _, _) = paired_axis(
                    &recs,
                    inputs.incumbent_arm,
                    inputs.candidate_arm,
                    f,
                    inputs.resamples,
                    inputs.seed ^ (0x31 + i as u64),
                );
                all_known &= known && ratio.is_some();
                verdicts.push(axis_verdict_v2(ratio, ci, known, (ma, mb)).to_string());
                ratios.push(ratio);
                cis.push(ci);
                means.push((ma, mb));
                ps.push(p_of(&f, 0x41 + i as u64));
            }
            let holds = all_known.then(|| verdicts.iter().all(|v| v == &verdicts[0]));
            let break_even = match (means.first(), means.last()) {
                (Some((Some(a0), Some(b0))), Some((Some(a1), Some(b1)))) => {
                    let (d0, d1) = (b0 - a0, b1 - a1);
                    ((d0 - d1).abs() > 1e-12 && d0 / (d0 - d1) >= 0.0).then(|| d0 / (d0 - d1))
                }
                _ => None,
            };
            let ops = &mut axes[0];
            if all_known {
                ops.verdict = if verdicts.iter().all(|v| v == "better") {
                    "better"
                } else if verdicts.iter().all(|v| v == "worse") {
                    "worse"
                } else {
                    "indistinguishable"
                }
                .into();
                ops.p_value = ps
                    .iter()
                    .copied()
                    .collect::<Option<Vec<f64>>>()
                    .map(|v| v.into_iter().fold(0.0, f64::max));
            } else {
                ops.verdict = "unknown".into();
                reasons.push(
                    "a bounded comparison cannot be ranked above w = 0: an unpriced counter has no count on some run (AGENTS.md §5a)"
                        .into(),
                );
            }
            band_verdict = Some(BandVerdict {
                weights: BAND_WEIGHTS.to_vec(),
                ratio_candidate_over_incumbent: ratios,
                ci95: cis,
                verdicts,
                break_even_price: break_even,
                holds_across_band: holds,
            });
        } else {
            axes[0].p_value = p_of(&|r: &Record| r.cost.s, 0x40);
        }
        for (i, a) in axes.iter_mut().enumerate().skip(1) {
            if !a.decides {
                continue;
            }
            a.p_value = match a.axis.as_str() {
                "memory" => p_of(&memory_entries, 0x50),
                "uncharged" => p_of(&|r: &Record| Some(uncharged_work(r)), 0x51),
                name => FIELD_AXES.iter().position(|f| *f == name).and_then(|j| {
                    p_of(
                        &move |r: &Record| field_axis(r, FIELD_AXES[j]),
                        0x52 + j as u64,
                    )
                }),
            };
            let _ = i;
        }
        multiplicity = Some(apply_holm(&mut axes, HOLM_ALPHA));
        if axes[0].verdict == "unknown" && !reasons.iter().any(|r| r.contains("bounded comparison"))
        {
            reasons.push(
                "the ops comparison is unknown (no paired ratio, or a ratio its own interval excludes)"
                    .into(),
            );
        }
    }

    // Accounting.
    let unch = axes.iter().find(|a| a.axis == "uncharged").cloned();
    let uncharged_ratio = unch.as_ref().and_then(|a| a.ratio_candidate_over_incumbent);
    let accounting = Accounting {
        incumbent_unpriced: inc_bound.admissibility.unpriced.clone(),
        candidate_unpriced: cand_bound.admissibility.unpriced.clone(),
        bounded: inc_bound.admissibility.bounded || cand_bound.admissibility.bounded,
        uncharged_ratio,
        tolerance: ch.acceptance.uncharged_tolerance,
        uncharged_shift: uncharged_ratio
            .is_some_and(|x| x > 1.0 + ch.acceptance.uncharged_tolerance)
            || (unch.as_ref().is_some_and(|a| a.incumbent_mean == Some(0.0))
                && unch
                    .as_ref()
                    .and_then(|a| a.candidate_mean)
                    .is_some_and(|m| m > 0.0)),
    };

    // Stages: shares from each bound, ratio over matched pairs.
    let mut names: Vec<String> = Vec::new();
    for s in inc_bound.stages.iter().chain(cand_bound.stages.iter()) {
        if !names.contains(&s.name) {
            names.push(s.name.clone());
        }
    }
    let share = |b: &Bound, n: &str| {
        b.stages
            .iter()
            .find(|s| s.name == n)
            .map(|s| s.share_of_gae)
            .unwrap_or(0.0)
    };
    let mut stages = Vec::new();
    for n in names {
        let phase = |r: &Record| -> Option<f64> {
            Some(r.phases.iter().filter(|p| p.name == n).map(|p| p.gae).sum())
        };
        let (_, _, _, ratio, _, _, _) = paired_axis(
            &recs,
            inputs.incumbent_arm,
            inputs.candidate_arm,
            phase,
            16,
            inputs.seed,
        );
        stages.push(StageDelta {
            name: n.clone(),
            incumbent_share: share(&inc_bound, &n),
            candidate_share: share(&cand_bound, &n),
            ratio_candidate_over_incumbent: ratio,
        });
    }

    // Control: the incumbent against itself, a determinism check.
    let control = inputs
        .control_arm
        .filter(|c| plan_doc.arms.iter().any(|a| &a.name == c))
        .map(|c| {
            compare(
                dir,
                inputs.incumbent_arm,
                None,
                c,
                inputs.resamples,
                inputs.seed,
            )
            .map(|k| Control {
                arm: c.to_string(),
                ratio: k.ops.ratio_b_over_a,
                ci95: k.ops.ci95,
                purpose: v2.then(|| {
                    "determinism check: the incumbent against itself with shared seeds is identical by construction, so this reads 1.000 [1.000, 1.000] whatever the noise; it is not a noise gate for ops".to_string()
                }),
            })
        })
        .transpose()?;

    // Fits and the level moved.
    let fits = Fits {
        incumbent: fit_summary(&inc_bound),
        candidate: fit_summary(&cand_bound),
        exponent_moved: match (
            inc_bound.fit.scaling_claim && cand_bound.fit.scaling_claim,
            inc_bound.fit.alpha_ci95,
            cand_bound.fit.alpha_ci95,
        ) {
            (true, Some((ilo, _)), Some((_, chi))) => Some(chi < ilo),
            _ => None,
        },
    };

    // The incumbent's recorded figure against what it measured here.
    let incumbent_drift = match (&ch.incumbent.bound_id, inputs.bounds) {
        (Some(id), Some(bdir)) => {
            let all = load_bounds(&[bdir.to_path_buf()])?;
            let rec = all
                .iter()
                .find(|b| &b.bound_id == id)
                .ok_or_else(|| format!("incumbent bound {id} is not under {}", bdir.display()))?;
            let (ro, rci) = rec.ops().map(|d| (d.value, d.ci95)).unwrap_or((None, None));
            let fresh = inc_bound.ops().and_then(|d| d.value);
            let fci = inc_bound.ops().and_then(|d| d.ci95);
            let outside = match (ro, rci, fresh, fci) {
                (Some(r), Some((rlo, rhi)), Some(f), Some((flo, fhi))) => {
                    Some(f < rlo || f > rhi || r < flo || r > fhi)
                }
                _ => None,
            };
            if outside == Some(true) {
                reasons.push(format!(
                    "the incumbent measured {:.3}× floor here against {:.3}× recorded in {id}: the recorded bound and this session disagree",
                    fresh.unwrap_or(f64::NAN),
                    ro.unwrap_or(f64::NAN)
                ));
            }
            Some(Drift {
                recorded_bound_id: id.clone(),
                recorded_ops: ro,
                recorded_ci95: rci,
                fresh_ops: fresh,
                fresh_ci95: fci,
                outside,
            })
        }
        _ => None,
    };

    // Outcome.
    let admissible = reasons.is_empty()
        && inc_bound.admissible()
        && cand_bound.admissible()
        && spec_matches
        && cmp.ops.status == "ok";
    for b in [&inc_bound, &cand_bound] {
        for r in &b.admissibility.reasons {
            if !r.contains("exponent is descriptive") && !reasons.contains(r) {
                reasons.push(format!("{}: {r}", b.method.id));
            }
        }
    }
    let outcome = if admissible {
        outcome_of(&axes)
    } else {
        "inadmissible"
    };
    let on = |v: &str| -> Vec<String> {
        axes.iter()
            .filter(|a| a.decides && a.verdict == v)
            .map(|a| a.axis.clone())
            .collect()
    };
    let (advances_on, regresses_on) = if outcome == "inadmissible" {
        (vec![], vec![])
    } else {
        (on("better"), on("worse"))
    };
    let level_moved = level_of(outcome, &advances_on, fits.exponent_moved).map(String::from);
    let ops = &axes[0];
    let fmt_ci = |c: Option<(f64, f64)>| {
        c.map(|(l, h)| format!(" [{l:.3}, {h:.3}]"))
            .unwrap_or_default()
    };
    let statement = match outcome {
        "inadmissible" => format!(
            "inadmissible: {}. The paired ratio {} / {} = {}{} is reported for information only.",
            reasons.join("; "),
            candidate.method,
            incumbent.method,
            ops.ratio_candidate_over_incumbent.map(|r| format!("{r:.4}")).unwrap_or_else(|| "unknown".into()),
            fmt_ci(ops.ci95)
        ),
        _ => format!(
            "{outcome}{}{}: {} / {} = {}{} in {} over {} pairs on {} size(s) ({}); {}; α {} against {}{}{}",
            if advances_on.is_empty() && regresses_on.is_empty() {
                String::new()
            } else {
                format!(
                    " ({}{}{})",
                    if advances_on.is_empty() { String::new() } else { format!("better on {}", advances_on.join(", ")) },
                    if !advances_on.is_empty() && !regresses_on.is_empty() { "; " } else { "" },
                    if regresses_on.is_empty() { String::new() } else { format!("worse on {}", regresses_on.join(", ")) },
                )
            },
            level_moved.as_ref().map(|l| format!(" at the {l} level")).unwrap_or_default(),
            candidate.method,
            incumbent.method,
            ops.ratio_candidate_over_incumbent.map(|r| format!("{r:.4}")).unwrap_or_else(|| "unknown".into()),
            fmt_ci(ops.ci95),
            ch.domain.unit,
            ops.pairs,
            sizes,
            axes.iter().filter(|a| a.axis != "ops").map(|a| format!("{} {}{}", a.axis, a.verdict, if a.decides { "" } else { " (reported, not deciding)" })).collect::<Vec<_>>().join(", "),
            if accounting.bounded { "bounded: unpriced work on one or both arms" } else { "fully priced" },
            cand_bound.fit.alpha.map(|a| format!("{a:.3}")).unwrap_or_else(|| "–".into()),
            inc_bound.fit.alpha.map(|a| format!("{a:.3}")).unwrap_or_else(|| "–".into()),
            if accounting.uncharged_shift { "; the candidate counts more unpriced work per √r than the incumbent" } else { "" },
            control.as_ref().and_then(|c| c.ratio.map(|r| format!("; A/A {} {r:.4}{}", if v2 { "determinism check" } else { "control" }, fmt_ci(c.ci95)))).unwrap_or_default(),
        ),
    };
    let mut v = Verdict {
        schema: VERDICT_SCHEMA.into(),
        verdict_id: String::new(),
        challenge_id: ch.challenge_id.clone(),
        epoch: inputs.epoch,
        domain_id: ch.domain_id.clone(),
        session: session_summary,
        audit: audit_summary,
        incumbent,
        candidate,
        control,
        acceptance: ch.acceptance.clone(),
        axes,
        per_curve: cmp
            .curves
            .iter()
            .map(|k| CurveVerdict {
                slug: k.slug.clone(),
                log2_r: k.log2_r,
                pairs: k.pairs,
                ratio_candidate_over_incumbent: k.ratio_b_over_a,
                ci95: k.ci95,
            })
            .collect(),
        fits,
        stages,
        accounting,
        incumbent_drift,
        outcome: outcome.into(),
        advances_on,
        regresses_on,
        level_moved,
        reasons,
        statement,
        procedure: inputs.procedure,
        resamples: v2.then_some(inputs.resamples),
        seed: v2.then_some(inputs.seed),
        commitment: if v2 {
            inputs
                .commitment
                .zip(inputs.beacon)
                .map(|(c, b)| CommitmentRef {
                    commitment_id: c.commitment_id.clone(),
                    committed_unix_ms: c.committed_unix_ms,
                    beacon: b.to_string(),
                })
        } else {
            None
        },
        multiplicity,
        band: band_verdict,
    };
    let verdict_text = v.seal()?;

    let (candidate_bound, candidate_bound_text) = if matches!(outcome, "advances" | "trade") {
        let mut b = cand_bound;
        // A trade is better somewhere and worse somewhere: it improves on
        // nothing (procedure 1 set `improves_on` on a trade too).
        if !v2 || outcome == "advances" {
            b.improves_on = ch.incumbent.bound_id.iter().cloned().collect();
        }
        b.verdict_id = Some(v.verdict_id.clone());
        let text = b.seal()?;
        (Some(b), Some(text))
    } else {
        (None, None)
    };
    Ok(VerdictOutput {
        verdict: v,
        verdict_text,
        audit,
        audit_text,
        candidate_bound,
        candidate_bound_text,
    })
}

/// Build a challenge document from its parts and seal it.
pub fn new_challenge(
    label: &str,
    description: &str,
    domain: Domain,
    incumbent: Incumbent,
    workloads: ChallengeWorkloads,
    measurement: ChallengeMeasurement,
    acceptance: Acceptance,
) -> Result<(Challenge, String), String> {
    let domain_id = domain.id()?;
    let mut c = Challenge {
        schema: CHALLENGE_SCHEMA.into(),
        challenge_id: String::new(),
        label: label.into(),
        description: description.into(),
        domain,
        domain_id,
        incumbent,
        workloads,
        measurement,
        acceptance,
    };
    validate(&c)?;
    let text = c.seal()?;
    Ok((c, text))
}

/// Every `*.json` challenge under `paths`.
pub fn load_challenges(paths: &[PathBuf]) -> Result<Vec<Challenge>, String> {
    let mut files: Vec<PathBuf> = Vec::new();
    for p in paths {
        if p.is_dir() {
            let mut inner: Vec<PathBuf> = std::fs::read_dir(p)
                .map_err(|e| format!("{}: {e}", p.display()))?
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|q| q.extension().and_then(|x| x.to_str()) == Some("json"))
                .collect();
            inner.sort();
            files.extend(inner);
        } else {
            files.push(p.clone());
        }
    }
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for f in files {
        let c = Challenge::read(&f)?;
        if !seen.insert(c.challenge_id.clone()) {
            return Err(format!(
                "{}: challenge {} appears twice",
                f.display(),
                c.challenge_id
            ));
        }
        out.push(c);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ecbench::bounds::{Envelope, PROBLEM};

    fn axis(name: &str, verdict: &str, decides: bool) -> AxisVerdict {
        AxisVerdict {
            axis: name.into(),
            known: verdict != "unknown",
            incumbent_mean: None,
            candidate_mean: None,
            ratio_candidate_over_incumbent: None,
            ci95: None,
            ci_method: "none".into(),
            pairs: 0,
            verdict: verdict.into(),
            decides,
            p_value: None,
            holm_threshold: None,
            unadjusted_verdict: None,
        }
    }

    #[test]
    fn the_outcome_is_pareto_over_deciding_axes_only() {
        assert_eq!(
            outcome_of(&[
                axis("ops", "better", true),
                axis("memory", "indistinguishable", true)
            ]),
            "advances"
        );
        assert_eq!(
            outcome_of(&[axis("ops", "better", true), axis("memory", "worse", true)]),
            "trade"
        );
        assert_eq!(
            outcome_of(&[
                axis("ops", "indistinguishable", true),
                axis("memory", "worse", true)
            ]),
            "regresses"
        );
        assert_eq!(
            outcome_of(&[
                axis("ops", "indistinguishable", true),
                axis("memory", "unknown", true)
            ]),
            "matches"
        );
        // A reported axis never decides.
        assert_eq!(
            outcome_of(&[
                axis("ops", "better", true),
                axis("uncharged", "worse", false)
            ]),
            "advances"
        );
    }

    #[test]
    fn the_level_is_named_from_the_axes_advanced_on() {
        let v = |xs: &[&str]| xs.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(
            level_of("advances", &v(&["ops"]), Some(true)),
            Some("exponent")
        );
        assert_eq!(
            level_of("advances", &v(&["ops"]), Some(false)),
            Some("constant")
        );
        assert_eq!(level_of("advances", &v(&["ops"]), None), Some("constant"));
        // Operations held, the field work behind them fell: primitive.
        assert_eq!(
            level_of("advances", &v(&["field_sqrs"]), None),
            Some("primitive")
        );
        assert_eq!(
            level_of(
                "advances",
                &v(&["memory", "field_muls", "field_invs"]),
                Some(true)
            ),
            Some("primitive")
        );
        // Ops among the axes names its own level even when field axes moved too.
        assert_eq!(
            level_of("advances", &v(&["ops", "field_sqrs"]), Some(false)),
            Some("constant")
        );
        // Memory alone names no level; nothing but an advance does.
        assert_eq!(level_of("advances", &v(&["memory"]), Some(true)), None);
        assert_eq!(level_of("trade", &v(&["ops"]), Some(true)), None);
        assert_eq!(level_of("matches", &[], None), None);
        assert_eq!(level_of("inadmissible", &v(&["field_sqrs"]), None), None);
    }

    #[test]
    fn a_challenge_may_name_a_field_axis_and_nothing_else_new() {
        let mut c = challenge();
        c.acceptance.axes = vec!["ops".into(), "field_sqrs".into()];
        validate(&c).unwrap();
        c.acceptance.axes = vec!["ops".into(), "field_cubes".into()];
        assert!(validate(&c)
            .unwrap_err()
            .contains("unknown acceptance axis"));
    }

    #[test]
    fn axis_verdicts_read_the_interval_against_one() {
        let m = (Some(1.0), Some(0.7));
        assert_eq!(axis_verdict(Some(0.7), Some((0.6, 0.8)), true, m), "better");
        assert_eq!(axis_verdict(Some(1.3), Some((1.1, 1.5)), true, m), "worse");
        assert_eq!(
            axis_verdict(Some(0.95), Some((0.8, 1.1)), true, m),
            "indistinguishable"
        );
        assert_eq!(axis_verdict(Some(0.7), None, true, m), "unknown");
        assert_eq!(
            axis_verdict(Some(0.7), Some((0.6, 0.8)), false, m),
            "unknown"
        );
        // No ratio because the incumbent counted nothing: the means decide.
        assert_eq!(
            axis_verdict(None, None, true, (Some(0.0), Some(1.1))),
            "worse"
        );
        assert_eq!(
            axis_verdict(None, None, true, (Some(0.0), Some(0.0))),
            "indistinguishable"
        );
    }

    fn challenge() -> Challenge {
        let domain = Domain {
            problem: PROBLEM.into(),
            family: "prime".into(),
            target_kind: "planted".into(),
            unit: "ecbench.gae".into(),
            tier: "toy".into(),
            envelope: Envelope::default(),
        };
        let (c, _) = new_challenge(
            "t",
            "",
            domain,
            Incumbent {
                bound_id: None,
                method: MethodSpec {
                    id: "rho.negation".into(),
                    params: BTreeMap::new(),
                },
            },
            ChallengeWorkloads {
                curves: (0..4)
                    .map(|i| CurveSpec::PrimeSearch {
                        bits: 12 + 2 * i,
                        seed: 59297,
                    })
                    .collect(),
                targets_per_curve: 2,
                target_kind: TargetKind::Planted,
                nonce: 77,
            },
            ChallengeMeasurement {
                rounds: 1,
                warmup: 0,
                isolation_required: Level::L0,
                timeout_seconds: 60,
            },
            Acceptance::default(),
        )
        .unwrap();
        c
    }

    #[test]
    fn the_spec_is_a_function_of_challenge_candidate_and_epoch() {
        let c = challenge();
        assert!(c.challenge_id.starts_with(CHALLENGE_PREFIX));
        let cand = MethodSpec {
            id: "rho.plain".into(),
            params: BTreeMap::new(),
        };
        let a = plan(spec_for(&c, &cand, 1)).unwrap();
        let b = plan(spec_for(&c, &cand, 1)).unwrap();
        let other = plan(spec_for(&c, &cand, 2)).unwrap();
        assert_eq!(a.spec_id, b.spec_id);
        assert_ne!(a.spec_id, other.spec_id);
        assert_ne!(
            a.spec.workloads.target_seed,
            other.spec.workloads.target_seed
        );
        let names: Vec<&str> = a.arms.iter().map(|x| x.name.as_str()).collect();
        assert_eq!(names, vec![INCUMBENT_ARM, CANDIDATE_ARM, CONTROL_ARM]);
        assert_eq!(a.arms[0].method.method_id, a.arms[2].method.method_id);
    }

    #[test]
    fn a_challenge_outside_its_tier_or_family_is_refused() {
        let mut c = challenge();
        c.domain.tier = "medium".into();
        assert!(validate(&c).unwrap_err().contains("tier"));
        let mut c = challenge();
        c.domain.family = "koblitz".into();
        assert!(
            validate(&c).unwrap_err().contains("family")
                || validate(&c).unwrap_err().contains("curve")
        );
        let mut c = challenge();
        c.workloads.curves.truncate(3);
        assert!(validate(&c).unwrap_err().contains("requires 4 sizes"));
    }
    // ── Procedure 2 ────────────────────────────────────────────────

    fn tested(name: &str, verdict: &str, p: f64) -> AxisVerdict {
        let mut a = axis(name, verdict, true);
        a.p_value = Some(p);
        a
    }

    #[test]
    fn holm_corrects_the_per_axis_tests_and_records_the_family() {
        // Three deciding axes, each "better" by its own 95 % interval; one
        // of them (p = 0.03) is not better after the correction, and the
        // outcome follows.
        let mut axes = vec![
            tested("ops", "better", 0.001),
            tested("memory", "better", 0.03),
            tested("field_muls", "indistinguishable", 0.4),
        ];
        assert_eq!(outcome_of(&axes), "advances");
        let m = apply_holm(&mut axes, HOLM_ALPHA);
        assert_eq!(m.method, "holm");
        assert_eq!(m.family.len(), 3);
        // Sorted p: 0.001 <= 0.05/3 (kept), 0.03 > 0.05/2 (not rejected).
        assert_eq!(axes[0].verdict, "better");
        assert_eq!(axes[1].verdict, "indistinguishable");
        assert_eq!(axes[1].unadjusted_verdict.as_deref(), Some("better"));
        assert!((axes[1].holm_threshold.unwrap() - 0.025).abs() < 1e-12);
        assert_eq!(outcome_of(&axes), "advances");
        // Without ops carrying it, the corrected outcome changes.
        let mut axes = vec![
            tested("ops", "better", 0.03),
            tested("memory", "better", 0.04),
        ];
        assert_eq!(outcome_of(&axes), "advances");
        apply_holm(&mut axes, HOLM_ALPHA);
        assert_eq!(outcome_of(&axes), "matches");
        // A single deciding axis is held to the full alpha.
        let mut one = vec![tested("ops", "better", 0.04)];
        apply_holm(&mut one, HOLM_ALPHA);
        assert_eq!(one[0].verdict, "better");
        // Reported axes are outside the family.
        let mut rep = vec![tested("ops", "better", 0.01), {
            let mut a = tested("uncharged", "worse", 0.9);
            a.decides = false;
            a
        }];
        let m = apply_holm(&mut rep, HOLM_ALPHA);
        assert_eq!(m.family, vec!["ops".to_string()]);
        assert_eq!(rep[1].verdict, "worse");
    }

    #[test]
    fn the_bootstrap_p_value_is_a_two_sided_tail_about_one() {
        let below: Vec<f64> = (0..1999).map(|i| 0.5 + i as f64 * 1e-4).collect();
        assert!(two_sided_p(&below) < 0.002);
        let straddle: Vec<f64> = (0..2000).map(|i| 0.9 + i as f64 * 1e-4).collect();
        assert!(two_sided_p(&straddle) > 0.4);
        assert!(two_sided_p(&[1.0; 10]) <= 1.0);
    }

    #[test]
    fn a_paired_ratio_outside_its_own_interval_is_unknown() {
        let m = (Some(1.0), Some(0.7));
        // The interval says "better" but excludes the ratio it belongs to.
        assert_eq!(
            axis_verdict(Some(0.95), Some((0.6, 0.8)), true, m),
            "better"
        );
        assert_eq!(
            axis_verdict_v2(Some(0.95), Some((0.6, 0.8)), true, m),
            "unknown"
        );
        assert_eq!(
            axis_verdict_v2(Some(0.7), Some((0.6, 0.8)), true, m),
            "better"
        );
    }

    fn method(id: &str) -> MethodSpec {
        MethodSpec {
            id: id.into(),
            params: BTreeMap::new(),
        }
    }

    #[test]
    fn the_epoch_derives_from_a_commitment_and_a_later_beacon() {
        let c = challenge();
        let cand = method("rho.plain");
        let (cm, text) = commit(&c, &cand, "first", 1_700_000_000_000).unwrap();
        assert!(cm.commitment_id.starts_with(COMMITMENT_PREFIX));
        // The commitment seals and reads back.
        let path = std::env::temp_dir().join(format!("ecbench-commit-{}.json", std::process::id()));
        std::fs::write(&path, &text).unwrap();
        assert_eq!(Commitment::read(&path).unwrap(), cm);
        let _ = std::fs::remove_file(&path);
        let b1 = "0123456789abcdef0123";
        let e1 = epoch_from(&c, &cm, b1).unwrap();
        assert_eq!(e1, epoch_from(&c, &cm, b1).unwrap());
        // Another beacon, another commitment: another epoch; no free choice.
        assert_ne!(e1, epoch_from(&c, &cm, "fedcba9876543210fedc").unwrap());
        let (cm2, _) = commit(&c, &cand, "second", 1_700_000_000_001).unwrap();
        assert_ne!(e1, epoch_from(&c, &cm2, b1).unwrap());
        // A beacon is a public hex value, not a word.
        assert!(epoch_from(&c, &cm, "tomorrow").is_err());
        assert!(epoch_from(&c, &cm, "abc").is_err());
        // A commitment to another challenge is refused.
        let mut other = c.clone();
        other.label = "other".into();
        other.seal().unwrap();
        assert!(epoch_from(&other, &cm, b1).unwrap_err().contains("not"));
    }

    #[test]
    fn a_committed_spec_names_its_commitment_and_only_that_candidate_runs_it() {
        let c = challenge();
        let cand = method("rho.plain");
        let (cm, _) = commit(&c, &cand, "", 1).unwrap();
        let beacon = "0123456789abcdef0123";
        let s = spec_for_committed(&c, &cand, &cm, beacon).unwrap();
        assert!(s.description.contains(&cm.commitment_id) && s.description.contains(beacon));
        let epoch = epoch_from(&c, &cm, beacon).unwrap();
        // The seeds are the legacy law at the derived epoch.
        assert_eq!(
            s.workloads.target_seed,
            spec_for(&c, &cand, epoch).workloads.target_seed
        );
        // Another candidate cannot use the commitment.
        assert!(spec_for_committed(&c, &method("rho.negation"), &cm, beacon)
            .unwrap_err()
            .contains("commitment names"));
        // The version-1 spec is untouched by all of this.
        let a = plan(spec_for(&c, &cand, 1)).unwrap();
        let b = plan(spec_for(&c, &cand, 1)).unwrap();
        assert_eq!(a.spec_id, b.spec_id);
    }

    #[test]
    fn the_committed_verdicts_read_as_version_1_and_a_new_one_is_version_2() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let v = Verdict::read(&root.join(
            "research/ecbench_bounds_challenges_20261006/verdict-prime-toy-reference-e1.json",
        ))
        .unwrap();
        assert_eq!(v.procedure, 1);
        assert!(v.resamples.is_none() && v.commitment.is_none() && v.multiplicity.is_none());
        assert_eq!(v.outcome, "trade");
        // Version 1 wrote no purpose on the control; none is invented.
        assert!(v.control.as_ref().unwrap().purpose.is_none());
        assert_eq!(VERDICT_PROCEDURE, 2);
    }
}
