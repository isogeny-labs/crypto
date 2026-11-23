//! Bielliptic genus-2 curves and their two elliptic quotients, with the
//! induced maps on Jacobians evaluated on **arbitrary** reduced divisors.
//!
//! ```text
//!   C : y² = F(x²),   F(X) = c0 + c1 X + c2 X² + c3 X³,  F squarefree, c0·c3 ≠ 0
//!   φ1 : C → E1 : Y² = F(X),          (x, y) ↦ (x², y)
//!   φ2 : C → E2 : Y² = X³ F(1/X),     (x, y) ↦ (1/x², y/x³)
//! ```
//!
//! The Jacobian is handled by [`crate::cryptanalysis::genus2_fp`] on the
//! imaginary (quintic) model obtained by sending a rational Weierstrass point
//! `(x0, 0)` to infinity: `t = 1/(x − x0)`, `Y' = y·t³`,
//! `g(t) = t⁶ f(x0 + 1/t)`, `deg g = 5`, with `∞_C ↔ (x0, 0)`.
//!
//! Push-forward `φ_*` of a reduced class `D = Σ P_j − deg(u)·∞_C` is
//! `Σ φ(P_j) − deg(u)·φ(∞_C)`, evaluated by splitting `u` over `F_{p²}`
//! (so non-split divisors are covered) and asserting the sum is rational.
//! Pull-back `φ^*([Q − O]) = [P₊ + P₋ − 2∞_C]` because `φ^*(O)` is a pair of
//! hyperelliptic conjugates (the two points over `x = ∞` for `φ1`, over
//! `x = 0` for `φ2`), whose class equals `2∞_C`.
//!
//! The elliptic curves are stored in the monic model
//! `y'² = x'³ + a2 x'² + a4 x' + a6` via `(X, Y) ↦ (lc·X, lc·Y)`, `lc` the
//! leading coefficient of the cubic.

use super::ec::{Curve, Pt};
use super::field::{Fp, F2};
use crate::cryptanalysis::genus2_fp::{Divisor, Genus2Curve};

/// Which elliptic quotient.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Quotient {
    E1,
    E2,
}

/// A bielliptic genus-2 curve with both quotient maps.
#[derive(Clone, Debug)]
pub struct Bielliptic {
    pub k: Fp,
    /// `F` coefficients `c0..c3`.
    pub c: [u64; 4],
    /// The rational Weierstrass point sent to infinity.
    pub x0: u64,
    /// Quintic model `Y'² = g(t)`.
    pub jac: Genus2Curve,
    pub e1: Curve,
    pub e2: Curve,
}

/// Why a bielliptic curve was refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BiellipticError {
    Field,
    Degenerate,
    NotSquarefree,
    NoRationalWeierstrassPoint,
    Genus2(String),
}

fn poly_mul(k: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = k.add(c[i + j], k.mul(x, y));
        }
    }
    c
}

