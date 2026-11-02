//! ic-weakpoints: exact measurements for the six EC index-calculus weak points
//! on Gaudry/Diem Weil-descent systems over F_{p^n}, n in {2,3}.
//!
//!   algebra  --n N --p P --inst K --seed S [--enum]   rows 1 and 5
//!   yield    --p P --curves C --targets T --seed S     rows 3 and 2
//!   pipeline --p P --curves C --rho R --seed S         rows 4 and 6 (+ cost band)
//!
//! Every line printed on stdout is one JSON record. Counts only: no timings.
mod curve;
mod field;
mod poly;
mod summarize;

use curve::*;
use field::*;
use poly::*;
use std::collections::{HashMap, HashSet};

fn arg(args: &[String], k: &str, d: u64) -> u64 {
    args.iter().position(|a| a == k).map(|i| args[i + 1].parse().unwrap()).unwrap_or(d)
}
fn flag(args: &[String], k: &str) -> bool {
    args.iter().any(|a| a == k)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("algebra") => algebra(&args),
        Some("yield") => yield_cmd(&args),
        Some("pipeline") => pipeline(&args),
        Some("selftest") => selftest(),
        Some("summarize") => summarize::run(args.get(2).map(|s| s.as_str()).unwrap_or("data")),
        _ => eprintln!("usage: ic-weakpoints algebra|yield|pipeline|selftest ..."),
    }
}

/// e-variable Weil-restricted system for decomposing x_R into n factor-base points.
fn esystem(c: &Curve, n: usize, xr: &E) -> (Poly, Vec<PPoly>) {
    let s = semaev(c, n, xr);
    let e = symmetrize(&c.f, &s, n);
    let sys = weil(&c.f, &e);
    (e, sys)
}

fn selftest() {
    let mut rng = Rng(7);
    for &(p, n) in &[(101u64, 2usize), (103, 3), (1009, 3)] {
        let f = Fq::new(p, n, &mut rng);
        // field sanity
        for _ in 0..50 {
            let a = f.rand(&mut rng);
            if !f.is_zero(&a) {
                assert_eq!(f.mul(&a, &f.inv(&a)), f.one());
                let s = f.sq(&a);
                let r = f.sqrt(&s).unwrap();
                assert_eq!(f.sq(&r), s);
            }
        }
        let c = Curve { f: f.clone(), a: f.rand(&mut rng), b: f.rand(&mut rng) };
        let n_ord = group_order(&c, &mut rng).expect("order");
        let pt = c.rand_point(&mut rng);
        assert!(c.mul(n_ord, &pt).is_none(), "order check");
        // Semaev check
        for _ in 0..5 {
            let pts: Vec<Pt> = (0..n).map(|_| c.rand_point(&mut rng)).collect();
            let mut r: Pt = None;
            for q in &pts {
                r = c.add(&r, q);
            }
            let xr = r.unwrap().0;
            let xs: Vec<E> = pts.iter().map(|q| q.unwrap().0).collect();
            let s = semaev(&c, n, &xr);
            assert!(f.is_zero(&poly::mpeval(&f, &s, &xs)), "S_{} vanishes", n + 1);
            let e = symmetrize(&f, &s, n);
            let ev: Vec<E> = match n {
                2 => vec![f.add(&xs[0], &xs[1]), f.mul(&xs[0], &xs[1])],
                _ => vec![
                    f.add(&f.add(&xs[0], &xs[1]), &xs[2]),
                    f.add(&f.add(&f.mul(&xs[0], &xs[1]), &f.mul(&xs[0], &xs[2])), &f.mul(&xs[1], &xs[2])),
                    f.mul(&f.mul(&xs[0], &xs[1]), &xs[2]),
                ],
            };
            assert!(f.is_zero(&poly::mpeval(&f, &e, &ev)), "symmetrised form vanishes");
            // random non-sum must not vanish (overwhelmingly)
            let bad = f.rand(&mut rng);
            let s2 = semaev(&c, n, &bad);
            assert!(!f.is_zero(&poly::mpeval(&f, &s2, &xs)));
        }
        println!("{{\"selftest\":\"ok\",\"p\":{},\"n\":{},\"order\":{}}}", p, n, n_ord);
    }
}

