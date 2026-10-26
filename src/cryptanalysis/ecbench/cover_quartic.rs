//! `transfer.quartic_cover`: an elliptic-curve discrete logarithm carried
//! into the Jacobian of a smooth plane quartic (a non-hyperelliptic curve
//! of genus 3) by pulling back along a double cover, solved there, and
//! checked on the curve.
//!
//! # The cover
//!
//! For `E : y² = f(x) = x³ + ax + b` over `F_p` (`p > 3`), the curve
//!
//! ```text
//!   C : V⁴ = X³Z + aXZ³ + bZ⁴        (projective, coordinates V, X, Z)
//! ```
//!
//! is a smooth plane quartic (its only candidate singularities would need
//! `V = 0` and a repeated root of `f`), and `π : C → E, (V, x) ↦ (x, V²)`
//! has degree 2.  The pullback `π* : Pic⁰(E) → Pic⁰(C)` is a homomorphism
//! with `π_* ∘ π* = [2]`, so it is injective on the odd-order subgroup
//! `⟨G⟩`: `Q = [k]G` becomes `π*Q = [k]π*G` in `J(C)(F_p)`, with the same
//! `k` and the same order `r`.  `J(C)` is isogenous to `E × P` (`P` the
//! Prym surface), so nothing here is expected to be cheaper than rho on
//! `E`; the method exists to measure what a non-hyperelliptic transfer
//! costs, in the same unit as everything else.
//!
//! # The divisors
//!
//! The group law (`plane_quartic`) needs an `X⁴` coefficient of 1 and
//! partials with a `Y³` term, so `C` is used in the model `X = V − cY`,
//! `Y = x`, `Z = z`: `F = (X + cY)⁴ − Y³Z − aYZ³ − bZ⁴`, the smallest `c ≥
//! 1` whose model the smoothness certificate accepts.  For an affine point
//! `P = (x₀, y₀)` of `E` (`y₀ ≠ 0`), the fibre `π*(P)` is the pair `V = ±s`,
//! `s² = y₀` (rational or conjugate); a cubic form vanishes on it iff both
//! `F_p`-components of `h(s − cx₀, x₀, 1)` in `F_p[s]/(s² − y₀)` vanish:
//! two linear conditions.  With `T = (−c : 1 : 0)`, the point of `C` over
//! `O_E`, the class `[π*(P) + T − D∞]` comes from three conditions, and
//! for a fixed affine `B ∈ E(F_p)`
//!
//! ```text
//!   π*(P) = [π*(P + B) + T − D∞] − [π*(B) + T − D∞]
//! ```
//!
//! (`[(P + B)] − [(B)] ~ [(P)] − [(O)]` on `E`).
//!
//! # What is charged
//!
//! The source unit is one affine point addition on `E`.  The Jacobian
//! work (building `π*G`, `π*Q`, and the inner solve) is counted in `F_p`
//! multiplications by the plane-quartic module's counter (an inversion is
//! its Fermat exponentiation's multiplications), and converted at `c_E`,
//! the multiplications of one affine addition on `E` performed with the
//! same counted field arithmetic and measured in the run (64 additions,
//! as `jv_cover::unit_costs` measures its own `c_E`).  The point additions
//! `G + B`, `Q + B` and the check `[k]G = Q` are group operations on `E`.

use crate::cryptanalysis::ecbench::plane_quartic::{
    basis, field_muls, invm, mulm, QClass, QuarticGroup, QuarticJacobian,
};
use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};

/// The cover and its Jacobian for one curve.
pub struct QuarticCover {
    pub c: u64,
    pub jac: QuarticJacobian,
    /// `T = (−c : 1 : 0)`, the point of `C` over `O_E`.
    pub t: [u64; 3],
}

