//! Jacobians of curves of genus above one as ecbench workloads.
//!
//! An elliptic curve is its own Jacobian; the curves here are not.  A
//! workload on one of them is the discrete logarithm in a prime-order
//! subgroup of `J(F_p)`, and the counted unit is the **Jacobian group
//! operation**: one addition or one doubling of divisor classes is one
//! group-addition equivalent (GAE), exactly as one point addition is on an
//! elliptic curve.  `S = GAE / √r` and the generic floor `√(π / 2A)` then
//! mean what they mean on a curve, and every generic method (rho, BSGS,
//! kangaroo) runs unchanged through [`CountedGroup`].  A Jacobian GAE is
//! not an elliptic-curve GAE: comparing a Jacobian workload with an
//! elliptic one needs the conversion a transfer method states
//! (`docs/ecbench/README.md` §7, "Curves of higher genus").
//!
//! Two families:
//!
//! - [`Genus2Instance`]: an imaginary genus-2 curve `y² = f(x)`,
//!   `deg f = 5`, over `F_p` (`p < 2^16`), Cantor arithmetic from
//!   [`crate::cryptanalysis::genus2_fp`], `#J(F_p)` from the curve's
//!   `L`-polynomial (point counts over `F_p` and `F_{p²}`).
//! - [`super::plane_quartic::QuarticInstance`]: a smooth plane quartic, the
//!   non-hyperelliptic curves of genus 3, with the linear-algebra group law
//!   of that module.
//!
//! A divisor class is written in a record as two hex strings, like a
//! point: for genus 2, `[deg u · p² + u₁ p + u₀, v₁ p + v₀]`.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::cryptanalysis::ecbench::canonical::{sha256_hex, splitmix64};
use crate::cryptanalysis::genus2_fp::{Divisor, Genus2Curve};
use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};
use crate::cryptanalysis::residual_walk::is_prime_u64;

/// The largest prime the genus-2 family accepts: `#J(F_p)` comes from
/// point counts over `F_{p²}`, `p²` abscissae.
pub const GENUS2_MAX_P: u64 = 1 << 16;

/// A genus-2 curve's Jacobian as a counted group: one Cantor addition or
/// doubling is one group operation.  Negation `(u, v) ↦ (u, −v)` is free,
/// as it is on an elliptic curve, so the negation map is available
/// (`A = 2`).  Field operations are not tallied (unknown, never zero).
pub struct Genus2Group<'a>(pub &'a Genus2Curve);

impl CountedGroup for Genus2Group<'_> {
    type Elt = Divisor;
    fn identity(&self) -> Divisor {
        self.0.identity()
    }
    fn is_identity(&self, p: &Divisor) -> bool {
        p.is_identity()
    }
    fn add(&self, ops: &mut GroupOps, p: Divisor, q: Divisor) -> Divisor {
        ops.adds += 1;
        self.0.add(&p, &q)
    }
    fn double(&self, ops: &mut GroupOps, p: Divisor) -> Divisor {
        ops.doubles += 1;
        self.0.double(&p)
    }
    fn neg(&self, p: Divisor) -> Divisor {
        self.0.neg(&p)
    }
    fn key(&self, d: &Divisor) -> u64 {
        if d.is_identity() {
            return 0;
        }
        let (x, y) = encode_g2(self.0.p(), d);
        // Never 0: the identity's key.
        splitmix64(x ^ splitmix64(y)) | 1
    }
}

/// `[deg u · p² + u₁ p + u₀, v₁ p + v₀]`; `(0, 0)` is the identity.
pub fn encode_g2(p: u64, d: &Divisor) -> (u64, u64) {
    let u = d.u_coeffs();
    let v = d.v_coeffs();
    let deg = d.degree() as u64;
    let u0 = if deg >= 1 { u[0] } else { 0 };
    let u1 = if deg >= 2 { u[1] } else { 0 };
    let v0 = v.first().copied().unwrap_or(0);
    let v1 = v.get(1).copied().unwrap_or(0);
    (deg * p * p + u1 * p + u0, v1 * p + v0)
}

/// The inverse of [`encode_g2`], validated on the curve.
pub fn decode_g2(c: &Genus2Curve, x: u64, y: u64) -> Result<Divisor, String> {
    let p = c.p();
    let deg = x / (p * p);
    let (u1, u0) = ((x / p) % p, x % p);
    let (v1, v0) = (y / p, y % p);
    if deg > 2 || v1 >= p {
        return Err(format!(
            "0x{x:x}, 0x{y:x} is not a genus-2 divisor encoding"
        ));
    }
    let (u, v): (Vec<u64>, Vec<u64>) = match deg {
        0 => (vec![1], vec![]),
        1 => (vec![u0, 1], vec![v0]),
        _ => (vec![u0, u1, 1], vec![v0, v1]),
    };
    if deg < 2 && (u1 != 0 || v1 != 0) || deg == 0 && (u0 != 0 || v0 != 0) {
        return Err(format!("0x{x:x}, 0x{y:x} is not a reduced encoding"));
    }
    let v: Vec<u64> = {
        let mut v = v;
        while v.last() == Some(&0) {
            v.pop();
        }
        v
    };
    c.divisor(&u, &v).map_err(|e| e.to_string())
}

