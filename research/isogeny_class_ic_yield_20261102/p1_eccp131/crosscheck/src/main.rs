//! Independent cross-check of the P1 Macaulay/XL engine (`../rust/src/engine.rs`) against the
//! repository's F4 over F_p (`src/cryptanalysis/f4_fp.rs`, p < 2^63).
//!
//! The P1 binary's own system builder (`build_system`, `assignment`, `count_solutions`, the Montgomery
//! field and the engine) is compiled into this crate unchanged by `#[path]` and is run at primes below
//! 2^61, where `f4_fp` can solve the very same polynomial systems.  Per system it compares
//!
//!   * the exact F_p solution set: ground truth by enumeration of the tower (independent of both
//!     solvers) against `f4_fp::solve`; the engine produces counts, not points, so its count is compared
//!     (`n_standard_monomials_final`) with the quotient dimension of the F4 basis and with the exact
//!     solution count over the closure `K` of `count_solutions`;
//!   * the dimension of the quotient (standard monomials of the F4 reduced basis);
//!   * the degree at which each solver finishes (engine `D_solve`, F4 `solving_degree`,
//!     `solving_degree_max`, `degree_reached`), against the inclusion relation proved in CROSSCHECK.md;
//!   * the per-degree rank / pivot / staircase profile of the engine against an independent dense
//!     Gaussian elimination of the same Macaulay matrix written here (own monomial order, own
//!     arithmetic), for every degree whose matrix is small enough.
//!
//!   p1_crosscheck --p DEC --tower tower.json [--configs m2t3,m3t2] [--curves 3] [--null 2]
//!                 [--planted 2] [--abscissa 2] [--ladder 1] [--dense-maxcols 2000] [--f4-budget-s 120]
//!                 [--seed S] [--out rows.jsonl]
#![allow(dead_code)]

#[path = "../../rust/src/main.rs"]
mod p1;
pub use p1::{arg, sha_hex};
pub(crate) use p1::{engine, fp, poly};

use crypto::cryptanalysis::f4_fp::{self, F4Options, Ordering as FOrd, Verdict};
use fp::Fp;
use p1::{assignment, build_system, count_solutions, ec_add, fb_abscissae, load_tower, rhs, Cfg, Tower};
use poly::{mono_exp, Poly};
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashMap};
use std::io::Write;
use std::time::{Duration, Instant};

