//! Gaudry/Nagao decomposition index calculus on genus-2 Jacobians over
//! F_{q^2} (no cover), against matched negation-map Pollard rho.
//! Local development check, not a lab benchmark; every number it prints is
//! unverified.  See research/extfield-jacobian-decomp_20261115/.
//!
//! Usage:
//!   extfield_jac_decomp run <q> <seed> [ic|rho|both] [internal_seed_tag]
//!   extfield_jac_decomp oracle <q> <seed> <trials>
//!   extfield_jac_decomp lp <q> <seed> <tag>      (round 2: five arms on one paired decomposition stream)

#[path = "../../research/extfield-jacobian-decomp_20261115/src/decomp.rs"]
mod decomp;
#[path = "../../research/extfield-jacobian-decomp_20261115/src/fq2.rs"]
mod fq2;
#[path = "../../research/extfield-jacobian-decomp_20261115/src/jac.rs"]
mod jac;
// Round 2 (2026-11-21): reduced factor base, large primes, sparse LA.
#[allow(dead_code, clippy::all)]
#[path = "../../research/extfield-jacobian-decomp_20261121/src/large_prime_filter.rs"]
mod large_prime_filter;
#[path = "../../research/extfield-jacobian-decomp_20261121/src/lp_hooks.rs"]
mod lp_hooks;
#[path = "../../research/extfield-jacobian-decomp_20261121/src/sparse_la.rs"]
mod sparse_la;

use decomp::{
    build_fb, decomposition_from_solution, equations, native_now, oracle, solve_escalating,
    verify_decomp, Macaulay, Status,
};
use fq2::{Fld, E, ZERO};
use jac::{factor, Curve, Div, Key, Rng};
use large_prime_filter::{filter_large_primes, LargePrimeFilterOptions, PartialRelation};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Instant;

fn splitmix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, b, m);
        }
        b = mulmod(b, b, m);
        e >>= 1;
    }
    r
}

fn invmod(a: u64, m: u64) -> u64 {
    powmod(a, m - 2, m) // m prime
}

struct Instance {
    q: u32,
    seed: u64,
    cv: Curve,
    n: u64,
    ell: u64,
    h: u64,
    d: Div,
    qq: Div,
    k: u64,
    tries: u32,
}

fn make_instance(q: u32, seed: u64) -> Instance {
    let fld = Fld::new(q);
    let mut rng = Rng(splitmix(q as u64 * 1_000_003 + seed));
    let mut tries = 0u32;
    loop {
        tries += 1;
        let f = Curve::random_f(&fld, &mut rng);
        let cv = Curve::new(fld.clone(), f);
        if !cv.is_squarefree() {
            continue;
        }
        let n1 = cv.count_points_fq2();
        let Some(n) = cv.find_order(n1, &mut rng) else {
            continue;
        };
        let fac = factor(n);
        let (l, e) = *fac.last().unwrap();
        if e != 1 {
            continue;
        }
        let h = n / l;
        if h > 16 || l < 1000 {
            continue;
        }
        let d0 = cv.random_elt(&mut rng);
        let d = cv.mul(&d0, h);
        if cv.is_identity(&d) {
            continue;
        }
        let k = 2 + rng.below(l - 3);
        let qq = cv.mul(&d, k);
        return Instance {
            q,
            seed,
            cv,
            n,
            ell: l,
            h,
            d,
            qq,
            k,
            tries,
        };
    }
}

fn pool(inst: &Instance, rng: &mut Rng, size: usize) -> Vec<(Div, u64, u64)> {
    let cv = &inst.cv;
    (0..size)
        .map(|_| {
            let c = rng.below(inst.ell);
            let d = rng.below(inst.ell);
            let m = cv.add(&cv.mul(&inst.d, c.max(1)), &cv.mul(&inst.qq, d.max(1)));
            (m, c.max(1), d.max(1))
        })
        .collect()
}

// ------------------------------------------------------------------ IC arm

struct RowT {
    cols: Vec<(usize, i64)>, // (class id, coefficient)
    rhs: u64,
    kcoef: u64,
}

/// Forward elimination mod ell; returns (k if determined, mulmods, rank, inconsistent).
fn la_solve(rows: &[RowT], class_ids: &[usize], ell: u64) -> (Option<u64>, u64, usize, bool) {
    let m = class_ids.len();
    let idx: HashMap<usize, usize> = class_ids.iter().enumerate().map(|(i, &c)| (c, i)).collect();
    let w = m + 2; // classes, k, rhs
    let mut a: Vec<Vec<u64>> = rows
        .iter()
        .map(|r| {
            let mut v = vec![0u64; w];
            for &(cid, co) in &r.cols {
                let j = idx[&cid];
                let c = co.rem_euclid(ell as i64) as u64;
                v[j] = (v[j] + c) % ell;
            }
            v[m] = r.kcoef % ell;
            v[m + 1] = (ell - r.rhs % ell) % ell; // R + sum = 0 => sum phi + g k = -e
            v
        })
        .collect();
    let nr = a.len();
    let mut ops = 0u64;
    let mut r = 0usize;
    let mut k_pivot_row = None;
    for c in 0..=m {
        let Some(p) = (r..nr).find(|&i| a[i][c] != 0) else {
            continue;
        };
        a.swap(p, r);
        let inv = invmod(a[r][c], ell);
        for j in c..w {
            a[r][j] = mulmod(a[r][j], inv, ell);
        }
        ops += (w - c) as u64;
        let prow = a[r].clone();
        for row in a.iter_mut().skip(r + 1) {
            let f = row[c];
            if f == 0 {
                continue;
            }
            for j in c..w {
                if prow[j] != 0 {
                    let t = mulmod(f, prow[j], ell);
                    row[j] = (row[j] + ell - t) % ell;
                }
            }
            ops += (w - c) as u64;
        }
        if c == m {
            k_pivot_row = Some(r);
        }
        r += 1;
        if r == nr {
            break;
        }
    }
    let mut inconsistent = false;
    for row in a.iter().skip(r) {
        if row[..=m].iter().all(|&x| x == 0) && row[m + 1] != 0 {
            inconsistent = true;
        }
    }
    let k = k_pivot_row.map(|pr| a[pr][m + 1]);
    (k, ops, r, inconsistent)
}

