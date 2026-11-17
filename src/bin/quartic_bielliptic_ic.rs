//! `quartic_bielliptic_ic`: open case O4 of the chained-transfer proof note.
//! The line-relation index calculus on the bielliptic plane quartic
//! `C: v^4 = x^3 + a x + b` (arm Q), the elliptic-addition control with the
//! same columns and linear algebra (arm D), against the negation-map rho on
//! `E: y^2 = x^3 + a x + b` in the same process (plain rho as context).
//!
//! ```text
//! quartic_bielliptic_ic --threads 2 --min-r-frac 0.85 --max-prime-hops 20 \
//!   --stage1 229,421,821,1621,3209,6421,12841 --stage2 25601,51217,102409 \
//!   --budget-s 1500 --inv-weight 10 --seed0 0x5EED0415 \
//!   --out-dir DIR --protocol PATH
//! quartic_bielliptic_ic --select-only     # the A4 table, compared row for row
//! quartic_bielliptic_ic --exhaustive      # the p = 53 known-answer replay (A9 i)
//! ```
//!
//! Protocol (frozen by hash): `research/chained_transfer_next_20261115/PROTOCOL.md`,
//! "Amendment 1" governing.  Primary unit: one `F_p` or `F_r` multiplication or
//! squaring, an inversion at `--inv-weight` (A1).  Writes `quartic_ic.json`,
//! `quartic_ic_table.md` and `quartic_ic.svg` to `--out-dir`.  Exit status 0
//! only when every log of every arm was certified by `[t]G == T` (by `ec.rs`)
//! and every control passed.  Every figure is `[pcc:unverified]`.

#![recursion_limit = "256"]

use std::process::ExitCode;
use std::time::Instant;

use clap::Parser;
use rayon::prelude::*;
use serde_json::{json, Value};

use crypto_lib::cryptanalysis::chained_transfer::full_jacobian_ic::{
    fit_loglog, rho_full_ec_tallied, t95,
};
use crypto_lib::cryptanalysis::chained_transfer::negation_remeasure::{
    classify, mean_rho, measure_class_times, time_add_ec, Class, ClassTimes, RhoFull, BAND,
    CAL_FACTORS, TIMING_BATCHES, TIMING_BATCH_LEN,
};
use crypto_lib::cryptanalysis::chained_transfer::quartic_ic::{
    a4_matches, bsgs_check, certificate_tamper_rejected, dense_solve, exhaustive, exponent_report,
    ic_seed, make_instance, measure_quartic_class_times, rho_seed, seed_k, select_prime_order,
    solve_direct_ic, solve_quartic_ic, tamper_control, time_add_u64, ArmKind, EcU64, Inst, Ops,
    QClassTimes, QuarticCover, RunOut, RunParams, A4_TABLE, LOWER_BOUND_NOTE, PHASES, SEED0,
};
use crypto_lib::cryptanalysis::ic_boundary::FieldOps;

fn parse_u64(s: &str) -> Result<u64, String> {
    let t = s.replace('_', "");
    if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        u64::from_str_radix(h, 16).map_err(|e| e.to_string())
    } else {
        t.parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())
    }
}

#[derive(Parser, Debug)]
#[command(
    about = "O4: line-relation index calculus on a bielliptic plane quartic against matched rho"
)]
struct Args {
    #[arg(long, default_value_t = 2)]
    threads: usize,
    #[arg(long, default_value_t = 0.85)]
    min_r_frac: f64,
    #[arg(long, default_value_t = 20)]
    max_prime_hops: u32,
    #[arg(long, default_value = "229,421,821,1621,3209,6421,12841")]
    stage1: String,
    #[arg(long, default_value = "25601,51217,102409")]
    stage2: String,
    /// Wall-clock budget: no stage-2 prime is started after it.
    #[arg(long, default_value_t = 1500)]
    budget_s: u64,
    /// Weight of one inversion in multiplications (field unit, A1).
    #[arg(long, default_value_t = 10.0)]
    inv_weight: f64,
    #[arg(long, default_value = "0x5EED0415", value_parser = parse_u64)]
    seed0: u64,
    #[arg(long, default_value = "research/chained_transfer_next_20261115")]
    out_dir: String,
    #[arg(
        long,
        default_value = "research/chained_transfer_next_20261115/PROTOCOL.md"
    )]
    protocol: String,
    /// Print the selection of every requested prime, compare it with the A4
    /// table row for row, and exit (no arm, no file).
    #[arg(long, default_value_t = false)]
    select_only: bool,
    /// The p = 53 known-answer replay in the old pair order (A9 i), and exit.
    #[arg(long, default_value_t = false)]
    exhaustive: bool,
    /// Isolation label of the host, recorded as given (A18).
    #[arg(long, default_value = "not stated: treat as a shared VM")]
    isolation: String,
    // ── smoke-only overrides; the defaults are the frozen values ──
    /// Instances per stage-1 prime (frozen: 3).
    #[arg(long, default_value_t = 3)]
    instances: u64,
    /// IC seeds per instance, stage 1 (frozen: 4).
    #[arg(long, default_value_t = 4)]
    ic_seeds: u64,
    /// Rho seeds per instance, stage 1 (frozen: 16).
    #[arg(long, default_value_t = 16)]
    rho_seeds: u64,
    /// Unit class times, no calibration (smoke only; the output says so).
    #[arg(long, default_value_t = false)]
    no_calibration: bool,
}

impl Args {
    fn deviations(&self) -> Vec<String> {
        let mut d = vec![];
        if self.instances != 3 {
            d.push(format!("instances {}", self.instances));
        }
        if self.ic_seeds != 4 {
            d.push(format!("ic_seeds {}", self.ic_seeds));
        }
        if self.rho_seeds != 16 {
            d.push(format!("rho_seeds {}", self.rho_seeds));
        }
        if self.no_calibration {
            d.push("no_calibration".into());
        }
        if self.inv_weight != 10.0 {
            d.push(format!("inv_weight {}", self.inv_weight));
        }
        if self.seed0 != SEED0 {
            d.push(format!("seed0 {:#x}", self.seed0));
        }
        if self.stage1 != "229,421,821,1621,3209,6421,12841" {
            d.push(format!("stage1 {}", self.stage1));
        }
        if self.stage2 != "25601,51217,102409" {
            d.push(format!("stage2 {}", self.stage2));
        }
        d
    }
}

