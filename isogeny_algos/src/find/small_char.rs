//! The isogeny between two given l-isogenous curves over F_q, q = p^n, when the characteristic is
//! small (p <= 4l, so that the large-characteristic methods divide by p): the p-adic lift of
//! Joux–Lercier and Lercier–Sirvent (JTNB 20, 2008, §4), on `padic::PadicField`.
//!
//!  1. Lift the curve E arbitrarily to Z_q (digit-wise) and pass to the short model
//!     y^2 = x^3 - c4/48 x - c6/864 over Q_q (x_short = x + b2/12; for p = 2, 3 this divides by p).
//!  2. Lift j~: the root of Phi_l(j(E), Y) above j~ by Newton's iteration (j~ must be a simple
//!     root of Phi_l(j, Y) mod p). Phi_l comes from its integer coefficients
//!     (`modpoly::integer_coeffs`, cached).
//!  3. Elkies' normalised codomain over Q_q (`elkies::elkies_codomain`), then the BMSS series
//!     U = x-map expansion from the differential equation (`bmss::u_from_ode`), and the Padé
//!     approximant giving D = h^2, solved with p-adic pivoting (largest |.|_p first).
//!  4. h(x) = h_short(x + b2/12), reduced mod p, then verified in F_q by the caller (division
//!     polynomial, kernel test, codomain j-invariant). At too low a precision the reduction is
//!     wrong or impossible; the precision is raised and the lift redone (Las Vegas).
//!
//! Lercier–Sirvent bound the loss of precision by O(log^2 l / log p) digits for p >= 5. In
//! characteristic 2 and 3 the same route loses digits linearly in l (Caruso–Eid–Lercier 2020,
//! introduction), so the working precision there is capped by p^N < 2^127 (N <= 126 for p = 2,
//! N <= 80 for p = 3): the degree reachable is limited accordingly. Wrappers for GF(2^n)
//! (`binary`), GF(3^n) (`gf3n`), F_p and F_{p^r} (`fpr`) are at the end.
use crate::binary::BinCurve;
use crate::curve::{jinv, Curve};
use crate::field::{Field, Rng};
use crate::find::bmss;
use crate::find::elkies::elkies_codomain;
use crate::find::modpoly::{integer_coeffs, Phi};
use crate::gf2n::GF2n;
use crate::gf3n::GF3n;
use crate::int::Int;
use crate::padic::{max_precision, PadicField, Qq};
use crate::poly::{self, Poly};
use crate::weier::{self, GWCurve};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// Integer coefficients of Phi_l, computed once per l.
pub fn phi_integer(ell: usize) -> Arc<Vec<Vec<Int>>> {
    static CACHE: OnceLock<Mutex<HashMap<usize, Arc<Vec<Vec<Int>>>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(c) = cache.lock().unwrap().get(&ell) {
        return c.clone();
    }
    let c = Arc::new(integer_coeffs(ell));
    cache.lock().unwrap().insert(ell, c.clone());
    c
}

/// F_q = F_p[t]/(modulus), elements given by their p-adic digits (coefficients of t^i).
#[derive(Clone, Debug)]
pub struct Residue {
    pub p: u64,
    /// monic, lowest coefficient first
    pub modulus: Vec<u64>,
}

/// Result of a successful lift.
#[derive(Clone, Debug)]
pub struct LiftOutcome {
    /// monic kernel polynomial, coefficients as residue digit vectors, lowest first
    pub kernel: Vec<Vec<u64>>,
    /// p-adic precision of the successful attempt
    pub precision: u32,
    /// attempts made (precisions tried)
    pub attempts: usize,
}

/// Precisions to try: the Lercier–Sirvent estimate (plus l for p <= 3), then larger, up to the
/// cap p^N < 2^127.
fn schedule(p: u64, ell: usize) -> Vec<u32> {
    let nmax = max_precision(p);
    let lg = ((4 * ell) as f64).log2() + 1.0;
    let mut est = 12.0 + lg * lg / (p as f64).log2();
    if p <= 3 {
        est += ell as f64;
    }
    let mut out = vec![];
    let mut n = (est.ceil() as u32).min(nmax);
    loop {
        out.push(n);
        if n == nmax {
            break;
        }
        n = ((n as f64 * 1.5).ceil() as u32).min(nmax);
    }
    out
}

/// Gaussian elimination over Q_q, pivoting on the entry of least valuation in the column.
fn solve_padic(f: &PadicField, mut m: Vec<Vec<Qq>>, mut b: Vec<Qq>) -> Option<Vec<Qq>> {
    let n = b.len();
    for col in 0..n {
        let pr = (col..n)
            .filter(|&r| !f.is_zero(m[r][col]))
            .min_by_key(|&r| f.valuation(m[r][col]))?;
        m.swap(col, pr);
        b.swap(col, pr);
        let inv = f.inv(m[col][col]);
        for c in col..n {
            m[col][c] = f.mul(m[col][c], inv);
        }
        b[col] = f.mul(b[col], inv);
        for r in 0..n {
            if r != col && !f.is_zero(m[r][col]) {
                let fct = m[r][col];
                for c in col..n {
                    let t = f.mul(fct, m[col][c]);
                    m[r][c] = f.sub(m[r][c], t);
                }
                let t = f.mul(fct, b[col]);
                b[r] = f.sub(b[r], t);
            }
        }
    }
    Some(b)
}

