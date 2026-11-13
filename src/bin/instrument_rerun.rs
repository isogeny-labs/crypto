//! `instrument_rerun`: the chained-transfer measurements rerun with the fixed
//! instrument (`chained_transfer::instrument`).
//!
//! ```text
//! instrument_rerun [v3] [tower] [smallde] [--out-dir DIR] [--json rerun.json]
//!                  [--tower-primes 251,...,16001] [--threads 4]
//! ```
//!
//! Protocol (frozen by hash): `research/instrument_fixes_20261112/PROTOCOL.md`.
//! Subcommands default to all three.  The binary writes new values only; the
//! old-versus-new join is `render_rerun.py`.  Every figure is
//! `[pcc:unverified]` until a validator replays it.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use clap::Parser;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde_json::{json, Value};

use crypto_lib::cryptanalysis::chained_transfer::bielliptic::{Bielliptic, Quotient};
use crypto_lib::cryptanalysis::chained_transfer::certify;
use crypto_lib::cryptanalysis::chained_transfer::chain::{Chain, Elt, Hop};
use crypto_lib::cryptanalysis::chained_transfer::ec::{Curve, Pt, VeluIsogeny};
use crypto_lib::cryptanalysis::chained_transfer::hyper64::build64;
use crypto_lib::cryptanalysis::chained_transfer::ic_control::{Arm, IcResult};
use crypto_lib::cryptanalysis::chained_transfer::instrument::{
    band_class, convert_div, crosscheck_genus2, genus2_to_h64, ic_on_rung, instance_for_rung,
    legacy_matrix_size, mean_se, rho_on_curve, rho_split_prime, solve_linear_counted, RhoRef,
    W_LA_CAL, W_REL_CAL,
};
use crypto_lib::cryptanalysis::chained_transfer::measure::{
    genus2_collect_rows, genus2_index_calculus, rho_on_ec, solve_linear_free,
};
use crypto_lib::cryptanalysis::chained_transfer::small_de::{
    construct, endo_for, fb_aut, fb_interval, fb_layered, relation_phase, scalar_endo,
    solve_target, EndoKind, Fb, Inst, RhoKind,
};
use crypto_lib::cryptanalysis::genus2_fp::Genus2Curve;
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimePoint};

const W: [f64; 3] = [0.0, 0.1, 1.0];

#[derive(Parser, Debug)]
#[command(about = "Chained-transfer measurements rerun with the fixed instrument")]
struct Args {
    /// Sections to run: v3, tower, smallde (default: all).
    sections: Vec<String>,
    #[arg(long, default_value = "research/instrument_fixes_20261112")]
    out_dir: String,
    #[arg(long, default_value = "rerun.json")]
    json: String,
    #[arg(long, default_value = "251,503,1009,2003,4001,8009,16001")]
    tower_primes: String,
    #[arg(long, default_value_t = 3)]
    tower_instances: usize,
    #[arg(long, default_value_t = 4)]
    tower_seeds: u64,
    #[arg(long, default_value_t = 64)]
    tower_rho_seeds: u64,
    #[arg(long, default_value_t = 8)]
    v3_seeds: u64,
    #[arg(long, default_value_t = 32)]
    v3_rho_seeds: u64,
    #[arg(long, default_value = "24,28")]
    sd_sizes: String,
    #[arg(long, default_value = "24:6,28:3")]
    sd_reps: String,
    #[arg(long, default_value_t = 16)]
    sd_targets: usize,
    #[arg(long, default_value_t = 20261112)]
    sd_seed: u64,
    #[arg(long, default_value_t = 4)]
    threads: usize,
    #[arg(long, default_value_t = 40_000_000)]
    max_trials: u64,
}

// ── aggregates ───────────────────────────────────────────────────────────

#[derive(Clone, Default)]
struct IcAgg {
    runs: usize,
    certified: usize,
    exhausted: usize,
    group_ops: f64,
    group_ops_se: f64,
    setup_ops: f64,
    walk_ops: f64,
    verify_ops: f64,
    rel_mul: f64,
    rel_add: f64,
    la_mul: f64,
    la_add: f64,
    trials: f64,
    relations: f64,
    columns: f64,
    core_cols: f64,
    attempts: f64,
}

