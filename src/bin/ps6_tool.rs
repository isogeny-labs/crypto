//! PS-6 helper: conversion calibration, table memory, and the tables of the
//! PS-6 report (`research/pdp_oracle_speedup_20261011/ps6_charge/`).
//!
//! ```text
//! ps6_tool calibrate --out FILE [--rungs 15,18,21,24,27] [--reps 11]
//! ps6_tool memory    --n N --fold full|negation --out FILE
//! ps6_tool analyze   --counted DIR... [--timed DIR...] --calibration FILE
//!                    --memory FILE --out summary.json --markdown summary.md
//! ```
//!
//! This is analysis and measurement code in Rust, as the repository requires;
//! the report's graph is drawn from `summary.json` by a plotting script.
//!
//! * `calibrate` measures, per rung, the nanoseconds of one baseline pair,
//!   of one group-law kernel pair and of one group addition in the same
//!   process, interleaved, so that the kernel's pair can be priced in the
//!   unit as the framework prices every native unit: a measured ratio to
//!   `ns_per_add`, recorded.  The kernel has no pinned ratio in
//!   `docs/ic/calibration.json`; this measured one is therefore **not a
//!   pinned conversion** and the tables label every figure that uses it.
//! * `memory` builds the repository's pair table on a rung's factor base and
//!   reports its entries and the resident-memory growth, for the memory column
//!   `docs/ecbench/README.md` §7 says `S` omits.
//! * `analyze` reads sealed ecbench sessions (records only; it recomputes no
//!   count) and the two files above, and writes the per-rung tables, the
//!   exponent fits and their bootstrap intervals.

#[path = "ecbench/ps6_kernel.rs"]
mod ps6_kernel;

use std::collections::BTreeMap;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use clap::{Parser, Subcommand};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

use crypto_lib::cryptanalysis::ecbench::compare::Comparison;
use crypto_lib::cryptanalysis::ecbench::record::Record;
use crypto_lib::cryptanalysis::ecbench::runner::{read_records, read_session};
use crypto_lib::cryptanalysis::ecbench::stats::{bootstrap_ci, mean, median, quantile, Resampler};
use crypto_lib::cryptanalysis::ic_boundary::{
    binary_subspace_factor_base, calibrate_group, random_binary_instance, BinaryGroup,
    BinaryInstance, Calibration, CountedGroup, FactorBase, GroupOps, PairTable,
};
use crypto_lib::cryptanalysis::koblitz_fast::FastPoint;
use crypto_lib::cryptanalysis::semaev_decomp::SubspaceOracle;

use ps6_kernel::{base_with_torsion, degenerate_target, Kernel};

/// The boundary ledger's seed (`BoundaryConfig::default().seed`), which fixes
/// its char2 curves at `random_binary_instance(n, SEED, 8)`.
const LEDGER_SEED: u64 = 0x1C_B0_0B_DA_7A;

#[derive(Parser)]
#[command(name = "ps6_tool", about = "PS-6 calibration, memory and analysis")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Measure ns per baseline pair, kernel pair and group addition.
    Calibrate {
        #[arg(long)]
        out: PathBuf,
        #[arg(long, value_delimiter = ',', default_value = "15,18,21,24,27")]
        rungs: Vec<u32>,
        #[arg(long, default_value_t = 11)]
        reps: usize,
        /// Baseline pairs to time per repetition, at least.
        #[arg(long, default_value_t = 3_000_000)]
        min_pairs: u64,
    },
    /// Build the repository's pair table for one rung and report its memory.
    Memory {
        #[arg(long)]
        n: u32,
        #[arg(long, default_value = "full")]
        fold: String,
        #[arg(long)]
        out: PathBuf,
    },
    /// Tables and fits from sealed sessions.
    Analyze {
        #[arg(long, num_args = 1.., required = true)]
        counted: Vec<PathBuf>,
        #[arg(long, num_args = 0..)]
        timed: Vec<PathBuf>,
        #[arg(long)]
        calibration: PathBuf,
        #[arg(long)]
        memory: Option<PathBuf>,
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        markdown: PathBuf,
        #[arg(long, default_value_t = 4000)]
        resamples: usize,
    },
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("ps6_tool: {e}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<(), String> {
    match cli.cmd {
        Cmd::Calibrate {
            out,
            rungs,
            reps,
            min_pairs,
        } => calibrate(&out, &rungs, reps, min_pairs),
        Cmd::Memory { n, fold, out } => memory(n, &fold, &out),
        Cmd::Analyze {
            counted,
            timed,
            calibration,
            memory,
            out,
            markdown,
            resamples,
        } => analyze(
            &counted,
            &timed,
            &calibration,
            memory.as_deref(),
            &out,
            &markdown,
            resamples,
        ),
    }
}

