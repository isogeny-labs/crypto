//! Coppersmith / Howgrave-Graham lattice for a polynomial f(x_0..x_{nv-1})
//! modulo p, monic in the corner monomial x^d of its Newton box, and root
//! extraction from the reduced basis (resultants and roots over F_q).
use crate::fq::{self, P};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use rand::Rng;
use std::collections::HashMap;

pub type M3 = [u8; 3];
pub type ZPoly = HashMap<M3, BigInt>;

pub fn zmul(a: &ZPoly, b: &ZPoly, modulus: &BigInt) -> ZPoly {
    let mut r: ZPoly = HashMap::new();
    for (e1, c1) in a {
        for (e2, c2) in b {
            let e = [e1[0] + e2[0], e1[1] + e2[1], e1[2] + e2[2]];
            *r.entry(e).or_insert_with(BigInt::zero) += c1 * c2;
        }
    }
    for v in r.values_mut() { *v = v.mod_floor(modulus); }
    r.retain(|_, v| !v.is_zero());
    r
}

pub fn centered(v: &BigInt, m: &BigInt) -> BigInt {
    let r = v.mod_floor(m);
    if &r + &r > *m { r - m } else { r }
}

/// Monomials of the box [0, m d_i]^nv, ordered by (total degree, lex).
pub fn box_monomials(nv: usize, d: M3, m: u32) -> Vec<M3> {
    let mut out = vec![];
    let lim = |i: usize| if i < nv { d[i] as u32 * m } else { 0 };
    for a in 0..=lim(0) { for b in 0..=lim(1) { for c in 0..=lim(2) {
        out.push([a as u8, b as u8, c as u8]);
    } } }
    out.sort_by_key(|e| (e[0] as u32 + e[1] as u32 + e[2] as u32, *e));
    out
}

pub struct Lattice {
    pub monos: Vec<M3>,
    pub rows: Vec<Vec<BigInt>>,
    pub pm: BigInt, // p^m
    pub xbound: BigInt,
}

/// Shifts g_e = x^{e - k d} f^k p^{m-k}, k = min_i floor(e_i/d_i) (capped at m),
/// one per monomial e of the box; columns scaled by X^{|e|}.
pub fn build(f: &ZPoly, nv: usize, d: M3, m: u32, p: u64, xb: u64) -> Lattice {
    let pb = BigInt::from(p);
    let pm = num_traits::pow(pb.clone(), m as usize);
    let monos = box_monomials(nv, d, m);
    let idx: HashMap<M3, usize> = monos.iter().enumerate().map(|(i, e)| (*e, i)).collect();
    // powers of f mod p^m
    let mut fpow: Vec<ZPoly> = vec![HashMap::from([([0, 0, 0], BigInt::one())])];
    for _ in 0..m { let last = fpow.last().unwrap().clone(); fpow.push(zmul(&last, f, &pm)); }
    let xbig = BigInt::from(xb);
    let maxdeg = monos.iter().map(|e| e.iter().map(|&t| t as usize).sum::<usize>()).max().unwrap();
    let xpow: Vec<BigInt> = (0..=maxdeg).map(|t| num_traits::pow(xbig.clone(), t)).collect();
    let mut rows = vec![];
    for e in &monos {
        let mut k = m;
        for i in 0..nv { k = k.min(e[i] as u32 / d[i] as u32); }
        let sh = [e[0] - k as u8 * d[0], e[1] - k as u8 * d[1], e[2] - k as u8 * d[2]];
        let pk = num_traits::pow(pb.clone(), (m - k) as usize);
        let mut row = vec![BigInt::zero(); monos.len()];
        for (fe, c) in &fpow[k as usize] {
            let ne = [fe[0] + sh[0], fe[1] + sh[1], fe[2] + sh[2]];
            let j = *idx.get(&ne).expect("shift leaves the box");
            // off-diagonal entries reduced mod p^m (p^m x^ne lies in the lattice);
            // the diagonal entry p^{m-k} is kept as is.
            let v = if ne == *e { c * &pk } else { centered(&(c * &pk), &pm) };
            let deg = ne.iter().map(|&t| t as usize).sum::<usize>();
            row[j] = v * &xpow[deg];
        }
        rows.push(row);
    }
    Lattice { monos, rows, pm, xbound: xbig }
}

