//! Non-hyperelliptic genus-4 curves over F_2 for the étale screen at deployed sizes.
//!
//! The canonical model of a non-hyperelliptic genus-4 curve is X = Q ∩ C in P^3, with Q the
//! unique quadric through it (smooth hyperbolic, smooth elliptic, or a cone over a conic;
//! one normal form each up to PGL_4(F_2)) and C a cubic determined modulo Q·(linear forms).
//! With x0 x1 the leading monomial of every normal form, the 16 cubic monomials not divisible
//! by x0 x1 span a complement, so each nonzero C there is one model.
//!
//! X is smooth iff no point of X has rank[grad Q; grad C] < 2. Every singular point of such
//! an X has degree <= 6 (an irreducible X has at most 4 singular points; components of a
//! reducible X meet in at most 9 points permuted by Frobenius in orbits of size <= 6), so
//! the test runs over P^3(F_16), P^3(F_32) and P^3(F_64), which contain every degree <= 6.
//! Point counts over F_2, F_4, F_8, F_16 give L_X. The screen then records which X have
//! n | P_X(lambda) for lambda of small order, as in `screen`.

use crate::{lpoly_from_counts, Stats};
use std::collections::BTreeMap;

/// GF(2^k), k <= 6, with log tables.
struct Fq {
    q: u32,
    exp: Vec<u8>,
    log: Vec<u8>,
}
const MODULI: [u32; 7] = [0, 0b11, 0b111, 0b1011, 0b10011, 0b100101, 0b1000011];
impl Fq {
    fn new(k: usize) -> Fq {
        let q = 1u32 << k;
        let m = MODULI[k];
        let raw = |a: u32, b: u32| -> u32 {
            let (mut a, mut b, mut r) = (a, b, 0u32);
            while b != 0 {
                if b & 1 == 1 {
                    r ^= a;
                }
                a <<= 1;
                b >>= 1;
            }
            for s in (0..8).rev() {
                if r >> (s + k) & 1 == 1 {
                    r ^= m << s;
                }
            }
            r
        };
        // a generator: an element whose powers run through all q - 1 units
        let order = (q - 1) as usize;
        for g in 1..q {
            let mut exp = vec![0u8; 2 * order.max(1)];
            let mut log = vec![0u8; q as usize];
            let mut seen = vec![false; q as usize];
            let mut t = 1u32;
            let mut ok = true;
            for i in 0..order {
                if t == 0 || seen[t as usize] {
                    ok = false;
                    break;
                }
                seen[t as usize] = true;
                exp[i] = t as u8;
                exp[i + order] = t as u8;
                log[t as usize] = i as u8;
                t = raw(t, g);
            }
            if ok && t == 1 {
                return Fq { q, exp, log };
            }
        }
        panic!("GF(2^{}) modulus not primitive", k);
    }
    #[inline]
    fn mul(&self, a: u8, b: u8) -> u8 {
        if a == 0 || b == 0 {
            0
        } else {
            self.exp[self.log[a as usize] as usize + self.log[b as usize] as usize]
        }
    }
}

type P4 = [u8; 4];

#[derive(Clone, Copy)]
enum QType {
    Hyperbolic,
    Elliptic,
    Cone,
}
impl QType {
    fn name(&self) -> &'static str {
        match self {
            QType::Hyperbolic => "hyperbolic",
            QType::Elliptic => "elliptic",
            QType::Cone => "cone",
        }
    }
    fn eval(&self, f: &Fq, p: P4) -> u8 {
        let x01 = f.mul(p[0], p[1]);
        match self {
            QType::Hyperbolic => x01 ^ f.mul(p[2], p[3]),
            QType::Elliptic => x01 ^ f.mul(p[2], p[2]) ^ f.mul(p[2], p[3]) ^ f.mul(p[3], p[3]),
            QType::Cone => x01 ^ f.mul(p[2], p[2]),
        }
    }
    fn grad(&self, p: P4) -> P4 {
        match self {
            QType::Hyperbolic | QType::Elliptic => [p[1], p[0], p[3], p[2]],
            QType::Cone => [p[1], p[0], 0, 0],
        }
    }
}

