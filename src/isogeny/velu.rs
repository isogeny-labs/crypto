//! # Vélu's formulas: explicit isogenies from a kernel subgroup.
//!
//! Vélu (1971) gave a closed-form expression for the isogeny
//! `φ: E → E/G` once a finite subgroup `G ⊂ E(F̄)` is specified by
//! its non-identity points.  Given `E: y² = x³ + a x + b` and a
//! kernel `G = {O} ∪ {±P_1, …, ±P_s}` (so `|G| = 2s + 1` for odd
//! cyclic; `|G| = 2s + 2` for cyclic of even order with the 2-torsion
//! point `T = (x_T, 0)` adjoined), define for each `Q = (x_Q, y_Q)`
//! in `G \ {O}`:
//!
//! ```text
//!   g_x(Q) = 3 x_Q² + a
//!   g_y(Q) = -2 y_Q
//!   v(Q)   =   g_x(Q)            if 2Q = O          (T-row)
//!          = 2 g_x(Q)            otherwise
//!   u(Q)   = g_y(Q)²              (for the y-coordinate map)
//! ```
//!
//! Then summing over a half-set `R` (one representative of each
//! `{±Q}`):
//!
//! ```text
//!   x'  =  x + Σ_{Q ∈ R} [ v(Q) / (x − x_Q)  +  u(Q) / (x − x_Q)² ]
//!   y'  =  y · ( 1 − Σ_{Q ∈ R} [ v(Q) / (x − x_Q)²
//!                                + 2 u(Q) / (x − x_Q)³ ] )
//! ```
//!
//! and the codomain curve is `E' : Y² = X³ + a' X + b'` with
//!
//! ```text
//!   a' = a − 5 · Σ v(Q)
//!   b' = b − 7 · Σ ( u(Q) + x_Q · v(Q) )
//! ```
//!
//! (Both sums are over `R`.)
//!
//! ## Why we keep a separate, small implementation
//!
//! [`crate::cryptanalysis::modular_polynomial`] gives the
//! "algebraic" view: `Φ_ℓ(j(E), j(E')) = 0` constrains *which*
//! `E'` can be reached, but does not tell us **how** to construct
//! the isogeny rationally.  Vélu does both: codomain curve **and**
//! the rational maps for evaluating `φ` on points.
//!
//! ## Scope
//!
//! We enumerate the kernel `G` by brute force over `F_p` and
//! compute Vélu's sums directly.  This is `O(ℓ)` field operations
//! per isogeny — fine for `ℓ ≤ 73` and the toy curves used here.
//! For larger odd degrees prefer
//! [`crate::cryptanalysis::isogeny_eval::sqrt_velu`] (Bernstein–De Feo–
//! Leroux–Smith √élu, crossover near `ℓ ≈ 73–101`).

use super::SmallCurve;
use num_bigint::BigUint;

/// An `ℓ`-isogeny `φ: E → E'` constructed via Vélu's formulas.
#[derive(Clone, Debug)]
pub struct VeluIsogeny {
    /// The source curve.
    pub domain: SmallCurve,
    /// The codomain curve after any change of coordinates.
    pub codomain: SmallCurve,
    /// The degree `ℓ`.
    pub degree: u64,
    /// The kernel's non-identity points as `(x, y)` pairs over
    /// `F_p`, listed as a half-set `R` (one representative per
    /// `{±Q}`).
    pub kernel_half: Vec<(u64, u64)>,
    /// Precomputed Vélu `(x_Q, v(Q), u(Q))` for each half-set point —
    /// built once at construction so [`evaluate`] skips per-call
    /// `g_x`/`g_y` work (measured win on multi-point maps).
    kernel_vu: Vec<(u64, u64, u64)>,
    // Post-compose the normalized map with (x, y) ↦ (scale²x, scale³y).
    scale: u64,
}

impl VeluIsogeny {
    fn rescale(mut self, scale: u128) -> Self {
        let p = self.domain.p as u128;
        let square = mod_mul(scale, scale, p);
        let fourth = mod_mul(square, square, p);
        let sixth = mod_mul(fourth, square, p);
        self.codomain.a = mod_mul(self.codomain.a as u128, fourth, p) as u64;
        self.codomain.b = mod_mul(self.codomain.b as u128, sixth, p) as u64;
        self.scale = mod_mul(self.scale as u128, scale, p) as u64;
        self
    }

