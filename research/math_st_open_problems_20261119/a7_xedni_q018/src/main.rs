//! A7 (P1.6/Q018): xedni p=17 Experiment A under three explicit samplers.
//! Protocol: ../PROTOCOLS.md section A7. Sampler constants: REPORT.md "Sampler specification".
//!
//! Usage:
//!   a7_xedni_q018 run <runs_per_sampler> <outdir>
//!   a7_xedni_q018 controls <outdir>
//!   a7_xedni_q018 fixtures <outdir>

mod ec;
mod heights;
mod lattice;
mod modl;

use ec::{pt_string, General, Pt, Short, Q};
use heights::{regulator, HeightCtx, RegResult};
use lattice::{closest_to_zero, neighbourhood, solve, CoeffLattice, LatErr, V7};
use modl::{is_prime, rat_mod, CurveP, Fp};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;
use serde_json::json;
use std::fs;
use std::io::Write;
use std::time::Instant;

// ---- constants fixed before any run (REPORT.md "Sampler specification") ----
const P: i64 = 17;
const A_FIN: i64 = 2;
const B_FIN: i64 = 2;
const P0: (u64, u64) = (3, 1);
const GROUP_ORDER: u64 = 19;
const SEED: u64 = 20261125;
const R_BOX: i64 = 16;
const POOL: usize = 8;
const NEIGH_RADIUS: i128 = 1;
const B: i64 = 8;
const B2: i64 = 64;
const N_FILTER_PRIMES: usize = 6;
const FILTER_PRIME_START: u64 = 1009;
const DOUBLINGS: [u32; 4] = [3, 4, 5, 6];
const BIT_CAP: u64 = 1 << 22;
const MAX_RESAMPLES: u32 = 10_000;
const MAX_B2_VERIFY: usize = 20;
const PUBLISHED_DEP: u64 = 317;
const PUBLISHED_N: u64 = 100_000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sampler {
    S1,
    S2,
    S3,
}
impl Sampler {
    fn id(&self) -> u64 {
        match self {
            Sampler::S1 => 1,
            Sampler::S2 => 2,
            Sampler::S3 => 3,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Sampler::S1 => "S1",
            Sampler::S2 => "S2",
            Sampler::S3 => "S3",
        }
    }
}

fn fin_curve() -> CurveP {
    CurveP { p: P as u64, a: A_FIN as u64, b: B_FIN as u64 }
}

fn balanced(v: i64, p: i64) -> i64 {
    let r = v.rem_euclid(p);
    if r > p / 2 {
        r - p
    } else {
        r
    }
}

fn gcd3(a: i64, b: i64, c: i64) -> i64 {
    a.abs().gcd(&b.abs()).gcd(&c.abs())
}

/// Upstream A002 `projective_lift_pool(point, p, pool_size)`.
fn pool_lifts(pt: (u64, u64), pool: usize) -> Vec<[i64; 3]> {
    let (x, y) = (pt.0 as i64, pt.1 as i64);
    let mut cands: Vec<([i64; 3], i64)> = Vec::new();
    for s in 1..P {
        let v = [balanced(s * x, P), balanced(s * y, P), balanced(s, P)];
        let g = gcd3(v[0], v[1], v[2]);
        let mut w = [v[0] / g, v[1] / g, v[2] / g];
        let first = *w.iter().find(|&&c| c != 0).unwrap();
        if first < 0 {
            for c in w.iter_mut() {
                *c = -*c;
            }
        }
        if w[2] % P == 0 {
            continue;
        }
        if !cands.iter().any(|(c, _)| *c == w) {
            let n = w.iter().map(|c| c * c).sum();
            cands.push((w, n));
        }
    }
    cands.sort_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0)));
    cands.into_iter().take(pool).map(|(w, _)| w).collect()
}

/// All primitive lifts in the box |X|,|Y|,|Z| <= R_BOX, one per +- pair (Z > 0).
fn box_lifts(pt: (u64, u64)) -> Vec<[i64; 3]> {
    let (x, y) = (pt.0 as i64, pt.1 as i64);
    let mut out = Vec::new();
    for z in 1..=R_BOX {
        if z % P == 0 {
            continue;
        }
        for xx in -R_BOX..=R_BOX {
            if (xx - x * z).rem_euclid(P) != 0 {
                continue;
            }
            for yy in -R_BOX..=R_BOX {
                if (yy - y * z).rem_euclid(P) != 0 {
                    continue;
                }
                if gcd3(xx, yy, z) != 1 {
                    continue;
                }
                out.push([xx, yy, z]);
            }
        }
    }
    out
}

/// The lifted curve, in the paper's projective form, the integral general model and the short model.
#[derive(Clone)]
struct Lifted {
    u: V7,
    gen: General,
    disc: BigInt,
    a002_key: BigInt, // max(|n|,|d|) of disc / alpha^12 in lowest terms
    pts_gen: [Pt; 2],
    short: Short,
    pts_short: [Pt; 2],
}

fn bi(v: i128) -> BigInt {
    BigInt::from(v)
}

