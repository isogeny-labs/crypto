//! **Lead R6.6: genus 2 under an algebraic (Nagao-style) decomposition
//! oracle, with a genuine Richelot (2,2)-isogeny image arm.**
//!
//! Round-4 lead 16.6 (`examples/r4_16_6_genus2_census.rs`) measured the
//! affine saturation law on `J(C)`, `C: y² = x⁵ + b` over `F_p`,
//! `p ≡ 1 (mod 10)`, with an abstract mitm census. This example asks whether
//! the law survives an *algebraic* oracle, which decomposes a target class
//! `D` as a sum of `m` degree-1 divisors `[P_i − ∞]` whose `x(P_i)` lie in
//! the base's `x`-set, and it replaces the round-4 "image arm" (the subgroup
//! `[2]J`) by a genuine Richelot `(2,2)`-isogeny `φ: J → J′`.
//!
//! * Oracle (a): the exact enumeration of `m = 2, 3` sums of degree-1 base
//!   divisors (the oracle's reach, exact census numbers).
//! * Oracle (b): the Nagao-style algebraic solve for `m = 2`:
//!   `D = [P₁ − ∞] + [P₂ − ∞]` iff the Mumford `u` of `D` splits as
//!   `(x − x₁)(x − x₂)` with both roots in the `x`-set. Counted in `F_p`
//!   multiplications, inversions and square roots, validated against (a) on
//!   random targets of `G`. For `m = 3` the `λ`-sweep solve (`y = λu − v`)
//!   is run on a small sample as a consistency check.
//! * Bases: plain (random `x`), `A`-closed (`ζ₅`-orbits of the `x`-set, both
//!   signs), the Richelot image arm (degree-1 divisors in the index-4
//!   subgroup `H₄ = φ̂(J′(F_p))`, cut out by the `(2,2)`-descent map
//!   `D ↦ (χ(G₁(D)), χ(G₂(D)))`), the `G` control (`i = h`), and on `J′` the
//!   literal image base `φ(F)` against a plain degree-1 base on `C′`.
//! * Richelot: `f = G₁G₂G₃` with `G₃` linear, `H_i = [G_j, G_k]`,
//!   `C′: Y² = −κ⁻¹ H₁H₂H₃` moved to a quintic model by a Möbius map, the
//!   correspondence `G₁(x)H₁(X) + G₂(x)H₂(X) = 0`, `y·y′ = G₁(x)H₁(X)(x − X)`
//!   evaluated in the algebra `F_p[x]/(u)`, and `φ(D) = Φ(D + A) − Φ(A)`.
//!   Validated: homomorphism, kernel exactly the `(2,2)`-group, isotropy,
//!   `#J′(F_p) = #J(F_p)`, `φ(J(F_p)) = ker ψ′` of index 4.
//! * Translation closure: `|(C − ∞) ∩ (C − ∞ − t)| ≤ 2` measured for every
//!   rational 2-torsion `t` and random `t`.
//!
//! ```text
//! cargo run --release --example r6_6_genus2_oracle -- --p 1051 --b 1042 \
//!     --size 80 --reps 40 --seed 1 --json out.json
//! ```

use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

use crypto_lib::cryptanalysis::genus2_fp::{Divisor, Genus2Curve};
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

// ── F_p with operation counters ─────────────────────────────────────────────

struct Fp {
    p: u64,
    w: u64,
    muls: Cell<u64>,
    invs: Cell<u64>,
    sqrts: Cell<u64>,
}

impl Fp {
    fn new(p: u64) -> Fp {
        let k = Fp {
            p,
            w: 0,
            muls: Cell::new(0),
            invs: Cell::new(0),
            sqrts: Cell::new(0),
        };
        let w = (2..p).find(|&w| k.legendre(w) == -1).unwrap_or(p - 1);
        Fp { w, ..k }
    }
    fn reset(&self) {
        self.muls.set(0);
        self.invs.set(0);
        self.sqrts.set(0);
    }
    fn counts(&self) -> (u64, u64, u64) {
        (self.muls.get(), self.invs.get(), self.sqrts.get())
    }
    #[inline]
    fn add(&self, a: u64, b: u64) -> u64 {
        (a + b) % self.p
    }
    #[inline]
    fn sub(&self, a: u64, b: u64) -> u64 {
        (a + self.p - b) % self.p
    }
    #[inline]
    fn neg(&self, a: u64) -> u64 {
        (self.p - a) % self.p
    }
    #[inline]
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.muls.set(self.muls.get() + 1);
        a * b % self.p
    }
    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1 % self.p;
        a %= self.p;
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
        self.invs.set(self.invs.get() + 1);
        self.pow(a, self.p - 2)
    }
    fn legendre(&self, a: u64) -> i8 {
        let a = a % self.p;
        if a == 0 {
            0
        } else if self.pow(a, (self.p - 1) / 2) == 1 {
            1
        } else {
            -1
        }
    }
    /// Tonelli–Shanks; `None` for a non-residue.
    fn sqrt(&self, a: u64) -> Option<u64> {
        self.sqrts.set(self.sqrts.get() + 1);
        let p = self.p;
        let a = a % p;
        if a == 0 {
            return Some(0);
        }
        if self.legendre(a) != 1 {
            return None;
        }
        let mut q = p - 1;
        let mut s = 0u32;
        while q.is_multiple_of(2) {
            q /= 2;
            s += 1;
        }
        let mut m = s;
        let mut c = self.pow(self.w, q);
        let mut t = self.pow(a, q);
        let mut r = self.pow(a, q.div_ceil(2));
        while t != 1 {
            let mut i = 0u32;
            let mut tt = t;
            while tt != 1 && i < m {
                tt = self.mul(tt, tt);
                i += 1;
            }
            if i >= m {
                return None;
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
}

// ── Polynomials over F_p (trimmed Vec, lowest degree first) ────────────────

type Poly = Vec<u64>;

fn trim(mut a: Poly) -> Poly {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}

fn padd(k: &Fp, a: &[u64], b: &[u64]) -> Poly {
    let n = a.len().max(b.len());
    trim(
        (0..n)
            .map(|i| k.add(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)))
            .collect(),
    )
}

fn psub(k: &Fp, a: &[u64], b: &[u64]) -> Poly {
    let n = a.len().max(b.len());
    trim(
        (0..n)
            .map(|i| k.sub(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)))
            .collect(),
    )
}

fn pmul(k: &Fp, a: &[u64], b: &[u64]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = k.add(r[i + j], k.mul(x, y));
        }
    }
    trim(r)
}

fn pscale(k: &Fp, a: &[u64], c: u64) -> Poly {
    trim(a.iter().map(|&x| k.mul(x, c)).collect())
}

fn pdivrem(k: &Fp, a: &[u64], b: &[u64]) -> (Poly, Poly) {
    let mut r: Poly = trim(a.to_vec());
    let b = trim(b.to_vec());
    let Some(&lb) = b.last() else {
        return (Vec::new(), r);
    };
    if r.len() < b.len() {
        return (Vec::new(), r);
    }
    let db = b.len() - 1;
    let li = k.inv(lb);
    let mut q = vec![0u64; r.len() - db];
    while r.len() >= b.len() {
        let d = r.len() - 1;
        let c = k.mul(r[d], li);
        q[d - db] = c;
        for (j, &bj) in b.iter().enumerate() {
            r[d - db + j] = k.sub(r[d - db + j], k.mul(c, bj));
        }
        r = trim(r);
    }
    (trim(q), r)
}

fn pmonic(k: &Fp, a: &[u64]) -> Poly {
    match a.last() {
        None => Vec::new(),
        Some(&lc) => pscale(k, a, k.inv(lc)),
    }
}

fn peval(k: &Fp, a: &[u64], x: u64) -> u64 {
    a.iter().rev().fold(0, |acc, &c| k.add(k.mul(acc, x), c))
}

fn pderiv(k: &Fp, a: &[u64]) -> Poly {
    trim(
        a.iter()
            .enumerate()
            .skip(1)
            .map(|(i, &c)| k.mul(i as u64 % k.p, c))
            .collect(),
    )
}

fn ppow(k: &Fp, a: &[u64], e: usize) -> Poly {
    let mut r = vec![1];
    for _ in 0..e {
        r = pmul(k, &r, a);
    }
    r
}

/// `Res(u, g) = Π_{u(x_j) = 0} g(x_j)` for monic `u` of degree ≤ 2.
fn resultant_with_monic(k: &Fp, u: &[u64], g: &[u64]) -> u64 {
    match u.len() {
        0 | 1 => 1,
        2 => peval(k, g, k.neg(u[0])),
        _ => {
            let (_, r) = pdivrem(k, g, u);
            let r1 = *r.get(1).unwrap_or(&0);
            let r0 = *r.first().unwrap_or(&0);
            // Π (r1·x_j + r0) = r1²·x₁x₂ + r1·r0·(x₁ + x₂) + r0²
            let t = k.add(k.mul(k.mul(r1, r1), u[0]), k.mul(r0, r0));
            k.sub(t, k.mul(k.mul(r1, r0), u[1]))
        }
    }
}

// ── The quadratic algebra A = F_p[x]/(x² + u1·x + u0) ───────────────────────

type Q = (u64, u64); // a0 + a1·x̄

struct Alg<'a> {
    k: &'a Fp,
    u1: u64,
    u0: u64,
}

