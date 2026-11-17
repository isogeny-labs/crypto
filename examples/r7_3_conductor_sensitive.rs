//! Round 7, lead R7.3: conductor-sensitive algorithms across an ℓ-volcano
//! crater/floor gap (a second toy test of Conjecture 10′).
//!
//! R6.11 measured only the plain x-frame index-calculus base.  This example
//! implements algorithms whose cost can depend on `End(E)`:
//!
//! * **Exp A (automorphism-class rho; the baseline control).**  Pollard rho
//!   with a canonical representative of the orbit under `Aut(E)`: `{±1}`
//!   (`pm`) or the full `Aut(E)` (`aut`, the ζ-fold: order 6 at `j = 0`,
//!   order 4 at `j = 1728`).  Craters at `D = −3, −4` against their
//!   `|Aut| = 2` floors.  The `aut/pm` ratio *on the crater* is the known
//!   automorphism-group effect; the `pm` crater/floor ratio is the
//!   conductor effect with that baseline removed.
//! * **Exp B (endomorphism-mixed rho, the GLV-type walk).**  The step table
//!   of an r-adding walk gets 2 of 16 slots that apply an explicit
//!   endomorphism `ε` (a Vélu-evaluated rational map, not a scalar
//!   multiplication).  Crater (`h = 1`, `D ∈ {−11, −19, −43, −67, −163}`):
//!   `ε = ω = (1+√D)/2` of degree `(1+|D|)/4`, built from the π-eigenline
//!   of `E[(1+|D|)/4]`.  Floor: `ε = ψ ∘ ω ∘ φ` (`φ`, `ψ` the vertical
//!   ℓ-isogenies), degree ℓ²·(1+|D|)/4, i.e. the cheapest non-scalar
//!   endomorphism of `Z + ℓO_K`.  Iterations and counted `F_p`
//!   multiplications are recorded separately.
//! * **GLV lattice.**  The reduced basis of `{(a,b): a + bμ ≡ 0 mod r}` for
//!   the measured eigenvalue μ of ε, and the decomposition length.
//! * **Exp C (transport).**  The census of a base transported along the
//!   vertical isogeny against the source base (an exact identity), and the
//!   DLP-transport check with its cost.
//!
//! ```text
//! r7_3_conductor_sensitive expA --ell 5 --d 3 --instances 6 --seed 1 --runs 400 --out DIR
//! r7_3_conductor_sensitive expB --ell 5 --instances 8 --seed 1 --runs 300 --out DIR
//! r7_3_conductor_sensitive summarise --dir DIR --out summary.json
//! ```

use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

use crypto_lib::cryptanalysis::isogeny_walk::curve::{self, Model};
use crypto_lib::cryptanalysis::isogeny_walk::field::Field as LField;
use crypto_lib::cryptanalysis::isogeny_walk::{kernel, modpoly, poly as lpoly};
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

// ---------------------------------------------------------------------------
// F_p, p < 2^32, with a global multiplication counter.

thread_local! {
    static MULS: Cell<u64> = const { Cell::new(0) };
}

fn mcount() -> u64 {
    MULS.with(|c| c.get())
}

#[derive(Clone, Copy, Debug)]
struct Fp {
    p: u64,
}

impl Fp {
    fn red_i(&self, v: i128) -> u64 {
        v.rem_euclid(self.p as i128) as u64
    }
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    #[inline]
    fn mul(&self, a: u64, b: u64) -> u64 {
        MULS.with(|c| c.set(c.get() + 1));
        a * b % self.p
    }
    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of zero");
        self.pow(a, self.p - 2)
    }
    fn legendre(&self, a: u64) -> i32 {
        if a == 0 {
            return 0;
        }
        if self.pow(a, (self.p - 1) / 2) == 1 {
            1
        } else {
            -1
        }
    }
    fn sqrt(&self, a: u64) -> Option<u64> {
        if a == 0 {
            return Some(0);
        }
        if self.legendre(a) != 1 {
            return None;
        }
        let p = self.p;
        let mut q = p - 1;
        let mut s = 0;
        while q.is_multiple_of(2) {
            q /= 2;
            s += 1;
        }
        if s == 1 {
            return Some(self.pow(a, p.div_ceil(4)));
        }
        let z = (2..p).find(|&z| self.legendre(z) == -1).unwrap();
        let mut m = s;
        let mut c = self.pow(z, q);
        let mut t = self.pow(a, q);
        let mut r = self.pow(a, q.div_ceil(2));
        while t != 1 {
            let mut i = 0;
            let mut tt = t;
            while tt != 1 {
                tt = self.mul(tt, tt);
                i += 1;
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = self.mul(b, b);
            }
            m = i;
            c = self.mul(b, b);
            t = self.mul(t, c);
            r = self.mul(r, b);
        }
        Some(r)
    }
    fn least_nonresidue(&self) -> u64 {
        (2..self.p).find(|&d| self.legendre(d) == -1).unwrap()
    }
}

// ---------------------------------------------------------------------------
// Short Weierstrass curve, affine points.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pt {
    x: u64,
    y: u64,
}

#[derive(Clone, Copy, Debug)]
struct Curve {
    f: Fp,
    a: u64,
    b: u64,
}

impl Curve {
    fn rhs(&self, x: u64) -> u64 {
        let f = &self.f;
        f.add(f.mul(f.add(f.mul(x, x), self.a), x), self.b)
    }
    fn on(&self, q: &Pt) -> bool {
        self.f.mul(q.y, q.y) == self.rhs(q.x)
    }
    fn neg(&self, q: &Pt) -> Pt {
        Pt {
            x: q.x,
            y: self.f.neg(q.y),
        }
    }
    fn add(&self, p: Option<Pt>, q: Option<Pt>) -> Option<Pt> {
        let f = &self.f;
        let (p, q) = match (p, q) {
            (None, q) => return q,
            (p, None) => return p,
            (Some(p), Some(q)) => (p, q),
        };
        let lam = if p.x == q.x {
            if f.add(p.y, q.y) == 0 {
                return None;
            }
            let num = f.add(f.mul(3, f.mul(p.x, p.x)), self.a);
            f.mul(num, f.inv(f.mul(2, p.y)))
        } else {
            f.mul(f.sub(q.y, p.y), f.inv(f.sub(q.x, p.x)))
        };
        let x3 = f.sub(f.sub(f.mul(lam, lam), p.x), q.x);
        let y3 = f.sub(f.mul(lam, f.sub(p.x, x3)), p.y);
        Some(Pt { x: x3, y: y3 })
    }
    fn mul(&self, p: &Pt, mut k: u64) -> Option<Pt> {
        let mut acc: Option<Pt> = None;
        let mut base = Some(*p);
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(acc, base);
            }
            base = self.add(base, base);
            k >>= 1;
        }
        acc
    }
    fn j(&self) -> u64 {
        let f = &self.f;
        let a3 = f.mul(f.mul(self.a, self.a), self.a);
        let disc = f.add(f.mul(4, a3), f.mul(27, f.mul(self.b, self.b)));
        f.mul(f.mul(1728 * 4 % f.p, a3), f.inv(disc))
    }
    fn key(q: &Pt) -> u64 {
        (q.x << 32) | q.y
    }
    fn random_point(&self, rng: &mut StdRng) -> Pt {
        loop {
            let x = rng.gen_range(0..self.f.p);
            let r = self.rhs(x);
            if r == 0 {
                continue;
            }
            if let Some(y) = self.f.sqrt(r) {
                return Pt { x, y };
            }
        }
    }
    fn model(&self, lf: &LField) -> Model {
        Model {
            a: lf.from_u64(self.a),
            b: lf.from_u64(self.b),
        }
    }
}

fn fe_to_u64(lf: &LField, x: &crypto_lib::cryptanalysis::isogeny_walk::field::Fe) -> u64 {
    lf.to_big(x).to_u64().unwrap()
}

fn gcd_u(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd_u(b, a % b)
    }
}

