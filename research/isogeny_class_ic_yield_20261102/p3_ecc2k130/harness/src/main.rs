//! `p3h`: harness for P3 of `research/isogeny_class_ic_yield_20261102`.
//!
//! Instrument: the Weil-descended (Semaev S4 resultant) decomposition system of
//! `R = P1 + P2 + P3` (m = 3) or `R = P1 + P2` (m = 2) over `F_{2^n}`, with the
//! summand abscissae restricted to a k-dimensional F_2-subspace V.  The system
//! has 3k (2k) Boolean unknowns and n equations; over F_{2^n} it depends on the
//! curve through `b` alone (`S3(a,c,d) = (a+c)^2 d^2 + acd + (ac)^2 + b`).
//! Works for any n (n = 43 for the analogue, n = 131 for ECC2K-130).
//!
//! Subcommands: `class`, `gt`, `selftest`, `measure`, `analyze`.  See PROTOCOL.md.

use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::koblitz_groebner::{
    first_fall_degree, solve_boolean_system, solving_degree_from, SolveOptions,
};
use crypto_lib::cryptanalysis::pq_groebner_f2::{F2BoolMono, F2BoolPoly, Mask};
use crypto_lib::cryptanalysis::volcano::{binary, f2poly, level1, profile};
use num_bigint::{BigInt, BigUint};
use num_traits::{Num, Zero};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;

// ───────────────────────── small utilities ─────────────────────────