// ---------------------------------------------------------------- factor base / oracle
struct Fb {
    xs: Vec<u64>,
    ys: Vec<E>, // canonical y
    idx: HashMap<u64, usize>,
}
fn factor_base(c: &Curve) -> Fb {
    let f = &c.f;
    let (mut xs, mut ys, mut idx) = (vec![], vec![], HashMap::new());
    for x in 0..f.p {
        if let Some(y) = f.sqrt(&c.rhs(&f.c(x))) {
            let (cy, _) = f.canon_sign(&y);
            idx.insert(x, xs.len());
            xs.push(x);
            ys.push(cy);
        }
    }
    Fb { xs, ys, idx }
}
fn fb_point(f: &Fq, fb: &Fb, i: usize, s: i8) -> Pt {
    let y = if s > 0 { fb.ys[i] } else { f.neg(&fb.ys[i]) };
    Some((f.c(fb.xs[i]), y))
}
/// sign of point P relative to the canonical y of its x-class (+1 for y = 0)
fn sign_of(f: &Fq, fb: &Fb, i: usize, y: &E) -> i8 {
    if f.is_zero(y) || *y == fb.ys[i] { 1 } else { -1 }
}

type Decomp = [(usize, i8); 3];

/// All decompositions R = P1 + P2 + P3 with x(P_i) in F_p (n = 3), each verified by point addition.
/// Returns (decompositions, point additions spent).
fn decompose3(c: &Curve, fb: &Fb, r: &Pt, rng: &mut Rng) -> (Vec<Decomp>, u64) {
    let f = &c.f;
    let p = f.p;
    let mut out: HashSet<Decomp> = HashSet::new();
    let mut adds = 0u64;
    for i1 in 0..fb.xs.len() {
        for s1 in [1i8, -1] {
            if s1 < 0 && f.is_zero(&fb.ys[i1]) {
                continue;
            }
            let p1 = fb_point(f, fb, i1, s1);
            let rp = c.sub(r, &p1);
            adds += 1;
            let Some((xr, _)) = rp else { continue };
            for (s, t) in two_decomp_st(c, &xr, rng) {
                let disc = (mulm(s, s, p) + p - mulm(4, t, p)) % p;
                let Some(sq) = sqrt_mod(disc, p) else { continue };
                let inv2 = invm(2, p);
                let x2 = mulm((s + sq) % p, inv2, p);
                let x3 = mulm((s + p - sq) % p, inv2, p);
                let (Some(&i2), Some(&i3)) = (fb.idx.get(&x2), fb.idx.get(&x3)) else { continue };
                for s2 in [1i8, -1] {
                    let p2 = fb_point(f, fb, i2, s2);
                    let p3 = c.sub(&rp, &p2);
                    adds += 1;
                    if let Some((x, y)) = p3 {
                        if x == f.c(x3) {
                            let s3 = sign_of(f, fb, i3, &y);
                            let s2n = sign_of(f, fb, i2, &p2.unwrap().1);
                            let s1n = sign_of(f, fb, i1, &p1.unwrap().1);
                            let mut d = [(i1, s1n), (i2, s2n), (i3, s3)];
                            d.sort();
                            out.insert(d);
                        }
                    }
                }
            }
        }
    }
    let v: Vec<Decomp> = out.into_iter().collect();
    for d in &v {
        let mut acc: Pt = None;
        for &(i, s) in d {
            acc = c.add(&acc, &fb_point(f, fb, i, s));
        }
        assert_eq!(acc, *r, "decomposition failed verification");
    }
    (v, adds)
}