fn run_ic(inst: &Instance, tag: u64, w_cal: f64) -> Value {
    let cv = &inst.cv;
    let ell = inst.ell;
    let t_start = Instant::now();
    let adds0 = cv.adds.get();
    let nat0 = native_now();
    let mut rng = Rng(splitmix(
        inst.q as u64 * 7_919 + inst.seed * 104_729 + tag * 31 + 1,
    ));
    let fb = build_fb(cv);
    let macs = [Macaulay::new(5), Macaulay::new(6), Macaulay::new(7)];
    let pl = pool(inst, &mut rng, 16);
    let (mut r, mut c, mut d) = (pl[0].0.clone(), pl[0].1, pl[0].2);
    let mut lrng = splitmix(tag + 99);
    let cap_trials = 300 * fb.n_classes as u64;
    let (mut trials, mut success_trials, mut degenerate, mut rejected) = (0u64, 0u64, 0u64, 0u64);
    let (mut amb, mut esc6, mut esc7, mut incons) = (0u64, 0u64, 0u64, 0u64);
    let mut n_sols = 0u64;
    let mut nongeneric_dim = 0u64;
    let (mut solver_native_sum, mut solver_native_n) = (0u64, 0u64);
    let mut solver_time = 0f64;
    let mut rows: Vec<RowT> = Vec::new();
    let mut class_seen: HashMap<usize, usize> = HashMap::new();
    let mut class_ids: Vec<usize> = Vec::new();
    let mut la_ops = 0u64;
    let mut la_attempts = 0u64;
    let mut la_time = 0f64;
    let mut next_attempt = 0usize;
    let mut k_found: Option<u64> = None;
    while k_found.is_none() && trials < cap_trials {
        let j = rng.below(16) as usize;
        r = cv.add(&r, &pl[j].0);
        c = (c + pl[j].1) % ell;
        d = (d + pl[j].2) % ell;
        trials += 1;
        if r.u.len() != 3 {
            degenerate += 1;
            continue;
        }
        let Some((eqs, w)) = equations(cv, &r) else {
            degenerate += 1;
            continue;
        };
        let ts = Instant::now();
        let n0 = native_now();
        let (out, lvl) = solve_escalating(&macs, &cv.fld, &eqs, &mut lrng);
        solver_native_sum += native_now() - n0;
        solver_native_n += 1;
        solver_time += ts.elapsed().as_secs_f64();
        if out.dim != 16 {
            nongeneric_dim += 1;
        }
        match out.status {
            Status::Insufficient => {
                degenerate += 1;
                continue;
            }
            Status::Ambiguous => amb += 1,
            Status::Inconsistent => {
                continue;
            }
            Status::Ok => {}
        }
        if lvl == 1 {
            esc6 += 1;
        }
        if lvl == 2 {
            esc7 += 1;
        }
        let mut got = false;
        for z in &out.sols {
            n_sols += 1;
            if let Some(dec) = decomposition_from_solution(cv, &r, &w, z, false) {
                if verify_decomp(cv, &r, &dec) {
                    let mut cols: Vec<(usize, i64)> = Vec::new();
                    for &(x, y, m) in &dec.pts {
                        let (cid, sign) = fb.class_of[&(x, y.a, y.b)];
                        if sign == 0 {
                            continue;
                        }
                        cols.push((cid, sign as i64 * m as i64));
                        class_seen.entry(cid).or_insert_with(|| {
                            class_ids.push(cid);
                            class_ids.len() - 1
                        });
                    }
                    rows.push(RowT {
                        cols,
                        rhs: c,
                        kcoef: d,
                    });
                    got = true;
                } else {
                    rejected += 1;
                }
            }
        }
        if got {
            success_trials += 1;
        }
        if rows.len() >= class_ids.len() + 10 && rows.len() >= next_attempt {
            let tl = Instant::now();
            let (k, ops, _rank, bad) = la_solve(&rows, &class_ids, ell);
            la_time += tl.elapsed().as_secs_f64();
            la_ops += ops;
            la_attempts += 1;
            if bad {
                incons += 1;
            }
            next_attempt = rows.len() + 10.max(rows.len() / 20);
            if let Some(k) = k {
                if cv.mul(&inst.d, k) == inst.qq {
                    k_found = Some(k);
                }
            }
        }
    }
    let adds = cv.adds.get() - adds0;
    let native = native_now() - nat0;
    let solved = k_found == Some(inst.k);
    let price = |w: f64| adds as f64 + w * (native + la_ops) as f64;
    let json = json!({
        "arm": "ic", "q": inst.q, "seed": inst.seed, "tag": tag, "solved": solved,
        "k_true": inst.k, "k_found": k_found,
        "fb_points": fb.pts.len(), "fb_classes": fb.n_classes,
        "trials": trials, "success_trials": success_trials,
        "relations": rows.len(), "classes_used": class_ids.len(),
        "degenerate_R": degenerate, "rejected_candidates": rejected,
        "ambiguous": amb, "escalated_D6": esc6, "escalated_D7": esc7,
        "solutions_total": n_sols, "quotient_dim_not_16": nongeneric_dim, "la_inconsistent": incons,
        "group_adds": adds, "native_fq_ops_total": native,
        "solver_native_per_trial": if solver_native_n > 0 { solver_native_sum as f64 / solver_native_n as f64 } else { 0.0 },
        "solver_time_per_trial_s": if solver_native_n > 0 { solver_time / solver_native_n as f64 } else { 0.0 },
        "la_mulmods_dense": la_ops, "la_attempts": la_attempts, "la_time_s": la_time,
        "la_model_wiedemann_mulmods": 2 * class_ids.len() as u64 * rows.iter().map(|r| r.cols.len() as u64 + 1).sum::<u64>(),
        "S_w0": price(0.0), "S_w0p1": price(0.1), "S_w1": price(1.0), "S_wcal": price(w_cal),
        "wall_s": t_start.elapsed().as_secs_f64(),
    });
    json
}