impl Alg<'_> {
    fn c(&self, a: u64) -> Q {
        (a % self.k.p, 0)
    }
    fn add(&self, a: Q, b: Q) -> Q {
        (self.k.add(a.0, b.0), self.k.add(a.1, b.1))
    }
    fn sub(&self, a: Q, b: Q) -> Q {
        (self.k.sub(a.0, b.0), self.k.sub(a.1, b.1))
    }
    fn mul(&self, a: Q, b: Q) -> Q {
        let k = self.k;
        let a1b1 = k.mul(a.1, b.1);
        (
            k.sub(k.mul(a.0, b.0), k.mul(a1b1, self.u0)),
            k.sub(
                k.add(k.mul(a.0, b.1), k.mul(a.1, b.0)),
                k.mul(a1b1, self.u1),
            ),
        )
    }
    /// The conjugate `x̄ ↦ −u1 − x̄`.
    fn conj(&self, a: Q) -> Q {
        (self.k.sub(a.0, self.k.mul(a.1, self.u1)), self.k.neg(a.1))
    }
    fn inv(&self, a: Q) -> Option<Q> {
        let n = self.mul(a, self.conj(a));
        debug_assert_eq!(n.1, 0);
        if n.0 == 0 {
            return None;
        }
        let ni = self.k.inv(n.0);
        let c = self.conj(a);
        Some((self.k.mul(c.0, ni), self.k.mul(c.1, ni)))
    }
    /// `g(x̄)` for `g ∈ F_p[x]`.
    fn eval(&self, g: &[u64]) -> Q {
        let xb = (0, 1);
        g.iter()
            .rev()
            .fold((0, 0), |acc, &c| self.add(self.mul(acc, xb), self.c(c)))
    }
}

type APoly = Vec<Q>;

fn atrim(mut a: APoly) -> APoly {
    while a.last() == Some(&(0, 0)) {
        a.pop();
    }
    a
}

fn apadd(al: &Alg, a: &[Q], b: &[Q]) -> APoly {
    let n = a.len().max(b.len());
    atrim(
        (0..n)
            .map(|i| al.add(*a.get(i).unwrap_or(&(0, 0)), *b.get(i).unwrap_or(&(0, 0))))
            .collect(),
    )
}

fn apsub(al: &Alg, a: &[Q], b: &[Q]) -> APoly {
    let n = a.len().max(b.len());
    atrim(
        (0..n)
            .map(|i| al.sub(*a.get(i).unwrap_or(&(0, 0)), *b.get(i).unwrap_or(&(0, 0))))
            .collect(),
    )
}

fn apmul(al: &Alg, a: &[Q], b: &[Q]) -> APoly {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut r = vec![(0, 0); a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = al.add(r[i + j], al.mul(x, y));
        }
    }
    atrim(r)
}

fn apscale(al: &Alg, a: &[Q], c: Q) -> APoly {
    atrim(a.iter().map(|&x| al.mul(x, c)).collect())
}

fn apconj(al: &Alg, a: &[Q]) -> APoly {
    atrim(a.iter().map(|&x| al.conj(x)).collect())
}

/// Division with remainder; `None` when the leading coefficient of `b` is
/// not a unit of the algebra (a zero divisor, in the split case).
fn apdivrem(al: &Alg, a: &[Q], b: &[Q]) -> Option<(APoly, APoly)> {
    let mut r = atrim(a.to_vec());
    let b = atrim(b.to_vec());
    let lb = *b.last()?;
    let li = al.inv(lb)?;
    if r.len() < b.len() {
        return Some((Vec::new(), r));
    }
    let db = b.len() - 1;
    let mut q = vec![(0, 0); r.len() - db];
    while r.len() >= b.len() {
        let d = r.len() - 1;
        let c = al.mul(r[d], li);
        q[d - db] = c;
        for (j, &bj) in b.iter().enumerate() {
            r[d - db + j] = al.sub(r[d - db + j], al.mul(c, bj));
        }
        r = atrim(r);
    }
    Some((atrim(q), r))
}

/// `a⁻¹ mod m` in `A[t]` by the extended Euclidean algorithm; `None` when a
/// non-unit leading coefficient appears or the gcd is not a unit.
fn apinv_mod(al: &Alg, a: &[Q], m: &[Q]) -> Option<APoly> {
    let (mut r0, mut r1) = (atrim(m.to_vec()), atrim(a.to_vec()));
    let (mut t0, mut t1): (APoly, APoly) = (Vec::new(), vec![(1, 0)]);
    while !r1.is_empty() {
        let (q, r) = apdivrem(al, &r0, &r1)?;
        let t = apsub(al, &t0, &apmul(al, &q, &t1));
        r0 = std::mem::replace(&mut r1, r);
        t0 = std::mem::replace(&mut t1, t);
    }
    if r0.len() != 1 {
        return None;
    }
    let li = al.inv(r0[0])?;
    let (_, t) = apdivrem(al, &apscale(al, &t0, li), m)?;
    Some(t)
}

// ── Richelot (2,2)-isogeny ──────────────────────────────────────────────────

#[derive(Serialize, Clone)]
struct RichelotRecord {
    roots: Vec<u64>,
    partition: [Vec<u64>; 3],
    kappa: u64,
    delta: u64,
    lambda: u64,
    f_prime_sextic: Vec<u64>,
    rational_weierstrass_points_sextic: Vec<u64>,
    rho: u64,
    f_tilde_quintic: Vec<u64>,
    order_j: u64,
    order_j_prime: u64,
    order_j_prime_sextic_count_fp: u64,
    order_j_prime_quintic_count_fp: u64,
    kernel_elements: Vec<String>,
    kernel_closed: bool,
    kernel_isotropic_derived: bool,
    phi_zero_on_kernel: usize,
    phi_nonzero_on_other_2_torsion: usize,
    hom_checks: usize,
    hom_failures: usize,
    welldef_checks: usize,
    welldef_failures: usize,
    degree1_direct_formula_agreements: usize,
    degree1_direct_formula_checks: usize,
    image_invalid_mumford: usize,
    image_nongeneric_retries: usize,
    psi_prime_trivial_on_image: usize,
    psi_prime_checks: usize,
    psi_prime_values_seen: usize,
    psi_hom_checks: usize,
    psi_hom_failures: usize,
    psi_values_seen: usize,
    psi_trivial_on_g: usize,
    psi_g_checks: usize,
    psi_kernel_density: f64,
    psi_density_samples: usize,
    phi_of_h4_in_2jprime: usize,
    phi_of_h4_checks: usize,
    jprime_rational_2_torsion: usize,
}

struct Richelot<'a> {
    k: &'a Fp,
    g: [Poly; 3],
    h: [Poly; 3],
    rho: u64,
    /// `H_i(ρ)`, `H_i′(ρ)`, `lc(H_i)` for `i = 1, 2`.
    hr: [u64; 2],
    hdr: [u64; 2],
    ha: [u64; 2],
    ftilde: Poly,
    cp: Genus2Curve,
    nongeneric: Cell<usize>,
    invalid: Cell<usize>,
}