/// One attempt at precision `prec`: the kernel polynomial reduced mod p, or None.
fn attempt(
    res: &Residue,
    a: &[Vec<u64>; 5],
    jt: &[u64],
    ell: usize,
    prec: u32,
    coeffs: &[Vec<Int>],
) -> Option<Vec<Vec<u64>>> {
    let f = PadicField::new(res.p, &res.modulus, prec);
    let c = |v: u64| f.from_u64(v);
    let [a1, a2, a3, a4, a6] = [0, 1, 2, 3, 4].map(|i| f.from_residue(&a[i]));
    let b2 = f.add(f.sq(a1), f.mul(c(4), a2));
    let b4 = f.add(f.mul(c(2), a4), f.mul(a1, a3));
    let b6 = f.add(f.sq(a3), f.mul(c(4), a6));
    let c4 = f.sub(f.sq(b2), f.mul(c(24), b4));
    let c6 = f.sub(
        f.sub(f.mul(c(36), f.mul(b2, b4)), f.mul(b2, f.sq(b2))),
        f.mul(c(216), b6),
    );
    let e = Curve::new(f.neg(f.div(c4, c(48))), f.neg(f.div(c6, c(864))));
    let j = jinv(&f, &e);
    let phi = Phi {
        ell,
        c: coeffs
            .iter()
            .map(|row| row.iter().map(|x| f.from_int(x)).collect())
            .collect(),
    };
    // Newton on Y -> Phi(j, Y) from the lift of j~
    let mut y = f.from_residue(jt);
    if f.valuation(phi.grad(&f, j, y).1) != 0 {
        return None; // j~ is not a simple root of Phi_l(j, Y) mod p
    }
    for _ in 0..2 * (32 - prec.leading_zeros()) + 4 {
        let v = phi.eval(&f, j, y);
        if f.is_zero(v) {
            break;
        }
        let dy = phi.grad(&f, j, y).1;
        y = f.sub(y, f.div(v, dy));
    }
    let et = elkies_codomain(&f, &phi, &e, y)?;
    // N/D = x U(1/x); U = Nh/Dh with Dh(0) = 1, deg Dh <= l - 1 (as in BMSS fastElkies')
    let mlen = 2 * ell + 4;
    let u = bmss::u_from_ode(&f, &e, &et, mlen);
    let r = ell - 1;
    let mut mat = vec![vec![f.zero(); r]; r];
    let mut rhs = vec![f.zero(); r];
    for k in ell + 1..2 * ell {
        for i in 1..=r {
            mat[k - ell - 1][i - 1] = u[k - i];
        }
        rhs[k - ell - 1] = f.neg(u[k]);
    }
    let sol = solve_padic(&f, mat, rhs)?;
    // D(x) = x^{l-1} Dh(1/x), monic of degree l - 1 = 2d; h = sqrt(D) term by term
    let d = (ell - 1) / 2;
    let mut dd = vec![f.zero(); ell];
    dd[ell - 1] = f.one();
    for i in 1..=r {
        dd[ell - 1 - i] = sol[i - 1];
    }
    let mut g = vec![f.zero(); d + 1];
    g[d] = f.one();
    let inv2 = f.inv(c(2));
    for k in 1..=d {
        let mut s = dd[2 * d - k];
        for i in 1..k {
            s = f.sub(s, f.mul(g[d - i], g[d - k + i]));
        }
        g[d - k] = f.mul(s, inv2);
    }
    // back to the original abscissa: h(x) = g(x + b2/12) (Horner on the shift)
    let s = f.div(b2, c(12));
    let mut h: Vec<Qq> = vec![];
    for &gk in g.iter().rev() {
        // h <- h (x + s) + gk
        let mut nh = vec![f.zero(); h.len() + 1];
        for (i, &hi) in h.iter().enumerate() {
            nh[i + 1] = f.add(nh[i + 1], hi);
            nh[i] = f.add(nh[i], f.mul(hi, s));
        }
        nh[0] = f.add(nh[0], gk);
        h = nh;
    }
    h.iter().map(|&x| f.to_residue(x)).collect()
}

/// The kernel polynomial of the l-isogeny from E = [a1, a2, a3, a4, a6] (residue digits) to a
/// curve of j-invariant `jt`, by the p-adic lift; `verify` checks a candidate in F_q.
pub fn lift_kernel(
    res: &Residue,
    a: &[Vec<u64>; 5],
    jt: &[u64],
    ell: usize,
    verify: &mut dyn FnMut(&[Vec<u64>]) -> bool,
) -> Option<LiftOutcome> {
    if ell < 3 || ell.is_multiple_of(2) || res.p as usize == ell {
        return None;
    }
    let coeffs = phi_integer(ell);
    for (i, prec) in schedule(res.p, ell).into_iter().enumerate() {
        let attempts = i + 1;
        if let Some(h) = attempt(res, a, jt, ell, prec, &coeffs) {
            if verify(&h) {
                return Some(LiftOutcome {
                    kernel: h,
                    precision: prec,
                    attempts,
                });
            }
        }
    }
    None
}