// ----------------------------------------------------------------- rho arm

fn vkey(cv: &Curve, d: &Div) -> [u32; 4] {
    let v0 = d.v.first().copied().unwrap_or(ZERO);
    let v1 = d.v.get(1).copied().unwrap_or(ZERO);
    let _ = cv;
    [v1.a, v1.b, v0.a, v0.b]
}

/// Canonical representative of {X, -X} (smaller v-key); true when negated.
fn canon(cv: &Curve, x: &mut Div) -> bool {
    let k = vkey(cv, x);
    let nx = cv.neg(x);
    let nk = vkey(cv, &nx);
    if nk < k {
        *x = nx;
        true
    } else {
        false
    }
}

fn hash_div(cv: &Curve, x: &Div) -> u64 {
    let k: Key = cv.key(x);
    let mut h = 0u64;
    for &w in &k[..5] {
        h = splitmix(h ^ w as u64);
    }
    h
}

fn run_rho(inst: &Instance, tag: u64, w_cal: f64) -> Value {
    let cv = &inst.cv;
    let ell = inst.ell;
    let t_start = Instant::now();
    let adds0 = cv.adds.get();
    let mut rng = Rng(splitmix(
        inst.q as u64 * 15_485_863 + inst.seed * 32_452_843 + tag * 17 + 5,
    ));
    let pl = pool(inst, &mut rng, 64);
    let theta = ((((ell as f64).sqrt() / 40.0).log2()).floor() as i64).max(2) as u32;
    let mask = (1u64 << theta) - 1;
    let max_len = 40u64 << theta;
    let cap_steps = (40.0 * (ell as f64).sqrt()) as u64;
    let mut table: HashMap<Key, (u64, u64, u64)> = HashMap::new();
    let (mut steps, mut walks, mut abandoned, mut dps, mut useless) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut n_part, mut n_canon, mut n_dp, mut n_cyc, mut n_tab) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut escapes = 0u64;
    let mut k_found: Option<u64> = None;
    'outer: while steps < cap_steps {
        walks += 1;
        let c0 = rng.below(ell);
        let d0 = rng.below(ell);
        let mut x = cv.add(&cv.mul(&inst.d, c0.max(1)), &cv.mul(&inst.qq, d0.max(1)));
        let (mut c, mut d) = (c0.max(1), d0.max(1));
        if canon(cv, &mut x) {
            c = (ell - c) % ell;
            d = (ell - d) % ell;
        }
        let mut hist: [Key; 8] = [[0; 8]; 8];
        let mut hpos = 0usize;
        let mut reached_dp = false;
        for _ in 0..max_len {
            let h = hash_div(cv, &x);
            n_part += 1;
            n_dp += 1;
            if (h >> 20) & mask == 0 {
                n_tab += 1;
                dps += 1;
                let key = cv.key(&x);
                if let Some(&(c2, d2, w2)) = table.get(&key) {
                    if w2 != walks {
                        let dd = (d2 + ell - d) % ell;
                        if dd != 0 {
                            let k = mulmod((c + ell - c2) % ell, invmod(dd, ell), ell);
                            if cv.mul(&inst.d, k) == inst.qq {
                                k_found = Some(k);
                                break 'outer;
                            }
                        }
                        useless += 1;
                    }
                } else {
                    table.insert(key, (c, d, walks));
                }
                reached_dp = true;
                break;
            }
            let j = (h & 63) as usize;
            let mut nx = cv.add(&x, &pl[j].0);
            let mut nc = (c + pl[j].1) % ell;
            let mut nd = (d + pl[j].2) % ell;
            steps += 1;
            n_canon += 1;
            if canon(cv, &mut nx) {
                nc = (ell - nc) % ell;
                nd = (ell - nd) % ell;
            }
            n_cyc += 1;
            let nk = cv.key(&nx);
            if hist.contains(&nk) {
                escapes += 1;
                nx = cv.add(&nx, &nx);
                nc = (2 * nc) % ell;
                nd = (2 * nd) % ell;
                if canon(cv, &mut nx) {
                    nc = (ell - nc) % ell;
                    nd = (ell - nd) % ell;
                }
            }
            hist[hpos] = cv.key(&nx);
            hpos = (hpos + 1) % 8;
            x = nx;
            c = nc;
            d = nd;
            if steps >= cap_steps {
                break 'outer;
            }
        }
        if !reached_dp {
            abandoned += 1;
        }
    }
    let solved = k_found == Some(inst.k);
    let adds = cv.adds.get() - adds0;
    let native = n_part + n_canon + n_dp + n_cyc + n_tab;
    let price = |w: f64| adds as f64 + w * native as f64;
    json!({
        "arm": "rho", "q": inst.q, "seed": inst.seed, "tag": tag, "solved": solved,
        "k_true": inst.k, "k_found": k_found,
        "theta": theta, "walk_steps": steps, "walks": walks, "abandoned_walks": abandoned,
        "distinguished_points": dps, "useless_collisions": useless, "cycle_escapes": escapes,
        "group_adds": adds,
        "native": {"partition_hash": n_part, "canonicalisation": n_canon, "dp_test": n_dp, "cycle_history_check": n_cyc, "table_ops": n_tab, "total": native},
        "S_w0": price(0.0), "S_w0p1": price(0.1), "S_w1": price(1.0), "S_wcal": price(w_cal),
        "wall_s": t_start.elapsed().as_secs_f64(),
    })
}