fn splitmix(x: &mut u64) -> u64 {
    *x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
/// Seed derived from the protocol seed and a tuple of integers (never chosen after a look).
fn derive(seed: u64, parts: &[u64]) -> u64 {
    let mut s = seed ^ 0xA5A5_5A5A_1234_8765;
    for &p in parts {
        s ^= p;
        let _ = splitmix(&mut s);
        s = splitmix(&mut s.clone());
    }
    s
}
fn hex(e: &F2mElement) -> String {
    e.to_biguint().to_str_radix(16)
}
fn parse_hex(s: &str, n: u32) -> F2mElement {
    F2mElement::from_biguint(&BigUint::from_str_radix(s, 16).expect("hex"), n)
}
fn sha_hex(b: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(b);
    h.finalize().iter().map(|x| format!("{x:02x}")).collect()
}
fn rand_elem(n: u32, rng: &mut u64) -> F2mElement {
    let w: Vec<u64> = (0..n.div_ceil(64)).map(|_| splitmix(rng)).collect();
    F2mElement::from_words(&w, n)
}
fn bit_of(e: &F2mElement, i: usize) -> bool {
    (e.raw_bits()[i / 64] >> (i % 64)) & 1 == 1
}
/// k F_2-independent random vectors (a uniformly random k-subspace basis).
fn rand_subspace(n: u32, k: usize, rng: &mut u64) -> Vec<F2mElement> {
    loop {
        let v: Vec<F2mElement> = (0..k).map(|_| rand_elem(n, rng)).collect();
        if rank(&v) == k {
            return v;
        }
    }
}
fn std_subspace(n: u32, k: usize) -> Vec<F2mElement> {
    (0..k as u32).map(|i| F2mElement::from_bit_positions(&[i], n)).collect()
}
fn rank(v: &[F2mElement]) -> usize {
    let mut rows: Vec<Vec<u64>> = v.iter().map(|e| e.raw_bits().to_vec()).collect();
    let mut r = 0;
    let nbits = rows.first().map_or(0, |x| x.len() * 64);
    for col in 0..nbits {
        let Some(p) = (r..rows.len()).find(|&i| (rows[i][col / 64] >> (col % 64)) & 1 == 1) else {
            continue;
        };
        rows.swap(r, p);
        let pr = rows[r].clone();
        for i in 0..rows.len() {
            if i != r && (rows[i][col / 64] >> (col % 64)) & 1 == 1 {
                for (a, b) in rows[i].iter_mut().zip(&pr) {
                    *a ^= *b;
                }
            }
        }
        r += 1;
    }
    r
}

// ───────────────────── moduli, presentations ─────────────────────

fn poly_of(irr: &IrreduciblePoly) -> f2poly::F2Poly {
    let mut bits = vec![false; irr.degree as usize + 1];
    bits[irr.degree as usize] = true;
    for &t in &irr.low_terms {
        bits[t as usize] = true;
    }
    f2poly::F2Poly::from_bits(&bits)
}
fn modulus_string(irr: &IrreduciblePoly) -> String {
    let mut t: Vec<u32> = irr.low_terms.clone();
    t.sort_unstable_by(|a, b| b.cmp(a));
    let mut s = format!("z^{}", irr.degree);
    for x in t {
        s += &match x {
            0 => "+1".to_string(),
            1 => "+z".to_string(),
            _ => format!("+z^{x}"),
        };
    }
    s
}
/// The first `count` irreducible trinomials then pentanomials of degree n (lexicographic), excluding `skip`.
fn alt_moduli(n: u32, count: usize, skip: &IrreduciblePoly) -> Vec<IrreduciblePoly> {
    let nn = n as usize;
    let mut out: Vec<IrreduciblePoly> = Vec::new();
    let mut same = |lt: &[u32]| {
        let mut a = lt.to_vec();
        a.sort_unstable();
        let mut b = skip.low_terms.clone();
        b.sort_unstable();
        a == b
    };
    let mk = |lt: &[u32]| {
        let mut bits = vec![false; nn + 1];
        bits[nn] = true;
        for &t in lt {
            bits[t as usize] = true;
        }
        f2poly::F2Poly::from_bits(&bits)
    };
    for k in 1..nn as u32 {
        let lt = [0, k];
        if !same(&lt) && binary::is_irreducible(&mk(&lt), nn) {
            out.push(IrreduciblePoly { degree: n, low_terms: lt.to_vec() });
            if out.len() == count {
                return out;
            }
        }
    }
    for a in 3..nn as u32 {
        for b in 2..a {
            for c in 1..b {
                let lt = [0, c, b, a];
                if !same(&lt) && binary::is_irreducible(&mk(&lt), nn) {
                    out.push(IrreduciblePoly { degree: n, low_terms: lt.to_vec() });
                    if out.len() == count {
                        return out;
                    }
                }
            }
        }
    }
    out
}
/// Field isomorphism F_{M0} -> F_{M1}: z -> a root of M0 in F_{M1}; applied in the power basis.
struct Iso {
    imgs: Vec<F2mElement>,
    n: u32,
}
impl Iso {
    fn new(m0: &IrreduciblePoly, m1: &IrreduciblePoly) -> Iso {
        let n = m0.degree;
        let mut rng = 0xC0DE_u64;
        let r = binary::orbit_of(&poly_of(m0), n, m1, &mut rng)[0].clone();
        let mut imgs = vec![F2mElement::one(n)];
        for i in 1..n as usize {
            let nx = imgs[i - 1].mul(&r, m1);
            imgs.push(nx);
        }
        Iso { imgs, n }
    }
    fn apply(&self, e: &F2mElement) -> F2mElement {
        let mut acc = F2mElement::zero(self.n);
        for i in 0..self.n as usize {
            if bit_of(e, i) {
                acc = acc.add(&self.imgs[i]);
            }
        }
        acc
    }
}

// ───────────────── symbolic Boolean-ring algebra over F ─────────────────

/// Multilinear (Boolean) polynomial with F_{2^n} coefficients: monomial mask -> coefficient.
type Sym = HashMap<u32, F2mElement>;

fn s_add(a: &Sym, b: &Sym) -> Sym {
    let mut o = a.clone();
    for (m, c) in b {
        match o.get_mut(m) {
            Some(x) => {
                *x = x.add(c);
                if x.is_zero() {
                    o.remove(m);
                }
            }
            None => {
                o.insert(*m, c.clone());
            }
        }
    }
    o
}
fn s_mul(a: &Sym, b: &Sym, irr: &IrreduciblePoly) -> Sym {
    let mut o: Sym = HashMap::new();
    for (ma, ca) in a {
        for (mb, cb) in b {
            let p = ca.mul(cb, irr);
            let m = ma | mb;
            match o.get_mut(&m) {
                Some(x) => *x = x.add(&p),
                None => {
                    o.insert(m, p);
                }
            }
        }
    }
    o.retain(|_, c| !c.is_zero());
    o
}
/// Squaring is Frobenius on coefficients (m^2 = m in the Boolean ring, cross terms vanish).
fn s_sq(a: &Sym, irr: &IrreduciblePoly) -> Sym {
    a.iter().map(|(m, c)| (*m, c.square(irr))).collect()
}
fn s_const(e: &F2mElement) -> Sym {
    let mut o = Sym::new();
    if !e.is_zero() {
        o.insert(0, e.clone());
    }
    o
}
fn s_lin(basis: &[F2mElement], block: usize) -> Sym {
    let k = basis.len();
    basis.iter().enumerate().map(|(j, v)| (1u32 << (block * k + j), v.clone())).collect()
}
fn s_coords(s: &Sym, n: u32, nv: usize) -> Vec<F2BoolPoly> {
    let mut monos: Vec<Vec<F2BoolMono>> = vec![Vec::new(); n as usize];
    for (m, c) in s {
        for (j, mm) in monos.iter_mut().enumerate() {
            if bit_of(c, j) {
                mm.push(F2BoolMono::from_mask(*m as Mask));
            }
        }
    }
    monos.into_iter().filter(|v| !v.is_empty()).map(|v| F2BoolPoly::from_monos(v, nv)).collect()
}

/// The descended system for m = 2 or 3.  Returns (equations, n_vars).
fn build_system(m: usize, basis: &[F2mElement], xr: &F2mElement, b: &F2mElement, irr: &IrreduciblePoly) -> (Vec<F2BoolPoly>, usize) {
    let n = irr.degree;
    let k = basis.len();
    let bs = s_const(b);
    let xrs = s_const(xr);
    let xr2 = s_sq(&xrs, irr);
    let x1 = s_lin(basis, 0);
    let x2 = s_lin(basis, 1);
    let sq = |s: &Sym| s_sq(s, irr);
    if m == 2 {
        // S3(x1,x2,xR) = (x1^2+x2^2) xR^2 + x1 x2 xR + (x1 x2)^2 + b
        let bq = s_mul(&x1, &x2, irr);
        let t = s_mul(&s_add(&sq(&x1), &sq(&x2)), &xr2, irr);
        let t = s_add(&t, &s_mul(&bq, &xrs, irr));
        let t = s_add(&t, &sq(&bq));
        let t = s_add(&t, &bs);
        return (s_coords(&t, n, 2 * k), 2 * k);
    }
    assert_eq!(m, 3);
    let x3 = s_lin(basis, 2);
    // S3(x1,x2,t) = A t^2 + B t + C ; S3(t,x3,xR) = A' t^2 + B' t + C'
    let a = s_add(&sq(&x1), &sq(&x2));
    let bq = s_mul(&x1, &x2, irr);
    let c = s_add(&sq(&bq), &bs);
    let a2 = s_add(&sq(&x3), &xr2);
    let b2 = s_mul(&x3, &xrs, irr);
    let c2 = s_add(&sq(&b2), &bs);
    // Res = (A C' + A' C)^2 + (A B' + A' B)(B C' + B' C)   (char 2)
    let t1 = s_add(&s_mul(&a, &c2, irr), &s_mul(&a2, &c, irr));
    let t2 = s_add(&s_mul(&a, &b2, irr), &s_mul(&a2, &bq, irr));
    let t3 = s_add(&s_mul(&bq, &c2, irr), &s_mul(&b2, &c, irr));
    let res = s_add(&sq(&t1), &s_mul(&t2, &t3, irr));
    (s_coords(&res, n, 3 * k), 3 * k)
}
fn eval_poly(p: &F2BoolPoly, point: u64) -> bool {
    p.terms.iter().fold(false, |acc, t| {
        let m = t.mask as u64;
        acc ^ (m & !point == 0)
    })
}

// ───────────── numeric counts (yield) ─────────────

fn s3(a: &F2mElement, c: &F2mElement, d: &F2mElement, b: &F2mElement, irr: &IrreduciblePoly) -> F2mElement {
    let ac = a.mul(c, irr);
    a.add(c).square(irr).mul(&d.square(irr), irr).add(&ac.mul(d, irr)).add(&ac.square(irr)).add(b)
}
fn span_elems(basis: &[F2mElement], n: u32) -> Vec<F2mElement> {
    (0..1u64 << basis.len())
        .map(|c| {
            let mut e = F2mElement::zero(n);
            for (j, v) in basis.iter().enumerate() {
                if (c >> j) & 1 == 1 {
                    e = e.add(v);
                }
            }
            e
        })
        .collect()
}
/// m = 2 count: pairs (x1,x2) in V^2 with S3(x1,x2,xR)=0.  Brute force, 4^k evaluations.
fn count_m2(basis: &[F2mElement], xr: &F2mElement, b: &F2mElement, irr: &IrreduciblePoly) -> u64 {
    let sp = span_elems(basis, irr.degree);
    let mut c = 0;
    for x1 in &sp {
        for x2 in &sp {
            if s3(x1, x2, xr, b, irr).is_zero() {
                c += 1;
            }
        }
    }
    c
}
/// m = 3 count: triples (x1,x2,x3) in V^3 with a common root t of S3(x1,x2,t), S3(t,x3,xR) (i.e. S4 = 0), by
/// brute force over 8^k triples through the quadratic resultant; use only for 3k <= 21.
fn count_m3_numeric(basis: &[F2mElement], xr: &F2mElement, b: &F2mElement, irr: &IrreduciblePoly) -> u64 {
    let sp = span_elems(basis, irr.degree);
    let xr2 = xr.square(irr);
    let mut cnt = 0;
    // precompute per (x1,x2): A,B,C ; per x3: A',B',C'
    let side2: Vec<(F2mElement, F2mElement, F2mElement)> = sp
        .iter()
        .map(|x3| {
            let b2 = x3.mul(xr, irr);
            (x3.square(irr).add(&xr2), b2.clone(), b2.square(irr).add(b))
        })
        .collect();
    for x1 in &sp {
        for x2 in &sp {
            let a = x1.square(irr).add(&x2.square(irr));
            let bq = x1.mul(x2, irr);
            let c = bq.square(irr).add(b);
            for (a2, b2, c2) in &side2 {
                let t1 = a.mul(c2, irr).add(&a2.mul(&c, irr));
                let t2 = a.mul(b2, irr).add(&a2.mul(&bq, irr));
                let t3 = bq.mul(c2, irr).add(&b2.mul(&c, irr));
                if t1.square(irr).add(&t2.mul(&t3, irr)).is_zero() {
                    cnt += 1;
                }
            }
        }
    }
    cnt
}
/// Fast m = 3 count for n < 64 by Artin-Schreier lookups (library routine), ordered (x1,x2,x3,e).
fn count_m3_fast(basis: &[F2mElement], xr: &F2mElement, b: &F2mElement, irr: &IrreduciblePoly) -> u64 {
    crypto_lib::cryptanalysis::koblitz_bench::chained_s3_solution_count_fast(basis, xr, b, irr)
}

// ───────────── planted targets ─────────────

fn lift_x(x: &F2mElement, a2: bool, b: &F2mElement, irr: &IrreduciblePoly) -> Option<F2mElement> {
    if x.is_zero() {
        return None;
    }
    let n = irr.degree;
    let xi2 = x.square(irr).flt_inverse(irr)?;
    let mut c = x.add(&b.mul(&xi2, irr));
    if a2 {
        c = c.add(&F2mElement::one(n));
    }
    if c.trace(irr) != 0 {
        return None;
    }
    let w = c.half_trace(irr)?;
    Some(x.mul(&w, irr))
}
/// Planted instance: three liftable x in V (random combinations), xR = x(P1+P2+P3).  Returns (xR, point mask).
fn plant(basis: &[F2mElement], a2: bool, b: &F2mElement, irr: &IrreduciblePoly, rng: &mut u64) -> Option<(F2mElement, u64)> {
    use crypto_lib::binary_ecc::BinaryPoint;
    let n = irr.degree;
    let k = basis.len();
    let j = b.flt_inverse(irr)?;
    let curve = binary::curve_for_j(&j, a2, n, irr);
    for _ in 0..200 {
        let mut pts = Vec::new();
        let mut mask = 0u64;
        for blk in 0..3 {
            let mut tries = 0;
            loop {
                tries += 1;
                if tries > 400 {
                    return None;
                }
                let cm = splitmix(rng) & ((1u64 << k) - 1);
                let mut x = F2mElement::zero(n);
                for (jj, v) in basis.iter().enumerate() {
                    if (cm >> jj) & 1 == 1 {
                        x = x.add(v);
                    }
                }
                if let Some(y) = lift_x(&x, a2, b, irr) {
                    pts.push(BinaryPoint::Affine { x, y });
                    mask |= cm << (blk * k);
                    break;
                }
            }
        }
        let s01 = binary::add_points(&curve, &pts[0], &pts[1]);
        let s = binary::add_points(&curve, &s01, &pts[2]);
        if let BinaryPoint::Affine { x, .. } = s {
            return Some((x, mask));
        }
    }
    None
}

// ───────────── per-instance measurement ─────────────

struct Meas {
    solutions: i64, // exact count of x-triples (or pairs), -1 if not computed
    outcome: String, // resolved / atleast / capshit / sat / skipped
    dreg: i64,
    refuted: bool,
    ffd: i64,
    rows: usize,
    cols: usize,
    n_eqs: usize,
    n_vars: usize,
    build_s: f64,
    dreg_s: f64,
    solve_s: f64,
    solve_nsol: i64,
    solve_exhausted: bool,
    planted_ok: i64, // -1 n/a, 1 planted point satisfies all equations, 0 not
    planted_found: i64,
}
struct Opts {
    dmax: u32,
    ffd_max: u32,
    do_solve: bool,
    do_count: u8, // 0 none, 1 numeric/fast as available
}
fn measure_instance(m: usize, basis: &[F2mElement], xr: &F2mElement, b: &F2mElement, irr: &IrreduciblePoly, planted: Option<u64>, o: &Opts) -> Meas {
    let n = irr.degree;
    let t0 = Instant::now();
    let (eqs, nv) = build_system(m, basis, xr, b, irr);
    let build_s = t0.elapsed().as_secs_f64();
    let k = basis.len();
    let solutions = if o.do_count == 0 {
        -1
    } else if m == 2 && 2 * k <= 26 {
        count_m2(basis, xr, b, irr) as i64
    } else if m == 3 && n < 64 && k <= 15 {
        count_m3_fast(basis, xr, b, irr) as i64
    } else if m == 3 && 3 * k <= 21 {
        count_m3_numeric(basis, xr, b, irr) as i64
    } else {
        -1
    };
    let mut me = Meas {
        solutions,
        outcome: "skipped".into(),
        dreg: -1,
        refuted: false,
        ffd: -1,
        rows: 0,
        cols: 0,
        n_eqs: eqs.len(),
        n_vars: nv,
        build_s,
        dreg_s: 0.0,
        solve_s: 0.0,
        solve_nsol: -1,
        solve_exhausted: false,
        planted_ok: -1,
        planted_found: -1,
    };
    if let Some(pm) = planted {
        me.planted_ok = eqs.iter().all(|p| !eval_poly(p, pm)) as i64;
    }
    // resolving degree + first fall proxy (the refutation degree for unsatisfiable systems)
    let t1 = Instant::now();
    let (d, profs) = solving_degree_from(&eqs, nv, 1, o.dmax);
    me.dreg_s = t1.elapsed().as_secs_f64();
    if let Some(p) = profs.last() {
        me.rows = p.rows;
        me.cols = p.cols;
        me.refuted = p.refuted;
    }
    match d {
        Some(dd) => {
            me.dreg = dd as i64;
            me.outcome = "resolved".into();
        }
        None => {
            let built = profs.last().map(|p| p.degree);
            me.outcome = if built == Some(o.dmax) { "atleast".into() } else { "capshit".into() };
            me.dreg = o.dmax as i64 + 1;
        }
    }
    if o.ffd_max >= 2 {
        let (f, _) = first_fall_degree(&eqs, nv, o.ffd_max);
        me.ffd = f.map_or(-1, |x| x as i64);
    }
    if o.do_solve {
        let t2 = Instant::now();
        let (sols, st) = solve_boolean_system(&eqs, nv, &SolveOptions::default());
        me.solve_s = t2.elapsed().as_secs_f64();
        me.solve_nsol = sols.len() as i64;
        me.solve_exhausted = st.exhausted;
        if let Some(pm) = planted {
            me.planted_found = sols.iter().any(|s| (*s as u64) == pm) as i64;
        }
    }
    me
}

// ───────────── curve tables ─────────────

#[derive(Clone)]
struct CurveRow {
    idx: usize,
    label: String,
    orbit: String,
    frob: usize,
    a2: bool,
    b_hex: String,
}
fn read_curves(path: &str) -> Vec<CurveRow> {
    let txt = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    txt.lines()
        .filter(|l| !l.starts_with('#') && !l.starts_with("idx"))
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            CurveRow {
                idx: f[0].parse().unwrap(),
                label: f[1].into(),
                orbit: f[2].into(),
                frob: f[3].parse().unwrap(),
                a2: f[4] == "1",
                b_hex: f[5].into(),
            }
        })
        .collect()
}

