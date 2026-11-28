//! m6_calib: exhaustive calibration of the first-moment decomposition model
//! E = N^m / m! / r on small Koblitz curves with signed-Frobenius orbit
//! factor bases.
//!
//! Field arithmetic is the repository's `src/binary_ecc/f2m.rs`, included by
//! path (the full `crypto` crate does not fit the disk budget). Only the
//! affine chord-and-tangent formulas for `y^2 + xy = x^3 + a x^2 + 1` are
//! written here.
//!
//! Method: enumerate the prime-order subgroup <G> (r <= ~2^21), so every
//! factor-base point has an exact discrete log. Counting m-decompositions of
//! EVERY target R in <G> is then a dynamic program over Z/r (subset-sum
//! convolution), exhaustive over all targets, not sampled.
//!
//! Output: one JSON line per (curve, base-selection, m, variant).

#[allow(dead_code, clippy::all)]
#[path = "../../../src/binary_ecc/f2m.rs"]
mod f2m;
#[allow(dead_code, clippy::all)]
#[path = "../../../src/binary_ecc/operation_tally.rs"]
mod operation_tally;
// f2m.rs tests name `crate::binary_ecc::operation_tally`.
#[allow(unused_imports)]
mod binary_ecc {
    pub(crate) use super::operation_tally;
}

use f2m::{F2mElement, IrreduciblePoly};
use std::collections::HashMap;

/// Low-weight moduli, irreducibility checked at start-up.
fn modulus(n: u32) -> IrreduciblePoly {
    let low: Vec<u32> = match n {
        13 => vec![0, 1, 3, 4],
        17 => vec![0, 3],
        19 => vec![0, 1, 2, 5],
        23 => vec![0, 5],
        29 => vec![0, 2],
        31 => vec![0, 3],
        37 => vec![0, 1, 4, 6],
        41 => vec![0, 3],
        _ => panic!("no modulus for n={n}"),
    };
    IrreduciblePoly { degree: n, low_terms: low }
}

/// For prime n and odd weight: irreducible iff z^(2^n) = z mod f.
fn check_irreducible(n: u32, f: &IrreduciblePoly) {
    assert!(f.low_terms.contains(&0) && (f.low_terms.len() + 1) % 2 == 1);
    let z = F2mElement::z(n);
    assert_eq!(z.square_k_times(n, f), z, "modulus not irreducible for n={n}");
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Pt {
    O,
    A(F2mElement, F2mElement),
}

struct Curve {
    n: u32,
    a: u8,
    f: IrreduciblePoly,
}

impl Curve {
    fn a2(&self) -> F2mElement {
        if self.a == 1 { F2mElement::one(self.n) } else { F2mElement::zero(self.n) }
    }
    fn on_curve(&self, p: &Pt) -> bool {
        match p {
            Pt::O => true,
            Pt::A(x, y) => {
                let f = &self.f;
                let lhs = y.square(f).add(&x.mul(y, f));
                let x2 = x.square(f);
                let rhs = x2.mul(x, f).add(&self.a2().mul(&x2, f)).add(&F2mElement::one(self.n));
                lhs == rhs
            }
        }
    }
    fn neg(&self, p: &Pt) -> Pt {
        match p {
            Pt::O => Pt::O,
            Pt::A(x, y) => Pt::A(x.clone(), x.add(y)),
        }
    }
    fn frob(&self, p: &Pt) -> Pt {
        match p {
            Pt::O => Pt::O,
            Pt::A(x, y) => Pt::A(x.square(&self.f), y.square(&self.f)),
        }
    }
    fn add(&self, p: &Pt, q: &Pt) -> Pt {
        let f = &self.f;
        match (p, q) {
            (Pt::O, _) => q.clone(),
            (_, Pt::O) => p.clone(),
            (Pt::A(x1, y1), Pt::A(x2, y2)) => {
                if x1 == x2 {
                    if *y2 == x1.add(y1) || x1.is_zero() {
                        return Pt::O;
                    }
                    // doubling: l = x + y/x ; x3 = l^2 + l + a ; y3 = x^2 + (l+1) x3
                    let l = x1.add(&y1.mul(&x1.flt_inverse(f).unwrap(), f));
                    let x3 = l.square(f).add(&l).add(&self.a2());
                    let y3 = x1.square(f).add(&l.add(&F2mElement::one(self.n)).mul(&x3, f));
                    return Pt::A(x3, y3);
                }
                let dx = x1.add(x2);
                let l = y1.add(y2).mul(&dx.flt_inverse(f).unwrap(), f);
                let x3 = l.square(f).add(&l).add(&dx).add(&self.a2());
                let y3 = l.mul(&x1.add(&x3), f).add(&x3).add(y1);
                Pt::A(x3, y3)
            }
        }
    }
    fn mul(&self, p: &Pt, k: u64) -> Pt {
        let mut acc = Pt::O;
        let mut base = p.clone();
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(&acc, &base);
            }
            base = self.add(&base, &base);
            k >>= 1;
        }
        acc
    }
    /// Point with given x if one exists (x != 0).
    fn lift_x(&self, x: &F2mElement) -> Option<Pt> {
        let f = &self.f;
        // y = x w, w^2 + w = x + a + 1/x^2
        let xi = x.flt_inverse(f)?;
        let c = x.add(&self.a2()).add(&xi.square(f));
        if c.trace(f) != 0 {
            return None;
        }
        let w = c.half_trace(f)?;
        let p = Pt::A(x.clone(), x.mul(&w, f));
        assert!(self.on_curve(&p));
        Some(p)
    }
}