impl<'a> Richelot<'a> {
    /// `f` must split completely; `part = [k0, k1, k2, k3, k4]` indexes the
    /// roots: `G₃ = x − e_{k0}`, `G₁ = (x − e_{k1})(x − e_{k2})`,
    /// `G₂ = (x − e_{k3})(x − e_{k4})`.
    fn build(
        k: &'a Fp,
        c: &Genus2Curve,
        roots: &[u64],
        part: [usize; 5],
    ) -> Result<(Richelot<'a>, RichelotRecord), String> {
        let lin = |e: u64| vec![k.neg(e), 1];
        let g3 = lin(roots[part[0]]);
        let g1 = pmul(k, &lin(roots[part[1]]), &lin(roots[part[2]]));
        let g2 = pmul(k, &lin(roots[part[3]]), &lin(roots[part[4]]));
        let f: Poly = trim(c.f().to_vec());
        assert_eq!(pmul(k, &pmul(k, &g1, &g2), &g3), f, "G₁G₂G₃ ≠ f");
        let br =
            |a: &[u64], b: &[u64]| psub(k, &pmul(k, &pderiv(k, a), b), &pmul(k, a, &pderiv(k, b)));
        let h1 = br(&g2, &g3);
        let h2 = br(&g3, &g1);
        let h3 = br(&g1, &g2);
        // Σ G_i(x)H_i(X) = κ (x − X)²: κ = S(1, 0); verified at random points.
        let s = |x: u64, xx: u64| {
            let mut t = 0;
            for (gi, hi) in [(&g1, &h1), (&g2, &h2), (&g3, &h3)] {
                t = k.add(t, k.mul(peval(k, gi, x), peval(k, hi, xx)));
            }
            t
        };
        let kappa = s(1, 0);
        for (x, xx) in [(2, 5), (7, 3), (11, 13), (0, 1), (17, 17)] {
            let d = k.sub(x % k.p, xx % k.p);
            assert_eq!(
                s(x % k.p, xx % k.p),
                k.mul(kappa, k.mul(d, d)),
                "Σ G_iH_i ≠ κ(x−X)²"
            );
        }
        // δ = det of the coefficient matrix (rows G₁, G₂, G₃; columns x², x, 1).
        let co = |g: &Poly, i: usize| *g.get(i).unwrap_or(&0);
        let m = [
            [co(&g1, 2), co(&g1, 1), co(&g1, 0)],
            [co(&g2, 2), co(&g2, 1), co(&g2, 0)],
            [co(&g3, 2), co(&g3, 1), co(&g3, 0)],
        ];
        let det2 = |a: u64, b: u64, c2: u64, d: u64| k.sub(k.mul(a, d), k.mul(b, c2));
        let delta = k.add(
            k.sub(
                k.mul(m[0][0], det2(m[1][1], m[1][2], m[2][1], m[2][2])),
                k.mul(m[0][1], det2(m[1][0], m[1][2], m[2][0], m[2][2])),
            ),
            k.mul(m[0][2], det2(m[1][0], m[1][1], m[2][0], m[2][1])),
        );
        if kappa == 0 {
            return Err("κ = 0: the codomain is a product of elliptic curves".into());
        }
        let lambda = k.neg(k.inv(kappa));
        let fsext = pscale(k, &pmul(k, &pmul(k, &h1, &h2), &h3), lambda);
        let wpoints: Vec<u64> = (0..k.p).filter(|&x| peval(k, &fsext, x) == 0).collect();
        let Some(&rho) = wpoints.first() else {
            return Err("F′ has no rational root: no quintic model over F_p".into());
        };
        // F̃(t) = t⁶ F′(ρ + 1/t) = Σ c_i (ρt + 1)^i t^{6−i}
        let mut ftilde: Poly = Vec::new();
        for (i, &ci) in fsext.iter().enumerate() {
            let term = pmul(k, &ppow(k, &[1, rho], i), &ppow(k, &[0, 1], 6 - i));
            ftilde = padd(k, &ftilde, &pscale(k, &term, ci));
        }
        if ftilde.len() != 6 {
            return Err(format!(
                "F̃ has degree {} ≠ 5",
                ftilde.len().saturating_sub(1)
            ));
        }
        let mut fq = [0u64; 6];
        fq.copy_from_slice(&ftilde);
        let cp = Genus2Curve::new(k.p, fq).map_err(|e| format!("codomain: {e}"))?;
        let lc = |h: &Poly| *h.get(2).unwrap_or(&0);
        let r = Richelot {
            k,
            g: [g1.clone(), g2.clone(), g3.clone()],
            h: [h1.clone(), h2.clone(), h3.clone()],
            rho,
            hr: [peval(k, &h1, rho), peval(k, &h2, rho)],
            hdr: [
                peval(k, &pderiv(k, &h1), rho),
                peval(k, &pderiv(k, &h2), rho),
            ],
            ha: [lc(&h1), lc(&h2)],
            ftilde: ftilde.clone(),
            cp,
            nongeneric: Cell::new(0),
            invalid: Cell::new(0),
        };
        let rec = RichelotRecord {
            roots: roots.to_vec(),
            partition: [
                vec![roots[part[0]]],
                vec![roots[part[1]], roots[part[2]]],
                vec![roots[part[3]], roots[part[4]]],
            ],
            kappa,
            delta,
            lambda,
            f_prime_sextic: fsext,
            rational_weierstrass_points_sextic: wpoints,
            rho,
            f_tilde_quintic: ftilde,
            order_j: 0,
            order_j_prime: 0,
            order_j_prime_sextic_count_fp: 0,
            order_j_prime_quintic_count_fp: 0,
            kernel_elements: Vec::new(),
            kernel_closed: false,
            kernel_isotropic_derived: true,
            phi_zero_on_kernel: 0,
            phi_nonzero_on_other_2_torsion: 0,
            hom_checks: 0,
            hom_failures: 0,
            welldef_checks: 0,
            welldef_failures: 0,
            degree1_direct_formula_agreements: 0,
            degree1_direct_formula_checks: 0,
            image_invalid_mumford: 0,
            image_nongeneric_retries: 0,
            psi_prime_trivial_on_image: 0,
            psi_prime_checks: 0,
            psi_prime_values_seen: 0,
            psi_hom_checks: 0,
            psi_hom_failures: 0,
            psi_values_seen: 0,
            psi_trivial_on_g: 0,
            psi_g_checks: 0,
            psi_kernel_density: 0.0,
            psi_density_samples: 0,
            phi_of_h4_in_2jprime: 0,
            phi_of_h4_checks: 0,
            jprime_rational_2_torsion: 0,
        };
        Ok((r, rec))
    }

    /// `#C′(F_p)` on the sextic model (points at infinity included).
    fn sextic_point_count(&self, fsext: &[u64]) -> u64 {
        let k = self.k;
        let mut n = 0u64;
        for x in 0..k.p {
            n += (1 + i64::from(k.legendre(peval(k, fsext, x)))) as u64;
        }
        n + (1 + i64::from(k.legendre(*fsext.get(6).unwrap_or(&0)))) as u64
    }

    /// The `(2,2)`-descent map `D ↦ (χ(G₁(D)), χ(G₂(D)))`; `None` when a
    /// Weierstrass point is in the support.
    fn psi(&self, d: &Divisor) -> Option<(i8, i8)> {
        let u = d.u_coeffs();
        let a = self
            .k
            .legendre(resultant_with_monic(self.k, &u, &self.g[0]));
        let b = self
            .k
            .legendre(resultant_with_monic(self.k, &u, &self.g[1]));
        (a != 0 && b != 0).then_some((a, b))
    }

    /// `t²·H_i(ρ + 1/t)`, the factors of `F̃` up to the scalar `λ`.
    fn h_tilde(&self, i: usize) -> Poly {
        let k = self.k;
        let h = &self.h[i];
        let lc = *h.get(2).unwrap_or(&0);
        trim(vec![
            lc,
            peval(k, &pderiv(k, h), self.rho),
            peval(k, h, self.rho),
        ])
    }

    /// The descent map on `J′` by the quadratic characters of the three
    /// factors `H̃_i`; `None` on a Weierstrass support.
    fn psi_prime(&self, d: &Divisor) -> Option<[i8; 3]> {
        let u = d.u_coeffs();
        let mut out = [0i8; 3];
        for (i, o) in out.iter_mut().enumerate() {
            *o = self
                .k
                .legendre(resultant_with_monic(self.k, &u, &self.h_tilde(i)));
            if *o == 0 {
                return None;
            }
        }
        Some(out)
    }

    /// The 2-descent map on `J′` over the rational roots of `F̃` (one
    /// character per rational Weierstrass point); `None` on Weierstrass
    /// support. Its kernel is `2J′(F_p)` when `J′[2]` is rational.
    fn two_descent_prime(&self, d: &Divisor, wroots: &[u64]) -> Option<Vec<i8>> {
        let u = d.u_coeffs();
        let mut out = Vec::new();
        for &e in wroots {
            let c = self
                .k
                .legendre(resultant_with_monic(self.k, &u, &[self.k.neg(e), 1]));
            if c == 0 {
                return None;
            }
            out.push(c);
        }
        Some(out)
    }

    /// The correspondence applied to a finite non-Weierstrass rational
    /// point: `[R(P) − 2∞̃]` on `J′` (direct formula; `None` when degenerate).
    fn image_point(&self, x1: u64, y1: u64) -> Option<Divisor> {
        let k = self.k;
        let g1 = peval(k, &self.g[0], x1);
        let g2 = peval(k, &self.g[1], x1);
        if y1 == 0 {
            return None;
        }
        let c2 = k.add(k.mul(g1, self.hr[0]), k.mul(g2, self.hr[1]));
        let c1 = k.add(k.mul(g1, self.hdr[0]), k.mul(g2, self.hdr[1]));
        let c0 = k.add(k.mul(g1, self.ha[0]), k.mul(g2, self.ha[1]));
        if c2 == 0 {
            return None;
        }
        let lm = pmonic(k, &[c0, c1, c2]);
        // M(t) = G₁(x)·(H₁(ρ)t² + H₁′(ρ)t + a₁)·((x − ρ)t − 1)
        let hq = trim(vec![self.ha[0], self.hdr[0], self.hr[0]]);
        let m = pscale(k, &pmul(k, &hq, &[k.neg(1), k.sub(x1, self.rho)]), g1);
        let w = pdivrem(k, &pscale(k, &m, k.inv(y1)), &lm).1;
        self.cp.divisor(&lm, &w).ok()
    }