// ───────────── arg helpers ─────────────

struct Args(Vec<String>);
impl Args {
    fn get(&self, k: &str) -> Option<String> {
        self.0.iter().position(|a| a == k).and_then(|i| self.0.get(i + 1)).cloned()
    }
    fn req(&self, k: &str) -> String {
        self.get(k).unwrap_or_else(|| panic!("missing {k}"))
    }
    fn num<T: std::str::FromStr>(&self, k: &str, d: T) -> T {
        self.get(k).and_then(|v| v.parse().ok()).unwrap_or(d)
    }
    fn flag(&self, k: &str) -> bool {
        self.0.iter().any(|a| a == k)
    }
}

fn base_modulus(n: u32, which: &str) -> IrreduciblePoly {
    match which {
        "131" => IrreduciblePoly { degree: 131, low_terms: vec![0, 1, 2, 13] },
        _ => binary::find_irreducible(n),
    }
}

// ───────────── class: P3a curve construction ─────────────

fn cmd_class(a: &Args) {
    let n: u32 = a.req("--n").parse().unwrap();
    let ell: u64 = a.req("--ell").parse().unwrap();
    let a2k = a.num::<u8>("--a2", 0) == 1;
    let out = a.req("--out");
    let t_all = Instant::now();
    assert!(n % 2 == 1, "odd n only (half-trace)");
    let (t, u) = binary::koblitz_trace(n, a2k);
    let f = u.magnitude().clone();
    assert!((&f % BigUint::from(ell)).is_zero(), "ell does not divide the conductor");
    let kron = profile::kronecker(&BigInt::from(-7), &BigUint::from(ell));
    let h = profile::class_number_of_order(-7, 1, &BigUint::from(ell));
    let horiz = 1 + kron;
    eprintln!("# n={n} ell={ell} a2={} t={t} f={f} (-7/ell)={kron} h(O_ell)={h}", a2k as u8);
    let (coeffs, rep) = level1::phi_at_crater(-7, ell, a.num("--batch", 16usize), true).expect("phi");
    eprintln!("Phi_ell(X, j0): {} primes, {:.1}s", rep.primes_used, rep.seconds);
    let bits = level1::mod2(&coeffs);
    let phi2 = f2poly::F2Poly::from_bits(&bits);
    let rf = f2poly::rational_factors(&phi2, n as usize);
    let irr = binary::find_irreducible(n);
    let mut one_mult = 0u64;
    let mut facs = Vec::new();
    for (g, d, mu) in &rf.factors {
        if *d == 1 && g.coeff(0) && g.deg() == Some(1) {
            one_mult += *mu;
            continue;
        }
        facs.push((g.clone(), *d));
    }
    let big_n = binary::koblitz_order(n, a2k);
    let mut rows: Vec<(String, usize, F2mElement)> = Vec::new(); // (orbit, frob, j)
    for (oi, (g, _d)) in facs.iter().enumerate() {
        let mut rng = 0xC0FFEEu64 ^ (oi as u64).wrapping_mul(0x9E3779B97F4A7C15);
        for (fi, j) in binary::orbit_of(g, n, &irr, &mut rng).into_iter().enumerate() {
            rows.push((format!("O{oi}"), fi, j));
        }
    }
    let mut seen = std::collections::HashSet::new();
    let distinct = rows.iter().filter(|r| seen.insert(r.2.to_biguint())).count();
    // Verify every level-1 curve: exactly one twist has order N (all points killed by N), a2 chosen accordingly.
    let mut ok_twist = 0usize;
    let mut lines = vec!["idx\tlabel\torbit\tfrob\ta2\tb_hex\tj_hex".to_string()];
    let e0b = F2mElement::one(n);
    lines.push(format!("0\tE0\tcrater\t0\t{}\t{}\t{}", a2k as u8, hex(&e0b), hex(&e0b)));
    for (i, (orb, fi, j)) in rows.iter().enumerate() {
        let mut rng = 0xBADC0DEu64 ^ (i as u64).wrapping_mul(0x2545F4914F6CDD1D);
        let c0 = binary::curve_for_j(j, false, n, &irr);
        let c1 = binary::curve_for_j(j, true, n, &irr);
        let (p0, p1) = (binary::order_test(&c0, &big_n, 6, &mut rng), binary::order_test(&c1, &big_n, 6, &mut rng));
        if p0 ^ p1 {
            ok_twist += 1;
        }
        let a2 = p1 && !p0;
        let b = j.flt_inverse(&irr).unwrap();
        lines.push(format!("{}\tL{:03}\t{}\t{}\t{}\t{}\t{}", i + 1, i, orb, fi, a2 as u8, hex(&b), hex(j)));
    }
    let mut rng = 0xFEEDu64;
    let mut false_pos = 0;
    for _ in 0..8 {
        let jr = rand_elem(n, &mut rng);
        if jr.is_zero() {
            continue;
        }
        let c0 = binary::curve_for_j(&jr, false, n, &irr);
        let c1 = binary::curve_for_j(&jr, true, n, &irr);
        if binary::order_test(&c0, &big_n, 3, &mut rng) || binary::order_test(&c1, &big_n, 3, &mut rng) {
            false_pos += 1;
        }
    }
    let body = lines.join("\n") + "\n";
    std::fs::write(&out, &body).unwrap();
    let sha = sha_hex(body.as_bytes());
    let ok = one_mult == horiz as u64 && BigUint::from(distinct) == h && distinct == rows.len() && ok_twist == rows.len() && false_pos == 0 && rf.leftover_deg == 0;
    let meta = format!(
        "{{\"n\":{n},\"ell\":{ell},\"a2_koblitz\":{},\"modulus\":\"{}\",\"trace\":\"{t}\",\"conductor\":\"{f}\",\"kronecker\":{kron},\"h_Oell\":\"{h}\",\"level1_curves\":{},\"distinct_j\":{distinct},\"crater_multiplicity\":{one_mult},\"expected_horizontal\":{horiz},\"orbits\":{},\"verified_one_twist_order_N\":{ok_twist},\"random_j_false_positives\":{false_pos},\"all_checks_pass\":{ok},\"curves_tsv_sha256\":\"{sha}\",\"seconds\":{:.1}}}\n",
        a2k as u8,
        modulus_string(&irr),
        rows.len(),
        facs.len(),
        t_all.elapsed().as_secs_f64()
    );
    std::fs::write(format!("{out}.meta.json"), &meta).unwrap();
    print!("{meta}");
    if !ok {
        std::process::exit(1);
    }
}