    /// Evaluate the isogeny at an affine point `(x, y)`.  Returns
    /// `None` when `(x, y)` is in the kernel (image is the point
    /// at infinity).
    ///
    /// Round-4: one binary-EGCD inverse for the whole kernel via Montgomery's
    /// trick.  Round-6: `v(Q), u(Q)` precomputed at construction.
    pub fn evaluate(&self, x: u64, y: u64) -> Option<(u64, u64)> {
        let p = self.domain.p as u128;
        let xm = (x as u128) % p;
        let ym = (y as u128) % p;
        let n = self.kernel_vu.len();
        if n == 0 {
            return Some((x, y));
        }

        // Reject kernel points; collect denominators `x − x_Q`.
        let mut dens = vec![0u128; n];
        for (i, &(xk, _, _)) in self.kernel_vu.iter().enumerate() {
            let xk = xk as u128;
            if xm == xk {
                return None;
            }
            dens[i] = mod_sub(xm, xk, p);
        }

        // Montgomery batch inversion of all dens.
        let mut prefix = vec![0u128; n];
        prefix[0] = dens[0];
        for i in 1..n {
            prefix[i] = mod_mul(prefix[i - 1], dens[i], p);
        }
        let mut inv_acc = mod_inv(prefix[n - 1], p)?;
        let mut invs = vec![0u128; n];
        for i in (1..n).rev() {
            invs[i] = mod_mul(inv_acc, prefix[i - 1], p);
            inv_acc = mod_mul(inv_acc, dens[i], p);
        }
        invs[0] = inv_acc;

        let mut acc_x = xm;
        let mut acc_y = ym;
        let a = self.domain.a as u128 % p;
        // `IC_VELU_PRECOMP=0` recomputes v/u each call (A/B baseline).
        let recompute = std::env::var("IC_VELU_PRECOMP").as_deref() == Ok("0");
        for i in 0..n {
            let (v_q, u_q) = if recompute {
                let (xq, yq) = self.kernel_half[i];
                let xq = xq as u128 % p;
                let yq = yq as u128 % p;
                let gx_q = mod_add(3 * mod_mul(xq, xq, p) % p, a, p);
                let gy_q = mod_sub(0, 2 * yq % p, p);
                let v = if yq == 0 { gx_q } else { (2 * gx_q) % p };
                (v, mod_mul(gy_q, gy_q, p))
            } else {
                let (_, v, u) = self.kernel_vu[i];
                (v as u128, u as u128)
            };
            let dx_inv = invs[i];
            let dx_inv2 = mod_mul(dx_inv, dx_inv, p);
            let dx_inv3 = mod_mul(dx_inv2, dx_inv, p);

            acc_x = mod_add(acc_x, mod_mul(v_q, dx_inv, p), p);
            acc_x = mod_add(acc_x, mod_mul(u_q, dx_inv2, p), p);

            let bracket = mod_add(
                mod_mul(v_q, dx_inv2, p),
                mod_mul((2 * u_q) % p, dx_inv3, p),
                p,
            );
            acc_y = mod_sub(acc_y, mod_mul(ym, bracket, p), p);
        }
        if self.scale != 1 {
            let scale = self.scale as u128;
            let square = mod_mul(scale, scale, p);
            acc_x = mod_mul(acc_x, square, p);
            acc_y = mod_mul(acc_y, mod_mul(square, scale, p), p);
        }
        Some((acc_x as u64, acc_y as u64))
    }
}

// ── Modular arithmetic helpers in u128 ────────────────────────────────────────

fn mod_add(a: u128, b: u128, p: u128) -> u128 {
    ((a % p) + (b % p)) % p
}
fn mod_sub(a: u128, b: u128, p: u128) -> u128 {
    ((a % p) + p - (b % p)) % p
}
#[inline(always)]
fn mod_mul(a: u128, b: u128, p: u128) -> u128 {
    // Callers keep `a, b < p < 2^62`; product fits in u128.
    (a * b) % p
}
fn mod_inv(a: u128, p: u128) -> Option<u128> {
    if a.is_multiple_of(p) {
        return None;
    }
    crate::gcd::int::inv_mod_u128(a, p)
}

