//! Exhaustive search for Moebius maps `mu` over `F_q` (q = p^2) that carry the branch set of the
//! hyperelliptic curve `y^2 = h_src(x)` (h monic, degree 7, so infinity is a branch point) onto that of
//! `y^2 = h_tgt(x)`: `H_src(mu(x)) (cx+d)^8 = kappa * h_tgt(x)`.
//!
//! The linear case is `(h_src, h_tgt) = (h, h)`; the semilinear case (an isomorphism of H with its
//! p-Frobenius conjugate composed with Frobenius) is `(h^tau, h)`.  Both are exhaustive over
//! all of `PGL2(F_q)` that can permute the rational branch points (see PLAN.md): affine maps with
//! `t` forced by the x^6 coefficient, and `u + k/(x-v)` for rational finite roots `u`, `v`.

use crypto_lib::cryptanalysis::jv_cover::{Fld, Fq, E2};

pub type P = Vec<E2>;

#[derive(Clone, Debug)]
#[allow(dead_code)] // the map's coefficients are read by Debug output and the unit tests
pub struct Mob {
    pub a: E2,
    pub b: E2,
    pub c: E2,
    pub d: E2,
    /// `H(mu x) (cx+d)^8 = kappa * h_tgt(x)`
    pub kappa: E2,
    pub kappa_square: bool,
    /// the map sends `P^1(F_p)` to itself (coefficients proportional to `F_p`)
    pub preserves_fp_line: bool,
    pub is_identity: bool,
}

pub fn peval(f: &Fq, h: &[E2], x: &E2) -> E2 {
    let mut acc = E2::ZERO;
    for c in h.iter().rev() {
        acc = f.add(&f.mul(&acc, x), c);
    }
    acc
}

/// `sum_i h_i num^i den^(8-i)`, the degree-8 form (h has degree <= 7).
pub fn hom_eval(f: &Fq, h: &[E2], num: &E2, den: &E2) -> E2 {
    let mut dp = [E2::ONE; 9];
    for i in 1..9 {
        dp[i] = f.mul(&dp[i - 1], den);
    }
    let mut acc = E2::ZERO;
    let mut np = E2::ONE;
    for (i, c) in h.iter().enumerate() {
        acc = f.add(&acc, &f.mul(&f.mul(c, &np), &dp[8 - i]));
        np = f.mul(&np, num);
    }
    acc
}

pub fn padd(f: &Fq, a: &[E2], b: &[E2]) -> P {
    let n = a.len().max(b.len());
    (0..n)
        .map(|i| f.add(a.get(i).unwrap_or(&E2::ZERO), b.get(i).unwrap_or(&E2::ZERO)))
        .collect()
}

pub fn pmul(f: &Fq, a: &[E2], b: &[E2]) -> P {
    let mut r = vec![E2::ZERO; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            r[i + j] = f.add(&r[i + j], &f.mul(x, y));
        }
    }
    r
}

/// The octic polynomial `sum_i h_i (ax+b)^i (cx+d)^(8-i)` (coefficients in x, length 9).
pub fn transform(f: &Fq, h: &[E2], m: &[E2; 4]) -> P {
    let num: P = vec![m[1], m[0]];
    let den: P = vec![m[3], m[2]];
    let mut out: P = vec![E2::ZERO; 9];
    let mut dp: Vec<P> = vec![vec![E2::ONE]];
    for i in 1..9 {
        dp.push(pmul(f, &dp[i - 1], &den));
    }
    let mut np: P = vec![E2::ONE];
    for (i, c) in h.iter().enumerate() {
        let term = pmul(f, &np, &dp[8 - i]);
        let scaled: P = term.iter().map(|t| f.mul(t, c)).collect();
        out = padd(f, &out, &scaled);
        np = pmul(f, &np, &num);
    }
    out.resize(9, E2::ZERO);
    out
}

fn trim(f: &Fq, mut a: P) -> P {
    while a.last().is_some_and(|c| f.is_zero(c)) {
        a.pop();
    }
    a
}

/// `Some(kappa)` iff `a = kappa * b` as polynomials, `b != 0`.
pub fn proportional(f: &Fq, a: &[E2], b: &[E2]) -> Option<E2> {
    let a = trim(f, a.to_vec());
    let b = trim(f, b.to_vec());
    if b.is_empty() || a.len() != b.len() {
        return None;
    }
    let k = f.mul(a.last().unwrap(), &f.inv(b.last().unwrap()));
    for (x, y) in a.iter().zip(&b) {
        if *x != f.mul(&k, y) {
            return None;
        }
    }
    Some(k)
}

pub fn conj_poly(f: &Fq, h: &[E2]) -> P {
    h.iter().map(|c| f.conj(c)).collect()
}

fn all_nonzero(p: u64) -> impl Iterator<Item = E2> {
    (0..p)
        .flat_map(move |y| (0..p).map(move |x| E2([x, y])))
        .filter(|e| *e != E2::ZERO)
}