// ───────────── gt: ECC2K-130 ground truth b-list ─────────────

fn cmd_gt(a: &Args) {
    let path = a.req("--json");
    let out = a.req("--out");
    let raw = std::fs::read(&path).unwrap();
    let json_sha = sha_hex(&raw);
    let v: serde_json::Value = serde_json::from_slice(&raw).unwrap();
    let irr = base_modulus(131, "131");
    assert_eq!(v["meta"]["modulus_int"].as_str().unwrap(), "2722258935367507707706996859454145699847");
    let mut lines = vec!["idx\tlabel\torbit\tfrob\ta2\tb_hex\tj_hex".to_string()];
    for (i, c) in v["curves"].as_array().unwrap().iter().enumerate() {
        let b = F2mElement::from_biguint(&BigUint::from_str_radix(c["b_int"].as_str().unwrap(), 10).unwrap(), 131);
        let j = F2mElement::from_biguint(&BigUint::from_str_radix(c["j_int"].as_str().unwrap(), 10).unwrap(), 131);
        assert_eq!(b.mul(&j, &irr), F2mElement::one(131), "b*j != 1 at {i}");
        lines.push(format!(
            "{i}\t{}\t{}\t{}\t{}\t{}\t{}",
            c["label"].as_str().unwrap(),
            c["orbit"].as_str().unwrap(),
            c["frob_index"],
            c["a2"],
            hex(&b),
            hex(&j)
        ));
    }
    let body = lines.join("\n") + "\n";
    std::fs::write(&out, &body).unwrap();
    println!("{{\"source\":\"{path}\",\"source_sha256\":\"{json_sha}\",\"curves\":{},\"tsv_sha256\":\"{}\",\"modulus\":\"{}\"}}", lines.len() - 1, sha_hex(body.as_bytes()), modulus_string(&irr));
}