/// The 15 coefficients of `(X + cY)⁴ − Y³Z − aYZ³ − bZ⁴` in the
/// plane-quartic module's monomial order.
pub fn cover_model(p: u64, a: u64, b: u64, c: u64) -> [u64; 15] {
    let m = |x: u64, y: u64| (x as u128 * y as u128 % p as u128) as u64;
    let neg = |x: u64| (p - x % p) % p;
    let c2 = m(c, c);
    let c3 = m(c2, c);
    let c4 = m(c3, c);
    let mut f = [0u64; 15];
    for (k, &mono) in crate::cryptanalysis::ecbench::plane_quartic::monomials(4)
        .iter()
        .enumerate()
    {
        f[k] = match mono {
            (4, 0, 0) => 1,
            (3, 1, 0) => m(4, c),
            (2, 2, 0) => m(6, c2),
            (1, 3, 0) => m(4, c3),
            (0, 4, 0) => c4,
            (0, 3, 1) => neg(1),
            (0, 1, 3) => neg(a),
            (0, 0, 4) => neg(b),
            _ => 0,
        };
    }
    f
}

impl QuarticCover {
    /// Build the cover of `E`, with rational points of `C` found from
    /// abscissae `x = 0, 1, 2, …` where `f(x)` is a fourth power.
    pub fn new(e: &PrimeCurve) -> Result<QuarticCover, String> {
        let p = e.p;
        if !(29..crate::cryptanalysis::ecbench::plane_quartic::QUARTIC_LAW_MAX_P).contains(&p) {
            return Err(format!("the quartic cover takes 29 ≤ p < 2^31, not {p}"));
        }
        // Rational points (V, x, 1) of V⁴ = f(x), V ≠ 0.
        let mut base = Vec::new();
        let mut x = 0u64;
        while base.len() < 12 && x < p {
            let fx = e.rhs(x);
            if fx != 0 {
                if let Some(s) = e.sqrt(fx) {
                    for s in [s, (p - s) % p] {
                        if let Some(v) = e.sqrt(s) {
                            base.push((v, x));
                        }
                    }
                }
            }
            x += 1;
        }
        if base.len() < 4 {
            return Err("the cover has too few rational points".into());
        }
        let mut last = String::new();
        for c in 1..32u64 {
            let f = cover_model(p, e.a % p, e.b % p, c);
            let pts: Vec<[u64; 3]> = base
                .iter()
                .map(|&(v, x)| {
                    [
                        (v + p - (c as u128 * x as u128 % p as u128) as u64) % p,
                        x,
                        1,
                    ]
                })
                .collect();
            match QuarticJacobian::with_points(p, f, pts) {
                Ok(jac) => {
                    return Ok(QuarticCover {
                        c,
                        jac,
                        t: [(p - c % p) % p, 1, 0],
                    })
                }
                Err(err) => last = err,
            }
        }
        Err(format!(
            "no model of the cover was certified smooth: {last}"
        ))
    }

    /// The two `F_p`-conditions on `S_3` of the fibre over `(x₀, y₀)`.
    fn fibre_rows(&self, x0: u64, y0: u64) -> [Vec<u64>; 2] {
        let p = self.jac.p;
        let d = (p - mulm(self.c % p, x0, p)) % p; // X = s + d
                                                   // (s + d)^a = alpha_a + beta_a s, using s² = y0.
        let mut pw = vec![(1u64, 0u64)];
        for _ in 0..3 {
            let (al, be) = *pw.last().unwrap();
            // (al + be s)(d + s) = al d + be y0 + (al + be d) s
            pw.push((
                (mulm(al, d, p) + mulm(be, y0, p)) % p,
                (al + mulm(be, d, p)) % p,
            ));
        }
        let mut r0 = Vec::new();
        let mut r1 = Vec::new();
        for (a, b, _) in basis(3) {
            let yb = (0..b).fold(1u64, |acc, _| mulm(acc, x0, p));
            let (al, be) = pw[a as usize];
            r0.push(mulm(al, yb, p));
            r1.push(mulm(be, yb, p));
        }
        [r0, r1]
    }

    /// `[π*(P) + T − D∞]` for an affine `P = (x₀, y₀)`, `y₀ ≠ 0`.
    pub fn fibre_class(&self, pt: PrimePoint) -> Result<QClass, String> {
        if pt.infinity || pt.y == 0 {
            return Err("the fibre of O or of a 2-torsion point is not used".into());
        }
        let [r0, r1] = self.fibre_rows(pt.x, pt.y);
        self.jac
            .class_from_conditions(vec![r0, r1, self.jac.eval_row3(self.t)])
    }
}