fn agg_ic(rs: &[(IcResult, bool)]) -> IcAgg {
    let n = rs.len().max(1) as f64;
    let m = |f: &dyn Fn(&IcResult) -> f64| rs.iter().map(|(r, _)| f(r)).sum::<f64>() / n;
    let go: Vec<f64> = rs.iter().map(|(r, _)| r.group_ops() as f64).collect();
    IcAgg {
        runs: rs.len(),
        certified: rs.iter().filter(|(_, c)| *c).count(),
        exhausted: rs.iter().filter(|(r, _)| r.exhausted).count(),
        group_ops: m(&|r| r.group_ops() as f64),
        group_ops_se: mean_se(&go).1,
        setup_ops: m(&|r| r.setup_ops as f64),
        walk_ops: m(&|r| r.walk_ops as f64),
        verify_ops: m(&|r| r.verify_ops as f64),
        rel_mul: m(&|r| r.rel.mul as f64),
        rel_add: m(&|r| r.rel.add as f64),
        la_mul: m(&|r| r.la.mul as f64),
        la_add: m(&|r| r.la.add as f64),
        trials: m(&|r| r.trials as f64),
        relations: m(&|r| r.relations as f64),
        columns: m(&|r| r.fb_cols as f64),
        core_cols: m(&|r| r.core_cols as f64),
        attempts: m(&|r| f64::from(r.attempts)),
    }
}

impl IcAgg {
    fn total(&self, w: f64) -> f64 {
        self.group_ops + w * (self.rel_mul + self.la_mul)
    }
    fn total_cal(&self) -> f64 {
        self.group_ops + W_REL_CAL * self.rel_mul + W_LA_CAL * self.la_mul
    }
    fn total_with_adds(&self, w: f64) -> f64 {
        self.group_ops + w * (self.rel_mul + self.rel_add + self.la_mul + self.la_add)
    }
    fn json(&self) -> Value {
        json!({
            "runs": self.runs, "certified": self.certified, "exhausted": self.exhausted,
            "group_ops": self.group_ops, "group_ops_se": self.group_ops_se,
            "setup_ops": self.setup_ops, "walk_ops": self.walk_ops, "verify_ops": self.verify_ops,
            "rel_mul": self.rel_mul, "rel_add": self.rel_add,
            "la_mul": self.la_mul, "la_add": self.la_add,
            "trials": self.trials, "relations": self.relations, "columns": self.columns,
            "core_cols": self.core_cols, "attempts": self.attempts,
            "total": W.map(|w| self.total(w)), "total_cal": self.total_cal(),
            "total_w1_with_adds": self.total_with_adds(1.0),
        })
    }
}

#[derive(Clone, Default)]
struct RhoAgg {
    runs: usize,
    certified: usize,
    total_ops: f64,
    total_ops_se: f64,
    walk_ops: f64,
    other_ops: f64,
    native: f64,
    steps: f64,
}

fn agg_rho(rs: &[RhoRef]) -> RhoAgg {
    let n = rs.len().max(1) as f64;
    let m = |f: &dyn Fn(&RhoRef) -> f64| rs.iter().map(f).sum::<f64>() / n;
    let tot: Vec<f64> = rs.iter().map(|r| r.total_ops as f64).collect();
    RhoAgg {
        runs: rs.len(),
        certified: rs.iter().filter(|r| r.certified).count(),
        total_ops: m(&|r| r.total_ops as f64),
        total_ops_se: mean_se(&tot).1,
        walk_ops: m(&|r| r.walk_ops as f64),
        other_ops: m(&|r| r.other_ops as f64),
        native: m(&|r| r.native as f64),
        steps: m(&|r| r.steps as f64),
    }
}

impl RhoAgg {
    fn total(&self, w: f64) -> f64 {
        self.total_ops + w * self.native
    }
    fn walk_only(&self, w: f64) -> f64 {
        self.walk_ops + w * self.native
    }
    fn json(&self) -> Value {
        json!({
            "runs": self.runs, "certified": self.certified,
            "total_ops": self.total_ops, "total_ops_se": self.total_ops_se,
            "walk_ops": self.walk_ops, "other_ops": self.other_ops,
            "native": self.native, "steps": self.steps,
            "walk_share": self.walk_ops / self.total_ops.max(1.0),
            "total": W.map(|w| self.total(w)),
            "walk_only": W.map(|w| self.walk_only(w)),
        })
    }
}

/// Distances of one IC arm to the matched rho references.
fn compare(ic: &IcAgg, neg: &RhoAgg, plain: &RhoAgg) -> Value {
    let it = W.map(|w| ic.total(w));
    let nt = W.map(|w| neg.total(w));
    let pt = W.map(|w| plain.total_ops + w * 0.0);
    let nw = W.map(|w| neg.walk_only(w));
    let r = |a: [f64; 3], b: [f64; 3]| [a[0] / b[0], a[1] / b[1], a[2] / b[2]];
    let neg_cal = neg.total_ops + W_REL_CAL * neg.native;
    json!({
        "ic_total": it, "ic_total_cal": ic.total_cal(),
        "ratio_neg": r(it, nt), "ratio_neg_cal": ic.total_cal() / neg_cal,
        "ratio_plain": r(it, pt), "ratio_neg_walk": r(it, nw),
        "class_neg": band_class(&it, &nt),
        "class_plain": band_class(&it, &pt),
        "class_neg_walk": band_class(&it, &nw),
        "order_at_0_vs_neg": if it[0] < nt[0] { "ic below rho" } else { "ic above rho" },
        "order_at_0_vs_plain": if it[0] < pt[0] { "ic below rho" } else { "ic above rho" },
    })
}

