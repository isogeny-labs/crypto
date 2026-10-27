//! Measurement driver for the orbit-aware double-large-prime experiment
//! (`research/orbit_aware_large_prime_20261011/`).
//!
//! ```text
//! orbit_large_prime_bench cell  --n 17 --arm dlp_orbit --seed 1 --small 5 --env 9 \
//!         --summands 4 --trials 2000 --max-states 2000000 --out DIR [--rep 1]
//! orbit_large_prime_bench manifest --n 17 --small 5 --env 9 --out FILE
//! orbit_large_prime_bench summarize --in DIR --out DIR
//! ```
//!
//! Options of `cell` added by the 2026-10-26 addendum
//! (`research/orbit_aware_large_prime_20261011/PROTOCOL_ADDENDUM_20261026.md`):
//!
//! * `--arm dlp_orbit_post`: decompositions are accepted on the plain column
//!   count (the stream of `dlp`) and the orbit map is applied afterwards.
//! * `--modulus-seed K`: the same abstract curve over the K-th seeded random
//!   irreducible modulus (the realisation series).
//! * `--cutoffs 50,100,200`: also report the pipeline state at those trial
//!   cutoffs, and the trial at which the verified rank first reaches 50, 75,
//!   90 and 100 percent of S+1 (the cap-free metric); the run length is
//!   `--trials`.
//!
//! Every cell hashes its accepted decompositions (`report.filter_pipeline.stream`)
//! so two arms can be shown to have seen the same relation stream.
//!
//! `cell` runs exactly one (curve, arm, seed) in this process, so the peak
//! resident set size it records belongs to that cell alone, and writes one
//! JSON receipt.  `summarize` reads every receipt in a directory and writes
//! the tables and the SVG chart.  No Python is involved anywhere.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use crypto_lib::cryptanalysis::ecbench_large_prime::{
    process_cpu_ns, solve_with_options, LargePrimeGraph, SolveConfig, SolveOptions,
    TrajectoryRequest,
};
use crypto_lib::cryptanalysis::glv_gaudry::peak_rss_bytes;
use crypto_lib::cryptanalysis::ic_boundary::{koblitz_instance, BinaryInstance};
use crypto_lib::cryptanalysis::koblitz_fast::FastPoint;
use serde_json::{json, Value};

#[path = "../tests/common/orbit_support.rs"]
mod orbit_support;
use orbit_support::{instance_with_modulus, random_modulus, SplitMix};

const SCHEMA: &str = "orbit-lp-cell/v1";

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn need(args: &[String], name: &str) -> Result<String, String> {
    arg(args, name).ok_or_else(|| format!("missing {name}"))
}

fn num(args: &[String], name: &str) -> Result<u64, String> {
    need(args, name)?
        .parse::<u64>()
        .map_err(|e| format!("{name}: {e}"))
}

/// The Koblitz curve K_a / F_{2^n} (a in {0,1}) whose largest prime factor
/// of the group order is the biggest admitted subgroup, and a planted target.
///
/// With `modulus_seed` the same abstract curve is represented over the K-th
/// seeded random irreducible modulus (the realisation series): same group,
/// `r` and cofactor, but the prefix window `x < 2^env` is a different
/// `F_2`-subspace of the field.
fn case(n: u32, modulus_seed: Option<u64>) -> Result<(BinaryInstance, FastPoint, u64), String> {
    let candidates = [koblitz_instance(1, n), koblitz_instance(0, n)];
    let inst = candidates
        .into_iter()
        .flatten()
        .max_by_key(|i| i.r)
        .ok_or_else(|| format!("no Koblitz curve with a usable prime subgroup at n={n}"))?;
    let inst = match modulus_seed {
        None => inst,
        Some(k) => {
            instance_with_modulus(inst.a, n, random_modulus(n, k), inst.group_order, inst.r)?
        }
    };
    let mut rng = SplitMix(0x6f72_6269_745f_6c70 ^ u64::from(n));
    let planted = 1 + rng.next() % (inst.r - 1);
    let target = inst.fast.mul_u64(inst.generator, planted);
    Ok((inst, target, planted))
}

fn arm_config(arm: &str) -> Result<(LargePrimeGraph, u8), String> {
    Ok(match arm {
        "none" => (LargePrimeGraph::Filter, 0),
        "dlp" => (LargePrimeGraph::Filter, 2),
        "dlp_orbit" => (LargePrimeGraph::FilterOrbit, 2),
        "dlp_orbit_post" => (LargePrimeGraph::FilterOrbitPostCollect, 2),
        "dlp_orbit_absorb" => (LargePrimeGraph::FilterOrbitAbsorb, 2),
        "dlp_online" => (LargePrimeGraph::Online, 2),
        other => return Err(format!("unknown arm `{other}`")),
    })
}

