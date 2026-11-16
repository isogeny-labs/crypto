//! `negation_remeasure`: v3, cover_pushforward and tower_large re-measured
//! against the negation-map rho in the same process, with every native counter
//! of every arm emitted and priced at the 5a band and at the calibrated weight.
//!
//! ```text
//! negation_remeasure [v3] [cover] [tower] [merge] [--out-dir DIR] [--tag T]
//!                    [--tower-primes 251,...] [--threads 4]
//! ```
//!
//! Protocol (frozen by hash): `research/negation_remeasure_20261113/PROTOCOL.md`.
//! Each section writes `remeasure_<section><tag>.json`; `merge` joins the parts
//! into `remeasure.json` and `remeasure.svg`.  Exit status 0 only when every log
//! of every arm was certified by an independent `[k]G == T` and every
//! reproduction gate passed.  Every figure is `[pcc:unverified]`.

use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use clap::Parser;
use rayon::prelude::*;
use serde_json::{json, Value};

use crypto_lib::cryptanalysis::chained_transfer::amortized::certify_dlog;
use crypto_lib::cryptanalysis::chained_transfer::bielliptic::Quotient;
use crypto_lib::cryptanalysis::chained_transfer::hyper64::{
    build64, crosscheck, ic, H64Curve, H64Div, Tower64,
};
use crypto_lib::cryptanalysis::chained_transfer::ic_control::{Arm as IcArm, IcResult};
use crypto_lib::cryptanalysis::chained_transfer::instrument::{
    convert_div, genus2_to_h64, ic_on_rung, instance_for_rung, solve_linear_counted,
};
use crypto_lib::cryptanalysis::chained_transfer::measure::{
    genus2_collect_rows, genus2_index_calculus, rho_on_ec, EcGroup,
};
use crypto_lib::cryptanalysis::chained_transfer::negation_remeasure::{
    aut_kind, classify, cover_select, mean_rho, measure_class_times, report_arm, rho_full_ec,
    v3_first, Arm, Class, ClassTimes, RefSet, RhoFull, BAND, CAL_FACTORS, TIMING_BATCHES,
    TIMING_BATCH_LEN,
};
use crypto_lib::cryptanalysis::chained_transfer::pushforward::{
    certify_ec, collect_jc, e1_birthday_first_collision, e1_fibre_ic, e1_two_term_ic,
    jc_first_collision, solve_rows,
};
use crypto_lib::cryptanalysis::ic_boundary::rho_reference;

#[derive(Parser, Debug)]
#[command(about = "Chained-transfer targets re-measured against the negation-map rho")]
struct Args {
    /// Sections: v3, cover, tower, merge.
    sections: Vec<String>,
    #[arg(long, default_value = "research/negation_remeasure_20261113")]
    out_dir: String,
    /// Suffix of the part file (`remeasure_<section><tag>.json`).
    #[arg(long, default_value = "")]
    tag: String,
    #[arg(long, default_value = "251,503,1009,2003,4001,8009,16001")]
    tower_primes: String,
    #[arg(long, default_value_t = 3)]
    tower_instances: usize,
    /// Algorithm seeds of the recorded tower arm (the record used 8).
    #[arg(long, default_value_t = 8)]
    tower_old_seeds: u64,
    /// Algorithm seeds of arms A, B, C on the tower.
    #[arg(long, default_value_t = 4)]
    tower_seeds: u64,
    #[arg(long, default_value_t = 64)]
    tower_rho_seeds: u64,
    #[arg(long, default_value_t = 8)]
    v3_seeds: u64,
    #[arg(long, default_value_t = 32)]
    v3_rho_seeds: u64,
    #[arg(long, default_value = "211,419,821,1619,3203,6421,12841")]
    cover_primes: String,
    #[arg(long, default_value_t = 3)]
    cover_seeds: usize,
    #[arg(long, default_value_t = 8)]
    cover_ic_seeds: u64,
    #[arg(long, default_value_t = 32)]
    cover_rho_seeds: u64,
    #[arg(long, default_value_t = 4)]
    threads: usize,
    #[arg(long, default_value_t = 40_000_000)]
    max_trials: u64,
    /// Skip the timing calibration (smoke runs only; the part says so).
    #[arg(long, default_value_t = false)]
    no_calibration: bool,
}

// ── shared helpers ───────────────────────────────────────────────────────

fn host() -> Value {
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
    json!({"cpu": cpu, "cpus": cpus, "isolation": "none: shared VM, other processes may run"})
}

fn calibration_meta() -> Value {
    json!({
        "method": "best-of-N timing (not perf-ipc; no isolated host): minimum over batches of ns per operation",
        "batches": TIMING_BATCHES, "batch_len": TIMING_BATCH_LEN,
        "definition": "w_cal(class) = time(one native operation of the class) / time(one group addition of the arm's own group), same host and process",
        "calibrated_reading": "every native class at 0, 1 and 10 times its own w_cal",
        "host": host(),
    })
}

struct Cal {
    times: ClassTimes,
    skipped: bool,
}