fn arm_of(label: &str) -> Arm {
    match label {
        "A" => Arm::Full,
        "B" => Arm::SingleLp,
        _ => Arm::DoubleLp,
    }
}

// ── v3 instances (copy of chained_transfer_bench's candidate search) ─────

fn largest_cyclic_prime(mut n: u64) -> u64 {
    let mut best = 1u64;
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            let mut e = 0;
            while n.is_multiple_of(d) {
                n /= d;
                e += 1;
            }
            if e == 1 {
                best = best.max(d);
            }
        }
        d += 1;
    }
    if n > 1 {
        best = best.max(n);
    }
    best
}

fn ec_instance(e: &Curve, r: u64, x: u64) -> Option<(Pt, Pt)> {
    let order = e.order();
    if !order.is_multiple_of(r) {
        return None;
    }
    let cof = order / r;
    for s in 1..5000u64 {
        let g = e.mul(&e.point_from_seed(s.wrapping_mul(0x9E37_79B9)), cof as u128);
        if g.is_some() && e.mul(&g, r as u128).is_none() {
            return Some((g, e.mul(&g, x as u128)));
        }
    }
    None
}

fn find_isogeny(e: &Curve, r: u64) -> Option<(VeluIsogeny, Pt)> {
    let order = e.order();
    for ell in [2u64, 3, 5, 7] {
        if !order.is_multiple_of(ell) || r == ell {
            continue;
        }
        let cof = order / ell;
        for s in 1..2000u64 {
            let g = e.mul(
                &e.point_from_seed(s.wrapping_mul(2_654_435_761)),
                cof as u128,
            );
            if g.is_none() {
                continue;
            }
            if let Some(iso) = VeluIsogeny::from_generator(e, &g) {
                return Some((iso, g));
            }
        }
    }
    None
}

struct V3Inst {
    e1: Curve,
    jac: Genus2Curve,
    r: u64,
    x: u64,
    gen: Pt,
    target: Pt,
    dg: crypto_lib::cryptanalysis::genus2_fp::Divisor,
    dt: crypto_lib::cryptanalysis::genus2_fp::Divisor,
    e1_order: u64,
    e2_order: u64,
    jac_order: u64,
    seed: u64,
}

/// The structural prefix of `chained_transfer_bench::measure_curve`; `Err`
/// exactly where the bench skipped the candidate.
fn v3_candidate(p: u64, c: [u64; 4], seed: u64) -> Result<V3Inst, &'static str> {
    let b = Bielliptic::new(p, c).map_err(|_| "bielliptic_rejected")?;
    let e1 = *b.curve(Quotient::E1);
    let e2 = *b.curve(Quotient::E2);
    let jac = b.jac.clone();
    let (e1_order, e2_order) = (e1.order(), e2.order());
    let jac_order = e1_order * e2_order;
    let r = largest_cyclic_prime(e1_order);
    if r < 50 {
        return Err("r_below_50");
    }
    if e2_order.is_multiple_of(r) || jac_order.is_multiple_of(r * r) {
        return Err("r_not_cyclic");
    }
    let x = (seed.wrapping_mul(2_246_822_519).wrapping_add(7)) % r;
    let (gen, target) = ec_instance(&e1, r, x).ok_or("no_order_r_point")?;
    certify::pt_elem(&gen).map_err(|_| "non_rational_point")?;
    certify::pt_elem(&target).map_err(|_| "non_rational_point")?;
    let (iso, kernel) = find_isogeny(&e1, r).ok_or("no_small_isogeny")?;
    let e_prime = iso.codomain;
    let hop = Hop::isogeny(iso, &gen, &target, r).map_err(|_| "isogeny_hop_failed")?;
    let ch = Chain::new(vec![hop]);
    if !ch.verify(&Elt::E(gen), r, x as u128) {
        return Err("isogeny_chain_unverified");
    }
    let (gp, tp) = match (ch.transport(&Elt::E(gen)), ch.transport(&Elt::E(target))) {
        (Elt::E(a), Elt::E(b)) => (a, b),
        _ => (None, None),
    };
    let _ = (e_prime, kernel);
    certify::pt_elem(&gp).map_err(|_| "non_rational_point")?;
    certify::pt_elem(&tp).map_err(|_| "non_rational_point")?;
    let hop = Hop::pullback(&b, Quotient::E1, &gen, r).map_err(|_| "pullback_hop_failed")?;
    let ch = Chain::new(vec![hop]);
    if !ch.verify(&Elt::E(gen), r, x as u128) {
        return Err("pullback_chain_unverified");
    }
    let (dg, dt) = match (ch.transport(&Elt::E(gen)), ch.transport(&Elt::E(target))) {
        (Elt::J(a), Elt::J(b)) => (a, b),
        _ => return Err("pullback_not_a_divisor"),
    };
    Ok(V3Inst {
        e1,
        jac,
        r,
        x,
        gen,
        target,
        dg,
        dt,
        e1_order,
        e2_order,
        jac_order,
        seed,
    })
}