// ------------------------------------------------------------------ wrappers

fn bits(x: u64, n: usize) -> Vec<u64> {
    (0..n).map(|i| (x >> i) & 1).collect()
}

fn unbits(d: &[u64]) -> u64 {
    d.iter()
        .enumerate()
        .fold(0, |acc, (i, &b)| acc | (b & 1) << i)
}

fn trits(mut x: u64, n: usize) -> Vec<u64> {
    (0..n)
        .map(|_| {
            let d = x % 3;
            x /= 3;
            d
        })
        .collect()
}

fn untrits(d: &[u64]) -> u64 {
    d.iter().rev().fold(0, |acc, &t| acc * 3 + t % 3)
}

/// GF(2^n): the kernel polynomial of the l-isogeny from the binary curve e to a curve of
/// j-invariant jt (l odd prime), with the outcome; verified by the l-division polynomial, the
/// doubling test (`binary::is_kernel`) and the codomain's j-invariant.
pub fn binary_kernel(
    f: &GF2n,
    e: &BinCurve,
    jt: u64,
    ell: usize,
    rng: &mut Rng,
) -> Option<(Poly<GF2n>, LiftOutcome)> {
    let n = f.n as usize;
    let mut modulus = bits(f.red, n);
    modulus.push(1);
    let res = Residue { p: 2, modulus };
    let a = [
        vec![1],
        bits(e.a2, n),
        vec![0],
        bits(e.a4, n),
        bits(e.a6, n),
    ]
    .map(|v| {
        if v.len() < n {
            [v.clone(), vec![0; n - v.len()]].concat()
        } else {
            v
        }
    });
    let psi = crate::binary::division_poly(f, &e.normalized(f), ell);
    let mut verify = |h: &[Vec<u64>]| -> bool {
        let hp: Poly<GF2n> = h.iter().map(|d| unbits(d)).collect();
        poly::rem(f, &psi, &hp).iter().all(|&c| c == 0)
            && crate::binary::kohel_codomain(f, e, &hp).j(f) == jt
            && crate::binary::is_kernel(f, e, &hp, rng)
    };
    let out = lift_kernel(&res, &a, &bits(jt, n), ell, &mut verify)?;
    let h = out.kernel.iter().map(|d| unbits(d)).collect();
    Some((h, out))
}

/// GF(3^n): the same for a general Weierstrass curve over GF(3^n) (elements packed base 3).
pub fn ternary_kernel(
    f: &GF3n,
    e: &GWCurve<u64>,
    jt: u64,
    ell: usize,
    rng: &mut Rng,
) -> Option<(Poly<GF3n>, LiftOutcome)> {
    let n = f.n as usize;
    let res = Residue {
        p: 3,
        modulus: f.modulus(),
    };
    let a = [e.a1, e.a2, e.a3, e.a4, e.a6].map(|x| trits(x, n));
    let xs = weier::random_abscissas(f, e, 6, rng);
    let mut verify = |h: &[Vec<u64>]| -> bool {
        let hp: Poly<GF3n> = h.iter().map(|d| untrits(d)).collect();
        weier::kohel_codomain(f, e, &hp).j(f) == jt && weier::is_kernel(f, e, &hp, &xs)
    };
    let out = lift_kernel(&res, &a, &trits(jt, n), ell, &mut verify)?;
    let h = out.kernel.iter().map(|d| untrits(d)).collect();
    Some((h, out))
}

/// Any field whose elements are given by residue digits over F_p (F_p itself, F_{p^r} through
/// `fpr::FpR`): the caller supplies the conversions.
pub fn generic_kernel<F: Field>(
    f: &F,
    res: &Residue,
    e: &GWCurve<F::E>,
    jt: F::E,
    ell: usize,
    to_digits: impl Fn(F::E) -> Vec<u64>,
    from_digits: impl Fn(&[u64]) -> F::E,
    rng: &mut Rng,
) -> Option<(Poly<F>, LiftOutcome)> {
    let a = [e.a1, e.a2, e.a3, e.a4, e.a6].map(&to_digits);
    let xs = weier::random_abscissas(f, e, 6, rng);
    let mut verify = |h: &[Vec<u64>]| -> bool {
        let hp: Poly<F> = h.iter().map(|d| from_digits(d)).collect();
        weier::kohel_codomain(f, e, &hp).j(f) == jt && weier::is_kernel(f, e, &hp, &xs)
    };
    let out = lift_kernel(res, &a, &to_digits(jt), ell, &mut verify)?;
    let h = out.kernel.iter().map(|d| from_digits(d)).collect();
    Some((h, out))
}
