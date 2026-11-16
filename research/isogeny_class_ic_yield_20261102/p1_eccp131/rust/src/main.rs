//! P1 instrument: decomposition-system measurements on ECCp-131 and its isogeny-class neighbours.
//!
//!   p1_eccp131 selftest
//!   p1_eccp131 run --targets targets.json --tower tower.json --out rows.jsonl [options]
//!   p1_eccp131 analyze --rows rows.jsonl [--out analysis.json]
//!
//! See PROTOCOL.md.  Formulation: the PKM isogeny-tower factor base (the repository's
//! `prime-isogeny` base) with the Semaev decomposition system at m = 2 and m = 3.

pub(crate) mod analyze;
pub(crate) mod engine;
pub(crate) mod fp;
pub(crate) mod poly;
mod pari;
mod jsonv;
mod construct;

use fp::Fp;
use poly::Poly;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

// ---------------------------------------------------------------- derivation

fn derive(seed: u64, label: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(format!("{seed}|{label}").as_bytes());
    h.finalize().into()
}

fn derive_u64(seed: u64, label: &str) -> u64 {
    let d = derive(seed, label);
    u64::from_be_bytes(d[..8].try_into().unwrap())
}

fn derive_fp(seed: u64, label: &str) -> Fp {
    let mut bytes = derive(seed, &format!("{label}|0")).to_vec();
    bytes.extend_from_slice(&derive(seed, &format!("{label}|1")));
    Fp::from_bytes_mod(&bytes)
}

fn shuffle<T>(v: &mut [T], seed: u64, label: &str) {
    for i in (1..v.len()).rev() {
        let j = (derive_u64(seed, &format!("{label}|{i}")) % (i as u64 + 1)) as usize;
        v.swap(i, j);
    }
}

pub fn sha_hex(s: &str) -> String {
    Sha256::digest(s.as_bytes()).iter().map(|b| format!("{b:02x}")).collect()
}

fn sha_file(path: &str) -> String {
    let data = std::fs::read(path).unwrap_or_default();
    Sha256::digest(&data).iter().map(|b| format!("{b:02x}")).collect()
}

// ---------------------------------------------------------------- tower, curve

#[derive(Clone)]
pub(crate) struct Tower {
    pub(crate) t: usize,
    pub(crate) c: Fp,
    pub(crate) stages: Vec<([Fp; 3], [Fp; 2])>,
    pub(crate) v: Vec<Fp>,
}

pub(crate) fn load_tower(j: &Value, t: usize) -> Tower {
    let tj = &j["towers"][t.to_string()];
    let h = |x: &Value| Fp::from_hex(x.as_str().unwrap());
    let stages = tj["stages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| {
            let n = &s["num_low_to_high"];
            let d = &s["den_low_to_high"];
            ([h(&n[0]), h(&n[1]), h(&n[2])], [h(&d[0]), h(&d[1])])
        })
        .collect();
    let v = tj["V_sorted"].as_array().unwrap().iter().map(h).collect();
    Tower { t, c: h(&tj["c"]), stages, v }
}

pub(crate) fn rhs(x: Fp, a: Fp, b: Fp) -> Fp {
    x.mul(x).mul(x).add(a.mul(x)).add(b)
}

pub(crate) fn fb_abscissae(tw: &Tower, a: Fp, b: Fp) -> Vec<Fp> {
    tw.v.iter().copied().filter(|&x| rhs(x, a, b).is_square()).collect()
}

pub(crate) type Pt = Option<(Fp, Fp)>;

pub(crate) fn ec_add(p1: Pt, p2: Pt, a: Fp) -> Pt {
    match (p1, p2) {
        (None, q) | (q, None) => q,
        (Some((x1, y1)), Some((x2, y2))) => {
            let lam = if x1 == x2 {
                if y1.add(y2).is_zero() {
                    return None;
                }
                let three = Fp::from_u64(3);
                three.mul(x1.sq()).add(a).mul(y1.add(y1).inv())
            } else {
                y2.sub(y1).mul(x2.sub(x1).inv())
            };
            let x3 = lam.sq().sub(x1).sub(x2);
            let y3 = lam.mul(x1.sub(x3)).sub(y1);
            Some((x3, y3))
        }
    }
}

// ---------------------------------------------------------------- systems

#[derive(Clone, Copy, Debug)]
pub(crate) struct Cfg {
    pub(crate) m: usize,
    pub(crate) t: usize,
}