// ───────────── selftest ─────────────

fn cmd_selftest(_a: &Args) {
    let mut ok = true;
    // 1. descended system vs numeric S4 counts, planted point satisfies equations, and Frobenius invariance of the system
    for &(n, k) in &[(13u32, 3usize), (17, 4), (21, 4)] {
        let irr = binary::find_irreducible(n);
        let mut rng = derive(7, &[n as u64, k as u64]);
        for trial in 0..4 {
            let basis = rand_subspace(n, k, &mut rng);
            let b = rand_elem(n, &mut rng);
            let (xr, pm) = match plant(&basis, false, &b, &irr, &mut rng) {
                Some(x) => x,
                None => continue,
            };
            let (eqs, nv) = build_system(3, &basis, &xr, &b, &irr);
            let sat = eqs.iter().all(|p| !eval_poly(p, pm));
            // brute-force evaluation of the system over all 2^{3k} points = numeric count
            let mut brute = 0u64;
            for pt in 0..(1u64 << nv) {
                if eqs.iter().all(|p| !eval_poly(p, pt)) {
                    brute += 1;
                }
            }
            let num = count_m3_numeric(&basis, &xr, &b, &irr);
            let fast = count_m3_fast(&basis, &xr, &b, &irr);
            let good = sat && brute == num && brute >= 1 && fast >= brute;
            println!("selftest n={n} k={k} trial={trial} planted_satisfies={sat} brute={brute} numeric={num} chained_fast={fast} -> {}", if good { "ok" } else { "FAIL" });
            ok &= good;
            // Frobenius conjugate instance has identical count
            let (b2, xr2) = (b.square(&irr), xr.square(&irr));
            let basis2: Vec<F2mElement> = basis.iter().map(|v| v.square(&irr)).collect();
            let c2 = count_m3_numeric(&basis2, &xr2, &b2, &irr);
            ok &= c2 == num;
            println!("   frobenius-conjugate count {c2} == {num}: {}", c2 == num);
        }
    }
    // 2. m=2 system vs numeric count
    {
        let (n, k) = (13u32, 5usize);
        let irr = binary::find_irreducible(n);
        let mut rng = derive(9, &[1]);
        for _ in 0..4 {
            let basis = rand_subspace(n, k, &mut rng);
            let b = rand_elem(n, &mut rng);
            let xr = rand_elem(n, &mut rng);
            let (eqs, nv) = build_system(2, &basis, &xr, &b, &irr);
            let mut brute = 0u64;
            for pt in 0..(1u64 << nv) {
                if eqs.iter().all(|p| !eval_poly(p, pt)) {
                    brute += 1;
                }
            }
            let c = count_m2(&basis, &xr, &b, &irr);
            println!("selftest m=2 n={n} k={k} brute={brute} numeric={c} -> {}", if brute == c { "ok" } else { "FAIL" });
            ok &= brute == c;
        }
    }
    // 3. isomorphism between presentations preserves field arithmetic
    {
        let n = 43u32;
        let m0 = binary::find_irreducible(n);
        let alts = alt_moduli(n, 2, &m0);
        let mut rng = derive(11, &[1]);
        for m1 in &alts {
            let iso = Iso::new(&m0, m1);
            let (x, y) = (rand_elem(n, &mut rng), rand_elem(n, &mut rng));
            let l = iso.apply(&x.mul(&y, &m0));
            let r = iso.apply(&x).mul(&iso.apply(&y), m1);
            let good = l == r && iso.apply(&x.add(&y)) == iso.apply(&x).add(&iso.apply(&y));
            println!("selftest iso {} -> {} : {}", modulus_string(&m0), modulus_string(m1), if good { "ok" } else { "FAIL" });
            ok &= good;
        }
    }
    println!("selftest {}", if ok { "ALL OK" } else { "FAILED" });
    if !ok {
        std::process::exit(1);
    }
}