/// Values the v3 record holds, per prime: candidate, rho on E1, IC group
/// operations, IC trials, LA mulmods.
const V3_RECORDED: [(u64, u64, u64, u64, u64, u64); 4] = [
    (211, 9, 428, 3419, 226, 14633),
    (419, 27, 357, 8475, 542, 51976),
    (821, 2, 370, 11353, 851, 164828),
    (1619, 1, 847, 34521, 1653, 603721),
];

fn run_v3(a: &Args, log: &mut Vec<String>) -> (Value, bool) {
    let primes = [211u64, 419, 821, 1619];
    let mut out = vec![];
    let mut all_ok = true;
    for (i, &p) in primes.iter().enumerate() {
        let t0 = Instant::now();
        let mut found = None;
        for s in 0..4000u64 {
            let c = [
                1 + (s.wrapping_mul(2_654_435_761) >> 3) % (p - 1),
                (s.wrapping_mul(40_503) >> 5) % p,
                (s.wrapping_mul(2_246_822_519) >> 7) % p,
                1 + (s.wrapping_mul(3_266_489_917) >> 9) % (p - 1),
            ];
            if let Ok(v) = v3_candidate(p, c, 0xC0FFEE + i as u64 * 101 + s) {
                found = Some((s, v));
                break;
            }
        }
        let Some((cand, v)) = found else {
            log.push(format!("v3 p={p}: no candidate"));
            all_ok = false;
            continue;
        };
        let rec = V3_RECORDED[i];
        // Old instrument, as recorded (the gate).
        let rho_old = rho_on_ec(&v.e1, v.gen, v.target, v.r, v.seed);
        let rho_old_ops = rho_old.group_ops.adds + rho_old.group_ops.doubles;
        let ic_old =
            genus2_index_calculus(&v.jac, &v.dg, &v.dt, v.r, p as usize, 50_000_000, v.seed);
        let gate_ok = cand == rec.1
            && rho_old_ops == rec.2
            && ic_old.group_ops == rec.3
            && ic_old.trials == rec.4
            && ic_old.la_field_ops == rec.5
            && ic_old.recovered == Some(v.x);
        all_ok &= gate_ok;
        log.push(format!(
            "v3 p={p}: gate candidate {cand} rho {rho_old_ops} ic {} la {} trials {} -> {}",
            ic_old.group_ops,
            ic_old.la_field_ops,
            ic_old.trials,
            if gate_ok {
                "reproduces the record"
            } else {
                "MISMATCH"
            }
        ));
        // Arm L: the old relations, the counted elimination.
        let rows = genus2_collect_rows(&v.jac, &v.dg, &v.dt, v.r, p as usize, 50_000_000, v.seed);
        let (lx_free, lops_free) = solve_linear_free(&rows.rows, rows.fb_size, v.r);
        let (lx, lnat) = solve_linear_counted(&rows.rows, rows.fb_size, v.r);
        let l_cert = lx.is_some_and(|k| v.jac.mul(&v.dg, k) == v.dt);
        let msize = legacy_matrix_size(rows.fb_size, rows.rows.len());
        let legacy_eq = ic_old.la_field_ops == msize + 1;
        let l_tot = |w: f64| rows.group_ops as f64 + w * lnat.mul as f64;
        // Arms A, B, C on the word Jacobian.
        let h = genus2_to_h64(&v.jac).expect("word Jacobian");
        let cross = crosscheck_genus2(&v.jac, &h, 600, 77 + p);
        let hg = convert_div(&h, &v.dg).expect("G converts");
        let ht = convert_div(&h, &v.dt).expect("T converts");
        let inst = instance_for_rung(&h, v.jac_order, v.r, hg, ht, v.x).expect("instance");
        // Rho on E1, plain and negation.
        let rho_runs = |neg: bool| -> Vec<RhoRef> {
            (0..a.v3_rho_seeds)
                .into_par_iter()
                .map(|s| rho_on_curve(&v.e1, v.gen, v.target, v.r, 1000 + s, neg))
                .collect()
        };
        let (rneg, rplain) = (agg_rho(&rho_runs(true)), agg_rho(&rho_runs(false)));
        let mut arms = serde_json::Map::new();
        let mut cmp = serde_json::Map::new();
        for label in ["A", "B", "C"] {
            let rs: Vec<(IcResult, bool)> = (0..a.v3_seeds)
                .into_par_iter()
                .map(|s| {
                    let res = ic_on_rung(&inst, arm_of(label), 500 + s, a.max_trials);
                    let ok = res.recovered.is_some_and(|k| {
                        res.certified && h.mul_rtl(hg, k) == ht && v.jac.mul(&v.dg, k) == v.dt
                    });
                    (res, ok)
                })
                .collect();
            let ag = agg_ic(&rs);
            all_ok &= ag.certified == ag.runs && ag.exhausted == 0;
            cmp.insert(label.into(), compare(&ag, &rneg, &rplain));
            arms.insert(label.into(), ag.json());
        }
        // The old and the L arm against the same references.
        let l_ag = IcAgg {
            runs: 1,
            certified: usize::from(l_cert),
            group_ops: rows.group_ops as f64,
            la_mul: lnat.mul as f64,
            la_add: lnat.add as f64,
            trials: rows.trials as f64,
            relations: rows.rows.len() as f64,
            columns: rows.fb_size as f64,
            ..Default::default()
        };
        all_ok &= l_cert;
        cmp.insert("L".into(), compare(&l_ag, &rneg, &rplain));
        let old_ag = IcAgg {
            runs: 1,
            certified: usize::from(ic_old.recovered == Some(v.x)),
            group_ops: ic_old.group_ops as f64,
            la_mul: ic_old.la_field_ops as f64,
            trials: ic_old.trials as f64,
            relations: ic_old.relations as f64,
            columns: ic_old.fb_size as f64,
            ..Default::default()
        };
        cmp.insert("old".into(), compare(&old_ag, &rneg, &rplain));
        out.push(json!({
            "p": p, "r": v.r, "candidate": cand, "x": v.x,
            "e1_order": v.e1_order, "e2_order": v.e2_order, "jac_order": v.jac_order,
            "gate_reproduces_record": gate_ok,
            "word_jacobian_crosscheck": cross,
            "old": {
                "rho_src_ops": rho_old_ops, "ic_group_ops": ic_old.group_ops,
                "ic_trials": ic_old.trials, "ic_relations": ic_old.relations,
                "ic_fb": ic_old.fb_size, "ic_la_mulmods": ic_old.la_field_ops,
                "ratio_vs_recorded_rho": W.map(|w| (ic_old.group_ops as f64 + w * ic_old.la_field_ops as f64) / rho_old_ops as f64),
            },
            "legacy_counter": {
                "la_field_ops": ic_old.la_field_ops,
                "matrix_size_fb_plus_2_times_rows": msize,
                "recorded_minus_matrix_size": ic_old.la_field_ops as i64 - msize as i64,
                "equal": legacy_eq,
                "rows": rows.rows.len(), "fb_size": rows.fb_size,
                "free_path_same_answer": lx_free == lx,
                "free_path_ops": lops_free,
            },
            "L": {
                "group_ops": rows.group_ops, "trials": rows.trials, "relations": rows.rows.len(),
                "la_mul": lnat.mul, "la_add": lnat.add,
                "la_mul_over_recorded": lnat.mul as f64 / ic_old.la_field_ops.max(1) as f64,
                "certified": l_cert,
                "total": W.map(l_tot),
            },
            "arms": arms,
            "rho_negation": rneg.json(), "rho_plain": rplain.json(),
            "compare": cmp,
            "wall_s": t0.elapsed().as_secs_f64(),
        }));
    }
    (Value::Array(out), all_ok)
}