// -------------------------------------------------------------- calibration

fn calibrate(inst: &Instance) -> (f64, f64, f64) {
    let cv = &inst.cv;
    let mut rng = Rng(4242);
    let elts: Vec<Div> = (0..32).map(|_| cv.random_elt(&mut rng)).collect();
    let mut best_add = f64::MAX;
    for _ in 0..3 {
        let t = Instant::now();
        let mut acc = elts[0].clone();
        let reps = 3000;
        for i in 0..reps {
            acc = cv.add(&acc, &elts[i % 32]);
        }
        std::hint::black_box(&acc);
        best_add = best_add.min(t.elapsed().as_secs_f64() / reps as f64);
    }
    let fl = &cv.fld;
    let mut best_mul = f64::MAX;
    for _ in 0..3 {
        let t = Instant::now();
        let mut x = 12345u32 % fl.q;
        let reps = 20_000_000u64;
        for i in 0..reps {
            x = fl.qmul(x, 7 + (i as u32 & 3));
            x = fl.qadd(x, 1);
        }
        std::hint::black_box(x);
        best_mul = best_mul.min(t.elapsed().as_secs_f64() / reps as f64);
    }
    (best_mul / best_add, best_mul, best_add)
}

fn instance_json(inst: &Instance) -> Value {
    let f: Vec<[u32; 2]> = inst.cv.f.iter().map(|c| [c.a, c.b]).collect();
    json!({
        "q": inst.q, "seed": inst.seed, "nonresidue": inst.cv.fld.nr, "f_coeffs_low_to_high_[re,im]": f,
        "group_order": inst.n, "ell": inst.ell, "cofactor": inst.h, "curve_tries": inst.tries,
        "k": inst.k,
    })
}

// ------------------------------------------------- round 2: reduced FB + large primes

struct ArmCfg {
    name: &'static str,
    alpha: f64,
    c: f64,
}

const ARMS: [ArmCfg; 5] = [
    ArmCfg {
        name: "control_a1",
        alpha: 1.0,
        c: 1.0,
    },
    ArmCfg {
        name: "a0.75_c1",
        alpha: 0.75,
        c: 1.0,
    },
    ArmCfg {
        name: "a0.75_c2",
        alpha: 0.75,
        c: 2.0,
    },
    ArmCfg {
        name: "a0.6_c1",
        alpha: 0.6,
        c: 1.0,
    },
    ArmCfg {
        name: "a0.6_c2",
        alpha: 0.6,
        c: 2.0,
    },
];

/// A verified decomposition of R: (class id, signed multiplicity) pairs and the (c, d) of R = cD + dQ.
struct RelT {
    cols: Vec<(usize, i64)>,
    c: u64,
    d: u64,
}

struct Done {
    trial: u64,
    adds: u64,
    native: u64,
}

struct Arm {
    cfg: &'static ArmCfg,
    nf: usize,
    clamped: bool,
    in_f: Vec<i32>,
    rows: Vec<PartialRelation>,
    next_attempt: usize,
    lp_hist: [u64; 3],
    too_many_lp: u64,
    attempts: u64,
    wied_runs: u64,
    wied_tries: u64,
    filter_ops: u64,
    la_ops: u64,
    extra_adds: u64,
    la_time: f64,
    last_complete: usize,
    last_ncols: usize,
    last_report: String,
    wrong_k: u64,
    done: Option<Done>,
    k_found: Option<u64>,
    solve_rows: usize,
    solve_complete: usize,
    solve_ncols: usize,
}

