//! `ghs_fp6` workloads and `transfer.ghs_cover`: an elliptic curve over
//! `F_{q³}`, `q = p²`, whose GHS cover is a genus-3 hyperelliptic curve
//! over `F_q`, and the conorm–norm transfer of its discrete logarithm into
//! that Jacobian.
//!
//! # The curves
//!
//! `E : y² = x(x − α)(x − σα)`, `α ∈ F_{q³} ∖ F_q`, `σ` the `q`-Frobenius
//! (Joux–Vitse, Eurocrypt 2012), with `#E(F_{q³}) = 4ℓ`, `ℓ` prime; the
//! group is `Z/2 × Z/2ℓ` and the logarithm lives in the subgroup of order
//! `ℓ`.  The arithmetic is `jv_cover`'s: the tower `F_p ⊂ F_q ⊂ F_{q³}`,
//! the affine addition on `E` and the cover `H : y² = F(x)N(x)` with the
//! transfer `Φ : E(F_{q³}) → Jac_H(F_q)`.  A point's coordinate is six
//! base-`p` digits packed in one word, so `p ≤ 1,600` (`p⁶ < 2⁶⁴`);
//! `p ≢ 0 (mod 3)`.
//!
//! # What is charged
//!
//! On the workload itself every generic method is charged in its unit:
//! one affine addition on `E(F_{q³})`.  `transfer.ghs_cover` keeps that
//! unit: the cover, the two transfers and the inner solve on `Jac_H(F_q)`
//! are counted in `F_p` multiplications by the tower's own counter and
//! divided by `c_E`, the multiplications of one addition on `E` measured
//! in the run with the same counter (as `jv_cover::unit_costs` does).
//! The inner solve is a generic method on the Jacobian; the Joux–Vitse
//! decomposition index calculus on the cover stays in `jv_cover` and its
//! own binaries.

use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::cryptanalysis::ecbench::canonical::sha256_hex;
use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};
use crate::cryptanalysis::jv_cover::{generate_spec, Cover, Div, EllE, Fq, Fq3, Hyp, PtE6, E2, E6};
use crate::cryptanalysis::residual_walk::{is_prime_u64, mix64};

/// The largest `p` whose six-digit coordinates fit a word.
pub const GHS_MAX_P: u64 = 1_600;

/// One GHS workload: the curve's `α`, the subgroup order `ℓ` and a
/// generator of it.  The tower is rebuilt on demand (it carries a counter
/// that is not shared between threads).
#[derive(Clone, Debug)]
pub struct GhsInstance {
    pub p: u64,
    pub alpha: E6,
    pub l: u64,
    pub generator: PtE6,
}

fn check_p(p: u64) -> Result<(), String> {
    if !(11..=GHS_MAX_P).contains(&p) || !is_prime_u64(p) || p % 3 == 0 {
        return Err(format!(
            "a GHS workload takes a prime 11 ≤ p ≤ {GHS_MAX_P}, p ≠ 3, not {p}"
        ));
    }
    Ok(())
}

impl GhsInstance {
    /// `jv_cover::generate_spec(p, seed)`'s curve and generator.
    pub fn search(p: u64, seed: u64) -> Result<GhsInstance, String> {
        check_p(p)?;
        let spec = generate_spec(p, seed);
        Ok(GhsInstance {
            p,
            alpha: spec.alpha,
            l: spec.l,
            generator: spec.g,
        })
    }