fn key(p: &Pt) -> u128 {
    match p {
        Pt::O => u128::MAX,
        Pt::A(x, y) => ((x.raw_bits()[0] as u128) << 64) | y.raw_bits()[0] as u128,
    }
}

/// #E(F_{2^n}) for the Koblitz curve with parameter a (Lucas sequence).
fn koblitz_order(n: u32, a: u8) -> u64 {
    let mu: i128 = if a == 1 { 1 } else { -1 };
    let (mut t0, mut t1): (i128, i128) = (2, mu);
    for _ in 1..n {
        let t2 = mu * t1 - 2 * t0;
        t0 = t1;
        t1 = t2;
    }
    ((1i128 << n) + 1 - t1) as u64
}

fn largest_prime_factor(mut v: u64) -> u64 {
    let mut best = 1;
    let mut d = 2u64;
    while d * d <= v {
        while v % d == 0 {
            best = d;
            v /= d;
        }
        d += 1;
    }
    if v > 1 { v.max(best) } else { best }
}

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

fn lgamma_binom_ln(nn: f64, m: u32) -> f64 {
    // ln C(nn, m)
    (0..m).map(|i| (nn - i as f64).ln() - ((i + 1) as f64).ln()).sum()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        eprintln!("usage: m6_calib <n> <a> <c_orbits> <mmax> [seeds...]  (seed 0 = min-x orbits)");
        std::process::exit(2);
    }
    let n: u32 = args[1].parse().unwrap();
    let a: u8 = args[2].parse().unwrap();
    let c: usize = args[3].parse().unwrap();
    let mmax: usize = args[4].parse().unwrap();
    let seeds: Vec<u64> = if args.len() > 5 {
        args[5..].iter().map(|s| s.parse().unwrap()).collect()
    } else {
        vec![0]
    };

    let f = modulus(n);
    check_irreducible(n, &f);
    let cv = Curve { n, a, f };
    let order = koblitz_order(n, a);
    let r = largest_prime_factor(order);
    let h = order / r;
    assert!(r < (1 << 26), "subgroup too large for exhaustive enumeration");

    // generator: first x giving a point of full order r after cofactor clearing
    let mut g = Pt::O;
    for xv in 2u64.. {
        let x = F2mElement::from_words(&[xv], n);
        if let Some(p) = cv.lift_x(&x) {
            let q = cv.mul(&p, h);
            if q != Pt::O {
                assert_eq!(cv.mul(&q, r), Pt::O);
                g = q;
                break;
            }
        }
    }
    // enumerate <G>: dlog map; also verifies the order (G^r = O, no earlier O)
    let mut dlog: HashMap<u128, u32> = HashMap::with_capacity(r as usize);
    let mut pts: Vec<Pt> = Vec::with_capacity(r as usize);
    let mut cur = Pt::O;
    for k in 0..r {
        if k > 0 {
            assert!(cur != Pt::O, "order smaller than r");
        }
        dlog.insert(key(&cur), k as u32);
        pts.push(cur.clone());
        cur = cv.add(&cur, &g);
    }
    assert_eq!(cur, Pt::O, "G^r != O");
    // Frobenius eigenvalue lambda: tau(G) = lambda G
    let lambda = dlog[&key(&cv.frob(&g))] as u64;

    // affine points in the subgroup sorted by x (min-x selection order)
    let mut by_x: Vec<u32> = (1..r as u32).collect();
    by_x.sort_by_key(|&k| match &pts[k as usize] {
        Pt::A(x, _) => x.raw_bits()[0],
        Pt::O => u64::MAX,
    });

    for &seed in &seeds {
        let mut order_list = by_x.clone();
        if seed != 0 {
            let mut rng = SplitMix(seed ^ ((n as u64) << 32) ^ ((a as u64) << 48));
            for i in (1..order_list.len()).rev() {
                let j = (rng.next() % (i as u64 + 1)) as usize;
                order_list.swap(i, j);
            }
        }
        // choose c signed-Frobenius orbits; dlog of orbit = {+-lambda^j d}
        let mut used = vec![false; r as usize];
        let mut base: Vec<u32> = Vec::new();
        let mut classes: Vec<(u32, u32)> = Vec::new(); // (d, -d) pairs
        let mut chosen = 0;
        for &d0 in &order_list {
            if chosen == c {
                break;
            }
            if used[d0 as usize] {
                continue;
            }
            let mut d = d0 as u64;
            for _ in 0..n {
                let nd = (r - d) % r;
                assert!(!used[d as usize] && !used[nd as usize], "orbit smaller than 2n");
                used[d as usize] = true;
                used[nd as usize] = true;
                base.push(d as u32);
                base.push(nd as u32);
                classes.push((d as u32, nd as u32));
                d = d * lambda % r;
            }
            assert_eq!(d, d0 as u64, "tau^n != 1");
            chosen += 1;
        }
        // group-check the orbit structure on points (tau P and -P of one point)
        {
            let p = &pts[base[0] as usize];
            assert_eq!(dlog[&key(&cv.frob(p))] as u64, base[0] as u64 * lambda % r);
            assert_eq!(dlog[&key(&cv.neg(p))], base[1]);
        }
        let nb = base.len();
        let ru = r as usize;

        // variant "distinct": m distinct base points (any signs);
        // variant "pairfree": additionally never both P and -P.
        for variant in ["distinct", "pairfree"] {
            let mut cnt: Vec<Vec<u64>> = vec![vec![0u64; ru]; mmax + 1];
            cnt[0][0] = 1;
            let mut tmp = vec![0u64; ru];
            match variant {
                "distinct" => {
                    for &d in &base {
                        let d = d as usize;
                        for m in (1..=mmax).rev() {
                            let (lo, hi) = cnt.split_at_mut(m);
                            let src = &lo[m - 1];
                            let dst = &mut hi[0];
                            // dst[x] += src[x - d]
                            for x in 0..ru {
                                let y = if x >= d { x - d } else { x + ru - d };
                                dst[x] += src[y];
                            }
                        }
                    }
                }
                _ => {
                    for &(d1, d2) in &classes {
                        let (d1, d2) = (d1 as usize, d2 as usize);
                        for m in (1..=mmax).rev() {
                            let (lo, hi) = cnt.split_at_mut(m);
                            let src = &lo[m - 1];
                            for x in 0..ru {
                                let y1 = if x >= d1 { x - d1 } else { x + ru - d1 };
                                let y2 = if x >= d2 { x - d2 } else { x + ru - d2 };
                                tmp[x] = src[y1] + src[y2];
                            }
                            for x in 0..ru {
                                hi[0][x] += tmp[x];
                            }
                        }
                    }
                }
            }
            for m in 2..=mmax {
                let v = &cnt[m];
                // targets: all nonzero R (R = O excluded; it is the free-relation case)
                let mut s = 0f64;
                let mut s2 = 0f64;
                let mut zeros = 0u64;
                let mut mx = 0u64;
                let mut total: u128 = 0;
                for x in 1..ru {
                    let w = v[x];
                    total += w as u128;
                    s += w as f64;
                    s2 += (w as f64) * (w as f64);
                    if w == 0 {
                        zeros += 1;
                    }
                    mx = mx.max(w);
                }
                let total_all = total + v[0] as u128;
                let t = (ru - 1) as f64;
                let mean = s / t;
                let var = s2 / t - mean * mean;
                let nf = nb as f64;
                let lnfact: f64 = (1..=m).map(|i| (i as f64).ln()).sum();
                let e_model = (m as f64 * nf.ln() - lnfact - (r as f64).ln()).exp();
                let e_exact_distinct = (lgamma_binom_ln(nf, m as u32) - (r as f64).ln()).exp();
                let frac_nonzero = 1.0 - zeros as f64 / t;
                let poisson_nonzero = 1.0 - (-mean).exp();
                // Frobenius invariance: count(R) == count(tau R) == count(-R), sampled
                let mut rng = SplitMix(0xC0FFEE ^ seed);
                let mut frob_ok = true;
                for _ in 0..1000 {
                    let x = 1 + (rng.next() % (r - 1)) as usize;
                    let tx = (x as u64 * lambda % r) as usize;
                    if v[x] != v[tx] || v[x] != v[ru - x] {
                        frob_ok = false;
                    }
                }
                println!(
                    "{{\"n\":{n},\"a\":{a},\"r\":{r},\"cofactor\":{h},\"lambda\":{lambda},\"c\":{c},\"N\":{nb},\"seed\":{seed},\"variant\":\"{variant}\",\"m\":{m},\"targets\":{},\"sum_all_R_incl_O\":{total_all},\"mean\":{mean:.6e},\"E_model\":{e_model:.6e},\"E_exact_distinct\":{e_exact_distinct:.6e},\"mean_over_E_model\":{:.6},\"var_over_mean\":{:.4},\"frac_targets_with_decomp\":{frac_nonzero:.6e},\"poisson_frac\":{poisson_nonzero:.6e},\"frac_over_poisson\":{:.4},\"max\":{mx},\"frob_invariant_sampled\":{frob_ok}}}",
                    ru - 1,
                    mean / e_model,
                    if mean > 0.0 { var / mean } else { 0.0 },
                    if poisson_nonzero > 0.0 { frac_nonzero / poisson_nonzero } else { 0.0 },
                );
            }
            // group-check: extract one m=min(4,mmax) decomposition of a target by
            // search in dlog space and verify it with point additions
            if variant == "distinct" {
                let m = mmax.min(3);
                let v = &cnt[m];
                if let Some(tx) = (1..ru).find(|&x| v[x] > 0) {
                    let found = find_decomp(&base, tx as u64, r, m);
                    let mut acc = Pt::O;
                    for &d in &found {
                        acc = cv.add(&acc, &pts[d as usize]);
                    }
                    assert_eq!(acc, pts[tx], "group check of extracted decomposition failed");
                    eprintln!("group-check ok: n={n} m={m} target dlog {tx} = sum of {:?}", found);
                }
            }
        }
    }
}