/// Deterministic factor-base subset: the `nf` classes with the smallest hash of x (nested across arms).
fn fb_subset(class_x: &[u32], nf: usize) -> Vec<i32> {
    let mut order: Vec<usize> = (0..class_x.len()).collect();
    order.sort_by_key(|&c| splitmix(class_x[c] as u64 ^ 0xA5A5_5A5A_1234_0000));
    let mut in_f = vec![-1i32; class_x.len()];
    for (i, &c) in order.iter().take(nf).enumerate() {
        in_f[c] = i as i32;
    }
    in_f
}

fn arm_nf(cfg: &ArmCfg, q: u32, n_classes: usize) -> usize {
    if cfg.alpha >= 1.0 {
        n_classes
    } else {
        ((cfg.c * (q as f64).powf(cfg.alpha)).round() as usize).clamp(1, n_classes)
    }
}

impl Arm {
    fn new(cfg: &'static ArmCfg, q: u32, class_x: &[u32]) -> Arm {
        let n_classes = class_x.len();
        let nf = arm_nf(cfg, q, n_classes);
        Arm {
            cfg,
            nf,
            clamped: cfg.alpha < 1.0 && nf == n_classes,
            in_f: fb_subset(class_x, nf),
            rows: Vec::new(),
            next_attempt: nf + 10,
            lp_hist: [0; 3],
            too_many_lp: 0,
            attempts: 0,
            wied_runs: 0,
            wied_tries: 0,
            filter_ops: 0,
            la_ops: 0,
            extra_adds: 0,
            la_time: 0.0,
            last_complete: 0,
            last_ncols: 0,
            last_report: String::new(),
            wrong_k: 0,
            done: None,
            k_found: None,
            solve_rows: 0,
            solve_complete: 0,
            solve_ncols: 0,
        }
    }

    /// Accept a verified decomposition if it has at most two classes outside F.
    fn feed(&mut self, rel: &RelT, ell: u64, id: u64) {
        let mut small: Vec<(u32, u64)> = Vec::new();
        let mut large: Vec<(u64, u64)> = Vec::new();
        for &(cid, co) in &rel.cols {
            let cm = co.rem_euclid(ell as i64) as u64;
            if self.in_f[cid] >= 0 {
                small.push((self.in_f[cid] as u32, cm));
            } else {
                large.push((cid as u64, cm));
            }
        }
        if large.len() > 2 {
            self.too_many_lp += 1;
            return;
        }
        self.lp_hist[large.len()] += 1;
        small.push((self.nf as u32, rel.d % ell)); // the column of k
        self.rows.push(PartialRelation {
            id,
            small,
            large,
            rhs: (ell - rel.c % ell) % ell,
        });
    }

    /// One filter pass plus, when the merged rows can determine the unknowns, one sparse solve.
    /// `foreign` accumulates group additions spent here so the shared stream can exclude them.
    fn attempt(&mut self, inst: &Instance, seed: u64, foreign: &mut u64) -> bool {
        let ell = inst.ell;
        self.attempts += 1;
        self.next_attempt = self.rows.len() + 10.max(self.rows.len() / 10);
        let t0 = lp_hooks::total();
        let tl = Instant::now();
        let opts = LargePrimeFilterOptions {
            modulus: ell,
            ..LargePrimeFilterOptions::default()
        };
        let Ok((complete, rep)) = filter_large_primes(self.rows.clone(), &opts) else {
            self.filter_ops += lp_hooks::total() - t0;
            return false;
        };
        self.filter_ops += lp_hooks::total() - t0;
        self.last_report = format!(
            "{{\"peeled\":{},\"core\":{},\"merges\":{},\"cycles_closed\":{},\"complete\":{},\"dups\":{}}}",
            rep.peeled_relations, rep.core_relations, rep.merges, rep.cycles_closed,
            rep.complete_relations, rep.duplicate_relations
        );
        let mut idx: HashMap<u32, u32> = HashMap::new();
        for r in &complete {
            for &(c, _) in &r.small {
                let n = idx.len() as u32;
                idx.entry(c).or_insert(n);
            }
        }
        self.last_complete = complete.len();
        self.last_ncols = idx.len();
        if !idx.contains_key(&(self.nf as u32)) || complete.len() < idx.len() + 2 {
            self.la_time += tl.elapsed().as_secs_f64();
            return false;
        }
        let rows: Vec<Vec<(u32, u64)>> = complete
            .iter()
            .map(|r| r.small.iter().map(|&(c, k)| (idx[&c], k)).collect())
            .collect();
        let b: Vec<u64> = complete.iter().map(|r| r.rhs).collect();
        let out = sparse_la::solve(&rows, &b, idx.len(), ell, splitmix(seed ^ self.attempts));
        self.wied_runs += 1;
        self.wied_tries += out.tries as u64;
        self.la_ops += out.mulmods;
        self.la_time += tl.elapsed().as_secs_f64();
        let Some(x) = out.x else {
            return false;
        };
        let k = x[idx[&(self.nf as u32)] as usize];
        let a0 = inst.cv.adds.get();
        let ok = k != 0 && inst.cv.mul(&inst.d, k) == inst.qq;
        let da = inst.cv.adds.get() - a0;
        self.extra_adds += da;
        *foreign += da;
        if ok {
            self.k_found = Some(k);
            self.solve_rows = self.rows.len();
            self.solve_complete = complete.len();
            self.solve_ncols = idx.len();
            true
        } else {
            self.wrong_k += 1;
            false
        }
    }
}