/// (s,t) in F_p^2 with S_3(u,v,X)=0, s=u+v, t=uv, by exact Weil restriction (n=3).
fn two_decomp_st(c: &Curve, x: &E, rng: &mut Rng) -> Vec<(u64, u64)> {
    let f = &c.f;
    let p = f.p;
    let (a, b) = (c.a, c.b);
    let x2 = f.sq(x);
    // E(s,t) = t^2 + (c1a + c1b s) t + (c0a + c0b s + c0c s^2)
    let c1a = f.sub(&f.mul(&f.ci(-4), &x2), &f.add(&a, &a));
    let c1b = f.mul(&f.ci(-2), x);
    let c0a = f.sub(&f.sq(&a), &f.mul(&f.c(4), &f.mul(&b, x)));
    let c0b = f.neg(&f.add(&f.mul(&f.c(2), &f.mul(&a, x)), &f.mul(&f.c(4), &b)));
    let c0c = x2;
    let comp = |j: usize| -> (Vec<u64>, Vec<u64>) {
        (vec![c1a[j], c1b[j]], vec![c0a[j], c0b[j], c0c[j]])
    };
    let (l1, q1) = comp(1);
    let (l2, q2) = comp(2);
    let (l0, q0) = comp(0);
    let res = ptrim({
        let a1 = pmul(&l1, &q2, p);
        let a2 = pmul(&l2, &q1, p);
        let n = a1.len().max(a2.len());
        (0..n)
            .map(|i| (a1.get(i).copied().unwrap_or(0) + p - a2.get(i).copied().unwrap_or(0)) % p)
            .collect()
    });
    let svals: Vec<u64> = if res.is_empty() { (0..p).collect() } else { proots(&res, p, rng) };
    let mut out = vec![];
    for s in svals {
        let (a1, b1) = (peval(&l1, s, p), peval(&q1, s, p));
        let (a2, b2) = (peval(&l2, s, p), peval(&q2, s, p));
        let mut tc = vec![];
        if a1 != 0 {
            tc.push(mulm(p - b1, invm(a1, p), p));
        } else if a2 != 0 {
            tc.push(mulm(p - b2, invm(a2, p), p));
        } else if b1 == 0 && b2 == 0 {
            let (a0, b0) = (peval(&l0, s, p), peval(&q0, s, p));
            tc.extend(proots(&[b0, a0, 1], p, rng));
        }
        for t in tc {
            let ok = [0usize, 1, 2].iter().all(|&j| {
                let (l, q) = comp(j);
                let v = (if j == 0 { mulm(t, t, p) } else { 0 } + mulm(peval(&l, s, p), t, p) + peval(&q, s, p)) % p;
                v == 0
            });
            if ok {
                out.push((s, t));
            }
        }
    }
    out
}

// ---------------------------------------------------------------- algebra (rows 1, 5)
fn algebra(args: &[String]) {
    let n = arg(args, "--n", 3) as usize;
    let p = arg(args, "--p", 101);
    let inst = arg(args, "--inst", 4);
    let seed = arg(args, "--seed", 20261102);
    let dcap = arg(args, "--dcap", 16) as u32;
    let do_enum = flag(args, "--enum");
    for k in 0..inst {
        let mut rng = Rng(derive(seed, "algebra", p * 10 + n as u64, k));
        let f = Fq::new(p, n, &mut rng);
        let c = loop {
            let c = Curve { f: f.clone(), a: f.rand(&mut rng), b: f.rand(&mut rng) };
            if c.disc_ok() {
                break c;
            }
        };
        let r = c.rand_point(&mut rng);
        let xr = r.unwrap().0;
        let (e, sys) = esystem(&c, n, &xr);
        let maxdeg = e.keys().map(tdeg).max().unwrap_or(0);
        let mac = macaulay_probe(p, &sys, n, dcap);
        let steps: Vec<String> = mac
            .steps
            .iter()
            .map(|s| {
                format!(
                    "{{\"d\":{},\"rows\":{},\"cols\":{},\"rank\":{},\"mults\":{},\"std\":{},\"fall\":{}}}",
                    s.d, s.rows, s.cols, s.rank, s.mults,
                    s.std.map_or("null".into(), |v| v.to_string()), s.fall
                )
            })
            .collect();
        let sd = mac.solving_degree;
        let at_sd = sd.and_then(|d| mac.steps.iter().find(|s| s.d == d));
        let mut extra = String::new();
        if do_enum && n == 3 {
            extra = enum3(&c, &sys, &r, &mut rng);
        }
        if do_enum && n == 2 {
            extra = enum2(&c, &sys);
        }
        println!(
            "{{\"exp\":\"algebra\",\"n\":{},\"p\":{},\"inst\":{},\"terms_e\":{},\"eqs\":{},\"maxdeg_e\":{},\"bezout\":{},\"first_fall\":{},\"solving_degree\":{},\"quotient_dim\":{},\"zero_dim\":{},\"sd_rows\":{},\"sd_cols\":{},\"sd_mults\":{},\"steps\":[{}]{}}}",
            n, p, k, e.len(), sys.len(), maxdeg,
            (maxdeg as u64).pow(n as u32),
            mac.first_fall.map_or("null".into(), |v| v.to_string()),
            sd.map_or("null".into(), |v| v.to_string()),
            mac.quotient_dim.map_or("null".into(), |v| v.to_string()),
            mac.quotient_dim.is_some(),
            at_sd.map_or(0, |s| s.rows), at_sd.map_or(0, |s| s.cols), at_sd.map_or(0, |s| s.mults),
            steps.join(","), extra
        );
    }
}

