//! Tables for the identical-stream re-measurement
//! (`research/orbit_aware_large_prime_20261011/PROTOCOL_ADDENDUM_20261026.md`).
//!
//! ```text
//! orbit_addendum_summary --in receipts_addendum --baseline receipts --out results_addendum
//! ```
//!
//! Reads the cell receipts written by `orbit_large_prime_bench cell` (series
//! `primary`, `trajectory`, `realisation`) and the merged run's receipts, and
//! writes `tables.md`, `summary.json` and `chart.svg`.  Every counter that a
//! conclusion rests on is a required field: a missing one is an error, never
//! a zero.  No Python is involved.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

const SCHEMA: &str = "orbit-lp-cell/v1";
const ARMS: [&str; 3] = ["dlp", "dlp_orbit", "dlp_orbit_post"];

fn label(arm: &str) -> &'static str {
    match arm {
        "dlp" => "(ii) plain",
        "dlp_orbit" => "(iii) orbit",
        "dlp_orbit_post" => "(iii-post) orbit after collection",
        _ => "?",
    }
}

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn at<'a>(v: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.').try_fold(v, |cur, key| cur.get(key))
}

/// A required unsigned integer field: missing or non-integer is an error.
fn req(v: &Value, path: &str) -> Result<u64, String> {
    at(v, path)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing or non-integer field `{path}`"))
}

fn req_f(v: &Value, path: &str) -> Result<f64, String> {
    req(v, path).map(|x| x as f64 / 1e9)
}

#[derive(Clone, Debug)]
struct Hit {
    target: String,
    rank: u64,
    trial: Option<u64>,
}

#[derive(Clone, Debug)]
struct Traj {
    monotone: bool,
    consistent: bool,
    hits: Vec<Hit>,
    /// (cutoff trial, complete, independent, rank, pinned)
    cutoffs: Vec<(u64, u64, u64, u64, bool)>,
}

#[derive(Clone, Debug)]
struct Rec {
    series: String,
    n: u32,
    arm: String,
    seed: u64,
    rep: u64,
    cap: u64,
    modulus_seed: Option<u64>,
    trials: u64,
    decompositions: u64,
    accepted: u64,
    histogram: [u64; 4],
    partial: [u64; 3],
    complete: u64,
    verified: u64,
    replays_failed: u64,
    failed_linear: u64,
    failed_curve: u64,
    audit_failed: u64,
    independent: u64,
    rank: u64,
    columns: u64,
    digest: Option<String>,
    collect: f64,
    filter: f64,
    replay: f64,
    setup: f64,
    audit: f64,
    large_points: Option<u64>,
    large_orbits: Option<u64>,
    merged: Option<u64>,
    orbit_class: Option<[u64; 3]>,
    class_changed: Option<u64>,
    recovered_ok: Option<bool>,
    traj: Option<Traj>,
}