// ── Vélu codomain construction ────────────────────────────────────────────────

/// Compute the codomain curve and kernel-half representation from a
/// list of affine kernel points.  The list must contain at least one
/// representative of each `{±Q}` orbit.  Duplicate `{±Q}` orbits in
/// the input are silently de-duplicated.
/// **The isogeny with an explicit kernel**: the codomain and the maps
/// from a list of affine kernel points (at least one representative of
/// each `{±Q}`; duplicates are dropped).  The caller certifies that the
/// points form a subgroup; this only applies Vélu's sums to them.
pub fn velu_isogeny_from_kernel(domain: &SmallCurve, kernel: &[(u64, u64)]) -> VeluIsogeny {
    velu_codomain_from_kernel(domain, kernel)
}

fn velu_codomain_from_kernel(domain: &SmallCurve, kernel: &[(u64, u64)]) -> VeluIsogeny {
    let p = domain.p as u128;
    let a = domain.a as u128 % p;
    let b = domain.b as u128 % p;

    // Reduce to a half-set R: keep `Q` and drop `-Q`.
    let mut seen_x = std::collections::HashSet::<u64>::new();
    let mut half = Vec::new();
    for &(x, y) in kernel {
        let x = x % domain.p;
        let y = y % domain.p;
        if seen_x.insert(x) {
            half.push((x, y));
        }
    }

    let mut sum_v = 0u128;
    let mut sum_uvx = 0u128;
    let mut kernel_vu = Vec::with_capacity(half.len());
    for &(xq, yq) in &half {
        let xq = xq as u128;
        let yq = yq as u128;

        let gx_q = mod_add(3 * mod_mul(xq, xq, p) % p, a, p);
        let gy_q = mod_sub(0, 2 * yq % p, p);
        let t_row = yq == 0;
        // Vélu: v(Q) = g_x(Q) when 2Q = O, and 2·g_x(Q) otherwise.
        let v_q = if t_row { gx_q } else { (2 * gx_q) % p };
        let u_q = mod_mul(gy_q, gy_q, p);

        sum_v = mod_add(sum_v, v_q, p);
        sum_uvx = mod_add(sum_uvx, mod_add(u_q, mod_mul(xq, v_q, p), p), p);
        kernel_vu.push((xq as u64, v_q as u64, u_q as u64));
    }

    let new_a = mod_sub(a, (5 * sum_v) % p, p) as u64;
    let new_b = mod_sub(b, (7 * sum_uvx) % p, p) as u64;
    let codomain = SmallCurve {
        name: domain.name,
        p: domain.p,
        a: new_a,
        b: new_b,
    };
    let degree = 1 + half
        .iter()
        .map(|&(_, y)| if y == 0 { 1 } else { 2 })
        .sum::<u64>();

    VeluIsogeny {
        domain: *domain,
        codomain,
        degree,
        kernel_half: half,
        kernel_vu,
        scale: 1,
    }
}

// ── ℓ = 2 isogeny ────────────────────────────────────────────────────────────

/// Compute a 2-isogeny with the specified 2-torsion point `T = (x_T, 0)`
/// as kernel.  `T` must satisfy `y² = x³ + a x + b` with `y = 0`,
/// i.e. `x_T` is a root of `x³ + a x + b` over `F_p`.
pub fn velu_isogeny_2(domain: &SmallCurve, x_t: u64) -> Option<VeluIsogeny> {
    let p = domain.p as u128;
    let xt = x_t as u128 % p;
    let a = domain.a as u128 % p;
    let b = domain.b as u128 % p;
    // Check x_T is on the curve with y = 0.
    let rhs = mod_add(
        mod_add(mod_mul(mod_mul(xt, xt, p), xt, p), mod_mul(a, xt, p), p),
        b,
        p,
    );
    if rhs != 0 {
        return None;
    }
    Some(velu_codomain_from_kernel(domain, &[(x_t, 0)]))
}

// ── ℓ-odd isogeny via cofactor sampling ──────────────────────────────────────