// ------------------------------------------------------------------ small helpers

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn fp(&mut self, p: u64) -> Fp {
        Fp::from_u64(self.next() % p)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn raw(c: Fp) -> u64 {
    let r = c.to_raw();
    assert!(r[1] == 0 && r[2] == 0, "value does not fit 64 bits");
    r[0]
}

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn subm(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}
fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
fn invm(a: u64, p: u64) -> u64 {
    powm(a, p - 2, p)
}

fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 {
            return n == q;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powm(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulm(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

// ------------------------------------------------------------------ engine Poly -> f4_fp Poly

fn to_f4(p: &Poly, n: usize, modulus: u64) -> f4_fp::Poly {
    let terms: Vec<(Vec<u32>, u64)> = p.t.iter().map(|(m, c)| ((0..n).map(|i| mono_exp(*m, i)).collect(), raw(*c))).collect();
    f4_fp::normalise(&terms, modulus, FOrd::Grevlex)
}

/// terms as (exponent vector, coefficient) for the dense reference (no normalisation)
fn terms_of(p: &Poly, n: usize) -> Vec<(Vec<u8>, u64)> {
    p.t.iter().map(|(m, c)| ((0..n).map(|i| mono_exp(*m, i) as u8).collect(), raw(*c))).collect()
}

// ------------------------------------------------------------------ quotient dimension of an F4 basis

/// (number of standard monomials, per degree) of the leading-monomial ideal; None if positive-dimensional.
fn f4_std(basis: &[f4_fp::Poly], n: usize) -> Option<(usize, Vec<usize>)> {
    let lms: Vec<&Vec<u32>> = basis.iter().filter(|g| !g.is_empty()).map(|g| &g[0].0).collect();
    if lms.iter().any(|m| m.iter().all(|&e| e == 0)) {
        return Some((0, vec![]));
    }
    let mut bound = vec![u32::MAX; n];
    for m in &lms {
        let nz: Vec<usize> = (0..n).filter(|&i| m[i] > 0).collect();
        if nz.len() == 1 {
            bound[nz[0]] = bound[nz[0]].min(m[nz[0]]);
        }
    }
    if bound.iter().any(|&b| b == u32::MAX) {
        return None;
    }
    let mut by_deg: Vec<usize> = Vec::new();
    let mut cur = vec![0u32; n];
    fn rec(i: usize, n: usize, bound: &[u32], cur: &mut Vec<u32>, lms: &[&Vec<u32>], by_deg: &mut Vec<usize>) {
        if i == n {
            if lms.iter().all(|m| !(0..n).all(|k| m[k] <= cur[k])) {
                let d: u32 = cur.iter().sum();
                if by_deg.len() <= d as usize {
                    by_deg.resize(d as usize + 1, 0);
                }
                by_deg[d as usize] += 1;
            }
            return;
        }
        for e in 0..bound[i] {
            cur[i] = e;
            rec(i + 1, n, bound, cur, lms, by_deg);
        }
        cur[i] = 0;
    }
    rec(0, n, &bound, &mut cur, &lms, &mut by_deg);
    Some((by_deg.iter().sum(), by_deg))
}

// ------------------------------------------------------------------ independent dense Macaulay reference

struct DenseStat {
    d: u32,
    rows_cum: usize,
    cols: usize,
    rank: usize,
    new_pivots: usize,
    new_pivots_below_d: usize,
    zero_red: usize,
    nstd: usize,
    pure_powers: usize,
    const_pivot: bool,
    lead_deg_counts: Vec<usize>,
}

fn cmp_dr(a: &[u8], b: &[u8]) -> std::cmp::Ordering {
    // ascending degrevlex (x0 > x1 > ...): same convention as f4_fp's Grevlex
    use std::cmp::Ordering::*;
    let (da, db): (u32, u32) = (a.iter().map(|&x| x as u32).sum(), b.iter().map(|&x| x as u32).sum());
    if da != db {
        return da.cmp(&db);
    }
    for i in (0..a.len()).rev() {
        if a[i] != b[i] {
            return if a[i] > b[i] { Less } else { Greater };
        }
    }
    Equal
}

fn gen_monos(n: usize, dmax: u32) -> Vec<Vec<u8>> {
    fn rec(i: usize, n: usize, left: u32, cur: &mut Vec<u8>, out: &mut Vec<Vec<u8>>) {
        if i == n {
            out.push(cur.clone());
            return;
        }
        for e in 0..=left {
            cur[i] = e as u8;
            rec(i + 1, n, left - e, cur, out);
        }
        cur[i] = 0;
    }
    let mut out = Vec::new();
    rec(0, n, dmax, &mut vec![0u8; n], &mut out);
    out.sort_by(|a, b| cmp_dr(b, a)); // descending: column 0 is the largest monomial
    out
}

fn dense_profile(polys: &[Poly], n: usize, p: u64, dmax: u32, dmin: u32) -> Vec<DenseStat> {
    let mons = gen_monos(n, dmax);
    let nc = mons.len();
    let idx: HashMap<Vec<u8>, usize> = mons.iter().cloned().enumerate().map(|(i, m)| (m, i)).collect();
    let deg_of = |m: &Vec<u8>| m.iter().map(|&x| x as u32).sum::<u32>();
    let cdeg: Vec<u32> = mons.iter().map(deg_of).collect();
    let fs: Vec<Vec<(Vec<u8>, u64)>> = polys.iter().map(|q| terms_of(q, n)).collect();
    let fdeg: Vec<u32> = fs.iter().map(|t| t.iter().map(|(m, _)| deg_of(m)).max().unwrap()).collect();
    let mut piv: Vec<Option<Vec<u64>>> = vec![None; nc];
    let mut out = Vec::new();
    let mut rows_cum = 0usize;
    let mut rank = 0usize;
    for d in dmin..=dmax {
        let mut rows_added = 0usize;
        let mut new_p = 0usize;
        let mut new_below = 0usize;
        for (i, f) in fs.iter().enumerate() {
            if fdeg[i] > d {
                continue;
            }
            for mu in mons.iter().filter(|m| deg_of(m) == d - fdeg[i]) {
                let mut row = vec![0u64; nc];
                for (m, c) in f {
                    let prod: Vec<u8> = m.iter().zip(mu).map(|(a, b)| a + b).collect();
                    let j = idx[&prod];
                    row[j] = (row[j] + c) % p;
                }
                rows_added += 1;
                let mut c0 = 0usize;
                loop {
                    while c0 < nc && row[c0] == 0 {
                        c0 += 1;
                    }
                    if c0 == nc {
                        break;
                    }
                    if let Some(pv) = &piv[c0] {
                        let fct = row[c0];
                        for j in c0..nc {
                            if pv[j] != 0 {
                                row[j] = subm(row[j], mulm(fct, pv[j], p), p);
                            }
                        }
                        c0 += 1;
                    } else {
                        let inv = invm(row[c0], p);
                        for v in row.iter_mut() {
                            *v = mulm(*v, inv, p);
                        }
                        piv[c0] = Some(row);
                        new_p += 1;
                        if cdeg[c0] < d {
                            new_below += 1;
                        }
                        break;
                    }
                }
            }
        }
        rows_cum += rows_added;
        rank += new_p;
        // staircase over the monomials of degree <= d
        let cols_d = mons.iter().filter(|m| deg_of(m) <= d).count();
        let lead: Vec<&Vec<u8>> = (0..nc).filter(|&j| piv[j].is_some()).map(|j| &mons[j]).collect();
        let nstd = mons
            .iter()
            .filter(|m| deg_of(m) <= d)
            .filter(|m| !lead.iter().any(|l| l.iter().zip(m.iter()).all(|(a, b)| a <= b)))
            .count();
        let pure = (0..n)
            .filter(|&v| {
                (1..=d).any(|e| {
                    let mut m = vec![0u8; n];
                    m[v] = e as u8;
                    idx.get(&m).map_or(false, |&j| piv[j].is_some())
                })
            })
            .count();
        let const_pivot = piv[idx[&vec![0u8; n]]].is_some();
        let mut ldc = vec![0usize; d as usize + 1];
        for l in &lead {
            ldc[deg_of(l) as usize] += 1;
        }
        out.push(DenseStat {
            d,
            rows_cum,
            cols: cols_d,
            rank,
            new_pivots: new_p,
            new_pivots_below_d: new_below,
            zero_red: rows_added - new_p,
            nstd,
            pure_powers: pure,
            const_pivot,
            lead_deg_counts: ldc,
        });
    }
    out
}

// ------------------------------------------------------------------ ground truth by enumeration

/// S3(x1, x2, X) as a quadratic in X (low to high); same formula as PROTOCOL.md section 4.
fn s3q(x1: Fp, x2: Fp, a: Fp, b: Fp) -> [Fp; 3] {
    let two = Fp::from_u64(2);
    let four = Fp::from_u64(4);
    let d = x1.sub(x2);
    let s = x1.add(x2);
    let pr = x1.mul(x2);
    let c2 = d.mul(d);
    let c1 = two.neg().mul(s.mul(pr.add(a)).add(two.mul(b)));
    let e = pr.sub(a);
    let c0 = e.mul(e).sub(four.mul(b).mul(s));
    [c0, c1, c2]
}

/// Roots in F_p of a polynomial of degree <= 2 (low to high); Err(()) if identically zero.
fn roots_le2(c: &[Fp; 3]) -> Result<Vec<Fp>, ()> {
    let two = Fp::from_u64(2);
    let four = Fp::from_u64(4);
    if !c[2].is_zero() {
        let disc = c[1].mul(c[1]).sub(four.mul(c[2]).mul(c[0]));
        if disc.is_zero() {
            return Ok(vec![c[1].neg().mul(two.mul(c[2]).inv())]);
        }
        match disc.sqrt() {
            None => Ok(vec![]),
            Some(sq) => {
                let inv2a = two.mul(c[2]).inv();
                Ok(vec![c[1].neg().add(sq).mul(inv2a), c[1].neg().sub(sq).mul(inv2a)])
            }
        }
    } else if !c[1].is_zero() {
        Ok(vec![c[0].neg().mul(c[1].inv())])
    } else if !c[0].is_zero() {
        Ok(vec![])
    } else {
        Err(())
    }
}

fn q_at(c: &[Fp; 3], x: Fp) -> Fp {
    c[0].add(c[1].mul(x)).add(c[2].mul(x.mul(x)))
}

/// All F_p solutions of the system, by enumerating the tower (every variable is then determined).
/// None if some quadratic degenerates to 0 = 0 (positive-dimensional fibre).
fn truth_solutions(cfg: Cfg, tw: &Tower, a: Fp, b: Fp, xr: Fp, perm: &[usize]) -> Option<BTreeSet<Vec<u64>>> {
    let mut set = BTreeSet::new();
    let v = &tw.v;
    if cfg.m == 2 {
        for &x1 in v {
            for &x2 in v {
                if q_at(&s3q(x1, x2, a, b), xr).is_zero() {
                    let vals = assignment(cfg, tw, &[x1, x2], None, perm);
                    set.insert(vals.iter().map(|&c| raw(c)).collect());
                }
            }
        }
    } else {
        for &x1 in v {
            for &x2 in v {
                let q1 = s3q(x1, x2, a, b);
                let ws = roots_le2(&q1).ok()?;
                for &x3 in v {
                    let q2 = s3q(x3, xr, a, b); // S3(w, x3, xr) as a quadratic in w (S3 is symmetric)
                    for &w in &ws {
                        if q_at(&q2, w).is_zero() {
                            let vals = assignment(cfg, tw, &[x1, x2, x3], Some(w), perm);
                            set.insert(vals.iter().map(|&c| raw(c)).collect());
                        }
                    }
                }
            }
        }
        // degenerate q1 = 0: any w; then S3(w,x3,xr) must vanish: positive-dimensional unless q2 forces w
        for &x1 in v {
            for &x2 in v {
                if roots_le2(&s3q(x1, x2, a, b)).is_err() {
                    return None;
                }
            }
        }
    }
    Some(set)
}

// ------------------------------------------------------------------ instances

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Null,
    Planted,
    Abscissa,
    Ladder,
}
impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Null => "null",
            Kind::Planted => "planted",
            Kind::Abscissa => "abscissa",
            Kind::Ladder => "ladder",
        }
    }
}