fn cell(args: &[String]) -> Result<(), String> {
    let n = num(args, "--n")? as u32;
    let arm = need(args, "--arm")?;
    let seed = num(args, "--seed")?;
    let rep = arg(args, "--rep").map_or(Ok(1), |v| v.parse::<u64>().map_err(|e| e.to_string()))?;
    let out = PathBuf::from(need(args, "--out")?);
    let series = arg(args, "--series").unwrap_or_else(|| "primary".into());
    let (graph, max_lp) = arm_config(&arm)?;
    let modulus_seed = arg(args, "--modulus-seed")
        .map(|v| v.parse::<u64>().map_err(|e| format!("--modulus-seed: {e}")))
        .transpose()?;
    let cutoffs: Vec<u64> = match arg(args, "--cutoffs") {
        Some(list) => list
            .split(',')
            .map(|v| {
                v.trim()
                    .parse::<u64>()
                    .map_err(|e| format!("--cutoffs: {e}"))
            })
            .collect::<Result<_, _>>()?,
        None => Vec::new(),
    };
    let wall = Instant::now();
    let cpu0 = process_cpu_ns();
    let (inst, target, planted) = case(n, modulus_seed)?;
    let cpu_instance = process_cpu_ns().saturating_sub(cpu0);
    let summands = match need(args, "--summands")?.as_str() {
        "n-1" => n - 1,
        v => v.parse::<u32>().map_err(|e| format!("--summands: {e}"))?,
    };
    let config = SolveConfig {
        small_dimension: num(args, "--small")? as u32,
        envelope_dimension: num(args, "--env")? as u32,
        summands,
        max_large_primes: max_lp,
        max_trials: num(args, "--trials")?,
        max_states: num(args, "--max-states")?,
        seed,
        large_prime_graph: graph,
    };
    let max_trials = config.max_trials;
    // The accepted-decomposition stream is hashed in every cell so that two
    // arms can be shown to have seen the same relation stream.
    let options = SolveOptions {
        stream_digest: graph.uses_filter(),
        trajectory: (!cutoffs.is_empty()).then_some(TrajectoryRequest { cutoffs }),
        ..Default::default()
    };
    let report = solve_with_options(&inst, target, config, options)?;
    let cpu_total = process_cpu_ns().saturating_sub(cpu0);
    let modulus_mask = inst
        .irreducible
        .low_terms
        .iter()
        .fold(1u64 << n, |m, &t| m | (1u64 << t));
    let receipt = json!({
        "schema": SCHEMA,
        "n": n,
        "curve": { "a": inst.a, "b": inst.b, "name": inst.name, "group_order": inst.group_order,
                   "r": inst.r, "cofactor": inst.cofactor,
                   "modulus_mask": modulus_mask, "modulus_seed": modulus_seed },
        "series": series,
        "arm": arm,
        "rep": rep,
        "seed": seed,
        "planted_log": planted,
        "recovered_matches_planted": report.recovered.map(|d| d == planted),
        "cpu_instance_ns": cpu_instance,
        "cpu_process_ns": cpu_total,
        "wall_ns": wall.elapsed().as_nanos() as u64,
        "peak_rss_bytes": peak_rss_bytes(),
        "report": serde_json::to_value(&report).map_err(|e| e.to_string())?,
    });
    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let name = format!("cell_{series}_n{n}_{arm}_t{max_trials}_s{seed}_r{rep}.json");
    let path = out.join(name);
    fs::write(
        &path,
        serde_json::to_string_pretty(&receipt).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("write {}: {e}", path.display()))?;
    eprintln!("wrote {}", path.display());
    Ok(())
}

/// An `ecbench.curve-corpus/v1` manifest for the same curve and target, so
/// `ic large-prime` can import and cross-check a cell.  Only `a = 1` curves
/// are importable by `ic`.
fn manifest(args: &[String]) -> Result<(), String> {
    let n = num(args, "--n")? as u32;
    let (inst, target, planted) = case(n, None)?;
    if inst.a != 1 {
        return Err(format!(
            "n={n} selects a=0, which the corpus importer does not accept"
        ));
    }
    let mut modulus = 1u64 << n;
    for t in &inst.irreducible.low_terms {
        modulus |= 1u64 << t;
    }
    let doc = json!({
        "schema": "ecbench.curve-corpus/v1",
        "families": [{
            "id": format!("koblitz-n{n}-orbit-lp"),
            "field": { "characteristic": 2, "degree": n, "modulus": modulus },
            "curve": { "equation": "y^2 + x*y = x^3 + x^2 + 1", "order": inst.group_order },
            "factor_base": { "dimension": num(args, "--env")?, "small_dimension": num(args, "--small")? },
            "dlog_challenge": {
                "base_point": { "x": inst.generator.x, "y": inst.generator.y },
                "target_point": { "x": target.x, "y": target.y },
                "group_order": inst.group_order,
                "subgroup_order": inst.r,
                "cofactor": inst.cofactor,
                "planted_log": planted
            }
        }]
    });
    let path = PathBuf::from(need(args, "--out")?);
    fs::write(
        &path,
        serde_json::to_string_pretty(&doc).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("write {}: {e}", path.display()))?;
    eprintln!("wrote {}", path.display());
    Ok(())
}

// ---------------------------------------------------------------- summarize

fn at<'a>(v: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.').try_fold(v, |cur, key| cur.get(key))
}