/// Enumerate F_p-rational solutions of the n=3 e-system; classify genuine vs spurious; multiplicity.
fn enum3(c: &Curve, sys: &[PPoly], r: &Pt, rng: &mut Rng) -> String {
    let f = &c.f;
    let p = f.p;
    let fb = factor_base(c);
    let mut sols = vec![];
    for e1 in 0..p {
        for e2 in 0..p {
            // univariate in e3: gcd of the restricted equations
            let mut g: Option<Vec<u64>> = None;
            for q in sys {
                let mut u = vec![0u64; 9];
                for (m, co) in q {
                    let v = mulm(*co, mulm(powm(e1, m[0] as u128, p), powm(e2, m[1] as u128, p), p), p);
                    u[m[2] as usize] = (u[m[2] as usize] + v) % p;
                }
                let u = ptrim(u);
                g = Some(match g {
                    None => u,
                    Some(h) => pgcd(&h, &u, p),
                });
            }
            let g = g.unwrap();
            if g.is_empty() {
                sols.push((e1, e2, u64::MAX)); // whole line: positive-dimensional
                continue;
            }
            for e3 in proots(&g, p, rng) {
                sols.push((e1, e2, e3));
            }
        }
    }
    let posdim = sols.iter().filter(|s| s.2 == u64::MAX).count();
    let (mut genuine, mut split_not_fb, mut nonsplit, mut multiple) = (0, 0, 0, 0);
    let jac: Vec<Vec<PPoly>> = sys.iter().map(|q| (0..3).map(|i| ppderiv(p, q, i)).collect()).collect();
    for &(e1, e2, e3) in sols.iter().filter(|s| s.2 != u64::MAX) {
        // cubic T^3 - e1 T^2 + e2 T - e3
        let cub = vec![(p - e3) % p, e2, (p - e1) % p, 1];
        let rts = proots(&cub, p, rng);
        // multiplicity-aware split test: product of (T - r)^k must equal cubic
        let mut rem = cub.clone();
        let mut roots_m = vec![];
        for &x in &rts {
            while rem.len() > 1 && peval(&rem, x, p) == 0 {
                rem = pdiv_exact(&rem, &[(p - x) % p, 1], p);
                roots_m.push(x);
            }
        }
        if roots_m.len() == 3 {
            if roots_m.iter().all(|x| fb.idx.contains_key(x)) {
                genuine += 1;
            } else {
                split_not_fb += 1;
            }
        } else {
            nonsplit += 1;
        }
        // Jacobian rank at the point (3 or more equations, 3 unknowns): simple iff rank 3
        let v = [e1, e2, e3];
        let mut mrows: Vec<Vec<u64>> = jac.iter().map(|row| row.iter().map(|d| ppeval(p, d, &v)).collect()).collect();
        if rank_mod(&mut mrows, p) < 3 {
            multiple += 1;
        }
    }
    let (dec, _) = decompose3(c, &fb, r, rng);
    let xsets: HashSet<[usize; 3]> = dec.iter().map(|d| [d[0].0, d[1].0, d[2].0]).collect();
    format!(
        ",\"enum\":{{\"rational\":{},\"posdim_lines\":{},\"genuine\":{},\"split_not_fb\":{},\"nonsplit\":{},\"singular\":{},\"oracle_xsets\":{},\"oracle_signed\":{}}}",
        sols.len() - posdim, posdim, genuine, split_not_fb, nonsplit, multiple, xsets.len(), dec.len()
    )
}

fn enum2(c: &Curve, sys: &[PPoly]) -> String {
    let p = c.f.p;
    let mut rat = 0;
    for e1 in 0..p {
        for e2 in 0..p {
            if sys.iter().all(|q| ppeval(p, q, &[e1, e2]) == 0) {
                rat += 1;
            }
        }
    }
    format!(",\"enum\":{{\"rational\":{}}}", rat)
}