/// Cubic monomials not divisible by x0 x1, as exponent vectors.
fn complement_monomials() -> Vec<[u32; 4]> {
    let mut v = vec![];
    for a in 0..=3u32 {
        for b in 0..=3 - a {
            for c in 0..=3 - a - b {
                let e = [a, b, c, 3 - a - b - c];
                if !(a >= 1 && b >= 1) {
                    v.push(e);
                }
            }
        }
    }
    assert_eq!(v.len(), 16);
    v
}

/// Points of the quadric over F_q (normalised: first nonzero coordinate 1), with the values
/// of the 16 monomials, their 4 partial derivatives, and grad Q.
struct QPoints {
    mono: Vec<[u8; 16]>,
    dmono: Vec<[[u8; 4]; 16]>,
    gq: Vec<P4>,
}
fn quadric_points(f: &Fq, qt: QType, mons: &[[u32; 4]]) -> QPoints {
    let q = f.q as u8;
    let mut pts: Vec<P4> = vec![];
    for a in 0..q {
        for b in 0..q {
            for c in 0..q {
                pts.push([1, a, b, c]);
            }
        }
    }
    for b in 0..q {
        for c in 0..q {
            pts.push([0, 1, b, c]);
        }
    }
    for c in 0..q {
        pts.push([0, 0, 1, c]);
    }
    pts.push([0, 0, 0, 1]);
    let pw = |x: u8, e: u32| (0..e).fold(1u8, |acc, _| f.mul(acc, x));
    let mut out = QPoints {
        mono: vec![],
        dmono: vec![],
        gq: vec![],
    };
    for p in pts {
        if qt.eval(f, p) != 0 {
            continue;
        }
        let mut mv = [0u8; 16];
        let mut dv = [[0u8; 4]; 16];
        for (m, e) in mons.iter().enumerate() {
            mv[m] = (0..4).fold(1u8, |acc, i| f.mul(acc, pw(p[i], e[i])));
            for (i, d) in dv[m].iter_mut().enumerate() {
                // d/dx_i x^e = e_i x^(e - 1_i); in characteristic 2 only odd e_i survive
                if e[i] % 2 == 1 {
                    *d = (0..4).fold(1u8, |acc, j| {
                        f.mul(acc, pw(p[j], if j == i { e[j] - 1 } else { e[j] }))
                    });
                }
            }
        }
        out.mono.push(mv);
        out.dmono.push(dv);
        out.gq.push(qt.grad(p));
    }
    out
}

/// Weil bound on the L-polynomial coefficients: |a_i| <= C(2g, i) 2^(i/2).
fn within_weil(g: usize, lp: &[i64]) -> bool {
    let mut binom = 1f64;
    lp.iter().enumerate().all(|(i, &a)| {
        if i > 0 {
            binom = binom * (2 * g + 1 - i) as f64 / i as f64;
        }
        (a.abs() as f64) <= binom * 2f64.powf(i as f64 / 2.0) + 1e-9
    })
}

/// Power sums p_k = sum of alpha_i^k over the inverse roots of L = sum a_j X^j, k <= kmax:
/// p_k = -k a_k - sum_{i<k} p_i a_{k-i}.
fn power_sums(lp: &[i64], kmax: usize) -> Vec<i64> {
    let a = |j: usize| lp.get(j).copied().unwrap_or(0);
    let mut p = vec![0i64; kmax + 1];
    for k in 1..=kmax {
        p[k] = -(k as i64) * a(k) - (1..k).map(|i| p[i] * a(k - i)).sum::<i64>();
    }
    p
}