fn ledger_instance(n: u32) -> Result<BinaryInstance, String> {
    random_binary_instance(n, LEDGER_SEED, 8).ok_or(format!("no ledger curve at n = {n}"))
}

/// The base and kernel of a rung, built as the ecbench plug-in builds them.
fn rung_parts(
    inst: &BinaryInstance,
) -> (u32, Vec<u64>, FactorBase<FastPoint>, Vec<(u64, u64)>) {
    let l = inst.n.div_ceil(3);
    let basis: Vec<u64> = (0..l).map(|i| 1u64 << i).collect();
    let fb = binary_subspace_factor_base(inst, &basis);
    let mut pts = Vec::new();
    for &x in &fb.abscissa_list {
        let p = fb.points[fb.x_index[&x][0]];
        pts.push((p.x, p.y));
    }
    let base = base_with_torsion(&inst.gf, inst.b, pts);
    (l, basis, fb, base)
}

fn write_jsonl(path: &Path, rows: &[Value]) -> Result<(), String> {
    let mut f = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    for r in rows {
        writeln!(f, "{r}").map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ── calibrate ─────────────────────────────────────────────────────

fn calibrate(out: &Path, rungs: &[u32], reps: usize, min_pairs: u64) -> Result<(), String> {
    let mut rows = Vec::new();
    for &n in rungs {
        let inst = ledger_instance(n)?;
        let (l, basis, _fb, base) = rung_parts(&inst);
        let slug = inst.curve_id().slug;
        let baseline = SubspaceOracle::new(&basis, inst.b, &inst.gf);
        let mut kernel = Kernel::new(&inst.gf, inst.a, l, base);
        let span = 1u64 << l;
        let full_pairs = span * (span + 1) / 2;
        let want = (min_pairs / full_pairs).clamp(8, 4000) as usize;
        let mut rng = StdRng::seed_from_u64(0x5CA1_AB1E ^ n as u64);
        let mut targets: Vec<(u64, u64)> = Vec::new();
        while targets.len() < want {
            let x = rng.gen::<u64>() & inst.gf.mask;
            if x == 0 || degenerate_target(&inst.gf, inst.b, l, x) {
                continue;
            }
            if let Some(p) = inst.points_with_x(x).first() {
                targets.push((p.x, p.y));
            }
        }
        let g = BinaryGroup(&inst.fast);
        let mut ops = GroupOps::default();
        let pts: Vec<FastPoint> = (0..64)
            .map(|_| g.mul(&mut ops, inst.generator, rng.gen_range(1..inst.r)))
            .collect();
        let (mut ns_add, mut ns_s4, mut ns_gl) = (Vec::new(), Vec::new(), Vec::new());
        let (mut s4_pairs_total, mut gl_pairs_total) = (0u64, 0u64);
        for rep in 0..reps {
            let mut time_add = || {
                let mut c = Calibration::default();
                calibrate_group(&g, &pts, &mut c);
                ns_add.push(c.ns_per_add);
            };
            let mut time_s4 = |ns: &mut Vec<f64>, tot: &mut u64| {
                let mut pairs = 0u64;
                let t = Instant::now();
                for &(x, _) in &targets {
                    pairs += baseline.decompose(x, &inst.gf).1;
                }
                let e = t.elapsed().as_nanos() as f64;
                ns.push(e / pairs as f64);
                *tot = pairs;
            };
            let mut time_gl = |ns: &mut Vec<f64>, tot: &mut u64| {
                let mut pairs = 0u64;
                let t = Instant::now();
                for &p in &targets {
                    pairs += kernel.first(p).1.pairs;
                }
                let e = t.elapsed().as_nanos() as f64;
                ns.push(e / pairs as f64);
                *tot = pairs;
            };
            // Alternate the order each repetition, so drift cancels over a pair.
            if rep % 2 == 0 {
                time_gl(&mut ns_gl, &mut gl_pairs_total);
                time_s4(&mut ns_s4, &mut s4_pairs_total);
                time_add();
            } else {
                time_add();
                time_s4(&mut ns_s4, &mut s4_pairs_total);
                time_gl(&mut ns_gl, &mut gl_pairs_total);
            }
        }
        let ratio = |a: &[f64], b: &[f64]| -> Vec<f64> {
            a.iter().zip(b).map(|(x, y)| x / y).collect()
        };
        let r_gl = ratio(&ns_gl, &ns_add);
        let r_s4 = ratio(&ns_s4, &ns_add);
        // The pinned ratio the framework prices a baseline pair at.
        let mut c = Calibration {
            ns_per_add: 1.0,
            ns_per_double: 1.0,
            ns_per_s4_pair: Some(1.0),
            ..Default::default()
        };
        let pin = c.pin("char2", &slug);
        let pinned_s4 = pin
            .pinned
            .iter()
            .any(|u| u == "ns_per_s4_pair")
            .then_some(c.ns_per_s4_pair)
            .flatten();
        rows.push(json!({
            "kind": "rung", "n": n, "l": l, "slug": slug, "reps": reps,
            "targets": targets.len(),
            "baseline_pairs_per_rep": s4_pairs_total, "kernel_pairs_per_rep": gl_pairs_total,
            "ns_per_add": ns_add, "ns_per_s4_pair": ns_s4, "ns_per_gl_pair": ns_gl,
            "ratio_gl_per_add": r_gl, "ratio_s4_per_add": r_s4,
            "ratio_gl_median": median(&r_gl), "ratio_gl_min": quantile(&r_gl, 0.0), "ratio_gl_max": quantile(&r_gl, 1.0),
            "ratio_s4_median": median(&r_s4),
            "ratio_s4_pinned": pinned_s4,
            "gl_over_s4_ns_median": median(&ns_gl.iter().zip(&ns_s4).map(|(a, b)| a / b).collect::<Vec<_>>()),
        }));
        eprintln!(
            "n={n} l={l} {slug}: ns/add {:.1}, ns/s4-pair {:.1}, ns/gl-pair {:.1}; ratio gl {:.3} s4 {:.3} (pinned {:?})",
            median(&ns_add).unwrap_or(f64::NAN),
            median(&ns_s4).unwrap_or(f64::NAN),
            median(&ns_gl).unwrap_or(f64::NAN),
            median(&r_gl).unwrap_or(f64::NAN),
            median(&r_s4).unwrap_or(f64::NAN),
            pinned_s4
        );
    }
    write_jsonl(out, &rows)
}

// ── memory ────────────────────────────────────────────────────────

fn proc_kib(key: &str) -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|t| {
            t.lines()
                .find(|l| l.starts_with(key))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|v| v.parse().ok())
        })
        .unwrap_or(0)
}