/// A required unsigned integer field.  A missing or non-integer field is an
/// error, not a zero: an absent integrity counter must not read as "0
/// failures".
fn req(v: &Value, path: &str) -> Result<u64, String> {
    at(v, path)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing or non-integer field `{path}`"))
}

/// One receipt.
#[derive(Clone, Debug)]
struct Cell {
    series: String,
    n: u32,
    arm: String,
    rep: u64,
    seed: u64,
    a: u64,
    r: u64,
    cofactor: u64,
    trials: u64,
    partial: [u64; 3],
    complete: u64,
    verified: u64,
    failed_linear: u64,
    failed_curve: u64,
    audit_failed: u64,
    independent: u64,
    rank: u64,
    small_columns: u64,
    cycles_closed: u64,
    online_cpu: f64,
    setup_cpu: f64,
    audit_cpu: f64,
    rss_mb: f64,
    states: u64,
    recovered_ok: Option<bool>,
    large_points: Option<u64>,
    large_orbits: Option<u64>,
    merged: Option<u64>,
    absorbed: Option<u64>,
    file: String,
}

impl Cell {
    fn from_value(v: &Value, file: &str) -> Result<Self, String> {
        if v["schema"] != SCHEMA {
            return Err("not an orbit-lp cell receipt".into());
        }
        // Every counter, and above all every integrity counter (replay and
        // audit failures), is required: a receipt that lacks one is an
        // error, never a zero.
        let ns = |p: &str| -> Result<f64, String> { Ok(req(v, p)? as f64 / 1e9) };
        let orbit = at(v, "report.factor_base.orbit").filter(|o| !o.is_null());
        let pc = |i: usize| -> Result<u64, String> {
            at(v, "report.filter_pipeline.partials_by_class")
                .and_then(|a| a.get(i))
                .and_then(Value::as_u64)
                .ok_or_else(|| format!("missing partials_by_class[{i}]"))
        };
        let orbit_field =
            |name: &str| -> Result<Option<u64>, String> { orbit.map(|o| req(o, name)).transpose() };
        Ok(Self {
            series: v["series"].as_str().unwrap_or("primary").to_string(),
            n: req(v, "n")? as u32,
            arm: v["arm"].as_str().unwrap_or("?").to_string(),
            rep: req(v, "rep")?,
            seed: req(v, "seed")?,
            a: req(v, "curve.a")?,
            r: req(v, "curve.r")?,
            cofactor: req(v, "curve.cofactor")?,
            trials: req(v, "report.counters.trials")?,
            partial: [pc(0)?, pc(1)?, pc(2)?],
            complete: req(v, "report.filter_pipeline.complete_relations")?,
            verified: req(v, "report.filter_pipeline.replays_verified")?,
            failed_linear: req(v, "report.filter_pipeline.replay_failed_linear")?,
            failed_curve: req(v, "report.filter_pipeline.replay_failed_curve")?,
            audit_failed: req(v, "report.filter_pipeline.partial_audit_failed")?,
            independent: req(v, "report.filter_pipeline.independent_verified")?,
            rank: req(v, "report.filter_pipeline.rank")?,
            small_columns: req(v, "report.factor_base.small_columns")?,
            cycles_closed: req(v, "report.filter_pipeline.filter.cycles_closed")?,
            online_cpu: ns("report.filter_pipeline.cpu_ns.collect")?
                + ns("report.filter_pipeline.cpu_ns.filter")?
                + ns("report.filter_pipeline.cpu_ns.replay")?,
            setup_cpu: ns("report.filter_pipeline.cpu_ns.setup")?,
            audit_cpu: ns("report.filter_pipeline.cpu_ns.audit")?,
            rss_mb: req(v, "peak_rss_bytes")? as f64 / 1048576.0,
            states: req(v, "report.mitm.left_states")? + req(v, "report.mitm.right_states")?,
            recovered_ok: v["recovered_matches_planted"].as_bool(),
            large_points: orbit_field("large_points")?,
            large_orbits: orbit_field("large_orbits")?,
            merged: orbit_field("merged_points")?,
            absorbed: orbit_field("absorbed_into_small")?,
            file: file.to_string(),
        })
    }

    fn counters(&self) -> (u64, [u64; 3], u64, u64, u64, u64) {
        (
            self.trials,
            self.partial,
            self.complete,
            self.verified,
            self.independent,
            self.rank,
        )
    }
}

/// The repeats of one (series, n, cap, arm, seed): deterministic counters
/// from repeat 1, CPU averaged over the repeats.
#[derive(Clone, Debug)]
struct Agg {
    cell: Cell,
    reps: usize,
    online_cpu: f64,
    counters_agree: bool,
}

impl Agg {
    fn cpu_per_verified(&self) -> Option<f64> {
        (self.cell.independent > 0).then(|| self.online_cpu / self.cell.independent as f64)
    }

    fn complete_per_attempt(&self) -> f64 {
        self.cell.complete as f64 / self.cell.trials.max(1) as f64
    }
}

fn mean(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}

fn sd(xs: &[f64]) -> f64 {
    if xs.len() < 2 {
        return f64::NAN;
    }
    let m = mean(xs);
    (xs.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (xs.len() - 1) as f64).sqrt()
}