fn host(isolation: &str) -> Value {
    let cpu = std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("model name"))
                .and_then(|l| l.split(':').nth(1))
                .map(|x| x.trim().to_string())
        })
        .unwrap_or_default();
    let cpus = std::thread::available_parallelism().map_or(0, |n| n.get());
    json!({"cpu": cpu, "cpus": cpus, "isolation": isolation})
}

/// An arm priced in one unit: a base and native counts by class.
#[derive(Clone, Debug)]
struct Priced {
    base: f64,
    native: Vec<(&'static str, f64)>,
    lower_bound: bool,
}

impl Priced {
    fn band(&self) -> [f64; 3] {
        let s: f64 = self.native.iter().map(|x| x.1).sum();
        BAND.map(|w| self.base + w * s)
    }
    fn cal(&self, wcal: &dyn Fn(&str) -> f64) -> [f64; 3] {
        let s: f64 = self.native.iter().map(|(c, n)| wcal(c) * n).sum();
        CAL_FACTORS.map(|f| self.base + f * s)
    }
    fn with_base(&self, base: f64) -> Priced {
        Priced {
            base,
            ..self.clone()
        }
    }
}

fn rho_class_time(t: &ClassTimes, c: &str) -> f64 {
    t.get(match c {
        "canon" => Class::Canon,
        "hash" => Class::Hash,
        "lookup" => Class::Lookup,
        "insert" => Class::Insert,
        "word" => Class::Word,
        _ => return f64::NAN,
    })
}

fn rho_native(m: &Value) -> Vec<(&'static str, f64)> {
    let g = |k: &str| m[k].as_f64().unwrap_or(0.0);
    vec![
        ("canon", g("canonicalisations")),
        ("hash", g("partition_hashes")),
        ("lookup", g("dp_lookups")),
        ("insert", g("dp_inserts")),
        ("word", g("word_ops")),
    ]
}

fn field_means(f: &[FieldOps], w_inv: f64) -> Value {
    let n = f.len().max(1) as f64;
    let m = |g: &dyn Fn(&FieldOps) -> u64| f.iter().map(|x| g(x) as f64).sum::<f64>() / n;
    let (mu, sq, iv) = (m(&|x| x.muls), m(&|x| x.sqrs), m(&|x| x.invs));
    json!({"field_mul": mu, "field_sqr": sq, "field_inv": iv, "base": mu + sq + w_inv * iv})
}

/// Means of a set of IC runs, priced (field unit and GAE).
struct IcMean {
    field: Priced,
    gae: Priced,
    json: Value,
}

fn ic_mean(rs: &[RunOut], w_inv: f64) -> IcMean {
    let n = rs.len().max(1) as f64;
    let m = |f: &dyn Fn(&RunOut) -> f64| rs.iter().map(f).sum::<f64>() / n;
    let mo = |f: &dyn Fn(&Ops) -> u64| m(&|r| f(&r.total_ops()) as f64);
    let base = m(&|r| r.base_field(w_inv));
    let field_native: Vec<(&'static str, f64)> = Ops::default()
        .native_field()
        .iter()
        .map(|&(c, _)| (c, m(&|r| field_class(&r.total_ops(), c))))
        .collect();
    let all_native: Vec<(&'static str, f64)> = Ops::default()
        .native_all()
        .iter()
        .map(|&(c, _)| (c, m(&|r| field_class(&r.total_ops(), c))))
        .collect();
    let group_ops = m(&|r| r.total_group().ops() as f64);
    let phases: Vec<Value> = (0..4)
        .map(|ph| {
            json!({"phase": PHASES[ph],
                   "base": m(&|r| r.phase_base(ph, w_inv)),
                   "group_ops": m(&|r| r.group[ph].ops() as f64),
                   "mul_p": m(&|r| r.ops[ph].mul_p as f64), "mul_r": m(&|r| r.ops[ph].mul_r as f64),
                   "add_p": m(&|r| r.ops[ph].add_p as f64), "add_r": m(&|r| r.ops[ph].add_r as f64),
                   "inv_p": m(&|r| r.ops[ph].inv_p as f64), "inv_r": m(&|r| r.ops[ph].inv_r as f64)})
        })
        .collect();
    let cols = m(&|r| r.columns as f64);
    let json = json!({
        "runs": rs.len(),
        "certified": rs.iter().filter(|r| r.certified).count(),
        "exhausted": rs.iter().filter(|r| r.exhausted).count(),
        "failures": rs.iter().filter_map(|r| r.failure.clone()).collect::<Vec<_>>(),
        "lower_bound": true, "lower_bound_note": LOWER_BOUND_NOTE,
        "base_field": base,
        "base_field_w_inv_0": m(&|r| r.base_field(0.0)),
        "base_field_w_inv_30": m(&|r| r.base_field(30.0)),
        "group_ops": group_ops,
        "group_ops_max": rs.iter().map(|r| r.total_group().ops()).max(),
        "group_field_units": m(&|r| r.total_group().field_units(w_inv)),
        "mul_p": mo(&|o| o.mul_p), "add_p": mo(&|o| o.add_p), "inv_p": mo(&|o| o.inv_p),
        "mul_r": mo(&|o| o.mul_r), "add_r": mo(&|o| o.add_r), "inv_r": mo(&|o| o.inv_r),
        "canon": mo(&|o| o.canon), "lookup": mo(&|o| o.lookup), "insert": mo(&|o| o.insert),
        "draw": mo(&|o| o.draw),
        "columns": cols,
        "rows_used": m(&|r| r.rows_used as f64),
        "rows_over_n": m(&|r| r.rows_used as f64 / r.columns.max(1) as f64),
        "attempts": m(&|r| f64::from(r.attempts)),
        "trials": m(&|r| r.trials as f64),
        "duplicate_lines": m(&|r| r.duplicate_lines as f64),
        "nonsplit": m(&|r| r.nonsplit as f64),
        "zero_rows": m(&|r| r.zero_rows as f64),
        "rejected": m(&|r| r.rejected as f64),
        "core_rows": m(&|r| r.core_rows as f64), "core_cols": m(&|r| r.core_cols as f64),
        "peeled": m(&|r| r.peeled as f64),
        "anchor_multiple": m(&|r| r.anchor_multiple as f64),
        "shift": m(&|r| r.shift.unwrap_or(0) as f64),
        "phases": phases,
        "verification": {
            "relation_checks": rs.iter().map(|r| r.verify.relation_checks).sum::<u64>(),
            "relation_failures": rs.iter().map(|r| r.verify.relation_failures).sum::<u64>(),
            "replayed_columns": rs.iter().map(|r| r.verify.replayed_columns).sum::<u64>(),
            "replay_mismatches": rs.iter().map(|r| r.verify.replay_mismatches).sum::<u64>(),
            "replay_group_ops_mean": m(&|r| r.verify.replay_group.ops() as f64),
            "fibre_witness_missing": rs.iter().filter(|r| r.verify.fibre_witness == Some(false)).count(),
            "note": "outside every total (A6); sensitivity only",
        },
        "wall_s": m(&|r| r.wall_ns as f64 / 1e9),
        "wall_s_max": rs.iter().map(|r| r.wall_ns as f64 / 1e9).fold(0.0, f64::max),
    });
    IcMean {
        field: Priced {
            base,
            native: field_native,
            lower_bound: true,
        },
        gae: Priced {
            base: group_ops,
            native: all_native,
            lower_bound: true,
        },
        json,
    }
}

fn field_class(o: &Ops, c: &str) -> f64 {
    o.native_all()
        .iter()
        .find(|x| x.0 == c)
        .map_or(0.0, |x| x.1)
}

/// The class of an arm against rho under the field-tally base and both time
/// re-pricings (A15), in the band and the calibrated vector (A1).
fn cell_class(
    arm: &Priced,
    arm_wcal: &dyn Fn(&str) -> f64,
    rho: &Priced,
    rho_bases: &[(&str, f64)],
    rho_wcal: &dyn Fn(&str) -> f64,
) -> Value {
    let (ab, ac) = (arm.band(), arm.cal(arm_wcal));
    let mut labels = serde_json::Map::new();
    let mut all_above = true;
    let mut all_counted_below = true;
    for (name, base) in rho_bases {
        let r = rho.with_base(*base);
        let (rb, rc) = (r.band(), r.cal(rho_wcal));
        let (lb, lc) = (
            classify(&ab, &rb, arm.lower_bound),
            classify(&ac, &rc, arm.lower_bound),
        );
        all_above &= lb == "above" && lc == "above";
        all_counted_below &=
            classify(&ab, &rb, false) == "below" && classify(&ac, &rc, false) == "below";
        labels.insert(
            (*name).into(),
            json!({"band": lb, "cal": lc, "rho_band": rb, "rho_cal": rc,
                   "ratio_band": [ab[0] / rb[0], ab[1] / rb[1], ab[2] / rb[2]],
                   "ratio_cal": [ac[0] / rc[0], ac[1] / rc[1], ac[2] / rc[2]]}),
        );
    }
    let class = if all_above {
        "above"
    } else if all_counted_below {
        if arm.lower_bound {
            "unresolved (lower bound)"
        } else {
            "below"
        }
    } else {
        "unresolved"
    };
    json!({"class": class, "counted_below_rho": all_counted_below,
           "arm_band": ab, "arm_cal": ac, "by_rho_base": labels})
}

struct Stop(String);

fn run_prime(a: &Args, i: usize, p_req: u64, stage: u8, t_all: Instant) -> Result<Value, Stop> {
    let t0 = Instant::now();
    let sel = match select_prime_order(p_req, i, a.min_r_frac, a.max_prime_hops) {
        Ok(s) => s,
        Err(e) => {
            println!("p={p_req}: not run: {e}");
            return Ok(json!({"p_requested": p_req, "stage": stage, "not_run": e}));
        }
    };
    let frozen_ladder = a.stage1 == "229,421,821,1621,3209,6421,12841"
        && a.stage2 == "25601,51217,102409"
        && a.min_r_frac == 0.85;
    if frozen_ladder && !a4_matches(&sel) {
        return Err(Stop(format!(
            "selection at index {i} differs from the A4 table: {sel:?}"
        )));
    }
    let (p, r) = (sel.p, sel.r);
    let w_inv = a.inv_weight;
    let (ninst, nseeds, nrho) = if stage == 1 {
        (a.instances, a.ic_seeds, a.rho_seeds)
    } else {
        (1, 2.min(a.ic_seeds), 8.min(a.rho_seeds))
    };
    let insts: Vec<(Inst, u64, bool)> = (0..ninst)
        .map(|k| make_instance(p, sel.a, sel.b, r, a.seed0, k))
        .collect();
    for (k, (inst, x, moved)) in insts.iter().enumerate() {
        println!(
            "p={p} k={k}: seed_k={:#x} G=({},{}) x={x}{} T=({},{}) ic_seeds Q={:?} D={:?} rho_seeds={:?}",
            seed_k(a.seed0, k as u64),
            inst.g.0,
            inst.g.1,
            if *moved { " (x was 0, set to 1)" } else { "" },
            inst.t.0,
            inst.t.1,
            (0..nseeds).map(|j| ic_seed(ArmKind::Q, k as u64, j)).collect::<Vec<_>>(),
            (0..nseeds).map(|j| ic_seed(ArmKind::D, k as u64, j)).collect::<Vec<_>>(),
            (0..nrho).map(|j| rho_seed(k as u64, j)).collect::<Vec<_>>(),
        );
    }
    let ec = EcU64::new(p, sel.a, sel.b).expect("curve");
    let g0 = EcU64::to_pt(Some(insts[0].0.g));

    // ── calibration: sequential, no arm and no other thread running (A18) ──
    let unit = QClassTimes {
        mul_p: 1.0,
        add_p: 1.0,
        inv_p: 1.0,
        mul_r: 1.0,
        add_r: 1.0,
        inv_r: 1.0,
        canon: 1.0,
        lookup: 1.0,
        insert: 1.0,
        draw: 1.0,
    };
    let (qt, rt, t_add_ec, t_add_u64) = if a.no_calibration {
        (
            unit,
            ClassTimes {
                mul_p: 1.0,
                mul_r: 1.0,
                inv_r: 1.0,
                canon: 1.0,
                hash: 1.0,
                lookup: 1.0,
                insert: 1.0,
                draw: 1.0,
                word: 1.0,
            },
            1.0,
            1.0,
        )
    } else {
        (
            measure_quartic_class_times(p, r, TIMING_BATCHES, TIMING_BATCH_LEN),
            measure_class_times(p, r, &ec.curve, g0, TIMING_BATCHES, TIMING_BATCH_LEN),
            time_add_ec(&ec.curve, g0, TIMING_BATCHES, TIMING_BATCH_LEN),
            time_add_u64(&ec, Some(insts[0].0.g), TIMING_BATCHES, TIMING_BATCH_LEN),
        )
    };
    let t_mul = qt.mul_p;

    // ── matched rho on E: negation primary, plain context (A1, A6) ──
    let mut rneg: Vec<RhoFull> = vec![];
    let mut rplain: Vec<RhoFull> = vec![];
    let mut fneg: Vec<FieldOps> = vec![];
    let mut fplain: Vec<FieldOps> = vec![];
    for (k, (inst, _, _)) in insts.iter().enumerate() {
        let (g, t) = (EcU64::to_pt(Some(inst.g)), EcU64::to_pt(Some(inst.t)));
        for neg in [true, false] {
            let out: Vec<(RhoFull, FieldOps)> = (0..nrho)
                .into_par_iter()
                .map(|j| rho_full_ec_tallied(&ec.curve, g, t, r, rho_seed(k as u64, j), neg))
                .collect();
            for (o, f) in out {
                if neg {
                    rneg.push(o);
                    fneg.push(f);
                } else {
                    rplain.push(o);
                    fplain.push(f);
                }
            }
        }
    }
    let rho_ok = rneg.iter().chain(&rplain).all(|x| x.certified);

    // ── arms Q and D (cold runs; k = 0 and p <= 421 keep their system) ──
    let run_arm = |kind: ArmKind| -> Vec<RunOut> {
        let jobs: Vec<(u64, u64)> = (0..ninst)
            .flat_map(|k| (0..nseeds).map(move |j| (k, j)))
            .collect();
        jobs.into_par_iter()
            .map(|(k, j)| {
                let mut prm = RunParams::new(ic_seed(kind, k, j));
                prm.keep_system = k == 0 || p <= 421;
                let inst = &insts[k as usize].0;
                match kind {
                    ArmKind::Q => solve_quartic_ic(inst, prm),
                    ArmKind::D => solve_direct_ic(inst, prm),
                }
            })
            .collect()
    };
    let tq = Instant::now();
    let mut q_runs = run_arm(ArmKind::Q);
    let q_wall = tq.elapsed().as_secs_f64();
    let td = Instant::now();
    let mut d_runs = run_arm(ArmKind::D);
    let d_wall = td.elapsed().as_secs_f64();
    for r_ in &q_runs {
        println!(
            "p={p} Q seed {}: log {:?} certified {} rows {} attempts {} base {:.0} wall {:.2}s {}",
            r_.seed,
            r_.log,
            r_.certified,
            r_.rows_used,
            r_.attempts,
            r_.base_field(w_inv),
            r_.wall_ns as f64 / 1e9,
            r_.failure.as_deref().unwrap_or("")
        );
    }
    // two consecutive uncertified Q runs (in (k, j) order): failed cell
    let failed_cell = q_runs
        .windows(2)
        .any(|w| !w[0].certified && !w[1].certified);

    // ── controls (A5, A9) ──
    let mut controls = serde_json::Map::new();
    let mut dense = vec![];
    for r_ in q_runs.iter().chain(&d_runs) {
        if p > 421 {
            break;
        }
        let Some(sys) = &r_.system else { continue };
        let Some(logs) = &sys.logs else { continue };
        let (rank, sol) = dense_solve(&sys.rows, sys.cols.len(), r);
        let agree = sol.as_ref() == Some(logs);
        dense.push(json!({"arm": r_.arm, "seed": r_.seed, "rank": rank, "agrees": agree}));
        if !agree {
            return Err(Stop(format!(
                "p={p} {} seed {}: dense Gauss-Jordan disagrees with the Wiedemann logs",
                r_.arm, r_.seed
            )));
        }
    }
    controls.insert("dense_cross_check".into(), json!(dense));
    let mut bsgs = vec![];
    for r_ in q_runs.iter().chain(&d_runs) {
        if r_.seed != ic_seed(ArmKind::Q, 0, 0) && r_.seed != ic_seed(ArmKind::D, 0, 0) {
            continue;
        }
        let Some(sys) = &r_.system else { continue };
        let Some(logs) = &sys.logs else { continue };
        let (checked, bad) = bsgs_check(&insts[0].0, &sys.cols, logs);
        bsgs.push(json!({"arm": r_.arm, "seed": r_.seed, "checked": checked, "mismatches": bad}));
        if bad > 0 {
            return Err(Stop(format!(
                "p={p} {} seed {}: {bad} of {checked} BSGS column checks disagree",
                r_.arm, r_.seed
            )));
        }
    }
    controls.insert("bsgs_32_columns".into(), json!(bsgs));
    if stage == 1 {
        match tamper_control(&insts[0].0, ic_seed(ArmKind::Q, 0, 0)) {
            Some((_, t)) => {
                let detected = t.detected;
                controls.insert("tamper".into(), serde_json::to_value(&t).unwrap());
                if !detected {
                    return Err(Stop(format!("p={p}: the tampered system still certifies")));
                }
            }
            None => {
                controls.insert(
                    "tamper".into(),
                    json!({"not_run": "the clean run did not solve"}),
                );
            }
        }
    }
    if p == 12841 || stage == 2 {
        let q0 = q_runs
            .iter()
            .find(|r_| r_.seed == ic_seed(ArmKind::Q, 0, 0))
            .and_then(|r_| r_.system.as_ref())
            .and_then(|s| s.certs.first().cloned());
        let mut o = Ops::default();
        let cover = QuarticCover::new(p, sel.a, sel.b, &mut o).expect("cover");
        match q0 {
            Some(cert) => {
                let rejected = certificate_tamper_rejected(&cover, &cert);
                controls.insert("altered_certificate_rejected".into(), json!(rejected));
                if !rejected {
                    return Err(Stop(format!(
                        "p={p}: verify_relation accepted an altered certificate"
                    )));
                }
            }
            None => {
                controls.insert(
                    "altered_certificate_rejected".into(),
                    json!({"not_run": "no certificate kept"}),
                );
            }
        }
    }
    // determinism check: Q again on k = 0 of the first three stage-1 primes
    if stage == 1 && i < 3 {
        let firsts: Vec<&RunOut> = q_runs.iter().filter(|x| x.seed < 710).collect();
        let again: Vec<RunOut> = firsts
            .iter()
            .map(|x| solve_quartic_ic(&insts[0].0, RunParams::new(x.seed)))
            .collect();
        let same = firsts
            .iter()
            .zip(&again)
            .all(|(x, y)| x.ops == y.ops && x.group == y.group && x.log == y.log);
        controls.insert(
            "determinism".into(),
            json!({"identical_counts": same,
                   "wall_s_first": firsts.iter().map(|x| x.wall_ns as f64 / 1e9).collect::<Vec<_>>(),
                   "wall_s_second": again.iter().map(|x| x.wall_ns as f64 / 1e9).collect::<Vec<_>>()}),
        );
        if !same {
            return Err(Stop(format!("p={p}: the determinism check differs")));
        }
    }
    for r_ in q_runs.iter_mut().chain(d_runs.iter_mut()) {
        r_.system = None;
    }

    // ── pricing (A1, A15) ──
    let qm = ic_mean(&q_runs, w_inv);
    let dm = ic_mean(&d_runs, w_inv);
    let neg_m = serde_json::to_value(mean_rho(&rneg)).unwrap();
    let plain_m = serde_json::to_value(mean_rho(&rplain)).unwrap();
    let (neg_f, plain_f) = (field_means(&fneg, w_inv), field_means(&fplain, w_inv));
    let rho_neg = Priced {
        base: neg_f["base"].as_f64().unwrap(),
        native: rho_native(&neg_m),
        lower_bound: false,
    };
    let rho_plain = Priced {
        base: plain_f["base"].as_f64().unwrap(),
        native: rho_native(&plain_m),
        lower_bound: false,
    };
    let ops_neg = neg_m["total_ops"].as_f64().unwrap();
    let ops_plain = plain_m["total_ops"].as_f64().unwrap();
    let bases_neg = [
        ("field_tally", rho_neg.base),
        ("ec_rs_time", ops_neg * t_add_ec / t_mul),
        ("u64_time", ops_neg * t_add_u64 / t_mul),
    ];
    let bases_plain = [
        ("field_tally", rho_plain.base),
        ("ec_rs_time", ops_plain * t_add_ec / t_mul),
        ("u64_time", ops_plain * t_add_u64 / t_mul),
    ];
    let q_wcal = |c: &str| qt.get(c) / t_mul;
    let rho_wcal = |c: &str| rho_class_time(&rt, c) / t_mul;
    let q_neg = cell_class(&qm.field, &q_wcal, &rho_neg, &bases_neg, &rho_wcal);
    let d_neg = cell_class(&dm.field, &q_wcal, &rho_neg, &bases_neg, &rho_wcal);
    let q_plain = cell_class(&qm.field, &q_wcal, &rho_plain, &bases_plain, &rho_wcal);
    // GAE reading (secondary): 1 GAE = one addition on E (ec.rs time)
    let q_gae_wcal = |c: &str| qt.get(c) / t_add_ec;
    let rho_gae_wcal = |c: &str| rho_class_time(&rt, c) / t_add_ec;
    let rho_neg_gae = rho_neg.with_base(ops_neg);
    let q_gae = cell_class(
        &qm.gae,
        &q_gae_wcal,
        &rho_neg_gae,
        &[("group_ops", ops_neg)],
        &rho_gae_wcal,
    );
    // w_inv sensitivity: base only, native at 0
    let winv: Vec<Value> = [0.0, a.inv_weight, 30.0]
        .iter()
        .map(|&w| {
            let qb = q_runs.iter().map(|x| x.base_field(w)).sum::<f64>() / q_runs.len() as f64;
            let rb = field_means(&fneg, w)["base"].as_f64().unwrap();
            json!({"w_inv": w, "q_base": qb, "rho_base": rb,
                   "class": if qb > rb { "above" } else if qb < rb { "below" } else { "equal" }})
        })
        .collect();
    // Q against D: a relation-source component comparison (A14)
    let q_vs_d = {
        let (qb, db) = (qm.field.band(), dm.field.band());
        let (qc, dc) = (qm.field.cal(&q_wcal), dm.field.cal(&q_wcal));
        json!({"q_band": qb, "d_band": db, "q_cal": qc, "d_cal": dc,
               "q_below_d_at_every_price": qb.iter().zip(&db).all(|(x, y)| x < y)
                   && qc.iter().zip(&dc).all(|(x, y)| x < y),
               "q_above_d_in_both_vectors": qb.iter().zip(&db).all(|(x, y)| x > y)
                   && qc.iter().zip(&dc).all(|(x, y)| x > y),
               "note": "relation-source component comparison, not an improvement of the index-calculus baseline (A14) [pcc:unverified]"})
    };
    let qb = qm.field.band();
    let qc = qm.field.cal(&q_wcal);
    let rb = rho_neg.band();
    let rc = rho_neg.cal(&rho_wcal);
    let all_certified = q_runs.iter().chain(&d_runs).all(|x| x.certified) && rho_ok;
    let row = json!({
        "p": p, "p_requested": p_req, "stage": stage, "index": i,
        "selection": serde_json::to_value(&sel).unwrap(),
        "a4_match": a4_matches(&sel),
        "n": sel.n, "n_over_p": sel.n as f64 / p as f64, "r": r, "r_over_p": r as f64 / p as f64,
        "instances": insts.iter().map(|(inst, x, moved)| json!({"g": inst.g, "t": inst.t, "x": x, "x_moved_from_0": moved})).collect::<Vec<_>>(),
        "ic_seeds_per_instance": nseeds, "rho_seeds_per_instance": nrho,
        "calibration": {"skipped": a.no_calibration, "quartic_class_times_ns": serde_json::to_value(qt).unwrap(),
                        "rho_class_times_ns": serde_json::to_value(rt).unwrap(),
                        "t_mul_ns": t_mul, "t_add_ec_rs_ns": t_add_ec, "t_add_u64_ns": t_add_u64},
        "Q": qm.json, "D": dm.json,
        "Q_runs": serde_json::to_value(&q_runs).unwrap(),
        "D_runs": serde_json::to_value(&d_runs).unwrap(),
        "rho_E": {"negation": neg_m, "plain": plain_m, "field_negation": neg_f, "field_plain": plain_f,
                  "certified": rho_ok, "band_negation": rb, "cal_negation": rc,
                  "rho_bases_negation": bases_neg.iter().map(|(k, v)| json!({"base": k, "value": v})).collect::<Vec<_>>()},
        "cell": q_neg, "cell_D": d_neg, "cell_Q_vs_plain": q_plain, "cell_gae": q_gae,
        "R": qb[1] / rb[1], "R_cal": qc[1] / rc[1],
        "w_inv_sensitivity": winv,
        "Q_vs_D": q_vs_d,
        "controls": controls,
        "failed_cell": failed_cell,
        "all_certified": all_certified,
        "wall_s": t0.elapsed().as_secs_f64(), "wall_s_Q": q_wall, "wall_s_D": d_wall,
        "elapsed_since_start_s": t_all.elapsed().as_secs_f64(),
    });
    println!(
        "p={p}: n={} Q certified {}/{} R(0.1)={:.3} R_cal={:.3} class {} gae {} ({:.1}s)",
        sel.n,
        q_runs.iter().filter(|x| x.certified).count(),
        q_runs.len(),
        row["R"].as_f64().unwrap_or(f64::NAN),
        row["R_cal"].as_f64().unwrap_or(f64::NAN),
        row["cell"]["class"],
        row["cell_gae"]["class"],
        t0.elapsed().as_secs_f64()
    );
    Ok(row)
}

fn fits(rows: &[Value], label: &str) -> Value {
    let ps: Vec<f64> = rows.iter().map(|r| r["p"].as_f64().unwrap()).collect();
    let fit = |ys: Vec<f64>| {
        fit_loglog(&ps, &ys).map_or(Value::Null, |(s, a, se, ci)| {
            json!({"slope": s, "intercept": a, "se": se, "ci95": ci, "n": ps.len(), "t95": t95(ps.len().saturating_sub(2))})
        })
    };
    let col = |f: &dyn Fn(&Value) -> f64| rows.iter().map(f).collect::<Vec<f64>>();
    let q_phases: Vec<Vec<f64>> = (0..4)
        .map(|ph| col(&|r| r["Q"]["phases"][ph]["base"].as_f64().unwrap()))
        .collect();
    let rho_walk = col(&|r| r["rho_E"]["negation"]["walk_ops"].as_f64().unwrap());
    let rho_setup = col(&|r| r["rho_E"]["negation"]["other_ops"].as_f64().unwrap());
    json!({
        "label": label, "primes": ps,
        "s_ratio_w0.1": fit(col(&|r| r["R"].as_f64().unwrap())),
        "s_cal_ratio_cal1": fit(col(&|r| r["R_cal"].as_f64().unwrap())),
        "Q_mul_r": fit(col(&|r| r["Q"]["mul_r"].as_f64().unwrap())),
        "Q_add_r": fit(col(&|r| r["Q"]["add_r"].as_f64().unwrap())),
        "Q_total_w0.1": fit(col(&|r| r["cell"]["arm_band"][1].as_f64().unwrap())),
        "Q_base_field": fit(col(&|r| r["Q"]["base_field"].as_f64().unwrap())),
        "D_total_w0.1": fit(col(&|r| r["cell_D"]["arm_band"][1].as_f64().unwrap())),
        "rho_total_field": fit(col(&|r| r["rho_E"]["field_negation"]["base"].as_f64().unwrap())),
        "rho_total_ops": fit(col(&|r| r["rho_E"]["negation"]["total_ops"].as_f64().unwrap())),
        "rho_walk_ops": fit(rho_walk.clone()),
        "Q_phase_exponents": serde_json::to_value(exponent_report(&ps, &q_phases)).unwrap(),
        "Q_phase_names": PHASES,
        "rho_phase_exponents": serde_json::to_value(exponent_report(&ps, &[rho_setup, rho_walk])).unwrap(),
        "rho_phase_names": ["setup", "walk"],
    })
}

/// The decision rule of Amendment 1 over the stage-1 rows (A1, A11, A15).
fn verdict(rows: &[Value], requested: usize, fit: &Value, gae: bool) -> Value {
    let key = if gae { "cell_gae" } else { "cell" };
    let mut blocking: Vec<String> = vec![];
    if rows.len() != requested {
        blocking.push(format!(
            "{} of {requested} stage-1 cells present",
            rows.len()
        ));
    }
    let mut all_above = true;
    let mut any_below = false;
    let mut any_counted_below = false;
    for r in rows {
        let c = &r[key];
        let class = c["class"].as_str().unwrap_or("missing");
        if class != "above" {
            all_above = false;
            blocking.push(format!("p={} {class}", r["p"]));
        }
        any_below |= class == "below";
        any_counted_below |= c["counted_below_rho"].as_bool() == Some(true);
        let q = &r["Q"];
        if q["certified"] != q["runs"] || r["failed_cell"] == true {
            all_above = false;
            blocking.push(format!(
                "p={} Q certified {}/{}",
                r["p"], q["certified"], q["runs"]
            ));
        }
    }
    let lo_s = fit["s_ratio_w0.1"]["ci95"][0].as_f64();
    let lo_c = fit["s_cal_ratio_cal1"]["ci95"][0].as_f64();
    let slopes_ok = gae || (lo_s.is_some_and(|v| v > 0.2) && lo_c.is_some_and(|v| v > 0.2));
    if !slopes_ok {
        blocking.push(format!(
            "slope lower ends {lo_s:?} and {lo_c:?} not both above 0.2"
        ));
    }
    let v = if any_below {
        "Q_below_rho"
    } else if all_above && slopes_ok && rows.len() == requested {
        "Q_above_rho"
    } else if any_counted_below {
        "unresolved (lower bound)"
    } else {
        "unresolved"
    };
    json!({"verdict": v, "blocking": blocking, "any_counted_below_rho": any_counted_below,
           "slope_lower_ends": [lo_s, lo_c],
           "scope": "family v^4 = x^3 + a x + b, full factor base, the sizes run (A11)",
           "unit": if gae { "GAE: one addition on E; every other operation native (secondary, not predicted)" }
                   else { "field: one F_p or F_r multiplication or squaring, inversion at inv_weight (A1, primary)" }})
}

fn table(rows: &[Value]) -> String {
    let mut s = String::from("| p | n | r | Q cert | rows/n | Q base | rho base | R(0.1) | R_cal | class | class D | class GAE |\n|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    for r in rows {
        if r.get("Q").is_none() {
            s += &format!("| {} | not run | | | | | | | | | | |\n", r["p_requested"]);
            continue;
        }
        s += &format!(
            "| {} | {} | {} | {}/{} | {:.2} | {:.0} | {:.0} | {:.3} | {:.3} | {} | {} | {} |\n",
            r["p"],
            r["n"],
            r["r"],
            r["Q"]["certified"],
            r["Q"]["runs"],
            r["Q"]["rows_over_n"].as_f64().unwrap_or(f64::NAN),
            r["Q"]["base_field"].as_f64().unwrap_or(f64::NAN),
            r["rho_E"]["field_negation"]["base"]
                .as_f64()
                .unwrap_or(f64::NAN),
            r["R"].as_f64().unwrap_or(f64::NAN),
            r["R_cal"].as_f64().unwrap_or(f64::NAN),
            r["cell"]["class"].as_str().unwrap_or(""),
            r["cell_D"]["class"].as_str().unwrap_or(""),
            r["cell_gae"]["class"].as_str().unwrap_or(""),
        );
    }
    s + "\nEvery figure is `[pcc:unverified]`.\n"
}

/// Log-log plot of `R(p)` and `R_cal(p)` with the line `R = 1`.
fn svg(rows: &[Value]) -> String {
    let pts: Vec<(f64, f64, f64)> = rows
        .iter()
        .filter_map(|r| Some((r["p"].as_f64()?, r["R"].as_f64()?, r["R_cal"].as_f64()?)))
        .filter(|x| x.1 > 0.0 && x.2 > 0.0)
        .collect();
    let (w, h, m) = (640.0, 400.0, 60.0);
    if pts.is_empty() {
        return format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\"><text x=\"20\" y=\"40\">no rows</text></svg>\n");
    }
    let lx: Vec<f64> = pts.iter().map(|x| x.0.log10()).collect();
    let ly: Vec<f64> = pts
        .iter()
        .flat_map(|x| [x.1.log10(), x.2.log10()])
        .chain([0.0])
        .collect();
    let (x0, x1) = (
        lx.iter().copied().fold(f64::MAX, f64::min) - 0.1,
        lx.iter().copied().fold(f64::MIN, f64::max) + 0.1,
    );
    let (y0, y1) = (
        ly.iter().copied().fold(f64::MAX, f64::min) - 0.2,
        ly.iter().copied().fold(f64::MIN, f64::max) + 0.2,
    );
    let sx = |v: f64| m + (v - x0) / (x1 - x0) * (w - 2.0 * m);
    let sy = |v: f64| h - m - (v - y0) / (y1 - y0) * (h - 2.0 * m);
    let mut s = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" font-family=\"sans-serif\" font-size=\"12\">\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n");
    s += &format!(
        "<line x1=\"{m}\" y1=\"{0}\" x2=\"{1}\" y2=\"{0}\" stroke=\"#888\" stroke-dasharray=\"4 3\"/>\n",
        sy(0.0),
        w - m
    );
    s += &format!(
        "<text x=\"{}\" y=\"{}\">R = 1</text>\n",
        w - m - 40.0,
        sy(0.0) - 4.0
    );
    for (series, color, label, k) in [
        (1, "#1f77b4", "R (w = 0.1)", 0),
        (2, "#d62728", "R_cal (factor 1)", 1),
    ] {
        let path: Vec<String> = pts
            .iter()
            .map(|x| {
                let y = if series == 1 { x.1 } else { x.2 };
                format!("{:.1},{:.1}", sx(x.0.log10()), sy(y.log10()))
            })
            .collect();
        s += &format!(
            "<polyline fill=\"none\" stroke=\"{color}\" stroke-width=\"2\" points=\"{}\"/>\n",
            path.join(" ")
        );
        for pt in &path {
            let (x, y) = pt.split_once(',').unwrap();
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"3\" fill=\"{color}\"/>\n");
        }
        s += &format!(
            "<text x=\"{}\" y=\"{}\" fill=\"{color}\">{label}</text>\n",
            m + 10.0,
            m - 30.0 + 16.0 * k as f64
        );
    }
    for x in &pts {
        s += &format!(
            "<text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\">{}</text>\n",
            sx(x.0.log10()),
            h - m + 18.0,
            x.0
        );
    }
    s += &format!("<text x=\"{}\" y=\"{}\" text-anchor=\"middle\">p (log scale); ratio of Q to negation rho, field unit, log scale [pcc:unverified]</text>\n</svg>\n", w / 2.0, h - 15.0);
    s
}