/// `c_E`: `F_p` multiplications per affine addition on `E`, measured on 64
/// additions with the counted arithmetic (`λ = (y₂ − y₁)/(x₂ − x₁)`, `x₃ =
/// λ² − x₁ − x₂`, `y₃ = λ(x₁ − x₃) − y₁`; the inversion by Fermat).
pub fn affine_add_cost(e: &PrimeCurve, g: PrimePoint, h: PrimePoint) -> f64 {
    let p = e.p;
    let before = field_muls();
    let mut acc = g;
    let mut done = 0u32;
    for _ in 0..256 {
        if done == 64 {
            break;
        }
        if acc.infinity || acc.x == h.x {
            acc = g;
            continue;
        }
        let num = (h.y + p - acc.y) % p;
        let den = (h.x + p - acc.x) % p;
        let lam = mulm(num, invm(den, p), p);
        let x3 = (mulm(lam, lam, p) + 2 * p - acc.x - h.x) % p;
        let y3 = (mulm(lam, (acc.x + p - x3) % p, p) + p - acc.y) % p;
        acc = PrimePoint::affine(x3, y3);
        done += 1;
    }
    (field_muls() - before) as f64 / f64::from(done.max(1))
}

/// The transfer of `G` and `Q` into `J(C)`: `(π*G, π*Q)`, with the
/// group operations on `E` it spent (two additions with `B`).
pub fn transfer(
    cover: &QuarticCover,
    e: &PrimeCurve,
    g: PrimePoint,
    q: PrimePoint,
    ops: &mut GroupOps,
) -> Result<(QClass, QClass, PrimePoint), String> {
    let p = e.p;
    let grp = QuarticGroup(&cover.jac);
    // B: the first affine point (x, y), y ≠ 0, with G + B and Q + B affine
    // and off the 2-torsion.
    for x in 1..p.min(1 << 20) {
        let fx = e.rhs(x);
        let Some(y) = e.sqrt(fx) else { continue };
        if y == 0 {
            continue;
        }
        let b = PrimePoint::affine(x, y);
        let gb = e.add(ops, g, b);
        let qb = e.add(ops, q, b);
        if gb.infinity || qb.infinity || gb.y == 0 || qb.y == 0 {
            continue;
        }
        let xb = cover.fibre_class(b)?;
        let nb = grp.neg(xb);
        let mut scratch = GroupOps::default();
        let pg = grp.add(&mut scratch, cover.fibre_class(gb)?, nb);
        let pq = grp.add(&mut scratch, cover.fibre_class(qb)?, nb);
        return Ok((pg, pq, b));
    }
    Err("no base point B found".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::find_prime_order_curve;

    /// `π*` is a homomorphism on `⟨G⟩` with order `r`: `π*([k]G) =
    /// [k]π*(G)` and `[r]π*(G) = 0`, with `π*(G) ≠ 0`.
    #[test]
    fn the_pullback_is_an_injective_homomorphism_on_the_subgroup() {
        for (bits, seed) in [(10u32, 3u64), (12, 5)] {
            let inst = find_prime_order_curve(bits, seed);
            let e = &inst.curve;
            let cover = QuarticCover::new(e).unwrap();
            let grp = QuarticGroup(&cover.jac);
            let g = inst.generator_point();
            let mut ops = GroupOps::default();
            for k in [2u64, 5, inst.r - 1, inst.r / 3 + 1] {
                let q = e.mul(&mut ops, g, k);
                let (pg, pq, _) = transfer(&cover, e, g, q, &mut ops).unwrap();
                assert_ne!(pg, cover.jac.zero(), "π*G ≠ 0");
                let mut s = GroupOps::default();
                assert_eq!(grp.mul(&mut s, pg, k), pq, "bits {bits}, k {k}");
                assert_eq!(grp.mul(&mut s, pg, inst.r), cover.jac.zero());
            }
            let c = affine_add_cost(e, g, e.mul(&mut ops, g, 7));
            assert!(c > 3.0, "c_E = {c}");
        }
    }
}