fn build_curve(lifts: &[[i64; 3]; 2], u: &V7, fin_pts: &[(u64, u64); 2]) -> Option<Lifted> {
    let p = bi(P as i128);
    let alpha = BigInt::one() + &p * bi(u[0]);
    let beta = BigInt::one() + &p * bi(u[3]);
    let c1 = &p * bi(u[1]);
    let c2 = &p * bi(u[4]);
    let c3 = &p * bi(u[2]);
    let c4 = bi(A_FIN as i128) + &p * bi(u[5]);
    let c6 = bi(B_FIN as i128) + &p * bi(u[6]);
    // exact projective containment of the paper's equation
    for l in lifts {
        let (x, y, z) = (BigInt::from(l[0]), BigInt::from(l[1]), BigInt::from(l[2]));
        let lhs = &alpha * &y * &y * &z + &c1 * &x * &y * &z + &c3 * &y * &z * &z;
        let rhs = &beta * &x * &x * &x + &c2 * &x * &x * &z + &c4 * &x * &z * &z + &c6 * &z * &z * &z;
        assert_eq!(lhs, rhs, "lift not on the projective curve");
    }
    let gen = General {
        a1: c1.clone(),
        a2: &c2 * &alpha,
        a3: &c3 * &alpha * &beta,
        a4: &c4 * &alpha * &alpha * &beta,
        a6: &c6 * &alpha * &alpha * &alpha * &beta * &beta,
    };
    let (short, inv) = gen.to_short();
    if inv.disc.is_zero() {
        return None;
    }
    assert!(!short.disc4().is_zero());
    let ab = &alpha * &beta;
    let a2b = &alpha * &alpha * &beta;
    let mut pts_gen: [Pt; 2] = [None, None];
    for (i, l) in lifts.iter().enumerate() {
        let x = Q::new(&ab * BigInt::from(l[0]), BigInt::from(l[2]));
        let y = Q::new(&a2b * BigInt::from(l[1]), BigInt::from(l[2]));
        let pt = Some((x, y));
        assert!(gen.contains(&pt), "point not on general model");
        pts_gen[i] = pt;
    }
    // reduction mod 17: curve and points
    let m = |v: &BigInt| modl::bigint_mod(v, P as u64);
    assert_eq!(
        [m(&gen.a1), m(&gen.a2), m(&gen.a3), m(&gen.a4), m(&gen.a6)],
        [0, 0, 0, A_FIN as u64, B_FIN as u64],
        "wrong reduction"
    );
    assert!(!modl::bigint_divisible(&inv.disc, P as u64), "bad reduction at p");
    for i in 0..2 {
        let (x, y) = pts_gen[i].as_ref().unwrap();
        let xr = rat_mod(&x.n, &x.d, P as u64).expect("point reduces to O");
        let yr = rat_mod(&y.n, &y.d, P as u64).unwrap();
        assert_eq!((xr, yr), fin_pts[i], "point reduction mismatch");
    }
    let pts_short = [gen.map_to_short(&inv, &pts_gen[0]), gen.map_to_short(&inv, &pts_gen[1])];
    for q in &pts_short {
        assert!(short.contains(q), "point not on short model");
    }
    let a12 = alpha.pow(12u32);
    let g = inv.disc.gcd(&a12);
    let n = (&inv.disc / &g).abs();
    let d = (&a12 / &g).abs();
    let a002_key = std::cmp::max(n, d);
    Some(Lifted { u: *u, gen, disc: inv.disc, a002_key, pts_gen, short, pts_short })
}

// ---------------- dependence test ----------------

fn better(c: (i64, i64), best: &Option<(i64, i64)>) -> bool {
    match best {
        None => true,
        Some(b) => {
            let key = |r: (i64, i64)| (r.0.abs().max(r.1.abs()), r.0.abs() + r.1.abs(), r.0, r.1);
            key(c) < key(*b)
        }
    }
}

/// Candidate relations (n1, n2), normalised n1 > 0 or (n1 == 0, n2 > 0), max |ni| <= bound,
/// in increasing order of (max|ni|, |n1|+|n2|, n1, n2).
fn candidates(bound: i64) -> Vec<(i64, i64)> {
    let mut v = Vec::new();
    for n1 in 0..=bound {
        for n2 in -bound..=bound {
            if n1 == 0 && n2 <= 0 {
                continue;
            }
            v.push((n1, n2));
        }
    }
    v.sort_by_key(|r| (r.0.abs().max(r.1.abs()), r.0.abs() + r.1.abs(), r.0, r.1));
    v
}

/// Exhaustive exact search with |ni| <= bound on the short model; returns the minimal relation.
fn relation_search_exact(e: &Short, p1: &Pt, p2: &Pt, bound: i64) -> Option<(i64, i64)> {
    let mut m1: Vec<Pt> = vec![None];
    let mut m2: Vec<Pt> = vec![None];
    for k in 1..=bound as usize {
        m1.push(e.add(&m1[k - 1], p1));
        m2.push(e.add(&m2[k - 1], p2));
    }
    let mut best: Option<(i64, i64)> = None;
    for (n1, n2) in candidates(bound) {
        if let Some(b) = best {
            if n1.abs().max(n2.abs()) > b.0.abs().max(b.1.abs()) {
                break;
            }
        }
        let lhs = &m1[n1 as usize];
        let rhs = if n2 >= 0 { e.neg(&m2[n2 as usize]) } else { m2[(-n2) as usize].clone() };
        if *lhs == rhs && better((n1, n2), &best) {
            best = Some((n1, n2));
        }
    }
    best
}

fn verify_relation(l: &Lifted, r: (i64, i64)) -> bool {
    let s = l.short.add(&l.short.mul(r.0, &l.pts_short[0]), &l.short.mul(r.1, &l.pts_short[1]));
    let g = l.gen.add(&l.gen.mul(r.0, &l.pts_gen[0]), &l.gen.mul(r.1, &l.pts_gen[1]));
    s.is_none() && g.is_none()
}