    /// `Φ(E) = [R(E) − 4∞̃]` for a generic reduced `E` of degree 2 (both
    /// points finite, non-Weierstrass, distinct and generic); `None`
    /// otherwise. Any output has been checked as a valid Mumford pair.
    fn image_generic(&self, e: &Divisor) -> Option<Divisor> {
        let k = self.k;
        if e.degree() != 2 {
            return None;
        }
        let u = e.u_coeffs();
        let v = e.v_coeffs();
        // squarefree u
        let disc = k.sub(k.mul(u[1], u[1]), k.mul(4, u[0]));
        if disc == 0 {
            self.nongeneric.set(self.nongeneric.get() + 1);
            return None;
        }
        let al = Alg {
            k,
            u1: u[1],
            u0: u[0],
        };
        let g1 = al.eval(&self.g[0]);
        let g2 = al.eval(&self.g[1]);
        let y = al.eval(&v);
        let Some(yi) = al.inv(y) else {
            self.nongeneric.set(self.nongeneric.get() + 1);
            return None;
        };
        let c2 = al.add(al.mul(g1, al.c(self.hr[0])), al.mul(g2, al.c(self.hr[1])));
        let c1 = al.add(al.mul(g1, al.c(self.hdr[0])), al.mul(g2, al.c(self.hdr[1])));
        let c0 = al.add(al.mul(g1, al.c(self.ha[0])), al.mul(g2, al.c(self.ha[1])));
        let Some(c2i) = al.inv(c2) else {
            self.nongeneric.set(self.nongeneric.get() + 1);
            return None;
        };
        let lm = apscale(&al, &[c0, c1, c2], c2i);
        let hq: APoly = vec![al.c(self.ha[0]), al.c(self.hdr[0]), al.c(self.hr[0])];
        let lin: APoly = vec![al.c(k.neg(1)), al.sub((0, 1), al.c(self.rho))];
        let m = apscale(&al, &apmul(&al, &hq, &lin), al.mul(g1, yi));
        let Some((_, w)) = apdivrem(&al, &m, &lm) else {
            self.nongeneric.set(self.nongeneric.get() + 1);
            return None;
        };
        let lmc = apconj(&al, &lm);
        // U = Lm · L̄m ∈ F_p[t]; V ≡ W (mod Lm), V ≡ W̄ (mod L̄m).
        let uu = apmul(&al, &lm, &lmc);
        let Some(lminv) = apinv_mod(&al, &lm, &lmc) else {
            self.nongeneric.set(self.nongeneric.get() + 1);
            return None;
        };
        let diff = apsub(&al, &apconj(&al, &w), &w);
        let Some((_, q)) = apdivrem(&al, &apmul(&al, &diff, &lminv), &lmc) else {
            self.nongeneric.set(self.nongeneric.get() + 1);
            return None;
        };
        let vv = apadd(&al, &w, &apmul(&al, &lm, &q));
        if uu.iter().chain(vv.iter()).any(|c| c.1 != 0) {
            self.invalid.set(self.invalid.get() + 1);
            return None;
        }
        let uu: Poly = trim(uu.iter().map(|c| c.0).collect());
        let vv: Poly = trim(vv.iter().map(|c| c.0).collect());
        if uu.len() != 5 {
            self.nongeneric.set(self.nongeneric.get() + 1);
            return None;
        }
        let (q2, rem) = pdivrem(k, &psub(k, &self.ftilde, &pmul(k, &vv, &vv)), &uu);
        if !rem.is_empty() {
            self.invalid.set(self.invalid.get() + 1);
            return None;
        }
        // one Cantor reduction step: deg q2 ≤ 2
        let u2 = pmonic(k, &q2);
        let nv: Poly = trim(vv.iter().map(|&c| k.neg(c)).collect());
        let v2 = pdivrem(k, &nv, &u2).1;
        match self.cp.divisor(&u2, &v2) {
            Ok(d) => Some(d),
            Err(_) => {
                self.invalid.set(self.invalid.get() + 1);
                None
            }
        }
    }

    /// `φ(D) = Φ(D + A) − Φ(A)` for a random generic `A`.
    fn phi<R: Rng>(&self, c: &Genus2Curve, d: &Divisor, rng: &mut R) -> Divisor {
        if d.is_identity() {
            return Divisor::IDENTITY;
        }
        for _ in 0..200 {
            let a = c.random(rng);
            let e = c.add(d, &a);
            if let (Some(ie), Some(ia)) = (self.image_generic(&e), self.image_generic(&a)) {
                return self.cp.sub(&ie, &ia);
            }
        }
        panic!("Richelot image: no generic auxiliary divisor found after 200 draws");
    }
}

// ── Census (exactly §15.2's, ported from round 4 to the library types) ──────

#[derive(Serialize, Clone)]
struct MStat {
    distinct: u64,
    distinct_strict: u64,
    representations: u64,
    hit_rate: f64,
    c: f64,
    cost_model: f64,
}

#[derive(Serialize, Clone)]
struct OracleStat {
    targets: usize,
    enumeration_hits: usize,
    oracle_hits: usize,
    disagreements: usize,
    mean_muls: f64,
    mean_invs: f64,
    mean_sqrts: f64,
    exact_hit_rate: f64,
}

#[derive(Serialize, Clone)]
struct Census {
    arm: String,
    rep: usize,
    points: usize,
    degree1_points: usize,
    x_set_size: usize,
    columns: usize,
    classes: usize,
    m2: MStat,
    m3: MStat,
    oracle2: Option<OracleStat>,
    oracle3: Option<OracleStat>,
}

struct Ctx<'a> {
    c: &'a Genus2Curve,
    r: u64,
    h: u64,
    use_zeta: bool,
}

struct CensusSets {
    census: Census,
    hits2: HashSet<Divisor>,
    hits3: HashSet<Divisor>,
}

fn census(cx: &Ctx, arm: &str, rep: usize, base: &[Divisor]) -> CensusSets {
    let c = cx.c;
    let n = base.len();
    let keys: HashSet<Divisor> = base.iter().copied().collect();
    assert_eq!(keys.len(), n, "{arm}: duplicate base elements");
    for d in base {
        assert!(c.is_valid(d), "{arm}: invalid base element");
        assert!(keys.contains(&c.neg(d)), "{arm}: not signed");
    }
    let cls: Vec<Divisor> = base.iter().map(|d| c.mul(d, cx.r)).collect();
    let classes: HashSet<Divisor> = cls.iter().copied().collect();
    let mut seen: HashSet<Divisor> = HashSet::new();
    let mut cols = 0;
    for d in base {
        let q = c.mul(d, cx.h);
        if q.is_identity() || seen.contains(&q) {
            continue;
        }
        cols += 1;
        let mut z = q;
        let orbit = if cx.use_zeta { 5 } else { 1 };
        for _ in 0..orbit {
            seen.insert(z);
            seen.insert(c.neg(&z));
            if cx.use_zeta {
                z = c.zeta(&z).unwrap();
            }
        }
    }
    let mut pairs: HashMap<Divisor, Vec<(usize, usize, Divisor)>> = HashMap::new();
    let mut hits2: HashSet<Divisor> = HashSet::new();
    let mut hits2s: HashSet<Divisor> = HashSet::new();
    let mut reps2 = 0u64;
    for i in 0..n {
        for j in i..n {
            let s = c.add(&base[i], &base[j]);
            let cl = c.add(&cls[i], &cls[j]);
            if cl.is_identity() && !s.is_identity() {
                hits2.insert(s);
                if i < j {
                    hits2s.insert(s);
                }
                reps2 += 1;
            }
            pairs.entry(cl).or_default().push((i, j, s));
        }
    }
    let mut hits3: HashSet<Divisor> = HashSet::new();
    let mut hits3s: HashSet<Divisor> = HashSet::new();
    let mut reps3 = 0u64;
    for i in 0..n {
        let need = c.neg(&cls[i]);
        if let Some(v) = pairs.get(&need) {
            for (j, kk, s) in v {
                if *j < i {
                    continue;
                }
                let t = c.add(s, &base[i]);
                if !t.is_identity() {
                    hits3.insert(t);
                    if i < *j && j < kk {
                        hits3s.insert(t);
                    }
                    reps3 += 1;
                }
            }
        }
    }
    let mk = |d: usize, ds: usize, reps: u64, fact: f64, m: i32, l: f64| MStat {
        distinct: d as u64,
        distinct_strict: ds as u64,
        representations: reps,
        hit_rate: d as f64 / cx.r as f64,
        c: d as f64 * fact / (n as f64).powi(m),
        cost_model: if d > 0 {
            cols as f64 * (cx.r as f64 / d as f64) * l
        } else {
            f64::NAN
        },
    };
    let xset: HashSet<u64> = base
        .iter()
        .filter(|d| d.degree() == 1)
        .map(|d| (c.p() - d.u_coeffs()[0]) % c.p())
        .collect();
    CensusSets {
        census: Census {
            arm: arm.to_string(),
            rep,
            points: n,
            degree1_points: base.iter().filter(|d| d.degree() == 1).count(),
            x_set_size: xset.len(),
            columns: cols,
            classes: classes.len(),
            m2: mk(hits2.len(), hits2s.len(), reps2, 2.0, 2, 1.0),
            m3: mk(hits3.len(), hits3s.len(), reps3, 6.0, 3, n as f64),
            oracle2: None,
            oracle3: None,
        },
        hits2,
        hits3,
    }
}

// ── The Nagao-style algebraic oracle ────────────────────────────────────────

/// `m = 2`: `D = [P₁ − ∞] + [P₂ − ∞]` with `x(P_i)` in the `x`-set iff `u`
/// splits over the set. Operation counts accumulate in `k`.
fn nagao2(k: &Fp, xs: &[bool], t: &Divisor) -> bool {
    if t.degree() != 2 {
        return false;
    }
    let u = t.u_coeffs();
    let (u0, u1) = (u[0], u[1]);
    let disc = k.sub(k.mul(u1, u1), k.mul(4, u0));
    let half = k.inv(2);
    if disc == 0 {
        let x = k.mul(k.neg(u1), half);
        return xs[x as usize];
    }
    let Some(s) = k.sqrt(disc) else {
        return false;
    };
    let x1 = k.mul(k.add(k.neg(u1), s), half);
    let x2 = k.mul(k.sub(k.neg(u1), s), half);
    xs[x1 as usize] && xs[x2 as usize]
}

