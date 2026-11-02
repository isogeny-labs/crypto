//! Multivariate polynomials over Fq, Semaev S_3 / S_4, symmetrisation, Weil restriction,
//! and a Macaulay-matrix Groebner probe over F_p (solving degree, first fall degree).
use crate::curve::Curve;
use crate::field::*;
use std::collections::{BTreeMap, HashMap};

pub type Mono = [u8; 4];
pub type Poly = BTreeMap<Mono, E>;

pub fn konst(f: &Fq, c: E) -> Poly {
    let mut p = Poly::new();
    if !f.is_zero(&c) {
        p.insert([0; 4], c);
    }
    p
}
pub fn var(f: &Fq, i: usize) -> Poly {
    let mut m = [0u8; 4];
    m[i] = 1;
    let mut p = Poly::new();
    p.insert(m, f.one());
    p
}
pub fn padd(f: &Fq, a: &Poly, b: &Poly) -> Poly {
    let mut r = a.clone();
    for (m, c) in b {
        let e = r.entry(*m).or_insert(f.zero());
        *e = f.add(e, c);
        if f.is_zero(e) {
            r.remove(m);
        }
    }
    r
}
pub fn pscale(f: &Fq, a: &Poly, c: &E) -> Poly {
    if f.is_zero(c) {
        return Poly::new();
    }
    a.iter().map(|(m, x)| (*m, f.mul(x, c))).collect()
}
pub fn psub(f: &Fq, a: &Poly, b: &Poly) -> Poly {
    padd(f, a, &pscale(f, b, &f.ci(-1)))
}
pub fn pmulp(f: &Fq, a: &Poly, b: &Poly) -> Poly {
    let mut r: HashMap<Mono, E> = HashMap::new();
    for (ma, ca) in a {
        for (mb, cb) in b {
            let mut m = [0u8; 4];
            for i in 0..4 {
                m[i] = ma[i] + mb[i];
            }
            let e = r.entry(m).or_insert(f.zero());
            *e = f.add(e, &f.mul(ca, cb));
        }
    }
    r.into_iter().filter(|(_, c)| !f.is_zero(c)).collect()
}
pub fn ppow(f: &Fq, a: &Poly, e: u32) -> Poly {
    let mut r = konst(f, f.one());
    for _ in 0..e {
        r = pmulp(f, &r, a);
    }
    r
}
pub fn mpeval(f: &Fq, a: &Poly, v: &[E]) -> E {
    let mut s = f.zero();
    for (m, c) in a {
        let mut t = *c;
        for (i, &e) in m.iter().enumerate() {
            if e > 0 {
                t = f.mul(&t, &f.pow(&v[i], e as u128));
            }
        }
        s = f.add(&s, &t);
    }
    s
}
pub fn tdeg(m: &Mono) -> u32 {
    m.iter().map(|&e| e as u32).sum()
}

/// Coefficients (A, B, C) of S_3(u, v, X) = A X^2 + B X + C.
fn s3_coeffs(c: &Curve, u: &Poly, v: &Poly) -> (Poly, Poly, Poly) {
    let f = &c.f;
    let a = konst(f, c.a);
    let b = konst(f, c.b);
    let d = psub(f, u, v);
    let aa = pmulp(f, &d, &d);
    let s = padd(f, u, v);
    let pr = pmulp(f, u, v);
    let bb = pscale(
        f,
        &padd(f, &pmulp(f, &s, &padd(f, &pr, &a)), &pscale(f, &b, &f.c(2))),
        &f.ci(-2),
    );
    let t = psub(f, &pr, &a);
    let cc = psub(f, &pmulp(f, &t, &t), &pscale(f, &pmulp(f, &b, &s), &f.c(4)));
    (aa, bb, cc)
}

/// S_{n+1}(x_1..x_n, x_R) as a polynomial in x_1..x_n (vars 0..n-1), n in {2,3}.
pub fn semaev(c: &Curve, n: usize, xr: &E) -> Poly {
    let f = &c.f;
    let xrp = konst(f, *xr);
    match n {
        2 => {
            let (a, b, cc) = s3_coeffs(c, &var(f, 0), &var(f, 1));
            let x2 = pmulp(f, &xrp, &xrp);
            padd(f, &padd(f, &pmulp(f, &a, &x2), &pmulp(f, &b, &xrp)), &cc)
        }
        3 => {
            let (a, b, cc) = s3_coeffs(c, &var(f, 0), &var(f, 1));
            let (d, e, ff) = s3_coeffs(c, &var(f, 2), &xrp);
            // Res(aX^2+bX+c, dX^2+eX+f) = (af-cd)^2 - (ae-bd)(bf-ce)
            let afcd = psub(f, &pmulp(f, &a, &ff), &pmulp(f, &cc, &d));
            let aebd = psub(f, &pmulp(f, &a, &e), &pmulp(f, &b, &d));
            let bfce = psub(f, &pmulp(f, &b, &ff), &pmulp(f, &cc, &e));
            psub(f, &pmulp(f, &afcd, &afcd), &pmulp(f, &aebd, &bfce))
        }
        _ => panic!("n must be 2 or 3"),
    }
}