impl Cfg {
    fn parse(s: &str) -> Cfg {
        // m2t3
        let s = s.trim_start_matches('m');
        let (m, t) = s.split_once('t').expect("config like m2t3");
        Cfg { m: m.parse().unwrap(), t: t.parse().unwrap() }
    }
    fn name(&self) -> String {
        format!("m{}t{}", self.m, self.t)
    }
    pub(crate) fn nvars(&self) -> usize {
        self.m * self.t + usize::from(self.m == 3)
    }
}

fn s3(x1: &Poly, x2: &Poly, x3: &Poly, a: Fp, b: Fp) -> Poly {
    let two = Fp::from_u64(2);
    let four = Fp::from_u64(4);
    let d = x1.sub(x2);
    let t1 = d.mul(&d).mul(&x3.mul(x3));
    let s = x1.add(x2);
    let pr = x1.mul(x2);
    let inner = s.mul(&pr.add(&Poly::constant(a))).add(&Poly::constant(two.mul(b)));
    let t2 = inner.mul(x3).scale(two.neg());
    let e = pr.sub(&Poly::constant(a));
    let t3 = e.mul(&e).sub(&s.scale(four.mul(b)));
    t1.add(&t2).add(&t3)
}

/// S3(x1, x2, x3) as a quadratic in x3: coefficients low to high.
fn s3_quad(x1: Fp, x2: Fp, a: Fp, b: Fp) -> [Fp; 3] {
    let two = Fp::from_u64(2);
    let four = Fp::from_u64(4);
    let d = x1.sub(x2);
    let s = x1.add(x2);
    let pr = x1.mul(x2);
    let c2 = d.sq();
    let c1 = two.neg().mul(s.mul(pr.add(a)).add(two.mul(b)));
    let e = pr.sub(a);
    let c0 = e.sq().sub(four.mul(b).mul(s));
    [c0, c1, c2]
}

fn tower_poly(u: &Poly, unext: &Poly, st: &([Fp; 3], [Fp; 2])) -> Poly {
    let (n, d) = st;
    let num = Poly::constant(n[0]).add(&u.scale(n[1])).add(&u.mul(u).scale(n[2]));
    let den = Poly::constant(d[0]).add(&u.scale(d[1]));
    unext.mul(&den).sub(&num)
}

/// Logical variable index: element e, level j -> e*t + j; the auxiliary partial sum w -> m*t.
pub(crate) fn build_system(cfg: Cfg, tw: &Tower, a: Fp, b: Fp, xr: Fp, perm: &[usize]) -> Vec<Poly> {
    let v = |idx: usize| Poly::var(perm[idx]);
    let mut polys = Vec::new();
    for e in 0..cfg.m {
        for j in 0..cfg.t {
            let u = v(e * cfg.t + j);
            let unext = if j + 1 < cfg.t { v(e * cfg.t + j + 1) } else { Poly::constant(tw.c) };
            polys.push(tower_poly(&u, &unext, &tw.stages[j]));
        }
    }
    let x = |e: usize| v(e * cfg.t);
    let xrp = Poly::constant(xr);
    if cfg.m == 2 {
        polys.push(s3(&x(0), &x(1), &xrp, a, b));
    } else {
        let w = v(cfg.m * cfg.t);
        polys.push(s3(&x(0), &x(1), &w, a, b));
        polys.push(s3(&w, &x(2), &xrp, a, b));
    }
    polys
}

fn x_map(st: &([Fp; 3], [Fp; 2]), u: Fp) -> Fp {
    let (n, d) = st;
    let num = n[0].add(n[1].mul(u)).add(n[2].mul(u.sq()));
    let den = d[0].add(d[1].mul(u));
    num.mul(den.inv())
}

/// Values of every variable (by position) at the planted decomposition.
pub(crate) fn assignment(cfg: Cfg, tw: &Tower, xs: &[Fp], w: Option<Fp>, perm: &[usize]) -> Vec<Fp> {
    let mut vals = vec![Fp::ZERO; cfg.nvars()];
    for e in 0..cfg.m {
        let mut u = xs[e];
        for j in 0..cfg.t {
            vals[perm[e * cfg.t + j]] = u;
            if j + 1 < cfg.t {
                u = x_map(&tw.stages[j], u);
            }
        }
    }
    if let Some(w) = w {
        vals[perm[cfg.m * cfg.t]] = w;
    }
    vals
}