/// `m = 3` by the `λ`-sweep: `D = Σ [P_i − ∞]` iff for some `λ ∈ F_p` the
/// cubic `(f − (λu − v)²)/u` splits with all roots in the `x`-set
/// (`div(y − c(x))`, `c = λu − v`). Exhaustive in `λ`; cost grows as
/// `p·|S|`. Returns the decision; the sweep continues past the first hit
/// only when `full` is set (to count all `λ`).
fn nagao3_sweep(k: &Fp, f: &[u64], xset: &[u64], xs: &[bool], t: &Divisor) -> bool {
    if t.degree() != 2 {
        return false;
    }
    let u = t.u_coeffs();
    let v = t.v_coeffs();
    for lam in 0..k.p {
        let c = psub(k, &pscale(k, &u, lam), &v);
        let (q, rem) = pdivrem(k, &psub(k, f, &pmul(k, &c, &c)), &u);
        debug_assert!(rem.is_empty());
        if q.len() != 4 {
            continue;
        }
        // roots of the monic cubic q in the x-set (with multiplicity)
        let mut roots: Vec<u64> = Vec::new();
        for &s in xset {
            if peval(k, &q, s) == 0 {
                roots.push(s);
            }
        }
        if roots.is_empty() {
            continue;
        }
        let ok = match roots.len() {
            3 => true,
            2 => {
                let (a, b) = (roots[0], roots[1]);
                let cand1 = pmul(k, &pmul(k, &[k.neg(a), 1], &[k.neg(a), 1]), &[k.neg(b), 1]);
                let cand2 = pmul(k, &pmul(k, &[k.neg(b), 1], &[k.neg(b), 1]), &[k.neg(a), 1]);
                cand1 == q || cand2 == q
            }
            1 => {
                let a = roots[0];
                ppow(k, &[k.neg(a), 1], 3) == q
            }
            _ => false,
        };
        if ok && xs[roots[0] as usize] {
            return true;
        }
    }
    false
}

// ── Base construction ───────────────────────────────────────────────────────

/// Signed degree-1 base: `want` classes `[P − ∞]`, both signs, `x` drawn in
/// random order (or increasing), from distinct `ζ`-orbits when asked,
/// filtered by `keep` on the point class.
#[allow(clippy::too_many_arguments)]
fn pick_points<F: Fn(u64, &Divisor) -> bool>(
    c: &Genus2Curve,
    want: usize,
    rng: &mut StdRng,
    distinct_orbit: bool,
    keep: F,
) -> Option<Vec<Divisor>> {
    let p = c.p();
    let mut out = Vec::new();
    let mut used5: HashSet<u64> = HashSet::new();
    let mut xs: Vec<u64> = (1..p).collect();
    for i in (1..xs.len()).rev() {
        let j = rng.gen_range(0..=i);
        xs.swap(i, j);
    }
    for x in xs {
        if out.len() >= want {
            break;
        }
        let x5 = {
            let mut t = 1u64;
            for _ in 0..5 {
                t = t * x % p;
            }
            t
        };
        if distinct_orbit && used5.contains(&x5) {
            continue;
        }
        let l = c.divisors_with_u(&[(p - x) % p, 1]).unwrap();
        if l.len() != 2 || !keep(x, &l[0]) {
            continue;
        }
        used5.insert(x5);
        out.extend(l);
    }
    if out.len() < want {
        return None;
    }
    out.truncate(want);
    Some(out)
}

/// `S ∪ ζS ∪ … ∪ ζ⁴S` with `S` drawn from distinct orbits (`|F|` multiple of 10).
fn a_closed(c: &Genus2Curve, want: usize, rng: &mut StdRng) -> Option<Vec<Divisor>> {
    assert!(want.is_multiple_of(10));
    let s = pick_points(c, want / 5, rng, true, |_, _| true)?;
    let mut out = Vec::new();
    for d in s {
        let mut z = d;
        for _ in 0..5 {
            out.push(z);
            z = c.zeta(&z).unwrap();
        }
    }
    let set: HashSet<Divisor> = out.iter().copied().collect();
    assert_eq!(set.len(), out.len(), "A-closed base: orbit collision");
    Some(out)
}

// ── Summary with bootstrap ──────────────────────────────────────────────────

#[derive(Serialize, Clone)]
struct Stat {
    value: f64,
    ci95: (f64, f64),
}

#[derive(Serialize)]
struct ArmSummary {
    arm: String,
    reference: String,
    /// Predicted `[U ratio, c₂ ratio, c₃ ratio, cost₂ ratio, cost₃ ratio, unknowns gain |F|/U]`.
    predicted: [f64; 6],
    n_reps: usize,
    columns_ratio: Stat,
    unknowns_gain: Stat,
    c2: Stat,
    c3: Stat,
    c2_strict: Stat,
    c3_strict: Stat,
    cost2: Stat,
    cost3: Stat,
    cost2_nominal: Stat,
    cost3_nominal: Stat,
    hit_rate2: Stat,
    hit_rate3: Stat,
    ref_columns_frac: Stat,
    ref_diag_frac2: Stat,
    oracle2_muls_per_call: Stat,
    oracle2_sqrts_per_call: Stat,
    oracle2_calls_per_relation: Stat,
    oracle2_sampled_hit_rate: Stat,
    oracle2_disagreements: u64,
    oracle3_disagreements: u64,
    oracle3_targets: u64,
    /// Derived oracle cost ratio to the reference: `U · (calls per relation) · (muls per call)`.
    oracle_cost2_ratio: Stat,
}

const NC: usize = 20;

fn row(c: &Census, p: &Census) -> [f64; NC] {
    let o = c.oracle2.as_ref();
    [
        c.columns as f64,
        c.m2.distinct as f64,
        c.m3.distinct as f64,
        c.m2.distinct_strict as f64,
        c.m3.distinct_strict as f64,
        c.m3.hit_rate,
        p.columns as f64,
        p.m2.distinct as f64,
        p.m3.distinct as f64,
        p.m2.distinct_strict as f64,
        p.m3.distinct_strict as f64,
        p.points as f64 / 2.0,
        c.points as f64,
        c.m2.hit_rate,
        o.map_or(f64::NAN, |o| o.mean_muls),
        o.map_or(f64::NAN, |o| o.mean_sqrts),
        o.map_or(f64::NAN, |o| o.oracle_hits as f64 / o.targets.max(1) as f64),
        p.oracle2.as_ref().map_or(f64::NAN, |o| o.mean_muls),
        c.m2.hit_rate,
        p.m2.hit_rate,
    ]
}

fn metrics(m: &[f64; NC]) -> [f64; NC] {
    let u = m[0] / m[6];
    [
        u,
        m[1] / m[7],
        m[2] / m[8],
        m[3] / m[9],
        m[4] / m[10],
        u * m[7] / m[1],
        u * m[8] / m[2],
        (m[0] / m[11]) * m[7] / m[1],
        (m[0] / m[11]) * m[8] / m[2],
        m[5],
        m[6] / m[11],
        1.0 - m[9] / m[7],
        m[12] / m[0],
        m[13],
        m[14],
        m[15],
        1.0 / m[18],
        m[16],
        // oracle cost ratio: (U_arm · muls_arm / rate_arm) / (U_ref · muls_ref / rate_ref), nominal reference columns
        (m[0] / m[11]) * (m[14] / m[17]) * (m[19] / m[18]),
        0.0,
    ]
}

fn predicted(arm: &str, h: u64) -> [f64; 6] {
    let law = |a: f64, i: f64| [1.0 / a, i, i, 1.0 / (a * i), 1.0 / (a * i), 2.0 * a];
    match arm {
        "A" => law(5.0, 1.0),
        "R" | "phi_plain" => law(1.0, 4.0),
        "phi_A" => [1.0, 4.0, 4.0, 0.25, 0.25, 2.0],
        "G" => law(1.0, h as f64),
        _ => [1.0, 1.0, 1.0, 1.0, 1.0, 2.0],
    }
}