fn run_lp(inst: &Instance, tag: u64, w_cal: f64) -> Value {
    let cv = &inst.cv;
    let ell = inst.ell;
    let t_start = Instant::now();
    let adds0 = cv.adds.get();
    let nat0 = native_now();
    let mut rng = Rng(splitmix(
        inst.q as u64 * 7_919 + inst.seed * 104_729 + tag * 31 + 1,
    ));
    let fb = build_fb(cv);
    let mut class_x = vec![0u32; fb.n_classes];
    for &(x, y) in &fb.pts {
        class_x[fb.class_of[&(x, y.a, y.b)].0] = x;
    }
    let macs = [Macaulay::new(5), Macaulay::new(6), Macaulay::new(7)];
    let pl = pool(inst, &mut rng, 16);
    let (mut r, mut c, mut d) = (pl[0].0.clone(), pl[0].1, pl[0].2);
    let mut lrng = splitmix(tag + 99);
    let trial_cap: u64 = std::env::var("LP_TRIAL_CAP")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1_500_000);
    let time_cap: f64 = std::env::var("LP_TIME_CAP_S")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1200.0);
    let mut arms: Vec<Arm> = ARMS
        .iter()
        .map(|cfg| Arm::new(cfg, inst.q, &class_x))
        .collect();
    let (mut trials, mut success_trials, mut degenerate, mut rejected, mut decomps) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut solver_native_sum, mut solver_native_n, mut solver_time) = (0u64, 0u64, 0f64);
    let mut foreign = 0u64;
    let mut rel_id = 0u64;
    let mut capped = false;
    while arms.iter().any(|a| a.done.is_none()) {
        if trials >= trial_cap || t_start.elapsed().as_secs_f64() > time_cap {
            capped = true;
            break;
        }
        let j = rng.below(16) as usize;
        r = cv.add(&r, &pl[j].0);
        c = (c + pl[j].1) % ell;
        d = (d + pl[j].2) % ell;
        trials += 1;
        if r.u.len() != 3 {
            degenerate += 1;
            continue;
        }
        let Some((eqs, w)) = equations(cv, &r) else {
            degenerate += 1;
            continue;
        };
        let ts = Instant::now();
        let n0 = native_now();
        let (out, _lvl) = solve_escalating(&macs, &cv.fld, &eqs, &mut lrng);
        solver_native_sum += native_now() - n0;
        solver_native_n += 1;
        solver_time += ts.elapsed().as_secs_f64();
        match out.status {
            Status::Insufficient => {
                degenerate += 1;
                continue;
            }
            Status::Inconsistent => continue,
            Status::Ambiguous | Status::Ok => {}
        }
        let mut rels: Vec<RelT> = Vec::new();
        for z in &out.sols {
            if let Some(dec) = decomposition_from_solution(cv, &r, &w, z, false) {
                if !verify_decomp(cv, &r, &dec) {
                    rejected += 1;
                    continue;
                }
                let mut cols: Vec<(usize, i64)> = Vec::new();
                for &(x, y, m) in &dec.pts {
                    let (cid, sign) = fb.class_of[&(x, y.a, y.b)];
                    if sign == 0 {
                        continue;
                    }
                    cols.push((cid, sign as i64 * m as i64));
                }
                rels.push(RelT { cols, c, d });
            }
        }
        if rels.is_empty() {
            continue;
        }
        success_trials += 1;
        decomps += rels.len() as u64;
        let stream_adds = cv.adds.get() - adds0 - foreign;
        let stream_native = native_now() - nat0;
        for arm in arms.iter_mut().filter(|a| a.done.is_none()) {
            let before = arm.rows.len();
            for rel in &rels {
                rel_id += 1;
                arm.feed(rel, ell, rel_id);
            }
            if arm.rows.len() > before && arm.rows.len() >= arm.next_attempt {
                let seed = splitmix(inst.q as u64 * 13 + inst.seed * 17 + tag * 19);
                if arm.attempt(inst, seed, &mut foreign) {
                    arm.done = Some(Done {
                        trial: trials,
                        adds: stream_adds,
                        native: stream_native,
                    });
                }
            }
        }
    }
    let arm_json: Vec<Value> = arms
        .iter()
        .map(|a| {
            let (t_at, adds_at, nat_at, solved) = match &a.done {
                Some(dn) => (dn.trial, dn.adds, dn.native, a.k_found == Some(inst.k)),
                None => (trials, cv.adds.get() - adds0 - foreign, native_now() - nat0, false),
            };
            let la_native = a.filter_ops + a.la_ops;
            let s = |w: f64| (adds_at as f64 + w * nat_at as f64) + (a.extra_adds as f64 + w * la_native as f64);
            let s_rel = |w: f64| adds_at as f64 + w * nat_at as f64;
            let s_la = |w: f64| a.extra_adds as f64 + w * la_native as f64;
            json!({
                "arm": a.cfg.name, "alpha": a.cfg.alpha, "c": a.cfg.c, "nf_classes": a.nf, "clamped": a.clamped,
                "f_fraction": a.nf as f64 / fb.n_classes as f64,
                "solved": solved, "censored": a.done.is_none(), "k_found": a.k_found,
                "trials_to_stop": t_at,
                "accepted_rows": a.rows.len(), "accepted_by_lp_count": a.lp_hist, "decomps_with_gt2_lp": a.too_many_lp,
                "attempts": a.attempts, "wiedemann_runs": a.wied_runs, "wiedemann_tries": a.wied_tries, "wrong_k_attempts": a.wrong_k,
                "rows_at_solve": a.solve_rows, "complete_at_solve": a.solve_complete, "ncols_at_solve": a.solve_ncols,
                "last_complete": a.last_complete, "last_ncols": a.last_ncols, "last_filter_report": a.last_report,
                "stream_group_adds": adds_at, "stream_native": nat_at,
                "arm_extra_group_adds": a.extra_adds, "filter_ops": a.filter_ops, "wiedemann_mulmods": a.la_ops,
                "la_time_s": a.la_time,
                "S_w0": s(0.0), "S_w0p1": s(0.1), "S_w1": s(1.0), "S_wcal": s(w_cal),
                "S_rel_w0": s_rel(0.0), "S_rel_w0p1": s_rel(0.1), "S_rel_w1": s_rel(1.0),
                "S_la_w0": s_la(0.0), "S_la_w0p1": s_la(0.1), "S_la_w1": s_la(1.0),
            })
        })
        .collect();
    json!({
        "arm": "ic_lp", "q": inst.q, "seed": inst.seed, "tag": tag, "capped": capped,
        "fb_points": fb.pts.len(), "fb_classes": fb.n_classes,
        "stream_trials": trials, "stream_success_trials": success_trials, "stream_verified_decomps": decomps,
        "degenerate_R": degenerate, "rejected_candidates": rejected,
        "solver_native_per_trial": if solver_native_n > 0 { solver_native_sum as f64 / solver_native_n as f64 } else { 0.0 },
        "solver_time_per_trial_s": if solver_native_n > 0 { solver_time / solver_native_n as f64 } else { 0.0 },
        "wall_s": t_start.elapsed().as_secs_f64(), "arms": arm_json,
    })
}