/// Runs the screen, prints candidate and summary records, and returns the L-polynomials
/// found with model counts.
pub fn run(targets: &[u64], kmax: u64, st: &mut Stats) -> BTreeMap<Vec<i64>, u64> {
    let fields: Vec<Fq> = (0..=6)
        .map(|k| if k == 0 { Fq::new(1) } else { Fq::new(k) })
        .collect();
    let mons = complement_monomials();
    let units: Vec<Vec<(u64, u64)>> = targets
        .iter()
        .map(|&n| crate::screen::small_order_units(n, kmax))
        .collect();
    let mut cand: BTreeMap<(u64, u64), BTreeMap<Vec<i64>, u64>> = BTreeMap::new();
    let mut lcount: BTreeMap<Vec<i64>, u64> = BTreeMap::new();
    let mut weil_fail = 0u64;
    let mut per_type = vec![];
    for qt in [QType::Hyperbolic, QType::Elliptic, QType::Cone] {
        let pts: Vec<QPoints> = (0..=6)
            .map(|d| {
                if d == 0 {
                    QPoints {
                        mono: vec![],
                        dmono: vec![],
                        gq: vec![],
                    }
                } else {
                    quadric_points(&fields[d], qt, &mons)
                }
            })
            .collect();
        let mut val: Vec<Vec<u8>> = pts.iter().map(|p| vec![0u8; p.mono.len()]).collect();
        let mut grad: Vec<Vec<P4>> = pts.iter().map(|p| vec![[0u8; 4]; p.mono.len()]).collect();
        let mut smooth = 0u64;
        for step in 1u32..(1 << 16) {
            let bit = step.trailing_zeros() as usize;
            for d in 1..=6 {
                for (v, m) in val[d].iter_mut().zip(&pts[d].mono) {
                    *v ^= m[bit];
                }
                if d >= 4 {
                    for (gv, dm) in grad[d].iter_mut().zip(&pts[d].dmono) {
                        for i in 0..4 {
                            gv[i] ^= dm[bit][i];
                        }
                    }
                }
            }
            // singular iff some point of X has rank[grad Q; grad C] < 2 (degrees <= 6)
            let singular = (4..=6).any(|d| {
                let f = &fields[d];
                val[d].iter().enumerate().any(|(k, &v)| {
                    if v != 0 {
                        return false;
                    }
                    let (a, b) = (pts[d].gq[k], grad[d][k]);
                    (0..4).all(|i| (i + 1..4).all(|j| f.mul(a[i], b[j]) == f.mul(a[j], b[i])))
                })
            });
            if singular {
                continue;
            }
            smooth += 1;
            let counts: Vec<i64> = (1..=4)
                .map(|d| val[d].iter().filter(|&&v| v == 0).count() as i64)
                .collect();
            let lp = lpoly_from_counts(4, &counts);
            if !within_weil(4, &lp) {
                weil_fail += 1;
                eprintln!("Weil bound failed {} step={} L={:?}", qt.name(), step, lp);
                continue;
            }
            // out-of-sample: N_5 and N_6 predicted from L (built from N_1..N_4) against the
            // direct counts over F_32 and F_64
            let p = power_sums(&lp, 6);
            for k in [5usize, 6] {
                let predicted = (1i64 << k) + 1 - p[k];
                let counted = val[k].iter().filter(|&&v| v == 0).count() as i64;
                st.nk_checked += 1;
                if predicted != counted {
                    st.nk_fail += 1;
                    eprintln!(
                        "N_{} mismatch {} step={} predicted={} counted={}",
                        k,
                        qt.name(),
                        step,
                        predicted,
                        counted
                    );
                }
            }
            *lcount.entry(lp.clone()).or_default() += 1;
            for (t, &n) in targets.iter().enumerate() {
                let kmin = units[t]
                    .iter()
                    .filter(|&&(l, _)| crate::screen::p_at(&lp, l, n) == 0)
                    .map(|&(_, k)| k)
                    .min();
                if let Some(k) = kmin {
                    *cand
                        .entry((n, k))
                        .or_default()
                        .entry(lp.clone())
                        .or_default() += 1;
                }
            }
        }
        st.curves += smooth;
        per_type.push(format!("\"{}\":{}", qt.name(), smooth));
    }
    crate::screen::print_candidates("canonical_genus4", 4, &cand);
    crate::screen::print_summary(
        "canonical_genus4",
        4,
        st.curves,
        &lcount,
        targets,
        kmax,
        &cand,
        &format!(
            "\"models_by_quadric\":{{{}}},\"weil_fail\":{},\"n5_n6_checked\":{},\"n5_n6_fail\":{},",
            per_type.join(","),
            weil_fail,
            st.nk_checked,
            st.nk_fail
        ),
    );
    lcount
}