fn filter_primes(e: &Short, pts: &[Pt; 2]) -> Vec<(CurveP, [Fp; 2])> {
    let d = e.disc4();
    let mut out = Vec::new();
    let mut l = FILTER_PRIME_START;
    while out.len() < N_FILTER_PRIMES {
        if is_prime(l) && !modl::bigint_divisible(&d, l) {
            let c = CurveP { p: l, a: modl::bigint_mod(&e.a, l), b: modl::bigint_mod(&e.b, l) };
            let mut red: [Fp; 2] = [None, None];
            let mut ok = true;
            for i in 0..2 {
                match &pts[i] {
                    None => red[i] = None,
                    Some((x, y)) => match (rat_mod(&x.n, &x.d, l), rat_mod(&y.n, &y.d, l)) {
                        (Some(a), Some(b)) => red[i] = Some((a, b)),
                        _ => ok = false,
                    },
                }
            }
            if ok {
                assert!(c.contains(&red[0]) && c.contains(&red[1]));
                out.push((c, red));
            }
        }
        l += 1;
    }
    out
}

/// Filtered search with |ni| <= bound: candidates must hold mod every filter prime,
/// survivors verified exactly over Q. Returns (relation, survivors_checked).
fn relation_search_filtered(e: &Short, p1: &Pt, p2: &Pt, bound: i64) -> (Option<(i64, i64)>, usize) {
    let pts = [p1.clone(), p2.clone()];
    let primes = filter_primes(e, &pts);
    let tabs: Vec<(CurveP, Vec<Fp>, Vec<Fp>)> = primes
        .iter()
        .map(|(c, red)| {
            let mut t1 = vec![None];
            let mut t2 = vec![None];
            for k in 1..=bound as usize {
                t1.push(c.add(&t1[k - 1], &red[0]));
                t2.push(c.add(&t2[k - 1], &red[1]));
            }
            (*c, t1, t2)
        })
        .collect();
    let mut checked = 0;
    for (n1, n2) in candidates(bound) {
        let pass = tabs.iter().all(|(c, t1, t2)| {
            let lhs = t1[n1 as usize];
            let rhs = if n2 >= 0 { c.neg(&t2[n2 as usize]) } else { t2[(-n2) as usize] };
            lhs == rhs
        });
        if !pass {
            continue;
        }
        if checked >= MAX_B2_VERIFY {
            break;
        }
        checked += 1;
        let s = e.add(&e.mul(n1, p1), &e.mul(n2, p2));
        if s.is_none() {
            return (Some((n1, n2)), checked);
        }
    }
    (None, checked)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Class {
    Dependent,
    Independent,
    Undecided,
}
impl Class {
    fn s(&self) -> &'static str {
        match self {
            Class::Dependent => "dependent",
            Class::Independent => "independent",
            Class::Undecided => "undecided",
        }
    }
}

struct Verdict {
    class_b: Class,
    rel_b: Option<(i64, i64)>,
    class_b2: Class,
    rel_b2: Option<(i64, i64)>,
    reg: RegResult,
    contradiction: bool, // certified independence together with a verified relation
    reg_consistency_violation: bool, // relation known but reg interval excludes 0
}

fn dependence_test_short(e: &Short, p1: &Pt, p2: &Pt, verify: &dyn Fn((i64, i64)) -> bool) -> Verdict {
    let rel_b = relation_search_exact(e, p1, p2, B);
    if let Some(r) = rel_b {
        assert!(verify(r), "relation failed re-verification");
    }
    let (rel_b2_f, _) = relation_search_filtered(e, p1, p2, B2);
    if let Some(r) = rel_b2_f {
        assert!(verify(r), "B2 relation failed re-verification");
    }
    // a relation within B is also within B2: the filtered search must find one at least as small
    if let Some(r) = rel_b {
        let f = rel_b2_f.expect("filtered search missed a relation found exhaustively");
        assert_eq!(f, r, "filtered and exhaustive minimal relations differ");
    }
    let ctx = HeightCtx::new(e);
    let reg = regulator(e, &ctx, p1, p2, &DOUBLINGS, BIT_CAP, true);
    let class_b = if rel_b.is_some() {
        Class::Dependent
    } else if reg.certified_positive {
        Class::Independent
    } else {
        Class::Undecided
    };
    let class_b2 = if rel_b2_f.is_some() {
        Class::Dependent
    } else if reg.certified_positive {
        Class::Independent
    } else {
        Class::Undecided
    };
    let contradiction = reg.certified_positive && rel_b2_f.is_some();
    let reg_consistency_violation = rel_b2_f.is_some() && (reg.reg_lo > 0.0 || reg.reg_hi < 0.0);
    Verdict { class_b, rel_b, class_b2, rel_b2: rel_b2_f, reg, contradiction, reg_consistency_violation }
}

// ---------------- one run ----------------

struct Row {
    sampler: Sampler,
    run: u64,
    mu2: u64,
    lifts: [[i64; 3]; 2],
    resamples: u32,
    lifted: Lifted,
    v: Verdict,
    seconds: f64,
}

fn run_seed(sampler: Sampler, run: u64) -> [u8; 32] {
    let mut s = [0u8; 32];
    s[0..8].copy_from_slice(&SEED.to_le_bytes());
    s[8..16].copy_from_slice(&sampler.id().to_le_bytes());
    s[16..24].copy_from_slice(&run.to_le_bytes());
    s
}

struct Tables {
    fin_pts: Vec<(u64, u64)>, // index mu-1: mu * P0 for mu = 1..18
    boxes: Vec<Vec<[i64; 3]>>,
    pools: Vec<Vec<[i64; 3]>>,
}

fn tables() -> Tables {
    let c = fin_curve();
    let p0: Fp = Some(P0);
    let mut fin_pts = Vec::new();
    for mu in 1..GROUP_ORDER as i64 {
        fin_pts.push(c.mul_pt(mu, &p0).unwrap());
    }
    assert_eq!(c.mul_pt(GROUP_ORDER as i64, &p0), None);
    let boxes = fin_pts.iter().map(|&p| box_lifts(p)).collect();
    let pools = fin_pts.iter().map(|&p| pool_lifts(p, POOL)).collect();
    Tables { fin_pts, boxes, pools }
}