/// Unscale a reduced row into integer polynomial coefficients.
pub fn row_poly(lat: &Lattice, row: &[BigInt]) -> Vec<(M3, BigInt)> {
    let mut out = vec![];
    for (j, v) in row.iter().enumerate() {
        if v.is_zero() { continue; }
        let e = lat.monos[j];
        let deg = e.iter().map(|&t| t as usize).sum::<usize>();
        let s = num_traits::pow(lat.xbound.clone(), deg);
        let (q, r) = v.div_rem(&s);
        assert!(r.is_zero(), "row not divisible by its column scale");
        out.push((e, q));
    }
    out
}

pub fn row_norm_log2(row: &[BigInt]) -> f64 {
    let mut s = crate::dd::F::ZERO;
    for v in row { let f = crate::dd::F::from_big(v); s = s.add(f.mul(f)); }
    s.log2() / 2.0
}

pub fn eval_z(h: &[(M3, BigInt)], x: &[i64]) -> BigInt {
    let mut s = BigInt::zero();
    for (e, c) in h {
        let mut t = c.clone();
        for i in 0..3 { if e[i] > 0 { t *= num_traits::pow(BigInt::from(x[i]), e[i] as usize); } }
        s += t;
    }
    s
}

// ---------- root extraction over F_q ----------

#[derive(Clone, Debug)]
pub struct QP { pub nv: usize, pub t: Vec<(M3, u64)> }

impl QP {
    pub fn from_z(h: &[(M3, BigInt)], nv: usize) -> QP {
        let q = BigInt::from(fq::Q);
        let t = h.iter().map(|(e, c)| (*e, c.mod_floor(&q).to_u64().unwrap())).filter(|(_, c)| *c != 0).collect();
        QP { nv, t }
    }
    fn is_zero(&self) -> bool { self.t.is_empty() }
    fn deg(&self, i: usize) -> usize { self.t.iter().map(|(e, _)| e[i] as usize).max().unwrap_or(0) }
    /// Substitute the first nv-1 variables; univariate in the last.
    fn eval_prefix(&self, vals: &[u64]) -> P {
        let l = self.nv - 1;
        let mut r = vec![0u64; self.deg(l) + 1];
        for (e, c) in &self.t {
            let mut v = *c;
            for i in 0..l { v = fq::mq(v, fq::pq(vals[i], e[i] as u64)); }
            r[e[l] as usize] = fq::aq(r[e[l] as usize], v);
        }
        fq::trim(&mut r);
        r
    }
    /// Drop the last variable (only valid when it does not occur).
    fn drop_last(&self) -> QP { QP { nv: self.nv - 1, t: self.t.clone() } }
}