/// Finite roots of `h` in `F_q` (brute force over all q elements).
pub fn rational_roots(f: &Fq, h: &[E2]) -> Vec<E2> {
    let p = f.p;
    let mut out = Vec::new();
    for y in 0..p {
        for x in 0..p {
            let e = E2([x, y]);
            if f.is_zero(&peval(f, h, &e)) {
                out.push(e);
            }
        }
    }
    out
}

fn is_fp_line_map(f: &Fq, m: &[E2; 4]) -> bool {
    // proportional to a vector over F_p: normalise by the first nonzero entry
    let Some(first) = m.iter().find(|c| !f.is_zero(c)) else {
        return false;
    };
    let inv = f.inv(first);
    m.iter().all(|c| f.mul(c, &inv).in_fp())
}

fn finish(f: &Fq, m: [E2; 4], kappa: E2) -> Mob {
    let one = f.one();
    let is_id = f.is_zero(&m[1]) && f.is_zero(&m[2]) && f.mul(&m[0], &f.inv(&m[3])) == one;
    Mob {
        a: m[0],
        b: m[1],
        c: m[2],
        d: m[3],
        kappa,
        kappa_square: f.is_square(&kappa),
        preserves_fp_line: is_fp_line_map(f, &m),
        is_identity: is_id,
    }
}

/// All Moebius maps over `F_q` with `H_src(mu x)(cx+d)^8 = kappa h_tgt(x)`.
/// `h_src`, `h_tgt`: monic of degree 7 (length 8).
pub fn search(f: &Fq, h_src: &[E2], h_tgt: &[E2]) -> Vec<Mob> {
    assert_eq!(h_src.len(), 8);
    assert_eq!(h_tgt.len(), 8);
    assert!(h_src[7] == E2::ONE && h_tgt[7] == E2::ONE);
    assert!(f.p != 7);
    let mut found = Vec::new();
    // affine maps x -> s x + t, t = (s h_tgt6 - h_src6)/7
    let inv7 = f.inv(&f.from_fp(7));
    let (c6s, c6t) = (h_src[6], h_tgt[6]);
    let t0 = peval(f, h_tgt, &E2::ZERO);
    let t1 = peval(f, h_tgt, &E2::ONE);
    for s in all_nonzero(f.p) {
        let t = f.mul(&f.sub(&f.mul(&s, &c6t), &c6s), &inv7);
        let s2 = f.sq(&s);
        let s3 = f.mul(&s2, &s);
        let s7 = f.mul(&f.mul(&s3, &s3), &s);
        // x = 0: h_src(t) = s^7 h_tgt(0)
        if peval(f, h_src, &t) != f.mul(&s7, &t0) {
            continue;
        }
        // x = 1: h_src(s + t) = s^7 h_tgt(1)
        if peval(f, h_src, &f.add(&s, &t)) != f.mul(&s7, &t1) {
            continue;
        }
        let m = [s, t, E2::ZERO, E2::ONE];
        let tr = transform(f, h_src, &m);
        if let Some(k) = proportional(f, &tr, h_tgt) {
            found.push(finish(f, m, k));
        }
    }
    // pole maps x -> u + k/(x - v): mu(inf) = u is a root of h_src, mu(v) = inf so v is a root of h_tgt
    let rs = rational_roots(f, h_src);
    let rt = rational_roots(f, h_tgt);
    if !rs.is_empty() && !rt.is_empty() {
        // three test points away from v and from the roots of h_tgt
        let mut pts = Vec::new();
        let mut x = 2u64;
        while pts.len() < 3 {
            let e = E2([x, 1]);
            if !f.is_zero(&peval(f, h_tgt, &e)) && !rs.contains(&e) {
                pts.push((e, peval(f, h_tgt, &e)));
            }
            x += 1;
        }
        for u in &rs {
            for v in &rt {
                let uv = f.mul(u, v);
                for k in all_nonzero(f.p) {
                    // mu x = (u x + (k - u v)) / (x - v)
                    let b = f.sub(&k, &uv);
                    let mut vals = [E2::ZERO; 3];
                    let mut ok = true;
                    for (i, (xi, _)) in pts.iter().enumerate() {
                        let den = f.sub(xi, v);
                        if f.is_zero(&den) {
                            ok = false;
                            break;
                        }
                        let num = f.add(&f.mul(u, xi), &b);
                        vals[i] = hom_eval(f, h_src, &num, &den);
                    }
                    if !ok {
                        continue;
                    }
                    // vals[i] = kappa * h_tgt(x_i) for one kappa
                    if f.mul(&vals[0], &pts[1].1) != f.mul(&vals[1], &pts[0].1)
                        || f.mul(&vals[0], &pts[2].1) != f.mul(&vals[2], &pts[0].1)
                    {
                        continue;
                    }
                    let m = [*u, b, E2::ONE, f.neg(v)];
                    let tr = transform(f, h_src, &m);
                    if let Some(kp) = proportional(f, &tr, h_tgt) {
                        found.push(finish(f, m, kp));
                    }
                }
            }
        }
    }
    found
}