fn select_only(a: &Args, s1: &[u64], s2: &[u64]) -> ExitCode {
    let mut ok = true;
    for (i, &p) in s1.iter().chain(s2).enumerate() {
        match select_prime_order(p, i, a.min_r_frac, a.max_prime_hops) {
            Ok(s) => {
                let m = a4_matches(&s);
                ok &= m;
                println!(
                    "{}",
                    json!({"i": i, "p_requested": p, "p": s.p, "passed_over": s.passed_over,
                           "u": s.u, "a": s.a, "b": s.b, "r": s.r, "r_over_p": s.r as f64 / s.p as f64,
                           "points_c": s.points_c, "n": s.n, "n_over_p": s.n as f64 / s.p as f64,
                           "ln_n_over_4": (s.n as f64).ln() / 4.0, "skips": s.skips,
                           "a4_row": A4_TABLE.get(i).map(|t| json!({"p": t.p, "u": t.u, "a": t.a, "b": t.b, "r": t.r, "n": t.n})),
                           "a4_match": m})
                );
            }
            Err(e) => {
                ok = false;
                println!("{}", json!({"i": i, "p_requested": p, "error": e}));
            }
        }
    }
    println!(
        "select-only: A4 table {}",
        if ok {
            "reproduced row for row (p, u, a, b, r, n)"
        } else {
            "NOT reproduced: stop before any arm"
        }
    );
    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn exhaustive_mode() -> Result<Value, String> {
    let (p, a, b, r, g) = (53u64, 2u64, 1u64, 59u64, (0u64, 1u64));
    let ex = exhaustive(p, a, b, r, g)?;
    let ec = EcU64::new(p, a, b).ok_or("curve")?;
    let t17 = ec.mul_independent(Some(g), 17).ok_or("17G")?;
    let rec = ex.recover(t17);
    let mut replay_ok = 0u64;
    let mut replay_total = 0u64;
    for gen in [g, t17] {
        let exg = exhaustive(p, a, b, r, gen)?;
        for s in 0..r {
            if let Some(t) = ec.mul_independent(Some(gen), s) {
                replay_total += 1;
                if exg.recover(t) == Some((s, true)) {
                    replay_ok += 1;
                }
            }
        }
    }
    let pass = ex.rational_points == 45
        && ex.certified_relations == 79
        && ex.columns == 11
        && ex.dense_rank == 11
        && ex.la_agrees_with_dense
        && rec == Some((17, true))
        && replay_ok == replay_total
        && replay_total == 2 * (r - 1);
    Ok(json!({"exhaustive": serde_json::to_value(&ex).unwrap(),
              "target_scalar": rec.map(|x| x.0), "target_certified": rec.map(|x| x.1),
              "scalar_replay": {"generators": [g, t17], "recovered": replay_ok, "of": replay_total},
              "expected": {"rational_points": 45, "sections": 79, "columns": 11, "rank": 11, "scalar": 17},
              "pass": pass}))
}

fn main() -> ExitCode {
    let a = Args::parse();
    rayon::ThreadPoolBuilder::new()
        .num_threads(a.threads)
        .build_global()
        .ok();
    let t_all = Instant::now();
    let parse =
        |s: &str| -> Vec<u64> { s.split(',').filter_map(|x| x.trim().parse().ok()).collect() };
    let (s1, s2) = (parse(&a.stage1), parse(&a.stage2));
    if a.select_only {
        return select_only(&a, &s1, &s2);
    }
    if a.exhaustive {
        return match exhaustive_mode() {
            Ok(v) => {
                println!("{}", serde_json::to_string_pretty(&v).unwrap());
                if v["pass"] == true {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            Err(e) => {
                eprintln!("exhaustive: {e}");
                ExitCode::FAILURE
            }
        };
    }
    // the positive control runs first in the same process; a failure stops the run
    let positive = match exhaustive_mode() {
        Ok(v) if v["pass"] == true => v,
        Ok(v) => {
            eprintln!("positive control failed: {v}");
            return ExitCode::FAILURE;
        }
        Err(e) => {
            eprintln!("positive control failed: {e}");
            return ExitCode::FAILURE;
        }
    };
    let mut rows: Vec<Value> = vec![];
    let mut stopped: Option<String> = None;
    let mut not_run: Vec<Value> = vec![];
    for (i, &p) in s1.iter().enumerate() {
        match run_prime(&a, i, p, 1, t_all) {
            Ok(r) => rows.push(r),
            Err(Stop(why)) => {
                stopped = Some(why);
                break;
            }
        }
    }
    let stage1_rows: Vec<Value> = rows
        .iter()
        .filter(|r| r.get("Q").is_some())
        .cloned()
        .collect();
    let mut stage2_rows: Vec<Value> = vec![];
    if stopped.is_none() {
        for (j, &p) in s2.iter().enumerate() {
            if t_all.elapsed().as_secs() > a.budget_s {
                not_run.push(json!({"p": p, "reason": format!("budget of {} s passed before it started", a.budget_s)}));
                continue;
            }
            match run_prime(&a, s1.len() + j, p, 2, t_all) {
                Ok(r) => {
                    if r.get("Q").is_some() {
                        stage2_rows.push(r.clone());
                    }
                    rows.push(r);
                }
                Err(Stop(why)) => {
                    stopped = Some(why);
                    break;
                }
            }
        }
    }
    let f1 = fits(&stage1_rows, "stage 1 (verdict)");
    let f2 = fits(&stage2_rows, "stage 2 (separate, not in the verdict)");
    let v = verdict(&stage1_rows, s1.len(), &f1, false);
    let v_gae = verdict(&stage1_rows, s1.len(), &f1, true);
    let all_ok = stopped.is_none()
        && rows
            .iter()
            .all(|r| r.get("Q").is_none() || r["all_certified"] == true);
    let out = json!({
        "schema": "o4-quartic-ic/v1",
        "protocol": a.protocol,
        "args": format!("{a:?}"),
        "deviations_from_frozen_defaults": a.deviations(),
        "host": host(&a.isolation),
        "positive_control": positive,
        "stopped": stopped,
        "ok": all_ok,
        "rows": rows,
        "stage2_not_run": not_run,
        "fits_stage1": f1,
        "fits_stage2": f2,
        "verdict": v["verdict"],
        "verdict_detail": v,
        "verdict_gae": format!("{} (secondary, GAE reading, not predicted)", v_gae["verdict"].as_str().unwrap_or("")),
        "verdict_gae_detail": v_gae,
        "inv_weight": a.inv_weight,
        "pricing": {
            "field_unit": "1 = one F_p or F_r multiplication or squaring; one inversion = inv_weight (A1)",
            "q_d_base": "MulP + MulR + w_inv (InvP + InvR) + every E addition/doubling by ec_add_tally (muls + sqrs + w_inv invs)",
            "rho_base": "rho_full_ec_tallied FieldOps over the whole run (set-up and walk): muls + sqrs + w_inv invs; also total_ops * t_add / t_mul for the ec.rs and the u64 addition time (A15)",
            "native_q_d": ["canon", "lookup", "insert", "draw", "add_p", "add_r"],
            "native_rho": ["canon", "hash", "lookup", "insert", "word"],
            "band": BAND, "cal_factors": CAL_FACTORS,
            "w_cal": "time(class) / time(one F_p multiplication), best of 25 batches of 20,000 in the same process",
            "cell_class": "above (below) only when classify gives it in the band and the calibrated vector under all three rho bases; Q and D carry lower_bound",
        },
        "wall_s": t_all.elapsed().as_secs_f64(),
    });
    let _ = std::fs::create_dir_all(&a.out_dir);
    let path = format!("{}/quartic_ic.json", a.out_dir);
    std::fs::write(&path, serde_json::to_string_pretty(&out).unwrap()).expect("write json");
    std::fs::write(format!("{}/quartic_ic_table.md", a.out_dir), table(&rows))
        .expect("write table");
    std::fs::write(format!("{}/quartic_ic.svg", a.out_dir), svg(&stage1_rows)).expect("write svg");
    println!(
        "wrote {path}; ok={all_ok}; verdict: {}; verdict_gae: {}{}",
        out["verdict"],
        out["verdict_gae"],
        out["stopped"]
            .as_str()
            .map(|s| format!("; STOPPED: {s}"))
            .unwrap_or_default()
    );
    if all_ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