/// Two-sided 95% Student-t critical value for `df` degrees of freedom.
fn t95(df: usize) -> f64 {
    const T: [f64; 12] = [
        12.706, 4.303, 3.182, 2.776, 2.571, 2.447, 2.365, 2.306, 2.262, 2.228, 2.201, 2.179,
    ];
    T.get(df.saturating_sub(1)).copied().unwrap_or(1.96)
}

/// Mean and half-width of the 95% t interval (NaN for a single sample).
fn ci(xs: &[f64]) -> (f64, f64) {
    let m = mean(xs);
    if xs.len() < 2 {
        return (m, f64::NAN);
    }
    (m, t95(xs.len() - 1) * sd(xs) / (xs.len() as f64).sqrt())
}

fn fx(x: f64, digits: usize) -> String {
    if x.is_nan() {
        "n/a".into()
    } else {
        format!("{x:.digits$}")
    }
}

fn sg(x: f64, digits: usize) -> String {
    if x.is_nan() {
        "n/a".into()
    } else {
        format!("{x:+.digits$}")
    }
}

fn pm(xs: &[f64], digits: usize) -> String {
    let (m, h) = ci(xs);
    format!("{} +/- {}", sg(m, digits), fx(h, digits))
}

/// `k+ / k0 / k-`: in how many pairs the first arm exceeded, equalled or
/// fell short of the second.
fn signs(xs: &[f64]) -> String {
    let up = xs.iter().filter(|x| **x > 0.0).count();
    let zero = xs.iter().filter(|x| **x == 0.0).count();
    format!("{up}+/{zero}0/{}-", xs.len() - up - zero)
}