/// Res_{last}(a, b) as a polynomial in the first nv-1 variables (evaluation /
/// interpolation on an arithmetic-progression grid).
fn res_last<R: Rng>(a: &QP, b: &QP, rng: &mut R) -> Result<QP, String> {
    let nv = a.nv;
    let l = nv - 1;
    let (la, lb) = (a.deg(l), b.deg(l));
    let bounds: Vec<usize> = (0..l).map(|i| a.deg(i) * lb + la * b.deg(i)).collect();
    let s: Vec<u64> = (0..l).map(|_| rng.gen_range(1..fq::Q)).collect();
    let t: Vec<u64> = (0..l).map(|_| rng.gen_range(1..fq::Q)).collect();
    let pt = |i: usize, j: usize| fq::aq(s[i], fq::mq(t[i], j as u64));
    let val = |vals: &[u64]| -> Result<u64, String> {
        let pa = a.eval_prefix(vals);
        let pb = b.eval_prefix(vals);
        if fq::deg(&pa) != la as isize || fq::deg(&pb) != lb as isize { return Err("degree drop at grid point".into()); }
        Ok(fq::resultant(&pa, &pb))
    };
    let mut out = QP { nv: l, t: vec![] };
    if l == 1 {
        let ys: Result<Vec<u64>, String> = (0..=bounds[0]).map(|j| val(&[pt(0, j)])).collect();
        let c = fq::interp_ap(s[0], t[0], &ys?);
        for (i, &v) in c.iter().enumerate() { if v != 0 { out.t.push(([i as u8, 0, 0], v)); } }
    } else if l == 2 {
        let (n0, n1) = (bounds[0] + 1, bounds[1] + 1);
        let mut rowsc: Vec<P> = vec![];
        for i in 0..n0 {
            let ys: Result<Vec<u64>, String> = (0..n1).map(|j| val(&[pt(0, i), pt(1, j)])).collect();
            let mut c = fq::interp_ap(s[1], t[1], &ys?);
            c.resize(n1, 0);
            rowsc.push(c);
        }
        for j in 0..n1 {
            let ys: Vec<u64> = (0..n0).map(|i| rowsc[i][j]).collect();
            let c = fq::interp_ap(s[0], t[0], &ys);
            for (i, &v) in c.iter().enumerate() {
                if v != 0 {
                    if i > 255 || j > 255 { return Err("degree overflow".into()); }
                    out.t.push(([i as u8, j as u8, 0], v));
                }
            }
        }
    } else {
        return Err("unsupported".into());
    }
    Ok(out)
}

/// All common roots with |x_i| <= xb of the given polynomials (over F_q, lifted
/// to the symmetric range). Err if the system is underdetermined.
pub fn solve<R: Rng>(hs: &[QP], xb: i64, rng: &mut R) -> Result<Vec<Vec<i64>>, String> {
    let hs: Vec<QP> = hs.iter().filter(|h| !h.is_zero()).cloned().collect();
    if hs.is_empty() { return Err("no nonzero polynomial".into()); }
    let nv = hs[0].nv;
    let lift_ok = |r: u64| { let v = fq::lift(r); if v.abs() <= xb { Some(v) } else { None } };
    if nv == 1 {
        let mut g: P = vec![];
        for h in &hs { g = fq::gcd(&g, &h.eval_prefix(&[])); }
        if g.is_empty() { return Err("all zero".into()); }
        return Ok(fq::roots(&g, rng).into_iter().filter_map(lift_ok).map(|v| vec![v]).collect());
    }
    let l = nv - 1;
    let mut down: Vec<QP> = hs.iter().filter(|h| h.deg(l) == 0).map(|h| h.drop_last()).collect();
    let others: Vec<&QP> = hs.iter().filter(|h| h.deg(l) > 0).take(3).collect();
    for i in 0..others.len() {
        for j in i + 1..others.len() {
            if down.len() >= 3 { break; }
            let r = res_last(others[i], others[j], rng)?;
            if !r.is_zero() { down.push(r); }
        }
    }
    if down.is_empty() { return Err("all resultants vanish".into()); }
    let prefixes = solve(&down, xb, rng)?;
    let mut out = vec![];
    for pre in prefixes {
        let vals: Vec<u64> = pre.iter().map(|&v| fq::from_i128(v as i128)).collect();
        let mut g: P = vec![];
        for h in &hs { g = fq::gcd(&g, &h.eval_prefix(&vals)); }
        if g.is_empty() { continue; } // positive-dimensional fibre: skip
        for r in fq::roots(&g, rng) {
            if let Some(v) = lift_ok(r) { let mut t = pre.clone(); t.push(v); out.push(t); }
        }
    }
    Ok(out)
}

#[allow(dead_code)]
pub fn bigabs_log2(x: &BigInt) -> f64 { crate::dd::big_to_log2(&x.abs()) }