fn mulm(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn invm(a: u64, m: u64) -> u64 {
    // extended Euclid
    let (mut r0, mut r1) = (m as i128, (a % m) as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    assert_eq!(r0, 1, "not invertible");
    t0.rem_euclid(m as i128) as u64
}

// ---------------------------------------------------------------------------
// Instances.

const CM_J: [(u64, i128); 5] = [
    (11, -32768),
    (19, -884736),
    (43, -884736000),
    (67, -147197952000),
    (163, -262537412640768000),
];

/// The generic ordering of one instance.
#[derive(Clone, Debug)]
struct Inst {
    ell: u64,
    /// `|D|`.
    d: u64,
    j0: u64,
    p: u64,
    t: i64,
    v: u64,
    n: u64,
    /// Cofactor (2 at `j = 1728`, else 1) and prime subgroup order.
    cof: u64,
    r: u64,
    kron: i32,
}

fn n_omega(d: u64) -> u64 {
    (1 + d) / 4
}

fn find_instance(ell: u64, d: u64, lo: u64, hi: u64, rng: &mut StdRng) -> (Inst, u64) {
    let mut tries = 0u64;
    loop {
        tries += 1;
        let wmax = (((4 * hi as u128 / d as u128) as f64).sqrt() as u64 / ell).max(1);
        let w = rng.gen_range(1..=wmax);
        if w % 2 == 0 || w % ell == 0 {
            continue;
        }
        let v = ell * w;
        let sv = d as u128 * (v as u128) * (v as u128);
        if d == 4 {
            // p = s² + v², t = 2s with s even, N = (s−1)² + v² = 2r.
            let vv = v * v;
            if vv >= hi {
                continue;
            }
            let smax = ((hi - 1 - vv) as f64).sqrt() as u64;
            let smin = if vv >= lo {
                0
            } else {
                ((lo - vv) as f64).sqrt().ceil() as u64
            };
            if smin > smax {
                continue;
            }
            let s = rng.gen_range(smin..=smax);
            if s % 2 == 1 {
                continue;
            }
            let p = s * s + vv;
            if p < lo || p >= hi || !is_prime_u64(p) {
                continue;
            }
            if s % ell == 1 % ell {
                continue;
            }
            let n = ((s as i64 - 1) * (s as i64 - 1)) as u64 + vv;
            if !n.is_multiple_of(2) || (n / 2).is_multiple_of(2) || !is_prime_u64(n / 2) {
                continue;
            }
            let r = n / 2;
            let kron = Fp { p: ell }.legendre(Fp { p: ell }.red_i(-4));
            return (
                Inst {
                    ell,
                    d,
                    j0: 1728 % p,
                    p,
                    t: 2 * s as i64,
                    v,
                    n,
                    cof: 2,
                    r,
                    kron,
                },
                tries,
            );
        }
        let four_hi = 4 * hi as u128;
        if sv >= four_hi {
            continue;
        }
        let tmax = ((four_hi - 1 - sv) as f64).sqrt() as u64;
        let four_lo = 4 * lo as u128;
        let tmin = if sv >= four_lo {
            1
        } else {
            ((four_lo - sv) as f64).sqrt().ceil() as u64
        };
        if tmin > tmax {
            continue;
        }
        let t = rng.gen_range(tmin..=tmax);
        if t % 2 == 0 || t % ell == 2 % ell {
            continue;
        }
        let four_p = (t as u128) * (t as u128) + sv;
        if !four_p.is_multiple_of(4) {
            continue;
        }
        let p = (four_p / 4) as u64;
        if p < lo || p >= hi || !is_prime_u64(p) {
            continue;
        }
        let n = p + 1 - t;
        // d = 3: N prime.  Otherwise N = n_ω · r with r prime: the unique
        // π-eigenline of E[n_ω] forces n_ω | N when n_ω ∤ v.
        let (cof, r) = if d == 3 {
            if !is_prime_u64(n) {
                continue;
            }
            (1, n)
        } else {
            let no = n_omega(d);
            // t ≢ 0 (mod n_ω): the two rational lines have eigenvalues μ ≠ ±μ̄, so
            // the x-coordinate condition below selects exactly one of them.
            if t % no == 0 || !n.is_multiple_of(no) || !is_prime_u64(n / no) || n / no == no {
                continue;
            }
            (no, n / no)
        };
        let f = Fp { p };
        let j0 = if d == 3 {
            0
        } else {
            let (_, j_int) = CM_J.iter().find(|(dd, _)| *dd == d).expect("known D");
            if gcd_u(v, n_omega(d)) != 1 {
                continue;
            }
            let j = f.red_i(*j_int);
            if j == 0 || j == 1728 % p {
                continue;
            }
            j
        };
        let kron = Fp { p: ell }.legendre(Fp { p: ell }.red_i(-(d as i128)));
        assert!(!n.is_multiple_of(ell));
        assert!(!r.is_multiple_of(ell));
        return (
            Inst {
                ell,
                d,
                j0,
                p,
                t: t as i64,
                v,
                n,
                cof,
                r,
                kron,
            },
            tries,
        );
    }
}

fn order_ok(cv: &Curve, n: u64, checks: usize, rng: &mut StdRng) -> bool {
    (0..checks).all(|_| {
        let q = cv.random_point(rng);
        cv.mul(&q, n).is_none()
    })
}

fn curve_from_j(f: Fp, j: u64, n: u64, rng: &mut StdRng) -> Option<(Curve, bool)> {
    let k = f.sub(1728 % f.p, j);
    let a0 = f.mul(3, f.mul(j, k));
    let b0 = f.mul(2, f.mul(j, f.mul(k, k)));
    let d = f.least_nonresidue();
    for twisted in [false, true] {
        let (a, b) = if twisted {
            (f.mul(a0, f.mul(d, d)), f.mul(b0, f.mul(d, f.mul(d, d))))
        } else {
            (a0, b0)
        };
        let cv = Curve { f, a, b };
        assert_eq!(cv.j(), j);
        if order_ok(&cv, n, 8, rng) {
            return Some((cv, twisted));
        }
    }
    None
}

/// The crater curve with `#E = N` (any twist at `j = 0, 1728`).
fn crater_curve(f: Fp, inst: &Inst, rng: &mut StdRng) -> Option<(Curve, bool)> {
    match inst.d {
        3 => (1..200u64).find_map(|b| {
            let cv = Curve { f, a: 0, b };
            order_ok(&cv, inst.n, 4, rng).then_some((cv, false))
        }),
        4 => (1..200u64).find_map(|a| {
            let cv = Curve { f, a, b: 0 };
            order_ok(&cv, inst.n, 4, rng).then_some((cv, false))
        }),
        _ => curve_from_j(f, inst.j0, inst.n, rng),
    }
}

fn phi_roots(lf: &LField, phi: &modpoly::ModPoly, j: u64) -> Vec<u64> {
    let py = phi.at_x(lf, &lf.from_u64(j));
    let mut r: Vec<u64> = lpoly::roots(lf, &py, 0x7301)
        .iter()
        .map(|x| fe_to_u64(lf, x))
        .collect();
    r.sort_unstable();
    r
}

// ---------------------------------------------------------------------------
// Vélu evaluation of an odd-degree isogeny from its kernel polynomial.

#[derive(Clone, Debug)]
struct VeluMap {
    a: u64,
    b: u64,
    /// Monic kernel polynomial, low to high.
    h: Vec<u64>,
    /// Isomorphism from Vélu's codomain to the target: (x, y) ↦ (u2·x, u3·y).
    u2: u64,
    u3: u64,
    /// Target model (for the on-curve assertions).
    ta: u64,
    tb: u64,
}

impl VeluMap {
    /// `src` the source model, `h` the kernel polynomial (library `Poly`),
    /// `target` the model the codomain is F_p-isomorphic to.
    fn new(
        lf: &LField,
        fp: &Fp,
        src: &Model,
        h: &lpoly::Poly,
        target: &Model,
    ) -> Result<Self, String> {
        let codomain = kernel::velu_codomain(lf, src, h);
        let s = curve::isomorphism(lf, &codomain, target)
            .ok_or_else(|| "codomain not F_p-isomorphic to the target model".to_string())?;
        let u2 = fe_to_u64(lf, &s);
        let u = fp
            .sqrt(u2)
            .ok_or_else(|| "isomorphism scale is not a square".to_string())?;
        let u3 = fp.mul(u2, u);
        Ok(VeluMap {
            a: fe_to_u64(lf, &src.a),
            b: fe_to_u64(lf, &src.b),
            h: h.iter().map(|c| fe_to_u64(lf, c)).collect(),
            u2,
            u3,
            ta: fe_to_u64(lf, &target.a),
            tb: fe_to_u64(lf, &target.b),
        })
    }

    /// The image of the affine point `(x, y)`; `None` if `x` is a kernel
    /// abscissa (the image is the point at infinity).
    fn eval(&self, f: &Fp, pt: Pt) -> Option<Pt> {
        let d = self.h.len() - 1;
        let x = pt.x;
        // Taylor coefficients of h at x: t0 = h, t1 = h', t2 = h''/2, t3 = h'''/6.
        let (mut t0, mut t1, mut t2, mut t3) = (0u64, 0u64, 0u64, 0u64);
        for &c in self.h.iter().rev() {
            t3 = f.add(f.mul(t3, x), t2);
            t2 = f.add(f.mul(t2, x), t1);
            t1 = f.add(f.mul(t1, x), t0);
            t0 = f.add(f.mul(t0, x), c);
        }
        if t0 == 0 {
            return None;
        }
        let inv = f.inv(t0);
        let half = f.p.div_ceil(2);
        let r = f.mul(t1, inv); // S1
        let c1 = f.mul(f.mul(2, t2), inv); // h''/h
        let s2 = f.sub(f.mul(r, r), c1);
        let h3 = f.mul(6, t3);
        let r3 = f.mul(r, f.mul(r, r));
        let s3 = f.mul(
            half,
            f.add(f.sub(f.mul(h3, inv), f.mul(3, f.mul(r, c1))), f.mul(2, r3)),
        );
        let x2 = f.mul(x, x);
        let fx = f.add(f.mul(x, f.add(x2, self.a)), self.b);
        let a6 = f.add(f.mul(6, x2), f.mul(2, self.a));
        let a12 = f.add(f.mul(12, x2), f.mul(4, self.a));
        let p1 = f.neg(self.h[d - 1]);
        let dd = d as u64 % f.p;
        let sm1 = f.sub(f.mul(dd, x), p1);
        let x12 = f.mul(12, x);
        // X = x + Σ t/s + Σ u/s²
        let big_t1 = f.add(f.sub(f.mul(a6, r), f.mul(x12, dd)), f.mul(6, sm1));
        let big_u2 = f.sub(
            f.add(
                f.sub(f.mul(f.mul(4, fx), s2), f.mul(a12, r)),
                f.mul(x12, dd),
            ),
            f.mul(4, sm1),
        );
        let xx = f.add(x, f.add(big_t1, big_u2));
        // X' = 1 − Σ t/s² − 2 Σ u/s³
        let big_t2 = f.add(f.sub(f.mul(a6, s2), f.mul(x12, r)), f.mul(6, dd));
        let big_u3 = f.mul(
            2,
            f.sub(
                f.add(
                    f.sub(f.mul(f.mul(4, fx), s3), f.mul(a12, s2)),
                    f.mul(x12, r),
                ),
                f.mul(4, dd),
            ),
        );
        let dx = f.sub(f.sub(1, big_t2), big_u3);
        let yy = f.mul(pt.y, dx);
        let out = Pt {
            x: f.mul(self.u2, xx),
            y: f.mul(self.u3, yy),
        };
        debug_assert!({
            let tc = Curve {
                f: *f,
                a: self.ta,
                b: self.tb,
            };
            tc.on(&out)
        });
        Some(out)
    }
}

impl VeluMap {
    /// Correctness gate: images lie on the target curve and the map is a
    /// homomorphism, on `n` random points of `src`.
    fn verify(&self, f: &Fp, src: &Curve, n: usize, rng: &mut StdRng) {
        let tc = Curve {
            f: *f,
            a: self.ta,
            b: self.tb,
        };
        for _ in 0..n {
            let p = src.random_point(rng);
            let q = src.random_point(rng);
            let (ip, iq) = (self.eval(f, p).unwrap(), self.eval(f, q).unwrap());
            assert!(tc.on(&ip) && tc.on(&iq), "Vélu image off the target curve");
            if let Some(s) = src.add(Some(p), Some(q)) {
                assert_eq!(
                    self.eval(f, s),
                    tc.add(Some(ip), Some(iq)),
                    "Vélu map is not a homomorphism"
                );
            }
        }
    }
}

/// A chain of Vélu maps: an explicit isogeny or endomorphism.
#[derive(Clone, Debug)]
struct Chain {
    maps: Vec<VeluMap>,
}

impl Chain {
    fn eval(&self, f: &Fp, pt: Pt) -> Option<Pt> {
        let mut cur = pt;
        for m in &self.maps {
            cur = m.eval(f, cur)?;
        }
        Some(cur)
    }
}

/// The kernel polynomial of the π-eigenline `{P ∈ E[n] : π P = μ P}`:
/// `gcd(ψ_n, (x^p − x(μP)) · ψ_μ²-numerator)`.
fn eigenline_kernel(lf: &LField, m0: &Model, n: u64, mu: u64) -> Result<lpoly::Poly, String> {
    let nn = n as usize;
    // m = x^(n²+8) + 1: larger than every intermediate degree, so no reduction.
    let mut big = vec![lf.zero(); nn * nn + 9];
    big[0] = lf.one();
    big[nn * nn + 8] = lf.one();
    let f = kernel::division_polys_mod(lf, m0, nn + 1, &big);
    let fnn = lpoly::monic(lf, &f[nn]);
    let xp = lpoly::powmod(lf, &lpoly::x(lf), lf.modulus(), &fnn);
    let xm = lpoly::sub(lf, &xp, &lpoly::x(lf));
    let mu = mu as usize;
    let fm = &f[mu];
    let prod = lpoly::mul(lf, &f[mu - 1], &f[mu + 1]);
    let cubic = lpoly::trim(lf, vec![m0.b, m0.a, lf.zero(), lf.one()]);
    let g = if mu % 2 == 1 {
        // (x^p − x) f_μ² + 4 F f_{μ−1} f_{μ+1}
        let t1 = lpoly::mul(lf, &xm, &lpoly::mul(lf, fm, fm));
        let t2 = lpoly::scale(lf, &lpoly::mul(lf, &cubic, &prod), &lf.from_u64(4));
        lpoly::add(lf, &t1, &t2)
    } else {
        // 4 F (x^p − x) f_μ² + f_{μ−1} f_{μ+1}
        let t1 = lpoly::scale(
            lf,
            &lpoly::mul(lf, &cubic, &lpoly::mul(lf, &xm, &lpoly::mul(lf, fm, fm))),
            &lf.from_u64(4),
        );
        lpoly::add(lf, &t1, &prod)
    };
    let g = lpoly::rem(lf, &g, &fnn);
    let h = lpoly::monic(lf, &lpoly::gcd(lf, &fnn, &g));
    let deg = lpoly::degree(&h).unwrap_or(0);
    if deg != ((n - 1) / 2) as usize {
        return Err(format!(
            "eigenline kernel has degree {deg}, expected {}",
            (n - 1) / 2
        ));
    }
    kernel::verify_kernel(lf, m0, n, &h).map_err(|e| format!("verify_kernel: {e:?}"))?;
    Ok(h)
}

// ---------------------------------------------------------------------------
// Group data, automorphisms, endomorphism eigenvalues.

#[derive(Clone, Copy, Debug)]
struct AutEl {
    bx: u64,
    iy: u64,
    rho: u64,
}

struct Cx {
    cv: Curve,
    r: u64,
    p: Pt,
    q: Pt,
    ptab: Vec<Option<Pt>>,
    qtab: Vec<Option<Pt>>,
    /// `{±1}`.
    pm: Vec<AutEl>,
    /// Full `Aut(E)`.
    full: Vec<AutEl>,
    /// An explicit endomorphism and its eigenvalue on `⟨P⟩`.
    endo: Option<(Chain, u64)>,
}

fn pow_tab(cv: &Curve, p: &Pt, bits: usize) -> Vec<Option<Pt>> {
    let mut v = Vec::with_capacity(bits);
    let mut cur = Some(*p);
    for _ in 0..bits {
        v.push(cur);
        cur = cv.add(cur, cur);
    }
    v
}

fn tmul(cv: &Curve, tab: &[Option<Pt>], mut k: u64) -> Option<Pt> {
    let mut acc = None;
    let mut i = 0;
    while k > 0 {
        if k & 1 == 1 {
            acc = cv.add(acc, tab[i]);
        }
        k >>= 1;
        i += 1;
    }
    acc
}

/// Build the DLP context of one curve: generator of the order-`r` subgroup,
/// `Q = kP`, tables, automorphism lists (and their scalars on `⟨P⟩`).
fn build_cx(cv: Curve, inst: &Inst, k: u64, seed: u64) -> Cx {
    let f = cv.f;
    let r = inst.r;
    let mut rng = StdRng::seed_from_u64(seed);
    let p = loop {
        let p0 = cv.random_point(&mut rng);
        if let Some(p) = cv.mul(&p0, inst.cof) {
            if cv.mul(&p, r).is_none() {
                break p;
            }
        }
    };
    let q = cv.mul(&p, k).expect("k < r");
    let bits = 64 - r.leading_zeros() as usize;
    let ptab = pow_tab(&cv, &p, bits);
    let qtab = pow_tab(&cv, &q, bits);
    let neg = f.neg(1);
    let pm = vec![
        AutEl {
            bx: 1,
            iy: 1,
            rho: 1,
        },
        AutEl {
            bx: 1,
            iy: neg,
            rho: r - 1,
        },
    ];
    let mut full = pm.clone();
    let j = cv.j();
    if j == 0 {
        // ζ: (x, y) ↦ (βx, y), β of order 3, acting as ρ with ρ² + ρ + 1 = 0.
        let beta = (2..f.p)
            .map(|g| f.pow(g, (f.p - 1) / 3))
            .find(|&b| b != 1)
            .unwrap();
        let fr = Fp { p: r };
        let s = fr.sqrt(fr.red_i(-3)).expect("sqrt(-3) mod r");
        let rho0 = mulm(s + r - 1, invm(2, r), r);
        assert_eq!((mulm(rho0, rho0, r) + rho0 + 1) % r, 0);
        let img = Pt {
            x: f.mul(beta, p.x),
            y: p.y,
        };
        let rho = [rho0, mulm(rho0, rho0, r)]
            .into_iter()
            .find(|&c| cv.mul(&p, c) == Some(img))
            .expect("ζ acts as a primitive cube root of unity on ⟨P⟩");
        full.clear();
        for kk in 0..3u64 {
            let bk = f.pow(beta, kk);
            let rk = (0..kk).fold(1u64, |a, _| mulm(a, rho, r));
            full.push(AutEl {
                bx: bk,
                iy: 1,
                rho: rk,
            });
            full.push(AutEl {
                bx: bk,
                iy: neg,
                rho: (r - rk) % r,
            });
        }
    } else if j == 1728 % f.p {
        // i: (x, y) ↦ (−x, i y), acting as ρ with ρ² = −1.
        let ii = f.sqrt(neg).expect("p ≡ 1 mod 4");
        let fr = Fp { p: r };
        let rho0 = fr.sqrt(r - 1).expect("sqrt(-1) mod r");
        let img = Pt {
            x: f.neg(p.x),
            y: f.mul(ii, p.y),
        };
        let rho = [rho0, r - rho0]
            .into_iter()
            .find(|&c| cv.mul(&p, c) == Some(img))
            .expect("i acts as a square root of −1 on ⟨P⟩");
        full.clear();
        for kk in 0..4u64 {
            let bx = if kk % 2 == 0 { 1 } else { neg };
            let iy = f.pow(ii, kk);
            let rk = (0..kk).fold(1u64, |a, _| mulm(a, rho, r));
            full.push(AutEl { bx, iy, rho: rk });
        }
    }
    Cx {
        cv,
        r,
        p,
        q,
        ptab,
        qtab,
        pm,
        full,
        endo: None,
    }
}

/// Eigenvalue `c` with `ε(P) = [c]P` among `cands`.
fn find_eigen(cx: &Cx, chain: &Chain, cands: &[u64]) -> Option<u64> {
    let img = chain.eval(&cx.cv.f, cx.p)?;
    cands
        .iter()
        .copied()
        .find(|&c| cx.cv.mul(&cx.p, c) == Some(img))
}

// ---------------------------------------------------------------------------
// Rho with Aut-classes and optional endomorphism steps.

const TS: usize = 16;

#[derive(Clone, Copy)]
enum Step {
    Add { s: Pt, a: u64, b: u64 },
    Eps,
}

#[derive(Clone, Debug, Default)]
struct RunOut {
    iters: u64,
    mults: u64,
    dead: u32,
    esc: u32,
    ok: bool,
    bad: bool,
    eps_steps: u64,
    eps_mults: u64,
}

fn canon(cv: &Curve, auts: &[AutEl], pt: Pt) -> (Pt, u64) {
    let f = &cv.f;
    let mut best = pt;
    let mut brho = 1u64;
    for e in auts.iter().skip(1) {
        let x = if e.bx == 1 { pt.x } else { f.mul(e.bx, pt.x) };
        let y = if e.iy == 1 {
            pt.y
        } else if e.iy == f.neg(1) {
            f.neg(pt.y)
        } else {
            f.mul(e.iy, pt.y)
        };
        let c = Pt { x, y };
        if Curve::key(&c) < Curve::key(&best) {
            best = c;
            brho = e.rho;
        }
    }
    (best, brho)
}

fn rho_run(cx: &Cx, auts: &[AutEl], eps_slots: usize, seed: u64) -> RunOut {
    let cv = &cx.cv;
    let r = cx.r;
    let mut rng = StdRng::seed_from_u64(seed);
    // Step table (shared across curves by seed: the exponent pairs match).
    let mut slots: Vec<usize> = (0..TS).collect();
    for i in (1..TS).rev() {
        let j = rng.gen_range(0..=i);
        slots.swap(i, j);
    }
    let mut table: Vec<Step> = Vec::with_capacity(TS);
    for _ in 0..TS {
        let a = rng.gen_range(1..r);
        let b = rng.gen_range(1..r);
        let s = cv
            .add(tmul(cv, &cx.ptab, a), tmul(cv, &cx.qtab, b))
            .unwrap_or(Pt { x: 0, y: 0 });
        table.push(Step::Add { s, a, b });
    }
    if eps_slots > 0 {
        assert!(cx.endo.is_some());
        for &i in slots.iter().take(eps_slots) {
            table[i] = Step::Eps;
        }
    }
    let mut starts: Vec<(u64, u64)> = Vec::new();
    for _ in 0..64 {
        starts.push((rng.gen_range(1..r), rng.gen_range(1..r)));
    }
    let m_begin = mcount();
    let mut out = RunOut::default();
    let mut visited: HashMap<u64, (u64, u64, u32)> = HashMap::with_capacity(8192);
    let cap = 400 * ((r as f64).sqrt() as u64 + 1);
    let mut walk_id = 0u32;
    'walks: loop {
        let (a0, b0) = starts[(walk_id as usize) % starts.len()];
        walk_id += 1;
        let Some(mut rr) = cv.add(tmul(cv, &cx.ptab, a0), tmul(cv, &cx.qtab, b0)) else {
            continue;
        };
        let (mut a, mut b) = (a0, b0);
        let mut esc = 0;
        loop {
            if out.iters > cap {
                break 'walks;
            }
            let (cpt, rho) = canon(cv, auts, rr);
            a = mulm(a, rho, r);
            b = mulm(b, rho, r);
            rr = cpt;
            let key = Curve::key(&cpt);
            match visited.get(&key) {
                Some(&(a2, b2, wid)) => {
                    if (a2, b2) == (a, b) {
                        if wid == walk_id && esc < 8 {
                            // fruitless cycle: escape by doubling
                            match cv.add(Some(rr), Some(rr)) {
                                Some(d) => {
                                    rr = d;
                                    a = (2 * a) % r;
                                    b = (2 * b) % r;
                                    out.iters += 1;
                                    out.esc += 1;
                                    esc += 1;
                                    continue;
                                }
                                None => {
                                    out.dead += 1;
                                    break;
                                }
                            }
                        }
                        out.dead += 1;
                        break;
                    }
                    if b == b2 {
                        out.dead += 1;
                        break;
                    }
                    let num = (a2 + r - a) % r;
                    let den = (b + r - b2) % r;
                    let kk = mulm(num, invm(den, r), r);
                    out.ok = true;
                    out.bad = cv.mul(&cx.p, kk) != Some(cx.q);
                    out.mults = mcount() - m_begin;
                    return out;
                }
                None => {
                    visited.insert(key, (a, b, walk_id));
                    esc = 0;
                }
            }
            let idx = (cpt.x.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 60) as usize;
            match table[idx] {
                Step::Add { s, a: sa, b: sb } => {
                    match cv.add(Some(rr), Some(s)) {
                        Some(n) => rr = n,
                        None => {
                            out.dead += 1;
                            break;
                        }
                    }
                    a = (a + sa) % r;
                    b = (b + sb) % r;
                    out.iters += 1;
                }
                Step::Eps => {
                    let (chain, mu) = cx.endo.as_ref().unwrap();
                    let m1 = mcount();
                    let img = chain.eval(&cv.f, rr);
                    out.eps_mults += mcount() - m1;
                    out.eps_steps += 1;
                    match img {
                        Some(n) => rr = n,
                        None => {
                            out.dead += 1;
                            break;
                        }
                    }
                    a = mulm(a, *mu, r);
                    b = mulm(b, *mu, r);
                    out.iters += 1;
                }
            }
        }
    }
    out.mults = mcount() - m_begin;
    out
}

#[derive(Default, Clone)]
struct Agg {
    iters: Vec<f64>,
    mults: Vec<f64>,
    dead: f64,
    esc: f64,
    eps_steps: f64,
    eps_mults: f64,
    fail: u32,
    bad: u32,
}

impl Agg {
    fn push(&mut self, o: &RunOut) {
        if !o.ok {
            self.fail += 1;
            return;
        }
        if o.bad {
            self.bad += 1;
        }
        self.iters.push(o.iters as f64);
        self.mults.push(o.mults as f64);
        self.dead += o.dead as f64;
        self.esc += o.esc as f64;
        self.eps_steps += o.eps_steps as f64;
        self.eps_mults += o.eps_mults as f64;
    }
    fn js(&self) -> Value {
        let n = self.iters.len().max(1) as f64;
        json!({
            "runs": self.iters.len(),
            "failed_runs": self.fail,
            "wrong_dlp_runs": self.bad,
            "mean_iters": mean(&self.iters),
            "sd_iters": std_sample(&self.iters),
            "mean_mults": mean(&self.mults),
            "mean_dead_walks": self.dead / n,
            "mean_escapes": self.esc / n,
            "mean_eps_steps": self.eps_steps / n,
            "mean_eps_mults": self.eps_mults / n,
            "mean_mults_per_eps": if self.eps_steps > 0.0 { self.eps_mults / self.eps_steps } else { f64::NAN },
        })
    }
}

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.iter().sum::<f64>() / v.len() as f64
}