fn summarize(args: &[String]) -> Result<(), String> {
    let dir = PathBuf::from(need(args, "--in")?);
    let out = PathBuf::from(need(args, "--out")?);
    let mut files: Vec<_> = fs::read_dir(&dir)
        .map_err(|e| format!("read {}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s.starts_with("cell_") && s.ends_with(".json"))
        })
        .collect();
    files.sort();
    let mut cells = Vec::new();
    for path in &files {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let value: Value =
            serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        cells.push(Cell::from_value(&value, name).map_err(|e| format!("{name}: {e}"))?);
    }
    if cells.is_empty() {
        return Err("no cell receipts found".into());
    }
    // Fold repeats.
    type Key = (String, u32, u64, String, u64);
    let mut groups: BTreeMap<Key, Vec<Cell>> = BTreeMap::new();
    for c in &cells {
        groups
            .entry((c.series.clone(), c.n, c.trials, c.arm.clone(), c.seed))
            .or_default()
            .push(c.clone());
    }
    let mut aggs: BTreeMap<Key, Agg> = BTreeMap::new();
    for (key, mut reps) in groups {
        reps.sort_by_key(|c| c.rep);
        let first = reps[0].clone();
        let agree = reps.iter().all(|c| c.counters() == first.counters());
        let cpu = mean(&reps.iter().map(|c| c.online_cpu).collect::<Vec<_>>());
        aggs.insert(
            key,
            Agg {
                cell: first,
                reps: reps.len(),
                online_cpu: cpu,
                counters_agree: agree,
            },
        );
    }
    let get = |series: &str, n: u32, trials: u64, arm: &str, seed: u64| {
        aggs.get(&(series.to_string(), n, trials, arm.to_string(), seed))
    };
    let mut series_n: Vec<(String, u32, u64)> =
        aggs.keys().map(|k| (k.0.clone(), k.1, k.2)).collect();
    series_n.sort();
    series_n.dedup();
    let seeds_of = |series: &str, n: u32, trials: u64| {
        let mut v: Vec<u64> = aggs
            .keys()
            .filter(|k| k.0 == series && k.1 == n && k.2 == trials)
            .map(|k| k.4)
            .collect();
        v.sort_unstable();
        v.dedup();
        v
    };

    let mut md = String::new();
    // ---- Table B: arm means per (series, n, cap).
    writeln!(md, "### Table B. Arm means over seeds\n").ok();
    writeln!(md, "`complete` = rows emitted by the filter; `indep` = newly independent verified rows; `rank` out of S+1 columns; online CPU = collect+filter+replay, mean of the repeats; CPU per indep = mean over seeds with at least one independent row.\n").ok();
    writeln!(md, "| series | n | cap | arm | seeds | reps | partials f/s/d | complete | indep | rank (of S+1) | online CPU ms | CPU ms per indep | complete per attempt |").ok();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|---|---|").ok();
    for (series, n, trials) in &series_n {
        for arm in ["none", "dlp", "dlp_orbit", "dlp_orbit_absorb"] {
            let xs: Vec<&Agg> = seeds_of(series, *n, *trials)
                .into_iter()
                .filter_map(|s| get(series, *n, *trials, arm, s))
                .collect();
            if xs.is_empty() {
                continue;
            }
            let m = |g: &dyn Fn(&Agg) -> f64| mean(&xs.iter().map(|a| g(a)).collect::<Vec<_>>());
            let per: Vec<f64> = xs.iter().filter_map(|a| a.cpu_per_verified()).collect();
            writeln!(
                md,
                "| {} | {} | {} | {} | {} | {} | {:.1}/{:.1}/{:.1} | {} | {} | {} ({}) | {} | {} | {} |",
                series, n, trials, arm, xs.len(), xs[0].reps,
                m(&|a| a.cell.partial[0] as f64), m(&|a| a.cell.partial[1] as f64), m(&|a| a.cell.partial[2] as f64),
                fx(m(&|a| a.cell.complete as f64), 1), fx(m(&|a| a.cell.independent as f64), 1),
                fx(m(&|a| a.cell.rank as f64), 1), xs[0].cell.small_columns + 1,
                fx(m(&|a| a.online_cpu) * 1e3, 2),
                if per.is_empty() { "n/a".into() } else { format!("{} (n={})", fx(mean(&per) * 1e3, 3), per.len()) },
                fx(m(&|a| a.complete_per_attempt()), 4)
            )
            .ok();
        }
    }
    // ---- Table C: structure of the large columns.
    writeln!(
        md,
        "\n### Table C. What the quotient did to the large columns\n"
    )
    .ok();
    writeln!(md, "Random model: two distinct large points lie in one signed Frobenius orbit with probability about (2n-2)/(r-1), so E[colliding pairs] = C(L,2)(2n-2)/(r-1) for L large points. `merged` counts large points that share an orbit vertex with another large point (L minus orbits, absorbed excluded).\n").ok();
    writeln!(md, "| n | curve | r | cofactor | L (large points) | orbits | merged | absorbed (absorb arm) | E[pairs] random model | L/r |").ok();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|").ok();
    let mut seen_n: Vec<u32> = cells.iter().map(|c| c.n).collect();
    seen_n.sort_unstable();
    seen_n.dedup();
    for &n in &seen_n {
        if let Some(c) = cells
            .iter()
            .find(|c| c.n == n && c.arm == "dlp_orbit" && c.series == "primary")
        {
            let l = c.large_points.unwrap_or(0) as f64;
            let expected = l * (l - 1.0) / 2.0 * (2.0 * f64::from(n) - 2.0) / (c.r as f64 - 1.0);
            let absorbed = cells
                .iter()
                .find(|x| x.n == n && x.arm == "dlp_orbit_absorb" && x.series == "primary")
                .and_then(|x| x.absorbed);
            writeln!(
                md,
                "| {} | K_{} | {} | {} | {} | {} | {} | {} | {} | {:.4} |",
                n,
                c.a,
                c.r,
                c.cofactor,
                c.large_points.unwrap_or(0),
                c.large_orbits.unwrap_or(0),
                c.merged.unwrap_or(0),
                absorbed.map_or("-".into(), |v| v.to_string()),
                fx(expected, 1),
                l / c.r as f64
            )
            .ok();
        }
    }
    // ---- Tables D/E: paired differences against the plain arm, per series and cap.
    let mut agg_md = String::new();
    writeln!(md, "\n### Table D. Paired per-seed differences against the plain double large prime arm (ii)\n").ok();
    writeln!(md, "Counters are deterministic given the seed (the repeats agree exactly, see Integrity), so a nonzero counter difference is a property of the arm, not of timing. CPU differences are percent of the plain arm's online CPU.\n").ok();
    writeln!(md, "| series | n | cap | treatment | seed | d complete | d indep | d rank | d online CPU % | d CPU per indep % |").ok();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|").ok();
    writeln!(
        agg_md,
        "\n### Table E. Aggregate of the paired differences\n"
    )
    .ok();
    writeln!(agg_md, "Mean +/- 95% Student-t half-width over S seeds; signs = in how many seeds the treatment exceeded / equalled / fell below plain. Timing control = mean |relative change| of online CPU between repeats of the same cell (plain and orbit arms pooled). A CPU difference is `within noise` if |mean| is at most its t half-width or at most the timing control, else `outside noise`. Treatment `dlp_orbit` is arm (iii); `dlp_orbit_absorb` is the secondary arm (iv).\n").ok();
    writeln!(agg_md, "| series | n | cap | treatment | S | d complete | signs | d indep | signs | d rank | d CPU per indep % | verdict (CPU per indep) | d online CPU % | verdict (online CPU) | timing control % |").ok();
    writeln!(
        agg_md,
        "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"
    )
    .ok();
    let mut chart_rows: Vec<(u32, f64, f64, f64, f64, f64)> = Vec::new();
    let verdict = |m: f64, half: f64, control: f64| -> &'static str {
        if m.is_nan() {
            "n/a"
        } else if half.is_nan() {
            "S=1"
        } else if m.abs() <= half || (!control.is_nan() && m.abs() <= control) {
            "within noise"
        } else {
            "outside noise"
        }
    };
    for (series, n, trials) in &series_n {
        // Timing control: repeats of the same cell (plain and orbit arms).
        let mut control = Vec::new();
        for s in seeds_of(series, *n, *trials) {
            for arm in ["dlp", "dlp_orbit"] {
                let reps: Vec<&Cell> = cells
                    .iter()
                    .filter(|c| {
                        c.series == *series
                            && c.n == *n
                            && c.trials == *trials
                            && c.arm == arm
                            && c.seed == s
                    })
                    .collect();
                if let Some(first) = reps.iter().find(|c| c.rep == 1).copied() {
                    for other in reps.iter().filter(|c| c.rep != 1) {
                        control.push(
                            100.0
                                * ((other.online_cpu - first.online_cpu) / first.online_cpu).abs(),
                        );
                    }
                }
            }
        }
        let control_mean = if control.is_empty() {
            f64::NAN
        } else {
            mean(&control)
        };
        for treat in ["dlp_orbit", "dlp_orbit_absorb"] {
            let (mut dc, mut di, mut dr, mut dcpu, mut dper) =
                (vec![], vec![], vec![], vec![], vec![]);
            for s in seeds_of(series, *n, *trials) {
                let (Some(base), Some(orb)) = (
                    get(series, *n, *trials, "dlp", s),
                    get(series, *n, *trials, treat, s),
                ) else {
                    continue;
                };
                let d_complete = orb.cell.complete as f64 - base.cell.complete as f64;
                let d_indep = orb.cell.independent as f64 - base.cell.independent as f64;
                let d_rank = orb.cell.rank as f64 - base.cell.rank as f64;
                let d_cpu = 100.0 * (orb.online_cpu - base.online_cpu) / base.online_cpu;
                let d_per = match (orb.cpu_per_verified(), base.cpu_per_verified()) {
                    (Some(o), Some(b)) => 100.0 * (o - b) / b,
                    _ => f64::NAN,
                };
                writeln!(
                    md,
                    "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                    series,
                    n,
                    trials,
                    treat,
                    s,
                    sg(d_complete, 0),
                    sg(d_indep, 0),
                    sg(d_rank, 0),
                    sg(d_cpu, 2),
                    sg(d_per, 2)
                )
                .ok();
                dc.push(d_complete);
                di.push(d_indep);
                dr.push(d_rank);
                dcpu.push(d_cpu);
                if !d_per.is_nan() {
                    dper.push(d_per);
                }
            }
            if dc.is_empty() {
                continue;
            }
            let (m_cpu, h_cpu) = ci(&dcpu);
            let (m_per, h_per) = if dper.is_empty() {
                (f64::NAN, f64::NAN)
            } else {
                ci(&dper)
            };
            writeln!(agg_md, "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} +/- {} (n={}) | {} | {} +/- {} | {} | {} |",
                series, n, trials, treat, dc.len(), pm(&dc, 1), signs(&dc), pm(&di, 1), signs(&di), pm(&dr, 1),
                sg(m_per, 1), fx(h_per, 1), dper.len(), verdict(m_per, h_per, control_mean),
                sg(m_cpu, 1), fx(h_cpu, 1), verdict(m_cpu, h_cpu, control_mean), fx(control_mean, 1)).ok();
            if series == "primary" && treat == "dlp_orbit" {
                chart_rows.push((
                    *n,
                    m_cpu,
                    if h_cpu.is_nan() { 0.0 } else { h_cpu },
                    if control_mean.is_nan() {
                        0.0
                    } else {
                        control_mean
                    },
                    mean(&di),
                    ci(&di).1,
                ));
            }
        }
    }
    md.push_str(&agg_md);
    // ---- Table F: the other two arms against plain, primary series.
    writeln!(md, "\n### Table F. No-large-prime arm (i) and absorb arm against the plain double large prime arm (ii), primary series\n").ok();
    writeln!(md, "| n | cap | S | indep: none | indep: dlp | indep: orbit | indep: orbit+absorb | CPU ms per indep: none | dlp | orbit | orbit+absorb |").ok();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|").ok();
    for (series, n, trials) in series_n.iter().filter(|x| x.0 == "primary") {
        let seeds = seeds_of(series, *n, *trials);
        let col = |arm: &str| -> (String, String) {
            let xs: Vec<&Agg> = seeds
                .iter()
                .filter_map(|s| get(series, *n, *trials, arm, *s))
                .collect();
            if xs.is_empty() {
                return ("-".into(), "-".into());
            }
            let ind = mean(
                &xs.iter()
                    .map(|a| a.cell.independent as f64)
                    .collect::<Vec<_>>(),
            );
            let per: Vec<f64> = xs.iter().filter_map(|a| a.cpu_per_verified()).collect();
            (
                fx(ind, 1),
                if per.is_empty() {
                    "n/a".into()
                } else {
                    fx(mean(&per) * 1e3, 3)
                },
            )
        };
        let (a, b, c, d) = (
            col("none"),
            col("dlp"),
            col("dlp_orbit"),
            col("dlp_orbit_absorb"),
        );
        writeln!(
            md,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            n,
            trials,
            seeds.len(),
            a.0,
            b.0,
            c.0,
            d.0,
            a.1,
            b.1,
            c.1,
            d.1
        )
        .ok();
    }
    // ---- Integrity.
    let total_fail: u64 = cells
        .iter()
        .map(|c| c.failed_linear + c.failed_curve + c.audit_failed)
        .sum();
    let total_verified: u64 = cells.iter().map(|c| c.verified).sum();
    let bad_recovery = cells
        .iter()
        .filter(|c| c.recovered_ok == Some(false))
        .count();
    let recovered = cells
        .iter()
        .filter(|c| c.recovered_ok == Some(true))
        .count();
    let disagreeing = aggs.values().filter(|a| !a.counters_agree).count();
    let max_orbit_cpu = cells.iter().map(|c| c.audit_cpu).fold(0.0f64, f64::max);
    writeln!(md, "\n### Integrity\n\n- receipts: {}; (series, n, cap, arm, seed) groups: {}; verified complete relations: {}.\n- replay or partial-audit failures (linear + curve + audit, all receipts): {}.\n- receipts whose recovered scalar equals the planted log: {}; disagreeing with it: {}.\n- groups whose repeats disagree on a deterministic counter: {}.\n- longest per-cell audit CPU (excluded from online CPU): {:.3} s; setup CPU is excluded from online CPU and is identical across arms.",
        cells.len(), aggs.len(), total_verified, total_fail, recovered, bad_recovery, disagreeing, max_orbit_cpu).ok();
    // ---- Table A: every receipt.
    let mut a_md = String::new();
    writeln!(a_md, "### Table A. Every receipt\n").ok();
    writeln!(
        a_md,
        "ok/fail = replays verified / failed; rank/(S+1); online CPU = collect+filter+replay.\n"
    )
    .ok();
    writeln!(a_md, "| series | n | arm | cap | seed | rep | partial f/s/d | complete | ok/fail | indep | rank/(S+1) | online CPU ms | setup CPU ms | audit CPU ms | states | RSS MB |").ok();
    writeln!(
        a_md,
        "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"
    )
    .ok();
    let mut sorted = cells.clone();
    sorted.sort_by(|a, b| {
        (&a.series, a.n, a.trials, &a.arm, a.seed, a.rep)
            .cmp(&(&b.series, b.n, b.trials, &b.arm, b.seed, b.rep))
    });
    for c in &sorted {
        writeln!(a_md, "| {} | {} | {} | {} | {} | {} | {}/{}/{} | {} | {}/{} | {} | {}/{} | {} | {} | {} | {} | {} |",
            c.series, c.n, c.arm, c.trials, c.seed, c.rep, c.partial[0], c.partial[1], c.partial[2], c.complete,
            c.verified, c.failed_linear.max(c.failed_curve) + c.audit_failed, c.independent, c.rank, c.small_columns + 1,
            fx(c.online_cpu * 1e3, 2), fx(c.setup_cpu * 1e3, 1), fx(c.audit_cpu * 1e3, 1), c.states, fx(c.rss_mb, 0)).ok();
    }
    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    fs::write(out.join("tables.md"), &md).map_err(|e| e.to_string())?;
    fs::write(out.join("tables_all_receipts.md"), &a_md).map_err(|e| e.to_string())?;
    fs::write(out.join("chart.svg"), chart_svg(&aggs, &chart_rows)).map_err(|e| e.to_string())?;
    let summary: BTreeMap<String, Value> = cells
        .iter()
        .map(|c| {
            (
                c.file.trim_end_matches(".json").to_string(),
                json!({
                    "series": c.series, "n": c.n, "arm": c.arm, "cap": c.trials, "seed": c.seed, "rep": c.rep,
                    "partials": c.partial, "complete": c.complete, "verified": c.verified,
                    "failed_linear": c.failed_linear, "failed_curve": c.failed_curve,
                    "audit_failed": c.audit_failed, "independent": c.independent, "rank": c.rank,
                    "small_columns_plus_one": c.small_columns + 1, "cycles_closed": c.cycles_closed,
                    "online_cpu_s": c.online_cpu, "setup_cpu_s": c.setup_cpu, "audit_cpu_s": c.audit_cpu,
                    "rss_mb": c.rss_mb, "states": c.states,
                }),
            )
        })
        .collect();
    fs::write(
        out.join("summary.json"),
        serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    eprintln!(
        "wrote tables.md, tables_all_receipts.md, chart.svg, summary.json in {}",
        out.display()
    );
    Ok(())
}