fn do_run(t: &Tables, sampler: Sampler, run: u64) -> Row {
    let start = Instant::now();
    let mut rng = ChaCha20Rng::from_seed(run_seed(sampler, run));
    let mu2: u64 = rng.gen_range(2..=17);
    let fin = [t.fin_pts[0], t.fin_pts[(mu2 - 1) as usize]];
    let mut resamples = 0u32;
    loop {
        assert!(resamples < MAX_RESAMPLES, "too many resamples");
        let (src1, src2) = match sampler {
            Sampler::S2 => (&t.pools[0], &t.pools[(mu2 - 1) as usize]),
            _ => (&t.boxes[0], &t.boxes[(mu2 - 1) as usize]),
        };
        let lifts = [src1[rng.gen_range(0..src1.len())], src2[rng.gen_range(0..src2.len())]];
        let lat: CoeffLattice = match solve(&lifts, P as i128, A_FIN as i128, B_FIN as i128) {
            Ok(l) => l,
            Err(LatErr::Overflow) => panic!("i128 overflow in lattice solve for lifts {:?}", lifts),
            Err(_) => {
                resamples += 1;
                continue;
            }
        };
        let ustar = closest_to_zero(&lat).expect("cvp overflow");
        let lifted = match sampler {
            Sampler::S1 => build_curve(&lifts, &ustar, &fin),
            Sampler::S2 => {
                let neigh = neighbourhood(&ustar, &lat.basis, NEIGH_RADIUS).expect("overflow");
                let mut best: Option<Lifted> = None;
                for u in neigh {
                    if let Some(l) = build_curve(&lifts, &u, &fin) {
                        let take = match &best {
                            None => true,
                            Some(b) => {
                                let mu = |v: &V7| v.iter().map(|x| x.abs()).max().unwrap();
                                (&l.a002_key, mu(&l.u), l.u) < (&b.a002_key, mu(&b.u), b.u)
                            }
                        };
                        if take {
                            best = Some(l);
                        }
                    }
                }
                best
            }
            Sampler::S3 => {
                let neigh = neighbourhood(&ustar, &lat.basis, NEIGH_RADIUS).expect("overflow");
                let ok: Vec<Lifted> = neigh.iter().filter_map(|u| build_curve(&lifts, u, &fin)).collect();
                if ok.is_empty() {
                    None
                } else {
                    let i = rng.gen_range(0..ok.len());
                    Some(ok[i].clone())
                }
            }
        };
        let lifted = match lifted {
            Some(l) => l,
            None => {
                resamples += 1;
                continue;
            }
        };
        let lref = lifted.clone();
        let v = dependence_test_short(&lifted.short, &lifted.pts_short[0], &lifted.pts_short[1], &|r| {
            verify_relation(&lref, r)
        });
        return Row { sampler, run, mu2, lifts, resamples, lifted, v, seconds: start.elapsed().as_secs_f64() };
    }
}

fn log2_big(v: &BigInt) -> f64 {
    if v.is_zero() {
        return f64::NEG_INFINITY;
    }
    let (lo, hi) = heights::ln_bounds(v.magnitude());
    (lo + hi) / 2.0 / std::f64::consts::LN_2
}

fn rel_s(r: &Option<(i64, i64)>) -> String {
    match r {
        None => String::new(),
        Some((a, b)) => format!("{};{}", a, b),
    }
}

fn csv_row(r: &Row) -> String {
    let l = &r.lifted;
    let lift = |v: &[i64; 3]| format!("{};{};{}", v[0], v[1], v[2]);
    let u = l.u.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(";");
    let g = &l.gen;
    format!(
        "{},{},{},{},{},{},{},{},{},{},{},{},{},{:.3},{:.3},{},{},{},{},{},{},{},{:.9e},{:.9e},{:.9e},{:.9e},{:.9e},{:.9e},{},{:.6}\n",
        r.sampler.name(),
        r.run,
        SEED,
        r.mu2,
        lift(&r.lifts[0]),
        lift(&r.lifts[1]),
        r.resamples,
        u,
        g.a1,
        g.a2,
        g.a3,
        g.a4,
        g.a6,
        log2_big(&l.disc),
        log2_big(&l.a002_key),
        pt_string(&l.pts_gen[0]),
        pt_string(&l.pts_gen[1]),
        r.v.class_b.s(),
        rel_s(&r.v.rel_b),
        r.v.class_b2.s(),
        rel_s(&r.v.rel_b2),
        r.v.reg.level,
        r.v.reg.reg_lo,
        r.v.reg.reg_hi,
        r.v.reg.h1.0,
        r.v.reg.h1.1,
        r.v.reg.h2.0,
        r.v.reg.h2.1,
        r.v.reg.max_bits,
        r.seconds
    )
}

const CSV_HEADER: &str = "sampler,run,seed,mu2,lift1_XYZ,lift2_XYZ,resamples,u1..u7,a1,a2,a3,a4,a6,log2_abs_disc_integral_model,log2_a002_disc_height,P1,P2,class_B8,relation_B8,class_B64,relation_B64,reg_doublings,reg_lo,reg_hi,h1_lo,h1_hi,h2_lo,h2_hi,max_bits,seconds\n";

fn wilson(k: u64, n: u64) -> (f64, f64) {
    if n == 0 {
        return (f64::NAN, f64::NAN);
    }
    let z = 1.959963984540054f64;
    let p = k as f64 / n as f64;
    let nf = n as f64;
    let den = 1.0 + z * z / nf;
    let c = (p + z * z / (2.0 * nf)) / den;
    let h = z * (p * (1.0 - p) / nf + z * z / (4.0 * nf * nf)).sqrt() / den;
    ((c - h).max(0.0), (c + h).min(1.0))
}