/// Elementary symmetric polynomial sigma_i (i = 1..k) in vars 0..k-1.
fn sigma(f: &Fq, k: usize, i: usize) -> Poly {
    let mut r = Poly::new();
    for mask in 0u32..(1 << k) {
        if mask.count_ones() as usize == i {
            let mut m = [0u8; 4];
            for j in 0..k {
                if mask >> j & 1 == 1 {
                    m[j] = 1;
                }
            }
            r.insert(m, f.one());
        }
    }
    r
}

/// Rewrite a symmetric polynomial in vars 0..k-1 in elementary symmetric variables e_1..e_k
/// (stored as vars 0..k-1 of the output).
pub fn symmetrize(f: &Fq, g: &Poly, k: usize) -> Poly {
    let sig: Vec<Poly> = (1..=k).map(|i| sigma(f, k, i)).collect();
    let mut cache: HashMap<(usize, u8), Poly> = HashMap::new();
    let mut rest = g.clone();
    let mut out = Poly::new();
    let mut guard = 0;
    while let Some((m, c)) = rest.iter().next_back().map(|(m, c)| (*m, *c)) {
        guard += 1;
        assert!(guard < 1_000_000, "symmetrize did not terminate");
        let mut em = [0u8; 4];
        for i in 0..k {
            let next = if i + 1 < k { m[i + 1] } else { 0 };
            assert!(m[i] >= next, "polynomial is not symmetric");
            em[i] = m[i] - next;
        }
        let e = out.entry(em).or_insert(f.zero());
        *e = f.add(e, &c);
        if f.is_zero(e) {
            out.remove(&em);
        }
        let mut prod = konst(f, c);
        for i in 0..k {
            if em[i] > 0 {
                let pw = cache
                    .entry((i, em[i]))
                    .or_insert_with(|| ppow(f, &sig[i], em[i] as u32))
                    .clone();
                prod = pmulp(f, &prod, &pw);
            }
        }
        rest = psub(f, &rest, &prod);
    }
    out
}

/// F_p polynomial: (monomial, coefficient)
pub type PPoly = Vec<(Mono, u64)>;

/// Weil restriction of a polynomial with F_{p^n} coefficients in F_p-valued variables.
pub fn weil(f: &Fq, g: &Poly) -> Vec<PPoly> {
    let mut out = vec![PPoly::new(); f.n];
    for (m, c) in g {
        for j in 0..f.n {
            if c[j] != 0 {
                out[j].push((*m, c[j]));
            }
        }
    }
    out.retain(|q| !q.is_empty());
    out
}

pub fn ppeval(p: u64, g: &PPoly, v: &[u64]) -> u64 {
    let mut s = 0u64;
    for (m, c) in g {
        let mut t = *c;
        for (i, &e) in m.iter().enumerate() {
            if e > 0 {
                t = mulm(t, powm(v[i], e as u128, p), p);
            }
        }
        s = (s + t) % p;
    }
    s
}

pub fn ppderiv(p: u64, g: &PPoly, i: usize) -> PPoly {
    g.iter()
        .filter(|(m, _)| m[i] > 0)
        .map(|(m, c)| {
            let mut mm = *m;
            mm[i] -= 1;
            (mm, mulm(*c, m[i] as u64 % p, p))
        })
        .filter(|(_, c)| *c != 0)
        .collect()
}

// ---------------- Macaulay-matrix probe ----------------
fn monos_upto(k: usize, d: u32) -> Vec<Mono> {
    let mut out = vec![];
    fn rec(k: usize, i: usize, left: u32, cur: &mut Mono, out: &mut Vec<Mono>) {
        if i == k {
            out.push(*cur);
            return;
        }
        for e in 0..=left {
            cur[i] = e as u8;
            rec(k, i + 1, left - e, cur, out);
        }
        cur[i] = 0;
    }
    rec(k, 0, d, &mut [0u8; 4], &mut out);
    out
}
/// grevlex key: larger key = larger monomial
fn grevlex_key(m: &Mono, k: usize) -> (u32, [i32; 4]) {
    let mut r = [0i32; 4];
    for j in 0..k {
        r[j] = -(m[k - 1 - j] as i32);
    }
    (tdeg(m), r)
}

#[derive(Clone, Debug)]
pub struct MacStep {
    pub d: u32,
    pub rows: usize,
    pub cols: usize,
    pub rank: usize,
    pub mults: u64,
    pub std: Option<u64>,
    pub fall: bool,
}

pub struct MacResult {
    pub steps: Vec<MacStep>,
    pub first_fall: Option<u32>,
    pub solving_degree: Option<u32>,
    pub quotient_dim: Option<u64>,
}