/// Census record for one curve: linear and semilinear maps, identity excluded from the counts.
#[derive(Clone, Debug, Default)]
pub struct AutCensus {
    pub rational_finite_roots: usize,
    pub linear_total: usize,
    pub linear_nontrivial: usize,
    pub linear_nontrivial_square_kappa: usize,
    pub linear_nontrivial_fp_line: usize,
    pub semilinear_total: usize,
    pub semilinear_square_kappa: usize,
    pub semilinear_fp_line: usize,
}

pub fn census(f: &Fq, h: &[E2]) -> AutCensus {
    let lin = search(f, h, h);
    let semi = search(f, &conj_poly(f, h), h);
    let nt: Vec<&Mob> = lin.iter().filter(|m| !m.is_identity).collect();
    AutCensus {
        rational_finite_roots: rational_roots(f, h).len(),
        linear_total: lin.len(),
        linear_nontrivial: nt.len(),
        linear_nontrivial_square_kappa: nt.iter().filter(|m| m.kappa_square).count(),
        linear_nontrivial_fp_line: nt.iter().filter(|m| m.preserves_fp_line).count(),
        semilinear_total: semi.len(),
        semilinear_square_kappa: semi.iter().filter(|m| m.kappa_square).count(),
        semilinear_fp_line: semi.iter().filter(|m| m.preserves_fp_line).count(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn monic7(f: &Fq, rng: &mut StdRng) -> P {
        let mut h: P = (0..7).map(|_| f.random(rng)).collect();
        h.push(E2::ONE);
        h
    }

    #[test]
    fn planted_sixfold_symmetry_is_found() {
        // h = x^7 + c x: x -> s x with s^6 = 1 (6 maps incl. identity; q - 1 divisible by 6)
        let f = Fq::new(31);
        let c = f.from_fp(5);
        let mut h = vec![E2::ZERO; 8];
        h[7] = E2::ONE;
        h[1] = c;
        let r = search(&f, &h, &h);
        let affine: Vec<&Mob> = r.iter().filter(|m| f.is_zero(&m.c)).collect();
        assert_eq!(affine.len(), 6, "{r:?}");
        assert_eq!(r.iter().filter(|m| m.is_identity).count(), 1);
        for m in affine {
            // kappa = s^7 = s for s^6 = 1
            assert_eq!(m.kappa, m.a);
        }
    }

    #[test]
    fn random_curve_has_only_the_identity() {
        let f = Fq::new(31);
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..6 {
            let h = monic7(&f, &mut rng);
            let r = search(&f, &h, &h);
            assert_eq!(r.len(), 1);
            assert!(r[0].is_identity);
        }
    }

    #[test]
    fn planted_pole_map_is_recovered() {
        let f = Fq::new(31);
        let mut rng = StdRng::seed_from_u64(11);
        // h_src with a rational finite root v, h_tgt = transform of h_src by mu(x) = u + k/(x - v)
        let u = f.from_fp(3);
        let v = f.from_fp(9);
        let mut m6: P = (0..6).map(|_| f.random(&mut rng)).collect();
        m6.push(E2::ONE);
        let h = pmul(&f, &[f.neg(&u), E2::ONE], &m6);
        assert_eq!(h.len(), 8);
        assert_eq!(h[7], E2::ONE);
        let k = f.from_fp(4);
        let m = [u, f.sub(&k, &f.mul(&u, &v)), E2::ONE, f.neg(&v)];
        let tr = transform(&f, &h, &m);
        assert_eq!(
            tr[8],
            E2::ZERO,
            "u is a root of h so the transform has degree <= 7"
        );
        let lead = tr[7];
        let hn: P = tr[..8].iter().map(|c| f.mul(c, &f.inv(&lead))).collect();
        assert!(f.is_zero(&peval(&f, &hn, &v)));
        let r = search(&f, &h, &hn);
        assert!(
            r.iter().any(|x| !f.is_zero(&x.c)
                && f.mul(&x.a, &f.inv(&x.c)) == u
                && f.mul(&x.d, &f.inv(&x.c)) == f.neg(&v)),
            "{r:?}"
        );
    }

    #[test]
    fn semilinear_planted() {
        // h with coefficients in F_p: tau-conjugation fixes h, so (h^tau, h) finds the identity
        let f = Fq::new(31);
        let h: P = vec![
            f.from_fp(3),
            f.from_fp(1),
            f.from_fp(4),
            f.from_fp(1),
            f.from_fp(5),
            f.from_fp(9),
            f.from_fp(2),
            E2::ONE,
        ];
        let r = search(&f, &conj_poly(&f, &h), &h);
        assert!(r.iter().any(|m| m.is_identity));
        // a generic h (coefficients genuinely in F_{p^2}) has no semilinear map
        let mut rng = StdRng::seed_from_u64(3);
        let g = monic7(&f, &mut rng);
        assert!(search(&f, &conj_poly(&f, &g), &g).is_empty());
    }
}