fn stats(v: &[f64]) -> serde_json::Value {
    if v.is_empty() {
        return json!(null);
    }
    let mn = v.iter().cloned().fold(f64::INFINITY, f64::min);
    let mx = v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let av = v.iter().sum::<f64>() / v.len() as f64;
    json!({"min": mn, "max": mx, "mean": av})
}

fn summarise(rows: &[Row], sampler: Sampler) -> serde_json::Value {
    let rs: Vec<&Row> = rows.iter().filter(|r| r.sampler == sampler).collect();
    let n = rs.len() as u64;
    let cnt = |f: &dyn Fn(&Row) -> bool| rs.iter().filter(|r| f(r)).count() as u64;
    let dep = cnt(&|r| r.v.class_b == Class::Dependent);
    let ind = cnt(&|r| r.v.class_b == Class::Independent);
    let und = cnt(&|r| r.v.class_b == Class::Undecided);
    let dep2 = cnt(&|r| r.v.class_b2 == Class::Dependent);
    let und2 = cnt(&|r| r.v.class_b2 == Class::Undecided);
    let contra = cnt(&|r| r.v.contradiction);
    let regviol = cnt(&|r| r.v.reg_consistency_violation);
    let (lo, hi) = wilson(dep, n);
    let (dlo, dhi) = wilson(dep, dep + ind);
    let (lo2, hi2) = wilson(dep2, n);
    // coefficient table like the paper's Table 4: counts of |coefficient| values over dependent runs
    let mut coef = [0u64; 9];
    let mut rel_forms = std::collections::BTreeMap::<String, u64>::new();
    for r in &rs {
        if let Some((a, b)) = r.v.rel_b {
            coef[a.unsigned_abs() as usize] += 1;
            coef[b.unsigned_abs() as usize] += 1;
            *rel_forms.entry(format!("({},{})", a.abs(), b.abs())).or_insert(0) += 1;
        }
    }
    let p1_2p2 = rs
        .iter()
        .filter(|r| matches!(r.v.rel_b, Some((a, b)) if (a.abs(), b.abs()) == (1, 2) || (a.abs(), b.abs()) == (2, 1)))
        .count();
    let disc_all: Vec<f64> = rs.iter().map(|r| log2_big(&r.lifted.disc)).collect();
    let disc_dep: Vec<f64> =
        rs.iter().filter(|r| r.v.class_b == Class::Dependent).map(|r| log2_big(&r.lifted.disc)).collect();
    let a002_all: Vec<f64> = rs.iter().map(|r| log2_big(&r.lifted.a002_key)).collect();
    let a002_dep: Vec<f64> =
        rs.iter().filter(|r| r.v.class_b == Class::Dependent).map(|r| log2_big(&r.lifted.a002_key)).collect();
    let secs: Vec<f64> = rs.iter().map(|r| r.seconds).collect();
    let mut levels = std::collections::BTreeMap::<u32, u64>::new();
    for r in rs.iter().filter(|r| r.v.class_b == Class::Independent) {
        *levels.entry(r.v.reg.level).or_insert(0) += 1;
    }
    let resamples: u64 = rs.iter().map(|r| r.resamples as u64).sum();
    json!({
        "sampler": sampler.name(),
        "n": n,
        "B": B,
        "dependent": dep, "independent": ind, "undecided": und,
        "dependent_rate": dep as f64 / n as f64,
        "dependent_rate_wilson95": [lo, hi],
        "decided": dep + ind,
        "dependent_rate_among_decided": if dep + ind > 0 { dep as f64 / (dep + ind) as f64 } else { f64::NAN },
        "dependent_rate_among_decided_wilson95": [dlo, dhi],
        "undecided_rate": und as f64 / n as f64,
        "B2": B2,
        "dependent_B2": dep2, "undecided_B2": und2,
        "dependent_rate_B2": dep2 as f64 / n as f64,
        "dependent_rate_B2_wilson95": [lo2, hi2],
        "certificate_contradictions": contra,
        "regulator_interval_excludes_zero_on_dependent_runs": regviol,
        "relations_of_form_P1_eq_pm2P2_or_2P1_eq_pmP2": p1_2p2,
        "relation_abs_forms": rel_forms,
        "coefficient_abs_counts_0_to_8": coef,
        "independent_runs_by_doublings_used": levels,
        "log2_abs_disc_integral_model_all": stats(&disc_all),
        "log2_abs_disc_integral_model_dependent": stats(&disc_dep),
        "log2_a002_disc_height_all": stats(&a002_all),
        "log2_a002_disc_height_dependent": stats(&a002_dep),
        "lift_resamples_total": resamples,
        "seconds_per_run": stats(&secs),
    })
}