fn memory(n: u32, fold: &str, out: &Path) -> Result<(), String> {
    let inst = ledger_instance(n)?;
    let (l, _basis, fb, base) = rung_parts(&inst);
    let g = BinaryGroup(&inst.fast);
    let before = proc_kib("VmRSS:");
    let table = match fold {
        "full" => PairTable::build(&g, &fb),
        "negation" => PairTable::build_negation_folded(&g, &fb),
        other => return Err(format!("fold `{other}`: full or negation")),
    };
    let after = proc_kib("VmRSS:");
    let hwm = proc_kib("VmHWM:");
    std::hint::black_box(&table);
    let delta = after.saturating_sub(before);
    let entries = table.entries;
    // The kernel's and the baseline's own memory, from their sizes.
    let kernel_bytes = (base.len() * 16 + 2 * base.len() * 8 * 2) as u64;
    let baseline_bytes = ((1u64 << l) * 8) as u64;
    let row = json!({
        "kind": "memory", "n": n, "l": l, "slug": inst.curve_id().slug, "fold": fold,
        "signed_points": fb.points.len(), "abscissae": fb.abscissae,
        "table_entries": entries, "table_kind": table.kind,
        "rss_before_kib": before, "rss_after_kib": after, "rss_delta_kib": delta, "vmhwm_kib": hwm,
        "table_bytes_per_entry_from_rss": if entries > 0 { Some(delta as f64 * 1024.0 / entries as f64) } else { None },
        "table_build_adds": table.build_ops.adds,
        "kernel_state_bytes_estimate": kernel_bytes,
        "baseline_span_bytes": baseline_bytes,
    });
    write_jsonl(out, &[row])
}

// ── analyze ───────────────────────────────────────────────────────

/// One verified, measured run reduced to what the tables read.
#[derive(Clone)]
struct Row {
    s: f64,
    total_ops: f64,
    phases: BTreeMap<String, f64>,
    solver_ops: u64,
    rss_kib: u64,
    wall_ns: Option<u64>,
    level: String,
    lower_bound: bool,
    unpriced: Vec<String>,
}

struct Rung {
    n: u32,
    l: u32,
    slug: String,
    log2_r: f64,
    sqrt_r: f64,
    floor_s: f64,
    dir: PathBuf,
    session_id: String,
    /// arm → workload → rows over rounds
    arms: BTreeMap<String, BTreeMap<String, Vec<Row>>>,
    roles: BTreeMap<String, String>,
    measured: BTreeMap<String, u64>,
    verified: BTreeMap<String, u64>,
    workloads: Vec<String>,
}