    /// The same curve given outright: `α` as six `F_p` digits (`F_q`
    /// coefficients of `1, θ, θ²`, each `c₀ + c₁t`), `ℓ` trusted, `G`
    /// checked on the curve with order `ℓ`.
    pub fn explicit(
        p: u64,
        alpha: [u64; 6],
        l: u64,
        g: &[String; 2],
    ) -> Result<GhsInstance, String> {
        check_p(p)?;
        if !is_prime_u64(l) {
            return Err(format!("ℓ = {l} is not prime"));
        }
        let alpha = unpack(
            p,
            alpha
                .iter()
                .rev()
                .fold(0u128, |a, &d| a * u128::from(p) + u128::from(d % p)),
        );
        if alpha.in_fq() {
            return Err("α lies in F_q: the curve has no GHS cover".into());
        }
        let f = Fq3::new(p);
        Cover::new(&f, &alpha).ok_or("the cover refuses α")?;
        let inst = GhsInstance {
            p,
            alpha,
            l,
            generator: PtE6::INF,
        };
        let gen = inst.decode_with(&f, g)?;
        let ec = inst.curve(&f);
        if gen.inf || !ec.mul(&gen, l).inf {
            return Err("G does not have order ℓ".into());
        }
        Ok(GhsInstance {
            generator: gen,
            ..inst
        })
    }

    pub fn field(&self) -> Fq3 {
        Fq3::new(self.p)
    }

    pub fn curve<'a>(&self, f: &'a Fq3) -> EllE<'a> {
        EllE::new(f, &self.alpha)
    }

    pub fn group_order(&self) -> u64 {
        4 * self.l
    }

    /// `α`'s six digits, `F_q` coefficient `c₀` first.
    pub fn alpha_digits(&self) -> [u64; 6] {
        digits(&self.alpha)
    }

    pub fn encode(&self, pt: &PtE6) -> [String; 2] {
        [
            format!("0x{:x}", pack(self.p, &pt.x)),
            format!("0x{:x}", pack(self.p, &pt.y)),
        ]
    }

    pub fn decode(&self, t: &[String; 2]) -> Result<PtE6, String> {
        self.decode_with(&self.field(), t)
    }

    fn decode_with(&self, f: &Fq3, t: &[String; 2]) -> Result<PtE6, String> {
        let word = |s: &str| {
            u128::from_str_radix(s.trim_start_matches("0x"), 16)
                .map_err(|_| format!("bad hex `{s}`"))
        };
        let top = u128::from(self.p).pow(6);
        let (x, y) = (word(&t[0])?, word(&t[1])?);
        if x >= top || y >= top {
            return Err("a coordinate is not six base-p digits".into());
        }
        let pt = PtE6 {
            x: unpack(self.p, x),
            y: unpack(self.p, y),
            inf: false,
        };
        if !self.curve(f).on_curve(&pt) {
            return Err("the point is not on the curve".into());
        }
        Ok(pt)
    }

    pub fn mul_generator_hex(&self, k: u64) -> Option<[String; 2]> {
        let f = self.field();
        let pt = self.curve(&f).mul(&self.generator, k % self.l);
        (!pt.inf).then(|| self.encode(&pt))
    }

    /// The public-target law: the draw's low 63 bits modulo `p⁶` are the
    /// abscissa's digits, the square root is sampled by a generator seeded
    /// with the draw, the top bit picks `±y`, and `[4]` clears the
    /// cofactor.
    pub fn hash_to_subgroup_hex(&self, h: u64) -> Option<[String; 2]> {
        let f = self.field();
        let ec = self.curve(&f);
        let x = unpack(
            self.p,
            u128::from(h & (u64::MAX >> 1)) % u128::from(self.p).pow(6),
        );
        let mut rng = StdRng::seed_from_u64(h);
        let y = f.sqrt(&ec.rhs(&x), &mut rng)?;
        let pt = PtE6 { x, y, inf: false };
        let pt = if h >> 63 == 1 { ec.neg(&pt) } else { pt };
        let q = ec.mul(&pt, 4);
        (!q.inf).then(|| self.encode(&q))
    }

    /// `GHSV1:fp<p>:<α digits>:<#E>` and the slug `ghs-fp<p>x6-<sha8>`:
    /// the tower is not one `F_p`-modulus, so the curve has no ICV1 name
    /// and is never looked up in the registry.
    pub fn names(&self) -> (String, String) {
        let d: Vec<String> = self.alpha_digits().iter().map(u64::to_string).collect();
        let id = format!("GHSV1:fp{}:{}:{}", self.p, d.join(","), self.group_order());
        let h = &sha256_hex(id.as_bytes())[..8];
        (id.clone(), format!("ghs-fp{}x6-{h}", self.p))
    }
}