fn cmd_run(n: u64, outdir: &str) {
    fs::create_dir_all(outdir).unwrap();
    let t = tables();
    eprintln!(
        "box sizes: {:?}",
        t.boxes.iter().map(|b| b.len()).collect::<Vec<_>>()
    );
    eprintln!("pools: P0 {:?}", t.pools[0]);
    let wall = Instant::now();
    let mut all: Vec<Row> = Vec::new();
    for s in [Sampler::S1, Sampler::S2, Sampler::S3] {
        let st = Instant::now();
        let rows: Vec<Row> = (0..n).into_par_iter().map(|k| do_run(&t, s, k)).collect();
        eprintln!("{} done: {} runs in {:.1}s", s.name(), rows.len(), st.elapsed().as_secs_f64());
        all.extend(rows);
    }
    let mut f = fs::File::create(format!("{}/runs.csv", outdir)).unwrap();
    f.write_all(CSV_HEADER.as_bytes()).unwrap();
    for r in &all {
        f.write_all(csv_row(r).as_bytes()).unwrap();
    }
    let per: Vec<serde_json::Value> =
        [Sampler::S1, Sampler::S2, Sampler::S3].iter().map(|&s| summarise(&all, s)).collect();
    // H7 inputs
    let rates: Vec<f64> = per.iter().map(|v| v["dependent_rate"].as_f64().unwrap()).collect();
    let cis: Vec<(f64, f64)> = per
        .iter()
        .map(|v| (v["dependent_rate_wilson95"][0].as_f64().unwrap(), v["dependent_rate_wilson95"][1].as_f64().unwrap()))
        .collect();
    let (imax, imin) = {
        let mut ix: Vec<usize> = (0..3).collect();
        ix.sort_by(|&a, &b| rates[a].partial_cmp(&rates[b]).unwrap());
        (ix[2], ix[0])
    };
    let spread = if rates[imin] > 0.0 { rates[imax] / rates[imin] } else { f64::INFINITY };
    let nonoverlap = cis[imin].1 < cis[imax].0;
    let mut pairs = Vec::new();
    for i in 0..3 {
        for j in (i + 1)..3 {
            let ratio = if rates[i].min(rates[j]) > 0.0 { rates[i].max(rates[j]) / rates[i].min(rates[j]) } else { f64::INFINITY };
            let disjoint = cis[i].1 < cis[j].0 || cis[j].1 < cis[i].0;
            pairs.push(json!({"pair": format!("S{}-S{}", i + 1, j + 1), "ratio": ratio, "wilson_disjoint": disjoint}));
        }
    }
    let pub_rate = PUBLISHED_DEP as f64 / PUBLISHED_N as f64;
    let (plo, phi) = wilson(PUBLISHED_DEP, PUBLISHED_N);
    let vs_pub: Vec<serde_json::Value> = (0..3)
        .map(|i| {
            json!({"sampler": format!("S{}", i + 1), "ratio_to_published": rates[i] / pub_rate,
                   "wilson_disjoint_from_published": cis[i].1 < plo || phi < cis[i].0})
        })
        .collect();
    let kill = spread > 3.0 && nonoverlap;
    let summary = json!({
        "experiment": "A7 P1.6/Q018 xedni p=17 Experiment A, three samplers",
        "protocol": "PROTOCOLS.md section A7",
        "seed": SEED,
        "constants": {"P": P, "curve": "y^2=x^3+2x+2", "group_order": GROUP_ORDER, "P0": [P0.0, P0.1],
            "mu2_range": [2, 17], "R_BOX": R_BOX, "POOL": POOL, "NEIGH_RADIUS": NEIGH_RADIUS as i64,
            "neighbourhood_size": 243, "LLL_DELTA": 0.99, "B": B, "B2": B2, "N_FILTER_PRIMES": N_FILTER_PRIMES,
            "FILTER_PRIME_START": FILTER_PRIME_START, "DOUBLINGS": DOUBLINGS, "BIT_CAP": BIT_CAP},
        "per_sampler": per,
        "published": {"dependent": PUBLISHED_DEP, "n": PUBLISHED_N, "rate": pub_rate, "wilson95": [plo, phi]},
        "H7": {"max_sampler": format!("S{}", imax + 1), "min_sampler": format!("S{}", imin + 1),
               "max_over_min_rate": spread, "extreme_wilson_disjoint": nonoverlap,
               "pairs": pairs, "kill_condition_met": kill, "vs_published": vs_pub},
        "wall_seconds": wall.elapsed().as_secs_f64(),
        "threads": rayon::current_num_threads(),
    });
    fs::write(format!("{}/summary.json", outdir), serde_json::to_string_pretty(&summary).unwrap()).unwrap();
    eprintln!("total wall {:.1}s", wall.elapsed().as_secs_f64());
}

// ---------------- controls ----------------

fn general_from(a: [i64; 5]) -> General {
    General {
        a1: BigInt::from(a[0]),
        a2: BigInt::from(a[1]),
        a3: BigInt::from(a[2]),
        a4: BigInt::from(a[3]),
        a6: BigInt::from(a[4]),
    }
}

fn qpt(x: (i64, i64), y: (i64, i64)) -> Pt {
    Some((Q::new(BigInt::from(x.0), BigInt::from(x.1)), Q::new(BigInt::from(y.0), BigInt::from(y.1))))
}

struct CtrlOut {
    lines: Vec<String>,
    failures: u64,
}

fn control_case(out: &mut CtrlOut, name: &str, g: &General, p1: &Pt, p2: &Pt, expect: &str) {
    assert!(g.contains(p1) && g.contains(p2), "control point not on curve: {}", name);
    let (s, inv) = g.to_short();
    let q1 = g.map_to_short(&inv, p1);
    let q2 = g.map_to_short(&inv, p2);
    let gg = g.clone();
    let (pp1, pp2) = (p1.clone(), p2.clone());
    let v = dependence_test_short(&s, &q1, &q2, &move |r: (i64, i64)| {
        gg.add(&gg.mul(r.0, &pp1), &gg.mul(r.1, &pp2)).is_none()
    });
    let ok = match expect {
        "dependent" => v.class_b == Class::Dependent,
        "not_independent" => v.class_b != Class::Independent,
        "dependent_B64_only" => v.class_b == Class::Undecided && v.class_b2 == Class::Dependent,
        "independent" => v.class_b == Class::Independent,
        "not_dependent" => v.class_b != Class::Dependent && v.class_b2 != Class::Dependent,
        _ => panic!("bad expectation"),
    } && !v.contradiction && !v.reg_consistency_violation;
    if !ok {
        out.failures += 1;
    }
    out.lines.push(format!(
        "{},{},{},{},{},{},{},{},{:.9e},{:.9e},{:.9e},{:.9e},{:.9e},{:.9e},{}\n",
        name.replace(',', " "),
        expect,
        short_str(&pt_string(p1)),
        short_str(&pt_string(p2)),
        v.class_b.s(),
        rel_s(&v.rel_b),
        v.class_b2.s(),
        rel_s(&v.rel_b2),
        v.reg.reg_lo,
        v.reg.reg_hi,
        v.reg.h1.0,
        v.reg.h1.1,
        v.reg.h2.0,
        v.reg.h2.1,
        if ok { "pass" } else { "FAIL" }
    ));
}