fn std_sample(v: &[f64]) -> f64 {
    if v.len() < 2 {
        return f64::NAN;
    }
    let mu = mean(v);
    (v.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0)).sqrt()
}

fn rho_agg(cx: &Cx, auts: &[AutEl], eps_slots: usize, base_seed: u64, runs: u64) -> Agg {
    let mut agg = Agg::default();
    for i in 0..runs {
        let o = rho_run(cx, auts, eps_slots, base_seed.wrapping_mul(1_000_003) + i);
        agg.push(&o);
    }
    agg
}

/// Mean multiplications of one affine addition and of one ε evaluation.
fn primitive_costs(cx: &Cx, rng: &mut StdRng) -> (f64, f64) {
    let cv = &cx.cv;
    let pts: Vec<Pt> = (0..60).map(|_| cv.random_point(rng)).collect();
    let m0 = mcount();
    let mut n_add = 0;
    for w in pts.windows(2) {
        if cv.add(Some(w[0]), Some(w[1])).is_some() {
            n_add += 1;
        }
    }
    let add_cost = (mcount() - m0) as f64 / n_add as f64;
    let eps_cost = match &cx.endo {
        Some((chain, _)) => {
            let m1 = mcount();
            let mut n = 0;
            for q in pts.iter().take(30) {
                if chain.eval(&cv.f, *q).is_some() {
                    n += 1;
                }
            }
            (mcount() - m1) as f64 / n as f64
        }
        None => f64::NAN,
    };
    (add_cost, eps_cost)
}