impl Rec {
    fn parse(v: &Value) -> Result<Self, String> {
        if v["schema"] != SCHEMA {
            return Err("not an orbit-lp cell receipt".into());
        }
        let arr3 = |path: &str| -> Result<[u64; 3], String> {
            let a = at(v, path)
                .and_then(Value::as_array)
                .ok_or(format!("missing `{path}`"))?;
            let get = |i: usize| {
                a.get(i)
                    .and_then(Value::as_u64)
                    .ok_or(format!("bad `{path}`[{i}]"))
            };
            Ok([get(0)?, get(1)?, get(2)?])
        };
        let hist = {
            let a = at(v, "report.counters.relation_histogram")
                .and_then(Value::as_array)
                .ok_or("missing relation_histogram")?;
            let g = |i: usize| {
                a.get(i)
                    .and_then(Value::as_u64)
                    .ok_or("bad relation_histogram")
            };
            [g(0)?, g(1)?, g(2)?, g(3)?]
        };
        let orbit = at(v, "report.factor_base.orbit").filter(|o| !o.is_null());
        let orbit_field =
            |name: &str| -> Result<Option<u64>, String> { orbit.map(|o| req(o, name)).transpose() };
        let post = at(v, "report.filter_pipeline.post_collect").filter(|o| !o.is_null());
        let traj = match at(v, "report.filter_pipeline.trajectory").filter(|t| !t.is_null()) {
            None => None,
            Some(t) => {
                let hits = t["first_hits"]
                    .as_array()
                    .ok_or("trajectory without first_hits")?
                    .iter()
                    .map(|h| -> Result<Hit, String> {
                        Ok(Hit {
                            target: h["target"].as_str().ok_or("hit target")?.to_string(),
                            rank: req(h, "rank")?,
                            trial: h.get("trial").and_then(Value::as_u64),
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                let cutoffs = t["at_cutoffs"]
                    .as_array()
                    .ok_or("trajectory without at_cutoffs")?
                    .iter()
                    .map(|c| -> Result<(u64, u64, u64, u64, bool), String> {
                        Ok((
                            req(c, "cutoff_trial")?,
                            req(c, "complete")?,
                            req(c, "independent")?,
                            req(c, "rank")?,
                            c["pinned"].as_bool().ok_or("pinned")?,
                        ))
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                Some(Traj {
                    monotone: t["monotone"].as_bool().ok_or("monotone")?,
                    consistent: t["consistent_with_pipeline"]
                        .as_bool()
                        .ok_or("consistent")?,
                    hits,
                    cutoffs,
                })
            }
        };
        Ok(Self {
            series: v["series"].as_str().ok_or("series")?.to_string(),
            n: req(v, "n")? as u32,
            arm: v["arm"].as_str().ok_or("arm")?.to_string(),
            seed: req(v, "seed")?,
            rep: req(v, "rep")?,
            cap: req(v, "report.config.max_trials")?,
            modulus_seed: at(v, "curve.modulus_seed").and_then(Value::as_u64),
            trials: req(v, "report.counters.trials")?,
            decompositions: req(v, "report.counters.decompositions")?,
            accepted: req(v, "report.counters.accepted_relations")?,
            histogram: hist,
            partial: arr3("report.filter_pipeline.partials_by_class")?,
            complete: req(v, "report.filter_pipeline.complete_relations")?,
            verified: req(v, "report.filter_pipeline.replays_verified")?,
            replays_failed: req(v, "report.filter_pipeline.replays_failed")?,
            failed_linear: req(v, "report.filter_pipeline.replay_failed_linear")?,
            failed_curve: req(v, "report.filter_pipeline.replay_failed_curve")?,
            audit_failed: req(v, "report.filter_pipeline.partial_audit_failed")?,
            independent: req(v, "report.filter_pipeline.independent_verified")?,
            rank: req(v, "report.filter_pipeline.rank")?,
            columns: req(v, "report.factor_base.small_columns")? + 1,
            digest: at(v, "report.filter_pipeline.stream.digest")
                .and_then(Value::as_str)
                .map(str::to_string),
            collect: req_f(v, "report.filter_pipeline.cpu_ns.collect")?,
            filter: req_f(v, "report.filter_pipeline.cpu_ns.filter")?,
            replay: req_f(v, "report.filter_pipeline.cpu_ns.replay")?,
            setup: req_f(v, "report.filter_pipeline.cpu_ns.setup")?,
            audit: req_f(v, "report.filter_pipeline.cpu_ns.audit")?,
            large_points: orbit_field("large_points")?,
            large_orbits: orbit_field("large_orbits")?,
            merged: orbit_field("merged_points")?,
            orbit_class: post
                .map(|p| {
                    let a = p["partials_by_orbit_class"]
                        .as_array()
                        .ok_or("orbit class")?;
                    let g = |i: usize| a.get(i).and_then(Value::as_u64).ok_or("orbit class");
                    Ok::<_, String>([g(0)?, g(1)?, g(2)?])
                })
                .transpose()?,
            class_changed: post.map(|p| req(p, "class_changed")).transpose()?,
            recovered_ok: v["recovered_matches_planted"].as_bool(),
            traj,
        })
    }

    /// The deterministic counters a reproduction must match.
    fn fingerprint(
        &self,
    ) -> (
        u64,
        [u64; 3],
        u64,
        u64,
        u64,
        u64,
        Option<u64>,
        Option<u64>,
        Option<u64>,
    ) {
        (
            self.trials,
            self.partial,
            self.complete,
            self.verified,
            self.independent,
            self.rank,
            self.large_points,
            self.large_orbits,
            self.merged,
        )
    }
}

fn read_dir(dir: &Path) -> Result<Vec<Rec>, String> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|e| format!("read {}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s.starts_with("cell_") && s.ends_with(".json"))
        })
        .collect();
    files.sort();
    let mut out = Vec::new();
    for path in files {
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        let text = fs::read_to_string(&path).map_err(|e| format!("{name}: {e}"))?;
        let value: Value = serde_json::from_str(&text).map_err(|e| format!("{name}: {e}"))?;
        // The merged run's `none` and `absorb` receipts parse too; arms we do
        // not tabulate are simply never selected.
        out.push(Rec::parse(&value).map_err(|e| format!("{name}: {e}"))?);
    }
    if out.is_empty() {
        return Err(format!("no cell receipts in {}", dir.display()));
    }
    Ok(out)
}

// ----------------------------------------------------------------- statistics

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

/// Two-sided 95% Student-t critical value, `df` degrees of freedom.
fn t95(df: usize) -> f64 {
    const T: [f64; 6] = [12.706, 4.303, 3.182, 2.776, 2.571, 2.447];
    T.get(df.saturating_sub(1)).copied().unwrap_or(1.96)
}

/// Mean and 95% half-width (NaN for one sample).
fn ci(xs: &[f64]) -> (f64, f64) {
    if xs.len() < 2 {
        return (mean(xs), f64::NAN);
    }
    (
        mean(xs),
        t95(xs.len() - 1) * sd(xs) / (xs.len() as f64).sqrt(),
    )
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

fn signs(xs: &[f64]) -> String {
    let up = xs.iter().filter(|x| **x > 0.0).count();
    let zero = xs.iter().filter(|x| **x == 0.0).count();
    format!("{up}+/{zero}0/{}-", xs.len() - up - zero)
}

/// The verdict rule of the protocol (section 4): `survives` needs every seed
/// non-negative and the whole interval above zero; `reversed` the mirror
/// image; otherwise `no gain`.
fn verdict(xs: &[f64]) -> &'static str {
    let (m, h) = ci(xs);
    if h.is_nan() {
        return "n/a";
    }
    if m - h > 0.0 && xs.iter().all(|x| *x >= 0.0) {
        "survives"
    } else if m + h < 0.0 {
        "reversed"
    } else {
        "no gain"
    }
}

// ------------------------------------------------------------------ the report

type Key = (String, u32, String, u64, u64);

struct Tables {
    recs: Vec<Rec>,
}

impl Tables {
    fn get(&self, series: &str, n: u32, arm: &str, seed: u64, rep: u64) -> Option<&Rec> {
        self.recs.iter().find(|r| {
            r.series == series && r.n == n && r.arm == arm && r.seed == seed && r.rep == rep
        })
    }

    fn seeds(&self, series: &str, n: u32) -> Vec<u64> {
        let mut s: Vec<u64> = self
            .recs
            .iter()
            .filter(|r| r.series == series && r.n == n)
            .map(|r| r.seed)
            .collect();
        s.sort_unstable();
        s.dedup();
        s
    }

    fn ns(&self, series: &str) -> Vec<u32> {
        let mut s: Vec<u32> = self
            .recs
            .iter()
            .filter(|r| r.series == series)
            .map(|r| r.n)
            .collect();
        s.sort_unstable();
        s.dedup();
        s
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = run(&args) {
        eprintln!("error: {e}");
        std::process::exit(2);
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let input = PathBuf::from(arg(args, "--in").ok_or("missing --in")?);
    let baseline = arg(args, "--baseline").map(PathBuf::from);
    let out = PathBuf::from(arg(args, "--out").ok_or("missing --out")?);
    let tables = Tables {
        recs: read_dir(&input)?,
    };
    let merged_run: Vec<Rec> = match &baseline {
        Some(dir) => read_dir(dir)?,
        None => Vec::new(),
    };
    let mut md = String::new();
    let mut summary = serde_json::Map::new();

    // ---------------------------------------------------------- 1. identity
    writeln!(
        md,
        "### Table 1. Identity and reproduction checks (primary series)\n"
    )
    .ok();
    let mut invalid = Vec::new();
    let mut orbit_differs = Vec::new();
    let mut pairs = 0;
    for n in tables.ns("primary") {
        for seed in tables.seeds("primary", n) {
            let (Some(ii), Some(iii), Some(post)) = (
                tables.get("primary", n, "dlp", seed, 1),
                tables.get("primary", n, "dlp_orbit", seed, 1),
                tables.get("primary", n, "dlp_orbit_post", seed, 1),
            ) else {
                continue;
            };
            pairs += 1;
            let same = ii.digest.is_some()
                && ii.digest == post.digest
                && ii.accepted == post.accepted
                && ii.trials == post.trials
                && ii.decompositions == post.decompositions
                && ii.histogram == post.histogram
                && ii.partial == post.partial;
            if !same {
                invalid.push(format!("n={n} seed={seed}"));
            }
            if iii.digest != ii.digest {
                orbit_differs.push((n, seed, ii.accepted, iii.accepted, ii.partial, iii.partial));
            }
        }
    }
    writeln!(
        md,
        "* (ii) and (iii-post) accepted decomposition streams (BLAKE3 over trial, a, b, atom list, plus trials, decompositions, accepted, histogram, partial classes): identical in {} of {pairs} `(n, seed)` pairs; invalid identity pairs: {}.",
        pairs - invalid.len(),
        if invalid.is_empty() { "none".into() } else { invalid.join(", ") }
    )
    .ok();
    writeln!(
        md,
        "* (ii) and (iii) streams differ in {} of {pairs} pairs (their class histograms differ in a subset of those):\n",
        orbit_differs.len()
    )
    .ok();
    if !orbit_differs.is_empty() {
        writeln!(md, "| n | seed | accepted (ii) | accepted (iii) | partials f/s/d (ii) | partials f/s/d (iii) |\n|---|---|---|---|---|---|").ok();
        for (n, seed, a, b, pa, pb) in &orbit_differs {
            writeln!(
                md,
                "| {n} | {seed} | {a} | {b} | {}/{}/{} | {}/{}/{} |",
                pa[0], pa[1], pa[2], pb[0], pb[1], pb[2]
            )
            .ok();
        }
        writeln!(md).ok();
    }
    summary.insert("identity_pairs".into(), json!(pairs));
    summary.insert("invalid_identity_pairs".into(), json!(invalid));
    summary.insert(
        "stream_differs_ii_vs_iii".into(),
        json!(orbit_differs.len()),
    );

    // Reproduction against the merged receipts.
    let mut repro_total = 0;
    let mut repro_equal = 0;
    let mut repro_bad = Vec::new();
    for rec in tables
        .recs
        .iter()
        .filter(|r| r.series == "primary" && r.rep == 1 && r.arm != "dlp_orbit_post")
    {
        let merged = merged_run.iter().find(|m| {
            m.series == "primary"
                && m.n == rec.n
                && m.arm == rec.arm
                && m.seed == rec.seed
                && m.rep == 1
                && m.cap == rec.cap
        });
        let Some(merged) = merged else { continue };
        repro_total += 1;
        if merged.fingerprint() == rec.fingerprint() {
            repro_equal += 1;
        } else {
            repro_bad.push(format!("{} n={} seed={}", rec.arm, rec.n, rec.seed));
        }
    }
    if baseline.is_some() {
        writeln!(
            md,
            "* Reproduction of the merged receipts (arms (ii) and (iii), repeat 1; trials, partial classes, complete, verified, independent, rank, large points, orbits, merged): {repro_equal} of {repro_total} cells equal{}.\n",
            if repro_bad.is_empty() { String::new() } else { format!("; DIFFERENT: {}", repro_bad.join(", ")) }
        )
        .ok();
    }
    summary.insert(
        "reproduction".into(),
        json!({"cells": repro_total, "equal": repro_equal, "different": repro_bad}),
    );

    // Repeats agree.
    let mut by_cell: BTreeMap<Key, Vec<&Rec>> = BTreeMap::new();
    for r in &tables.recs {
        by_cell
            .entry((r.series.clone(), r.n, r.arm.clone(), r.seed, r.cap))
            .or_default()
            .push(r);
    }
    let repeat_groups = by_cell.values().filter(|v| v.len() > 1).count();
    let repeat_disagree = by_cell
        .values()
        .filter(|v| {
            v.len() > 1
                && v.iter()
                    .any(|r| r.fingerprint() != v[0].fingerprint() || r.digest != v[0].digest)
        })
        .count();
    writeln!(md, "* Cells with repeats: {repeat_groups}; repeats disagreeing on a deterministic counter or on the stream digest: {repeat_disagree}.\n").ok();
    summary.insert("repeat_groups".into(), json!(repeat_groups));
    summary.insert("repeat_disagree".into(), json!(repeat_disagree));

    // ----------------------------------------------------------- 2. arm means
    writeln!(
        md,
        "### Table 2. Arm means over seeds (primary series, repeat 1)\n"
    )
    .ok();
    writeln!(md, "`partials` are the plain classes full/single/double (equal for (ii) and (iii-post) by construction). `orbit classes` is the class after the orbit map ((iii-post) only). CPU is process CPU of the repeats, in ms (diagnostic; shared VM).\n").ok();
    writeln!(md, "| n | cap | arm | seeds | partials f/s/d | orbit classes f/s/d (changed) | complete | indep | rank (of S+1) | collect ms | filter ms | replay ms | setup ms | audit ms |").ok();
    writeln!(
        md,
        "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"
    )
    .ok();
    for n in tables.ns("primary") {
        for arm in ARMS {
            let xs: Vec<&Rec> = tables
                .seeds("primary", n)
                .into_iter()
                .filter_map(|s| tables.get("primary", n, arm, s, 1))
                .collect();
            if xs.is_empty() {
                continue;
            }
            let m = |f: &dyn Fn(&Rec) -> f64| mean(&xs.iter().map(|r| f(r)).collect::<Vec<_>>());
            let all_reps: Vec<&Rec> = tables
                .recs
                .iter()
                .filter(|r| r.series == "primary" && r.n == n && r.arm == arm)
                .collect();
            let mr =
                |f: &dyn Fn(&Rec) -> f64| mean(&all_reps.iter().map(|r| f(r)).collect::<Vec<_>>());
            let orbit_classes = if arm == "dlp_orbit_post" {
                let g = |i: usize| m(&|r: &Rec| r.orbit_class.map_or(0.0, |c| c[i] as f64));
                format!(
                    "{:.1}/{:.1}/{:.1} ({:.1})",
                    g(0),
                    g(1),
                    g(2),
                    m(&|r: &Rec| r.class_changed.map_or(0.0, |c| c as f64))
                )
            } else {
                "-".into()
            };
            writeln!(
                md,
                "| {n} | {} | {} | {} | {:.1}/{:.1}/{:.1} | {orbit_classes} | {} | {} | {} ({}) | {} | {} | {} | {} | {} |",
                xs[0].cap,
                label(arm),
                xs.len(),
                m(&|r| r.partial[0] as f64),
                m(&|r| r.partial[1] as f64),
                m(&|r| r.partial[2] as f64),
                fx(m(&|r| r.complete as f64), 1),
                fx(m(&|r| r.independent as f64), 1),
                fx(m(&|r| r.rank as f64), 1),
                xs[0].columns,
                fx(mr(&|r| r.collect) * 1e3, 1),
                fx(mr(&|r| r.filter) * 1e3, 2),
                fx(mr(&|r| r.replay) * 1e3, 2),
                fx(mr(&|r| r.setup) * 1e3, 0),
                fx(mr(&|r| r.audit) * 1e3, 1),
            )
            .ok();
        }
    }

    // ------------------------------------------------- 3. paired differences
    writeln!(
        md,
        "\n### Table 3. Paired differences by seed (primary series)\n"
    )
    .ok();
    writeln!(md, "`D_post` = (iii-post) - (ii): the quotient on the identical stream. `D_orig` = (iii) - (ii): the merged run's contrast. `D_acc` = (iii) - (iii-post) = `D_orig - D_post`: the part due to (iii) accepting more decompositions. Mean +/- 95% t half-width (df = S-1); signs = seeds above / equal / below zero. Verdict (protocol section 4) is for `D_post` on independent relations: `survives` = every seed >= 0 and the interval strictly above 0.\n").ok();
    writeln!(md, "| n | cap | S | metric | D_post | signs | D_orig | signs | D_acc | signs | verdict D_post |").ok();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|---|---|").ok();
    let mut contrasts = Vec::new();
    for n in tables.ns("primary") {
        let seeds = tables.seeds("primary", n);
        let trio: Vec<(&Rec, &Rec, &Rec)> = seeds
            .iter()
            .filter_map(|&s| {
                Some((
                    tables.get("primary", n, "dlp", s, 1)?,
                    tables.get("primary", n, "dlp_orbit", s, 1)?,
                    tables.get("primary", n, "dlp_orbit_post", s, 1)?,
                ))
            })
            .collect();
        if trio.is_empty() {
            continue;
        }
        for (metric, f) in [
            ("indep", (|r: &Rec| r.independent as f64) as fn(&Rec) -> f64),
            ("complete", |r: &Rec| r.complete as f64),
        ] {
            let d_post: Vec<f64> = trio.iter().map(|(a, _, c)| f(c) - f(a)).collect();
            let d_orig: Vec<f64> = trio.iter().map(|(a, b, _)| f(b) - f(a)).collect();
            let d_acc: Vec<f64> = trio.iter().map(|(_, b, c)| f(b) - f(c)).collect();
            writeln!(
                md,
                "| {n} | {} | {} | {metric} | {} | {} | {} | {} | {} | {} | {} |",
                trio[0].0.cap,
                trio.len(),
                pm(&d_post, 1),
                signs(&d_post),
                pm(&d_orig, 1),
                signs(&d_orig),
                pm(&d_acc, 1),
                signs(&d_acc),
                if metric == "indep" {
                    verdict(&d_post)
                } else {
                    "-"
                }
            )
            .ok();
            if metric == "indep" {
                contrasts.push(json!({
                    "n": n, "seeds": trio.len(),
                    "D_post": d_post, "D_orig": d_orig, "D_acc": d_acc,
                    "D_post_mean": mean(&d_post), "D_post_half": ci(&d_post).1,
                    "D_orig_mean": mean(&d_orig), "D_orig_half": ci(&d_orig).1,
                    "D_acc_mean": mean(&d_acc), "D_acc_half": ci(&d_acc).1,
                    "verdict_D_post": verdict(&d_post),
                }));
            }
        }
    }
    summary.insert("primary_contrasts".into(), Value::Array(contrasts.clone()));
    writeln!(
        md,
        "\nPer seed, independent relations at the cap (ii) / (iii) / (iii-post):\n"
    )
    .ok();
    writeln!(md, "| n | seed | (ii) | (iii) | (iii-post) | complete (ii) / (iii) / (iii-post) |\n|---|---|---|---|---|---|").ok();
    for n in tables.ns("primary") {
        for s in tables.seeds("primary", n) {
            if let (Some(a), Some(b), Some(c)) = (
                tables.get("primary", n, "dlp", s, 1),
                tables.get("primary", n, "dlp_orbit", s, 1),
                tables.get("primary", n, "dlp_orbit_post", s, 1),
            ) {
                writeln!(
                    md,
                    "| {n} | {s} | {} | {} | {} | {} / {} / {} |",
                    a.independent, b.independent, c.independent, a.complete, b.complete, c.complete
                )
                .ok();
            }
        }
    }

    // ------------------------------------------------------ 4. trajectory
    writeln!(
        md,
        "\n### Table 4. Cap-free metric: trials to reach a verified rank (trajectory series)\n"
    )
    .ok();
    writeln!(md, "First stored relation, in trials, whose prefix of the stream reaches the target (rank >= q, q = ceil(f (S+1)); `recovery` = the target column is pinned). `>max` = not reached within the run (censored; never imputed). Each cell lists the five seeds. The run length is 4x the primary cap (3x at n = 31).\n").ok();
    writeln!(md, "| n | run length | target | (ii) per seed | (iii) per seed | (iii-post) per seed | median (ii) / (iii) / (iii-post) | seeds (iii-post) earlier / equal / later than (ii) | mean paired (ii) - (iii-post) |").ok();
    writeln!(md, "|---|---|---|---|---|---|---|---|---|").ok();
    let mut traj_summary = Vec::new();
    let mut traj_flags = (0usize, 0usize, 0usize);
    for n in tables.ns("trajectory") {
        let seeds = tables.seeds("trajectory", n);
        let sample = tables.get("trajectory", n, "dlp", seeds[0], 1);
        let Some(sample) = sample else { continue };
        let max = sample.cap;
        let targets: Vec<(String, u64)> = sample
            .traj
            .as_ref()
            .map(|t| t.hits.iter().map(|h| (h.target.clone(), h.rank)).collect())
            .unwrap_or_default();
        for (target, _) in &targets {
            let col = |arm: &str| -> Vec<Option<u64>> {
                seeds
                    .iter()
                    .map(|&s| {
                        tables
                            .get("trajectory", n, arm, s, 1)
                            .and_then(|r| r.traj.as_ref())
                            .and_then(|t| t.hits.iter().find(|h| &h.target == target))
                            .and_then(|h| h.trial)
                    })
                    .collect()
            };
            let (ii, iii, post) = (col("dlp"), col("dlp_orbit"), col("dlp_orbit_post"));
            let show = |v: &[Option<u64>]| {
                v.iter()
                    .map(|t| t.map_or(format!(">{max}"), |x| x.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let med = |v: &[Option<u64>]| -> String {
                // Censored values sort last, so the median is finite only when more
                // than half of the seeds reached the target.
                let mut all: Vec<f64> = v
                    .iter()
                    .map(|t| t.map_or(f64::INFINITY, |x| x as f64))
                    .collect();
                all.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let m = if all.len() % 2 == 1 {
                    all[all.len() / 2]
                } else {
                    (all[all.len() / 2 - 1] + all[all.len() / 2]) / 2.0
                };
                if m.is_finite() {
                    format!("{m:.0}")
                } else {
                    format!(">{max}")
                }
            };
            let (mut earlier, mut equal, mut later) = (0, 0, 0);
            let mut diffs = Vec::new();
            for (a, b) in ii.iter().zip(&post) {
                match (a, b) {
                    (Some(a), Some(b)) => {
                        diffs.push(*a as f64 - *b as f64);
                        match b.cmp(a) {
                            std::cmp::Ordering::Less => earlier += 1,
                            std::cmp::Ordering::Equal => equal += 1,
                            std::cmp::Ordering::Greater => later += 1,
                        }
                    }
                    (None, Some(_)) => earlier += 1,
                    (Some(_), None) => later += 1,
                    (None, None) => equal += 1,
                }
            }
            let uncensored = ii.iter().chain(&post).all(Option::is_some);
            writeln!(
                md,
                "| {n} | {max} | {target} | {} | {} | {} | {} / {} / {} | {earlier} / {equal} / {later} | {} |",
                show(&ii), show(&iii), show(&post), med(&ii), med(&iii), med(&post),
                if uncensored { pm(&diffs, 0) } else { "censored".into() }
            )
            .ok();
            traj_summary.push(json!({
                "n": n, "run_length": max, "target": target,
                "ii": ii, "iii": iii, "iii_post": post,
                "post_earlier": earlier, "post_equal": equal, "post_later": later,
            }));
        }
        for &s in &seeds {
            for arm in ARMS {
                if let Some(t) = tables
                    .get("trajectory", n, arm, s, 1)
                    .and_then(|r| r.traj.as_ref())
                {
                    traj_flags.0 += 1;
                    traj_flags.1 += usize::from(t.monotone);
                    traj_flags.2 += usize::from(t.consistent);
                }
            }
        }
    }
    summary.insert("trajectory".into(), Value::Array(traj_summary));
    writeln!(md, "\nTrajectory cells: {}; rank monotone along the scanned prefixes: {}; full prefix reproduces the cell's own result: {}.\n", traj_flags.0, traj_flags.1, traj_flags.2).ok();
    summary.insert(
        "trajectory_cells".into(),
        json!({"cells": traj_flags.0, "monotone": traj_flags.1, "consistent": traj_flags.2}),
    );

    // Prefix-consistency with the primary cells and rank at cutoffs.
    writeln!(
        md,
        "#### Rank at trial cutoffs (mean over seeds of the verified rank; of S+1)\n"
    )
    .ok();
    writeln!(md, "| n | S+1 | arm | cutoff -> rank |\n|---|---|---|---|").ok();
    let mut prefix_total = 0;
    let mut prefix_equal = 0;
    let mut prefix_bad = Vec::new();
    for n in tables.ns("trajectory") {
        let seeds = tables.seeds("trajectory", n);
        for arm in ARMS {
            let recs: Vec<&Rec> = seeds
                .iter()
                .filter_map(|&s| tables.get("trajectory", n, arm, s, 1))
                .collect();
            let Some(first) = recs.first() else { continue };
            let Some(t0) = &first.traj else { continue };
            let cut = t0
                .cutoffs
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let rs: Vec<f64> = recs
                        .iter()
                        .filter_map(|r| r.traj.as_ref().and_then(|t| t.cutoffs.get(i)))
                        .map(|c| c.3 as f64)
                        .collect();
                    format!("{}: {}", c.0, fx(mean(&rs), 1))
                })
                .collect::<Vec<_>>()
                .join("; ");
            writeln!(md, "| {n} | {} | {} | {cut} |", first.columns, label(arm)).ok();
            // Prefix consistency: the cutoff at the primary cap equals the primary cell.
            for r in &recs {
                let Some(primary) = tables.recs.iter().find(|p| {
                    p.series == "primary"
                        && p.n == n
                        && p.arm == arm
                        && p.seed == r.seed
                        && p.rep == 1
                }) else {
                    continue;
                };
                if let Some(c) = r
                    .traj
                    .as_ref()
                    .and_then(|t| t.cutoffs.iter().find(|c| c.0 == primary.cap))
                {
                    prefix_total += 1;
                    if (c.1, c.2, c.3) == (primary.complete, primary.independent, primary.rank) {
                        prefix_equal += 1;
                    } else {
                        prefix_bad.push(format!("{arm} n={n} seed={}", r.seed));
                    }
                }
            }
        }
    }
    writeln!(md, "\nPrefix-consistency check (the trajectory cell at trial cutoff = primary cap against the primary cell: complete, independent, rank): {prefix_equal} of {prefix_total} equal{}.\n", if prefix_bad.is_empty() { String::new() } else { format!("; DIFFERENT: {}", prefix_bad.join(", ")) }).ok();
    summary.insert(
        "prefix_consistency".into(),
        json!({"cells": prefix_total, "equal": prefix_equal, "different": prefix_bad}),
    );

    // ------------------------------------------------------- 5. realisation
    writeln!(md, "### Table 5. Second realisation of the factor base (same curve over a seeded random modulus)\n").ok();
    writeln!(md, "Exploratory (protocol section 6). `merged` is the large-point merge count of that realisation; the primary table's merge count for the bench modulus is given for comparison.\n").ok();
    writeln!(md, "| n | S | L | orbits | merged (random modulus) | merged (bench modulus) | indep (ii) | indep (iii) | indep (iii-post) | D_post | signs | D_orig | signs | verdict D_post |").ok();
    writeln!(
        md,
        "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"
    )
    .ok();
    // The series are separated by the modulus: the realisation series is the
    // random modulus number 1, every other series the bench's own.
    if let Some(bad) = tables
        .recs
        .iter()
        .find(|r| (r.series == "realisation") != (r.modulus_seed == Some(1)))
    {
        return Err(format!(
            "receipt with the wrong modulus seed for its series: {} n={} arm={} seed={}",
            bad.series, bad.n, bad.arm, bad.seed
        ));
    }
    let mut realisation = Vec::new();
    for n in tables.ns("realisation") {
        let seeds = tables.seeds("realisation", n);
        let trio: Vec<(&Rec, &Rec, &Rec)> = seeds
            .iter()
            .filter_map(|&s| {
                Some((
                    tables.get("realisation", n, "dlp", s, 1)?,
                    tables.get("realisation", n, "dlp_orbit", s, 1)?,
                    tables.get("realisation", n, "dlp_orbit_post", s, 1)?,
                ))
            })
            .collect();
        if trio.is_empty() {
            continue;
        }
        let d_post: Vec<f64> = trio
            .iter()
            .map(|(a, _, c)| c.independent as f64 - a.independent as f64)
            .collect();
        let d_orig: Vec<f64> = trio
            .iter()
            .map(|(a, b, _)| b.independent as f64 - a.independent as f64)
            .collect();
        let m =
            |f: &dyn Fn(&(&Rec, &Rec, &Rec)) -> f64| mean(&trio.iter().map(f).collect::<Vec<_>>());
        let sparse_merged = tables
            .recs
            .iter()
            .find(|r| {
                r.series == "primary"
                    && r.n == n
                    && r.arm == "dlp_orbit"
                    && r.seed == 1
                    && r.rep == 1
            })
            .and_then(|r| r.merged);
        let o = trio[0].1;
        writeln!(
            md,
            "| {n} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            trio.len(),
            o.large_points.unwrap_or(0),
            o.large_orbits.unwrap_or(0),
            o.merged.unwrap_or(0),
            sparse_merged.map_or("-".into(), |v| v.to_string()),
            fx(m(&|t| t.0.independent as f64), 1),
            fx(m(&|t| t.1.independent as f64), 1),
            fx(m(&|t| t.2.independent as f64), 1),
            pm(&d_post, 1),
            signs(&d_post),
            pm(&d_orig, 1),
            signs(&d_orig),
            verdict(&d_post)
        )
        .ok();
        realisation.push(json!({"n": n, "seeds": trio.len(), "merged": o.merged, "D_post": d_post, "D_orig": d_orig, "verdict_D_post": verdict(&d_post)}));
    }
    summary.insert("realisation".into(), Value::Array(realisation));

    // ---------------------------------------------------------- 6. integrity
    let total_fail: u64 = tables
        .recs
        .iter()
        .map(|r| r.failed_linear + r.failed_curve + r.audit_failed + r.replays_failed)
        .sum();
    let total_verified: u64 = tables.recs.iter().map(|r| r.verified).sum();
    let recovered_bad = tables
        .recs
        .iter()
        .filter(|r| r.recovered_ok == Some(false))
        .count();
    let recovered_ok = tables
        .recs
        .iter()
        .filter(|r| r.recovered_ok == Some(true))
        .count();
    writeln!(md, "\n### Integrity\n\n- receipts: {}; verified complete relations: {total_verified}.\n- replay or partial-audit failures (linear + curve + failed replays + audit, all receipts): {total_fail}.\n- receipts whose recovered scalar equals the planted log: {recovered_ok}; disagreeing: {recovered_bad}.", tables.recs.len()).ok();
    summary.insert("integrity".into(), json!({"receipts": tables.recs.len(), "verified": total_verified, "failures": total_fail, "recovered_ok": recovered_ok, "recovered_bad": recovered_bad}));

    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    fs::write(out.join("tables.md"), &md).map_err(|e| e.to_string())?;
    fs::write(
        out.join("summary.json"),
        serde_json::to_string_pretty(&Value::Object(summary.clone())).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    fs::write(out.join("chart.svg"), chart(&contrasts, &summary)).map_err(|e| e.to_string())?;
    eprintln!(
        "wrote tables.md, summary.json and chart.svg in {}",
        out.display()
    );
    Ok(())
}

/// Left: independent relations, `D_orig` and `D_post` per n (mean and 95%
/// half-width).  Right: median trials to reach 90% of full rank, by arm, as a
/// multiple of the primary cap.
fn chart(contrasts: &[Value], summary: &serde_json::Map<String, Value>) -> String {
    let (w, h) = (900.0, 440.0);
    let mut s = String::new();
    writeln!(s, r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" font-family="sans-serif" font-size="12">"##).ok();
    writeln!(s, r##"<rect width="{w}" height="{h}" fill="#ffffff"/>"##).ok();
    writeln!(s, r##"<text x="20" y="24" font-size="15" font-weight="bold">Orbit quotient on the identical stream (paired by seed), Koblitz curves over F_(2^n)</text>"##).ok();
    let (y0, ph, pw, x0) = (70.0, 280.0, 360.0, 70.0);
    let lim = contrasts
        .iter()
        .flat_map(|c| {
            ["D_orig", "D_post"].map(|k| {
                c[format!("{k}_mean")].as_f64().unwrap_or(0.0).abs()
                    + c[format!("{k}_half")]
                        .as_f64()
                        .filter(|x| !x.is_nan())
                        .unwrap_or(0.0)
            })
        })
        .fold(1.0f64, f64::max)
        * 1.2;
    let ymap = |v: f64| y0 + ph - v.max(-lim * 0.1) / lim * ph * 0.9;
    writeln!(s, r##"<text x="{x0}" y="56" font-weight="bold">d independent relations at the cap (mean, 95% interval)</text>"##).ok();
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
    writeln!(s, r##"<text x="{}" y="{}" text-anchor="end">0</text><text x="{}" y="{}" text-anchor="end">+{lim:.0}</text>"##, x0 - 6.0, ymap(0.0) + 4.0, x0 - 6.0, y0 + 10.0).ok();
    let step = pw / contrasts.len().max(1) as f64;
    for (i, c) in contrasts.iter().enumerate() {
        let cx = x0 + step * (i as f64 + 0.5);
        for (k, dx, colour) in [("D_orig", -14.0, "#1f4e8c"), ("D_post", 14.0, "#b5541c")] {
            let m = c[format!("{k}_mean")].as_f64().unwrap_or(0.0);
            let hw = c[format!("{k}_half")]
                .as_f64()
                .filter(|x| !x.is_nan())
                .unwrap_or(0.0);
            writeln!(s, r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{colour}" stroke-width="2"/><circle cx="{}" cy="{}" r="4.5" fill="{colour}"/>"##, cx + dx, ymap(m + hw), cx + dx, ymap(m - hw), cx + dx, ymap(m)).ok();
        }
        writeln!(
            s,
            r##"<text x="{cx}" y="{}" text-anchor="middle">n={}</text>"##,
            y0 + ph + 16.0,
            c["n"]
        )
        .ok();
    }
    writeln!(s, r##"<text x="{x0}" y="{}" fill="#456">blue: (iii) - (ii), the merged headline. orange: (iii-post) - (ii), identical stream.</text>"##, y0 + ph + 36.0).ok();
    // Right panel.
    let rx0 = 500.0;
    writeln!(s, r##"<text x="{rx0}" y="56" font-weight="bold">median trials to 90% of full rank, in units of the primary cap</text>"##).ok();
    writeln!(
        s,
        r##"<rect x="{rx0}" y="{y0}" width="{pw}" height="{ph}" fill="none" stroke="#888"/>"##
    )
    .ok();
    let caps = [
        (17u64, 200.0),
        (19, 400.0),
        (23, 450.0),
        (29, 300.0),
        (31, 450.0),
    ];
    let mut rows: Vec<(u64, [Option<f64>; 3])> = Vec::new();
    if let Some(Value::Array(items)) = summary.get("trajectory") {
        for &(n, cap) in &caps {
            let mut row = [None, None, None];
            // The 90% target is the third rank target listed for the cell.
            let targets: Vec<&Value> = items
                .iter()
                .filter(|t| t["n"].as_u64() == Some(n))
                .collect();
            if let Some(t) = targets.get(2) {
                for (slot, key) in ["ii", "iii", "iii_post"].iter().enumerate() {
                    let mut v: Vec<f64> = t[*key]
                        .as_array()
                        .map(|a| {
                            a.iter()
                                .map(|x| x.as_u64().map_or(f64::INFINITY, |x| x as f64))
                                .collect()
                        })
                        .unwrap_or_default();
                    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    if let Some(&m) = v.get(v.len() / 2) {
                        if m.is_finite() {
                            row[slot] = Some(m / cap);
                        }
                    }
                }
            }
            rows.push((n, row));
        }
    }
    let top = rows
        .iter()
        .flat_map(|r| r.1.iter().flatten().copied())
        .fold(1.0f64, f64::max)
        * 1.15;
    let ymap2 = |v: f64| y0 + ph - v / top * ph;
    writeln!(s, r##"<text x="{}" y="{}" text-anchor="end">0</text><text x="{}" y="{}" text-anchor="end">{top:.1}</text>"##, rx0 - 6.0, y0 + ph + 4.0, rx0 - 6.0, y0 + 10.0).ok();
    let step2 = pw / rows.len().max(1) as f64;
    for (i, (n, row)) in rows.iter().enumerate() {
        let cx = rx0 + step2 * (i as f64 + 0.5);
        for (slot, (dx, colour)) in [(-18.0, "#555555"), (0.0, "#1f4e8c"), (18.0, "#b5541c")]
            .iter()
            .enumerate()
        {
            if let Some(v) = row[slot] {
                writeln!(
                    s,
                    r##"<circle cx="{}" cy="{}" r="5" fill="{colour}"/>"##,
                    cx + dx,
                    ymap2(v)
                )
                .ok();
            }
        }
        writeln!(
            s,
            r##"<text x="{cx}" y="{}" text-anchor="middle">n={n}</text>"##,
            y0 + ph + 16.0
        )
        .ok();
    }
    writeln!(s, r##"<text x="{rx0}" y="{}" fill="#456">grey (ii), blue (iii), orange (iii-post); censored medians are not drawn.</text>"##, y0 + ph + 36.0).ok();
    writeln!(s, r##"<text x="20" y="{}" fill="#456">Source: results_addendum/summary.json. Toy scale; no cryptographic-size or ECDLP speedup claim.</text>"##, h - 14.0).ok();
    writeln!(s, "</svg>").ok();
    s
}