fn calibrate(
    no_cal: bool,
    p: u64,
    r: u64,
    e: &crypto_lib::cryptanalysis::chained_transfer::ec::Curve,
    g: crypto_lib::cryptanalysis::chained_transfer::ec::Pt,
) -> Cal {
    if no_cal {
        // unit times: the calibrated reading collapses to the band at w = 1 and
        // is flagged by the part
        return Cal {
            times: ClassTimes {
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
            skipped: true,
        };
    }
    Cal {
        times: measure_class_times(p, r, e, g, TIMING_BATCHES, TIMING_BATCH_LEN),
        skipped: false,
    }
}

fn t_add_or_unit<F: FnMut()>(no_cal: bool, f: F) -> f64 {
    if no_cal {
        1.0
    } else {
        crypto_lib::cryptanalysis::chained_transfer::negation_remeasure::best_of(
            TIMING_BATCHES,
            TIMING_BATCH_LEN,
            f,
        )
    }
}

/// Mean of IC runs on a rung: the arm and the extras.
struct IcMean {
    arm: Arm,
    runs: usize,
    certified: usize,
    exhausted: usize,
    json: Value,
}

fn ic_mean(rs: &[(IcResult, bool)]) -> IcMean {
    let n = rs.len().max(1) as f64;
    let m = |f: &dyn Fn(&IcResult) -> f64| rs.iter().map(|(r, _)| f(r)).sum::<f64>() / n;
    let (go, rm, ra, lm, la) = (
        m(&|r| r.group_ops() as f64),
        m(&|r| r.rel.mul as f64),
        m(&|r| r.rel.add as f64),
        m(&|r| r.la.mul as f64),
        m(&|r| r.la.add as f64),
    );
    let certified = rs.iter().filter(|(_, c)| *c).count();
    let exhausted = rs.iter().filter(|(r, _)| r.exhausted).count();
    IcMean {
        arm: Arm {
            group_ops: go,
            native: vec![(Class::MulP, rm), (Class::MulR, lm)],
            lower_bound: false,
        },
        runs: rs.len(),
        certified,
        exhausted,
        json: json!({
            "runs": rs.len(), "certified": certified, "exhausted": exhausted,
            "group_ops": go, "setup_ops": m(&|r| r.setup_ops as f64),
            "walk_ops": m(&|r| r.walk_ops as f64), "verify_ops": m(&|r| r.verify_ops as f64),
            "rel_mul": rm, "rel_add": ra, "la_mul": lm, "la_add": la,
            "trials": m(&|r| r.trials as f64), "relations": m(&|r| r.relations as f64),
            "columns": m(&|r| r.fb_cols as f64), "core_cols": m(&|r| r.core_cols as f64),
            "attempts": m(&|r| f64::from(r.attempts)),
            "total_w1_with_additions": go + rm + ra + lm + la,
        }),
    }
}

fn arm_of(label: &str) -> IcArm {
    match label {
        "A" => IcArm::Full,
        "B" => IcArm::SingleLp,
        _ => IcArm::DoubleLp,
    }
}

fn rtl_ok(h: &H64Curve, d: H64Div, t: H64Div, k: u64) -> bool {
    h.is_valid(&d) && h.is_valid(&t) && h.mul_rtl(d, k) == t
}

/// An arm's entry: the report against the references plus extras.
fn entry(
    name: &str,
    arm: &Arm,
    t_add: f64,
    times: &ClassTimes,
    refs: &RefSet,
    extra: Value,
) -> Value {
    let mut v = serde_json::to_value(report_arm(name, arm, t_add, times, refs)).unwrap();
    if let (Some(o), Some(e)) = (v.as_object_mut(), extra.as_object()) {
        for (k, x) in e {
            o.insert(k.clone(), x.clone());
        }
    }
    v
}

/// The recorded distance: `(ops + w native)/old_rho_ops`, native as recorded.
fn recorded_ratio(ops: f64, native: f64, old_rho: f64) -> [f64; 3] {
    BAND.map(|w| (ops + w * native) / old_rho)
}

fn rho_block(
    neg: &[RhoFull],
    plain: &[RhoFull],
    e: &crypto_lib::cryptanalysis::chained_transfer::ec::Curve,
) -> Value {
    json!({
        "negation": serde_json::to_value(mean_rho(neg)).unwrap(),
        "plain": serde_json::to_value(mean_rho(plain)).unwrap(),
        "automorphism_fold": {
            "j_invariant": e.j_invariant(),
            "automorphism_beyond_negation": aut_kind(e),
            "fold_run": false,
            "note": "a curve with j = 0 or 1728 would be an implementation gap: the fold is not implemented for this curve type",
        },
    })
}

fn refs_of(neg: &[RhoFull], plain: &[RhoFull], t_add: f64, times: ClassTimes) -> RefSet {
    RefSet {
        neg: mean_rho(neg).arm(),
        plain: mean_rho(plain).arm(),
        t_add_ns: t_add,
        times,
    }
}

fn write_part(a: &Args, name: &str, body: Value) {
    let path = format!("{}/remeasure_{}{}.json", a.out_dir, name, a.tag);
    std::fs::write(&path, serde_json::to_string_pretty(&body).unwrap()).expect("write part");
    println!("wrote {path}");
}

// ── v3 ───────────────────────────────────────────────────────────────────

/// Values the v3 record holds, per prime: candidate, rho on E1, IC group
/// operations, IC trials, LA counter.
const V3_RECORDED: [(u64, u64, u64, u64, u64, u64); 4] = [
    (211, 9, 428, 3419, 226, 14633),
    (419, 27, 357, 8475, 542, 51976),
    (821, 2, 370, 11353, 851, 164828),
    (1619, 1, 847, 34521, 1653, 603721),
];

fn run_v3(a: &Args) -> (Value, bool) {
    let mut rows = vec![];
    let (mut ok, mut gate_ok) = (true, true);
    for (i, &(p, rec_cand, rec_rho, rec_ops, rec_trials, rec_la)) in V3_RECORDED.iter().enumerate()
    {
        let t0 = Instant::now();
        let Some((cand, v)) = v3_first(p, i) else {
            println!("v3 p={p}: no candidate");
            ok = false;
            continue;
        };
        // Recorded arms, as measured (the gate).
        let rho_old = rho_on_ec(&v.e1, v.gen, v.target, v.r, v.seed);
        let rho_old_ops = rho_old.group_ops.adds + rho_old.group_ops.doubles;
        let ic_old =
            genus2_index_calculus(&v.jac, &v.dg, &v.dt, v.r, p as usize, 50_000_000, v.seed);
        let gate = cand == rec_cand
            && rho_old_ops == rec_rho
            && ic_old.group_ops == rec_ops
            && ic_old.trials == rec_trials
            && ic_old.la_field_ops == rec_la
            && ic_old.recovered == Some(v.x);
        gate_ok &= gate;
        println!(
            "v3 p={p}: gate cand {cand} rho {rho_old_ops} ic {} trials {} la {} -> {}",
            ic_old.group_ops,
            ic_old.trials,
            ic_old.la_field_ops,
            if gate {
                "reproduces the record"
            } else {
                "MISMATCH"
            }
        );
        let old_cert = ic_old
            .recovered
            .is_some_and(|k| v.jac.mul(&v.dg, k) == v.dt);
        ok &= old_cert;
        // The word Jacobian.
        let h = genus2_to_h64(&v.jac).expect("word Jacobian");
        let hg = convert_div(&h, &v.dg).expect("G converts");
        let ht = convert_div(&h, &v.dt).expect("T converts");
        let inst = instance_for_rung(&h, v.jac_order, v.r, hg, ht, v.x).expect("instance");
        // Calibration (sequential, before the parallel work).
        let cal = calibrate(a.no_calibration, p, v.r, &v.e1, v.gen);
        let t_add_e = t_add_or_unit(a.no_calibration, {
            let pts: Vec<_> = (1..=64u128).map(|m| v.e1.mul(&v.gen, m * 7)).collect();
            let mut acc = None;
            let mut i = 0usize;
            move || {
                acc = v.e1.add(&acc, &pts[i % 64]);
                i += 1;
                std::hint::black_box(&acc);
            }
        });
        let t_add_jold = t_add_or_unit(a.no_calibration, {
            let ring: Vec<_> = (1..=64u64).map(|m| v.jac.mul(&v.dg, m * 7)).collect();
            let mut acc = crypto_lib::cryptanalysis::genus2_fp::Divisor::IDENTITY;
            let mut i = 0usize;
            let jr = &v.jac;
            move || {
                acc = jr.add(&acc, &ring[i % 64]);
                i += 1;
                std::hint::black_box(&acc);
            }
        });
        let t_add_h = t_add_or_unit(a.no_calibration, {
            let ring: Vec<H64Div> = (1..=64u64).map(|m| h.mul_rtl(hg, m * 7)).collect();
            let mut acc = ring[0];
            let mut i = 0usize;
            let hr = &h;
            move || {
                acc = hr.cantor(acc, ring[i % 64]);
                i += 1;
                std::hint::black_box(&acc);
            }
        });
        // Matched rho on E1: negation and plain, 32 seeds, in this process.
        let runs = |neg: bool| -> Vec<RhoFull> {
            (0..a.v3_rho_seeds)
                .into_par_iter()
                .map(|s| rho_full_ec(&v.e1, v.gen, v.target, v.r, 1000 + s, neg))
                .collect()
        };
        let (rneg, rplain) = (runs(true), runs(false));
        ok &= rneg.iter().chain(&rplain).all(|r| r.certified);
        let refs = refs_of(&rneg, &rplain, t_add_e, cal.times);
        let mut arms = serde_json::Map::new();
        // Recorded collector (lower bound: its relation-phase field work is not emitted).
        let old_arm = Arm {
            group_ops: ic_old.group_ops as f64,
            native: vec![(Class::MulR, ic_old.la_field_ops as f64)],
            lower_bound: true,
        };
        let rr = recorded_ratio(
            ic_old.group_ops as f64,
            ic_old.la_field_ops as f64,
            rho_old_ops as f64,
        );
        arms.insert(
            "recorded".into(),
            entry("recorded", &old_arm, t_add_jold, &cal.times, &refs, json!({
                "certified": old_cert, "recorded_ratio_to_old_plain_rho": rr,
                "recorded_class": classify(&BAND.map(|w| ic_old.group_ops as f64 + w * ic_old.la_field_ops as f64), &[rho_old_ops as f64; 3], false),
                "native_emitted": {"la_field_ops": ic_old.la_field_ops},
                "native_not_emitted": ["relation-phase field work of the collector (Cantor reductions, root finding)"],
                "trials": ic_old.trials, "relations": ic_old.relations, "fb": ic_old.fb_size,
            })),
        );
        // Arm L: the recorded relations with the counted elimination.
        let rows_l = genus2_collect_rows(&v.jac, &v.dg, &v.dt, v.r, p as usize, 50_000_000, v.seed);
        let (lx, lnat) = solve_linear_counted(&rows_l.rows, rows_l.fb_size, v.r);
        let l_cert = lx.is_some_and(|k| v.jac.mul(&v.dg, k) == v.dt);
        ok &= l_cert;
        let l_arm = Arm {
            group_ops: rows_l.group_ops as f64,
            native: vec![(Class::MulR, lnat.mul as f64)],
            lower_bound: true,
        };
        arms.insert(
            "L".into(),
            entry("L", &l_arm, t_add_jold, &cal.times, &refs, json!({
                "certified": l_cert, "la_mul": lnat.mul, "la_add_sensitivity": lnat.add,
                "la_mul_over_recorded_counter": lnat.mul as f64 / ic_old.la_field_ops.max(1) as f64,
                "kind": "accounting change only (same relations, counted elimination)",
            })),
        );
        // Arms A, B, C.
        for label in ["A", "B", "C"] {
            let rs: Vec<(IcResult, bool)> = (0..a.v3_seeds)
                .into_par_iter()
                .map(|s| {
                    let res = ic_on_rung(&inst, arm_of(label), 500 + s, a.max_trials);
                    let good = res.recovered.is_some_and(|k| {
                        res.certified && rtl_ok(&h, hg, ht, k) && v.jac.mul(&v.dg, k) == v.dt
                    });
                    (res, good)
                })
                .collect();
            let m = ic_mean(&rs);
            ok &= m.certified == m.runs && m.exhausted == 0;
            arms.insert(
                label.into(),
                entry(label, &m.arm, t_add_h, &cal.times, &refs, m.json),
            );
        }
        println!("v3 p={p}: done in {:.1}s", t0.elapsed().as_secs_f64());
        rows.push(json!({
            "target": "v3", "p": p, "r": v.r, "x": v.x, "candidate": cand,
            "e1_order": v.e1_order, "e2_order": v.e2_order, "jac_order": v.jac_order,
            "gate_reproduces_record": gate, "calibration_skipped": cal.skipped,
            "class_times_ns": serde_json::to_value(cal.times).unwrap(),
            "t_add_ns": {"E1": t_add_e, "J_recorded_impl": t_add_jold, "J_word_impl": t_add_h},
            "old_plain_rho_ops": rho_old_ops,
            "rho": rho_block(&rneg, &rplain, &v.e1),
            "arms": arms, "wall_s": t0.elapsed().as_secs_f64(),
        }));
    }
    (Value::Array(rows), ok && gate_ok)
}

// ── cover_pushforward ────────────────────────────────────────────────────

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n == 0 {
        return f64::NAN;
    }
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

/// One recorded-arm sample of a cover seed.
struct CoverSample {
    name: &'static str,
    ops: f64,
    native: Vec<(Class, f64)>,
    certified: bool,
}

fn run_cover(a: &Args) -> (Value, bool) {
    let primes: Vec<u64> = a
        .cover_primes
        .split(',')
        .filter_map(|x| x.trim().parse().ok())
        .collect();
    let recorded: Option<Value> =
        std::fs::read_to_string("research/cover_pushforward_20261112/pushforward.json")
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok());
    let mut rows = vec![];
    let (mut ok, mut gate_ok) = (true, true);
    for (i, &p) in primes.iter().enumerate() {
        let t0 = Instant::now();
        let Some(ci) = cover_select(p, i) else {
            println!("cover p={p}: no admissible curve");
            ok = false;
            continue;
        };
        let b = &ci.b;
        let (ge, r, e1) = (ci.gen, ci.r, b.e1);
        let jac_order = e1.order() * b.e2.order();
        let h = genus2_to_h64(&b.jac).expect("word Jacobian");
        let cal = calibrate(a.no_calibration, p, r, &e1, ge);
        let t_add_e = t_add_or_unit(a.no_calibration, {
            let pts: Vec<_> = (1..=64u128).map(|m| e1.mul(&ge, m * 7)).collect();
            let mut acc = None;
            let mut i = 0usize;
            move || {
                acc = e1.add(&acc, &pts[i % 64]);
                i += 1;
                std::hint::black_box(&acc);
            }
        });
        let dg0 = b.pull(Quotient::E1, &ge);
        let t_add_jold = t_add_or_unit(a.no_calibration, {
            let ring: Vec<_> = (1..=64u64).map(|m| b.jac.mul(&dg0, m * 7)).collect();
            let mut acc = crypto_lib::cryptanalysis::genus2_fp::Divisor::IDENTITY;
            let mut i = 0usize;
            let jr = &b.jac;
            move || {
                acc = jr.add(&acc, &ring[i % 64]);
                i += 1;
                std::hint::black_box(&acc);
            }
        });
        let hg0 = convert_div(&h, &dg0).expect("G converts");
        let t_add_h = t_add_or_unit(a.no_calibration, {
            let ring: Vec<H64Div> = (1..=64u64).map(|m| h.mul_rtl(hg0, m * 7)).collect();
            let mut acc = ring[0];
            let mut i = 0usize;
            let hr = &h;
            move || {
                acc = hr.cantor(acc, ring[i % 64]);
                i += 1;
                std::hint::black_box(&acc);
            }
        });
        // Recorded arms over the original seeds.
        let mut per_seed: Vec<Vec<CoverSample>> = vec![];
        let mut old_rho_ops: Vec<f64> = vec![];
        let mut gate_row = true;
        for k in 0..a.cover_seeds {
            let seed = ci.seed0 ^ (k as u64 * 0x1000);
            let x = (seed.wrapping_mul(2_246_822_519).wrapping_add(7)) % r;
            let te = e1.mul(&ge, x as u128);
            let dg = b.pull(Quotient::E1, &ge);
            let dt = b.pull(Quotient::E1, &te);
            let mut s: Vec<CoverSample> = vec![];
            let rho = rho_on_ec(&e1, ge, te, r, seed);
            let rho_ops = (rho.group_ops.adds + rho.group_ops.doubles) as f64;
            let rho_ok = rho.recovered == Some(x)
                && rho
                    .recovered
                    .is_some_and(|kk| certify_ec(&e1, &ge, &te, kk));
            old_rho_ops.push(rho_ops);
            s.push(CoverSample {
                name: "rho_e1_recorded",
                ops: rho_ops,
                native: vec![],
                certified: rho_ok,
            });
            let jc = collect_jc(&b.jac, &dg, &dt, r, p as usize, 50_000_000, seed);
            let (jc_rec, jc_la) = solve_rows(&jc.rels, jc.nfb, r);
            let jc_ok =
                jc_rec.is_some_and(|kk| certify_dlog(&b.jac, &dg, &dt, kk).0) && jc_rec == Some(x);
            s.push(CoverSample {
                name: "ic_jc",
                ops: jc.ops as f64,
                native: vec![(Class::MulR, jc_la as f64)],
                certified: jc_ok,
            });
            // counted elimination of the same relations
            let rows_l: Vec<(Vec<i64>, u64, u64)> = jc
                .rels
                .iter()
                .map(|q| (q.row.clone(), q.alpha, q.beta))
                .collect();
            let (lx, lnat) = solve_linear_counted(&rows_l, jc.nfb, r);
            let l_ok = lx.is_some_and(|kk| certify_dlog(&b.jac, &dg, &dt, kk).0) && lx == Some(x);
            s.push(CoverSample {
                name: "ic_jc_L",
                ops: jc.ops as f64,
                native: vec![(Class::MulR, lnat.mul as f64)],
                certified: l_ok,
            });
            let jc_coll = jc_first_collision(b, &jc.rels, r);
            let (a4, _a4_trials, a4_coll) =
                e1_fibre_ic(b, &ge, &te, r, jc.rels.len(), 50_000_000, seed);
            let a4_ok = a4.recovered == Some(x)
                && a4.recovered.is_some_and(|kk| certify_ec(&e1, &ge, &te, kk));
            s.push(CoverSample {
                name: "ic_e1_fibre_full",
                ops: a4.group_ops as f64,
                native: vec![
                    (Class::MulR, a4.la_mulmods as f64),
                    (Class::Lookup, a4.lookups as f64),
                ],
                certified: a4_ok,
            });
            let bday = e1_birthday_first_collision(b, &ge, &te, r, 50_000_000, seed);
            let a5 = e1_two_term_ic(b, &ge, &te, r, 50_000_000, seed);
            let a5_ok = a5.recovered == Some(x)
                && a5.recovered.is_some_and(|kk| certify_ec(&e1, &ge, &te, kk));
            s.push(CoverSample {
                name: "ic_e1_two_term",
                ops: a5.group_ops as f64,
                native: vec![
                    (Class::MulR, a5.la_mulmods as f64),
                    (Class::Lookup, a5.lookups as f64),
                ],
                certified: a5_ok,
            });
            if let Some(c) = &a4_coll {
                s.push(CoverSample {
                    name: "ic_e1_fibre_first_collision",
                    ops: c.ops as f64,
                    native: vec![],
                    certified: c.log.is_some_and(|kk| certify_ec(&e1, &ge, &te, kk)),
                });
            }
            if let Some(c) = &jc_coll {
                s.push(CoverSample {
                    name: "ic_jc_first_collision",
                    ops: c.ops as f64,
                    native: vec![],
                    certified: c.log.is_some_and(|kk| certify_dlog(&b.jac, &dg, &dt, kk).0),
                });
            }
            if let Some(c) = &bday {
                s.push(CoverSample {
                    name: "birthday_e1_first_collision",
                    ops: c.ops as f64,
                    native: vec![],
                    certified: c.log.is_some_and(|kk| certify_ec(&e1, &ge, &te, kk)),
                });
            }
            // gate against the committed record
            if let Some(rec) = recorded.as_ref().and_then(|r| r["rows"].as_array()) {
                if let Some(row) = rec.iter().find(|row| row["p"].as_u64() == Some(p)) {
                    if let Some(rs) = row["seeds"].get(k) {
                        for (mine, theirs) in [
                            ("rho_e1_recorded", "rho_e1"),
                            ("ic_jc", "ic_jc"),
                            ("ic_e1_fibre_full", "ic_e1_fibre_full"),
                            ("ic_e1_two_term", "ic_e1_two_term"),
                        ] {
                            let got = s.iter().find(|x| x.name == mine).map(|x| x.ops);
                            let want = rs["arms"][theirs]["group_ops"].as_f64();
                            if got != want {
                                gate_row = false;
                                println!("cover p={p} seed {k}: gate {mine} {got:?} != {want:?}");
                            }
                        }
                    } else {
                        gate_row = false;
                    }
                } else {
                    gate_row = false;
                }
            } else {
                gate_row = false;
            }
            ok &= s.iter().all(|x| x.certified);
            per_seed.push(s);
        }
        gate_ok &= gate_row;
        println!(
            "cover p={p}: gate {}",
            if gate_row {
                "reproduces the record"
            } else {
                "MISMATCH"
            }
        );
        // Matched rho on the seed-0 instance, in this process.
        let seed0 = ci.seed0;
        let x0 = (seed0.wrapping_mul(2_246_822_519).wrapping_add(7)) % r;
        let te0 = e1.mul(&ge, x0 as u128);
        let runs = |neg: bool| -> Vec<RhoFull> {
            (0..a.cover_rho_seeds)
                .into_par_iter()
                .map(|s| rho_full_ec(&e1, ge, te0, r, 2000 + s, neg))
                .collect()
        };
        let (rneg, rplain) = (runs(true), runs(false));
        ok &= rneg.iter().chain(&rplain).all(|r| r.certified);
        let refs = refs_of(&rneg, &rplain, t_add_e, cal.times);
        // Arms A, B, C on J(C), seed-0 instance.
        let dgs = b.pull(Quotient::E1, &ge);
        let dts = b.pull(Quotient::E1, &te0);
        let hg = convert_div(&h, &dgs).expect("G converts");
        let ht = convert_div(&h, &dts).expect("T converts");
        let inst = instance_for_rung(&h, jac_order, r, hg, ht, x0).expect("instance");
        let mut arms = serde_json::Map::new();
        // recorded arms: mean over the original seeds; recorded distance = median ratio
        let names: Vec<&'static str> = {
            let mut n: Vec<&'static str> = vec![];
            for s in &per_seed {
                for x in s {
                    if !n.contains(&x.name) {
                        n.push(x.name);
                    }
                }
            }
            n
        };
        for name in names {
            let samples: Vec<&CoverSample> = per_seed
                .iter()
                .flat_map(|s| s.iter().filter(|x| x.name == name))
                .collect();
            if name == "rho_e1_recorded" {
                continue;
            }
            let n = samples.len() as f64;
            let ops = samples.iter().map(|x| x.ops).sum::<f64>() / n;
            let mut native: Vec<(Class, f64)> = vec![];
            for c in [Class::MulR, Class::Lookup] {
                let tot: f64 = samples
                    .iter()
                    .flat_map(|x| x.native.iter().filter(|(cc, _)| *cc == c))
                    .map(|(_, v)| v)
                    .sum();
                if samples
                    .iter()
                    .any(|x| x.native.iter().any(|(cc, _)| *cc == c))
                {
                    native.push((c, tot / n));
                }
            }
            let on_j = name.starts_with("ic_jc");
            let arm = Arm {
                group_ops: ops,
                native,
                lower_bound: true,
            };
            let mut rat0: Vec<f64> = vec![];
            let mut rat01: Vec<f64> = vec![];
            let mut rat1: Vec<f64> = vec![];
            for (si, s) in per_seed.iter().enumerate() {
                if let Some(x) = s.iter().find(|x| x.name == name) {
                    let la: f64 = x
                        .native
                        .iter()
                        .filter(|(c, _)| *c == Class::MulR)
                        .map(|(_, v)| v)
                        .sum();
                    rat0.push(x.ops / old_rho_ops[si]);
                    rat01.push((x.ops + 0.1 * la) / old_rho_ops[si]);
                    rat1.push((x.ops + la) / old_rho_ops[si]);
                }
            }
            let rec_ratio = [median(&mut rat0), median(&mut rat01), median(&mut rat1)];
            let med_ops = median(&mut samples.iter().map(|x| x.ops).collect::<Vec<_>>());
            arms.insert(
                name.to_string(),
                entry(name, &arm, if on_j { t_add_jold } else { t_add_e }, &cal.times, &refs, json!({
                    "seeds": samples.len(), "certified": samples.iter().filter(|x| x.certified).count(),
                    "median_ops": med_ops,
                    "recorded_median_ratio_to_old_plain_rho": rec_ratio,
                    "recorded_class": classify(&rec_ratio, &[1.0; 3], false),
                    "native_not_emitted": if name.contains("first_collision") || name.starts_with("birthday") {
                        json!(["hash/table work of the collision search", "relation-phase field work"])
                    } else { json!(["relation-phase field work (square tests, roots, Cantor reductions)"]) },
                    "group": if on_j { "J(C), Cantor" } else { "E1" },
                })),
            );
        }
        for label in ["A", "B", "C"] {
            let rs: Vec<(IcResult, bool)> = (0..a.cover_ic_seeds)
                .into_par_iter()
                .map(|s| {
                    let res = ic_on_rung(&inst, arm_of(label), 500 + s, a.max_trials);
                    let good = res.recovered.is_some_and(|k| {
                        res.certified
                            && rtl_ok(&h, hg, ht, k)
                            && certify_dlog(&b.jac, &dgs, &dts, k).0
                    });
                    (res, good)
                })
                .collect();
            let m = ic_mean(&rs);
            ok &= m.certified == m.runs && m.exhausted == 0;
            arms.insert(
                label.into(),
                entry(label, &m.arm, t_add_h, &cal.times, &refs, m.json),
            );
        }
        let old_mean = old_rho_ops.iter().sum::<f64>() / old_rho_ops.len() as f64;
        println!("cover p={p}: done in {:.1}s", t0.elapsed().as_secs_f64());
        rows.push(json!({
            "target": "cover_pushforward", "p": p, "r": r, "candidate": ci.candidate,
            "e1_order": e1.order(), "e2_order": b.e2.order(), "jac_order": jac_order,
            "gate_reproduces_record": gate_row, "calibration_skipped": cal.skipped,
            "class_times_ns": serde_json::to_value(cal.times).unwrap(),
            "t_add_ns": {"E1": t_add_e, "J_recorded_impl": t_add_jold, "J_word_impl": t_add_h},
            "old_plain_rho_ops_per_seed": old_rho_ops, "old_plain_rho_ops_mean": old_mean,
            "rho": rho_block(&rneg, &rplain, &e1),
            "arms": arms, "wall_s": t0.elapsed().as_secs_f64(),
        }));
    }
    (Value::Array(rows), ok && gate_ok)
}