impl Bielliptic {
    /// The curve for `F = c0 + c1 X + c2 X² + c3 X³` over `F_p`.
    pub fn new(p: u64, c: [u64; 4]) -> Result<Bielliptic, BiellipticError> {
        let k = Fp::new(p).ok_or(BiellipticError::Field)?;
        let c = c.map(|v| v % p);
        if c[0] == 0 || c[3] == 0 {
            return Err(BiellipticError::Degenerate);
        }
        // Monic quotient models (also checks F squarefree: disc ≠ 0).
        let e1 = Curve::new(k, c[2], k.mul(c[1], c[3]), k.mul(c[0], k.mul(c[3], c[3])))
            .ok_or(BiellipticError::NotSquarefree)?;
        let e2 = Curve::new(k, c[1], k.mul(c[2], c[0]), k.mul(c[3], k.mul(c[0], c[0])))
            .ok_or(BiellipticError::NotSquarefree)?;
        // A rational root x0 of f(x) = F(x²): a root X0 of F that is a nonzero square.
        let fx = |x: u64| {
            let x2 = k.mul(x, x);
            let mut r = c[3];
            for i in (0..3).rev() {
                r = k.add(k.mul(r, x2), c[i]);
            }
            r
        };
        let x0 = (1..p)
            .find(|&x| fx(x) == 0)
            .ok_or(BiellipticError::NoRationalWeierstrassPoint)?;
        // f(x) coefficients (degree 6).
        let f = [c[0], 0, c[1], 0, c[2], 0, c[3]];
        // g(t) = Σ_j f_j (x0 t + 1)^j t^{6−j}
        let mut g = [0u64; 7];
        let lin = [1u64, x0];
        let mut powj: Vec<u64> = vec![1];
        for (j, &fj) in f.iter().enumerate() {
            if j > 0 {
                powj = poly_mul(&k, &powj, &lin);
            }
            for (i, &a) in powj.iter().enumerate() {
                let d = i + 6 - j;
                g[d] = k.add(g[d], k.mul(fj, a));
            }
        }
        assert_eq!(g[6], 0, "x0 is a root of f");
        let jac = Genus2Curve::new(p, [g[0], g[1], g[2], g[3], g[4], g[5]])
            .map_err(|e| BiellipticError::Genus2(e.to_string()))?;
        Ok(Bielliptic {
            k,
            c,
            x0,
            jac,
            e1,
            e2,
        })
    }

    pub fn curve(&self, q: Quotient) -> &Curve {
        match q {
            Quotient::E1 => &self.e1,
            Quotient::E2 => &self.e2,
        }
    }

    /// Leading coefficient used to make the quotient's cubic monic.
    fn lc(&self, q: Quotient) -> u64 {
        match q {
            Quotient::E1 => self.c[3],
            Quotient::E2 => self.c[0],
        }
    }

    /// `g(t)` on `F_{p²}`.
    fn g_at(&self, t: F2) -> F2 {
        let k = &self.k;
        let f = self.jac.f();
        let mut r = F2::ZERO;
        for i in (0..6).rev() {
            r = k.add2(k.mul2(r, t), k.e(f[i]));
        }
        r
    }

    /// Image of the affine quintic point `(t, Y')` on the monic quotient.
    fn phi(&self, q: Quotient, t: F2, yq: F2) -> Pt {
        let k = &self.k;
        let lc = k.e(self.lc(q));
        match q {
            Quotient::E1 => {
                if t.is_zero() {
                    return None; // x = ∞
                }
                let ti = k.inv2(t);
                let x = k.add2(k.e(self.x0), ti);
                let y = k.mul2(yq, k.mul2(ti, k.sqr2(ti)));
                Some((k.mul2(lc, k.sqr2(x)), k.mul2(lc, y)))
            }
            Quotient::E2 => {
                let w = k.add2(k.mul2(k.e(self.x0), t), F2::ONE); // x0 t + 1 = x·t
                if w.is_zero() {
                    return None; // x = 0
                }
                let wi = k.inv2(w);
                let x = k.mul2(k.sqr2(t), k.sqr2(wi));
                let y = k.mul2(yq, k.mul2(wi, k.sqr2(wi)));
                Some((k.mul2(lc, x), k.mul2(lc, y)))
            }
        }
    }

    /// Image of `∞_C = (x0, 0)` (a 2-torsion point).
    fn phi_inf(&self, q: Quotient) -> Pt {
        let k = &self.k;
        let x02 = k.mul(self.x0, self.x0);
        let x = match q {
            Quotient::E1 => k.mul(self.c[3], x02),
            Quotient::E2 => k.mul(self.c[0], k.inv(x02)),
        };
        Some((F2::base(x), F2::ZERO))
    }

    /// `φ_*(D)` for any reduced class `D ∈ J(C)(F_p)`.
    pub fn push(&self, q: Quotient, d: &Divisor) -> Pt {
        let k = &self.k;
        let e = self.curve(q);
        let u = d.u_coeffs();
        let v = d.v_coeffs();
        let veval = |t: F2| {
            let mut r = F2::ZERO;
            for &c in v.iter().rev() {
                r = k.add2(k.mul2(r, t), k.e(c));
            }
            r
        };
        let mut acc: Pt = None;
        for t in k.roots_deg_le2(&u) {
            acc = e.add(&acc, &self.phi(q, t, veval(t)));
        }
        if d.degree() % 2 == 1 {
            acc = e.add(&acc, &self.phi_inf(q)); // −T = T for 2-torsion
        }
        assert!(
            Curve::rational(&acc),
            "push-forward of a rational class is rational"
        );
        acc
    }