fn digits(a: &E6) -> [u64; 6] {
    [
        a.0[0].0[0],
        a.0[0].0[1],
        a.0[1].0[0],
        a.0[1].0[1],
        a.0[2].0[0],
        a.0[2].0[1],
    ]
}

pub fn pack(p: u64, a: &E6) -> u128 {
    digits(a)
        .iter()
        .rev()
        .fold(0u128, |acc, &d| acc * u128::from(p) + u128::from(d))
}

pub fn unpack(p: u64, mut v: u128) -> E6 {
    let mut d = [0u64; 6];
    for x in d.iter_mut() {
        *x = (v % u128::from(p)) as u64;
        v /= u128::from(p);
    }
    E6([E2([d[0], d[1]]), E2([d[2], d[3]]), E2([d[4], d[5]])])
}

/// `E(F_{q³})` as a counted group: one affine addition (or doubling) is
/// one operation.
pub struct GhsCurveGroup<'a>(pub EllE<'a>);

impl CountedGroup for GhsCurveGroup<'_> {
    type Elt = PtE6;
    fn identity(&self) -> PtE6 {
        PtE6::INF
    }
    fn is_identity(&self, p: &PtE6) -> bool {
        p.inf
    }
    fn add(&self, ops: &mut GroupOps, p: PtE6, q: PtE6) -> PtE6 {
        if p.inf || q.inf {
            return if p.inf { q } else { p };
        }
        if p == q {
            ops.doubles += 1;
        } else {
            ops.adds += 1;
        }
        self.0.add(&p, &q)
    }
    fn double(&self, ops: &mut GroupOps, p: PtE6) -> PtE6 {
        if p.inf {
            return p;
        }
        ops.doubles += 1;
        self.0.add(&p, &p)
    }
    fn neg(&self, p: PtE6) -> PtE6 {
        self.0.neg(&p)
    }
    fn key(&self, p: &PtE6) -> u64 {
        if p.inf {
            return u64::MAX;
        }
        let p_ = self.0.f.f.p;
        mix64((pack(p_, &p.x) as u64) ^ mix64(pack(p_, &p.y) as u64))
    }
}

/// A reduced divisor class on the genus-3 cover, in a fixed-size form:
/// `u` monic of degree `deg ≤ 3` (`u[deg] = 1`), `v` of degree `< deg`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GDiv {
    pub deg: u8,
    pub u: [E2; 4],
    pub v: [E2; 3],
}

impl GDiv {
    fn from_div(d: &Div<E2>) -> GDiv {
        let mut g = GDiv {
            deg: (d.u.len() - 1) as u8,
            u: [E2::ZERO; 4],
            v: [E2::ZERO; 3],
        };
        g.u[..d.u.len()].copy_from_slice(&d.u);
        g.v[..d.v.len()].copy_from_slice(&d.v);
        g
    }
    fn to_div(self) -> Div<E2> {
        let deg = usize::from(self.deg);
        let mut v = self.v[..deg].to_vec();
        while v.last() == Some(&E2::ZERO) {
            v.pop();
        }
        Div {
            u: self.u[..=deg].to_vec(),
            v,
        }
    }
}

/// `Jac_H(F_q)` as a counted group: one Cantor addition (or doubling)
/// with reduction is one operation; negation (`v ↦ −v`) is free.
pub struct GhsJacobianGroup<'a>(pub Hyp<'a, Fq>);