fn poly_trim(mut v: Vec<Fp>) -> Vec<Fp> {
    while v.last().map_or(false, |c| c.is_zero()) {
        v.pop();
    }
    v
}

fn poly_rem(a: &[Fp], b: &[Fp]) -> Vec<Fp> {
    let mut r = a.to_vec();
    let inv = b.last().unwrap().inv();
    while r.len() >= b.len() {
        let f = r.last().unwrap().mul(inv);
        let sh = r.len() - b.len();
        for (i, c) in b.iter().enumerate() {
            r[sh + i] = r[sh + i].sub(f.mul(*c));
        }
        r = poly_trim(r);
        if r.is_empty() {
            break;
        }
    }
    r
}

fn distinct_common_roots(f: &[Fp; 3], g: &[Fp; 3]) -> usize {
    let mut a = poly_trim(f.to_vec());
    let mut b = poly_trim(g.to_vec());
    if a.is_empty() && b.is_empty() {
        return usize::MAX;
    }
    while !b.is_empty() {
        let r = poly_rem(&a, &b);
        a = b;
        b = r;
    }
    match a.len() {
        0 | 1 => 0,
        2 => 1,
        _ => {
            let four = Fp::from_u64(4);
            let disc = a[1].sq().sub(four.mul(a[0]).mul(a[2]));
            if disc.is_zero() {
                1
            } else {
                2
            }
        }
    }
}

/// Exact number of solutions of the system over the algebraic closure (the tower fixes x_i in V).
pub(crate) fn count_solutions(cfg: Cfg, tw: &Tower, a: Fp, b: Fp, xr: Fp) -> usize {
    let v = &tw.v;
    let mut k = 0usize;
    if cfg.m == 2 {
        for &x1 in v {
            for &x2 in v {
                let q = s3_quad(x1, x2, a, b);
                let val = q[0].add(q[1].mul(xr)).add(q[2].mul(xr.sq()));
                if val.is_zero() {
                    k += 1;
                }
            }
        }
    } else {
        // precompute the third factor's quadratic per x3
        let q2: Vec<[Fp; 3]> = v.iter().map(|&x3| s3_quad(x3, xr, a, b)).collect();
        for &x1 in v {
            for &x2 in v {
                let q1 = s3_quad(x1, x2, a, b);
                for q in &q2 {
                    k += distinct_common_roots(&q1, q);
                }
            }
        }
    }
    k
}

// ---------------------------------------------------------------- arms and instances

#[derive(Clone)]
struct Arm {
    id: String,
    role: String,
    a: Fp,
    b: Fp,
    slug: String,
    group: String, // planted-instance seed group (A/A arms share E0's)
    perm_arm: bool,
    xs: Option<Vec<Fp>>, // transported x_R per tau
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Null,
    Planted,
    Ladder,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Null => "null",
            Kind::Planted => "planted",
            Kind::Ladder => "ladder",
        }
    }
}

#[derive(Clone)]
struct Job {
    order_index: usize,
    round: usize,
    rep: usize,
    arm: usize,
    cfg: Cfg,
    kind: Kind,
    tau: usize,
}

fn random_point_x(seed: u64, label: &str, a: Fp, b: Fp) -> (Fp, Fp) {
    for i in 0..1000 {
        let x = derive_fp(seed, &format!("{label}|x{i}"));
        if let Some(y) = rhs(x, a, b).sqrt() {
            return (x, y);
        }
    }
    panic!("no point");
}

struct Ctx {
    seed: u64,
    n_f64: f64,
    arms: Vec<Arm>,
    towers: HashMap<usize, Tower>,
    limits_dcap: u32,
    max_cols: usize,
    budget_s: f64,
    ladder_g: usize,
}

fn log2_binom(n: usize, k: usize) -> f64 {
    if n < k {
        return f64::NEG_INFINITY;
    }
    let mut r = 0.0f64;
    for i in 0..k {
        r += ((n - i) as f64).log2() - ((i + 1) as f64).log2();
    }
    r
}