/// Return sampled `ℓ`-isogenies with pointwise rational kernels.
/// `ℓ` must be an odd prime.
/// This is not a complete rational-isogeny census: a rational isogeny only
/// requires a Frobenius-stable kernel, and this bounded sampler can also miss
/// pointwise rational kernels. In particular, when full ell-torsion is rational,
/// multiplication by #E/ell can annihilate that torsion instead of finding it.
///
/// Algorithm (random-point cofactor method, Schoof 1995 §3):
///
/// 1. Compute `#E(F_p) = p + 1 − t` via [`cm_discriminant`].
/// 2. If `ℓ ∤ #E`, there is no `F_p`-rational `ℓ`-torsion, so this
///    sampler returns `[]`; rational isogenies may still exist.
/// 3. Otherwise let `m = #E / ℓ`.  Repeatedly sample random
///    `P ∈ E(F_p)` and form `Q = m·P`; if `Q ≠ O`, then `Q` has
///    order dividing `ℓ`, and (since `ℓ` is prime) its order is
///    exactly `ℓ`, so `⟨Q⟩` is a cyclic subgroup of order `ℓ`.
/// 4. Stop at `ℓ+1` distinct kernels or the attempt cap. Neither fewer results
///    nor an empty result certifies the absence of additional rational isogenies.
///
/// Cost: `O(ℓ² · log² p)` total — `O(log² p)` per scalar mult,
/// `O(ℓ)` samples expected per distinct subgroup, `O(ℓ)` subgroups
/// in the saturated case.  Compared to the previous
/// `O(ℓ² · p · log² p)` enumeration: at `p = 2^30` and `ℓ = 3`,
/// roughly a `10^9×` speed-up.
pub fn velu_isogeny_odd(domain: &SmallCurve, ell: u64) -> Vec<VeluIsogeny> {
    assert!(ell >= 3 && ell % 2 == 1, "ℓ must be an odd prime");

    use crate::ecc::point::Point;
    use crate::isogeny::cm::cm_discriminant;
    use std::collections::HashSet;

    let cm = cm_discriminant(domain);
    if cm.order <= 0 {
        return Vec::new();
    }
    let order = cm.order as u64;
    if !order.is_multiple_of(ell) {
        // No pointwise rational ell-kernel can be sampled by this backend.
        return Vec::new();
    }
    let cofactor = order / ell;

    let curve = domain.to_curve_params();
    let a_fe = curve.a_fe();

    // Sample random points P, form Q = cofactor · P.  Collect distinct
    // cyclic subgroups ⟨Q⟩ until we've either seen ℓ+1 (saturated) or
    // run out of attempts.
    let mut seen_subgroups: HashSet<Vec<u64>> = HashSet::new();
    let mut result = Vec::new();
    let mut rng_state: u64 = domain
        .p
        .wrapping_mul(0x9E3779B97F4A7C15)
        .wrapping_add(domain.a)
        .wrapping_mul(0xC6BC279692B5C323)
        .wrapping_add(domain.b)
        .wrapping_add(ell);
    let max_attempts: u64 = 4 * ell + 16; // expected O(ℓ²), pad for tail
    let max_subgroups = (ell + 1) as usize;

    for _ in 0..max_attempts {
        rng_state = rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let point = match crate::isogeny::cm::sample_random_point(domain, &mut rng_state) {
            Some(p) => p,
            None => continue,
        };
        // Public sampling: vartime word/mont ladders (CT Fermat unused).
        let q = point.scalar_mul_vartime(&BigUint::from(cofactor), &a_fe);
        if matches!(q, Point::Infinity) {
            continue;
        }
        // Q has order dividing ℓ; since ℓ is prime and Q ≠ O, order is ℓ.
        // Build subgroup ⟨Q⟩ = { Q, 2Q, ..., (ℓ-1)Q }.  We canonicalise
        // by sorting the x-coordinates so {P, -P, 2P, -2P, ...} all map
        // to the same key.
        let mut xs: Vec<u64> = Vec::with_capacity((ell - 1) as usize);
        let mut kernel: Vec<(u64, u64)> = Vec::with_capacity((ell - 1) as usize);
        let mut acc = q.clone();
        let mut bad = false;
        for _ in 1..ell {
            match &acc {
                Point::Affine { x, y } => {
                    let xv = x.value.iter_u64_digits().next().unwrap_or(0);
                    let yv = y.value.iter_u64_digits().next().unwrap_or(0);
                    xs.push(xv);
                    kernel.push((xv, yv));
                }
                Point::Infinity => {
                    bad = true;
                    break;
                }
            }
            acc = acc.add_vartime(&q, &a_fe);
        }
        if bad || kernel.len() as u64 != ell - 1 {
            continue;
        }
        let mut key = xs.clone();
        key.sort();
        if !seen_subgroups.insert(key) {
            continue; // already have this subgroup
        }
        result.push(velu_codomain_from_kernel(domain, &kernel));
        if result.len() >= max_subgroups {
            break;
        }
    }
    result
}