// ── tower ────────────────────────────────────────────────────────────────

struct TowerRun {
    g: u32,
    arm: &'static str,
    ic: Option<(IcResult, bool)>,
    old: Option<(ic::Ic64Outcome, bool)>,
}

struct TowerJob {
    p: u64,
    inst: usize,
    r: u64,
    n_e: u64,
    jac_order: [u64; 2],
    skipped: Vec<(u32, String)>,
    runs: Vec<TowerRun>,
    old_rho: Vec<(f64, f64)>,
    rho_neg: Vec<RhoFull>,
    rho_plain: Vec<RhoFull>,
    wall_s: f64,
    transfer_wall_s: f64,
    j_invariant: u64,
    aut: Option<&'static str>,
    keep: Option<Tower64>,
}

fn old_native(o: &ic::Ic64Outcome) -> Vec<(Class, f64)> {
    let c = |n: &str| o.counter(n) as f64;
    vec![
        (
            Class::MulR,
            o.la_mulmods() as f64 + c("large_prime_mulmods_uncharged"),
        ),
        (Class::InvR, c("linalg_inversions_uncharged")),
        (
            Class::MulP,
            c("oracle_fp_muls_uncharged") + c("factor_base_fp_muls_uncharged"),
        ),
        (
            Class::Lookup,
            c("factor_base_lookups_uncharged") + c("large_prime_lookups_uncharged"),
        ),
        (Class::Canon, c("canonicalisations_uncharged")),
        (Class::Draw, c("step_draws_uncharged")),
    ]
}