// ── tower ────────────────────────────────────────────────────────────────

struct TowerRun {
    g: u32,
    arm: &'static str,
    res: IcResult,
    certified: bool,
}

struct TowerJob {
    p: u64,
    inst: usize,
    r: u64,
    n_e: u64,
    skipped: Vec<(u32, String)>,
    runs: Vec<TowerRun>,
    rho_neg: Vec<RhoRef>,
    rho_plain: Vec<RhoRef>,
    wall_s: f64,
    transfer_wall_s: f64,
}

fn run_tower(a: &Args, log: &mut Vec<String>) -> (Value, bool) {
    let primes: Vec<u64> = a
        .tower_primes
        .split(',')
        .filter_map(|x| x.trim().parse().ok())
        .collect();
    let mut jobs: Vec<(u64, usize)> = primes
        .iter()
        .flat_map(|&p| (0..a.tower_instances).map(move |i| (p, i)))
        .collect();
    jobs.sort_by(|x, y| y.0.cmp(&x.0).then(x.1.cmp(&y.1)));
    let next = AtomicUsize::new(0);
    let done: Mutex<Vec<TowerJob>> = Mutex::new(vec![]);
    let logm: Mutex<Vec<String>> = Mutex::new(vec![]);
    std::thread::scope(|s| {
        for _ in 0..a.threads.max(1) {
            s.spawn(|| loop {
                let j = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(p, inst)) = jobs.get(j) else { break };
                let t0 = Instant::now();
                let seed = 1000 + 17 * inst as u64 + p % 997;
                let Some(t) = build64(p, seed, p / 4, 64) else {
                    logm.lock()
                        .unwrap()
                        .push(format!("tower p={p} inst {inst}: no tower instance"));
                    continue;
                };
                let mut job = TowerJob {
                    p,
                    inst,
                    r: t.r,
                    n_e: t.e.order(),
                    skipped: vec![],
                    runs: vec![],
                    rho_neg: vec![],
                    rho_plain: vec![],
                    wall_s: 0.0,
                    transfer_wall_s: t.transfer_wall_s,
                };
                for s in 0..a.tower_rho_seeds {
                    job.rho_neg
                        .push(rho_on_curve(&t.e, t.gen, t.target, t.r, 100 + s, true));
                    job.rho_plain
                        .push(rho_on_curve(&t.e, t.gen, t.target, t.r, 100 + s, false));
                }
                for rg in t.rungs.iter() {
                    let g = rg.cover.genus;
                    let curve = &rg.cover.curve;
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
                            let ok = res.certified
                                && res
                                    .recovered
                                    .is_some_and(|k| curve.mul_rtl(rg.d, k) == rg.t);
                            job.runs.push(TowerRun {
                                g,
                                arm: match label {
                                    "A" => "A",
                                    "B" => "B",
                                    _ => "C",
                                },
                                res,
                                certified: ok,
                            });
                        }
                    }
                }
                job.wall_s = t0.elapsed().as_secs_f64();
                logm.lock().unwrap().push(format!(
                    "tower p={p} inst {inst}: r={} #E={} done in {:.1}s, {} runs, skipped {:?}",
                    job.r,
                    job.n_e,
                    job.wall_s,
                    job.runs.len(),
                    job.skipped
                ));
                done.lock().unwrap().push(job);
            });
        }
    });
    log.extend(logm.into_inner().unwrap());
    let mut jobs = done.into_inner().unwrap();
    jobs.sort_by_key(|j| (j.p, j.inst));
    let mut ok = true;
    let mut rows = vec![];
    for &p in &primes {
        let js: Vec<&TowerJob> = jobs.iter().filter(|j| j.p == p).collect();
        if js.is_empty() {
            ok = false;
            continue;
        }
        let rneg = agg_rho(
            &js.iter()
                .flat_map(|j| j.rho_neg.iter().cloned())
                .collect::<Vec<_>>(),
        );
        let rplain = agg_rho(
            &js.iter()
                .flat_map(|j| j.rho_plain.iter().cloned())
                .collect::<Vec<_>>(),
        );
        ok &= rneg.certified == rneg.runs && rplain.certified == rplain.runs;
        let mut by = serde_json::Map::new();
        let mut cmp = serde_json::Map::new();
        for g in [2u32, 3] {
            for arm in ["A", "B", "C"] {
                let rs: Vec<(IcResult, bool)> = js
                    .iter()
                    .flat_map(|j| j.runs.iter())
                    .filter(|r| r.g == g && r.arm == arm)
                    .map(|r| (r.res.clone(), r.certified))
                    .collect();
                if rs.is_empty() {
                    continue;
                }
                let ag = agg_ic(&rs);
                ok &= ag.certified == ag.runs && ag.exhausted == 0;
                let key = format!("g{g}_{arm}");
                cmp.insert(key.clone(), compare(&ag, &rneg, &rplain));
                by.insert(key, ag.json());
            }
        }
        let r_mean = js.iter().map(|j| j.r as f64).sum::<f64>() / js.len() as f64;
        rows.push(json!({
            "p": p, "instances": js.len(), "r_mean": r_mean,
            "skipped": js.iter().flat_map(|j| j.skipped.iter().map(move |(g, e)| json!({"inst": j.inst, "genus": g, "reason": e}))).collect::<Vec<_>>(),
            "transfer_wall_s_mean_one_time": js.iter().map(|j| j.transfer_wall_s).sum::<f64>() / js.len() as f64,
            "rho_negation": rneg.json(), "rho_plain": rplain.json(),
            "arms": by, "compare": cmp,
        }));
    }
    (Value::Array(rows), ok)
}