struct Instance {
    polys: Vec<Poly>,
    n: usize,
    a: Fp,
    b: Fp,
    xr: Fp,
    planted: Option<Vec<Fp>>,
    kind: Kind,
}

fn random_curve(rng: &mut Rng, p: u64) -> (Fp, Fp) {
    loop {
        let (a, b) = (rng.fp(p), rng.fp(p));
        let four = Fp::from_u64(4);
        let tw7 = Fp::from_u64(27);
        let disc = four.mul(a).mul(a).mul(a).add(tw7.mul(b).mul(b));
        if !disc.is_zero() {
            return (a, b);
        }
    }
}

fn random_point(rng: &mut Rng, p: u64, a: Fp, b: Fp) -> (Fp, Fp) {
    loop {
        let x = rng.fp(p);
        if let Some(y) = rhs(x, a, b).sqrt() {
            return (x, y);
        }
    }
}

fn make_instance(kind: Kind, cfg: Cfg, tw: &Tower, a: Fp, b: Fp, p: u64, rng: &mut Rng) -> Option<Instance> {
    let n = cfg.nvars();
    let perm: Vec<usize> = (0..n).collect();
    let mut planted: Option<Vec<Fp>> = None;
    let xr: Fp;
    let mut extra: Vec<Poly> = Vec::new();
    match kind {
        Kind::Null => {
            xr = random_point(rng, p, a, b).0;
        }
        Kind::Planted | Kind::Ladder => {
            let fb = fb_abscissae(tw, a, b);
            if fb.len() < cfg.m {
                return None;
            }
            let mut idx: Vec<usize> = (0..fb.len()).collect();
            for i in (1..idx.len()).rev() {
                let j = rng.below(i + 1);
                idx.swap(i, j);
            }
            let mut pts = Vec::new();
            let mut xs = Vec::new();
            for e in 0..cfg.m {
                let x = fb[idx[e]];
                let mut y = rhs(x, a, b).sqrt().unwrap();
                if rng.next() & 1 == 1 {
                    y = y.neg();
                }
                pts.push((x, y));
                xs.push(x);
            }
            let s12 = ec_add(Some(pts[0]), Some(pts[1]), a)?;
            let (sum, w) = if cfg.m == 2 {
                (s12, None)
            } else {
                (ec_add(Some(s12), Some(pts[2]), a)?, Some(s12.0))
            };
            xr = sum.0;
            planted = Some(assignment(cfg, tw, &xs, w, &perm));
            if kind == Kind::Ladder {
                let vals = planted.as_ref().unwrap();
                let mut q = Poly::zero();
                for i in 0..n {
                    for j in i..n {
                        let c = rng.fp(p);
                        q = q.add(&Poly::var(i).mul(&Poly::var(j)).scale(c));
                    }
                    q = q.add(&Poly::var(i).scale(rng.fp(p)));
                }
                let c0 = q.eval(vals).neg();
                q = q.add(&Poly::constant(c0));
                extra.push(q);
            }
        }
        Kind::Abscissa => {
            // abscissa-level decomposition: tower elements need not carry a rational point
            let v = &tw.v;
            let mut tries = 0;
            loop {
                tries += 1;
                if tries > 2000 {
                    return None;
                }
                let x1 = v[rng.below(v.len())];
                let x2 = v[rng.below(v.len())];
                let q12 = s3q(x1, x2, a, b);
                let ws = roots_le2(&q12).unwrap_or_default();
                if cfg.m == 2 {
                    if let Some(&r) = ws.first() {
                        // S3(x1, x2, r) = 0 with r the target abscissa
                        xr = r;
                        planted = Some(assignment(cfg, tw, &[x1, x2], None, &perm));
                        break;
                    }
                } else {
                    let x3 = v[rng.below(v.len())];
                    if let Some(&w) = ws.first() {
                        let q2 = s3q(w, x3, a, b);
                        if let Some(&r) = roots_le2(&q2).unwrap_or_default().first() {
                            xr = r;
                            planted = Some(assignment(cfg, tw, &[x1, x2, x3], Some(w), &perm));
                            break;
                        }
                    }
                }
            }
        }
    }
    let mut polys = build_system(cfg, tw, a, b, xr, &perm);
    if let Some(vals) = &planted {
        assert!(polys.iter().all(|q| q.eval(vals).is_zero()), "planted point does not satisfy the system");
    }
    polys.extend(extra);
    Some(Instance { polys, n, a, b, xr, planted, kind })
}