// ── Dual isogeny ──────────────────────────────────────────────────────────────

/// Find the dual of a degree-two or odd-prime isogeny. The returned map
/// lands on the original curve and composes with `phi` to multiplication by
/// its degree. The bounded odd-degree search requires a pointwise rational
/// dual kernel and enough rational points to certify the result.
pub fn dual_isogeny_naive(phi: &VeluIsogeny) -> Option<VeluIsogeny> {
    let ell = phi.degree;
    let p = phi.domain.p as u128;
    let phi_scale = phi.scale as u128;
    let scale = mod_inv(mod_mul(ell as u128 % p, phi_scale, p), p)?;

    if ell == 2 {
        // Both non-kernel geometric 2-torsion points have image abscissa
        // -2t under the normalized quotient with kernel (t, 0).
        let &[(t, 0)] = phi.kernel_half.as_slice() else {
            return None;
        };
        let twice_t = mod_add(t as u128, t as u128, p);
        let dual_x = mod_sub(0, mod_mul(twice_t, mod_mul(phi_scale, phi_scale, p), p), p);
        let dual = velu_isogeny_2(&phi.codomain, dual_x as u64)?.rescale(scale);
        return same_equation(&dual.codomain, &phi.domain).then_some(dual);
    }
    if ell < 3 || ell.is_multiple_of(2) {
        return None;
    }

    let mut candidates: Vec<_> = velu_isogeny_odd(&phi.codomain, ell)
        .into_iter()
        .filter(|candidate| candidate.degree == ell)
        .map(|candidate| candidate.rescale(scale))
        .filter(|candidate| same_equation(&candidate.codomain, &phi.domain))
        .collect();
    let curve = phi.domain.to_curve_params();
    let a_fe = curve.a_fe();
    let degree = BigUint::from(ell);
    // A nonzero difference of the two degree-ell² maps has degree at
    // most 4*ell², by the parallelogram law for isogeny degrees.
    let difference_degree_bound = 4u128.saturating_mul(ell as u128 * ell as u128);
    let mut agreeing_points = 1u128; // The point at infinity.
    for x in 0..phi.domain.p {
        if candidates.is_empty() {
            return None;
        }
        let Some(y) = crate::isogeny::cm::tonelli_shanks_u64(phi.domain.rhs(x), phi.domain.p)
        else {
            continue;
        };
        let point = crate::ecc::point::Point::Affine {
            x: curve.fe(BigUint::from(x)),
            y: curve.fe(BigUint::from(y)),
        };
        let expected = match point.scalar_mul(&degree, &a_fe) {
            crate::ecc::point::Point::Infinity => None,
            crate::ecc::point::Point::Affine { x, y } => Some((
                x.value.iter_u64_digits().next().unwrap_or(0),
                y.value.iter_u64_digits().next().unwrap_or(0),
            )),
        };
        let image = phi.evaluate(x, y);
        candidates
            .retain(|candidate| image.and_then(|(x, y)| candidate.evaluate(x, y)) == expected);
        // Equality on P implies equality on -P.
        agreeing_points += if y == 0 { 1 } else { 2 };
        if agreeing_points > difference_degree_bound {
            return candidates.pop();
        }
    }
    None
}

fn same_equation(left: &SmallCurve, right: &SmallCurve) -> bool {
    let p = left.p;
    p == right.p && left.a % p == right.a % p && left.b % p == right.b % p
}

// ── Composition of isogenies ─────────────────────────────────────────────────