fn run_tower(a: &Args) -> (Value, bool) {
    let primes: Vec<u64> = a
        .tower_primes
        .split(',')
        .filter_map(|x| x.trim().parse().ok())
        .collect();
    let recorded: Option<Value> = std::fs::read_to_string(
        "research/chained_prime_transfer_20261111/tower_large/tower_large.json",
    )
    .ok()
    .and_then(|s| serde_json::from_str(&s).ok());
    let mut jobs: Vec<(u64, usize)> = primes
        .iter()
        .flat_map(|&p| (0..a.tower_instances).map(move |i| (p, i)))
        .collect();
    jobs.sort_by(|x, y| y.0.cmp(&x.0).then(x.1.cmp(&y.1)));
    let next = AtomicUsize::new(0);
    let done: Mutex<Vec<TowerJob>> = Mutex::new(vec![]);
    let params = crosscheck::ic_params();
    std::thread::scope(|s| {
        for _ in 0..a.threads.max(1) {
            s.spawn(|| loop {
                let j = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(p, inst)) = jobs.get(j) else { break };
                let t0 = Instant::now();
                let seed = 1000 + 17 * inst as u64 + p % 997;
                let Some(t) = build64(p, seed, p / 4, 64) else {
                    println!("tower p={p} inst {inst}: no tower instance");
                    continue;
                };
                let mut job = TowerJob {
                    p, inst, r: t.r, n_e: t.e.order(),
                    jac_order: [t.rungs[0].jac_order, t.rungs[1].jac_order],
                    skipped: vec![], runs: vec![], old_rho: vec![],
                    rho_neg: vec![], rho_plain: vec![], wall_s: 0.0,
                    transfer_wall_s: t.transfer_wall_s,
                    j_invariant: t.e.j_invariant(), aut: aut_kind(&t.e), keep: None,
                };
                let eg = EcGroup(t.e);
                for s in 0..a.tower_rho_seeds {
                    let rho = rho_reference(&eg, t.gen, t.target, t.r, 100 + s, 1 << 26);
                    job.old_rho.push((
                        (rho.group_ops.adds + rho.group_ops.doubles) as f64,
                        f64::from(u8::from(rho.recovered.is_some_and(|k| t.e.mul(&t.gen, k as u128) == t.target))),
                    ));
                    job.rho_neg.push(rho_full_ec(&t.e, t.gen, t.target, t.r, 100 + s, true));
                    job.rho_plain.push(rho_full_ec(&t.e, t.gen, t.target, t.r, 100 + s, false));
                }
                for rg in t.rungs.iter() {
                    let g = rg.cover.genus;
                    let curve = &rg.cover.curve;
                    // recorded arm: full factor base, ic_params, seeds 300 + s
                    for s in 0..a.tower_old_seeds {
                        match ic::solve(curve, rg.d, rg.t, t.r, rg.jac_order, 300 + s, &params) {
                            Ok(out) => {
                                let good = out.recovered == Some(t.x)
                                    && out.recovered.is_some_and(|k| rtl_ok(curve, rg.d, rg.t, k));
                                job.runs.push(TowerRun { g, arm: "recorded", ic: None, old: Some((out, good)) });
                            }
                            Err(e) => {
                                println!("tower p={p} g={g}: recorded arm: {e}");
                                job.runs.push(TowerRun { g, arm: "recorded", ic: None, old: None });
                            }
                        }
                    }
                    let ins = match instance_for_rung(curve, rg.jac_order, t.r, rg.d, rg.t, t.x) {
                        Ok(i) => i,
                        Err(e) => {
                            job.skipped.push((g, e));
                            continue;
                        }
                    };
                    for label in ["A", "B", "C"] {
                        for s in 0..a.tower_seeds {
                            let res = ic_on_rung(&ins, arm_of(label), 300 + s, a.max_trials);
                            let good = res.certified
                                && res.recovered.is_some_and(|k| rtl_ok(curve, rg.d, rg.t, k));
                            job.runs.push(TowerRun {
                                g,
                                arm: match label { "A" => "A", "B" => "B", _ => "C" },
                                ic: Some((res, good)),
                                old: None,
                            });
                        }
                    }
                }
                job.wall_s = t0.elapsed().as_secs_f64();
                println!(
                    "tower p={p} inst {inst}: r={} #E={} done in {:.1}s (transfer {:.1}s), skipped {:?}",
                    job.r, job.n_e, job.wall_s, job.transfer_wall_s, job.skipped
                );
                if inst == 0 {
                    job.keep = Some(t);
                }
                done.lock().unwrap().push(job);
            });
        }
    });
    let mut jobs = done.into_inner().unwrap();
    jobs.sort_by_key(|j| (j.p, j.inst));
    let (mut ok, mut gate_ok) = (true, true);
    let mut rows = vec![];
    for &p in &primes {
        let js: Vec<&TowerJob> = jobs.iter().filter(|j| j.p == p).collect();
        if js.is_empty() {
            ok = false;
            continue;
        }
        // calibration on instance 0, sequentially, after the parallel work
        let t0 = js.iter().find_map(|j| j.keep.as_ref());
        let (cal, t_add_e, t_add_g) = if let Some(t) = t0 {
            let cal = calibrate(a.no_calibration, p, t.r, &t.e, t.gen);
            let t_add_e = t_add_or_unit(a.no_calibration, {
                let pts: Vec<_> = (1..=64u128).map(|m| t.e.mul(&t.gen, m * 7)).collect();
                let mut acc = None;
                let mut i = 0usize;
                let e = t.e;
                move || {
                    acc = e.add(&acc, &pts[i % 64]);
                    i += 1;
                    std::hint::black_box(&acc);
                }
            });
            let mut tg = [1.0f64; 2];
            for (gi, rg) in t.rungs.iter().enumerate() {
                let c = &rg.cover.curve;
                tg[gi] = t_add_or_unit(a.no_calibration, {
                    let ring: Vec<H64Div> = (1..=64u64).map(|m| c.mul_rtl(rg.d, m * 7)).collect();
                    let mut acc = ring[0];
                    let mut i = 0usize;
                    move || {
                        acc = c.cantor(acc, ring[i % 64]);
                        i += 1;
                        std::hint::black_box(&acc);
                    }
                });
            }
            (cal, t_add_e, tg)
        } else {
            ok = false;
            continue;
        };
        let neg: Vec<RhoFull> = js.iter().flat_map(|j| j.rho_neg.iter().cloned()).collect();
        let plain: Vec<RhoFull> = js
            .iter()
            .flat_map(|j| j.rho_plain.iter().cloned())
            .collect();
        ok &= neg.iter().chain(&plain).all(|r| r.certified);
        let old_rho_mean = js
            .iter()
            .map(|j| j.old_rho.iter().map(|x| x.0).sum::<f64>() / j.old_rho.len().max(1) as f64)
            .sum::<f64>()
            / js.len() as f64;
        ok &= js.iter().all(|j| j.old_rho.iter().all(|x| x.1 == 1.0));
        let refs = refs_of(&neg, &plain, t_add_e, cal.times);
        let mut arms = serde_json::Map::new();
        for (gi, g) in [2u32, 3].into_iter().enumerate() {
            // recorded arm
            let olds: Vec<&(ic::Ic64Outcome, bool)> = js
                .iter()
                .flat_map(|j| j.runs.iter())
                .filter(|r| r.g == g && r.arm == "recorded")
                .filter_map(|r| r.old.as_ref())
                .collect();
            let n_old_attempts = js
                .iter()
                .flat_map(|j| j.runs.iter())
                .filter(|r| r.g == g && r.arm == "recorded")
                .count();
            if !olds.is_empty() {
                let n = olds.len() as f64;
                let ops = olds.iter().map(|(o, _)| o.group_ops() as f64).sum::<f64>() / n;
                let mut native: Vec<(Class, f64)> = vec![];
                for c in [
                    Class::MulR,
                    Class::InvR,
                    Class::MulP,
                    Class::Lookup,
                    Class::Canon,
                    Class::Draw,
                ] {
                    let tot: f64 = olds
                        .iter()
                        .flat_map(|(o, _)| old_native(o))
                        .filter(|(cc, _)| *cc == c)
                        .map(|(_, v)| v)
                        .sum();
                    native.push((c, tot / n));
                }
                let la: f64 = olds.iter().map(|(o, _)| o.la_mulmods() as f64).sum::<f64>() / n;
                let rr = recorded_ratio(ops, la, old_rho_mean);
                let arm = Arm {
                    group_ops: ops,
                    native,
                    lower_bound: false,
                };
                let cert = olds.iter().filter(|(_, c)| *c).count();
                ok &= cert == olds.len() && olds.len() == n_old_attempts;
                arms.insert(
                    format!("g{g}_recorded"),
                    entry(&format!("g{g}_recorded"), &arm, t_add_g[gi], &cal.times, &refs, json!({
                        "runs": olds.len(), "certified": cert,
                        "recorded_ratio_to_old_plain_rho_la_priced": rr,
                        "recorded_class": classify(&rr, &[1.0; 3], false),
                        "la_mulmods": la,
                        "counters_listed": ["linalg_mulmods", "linalg_inversions", "oracle_fp_muls", "factor_base_fp_muls", "factor_base_lookups", "canonicalisations", "step_draws", "large_prime_*"],
                    })),
                );
            }
            for label in ["A", "B", "C"] {
                let rs: Vec<(IcResult, bool)> = js
                    .iter()
                    .flat_map(|j| j.runs.iter())
                    .filter(|r| r.g == g && r.arm == label)
                    .filter_map(|r| r.ic.clone())
                    .collect();
                if rs.is_empty() {
                    continue;
                }
                let m = ic_mean(&rs);
                ok &= m.certified == m.runs && m.exhausted == 0;
                arms.insert(
                    format!("g{g}_{label}"),
                    entry(
                        &format!("g{g}_{label}"),
                        &m.arm,
                        t_add_g[gi],
                        &cal.times,
                        &refs,
                        m.json,
                    ),
                );
            }
        }
        // gate: instance identity (and the means where the seed counts agree)
        let mut gate_row = true;
        if let Some(rec) = recorded.as_ref().and_then(|r| r["instances"].as_array()) {
            for j in &js {
                match rec.iter().find(|x| {
                    x["p"].as_u64() == Some(p) && x["instance"].as_u64() == Some(j.inst as u64)
                }) {
                    Some(x) => {
                        let same = x["r"].as_u64() == Some(j.r)
                            && x["n_e"].as_u64() == Some(j.n_e)
                            && x["jac_order"][0].as_u64() == Some(j.jac_order[0])
                            && x["jac_order"][1].as_u64() == Some(j.jac_order[1]);
                        gate_row &= same;
                        if a.tower_rho_seeds == 64 {
                            let m = j.old_rho.iter().map(|y| y.0).sum::<f64>() / 64.0;
                            let want = x["rho_ops"].as_f64().unwrap_or(f64::NAN);
                            if (m - want).abs() > 1e-9 * want.abs() {
                                gate_row = false;
                                println!(
                                    "tower p={p} inst {}: old rho mean {m} != recorded {want}",
                                    j.inst
                                );
                            }
                        }
                        if a.tower_old_seeds == 8 {
                            for (gi, g) in [2u32, 3].into_iter().enumerate() {
                                let os: Vec<&(ic::Ic64Outcome, bool)> = j
                                    .runs
                                    .iter()
                                    .filter(|r| r.g == g && r.arm == "recorded")
                                    .filter_map(|r| r.old.as_ref())
                                    .collect();
                                if os.len() == 8 {
                                    let acc = os.iter().fold(0.0, |s, (o, _)| {
                                        s + o.group_ops() as f64 * (1.0 / 8.0)
                                    });
                                    let want = x["ic"][gi]["ops"].as_f64().unwrap_or(f64::NAN);
                                    if (acc - want).abs() > 1e-6 * want.abs() {
                                        gate_row = false;
                                        println!("tower p={p} inst {} g{g}: old ic ops {acc} != recorded {want}", j.inst);
                                    }
                                }
                            }
                        }
                    }
                    None => gate_row = false,
                }
            }
        } else {
            gate_row = false;
        }
        gate_ok &= gate_row;
        println!(
            "tower p={p}: gate {}",
            if gate_row {
                "reproduces the record"
            } else {
                "MISMATCH or not comparable"
            }
        );
        let r_mean = js.iter().map(|j| j.r as f64).sum::<f64>() / js.len() as f64;
        rows.push(json!({
            "target": "tower_large", "p": p, "instances": js.len(), "r_mean": r_mean,
            "gate_reproduces_record": gate_row, "calibration_skipped": cal.skipped,
            "class_times_ns": serde_json::to_value(cal.times).unwrap(),
            "t_add_ns": {"E": t_add_e, "J_g2": t_add_g[0], "J_g3": t_add_g[1]},
            "old_plain_rho_ops_mean": old_rho_mean,
            "skipped": js.iter().flat_map(|j| j.skipped.iter().map(move |(g, e)| json!({"inst": j.inst, "genus": g, "reason": e}))).collect::<Vec<_>>(),
            "curves_checked": js.iter().map(|j| json!({"inst": j.inst, "j_invariant": j.j_invariant, "automorphism_beyond_negation": j.aut})).collect::<Vec<_>>(),
            "transfer_wall_s_mean_one_time": js.iter().map(|j| j.transfer_wall_s).sum::<f64>() / js.len() as f64,
            "rho": rho_block(&neg, &plain, &t0.expect("kept").e),
            "arms": arms,
            "wall_s_max_instance": js.iter().map(|j| j.wall_s).fold(0.0, f64::max),
        }));
    }
    (Value::Array(rows), ok && gate_ok)
}