fn summarise(res: &[Census], reference: &str, h: u64, seed: u64) -> Vec<ArmSummary> {
    let mut arms: Vec<String> = Vec::new();
    for c in res {
        if c.arm != reference && !arms.contains(&c.arm) {
            arms.push(c.arm.clone());
        }
    }
    let refc = |rep: usize| res.iter().find(|c| c.arm == reference && c.rep == rep);
    let mean_rows = |rows: &[[f64; NC]], idx: &[usize]| -> [f64; NC] {
        let mut s = [0.0; NC];
        for &i in idx {
            for k in 0..NC {
                s[k] += rows[i][k];
            }
        }
        s.map(|x| x / idx.len() as f64)
    };
    let mut rng = StdRng::seed_from_u64(seed ^ 0x5eed);
    let mut out = Vec::new();
    for arm in arms {
        let mut rows: Vec<[f64; NC]> = Vec::new();
        let (mut dis2, mut dis3, mut n3) = (0u64, 0u64, 0u64);
        for c in res.iter().filter(|c| c.arm == arm) {
            let Some(p) = refc(c.rep) else { continue };
            rows.push(row(c, p));
            dis2 += c.oracle2.as_ref().map_or(0, |o| o.disagreements as u64);
            dis3 += c.oracle3.as_ref().map_or(0, |o| o.disagreements as u64);
            n3 += c.oracle3.as_ref().map_or(0, |o| o.targets as u64);
        }
        if rows.is_empty() {
            continue;
        }
        let all: Vec<usize> = (0..rows.len()).collect();
        let point = metrics(&mean_rows(&rows, &all));
        let mut boots: Vec<[f64; NC]> = Vec::new();
        for _ in 0..2000 {
            let idx: Vec<usize> = (0..rows.len())
                .map(|_| rng.gen_range(0..rows.len()))
                .collect();
            boots.push(metrics(&mean_rows(&rows, &idx)));
        }
        let col = |k: usize| {
            let mut v: Vec<f64> = boots.iter().map(|b| b[k]).collect();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            Stat {
                value: point[k],
                ci95: (v[50], v[1949]),
            }
        };
        out.push(ArmSummary {
            predicted: predicted(&arm, h),
            arm,
            reference: reference.to_string(),
            n_reps: rows.len(),
            columns_ratio: col(0),
            c2: col(1),
            c3: col(2),
            c2_strict: col(3),
            c3_strict: col(4),
            cost2: col(5),
            cost3: col(6),
            cost2_nominal: col(7),
            cost3_nominal: col(8),
            hit_rate3: col(9),
            ref_columns_frac: col(10),
            ref_diag_frac2: col(11),
            unknowns_gain: col(12),
            hit_rate2: col(13),
            oracle2_muls_per_call: col(14),
            oracle2_sqrts_per_call: col(15),
            oracle2_calls_per_relation: col(16),
            oracle2_sampled_hit_rate: col(17),
            oracle2_disagreements: dis2,
            oracle3_disagreements: dis3,
            oracle3_targets: n3,
            oracle_cost2_ratio: col(18),
        });
    }
    out
}

// ── Translation closure ─────────────────────────────────────────────────────

#[derive(Serialize)]
struct TranslationRecord {
    affine_points: usize,
    two_torsion_translates: usize,
    random_translates: usize,
    /// Histogram of `|(C − ∞) ∩ (C − ∞ − t)|` over the sampled `t ≠ 0`: index = size.
    histogram: Vec<u64>,
    max_intersection: usize,
    /// The largest degree-1 base closed under some sampled `t ≠ 0` has at most this many elements.
    bound_on_closed_base: usize,
    kernel_translates_detail: Vec<(String, usize)>,
}

fn translation_closure(
    c: &Genus2Curve,
    points: &[Divisor],
    ts: &[(String, Divisor)],
) -> TranslationRecord {
    let mut hist = vec![0u64; 4];
    let mut mx = 0;
    let mut detail = Vec::new();
    let mut two = 0;
    let mut rnd = 0;
    for (name, t) in ts {
        assert!(!t.is_identity());
        let n = points.iter().filter(|p| c.add(p, t).degree() <= 1).count();
        if n >= hist.len() {
            hist.resize(n + 1, 0);
        }
        hist[n] += 1;
        mx = mx.max(n);
        if name.starts_with("T2") || name.starts_with("T5") {
            two += 1;
            detail.push((name.clone(), n));
        } else {
            rnd += 1;
        }
    }
    TranslationRecord {
        affine_points: points.len(),
        two_torsion_translates: two,
        random_translates: rnd,
        histogram: hist,
        max_intersection: mx,
        bound_on_closed_base: mx,
        kernel_translates_detail: detail,
    }
}

// ── Output ──────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct Validation {
    order_lpoly: u64,
    n1: u64,
    n2: u64,
    h: u64,
    r: u64,
    random_order_checks: usize,
    assoc_checks: usize,
    zeta_hom_checks: usize,
    f_splits: bool,
    rational_2_torsion: usize,
}

#[derive(Serialize)]
struct Out {
    lead: &'static str,
    p: u64,
    b: u64,
    family: String,
    size: usize,
    reps: usize,
    seed: u64,
    oracle_targets: usize,
    sweep3_targets: usize,
    sweep3_bases: usize,
    arms: Vec<String>,
    validation: Validation,
    richelot: Option<RichelotRecord>,
    richelot_error: Option<String>,
    richelot_partitions_tried: Vec<(Vec<u64>, usize)>,
    translation: TranslationRecord,
    summary_j: Vec<ArmSummary>,
    summary_jprime: Vec<ArmSummary>,
    census_j: Vec<Census>,
    census_jprime: Vec<Census>,
    skipped: Vec<String>,
    wall_seconds_contended: f64,
}

fn arg(args: &[String], name: &str, default: &str) -> String {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
        .unwrap_or_else(|| default.to_string())
}

fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut d = 2;
    while d * d <= n {
        let mut e = 0;
        while n.is_multiple_of(d) {
            n /= d;
            e += 1;
        }
        if e > 0 {
            out.push((d, e));
        }
        d += 1;
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

fn oracle_sample(
    k: &Fp,
    cx: &Ctx,
    base: &[Divisor],
    sets: &CensusSets,
    targets: usize,
    rng: &mut StdRng,
    sweep3: usize,
) -> (OracleStat, Option<OracleStat>) {
    let p = cx.c.p();
    let mut xs = vec![false; p as usize];
    let mut xset: Vec<u64> = Vec::new();
    for d in base {
        if d.degree() == 1 {
            let x = (p - d.u_coeffs()[0]) % p;
            if !xs[x as usize] {
                xset.push(x);
            }
            xs[x as usize] = true;
        }
    }
    let f: Poly = trim(cx.c.f().to_vec());
    let draw = |rng: &mut StdRng| loop {
        let t = cx.c.mul(&cx.c.random(rng), cx.h);
        if !t.is_identity() {
            return t;
        }
    };
    let (mut eh, mut oh, mut dis) = (0usize, 0usize, 0usize);
    let (mut sm, mut si, mut ss) = (0u64, 0u64, 0u64);
    for _ in 0..targets {
        let t = draw(rng);
        let truth = sets.hits2.contains(&t);
        k.reset();
        let got = nagao2(k, &xs, &t);
        let (a, b, c2) = k.counts();
        sm += a;
        si += b;
        ss += c2;
        eh += truth as usize;
        oh += got as usize;
        dis += (truth != got) as usize;
    }
    let o2 = OracleStat {
        targets,
        enumeration_hits: eh,
        oracle_hits: oh,
        disagreements: dis,
        mean_muls: sm as f64 / targets as f64,
        mean_invs: si as f64 / targets as f64,
        mean_sqrts: ss as f64 / targets as f64,
        exact_hit_rate: sets.hits2.len() as f64 / (cx.r - 1) as f64,
    };
    let o3 = (sweep3 > 0).then(|| {
        let (mut eh, mut oh, mut dis) = (0usize, 0usize, 0usize);
        let (mut sm, mut si, mut ss) = (0u64, 0u64, 0u64);
        // half the targets are drawn from the enumeration's hit set so that
        // positives are exercised at low hit rates
        let hits: Vec<Divisor> = sets.hits3.iter().copied().collect();
        for i in 0..sweep3 {
            let t = if i % 2 == 0 && !hits.is_empty() {
                hits[rng.gen_range(0..hits.len())]
            } else {
                draw(rng)
            };
            let truth = sets.hits3.contains(&t);
            k.reset();
            let got = nagao3_sweep(k, &f, &xset, &xs, &t);
            let (a, b, c2) = k.counts();
            sm += a;
            si += b;
            ss += c2;
            eh += truth as usize;
            oh += got as usize;
            dis += (truth != got) as usize;
        }
        OracleStat {
            targets: sweep3,
            enumeration_hits: eh,
            oracle_hits: oh,
            disagreements: dis,
            mean_muls: sm as f64 / sweep3 as f64,
            mean_invs: si as f64 / sweep3 as f64,
            mean_sqrts: ss as f64 / sweep3 as f64,
            exact_hit_rate: sets.hits3.len() as f64 / (cx.r - 1) as f64,
        }
    });
    (o2, o3)
}

fn main() {
    let t0 = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();
    let p: u64 = arg(&args, "--p", "1051").parse().unwrap();
    let b: u64 = arg(&args, "--b", "1042").parse().unwrap();
    let size: usize = arg(&args, "--size", "80").parse().unwrap();
    let reps: usize = arg(&args, "--reps", "40").parse().unwrap();
    let seed: u64 = arg(&args, "--seed", "1").parse().unwrap();
    let targets: usize = arg(&args, "--targets", "20000").parse().unwrap();
    let sweep3: usize = arg(&args, "--sweep3", "20").parse().unwrap();
    let sweep3_bases: usize = arg(&args, "--sweep3-bases", "2").parse().unwrap();
    let arms_arg = arg(&args, "--arms", "plain,A,R,G,plainP,phi_plain,phi_A");
    let json = arg(&args, "--json", "");
    let n_rand_t: usize = arg(&args, "--translates", "400").parse().unwrap();
    assert!(p % 10 == 1 && is_prime_u64(p) && p < 1 << 13);
    let arms: Vec<String> = arms_arg.split(',').map(|s| s.to_string()).collect();
    let k = Fp::new(p);
    let c = Genus2Curve::x5_plus_b(p, b).expect("curve");
    let mut rng = StdRng::seed_from_u64(seed);
    let mut skipped: Vec<String> = Vec::new();

    // ── validation of the library curve ──
    let lp = c.l_polynomial().unwrap();
    let order = lp.jacobian_order();
    let fs = factor(order);
    let (r, er) = *fs.last().unwrap();
    assert_eq!(er, 1, "r² divides #J");
    let h = order / r;
    for _ in 0..40 {
        let d = c.random(&mut rng);
        assert!(c.mul(&d, order).is_identity(), "[#J]D != 0");
    }
    for _ in 0..200 {
        let (x, y, z) = (c.random(&mut rng), c.random(&mut rng), c.random(&mut rng));
        assert_eq!(c.add(&c.add(&x, &y), &z), c.add(&x, &c.add(&y, &z)));
        assert_eq!(c.add(&x, &y), c.add(&y, &x));
        assert!(c.add(&x, &c.neg(&x)).is_identity());
        assert_eq!(
            c.zeta(&c.add(&x, &y)).unwrap(),
            c.add(&c.zeta(&x).unwrap(), &c.zeta(&y).unwrap())
        );
    }
    // roots of f = x⁵ + b
    let roots: Vec<u64> = (0..p).filter(|&x| c.eval_f(x) == 0).collect();
    let f_splits = roots.len() == 5;
    let family = if f_splits { "C2" } else { "C1" }.to_string();
    // rational 2-torsion: [e_i − ∞] and [e_i + e_j − 2∞]
    let mut two_torsion: Vec<(String, Divisor)> = Vec::new();
    for (i, &e) in roots.iter().enumerate() {
        two_torsion.push((format!("T2[e{i}]"), c.point(e, 0).unwrap()));
        for (j, &e2) in roots.iter().enumerate().skip(i + 1) {
            let u = pmul(&k, &[k.neg(e), 1], &[k.neg(e2), 1]);
            two_torsion.push((format!("T2[e{i}+e{j}]"), c.divisor(&u, &[]).unwrap()));
        }
    }
    for (_, t) in &two_torsion {
        assert!(c.double(t).is_identity());
    }
    let validation = Validation {
        order_lpoly: order,
        n1: lp.n1,
        n2: lp.n2,
        h,
        r,
        random_order_checks: 40,
        assoc_checks: 200,
        zeta_hom_checks: 200,
        f_splits,
        rational_2_torsion: two_torsion.len(),
    };
    eprintln!("p = {p}, b = {b}, {family}: #J = {order} = {h} · {r}, f splits: {f_splits}, rational 2-torsion: {}", two_torsion.len());

    // ── Richelot ──
    let mut richelot: Option<Richelot> = None;
    let mut rich_rec: Option<RichelotRecord> = None;
    let mut rich_err: Option<String> = None;
    let mut tried: Vec<(Vec<u64>, usize)> = Vec::new();
    if f_splits {
        // all 15 partitions {k0}, {k1,k2}, {k3,k4}; keep the one with the most rational Weierstrass points on C′
        let mut best: Option<(usize, [usize; 5])> = None;
        for k0 in 0..5 {
            let rest: Vec<usize> = (0..5).filter(|&i| i != k0).collect();
            for (a, b2) in [(0, 1), (0, 2), (0, 3)] {
                let others: Vec<usize> = (0..4).filter(|&i| i != a && i != b2).collect();
                let part = [k0, rest[a], rest[b2], rest[others[0]], rest[others[1]]];
                match Richelot::build(&k, &c, &roots, part) {
                    Ok((_, rec)) => {
                        let n = rec.rational_weierstrass_points_sextic.len();
                        tried.push((part.iter().map(|&i| roots[i]).collect(), n));
                        if best.is_none_or(|(bn, _)| n > bn) {
                            best = Some((n, part));
                        }
                    }
                    Err(e) => {
                        tried.push((part.iter().map(|&i| roots[i]).collect(), 0));
                        rich_err = Some(e);
                    }
                }
            }
        }
        if let Some((_, part)) = best {
            let (rc, mut rec) = Richelot::build(&k, &c, &roots, part).unwrap();
            rich_err = None;
            // orders
            let lpp = rc.cp.l_polynomial().unwrap();
            rec.order_j = order;
            rec.order_j_prime = lpp.jacobian_order();
            rec.order_j_prime_sextic_count_fp = rc.sextic_point_count(&rec.f_prime_sextic);
            rec.order_j_prime_quintic_count_fp = lpp.n1;
            assert_eq!(rec.order_j_prime, order, "#J′ ≠ #J");
            assert_eq!(
                rec.order_j_prime_sextic_count_fp, lpp.n1,
                "sextic/quintic point counts differ"
            );
            // kernel K = {0, [e_{k0} − ∞], [G₁ roots], [G₂ roots]}
            let kel: Vec<Divisor> = vec![
                c.point(roots[part[0]], 0).unwrap(),
                c.divisor(&rc.g[0], &[]).unwrap(),
                c.divisor(&rc.g[1], &[]).unwrap(),
            ];
            rec.kernel_elements = kel
                .iter()
                .map(|d| format!("{:?}", (d.u_coeffs(), d.v_coeffs())))
                .collect();
            rec.kernel_closed =
                c.add(&kel[0], &kel[1]) == kel[2] && c.add(&kel[1], &kel[2]) == kel[0];
            // isotropy: e₂(D_S, D_T) = (−1)^{|S ∩ T|} on Weierstrass-point supports (∞ counted); the three
            // supports {e_{k0}, ∞}, {e_{k1}, e_{k2}}, {e_{k3}, e_{k4}} are pairwise disjoint.
            rec.kernel_isotropic_derived = true;
            for d in &kel {
                rec.phi_zero_on_kernel += rc.phi(&c, d, &mut rng).is_identity() as usize;
            }
            for (_, t) in &two_torsion {
                if !kel.contains(t) {
                    rec.phi_nonzero_on_other_2_torsion +=
                        (!rc.phi(&c, t, &mut rng).is_identity()) as usize;
                }
            }
            // homomorphism, well-definedness
            for _ in 0..300 {
                let (x, y) = (c.random(&mut rng), c.random(&mut rng));
                let s = c.add(&x, &y);
                let (px, py, ps) = (
                    rc.phi(&c, &x, &mut rng),
                    rc.phi(&c, &y, &mut rng),
                    rc.phi(&c, &s, &mut rng),
                );
                rec.hom_checks += 1;
                rec.hom_failures += (rc.cp.add(&px, &py) != ps) as usize;
                let px2 = rc.phi(&c, &x, &mut rng);
                rec.welldef_checks += 1;
                rec.welldef_failures += (px != px2) as usize;
            }
            // direct degree-1 formula (c₀ = [R(∞) − 2∞̃] = 0 iff it agrees)
            for _ in 0..300 {
                let x = rng.gen_range(1..p);
                let l = c.divisors_with_u(&[(p - x) % p, 1]).unwrap();
                if l.len() != 2 {
                    continue;
                }
                let y = l[0].v_coeffs().first().copied().unwrap_or(0);
                if let Some(img) = rc.image_point(x, y) {
                    rec.degree1_direct_formula_checks += 1;
                    rec.degree1_direct_formula_agreements +=
                        (img == rc.phi(&c, &l[0], &mut rng)) as usize;
                }
            }
            // ψ on J: homomorphism, surjectivity, trivial on G, kernel density
            let mut vals: HashSet<(i8, i8)> = HashSet::new();
            for _ in 0..2000 {
                let (x, y) = (c.random(&mut rng), c.random(&mut rng));
                if let (Some(a), Some(b2), Some(s)) =
                    (rc.psi(&x), rc.psi(&y), rc.psi(&c.add(&x, &y)))
                {
                    rec.psi_hom_checks += 1;
                    rec.psi_hom_failures += ((a.0 * b2.0, a.1 * b2.1) != s) as usize;
                    vals.insert(a);
                }
                if let Some(g) = rc.psi(&c.mul(&x, h)) {
                    rec.psi_g_checks += 1;
                    rec.psi_trivial_on_g += (g == (1, 1)) as usize;
                }
            }
            rec.psi_values_seen = vals.len();
            let mut inker = 0usize;
            let mut n = 0usize;
            for _ in 0..20000 {
                if let Some(a) = rc.psi(&c.random(&mut rng)) {
                    n += 1;
                    inker += (a == (1, 1)) as usize;
                }
            }
            rec.psi_kernel_density = inker as f64 / n as f64;
            rec.psi_density_samples = n;
            // ψ′ on J′: trivial on φ(J), surjective; 2-descent on J′ for φ(H₄)
            let wroots: Vec<u64> = (0..p).filter(|&t| peval(&k, &rc.ftilde, t) == 0).collect();
            rec.jprime_rational_2_torsion = {
                let nw = wroots.len() + 1; // + ∞̃
                nw * (nw - 1) / 2
            };
            let mut pvals: HashSet<[i8; 3]> = HashSet::new();
            for _ in 0..2000 {
                let d = c.random(&mut rng);
                let img = rc.phi(&c, &d, &mut rng);
                if let Some(v) = rc.psi_prime(&img) {
                    rec.psi_prime_checks += 1;
                    rec.psi_prime_trivial_on_image += (v == [1, 1, 1]) as usize;
                }
                if let Some(v) = rc.psi_prime(&rc.cp.random(&mut rng)) {
                    pvals.insert(v);
                }
                if rc.psi(&d) == Some((1, 1)) {
                    if let Some(v) = rc.two_descent_prime(&img, &wroots) {
                        rec.phi_of_h4_checks += 1;
                        rec.phi_of_h4_in_2jprime += v.iter().all(|&x| x == 1) as usize;
                    }
                }
            }
            rec.psi_prime_values_seen = pvals.len();
            rec.image_nongeneric_retries = rc.nongeneric.get();
            rec.image_invalid_mumford = rc.invalid.get();
            eprintln!(
                "Richelot: partition {:?}, #J′ = {} (= #J: {}), rational Weierstrass points on C′ (sextic) {:?}, φ hom failures {}/{}, kernel zeros {}/3, other 2-torsion nonzero {}/{}, degree-1 formula agreements {}/{}, ψ density {:.4}, invalid images {}",
                rec.partition, rec.order_j_prime, rec.order_j_prime == order, rec.rational_weierstrass_points_sextic,
                rec.hom_failures, rec.hom_checks, rec.phi_zero_on_kernel, rec.phi_nonzero_on_other_2_torsion,
                two_torsion.len() - 3, rec.degree1_direct_formula_agreements, rec.degree1_direct_formula_checks,
                rec.psi_kernel_density, rec.image_invalid_mumford
            );
            assert_eq!(rec.hom_failures, 0, "φ is not a homomorphism");
            assert_eq!(rec.phi_zero_on_kernel, 3, "φ does not kill K");
            assert_eq!(rec.image_invalid_mumford, 0, "invalid Richelot images");
            rich_rec = Some(rec);
            richelot = Some(rc);
        }
    } else {
        rich_err =
            Some("f = x⁵ + b has no rational root (C1 family): no rational (2,2)-kernel".into());
    }

    // ── translation closure ──
    let mut points: Vec<Divisor> = Vec::new();
    for x in 0..p {
        points.extend(c.divisors_with_u(&[(p - x) % p, 1]).unwrap());
    }
    let mut ts: Vec<(String, Divisor)> = two_torsion.clone();
    if !f_splits && h == 5 {
        // rational 5-torsion T₅ = (0, √b) − ∞ and its multiples
        if let Some(s) = k.sqrt(b) {
            let t5 = c.point(0, s).unwrap();
            let mut m = t5;
            for i in 1..5 {
                ts.push((format!("T5[{i}]"), m));
                m = c.add(&m, &t5);
            }
        }
    }
    for i in 0..n_rand_t {
        loop {
            let t = c.random(&mut rng);
            if !t.is_identity() {
                ts.push((format!("rand{i}"), t));
                break;
            }
        }
    }
    let translation = translation_closure(&c, &points, &ts);
    eprintln!(
        "translation closure: {} affine points, {} translates, histogram {:?}, max {}",
        points.len(),
        ts.len(),
        translation.histogram,
        translation.max_intersection
    );

    // ── census on J and on J′ ──
    let cx = Ctx {
        c: &c,
        r,
        h,
        use_zeta: true,
    };
    let mut census_j: Vec<Census> = Vec::new();
    let mut census_jp: Vec<Census> = Vec::new();
    let in_g = |d: &Divisor| c.mul(d, r).is_identity();
    for rep in 0..reps {
        let mut bases: Vec<(String, Vec<Divisor>)> = Vec::new();
        if arms.iter().any(|a| a == "plain") {
            bases.push((
                "plain".into(),
                pick_points(&c, size, &mut rng, false, |_, _| true).expect("plain base"),
            ));
        }
        if arms.iter().any(|a| a == "A") {
            match a_closed(&c, size, &mut rng) {
                Some(bb) => bases.push(("A".into(), bb)),
                None => skipped.push(format!("A rep {rep}: not enough orbits")),
            }
        }
        if arms.iter().any(|a| a == "R") {
            if let Some(rc) = &richelot {
                let keep = |x: u64, _: &Divisor| {
                    k.legendre(peval(&k, &rc.g[0], x)) == 1
                        && k.legendre(peval(&k, &rc.g[1], x)) == 1
                };
                match pick_points(&c, size, &mut rng, false, keep) {
                    Some(bb) => {
                        for d in &bb {
                            assert_eq!(rc.psi(d), Some((1, 1)));
                        }
                        bases.push(("R".into(), bb));
                    }
                    None => skipped.push(format!("R rep {rep}: not enough points in H₄")),
                }
            } else if rep == 0 {
                skipped.push("R: no Richelot isogeny on this curve".into());
            }
        }
        if arms.iter().any(|a| a == "G") {
            match pick_points(&c, size, &mut rng, false, |_, d| in_g(d)) {
                Some(bb) => bases.push(("G".into(), bb)),
                None => {
                    if rep == 0 {
                        skipped.push("G: not enough degree-1 points in G".into());
                    }
                }
            }
        }
        let do_sweep = if rep < sweep3_bases { sweep3 } else { 0 };
        for (arm, base) in &bases {
            let mut sets = census(&cx, arm, rep, base);
            let (o2, o3) = oracle_sample(&k, &cx, base, &sets, targets, &mut rng, do_sweep);
            sets.census.oracle2 = Some(o2);
            sets.census.oracle3 = o3;
            census_j.push(sets.census);
        }
        // J′ arms
        if let Some(rc) = &richelot {
            let cxp = Ctx {
                c: &rc.cp,
                r,
                h,
                use_zeta: false,
            };
            let mut pbases: Vec<(String, Vec<Divisor>)> = Vec::new();
            if arms.iter().any(|a| a == "plainP") {
                match pick_points(&rc.cp, size, &mut rng, false, |_, _| true) {
                    Some(bb) => pbases.push(("plainP".into(), bb)),
                    None => skipped.push(format!("plainP rep {rep}: not enough points on C′")),
                }
            }
            for (arm, src) in [("phi_plain", "plain"), ("phi_A", "A")] {
                if arms.iter().any(|a| a == arm) {
                    if let Some((_, bb)) = bases.iter().find(|(a, _)| a == src) {
                        let img: Vec<Divisor> =
                            bb.iter().map(|d| rc.phi(&c, d, &mut rng)).collect();
                        let set: HashSet<Divisor> = img.iter().copied().collect();
                        if set.len() == img.len() {
                            pbases.push((arm.into(), img));
                        } else {
                            skipped.push(format!(
                                "{arm} rep {rep}: φ not injective on the base ({} of {})",
                                set.len(),
                                img.len()
                            ));
                        }
                    }
                }
            }
            for (arm, base) in &pbases {
                let sets = census(&cxp, arm, rep, base);
                census_jp.push(sets.census);
            }
        }
        if rep == 0 || (rep + 1) % 10 == 0 {
            eprintln!(
                "rep {}: {}",
                rep + 1,
                census_j
                    .iter()
                    .filter(|cc| cc.rep == rep)
                    .map(|cc| format!(
                        "{} U={} D2={} D3={} oracle2 dis={} ",
                        cc.arm,
                        cc.columns,
                        cc.m2.distinct,
                        cc.m3.distinct,
                        cc.oracle2.as_ref().map_or(0, |o| o.disagreements)
                    ))
                    .collect::<String>()
            );
        }
    }
    let summary_j = summarise(&census_j, "plain", h, seed);
    let summary_jp = summarise(&census_jp, "plainP", h, seed);
    for s in summary_j.iter().chain(summary_jp.iter()) {
        eprintln!(
            "{:9} vs {:6}: U {:.3} [{:.3},{:.3}] (pred {:.3}); gain |F|/U {:.2} [{:.2},{:.2}] (pred {}); c2s {:.3} [{:.3},{:.3}] (pred {}); c3 {:.3} [{:.3},{:.3}] (pred {}); cost3 nom {:.3} (pred {:.4}); oracle2 muls/call {:.1}, dis {}; sweep3 dis {}/{}",
            s.arm, s.reference, s.columns_ratio.value, s.columns_ratio.ci95.0, s.columns_ratio.ci95.1, s.predicted[0],
            s.unknowns_gain.value, s.unknowns_gain.ci95.0, s.unknowns_gain.ci95.1, s.predicted[5],
            s.c2_strict.value, s.c2_strict.ci95.0, s.c2_strict.ci95.1, s.predicted[1],
            s.c3.value, s.c3.ci95.0, s.c3.ci95.1, s.predicted[2],
            s.cost3_nominal.value, s.predicted[4], s.oracle2_muls_per_call.value, s.oracle2_disagreements,
            s.oracle3_disagreements, s.oracle3_targets
        );
    }
    let out = Out {
        lead: "R6.6",
        p,
        b,
        family,
        size,
        reps,
        seed,
        oracle_targets: targets,
        sweep3_targets: sweep3,
        sweep3_bases,
        arms,
        validation,
        richelot: rich_rec,
        richelot_error: rich_err,
        richelot_partitions_tried: tried,
        translation,
        summary_j,
        summary_jprime: summary_jp,
        census_j,
        census_jprime: census_jp,
        skipped,
        wall_seconds_contended: t0.elapsed().as_secs_f64(),
    };
    let s = serde_json::to_string_pretty(&out).unwrap();
    if json.is_empty() {
        println!("{s}");
    } else {
        fs::write(&json, s).unwrap();
        eprintln!(
            "wrote {json} ({:.1} s contended wall)",
            t0.elapsed().as_secs_f64()
        );
    }
}