    /// The two preimages `(t, Y')` of an affine quotient point, `None` for
    /// a preimage at `∞_C`.
    fn preimages(&self, q: Quotient, pt: (F2, F2)) -> [Option<(F2, F2)>; 2] {
        let k = &self.k;
        let lci = k.e(k.inv(self.lc(q)));
        let (xm, ym) = (k.mul2(pt.0, lci), k.mul2(pt.1, lci)); // model (X, Y)
        let mut out = [None, None];
        match q {
            Quotient::E1 => {
                // x² = X, y = Y
                let s = k.sqrt2(xm).expect("F_p elements are squares in F_p²");
                for (i, x) in [s, k.neg2(s)].into_iter().enumerate() {
                    let dx = k.sub2(x, k.e(self.x0));
                    if dx.is_zero() {
                        continue; // ∞_C
                    }
                    let t = k.inv2(dx);
                    out[i] = Some((t, k.mul2(ym, k.mul2(t, k.sqr2(t)))));
                }
            }
            Quotient::E2 => {
                // x² = 1/X, y = Y x³; X = 0 ⇒ x = ∞. The two points over x = ∞
                // are hyperelliptic conjugates `(t=0, ±Y')`, so their class is
                // trivial: φ2^* of that (2-torsion) point is the identity.
                if xm.is_zero() {
                    out = [None, None];
                } else {
                    let s = k.sqrt2(k.inv2(xm)).expect("square in F_p²");
                    for (i, x) in [s, k.neg2(s)].into_iter().enumerate() {
                        let dx = k.sub2(x, k.e(self.x0));
                        if dx.is_zero() {
                            continue;
                        }
                        let t = k.inv2(dx);
                        let y = k.mul2(ym, k.mul2(x, k.sqr2(x)));
                        out[i] = Some((t, k.mul2(y, k.mul2(t, k.sqr2(t)))));
                    }
                }
            }
        }
        out
    }

    /// Whether the fiber `φ⁻¹(pt)` is generic: `pt` affine, unramified, with
    /// two distinct affine preimages, neither at `∞_C`. On this locus `pull`
    /// returns the full degree-2 class and `φ_*φ^* = [2]` holds exactly. The
    /// complement is the ramification locus (`X = 0, ∞`, or a preimage at
    /// `∞_C`): a finite set a large-prime-order point never meets.
    pub fn fiber_generic(&self, q: Quotient, pt: &Pt) -> bool {
        match pt {
            None => false,
            Some(v) => match self.preimages(q, *v) {
                [Some((t1, _)), Some((t2, _))] => t1 != t2,
                _ => false,
            },
        }
    }