// ── merge and figure ─────────────────────────────────────────────────────

fn arm_ratio0(row: &Value, arm: &str, key: &str, i: usize) -> Option<f64> {
    row["arms"][arm][key][i].as_f64()
}

fn render_svg(targets: &serde_json::Map<String, Value>) -> String {
    // Distance of selected arms to the negation rho at w = 0 and at w = 1.
    type Series = (
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        usize,
    );
    let series: [Series; 8] = [
        ("v3", "recorded", "#c0392b", "v3 recorded arm", 0),
        ("v3", "C", "#e67e22", "v3 arm C", 0),
        ("cover_pushforward", "ic_jc", "#8e44ad", "cover ic_jc", 0),
        ("cover_pushforward", "C", "#2980b9", "cover arm C", 0),
        (
            "tower_large",
            "g2_recorded",
            "#27ae60",
            "tower g2 recorded",
            0,
        ),
        ("tower_large", "g2_C", "#16a085", "tower g2 arm C", 0),
        (
            "tower_large",
            "g3_recorded",
            "#7f8c8d",
            "tower g3 recorded",
            0,
        ),
        ("tower_large", "g3_C", "#2c3e50", "tower g3 arm C", 0),
    ];
    let mut pts: Vec<Vec<(f64, f64)>> = vec![];
    for (tg, arm, _, _, i) in &series {
        let mut v = vec![];
        if let Some(rows) = targets.get(*tg).and_then(|t| t["rows"].as_array()) {
            for row in rows {
                if let (Some(p), Some(y)) = (
                    row["p"].as_f64(),
                    arm_ratio0(row, arm, "ratio_neg_band", *i),
                ) {
                    v.push((p, y));
                }
            }
        }
        pts.push(v);
    }
    let all: Vec<(f64, f64)> = pts.iter().flatten().copied().collect();
    if all.is_empty() {
        return String::from(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"10\" height=\"10\"/>\n",
        );
    }
    let (xmin, xmax) = (
        all.iter().map(|a| a.0).fold(f64::MAX, f64::min) / 1.3,
        all.iter().map(|a| a.0).fold(0.0, f64::max) * 1.3,
    );
    let ymin = 10f64.powf(
        all.iter()
            .map(|a| a.1)
            .fold(f64::MAX, f64::min)
            .log10()
            .floor()
            .min(0.0),
    );
    let ymax = 10f64.powf(all.iter().map(|a| a.1).fold(0.0, f64::max).log10().ceil());
    let (x0, x1, y0, y1) = (xmin.ln(), xmax.ln(), ymin.ln(), ymax.ln());
    let xp = |p: f64| 80.0 + (p.ln() - x0) / (x1 - x0) * 520.0;
    let yp = |v: f64| 360.0 - (v.max(ymin).ln() - y0) / (y1 - y0) * 300.0;
    let mut s = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"660\" height=\"470\" viewBox=\"0 0 660 470\" font-family=\"Helvetica,Arial,sans-serif\" font-size=\"12\">\n<rect width=\"660\" height=\"470\" fill=\"#fff\"/>\n");
    s.push_str("<text x=\"330\" y=\"24\" text-anchor=\"middle\" font-size=\"15\" font-weight=\"bold\">IC arm total over negation-map rho total, w = 0 (group operations)</text>\n");
    s.push_str("<text x=\"330\" y=\"42\" text-anchor=\"middle\" fill=\"#555\">ratio 1 = rho; every arm above it; [pcc:unverified]</text>\n");
    let mut t = ymin;
    while t <= ymax * 1.0001 {
        let y = yp(t);
        s.push_str(&format!("<line x1=\"80\" y1=\"{y:.1}\" x2=\"600\" y2=\"{y:.1}\" stroke=\"#e5e5e5\"/><text x=\"74\" y=\"{:.1}\" text-anchor=\"end\" fill=\"#444\">{t}</text>\n", y + 4.0));
        t *= 10.0;
    }
    let y1r = yp(1.0);
    s.push_str(&format!("<line x1=\"80\" y1=\"{y1r:.1}\" x2=\"600\" y2=\"{y1r:.1}\" stroke=\"#000\" stroke-dasharray=\"4,3\"/>\n"));
    for (k, (_, _, col, label, _)) in series.iter().enumerate() {
        let path: Vec<String> = pts[k]
            .iter()
            .map(|(p, v)| format!("{:.1},{:.1}", xp(*p), yp(*v)))
            .collect();
        if path.is_empty() {
            continue;
        }
        s.push_str(&format!(
            "<polyline fill=\"none\" stroke=\"{col}\" stroke-width=\"2\" points=\"{}\"/>\n",
            path.join(" ")
        ));
        let ly = 392 + 16 * (k / 2);
        let lx = 90 + 300 * (k % 2);
        s.push_str(&format!("<line x1=\"{lx}\" y1=\"{ly}\" x2=\"{}\" y2=\"{ly}\" stroke=\"{col}\" stroke-width=\"2\"/><text x=\"{}\" y=\"{}\">{label}</text>\n", lx + 24, lx + 30, ly + 4));
    }
    s.push_str("<text x=\"340\" y=\"380\" text-anchor=\"middle\" fill=\"#444\">field size p (log)</text>\n</svg>\n");
    s
}