// ───────────── measure ─────────────

fn jline(
    arm: &str,
    c: &CurveRow,
    n: u32,
    m: usize,
    k: usize,
    att: usize,
    vkind: &str,
    pres: usize,
    seq: usize,
    me: &Meas,
    vwords: &str,
    xr: &str,
) -> String {
    format!(
        "{{\"arm\":\"{arm}\",\"curve\":{},\"label\":\"{}\",\"orbit\":\"{}\",\"frob\":{},\"n\":{n},\"m\":{m},\"k\":{k},\"attempt\":{att},\"v\":\"{vkind}\",\"presentation\":{pres},\"seq\":{seq},\"n_vars\":{},\"n_eqs\":{},\"solutions\":{},\"outcome\":\"{}\",\"dreg\":{},\"refuted\":{},\"ffd\":{},\"rows\":{},\"cols\":{},\"build_s\":{:.4},\"dreg_s\":{:.4},\"solve_s\":{:.4},\"solve_nsol\":{},\"solve_exhausted\":{},\"planted_ok\":{},\"planted_found\":{},\"v_basis\":\"{vwords}\",\"x_r\":\"{xr}\"}}",
        c.idx, c.label, c.orbit, c.frob, me.n_vars, me.n_eqs, me.solutions, me.outcome, me.dreg, me.refuted, me.ffd, me.rows, me.cols, me.build_s, me.dreg_s, me.solve_s, me.solve_nsol, me.solve_exhausted, me.planted_ok, me.planted_found
    )
}