#[cfg(test)]
#[expect(
    clippy::items_after_test_module,
    reason = "the command entry point follows this established test module"
)]
mod tests {
    use super::*;

    /// Synthetic partial relations over a known (phi, k): merged complete rows must hold for the truth,
    /// and the sparse solver recovers k from them.
    #[test]
    fn filter_and_wiedemann_recover_k_on_synthetic_relations() {
        let ell = 1_000_000_007u64;
        let nf = 30usize;
        let nlp = 60usize;
        let mut st = 0xdead_beef_1234_5678u64;
        let mut rnd = move || {
            st ^= st << 13;
            st ^= st >> 7;
            st ^= st << 17;
            st
        };
        let phi_f: Vec<u64> = (0..nf).map(|_| rnd() % ell).collect();
        let phi_l: Vec<u64> = (0..nlp).map(|_| rnd() % ell).collect();
        let k = 123_456_789u64;
        let mut rows = Vec::new();
        for id in 0..400u64 {
            let nlarge = (rnd() % 3) as usize;
            let mut small: Vec<(u32, u64)> = Vec::new();
            let mut large: Vec<(u64, u64)> = Vec::new();
            let mut rhs = 0u64;
            let d = 1 + rnd() % (ell - 1);
            for _ in 0..(4 - nlarge) {
                let col = (rnd() % nf as u64) as u32;
                if small.iter().any(|&(c, _)| c == col) {
                    continue;
                }
                let co = 1 + rnd() % (ell - 1);
                small.push((col, co));
                rhs = (rhs + (co as u128 * phi_f[col as usize] as u128 % ell as u128) as u64) % ell;
            }
            for _ in 0..nlarge {
                let lp = rnd() % nlp as u64;
                if large.iter().any(|&(c, _)| c == lp) {
                    continue;
                }
                let co = 1 + rnd() % (ell - 1);
                large.push((lp, co));
                rhs = (rhs + (co as u128 * phi_l[lp as usize] as u128 % ell as u128) as u64) % ell;
            }
            rhs = (rhs + (d as u128 * k as u128 % ell as u128) as u64) % ell;
            small.push((nf as u32, d));
            rows.push(PartialRelation {
                id,
                small,
                large,
                rhs,
            });
        }
        let opts = LargePrimeFilterOptions {
            modulus: ell,
            ..LargePrimeFilterOptions::default()
        };
        let (complete, rep) = filter_large_primes(rows, &opts).unwrap();
        assert!(rep.cycles_closed > 0);
        let truth = |c: u32| {
            if (c as usize) < nf {
                phi_f[c as usize]
            } else {
                k
            }
        };
        for r in &complete {
            let lhs = r.small.iter().fold(0u64, |a, &(c, co)| {
                (a + (co as u128 * truth(c) as u128 % ell as u128) as u64) % ell
            });
            assert_eq!(lhs, r.rhs);
        }
        let mut idx: HashMap<u32, u32> = HashMap::new();
        for r in &complete {
            for &(c, _) in &r.small {
                let n = idx.len() as u32;
                idx.entry(c).or_insert(n);
            }
        }
        let srows: Vec<Vec<(u32, u64)>> = complete
            .iter()
            .map(|r| r.small.iter().map(|&(c, co)| (idx[&c], co)).collect())
            .collect();
        let b: Vec<u64> = complete.iter().map(|r| r.rhs).collect();
        let out = sparse_la::solve(&srows, &b, idx.len(), ell, 5);
        let x = out.x.expect("solve");
        assert_eq!(x[idx[&(nf as u32)] as usize], k);
    }