fn load_rung(dir: &Path) -> Result<Rung, String> {
    let session = read_session(dir)?;
    let recs = read_records(dir)?;
    let mut arms: BTreeMap<String, BTreeMap<String, Vec<Row>>> = BTreeMap::new();
    let mut roles = BTreeMap::new();
    let (mut measured, mut verified) = (BTreeMap::new(), BTreeMap::new());
    let mut first: Option<&Record> = None;
    let mut workloads = std::collections::BTreeSet::new();
    for r in recs.iter().filter(|r| !r.warmup) {
        first.get_or_insert(r);
        roles.insert(r.arm.clone(), r.role.clone());
        *measured.entry(r.arm.clone()).or_insert(0u64) += 1;
        workloads.insert(r.workload.workload_id.clone());
        if !r.counts() {
            continue;
        }
        *verified.entry(r.arm.clone()).or_insert(0u64) += 1;
        let r_sub = r.workload.curve.r as f64;
        let s = r.cost.s.ok_or("verified run without S")?;
        let total = r.cost.total_gae.ok_or("verified run without total")?;
        let row = Row {
            s,
            total_ops: total,
            phases: r.phases.iter().map(|p| (p.name.clone(), p.gae)).collect(),
            solver_ops: r.solver.as_ref().map_or(0, |s| s.ops),
            rss_kib: r.time.max_rss_kib,
            wall_ns: r.time.solve_wall_ns,
            level: format!("{:?}", r.isolation.level),
            lower_bound: r.cost.lower_bound,
            unpriced: r.cost.unpriced.clone(),
        };
        debug_assert!((total / r_sub.sqrt() - s).abs() <= 1e-6 * s.abs().max(1.0));
        arms.entry(r.arm.clone())
            .or_default()
            .entry(r.workload.workload_id.clone())
            .or_default()
            .push(row);
    }
    let f = first.ok_or("session has no measured records")?;
    let r_sub = f.workload.curve.r as f64;
    let n = f.workload.curve.field_degree.ok_or("not a binary curve")?;
    Ok(Rung {
        n,
        l: n.div_ceil(3),
        slug: f.workload.curve.slug.clone(),
        log2_r: r_sub.log2(),
        sqrt_r: r_sub.sqrt(),
        floor_s: f.boundaries.floor_s,
        dir: dir.to_path_buf(),
        session_id: session.session_id,
        arms,
        roles,
        measured,
        verified,
        workloads: workloads.into_iter().collect(),
    })
}

/// How the kernel's unpriced pairs are turned into the unit, if at all.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Conv {
    /// As recorded: the kernel's pairs left out (a lower bound).
    Lb,
    /// Each kernel pair priced as one baseline pair (the pinned `ns_per_s4_pair`).
    Pairs,
    /// Each kernel pair priced at the measured ratio to a group addition.
    Est,
}

impl Conv {
    fn name(self) -> &'static str {
        match self {
            Conv::Lb => "lower bound (recorded)",
            Conv::Pairs => "pair-count (kernel pair = baseline pair)",
            Conv::Est => "measured ratio",
        }
    }
}

struct Calib {
    c_s4_pinned: f64,
    c_gl: f64,
    c_gl_min: f64,
    c_gl_max: f64,
    c_s4_measured: f64,
}

/// Per-workload value of an arm: S (or total ops) under a convention, the
/// mean over rounds.
fn value(rung: &Rung, arm: &str, conv: Conv, cal: &Calib, ops: bool, wl: &str) -> Option<f64> {
    let rows = rung.arms.get(arm)?.get(wl)?;
    let vs: Vec<f64> = rows
        .iter()
        .map(|r| {
            let extra_ops = match conv {
                Conv::Lb => 0.0,
                Conv::Pairs => r.solver_ops as f64 * cal.c_s4_pinned,
                Conv::Est => r.solver_ops as f64 * cal.c_gl,
            };
            let total = r.total_ops + extra_ops;
            if ops {
                total
            } else {
                total / rung.sqrt_r
            }
        })
        .collect();
    mean(&vs)
}

fn values(rung: &Rung, arm: &str, conv: Conv, cal: &Calib, ops: bool) -> Vec<f64> {
    rung.workloads
        .iter()
        .filter_map(|w| value(rung, arm, conv, cal, ops, w))
        .collect()
}

fn ci_mean(xs: &[f64], resamples: usize, seed: u64) -> Option<(f64, f64)> {
    bootstrap_ci(&[xs.to_vec()], resamples, seed, |s| mean(&s.concat()))
}

fn ci_ratio(num: &[f64], den: &[f64], resamples: usize, seed: u64) -> Option<(f64, f64)> {
    let pairs: Vec<(f64, f64)> = num.iter().copied().zip(den.iter().copied()).collect();
    bootstrap_ci(&[pairs], resamples, seed, |s| {
        let (a, b) = s[0].iter().fold((0.0, 0.0), |(a, b), (x, y)| (a + x, b + y));
        (b > 0.0).then(|| a / b)
    })
}