fn divides(a: &Mono, b: &Mono, k: usize) -> bool {
    (0..k).all(|i| a[i] <= b[i])
}

fn std_count(lead: &[Mono], k: usize) -> Option<u64> {
    let mut bound = [0u32; 4];
    for i in 0..k {
        let mut best = None;
        for m in lead {
            if (0..k).all(|j| j == i || m[j] == 0) && m[i] > 0 {
                best = Some(best.map_or(m[i] as u32, |b: u32| b.min(m[i] as u32)));
            }
        }
        bound[i] = best?;
    }
    // minimal generators
    let mut gens: Vec<Mono> = vec![];
    let mut sorted = lead.to_vec();
    sorted.sort_by_key(tdeg);
    for m in sorted {
        if !gens.iter().any(|g| divides(g, &m, k)) {
            gens.push(m);
        }
    }
    let mut count = 0u64;
    let mut cur = [0u8; 4];
    loop {
        if !gens.iter().any(|g| divides(g, &cur, k)) {
            count += 1;
        }
        let mut i = 0;
        loop {
            if i == k {
                return Some(count);
            }
            cur[i] += 1;
            if (cur[i] as u32) < bound[i] {
                break;
            }
            cur[i] = 0;
            i += 1;
        }
    }
}

/// Row-echelon pivots (leading monomials of the span) of the degree-<=D Macaulay matrix.
fn macaulay_pivots(p: u64, sys: &[PPoly], k: usize, d: u32) -> (Vec<Mono>, usize, usize, u64) {
    let mut monos = monos_upto(k, d);
    monos.sort_by_key(|m| std::cmp::Reverse(grevlex_key(m, k)));
    let idx: HashMap<Mono, usize> = monos.iter().enumerate().map(|(i, m)| (*m, i)).collect();
    let cols = monos.len();
    let mut rows: Vec<Vec<u64>> = vec![];
    for g in sys {
        let dg = g.iter().map(|(m, _)| tdeg(m)).max().unwrap_or(0);
        if dg > d {
            continue;
        }
        for sh in monos_upto(k, d - dg) {
            let mut r = vec![0u64; cols];
            for (m, c) in g {
                let mut mm = *m;
                for i in 0..k {
                    mm[i] += sh[i];
                }
                r[idx[&mm]] = *c;
            }
            rows.push(r);
        }
    }
    let nrows = rows.len();
    let mut rank = 0;
    let mut mults = 0u64;
    let mut piv = vec![];
    for col in 0..cols {
        let Some(pr) = (rank..nrows).find(|&r| rows[r][col] != 0) else { continue };
        rows.swap(rank, pr);
        let inv = invm(rows[rank][col], p);
        let pivot_row: Vec<u64> = rows[rank][col..].iter().map(|&x| mulm(x, inv, p)).collect();
        mults += (cols - col) as u64;
        for r in rank + 1..nrows {
            let fct = rows[r][col];
            if fct != 0 {
                let row = &mut rows[r];
                for (j, &pv) in pivot_row.iter().enumerate() {
                    if pv != 0 {
                        row[col + j] = (row[col + j] + p - (fct * pv) % p) % p;
                    }
                }
                mults += (cols - col) as u64;
            }
        }
        piv.push(monos[col]);
        rank += 1;
    }
    (piv, nrows, cols, mults)
}

pub fn macaulay_probe(p: u64, sys: &[PPoly], k: usize, dcap: u32) -> MacResult {
    let dmin = sys.iter().flat_map(|g| g.iter().map(|(m, _)| tdeg(m))).max().unwrap_or(1);
    let mut steps: Vec<MacStep> = vec![];
    let mut prev: Vec<Mono> = vec![];
    let mut first_fall = None;
    for d in dmin..=dcap {
        let (piv, rows, cols, mults) = macaulay_pivots(p, sys, k, d);
        let fall = piv.iter().any(|m| tdeg(m) < d && !prev.contains(m));
        if fall && first_fall.is_none() {
            first_fall = Some(d);
        }
        let std = std_count(&piv, k);
        steps.push(MacStep { d, rows, cols, rank: piv.len(), mults, std, fall });
        prev = piv;
        let l = steps.len();
        if l >= 3 && std.is_some() && steps[l - 2].std == std && steps[l - 3].std == std {
            break;
        }
    }
    let l = steps.len();
    let stable = l >= 3 && steps[l - 1].std.is_some() && steps[l - 2].std == steps[l - 1].std && steps[l - 3].std == steps[l - 1].std;
    let (solving_degree, quotient_dim) = if stable {
        let fin = steps[l - 1].std;
        (steps.iter().find(|s| s.std == fin).map(|s| s.d), fin)
    } else {
        (None, None)
    };
    MacResult { steps, first_fall, solving_degree, quotient_dim }
}