fn short_str(s: &str) -> String {
    if s.len() > 160 {
        format!("(omitted: {} chars)", s.len())
    } else {
        s.to_string()
    }
}

fn cmd_controls(outdir: &str) {
    fs::create_dir_all(outdir).unwrap();
    let mut out = CtrlOut { lines: Vec::new(), failures: 0 };
    // 389.a1 [0,1,1,-2,0], rank 2, LMFDB generators (0,0), (1,0)
    let e389 = general_from([0, 1, 1, -2, 0]);
    control_case(&mut out, "389.a1 gens", &e389, &qpt((0, 1), (0, 1)), &qpt((1, 1), (0, 1)), "independent");
    control_case(&mut out, "389.a1 (-1,1),(0,0)", &e389, &qpt((-1, 1), (1, 1)), &qpt((0, 1), (0, 1)), "independent");
    // 5077.a1 [0,0,1,-7,6], rank 3; three integral points
    let e5077 = general_from([0, 0, 1, -7, 6]);
    control_case(&mut out, "5077.a1 (-2,3),(-1,3)", &e5077, &qpt((-2, 1), (3, 1)), &qpt((-1, 1), (3, 1)), "independent");
    control_case(&mut out, "5077.a1 (-2,3),(0,2)", &e5077, &qpt((-2, 1), (3, 1)), &qpt((0, 1), (2, 1)), "independent");
    // planted relations on 37.a1 [0,0,1,-1,0], generator (0,0)
    let e37 = general_from([0, 0, 1, -1, 0]);
    let g37 = qpt((0, 1), (0, 1));
    control_case(&mut out, "37.a1 P,2P", &e37, &g37, &e37.mul(2, &g37), "dependent");
    control_case(&mut out, "37.a1 2P,-3P", &e37, &e37.mul(2, &g37), &e37.mul(-3, &g37), "dependent");
    control_case(&mut out, "37.a1 P,11P", &e37, &g37, &e37.mul(11, &g37), "dependent_B64_only");
    // torsion: y^2 = x^3 + 1 (36.a), points of order 6 and 3
    let e36 = general_from([0, 0, 0, 0, 1]);
    control_case(&mut out, "y2=x3+1 (2,3),(0,1)", &e36, &qpt((2, 1), (3, 1)), &qpt((0, 1), (1, 1)), "dependent");
    // 11.a2 [0,-1,1,-10,-20], torsion Z/5 generator (5,5)
    let e11 = general_from([0, -1, 1, -10, -20]);
    let t11 = qpt((5, 1), (5, 1));
    control_case(&mut out, "11.a2 T,2T", &e11, &t11, &e11.mul(2, &t11), "dependent");
    // torsion + non-torsion: y^2 = x^3 - 2x, T=(0,0), P=(2,2)
    let e2 = general_from([0, 0, 0, -2, 0]);
    let p2 = qpt((2, 1), (2, 1));
    let t2 = qpt((0, 1), (0, 1));
    control_case(&mut out, "y2=x3-2x T,P", &e2, &t2, &p2, "dependent");
    control_case(&mut out, "y2=x3-2x P,P+T", &e2, &p2, &e2.add(&p2, &t2), "dependent");
    // planted on lifted xedni curves (first 200 S1 runs): (P1,2P1), (2P1,-3P1), (P1,11P1)
    let t = tables();
    let mut planted = [0u64; 3];
    for k in 0..200u64 {
        let row = do_run(&t, Sampler::S1, k);
        let l = &row.lifted;
        let p1 = l.pts_gen[0].clone();
        let name = format!("S1 run {} ", k);
        let n0 = out.failures;
        control_case(&mut out, &(name.clone() + "P1,2P1"), &l.gen, &p1, &l.gen.mul(2, &p1), "dependent");
        control_case(&mut out, &(name.clone() + "2P1,-3P1"), &l.gen, &l.gen.mul(2, &p1), &l.gen.mul(-3, &p1), "dependent");
        control_case(&mut out, &(name.clone() + "P1,11P1"), &l.gen, &p1, &l.gen.mul(11, &p1), "not_independent");
        if out.failures == n0 {
            planted[0] += 1;
        }
        let _ = &mut planted;
    }
    // height reference values (LMFDB/Sage normalisation), level-8 enclosures
    let mut href = Vec::new();
    for (name, g, p, val) in [
        ("37.a1 (0,0)", e37.clone(), qpt((0, 1), (0, 1)), 0.0511114082399688f64),
        ("389.a1 (0,0)", e389.clone(), qpt((0, 1), (0, 1)), 0.32700077365160495),
        ("389.a1 (1,0)", e389.clone(), qpt((1, 1), (0, 1)), 0.47671165934373954),
    ] {
        let (s, inv) = g.to_short();
        let q = g.map_to_short(&inv, &p);
        let ctx = HeightCtx::new(&s);
        let (lo, hi) = heights::height_enclosure(&ctx, &q, 8);
        let inside = lo <= val && val <= hi;
        if !inside {
            out.failures += 1;
        }
        href.push(json!({"point": name, "reference": val, "enclosure_level8": [lo, hi], "contains_reference": inside}));
    }
    // 389.a1 regulator at level 8 vs reference 0.152460177943144 (LMFDB)
    let (s, inv) = e389.to_short();
    let ctx = HeightCtx::new(&s);
    let r389 = regulator(
        &s,
        &ctx,
        &e389.map_to_short(&inv, &qpt((0, 1), (0, 1))),
        &e389.map_to_short(&inv, &qpt((1, 1), (0, 1))),
        &[8],
        BIT_CAP,
        false,
    );
    let reg_ref = 0.152460177943144f64;
    let reg_inside = r389.reg_lo <= reg_ref && reg_ref <= r389.reg_hi;
    if !reg_inside {
        out.failures += 1;
    }
    let mut f = fs::File::create(format!("{}/controls.csv", outdir)).unwrap();
    f.write_all(b"case,expectation,P1,P2,class_B8,relation_B8,class_B64,relation_B64,reg_lo,reg_hi,h1_lo,h1_hi,h2_lo,h2_hi,result\n")
        .unwrap();
    for l in &out.lines {
        f.write_all(l.as_bytes()).unwrap();
    }
    let passes = out.lines.iter().filter(|l| l.ends_with("pass\n")).count();
    let summary = json!({
        "cases": out.lines.len(),
        "case_passes": passes,
        "failures_total": out.failures,
        "planted_xedni_runs_all_three_pass": planted[0],
        "planted_xedni_runs": 200,
        "height_reference_checks": href,
        "regulator_389a1": {"reference": reg_ref, "enclosure_level8": [r389.reg_lo, r389.reg_hi], "contains_reference": reg_inside},
    });
    fs::write(format!("{}/controls_summary.json", outdir), serde_json::to_string_pretty(&summary).unwrap()).unwrap();
    eprintln!("controls: {} cases, failures {}", out.lines.len(), out.failures);
}