    #[test]
    fn fb_subset_is_deterministic_nested_and_sized() {
        let xs: Vec<u32> = (0..200).collect();
        let a = fb_subset(&xs, 20);
        let b = fb_subset(&xs, 50);
        assert_eq!(a.iter().filter(|&&v| v >= 0).count(), 20);
        for i in 0..200 {
            if a[i] >= 0 {
                assert!(b[i] >= 0);
            }
        }
        assert_eq!(a, fb_subset(&xs, 20));
    }

    #[test]
    fn arm_sizes() {
        assert_eq!(arm_nf(&ARMS[0], 2039, 1000), 1000);
        assert_eq!(
            arm_nf(&ARMS[1], 2039, 1020),
            (2039f64.powf(0.75)).round() as usize
        );
        assert_eq!(arm_nf(&ARMS[2], 131, 60), 60);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let git_head = std::env::var("GIT_HEAD").unwrap_or_default();
    match args.get(1).map(String::as_str) {
        Some("run") => {
            let q: u32 = args[2].parse().expect("q");
            let seed: u64 = args[3].parse().expect("seed");
            let which = args.get(4).map(String::as_str).unwrap_or("both");
            let tag: u64 = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(0);
            let t = Instant::now();
            let inst = make_instance(q, seed);
            let setup_s = t.elapsed().as_secs_f64();
            let (w_cal, t_mul, t_add) = calibrate(&inst);
            let mut out = json!({
                "git_head": git_head, "label": "[pcc:unverified]",
                "command": args.join(" "),
                "instance": instance_json(&inst), "setup_s": setup_s,
                "w_cal": w_cal, "t_fq_mulmod_s": t_mul, "t_group_add_s": t_add,
            });
            if which == "ic" || which == "both" {
                out["ic"] = run_ic(&inst, tag, w_cal);
            }
            if which == "rho" || which == "both" {
                out["rho"] = run_rho(&inst, tag, w_cal);
            }
            println!("{}", out);
        }
        Some("lp") => {
            let q: u32 = args[2].parse().expect("q");
            let seed: u64 = args[3].parse().expect("seed");
            let tag: u64 = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);
            let t = Instant::now();
            let inst = make_instance(q, seed);
            let setup_s = t.elapsed().as_secs_f64();
            let (w_cal, t_mul, t_add) = calibrate(&inst);
            let mut out = json!({
                "git_head": git_head, "label": "[pcc:unverified]",
                "command": args.join(" "),
                "instance": instance_json(&inst), "setup_s": setup_s,
                "w_cal": w_cal, "t_fq_mulmod_s": t_mul, "t_group_add_s": t_add,
            });
            out["ic_lp"] = run_lp(&inst, tag, w_cal);
            println!("{}", out);
        }
        Some("oracle") => {
            let q: u32 = args[2].parse().expect("q");
            let seed: u64 = args[3].parse().expect("seed");
            let trials: usize = args[4].parse().expect("trials");
            let fld = Fld::new(q);
            let mut rng = Rng(splitmix(q as u64 * 31 + seed));
            let cv = loop {
                let f = Curve::random_f(&fld, &mut rng);
                let c = Curve::new(fld.clone(), f);
                if c.is_squarefree() {
                    break c;
                }
            };
            let macs = [Macaulay::new(5), Macaulay::new(6), Macaulay::new(7)];
            let fb = build_fb(&cv);
            let mut lr = 2024u64;
            let (mut tested, mut agree, mut degenerate, mut skipped_root, mut found_oracle) =
                (0u64, 0u64, 0u64, 0u64, 0u64);
            let mut found_solver_verified = 0u64;
            for _ in 0..trials {
                let r = cv.random_elt(&mut rng);
                let Some((eqs, w)) = equations(&cv, &r) else {
                    continue;
                };
                if (0..q).any(|x| cv.fld.peval(&r.u, fld.embed(x)) == ZERO) {
                    skipped_root += 1;
                    continue;
                }
                let (out, _) = solve_escalating(&macs, &fld, &eqs, &mut lr);
                if out.status == Status::Insufficient {
                    degenerate += 1;
                    continue;
                }
                tested += 1;
                let mut sv: Vec<(E, E)> = Vec::new();
                for z in &out.sols {
                    if let Some(dec) = decomposition_from_solution(&cv, &r, &w, z, true) {
                        if verify_decomp(&cv, &r, &dec) {
                            found_solver_verified += 1;
                        }
                        sv.push(dec.ab);
                    }
                }
                sv.sort_by_key(|&(a, b)| (a.a, a.b, b.a, b.b));
                let orc = oracle(&cv, &r, &fb);
                found_oracle += orc.len() as u64;
                if sv == orc {
                    agree += 1;
                }
            }
            println!(
                "{}",
                json!({"git_head": git_head, "label": "[pcc:unverified]", "command": args.join(" "),
                    "q": q, "seed": seed, "trials_requested": trials, "tested_nondegenerate": tested,
                    "agree": agree, "degenerate_R": degenerate, "R_with_root_of_u_in_Fq_skipped": skipped_root,
                    "oracle_decompositions": found_oracle, "solver_decompositions_group_law_verified": found_solver_verified})
            );
        }
        _ => eprintln!("usage: extfield_jac_decomp run <q> <seed> [ic|rho|both] [tag] | oracle <q> <seed> <trials>"),
    }
}