// ── small_de ─────────────────────────────────────────────────────────────

fn splitmix(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn reps_for(spec: &str, bits: u32) -> usize {
    spec.split(',')
        .filter_map(|kv| {
            let (k, v) = kv.split_once(':')?;
            (k.trim().parse::<u32>().ok()? == bits).then(|| v.trim().parse().ok())?
        })
        .next()
        .unwrap_or(1)
}

fn nat_sum(n: &crypto_lib::cryptanalysis::chained_transfer::small_de::Native) -> f64 {
    n.total() as f64
}

fn sd_arm(
    inst: &Inst,
    fb: &Fb,
    label: &str,
    m: usize,
    reps: usize,
    ntargets: usize,
    seed: u64,
) -> Value {
    let mut stats = vec![];
    let mut known0 = None;
    for k in 0..reps {
        let out = relation_phase(inst, fb, m, splitmix(seed ^ (k as u64) << 20));
        stats.push(out.stats);
        if known0.is_none() {
            known0 = Some(out.known);
        }
    }
    let known = known0.unwrap();
    let mut tr = StdRng::seed_from_u64(inst.curve.p ^ 0x5EED_7A26);
    let mut ops = GroupOps::default();
    let mut tgts = vec![];
    let mut certified = 0usize;
    for k in 0..ntargets {
        let t = tr.gen_range(1..inst.r);
        let tp = inst.curve.mul(&mut ops, inst.g, t);
        let mut ts = solve_target(
            inst,
            fb,
            &known,
            m,
            tp,
            splitmix(seed ^ 0xA5 ^ (k as u64) << 8),
            200_000,
        );
        if ts.recovered.is_some_and(|x| x != t) {
            ts.certified = false;
        }
        certified += usize::from(ts.certified);
        tgts.push(ts);
    }
    let build_gae = fb.build_ops.gae();
    let build_nat = nat_sum(&fb.build_native);
    let nrep = stats.len() as f64;
    let rel = |w: f64| {
        stats
            .iter()
            .map(|s| s.gae + build_gae + w * (nat_sum(&s.native) + build_nat))
            .sum::<f64>()
            / nrep
    };
    let ok_t: Vec<&_> = tgts.iter().filter(|t| t.certified).collect();
    let tg = |w: f64| {
        ok_t.iter()
            .map(|t| t.gae + w * nat_sum(&t.native))
            .sum::<f64>()
            / ok_t.len().max(1) as f64
    };
    json!({
        "arm": label, "fb_points": fb.len(), "columns": fb.ncols,
        "reps": reps, "targets": ntargets, "certified_targets": certified,
        "rel": W.map(rel), "tgt": W.map(tg),
        "single": W.map(|w| rel(w) + tg(w)),
        "amortised": W.map(|w| rel(w) / ntargets as f64 + tg(w)),
        "relations": stats.iter().map(|s| s.relations as f64).sum::<f64>() / nrep,
        "la_total_ops": stats.iter().map(|s| s.la_total_ops as f64).sum::<f64>() / nrep,
    })
}

fn sd_cell(inst: &Inst, bits: u32, m: usize, a: &Args) -> Value {
    let reps = reps_for(&a.sd_reps, bits);
    let n_target = (inst.n as f64).powf(1.0 / m as f64).ceil() as usize;
    let seed = splitmix(a.sd_seed ^ ((bits as u64) << 32) ^ ((m as u64) << 48) ^ (inst.d as u64));
    let mut arms = vec![];
    if inst.d == 0 {
        let e = scalar_endo(inst, seed ^ 0xC0DE);
        let a3 = fb_layered(inst, &e, n_target, 4);
        let a0 = fb_interval(inst, a3.len());
        arms.push(sd_arm(inst, &a3, "A3", m, reps, a.sd_targets, seed ^ 3));
        arms.push(sd_arm(inst, &a0, "A0", m, reps, a.sd_targets, seed));
    } else {
        let e = endo_for(inst).expect("endomorphism");
        let a2 = match e.kind {
            EndoKind::Zeta { .. } | EndoKind::Iota { .. } => fb_aut(inst, &e, n_target),
            _ => fb_layered(inst, &e, n_target, 4),
        };
        let a1 = fb_interval(inst, a2.len());
        arms.push(sd_arm(inst, &a2, "A2", m, reps, a.sd_targets, seed ^ 2));
        arms.push(sd_arm(inst, &a1, "A1", m, reps, a.sd_targets, seed ^ 1));
    }
    let mut tr = StdRng::seed_from_u64(inst.curve.p ^ 0x5EED_7A26);
    let mut ops = GroupOps::default();
    let tps: Vec<PrimePoint> = (0..a.sd_targets)
        .map(|_| {
            let t = tr.gen_range(1..inst.r);
            inst.curve.mul(&mut ops, inst.g, t)
        })
        .collect();
    let mut kinds = vec![RhoKind::Plain, RhoKind::Negation];
    if inst.d == -3 || inst.d == -4 {
        kinds.push(RhoKind::Aut);
    }
    let mut rho = vec![];
    for kind in kinds {
        let runs: Vec<_> = tps
            .iter()
            .enumerate()
            .filter_map(|(k, tp)| {
                rho_split_prime(inst, kind, *tp, splitmix(seed ^ 0xB0 ^ (k as u64)))
            })
            .collect();
        if runs.is_empty() {
            continue;
        }
        let n = runs.len() as f64;
        let mean = |f: &dyn Fn(
            &crypto_lib::cryptanalysis::chained_transfer::instrument::RhoSplit,
        ) -> f64| { runs.iter().map(f).sum::<f64>() / n };
        rho.push(json!({
            "kind": runs[0].kind, "automorphisms": runs[0].automorphisms, "runs": runs.len(),
            "certified": runs.iter().filter(|r| r.certified).count(),
            "total_gae": mean(&|r| r.total_gae), "walk_ops": mean(&|r| r.walk_ops as f64),
            "other_gae": mean(&|r| r.other_gae), "native": mean(&|r| r.native as f64),
            "steps": mean(&|r| r.steps as f64),
            "total": W.map(|w| mean(&|r| r.total(w))),
            "walk_only": W.map(|w| mean(&|r| r.walk_only(w))),
        }));
    }
    json!({ "bits": bits, "m": m, "d": inst.d, "p": inst.curve.p, "r": inst.r,
            "n_target": n_target, "arms": arms, "rho": rho })
}

fn run_smallde(a: &Args, log: &mut Vec<String>) -> (Value, bool) {
    let sizes: Vec<u32> = a
        .sd_sizes
        .split(',')
        .filter_map(|x| x.trim().parse().ok())
        .collect();
    let ds: [i64; 6] = [-3, -4, -7, -8, -11, 0];
    let keys: Vec<(u32, i64)> = sizes
        .iter()
        .flat_map(|&b| ds.iter().map(move |&d| (b, d)))
        .collect();
    let built: Vec<(u32, i64, Option<Inst>)> = keys
        .par_iter()
        .map(|&(bits, d)| (bits, d, construct(d, bits, a.sd_seed)))
        .collect();
    let mut jobs: Vec<(u32, usize, usize)> = vec![];
    for (ci, (bits, _, r)) in built.iter().enumerate() {
        if r.is_some() {
            for m in [2usize, 3] {
                jobs.push((*bits, m, ci));
            }
        }
    }
    jobs.sort_by_key(|&(bits, m, _)| std::cmp::Reverse((bits, m)));
    let cells: Vec<Value> = jobs
        .par_iter()
        .map(|&(bits, m, ci)| {
            let inst = built[ci].2.as_ref().unwrap();
            let t0 = Instant::now();
            let c = sd_cell(inst, bits, m, a);
            eprintln!(
                "smallde cell bits={bits} m={m} D={} done in {:.1}s",
                inst.d,
                t0.elapsed().as_secs_f64()
            );
            c
        })
        .collect();
    let mut cells = cells;
    cells.sort_by_key(|c| {
        (
            c["bits"].as_u64().unwrap(),
            c["m"].as_u64().unwrap(),
            c["d"].as_i64().unwrap(),
        )
    });
    let ok = cells.iter().all(|c| {
        c["arms"]
            .as_array()
            .unwrap()
            .iter()
            .all(|x| x["certified_targets"].as_u64() == x["targets"].as_u64())
            && c["rho"]
                .as_array()
                .unwrap()
                .iter()
                .all(|x| x["certified"] == x["runs"])
    });
    log.push(format!(
        "smallde: {} cells, all certified: {ok}",
        cells.len()
    ));
    (Value::Array(cells), ok)
}

fn main() {
    let a = Args::parse();
    rayon::ThreadPoolBuilder::new()
        .num_threads(a.threads)
        .build_global()
        .ok();
    let want = |s: &str| a.sections.is_empty() || a.sections.iter().any(|x| x == s);
    let t0 = Instant::now();
    let mut log: Vec<String> = vec![];
    let mut doc = serde_json::Map::new();
    doc.insert("schema".into(), json!("instrument.rerun/v1"));
    doc.insert(
        "protocol".into(),
        json!("research/instrument_fixes_20261112/PROTOCOL.md"),
    );
    doc.insert("prices_w".into(), json!(W));
    doc.insert(
        "calibrated_informational".into(),
        json!({"w_rel_mod_p": W_REL_CAL, "w_la_mod_r": W_LA_CAL}),
    );
    let mut all_ok = true;
    if want("v3") {
        let (v, ok) = run_v3(&a, &mut log);
        all_ok &= ok;
        doc.insert("v3".into(), v);
        doc.insert("v3_ok".into(), json!(ok));
    }
    if want("tower") {
        let (v, ok) = run_tower(&a, &mut log);
        all_ok &= ok;
        doc.insert("tower".into(), v);
        doc.insert("tower_ok".into(), json!(ok));
    }
    if want("smallde") {
        let (v, ok) = run_smallde(&a, &mut log);
        all_ok &= ok;
        doc.insert("smallde".into(), v);
        doc.insert("smallde_ok".into(), json!(ok));
    }
    doc.insert("all_gates_and_certificates_ok".into(), json!(all_ok));
    doc.insert(
        "wall_s_bookkeeping".into(),
        json!(t0.elapsed().as_secs_f64()),
    );
    for l in &log {
        println!("{l}");
    }
    let path = format!("{}/{}", a.out_dir, a.json);
    std::fs::write(&path, serde_json::to_string(&Value::Object(doc)).unwrap()).expect("write json");
    println!("wrote {path}; all gates and certificates ok: {all_ok}");
    if !all_ok {
        std::process::exit(2);
    }
}