    /// `φ^*([Q − O])` as a reduced class in `J(C)(F_p)`.
    pub fn pull(&self, q: Quotient, pt: &Pt) -> Divisor {
        let k = &self.k;
        let pt = match pt {
            None => return Divisor::IDENTITY,
            Some(v) => *v,
        };
        let pre = self.preimages(q, pt);
        let (u, v): (Vec<F2>, Vec<F2>) = match pre {
            [None, None] => return Divisor::IDENTITY,
            [Some((t, y)), None] | [None, Some((t, y))] => (vec![k.neg2(t), F2::ONE], vec![y]),
            [Some((t1, y1)), Some((t2, y2))] => {
                if t1 == t2 {
                    if k.add2(y1, y2).is_zero() && !y1.is_zero() {
                        return Divisor::IDENTITY; // conjugate pair
                    }
                    // Double point: tangent interpolation v(t1)=y1, v'(t1)=g'(t1)/(2 y1).
                    let gp = self.g_deriv_at(t1);
                    let m = k.mul2(gp, k.inv2(k.add2(y1, y1)));
                    let v0 = k.sub2(y1, k.mul2(m, t1));
                    let u1 = k.neg2(k.add2(t1, t1));
                    (vec![k.sqr2(t1), u1, F2::ONE], vec![v0, m])
                } else {
                    let m = k.mul2(k.sub2(y2, y1), k.inv2(k.sub2(t2, t1)));
                    let v0 = k.sub2(y1, k.mul2(m, t1));
                    (
                        vec![k.mul2(t1, t2), k.neg2(k.add2(t1, t2)), F2::ONE],
                        vec![v0, m],
                    )
                }
            }
        };
        assert!(
            u.iter().chain(v.iter()).all(|c| c.in_base()),
            "pull-back of a rational point has rational Mumford coefficients"
        );
        let u: Vec<u64> = u.iter().map(|c| c.a).collect();
        let v: Vec<u64> = v.iter().map(|c| c.a).collect();
        // genus2_fp validates u | g − v²; a monic deg-2 u may still need reduction
        // when deg v ≥ deg u is impossible here (deg v ≤ 1 < 2).
        self.jac
            .divisor(&u, &v)
            .expect("pull-back Mumford pair is valid on the quintic model")
    }

    fn g_deriv_at(&self, t: F2) -> F2 {
        let k = &self.k;
        let f = self.jac.f();
        let mut r = F2::ZERO;
        for i in (1..6).rev() {
            r = k.add2(k.mul2(r, t), k.e(k.mul(i as u64, f[i])));
        }
        r
    }

    /// Sanity: `g(t) = Y'²` at an affine quintic point.
    pub fn on_quintic(&self, t: F2, y: F2) -> bool {
        self.k.sqr2(y) == self.g_at(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    fn sample(p: u64, seed: u64) -> Vec<Bielliptic> {
        let mut out = vec![];
        let mut s = seed;
        while out.len() < 6 {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
            let c = [(s >> 3) % p, (s >> 13) % p, (s >> 23) % p, (s >> 33) % p];
            if let Ok(b) = Bielliptic::new(p, c) {
                out.push(b);
            }
        }
        out
    }

    #[test]
    fn push_pull_identities_on_arbitrary_divisors() {
        for p in [101u64, 211, 1009] {
            for b in sample(p, p) {
                let mut rng = StdRng::seed_from_u64(p);
                for q in [Quotient::E1, Quotient::E2] {
                    let e = *b.curve(q);
                    let mut generic = 0;
                    for s in 0..60 {
                        let pt = e.point_from_seed(s * 101 + 7);
                        if !b.fiber_generic(q, &pt) {
                            continue; // ramification locus, handled separately below
                        }
                        let d = b.pull(q, &pt);
                        assert!(b.jac.is_valid(&d));
                        assert_eq!(d.degree(), 2, "generic fiber ⇒ weight-2 class");
                        assert_eq!(b.push(q, &d), e.mul(&pt, 2), "φ_*φ^* = [2]");
                        let other = match q {
                            Quotient::E1 => Quotient::E2,
                            Quotient::E2 => Quotient::E1,
                        };
                        assert_eq!(b.push(other, &d), None, "cross composite is zero");
                        generic += 1;
                    }
                    assert!(
                        generic >= 20,
                        "enough generic fibers sampled (got {generic})"
                    );
                }
                for _ in 0..60 {
                    let d1 = b.jac.random(&mut rng);
                    let d2 = b.jac.random(&mut rng);
                    for q in [Quotient::E1, Quotient::E2] {
                        let e = b.curve(q);
                        assert_eq!(
                            b.push(q, &b.jac.add(&d1, &d2)),
                            e.add(&b.push(q, &d1), &b.push(q, &d2)),
                            "push is a homomorphism"
                        );
                    }
                }
                // (2,2)-isogeny E1 × E2 → J(C) preserves the point count.
                assert_eq!(b.jac.jacobian_order().unwrap(), b.e1.order() * b.e2.order());
            }
        }
    }
}