/// Hand-written SVG, primary series.  Left: paired difference in newly
/// independent verified relations, orbit minus plain, per n (mean over seeds
/// with 95% t half-width).  Right: paired difference in online CPU (percent)
/// with the repeat-to-repeat timing control as a band.
fn chart_svg(
    aggs: &BTreeMap<(String, u32, u64, String, u64), Agg>,
    rows: &[(u32, f64, f64, f64, f64, f64)],
) -> String {
    let (w, h) = (900.0, 440.0);
    let mut s = String::new();
    writeln!(s, r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" font-family="sans-serif" font-size="12">"##).ok();
    writeln!(s, r##"<rect width="{w}" height="{h}" fill="#ffffff"/>"##).ok();
    writeln!(s, r##"<text x="20" y="24" font-size="15" font-weight="bold">Orbit-aware vs plain double large prime (paired by seed), Koblitz K_a over F_(2^n)</text>"##).ok();
    let (y0, ph, pw) = (70.0, 280.0, 360.0);
    // Left panel: d independent relations.
    let x0 = 70.0;
    let lim = rows
        .iter()
        .map(|r| r.4.abs() + if r.5.is_nan() { 0.0 } else { r.5 })
        .fold(1.0f64, f64::max)
        * 1.2;
    let ymap = |v: f64| y0 + ph / 2.0 - v / lim * ph / 2.0;
    writeln!(s, r##"<text x="{x0}" y="56" font-weight="bold">d newly independent verified relations (mean, 95% interval)</text>"##).ok();
    writeln!(
        s,
        r##"<rect x="{x0}" y="{y0}" width="{pw}" height="{ph}" fill="none" stroke="#888"/>"##
    )
    .ok();
    writeln!(
        s,
        r##"<line x1="{x0}" y1="{}" x2="{}" y2="{}" stroke="#444"/>"##,
        ymap(0.0),
        x0 + pw,
        ymap(0.0)
    )
    .ok();
    writeln!(s, r##"<text x="{}" y="{}" text-anchor="end">0</text><text x="{}" y="{}" text-anchor="end">+{lim:.0}</text><text x="{}" y="{}" text-anchor="end">-{lim:.0}</text>"##, x0 - 6.0, ymap(0.0) + 4.0, x0 - 6.0, y0 + 10.0, x0 - 6.0, y0 + ph).ok();
    let step = pw / rows.len().max(1) as f64;
    for (i, (n, _, _, _, d, hw)) in rows.iter().enumerate() {
        let cx = x0 + step * (i as f64 + 0.5);
        let hw = if hw.is_nan() { 0.0 } else { *hw };
        writeln!(s, r##"<line x1="{cx}" y1="{}" x2="{cx}" y2="{}" stroke="#1f4e8c" stroke-width="2"/><circle cx="{cx}" cy="{}" r="4.5" fill="#1f4e8c"/><text x="{cx}" y="{}" text-anchor="middle">n={n}</text>"##, ymap(d + hw), ymap(d - hw), ymap(*d), y0 + ph + 16.0).ok();
    }
    // Right panel: d online CPU %.
    let rx0 = 500.0;
    let lim2 = rows
        .iter()
        .flat_map(|r| [r.1.abs() + r.2, r.3])
        .fold(1.0f64, f64::max)
        * 1.2;
    let ymap2 = |v: f64| y0 + ph / 2.0 - v / lim2 * ph / 2.0;
    writeln!(s, r##"<text x="{rx0}" y="56" font-weight="bold">d online CPU, percent (mean, 95% interval)</text>"##).ok();
    writeln!(
        s,
        r##"<rect x="{rx0}" y="{y0}" width="{pw}" height="{ph}" fill="none" stroke="#888"/>"##
    )
    .ok();
    writeln!(
        s,
        r##"<line x1="{rx0}" y1="{}" x2="{}" y2="{}" stroke="#444"/>"##,
        ymap2(0.0),
        rx0 + pw,
        ymap2(0.0)
    )
    .ok();
    writeln!(s, r##"<text x="{}" y="{}" text-anchor="end">0</text><text x="{}" y="{}" text-anchor="end">+{lim2:.0}</text><text x="{}" y="{}" text-anchor="end">-{lim2:.0}</text>"##, rx0 - 6.0, ymap2(0.0) + 4.0, rx0 - 6.0, y0 + 10.0, rx0 - 6.0, y0 + ph).ok();
    for (i, (n, m, hw, control, _, _)) in rows.iter().enumerate() {
        let cx = rx0 + step * (i as f64 + 0.5);
        writeln!(s, r##"<rect x="{}" y="{}" width="28" height="{}" fill="#c9d6e8"/><line x1="{cx}" y1="{}" x2="{cx}" y2="{}" stroke="#b5541c" stroke-width="2"/><circle cx="{cx}" cy="{}" r="4.5" fill="#b5541c"/><text x="{cx}" y="{}" text-anchor="middle">n={n}</text>"##, cx - 14.0, ymap2(*control), (ymap2(-*control) - ymap2(*control)).max(0.0), ymap2(m + hw), ymap2(m - hw), ymap2(*m), y0 + ph + 16.0).ok();
    }
    writeln!(s, r##"<text x="{rx0}" y="{}" fill="#456">blue band: +/- mean repeat-to-repeat CPU change (timing control)</text>"##, y0 + ph + 36.0).ok();
    let seeds = aggs
        .keys()
        .filter(|k| k.0 == "primary")
        .map(|k| k.4)
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    writeln!(s, r##"<text x="20" y="{}" fill="#456">Source: results/summary.json (primary series, {seeds} seeds). Toy scale; no cryptographic-size or ECDLP speedup claim.</text>"##, h - 14.0).ok();
    writeln!(s, "</svg>").ok();
    s
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("cell") => cell(&args),
        Some("manifest") => manifest(&args),
        Some("summarize") => summarize(&args),
        _ => Err("usage: orbit_large_prime_bench cell|manifest|summarize ...".into()),
    };
    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(2);
    }
}