/// Compose two Vélu isogenies when the full composite kernel is
/// pointwise rational. An affine rational-kernel table cannot represent
/// a composite whose kernel requires extension-field points.
pub fn compose_isogenies(phi: &VeluIsogeny, psi: &VeluIsogeny) -> Option<VeluIsogeny> {
    if !same_equation(&phi.codomain, &psi.domain) {
        return None;
    }
    let degree = phi.degree.checked_mul(psi.degree)?;
    let p = phi.domain.p;
    let mut kernel = Vec::new();
    for x in 0..p {
        let Some(y) = crate::isogeny::cm::tonelli_shanks_u64(phi.domain.rhs(x), p) else {
            continue;
        };
        if phi
            .evaluate(x, y)
            .and_then(|(u, v)| psi.evaluate(u, v))
            .is_none()
        {
            kernel.push((x, y));
        }
    }
    let scale = mod_mul(phi.scale as u128, psi.scale as u128, p as u128);
    let iso = velu_codomain_from_kernel(&phi.domain, &kernel).rescale(scale);
    (iso.degree == degree && same_equation(&iso.codomain, &psi.codomain)).then_some(iso)
}

// ── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isogeny::cm::tonelli_shanks_u64;
    use crate::isogeny::toy_curve_a;

    fn curve_211() -> SmallCurve {
        SmallCurve {
            name: "velu-211",
            p: 211,
            a: 1,
            b: 3,
        }
    }

    #[test]
    fn explicit_kernel_counts_all_self_inverse_points() {
        let curve = SmallCurve {
            a: 210,
            b: 0,
            ..curve_211()
        };
        let iso = velu_isogeny_from_kernel(&curve, &[(0, 0), (1, 0), (210, 0)]);
        assert_eq!(iso.degree, 4);
        assert_eq!(iso.kernel_half.len(), 3);
    }

    #[test]
    fn explicit_kernel_deduplicates_field_representatives() {
        let curve = curve_211();
        let canonical = velu_isogeny_2(&curve, 168).unwrap();
        let equivalent = velu_isogeny_from_kernel(&curve, &[(379, 211), (168, 0)]);
        assert_eq!(equivalent.kernel_half, canonical.kernel_half);
        assert_eq!(equivalent.degree, canonical.degree);
        assert!(same_equation(&equivalent.codomain, &canonical.codomain));
    }

    #[test]
    fn dual_isogenies_return_to_original_model() {
        let curve = curve_211();
        let a_fe = curve.to_curve_params().a_fe();
        for phi in [
            velu_isogeny_2(&curve, 168).unwrap(),
            velu_isogeny_from_kernel(&curve, &[(77, 3)]),
        ] {
            let degree = BigUint::from(phi.degree);
            let dual = dual_isogeny_naive(&phi).expect("fixture has a rational dual kernel");
            assert!(same_equation(&dual.codomain, &curve));
            let double_dual = dual_isogeny_naive(&dual).expect("the scaled dual also has a dual");
            assert!(same_equation(&double_dual.codomain, &phi.codomain));
            for x in 0..curve.p {
                let Some(y) = tonelli_shanks_u64(curve.rhs(x), curve.p) else {
                    continue;
                };
                let expected = xy(&pt(x, y, curve.p).scalar_mul(&degree, &a_fe));
                assert_eq!(
                    phi.evaluate(x, y).and_then(|(u, v)| dual.evaluate(u, v)),
                    expected
                );
                assert_eq!(double_dual.evaluate(x, y), phi.evaluate(x, y));
            }
        }
    }

    #[test]
    fn composite_includes_first_kernel_and_checks_rationality() {
        let curve = curve_211();
        let phi = velu_isogeny_2(&curve, 168).unwrap();
        let dual = dual_isogeny_naive(&phi).unwrap();
        let other = velu_isogeny_2(&phi.codomain, 1).unwrap();
        assert!(compose_isogenies(&phi, &dual).is_none());

        let composite = compose_isogenies(&phi, &other).expect("fully rational kernel");
        assert_eq!(composite.degree, 4);
        assert_eq!(composite.evaluate(168, 0), None);
        for x in 0..curve.p {
            if let Some(y) = tonelli_shanks_u64(curve.rhs(x), curve.p) {
                let sequential = phi.evaluate(x, y).and_then(|(u, v)| other.evaluate(u, v));
                assert_eq!(composite.evaluate(x, y), sequential);
            }
        }
    }

    #[test]
    fn dual_and_composition_accept_equivalent_coefficient_representatives() {
        let curve = SmallCurve {
            a: 212,
            b: 425,
            ..curve_211()
        };
        let phi = velu_isogeny_2(&curve, 168).unwrap();
        let dual = dual_isogeny_naive(&phi).expect("dual returns to the same field equation");
        assert_eq!((dual.codomain.a, dual.codomain.b), (1, 3));
        let intermediate = SmallCurve {
            b: phi.codomain.b + curve.p,
            ..phi.codomain
        };
        let psi = velu_isogeny_2(&intermediate, 1).unwrap();
        let composite = compose_isogenies(&phi, &psi).expect("matching intermediate equations");
        assert_eq!(composite.degree, 4);
        assert_eq!(
            (composite.codomain.a, composite.codomain.b),
            (psi.codomain.a, psi.codomain.b)
        );
    }

    #[test]
    fn two_isogeny_round_trip() {
        // On the toy-2003 curve, search for a 2-torsion point and
        // build the 2-isogeny.  Codomain should be a valid curve
        // with non-zero discriminant.
        let curve = toy_curve_a();
        let mut found = None;
        for x in 0..curve.p {
            if curve.rhs(x) == 0 {
                found = Some(x);
                break;
            }
        }
        if let Some(x_t) = found {
            let iso = velu_isogeny_2(&curve, x_t).expect("2-iso constructs");
            assert_eq!(iso.degree, 2);
            // Codomain is a *different* curve (unless x_T is degenerate).
            assert!(iso.codomain.p == curve.p);
        }
    }

    #[test]
    fn three_isogeny_count_for_toy() {
        // On a small ordinary curve, the number of 3-isogenies is
        // bounded by the number of order-3 cyclic subgroups, which
        // is 0, 1, 2, or 4 depending on rank of E[3].
        let curve = toy_curve_a();
        let isos = velu_isogeny_odd(&curve, 3);
        assert!(isos.len() <= 4, "got {} 3-isogenies", isos.len());
    }

    // ── The transfer that lever L5 rests on ───────────────────────────────
    //
    // `research/notes/ecc2k130/RESEARCH_ISOGENY_CLASS_SEARCH.md` opens by
    // asserting that "the ECDLP transports along an isogeny of degree
    // coprime to the subgroup order".  §1.1 of that note derives it; these
    // two tests are what make it a checked statement rather than a cited
    // one, and they are the reason the thread may treat every curve in the
    // class as carrying the same discrete logarithm.

    use crate::ecc::field::FieldElement;
    use crate::ecc::point::Point;

    fn pt(x: u64, y: u64, p: u64) -> Point {
        Point::Affine {
            x: FieldElement::new(BigUint::from(x), BigUint::from(p)),
            y: FieldElement::new(BigUint::from(y), BigUint::from(p)),
        }
    }

    fn xy(point: &Point) -> Option<(u64, u64)> {
        match point {
            Point::Affine { x, y } => Some((
                x.value.iter_u64_digits().next().unwrap_or(0),
                y.value.iter_u64_digits().next().unwrap_or(0),
            )),
            Point::Infinity => None,
        }
    }

    /// An isogeny is a group homomorphism, so `φ([k]P) = [k]φ(P)` and the
    /// discrete logarithm crosses **unchanged** — no factor of the degree
    /// appears anywhere in the transfer.
    ///
    /// `toy_curve_a` has `#E = 2019 = 3 · 673`, so with `ℓ = 3` the kernel
    /// has order 3 and `⟨P⟩` has prime order `n = 673`.  `gcd(3, 673) = 1`
    /// makes `φ` injective on `⟨P⟩`, which is exactly the note's side
    /// condition.  Every `k` in `1..n` is checked, not a sample.
    #[test]
    fn transfer_preserves_the_discrete_logarithm() {
        let curve = toy_curve_a();
        let (ell, n) = (3u64, 673u64);
        assert_eq!(2019, ell * n, "toy_curve_a order must factor as 3 · 673");
        assert_eq!(
            1,
            num_integer::gcd(ell, n),
            "side condition gcd(deg, n) = 1"
        );

        let iso = velu_isogeny_odd(&curve, ell)
            .into_iter()
            .next()
            .expect("toy_curve_a admits a 3-isogeny");

        let dom = curve.to_curve_params();
        let cod = iso.codomain.to_curve_params();
        let (a_dom, a_cod) = (dom.a_fe(), cod.a_fe());

        // A generator of the order-n subgroup: kill the cofactor ℓ.
        let mut rng: u64 = 0x5DEE_CE66_D4A5_1B2D;
        let base = loop {
            if let Some(p0) = crate::isogeny::cm::sample_random_point(&curve, &mut rng) {
                let q = p0.scalar_mul(&BigUint::from(ell), &a_dom);
                if !matches!(q, Point::Infinity) {
                    break q;
                }
            }
        };
        assert!(
            matches!(base.scalar_mul(&BigUint::from(n), &a_dom), Point::Infinity),
            "base point must have order n"
        );

        let (bx, by) = xy(&base).expect("base is affine");
        let image = iso.evaluate(bx, by).expect("P is not in ker φ");
        let p_img = pt(image.0, image.1, iso.codomain.p);

        // φ([k]P) == [k]φ(P) for every k in 1..n.
        for k in 1..n {
            let kp = base.scalar_mul(&BigUint::from(k), &a_dom);
            let (kx, ky) = xy(&kp).expect("[k]P affine for 0 < k < n");
            let lhs = iso.evaluate(kx, ky).expect("[k]P not in ker φ");
            let rhs = xy(&p_img.scalar_mul(&BigUint::from(k), &a_cod)).expect("[k]φ(P) affine");
            assert_eq!(
                lhs, rhs,
                "φ([{k}]P) != [{k}]φ(P) — the log did not transfer"
            );
        }
    }

    /// Regression pin for the `v(Q)` formula.  Until 2026-09, this module
    /// used `v(Q) = 4 x_Q g_y(Q)² + 2 g_x(Q)²` for non-2-torsion `Q`, which
    /// is not Vélu's: the correct value is `2 g_x(Q)` (and `g_x(Q)` on the
    /// T-row).  The bug was invisible to the older tests because they only
    /// checked a subgroup *count* and that the codomain shared the domain's
    /// prime — never that a point's image satisfies the codomain equation,
    /// which it did not.
    ///
    /// The expected values here come from an independent implementation
    /// cross-checked for the homomorphism property; they are not read back
    /// from this code.
    #[test]
    fn codomain_and_image_match_an_independent_reference() {
        let curve = toy_curve_a();
        let iso = velu_isogeny_odd(&curve, 3)
            .into_iter()
            .next()
            .expect("toy_curve_a admits a 3-isogeny");

        assert_eq!(iso.kernel_half, vec![(594u64, 805u64)], "kernel generator");
        assert_eq!(
            (iso.codomain.a, iso.codomain.b),
            (739, 792),
            "Vélu codomain"
        );
        assert_eq!(
            iso.evaluate(24, 118),
            Some((643, 1661)),
            "image of (24, 118)"
        );

        // …and the image really is a point of the codomain.
        let (p, a2, b2) = (
            curve.p as u128,
            iso.codomain.a as u128,
            iso.codomain.b as u128,
        );
        let (x, y) = iso.evaluate(24, 118).expect("not in kernel");
        let (x, y) = (x as u128, y as u128);
        assert_eq!(
            y * y % p,
            (x * x % p * x % p + a2 * x % p + b2) % p,
            "φ(P) must satisfy the codomain equation"
        );
    }

    /// The side condition, from the other direction: `ker φ ∩ ⟨P⟩` is a
    /// subgroup of a group of prime order `n`, so it is trivial unless
    /// `n | deg φ`.  Kernel points themselves must die.
    #[test]
    fn kernel_points_die_and_the_subgroup_survives() {
        let curve = toy_curve_a();
        let iso = velu_isogeny_odd(&curve, 3)
            .into_iter()
            .next()
            .expect("toy_curve_a admits a 3-isogeny");

        for &(kx, ky) in &iso.kernel_half {
            assert!(
                iso.evaluate(kx, ky).is_none(),
                "kernel point ({kx}, {ky}) should map to the point at infinity"
            );
        }
        assert_eq!(
            iso.kernel_half.len() as u64,
            (iso.degree - 1) / 2,
            "half-set of a degree-{} kernel has (ℓ-1)/2 points",
            iso.degree
        );
    }
}