fn find_decomp(base: &[u32], t: u64, r: u64, m: usize) -> Vec<u32> {
    fn rec(base: &[u32], start: usize, t: u64, r: u64, m: usize, acc: &mut Vec<u32>) -> bool {
        if m == 0 {
            return t == 0;
        }
        for i in start..base.len() {
            let d = base[i] as u64;
            acc.push(base[i]);
            if rec(base, i + 1, (t + r - d) % r, r, m - 1, acc) {
                return true;
            }
            acc.pop();
        }
        false
    }
    let mut acc = Vec::new();
    assert!(rec(base, 0, t, r, m, &mut acc));
    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn orders_match_panel() {
        // panel README: a0 n13 cofactor 4; a1 n17 cofactor 2; a0 n23 cofactor 4;
        // a1 n29 cofactor 12646; a0 n31 cofactor 1492
        for &(n, a, h) in &[(13u32, 0u8, 4u64), (17, 1, 2), (19, 0, 4), (23, 0, 4), (29, 1, 12646), (31, 0, 1492)] {
            let o = koblitz_order(n, a);
            let r = largest_prime_factor(o);
            assert_eq!(o / r, h, "n={n} a={a}");
        }
    }
    #[test]
    fn moduli_irreducible() {
        for n in [13, 17, 19, 23, 29, 31, 37, 41] {
            check_irreducible(n, &modulus(n));
        }
    }
}