fn run_job(cx: &Ctx, job: &Job) -> Value {
    let arm = &cx.arms[job.arm];
    let cfg = job.cfg;
    let tw = &cx.towers[&cfg.t];
    let n = cfg.nvars();
    let mut perm: Vec<usize> = (0..n).collect();
    if arm.perm_arm {
        shuffle(&mut perm, cx.seed, &format!("perm|{}", cfg.name()));
    }
    let fb = fb_abscissae(tw, arm.a, arm.b);
    let inst_seed = cx.seed;
    let lab = format!("{}|{}|{}|{}", arm.group, cfg.name(), job.kind.name(), job.tau);
    let mut planted_ok = Value::Null;
    let mut planted_info = Value::Null;
    let mut planted_vals: Option<Vec<Fp>> = None;
    let xr: Fp;
    let mut extra: Vec<Poly> = Vec::new();
    match job.kind {
        Kind::Null => {
            xr = match &arm.xs {
                Some(xs) => xs[job.tau],
                None => random_point_x(inst_seed, &format!("nullR|{}|{}", arm.group, job.tau), arm.a, arm.b).0,
            };
        }
        Kind::Planted | Kind::Ladder => {
            // choose cfg.m distinct factor-base abscissae and signs, sum the points
            let base_lab = format!("{}|{}|planted|{}", arm.group, cfg.name(), job.tau);
            let mut idx: Vec<usize> = (0..fb.len()).collect();
            if fb.len() < cfg.m {
                return json!({"schema": "p1.row/v1", "order_index": job.order_index, "rep": job.rep, "arm": arm.id, "role": arm.role,
                    "config": cfg.name(), "kind": job.kind.name(), "tau": job.tau,
                    "error": "planted instance not formed: factor base on this curve has fewer abscissae than m", "factor_base_abscissae_on_curve": fb.len()});
            }
            shuffle(&mut idx, inst_seed, &format!("{base_lab}|pick"));
            let mut pts: Vec<(Fp, Fp)> = Vec::new();
            let mut xs: Vec<Fp> = Vec::new();
            for e in 0..cfg.m {
                let x = fb[idx[e]];
                let mut y = rhs(x, arm.a, arm.b).sqrt().unwrap();
                if derive_u64(inst_seed, &format!("{base_lab}|sign{e}")) & 1 == 1 {
                    y = y.neg();
                }
                pts.push((x, y));
                xs.push(x);
            }
            let s12 = ec_add(Some(pts[0]), Some(pts[1]), arm.a).expect("sum is infinity");
            let (sum, w) = if cfg.m == 2 {
                (s12, None)
            } else {
                (ec_add(Some(s12), Some(pts[2]), arm.a).expect("sum is infinity"), Some(s12.0))
            };
            xr = sum.0;
            let vals = assignment(cfg, tw, &xs, w, &perm);
            planted_info = json!({"x_base": xs.iter().map(|x| x.to_hex()).collect::<Vec<_>>()});
            planted_vals = Some(vals);
            if job.kind == Kind::Ladder {
                let vals = planted_vals.as_ref().unwrap();
                for g in 0..cx.ladder_g.saturating_sub(1) {
                    // random quadric (all monomials of degree <= 2) vanishing at the planted point
                    let mut q = Poly::zero();
                    for i in 0..n {
                        for j in i..n {
                            let c = derive_fp(inst_seed, &format!("{lab}|ladder{g}|q{i}|{j}"));
                            let m = Poly::var(i).mul(&Poly::var(j));
                            q = q.add(&m.scale(c));
                        }
                        let c = derive_fp(inst_seed, &format!("{lab}|ladder{g}|l{i}"));
                        q = q.add(&Poly::var(i).scale(c));
                    }
                    let c0 = q.eval(vals).neg();
                    q = q.add(&Poly::constant(c0));
                    extra.push(q);
                }
            }
        }
    }
    let mut polys = build_system(cfg, tw, arm.a, arm.b, xr, &perm);
    if let Some(vals) = &planted_vals {
        let ok = polys.iter().all(|p| p.eval(vals).is_zero());
        planted_ok = json!(ok);
        assert!(ok, "planted point does not satisfy the system: {} {}", arm.id, lab);
    }
    polys.extend(extra);
    let degs: Vec<u32> = polys.iter().map(|p| p.degree()).collect();
    let k_sol = count_solutions(cfg, tw, arm.a, arm.b, xr);
    if job.kind == Kind::Ladder {
        // extra quadrics cut the solution set: recount by evaluating the planted point only
        // (the exact count of the augmented system is not enumerated; K is left unset)
    }
    let k_for_engine = if job.kind == Kind::Ladder { 1 } else { k_sol };
    let lim = engine::Limits { dcap: cx.limits_dcap, max_cols: cx.max_cols, budget_s: cx.budget_s };
    let wall0 = Instant::now();
    let cpu0 = engine::thread_cpu_s();
    let out = engine::run(&polys, n, k_for_engine, &lim);
    let cpu = engine::thread_cpu_s() - cpu0;
    let wall = wall0.elapsed().as_secs_f64();
    let sr = engine::semiregular_degree(&degs, n);
    let b_c = fb.len();
    let pred_log2 = (cfg.m as f64) + log2_binom(b_c, cfg.m) - cx.n_f64.log2();
    let per_degree: Vec<Value> = out
        .stats
        .iter()
        .map(|s| {
            json!({"D": s.d, "rows_added": s.rows_added, "rows_cum": s.rows_cum, "cols": s.cols, "rank": s.rank,
                   "new_pivots": s.new_pivots, "new_pivots_below_D": s.new_pivots_below_d, "zero_reductions": s.zero_red,
                   "nnz_pivots": s.nnz_pivots, "n_standard_monomials": s.nstd, "pure_power_vars": s.pure_powers,
                   "constant_is_pivot": s.const_pivot, "field_mults": s.mults, "row_ops": s.row_ops, "cpu_s": s.cpu_s})
        })
        .collect();
    json!({
        "schema": "p1.row/v1",
        "order_index": job.order_index, "round": job.round, "rep": job.rep,
        "arm": arm.id, "role": arm.role, "curve_slug": arm.slug,
        "config": cfg.name(), "m": cfg.m, "t": cfg.t, "n_vars": n, "n_eqs": polys.len(), "eq_degrees": degs,
        "kind": job.kind.name(), "tau": job.tau, "x_R": xr.to_hex(),
        "variable_order": if arm.perm_arm { "seeded_permutation" } else { "natural" },
        "factor_base_abscissae_on_curve": b_c, "factor_base_points": 2 * b_c, "tower_size": tw.v.len(),
        "predicted_yield_log2_per_trial": pred_log2,
        "K_exact_solutions": if job.kind == Kind::Ladder { Value::Null } else { json!(k_sol) },
        "planted_verified": planted_ok, "planted": planted_info,
        "status": out.status, "D_solve": out.d_solve, "D_max_reached": out.d_max,
        "first_fall_degree_obs": out.first_fall, "D_semiregular_ref": sr,
        "inconsistent_detected": out.inconsistent, "n_standard_monomials_final": out.nstd_final,
        "final_rows": out.final_rows, "final_cols": out.final_cols, "final_rank": out.final_rank,
        "rank_profile_by_leading_degree": out.pivots_by_lead_degree,
        "field_mults": out.mults, "row_ops": out.row_ops, "cpu_s": cpu, "wall_s": wall,
        "per_degree": per_degree,
    })
}