fn run_merge(a: &Args) -> bool {
    let mut targets = serde_json::Map::new();
    let mut all_ok = true;
    for (key, base) in [
        ("v3", "v3"),
        ("cover_pushforward", "cover"),
        ("tower_large", "tower"),
    ] {
        let mut rows: Vec<Value> = vec![];
        let mut parts: Vec<String> = vec![];
        let mut ok = true;
        let dir = std::fs::read_dir(&a.out_dir).expect("out dir");
        let mut files: Vec<String> = dir
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|n| n.starts_with(&format!("remeasure_{base}")) && n.ends_with(".json"))
            .collect();
        files.sort();
        let mut aut_rows: Vec<Value> = vec![];
        for f in files {
            let Ok(txt) = std::fs::read_to_string(format!("{}/{}", a.out_dir, f)) else {
                continue;
            };
            let Ok(v) = serde_json::from_str::<Value>(&txt) else {
                continue;
            };
            if v["section"].as_str() != Some(base) {
                continue;
            }
            // `_aut` parts only list the curves (j-invariants) of every instance.
            if f.contains("_aut") {
                if let Some(r) = v["rows"].as_array() {
                    aut_rows.extend(r.iter().cloned());
                }
                parts.push(f);
                continue;
            }
            ok &= v["ok"].as_bool().unwrap_or(false);
            if let Some(r) = v["rows"].as_array() {
                rows.extend(r.iter().cloned());
            }
            parts.push(f);
        }
        for row in rows.iter_mut() {
            if let Some(ar) = aut_rows.iter().find(|x| x["p"] == row["p"]) {
                row["curves_checked_all"] = ar["curves_checked"].clone();
            }
        }
        rows.sort_by_key(|r| r["p"].as_u64().unwrap_or(0));
        all_ok &= ok && !rows.is_empty();
        targets.insert(key.into(), json!({"parts": parts, "ok": ok, "rows": rows}));
    }
    let doc = json!({
        "schema": "negation.remeasure/v1",
        "protocol": "research/negation_remeasure_20261113/PROTOCOL.md",
        "unit": "group operations of the arm's own group (1 GAE each); native counters priced at the band and at the calibrated weight",
        "bands": BAND, "calibrated_factors": CAL_FACTORS,
        "calibration": calibration_meta(),
        "all_ok": all_ok,
        "targets": Value::Object(targets.clone()),
    });
    std::fs::write(
        format!("{}/remeasure.json", a.out_dir),
        serde_json::to_string_pretty(&doc).unwrap(),
    )
    .expect("write");
    std::fs::write(format!("{}/remeasure.svg", a.out_dir), render_svg(&targets))
        .expect("write svg");
    println!("merged: all_ok = {all_ok}");
    all_ok
}

