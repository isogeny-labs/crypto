//! `ecbench report`: a session directory as a Markdown report, SVG graphs,
//! a Typst source and a machine-readable JSON, all derived from the
//! session's own files.
//!
//! The report states observations only: what ran, on which host, at which
//! isolation level, what each arm cost in the one unit (group-addition
//! equivalents, and `S = gae / √r`), which runs failed and why, and the
//! exponent a scaling fit over the session's sizes returns.  It never
//! ranks arms and never says "wins" or "significant"; the comparisons are
//! `ecbench compare` and `ecbench band`.
//!
//! Layout written to `--out`:
//!
//! | file | what |
//! |---|---|
//! | `report.md` | the report |
//! | `report.json` | every table, machine-readable (`ecbench.report/v1`) |
//! | `report.typ` | a Typst document embedding the SVGs; the caller compiles it |
//! | `s_by_arm.svg`, `.csv` | mean `S` per arm, per curve, with interval whiskers, the floor and the expected `S` |
//! | `s_distribution.svg`, `.csv` | every verified run's `S` per arm: box and strip |
//! | `phases.svg`, `.csv` | stacked mean gae by phase of the index-calculus arms |
//! | `scaling.svg`, `.csv`, `scaling_fit.csv` | `log2 gae` against `log2 r`, with the OLS fit (several sizes only) |
//! | `raw_records.csv` | every record of the session: id, run id, arm, workload, status, level |
//!
//! Derived numbers are marked with the derivation that produced them (the
//! `[dN]` keys of the "Derivations" section).  Wall and CPU times follow
//! the rule `compare` applies: a figure is *admitted* only when every run
//! behind it earned the level the spec requires
//! (`measurement.isolation_required`, L2 when the spec is absent);
//! otherwise it is printed as informative at the lowest level earned.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::cryptanalysis::ecbench::bounds::ols;
use crate::cryptanalysis::ecbench::host::HostCapsule;
use crate::cryptanalysis::ecbench::methods::expected_s;
use crate::cryptanalysis::ecbench::record::Record;
use crate::cryptanalysis::ecbench::runner::{self, PlanDoc, Session};
use crate::cryptanalysis::ecbench::spec::{Level, Spec};
use crate::cryptanalysis::ecbench::stats;

pub const REPORT_SCHEMA: &str = "ecbench.report/v1";
/// Bootstrap resamples behind every interval of the report.
pub const RESAMPLES: usize = 4000;
/// The bootstrap seed (the one `ecbench table` uses).
pub const SEED: u64 = 20261002;

// ── Loading ────────────────────────────────────────────────────────

/// A session directory read into memory.  `spec.json`, `session.json` and
/// `host.json` are optional (a report of a partial directory names what is
/// missing); `plan.json` and `records.jsonl` are required.
pub struct Loaded {
    pub dir: PathBuf,
    pub spec: Option<Spec>,
    pub session: Option<Session>,
    pub host: Option<HostCapsule>,
    pub plan: PlanDoc,
    pub records: Vec<Record>,
    pub missing: Vec<String>,
}

fn read_text(p: &Path) -> Result<String, String> {
    std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))
}

pub fn load(dir: &Path) -> Result<Loaded, String> {
    let mut missing = Vec::new();
    let plan: PlanDoc = serde_json::from_str(&read_text(&dir.join("plan.json"))?)
        .map_err(|e| format!("plan.json: {e}"))?;
    let records = runner::read_records(dir)?;
    let spec = match read_text(&dir.join("spec.json")) {
        Ok(t) => Some(Spec::from_json(&t)?),
        Err(_) => {
            missing.push("spec.json".into());
            None
        }
    };
    let session = match runner::read_session(dir) {
        Ok(s) => Some(s),
        Err(_) => {
            missing.push("session.json".into());
            None
        }
    };
    let host = match read_text(&dir.join("host.json")) {
        Ok(t) => Some(serde_json::from_str(&t).map_err(|e| format!("host.json: {e}"))?),
        Err(_) => {
            missing.push("host.json".into());
            None
        }
    };
    Ok(Loaded {
        dir: dir.to_path_buf(),
        spec,
        session,
        host,
        plan,
        records,
        missing,
    })
}

// ── The report ─────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize)]
pub struct Summary {
    pub n: usize,
    pub mean: f64,
    pub median: f64,
    pub min: f64,
    pub max: f64,
    pub stdev: Option<f64>,
    /// 95% percentile bootstrap interval of the mean ([d2]).
    pub ci95: Option<(f64, f64)>,
}

fn summarize(xs: &[f64]) -> Option<Summary> {
    let mean = stats::mean(xs)?;
    let ci95 = if xs.len() >= 2 {
        stats::bootstrap_ci(&[xs.to_vec()], RESAMPLES, SEED, |s| {
            stats::mean(&s.concat())
        })
    } else {
        None
    };
    Some(Summary {
        n: xs.len(),
        mean,
        median: stats::median(xs)?,
        min: xs.iter().copied().fold(f64::INFINITY, f64::min),
        max: xs.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        stdev: stats::stdev(xs),
        ci95,
    })
}