// ------------------------------------------------------------------ the comparison

struct Opts {
    dense_maxcols: usize,
    dense_every: usize,
    f4_budget: f64,
    f4_maxdeg: u32,
    dcap: u32,
    max_cols: usize,
    engine_budget: f64,
}

fn binom(n: usize, k: usize) -> usize {
    let k = k.min(n - k);
    let mut r = 1usize;
    for i in 0..k {
        r = r * (n - i) / (i + 1);
    }
    r
}

fn compare(inst: &Instance, cfg: Cfg, tw: &Tower, modulus: u64, o: &Opts, do_dense: bool, label: &str) -> Value {
    let n = inst.n;
    let polys = &inst.polys;
    let mut failures: Vec<String> = Vec::new();
    let mut notes: Vec<String> = Vec::new();
    let k_closure = count_solutions(cfg, tw, inst.a, inst.b, inst.xr);
    let perm: Vec<usize> = (0..n).collect();
    let truth = if inst.kind == Kind::Ladder { None } else { truth_solutions(cfg, tw, inst.a, inst.b, inst.xr, &perm) };

    // ---- engine
    let k_for_engine = if inst.kind == Kind::Ladder { 1 } else { k_closure };
    let lim = engine::Limits { dcap: o.dcap, max_cols: o.max_cols, budget_s: o.engine_budget };
    let cpu0 = engine::thread_cpu_s();
    let eo = engine::run(polys, n, k_for_engine, &lim);
    let eng_cpu = engine::thread_cpu_s() - cpu0;

    // ---- f4_fp: basis (dimension, degrees) and solutions
    let f4in: Vec<f4_fp::Poly> = polys.iter().map(|q| to_f4(q, n, modulus)).collect();
    let opts = F4Options::new(FOrd::Grevlex, o.f4_maxdeg).with_budget(Duration::from_secs_f64(o.f4_budget)).capturing_run_log();
    let t0 = Instant::now();
    let rep = f4_fp::f4(&f4in, n, modulus, &opts);
    let f4_ms = t0.elapsed().as_secs_f64() * 1e3;
    let log = f4_fp::take_run_log();
    let f4_complete = !rep.timed_out && rep.pairs_above_bound == 0;
    let f4_dim: Option<usize> = if !f4_complete {
        None
    } else if rep.inconsistent {
        Some(0)
    } else {
        f4_std(&rep.basis, n).map(|x| x.0)
    };
    let f4_std_by_deg = if f4_complete && !rep.inconsistent { f4_std(&rep.basis, n).map(|x| x.1) } else { None };
    let sopts = F4Options::new(FOrd::Grevlex, o.f4_maxdeg).with_budget(Duration::from_secs_f64(o.f4_budget));
    let ts = Instant::now();
    let srep = f4_fp::solve(&f4in, n, modulus, &sopts);
    let solve_ms = ts.elapsed().as_secs_f64() * 1e3;
    let f4_sols: Option<BTreeSet<Vec<u64>>> = match &srep.verdict {
        Verdict::Solutions(v) => Some(v.iter().cloned().collect()),
        Verdict::Inconsistent => Some(BTreeSet::new()),
        Verdict::Undetermined => None,
    };

    // ---- 1. solution sets
    let mut sol_status = "not_compared";
    if let (Some(t), Some(f)) = (&truth, &f4_sols) {
        if t == f {
            sol_status = "equal";
        } else {
            sol_status = "DIFFER";
            failures.push(format!("solution sets differ: truth {} vs f4_fp {}", t.len(), f.len()));
        }
    } else if truth.is_none() && inst.kind != Kind::Ladder {
        notes.push("truth enumeration degenerate (positive-dimensional fibre)".into());
    }
    if let (Some(pv), Some(f)) = (&inst.planted, &f4_sols) {
        let key: Vec<u64> = pv.iter().map(|&c| raw(c)).collect();
        if !f.contains(&key) {
            failures.push("planted assignment missing from f4_fp solutions".into());
        }
    }
    if inst.kind == Kind::Ladder {
        if let (Some(pv), Some(f)) = (&inst.planted, &f4_sols) {
            let key: Vec<u64> = pv.iter().map(|&c| raw(c)).collect();
            sol_status = if f.len() == 1 && f.contains(&key) { "planted_unique" } else { "ladder_not_unique" };
            if f.len() != 1 {
                notes.push(format!("ladder: f4_fp finds {} solutions (engine assumes K = 1)", f.len()));
            }
        }
    }

    // ---- 2. dimensions
    if let Some(d) = f4_dim {
        if inst.kind != Kind::Ladder && d != k_closure {
            failures.push(format!("quotient dimension {d} (F4) != K {k_closure} (count_solutions): ideal not radical or K wrong"));
        }
        if eo.status == "solved" && !eo.inconsistent && eo.nstd_final != d {
            failures.push(format!("engine n_std {} != F4 quotient dimension {d}", eo.nstd_final));
        }
    }
    if eo.status == "solved" && eo.inconsistent != rep.inconsistent && f4_complete {
        failures.push(format!("inconsistency: engine {} vs F4 {}", eo.inconsistent, rep.inconsistent));
    }
    if eo.status == "contradiction_with_K" {
        failures.push("engine: constant 1 found although K > 0".into());
    }
    if eo.status != "solved" {
        notes.push(format!("engine status {}", eo.status));
    }
    if !f4_complete {
        notes.push(format!("f4 incomplete (timed_out {}, pairs_above_bound {})", rep.timed_out, rep.pairs_above_bound));
    }

    // ---- 3. degrees.  Inclusion: every element F4 derives at step degree d lies in the Macaulay span at
    // degree d, hence D_engine <= F4 degree.  Report the relation; a violation is a blocker.
    let f4_deg = if rep.inconsistent { rep.solving_degree } else { rep.solving_degree_max };
    let rel = match eo.d_solve {
        Some(d) if eo.status == "solved" && f4_complete => {
            if d > f4_deg {
                failures.push(format!("engine D {d} > F4 degree {f4_deg}: contradicts the inclusion XL(d) contains F4(d)"));
                "VIOLATION"
            } else if d == f4_deg {
                "equal"
            } else {
                "engine_lower"
            }
        }
        _ => "n/a",
    };

    // ---- 4. per-degree profile against the dense reference
    let mut dense_json = Value::Null;
    let mut dense_status = "not_run";
    if do_dense && !eo.stats.is_empty() {
        let dmin = eo.stats[0].d;
        let mut dmax = dmin;
        for s in &eo.stats {
            if s.cols <= o.dense_maxcols {
                dmax = s.d;
            }
        }
        if eo.stats[0].cols <= o.dense_maxcols {
            let ds = dense_profile(polys, n, modulus, dmax, dmin);
            let mut rows = Vec::new();
            let mut ok = true;
            for (s, dsr) in eo.stats.iter().zip(ds.iter()) {
                let mut mism: Vec<&str> = Vec::new();
                if s.d != dsr.d {
                    mism.push("d");
                }
                if s.rows_cum != dsr.rows_cum {
                    mism.push("rows_cum");
                }
                if s.cols != dsr.cols {
                    mism.push("cols");
                }
                if s.rank != dsr.rank {
                    mism.push("rank");
                }
                if s.new_pivots != dsr.new_pivots {
                    mism.push("new_pivots");
                }
                if s.new_pivots_below_d != dsr.new_pivots_below_d {
                    mism.push("new_pivots_below_d");
                }
                if s.zero_red != dsr.zero_red {
                    mism.push("zero_red");
                }
                if s.nstd != dsr.nstd {
                    mism.push("nstd");
                }
                if s.pure_powers != dsr.pure_powers {
                    mism.push("pure_powers");
                }
                if s.const_pivot != dsr.const_pivot {
                    mism.push("const_pivot");
                }
                // closed forms
                let closed_cols = binom(n + s.d as usize, n);
                if closed_cols != s.cols {
                    mism.push("cols_closed_form");
                }
                let closed_rows: usize = polys
                    .iter()
                    .map(|q| {
                        let dq = q.degree() as usize;
                        if (s.d as usize) >= dq {
                            binom(n + s.d as usize - dq, n)
                        } else {
                            0
                        }
                    })
                    .sum();
                let _ = closed_rows; // cumulative closed form: sum over degrees <= d below
                if !mism.is_empty() {
                    ok = false;
                    failures.push(format!("dense reference mismatch at D={}: {:?}", s.d, mism));
                }
                rows.push(json!({"D": s.d, "engine": {"rank": s.rank, "new": s.new_pivots, "below": s.new_pivots_below_d, "nstd": s.nstd, "pure": s.pure_powers, "rows_cum": s.rows_cum},
                                 "dense": {"rank": dsr.rank, "new": dsr.new_pivots, "below": dsr.new_pivots_below_d, "nstd": dsr.nstd, "pure": dsr.pure_powers, "rows_cum": dsr.rows_cum,
                                           "pivots_by_lead_degree": dsr.lead_deg_counts}, "mismatch": mism}));
            }
            // cumulative row count closed form: rows(D) = sum_i sum_{d<=D} C(n + d - deg_i - 1, n - 1)... checked
            // implicitly by rows_cum agreement with the dense generation (independent monomial enumeration).
            dense_status = if ok { "agree" } else { "MISMATCH" };
            dense_json = json!({"dmax": dmax, "rows": rows});
        } else {
            dense_status = "skipped_too_large";
        }
    }

    json!({
        "label": label, "config": format!("m{}t{}", cfg.m, cfg.t), "kind": inst.kind.name(), "n_vars": n,
        "degrees": polys.iter().map(|q| q.degree()).collect::<Vec<_>>(),
        "K_closure": k_closure,
        "truth_rational_solutions": truth.as_ref().map(|t| t.len()),
        "f4_rational_solutions": f4_sols.as_ref().map(|t| t.len()),
        "solution_sets": sol_status,
        "engine": {"status": eo.status, "D_solve": eo.d_solve, "D_max": eo.d_max, "first_fall": eo.first_fall, "nstd_final": eo.nstd_final,
                   "inconsistent": eo.inconsistent, "final_rows": eo.final_rows, "final_cols": eo.final_cols, "final_rank": eo.final_rank,
                   "pivots_by_lead_degree": eo.pivots_by_lead_degree, "cpu_s": eng_cpu},
        "f4": {"complete": f4_complete, "inconsistent": rep.inconsistent, "dim": f4_dim, "std_by_degree": f4_std_by_deg,
               "solving_degree": rep.solving_degree, "solving_degree_max": rep.solving_degree_max, "degree_reached": rep.degree_reached,
               "steps": rep.steps, "max_rows": rep.max_rows, "max_cols": rep.max_cols, "ms": f4_ms, "solve_ms": solve_ms,
               "step_degrees": log.steps.iter().map(|s| s.degree).collect::<Vec<_>>(),
               "basis_size": rep.basis.len(),
               "basis_lead_degrees": rep.basis.iter().map(|g| g.first().map(|t| t.0.iter().sum::<u32>()).unwrap_or(0)).collect::<Vec<_>>()},
        "degree_relation_engine_vs_f4": rel,
        "dense_reference": dense_status, "dense": dense_json,
        "failures": failures, "notes": notes,
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let modulus: u64 = arg(&args, "--p", "0").parse().expect("--p DEC");
    assert!(modulus > 3 && modulus < (1u64 << 61) + 1 && is_prime_u64(modulus) && modulus % 4 == 3, "p must be a prime < 2^61, = 3 mod 4");
    fp::set_modulus(&format!("0x{modulus:x}"));
    let tower_path = arg(&args, "--tower", "tower.json");
    let tj: Value = serde_json::from_str(&std::fs::read_to_string(&tower_path).unwrap()).unwrap();
    assert_eq!(tj["p"].as_str().unwrap(), format!("0x{modulus:x}"), "tower built for another prime");
    let configs: Vec<Cfg> = arg(&args, "--configs", "m2t3,m3t2")
        .split(',')
        .map(|s| {
            let s = s.trim_start_matches('m');
            let (m, t) = s.split_once('t').unwrap();
            Cfg { m: m.parse().unwrap(), t: t.parse().unwrap() }
        })
        .collect();
    let ncurves: usize = arg(&args, "--curves", "3").parse().unwrap();
    let nnull: usize = arg(&args, "--null", "2").parse().unwrap();
    let nplanted: usize = arg(&args, "--planted", "2").parse().unwrap();
    let nabs: usize = arg(&args, "--abscissa", "2").parse().unwrap();
    let nladder: usize = arg(&args, "--ladder", "1").parse().unwrap();
    let seed: u64 = arg(&args, "--seed", "20261102").parse().unwrap();
    let out_path = arg(&args, "--out", "crosscheck_rows.jsonl");
    let o = Opts {
        dense_maxcols: arg(&args, "--dense-maxcols", "2000").parse().unwrap(),
        dense_every: arg(&args, "--dense-every", "3").parse().unwrap(),
        f4_budget: arg(&args, "--f4-budget-s", "120").parse().unwrap(),
        f4_maxdeg: arg(&args, "--f4-maxdeg", "16").parse().unwrap(),
        dcap: arg(&args, "--dcap", "12").parse().unwrap(),
        max_cols: arg(&args, "--max-cols", "60000").parse().unwrap(),
        engine_budget: arg(&args, "--engine-budget-s", "300").parse().unwrap(),
    };
    let mut rng = Rng(seed ^ modulus.wrapping_mul(0x2545F4914F6CDD1D));
    let mut out = std::fs::File::create(&out_path).unwrap();
    let mut counter = 0usize;
    let mut n_sys = 0usize;
    let mut n_fail = 0usize;
    let mut n_dense = 0usize;
    let mut n_dense_ok = 0usize;
    eprintln!("p = {modulus} ({} bits), tower {tower_path}", 64 - modulus.leading_zeros());
    for cfg in &configs {
        let tw = load_tower(&tj, cfg.t);
        for ci in 0..ncurves {
            let (a, b) = random_curve(&mut rng, modulus);
            let kinds: Vec<(Kind, usize)> = vec![(Kind::Null, nnull), (Kind::Planted, nplanted), (Kind::Abscissa, nabs), (Kind::Ladder, nladder)];
            for (kind, cnt) in kinds {
                for ti in 0..cnt {
                    let label = format!("p{modulus}|m{}t{}|curve{ci}|{}|{ti}", cfg.m, cfg.t, kind.name());
                    let Some(inst) = make_instance(kind, *cfg, &tw, a, b, modulus, &mut rng) else {
                        eprintln!("{label}: instance not formed (factor base too small on this curve)");
                        continue;
                    };
                    counter += 1;
                    let do_dense = counter % o.dense_every == 0;
                    let row = compare(&inst, *cfg, &tw, modulus, &o, do_dense, &label);
                    n_sys += 1;
                    let fails = row["failures"].as_array().unwrap().len();
                    if fails > 0 {
                        n_fail += 1;
                    }
                    if row["dense_reference"] == "agree" {
                        n_dense += 1;
                        n_dense_ok += 1;
                    } else if row["dense_reference"] == "MISMATCH" {
                        n_dense += 1;
                    }
                    eprintln!(
                        "{label}: K={} sols[truth/f4]={}/{} sets={} engine[{} D={} nstd={}] f4[dim={} sd={} sdmax={} reached={}] rel={} dense={} fail={}",
                        row["K_closure"], row["truth_rational_solutions"], row["f4_rational_solutions"], row["solution_sets"],
                        row["engine"]["status"], row["engine"]["D_solve"], row["engine"]["nstd_final"],
                        row["f4"]["dim"], row["f4"]["solving_degree"], row["f4"]["solving_degree_max"], row["f4"]["degree_reached"],
                        row["degree_relation_engine_vs_f4"], row["dense_reference"], fails
                    );
                    writeln!(out, "{}", row).unwrap();
                }
            }
        }
    }
    eprintln!("SUMMARY p={modulus}: systems={n_sys} with_failures={n_fail} dense_checked={n_dense} dense_agree={n_dense_ok}");
    println!("{}", json!({"p": modulus, "systems": n_sys, "with_failures": n_fail, "dense_checked": n_dense, "dense_agree": n_dense_ok}));
    if n_fail > 0 {
        std::process::exit(1);
    }
}