fn main() -> ExitCode {
    let a = Args::parse();
    rayon::ThreadPoolBuilder::new()
        .num_threads(a.threads.max(1))
        .build_global()
        .ok();
    let sections: Vec<&str> = if a.sections.is_empty() {
        vec!["v3", "cover", "tower", "merge"]
    } else {
        a.sections.iter().map(String::as_str).collect()
    };
    let mut all_ok = true;
    for sec in sections {
        let t0 = Instant::now();
        match sec {
            "v3" | "cover" | "tower" => {
                let (rows, ok) = match sec {
                    "v3" => run_v3(&a),
                    "cover" => run_cover(&a),
                    _ => run_tower(&a),
                };
                all_ok &= ok;
                write_part(
                    &a,
                    sec,
                    json!({
                        "section": sec, "ok": ok, "rows": rows, "wall_s": t0.elapsed().as_secs_f64(),
                        "calibration": calibration_meta(), "calibration_skipped": a.no_calibration,
                        "args": format!("{a:?}"),
                    }),
                );
            }
            "merge" => all_ok &= run_merge(&a),
            other => {
                eprintln!("unknown section {other}");
                all_ok = false;
            }
        }
        println!("section {sec}: {:.1}s", t0.elapsed().as_secs_f64());
    }
    if all_ok {
        ExitCode::SUCCESS
    } else {
        eprintln!("FAILED: an uncertified log or a gate mismatch (see the log)");
        ExitCode::FAILURE
    }
}