// ---------------------------------------------------------------------------
// GLV lattice.

fn glv_stats(mu: u64, r: u64, rng: &mut StdRng) -> Value {
    let (mut u, mut v) = ((r as i128, 0i128), (-(mu as i128), 1i128));
    // Gauss reduction.
    let nrm = |a: (i128, i128)| a.0 * a.0 + a.1 * a.1;
    loop {
        if nrm(u) > nrm(v) {
            std::mem::swap(&mut u, &mut v);
        }
        let dot = u.0 * v.0 + u.1 * v.1;
        let q = (2 * dot + nrm(u)).div_euclid(2 * nrm(u));
        if q == 0 {
            break;
        }
        v = (v.0 - q * u.0, v.1 - q * u.1);
    }
    let det = (u.0 * v.1 - u.1 * v.0) as f64;
    let mut bits = Vec::new();
    for _ in 0..2000 {
        let k = rng.gen_range(0..r) as f64;
        let al = (k * v.1 as f64 / det).round();
        let be = (-(u.1 as f64) * k / det).round();
        let k1 = k - al * u.0 as f64 - be * v.0 as f64;
        let k2 = -al * u.1 as f64 - be * v.1 as f64;
        bits.push((k1.abs().max(k2.abs()) + 1.0).log2());
    }
    json!({
        "mu": mu,
        "shortest_vector_log2": (nrm(u) as f64).sqrt().log2(),
        "second_vector_log2": (nrm(v) as f64).sqrt().log2(),
        "basis_u": [u.0 as i64, u.1 as i64],
        "basis_v": [v.0 as i64, v.1 as i64],
        "mean_log2_max_k": mean(&bits),
        "log2_sqrt_r": (r as f64).sqrt().log2(),
    })
}