// ---------------------------------------------------------------- CLI

pub fn arg(args: &[String], name: &str, default: &str) -> String {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned().unwrap_or_else(|| default.to_string())
}

fn cmd_run(args: &[String]) {
    let tj: Value = serde_json::from_str(&std::fs::read_to_string(arg(args, "--targets", "targets.json")).unwrap()).unwrap();
    let tower_path = arg(args, "--tower", "tower.json");
    let tr: Value = serde_json::from_str(&std::fs::read_to_string(&tower_path).unwrap()).unwrap();
    fp::set_modulus(tj["p"].as_str().unwrap());
    assert_eq!(tr["p"].as_str().unwrap(), tj["p"].as_str().unwrap());
    let seed: u64 = arg(args, "--seed", "20261102").parse().unwrap();
    let cfgs: Vec<Cfg> = arg(args, "--configs", "m2t3").split(',').map(Cfg::parse).collect();
    let nt: usize = arg(args, "--ntargets", "4").parse().unwrap();
    let npl: usize = arg(args, "--nplanted", "2").parse().unwrap();
    let reps: usize = arg(args, "--reps", "1").parse().unwrap();
    let nrnd: usize = arg(args, "--rnd", "0").parse().unwrap();
    let workers: usize = arg(args, "--workers", "1").parse().unwrap();
    let dcap: u32 = arg(args, "--dcap", "8").parse().unwrap();
    let max_cols: usize = arg(args, "--max-cols", "60000").parse().unwrap();
    let budget_s: f64 = arg(args, "--budget-s", "60").parse().unwrap();
    let ladder_g: usize = arg(args, "--ladder-g", "2").parse().unwrap();
    let kinds_s = arg(args, "--kinds", "null,planted,ladder");
    let arms_filter: Vec<String> = arg(args, "--arms", "all").split(',').map(String::from).collect();
    let out_path = arg(args, "--out", "rows.jsonl");

    let mut arms: Vec<Arm> = Vec::new();
    let all_targets = tj["targets"].as_array().unwrap();
    for aj in tj["arms"].as_array().unwrap() {
        let id = aj["arm"].as_str().unwrap().to_string();
        if arms_filter[0] != "all" && !arms_filter.contains(&id) {
            continue;
        }
        let xs: Vec<Fp> = all_targets.iter().map(|t| Fp::from_hex(t["x_by_arm"][&id].as_str().unwrap())).collect();
        let group = if id == "AA_repeat" || id == "AA_perm" { "E0".to_string() } else { id.clone() };
        arms.push(Arm {
            id: id.clone(),
            role: aj["role"].as_str().unwrap().into(),
            a: Fp::from_hex(aj["a"].as_str().unwrap()),
            b: Fp::from_hex(aj["b"].as_str().unwrap()),
            slug: aj["slug"].as_str().unwrap().into(),
            group,
            perm_arm: id == "AA_perm",
            xs: Some(xs),
        });
    }
    for i in 0..nrnd {
        let a = derive_fp(seed, &format!("rnd|{i}|a"));
        let b = derive_fp(seed, &format!("rnd|{i}|b"));
        arms.push(Arm {
            id: format!("rnd{i}"),
            role: "random_curve_null_population".into(),
            a,
            b,
            slug: format!("random-curve-{i}"),
            group: format!("rnd{i}"),
            perm_arm: false,
            xs: None,
        });
    }
    let nspecial: usize = arg(args, "--special", "0").parse().unwrap();
    for i in 0..nspecial {
        // exploratory coefficient-structure controls: a = 0 (j = 0) and b = 0 (j = 1728)
        let r = derive_fp(seed, &format!("special|{i}"));
        let (a0, b0, nm) = (Fp::ZERO, r, "sp_j0");
        let (a1, b1, nm1) = (r, Fp::ZERO, "sp_j1728");
        for (a, b, name) in [(a0, b0, nm), (a1, b1, nm1)] {
            arms.push(Arm {
                id: format!("{name}_{i}"),
                role: "special_coefficient_control".into(),
                a,
                b,
                slug: format!("{name}-{i}"),
                group: format!("{name}_{i}"),
                perm_arm: false,
                xs: None,
            });
        }
    }
    let mut towers = HashMap::new();
    for c in &cfgs {
        towers.entry(c.t).or_insert_with(|| load_tower(&tr, c.t));
    }
    let n_f64 = {
        let r = fp::parse_hex_raw(tj["n"].as_str().unwrap());
        r[0] as f64 + (r[1] as f64) * 2f64.powi(64) + (r[2] as f64) * 2f64.powi(128)
    };
    let cx = Arc::new(Ctx { seed, n_f64, arms: arms.clone(), towers, limits_dcap: dcap, max_cols, budget_s, ladder_g });

    // schedule: slots (config, kind, tau), repeated `reps` times; arms shuffled within each slot
    let kinds: Vec<Kind> = kinds_s
        .split(',')
        .map(|k| match k {
            "null" => Kind::Null,
            "planted" => Kind::Planted,
            "ladder" => Kind::Ladder,
            _ => panic!("kind"),
        })
        .collect();
    let mut jobs: Vec<Job> = Vec::new();
    let mut round = 0usize;
    for rep in 0..reps {
        for cfg in &cfgs {
            for &kind in &kinds {
                let ntau = if kind == Kind::Null { nt } else { npl };
                for tau in 0..ntau {
                    let mut order: Vec<usize> = (0..arms.len()).collect();
                    if kind == Kind::Ladder {
                        order.retain(|&i| arms[i].id == "E0" || arms[i].id == "AA_repeat");
                    }
                    shuffle(&mut order, seed, &format!("sched|{rep}|{}|{}|{tau}", cfg.name(), kind.name()));
                    for arm in order {
                        jobs.push(Job { order_index: jobs.len(), round, rep, arm, cfg: *cfg, kind, tau });
                    }
                    round += 1;
                }
            }
        }
    }
    let header = json!({
        "schema": "p1.header/v1", "seed": seed, "configs": cfgs.iter().map(|c| c.name()).collect::<Vec<_>>(),
        "ntargets": nt, "nplanted": npl, "reps": reps, "rnd_curves": nrnd, "workers": workers,
        "dcap": dcap, "max_cols": max_cols, "budget_s": budget_s, "kinds": kinds_s, "arms": arms.iter().map(|a| a.id.clone()).collect::<Vec<_>>(),
        "n_jobs": jobs.len(), "targets_sha256": sha_file(&arg(args, "--targets", "targets.json")), "tower_sha256": sha_file(&tower_path),
        "binary_sha256": sha_file(&std::env::current_exe().map(|p| p.to_string_lossy().to_string()).unwrap_or_default()),
        "args": args, "cpu": std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default().lines().find(|l| l.starts_with("model name")).unwrap_or("").to_string(),
        "nproc": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(0),
        "engine": "xl-incremental-sparse-echelon (fp.rs 3x64 Montgomery), see PROTOCOL.md",
        "order_rule": "per (rep, config, kind, tau) slot the arm order is a SHA-256-derived shuffle",
    });
    let file = Arc::new(Mutex::new(std::fs::File::create(&out_path).unwrap()));
    writeln!(file.lock().unwrap(), "{header}").unwrap();
    let jobs = Arc::new(jobs);
    let next = Arc::new(AtomicUsize::new(0));
    let t0 = Instant::now();
    let mut hs = Vec::new();
    for _ in 0..workers {
        let (cx, jobs, next, file) = (cx.clone(), jobs.clone(), next.clone(), file.clone());
        hs.push(std::thread::spawn(move || loop {
            let i = next.fetch_add(1, Ordering::SeqCst);
            if i >= jobs.len() {
                break;
            }
            let row = run_job(&cx, &jobs[i]);
            let mut f = file.lock().unwrap();
            writeln!(f, "{row}").unwrap();
            eprintln!(
                "[{}/{}] {} {} {} tau={} status={} D={:?} cpu={:.2}s",
                i + 1,
                jobs.len(),
                row["arm"],
                row["config"],
                row["kind"],
                row["tau"],
                row["status"],
                row["D_solve"],
                row["cpu_s"].as_f64().unwrap_or(0.0)
            );
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    eprintln!("done {} jobs in {:.1}s", jobs.len(), t0.elapsed().as_secs_f64());
}

fn cmd_selftest() {
    fp::set_modulus("0x48e1d43f293469e33194c43186b3abc0b");
    // field laws and an end-to-end toy: a tiny system with a known solution count
    let x = Fp::from_u64(3);
    assert_eq!(x.mul(x.inv()), Fp::one());
    let n = 3usize;
    // x^2 - 1, y^2 - 4, z - x*y : 4 solutions
    let one = Poly::constant(Fp::one());
    let (px, py, pz) = (Poly::var(0), Poly::var(1), Poly::var(2));
    let polys = vec![
        px.mul(&px).sub(&one),
        py.mul(&py).sub(&Poly::constant(Fp::from_u64(4))),
        pz.sub(&px.mul(&py)),
    ];
    let lim = engine::Limits { dcap: 6, max_cols: 10000, budget_s: 10.0 };
    let o = engine::run(&polys, n, 4, &lim);
    assert_eq!(o.status, "solved", "{:?}", o.status);
    // inconsistent: x^2 - 1 and x - 2
    let polys = vec![px.mul(&px).sub(&one), px.sub(&Poly::constant(Fp::from_u64(2)))];
    let o = engine::run(&polys, 1, 0, &lim);
    assert_eq!(o.status, "solved");
    assert!(o.inconsistent);
    println!("selftest ok");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("selftest") => cmd_selftest(),
        Some("run") => cmd_run(&args[2..]),
        Some("analyze") => analyze::cmd_analyze(&args[2..]),
        Some("build-neighbours") => construct::cmd_build_neighbours(&args[2..]),
        Some("build-tower") => construct::cmd_build_tower(&args[2..]),
        Some("make-targets") => construct::cmd_make_targets(&args[2..]),
        Some("jsoncmp") => construct::cmd_jsoncmp(&args[2..]),
        Some("gp") => {
            let p = pari::Pari::open(&pari::find_libpari(), 1usize << 30).unwrap();
            println!("{}", p.eval(&args[2]).unwrap());
        }
        _ => eprintln!("usage: p1_eccp131 selftest | run ... | analyze --rows rows.jsonl"),
    }
}