fn rank_mod(m: &mut [Vec<u64>], p: u64) -> usize {
    let (rows, cols) = (m.len(), m.first().map_or(0, |r| r.len()));
    let mut rank = 0;
    for col in 0..cols {
        let Some(pr) = (rank..rows).find(|&r| m[r][col] != 0) else { continue };
        m.swap(rank, pr);
        let inv = invm(m[rank][col], p);
        for r in rank + 1..rows {
            let fct = mulm(m[r][col], inv, p);
            if fct != 0 {
                for j in col..cols {
                    m[r][j] = (m[r][j] + p - mulm(fct, m[rank][j], p)) % p;
                }
            }
        }
        rank += 1;
    }
    rank
}

// ---------------------------------------------------------------- yield (rows 3, 2)
fn yield_cmd(args: &[String]) {
    let p = arg(args, "--p", 101);
    let curves = arg(args, "--curves", 2);
    let targets = arg(args, "--targets", 500);
    let seed = arg(args, "--seed", 20261102);
    let tail = arg(args, "--tail", u64::MAX);
    let phis = [1.0f64, 0.5, 0.25, 0.125];
    for ci in 0..curves {
        let mut rng = Rng(derive(seed, "yield", p, ci));
        let f = Fq::new(p, 3, &mut rng);
        let (c, nord, h, ell) = prime_order_curve(&f, 16, &mut rng);
        let fb = factor_base(&c);
        let nfb = fb.xs.len();
        let mut counts = vec![];
        let mut full_phi = vec![vec![]; phis.len()];
        let mut lp1_phi = vec![vec![]; phis.len()];
        let mut adds_total = 0u64;
        for _ in 0..targets {
            let r = c.rand_point(&mut rng);
            let (dec, adds) = decompose3(&c, &fb, &r, &mut rng);
            adds_total += adds;
            counts.push(dec.len() as u64);
            if dec.len() as u64 >= tail {
                let (x, _) = r.unwrap();
                let xs: std::collections::BTreeSet<usize> = dec.iter().flat_map(|d| d.iter().map(|z| z.0)).collect();
                let rep = dec.iter().filter(|d| d[0].0 == d[1].0 || d[1].0 == d[2].0).count();
                let o = point_order(&c, &r, nord);
                println!("{{\"exp\":\"tail\",\"p\":{},\"curve\":{},\"target\":{},\"count\":{},\"xR\":{:?},\"xR_in_Fp\":{},\"order_R\":{},\"distinct_x_used\":{},\"with_repeated_point\":{}}}", p, ci, counts.len() - 1, dec.len(), &x[..3], f.in_base(&x), o, xs.len(), rep);
            }
            for (pi, &phi) in phis.iter().enumerate() {
                let cut = ((phi * nfb as f64).ceil() as usize).max(1);
                let inside = |d: &Decomp| d.iter().filter(|&&(i, _)| i < cut).count();
                full_phi[pi].push(dec.iter().filter(|d| inside(d) == 3).count() as u64);
                lp1_phi[pi].push(dec.iter().filter(|d| inside(d) == 2).count() as u64);
            }
        }
        // prediction: unordered multisets of FB points (2 signs, y=0 counted once) summing to R
        let npts: f64 = fb.ys.iter().map(|y| if f.is_zero(y) { 1.0 } else { 2.0 }).sum();
        let lambda = npts.powi(3) / 6.0 / nord as f64;
        let js = |v: &Vec<u64>| format!("[{}]", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
        let fulls: Vec<String> = full_phi.iter().map(js).collect();
        let lp1s: Vec<String> = lp1_phi.iter().map(js).collect();
        println!(
            "{{\"exp\":\"yield\",\"p\":{},\"curve\":{},\"N\":{},\"h\":{},\"ell\":{},\"fb_xclasses\":{},\"fb_points\":{},\"lambda_pred\":{:.6},\"targets\":{},\"oracle_adds\":{},\"phis\":[1,0.5,0.25,0.125],\"counts\":{},\"full_phi\":[{}],\"lp1_phi\":[{}]}}",
            p, ci, nord, h, ell, nfb, npts, lambda, targets, adds_total, js(&counts), fulls.join(","), lp1s.join(",")
        );
    }
}

// ---------------------------------------------------------------- pipeline (rows 4, 6)
struct Rel {
    cols: Vec<(usize, i64)>,
    b: u128,
    a: u128,
}

fn solve_mod(rels: &[Rel], ncol: usize, ell: u128, kcol: usize, use_signs: bool, h_scale: bool, h: u128) -> (usize, usize, bool, Option<u128>, u64) {
    // returns (rank, aug_rank, consistent, k if determined, mults)
    let m = ell;
    let mm = |a: u128, b: u128| mulmod128(a, b, m);
    let inv = |a: u128| {
        let mut r = 1u128;
        let (mut b, mut e) = (a % m, m - 2);
        while e > 0 {
            if e & 1 == 1 {
                r = mm(r, b);
            }
            b = mm(b, b);
            e >>= 1;
        }
        r
    };
    let w = ncol + 1;
    let mut a: Vec<Vec<u128>> = rels
        .iter()
        .map(|r| {
            let mut row = vec![0u128; w];
            for &(c, s) in &r.cols {
                let s = if use_signs { s } else { s.abs() };
                row[c] = (row[c] + if s >= 0 { s as u128 } else { m - ((-s) as u128 % m) }) % m;
            }
            let scale = if h_scale { h % m } else { 1 };
            row[kcol] = (m - mm(scale, r.b % m)) % m;
            row[ncol] = mm(scale, r.a % m);
            row
        })
        .collect();
    let rows = a.len();
    let mut rank = 0;
    let mut mults = 0u64;
    let mut pivcols = vec![];
    for col in 0..w {
        let Some(pr) = (rank..rows).find(|&r| a[r][col] != 0) else { continue };
        a.swap(rank, pr);
        let iv = inv(a[rank][col]);
        for j in col..w {
            a[rank][j] = mm(a[rank][j], iv);
        }
        mults += (w - col) as u64;
        for r in 0..rows {
            if r != rank && a[r][col] != 0 {
                let fct = a[r][col];
                for j in col..w {
                    if a[rank][j] != 0 {
                        a[r][j] = (a[r][j] + m - mm(fct, a[rank][j])) % m;
                    }
                }
                mults += (w - col) as u64;
            }
        }
        pivcols.push(col);
        rank += 1;
    }
    let consistent = !pivcols.contains(&ncol);
    let coef_rank = pivcols.iter().filter(|&&c| c != ncol).count();
    let mut k = None;
    if consistent {
        if let Some(ri) = pivcols.iter().position(|&c| c == kcol) {
            let free: Vec<usize> = (0..ncol).filter(|c| !pivcols.contains(c)).collect();
            if free.iter().all(|&fc| a[ri][fc] == 0) {
                k = Some(a[ri][ncol]);
            }
        }
    }
    (coef_rank, rank, consistent, k, mults)
}

fn pipeline(args: &[String]) {
    let p = arg(args, "--p", 101);
    let curves = arg(args, "--curves", 2);
    let rho_runs = arg(args, "--rho", 20);
    let seed = arg(args, "--seed", 20261102);
    let extra = arg(args, "--extra", 20) as usize;
    for ci in 0..curves {
        let mut rng = Rng(derive(seed, "pipeline", p, ci));
        let f = Fq::new(p, 3, &mut rng);
        let (c, nord, h, ell) = prime_order_curve(&f, 16, &mut rng);
        let g = loop {
            let g = c.mul(h, &c.rand_point(&mut rng));
            if g.is_some() {
                break g;
            }
        };
        assert!(c.mul(ell, &g).is_none());
        let k_secret = 1 + rng.below128(ell - 1);
        let q = c.mul(k_secret, &g);
        let fb = factor_base(&c);
        let nfb = fb.xs.len();
        let kcol = nfb;
        let ncol = nfb + 1;
        // c(3): Macaulay cost at the solving degree on 3 targets of this curve
        let mut c3 = vec![];
        let mut sds = vec![];
        for _ in 0..3 {
            let xr = c.rand_point(&mut rng).unwrap().0;
            let (_, sys) = esystem(&c, 3, &xr);
            let mac = macaulay_probe(p, &sys, 3, 16);
            let sd = mac.solving_degree.unwrap_or(0);
            sds.push(sd);
            c3.push(mac.steps.iter().find(|s| s.d == sd).map_or(0, |s| s.mults));
        }
        let c3_mean = c3.iter().sum::<u64>() as f64 / c3.len() as f64;
        // relation collection: r-adding walk, 1 group addition per trial
        let rw = 32;
        let table: Vec<(u128, u128, Pt)> = (0..rw)
            .map(|_| {
                let (a, b) = (rng.below128(ell), rng.below128(ell));
                (a, b, c.add(&c.mul(a, &g), &c.mul(b, &q)))
            })
            .collect();
        let (mut ra, mut rb) = (rng.below128(ell), 1 + rng.below128(ell - 1));
        let mut r = c.add(&c.mul(ra, &g), &c.mul(rb, &q));
        let mut rels: Vec<Rel> = vec![];
        let (mut trials, mut walk_adds, mut oracle_adds, mut empty_trials) = (0u64, 0u64, 0u64, 0u64);
        let mut per_trial = vec![];
        while rels.len() < ncol + extra && trials < 200 * ncol as u64 {
            trials += 1;
            if r.is_some() {
                let (dec, ad) = decompose3(&c, &fb, &r, &mut rng);
                oracle_adds += ad;
                per_trial.push(dec.len());
                if dec.is_empty() {
                    empty_trials += 1;
                }
                for d in dec {
                    let mut m: HashMap<usize, i64> = HashMap::new();
                    for (i, s) in d {
                        *m.entry(i).or_insert(0) += s as i64;
                    }
                    rels.push(Rel { cols: m.into_iter().filter(|x| x.1 != 0).collect(), b: rb, a: ra });
                }
            }
            let j = r.map_or(0, |(x, _)| (x[0] ^ x[1].rotate_left(7) ^ x[2].rotate_left(13)) as usize % rw);
            r = c.add(&r, &table[j].2);
            ra = (ra + table[j].0) % ell;
            rb = (rb + table[j].1) % ell;
            walk_adds += 1;
        }
        let used: HashSet<usize> = rels.iter().flat_map(|r| r.cols.iter().map(|x| x.0)).collect();
        let empty_cols = nfb - used.len();
        let nnz: usize = rels.iter().map(|r| r.cols.len() + 1).sum();
        let (rank, _aug, cons, k, la_mults) = solve_mod(&rels, ncol, ell, kcol, true, false, h);
        let k_ok = k.map(|kk| kk == k_secret && c.mul(kk, &g) == q);
        // control A: cofactor scaling (h) applied: must give the same k (gcd(h, ell) = 1)
        let (_, _, cons_h, k_h, _) = solve_mod(&rels, ncol, ell, kcol, true, true, h);
        // control B: signs dropped (sign bug): detector must flag inconsistency / no correct k
        let (_, _, cons_ns, k_ns, _) = solve_mod(&rels, ncol, ell, kcol, false, false, h);
        // control C: 10 duplicated rows must not raise the rank
        let mut dup: Vec<Rel> = rels.iter().map(|r| Rel { cols: r.cols.clone(), b: r.b, a: r.a }).collect();
        for i in 0..10.min(rels.len()) {
            dup.push(Rel { cols: rels[i].cols.clone(), b: rels[i].b, a: rels[i].a });
        }
        let (rank_dup, _, _, _, _) = solve_mod(&dup, ncol, ell, kcol, true, false, h);
        // rho reference: r-adding walk (r=20) with Brent cycle detection, measured
        let mut rho_steps = vec![];
        for _ in 0..rho_runs {
            let ks = 1 + rng.below128(ell - 1);
            let qq = c.mul(ks, &g);
            let tab: Vec<(u128, u128, Pt)> = (0..20)
                .map(|_| {
                    let (a, b) = (rng.below128(ell), rng.below128(ell));
                    (a, b, c.add(&c.mul(a, &g), &c.mul(b, &qq)))
                })
                .collect();
            let step = |pt: &Pt, a: u128, b: u128| -> (Pt, u128, u128) {
                let j = pt.map_or(0, |(x, _)| (x[0] ^ x[1].rotate_left(7) ^ x[2].rotate_left(13)) as usize % 20);
                (c.add(pt, &tab[j].2), (a + tab[j].0) % ell, (b + tab[j].1) % ell)
            };
            let mut steps = 0u64;
            loop {
                let (a0, b0) = (rng.below128(ell), rng.below128(ell));
                let x0 = c.add(&c.mul(a0, &g), &c.mul(b0, &qq));
                let (mut tort, mut ta, mut tb) = (x0, a0, b0);
                let (mut hare, mut ha, mut hb) = step(&x0, a0, b0);
                steps += 1;
                let (mut power, mut lam) = (1u64, 1u64);
                while tort != hare {
                    if power == lam {
                        tort = hare;
                        ta = ha;
                        tb = hb;
                        power *= 2;
                        lam = 0;
                    }
                    let s = step(&hare, ha, hb);
                    hare = s.0;
                    ha = s.1;
                    hb = s.2;
                    steps += 1;
                    lam += 1;
                }
                let db = (hb + ell - tb) % ell;
                if db != 0 {
                    let da = (ta + ell - ha) % ell;
                    let mut ivb = 1u128;
                    let (mut bb, mut e) = (db, ell - 2);
                    while e > 0 {
                        if e & 1 == 1 {
                            ivb = mulmod128(ivb, bb, ell);
                        }
                        bb = mulmod128(bb, bb, ell);
                        e >>= 1;
                    }
                    let kk = mulmod128(da, ivb, ell);
                    assert!(c.mul(kk, &g) == qq, "rho produced a wrong log");
                    break;
                }
            }
            rho_steps.push(steps);
        }
        let rho_mean = rho_steps.iter().sum::<u64>() as f64 / rho_steps.len() as f64;
        let rho_analytic = (std::f64::consts::PI * ell as f64 / 2.0).sqrt();
        let rho_neg = (std::f64::consts::PI * ell as f64 / 4.0).sqrt();
        // cost composition in group-addition equivalents (GAE); native ops priced at w
        let la_sparse = 3.0 * (ncol as f64) * nnz as f64; // Lanczos/Wiedemann model: ~ncol iterations x nnz
        let band: Vec<String> = [0.0f64, 0.1, 1.0]
            .iter()
            .map(|&w| {
                let ic = walk_adds as f64 + w * (trials as f64 * c3_mean + la_sparse.min(la_mults as f64));
                // rho unpriced: one comparison per step (Brent) priced at w
                let rho = rho_mean * (1.0 + w);
                let rho_n = rho_neg * (1.0 + w);
                format!(
                    "{{\"w\":{},\"ic_gae\":{:.1},\"rho_measured_gae\":{:.1},\"rho_negmap_analytic_gae\":{:.1},\"ic_over_rho_measured\":{:.4},\"ic_over_rho_negmap\":{:.4}}}",
                    w, ic, rho, rho_n, ic / rho, ic / rho_n
                )
            })
            .collect();
        let js = |v: &Vec<u64>| format!("[{}]", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
        let hist = {
            let mx = per_trial.iter().copied().max().unwrap_or(0);
            let mut hh = vec![0u64; mx + 1];
            for &x in &per_trial {
                hh[x] += 1;
            }
            hh
        };
        println!(
            "{{\"exp\":\"pipeline\",\"p\":{},\"curve\":{},\"N\":{},\"h\":{},\"ell\":{},\"fb_xclasses\":{},\"unknowns\":{},\"relations\":{},\"trials\":{},\"empty_trials\":{},\"walk_adds\":{},\"oracle_adds_unpriced_excluded\":{},\"per_trial_hist\":{},\"empty_cols\":{},\"rank\":{},\"consistent\":{},\"k_determined\":{},\"k_correct\":{},\"la_dense_mults\":{},\"la_sparse_model\":{:.0},\"nnz\":{},\"ctrl_cofactor_scaled\":{{\"consistent\":{},\"same_k\":{}}},\"ctrl_signs_dropped\":{{\"consistent\":{},\"k_correct\":{}}},\"ctrl_dup_rows\":{{\"rank\":{},\"unchanged\":{}}},\"c3_solving_degrees\":{},\"c3_mults\":{},\"c3_mean\":{:.1},\"rho_runs\":{},\"rho_steps\":{},\"rho_mean\":{:.1},\"rho_analytic_no_neg\":{:.1},\"rho_analytic_negmap\":{:.1},\"band\":[{}]}}",
            p, ci, nord, h, ell, nfb, ncol, rels.len(), trials, empty_trials, walk_adds, oracle_adds,
            js(&hist), empty_cols, rank, cons, k.is_some(), k_ok.map_or("null".into(), |b| b.to_string()),
            la_mults, la_sparse, nnz,
            cons_h, k_h == k,
            cons_ns, k_ns.map_or("null".into(), |kk| (kk == k_secret).to_string()),
            rank_dup, rank_dup == rank,
            js(&sds.iter().map(|&x| x as u64).collect()), js(&c3), c3_mean,
            rho_runs, js(&rho_steps), rho_mean, rho_analytic, rho_neg, band.join(",")
        );
    }
}