// ---------------------------------------------------------------------------
// Instance set-up shared by all experiments.

struct Level {
    cv: Curve,
    j: u64,
    nroots: usize,
}

struct Setup {
    inst: Inst,
    lf: LField,
    phi: modpoly::ModPoly,
    crater: Level,
    floors: Vec<Level>,
    tries: u64,
    crater_roots: Vec<u64>,
}

fn setup(ell: u64, d: u64, idx: u64, seed: u64) -> Setup {
    let mut rng = StdRng::seed_from_u64(seed ^ (ell << 32) ^ (idx << 16) ^ d ^ 0x73);
    let (inst, tries) = find_instance(ell, d, 1 << 20, 1 << 22, &mut rng);
    let f = Fp { p: inst.p };
    let lf = LField::new(&BigUint::from(inst.p)).expect("prime field");
    let phi = modpoly::modular_polynomial(&lf, ell).expect("Φ_ℓ");
    let (e0, _) = crater_curve(f, &inst, &mut rng).expect("crater curve with #E = N");
    let j0 = e0.j();
    assert_eq!(j0, inst.j0, "crater j");
    let roots0 = phi_roots(&lf, &phi, j0);
    let floor_js: Vec<u64> = roots0.iter().copied().filter(|&x| x != j0).collect();
    let div = match d {
        3 => 3,
        4 => 2,
        _ => 1,
    };
    let expected = (ell as i64 - inst.kron as i64) / div;
    assert_eq!(
        floor_js.len() as i64,
        expected,
        "floor count (ℓ={ell}, D=−{d}, p={})",
        inst.p
    );
    let mut floors = Vec::new();
    for &j1 in &floor_js {
        assert!(j1 != 0 && j1 != 1728 % inst.p);
        let (e1, _) =
            curve_from_j(f, j1, inst.n, &mut rng).expect("floor j must give a twist with #E = N");
        let nroots = phi_roots(&lf, &phi, j1).len();
        assert_eq!(nroots, 1, "a floor curve has exactly one rational ℓ-root");
        floors.push(Level {
            cv: e1,
            j: j1,
            nroots,
        });
    }
    Setup {
        crater: Level {
            cv: e0,
            j: j0,
            nroots: roots0.len(),
        },
        inst,
        lf,
        phi,
        floors,
        tries,
        crater_roots: roots0,
    }
}

// ---------------------------------------------------------------------------
// Exp A.

fn exp_a(ell: u64, d: u64, idx: u64, seed: u64, runs: u64) -> Value {
    let su = setup(ell, d, idx, seed);
    let inst = &su.inst;
    eprintln!(
        "expA ℓ={ell} D=−{d} #{idx}: p={} t={} v={} N={} r={} floors={}",
        inst.p,
        inst.t,
        inst.v,
        inst.n,
        inst.r,
        su.floors.len()
    );
    let mut rng = StdRng::seed_from_u64(seed ^ 0xA11 ^ idx);
    let k = rng.gen_range(2..inst.r - 1);
    let base = seed.wrapping_mul(7919) ^ (idx << 8) ^ ell;
    let cc = build_cx(su.crater.cv, inst, k, base ^ 1);
    let aut_order = cc.full.len();
    let c_pm = rho_agg(&cc, &cc.pm, 0, base, runs);
    let c_aut = rho_agg(&cc, &cc.full, 0, base, runs);
    let mut floors = Vec::new();
    for (i, fl) in su.floors.iter().enumerate() {
        let fc = build_cx(fl.cv, inst, k, base ^ (2 + i as u64));
        assert_eq!(fc.full.len(), 2, "floor Aut = ±1");
        let a = rho_agg(&fc, &fc.pm, 0, base, runs);
        floors.push(json!({"j": fl.j, "n_phi_roots": fl.nroots, "pm": a.js()}));
    }
    json!({
        "exp": "A",
        "ell": ell, "D": -(d as i64), "instance": idx, "seed": seed,
        "p": inst.p, "t": inst.t, "v": inst.v, "N": inst.n, "r": inst.r, "cof": inst.cof,
        "kronecker_D_ell": inst.kron, "ell_check": inst.ell,
        "j_crater": inst.j0, "crater_phi_roots": su.crater_roots,
        "aut_crater": aut_order, "aut_floor": 2,
        "n_floor": su.floors.len(),
        "k": k, "runs": runs, "search_draws": su.tries,
        "crater": {"pm": c_pm.js(), "aut": c_aut.js()},
        "floors": floors,
    })
}

// ---------------------------------------------------------------------------
// Exp B (+ C, GLV).

fn base_from_xs(cv: &Curve, size: usize) -> Vec<Pt> {
    let mut out = Vec::with_capacity(size);
    for x in 0..cv.f.p {
        if out.len() >= size {
            break;
        }
        let r = cv.rhs(x);
        if r == 0 {
            continue;
        }
        if let Some(y) = cv.f.sqrt(r) {
            let q = Pt { x, y };
            out.push(q);
            out.push(cv.neg(&q));
        }
    }
    assert_eq!(out.len(), size);
    out
}

/// Distinct non-zero targets hit by 2- and 3-multisets (as in R6.11).
fn census_d(cv: &Curve, base: &[Pt]) -> (u64, u64) {
    let n = base.len();
    let keys: HashSet<u64> = base.iter().map(Curve::key).collect();
    assert_eq!(keys.len(), n, "duplicate base elements");
    for q in base {
        assert!(cv.on(q));
        assert!(keys.contains(&Curve::key(&cv.neg(q))), "base not signed");
    }
    let mut pairs: Vec<(usize, Option<Pt>)> = Vec::with_capacity(n * (n + 1) / 2);
    let mut h2: HashSet<u64> = HashSet::new();
    for (i, bi) in base.iter().enumerate() {
        for (j, bj) in base.iter().enumerate().skip(i) {
            let s = cv.add(Some(*bi), Some(*bj));
            if let Some(q) = s {
                h2.insert(Curve::key(&q));
            }
            pairs.push((j, s));
        }
    }
    let mut h3: HashSet<u64> = HashSet::new();
    for (j, s) in &pairs {
        for bk in base.iter().skip(*j) {
            if let Some(q) = cv.add(*s, Some(*bk)) {
                h3.insert(Curve::key(&q));
            }
        }
    }
    (h2.len() as u64, h3.len() as u64)
}

