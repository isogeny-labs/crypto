//! PS-6 helper: conversion calibration, table memory, and the tables of the
//! PS-6 report (`research/pdp_oracle_speedup_20261011/ps6_charge/`).
//!
//! ```text
//! ps6_tool calibrate --out FILE [--rungs 15,18,21,24,27] [--reps 11]
//! ps6_tool memory    --n N --fold full|negation --out FILE
//! ps6_tool analyze   --counted DIR... [--timed DIR...] --calibration FILE
//!                    --memory FILE --out summary.json --markdown summary.md
//! ps6_tool certify   --sessions DIR... --out replay-certificate.json
//! ps6_tool validate  --counted DIR... --summary summary.json --out validator-report.json
//! ps6_tool gate-json --summary summary.json --certificate F --report F --out VALIDATION.json
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

use ps6_kernel::{base_with_torsion, degenerate_target, Kernel, Pt};

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
    /// Count field operations of the baseline loop and the kernel on the same targets
    /// (needs a build with `--features pdp-opcount`).
    Opcount {
        #[arg(long)]
        out: PathBuf,
        #[arg(long, value_delimiter = ',', default_value = "15,18,21,24,27")]
        rungs: Vec<u32>,
        #[arg(long, default_value_t = 64)]
        targets: usize,
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
    /// Re-verify every recovered scalar with arithmetic that shares no code with the harness's
    /// group law (the validation gate's replay certificate).
    Certify {
        #[arg(long, num_args = 1.., required = true)]
        sessions: Vec<PathBuf>,
        #[arg(long)]
        out: PathBuf,
    },
    /// Recompute the headline numbers from the raw counters and the pinned calibration file,
    /// without `price_phase` or the analysis code (the gate's validator report).
    Validate {
        #[arg(long, num_args = 1.., required = true)]
        counted: Vec<PathBuf>,
        #[arg(long)]
        summary: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Write the gate's VALIDATION.json from the summary and the two artifacts.
    GateJson {
        #[arg(long)]
        summary: PathBuf,
        #[arg(long)]
        certificate: PathBuf,
        #[arg(long)]
        report: PathBuf,
        #[arg(long)]
        row: String,
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
        /// `ps6_tool opcount` output, for the count-based conversion.
        #[arg(long)]
        opcount: Option<PathBuf>,
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
        Cmd::Opcount {
            out,
            rungs,
            targets,
        } => opcount(&out, &rungs, targets),
        Cmd::Certify { sessions, out } => certify(&sessions, &out),
        Cmd::Validate {
            counted,
            summary,
            out,
        } => validate(&counted, &summary, &out),
        Cmd::GateJson {
            summary,
            certificate,
            report,
            row,
            out,
        } => gate_json(&summary, &certificate, &report, &row, &out),
        Cmd::Memory { n, fold, out } => memory(n, &fold, &out),
        Cmd::Analyze {
            counted,
            timed,
            calibration,
            opcount,
            memory,
            out,
            markdown,
            resamples,
        } => analyze(
            &counted,
            &timed,
            &calibration,
            opcount.as_deref(),
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
fn rung_parts(inst: &BinaryInstance) -> (u32, Vec<u64>, FactorBase<FastPoint>, Vec<(u64, u64)>) {
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

/// `want` random rational non-degenerate target points of the rung.
fn rung_targets(inst: &BinaryInstance, l: u32, want: usize, rng: &mut StdRng) -> Vec<(u64, u64)> {
    let mut targets: Vec<(u64, u64)> = Vec::new();
    while targets.len() < want {
        let x = rng.gen::<u64>() & ((1u64 << inst.n) - 1);
        if x == 0 || degenerate_target(&inst.gf, inst.b, l, x) {
            continue;
        }
        if let Some(p) = inst.points_with_x(x).first() {
            targets.push((p.x, p.y));
        }
    }
    targets
}

// ── opcount ───────────────────────────────────────────────────────

#[cfg(feature = "pdp-opcount")]
fn opcount(out: &Path, rungs: &[u32], want: usize) -> Result<(), String> {
    use crypto_lib::cryptanalysis::semaev_decomp::opcount as oc;
    let mut rows = Vec::new();
    for &n in rungs {
        let inst = ledger_instance(n)?;
        let (l, basis, _fb, base) = rung_parts(&inst);
        let baseline = SubspaceOracle::new(&basis, inst.b, &inst.gf);
        let mut kernel = Kernel::new(&inst.gf, inst.a, l, base);
        let mut rng = StdRng::seed_from_u64(0x0C0C_0C0C ^ n as u64);
        let targets = rung_targets(&inst, l, want, &mut rng);
        oc::reset();
        let mut s4_pairs = 0u64;
        for &(x, _) in &targets {
            s4_pairs += baseline.decompose(x, &inst.gf).1;
        }
        let cb = oc::snapshot();
        oc::reset();
        let mut gl_pairs = 0u64;
        for &p in &targets {
            gl_pairs += kernel.first(p).1.pairs;
        }
        let ck = oc::snapshot();
        let per = |c: u64, p: u64| c as f64 / p as f64;
        rows.push(json!({
            "kind": "opcount", "n": n, "l": l, "targets": targets.len(),
            "baseline_pairs": s4_pairs, "kernel_pairs": gl_pairs,
            "baseline_counts": serde_json::from_str::<Value>(&cb.json()).map_err(|e| e.to_string())?,
            "kernel_counts": serde_json::from_str::<Value>(&ck.json()).map_err(|e| e.to_string())?,
            "baseline_pclmul_per_pair": per(cb.pclmul(), s4_pairs),
            "kernel_pclmul_per_pair": per(ck.pclmul(), gl_pairs),
            "kernel_over_baseline_pclmul_per_pair": per(ck.pclmul(), gl_pairs) / per(cb.pclmul(), s4_pairs),
        }));
    }
    write_jsonl(out, &rows)
}

#[cfg(not(feature = "pdp-opcount"))]
fn opcount(_out: &Path, _rungs: &[u32], _want: usize) -> Result<(), String> {
    Err("rebuild with --features pdp-opcount: this build counts nothing".into())
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
        let targets = rung_targets(&inst, l, want, &mut rng);
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
            let time_s4 = |ns: &mut Vec<f64>, tot: &mut u64| {
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
        let ratio =
            |a: &[f64], b: &[f64]| -> Vec<f64> { a.iter().zip(b).map(|(x, y)| x / y).collect() };
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
            "gl_over_s4_ns_per_rep": ratio(&ns_gl, &ns_s4),
            "gl_over_s4_ns_median": median(&ratio(&ns_gl, &ns_s4)),
            "gl_over_s4_ns_min": quantile(&ratio(&ns_gl, &ns_s4), 0.0),
            "gl_over_s4_ns_max": quantile(&ratio(&ns_gl, &ns_s4), 1.0),
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
    let baseline_bytes = (1u64 << l) * 8;
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
    total_ops: f64,
    phases: BTreeMap<String, f64>,
    solver_ops: u64,
    rss_kib: u64,
    wall_ns: Option<u64>,
    level: String,
    lower_bound: bool,
    unpriced: Vec<String>,
    /// Native operations the run left unpriced (the AGENTS.md section 5a band prices each at
    /// 0, 0.1 and 1 GAE): kernel pairs, counted `*_uncharged` words, rho's canonicalisations.
    unpriced_ops: f64,
}

/// Count of the native operations behind a record's `unpriced` list.
fn unpriced_ops(r: &Record) -> f64 {
    let mut total = 0.0;
    for name in &r.cost.unpriced {
        if name.starts_with("solver_") {
            total += r.solver.as_ref().map_or(0, |s| s.ops) as f64;
        } else if let Some(v) = r.counters.get(name) {
            total += *v as f64;
        } else {
            let base = name.strip_suffix("_uncharged").unwrap_or(name);
            for p in &r.phases {
                total += p
                    .native
                    .get(name)
                    .or_else(|| p.native.get(base))
                    .copied()
                    .unwrap_or(0) as f64;
            }
        }
    }
    total
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
    measured: BTreeMap<String, u64>,
    verified: BTreeMap<String, u64>,
    workloads: Vec<String>,
    parity: Parity,
}

/// Verdict parity between the baseline and the group-law arms, read from the records.
#[derive(Default, Clone, Copy)]
struct Parity {
    /// (workload, round) groups in which all three S4 arms are present.
    groups: u64,
    /// Groups whose targets tried, relations found, matrix rank and recovered scalar agree.
    identical: u64,
    /// Witness comparisons the audit arm made (non-degenerate targets) and its mismatches.
    audit_checks: u64,
    audit_mismatches: u64,
    /// Targets sent to the baseline by the group-law arm (degenerate).
    degenerate_targets: u64,
}

fn load_rung(dir: &Path) -> Result<Rung, String> {
    let session = read_session(dir)?;
    let recs = read_records(dir)?;
    let mut arms: BTreeMap<String, BTreeMap<String, Vec<Row>>> = BTreeMap::new();
    let (mut measured, mut verified) = (BTreeMap::new(), BTreeMap::new());
    let mut first: Option<&Record> = None;
    let mut workloads = std::collections::BTreeSet::new();
    for r in recs.iter().filter(|r| !r.warmup) {
        first.get_or_insert(r);
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
            total_ops: total,
            phases: r.phases.iter().map(|p| (p.name.clone(), p.gae)).collect(),
            solver_ops: r.solver.as_ref().map_or(0, |s| s.ops),
            rss_kib: r.time.max_rss_kib,
            wall_ns: r.time.solve_wall_ns,
            level: format!("{:?}", r.isolation.level),
            lower_bound: r.cost.lower_bound,
            unpriced: r.cost.unpriced.clone(),
            unpriced_ops: unpriced_ops(r),
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
    let mut parity = Parity::default();
    let mut groups: BTreeMap<(String, u32), BTreeMap<String, &Record>> = BTreeMap::new();
    for r in recs.iter().filter(|r| r.counts()) {
        groups
            .entry((r.workload.workload_id.clone(), r.round))
            .or_default()
            .insert(r.arm.clone(), r);
    }
    for arms_in in groups.values() {
        let names = ["s4-baseline", "s4-grouplaw", "s4-grouplaw-audit"];
        if !names.iter().all(|a| arms_in.contains_key(*a)) {
            continue;
        }
        parity.groups += 1;
        let sig = |a: &str| {
            let r = arms_in[a];
            (
                r.counters.get("targets_tried").copied(),
                r.counters.get("relations_found").copied(),
                r.counters.get("matrix_rank").copied(),
                r.outcome.recovered.clone(),
            )
        };
        if names.iter().all(|a| sig(a) == sig(names[0])) {
            parity.identical += 1;
        }
        if let Some(sv) = &arms_in["s4-grouplaw-audit"].solver {
            parity.audit_checks += sv.extra.get("audit_checks").copied().unwrap_or(0);
            parity.audit_mismatches += sv.extra.get("audit_mismatches").copied().unwrap_or(0);
            parity.degenerate_targets += sv.extra.get("degenerate_targets").copied().unwrap_or(0);
        }
    }
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
        measured,
        verified,
        workloads: workloads.into_iter().collect(),
        parity,
    })
}

/// How the kernel's unpriced pairs are turned into the unit, if at all.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Conv {
    /// As recorded: every unpriced operation at weight 0 (a lower bound).
    Lb,
    /// Section 5a band: every unpriced native operation of every arm at 0.1 GAE.
    B01,
    /// Section 5a band: every unpriced native operation of every arm at 1 GAE.
    B1,
    /// Each kernel pair priced as one baseline pair (the pinned `ns_per_s4_pair`).
    Pairs,
    /// Each kernel pair priced at the pinned baseline-pair price times the measured
    /// kernel/baseline time ratio per pair (same process, interleaved).
    Est,
    /// Each kernel pair priced at its measured time over a measured group addition.
    EstAdd,
    /// Each kernel pair priced at the pinned baseline-pair price times the counted
    /// carry-less-multiply ratio per pair (deterministic; needs the opcount build).
    EstClmul,
}

impl Conv {
    /// GAE per unpriced native operation, for the band rows.
    fn weight(self) -> Option<f64> {
        match self {
            Conv::Lb => Some(0.0),
            Conv::B01 => Some(0.1),
            Conv::B1 => Some(1.0),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Conv::Lb => "w = 0 (recorded)",
            Conv::B01 => "w = 0.1",
            Conv::B1 => "w = 1",
            Conv::Pairs => "pair-count (kernel pair = baseline pair)",
            Conv::Est => "anchored estimate",
            Conv::EstAdd => "add-ratio estimate",
            Conv::EstClmul => "clmul-count estimate",
        }
    }
}

struct Calib {
    c_s4_pinned: f64,
    /// Kernel pair over a measured group addition (median, min, max over repetitions).
    c_gl: f64,
    c_gl_min: f64,
    c_gl_max: f64,
    c_s4_measured: f64,
    /// Kernel pair time over baseline pair time, same process (median, min, max).
    gl_over_s4: f64,
    gl_over_s4_min: f64,
    gl_over_s4_max: f64,
    /// Counted carry-less multiplies per kernel pair over per baseline pair.
    clmul_ratio: Option<f64>,
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
                Conv::B01 => 0.1 * r.unpriced_ops,
                Conv::B1 => r.unpriced_ops,
                Conv::Pairs => r.solver_ops as f64 * cal.c_s4_pinned,
                Conv::Est => r.solver_ops as f64 * cal.c_s4_pinned * cal.gl_over_s4,
                Conv::EstAdd => r.solver_ops as f64 * cal.c_gl,
                Conv::EstClmul => {
                    r.solver_ops as f64 * cal.c_s4_pinned * cal.clmul_ratio.unwrap_or(f64::NAN)
                }
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
        let (a, b) = s[0]
            .iter()
            .fold((0.0, 0.0), |(a, b), (x, y)| (a + x, b + y));
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
    opcount_file: Option<&Path>,
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
                gl_over_s4: g("gl_over_s4_ns_median")?,
                gl_over_s4_min: g("gl_over_s4_ns_min")?,
                gl_over_s4_max: g("gl_over_s4_ns_max")?,
                clmul_ratio: None,
            },
        );
    }
    if let Some(p) = opcount_file {
        for line in std::fs::read_to_string(p)
            .map_err(|e| format!("{}: {e}", p.display()))?
            .lines()
        {
            let v: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
            if v["kind"] != "opcount" {
                continue;
            }
            let n = v["n"].as_u64().ok_or("opcount n")? as u32;
            if let Some(c) = cal.get_mut(&n) {
                c.clmul_ratio = v["kernel_over_baseline_pclmul_per_pair"].as_f64();
            }
        }
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
    let have_clmul = !cal.is_empty() && cal.values().all(|c| c.clmul_ratio.is_some());
    let mut arm_list: Vec<(&str, Conv)> = Vec::new();
    for arm in [rho, base, gl, mitm, mitm_nf] {
        for conv in [Conv::Lb, Conv::B01, Conv::B1] {
            arm_list.push((arm, conv));
        }
        if arm == gl {
            // Further points, never a replacement for the band (AGENTS.md section 5a).
            arm_list.extend([
                (gl, Conv::Pairs),
                (gl, Conv::Est),
                (gl, Conv::EstAdd),
                (gl, Conv::EstClmul),
            ]);
        }
    }
    arm_list.retain(|(_, c)| *c != Conv::EstClmul || have_clmul);
    let label = |a: &str, c: Conv| -> String { format!("{a} [{}]", c.name()) };

    let mut rung_json = Vec::new();
    let mut md = String::new();
    md.push_str("# PS-6 tables (generated by `ps6_tool analyze`; do not edit)\n\n");
    for (ri, rg) in rungs.iter().enumerate() {
        let c = cal
            .get(&rg.n)
            .ok_or(format!("no calibration for n = {}", rg.n))?;
        md.push_str(&format!(
            "## n = {} (l = {}), {} , r = 2^{:.2}, floor S = {:.4}; session {}\n\n",
            rg.n, rg.l, rg.slug, rg.log2_r, rg.floor_s, rg.session_id
        ));
        md.push_str("| arm [convention] | verified/measured | lower bound | mean S [95% CI over targets] | S / floor | S / matched rho [95% CI] |\n|:--|--:|:--|--:|--:|--:|\n");
        let mut arms_json = Vec::new();
        for (arm, conv) in &arm_list {
            if !rg.arms.contains_key(*arm) {
                continue;
            }
            let v = values(rg, arm, *conv, c, false);
            // The reference is priced at the same weight as the arm; a further (estimate) point is
            // not ranked against a reference that is only a lower bound (section 5a).
            let rho_vals = if conv.weight().is_some() {
                values(rg, rho, *conv, c, false)
            } else {
                Vec::new()
            };
            let seed = 1000 * ri as u64 + 17;
            let ci = ci_mean(&v, resamples, seed);
            let m = mean(&v);
            let ratio_floor = m.map(|x| x / rg.floor_s);
            let rr = (!rho_vals.is_empty() && v.len() == rho_vals.len()).then(|| {
                (
                    v.iter().sum::<f64>() / rho_vals.iter().sum::<f64>(),
                    ci_ratio(&v, &rho_vals, resamples, seed + 1),
                )
            });
            let lb = rg.arms[*arm].values().flatten().any(|r| r.lower_bound) && *conv == Conv::Lb;
            let lb_text = match conv {
                Conv::Lb if *arm == rho => "yes (rho's canonicalisations unpriced)",
                Conv::Lb if lb => "yes",
                Conv::Lb => "no",
                Conv::B01 | Conv::B1 => "banded",
                _ => "estimate, not pinned",
            };
            md.push_str(&format!(
                "| {} | {}/{} | {} | {} {} | {} | {} {} |\n",
                label(arm, *conv),
                rg.verified.get(*arm).copied().unwrap_or(0),
                rg.measured.get(*arm).copied().unwrap_or(0),
                lb_text,
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
        let mut cmp_json = Vec::new();
        for (arm, conv) in &arm_list {
            if *arm == rho || *arm == base || !rg.arms.contains_key(*arm) {
                continue;
            }
            let bconv = if conv.weight().is_some() {
                *conv
            } else {
                Conv::Lb
            };
            let base_vals = values(rg, base, bconv, c, false);
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
            cmp_json.push(
                json!({"arm": arm, "convention": conv.name(), "baseline_over_arm": r, "ci95": ci}),
            );
        }
        // Section 5a: rank only where the order holds at every weight of the band.
        md.push_str("\n| section 5a order test: sum S_A / sum S_B | w = 0 | w = 0.1 | w = 1 | verdict |\n|:--|--:|--:|--:|:--|\n");
        let mut order_json = Vec::new();
        for (a, b) in [
            (gl, base),
            (mitm, base),
            (gl, mitm),
            (mitm_nf, mitm),
            (gl, rho),
            (base, rho),
            (mitm, rho),
            (mitm_nf, rho),
        ] {
            if !rg.arms.contains_key(a) || !rg.arms.contains_key(b) {
                continue;
            }
            let rs: Vec<Option<f64>> = [Conv::Lb, Conv::B01, Conv::B1]
                .iter()
                .map(|cv| {
                    let va = values(rg, a, *cv, c, false);
                    let vb = values(rg, b, *cv, c, false);
                    (va.len() == vb.len() && !va.is_empty())
                        .then(|| va.iter().sum::<f64>() / vb.iter().sum::<f64>())
                })
                .collect();
            let all: Option<Vec<f64>> = rs.iter().copied().collect();
            let verdict = match &all {
                None => "not computable",
                Some(v) if v.iter().all(|x| *x < 1.0) => "A below B at every weight",
                Some(v) if v.iter().all(|x| *x > 1.0) => "A above B at every weight",
                Some(_) => "unresolved: the order changes inside the band",
            };
            md.push_str(&format!(
                "| {} / {} | {} | {} | {} | {} |\n",
                a,
                b,
                fnum(rs[0]),
                fnum(rs[1]),
                fnum(rs[2]),
                verdict
            ));
            order_json.push(
                json!({"a": a, "b": b, "ratio_a_over_b_at_w0_w01_w1": rs, "verdict": verdict}),
            );
        }
        if rg.parity.groups > 0 {
            md.push_str(&format!(
                "\nVerdict parity (baseline, group-law and audit arms): {} of {} (workload, round) groups have identical targets tried, relations found, matrix rank and recovered scalar; the audit arm compared {} witnesses with the baseline's and found {} mismatches; {} degenerate targets went to the baseline.\n",
                rg.parity.identical,
                rg.parity.groups,
                rg.parity.audit_checks,
                rg.parity.audit_mismatches,
                rg.parity.degenerate_targets
            ));
        }
        md.push_str(&format!(
            "\nConversions at this rung (GAE per pair): baseline pair pinned {:.3}; baseline pair measured here {:.3}; kernel pair / baseline pair time {:.4} (min {:.4}, max {:.4}), so anchored kernel pair {:.4}; kernel pair over a measured addition {:.4} (min {:.4}, max {:.4}).\n\n",
            c.c_s4_pinned, c.c_s4_measured, c.gl_over_s4, c.gl_over_s4_min, c.gl_over_s4_max,
            c.c_s4_pinned * c.gl_over_s4, c.c_gl, c.c_gl_min, c.c_gl_max
        ));
        rung_json.push(json!({
            "n": rg.n, "l": rg.l, "slug": rg.slug, "log2_r": rg.log2_r, "floor_s": rg.floor_s,
            "session": rg.session_id, "dir": rg.dir, "workloads": rg.workloads.len(),
            "arms": arms_json, "baseline_over_arm": cmp_json, "order_across_band": order_json,
            "parity": {"groups": rg.parity.groups, "identical": rg.parity.identical, "audit_checks": rg.parity.audit_checks, "audit_mismatches": rg.parity.audit_mismatches, "degenerate_targets": rg.parity.degenerate_targets},
            "conversions": {"c_s4_pinned": c.c_s4_pinned, "c_gl_over_add_median": c.c_gl, "c_gl_over_add_min": c.c_gl_min, "c_gl_over_add_max": c.c_gl_max, "c_s4_measured_over_add": c.c_s4_measured, "gl_over_s4_median": c.gl_over_s4, "gl_over_s4_min": c.gl_over_s4_min, "gl_over_s4_max": c.gl_over_s4_max, "c_gl_anchored": c.c_s4_pinned * c.gl_over_s4},
        }));
    }

    // Exponent fits: total operations T = S sqrt(r) against r, jointly
    // bootstrapped over targets (the same resampled targets for every arm).
    md.push_str("## Exponent fits (total operations T = S sqrt(r) proportional to r^e; S / floor proportional to r^(e - 1/2))\n\n");
    md.push_str("| arm [convention] | e (all rungs) | 95% CI | R^2 | slope of S/floor = e - 1/2 |\n|:--|--:|:--|--:|--:|\n");
    let xs: Vec<f64> = rungs
        .iter()
        .map(|r| r.log2_r * std::f64::consts::LN_2)
        .collect();
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
        let ci = (boot[k].len() >= resamples * 9 / 10).then(|| {
            (
                quantile(&boot[k], 0.025).unwrap(),
                quantile(&boot[k], 0.975).unwrap(),
            )
        });
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
        // Paired wall-time ratios B/A over (target, round) pairs, with a cluster bootstrap over
        // targets: every timed row gets its interval, admitted or not; the levels the pairs reached
        // are printed beside it, and a figure below the spec's level is descriptive.
        md.push_str("| paired wall ratio B/A | pairs | median | 95% CI | pairs at each level (lower of the two runs) |\n|:--|--:|--:|:--|:--|\n");
        let mut paired_json = Vec::new();
        for (a, b) in [
            ("s4-baseline", "s4-baseline-control"),
            ("s4-baseline", "s4-grouplaw"),
            ("s4-baseline", "mitm"),
            ("s4-baseline", "mitm-negfold"),
            ("s4-grouplaw", "mitm"),
            ("s4-grouplaw", "mitm-negfold"),
            ("rho-negation", "s4-baseline"),
            ("rho-negation", "mitm"),
        ] {
            let (Some(ra), Some(rb)) = (tr.arms.get(a), tr.arms.get(b)) else {
                continue;
            };
            let mut strata: Vec<Vec<f64>> = Vec::new();
            let mut levels: BTreeMap<String, u64> = BTreeMap::new();
            for (w, va) in ra {
                let Some(vb) = rb.get(w) else { continue };
                let mut s = Vec::new();
                for (x, y) in va.iter().zip(vb) {
                    if let (Some(wa), Some(wb)) = (x.wall_ns, y.wall_ns) {
                        s.push(wb as f64 / wa as f64);
                        let lo = if x.level < y.level {
                            &x.level
                        } else {
                            &y.level
                        };
                        *levels.entry(lo.clone()).or_insert(0) += 1;
                    }
                }
                if !s.is_empty() {
                    strata.push(s);
                }
            }
            let all: Vec<f64> = strata.concat();
            let stat = |s: &[Vec<f64>]| median(&s.concat());
            let ci = if strata.len() >= 2 {
                crypto_lib::cryptanalysis::ecbench::stats::cluster_bootstrap_ci(
                    &strata, resamples, 31337, stat,
                )
            } else {
                bootstrap_ci(&strata, resamples, 31337, stat)
            };
            md.push_str(&format!(
                "| {} / {} | {} | {} | {} | {} |\n",
                b,
                a,
                all.len(),
                fnum(median(&all)),
                fci(ci),
                levels
                    .iter()
                    .map(|(k, v)| format!("{k}:{v}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
            paired_json.push(json!({"a": a, "b": b, "pairs": all.len(), "median_ratio_b_over_a": median(&all), "ci95": ci, "levels": levels}));
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
        timed_json.push(json!({"n": tr.n, "slug": tr.slug, "session": tr.session_id, "dir": tr.dir, "arms": arms_j, "paired_wall": paired_json, "comparisons": comps}));
    }

    let summary = json!({
        "schema": "ps6_tool.summary/v1",
        "rungs": rung_json, "fits": fits_json, "slope_differences": diff_json,
        "memory": mem, "timed": timed_json, "resamples": resamples,
        "note": "mean S over targets; intervals are percentile bootstraps over targets; kernel conventions: lower bound (recorded), pair-count, anchored estimate and add-ratio estimate (the last three are NOT pinned conversions)",
    });
    std::fs::write(
        out,
        serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("{}: {e}", out.display()))?;
    std::fs::write(markdown, md).map_err(|e| format!("{}: {e}", markdown.display()))?;
    Ok(())
}

// ── certify: independent scalar verification ──────────────────────

fn hex_u64(s: &str) -> Result<u64, String> {
    u64::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|e| format!("hex `{s}`: {e}"))
}

/// `[k]P` by double-and-add on the kernel module's own group law (`ps6_kernel::add`, built on
/// `Gf2` and written separately from `FastCurve` / `BinaryGroup`, which the harness uses).
fn smul(gf: &crypto_lib::cryptanalysis::semaev_decomp::Gf2, a: u64, k: u64, p: Pt) -> Pt {
    let mut acc = Pt::Inf;
    let mut base = p;
    let mut k = k;
    while k > 0 {
        if k & 1 == 1 {
            acc = ps6_kernel::add(gf, a, acc, base);
        }
        base = ps6_kernel::add(gf, a, base, base);
        k >>= 1;
    }
    acc
}

fn on_curve(
    gf: &crypto_lib::cryptanalysis::semaev_decomp::Gf2,
    a: u64,
    b: u64,
    x: u64,
    y: u64,
) -> bool {
    let x2 = gf.sqr(x);
    gf.sqr(y) ^ gf.mul(x, y) == gf.mul(x2, x) ^ gf.mul(a, x2) ^ b
}

fn sha256_file(p: &Path) -> Result<String, String> {
    let bytes = std::fs::read(p).map_err(|e| format!("{}: {e}", p.display()))?;
    Ok(crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex(
        &bytes,
    ))
}

fn certify(sessions: &[PathBuf], out: &Path) -> Result<(), String> {
    use crypto_lib::cryptanalysis::ecbench::workload::CurveSpec;
    let mut runs = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    let (mut checked, mut negatives, mut curves_ok) = (0u64, 0u64, 0u64);
    let mut receipts = Vec::new();
    let mut seen_curves: BTreeMap<String, bool> = BTreeMap::new();
    for dir in sessions {
        let session = read_session(dir)?;
        let recs = read_records(dir)?;
        let audit = PathBuf::from(format!("{}.audit.json", dir.display()));
        if audit.is_file() {
            let a: Value =
                serde_json::from_str(&std::fs::read_to_string(&audit).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            receipts.push(json!({
                "session": session.session_id, "path": audit, "sha256": sha256_file(&audit)?,
                "ok": a["ok"], "records": a["records"], "verified_records": a["verified_records"],
                "replays": a["replays"].as_array().map(|v| v.len()),
                "all_replays_reproduced": a["replays"].as_array().map(|v| v.iter().all(|r| r["reproduced"] == true)),
                "auditor_env_class_id": a["auditor_env_class_id"], "original_env_class_id": session.env_class_id,
            }));
        }
        for r in recs.iter().filter(|r| r.counts()) {
            let CurveSpec::BinaryRandom {
                n,
                seed,
                max_cofactor,
            } = r.workload.curve_spec.clone()
            else {
                errors.push(format!(
                    "{} seq {}: not a binary_random curve",
                    session.session_id, r.seq
                ));
                continue;
            };
            let inst = random_binary_instance(n, seed, max_cofactor)
                .ok_or(format!("no curve for n = {n}"))?;
            let gf = &inst.gf;
            let g = (
                hex_u64(&r.workload.curve.generator[0])?,
                hex_u64(&r.workload.curve.generator[1])?,
            );
            let q = (
                hex_u64(&r.workload.target[0])?,
                hex_u64(&r.workload.target[1])?,
            );
            let key = r.workload.curve.slug.clone();
            if !seen_curves.contains_key(&key) {
                // The curve's own facts, checked once: G and Q on the curve, [r]G = O.
                let ok = (g.0, g.1) == (inst.generator.x, inst.generator.y)
                    && on_curve(gf, inst.a, inst.b, g.0, g.1)
                    && smul(gf, inst.a, inst.r, Pt::Aff(g.0, g.1)) == Pt::Inf
                    && smul(gf, inst.a, inst.r - 1, Pt::Aff(g.0, g.1)) != Pt::Inf;
                seen_curves.insert(key.clone(), ok);
                if ok {
                    curves_ok += 1;
                } else {
                    errors.push(format!(
                        "{key}: generator not on the curve or not of order r"
                    ));
                }
            }
            if !on_curve(gf, inst.a, inst.b, q.0, q.1) {
                errors.push(format!(
                    "{} seq {}: target not on the curve",
                    session.session_id, r.seq
                ));
            }
            let d: u64 = r
                .outcome
                .recovered
                .as_deref()
                .ok_or("verified record without a scalar")?
                .parse()
                .map_err(|e| format!("scalar: {e}"))?;
            let gp = Pt::Aff(g.0, g.1);
            let hit = smul(gf, inst.a, d, gp) == Pt::Aff(q.0, q.1);
            let wrong = smul(gf, inst.a, (d + 1) % inst.r, gp) != Pt::Aff(q.0, q.1);
            checked += 1;
            negatives += u64::from(wrong);
            if !hit {
                errors.push(format!("{} seq {}: [d]G != Q", session.session_id, r.seq));
            }
            if !wrong {
                errors.push(format!(
                    "{} seq {}: [d+1]G = Q (negative control accepted)",
                    session.session_id, r.seq
                ));
            }
            runs.push(json!({
                "session": session.session_id, "seq": r.seq, "record_id": r.record_id, "arm": r.arm,
                "workload_id": r.workload.workload_id, "round": r.round, "scalar": d.to_string(),
                "verified": hit, "negative_control_rejected": wrong,
            }));
        }
    }
    let ok = errors.is_empty() && checked > 0;
    let cert = json!({
        "schema": "ps6-replay-certificate/v1",
        "status": if ok { "verified" } else { "failed" },
        "independent": true,
        "scalars_verified": ok,
        "method": "ps6_tool certify: for every verified record of the listed sessions, [d]G is recomputed with the group law of src/bin/ecbench/ps6_kernel.rs (Gf2 arithmetic, written separately from the FastCurve/BinaryGroup arithmetic the harness uses) from the curve parameters rebuilt from the workload's constructor call, and compared with the recorded target Q; [d+1]G != Q is checked as a negative control; each curve's generator is checked to lie on the curve and to have order r ([r]G = O, [r-1]G != O). The ecbench audit receipts (ecbench verify, exact re-execution of the runs by a separate process) are listed beside it.",
        "limits": "One host class: the harness runs, the ecbench replays and this check ran on the same Xeon 2.10 GHz KVM guest. No second host class replayed these sessions. The scalar check is a separate implementation of the group law, by the same author; it is not a second party's.",
        "outcomes_checked": checked, "negative_controls_rejected": negatives, "curves_checked": curves_ok,
        "errors": errors, "ecbench_audit_receipts": receipts, "runs": runs,
    });
    std::fs::write(
        out,
        serde_json::to_string_pretty(&cert).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("{}: {e}", out.display()))?;
    eprintln!(
        "certify: {checked} outcomes, {negatives} negative controls rejected, {} errors",
        cert["errors"].as_array().map_or(0, |e| e.len())
    );
    if ok {
        Ok(())
    } else {
        Err("certificate failed".into())
    }
}

// ── validate: recompute the headline numbers from raw counters ────

/// Pinned GAE-per-operation ratios of one curve, read through `Calibration::pin` (the lookup
/// only; the pricing below is this function's own arithmetic).
fn pinned_ratios(slug: &str) -> BTreeMap<&'static str, f64> {
    let mut c = Calibration {
        ns_per_add: 1.0,
        ns_per_double: 1.0,
        ns_per_lookup: 1.0,
        ns_per_row_op: 1.0,
        ns_per_sqrt: Some(1.0),
        ns_per_as_solve: Some(1.0),
        ns_per_s4_pair: Some(1.0),
        ns_per_word_xor: Some(1.0),
        ns_per_legendre: Some(1.0),
        ns_per_inversion: Some(1.0),
        ns_per_frobenius: Some(1.0),
        ns_per_canon: Some(1.0),
        ..Default::default()
    };
    let pin = c.pin("char2", slug);
    let has = |u: &str| pin.pinned.iter().any(|p| p == u);
    let mut m = BTreeMap::new();
    for (counter, unit, value) in [
        ("sqrt_solves", "ns_per_sqrt", c.ns_per_sqrt),
        ("as_solves", "ns_per_as_solve", c.ns_per_as_solve),
        ("s4_pairs", "ns_per_s4_pair", c.ns_per_s4_pair),
        ("lookups", "ns_per_lookup", Some(c.ns_per_lookup)),
        ("row_ops", "ns_per_row_op", Some(c.ns_per_row_op)),
        ("legendre_symbols", "ns_per_legendre", c.ns_per_legendre),
        ("inversions", "ns_per_inversion", c.ns_per_inversion),
        ("frobenius_maps", "ns_per_frobenius", c.ns_per_frobenius),
        ("canonicalisations", "ns_per_canon", c.ns_per_canon),
        (
            "target_guard_probes",
            "ns_per_lookup",
            Some(c.ns_per_lookup),
        ),
    ] {
        if has(unit) {
            if let Some(v) = value {
                m.insert(counter, v);
            }
        }
    }
    m
}

/// Total operations of one record from its raw counters.
fn recompute_total(r: &Record, ratios: &BTreeMap<&'static str, f64>) -> Result<f64, String> {
    if r.method.id.starts_with("ic.") {
        let mut t = 0.0;
        for p in &r.phases {
            t += (p.adds + p.doubles) as f64;
            for (name, count) in &p.native {
                if let Some(ratio) = ratios.get(name.as_str()) {
                    t += *count as f64 * ratio;
                }
            }
        }
        Ok(t)
    } else if r.method.id == "rho.negation" {
        let c = |k: &str| r.counters.get(k).copied().unwrap_or(0) as f64;
        Ok(c("setup_additions")
            + c("setup_doublings")
            + c("steps")
            + c("look_ahead_additions")
            + c("start_additions")
            + c("cycle_escape_doublings")
            + c("verification_operations"))
    } else {
        Err(format!("no recompute for method {}", r.method.id))
    }
}

/// Unpriced native operations of a record, from an explicit list of counter names.
fn recompute_unpriced(r: &Record) -> f64 {
    let mut t = r.solver.as_ref().map_or(0, |s| s.ops) as f64;
    for p in &r.phases {
        for k in [
            "inversions",
            "grouplaw_base_points_uncharged",
            "s4_oracle_span_words_uncharged",
            "s4_oracle_lv_coefficients_uncharged",
        ] {
            // `inversions` is unpriced only on the group-law arm (no pinned ratio for char2).
            if k == "inversions"
                && r.method
                    .params
                    .get("oracle")
                    .map_or(true, |o| !o.starts_with("semaev-s4-grouplaw"))
            {
                continue;
            }
            t += p.native.get(k).copied().unwrap_or(0) as f64;
        }
    }
    t + r
        .counters
        .get("canonicalisations_uncharged")
        .copied()
        .unwrap_or(0) as f64
}

fn validate(counted: &[PathBuf], summary: &Path, out: &Path) -> Result<(), String> {
    let sum: Value = serde_json::from_str(
        &std::fs::read_to_string(summary).map_err(|e| format!("{}: {e}", summary.display()))?,
    )
    .map_err(|e| e.to_string())?;
    let mut headline = Vec::new();
    let mut raw_data = Vec::new();
    let mut mismatches = Vec::new();
    let mut by_n: BTreeMap<u32, BTreeMap<String, Vec<(f64, f64)>>> = BTreeMap::new();
    let (mut records_checked, mut totals_equal) = (0u64, 0u64);
    for dir in counted {
        let recs = read_records(dir)?;
        raw_data.push(json!({"path": dir.join("records.jsonl"), "sha256": sha256_file(&dir.join("records.jsonl"))?}));
        for r in recs.iter().filter(|r| r.counts()) {
            let slug = &r.workload.curve.slug;
            let ratios = pinned_ratios(slug);
            let r_sub = r.workload.curve.r as f64;
            let total = recompute_total(r, &ratios)?;
            let recorded = r.cost.total_gae.ok_or("no total")?;
            records_checked += 1;
            if (total - recorded).abs() <= 1e-9 * recorded.abs().max(1.0) {
                totals_equal += 1;
            } else {
                mismatches.push(format!(
                    "{} seq {} arm {}: recomputed {total} but recorded {recorded}",
                    dir.display(),
                    r.seq,
                    r.arm
                ));
            }
            let n = r.workload.curve.field_degree.ok_or("degree")?;
            let u = recompute_unpriced(r);
            // (S at w = 0, S at w = 1), per record.
            by_n.entry(n)
                .or_default()
                .entry(r.arm.clone())
                .or_default()
                .push((total / r_sub.sqrt(), (total + u) / r_sub.sqrt()));
        }
    }
    // Headline: every rung's mean S per arm at w = 0 and at w = 1, and the baseline/kernel ratio.
    let claimed_arm = |n: u32, arm: &str, conv: &str| -> Option<f64> {
        sum["rungs"].as_array()?.iter().find(|r| r["n"] == n)?["arms"]
            .as_array()?
            .iter()
            .find(|a| a["arm"] == arm && a["convention"] == conv)?["mean_S"]
            .as_f64()
    };
    let claimed_ratio = |n: u32, arm: &str, conv: &str| -> Option<f64> {
        sum["rungs"].as_array()?.iter().find(|r| r["n"] == n)?["baseline_over_arm"]
            .as_array()?
            .iter()
            .find(|a| a["arm"] == arm && a["convention"] == conv)?["baseline_over_arm"]
            .as_f64()
    };
    for (n, arms) in &by_n {
        let mean_of = |arm: &str, w1: bool| -> Option<f64> {
            let v = arms.get(arm)?;
            // Mean over every record; with equal rounds per target this is the mean over
            // targets of the per-target means.
            Some(v.iter().map(|x| if w1 { x.1 } else { x.0 }).sum::<f64>() / v.len() as f64)
        };
        for arm in [
            "rho-negation",
            "s4-baseline",
            "s4-grouplaw",
            "mitm",
            "mitm-negfold",
        ] {
            for (w1, conv) in [(false, "w = 0 (recorded)"), (true, "w = 1")] {
                if let (Some(c), Some(rc)) = (claimed_arm(*n, arm, conv), mean_of(arm, w1)) {
                    headline.push(json!({"name": format!("n={n} {arm} mean S, {conv}"), "claimed": c, "recomputed": rc, "rel_tol": 1e-9}));
                }
            }
        }
        for (arm, conv) in [
            ("s4-grouplaw", "w = 0 (recorded)"),
            ("s4-grouplaw", "w = 1"),
            ("mitm", "w = 0 (recorded)"),
        ] {
            let w1 = conv == "w = 1";
            if let (Some(c), Some(b), Some(k)) = (
                claimed_ratio(*n, arm, conv),
                mean_of("s4-baseline", w1),
                mean_of(arm, w1),
            ) {
                // The tool's ratio is sum S_baseline / sum S_arm over targets; with equal
                // target counts it equals the ratio of means.
                headline.push(json!({"name": format!("n={n} baseline / {arm} sum-S ratio, {conv}"), "claimed": c, "recomputed": b / k, "rel_tol": 1e-9}));
            }
        }
    }
    let ok = mismatches.is_empty() && !headline.is_empty();
    let all_match = headline.iter().all(|h| {
        let (c, r) = (
            h["claimed"].as_f64().unwrap_or(f64::NAN),
            h["recomputed"].as_f64().unwrap_or(f64::NAN),
        );
        (c - r).abs() <= 1e-9 * c.abs().max(r.abs())
    });
    let report = json!({
        "schema": "ps6-validator-report/v1",
        "status": if ok && all_match { "pass" } else { "fail" },
        "independent": true,
        "method": "ps6_tool validate: each verified record's total operations are recomputed from its raw counters (group additions and doublings per phase, native counters times the pinned ratios of docs/ic/calibration.json read through Calibration::pin; rho from its step counters) with arithmetic that shares no code with price_phase, the ecbench runner or `ps6_tool analyze`, and compared with the recorded total; the headline numbers of summary.json are then recomputed from those totals.",
        "limits": "Same author and host as the measurement; the pinned ratios are the repository's, not re-derived. The kernel's pairs are priced here only as the section 5a band (their count times 0 or 1 GAE).",
        "records_checked": records_checked, "records_with_recomputed_total_equal_to_recorded": totals_equal,
        "mismatches": mismatches, "raw_data": raw_data, "headline": headline,
    });
    std::fs::write(
        out,
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("{}: {e}", out.display()))?;
    eprintln!("validate: {records_checked} records, {totals_equal} totals equal; {} headline numbers; status {}", report["headline"].as_array().map_or(0, |h| h.len()), report["status"]);
    if report["status"] == "pass" {
        Ok(())
    } else {
        Err("validator report failed".into())
    }
}

fn gate_json(
    summary: &Path,
    cert: &Path,
    report: &Path,
    row: &str,
    out: &Path,
) -> Result<(), String> {
    let sum: Value =
        serde_json::from_str(&std::fs::read_to_string(summary).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    // The largest rung's mean S of the group-law arm (ic) and of the matched rho at each weight.
    let top = sum["rungs"]
        .as_array()
        .and_then(|r| r.last())
        .ok_or("no rungs")?;
    let get = |arm: &str, conv: &str| -> Result<f64, String> {
        top["arms"]
            .as_array()
            .ok_or("arms")?
            .iter()
            .find(|a| a["arm"] == arm && a["convention"] == conv)
            .and_then(|a| a["mean_S"].as_f64())
            .ok_or(format!("no {arm} {conv}"))
    };
    let band = |conv: &str| -> Result<Value, String> {
        Ok(json!({"ic": get("s4-grouplaw", conv)?, "rho": get("rho-negation", conv)?}))
    };
    let (b0, b01, b1) = (band("w = 0 (recorded)")?, band("w = 0.1")?, band("w = 1")?);
    let order_holds = [&b0, &b01, &b1]
        .iter()
        .all(|b| b["ic"].as_f64().unwrap_or(0.0) > b["rho"].as_f64().unwrap_or(f64::MAX));
    let rel = |p: &Path, from: &Path| -> String { pathdiff(p, from) };
    let dir = out.parent().ok_or("out dir")?;
    let v = json!({
        "schema": "validation-gate/v1",
        "rows": [row],
        "replay_certificate": {"path": rel(cert, dir), "sha256": sha256_file(cert)?},
        "validator_report": {"path": rel(report, dir), "sha256": sha256_file(report)?},
        "rho_reference": {
            "kind": "negation",
            "protocol": "PS-6 (protocols/PS-6.md): matched strong rho on the same one-target workloads; on a random binary curve the automorphism group is {+-1}, so rho.negation is the strongest single-target rho the harness has for these curves (no Frobenius structure).",
            "unit": "S = group-addition equivalents / sqrt(r), cold, one target per run",
            "binary_or_process": "one ecbench binary (SHA-256 in each session.json), every arm a fresh process of it inside one session, same workloads and algorithm seeds",
            "matches_protocol": true, "strongest_available": true, "same_unit": true, "same_binary_or_process": true,
            "gae_band": {"0": b0, "0.1": b01, "1": b1},
            "gae_band_basis": format!("rung n = {}: ic is the group-law kernel arm (its {} unpriced native operations per run dominated by the kernel's pairs) and rho the rho.negation arm (its canonicalisations), each unpriced native operation priced at w GAE; means over the rung's targets and rounds; every other rung and arm is in summary.json", top["n"], "counted"),
            "order_holds_across_band": order_holds,
        },
    });
    std::fs::write(
        out,
        serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("{}: {e}", out.display()))
}

/// `p` relative to directory `from`, both inside the repository (lexical).
fn pathdiff(p: &Path, from: &Path) -> String {
    let p = std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    let from = std::fs::canonicalize(from).unwrap_or_else(|_| from.to_path_buf());
    let pc: Vec<_> = p.components().collect();
    let fc: Vec<_> = from.components().collect();
    let common = pc.iter().zip(&fc).take_while(|(a, b)| a == b).count();
    let mut out = PathBuf::new();
    for _ in common..fc.len() {
        out.push("..");
    }
    for c in &pc[common..] {
        out.push(c.as_os_str());
    }
    out.to_string_lossy().into_owned()
}