/// One job = one (arm, curve, attempt).  Jobs of an attempt are shuffled with a seeded RNG (interleaving).
fn cmd_measure(a: &Args) {
    let n: u32 = a.req("--n").parse().unwrap();
    let curves = read_curves(&a.req("--curves"));
    let m: usize = a.num("--m", 3);
    let ks: Vec<usize> = a.req("--ks").split(',').map(|x| x.parse().unwrap()).collect();
    let attempts: usize = a.num("--attempts", 4);
    let first_attempt: usize = a.num("--first-attempt", 0);
    let seed: u64 = a.num("--seed", 20261102);
    let which = a.get("--modulus").unwrap_or_else(|| if n == 131 { "131".into() } else { "vol".into() });
    let m0 = base_modulus(n, &which);
    let arms: Vec<String> = a.get("--arms").unwrap_or("main".into()).split(',').map(String::from).collect();
    let n_null: usize = a.num("--null", 8);
    let pres_n: usize = a.num("--presentations", 3);
    let sel: Option<Vec<usize>> = a.get("--select").map(|s| s.split(',').map(|x| x.parse().unwrap()).collect());
    let opts = Opts { dmax: a.num("--dmax", 8), ffd_max: a.num("--ffd-max", 0), do_solve: a.flag("--solve"), do_count: a.num("--count", 1) };
    let out = a.req("--out");
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&out).unwrap();
    let alts = alt_moduli(n, pres_n, &m0);
    let isos: Vec<Iso> = alts.iter().map(|mm| Iso::new(&m0, mm)).collect();
    let rows: Vec<&CurveRow> = curves.iter().filter(|c| sel.as_ref().is_none_or(|s| s.contains(&c.idx))).collect();
    let nullc: Vec<CurveRow> = (0..n_null)
        .map(|i| {
            let mut r = derive(seed, &[0x4E55_4C4C, n as u64, i as u64]);
            let mut b = rand_elem(n, &mut r);
            while b.is_zero() {
                b = rand_elem(n, &mut r);
            }
            CurveRow { idx: 100000 + i, label: format!("N{i:02}"), orbit: "null".into(), frob: 0, a2: false, b_hex: hex(&b) }
        })
        .collect();
    let mut seq = 0usize;
    for &k in &ks {
        for att in first_attempt..first_attempt + attempts {
            // draws shared by every curve of this attempt (paired design)
            let mut rng = derive(seed, &[n as u64, m as u64, k as u64, att as u64]);
            let v_rand = rand_subspace(n, k, &mut rng);
            let xr_rand = rand_elem(n, &mut rng);
            let plant_seed = derive(seed, &[0x504C_4E54, n as u64, k as u64, att as u64]);
            // job list: (arm, row, pres)
            let mut jobs: Vec<(String, CurveRow, usize)> = Vec::new();
            for r in &rows {
                for arm in &arms {
                    match arm.as_str() {
                        "main" | "iso" | "aa" | "planted" => jobs.push((arm.clone(), (*r).clone(), 0)),
                        "stdv" => {
                            for p in 0..=isos.len() {
                                jobs.push((arm.clone(), (*r).clone(), p));
                            }
                        }
                        _ => {}
                    }
                }
            }
            if arms.iter().any(|x| x == "null") {
                for r in &nullc {
                    jobs.push(("null".into(), r.clone(), 0));
                }
            }
            // seeded Fisher-Yates shuffle (interleaved arm order)
            let mut sr = derive(seed, &[0x5348_5546, n as u64, k as u64, att as u64]);
            for i in (1..jobs.len()).rev() {
                let j = (splitmix(&mut sr) % (i as u64 + 1)) as usize;
                jobs.swap(i, j);
            }
            for (arm, c, pres) in jobs {
                let b = parse_hex(&c.b_hex, n);
                let (name, me, vw, xrs) = match arm.as_str() {
                    "main" | "null" | "aa" => {
                        let me = measure_instance(m, &v_rand, &xr_rand, &b, &m0, None, &opts);
                        (if arm == "null" { "null" } else { arm.as_str() }, me, words(&v_rand), hex(&xr_rand))
                    }
                    "iso" => {
                        let b2 = b.square(&m0);
                        let v2: Vec<F2mElement> = v_rand.iter().map(|x| x.square(&m0)).collect();
                        let x2 = xr_rand.square(&m0);
                        let me = measure_instance(m, &v2, &x2, &b2, &m0, None, &opts);
                        ("iso", me, words(&v2), hex(&x2))
                    }
                    "planted" => {
                        let mut pr = derive(plant_seed, &[c.idx as u64]);
                        match plant(&v_rand, c.a2, &b, &m0, &mut pr) {
                            Some((xr, pm)) if m == 3 => {
                                let me = measure_instance(m, &v_rand, &xr, &b, &m0, Some(pm), &opts);
                                ("planted", me, words(&v_rand), hex(&xr))
                            }
                            _ => continue,
                        }
                    }
                    "stdv" => {
                        // same curve, same abstract problem class, presentation p: V = span(1..z^{k-1}) in modulus p
                        let (mm, bb, xr) = if pres == 0 {
                            (m0.clone(), b.clone(), xr_rand.clone())
                        } else {
                            (alts[pres - 1].clone(), isos[pres - 1].apply(&b), isos[pres - 1].apply(&xr_rand))
                        };
                        let basis = std_subspace(n, k);
                        let me = measure_instance(m, &basis, &xr, &bb, &mm, None, &opts);
                        ("stdv", me, words(&basis), hex(&xr))
                    }
                    _ => unreachable!(),
                };
                let line = jline(name, &c, n, m, k, att, if name == "stdv" { "std" } else { "rand" }, pres, seq, &me, &vw, &xrs);
                seq += 1;
                writeln!(f, "{line}").unwrap();
                f.flush().unwrap();
            }
        }
    }
}
fn words(v: &[F2mElement]) -> String {
    v.iter().map(hex).collect::<Vec<_>>().join(",")
}

// ───────────── analyze ─────────────

fn jget<'a>(v: &'a serde_json::Value, k: &str) -> &'a serde_json::Value {
    &v[k]
}
fn mean_sd(x: &[f64]) -> (f64, f64) {
    if x.is_empty() {
        return (f64::NAN, f64::NAN);
    }
    let m = x.iter().sum::<f64>() / x.len() as f64;
    let v = if x.len() > 1 { x.iter().map(|y| (y - m) * (y - m)).sum::<f64>() / (x.len() as f64 - 1.0) } else { 0.0 };
    (m, v.sqrt())
}
fn median(x: &mut Vec<f64>) -> f64 {
    if x.is_empty() {
        return f64::NAN;
    }
    x.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let l = x.len();
    if l % 2 == 1 { x[l / 2] } else { (x[l / 2 - 1] + x[l / 2]) / 2.0 }
}