#[derive(Clone, Debug, Serialize)]
pub struct HostSummary {
    pub env_class_id: String,
    pub cpu_model: Option<String>,
    pub cpu_vendor: Option<String>,
    pub logical_cpus: u32,
    pub physical_cores: Option<u32>,
    pub numa_nodes: u32,
    pub kernel: Option<String>,
    pub os: String,
    pub arch: String,
    pub mem_gib: Option<f64>,
    pub virtualization: BTreeMap<String, String>,
    pub debug_build: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct IsolationSummary {
    /// Measured (non-warm-up) records per level earned.
    pub levels: BTreeMap<String, usize>,
    /// The level the spec requires of a wall-clock figure.
    pub required: String,
    /// Blockers recorded, with the number of measured records each blocks.
    pub blockers: Vec<(String, usize)>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ArmRow {
    pub name: String,
    pub role: String,
    pub method_id: String,
    pub family: String,
    pub params: BTreeMap<String, String>,
    pub method_hash: String,
    /// `methods::expected_s(id, A)` for the first curve of the session.
    pub expected_s: Option<f64>,
    pub expected_s_curves: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct WorkloadRow {
    pub workload_id: String,
    pub slug: String,
    pub family: String,
    pub construction: String,
    pub field_size: Option<u32>,
    pub r: f64,
    pub log2_r: f64,
    pub cofactor: f64,
    pub automorphisms: u32,
    pub target_index: u64,
    pub target_seed: u64,
    pub target_law: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct FailureRow {
    pub seq: u64,
    pub run_id: String,
    pub record_id: String,
    pub arm: String,
    pub workload_id: String,
    pub warmup: bool,
    pub status: String,
    pub error: Option<String>,
    pub exit: Option<String>,
    pub stderr: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RunCounts {
    /// Executions the plan lists, warm-ups included.
    pub planned: usize,
    pub planned_warmup: usize,
    pub planned_measured: usize,
    pub records: usize,
    pub measured_records: usize,
    pub verified: usize,
    /// Verified records whose `matches_target` is true.
    pub matches_target: usize,
    pub failed: usize,
    /// Planned executions with no record (an interrupted session).
    pub missing: usize,
    pub warmup_failed: usize,
    pub status_counts: BTreeMap<String, usize>,
}

/// One arm on one workload or one curve (pooled over its workloads).
#[derive(Clone, Debug, Serialize)]
pub struct Cell {
    pub arm: String,
    pub method_id: String,
    pub scope: String,
    pub slug: String,
    pub workload_id: Option<String>,
    pub field_size: Option<u32>,
    pub log2_r: f64,
    pub planned: usize,
    pub records: usize,
    pub verified: usize,
    pub matches_target: usize,
    pub s: Option<Summary>,
    pub mean_gae: Option<f64>,
    pub ratio_to_floor: Option<f64>,
    pub floor_s: f64,
    pub expected_s: Option<f64>,
    pub distinguished_points: Option<f64>,
    pub steps: Option<f64>,
    pub field_muls: Option<f64>,
    pub field_sqrs: Option<f64>,
    pub field_invs: Option<f64>,
    pub wall_ms_median: Option<f64>,
    pub cpu_ms_median: Option<f64>,
    /// `admitted (≥Lk)` or `informative, L0` (the lowest level earned).
    pub time_status: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PhaseRow {
    pub arm: String,
    pub slug: String,
    pub phase: String,
    pub mean_gae: f64,
    /// `mean_gae` over the arm's mean total on this curve.
    pub share: f64,
    pub runs: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct SizePoint {
    pub slug: String,
    pub field_size: Option<u32>,
    pub log2_r: f64,
    pub n: usize,
    pub mean_gae: f64,
    pub log2_mean_gae: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct ScalingFit {
    pub arm: String,
    pub method_id: String,
    pub sizes: usize,
    pub runs: usize,
    pub points: Vec<SizePoint>,
    /// The slope `α` of `log2 gae` against `log2 r` ([d5]).
    pub alpha: f64,
    /// Standard error of the slope, from the residuals with `sizes − 2`
    /// degrees of freedom; `None` below three sizes.
    pub se: Option<f64>,
    /// `log2 C` with `gae = C · r^α`.
    pub log2_c: f64,
    pub c: f64,
    pub r2: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct RawRecord {
    pub record_id: String,
    pub run_id: String,
    pub seq: u64,
    pub round: u32,
    pub warmup: bool,
    pub arm: String,
    pub workload_id: String,
    pub status: String,
    pub level: String,
    pub s: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Report {
    pub schema: String,
    pub session_id: String,
    pub label: String,
    pub description: String,
    pub spec_id: Option<String>,
    pub commit: Option<String>,
    pub dirty: Option<bool>,
    pub binary_sha256: Option<String>,
    pub session_status: Option<String>,
    pub run_options: Option<String>,
    pub host: Option<HostSummary>,
    pub isolation: IsolationSummary,
    pub arms: Vec<ArmRow>,
    pub workloads: Vec<WorkloadRow>,
    pub runs: RunCounts,
    pub failures: Vec<FailureRow>,
    pub cells: Vec<Cell>,
    pub curve_cells: Vec<Cell>,
    pub phases: Vec<PhaseRow>,
    pub scaling: Vec<ScalingFit>,
    pub raw_dir: String,
    pub raw: Vec<RawRecord>,
    pub exec_files: usize,
    pub missing_files: Vec<String>,
}

fn level_name(l: Level) -> &'static str {
    l.name()
}

fn counter(r: &Record, k: &str) -> Option<f64> {
    r.counters.get(k).map(|v| *v as f64)
}

fn steps_of(r: &Record) -> Option<f64> {
    counter(r, "steps").or_else(
        || match (counter(r, "baby_steps"), counter(r, "giant_steps")) {
            (Some(a), Some(b)) => Some(a + b),
            _ => None,
        },
    )
}

fn mean_opt(xs: Vec<f64>) -> Option<f64> {
    stats::mean(&xs)
}

/// Summarise the records of one arm over `recs` (all measured records of a
/// workload or a curve).
fn cell(
    arm: &str,
    method_id: &str,
    scope: &str,
    recs: &[&Record],
    planned: usize,
    required: Level,
) -> Option<Cell> {
    let first = recs.first()?;
    let ok: Vec<&&Record> = recs.iter().filter(|r| r.counts()).collect();
    let s_vals: Vec<f64> = ok.iter().filter_map(|r| r.cost.s).collect();
    let gae: Vec<f64> = ok.iter().filter_map(|r| r.cost.total_gae).collect();
    let ratios: Vec<f64> = ok
        .iter()
        .filter_map(|r| r.boundaries.ratio_to_floor)
        .collect();
    let fo = |f: fn(&crate::cryptanalysis::ic_boundary::FieldOps) -> u64| {
        mean_opt(
            ok.iter()
                .filter_map(|r| r.field_ops.as_ref().map(|o| f(o) as f64))
                .collect(),
        )
    };
    let walls: Vec<f64> = ok
        .iter()
        .filter_map(|r| r.time.solve_wall_ns.map(|n| n as f64 / 1e6))
        .collect();
    let cpus: Vec<f64> = ok
        .iter()
        .map(|r| (r.time.user_ns + r.time.sys_ns) as f64 / 1e6)
        .collect();
    let lowest = ok
        .iter()
        .map(|r| r.isolation.level)
        .min()
        .unwrap_or(Level::L0);
    let time_status = if ok.is_empty() {
        "none".to_string()
    } else if lowest >= required {
        format!("admitted (≥{})", level_name(required))
    } else {
        format!("informative, {}", level_name(lowest))
    };
    let curve = &first.workload.curve;
    Some(Cell {
        arm: arm.into(),
        method_id: method_id.into(),
        scope: scope.into(),
        slug: curve.slug.clone(),
        workload_id: (scope == "workload").then(|| first.workload.workload_id.clone()),
        field_size: if curve.family == "prime" {
            curve.field_bits
        } else {
            curve.field_degree
        },
        log2_r: (curve.r as f64).log2(),
        planned,
        records: recs.len(),
        verified: ok.len(),
        matches_target: ok
            .iter()
            .filter(|r| r.outcome.matches_target == Some(true))
            .count(),
        s: summarize(&s_vals),
        mean_gae: stats::mean(&gae),
        ratio_to_floor: stats::mean(&ratios),
        floor_s: first.boundaries.floor_s,
        expected_s: expected_s(method_id, first.boundaries.automorphisms_available),
        distinguished_points: mean_opt(
            ok.iter()
                .filter_map(|r| counter(r, "distinguished_points"))
                .collect(),
        ),
        steps: mean_opt(ok.iter().filter_map(|r| steps_of(r)).collect()),
        field_muls: fo(|o| o.muls),
        field_sqrs: fo(|o| o.sqrs),
        field_invs: fo(|o| o.invs),
        wall_ms_median: stats::median(&walls),
        cpu_ms_median: stats::median(&cpus),
        time_status,
    })
}

/// OLS standard error of the slope: `√(SS_res / (n − 2) / S_xx)`.
fn slope_se(points: &[(f64, f64)], alpha: f64, b: f64) -> Option<f64> {
    let n = points.len();
    if n < 3 {
        return None;
    }
    let xbar = points.iter().map(|p| p.0).sum::<f64>() / n as f64;
    let sxx: f64 = points.iter().map(|p| (p.0 - xbar).powi(2)).sum();
    if sxx <= 0.0 {
        return None;
    }
    let ss: f64 = points
        .iter()
        .map(|p| (p.1 - (alpha * p.0 + b)).powi(2))
        .sum();
    Some((ss / (n as f64 - 2.0) / sxx).sqrt())
}

/// Fit `log2 gae = α · log2 r + log2 C` over per-size mean gae, using
/// [`ols`] for the line and the residual formula for its standard error.
pub fn fit_scaling(arm: &str, method_id: &str, cells: &[&Cell]) -> Option<ScalingFit> {
    let mut points: Vec<SizePoint> = cells
        .iter()
        .filter_map(|c| {
            let g = c.mean_gae?;
            (g > 0.0).then(|| SizePoint {
                slug: c.slug.clone(),
                field_size: c.field_size,
                log2_r: c.log2_r,
                n: c.verified,
                mean_gae: g,
                log2_mean_gae: g.log2(),
            })
        })
        .collect();
    points.sort_by(|a, b| a.log2_r.total_cmp(&b.log2_r));
    let xy: Vec<(f64, f64)> = points.iter().map(|p| (p.log2_r, p.log2_mean_gae)).collect();
    let (alpha, b, r2) = ols(&xy)?;
    Some(ScalingFit {
        arm: arm.into(),
        method_id: method_id.into(),
        sizes: points.len(),
        runs: points.iter().map(|p| p.n).sum(),
        se: slope_se(&xy, alpha, b),
        alpha,
        log2_c: b,
        c: b.exp2(),
        r2,
        points,
    })
}

pub fn build(l: &Loaded) -> Report {
    let recs = &l.records;
    let required = l
        .spec
        .as_ref()
        .map(|s| s.measurement.isolation_required)
        .unwrap_or(Level::L2);
    let session_id = l
        .session
        .as_ref()
        .map(|s| s.session_id.clone())
        .or_else(|| recs.first().map(|r| r.session_id.clone()))
        .unwrap_or_else(|| "(no session id)".into());
    let measured: Vec<&Record> = recs.iter().filter(|r| !r.warmup).collect();

    // Arms and workloads, in plan order.
    let arms: Vec<ArmRow> = l
        .plan
        .arms
        .iter()
        .map(|a| {
            let aut: Vec<u32> = {
                let mut v: Vec<u32> = l
                    .plan
                    .workloads
                    .iter()
                    .map(|w| w.curve.automorphisms_available)
                    .collect();
                v.dedup();
                v
            };
            let exp = aut.first().and_then(|&a_av| expected_s(&a.method.id, a_av));
            let note = if aut.len() > 1 && exp.is_some() {
                "depends on A; value for the first curve (see the per-curve tables)".to_string()
            } else if exp.is_some() {
                "methods::expected_s, setup omitted".to_string()
            } else {
                "none registered".to_string()
            };
            ArmRow {
                name: a.name.clone(),
                role: format!("{:?}", a.role).to_lowercase(),
                method_id: a.method.id.clone(),
                family: a.method.family.clone(),
                params: a.method.params.clone(),
                method_hash: a.method.method_id.clone(),
                expected_s: exp,
                expected_s_curves: note,
            }
        })
        .collect();
    let workloads: Vec<WorkloadRow> = l
        .plan
        .workloads
        .iter()
        .map(|w| WorkloadRow {
            workload_id: w.workload_id.clone(),
            slug: w.curve.slug.clone(),
            family: w.curve.family.clone(),
            construction: w.curve.construction.clone(),
            field_size: if w.curve.family == "prime" {
                w.curve.field_bits
            } else {
                w.curve.field_degree
            },
            r: w.curve.r as f64,
            log2_r: (w.curve.r as f64).log2(),
            cofactor: w.curve.cofactor as f64,
            automorphisms: w.curve.automorphisms_available,
            target_index: w.target_index,
            target_seed: w.target_seed,
            target_law: w.target_law.clone(),
        })
        .collect();

    // Runs.
    let planned_warmup = l.plan.executions.iter().filter(|e| e.warmup).count();
    let planned = l.plan.executions.len();
    let mut status_counts: BTreeMap<String, usize> = BTreeMap::new();
    for r in recs {
        *status_counts.entry(r.outcome.status.clone()).or_default() += 1;
    }
    let failures: Vec<FailureRow> = recs
        .iter()
        .filter(|r| r.outcome.status != "verified")
        .map(|r| FailureRow {
            seq: r.seq,
            run_id: r.run_id.clone(),
            record_id: r.record_id.clone(),
            arm: r.arm.clone(),
            workload_id: r.workload.workload_id.clone(),
            warmup: r.warmup,
            status: r.outcome.status.clone(),
            error: r.outcome.error.clone(),
            exit: r.outcome.exit.clone(),
            stderr: r.outcome.stderr_sha256.as_ref().map(|_| {
                l.dir
                    .join("exec")
                    .join(format!("{:06}.stderr", r.seq))
                    .display()
                    .to_string()
            }),
        })
        .collect();
    let runs = RunCounts {
        planned,
        planned_warmup,
        planned_measured: planned - planned_warmup,
        records: recs.len(),
        measured_records: measured.len(),
        verified: measured.iter().filter(|r| r.counts()).count(),
        matches_target: measured
            .iter()
            .filter(|r| r.counts() && r.outcome.matches_target == Some(true))
            .count(),
        failed: measured
            .iter()
            .filter(|r| r.outcome.status != "verified")
            .count(),
        missing: planned.saturating_sub(recs.len()),
        warmup_failed: recs
            .iter()
            .filter(|r| r.warmup && r.outcome.status != "verified")
            .count(),
        status_counts,
    };

    // Isolation.
    let mut levels: BTreeMap<String, usize> = ["L0", "L1", "L2", "L3"]
        .iter()
        .map(|k| (k.to_string(), 0))
        .collect();
    let mut blockers: BTreeMap<String, usize> = BTreeMap::new();
    for r in &measured {
        *levels
            .entry(level_name(r.isolation.level).into())
            .or_default() += 1;
        for b in &r.isolation.blockers {
            *blockers.entry(b.clone()).or_default() += 1;
        }
    }
    let mut blockers: Vec<(String, usize)> = blockers.into_iter().collect();
    blockers.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let isolation = IsolationSummary {
        levels,
        required: level_name(required).into(),
        blockers,
    };

    // Per-workload cells (plan order) and per-curve cells.
    let mut cells = Vec::new();
    let mut curve_cells = Vec::new();
    let mut curves: Vec<String> = Vec::new();
    for w in &l.plan.workloads {
        if !curves.contains(&w.curve.slug) {
            curves.push(w.curve.slug.clone());
        }
    }
    for a in &l.plan.arms {
        for w in &l.plan.workloads {
            let mine: Vec<&Record> = measured
                .iter()
                .copied()
                .filter(|r| r.arm == a.name && r.workload.workload_id == w.workload_id)
                .collect();
            let plan_n = l
                .plan
                .executions
                .iter()
                .filter(|e| {
                    !e.warmup
                        && l.plan.arms[e.arm].name == a.name
                        && l.plan.workloads[e.workload].workload_id == w.workload_id
                })
                .count();
            if let Some(c) = cell(&a.name, &a.method.id, "workload", &mine, plan_n, required) {
                cells.push(c);
            }
        }
        for slug in &curves {
            let mine: Vec<&Record> = measured
                .iter()
                .copied()
                .filter(|r| r.arm == a.name && &r.workload.curve.slug == slug)
                .collect();
            let plan_n = l
                .plan
                .executions
                .iter()
                .filter(|e| {
                    !e.warmup
                        && l.plan.arms[e.arm].name == a.name
                        && &l.plan.workloads[e.workload].curve.slug == slug
                })
                .count();
            if let Some(c) = cell(&a.name, &a.method.id, "curve", &mine, plan_n, required) {
                curve_cells.push(c);
            }
        }
    }

    // Phases of the index-calculus arms, pooled per curve.
    let mut phases = Vec::new();
    for a in l.plan.arms.iter().filter(|a| a.method.family == "ic") {
        for slug in &curves {
            let ok: Vec<&Record> = measured
                .iter()
                .copied()
                .filter(|r| r.counts() && r.arm == a.name && &r.workload.curve.slug == slug)
                .filter(|r| !r.phases.is_empty())
                .collect();
            if ok.is_empty() {
                continue;
            }
            let mut names: Vec<String> = Vec::new();
            for r in &ok {
                for p in &r.phases {
                    if !names.contains(&p.name) {
                        names.push(p.name.clone());
                    }
                }
            }
            let per: Vec<(String, f64)> = names
                .iter()
                .map(|n| {
                    let total: f64 = ok
                        .iter()
                        .map(|r| {
                            r.phases
                                .iter()
                                .filter(|p| &p.name == n)
                                .map(|p| p.gae)
                                .sum::<f64>()
                        })
                        .sum();
                    (n.clone(), total / ok.len() as f64)
                })
                .collect();
            let total: f64 = per.iter().map(|p| p.1).sum();
            for (n, g) in per {
                phases.push(PhaseRow {
                    arm: a.name.clone(),
                    slug: slug.clone(),
                    phase: n,
                    mean_gae: g,
                    share: if total > 0.0 { g / total } else { 0.0 },
                    runs: ok.len(),
                });
            }
        }
    }

    // Scaling: one fit per arm over its per-curve mean gae.
    let mut scaling = Vec::new();
    let sizes: std::collections::BTreeSet<u64> = l
        .plan
        .workloads
        .iter()
        .map(|w| (w.curve.r as f64).to_bits())
        .collect();
    if sizes.len() >= 2 {
        for a in &l.plan.arms {
            let mine: Vec<&Cell> = curve_cells.iter().filter(|c| c.arm == a.name).collect();
            if let Some(f) = fit_scaling(&a.name, &a.method.id, &mine) {
                if f.sizes >= 2 {
                    scaling.push(f);
                }
            }
        }
    }

    let raw: Vec<RawRecord> = recs
        .iter()
        .map(|r| RawRecord {
            record_id: r.record_id.clone(),
            run_id: r.run_id.clone(),
            seq: r.seq,
            round: r.round,
            warmup: r.warmup,
            arm: r.arm.clone(),
            workload_id: r.workload.workload_id.clone(),
            status: r.outcome.status.clone(),
            level: level_name(r.isolation.level).into(),
            s: r.cost.s,
        })
        .collect();
    let exec_files = std::fs::read_dir(l.dir.join("exec"))
        .map(|d| d.count())
        .unwrap_or(0);

    Report {
        schema: REPORT_SCHEMA.into(),
        label: l
            .spec
            .as_ref()
            .map(|s| s.label.clone())
            .or_else(|| l.session.as_ref().map(|s| s.label.clone()))
            .unwrap_or_default(),
        description: l
            .spec
            .as_ref()
            .map(|s| s.description.clone())
            .unwrap_or_default(),
        spec_id: l
            .session
            .as_ref()
            .map(|s| s.spec_id.clone())
            .or_else(|| Some(l.plan.spec_id.clone())),
        commit: l.session.as_ref().and_then(|s| s.git_commit.clone()),
        dirty: l.session.as_ref().and_then(|s| s.git_dirty),
        binary_sha256: l.session.as_ref().and_then(|s| s.binary_sha256.clone()),
        session_status: l.session.as_ref().map(|s| s.status.clone()),
        run_options: l.session.as_ref().map(|s| {
            format!(
                "cpus={}, lock={}, wait_for_lock={}, allow_busy={}",
                serde_json::to_string(&s.options.cpus).unwrap_or_default(),
                s.options.lock_path,
                s.options.wait_for_lock,
                s.options.allow_busy
            )
        }),
        host: l.host.as_ref().map(|h| HostSummary {
            env_class_id: h.env_class_id.clone(),
            cpu_model: h.stable.cpu_model.clone(),
            cpu_vendor: h.stable.cpu_vendor.clone(),
            logical_cpus: h.stable.logical_cpus,
            physical_cores: h.stable.physical_cores,
            numa_nodes: h.stable.numa_nodes,
            kernel: h.stable.kernel_release.clone(),
            os: h.stable.os.clone(),
            arch: h.stable.arch.clone(),
            mem_gib: h.stable.mem_total_kib.map(|k| k as f64 / 1048576.0),
            virtualization: h.stable.virtualization.clone(),
            debug_build: h.build.debug_assertions,
        }),
        session_id,
        isolation,
        arms,
        workloads,
        runs,
        failures,
        cells,
        curve_cells,
        phases,
        scaling,
        raw_dir: l.dir.display().to_string(),
        raw,
        exec_files,
        missing_files: l.missing.clone(),
    }
}

// ── Number formatting ──────────────────────────────────────────────

fn f4(x: f64) -> String {
    if x.is_finite() {
        format!("{x:.4}")
    } else {
        "–".into()
    }
}
fn f3(x: f64) -> String {
    if x.is_finite() {
        format!("{x:.3}")
    } else {
        "–".into()
    }
}
fn o4(x: Option<f64>) -> String {
    x.map(f4).unwrap_or_else(|| "–".into())
}
fn o1(x: Option<f64>) -> String {
    x.filter(|v| v.is_finite())
        .map(|v| format!("{v:.1}"))
        .unwrap_or_else(|| "–".into())
}
fn ci(x: &Option<Summary>) -> String {
    match x.as_ref().and_then(|s| s.ci95) {
        Some((a, b)) => format!("[{}, {}]", f4(a), f4(b)),
        None => "–".into(),
    }
}

// ── Markdown ───────────────────────────────────────────────────────

fn md_esc(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

fn md_table(out: &mut String, head: &[&str], rows: &[Vec<String>]) {
    let _ = writeln!(out, "| {} |", head.join(" | "));
    let _ = writeln!(
        out,
        "|{}|",
        head.iter().map(|_| "---").collect::<Vec<_>>().join("|")
    );
    for r in rows {
        let _ = writeln!(
            out,
            "| {} |",
            r.iter().map(|c| md_esc(c)).collect::<Vec<_>>().join(" | ")
        );
    }
    out.push('\n');
}

/// The derivation keys every `[dN]` in the report refers to.
pub const DERIVATIONS: &[(&str, &str)] = &[
    ("d1", "S = cost.s of each verified, non-warm-up record (`total_gae / √r`, stored by the runner); mean, median, min, max and sample stdev (stats.rs) are over those records."),
    ("d2", "95% percentile bootstrap interval of the mean S: 4000 resamples of the verified runs of the cell with replacement (stats::bootstrap_ci, seed 20261002). A cell with one run has no interval. Not a significance test."),
    ("d3", "total gae and S / floor are means over the verified runs of the cell of cost.total_gae and boundaries.ratio_to_floor; floor S is boundaries.floor_s = √(π / 2A), A = automorphisms the curve offers a generic method."),
    ("d4", "expected S is methods::expected_s(method, A): the closed form the method's mean S approaches as r grows (set-up omitted). A reference mark, not a measurement."),
    ("d5", "scaling: per arm, per curve, the mean gae over the arm's verified runs on that curve; the line is ordinary least squares of log2(mean gae) on log2(r) over the curves (bounds::ols), slope α, intercept log2 C so gae = C·r^α. The standard error of α is √(SS_res/(sizes−2)/S_xx) from those residuals (none below 3 sizes). Counted operations, one point per size: it is not the cluster-bootstrap interval of `ecbench bound fit`, which resamples runs and sizes."),
    ("d6", "counters (distinguished points, steps) and field operations (muls, sqrs, invs) are means over the cell's verified runs of the counters the record carries (steps = `steps`, or baby + giant steps for BSGS); a dash means the method did not count it (unknown, not zero)."),
    ("d7", "wall and CPU are medians over the cell's verified runs of time.solve_wall_ns and time.user_ns + sys_ns. The figure is admitted only when every run behind it earned the level the spec requires (compare.rs's gate); otherwise it is informative at the lowest level earned, and never a result."),
    ("d8", "phase share = the phase's mean gae over the sum of the arm's mean gae across phases on that curve, over verified runs."),
];

pub fn markdown(r: &Report) -> String {
    let mut o = String::new();
    let _ = writeln!(o, "# ecbench report: {}\n", r.session_id);
    let _ = writeln!(o, "**Label:** {}\n", r.label);
    if !r.description.is_empty() {
        let _ = writeln!(o, "**Description:** {}\n", r.description);
    }
    let _ = writeln!(
        o,
        "Generated by `ecbench report` (`{}`) from `{}`. Observations only; every derived number carries the key of its derivation (see Derivations).\n",
        r.schema, r.raw_dir
    );
    if !r.missing_files.is_empty() {
        let _ = writeln!(
            o,
            "**Files absent from the session directory:** {}.\n",
            r.missing_files.join(", ")
        );
    }

    let _ = writeln!(o, "## Session\n");
    let mut rows = vec![
        vec!["session id".into(), r.session_id.clone()],
        vec![
            "spec id".into(),
            r.spec_id.clone().unwrap_or_else(|| "not recorded".into()),
        ],
        vec![
            "commit".into(),
            match (&r.commit, r.dirty) {
                (Some(c), Some(true)) => format!("{c} (dirty tree)"),
                (Some(c), _) => c.clone(),
                _ => "not recorded".into(),
            },
        ],
        vec![
            "binary sha256".into(),
            r.binary_sha256
                .clone()
                .unwrap_or_else(|| "not recorded".into()),
        ],
        vec![
            "status".into(),
            r.session_status
                .clone()
                .unwrap_or_else(|| "not recorded".into()),
        ],
        vec![
            "run options".into(),
            r.run_options
                .clone()
                .unwrap_or_else(|| "not recorded".into()),
        ],
    ];
    rows.push(vec![
        "command".into(),
        "`ecbench run --spec <session>/spec.json --out <session>` with the options above (the original command line is not recorded)".into(),
    ]);
    md_table(&mut o, &["field", "value"], &rows);

    let _ = writeln!(o, "## Host and isolation\n");
    if let Some(h) = &r.host {
        let virt = if h.virtualization.is_empty() {
            "none recorded".to_string()
        } else {
            h.virtualization
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let rows = vec![
            vec![
                "CPU".into(),
                format!(
                    "{} ({})",
                    h.cpu_model.clone().unwrap_or_else(|| "unknown".into()),
                    h.cpu_vendor.clone().unwrap_or_else(|| "unknown".into())
                ),
            ],
            vec![
                "cores".into(),
                format!(
                    "{} logical, {} physical, {} NUMA node(s)",
                    h.logical_cpus,
                    h.physical_cores
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "?".into()),
                    h.numa_nodes
                ),
            ],
            vec![
                "kernel / OS".into(),
                format!(
                    "{} / {} {}",
                    h.kernel.clone().unwrap_or_else(|| "unknown".into()),
                    h.os,
                    h.arch
                ),
            ],
            vec![
                "memory".into(),
                h.mem_gib
                    .map(|g| format!("{g:.1} GiB"))
                    .unwrap_or_else(|| "unknown".into()),
            ],
            vec!["virtualization".into(), virt],
            vec!["env class".into(), h.env_class_id.clone()],
            vec!["debug build".into(), h.debug_build.to_string()],
        ];
        md_table(&mut o, &["host", "value"], &rows);
    } else {
        let _ = writeln!(o, "host.json is absent.\n");
    }
    let lv = &r.isolation.levels;
    let _ = writeln!(
        o,
        "Isolation level earned by the {} measured (non-warm-up) records: L0 {}, L1 {}, L2 {}, L3 {}. The spec requires {} of a wall-clock figure; wall and CPU columns below are admitted only where every run reached it.\n",
        r.runs.measured_records, lv["L0"], lv["L1"], lv["L2"], lv["L3"], r.isolation.required
    );
    if r.isolation.blockers.is_empty() {
        let _ = writeln!(o, "No isolation blockers recorded.\n");
    } else {
        let rows: Vec<Vec<String>> = r
            .isolation
            .blockers
            .iter()
            .take(12)
            .map(|(b, n)| vec![b.clone(), n.to_string()])
            .collect();
        let _ = writeln!(
            o,
            "Blockers recorded (the level each blocks is the one in its prefix); top {} of {}:\n",
            rows.len(),
            r.isolation.blockers.len()
        );
        md_table(&mut o, &["blocker", "measured records"], &rows);
    }

    let _ = writeln!(o, "## Arms\n");
    let rows: Vec<Vec<String>> = r
        .arms
        .iter()
        .map(|a| {
            vec![
                a.name.clone(),
                a.role.clone(),
                a.method_id.clone(),
                if a.params.is_empty() {
                    "defaults".into()
                } else {
                    a.params
                        .iter()
                        .map(|(k, v)| format!("{k}={v}"))
                        .collect::<Vec<_>>()
                        .join("; ")
                },
                a.method_hash.clone(),
                a.expected_s.map(f4).unwrap_or_else(|| "–".into()),
            ]
        })
        .collect();
    md_table(
        &mut o,
        &[
            "arm",
            "role",
            "method",
            "params",
            "method id",
            "expected S [d4]",
        ],
        &rows,
    );

    let _ = writeln!(o, "## Workloads\n");
    let rows: Vec<Vec<String>> = r
        .workloads
        .iter()
        .map(|w| {
            vec![
                w.workload_id.clone(),
                w.slug.clone(),
                w.family.clone(),
                w.field_size
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "–".into()),
                format!("{:.0}", w.r),
                f3(w.log2_r),
                format!("{:.0}", w.cofactor),
                w.automorphisms.to_string(),
                w.target_index.to_string(),
                w.target_seed.to_string(),
            ]
        })
        .collect();
    md_table(
        &mut o,
        &[
            "workload",
            "curve",
            "family",
            "field bits / degree",
            "r",
            "log2 r",
            "cofactor",
            "A",
            "target index",
            "target seed",
        ],
        &rows,
    );

    let c = &r.runs;
    let _ = writeln!(o, "## Runs\n");
    let _ = writeln!(
        o,
        "Executions planned: {} ({} warm-up, {} measured). Records written: {} ({} measured). Measured and verified: {} ({} with `matches_target` true). Measured and not verified: {}. Planned executions without a record: {}. Warm-up records not verified: {}. Status counts over all records: {}.\n",
        c.planned,
        c.planned_warmup,
        c.planned_measured,
        c.records,
        c.measured_records,
        c.verified,
        c.matches_target,
        c.failed,
        c.missing,
        c.warmup_failed,
        c.status_counts
            .iter()
            .map(|(k, v)| format!("{k} {v}"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    if r.failures.is_empty() {
        let _ = writeln!(o, "No record has a status other than `verified`.\n");
    } else {
        let _ = writeln!(o, "Records whose status is not `verified`:\n");
        let rows: Vec<Vec<String>> = r
            .failures
            .iter()
            .map(|f| {
                vec![
                    f.seq.to_string(),
                    f.run_id.clone(),
                    f.arm.clone(),
                    f.workload_id.clone(),
                    if f.warmup { "warm-up" } else { "measured" }.into(),
                    f.status.clone(),
                    f.error.clone().unwrap_or_else(|| "–".into()),
                    f.exit.clone().unwrap_or_else(|| "–".into()),
                    f.stderr.clone().unwrap_or_else(|| "–".into()),
                ]
            })
            .collect();
        md_table(
            &mut o,
            &[
                "seq",
                "run id",
                "arm",
                "workload",
                "kind",
                "status",
                "recorded reason",
                "exit",
                "stderr file",
            ],
            &rows,
        );
    }

    // Results per curve, then per workload.
    let _ = writeln!(
        o,
        "## Results per arm and curve (the one unit: S = gae / √r)\n\nEach row pools the arm's verified runs on all the curve's workloads. Warm-ups and non-verified records are excluded. S columns [d1], interval [d2], cost and floor [d3], expected S [d4].\n"
    );
    let rows = s_rows(&r.curve_cells, false);
    md_table(&mut o, S_HEAD, &rows);
    let _ = writeln!(
        o,
        "### Cost, counters, field operations and time per arm and curve\n\nTotal gae and ratios [d3], counters and field operations [d6], time [d7].\n"
    );
    let rows = cost_rows(&r.curve_cells, false);
    md_table(&mut o, COST_HEAD, &rows);

    let _ = writeln!(
        o,
        "## Results per arm and workload\n\nOne row per (arm, workload): the independent single-target workloads, never pooled.\n"
    );
    let rows = s_rows(&r.cells, true);
    md_table(&mut o, S_HEAD_W, &rows);
    let rows = cost_rows(&r.cells, true);
    md_table(&mut o, COST_HEAD_W, &rows);

    if !r.phases.is_empty() {
        let _ = writeln!(
            o,
            "## Phases of the index-calculus arms\n\nMean gae per phase over the arm's verified runs on the curve; share of the arm's total [d8].\n"
        );
        let rows: Vec<Vec<String>> = r
            .phases
            .iter()
            .map(|p| {
                vec![
                    p.arm.clone(),
                    p.slug.clone(),
                    p.phase.clone(),
                    p.runs.to_string(),
                    o1(Some(p.mean_gae)),
                    format!("{:.1}%", 100.0 * p.share),
                ]
            })
            .collect();
        md_table(
            &mut o,
            &["arm", "curve", "phase", "runs", "mean gae", "share [d8]"],
            &rows,
        );
    }

    if !r.scaling.is_empty() {
        let _ = writeln!(
            o,
            "## Scaling over the session's sizes\n\nFit of log2(mean gae) on log2(r) per arm [d5]. These are counted group operations; the exponent a generic method approaches is 1/2, so α is read against 0.5 as a distance, nothing more.\n"
        );
        let rows: Vec<Vec<String>> = r
            .scaling
            .iter()
            .map(|f| {
                vec![
                    f.arm.clone(),
                    f.method_id.clone(),
                    f.sizes.to_string(),
                    f.runs.to_string(),
                    f3(f.alpha),
                    f.se.map(f3).unwrap_or_else(|| "–".into()),
                    f3(f.log2_c),
                    f4(f.c),
                    f4(f.r2),
                ]
            })
            .collect();
        md_table(
            &mut o,
            &[
                "arm",
                "method",
                "sizes",
                "verified runs",
                "α [d5]",
                "se(α)",
                "log2 C",
                "C",
                "R²",
            ],
            &rows,
        );
        let _ = writeln!(
            o,
            "Points behind each fit (size, log2 r, verified runs, mean gae):\n"
        );
        let mut rows = Vec::new();
        for f in &r.scaling {
            for p in &f.points {
                rows.push(vec![
                    f.arm.clone(),
                    p.slug.clone(),
                    p.field_size
                        .map(|b| b.to_string())
                        .unwrap_or_else(|| "–".into()),
                    f3(p.log2_r),
                    p.n.to_string(),
                    o1(Some(p.mean_gae)),
                ]);
            }
        }
        md_table(
            &mut o,
            &["arm", "curve", "size", "log2 r", "runs", "mean gae"],
            &rows,
        );
    }

    let _ = writeln!(
        o,
        "## Graphs\n\n\
         - `s_by_arm.svg`: mean S per arm per curve, 95% bootstrap whiskers [d2], the floor (dashed) and expected S marks [d4].\n\
         - `s_distribution.svg`: every verified run's S per arm and curve (box: quartiles and median; whiskers: min and max).\n"
    );
    if !r.phases.is_empty() {
        let _ = writeln!(
            o,
            "- `phases.svg`: stacked mean gae by phase, index-calculus arms [d8].\n"
        );
    }
    if !r.scaling.is_empty() {
        let _ = writeln!(
            o,
            "- `scaling.svg`: log2(mean gae) against log2(r) with the OLS line and α ± se in the legend [d5].\n"
        );
    }
    let _ = writeln!(
        o,
        "Each graph's plotted data is the `.csv` beside it. Caption of every graph: session {}, n = {} verified runs, unit gae (S = gae / √r).\n",
        r.session_id, r.runs.verified
    );

    let _ = writeln!(o, "## Raw records\n");
    let _ = writeln!(
        o,
        "- records: `{}/records.jsonl` ({} lines)\n- plan, spec, session, host: `{}/plan.json`, `spec.json`, `session.json`, `host.json`\n- child stderr files: `{}/exec/` ({} file(s))\n- every record id in `raw_records.csv` and `report.json`\n",
        r.raw_dir, r.records_len(), r.raw_dir, r.raw_dir, r.exec_files
    );
    let rows: Vec<Vec<String>> = r
        .raw
        .iter()
        .take(60)
        .map(|x| {
            vec![
                x.seq.to_string(),
                x.record_id.clone(),
                x.run_id.clone(),
                x.arm.clone(),
                x.status.clone(),
                x.level.clone(),
                if x.warmup { "warm-up" } else { "measured" }.into(),
            ]
        })
        .collect();
    let _ = writeln!(o, "First {} of {} records:\n", rows.len(), r.raw.len());
    md_table(
        &mut o,
        &[
            "seq",
            "record id",
            "run id",
            "arm",
            "status",
            "level",
            "kind",
        ],
        &rows,
    );

    let _ = writeln!(o, "## Derivations\n");
    for (k, v) in DERIVATIONS {
        let _ = writeln!(o, "- **[{k}]** {v}");
    }
    o.push('\n');
    o
}

impl Report {
    fn records_len(&self) -> usize {
        self.raw.len()
    }
}

const S_HEAD: &[&str] = &[
    "arm",
    "curve",
    "log2 r",
    "verified / planned",
    "mean S [d1]",
    "median S",
    "min S",
    "max S",
    "stdev S",
    "95% CI of mean [d2]",
    "floor S [d3]",
    "expected S [d4]",
];
const S_HEAD_W: &[&str] = &[
    "arm",
    "workload",
    "curve",
    "verified / planned",
    "mean S [d1]",
    "median S",
    "min S",
    "max S",
    "stdev S",
    "95% CI of mean [d2]",
    "floor S [d3]",
    "expected S [d4]",
];
const COST_HEAD: &[&str] = &[
    "arm",
    "curve",
    "matches target",
    "mean gae [d3]",
    "S / floor [d3]",
    "dp [d6]",
    "steps [d6]",
    "muls [d6]",
    "sqrs [d6]",
    "invs [d6]",
    "wall ms [d7]",
    "cpu ms [d7]",
    "time status [d7]",
];
const COST_HEAD_W: &[&str] = &[
    "arm",
    "workload",
    "matches target",
    "mean gae [d3]",
    "S / floor [d3]",
    "dp [d6]",
    "steps [d6]",
    "muls [d6]",
    "sqrs [d6]",
    "invs [d6]",
    "wall ms [d7]",
    "cpu ms [d7]",
    "time status [d7]",
];

fn s_rows(cells: &[Cell], per_workload: bool) -> Vec<Vec<String>> {
    cells
        .iter()
        .map(|c| {
            let s = &c.s;
            vec![
                c.arm.clone(),
                if per_workload {
                    c.workload_id.clone().unwrap_or_default()
                } else {
                    c.slug.clone()
                },
                if per_workload {
                    c.slug.clone()
                } else {
                    f3(c.log2_r)
                },
                format!("{} / {}", c.verified, c.planned),
                o4(s.as_ref().map(|x| x.mean)),
                o4(s.as_ref().map(|x| x.median)),
                o4(s.as_ref().map(|x| x.min)),
                o4(s.as_ref().map(|x| x.max)),
                o4(s.as_ref().and_then(|x| x.stdev)),
                ci(s),
                f4(c.floor_s),
                o4(c.expected_s),
            ]
        })
        .collect()
}

fn cost_rows(cells: &[Cell], per_workload: bool) -> Vec<Vec<String>> {
    cells
        .iter()
        .map(|c| {
            vec![
                c.arm.clone(),
                if per_workload {
                    c.workload_id.clone().unwrap_or_default()
                } else {
                    c.slug.clone()
                },
                format!("{} / {}", c.matches_target, c.verified),
                o1(c.mean_gae),
                o4(c.ratio_to_floor),
                o1(c.distinguished_points),
                o1(c.steps),
                o1(c.field_muls),
                o1(c.field_sqrs),
                o1(c.field_invs),
                o4(c.wall_ms_median),
                o4(c.cpu_ms_median),
                c.time_status.clone(),
            ]
        })
        .collect()
}

// ── Typst ──────────────────────────────────────────────────────────

fn typ_esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for ch in s.chars() {
        if "\\#$*_`<>@[]~/".contains(ch) {
            o.push('\\');
        }
        o.push(ch);
    }
    o
}

fn typ_table(out: &mut String, head: &[&str], rows: &[Vec<String>]) {
    let n = head.len();
    let _ = writeln!(
        out,
        "#table(\n  columns: {n},\n  inset: 3pt,\n  stroke: 0.4pt + luma(170),\n  fill: (x, y) => if y == 0 {{ luma(235) }} else {{ none }},"
    );
    let _ = write!(out, "  table.header(");
    for h in head {
        let _ = write!(out, "[*{}*], ", typ_esc(h));
    }
    let _ = writeln!(out, "),");
    for r in rows {
        let _ = write!(out, "  ");
        for c in r {
            let _ = write!(out, "[{}], ", typ_esc(c));
        }
        let _ = writeln!(out);
    }
    let _ = writeln!(out, ")\n");
}

pub fn typst(r: &Report, svgs: &[(&str, String)]) -> String {
    let mut o = String::new();
    let _ = writeln!(
        o,
        "#set page(paper: \"a4\", flipped: true, margin: 1.4cm)\n#set text(size: 7.5pt)\n#set par(justify: false)\n#show heading: set block(above: 1em, below: 0.5em)\n"
    );
    let _ = writeln!(o, "= ecbench report: {}\n", typ_esc(&r.session_id));
    let _ = writeln!(o, "*Label:* {}\n", typ_esc(&r.label));
    if !r.description.is_empty() {
        let _ = writeln!(o, "*Description:* {}\n", typ_esc(&r.description));
    }
    let _ = writeln!(
        o,
        "Generated by `ecbench report` ({}). Observations only; derived numbers carry the key of their derivation (last section).\n",
        typ_esc(&r.schema)
    );

    let _ = writeln!(o, "== Session\n");
    let rows = vec![
        vec!["session id".into(), r.session_id.clone()],
        vec!["spec id".into(), r.spec_id.clone().unwrap_or_default()],
        vec![
            "commit".into(),
            format!(
                "{}{}",
                r.commit.clone().unwrap_or_else(|| "not recorded".into()),
                if r.dirty == Some(true) {
                    " (dirty)"
                } else {
                    ""
                }
            ),
        ],
        vec![
            "status".into(),
            r.session_status.clone().unwrap_or_default(),
        ],
        vec![
            "run options".into(),
            r.run_options.clone().unwrap_or_default(),
        ],
    ];
    typ_table(&mut o, &["field", "value"], &rows);
    let _ = writeln!(o, "== Host and isolation\n");
    if let Some(h) = &r.host {
        let _ = writeln!(
            o,
            "{}; {} logical / {} physical cores; kernel {}; {:.1} GiB; env class {}; virtualization {}.\n",
            typ_esc(&h.cpu_model.clone().unwrap_or_else(|| "unknown CPU".into())),
            h.logical_cpus,
            h.physical_cores.map(|c| c.to_string()).unwrap_or_else(|| "?".into()),
            typ_esc(&h.kernel.clone().unwrap_or_default()),
            h.mem_gib.unwrap_or(0.0),
            typ_esc(&h.env_class_id),
            typ_esc(&h.virtualization.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(", "))
        );
    }
    let lv = &r.isolation.levels;
    let _ = writeln!(
        o,
        "Isolation earned by the {} measured records: L0 {}, L1 {}, L2 {}, L3 {}. Required of a wall-clock figure: {}.\n",
        r.runs.measured_records, lv["L0"], lv["L1"], lv["L2"], lv["L3"], r.isolation.required
    );
    if !r.isolation.blockers.is_empty() {
        let rows: Vec<Vec<String>> = r
            .isolation
            .blockers
            .iter()
            .take(12)
            .map(|(b, n)| vec![b.clone(), n.to_string()])
            .collect();
        typ_table(&mut o, &["blocker", "measured records"], &rows);
    }

    let _ = writeln!(o, "== Arms\n");
    let rows: Vec<Vec<String>> = r
        .arms
        .iter()
        .map(|a| {
            vec![
                a.name.clone(),
                a.role.clone(),
                a.method_id.clone(),
                if a.params.is_empty() {
                    "defaults".into()
                } else {
                    a.params
                        .iter()
                        .map(|(k, v)| format!("{k}={v}"))
                        .collect::<Vec<_>>()
                        .join("; ")
                },
                a.expected_s.map(f4).unwrap_or_else(|| "–".into()),
            ]
        })
        .collect();
    typ_table(
        &mut o,
        &["arm", "role", "method", "params", "expected S [d4]"],
        &rows,
    );

    let _ = writeln!(o, "== Workloads\n");
    let rows: Vec<Vec<String>> = r
        .workloads
        .iter()
        .map(|w| {
            vec![
                w.workload_id.clone(),
                w.slug.clone(),
                w.family.clone(),
                f3(w.log2_r),
                w.automorphisms.to_string(),
                w.target_index.to_string(),
                w.target_seed.to_string(),
            ]
        })
        .collect();
    typ_table(
        &mut o,
        &[
            "workload",
            "curve",
            "family",
            "log2 r",
            "A",
            "target index",
            "target seed",
        ],
        &rows,
    );

    let c = &r.runs;
    let _ = writeln!(o, "== Runs\n");
    let _ = writeln!(
        o,
        "Planned {} ({} warm-up, {} measured); records {} ({} measured); verified {} ({} with matches\\_target); not verified {}; planned without a record {}.\n",
        c.planned, c.planned_warmup, c.planned_measured, c.records, c.measured_records, c.verified, c.matches_target, c.failed, c.missing
    );
    if r.failures.is_empty() {
        let _ = writeln!(o, "No record has a status other than verified.\n");
    } else {
        let rows: Vec<Vec<String>> = r
            .failures
            .iter()
            .map(|f| {
                vec![
                    f.seq.to_string(),
                    f.arm.clone(),
                    f.workload_id.clone(),
                    if f.warmup { "warm-up" } else { "measured" }.into(),
                    f.status.clone(),
                    f.error.clone().unwrap_or_else(|| "–".into()),
                ]
            })
            .collect();
        typ_table(
            &mut o,
            &[
                "seq",
                "arm",
                "workload",
                "kind",
                "status",
                "recorded reason",
            ],
            &rows,
        );
    }

    let _ = writeln!(o, "== Graphs\n");
    let caption = format!(
        "session {}, n = {} verified runs, unit gae (S = gae / √r)",
        r.session_id, r.runs.verified
    );
    for (name, what) in svgs {
        let _ = writeln!(
            o,
            "#figure(image(\"{name}\", width: 100%), caption: [{}; {}])\n",
            typ_esc(what),
            typ_esc(&caption)
        );
    }

    let _ = writeln!(o, "== Results per arm and curve\n");
    typ_table(&mut o, S_HEAD, &s_rows(&r.curve_cells, false));
    typ_table(&mut o, COST_HEAD, &cost_rows(&r.curve_cells, false));
    let _ = writeln!(o, "== Results per arm and workload\n");
    typ_table(&mut o, S_HEAD_W, &s_rows(&r.cells, true));
    typ_table(&mut o, COST_HEAD_W, &cost_rows(&r.cells, true));

    if !r.phases.is_empty() {
        let _ = writeln!(o, "== Phases of the index-calculus arms\n");
        let rows: Vec<Vec<String>> = r
            .phases
            .iter()
            .map(|p| {
                vec![
                    p.arm.clone(),
                    p.slug.clone(),
                    p.phase.clone(),
                    p.runs.to_string(),
                    o1(Some(p.mean_gae)),
                    format!("{:.1}%", 100.0 * p.share),
                ]
            })
            .collect();
        typ_table(
            &mut o,
            &["arm", "curve", "phase", "runs", "mean gae", "share [d8]"],
            &rows,
        );
    }
    if !r.scaling.is_empty() {
        let _ = writeln!(o, "== Scaling\n");
        let rows: Vec<Vec<String>> = r
            .scaling
            .iter()
            .map(|f| {
                vec![
                    f.arm.clone(),
                    f.sizes.to_string(),
                    f.runs.to_string(),
                    f3(f.alpha),
                    f.se.map(f3).unwrap_or_else(|| "–".into()),
                    f3(f.log2_c),
                    f4(f.r2),
                ]
            })
            .collect();
        typ_table(
            &mut o,
            &[
                "arm",
                "sizes",
                "verified runs",
                "α [d5]",
                "se(α)",
                "log2 C",
                "R²",
            ],
            &rows,
        );
    }
    let _ = writeln!(o, "== Raw records\n");
    let _ = writeln!(
        o,
        "`{}/records.jsonl` ({} records; ids in `raw_records.csv` and `report.json`); child stderr in `{}/exec/` ({} file(s)).\n",
        typ_esc(&r.raw_dir),
        r.raw.len(),
        typ_esc(&r.raw_dir),
        r.exec_files
    );
    let _ = writeln!(o, "== Derivations\n");
    for (k, v) in DERIVATIONS {
        let _ = writeln!(o, "- *[{k}]* {}", typ_esc(v));
    }
    o
}

// ── SVG ────────────────────────────────────────────────────────────

const FONT: &str = "Libertinus Serif, DejaVu Sans, Helvetica, Arial, sans-serif";
const PALETTE: &[&str] = &[
    "#0072B2", "#D55E00", "#009E73", "#CC79A7", "#B8860B", "#56B4E9", "#7F3C8D", "#444444",
    "#E4572E", "#2A9D8F", "#8C6D31", "#17BECF",
];

fn color(i: usize) -> &'static str {
    PALETTE[i % PALETTE.len()]
}

fn xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

struct Svg {
    w: f64,
    h: f64,
    body: String,
}

impl Svg {
    fn new(w: f64, h: f64) -> Self {
        Svg {
            w,
            h,
            body: String::new(),
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn text(&mut self, x: f64, y: f64, s: &str, size: f64, anchor: &str, fill: &str, rot: f64) {
        let tr = if rot != 0.0 {
            format!(" transform=\"rotate({rot:.1} {x:.1} {y:.1})\"")
        } else {
            String::new()
        };
        let _ = writeln!(
            self.body,
            "<text x=\"{x:.1}\" y=\"{y:.1}\" font-size=\"{size}\" text-anchor=\"{anchor}\" fill=\"{fill}\"{tr}>{}</text>",
            xml(s)
        );
    }
    fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, stroke: &str, wd: f64, dash: &str) {
        let d = if dash.is_empty() {
            String::new()
        } else {
            format!(" stroke-dasharray=\"{dash}\"")
        };
        let _ = writeln!(
            self.body,
            "<line x1=\"{x1:.1}\" y1=\"{y1:.1}\" x2=\"{x2:.1}\" y2=\"{y2:.1}\" stroke=\"{stroke}\" stroke-width=\"{wd}\"{d}/>"
        );
    }
    fn rect(&mut self, x: f64, y: f64, w: f64, h: f64, fill: &str, stroke: &str) {
        let _ = writeln!(
            self.body,
            "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"0.8\"/>",
            w.max(0.0),
            h.max(0.0)
        );
    }
    fn circle(&mut self, x: f64, y: f64, r: f64, fill: &str, stroke: &str) {
        let _ = writeln!(
            self.body,
            "<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{r}\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"1\"/>"
        );
    }
    fn poly(&mut self, pts: &[(f64, f64)], stroke: &str, wd: f64, dash: &str) {
        let p: Vec<String> = pts.iter().map(|(x, y)| format!("{x:.1},{y:.1}")).collect();
        let d = if dash.is_empty() {
            String::new()
        } else {
            format!(" stroke-dasharray=\"{dash}\"")
        };
        let _ = writeln!(
            self.body,
            "<polyline points=\"{}\" fill=\"none\" stroke=\"{stroke}\" stroke-width=\"{wd}\"{d}/>",
            p.join(" ")
        );
    }
    fn finish(self, title: &str, desc: &str) -> String {
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w:.0}\" height=\"{h:.0}\" viewBox=\"0 0 {w:.0} {h:.0}\" font-family=\"{FONT}\">\n<title>{t}</title>\n<desc>{d}</desc>\n<rect width=\"{w:.0}\" height=\"{h:.0}\" fill=\"#ffffff\"/>\n{body}</svg>\n",
            w = self.w,
            h = self.h,
            t = xml(title),
            d = xml(desc),
            body = self.body
        )
    }
}

/// Tick values covering `[lo, hi]` in 1/2/5 × 10^k steps, about `n` of them.
fn nice_ticks(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    if hi <= lo || !lo.is_finite() || !hi.is_finite() {
        return vec![lo];
    }
    let raw = (hi - lo) / n.max(1) as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let step = [1.0, 2.0, 5.0, 10.0]
        .iter()
        .map(|m| m * mag)
        .find(|s| *s >= raw)
        .unwrap_or(10.0 * mag);
    let mut v = (lo / step).ceil() * step;
    let mut out = Vec::new();
    while v <= hi + step * 1e-9 {
        out.push(if v.abs() < step * 1e-9 { 0.0 } else { v });
        v += step;
    }
    out
}

fn tick_label(v: f64) -> String {
    if v.abs() >= 1000.0 || (v - v.round()).abs() < 1e-9 {
        format!("{v:.0}")
    } else {
        format!("{v:.2}")
    }
}

/// A plot rectangle with a y (and optionally x) scale.
struct Panel {
    x0: f64,
    y0: f64,
    w: f64,
    h: f64,
    ymin: f64,
    ymax: f64,
}

impl Panel {
    fn y(&self, v: f64) -> f64 {
        self.y0 + self.h - (v - self.ymin) / (self.ymax - self.ymin) * self.h
    }
    fn frame(&self, svg: &mut Svg, ylabel: &str) {
        for t in nice_ticks(self.ymin, self.ymax, 5) {
            let y = self.y(t);
            svg.line(self.x0, y, self.x0 + self.w, y, "#e3e3e3", 0.6, "");
            svg.text(
                self.x0 - 4.0,
                y + 3.0,
                &tick_label(t),
                8.5,
                "end",
                "#333",
                0.0,
            );
        }
        svg.line(self.x0, self.y0, self.x0, self.y0 + self.h, "#333", 1.0, "");
        svg.line(
            self.x0,
            self.y0 + self.h,
            self.x0 + self.w,
            self.y0 + self.h,
            "#333",
            1.0,
            "",
        );
        if !ylabel.is_empty() {
            svg.text(
                self.x0 - 30.0,
                self.y0 + self.h / 2.0,
                ylabel,
                9.0,
                "middle",
                "#222",
                -90.0,
            );
        }
    }
}

struct CurveInfo {
    slug: String,
    size: Option<u32>,
    log2_r: f64,
}

fn curves_of(r: &Report) -> Vec<CurveInfo> {
    let mut v: Vec<CurveInfo> = Vec::new();
    for w in &r.workloads {
        if !v.iter().any(|c| c.slug == w.slug) {
            v.push(CurveInfo {
                slug: w.slug.clone(),
                size: w.field_size,
                log2_r: w.log2_r,
            });
        }
    }
    v
}

fn panel_title(c: &CurveInfo) -> String {
    format!(
        "{} bits {}, log2 r = {:.1}",
        "field",
        c.size.map(|b| b.to_string()).unwrap_or_else(|| "?".into()),
        c.log2_r
    )
}

/// Facet one panel per curve, `cols` wide.
fn facet(
    r: &Report,
    ylabel: &str,
    title: &str,
    ymax_of: impl Fn(&CurveInfo) -> f64,
    draw: impl Fn(&mut Svg, &Panel, &CurveInfo),
    legend: &[(String, String)],
) -> String {
    let curves = curves_of(r);
    let n = curves.len().max(1);
    let cols = if n == 1 { 1 } else { n.min(4) };
    let rows = n.div_ceil(cols);
    let pw = if cols == 1 { 640.0 } else { 230.0 };
    let ph = 190.0;
    let left = 52.0;
    let gap_x = 20.0;
    let top = 40.0;
    let row_h = ph + 95.0;
    let width = left + cols as f64 * (pw + gap_x) + 10.0;
    let legend_h = 18.0 * legend.len() as f64 + 10.0;
    let height = top + rows as f64 * row_h + legend_h + 26.0;
    let mut svg = Svg::new(width, height);
    svg.text(left, 20.0, title, 12.0, "start", "#111", 0.0);
    for (i, c) in curves.iter().enumerate() {
        let (cx, cy) = ((i % cols) as f64, (i / cols) as f64);
        let p = Panel {
            x0: left + cx * (pw + gap_x),
            y0: top + cy * row_h + 14.0,
            w: pw,
            h: ph,
            ymin: 0.0,
            ymax: ymax_of(c).max(1e-9),
        };
        svg.text(p.x0, p.y0 - 5.0, &panel_title(c), 9.5, "start", "#111", 0.0);
        p.frame(&mut svg, if i % cols == 0 { ylabel } else { "" });
        draw(&mut svg, &p, c);
    }
    let ly = top + rows as f64 * row_h + 4.0;
    for (k, (sw, text)) in legend.iter().enumerate() {
        // One entry per line: the texts are long and the panel width varies.
        let (lx, lyy) = (left, ly + k as f64 * 18.0);
        if sw.starts_with('#') {
            svg.rect(lx, lyy - 8.0, 10.0, 10.0, sw, sw);
        } else {
            svg.line(lx, lyy - 3.0, lx + 14.0, lyy - 3.0, "#222", 1.4, sw);
        }
        svg.text(lx + 18.0, lyy, text, 9.0, "start", "#222", 0.0);
    }
    let cap = format!(
        "Session {}, n = {} verified runs. Unit: S = gae / √r.",
        r.session_id, r.runs.verified
    );
    svg.text(left, height - 8.0, &cap, 9.0, "start", "#333", 0.0);
    svg.finish(title, &cap)
}

fn arm_color_index(r: &Report, arm: &str) -> usize {
    r.arms.iter().position(|a| a.name == arm).unwrap_or(0)
}

/// (a) mean S per arm per curve.
pub fn svg_s_by_arm(r: &Report) -> (String, String) {
    let ymax = |c: &CurveInfo| {
        let mut m = 0.0f64;
        for x in r.curve_cells.iter().filter(|x| x.slug == c.slug) {
            if let Some(s) = &x.s {
                m = m.max(s.ci95.map(|i| i.1).unwrap_or(s.mean)).max(s.mean);
            }
            m = m.max(x.floor_s);
            if let Some(e) = x.expected_s {
                m = m.max(e);
            }
        }
        m * 1.1
    };
    let narms = r.arms.len().max(1);
    let draw = |svg: &mut Svg, p: &Panel, c: &CurveInfo| {
        let slot = p.w / narms as f64;
        let floor = r
            .curve_cells
            .iter()
            .find(|x| x.slug == c.slug)
            .map(|x| x.floor_s);
        if let Some(f) = floor {
            if f < p.ymax {
                svg.line(p.x0, p.y(f), p.x0 + p.w, p.y(f), "#B00020", 1.0, "4 3");
            }
        }
        for (i, a) in r.arms.iter().enumerate() {
            let xc = p.x0 + slot * (i as f64 + 0.5);
            svg.text(
                xc + 3.0,
                p.y0 + p.h + 10.0,
                &a.name,
                8.5,
                "end",
                "#222",
                -55.0,
            );
            let Some(cc) = r
                .curve_cells
                .iter()
                .find(|x| x.slug == c.slug && x.arm == a.name)
            else {
                continue;
            };
            let col = color(arm_color_index(r, &a.name));
            if let Some(e) = cc.expected_s {
                svg.line(xc - 7.0, p.y(e), xc + 7.0, p.y(e), "#111", 2.2, "");
            }
            for w in r
                .cells
                .iter()
                .filter(|x| x.slug == c.slug && x.arm == a.name)
            {
                if let Some(s) = &w.s {
                    svg.circle(xc + 6.0, p.y(s.mean), 2.0, "none", col);
                }
            }
            if let Some(s) = &cc.s {
                if let Some((lo, hi)) = s.ci95 {
                    svg.line(xc, p.y(lo), xc, p.y(hi), col, 1.4, "");
                    svg.line(xc - 3.5, p.y(lo), xc + 3.5, p.y(lo), col, 1.4, "");
                    svg.line(xc - 3.5, p.y(hi), xc + 3.5, p.y(hi), col, 1.4, "");
                }
                svg.circle(xc, p.y(s.mean), 3.4, col, col);
            }
        }
    };
    let legend = vec![
        ("".to_string(), "dot: mean S of the arm's verified runs on the curve; whisker: 95% bootstrap interval [d2]".to_string()),
        ("4 3".to_string(), "dashed red: floor √(π/2A) [d3]".to_string()),
        ("".to_string(), "black tick: expected S [d4]; hollow dot: per-workload mean".to_string()),
    ];
    let svg = facet(
        r,
        "S = gae / √r",
        "Mean S by arm, per curve",
        ymax,
        draw,
        &legend,
    );
    let mut csv = String::from("curve,field_size,log2_r,arm,method,verified,planned,mean_s,ci95_lo,ci95_hi,floor_s,expected_s\n");
    for c in &r.curve_cells {
        let (lo, hi) =
            c.s.as_ref()
                .and_then(|s| s.ci95)
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .unwrap_or_default();
        let _ = writeln!(
            csv,
            "{},{},{},{},{},{},{},{},{},{},{},{}",
            c.slug,
            c.field_size.map(|b| b.to_string()).unwrap_or_default(),
            c.log2_r,
            c.arm,
            c.method_id,
            c.verified,
            c.planned,
            c.s.as_ref().map(|s| s.mean.to_string()).unwrap_or_default(),
            lo,
            hi,
            c.floor_s,
            c.expected_s.map(|e| e.to_string()).unwrap_or_default()
        );
    }
    (svg, csv)
}

/// (b) distribution of S over runs, per arm, per curve.
pub fn svg_s_distribution(r: &Report, l: &Loaded) -> (String, String) {
    let runs_of = |slug: &str, arm: &str| -> Vec<f64> {
        l.records
            .iter()
            .filter(|x| x.counts() && x.arm == arm && x.workload.curve.slug == slug)
            .filter_map(|x| x.cost.s)
            .collect()
    };
    let ymax = |c: &CurveInfo| {
        let mut m = 0.0f64;
        for a in &r.arms {
            for v in runs_of(&c.slug, &a.name) {
                m = m.max(v);
            }
        }
        m * 1.08
    };
    let narms = r.arms.len().max(1);
    let draw = |svg: &mut Svg, p: &Panel, c: &CurveInfo| {
        let slot = p.w / narms as f64;
        for (i, a) in r.arms.iter().enumerate() {
            let xc = p.x0 + slot * (i as f64 + 0.5);
            svg.text(
                xc + 3.0,
                p.y0 + p.h + 10.0,
                &a.name,
                8.5,
                "end",
                "#222",
                -55.0,
            );
            let v = runs_of(&c.slug, &a.name);
            if v.is_empty() {
                continue;
            }
            let col = color(arm_color_index(r, &a.name));
            let q = |x| stats::quantile(&v, x).unwrap_or(0.0);
            let (mn, q1, md, q3, mx) = (q(0.0), q(0.25), q(0.5), q(0.75), q(1.0));
            let bw = (slot * 0.5).min(18.0);
            svg.line(xc, p.y(mn), xc, p.y(mx), col, 1.0, "");
            svg.rect(xc - bw / 2.0, p.y(q3), bw, p.y(q1) - p.y(q3), "none", col);
            svg.line(xc - bw / 2.0, p.y(md), xc + bw / 2.0, p.y(md), col, 2.0, "");
            let k = v.len();
            for (j, y) in v.iter().enumerate() {
                let jit = ((j as f64 + 0.5) / k as f64 - 0.5) * bw * 1.6;
                svg.circle(xc + jit, p.y(*y), 1.5, col, "none");
            }
        }
    };
    let legend = vec![
        ("".to_string(), "box: quartiles and median of the verified runs' S [d1]; whiskers: min and max; dots: every run".to_string()),
    ];
    let svg = facet(
        r,
        "S = gae / √r",
        "Distribution of S over runs, per arm and curve",
        ymax,
        draw,
        &legend,
    );
    let mut csv = String::from("curve,arm,run_id,workload_id,round,s\n");
    for x in l.records.iter().filter(|x| x.counts()) {
        let _ = writeln!(
            csv,
            "{},{},{},{},{},{}",
            x.workload.curve.slug,
            x.arm,
            x.run_id,
            x.workload.workload_id,
            x.round,
            x.cost.s.map(|s| s.to_string()).unwrap_or_default()
        );
    }
    (svg, csv)
}

/// (c) stacked mean gae by phase of the index-calculus arms.
pub fn svg_phases(r: &Report) -> Option<(String, String)> {
    if r.phases.is_empty() {
        return None;
    }
    let mut ic_arms: Vec<&str> = Vec::new();
    let mut phase_names: Vec<&str> = Vec::new();
    for p in &r.phases {
        if !ic_arms.contains(&p.arm.as_str()) {
            ic_arms.push(&p.arm);
        }
        if !phase_names.contains(&p.phase.as_str()) {
            phase_names.push(&p.phase);
        }
    }
    let curves = curves_of(r);
    let n = ic_arms.len();
    let cols = n.min(3);
    let rows = n.div_ceil(cols);
    let pw = 300.0;
    let ph = 200.0;
    let left = 62.0;
    let top = 40.0;
    let row_h = ph + 80.0;
    let width = left + cols as f64 * (pw + 30.0) + 10.0;
    let height = top + rows as f64 * row_h + 60.0;
    let mut svg = Svg::new(width, height);
    svg.text(
        left,
        20.0,
        "Index calculus: mean gae by phase, per curve",
        12.0,
        "start",
        "#111",
        0.0,
    );
    for (ai, arm) in ic_arms.iter().enumerate() {
        let totals: Vec<f64> = curves
            .iter()
            .map(|c| {
                r.phases
                    .iter()
                    .filter(|p| p.arm == *arm && p.slug == c.slug)
                    .map(|p| p.mean_gae)
                    .sum()
            })
            .collect();
        let ymax = totals.iter().copied().fold(0.0, f64::max) * 1.08;
        let p = Panel {
            x0: left + (ai % cols) as f64 * (pw + 30.0),
            y0: top + (ai / cols) as f64 * row_h + 14.0,
            w: pw,
            h: ph,
            ymin: 0.0,
            ymax: ymax.max(1e-9),
        };
        svg.text(p.x0, p.y0 - 5.0, arm, 9.5, "start", "#111", 0.0);
        p.frame(&mut svg, if ai % cols == 0 { "mean gae" } else { "" });
        let slot = pw / curves.len().max(1) as f64;
        for (ci, c) in curves.iter().enumerate() {
            let xc = p.x0 + slot * (ci as f64 + 0.5);
            let bw = (slot * 0.6).min(40.0);
            let mut acc = 0.0;
            for (pi, name) in phase_names.iter().enumerate() {
                let g = r
                    .phases
                    .iter()
                    .find(|x| x.arm == *arm && x.slug == c.slug && x.phase == *name)
                    .map(|x| x.mean_gae)
                    .unwrap_or(0.0);
                if g <= 0.0 {
                    continue;
                }
                let (y_top, y_bot) = (p.y(acc + g), p.y(acc));
                svg.rect(xc - bw / 2.0, y_top, bw, y_bot - y_top, color(pi), "#fff");
                acc += g;
            }
            svg.text(
                xc,
                p.y0 + p.h + 12.0,
                &c.size.map(|b| b.to_string()).unwrap_or_default(),
                8.5,
                "middle",
                "#222",
                0.0,
            );
        }
        svg.text(
            p.x0 + pw / 2.0,
            p.y0 + p.h + 26.0,
            "field size (bits of p, or degree n)",
            9.0,
            "middle",
            "#222",
            0.0,
        );
    }
    let ly = top + rows as f64 * row_h;
    for (pi, name) in phase_names.iter().enumerate() {
        let lx = left + (pi % 5) as f64 * 140.0;
        let lyy = ly + (pi / 5) as f64 * 16.0;
        svg.rect(lx, lyy - 8.0, 10.0, 10.0, color(pi), color(pi));
        svg.text(lx + 14.0, lyy, name, 9.0, "start", "#222", 0.0);
    }
    let cap = format!(
        "Session {}, n = {} verified runs. Unit: gae. Shares [d8].",
        r.session_id, r.runs.verified
    );
    svg.text(left, height - 8.0, &cap, 9.0, "start", "#333", 0.0);
    let mut csv = String::from("arm,curve,phase,runs,mean_gae,share\n");
    for p in &r.phases {
        let _ = writeln!(
            csv,
            "{},{},{},{},{},{}",
            p.arm, p.slug, p.phase, p.runs, p.mean_gae, p.share
        );
    }
    Some((
        svg.finish("Index calculus phases", "Stacked mean gae by phase"),
        csv,
    ))
}

/// (d) log2 gae against log2 r with the OLS lines.
pub fn svg_scaling(r: &Report) -> Option<(String, String, String)> {
    if r.scaling.is_empty() {
        return None;
    }
    let (mut xmin, mut xmax, mut ymin, mut ymax) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
    for f in &r.scaling {
        for p in &f.points {
            xmin = xmin.min(p.log2_r);
            xmax = xmax.max(p.log2_r);
            ymin = ymin.min(p.log2_mean_gae);
            ymax = ymax.max(p.log2_mean_gae);
        }
    }
    let (xpad, ypad) = (
        ((xmax - xmin) * 0.04).max(0.2),
        ((ymax - ymin) * 0.06).max(0.2),
    );
    let (xmin, xmax, ymin, ymax) = (xmin - xpad, xmax + xpad, ymin - ypad, ymax + ypad);
    let (pw, ph, left, top) = (560.0, 380.0, 62.0, 40.0);
    let legend_w = 300.0;
    let width = left + pw + legend_w + 20.0;
    let height = top + ph + 90.0;
    let mut svg = Svg::new(width, height);
    svg.text(
        left,
        20.0,
        "Scaling: log2 mean gae against log2 r",
        12.0,
        "start",
        "#111",
        0.0,
    );
    let sx = |v: f64| left + (v - xmin) / (xmax - xmin) * pw;
    let sy = |v: f64| top + ph - (v - ymin) / (ymax - ymin) * ph;
    for t in nice_ticks(ymin, ymax, 6) {
        svg.line(left, sy(t), left + pw, sy(t), "#e3e3e3", 0.6, "");
        svg.text(
            left - 4.0,
            sy(t) + 3.0,
            &tick_label(t),
            8.5,
            "end",
            "#333",
            0.0,
        );
    }
    for t in nice_ticks(xmin, xmax, 7) {
        svg.line(sx(t), top, sx(t), top + ph, "#f0f0f0", 0.6, "");
        svg.text(
            sx(t),
            top + ph + 12.0,
            &tick_label(t),
            8.5,
            "middle",
            "#333",
            0.0,
        );
    }
    svg.line(left, top, left, top + ph, "#333", 1.0, "");
    svg.line(left, top + ph, left + pw, top + ph, "#333", 1.0, "");
    svg.text(
        left + pw / 2.0,
        top + ph + 30.0,
        "log2 r (r = prime subgroup order)",
        9.5,
        "middle",
        "#222",
        0.0,
    );
    svg.text(
        left - 40.0,
        top + ph / 2.0,
        "log2 of mean gae",
        9.5,
        "middle",
        "#222",
        -90.0,
    );
    let mut fit_csv = String::from("arm,method,sizes,runs,alpha,se_alpha,log2_c,c,r2\n");
    let mut pts_csv = String::from("arm,curve,field_size,log2_r,runs,mean_gae,log2_mean_gae\n");
    for (i, f) in r.scaling.iter().enumerate() {
        let col = color(arm_color_index(r, &f.arm));
        let pts: Vec<(f64, f64)> = f
            .points
            .iter()
            .map(|p| (sx(p.log2_r), sy(p.log2_mean_gae)))
            .collect();
        let (a, b) = (f.points[0].log2_r, f.points[f.points.len() - 1].log2_r);
        svg.poly(
            &[
                (sx(a), sy(f.alpha * a + f.log2_c)),
                (sx(b), sy(f.alpha * b + f.log2_c)),
            ],
            col,
            1.2,
            "5 3",
        );
        for (x, y) in &pts {
            svg.circle(*x, *y, 3.0, col, col);
        }
        let ly = top + 14.0 + i as f64 * 17.0;
        let lx = left + pw + 16.0;
        svg.circle(lx + 5.0, ly - 3.0, 3.0, col, col);
        let se =
            f.se.map(|s| format!("{s:.3}"))
                .unwrap_or_else(|| "n/a".into());
        svg.text(
            lx + 14.0,
            ly,
            &format!("{}: α = {:.3} ± {} (sizes {})", f.arm, f.alpha, se, f.sizes),
            9.0,
            "start",
            "#222",
            0.0,
        );
        let _ = writeln!(
            fit_csv,
            "{},{},{},{},{},{},{},{},{}",
            f.arm,
            f.method_id,
            f.sizes,
            f.runs,
            f.alpha,
            f.se.map(|s| s.to_string()).unwrap_or_default(),
            f.log2_c,
            f.c,
            f.r2
        );
        for p in &f.points {
            let _ = writeln!(
                pts_csv,
                "{},{},{},{},{},{},{}",
                f.arm,
                p.slug,
                p.field_size.map(|b| b.to_string()).unwrap_or_default(),
                p.log2_r,
                p.n,
                p.mean_gae,
                p.log2_mean_gae
            );
        }
    }
    let ly = top + 14.0 + r.scaling.len() as f64 * 17.0 + 10.0;
    svg.text(
        left + pw + 16.0,
        ly,
        "dashed line: OLS fit over sizes [d5];",
        8.5,
        "start",
        "#333",
        0.0,
    );
    svg.text(
        left + pw + 16.0,
        ly + 12.0,
        "± is the OLS standard error of the slope.",
        8.5,
        "start",
        "#333",
        0.0,
    );
    let cap = format!(
        "Session {}, n = {} verified runs over {} sizes. Unit: gae (counted group operations).",
        r.session_id,
        r.runs.verified,
        r.scaling.iter().map(|f| f.sizes).max().unwrap_or(0)
    );
    svg.text(left, height - 10.0, &cap, 9.0, "start", "#333", 0.0);
    Some((
        svg.finish("Scaling fit", "log2 mean gae against log2 r with OLS lines"),
        pts_csv,
        fit_csv,
    ))
}

// ── Writing ────────────────────────────────────────────────────────

fn write(path: &Path, text: &str) -> Result<(), String> {
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

/// Everything the report writes, by file name.
pub fn render(l: &Loaded) -> Result<(Report, Vec<(String, String)>), String> {
    let r = build(l);
    let mut files: Vec<(String, String)> = Vec::new();
    let mut figs: Vec<(&str, String)> = Vec::new();

    let (svg, csv) = svg_s_by_arm(&r);
    files.push(("s_by_arm.svg".into(), svg));
    files.push(("s_by_arm.csv".into(), csv));
    figs.push((
        "s_by_arm.svg",
        "Mean S by arm per curve with 95% bootstrap whiskers, the floor and expected S".into(),
    ));
    let (svg, csv) = svg_s_distribution(&r, l);
    files.push(("s_distribution.svg".into(), svg));
    files.push(("s_distribution.csv".into(), csv));
    figs.push((
        "s_distribution.svg",
        "Distribution of S over the verified runs, per arm and curve".into(),
    ));
    if let Some((svg, csv)) = svg_phases(&r) {
        files.push(("phases.svg".into(), svg));
        files.push(("phases.csv".into(), csv));
        figs.push((
            "phases.svg",
            "Mean gae by phase, index-calculus arms".into(),
        ));
    }
    if let Some((svg, pts, fits)) = svg_scaling(&r) {
        files.push(("scaling.svg".into(), svg));
        files.push(("scaling.csv".into(), pts));
        files.push(("scaling_fit.csv".into(), fits));
        figs.push((
            "scaling.svg",
            "log2 mean gae against log2 r with OLS fits and fitted exponents".into(),
        ));
    }
    let mut raw =
        String::from("record_id,run_id,seq,round,warmup,arm,workload_id,status,level,s\n");
    for x in &r.raw {
        let _ = writeln!(
            raw,
            "{},{},{},{},{},{},{},{},{},{}",
            x.record_id,
            x.run_id,
            x.seq,
            x.round,
            x.warmup,
            x.arm,
            x.workload_id,
            x.status,
            x.level,
            x.s.map(|s| s.to_string()).unwrap_or_default()
        );
    }
    files.push(("raw_records.csv".into(), raw));
    files.push(("report.md".into(), markdown(&r)));
    files.push(("report.typ".into(), typst(&r, &figs)));
    files.push((
        "report.json".into(),
        serde_json::to_string_pretty(&r).map_err(|e| e.to_string())? + "\n",
    ));
    Ok((r, files))
}

/// Read `dir`, write the report into `out`; returns the files written.
pub fn write_report(dir: &Path, out: &Path) -> Result<(Report, Vec<PathBuf>), String> {
    let l = load(dir)?;
    let (r, files) = render(&l)?;
    std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let mut paths = Vec::new();
    for (name, text) in files {
        let p = out.join(&name);
        write(&p, &text)?;
        paths.push(p);
    }
    Ok((r, paths))
}