/// A built genus-2 workload curve: `C : y² = f(x)` over `F_p`, `#J(F_p) =
/// h·r` with `r` prime, and a generator of the order-`r` subgroup.
#[derive(Clone, Debug)]
pub struct Genus2Instance {
    pub curve: Genus2Curve,
    pub group_order: u64,
    pub r: u64,
    pub cofactor: u64,
    pub generator: Divisor,
}

impl Genus2Instance {
    pub fn p(&self) -> u64 {
        self.curve.p()
    }

    /// Search: a prime `p` of `p_bits` bits and quintic `f`, both from
    /// `seed`, until `#J(F_p) = h·r` with `r` prime, `h ≤ max_cofactor`
    /// and `r ∤ h`.  The generator is `[h]D` for a seeded random class
    /// `D`, the first that is not the identity.
    pub fn search(p_bits: u32, seed: u64, max_cofactor: u64) -> Result<Self, String> {
        if !(5..=16).contains(&p_bits) {
            return Err(format!("genus2_search takes 5 ≤ p_bits ≤ 16, not {p_bits}"));
        }
        if max_cofactor == 0 {
            return Err("max_cofactor must be at least 1".into());
        }
        let mut rng = StdRng::seed_from_u64(seed ^ (u64::from(p_bits) << 48) ^ 0x6732);
        for _ in 0..100_000 {
            let p = loop {
                let c = (rng.gen::<u64>() & ((1 << p_bits) - 1)) | (1 << (p_bits - 1)) | 1;
                if is_prime_u64(c) && c < GENUS2_MAX_P {
                    break c;
                }
            };
            let mut f = [0u64; 6];
            for c in f.iter_mut().take(5) {
                *c = rng.gen_range(0..p);
            }
            f[5] = 1;
            let Ok(curve) = Genus2Curve::new(p, f) else {
                continue;
            };
            let Ok(n) = curve.jacobian_order() else {
                continue;
            };
            let Some((r, h)) = prime_part(n, max_cofactor) else {
                continue;
            };
            let Some(g) = generator(&curve, h, r, &mut rng) else {
                continue;
            };
            return Ok(Genus2Instance {
                curve,
                group_order: n,
                r,
                cofactor: h,
                generator: g,
            });
        }
        Err(format!(
            "no genus-2 curve with #J = h·r, h ≤ {max_cofactor}, found for p_bits = {p_bits}, seed = {seed}"
        ))
    }

    /// The same curve given outright.  The group order is trusted, as the
    /// explicit elliptic forms trust theirs; `r` prime, `r | #J`, `G` on
    /// the Jacobian, `G ≠ 0` and `[r]G = 0` are checked.
    pub fn explicit(
        p: u64,
        f: [u64; 6],
        group_order: u64,
        r: u64,
        g: [u64; 2],
    ) -> Result<Self, String> {
        if p >= GENUS2_MAX_P {
            return Err(format!("genus2 takes p < 2^16, not {p}"));
        }
        let curve = Genus2Curve::new(p, f).map_err(|e| e.to_string())?;
        if r < 5 || !is_prime_u64(r) || !group_order.is_multiple_of(r) {
            return Err(format!(
                "r = {r} is not a prime divisor of #J = {group_order}"
            ));
        }
        let generator = decode_g2(&curve, g[0], g[1])?;
        if generator.is_identity() || !curve.mul(&generator, r).is_identity() {
            return Err("the generator does not have order r".into());
        }
        Ok(Genus2Instance {
            curve,
            group_order,
            r,
            cofactor: group_order / r,
            generator,
        })
    }