fn exp_b(ell: u64, d: u64, idx: u64, seed: u64, runs: u64, census_floors: usize) -> Value {
    let su = setup(ell, d, idx, seed);
    let inst = &su.inst;
    let lf = &su.lf;
    let f = Fp { p: inst.p };
    eprintln!(
        "expB ℓ={ell} D=−{d} #{idx}: p={} t={} v={} N={} floors={}",
        inst.p,
        inst.t,
        inst.v,
        inst.n,
        su.floors.len()
    );
    let mut rng = StdRng::seed_from_u64(seed ^ 0xB22 ^ idx);
    let k = rng.gen_range(2..inst.r - 1);
    let base = seed.wrapping_mul(104_729) ^ (idx << 8) ^ ell;
    let n = n_omega(d);
    let m0 = su.crater.cv.model(lf);
    // ω on the crater: kernel = π-eigenline with eigenvalue (t−v)/2 mod n
    // (or (t+v)/2: then the endomorphism is ω̄ = 1 − ω).
    let tv = |s: i64| ((inst.t + s * inst.v as i64) / 2).rem_euclid(n as i64) as u64;
    let mut omega_map: Option<VeluMap> = None;
    let mut omega_which = "";
    for (name, mu) in [("omega", tv(-1)), ("omega_bar", tv(1))] {
        if mu == 0 {
            continue;
        }
        match eigenline_kernel(lf, &m0, n, mu) {
            Ok(h) => match VeluMap::new(lf, &f, &m0, &h, &m0) {
                Ok(vm) => {
                    omega_map = Some(vm);
                    omega_which = name;
                    break;
                }
                Err(e) => eprintln!("  {name}: {e}"),
            },
            Err(e) => eprintln!("  {name}: {e}"),
        }
    }
    let omega_map = omega_map.expect("an endomorphism of the crater");
    omega_map.verify(&f, &su.crater.cv, 12, &mut rng);
    let omega = Chain {
        maps: vec![omega_map.clone()],
    };
    let mut cc = build_cx(su.crater.cv, inst, k, base ^ 1);
    // Eigenvalue of ω on ⟨P⟩: roots of X² − X + n_ω (ω) or its conjugate.
    let fr = Fp { p: inst.r };
    let sq = fr
        .sqrt(fr.red_i(-(d as i128)))
        .expect("√D mod r exists on the crater");
    let inv2 = invm(2, inst.r);
    let l1 = mulm(1 + sq, inv2, inst.r);
    let l2 = mulm(inst.r + 1 - sq, inv2, inst.r);
    let cands = [l1, l2, inst.r - l1, inst.r - l2];
    let lam = find_eigen(&cc, &omega, &cands)
        .expect("ω acts on ⟨P⟩ as ±(1±√D)/2: Vélu evaluation consistent");
    cc.endo = Some((omega.clone(), lam));
    let (add_c, eps_c) = primitive_costs(&cc, &mut rng);
    let plain_c = rho_agg(&cc, &cc.pm, 0, base, runs);
    let eps_c_agg = rho_agg(&cc, &cc.pm, 2, base, runs);
    let glv_c = glv_stats(lam, inst.r, &mut rng);
    let crater = json!({
        "j": su.crater.j, "n_phi_roots": su.crater.nroots,
        "endo": omega_which, "endo_degree": n, "endo_eigenvalue": lam,
        "mults_per_add": add_c, "mults_per_eps_eval": eps_c,
        "plain": plain_c.js(), "eps": eps_c_agg.js(), "glv": glv_c,
    });
    let mut floors = Vec::new();
    let mut transport = Vec::new();
    for (i, fl) in su.floors.iter().enumerate() {
        let m1 = fl.cv.model(lf);
        let j1 = lf.from_u64(fl.j);
        let j0 = lf.from_u64(su.crater.j);
        // ψ: E0 → E1 (down), φ: E1 → E0 (up).
        let down = kernel::elkies_isogeny(lf, &m0, &su.phi, &j1).expect("Elkies down");
        let up = kernel::elkies_isogeny(lf, &m1, &su.phi, &j0).expect("Elkies up");
        let psi = VeluMap::new(lf, &f, &m0, &down.kernel, &m1).expect("ψ codomain ≅ floor");
        let phi_m = VeluMap::new(lf, &f, &m1, &up.kernel, &m0).expect("φ codomain ≅ crater");
        psi.verify(&f, &su.crater.cv, 8, &mut rng);
        phi_m.verify(&f, &fl.cv, 8, &mut rng);
        let eps = Chain {
            maps: vec![phi_m.clone(), omega_map.clone(), psi.clone()],
        };
        let mut fc = build_cx(fl.cv, inst, k, base ^ (2 + i as u64));
        let ell_lam = [
            mulm(ell, l1, inst.r),
            mulm(ell, l2, inst.r),
            inst.r - mulm(ell, l1, inst.r),
            inst.r - mulm(ell, l2, inst.r),
        ];
        let mu = find_eigen(&fc, &eps, &ell_lam)
            .expect("ε = ψωφ acts as ±ℓ·(eigenvalue of ω or ω̄) on the floor");
        fc.endo = Some((eps.clone(), mu));
        let (add_f, eps_f) = primitive_costs(&fc, &mut rng);
        let plain_f = rho_agg(&fc, &fc.pm, 0, base, runs);
        let eps_f_agg = rho_agg(&fc, &fc.pm, 2, base, runs);
        let glv_f = glv_stats(mu, inst.r, &mut rng);
        floors.push(json!({
            "j": fl.j, "n_phi_roots": fl.nroots,
            "endo_degree": ell * ell * n, "endo_eigenvalue": mu,
            "mults_per_add": add_f, "mults_per_eps_eval": eps_f,
            "plain": plain_f.js(), "eps": eps_f_agg.js(), "glv": glv_f,
        }));
        // Exp C: transport checks on the first `census_floors` floors.
        if i < census_floors {
            let size = 100;
            let b0 = base_from_xs(&cc.cv, size);
            let b1 = base_from_xs(&fc.cv, size);
            let (d0_2, d0_3) = census_d(&cc.cv, &b0);
            let (d1_2, d1_3) = census_d(&fc.cv, &b1);
            let tr01: Vec<Pt> = b0
                .iter()
                .map(|q| psi.eval(&f, *q).expect("base point outside kernel"))
                .collect();
            let tr10: Vec<Pt> = b1
                .iter()
                .map(|q| phi_m.eval(&f, *q).expect("base point outside kernel"))
                .collect();
            let (t01_2, t01_3) = census_d(&fc.cv, &tr01);
            let (t10_2, t10_3) = census_d(&cc.cv, &tr10);
            // DLP transport: φ(Q1) = k·φ(P1); cost = two φ evaluations.
            let mt = mcount();
            let p0 = phi_m.eval(&f, fc.p).unwrap();
            let q0 = phi_m.eval(&f, fc.q).unwrap();
            let transport_mults = mcount() - mt;
            let consistent = cc.cv.mul(&p0, k) == Some(q0);
            transport.push(json!({
                "floor_index": i, "size": size,
                "crater_native": [d0_2, d0_3], "floor_native": [d1_2, d1_3],
                "crater_to_floor_transported": [t01_2, t01_3],
                "floor_to_crater_transported": [t10_2, t10_3],
                "identity_crater_to_floor": (t01_2, t01_3) == (d0_2, d0_3),
                "identity_floor_to_crater": (t10_2, t10_3) == (d1_2, d1_3),
                "dlp_transport_consistent": consistent,
                "dlp_transport_mults": transport_mults,
            }));
        }
    }
    json!({
        "exp": "B",
        "ell": ell, "D": -(d as i64), "instance": idx, "seed": seed,
        "p": inst.p, "t": inst.t, "v": inst.v, "N": inst.n, "r": inst.r,
        "kronecker_D_ell": inst.kron, "n_omega": n,
        "n_floor": su.floors.len(), "k": k, "runs": runs, "search_draws": su.tries,
        "crater": crater, "floors": floors, "transport": transport,
    })
}

// ---------------------------------------------------------------------------
// Exp H: craters with h(D) > 1 (census of the plain x-frame base, as R6.11).

/// Class number `h(D)` of the discriminant `−d` (reduced primitive forms).
fn class_number(d: u64) -> u64 {
    let disc = -(d as i64);
    let mut h = 0;
    let mut a = 1i64;
    while 3 * a * a <= d as i64 {
        for b in -a + 1..=a {
            if (b - disc).rem_euclid(2) != 0 {
                continue;
            }
            let num = b * b - disc;
            if num % (4 * a) != 0 {
                continue;
            }
            let c = num / (4 * a);
            if c < a || (c == a && b < 0) {
                continue;
            }
            let g = gcd_u(gcd_u(a as u64, b.unsigned_abs()), c as u64);
            if g == 1 {
                h += 1;
            }
        }
        a += 1;
    }
    h
}

/// `ℓ` is non-principal in `O_K` iff `4ℓ = X² + |D|Y²` has no solution with
/// `X ≡ D·Y (mod 2)`.
fn ell_principal(ell: u64, d: u64) -> bool {
    let mut y = 0u64;
    while d * y * y <= 4 * ell {
        let rest = 4 * ell - d * y * y;
        let x = (rest as f64).sqrt().round() as u64;
        if x * x == rest && (x + d * y).is_multiple_of(2) {
            return true;
        }
        y += 1;
    }
    false
}