// ---------------- fixtures for the upstream-Python cross-check ----------------

fn cmd_fixtures(outdir: &str) {
    fs::create_dir_all(outdir).unwrap();
    let t = tables();
    let mut items = Vec::new();
    // fixed inputs: pools of P0 and mu*P0, mu in {2,5,11,17}, pool indices fixed
    for (mu, i1, i2) in [(2u64, 0usize, 0usize), (5, 1, 3), (11, 2, 7), (17, 4, 5), (8, 7, 2)] {
        let (pa, pb) = (&t.pools[0], &t.pools[(mu - 1) as usize]);
        let lifts = [pa[i1 % pa.len()], pb[i2 % pb.len()]];
        let fin = [t.fin_pts[0], t.fin_pts[(mu - 1) as usize]];
        let lat = match solve(&lifts, P as i128, A_FIN as i128, B_FIN as i128) {
            Ok(l) => l,
            Err(e) => {
                items.push(json!({"mu2": mu, "lifts": lifts, "error": format!("{:?}", e)}));
                continue;
            }
        };
        let ustar = closest_to_zero(&lat).unwrap();
        let l = build_curve(&lifts, &ustar, &fin);
        let s = |v: &BigInt| v.to_string();
        let model = l.as_ref().map(|l| {
            json!({"a1": s(&l.gen.a1), "a2": s(&l.gen.a2), "a3": s(&l.gen.a3), "a4": s(&l.gen.a4), "a6": s(&l.gen.a6),
                   "P1": pt_string(&l.pts_gen[0]), "P2": pt_string(&l.pts_gen[1]), "disc": s(&l.disc)})
        });
        let st = |v: &V7| v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        items.push(json!({
            "mu2": mu, "lifts": lifts, "pool_P0": t.pools[0], "pool_P2": t.pools[(mu - 1) as usize],
            "rows": lat.rows.iter().map(st).collect::<Vec<_>>(), "rhs": lat.rhs.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
            "particular": st(&lat.particular), "basis": lat.basis.iter().map(st).collect::<Vec<_>>(),
            "ustar": st(&ustar), "integral_model": model,
        }));
    }
    fs::write(format!("{}/fixtures.json", outdir), serde_json::to_string_pretty(&json!(items)).unwrap()).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("run") => cmd_run(args[2].parse().unwrap(), &args[3]),
        Some("controls") => cmd_controls(&args[2]),
        Some("fixtures") => cmd_fixtures(&args[2]),
        Some("time") => {
            // timing probe: n runs of each sampler, no output files
            let n: u64 = args[2].parse().unwrap();
            let t = tables();
            for s in [Sampler::S1, Sampler::S2, Sampler::S3] {
                let st = Instant::now();
                let rows: Vec<Row> = (0..n).into_par_iter().map(|k| do_run(&t, s, k)).collect();
                let undec = rows.iter().filter(|r| r.v.class_b == Class::Undecided).count();
                let lv: Vec<u32> = rows.iter().map(|r| r.v.reg.level).collect();
                let mx = rows.iter().map(|r| r.seconds).fold(0.0, f64::max);
                eprintln!(
                    "{}: {} runs {:.2}s, undecided {}, max run {:.3}s, levels>=5: {}",
                    s.name(),
                    n,
                    st.elapsed().as_secs_f64(),
                    undec,
                    mx,
                    lv.iter().filter(|&&x| x >= 5).count()
                );
            }
        }
        _ => {
            eprintln!("usage: a7_xedni_q018 run <n> <outdir> | controls <outdir> | fixtures <outdir> | time <n>");
            std::process::exit(2);
        }
    }
}