    pub fn group(&self) -> Genus2Group<'_> {
        Genus2Group(&self.curve)
    }

    pub fn encode(&self, d: &Divisor) -> [String; 2] {
        let (x, y) = encode_g2(self.p(), d);
        [format!("0x{x:x}"), format!("0x{y:x}")]
    }

    pub fn decode(&self, t: &[String; 2]) -> Result<Divisor, String> {
        let x = parse(&t[0])?;
        let y = parse(&t[1])?;
        decode_g2(&self.curve, x, y)
    }

    /// `[k]G`, or `None` for the identity.
    pub fn mul_generator_hex(&self, k: u64) -> Option<[String; 2]> {
        let d = self.curve.mul(&self.generator, k);
        (!d.is_identity()).then(|| self.encode(&d))
    }

    /// A public target from one hashed draw: the abscissa `h mod p`, one of
    /// the two classes `(x − x₀, ±y₀)` chosen by the top bit, times the
    /// cofactor.  `None` when that abscissa is not on the curve or the
    /// cleared class is the identity.
    pub fn hash_to_subgroup_hex(&self, h: u64) -> Option<[String; 2]> {
        let p = self.p();
        let x0 = h % p;
        let ds = self.curve.divisors_with_u(&[(p - x0) % p, 1]).ok()?;
        let d = *ds.get(usize::from((h >> 63) == 1 && ds.len() > 1))?;
        let q = self.curve.mul(&d, self.cofactor);
        (!q.is_identity()).then(|| self.encode(&q))
    }

    /// The model, in the form `JCV1:<family>:<field>:<coefficients>:<#J>`,
    /// and a slug `jcv1-g2-fp<bits>-<sha8>`: ICV1 names elliptic curves
    /// only, so a Jacobian gets its own namespace and is never looked up
    /// in the curve registry.
    pub fn names(&self) -> (String, String) {
        let f = self.curve.f();
        let coeffs: Vec<String> = f.iter().map(u64::to_string).collect();
        let id = format!(
            "JCV1:g2:fp{}:{}:{}",
            self.p(),
            coeffs.join(","),
            self.group_order
        );
        let h = &sha256_hex(id.as_bytes())[..8];
        let bits = 64 - self.p().leading_zeros();
        (id.clone(), format!("jcv1-g2-fp{bits}-{h}"))
    }
}

/// `(r, h)` with `n = h·r`, `r` the largest prime factor, when `h ≤
/// max_cofactor` and `r ∤ h` (so the `r`-part is cyclic of order `r`).
pub fn prime_part(n: u64, max_cofactor: u64) -> Option<(u64, u64)> {
    if n < 2 {
        return None;
    }
    let mut m = n;
    let mut largest = 1;
    let mut d = 2u64;
    while d.saturating_mul(d) <= m {
        while m.is_multiple_of(d) {
            largest = largest.max(d);
            m /= d;
        }
        d += 1;
    }
    if m > 1 {
        largest = largest.max(m);
    }
    let r = largest;
    let h = n / r;
    (r >= 5 && h <= max_cofactor && !h.is_multiple_of(r)).then_some((r, h))
}

fn generator(c: &Genus2Curve, h: u64, r: u64, rng: &mut StdRng) -> Option<Divisor> {
    for _ in 0..64 {
        let g = c.mul(&c.random(rng), h);
        if !g.is_identity() {
            return c.mul(&g, r).is_identity().then_some(g);
        }
    }
    None
}

fn parse(s: &str) -> Result<u64, String> {
    u64::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|_| format!("bad hex `{s}`"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_searched_curve_has_a_generator_of_prime_order_and_round_trips() {
        let i = Genus2Instance::search(9, 3, 16).unwrap();
        assert!(is_prime_u64(i.r));
        assert_eq!(i.group_order, i.r * i.cofactor);
        assert!(i.curve.mul(&i.generator, i.r).is_identity());
        let g = i.encode(&i.generator);
        let x = parse(&g[0]).unwrap();
        let y = parse(&g[1]).unwrap();
        let again =
            Genus2Instance::explicit(i.p(), i.curve.f(), i.group_order, i.r, [x, y]).unwrap();
        assert_eq!(again.generator, i.generator);
        assert_eq!(i.decode(&g).unwrap(), i.generator);
        // The order is the Jacobian's: [#J]D = 0 for random classes.
        let mut rng = StdRng::seed_from_u64(1);
        for _ in 0..8 {
            let d = i.curve.random(&mut rng);
            assert!(i.curve.mul(&d, i.group_order).is_identity());
        }
    }

    #[test]
    fn the_counted_group_matches_cantor_and_counts_every_operation() {
        let i = Genus2Instance::search(8, 1, 64).unwrap();
        let g = i.group();
        let mut ops = GroupOps::default();
        let k = 1 + (i.r / 3);
        let a = g.mul(&mut ops, i.generator, k);
        assert_eq!(a, i.curve.mul(&i.generator, k));
        assert_eq!(ops.scalar_mults, 1);
        let bits = 64 - k.leading_zeros() as u64;
        assert_eq!(ops.doubles, bits - 1);
        assert_eq!(ops.adds, u64::from(k.count_ones()) - 1);
        assert_eq!(g.key(&g.identity()), 0);
        assert_ne!(g.key(&a), g.key(&g.neg(a)));
    }

    #[test]
    fn bad_encodings_are_refused() {
        let i = Genus2Instance::search(8, 1, 64).unwrap();
        let p = i.p();
        assert!(decode_g2(&i.curve, 3 * p * p, 0).is_err());
        assert!(decode_g2(&i.curve, 0, 1).is_err());
        assert!(Genus2Instance::explicit(p, i.curve.f(), i.group_order, i.r + 2, [0, 0]).is_err());
    }
}