fn exp_h(ell: u64, d: u64, t_sign: i64, seed: u64, sizes: &[usize], max: usize) -> Vec<Value> {
    // All (t) with p = (t² + d ℓ²)/4 in [2^20, 2^22), p and N prime, ℓ ∤ N.
    let h_d = class_number(d);
    let kron = {
        let fl = Fp { p: ell };
        fl.legendre(fl.red_i(-(d as i128)))
    };
    let mut out = Vec::new();
    let tmin = ((4.0 * (1u64 << 20) as f64 - (d * ell * ell) as f64).sqrt()) as i64;
    let tmax = ((4.0 * (1u64 << 22) as f64 - (d * ell * ell) as f64).sqrt()) as i64;
    for ta in tmin..tmax {
        let t = t_sign * ta;
        if ta % 2 != (d * ell * ell) as i64 % 2 {
            continue;
        }
        let four_p = (ta * ta) as u64 + d * ell * ell;
        if !four_p.is_multiple_of(4) {
            continue;
        }
        let p = four_p / 4;
        if !(1 << 20..1 << 22).contains(&p) || !is_prime_u64(p) {
            continue;
        }
        let n = (p as i64 + 1 - t) as u64;
        if !is_prime_u64(n) || n.is_multiple_of(ell) || t.rem_euclid(ell as i64) == 2 % ell as i64 {
            continue;
        }
        out.push((t, p, n));
    }
    let mut recs = Vec::new();
    for (t, p, n) in out {
        if recs.len() >= max {
            break;
        }
        let mut rng = StdRng::seed_from_u64(seed ^ p ^ (ell << 40));
        let f = Fp { p };
        let lf = LField::new(&BigUint::from(p)).unwrap();
        let phi = modpoly::modular_polynomial(&lf, ell).unwrap();
        // A curve in the class: random j with #E = N (some twist).
        let mut start = None;
        for _ in 0..4_000_000u64 {
            let j = rng.gen_range(2..p);
            if j == 1728 % p {
                continue;
            }
            if let Some((cv, _)) = curve_from_j(f, j, n, &mut rng) {
                start = Some(cv.j());
                break;
            }
        }
        let Some(mut jc) = start else { continue };
        // climb to a crater (≥ 2 rational ℓ-roots), then walk the horizontal cycle
        let nroots = |j: u64| phi_roots(&lf, &phi, j);
        let rj = nroots(jc);
        if rj.len() == 1 {
            jc = rj[0];
        }
        let mut craters = vec![jc];
        let mut floors: Vec<(u64, u64)> = Vec::new(); // (floor j, parent crater j)
        let mut qi = 0;
        while qi < craters.len() {
            let c = craters[qi];
            qi += 1;
            for r in nroots(c) {
                if r == c {
                    continue;
                }
                if nroots(r).len() == 1 {
                    if !floors.iter().any(|x| x.0 == r) {
                        floors.push((r, c));
                    }
                } else if !craters.contains(&r) {
                    craters.push(r);
                }
            }
        }
        let want_floors = h_d as i64 * (ell as i64 - kron as i64);
        let complete = craters.len() as u64 == h_d && floors.len() as i64 == want_floors;
        // census
        let mut crater_recs = Vec::new();
        let curve_of = |j: u64, rng: &mut StdRng| curve_from_j(f, j, n, rng).map(|x| x.0);
        let census_of = |j: u64, rng: &mut StdRng| -> Value {
            let cv = curve_of(j, rng).expect("class member has a twist of order N");
            let mut rows = Vec::new();
            for &sz in sizes {
                let b = base_from_xs(&cv, sz);
                let (d2, d3) = census_d(&cv, &b);
                let nm2 = (sz as f64).powi(2);
                let nm3 = (sz as f64).powi(3);
                rows.push(json!({
                    "size": sz, "d2": d2, "d3": d3,
                    "c2": d2 as f64 * 2.0 / nm2, "c3": d3 as f64 * 6.0 / nm3,
                }));
            }
            json!({"j": j, "census": rows})
        };
        for &c in &craters {
            let cr = census_of(c, &mut rng);
            let fl: Vec<Value> = floors
                .iter()
                .filter(|x| x.1 == c)
                .map(|x| census_of(x.0, &mut rng))
                .collect();
            crater_recs.push(json!({"crater": cr, "floors": fl}));
        }
        eprintln!(
            "expH ℓ={ell} D=−{d} t={t} p={p} N={n}: craters {} (h={h_d}), floors {} (want {want_floors}), complete={complete}",
            craters.len(),
            floors.len()
        );
        recs.push(json!({
            "exp": "H", "ell": ell, "D": -(d as i64), "h_D": h_d, "t": t, "p": p, "N": n,
            "kronecker_D_ell": kron, "ell_principal": ell_principal(ell, d),
            "n_craters": craters.len(), "n_floors": floors.len(),
            "floors_expected": want_floors, "complete": complete,
            "sizes": sizes, "gaps": crater_recs,
        }));
    }
    recs
}

// ---------------------------------------------------------------------------
// Summary: cluster bootstrap over instances.

fn bootstrap_gm(xs: &[f64], seed: u64) -> (f64, f64, f64) {
    let logs: Vec<f64> = xs.iter().map(|x| x.ln()).collect();
    let n = logs.len();
    let gm = mean(&logs).exp();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut ms: Vec<f64> = (0..20000)
        .map(|_| (0..n).map(|_| logs[rng.gen_range(0..n)]).sum::<f64>() / n as f64)
        .collect();
    ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (
        gm,
        ms[(0.025 * ms.len() as f64) as usize].exp(),
        ms[(0.975 * ms.len() as f64) as usize].exp(),
    )
}

fn ci_json(xs: &[f64], seed: u64) -> Value {
    if xs.len() < 2 {
        return json!({"n": xs.len()});
    }
    let (gm, lo, hi) = bootstrap_gm(xs, seed);
    json!({
        "n": xs.len(), "geo_mean": gm, "ci95": [lo, hi],
        "contains_1": lo <= 1.0 && 1.0 <= hi,
        "min": xs.iter().cloned().fold(f64::INFINITY, f64::min),
        "max": xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
    })
}

fn gf(v: &Value, path: &[&str]) -> f64 {
    let mut cur = v;
    for p in path {
        cur = &cur[*p];
    }
    cur.as_f64().unwrap_or(f64::NAN)
}

fn load(dir: &str, prefix: &str) -> Vec<Value> {
    let mut files: Vec<_> = fs::read_dir(dir)
        .expect("dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|x| x == "json")
                && p.file_name()
                    .is_some_and(|n| n.to_string_lossy().starts_with(prefix))
        })
        .collect();
    files.sort();
    files
        .iter()
        .map(|p| serde_json::from_str(&fs::read_to_string(p).unwrap()).unwrap())
        .collect()
}