/// Analysis (masked unless --unmask): per (k) cell the Dreg and time distribution by arm; checks invariances.
fn cmd_analyze(a: &Args) {
    let path = a.req("--rows");
    let unmask = a.flag("--unmask");
    let txt = std::fs::read_to_string(&path).unwrap();
    let rows: Vec<serde_json::Value> = txt.lines().filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).unwrap()).collect();
    println!("rows: {}  (sha256 {})", rows.len(), sha_hex(txt.as_bytes()));
    // arm codes: blind by default
    let mut by: std::collections::BTreeMap<(i64, String), Vec<&serde_json::Value>> = Default::default();
    for r in &rows {
        by.entry((jget(r, "k").as_i64().unwrap(), jget(r, "arm").as_str().unwrap().to_string())).or_default().push(r);
    }
    let code = |arm: &str, i: usize| if unmask { arm.to_string() } else { format!("arm{}", (sha_hex(arm.as_bytes()).as_bytes()[i % 8] % 26 + b'A') as char) };
    println!("\n== per (k, arm): attempts x curves, Dreg (mean, sd, distribution), solutions mean, time medians ==");
    for ((k, arm), v) in &by {
        let dreg: Vec<f64> = v.iter().map(|r| r["dreg"].as_f64().unwrap()).collect();
        let sol: Vec<f64> = v.iter().map(|r| r["solutions"].as_f64().unwrap()).filter(|x| *x >= 0.0).collect();
        let mut tm: Vec<f64> = v.iter().map(|r| r["build_s"].as_f64().unwrap() + r["dreg_s"].as_f64().unwrap()).collect();
        let (md, sd) = mean_sd(&dreg);
        let mut hist: std::collections::BTreeMap<i64, usize> = Default::default();
        for r in v {
            *hist.entry(r["dreg"].as_i64().unwrap()).or_default() += 1;
        }
        println!("k={k} {:>8}  n={:<6} dreg mean {:.3} sd {:.3} hist {:?} sol_mean {:.4} t_med {:.4}s", code(arm, 0), v.len(), md, sd, hist, mean_sd(&sol).0, median(&mut tm));
    }
    // Instrument invariance: iso arm must equal main arm exactly, instance by instance.
    let mut key_main: HashMap<(i64, i64, i64), (i64, i64, i64)> = HashMap::new(); // (k,att,curve) -> (dreg, solutions, outcome-hash)
    for r in &rows {
        if r["arm"] == "main" {
            key_main.insert((r["k"].as_i64().unwrap(), r["attempt"].as_i64().unwrap(), r["curve"].as_i64().unwrap()), (r["dreg"].as_i64().unwrap(), r["solutions"].as_i64().unwrap(), r["refuted"].as_bool().unwrap() as i64));
        }
    }
    let (mut tot, mut same) = (0, 0);
    for r in &rows {
        if r["arm"] == "iso" {
            if let Some(m) = key_main.get(&(r["k"].as_i64().unwrap(), r["attempt"].as_i64().unwrap(), r["curve"].as_i64().unwrap())) {
                tot += 1;
                same += (*m == (r["dreg"].as_i64().unwrap(), r["solutions"].as_i64().unwrap(), r["refuted"].as_bool().unwrap() as i64)) as i32;
            }
        }
    }
    println!("\nisomorphic-model control (Frobenius conjugate instance): {same}/{tot} identical (dreg, solutions, refuted)");
    let (mut tot, mut same) = (0, 0);
    let mut aa_ratio: Vec<f64> = Vec::new();
    let mut tmain: HashMap<(i64, i64, i64), f64> = HashMap::new();
    for r in &rows {
        if r["arm"] == "main" {
            tmain.insert((r["k"].as_i64().unwrap(), r["attempt"].as_i64().unwrap(), r["curve"].as_i64().unwrap()), r["build_s"].as_f64().unwrap() + r["dreg_s"].as_f64().unwrap());
        }
    }
    for r in &rows {
        if r["arm"] == "aa" {
            let key = (r["k"].as_i64().unwrap(), r["attempt"].as_i64().unwrap(), r["curve"].as_i64().unwrap());
            if let Some(m) = key_main.get(&key) {
                tot += 1;
                same += (*m == (r["dreg"].as_i64().unwrap(), r["solutions"].as_i64().unwrap(), r["refuted"].as_bool().unwrap() as i64)) as i32;
                let t = r["build_s"].as_f64().unwrap() + r["dreg_s"].as_f64().unwrap();
                if let Some(t0) = tmain.get(&key) {
                    if *t0 > 0.0 {
                        aa_ratio.push(t / t0);
                    }
                }
            }
        }
    }
    let mut ar = aa_ratio.clone();
    println!("A/A repeat: {same}/{tot} identical counts; time ratio repeat/first median {:.3}, sd {:.3} (noise floor for the time column)", median(&mut ar), mean_sd(&aa_ratio).1);
    // Frobenius orbit invariance on the main arm: within an orbit, same (k,att) dreg must agree.
    let mut orb: HashMap<(i64, i64, String), std::collections::BTreeSet<i64>> = HashMap::new();
    for r in &rows {
        if r["arm"] == "main" {
            orb.entry((r["k"].as_i64().unwrap(), r["attempt"].as_i64().unwrap(), r["orbit"].as_str().unwrap().to_string())).or_default().insert(r["dreg"].as_i64().unwrap());
        }
    }
    println!("within-orbit Dreg spread (same V, x_R; curves of one orbit are Galois conjugates but V is NOT conjugated, so this is a between-presentation spread):");
    let mut ho: std::collections::BTreeMap<usize, usize> = Default::default();
    for (_, s) in &orb {
        *ho.entry(s.len()).or_default() += 1;
    }
    println!("  #distinct dreg values per (k, attempt, orbit): {ho:?}");
    // Between-orbit variance decomposition on the main arm (one-way): variance of per-curve dreg within vs between orbits.
    let mut gm: std::collections::BTreeMap<(i64, String), Vec<f64>> = Default::default();
    for r in &rows {
        if r["arm"] == "main" && unmask {
            gm.entry((r["k"].as_i64().unwrap(), r["orbit"].as_str().unwrap().to_string())).or_default().push(r["dreg"].as_f64().unwrap());
        }
    }
    if unmask {
        println!("\n== main arm mean dreg by (k, orbit) ==");
        for ((k, o), v) in &gm {
            println!("k={k} orbit={o} n={} mean={:.3} sd={:.3}", v.len(), mean_sd(v).0, mean_sd(v).1);
        }
    }
    println!("\nDecision rule, null set and thresholds: PROTOCOL.md section 6.  Masked output hides arm names; rerun with --unmask only after the analysis is frozen.");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: p3h <class|gt|selftest|measure|analyze> ...");
        std::process::exit(2);
    }
    let a = Args(args[1..].to_vec());
    match args[0].as_str() {
        "class" => cmd_class(&a),
        "gt" => cmd_gt(&a),
        "selftest" => cmd_selftest(&a),
        "measure" => cmd_measure(&a),
        "analyze" => cmd_analyze(&a),
        c => {
            eprintln!("unknown subcommand {c}");
            std::process::exit(2);
        }
    }
}