impl CountedGroup for GhsJacobianGroup<'_> {
    type Elt = GDiv;
    fn identity(&self) -> GDiv {
        GDiv::from_div(&self.0.identity())
    }
    fn is_identity(&self, p: &GDiv) -> bool {
        p.deg == 0
    }
    fn add(&self, ops: &mut GroupOps, p: GDiv, q: GDiv) -> GDiv {
        if p.deg == 0 || q.deg == 0 {
            return if p.deg == 0 { q } else { p };
        }
        if p == q {
            ops.doubles += 1;
        } else {
            ops.adds += 1;
        }
        GDiv::from_div(&self.0.add(&p.to_div(), &q.to_div()))
    }
    fn double(&self, ops: &mut GroupOps, p: GDiv) -> GDiv {
        if p.deg == 0 {
            return p;
        }
        ops.doubles += 1;
        let d = p.to_div();
        GDiv::from_div(&self.0.add(&d, &d))
    }
    fn neg(&self, p: GDiv) -> GDiv {
        GDiv::from_div(&self.0.neg(&p.to_div()))
    }
    fn key(&self, p: &GDiv) -> u64 {
        let mut h = u64::from(p.deg);
        for c in p.u.iter().chain(p.v.iter()) {
            h = mix64(h ^ c.0[0] ^ (c.0[1] << 32));
        }
        h
    }
}

/// `c_E`: `F_p` multiplications per affine addition on `E`, over 64
/// additions with the tower's counter.
pub fn affine_add_cost(f: &Fq3, ec: &EllE, g: &PtE6, h: &PtE6) -> f64 {
    let before = f.muls();
    let mut acc = *g;
    for _ in 0..64 {
        acc = ec.add(&acc, h);
    }
    (f.muls() - before) as f64 / 64.0
}

/// The cover, `Φ(G)` and `Φ(Q)`, or why not.
pub fn transfer(f: &Fq3, inst: &GhsInstance, q: &PtE6) -> Result<(Cover, GDiv, GDiv), String> {
    let cov = Cover::new(f, &inst.alpha).ok_or("the cover refuses α")?;
    let gj = cov.transfer(f, &inst.generator).ok_or("Φ(G) degenerates")?;
    let qj = cov.transfer(f, q).ok_or("Φ(Q) degenerates")?;
    let jac = Hyp {
        f: &f.f,
        h: cov.hx.clone(),
        g: 3,
    };
    if jac.is_identity(&gj) {
        return Err("Φ(G) is the identity: Φ is not injective on ⟨G⟩".into());
    }
    Ok((cov, GDiv::from_div(&gj), GDiv::from_div(&qj)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_searched_curve_round_trips_and_the_transfer_is_a_homomorphism() {
        let i = GhsInstance::search(19, 1).unwrap();
        let f = i.field();
        let ec = i.curve(&f);
        assert!(ec.mul(&i.generator, i.l).inf && !i.generator.inf);
        let g = i.encode(&i.generator);
        let again = GhsInstance::explicit(i.p, i.alpha_digits(), i.l, &g).unwrap();
        assert_eq!(again.generator, i.generator);
        let t = (0..64u64)
            .find_map(|c| i.hash_to_subgroup_hex(mix64(c)))
            .expect("a target");
        let q = i.decode(&t).unwrap();
        assert!(ec.mul(&q, i.l).inf);
        for k in [3u64, 1000, i.l - 2] {
            let q = ec.mul(&i.generator, k);
            let (cov, gj, qj) = transfer(&f, &i, &q).unwrap();
            let grp = GhsJacobianGroup(Hyp {
                f: &f.f,
                h: cov.hx.clone(),
                g: 3,
            });
            let mut ops = GroupOps::default();
            assert_eq!(grp.mul(&mut ops, gj, k), qj, "k = {k}");
            assert!(grp.is_identity(&grp.mul(&mut ops, gj, i.l)));
            let n = grp.neg(gj);
            assert!(grp.is_identity(&grp.add(&mut ops, gj, n)));
        }
        let e = GhsCurveGroup(i.curve(&f));
        let mut ops = GroupOps::default();
        let a = e.mul(&mut ops, i.generator, 5);
        let (g2, g3) = (
            e.mul(&mut ops, i.generator, 2),
            e.mul(&mut ops, i.generator, 3),
        );
        let b = e.add(&mut ops, g2, g3);
        assert_eq!(a, b);
        assert!(affine_add_cost(&f, &ec, &i.generator, &a) > 10.0);
    }
}