fn floor_mean(r: &Value, path: &[&str]) -> f64 {
    let v: Vec<f64> = r["floors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| gf(f, path))
        .collect();
    mean(&v)
}

fn summarise(dir: &str) -> Value {
    let a = load(dir, "r7_3_expA_");
    let b = load(dir, "r7_3_expB_");
    let mut out = serde_json::Map::new();
    // ---- Exp A
    let mut rows_a = Vec::new();
    for dd in [-3i64, -4] {
        let recs: Vec<&Value> = a.iter().filter(|r| r["D"] == dd).collect();
        let mut ells: Vec<u64> = recs.iter().map(|r| r["ell"].as_u64().unwrap()).collect();
        ells.sort_unstable();
        ells.dedup();
        let mut groups: Vec<(String, Vec<&Value>)> = vec![("all".into(), recs.clone())];
        for e in &ells {
            groups.push((
                format!("ell{e}"),
                recs.iter().filter(|r| r["ell"] == *e).cloned().collect(),
            ));
        }
        for (gname, rs) in &groups {
            for metric in ["mean_iters", "mean_mults"] {
                let aut_c: Vec<f64> = rs
                    .iter()
                    .map(|r| gf(r, &["crater", "aut", metric]) / gf(r, &["crater", "pm", metric]))
                    .collect();
                let cond: Vec<f64> = rs
                    .iter()
                    .map(|r| gf(r, &["crater", "pm", metric]) / floor_mean(r, &["pm", metric]))
                    .collect();
                let total: Vec<f64> = rs
                    .iter()
                    .map(|r| gf(r, &["crater", "aut", metric]) / floor_mean(r, &["pm", metric]))
                    .collect();
                let aut_order = rs[0]["aut_crater"].as_f64().unwrap();
                let base = (aut_order / 2.0).sqrt();
                let removed: Vec<f64> = total.iter().map(|x| x * base).collect();
                rows_a.push(json!({
                    "D": dd, "group": gname, "metric": metric,
                    "aut_crater": aut_order,
                    "predicted_aut_effect": 1.0 / base,
                    "aut_over_pm_on_crater": ci_json(&aut_c, 11),
                    "crater_pm_over_floor_pm_conductor_effect": ci_json(&cond, 12),
                    "crater_aut_over_floor_pm_total": ci_json(&total, 13),
                    "total_times_predicted_baseline_removed": ci_json(&removed, 14),
                }));
            }
        }
    }
    out.insert("expA".into(), Value::Array(rows_a));
    out.insert("expA_instances".into(), json!(a.len()));
    // ---- Exp B
    let mut rows_b = Vec::new();
    let mut ells: Vec<u64> = b.iter().map(|r| r["ell"].as_u64().unwrap()).collect();
    ells.sort_unstable();
    ells.dedup();
    let mut groups: Vec<(String, Vec<&Value>)> = vec![("all".into(), b.iter().collect())];
    for e in &ells {
        groups.push((
            format!("ell{e}"),
            b.iter().filter(|r| r["ell"] == *e).collect(),
        ));
    }
    for (gname, rs) in &groups {
        if rs.len() < 2 {
            continue;
        }
        let mut row = serde_json::Map::new();
        row.insert("group".into(), json!(gname));
        row.insert("n_instances".into(), json!(rs.len()));
        for (variant, name) in [("plain", "plain"), ("eps", "eps")] {
            for metric in ["mean_iters", "mean_mults"] {
                let v: Vec<f64> = rs
                    .iter()
                    .map(|r| {
                        gf(r, &["crater", variant, metric]) / floor_mean(r, &[variant, metric])
                    })
                    .collect();
                row.insert(
                    format!("{name}_{metric}_crater_over_floor"),
                    ci_json(&v, 21),
                );
            }
        }
        for metric in ["mean_iters", "mean_mults"] {
            let cr: Vec<f64> = rs
                .iter()
                .map(|r| gf(r, &["crater", "eps", metric]) / gf(r, &["crater", "plain", metric]))
                .collect();
            let fl: Vec<f64> = rs
                .iter()
                .map(|r| floor_mean(r, &["eps", metric]) / floor_mean(r, &["plain", metric]))
                .collect();
            row.insert(format!("eps_over_plain_{metric}_crater"), ci_json(&cr, 22));
            row.insert(format!("eps_over_plain_{metric}_floor"), ci_json(&fl, 23));
        }
        let evc: Vec<f64> = rs
            .iter()
            .map(|r| {
                floor_mean(r, &["mults_per_eps_eval"]) / gf(r, &["crater", "mults_per_eps_eval"])
            })
            .collect();
        row.insert("eps_eval_cost_floor_over_crater".into(), ci_json(&evc, 24));
        let evrel: Vec<f64> = rs
            .iter()
            .map(|r| {
                (floor_mean(r, &["mults_per_eps_eval"]) / floor_mean(r, &["mults_per_add"]))
                    / (gf(r, &["crater", "mults_per_eps_eval"])
                        / gf(r, &["crater", "mults_per_add"]))
            })
            .collect();
        row.insert(
            "eps_cost_in_adds_floor_over_crater".into(),
            ci_json(&evrel, 25),
        );
        let glv: Vec<f64> = rs
            .iter()
            .map(|r| {
                let c = gf(r, &["crater", "glv", "mean_log2_max_k"]);
                let f = floor_mean(r, &["glv", "mean_log2_max_k"]);
                f / c
            })
            .collect();
        row.insert("glv_log2_max_k_floor_over_crater".into(), ci_json(&glv, 26));
        let glv_d: Vec<f64> = rs
            .iter()
            .map(|r| {
                floor_mean(r, &["glv", "mean_log2_max_k"])
                    - gf(r, &["crater", "glv", "mean_log2_max_k"])
            })
            .collect();
        row.insert(
            "glv_log2_max_k_floor_minus_crater_bits".into(),
            json!({
                "n": glv_d.len(), "mean": mean(&glv_d), "sd": std_sample(&glv_d),
            }),
        );
        rows_b.push(Value::Object(row));
    }
    out.insert("expB".into(), Value::Array(rows_b));
    out.insert("expB_instances".into(), json!(b.len()));
    // transport
    let mut n_tr = 0;
    let mut n_id = 0;
    let mut n_dlp = 0;
    let mut tr_cost = Vec::new();
    let mut tr_rel = Vec::new();
    for r in &b {
        for t in r["transport"].as_array().unwrap() {
            n_tr += 1;
            if t["identity_crater_to_floor"] == true && t["identity_floor_to_crater"] == true {
                n_id += 1;
            }
            if t["dlp_transport_consistent"] == true {
                n_dlp += 1;
            }
            let c = t["dlp_transport_mults"].as_f64().unwrap();
            tr_cost.push(c);
            tr_rel.push(c / gf(r, &["crater", "plain", "mean_mults"]));
        }
    }
    out.insert(
        "expC".into(),
        json!({
            "transport_checks": n_tr,
            "exact_census_identity_both_directions": n_id,
            "dlp_transport_consistent": n_dlp,
            "mean_transport_mults": mean(&tr_cost),
            "mean_transport_over_crater_rho_mults": mean(&tr_rel),
            "max_transport_over_crater_rho_mults": tr_rel.iter().cloned().fold(0.0, f64::max),
        }),
    );
    // ---- Exp H (h(D) > 1)
    let hrecs = load(dir, "r7_3_expH_");
    let mut rows_h = Vec::new();
    let complete: Vec<&Value> = hrecs.iter().filter(|r| r["complete"] == true).collect();
    let mut dgroups: Vec<(String, Vec<&Value>)> = vec![("all".into(), complete.clone())];
    let mut dvals: Vec<i64> = complete.iter().map(|r| r["D"].as_i64().unwrap()).collect();
    dvals.sort_unstable();
    dvals.dedup();
    for dv in dvals {
        dgroups.push((
            format!("D{dv}"),
            complete.iter().filter(|r| r["D"] == dv).cloned().collect(),
        ));
    }
    for (gname, rs) in &dgroups {
        if rs.len() < 2 {
            continue;
        }
        let sizes: Vec<u64> = rs[0]["sizes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect();
        for (si, sz) in sizes.iter().enumerate() {
            for m in ["c2", "c3"] {
                let xs: Vec<f64> = rs
                    .iter()
                    .map(|r| {
                        let gaps = r["gaps"].as_array().unwrap();
                        let v: Vec<f64> = gaps
                            .iter()
                            .map(|g| {
                                let cc = g["crater"]["census"][si][m].as_f64().unwrap();
                                let fl: Vec<f64> = g["floors"]
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|f| f["census"][si][m].as_f64().unwrap())
                                    .collect();
                                cc / mean(&fl)
                            })
                            .collect();
                        mean(&v)
                    })
                    .collect();
                let hs: Vec<u64> = rs.iter().map(|r| r["h_D"].as_u64().unwrap()).collect();
                rows_h.push(json!({
                    "group": gname, "size": sz, "m": m,
                    "n_instances": rs.len(),
                    "h_D_values": hs.iter().copied().collect::<std::collections::BTreeSet<_>>(),
                    "crater_over_floor_class_factor": ci_json(&xs, 31),
                }));
            }
        }
    }
    out.insert("expH".into(), Value::Array(rows_h));
    out.insert("expH_records".into(), json!(hrecs.len()));
    out.insert("expH_complete".into(), json!(complete.len()));
    out.insert(
        "wrong_dlp_runs_total".into(),
        json!(count_bad(&a) + count_bad(&b)),
    );
    Value::Object(out)
}

fn count_bad(recs: &[Value]) -> u64 {
    let mut n = 0;
    for r in recs {
        let mut stack = vec![r];
        while let Some(v) = stack.pop() {
            match v {
                Value::Object(m) => {
                    if let Some(x) = m.get("wrong_dlp_runs").and_then(|x| x.as_u64()) {
                        n += x;
                    }
                    if let Some(x) = m.get("failed_runs").and_then(|x| x.as_u64()) {
                        n += x;
                    }
                    stack.extend(m.values());
                }
                Value::Array(a) => stack.extend(a.iter()),
                _ => {}
            }
        }
    }
    n
}

fn arg(args: &[String], name: &str, default: &str) -> String {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
        .unwrap_or_else(|| default.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("help");
    let ell: u64 = arg(&args, "--ell", "5").parse().unwrap();
    let n_inst: u64 = arg(&args, "--instances", "6").parse().unwrap();
    let seed: u64 = arg(&args, "--seed", "1").parse().unwrap();
    let runs: u64 = arg(&args, "--runs", "300").parse().unwrap();
    let out = arg(&args, "--out", ".");
    match mode {
        "expA" => {
            let d: u64 = arg(&args, "--d", "3").parse().unwrap();
            fs::create_dir_all(&out).unwrap();
            for idx in 0..n_inst {
                let rec = exp_a(ell, d, idx, seed, runs);
                let path = format!("{out}/r7_3_expA_d{d}_ell{ell}_i{idx}.json");
                fs::write(&path, serde_json::to_string_pretty(&rec).unwrap()).unwrap();
                eprintln!(
                    "  {path}: crater pm {:.1} aut {:.1}; floor0 pm {:.1}",
                    gf(&rec, &["crater", "pm", "mean_iters"]),
                    gf(&rec, &["crater", "aut", "mean_iters"]),
                    rec["floors"][0]["pm"]["mean_iters"]
                        .as_f64()
                        .unwrap_or(f64::NAN)
                );
            }
        }
        "expB" => {
            fs::create_dir_all(&out).unwrap();
            let cf: usize = arg(&args, "--census-floors", "3").parse().unwrap();
            // D = −11 (n_ω = 3) is excluded: its two rational lines have
            // opposite eigenvalues, which the x-coordinate condition cannot separate.
            let ds: Vec<u64> = CM_J
                .iter()
                .map(|(d, _)| *d)
                .filter(|d| *d != 11 && d % ell != 0 && n_omega(*d) != ell)
                .collect();
            for idx in 0..n_inst {
                let d = ds[idx as usize % ds.len()];
                let rec = exp_b(ell, d, idx, seed, runs, cf);
                let path = format!("{out}/r7_3_expB_ell{ell}_i{idx}.json");
                fs::write(&path, serde_json::to_string_pretty(&rec).unwrap()).unwrap();
                eprintln!(
                    "  {path}: plain c {:.1} f0 {:.1}; eps c {:.1} f0 {:.1}; mults/eps c {:.0} f0 {:.0}",
                    gf(&rec, &["crater", "plain", "mean_iters"]),
                    rec["floors"][0]["plain"]["mean_iters"].as_f64().unwrap_or(f64::NAN),
                    gf(&rec, &["crater", "eps", "mean_iters"]),
                    rec["floors"][0]["eps"]["mean_iters"].as_f64().unwrap_or(f64::NAN),
                    gf(&rec, &["crater", "mults_per_eps_eval"]),
                    rec["floors"][0]["mults_per_eps_eval"].as_f64().unwrap_or(f64::NAN),
                );
            }
        }
        "expH" => {
            let d: u64 = arg(&args, "--d", "15").parse().unwrap();
            let sizes: Vec<usize> = arg(&args, "--sizes", "100,200")
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            let max_inst: usize = arg(&args, "--max", "100").parse().unwrap();
            fs::create_dir_all(&out).unwrap();
            let mut written = 0;
            for sign in [1i64, -1] {
                for rec in exp_h(ell, d, sign, seed, &sizes, max_inst.saturating_sub(written)) {
                    let path = format!("{out}/r7_3_expH_d{d}_ell{ell}_i{written}.json");
                    fs::write(&path, serde_json::to_string_pretty(&rec).unwrap()).unwrap();
                    written += 1;
                }
            }
        }
        "summarise" => {
            let dir = arg(&args, "--dir", ".");
            let o = arg(&args, "--out", "summary.json");
            let s = summarise(&dir);
            fs::write(&o, serde_json::to_string_pretty(&s).unwrap()).unwrap();
            eprintln!("wrote {o}");
        }
        _ => eprintln!(
            "usage: r7_3_conductor_sensitive expA|expB|summarise  (see the module documentation)"
        ),
    }
}