/// Least-squares slope and R² of `y` on `x`.
fn ols(x: &[f64], y: &[f64]) -> Option<(f64, f64)> {
    let n = x.len() as f64;
    if x.len() < 3 {
        return None;
    }
    let (mx, my) = (x.iter().sum::<f64>() / n, y.iter().sum::<f64>() / n);
    let sxx: f64 = x.iter().map(|a| (a - mx) * (a - mx)).sum();
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let syy: f64 = y.iter().map(|b| (b - my) * (b - my)).sum();
    (sxx > 0.0 && syy > 0.0).then(|| (sxy / sxx, sxy * sxy / (sxx * syy)))
}

fn fnum(x: Option<f64>) -> String {
    match x {
        None => "-".into(),
        Some(v) if v.abs() >= 1000.0 => format!("{v:.0}"),
        Some(v) if v.abs() >= 100.0 => format!("{v:.1}"),
        Some(v) if v.abs() >= 10.0 => format!("{v:.2}"),
        Some(v) => format!("{v:.3}"),
    }
}

fn fci(ci: Option<(f64, f64)>) -> String {
    match ci {
        None => "-".into(),
        Some((a, b)) => format!("[{}, {}]", fnum(Some(a)), fnum(Some(b))),
    }
}

fn analyze(
    counted: &[PathBuf],
    timed: &[PathBuf],
    calibration: &Path,
    memory_file: Option<&Path>,
    out: &Path,
    markdown: &Path,
    resamples: usize,
) -> Result<(), String> {
    let mut rungs = counted
        .iter()
        .map(|d| load_rung(d))
        .collect::<Result<Vec<_>, _>>()?;
    rungs.sort_by_key(|r| r.n);
    let timed_rungs = timed
        .iter()
        .map(|d| load_rung(d))
        .collect::<Result<Vec<_>, _>>()?;
    // Calibration, by n.
    let mut cal: BTreeMap<u32, Calib> = BTreeMap::new();
    for line in std::fs::read_to_string(calibration)
        .map_err(|e| format!("{}: {e}", calibration.display()))?
        .lines()
    {
        let v: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
        if v["kind"] != "rung" {
            continue;
        }
        let g = |k: &str| v[k].as_f64().ok_or(format!("calibration lacks {k}"));
        cal.insert(
            v["n"].as_u64().ok_or("n")? as u32,
            Calib {
                c_s4_pinned: g("ratio_s4_pinned")?,
                c_gl: g("ratio_gl_median")?,
                c_gl_min: g("ratio_gl_min")?,
                c_gl_max: g("ratio_gl_max")?,
                c_s4_measured: g("ratio_s4_median")?,
            },
        );
    }
    let mem: Vec<Value> = match memory_file {
        Some(p) => std::fs::read_to_string(p)
            .map_err(|e| format!("{}: {e}", p.display()))?
            .lines()
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect(),
        None => Vec::new(),
    };

    // Arms the report reads, by role-independent name.
    let rho = "rho-negation";
    let base = "s4-baseline";
    let gl = "s4-grouplaw";
    let mitm = "mitm";
    let mitm_nf = "mitm-negfold";
    let arm_list: Vec<(&str, Conv)> = vec![
        (rho, Conv::Lb),
        (base, Conv::Lb),
        (gl, Conv::Lb),
        (gl, Conv::Pairs),
        (gl, Conv::Est),
        (mitm, Conv::Lb),
        (mitm_nf, Conv::Lb),
    ];
    let label = |a: &str, c: Conv| -> String {
        if a == gl {
            format!("{a} [{}]", c.name())
        } else {
            a.to_string()
        }
    };

    let mut rung_json = Vec::new();
    let mut md = String::new();
    md.push_str("# PS-6 tables (generated by `ps6_tool analyze`; do not edit)\n\n");
    for (ri, rg) in rungs.iter().enumerate() {
        let c = cal.get(&rg.n).ok_or(format!("no calibration for n = {}", rg.n))?;
        md.push_str(&format!(
            "## n = {} (l = {}), {} , r = 2^{:.2}, floor S = {:.4}; session {}\n\n",
            rg.n, rg.l, rg.slug, rg.log2_r, rg.floor_s, rg.session_id
        ));
        md.push_str("| arm [convention] | verified/measured | lower bound | mean S [95% CI over targets] | S / floor | S / matched rho [95% CI] |\n|:--|--:|:--|--:|--:|--:|\n");
        let rho_vals = values(rg, rho, Conv::Lb, c, false);
        let mut arms_json = Vec::new();
        for (arm, conv) in &arm_list {
            if !rg.arms.contains_key(*arm) {
                continue;
            }
            let v = values(rg, arm, *conv, c, false);
            let seed = 1000 * ri as u64 + 17;
            let ci = ci_mean(&v, resamples, seed);
            let m = mean(&v);
            let ratio_floor = m.map(|x| x / rg.floor_s);
            let rr = (!rho_vals.is_empty() && v.len() == rho_vals.len())
                .then(|| {
                    (
                        v.iter().sum::<f64>() / rho_vals.iter().sum::<f64>(),
                        ci_ratio(&v, &rho_vals, resamples, seed + 1),
                    )
                });
            let lb = rg.arms[*arm]
                .values()
                .flatten()
                .any(|r| r.lower_bound)
                && *conv == Conv::Lb;
            md.push_str(&format!(
                "| {} | {}/{} | {} | {} {} | {} | {} {} |\n",
                label(arm, *conv),
                rg.verified.get(*arm).copied().unwrap_or(0),
                rg.measured.get(*arm).copied().unwrap_or(0),
                if *arm == rho {
                    "yes (canonicalisation uncounted for negation)"
                } else if lb {
                    "yes"
                } else {
                    "no"
                },
                fnum(m),
                fci(ci),
                fnum(ratio_floor),
                fnum(rr.as_ref().map(|x| x.0)),
                fci(rr.as_ref().and_then(|x| x.1)),
            ));
            // Phase split of S for this arm (mean over targets).
            let mut phase_s: BTreeMap<String, f64> = BTreeMap::new();
            let rows: Vec<&Row> = rg.arms[*arm].values().flatten().collect();
            for r in &rows {
                for (p, g) in &r.phases {
                    *phase_s.entry(p.clone()).or_insert(0.0) += g / rg.sqrt_r / rows.len() as f64;
                }
            }
            let unpriced: Vec<String> = {
                let mut u: Vec<String> = rows.iter().flat_map(|r| r.unpriced.clone()).collect();
                u.sort();
                u.dedup();
                u
            };
            let rss: Vec<f64> = rows.iter().map(|r| r.rss_kib as f64).collect();
            arms_json.push(json!({
                "arm": arm, "convention": conv.name(), "values_S": v,
                "mean_S": m, "ci95_mean_S": ci, "S_over_floor": ratio_floor,
                "S_over_rho": rr.as_ref().map(|x| x.0), "S_over_rho_ci95": rr.and_then(|x| x.1),
                "lower_bound": lb, "unpriced": unpriced,
                "phase_S_mean": phase_s,
                "max_rss_kib_median": median(&rss), "max_rss_kib_max": quantile(&rss, 1.0),
                "verified": rg.verified.get(*arm), "measured": rg.measured.get(*arm),
            }));
        }
        md.push('\n');
        // Stage ratios against the baseline, paired by target.
        md.push_str("| comparison (STAGE DIAGNOSTIC unless noted) | sum S_baseline / sum S_arm [95% CI] |\n|:--|--:|\n");
        let base_vals = values(rg, base, Conv::Lb, c, false);
        let mut cmp_json = Vec::new();
        for (arm, conv) in &arm_list {
            if *arm == rho || *arm == base || !rg.arms.contains_key(*arm) {
                continue;
            }
            let v = values(rg, arm, *conv, c, false);
            if v.len() != base_vals.len() || v.is_empty() {
                continue;
            }
            let r = base_vals.iter().sum::<f64>() / v.iter().sum::<f64>();
            let ci = ci_ratio(&base_vals, &v, resamples, 99 + ri as u64);
            md.push_str(&format!(
                "| {} | {} {} |\n",
                label(arm, *conv),
                fnum(Some(r)),
                fci(ci)
            ));
            cmp_json.push(json!({"arm": arm, "convention": conv.name(), "baseline_over_arm": r, "ci95": ci}));
        }
        md.push_str(&format!(
            "\nConversions at this rung: baseline pair pinned {:.3} GAE; kernel pair measured {:.4} GAE (min {:.4}, max {:.4} over the calibration repetitions); baseline pair measured {:.3} GAE.\n\n",
            c.c_s4_pinned, c.c_gl, c.c_gl_min, c.c_gl_max, c.c_s4_measured
        ));
        rung_json.push(json!({
            "n": rg.n, "l": rg.l, "slug": rg.slug, "log2_r": rg.log2_r, "floor_s": rg.floor_s,
            "session": rg.session_id, "dir": rg.dir, "workloads": rg.workloads.len(),
            "arms": arms_json, "baseline_over_arm": cmp_json,
            "conversions": {"c_s4_pinned": c.c_s4_pinned, "c_gl_median": c.c_gl, "c_gl_min": c.c_gl_min, "c_gl_max": c.c_gl_max, "c_s4_measured": c.c_s4_measured},
        }));
    }

    // Exponent fits: total operations T = S sqrt(r) against r, jointly
    // bootstrapped over targets (the same resampled targets for every arm).
    md.push_str("## Exponent fits (total operations T = S sqrt(r) proportional to r^e; S / floor proportional to r^(e - 1/2))\n\n");
    md.push_str("| arm [convention] | e (all rungs) | 95% CI | R^2 | slope of S/floor = e - 1/2 |\n|:--|--:|:--|--:|--:|\n");
    let xs: Vec<f64> = rungs.iter().map(|r| r.log2_r * std::f64::consts::LN_2).collect();
    let fit_arms: Vec<(&str, Conv)> = arm_list.clone();
    let point_fit = |sel: &dyn Fn(usize, &str, Conv) -> Option<f64>, arm: &str, conv: Conv| {
        let ys: Option<Vec<f64>> = (0..rungs.len())
            .map(|i| sel(i, arm, conv).map(f64::ln))
            .collect();
        ys.and_then(|y| ols(&xs, &y))
    };
    let mean_t = |i: usize, arm: &str, conv: Conv| -> Option<f64> {
        let c = cal.get(&rungs[i].n)?;
        mean(&values(&rungs[i], arm, conv, c, true))
    };
    let mut fits_json = Vec::new();
    let mut boot: Vec<Vec<f64>> = vec![Vec::new(); fit_arms.len()];
    let mut diffs: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    let mut rs = Resampler::new(20261018);
    let draws: Vec<Vec<Vec<usize>>> = (0..resamples)
        .map(|_| {
            rungs
                .iter()
                .map(|r| {
                    let w = r.workloads.len();
                    (0..w).map(|_| rs.below(w)).collect()
                })
                .collect()
        })
        .collect();
    for draw in &draws {
        let mut slopes = Vec::new();
        for (k, (arm, conv)) in fit_arms.iter().enumerate() {
            let ys: Option<Vec<f64>> = (0..rungs.len())
                .map(|i| {
                    let c = cal.get(&rungs[i].n)?;
                    let all = values(&rungs[i], arm, *conv, c, true);
                    if all.len() != rungs[i].workloads.len() {
                        return None;
                    }
                    mean(&draw[i].iter().map(|&j| all[j]).collect::<Vec<_>>()).map(f64::ln)
                })
                .collect();
            let s = ys.and_then(|y| ols(&xs, &y)).map(|f| f.0);
            if let Some(s) = s {
                boot[k].push(s);
            }
            slopes.push(s);
        }
        // Differences from the baseline's slope, paired within the draw.
        let bi = fit_arms.iter().position(|(a, _)| *a == base).unwrap();
        for (k, (arm, conv)) in fit_arms.iter().enumerate() {
            if k == bi {
                continue;
            }
            if let (Some(a), Some(b)) = (slopes[k], slopes[bi]) {
                diffs
                    .entry(format!("{} minus baseline", label(arm, *conv)))
                    .or_default()
                    .push(a - b);
            }
        }
    }
    for (k, (arm, conv)) in fit_arms.iter().enumerate() {
        let f = point_fit(&mean_t, arm, *conv);
        let ci = (boot[k].len() >= resamples * 9 / 10)
            .then(|| (quantile(&boot[k], 0.025).unwrap(), quantile(&boot[k], 0.975).unwrap()));
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            label(arm, *conv),
            fnum(f.map(|x| x.0)),
            fci(ci),
            fnum(f.map(|x| x.1)),
            fnum(f.map(|x| x.0 - 0.5)),
        ));
        fits_json.push(json!({"arm": arm, "convention": conv.name(), "exponent_e": f.map(|x| x.0), "ci95": ci, "r2": f.map(|x| x.1), "slope_S_over_floor": f.map(|x| x.0 - 0.5), "rungs": rungs.iter().map(|r| r.n).collect::<Vec<_>>()}));
    }
    md.push_str("\n| slope difference (paired bootstrap over targets) | median | 95% CI | share of resamples below 0 |\n|:--|--:|:--|--:|\n");
    let mut diff_json = Vec::new();
    for (name, v) in &diffs {
        let ci = (quantile(v, 0.025), quantile(v, 0.975));
        let neg = v.iter().filter(|x| **x < 0.0).count() as f64 / v.len().max(1) as f64;
        md.push_str(&format!(
            "| {} | {} | [{}, {}] | {:.3} |\n",
            name,
            fnum(median(v)),
            fnum(ci.0),
            fnum(ci.1),
            neg
        ));
        diff_json.push(json!({"difference": name, "median": median(v), "ci95": [ci.0, ci.1], "share_below_zero": neg}));
    }
    md.push('\n');

    // Memory.
    if !mem.is_empty() {
        md.push_str("## Memory (not in S)\n\n| n | l | fold | signed points | table entries | RSS growth of the build, MiB | bytes per entry | build adds |\n|--:|--:|:--|--:|--:|--:|--:|--:|\n");
        for m in &mem {
            md.push_str(&format!(
                "| {} | {} | {} | {} | {} | {:.1} | {} | {} |\n",
                m["n"],
                m["l"],
                m["fold"].as_str().unwrap_or("?"),
                m["signed_points"],
                m["table_entries"],
                m["rss_delta_kib"].as_f64().unwrap_or(0.0) / 1024.0,
                fnum(m["table_bytes_per_entry_from_rss"].as_f64()),
                m["table_build_adds"]
            ));
        }
        md.push('\n');
    }

    // Timed sessions: saved ecbench comparisons and per-arm wall medians.
    let mut timed_json = Vec::new();
    if !timed_rungs.is_empty() {
        md.push_str("## Timed sessions (wall time of the solve, per run; descriptive unless the comparison says admitted)\n\n");
    }
    for tr in &timed_rungs {
        md.push_str(&format!(
            "### n = {} ({}), session {}\n\n| arm | runs | median wall, ms | min..max, ms | levels | max RSS, MiB (median) |\n|:--|--:|--:|:--|:--|--:|\n",
            tr.n, tr.slug, tr.session_id
        ));
        let mut arms_j = Vec::new();
        for (arm, per_w) in &tr.arms {
            let rows: Vec<&Row> = per_w.values().flatten().collect();
            let w: Vec<f64> = rows
                .iter()
                .filter_map(|r| r.wall_ns)
                .map(|x| x as f64 / 1e6)
                .collect();
            let mut levels: BTreeMap<String, u64> = BTreeMap::new();
            for r in &rows {
                *levels.entry(r.level.clone()).or_insert(0) += 1;
            }
            let rss: Vec<f64> = rows.iter().map(|r| r.rss_kib as f64 / 1024.0).collect();
            md.push_str(&format!(
                "| {} | {}/{} | {} | {}..{} | {} | {} |\n",
                arm,
                tr.verified.get(arm).copied().unwrap_or(0),
                tr.measured.get(arm).copied().unwrap_or(0),
                fnum(median(&w)),
                fnum(quantile(&w, 0.0)),
                fnum(quantile(&w, 1.0)),
                levels
                    .iter()
                    .map(|(k, v)| format!("{k}:{v}"))
                    .collect::<Vec<_>>()
                    .join(" "),
                fnum(median(&rss)),
            ));
            arms_j.push(json!({"arm": arm, "runs": rows.len(), "median_wall_ms": median(&w), "min_wall_ms": quantile(&w, 0.0), "max_wall_ms": quantile(&w, 1.0), "levels": levels, "median_rss_mib": median(&rss)}));
        }
        md.push('\n');
        let mut comps = Vec::new();
        let cdir = tr.dir.join("comparisons");
        if let Ok(rd) = std::fs::read_dir(&cdir) {
            let mut files: Vec<PathBuf> = rd.filter_map(|e| e.ok()).map(|e| e.path()).collect();
            files.sort();
            md.push_str("| comparison B/A | ops ratio (sum S_B / sum S_A) | wall status | wall median B/A | wall 95% CI | A/A median | A/A 95% CI | outside A/A noise |\n|:--|--:|:--|--:|:--|--:|:--|:--|\n");
            for f in files {
                let Ok(text) = std::fs::read_to_string(&f) else {
                    continue;
                };
                let Ok(c) = serde_json::from_str::<Comparison>(&text) else {
                    continue;
                };
                md.push_str(&format!(
                    "| {} / {} | {} | {} | {} | {} | {} | {} | {} |\n",
                    c.b.arm,
                    c.a.arm,
                    fnum(c.ops.ratio_b_over_a),
                    c.wall.status,
                    fnum(c.wall.median_ratio),
                    fci(c.wall.ci95),
                    fnum(c.wall.aa_median_ratio),
                    fci(c.wall.aa_ci95),
                    c.wall
                        .outside_noise
                        .map_or("-".to_string(), |b| b.to_string()),
                ));
                comps.push(serde_json::to_value(&c).map_err(|e| e.to_string())?);
            }
            md.push('\n');
        }
        timed_json.push(json!({"n": tr.n, "slug": tr.slug, "session": tr.session_id, "dir": tr.dir, "arms": arms_j, "comparisons": comps}));
    }

    let summary = json!({
        "schema": "ps6_tool.summary/v1",
        "rungs": rung_json, "fits": fits_json, "slope_differences": diff_json,
        "memory": mem, "timed": timed_json, "resamples": resamples,
        "note": "mean S over targets; intervals are percentile bootstraps over targets; kernel conventions: lower bound (recorded), pair-count and measured-ratio (both NOT pinned conversions)",
    });
    std::fs::write(
        out,
        serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("{}: {e}", out.display()))?;
    std::fs::write(markdown, md).map_err(|e| format!("{}: {e}", markdown.display()))?;
    Ok(())
}
