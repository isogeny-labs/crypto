//! Complete rational ℓ-isogeny graphs for ℓ = 7, 11, 13 over every F_p,
//! 5 <= p < 2^10 (the census-fp fields), computed by ecdb at import time.
//!
//! The census graphs for ℓ = 2, 3, 5 come from Sage records that ecdb
//! re-verifies (`generated.rs`). For ℓ = 7, 11, 13 ecdb finds every rational
//! ℓ-isogeny itself, from the Frobenius action on E[ℓ], and cross-checks the
//! result against the classical modular polynomial Φ_ℓ that ecdb already
//! ships (`enrich.rs`, PARI/GP `polmodular`).
//!
//! # Mathematics
//!
//! Let E: y^2 = x^3 + A x + B over F_p with trace t, and ℓ ≠ p an odd prime.
//! The F_p-rational cyclic subgroups C of order ℓ are the lines of E[ℓ] fixed
//! by Frobenius π, whose characteristic polynomial on E[ℓ] is
//! X^2 - t X + p mod ℓ. Hence:
//!
//! * no root mod ℓ (ℓ inert in Z[π]): no rational ℓ-isogeny;
//! * two roots λ ≠ λ': exactly two (the two eigenlines), `split`;
//! * a double root λ = t/2: π is either the scalar λ (all ℓ + 1 subgroups
//!   rational, `scalar`) or a Jordan block (one, `ramified` when ℓ ∤ v and
//!   `jordan` when ℓ | v, where t^2 - 4p = v^2 D_K; for odd ℓ, ℓ | v iff
//!   ℓ^2 | t^2 - 4p).
//!
//! **The eigenline of λ.** With f_n the x-only division polynomials
//! (ψ_n = f_n for n odd, 2y f_n for n even, F = x^3 + A x + B) and n = λ when
//! λ is odd, n = ℓ - λ (odd, and [n] = -[λ] on E[ℓ]) otherwise, for a point
//! P = (x, y) ≠ O of E[ℓ]:
//!
//! >  x([n]P) = x - 4F f_{n-1} f_{n+1} / f_n^2,
//! >  y([n]P) = y Ω_n / f_n^3,  Ω_n = f_{n+2} f_{n-1}^2 - f_{n-2} f_{n+1}^2,
//!
//! so πP = λP iff x^p f_n^2 = x f_n^2 - 4F f_{n-1} f_{n+1} and
//! F^((p-1)/2) f_n^3 = s Ω_n (s = 1 when n = λ, -1 when n = ℓ - λ) at x(P).
//! The kernel polynomial of the eigenline is therefore
//!
//! >  h_λ = gcd(f_ℓ, x^p f_n^2 - x f_n^2 + 4F f_{n-1} f_{n+1}, F^((p-1)/2) f_n^3 - s Ω_n),
//!
//! of degree (ℓ - 1)/2 for a line and (ℓ^2 - 1)/2 = deg f_ℓ when π = λ. The
//! y-condition separates λ from -λ (both eigenvalues when ℓ | t).
//!
//! **The scalar case.** Every subgroup is rational. The x-coordinates of
//! E[ℓ] \ {O} fall into Frobenius orbits of size d, the order of λ in
//! F_ℓ^* / {±1}, so f_ℓ is a product of irreducible factors of degree d,
//! split by Cantor–Zassenhaus equal-degree factorization. For one factor u
//! (a root X: x(P) for a point P of order ℓ), the kernel polynomial of <P> is
//! K(z) = ∏_{i=1}^{(ℓ-1)/2} (z - x([i]P)), computed in F_p[x]/(u); its
//! coefficients must be constants. K must divide what is left of f_ℓ, and
//! exactly ℓ + 1 kernels must exhaust it.
//!
//! **The codomain.** Kohel's formulas for a kernel polynomial of odd degree
//! d with power sums p_1, p_2, p_3 of its roots: E/C is
//! y^2 = x^3 + (A - 5T) x + (B - 7W), T = 6 p_2 + 2 A d,
//! W = 10 p_3 + 6 A p_1 + 4 B d. The stored target is the curve of the field
//! in its F_p-isomorphism class ((A, B) ~ (u^4 A, u^6 B)); the
//! post-isomorphism is (x, y) -> (w x, w^(3/2) y) with w = u^2, as for the
//! ℓ = 2, 3, 5 edges.
//!
//! # Checks
//!
//! Each runs before any write. A failure is a reported problem, and the whole
//! field is then left out for that ℓ (no edges, no torsion rows):
//! - the field: p prime, ℓ ≠ p, and the stored curves are exactly the
//!   F_p-isomorphism classes, 2p + 6, 2p + 2, 2p + 4 or 2p of them for
//!   p ≡ 1, 5, 7, 11 mod 12 (so every target is present);
//! - each curve: nonsingular, the stored trace equals the point count;
//! - each curve: every eigenvalue gives a kernel polynomial of the degree the
//!   theory allows (above); the scalar case only when ℓ^2 | t^2 - 4p;
//! - each edge: the codomain is nonsingular, its trace (counted) is t, it is
//!   isomorphic to a stored curve of the field whose stored trace is t, and
//!   its j-invariant is a root of Φ_ℓ(j(E), Y);
//! - each curve, against Φ_ℓ (Φ_ℓ(j(E), Y) = ∏_C (Y - j(E/C)) over all ℓ + 1
//!   subgroups, any j, so the j = 0, 1728 multiplicities are included): the
//!   multiset of target j-invariants is contained in the multiset of F_p-roots
//!   of Φ_ℓ(j(E), Y), and every simple F_p-root is a target (a simple root
//!   belongs to a Frobenius-stable subgroup). An F_p-root of multiplicity m
//!   hit k < m times is m - k non-rational subgroups permuted by Frobenius
//!   (resolved by the eigenvalue count); such curves are counted;
//! - the field: every edge E -> E' has a reverse edge E' -> E (the dual), and
//!   when neither j is 0 or 1728 the two multiplicities agree;
//! - against ecdb: stored ℓ-isogenies from these curves (ℓ = 7, 11, 13) must be
//!   among the computed edges (same source, target and kernel) and are not
//!   inserted twice; stored `curve_ell_torsion` rows must agree.
//!
//! # Writes
//!
//! `isogeny` rows (formula `velu`, kernel polynomial, Kohel codomain,
//! post-isomorphism, status `proved`, external id `velu:` + SHA-256 of
//! source slug, ℓ and kernel, the scheme of the ℓ = 2, 3, 5 census edges),
//! and one `curve_ell_torsion` row per curve and ℓ of every accepted field
//! (Frobenius action, number of rational ℓ-isogenies, status `proved`): this
//! row is the completeness mark. A stored row with a NULL count is filled in
//! place. Directions, volcano components and levels follow from the volcano
//! importer (`volcano.rs`, which now covers ℓ = 7, 11, 13). The dual is checked
//! but `dual_isogeny_id` stays NULL (which of several parallel edges is the
//! dual is not determined here).

use super::end_ring::{division_polys, inv, is_prime, monic, pmul, powmod, prem, pscale, psub, trim, xpow};
use super::{bulk, bulk1, Ctx, Report};
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

/// The primes ℓ covered here (ℓ = 2, 3, 5 come from the Sage census records).
pub const ELLS: [u64; 3] = [7, 11, 13];
/// Fields covered: 5 <= p < MAX_P (the census-fp fields).
pub const MAX_P: u64 = 1024;

const METHOD: &str = "rational l-isogenies from the Frobenius eigenlines on E[l] (kernel = gcd of f_l with the \
                      x- and y-conditions of pi P = lambda P; scalar case by equal-degree factorization), Kohel \
                      codomain; cross-checked against Phi_l(j, Y), traces, and duals";

const CERTIFICATE: &str = "ecdb: kernel polynomial of a Frobenius eigenline on E[l] (monic, degree (l-1)/2, \
                           divides f_l); Kohel codomain from its power sums, trace recounted equal; F_p-isomorphic \
                           to the target by (a w^2, b w^3), w a nonzero square; j(target) a root of Phi_l(j(E), Y); \
                           dual edge present";

type Poly = Vec<u64>;

// ------------------------------------------------------------- polynomials over F_p

/// Quotient and remainder of a by a monic b.
fn pdivrem(a: &[u64], b: &[u64], p: u64) -> (Poly, Poly) {
    let db = b.len() - 1;
    let mut r = trim(a.to_vec());
    if r.len() <= db {
        return (Vec::new(), r);
    }
    let mut q = vec![0u64; r.len() - db];
    while r.len() > db {
        let lead = r.pop().unwrap();
        let shift = r.len() - db;
        q[shift] = lead;
        if lead != 0 {
            for (i, &c) in b[..db].iter().enumerate() {
                r[shift + i] = (r[shift + i] + (p - lead) * c) % p;
            }
        }
    }
    (trim(q), trim(r))
}

/// Monic gcd (empty when both are zero).
fn pgcd(a: &[u64], b: &[u64], p: u64) -> Poly {
    let (mut a, mut b) = (trim(a.to_vec()), trim(b.to_vec()));
    while !b.is_empty() {
        let bm = monic(&b, p);
        let r = prem(&a, &bm, p);
        a = bm;
        b = r;
    }
    if a.is_empty() {
        a
    } else {
        monic(&a, p)
    }
}

/// base^e mod g (g monic).
fn ppow(base: &[u64], mut e: u64, g: &[u64], p: u64) -> Poly {
    let mut r = prem(&[1], g, p);
    let mut b = prem(base, g, p);
    while e > 0 {
        if e & 1 == 1 {
            r = prem(&pmul(&r, &b, p), g, p);
        }
        b = prem(&pmul(&b, &b, p), g, p);
        e >>= 1;
    }
    r
}

/// The inverse of a modulo u, if gcd(a, u) = 1.
fn pinv(a: &[u64], u: &[u64], p: u64) -> Option<Poly> {
    let (mut r0, mut r1) = (u.to_vec(), prem(a, u, p));
    let (mut s0, mut s1): (Poly, Poly) = (Vec::new(), vec![1]);
    while !r1.is_empty() {
        let li = inv(*r1.last().unwrap(), p);
        let (q, r) = pdivrem(&r0, &pscale(&r1, li, p), p);
        let s2 = psub(&s0, &pmul(&pscale(&q, li, p), &s1, p), p);
        (r0, r1, s0, s1) = (r1, r, s1, s2);
    }
    if r0.len() != 1 {
        return None;
    }
    Some(prem(&pscale(&s0, inv(r0[0], p), p), u, p))
}

/// Value of f at x.
fn peval(f: &[u64], x: u64, p: u64) -> u64 {
    f.iter().rev().fold(0, |acc, &c| (acc * x + c) % p)
}

/// The F_p-roots of f (nonzero) with their multiplicities.
pub fn roots_with_multiplicity(f: &[u64], p: u64) -> BTreeMap<u64, u32> {
    let mut out = BTreeMap::new();
    for x in 0..p {
        let mut g = trim(f.to_vec());
        let mut m = 0;
        while g.len() > 1 && peval(&g, x, p) == 0 {
            g = pdivrem(&g, &[(p - x) % p, 1], p).0;
            m += 1;
        }
        if m > 0 {
            out.insert(x, m);
        }
    }
    out
}

// ------------------------------------------------------------- curves

pub fn is_singular(p: u64, a: u64, b: u64) -> bool {
    (4 * powmod(a, 3, p) + 27 * (b % p) * (b % p)).is_multiple_of(p)
}

/// j = 1728 * 4A^3 / (4A^3 + 27B^2).
pub fn j_invariant(p: u64, a: u64, b: u64) -> u64 {
    let a3 = 4 * powmod(a, 3, p) % p;
    let d = (a3 + 27 * (b % p) * (b % p)) % p;
    1728 % p * a3 % p * inv(d, p) % p
}

/// Per-field tables: squares, and (u^4, u^6) for the isomorphism classes.
struct FieldTables {
    p: u64,
    is_sq: Vec<bool>,
    u46: Vec<(u64, u64, u64)>,
}

impl FieldTables {
    fn new(p: u64) -> FieldTables {
        let mut is_sq = vec![false; p as usize];
        for y in 1..p {
            is_sq[(y * y % p) as usize] = true;
        }
        let u46 = (1..p).map(|u| (u, powmod(u, 4, p), powmod(u, 6, p))).collect();
        FieldTables { p, is_sq, u46 }
    }

    /// t = -sum_x chi(x^3 + A x + B).
    fn trace(&self, a: u64, b: u64) -> i64 {
        let p = self.p;
        -(0..p)
            .map(|x| match (x * x % p * x + a * x + b) % p {
                0 => 0,
                r if self.is_sq[r as usize] => 1i64,
                _ => -1,
            })
            .sum::<i64>()
    }

    /// The F_p-isomorphism class: min over u of (u^4 A, u^6 B).
    fn iso_key(&self, a: u64, b: u64) -> (u64, u64) {
        let p = self.p;
        self.u46.iter().map(|&(_, u4, u6)| (a * u4 % p, b * u6 % p)).min().unwrap()
    }

    /// Some u with (u^4 A, u^6 B) = (A', B').
    fn iso_u(&self, from: (u64, u64), to: (u64, u64)) -> Option<u64> {
        let p = self.p;
        self.u46.iter().find(|&&(_, u4, u6)| (from.0 * u4 % p, from.1 * u6 % p) == to).map(|x| x.0)
    }
}

/// Number of F_p-isomorphism classes of elliptic curves over F_p, p >= 5.
pub fn class_count(p: u64) -> u64 {
    2 * p
        + match p % 12 {
            1 => 6,
            5 => 2,
            7 => 4,
            _ => 0,
        }
}

/// Kohel's codomain for an odd-degree kernel polynomial h (monic, ascending).
pub fn kohel_codomain(h: &[u64], a: u64, b: u64, p: u64) -> (u64, u64) {
    let d = h.len() - 1;
    let c = |i: isize| if i < 0 { 0 } else { h[i as usize] % p };
    let di = d as isize;
    let s1 = (p - c(di - 1)) % p;
    let s2 = c(di - 2);
    let s3 = (p - c(di - 3)) % p;
    let p1 = s1;
    let p2 = (s1 * s1 + 2 * (p - s2)) % p;
    let p3 = (s1 * s1 % p * s1 + 3 * (p - s1 * s2 % p) + 3 * s3) % p;
    let dd = d as u64 % p;
    let tt = (6 * p2 + 2 * a % p * dd) % p;
    let ww = (10 * p3 + 6 * a % p * p1 + 4 * b % p * dd) % p;
    ((a + 5 * (p - tt)) % p, (b + 7 * (p - ww)) % p)
}

/// Frobenius action on E[ℓ], in the vocabulary of `curve_ell_torsion.frobenius_action`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Inert,
    Split,
    Ramified,
    Jordan,
    Scalar,
}

impl Action {
    pub fn as_str(self) -> &'static str {
        match self {
            Action::Inert => "inert",
            Action::Split => "split",
            Action::Ramified => "ramified",
            Action::Jordan => "jordan",
            Action::Scalar => "scalar",
        }
    }
}

/// Deterministic xorshift for the equal-degree factorization.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

/// Cantor–Zassenhaus: f squarefree, every irreducible factor of degree d.
fn equal_degree(f: &[u64], d: usize, p: u64, rng: &mut Rng, out: &mut Vec<Poly>) -> Result<(), String> {
    let n = f.len() - 1;
    if n == d {
        out.push(f.to_vec());
        return Ok(());
    }
    if n == 0 || !n.is_multiple_of(d) {
        return Err(format!("a factor of degree {n} is not a product of degree-{d} factors"));
    }
    for _ in 0..400 {
        let r = trim((0..n).map(|_| rng.next() % p).collect());
        if r.len() < 2 {
            continue;
        }
        let (mut acc, mut s) = (r.clone(), r.clone());
        for _ in 1..d {
            s = ppow(&s, p, f, p);
            acc = prem(&pmul(&acc, &s, p), f, p);
        }
        let w = ppow(&acc, (p - 1) / 2, f, p);
        let g = pgcd(f, &psub(&w, &[1], p), p);
        if g.len() > 1 && g.len() < f.len() {
            let (q, rem) = pdivrem(f, &g, p);
            if !rem.is_empty() {
                return Err("gcd does not divide".into());
            }
            equal_degree(&g, d, p, rng, out)?;
            return equal_degree(&monic(&q, p), d, p, rng, out);
        }
    }
    Err(format!("equal-degree factorization of a degree-{n} polynomial did not split"))
}

/// x([i]P) - x as num/den: (-4F f_{i-1} f_{i+1}, f_i^2) for i odd, (-f_{i-1} f_{i+1}, 4F f_i^2) for i even.
fn mult_x(i: usize, fs: &[Poly], big_f: &[u64], p: u64) -> (Poly, Poly) {
    let prod = pmul(&fs[i - 1], &fs[i + 1], p);
    let four_f = pscale(big_f, 4, p);
    if i % 2 == 1 {
        (psub(&[], &pmul(&four_f, &prod, p), p), pmul(&fs[i], &fs[i], p))
    } else {
        (psub(&[], &prod, p), pmul(&four_f, &pmul(&fs[i], &fs[i], p), p))
    }
}

/// The kernel polynomial of <P>, x(P) a root of the irreducible u.
fn kernel_from_factor(u: &[u64], l: usize, fs: &[Poly], big_f: &[u64], p: u64) -> Result<Poly, String> {
    let x = prem(&[0, 1], u, p);
    let mut k: Vec<Poly> = vec![vec![1]]; // coefficients in F_p[x]/(u), ascending in z
    for i in 1..=(l - 1) / 2 {
        let xi = if i == 1 {
            x.clone()
        } else {
            let (num, den) = mult_x(i, fs, big_f, p);
            let di = pinv(&den, u, p).ok_or("x([i]P) has a vanishing denominator")?;
            prem(&psub(&x, &psub(&[], &pmul(&prem(&num, u, p), &di, p), p), p), u, p)
        };
        // k <- k * (z - xi)
        let mut next: Vec<Poly> = vec![Vec::new(); k.len() + 1];
        for (j, c) in k.iter().enumerate() {
            next[j + 1] = trim([next[j + 1].clone(), c.clone()].iter().fold(Vec::new(), |acc, v| {
                let n = acc.len().max(v.len());
                (0..n).map(|t| (acc.get(t).copied().unwrap_or(0) + v.get(t).copied().unwrap_or(0)) % p).collect()
            }));
            next[j] = psub(&next[j], &prem(&pmul(c, &xi, p), u, p), p);
        }
        k = next;
    }
    k.iter()
        .map(|c| match c.len() {
            0 => Ok(0),
            1 => Ok(c[0]),
            _ => Err("a kernel coefficient is not in F_p".to_string()),
        })
        .collect::<Result<Poly, String>>()
        .map(trim)
}

/// Every F_p-rational ℓ-isogeny kernel of y^2 = x^3 + A x + B over F_p with
/// trace t (ℓ odd, ℓ ≠ p), and the Frobenius action on E[ℓ].
pub fn rational_kernels(p: u64, a: u64, b: u64, t: i64, l: u64) -> Result<(Action, Vec<Poly>), String> {
    let (li, pi) = (l as i64, p as i64);
    let lam: Vec<u64> = (1..l).filter(|&x| (x as i64 * x as i64 - t * x as i64 + pi).rem_euclid(li) == 0).collect();
    let l_divides_v = (t * t - 4 * pi).rem_euclid(li * li) == 0;
    if lam.is_empty() {
        return Ok((Action::Inert, Vec::new()));
    }
    let lu = l as usize;
    let fs = division_polys(lu, a, b, p);
    let g = monic(&fs[lu], p);
    let full = (lu * lu - 1) / 2;
    let half = (lu - 1) / 2;
    if g.len() != full + 1 {
        return Err(format!("f_{l} has degree {} != {full}", g.len() - 1));
    }
    let big_f = trim(vec![b, a, 0, 1]);
    let xp = xpow(p, &g, p);
    let fh = ppow(&big_f, (p - 1) / 2, &g, p);
    let neg_one: Poly = vec![p - 1];
    let f_at = |i: isize| -> Poly {
        match i {
            -1 => neg_one.clone(),
            i => fs[i as usize].clone(),
        }
    };
    let mut kernels = Vec::new();
    let mut scalar = false;
    for &lambda in &lam {
        let (n, s) = if lambda % 2 == 1 { (lambda as isize, 1u64) } else { ((l - lambda) as isize, p - 1) };
        let (fm1, f0, fp1) = (f_at(n - 1), f_at(n), f_at(n + 1));
        let den = pmul(&f0, &f0, p);
        let num = pmul(&pscale(&big_f, 4, p), &pmul(&fm1, &fp1, p), p);
        let cond_x = prem(&psub(&pmul(&den, &xp, p), &psub(&pmul(&den, &[0, 1], p), &num, p), p), &g, p);
        let omega = psub(&pmul(&f_at(n + 2), &pmul(&fm1, &fm1, p), p), &pmul(&f_at(n - 2), &pmul(&fp1, &fp1, p), p), p);
        let cond_y = prem(&psub(&pmul(&pmul(&den, &f0, p), &fh, p), &pscale(&omega, s, p), p), &g, p);
        let h = pgcd(&pgcd(&g, &cond_x, p), &cond_y, p);
        let deg = h.len().saturating_sub(1);
        if deg == half {
            kernels.push(h);
        } else if deg == full && lam.len() == 1 {
            if !l_divides_v {
                return Err(format!("pi is the scalar {lambda} on E[{l}] but {l}^2 does not divide t^2 - 4p"));
            }
            scalar = true;
            // orbit size of x-coordinates: order of lambda in F_l^* / {±1}
            let d =
                (1..=lu).find(|&k| matches!(powmod(lambda, k as u64, l), 1) || powmod(lambda, k as u64, l) == l - 1);
            let d = d.ok_or("no order")?;
            let mut factors = Vec::new();
            let mut rng = Rng(0x9e37_79b9_7f4a_7c15 ^ (p << 32) ^ (a << 16) ^ b ^ (l << 48));
            equal_degree(&g, d, p, &mut rng, &mut factors)?;
            let mut rest = g.clone();
            for u in &factors {
                if !prem(&rest, u, p).is_empty() {
                    continue; // already in a kernel found from another factor
                }
                let k = kernel_from_factor(u, lu, &fs, &big_f, p)?;
                if k.len() != half + 1 || k[half] != 1 {
                    return Err("a scalar-case kernel is not monic of degree (l-1)/2".into());
                }
                let (q, r) = pdivrem(&rest, &k, p);
                if !r.is_empty() {
                    return Err("a scalar-case kernel does not divide the rest of f_l".into());
                }
                rest = q;
                kernels.push(k);
            }
            if rest != vec![1] || kernels.len() != lu + 1 {
                return Err(format!("the scalar case gave {} kernels, not {}", kernels.len(), l + 1));
            }
        } else {
            return Err(format!("the eigenvalue {lambda} gives a kernel of degree {deg}"));
        }
    }
    let action = match (lam.len(), scalar, l_divides_v) {
        (2, _, _) => Action::Split,
        (_, true, _) => Action::Scalar,
        (_, false, true) => Action::Jordan,
        _ => Action::Ramified,
    };
    if lam.len() == 2 && pgcd(&kernels[0], &kernels[1], p).len() > 1 {
        return Err("the two eigenlines share a point".into());
    }
    Ok((action, kernels))
}

// ------------------------------------------------------------- Φ_ℓ

/// Φ_ℓ mod p as a dense matrix c[i][j] (X^i Y^j).
pub fn phi_mod_p(terms: &super::enrich::Bivariate, l: u64, p: u64) -> Vec<Vec<u64>> {
    let n = l as usize + 2;
    let mut m = vec![vec![0u64; n]; n];
    let pb = num_bigint::BigInt::from(p);
    for (i, j, c) in terms {
        let r = num_integer::Integer::mod_floor(c, &pb);
        m[*i][*j] = num_traits::ToPrimitive::to_u64(&r).unwrap();
    }
    m
}

/// Φ_ℓ(j, Y), ascending in Y.
fn phi_at(phi: &[Vec<u64>], j: u64, p: u64) -> Poly {
    let n = phi.len();
    trim((0..n).map(|k| (0..n).rev().fold(0, |acc, i| (acc * j + phi[i][k]) % p)).collect())
}

// ------------------------------------------------------------- one field

#[derive(Clone, Debug)]
pub struct FieldCurve {
    pub a: u64,
    pub b: u64,
    pub t: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub source: usize,
    pub target: usize,
    pub kernel: Poly,
    pub codomain: (u64, u64),
    pub w: u64,
}

#[derive(Debug, Default)]
pub struct FieldGraph {
    /// Per curve: Frobenius action and number of rational ℓ-isogenies.
    pub curves: Vec<(Option<Action>, usize)>,
    pub edges: Vec<Edge>,
    /// Curves where Φ_ℓ(j, Y) has more F_p-roots (with multiplicity) than rational ℓ-isogenies.
    pub phi_excess_curves: u64,
    /// F_p-roots of Φ_ℓ(j, Y) with multiplicity, summed over the curves.
    pub phi_roots: u64,
}

/// The complete rational ℓ-isogeny graph of one field. `curves` must be the
/// F_p-isomorphism classes, one model each; every check of the module
/// documentation runs, and any failure is the error.
pub fn field_graph(p: u64, l: u64, curves: &[FieldCurve], phi: &[Vec<u64>]) -> Result<FieldGraph, String> {
    if !is_prime(p) || p < 5 || l == p || !is_prime(l) || l < 3 {
        return Err(format!("p = {p}, l = {l} is out of scope"));
    }
    let tab = FieldTables::new(p);
    let mut index: BTreeMap<(u64, u64), usize> = BTreeMap::new();
    for (i, c) in curves.iter().enumerate() {
        if c.a >= p || c.b >= p || is_singular(p, c.a, c.b) {
            return Err(format!("curve ({}, {}) is not a nonsingular model over F_{p}", c.a, c.b));
        }
        if tab.trace(c.a, c.b) != c.t {
            return Err(format!(
                "curve ({}, {}): stored trace {} != point count {}",
                c.a,
                c.b,
                c.t,
                tab.trace(c.a, c.b)
            ));
        }
        if index.insert(tab.iso_key(c.a, c.b), i).is_some() {
            return Err(format!("curve ({}, {}) duplicates an isomorphism class", c.a, c.b));
        }
    }
    if index.len() as u64 != class_count(p) {
        return Err(format!("{} isomorphism classes stored over F_{p}, {} exist", index.len(), class_count(p)));
    }
    let mut out = FieldGraph::default();
    for (i, c) in curves.iter().enumerate() {
        let who = || format!("curve ({}, {}) t = {}", c.a, c.b, c.t);
        let (action, kernels) = rational_kernels(p, c.a, c.b, c.t, l).map_err(|e| format!("{}: {e}", who()))?;
        let j = j_invariant(p, c.a, c.b);
        let roots = roots_with_multiplicity(&phi_at(phi, j, p), p);
        let mut hits: BTreeMap<u64, u32> = BTreeMap::new();
        for k in kernels {
            let cod = kohel_codomain(&k, c.a, c.b, p);
            if is_singular(p, cod.0, cod.1) {
                return Err(format!("{}: singular codomain {cod:?}", who()));
            }
            if tab.trace(cod.0, cod.1) != c.t {
                return Err(format!("{}: codomain {cod:?} has trace {}", who(), tab.trace(cod.0, cod.1)));
            }
            let &target = index
                .get(&tab.iso_key(cod.0, cod.1))
                .ok_or_else(|| format!("{}: codomain {cod:?} is not a stored curve", who()))?;
            let tc = &curves[target];
            if tc.t != c.t {
                return Err(format!("{}: target has stored trace {}", who(), tc.t));
            }
            let u = tab.iso_u(cod, (tc.a, tc.b)).ok_or_else(|| format!("{}: no isomorphism onto the target", who()))?;
            let jt = j_invariant(p, cod.0, cod.1);
            *hits.entry(jt).or_default() += 1;
            out.edges.push(Edge { source: i, target, kernel: k, codomain: cod, w: u * u % p });
        }
        let n: u32 = hits.values().sum();
        for (jt, k) in &hits {
            let m = roots.get(jt).copied().unwrap_or(0);
            if *k > m {
                return Err(format!(
                    "{}: {k} isogenies reach j = {jt}, a root of Phi_{l}(j, Y) of multiplicity {m}",
                    who()
                ));
            }
        }
        for (r, m) in &roots {
            if *m == 1 && !hits.contains_key(r) {
                return Err(format!("{}: the simple root {r} of Phi_{l}(j, Y) has no rational isogeny", who()));
            }
        }
        let nroots: u32 = roots.values().sum();
        out.phi_roots += nroots as u64;
        if nroots > n {
            out.phi_excess_curves += 1;
        }
        out.curves.push((Some(action), n as usize));
    }
    // duals
    let mut mult: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    for e in &out.edges {
        *mult.entry((e.source, e.target)).or_default() += 1;
    }
    let special = |i: usize| {
        let j = j_invariant(p, curves[i].a, curves[i].b);
        j == 0 || j == 1728 % p
    };
    for (&(s, t), &m) in &mult {
        let back = mult.get(&(t, s)).copied().unwrap_or(0);
        if back == 0 || (!special(s) && !special(t) && back != m) {
            return Err(format!(
                "edges ({}, {}) -> ({}, {}): {m} forward, {back} back (dual missing)",
                curves[s].a, curves[s].b, curves[t].a, curves[t].b
            ));
        }
    }
    Ok(out)
}

// ------------------------------------------------------------- import

const CURVES_SQL: &str = "
SELECT c.id, coalesce(c.family, ''), c.icv1_slug, ic.p::int8, c.coefficients->>'a', c.coefficients->>'b',
       fg.trace::int8,
       (SELECT coalesce(jsonb_agg(jsonb_build_array(t.ell::int8, t.rational_isogenies, t.frobenius_action)), '[]'::jsonb)
          FROM ec.curve_ell_torsion t WHERE t.curve_id = c.id AND t.ell IN (7, 11, 13))
FROM ec.curve c
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
JOIN ec.frobenius_genus1 fg ON fg.isogeny_class_id = ic.id
WHERE c.genus = 1 AND c.model = 'short_weierstrass' AND ic.g = 1 AND ic.q = ic.p AND ic.p >= 5 AND ic.p < $1::int8
ORDER BY ic.p, c.id";

const EXISTING_SQL: &str = "
SELECT i.source_curve_id, i.target_curve_id, i.ell::int8, i.kernel_polynomial
FROM ec.isogeny i
JOIN ec.abelian_variety av ON av.curve_id = i.source_curve_id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
WHERE ic.q = ic.p AND ic.p < $1::int8 AND i.dimension = 1 AND i.ell IN (7, 11, 13)";

const ISOGENY_SQL: &str = "
INSERT INTO ec.isogeny (external_id, source_av_id, target_av_id, source_curve_id, target_curve_id, dimension, degree,
       ell, kernel_structure, is_separable, field_of_definition_id, formula, kernel_polynomial, codomain_raw,
       post_isomorphism, is_normalized, certificate, status, provenance_id)
SELECT r.id, sa.id, ta.id, sc.id, tc.id, 1, r.ell, r.ell, ARRAY[r.ell]::numeric[], true, sc.field_id, 'velu',
       r.kernel, r.codomain, r.post, true, r.certificate, 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(id text, source bigint, target bigint, ell numeric, kernel jsonb,
     codomain jsonb, post jsonb, certificate text)
JOIN ec.curve sc ON sc.id = r.source
JOIN ec.curve tc ON tc.id = r.target
JOIN ec.abelian_variety sa ON sa.curve_id = sc.id
JOIN ec.abelian_variety ta ON ta.curve_id = tc.id
ON CONFLICT DO NOTHING";

const TORSION_SQL: &str = "
INSERT INTO ec.curve_ell_torsion (curve_id, ell, frobenius_action, rational_isogenies, status, provenance_id)
SELECT r.curve_id, r.ell, r.action, r.n, 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, ell numeric, action text, n int)
ON CONFLICT (curve_id, ell) DO NOTHING";

/// Fill a stored row whose count is unknown; a stored action must already agree.
const TORSION_FILL_SQL: &str = "
UPDATE ec.curve_ell_torsion t SET rational_isogenies = r.n, frobenius_action = coalesce(t.frobenius_action, r.action)
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, ell numeric, action text, n int)
WHERE t.curve_id = r.curve_id AND t.ell = r.ell AND t.rational_isogenies IS NULL
  AND (t.frobenius_action IS NULL OR t.frobenius_action = r.action)";

/// A stored ℓ-isogeny of a source curve: (target curve id, kernel polynomial if recorded).
type StoredEdge = (i64, Option<Vec<u64>>);

struct Row {
    id: i64,
    census: bool,
    slug: Option<String>,
    a: u64,
    b: u64,
    t: i64,
    /// stored (ℓ, rational isogenies, action)
    torsion: Vec<(u64, Option<i32>, Option<String>)>,
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let mut report = Report {
        input: format!(
            "short-Weierstrass curves over F_p, 5 <= p < {MAX_P} (stored ec.curve); Phi_l from {}",
            super::enrich::MODULAR_FILE
        ),
        ..Default::default()
    };
    let client = ctx.client;
    let text = super::read_text(&ctx.repo_root.join(super::enrich::MODULAR_FILE))?;
    let phis = super::enrich::modular_polynomials(&text)?;
    let src = ctx
        .source(
            "ecdb/src/import/ell_isogenies.rs",
            &crate::math::sha256_hex(include_str!("ell_isogenies.rs")),
            "Complete rational l-isogeny graphs (l = 7, 11, 13) over F_p, p < 2^10, computed by ecdb",
        )
        .await?;
    let params = json!({ "modular_polynomials": super::enrich::MODULAR_FILE,
                         "modular_polynomials_sha256": crate::math::sha256_hex(&text) });
    let prov = ctx.provenance(src, METHOD, Some(&params)).await?;

    let max_p = MAX_P as i64;
    let mut fields: BTreeMap<u64, Vec<Row>> = BTreeMap::new();
    for r in client.query(CURVES_SQL, &[&max_p]).await? {
        let p = r.get::<_, i64>(3) as u64;
        let coeff = |i: usize| r.get::<_, Option<String>>(i).and_then(|s| s.parse::<u64>().ok());
        let (Some(a), Some(b)) = (coeff(4), coeff(5)) else {
            report.count("ell_isogeny_curves_refused_input", 1);
            report.problem(format!("curve {}: coefficients a, b are not field elements", r.get::<_, i64>(0)));
            continue;
        };
        let tors: Value = r.get(7);
        let torsion = tors
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|x| Some((x[0].as_u64()?, x[1].as_i64().map(|n| n as i32), x[2].as_str().map(String::from))))
            .collect();
        fields.entry(p).or_default().push(Row {
            id: r.get(0),
            census: r.get::<_, String>(1) == "census-fp",
            slug: r.get(2),
            a: a % p,
            b: b % p,
            t: r.get(6),
            torsion,
        });
    }
    let mut existing: BTreeMap<(i64, u64), Vec<StoredEdge>> = BTreeMap::new();
    for r in client.query(EXISTING_SQL, &[&max_p]).await? {
        let kernel: Option<Value> = r.get(3);
        let kernel = kernel.and_then(|k| {
            k.as_array().map(|a| a.iter().filter_map(|c| super::s(c).and_then(|s| s.parse().ok())).collect())
        });
        existing.entry((r.get(0), r.get::<_, i64>(2) as u64)).or_default().push((r.get(1), kernel));
    }

    let (mut isogenies, mut torsion, mut fills) = (Vec::new(), Vec::new(), Vec::new());
    for (&p, rows) in &fields {
        // one model per isomorphism class: the census-fp curve, else the lowest id
        let tab = FieldTables::new(p);
        let mut reps: BTreeMap<(u64, u64), usize> = BTreeMap::new();
        for (i, r) in rows.iter().enumerate() {
            if is_singular(p, r.a, r.b) {
                // not an elliptic curve: it gets nothing; the field is still complete when every class
                // has another model (field_graph counts the classes)
                report.count("ell_isogeny_curves_refused_singular", 1);
                report.problem(format!("curve {}: y^2 = x^3 + {} x + {} is singular over F_{p}", r.id, r.a, r.b));
                continue;
            }
            let key = tab.iso_key(r.a, r.b);
            match reps.get(&key) {
                Some(&j) if rows[j].census || !r.census => report.count("ell_isogeny_duplicate_models_skipped", 1),
                Some(_) => {
                    report.count("ell_isogeny_duplicate_models_skipped", 1);
                    reps.insert(key, i);
                }
                None => {
                    reps.insert(key, i);
                }
            }
        }
        let chosen: Vec<&Row> = reps.values().map(|&i| &rows[i]).collect();
        let curves: Vec<FieldCurve> = chosen.iter().map(|r| FieldCurve { a: r.a, b: r.b, t: r.t }).collect();
        for l in ELLS {
            if l == p {
                report.count("ell_isogeny_fields_skipped_l_equals_p", 1);
                continue;
            }
            let Some(terms) = phis.get(&(l as u32)) else {
                report.problem(format!("no Phi_{l} in {}", super::enrich::MODULAR_FILE));
                continue;
            };
            let phi = phi_mod_p(terms, l, p);
            let g = match field_graph(p, l, &curves, &phi) {
                Ok(g) => g,
                Err(why) => {
                    report.count(&format!("ell_isogeny_fields_refused_l{l}"), 1);
                    report.problem(format!("F_{p}, l = {l}: {why}"));
                    continue;
                }
            };
            // agreement with what ecdb already holds
            let plan = match reconcile(l, &chosen, &existing, &g) {
                Ok(plan) => plan,
                Err(why) => {
                    report.count(&format!("ell_isogeny_fields_refused_l{l}"), 1);
                    report.problem(format!("F_{p}, l = {l}: {why}"));
                    continue;
                }
            };
            let skip = plan.skip;
            report.count(&format!("ell_isogeny_fields_ok_l{l}"), 1);
            report.count(&format!("ell_isogeny_curves_complete_l{l}"), chosen.len() as u64);
            report.count(&format!("ell_isogeny_edges_computed_l{l}"), g.edges.len() as u64);
            report.count(&format!("ell_isogeny_edges_already_stored_l{l}"), skip.len() as u64);
            report.count(&format!("ell_isogeny_curves_phi_excess_roots_l{l}"), g.phi_excess_curves);
            report.count(&format!("ell_isogeny_phi_roots_l{l}"), g.phi_roots);
            report.count("curve_ell_torsion_existing_agree", plan.agree as u64);
            torsion.extend(plan.inserts);
            fills.extend(plan.fills);
            for (k, e) in g.edges.iter().enumerate() {
                if skip.contains(&k) {
                    continue;
                }
                let (s, t) = (chosen[e.source], chosen[e.target]);
                let Some(slug) = &s.slug else {
                    report.problem(format!("curve {} has no ICV1 slug: its edges get no external id", s.id));
                    continue;
                };
                let ksig: Vec<String> = e.kernel.iter().map(|c| c.to_string()).collect();
                let id = format!("velu:{}", &crate::math::sha256_hex(&format!("{slug}|{l}|{}", ksig.join(",")))[..20]);
                isogenies.push(json!({
                    "id": id, "source": s.id, "target": t.id, "ell": l, "kernel": ksig,
                    "codomain": { "a4": e.codomain.0.to_string(), "a6": e.codomain.1.to_string() },
                    "post": { "w": e.w.to_string(), "map": "(x, y) -> (w x, w^(3/2) y), w a square" },
                    "certificate": CERTIFICATE,
                }));
            }
        }
    }
    // Every row sent was checked against what ecdb held when it was read; a row the database still
    // turns away (ON CONFLICT DO NOTHING, or the guard of TORSION_FILL_SQL) is a conflict that
    // appeared since, or an external id taken by a different row. Nothing is overwritten, but say so.
    let inserted = bulk(client, ISOGENY_SQL, &isogenies, prov).await?;
    report.count("isogenies_ell_7_11_13", inserted);
    shortfall(&mut report, "isogeny rows (external id already taken by another row)", isogenies.len(), inserted);
    let inserted = bulk(client, TORSION_SQL, &torsion, prov).await?;
    report.count("curve_ell_torsion_inserted", inserted);
    shortfall(&mut report, "curve_ell_torsion rows (a row for that curve and l appeared)", torsion.len(), inserted);
    let filled = bulk1(client, TORSION_FILL_SQL, &fills).await?;
    report.count("curve_ell_torsion_filled_null_count", filled);
    shortfall(&mut report, "curve_ell_torsion NULL-count fills (the stored row changed)", fills.len(), filled);
    Ok(report)
}

/// Report rows the database refused to write although they passed every check against what was read.
fn shortfall(report: &mut Report, what: &str, sent: usize, written: u64) {
    if let Some(msg) = shortfall_problem(what, sent, written) {
        report.count("ell_isogeny_rows_refused_by_database", sent as u64 - written);
        report.problem(msg);
    }
}

fn shortfall_problem(what: &str, sent: usize, written: u64) -> Option<String> {
    (written < sent as u64)
        .then(|| format!("{} of {sent} {what} were not written: nothing was overwritten", sent as u64 - written))
}

/// What one accepted field writes, after comparing the computed graph with what ecdb holds.
#[derive(Debug, Default)]
struct Plan {
    /// Computed edges (indices into `FieldGraph::edges`) already stored: not inserted again.
    skip: BTreeSet<usize>,
    /// New `curve_ell_torsion` rows.
    inserts: Vec<Value>,
    /// Stored rows with a NULL count, to fill.
    fills: Vec<Value>,
    /// Stored rows whose count and action already agree.
    agree: usize,
}

/// Compare the computed graph of one field with the stored ℓ-isogenies and `curve_ell_torsion`
/// rows of its curves. Any disagreement refuses the whole field (the error); nothing stored is
/// ever replaced.
fn reconcile(
    l: u64,
    chosen: &[&Row],
    existing: &BTreeMap<(i64, u64), Vec<StoredEdge>>,
    g: &FieldGraph,
) -> Result<Plan, String> {
    let mut plan = Plan::default();
    for (ci, r) in chosen.iter().enumerate() {
        for (tgt, kernel) in existing.get(&(r.id, l)).into_iter().flatten() {
            let found = g.edges.iter().enumerate().position(|(k, e)| {
                !plan.skip.contains(&k)
                    && e.source == ci
                    && chosen[e.target].id == *tgt
                    && kernel.as_ref().is_none_or(|k| *k == e.kernel)
            });
            match found {
                Some(k) => {
                    plan.skip.insert(k);
                }
                None => return Err(format!("stored l = {l} isogeny {} -> {tgt} is not a computed edge", r.id)),
            }
        }
        let (action, n) = g.curves[ci];
        let action = action.map(Action::as_str);
        let row = json!({ "curve_id": r.id, "ell": l, "action": action, "n": n });
        match r.torsion.iter().find(|x| x.0 == l) {
            None => plan.inserts.push(row),
            Some((_, tn, ta)) => {
                // A stored `jordan` (the DiSSECT and ell_torsion.rs label for any Jordan block) also
                // covers this importer's finer `ramified` (a Jordan block with l not dividing v).
                let action_conflict =
                    ta.as_deref().is_some_and(|x| Some(x) != action && !(x == "jordan" && action == Some("ramified")));
                if tn.is_some_and(|x| x < 0 || x as usize != n) || action_conflict {
                    return Err(format!(
                        "curve {}: stored curve_ell_torsion ({tn:?}, {ta:?}) at l = {l}, ecdb computes ({n}, {action:?})",
                        r.id
                    ));
                }
                if tn.is_none() {
                    plan.fills.push(row);
                } else {
                    plan.agree += 1;
                }
            }
        }
    }
    Ok(plan)
}

#[cfg(test)]
mod tests {
    use super::*;

    type Pt = Option<(u64, u64)>;

    fn add(p: u64, a: u64, s: Pt, t: Pt) -> Pt {
        let (Some((x1, y1)), Some((x2, y2))) = (s, t) else { return s.or(t) };
        let lam = if x1 == x2 {
            if (y1 + y2) % p == 0 {
                return None;
            }
            (3 * x1 * x1 + a) % p * inv(2 * y1 % p, p) % p
        } else {
            (y2 + p - y1) % p * inv((x2 + p - x1) % p, p) % p
        };
        let x3 = (lam * lam % p + 2 * p - x1 - x2) % p;
        let y3 = (lam * ((x1 + p - x3) % p) % p + p - y1) % p;
        Some((x3, y3))
    }

    fn points(p: u64, a: u64, b: u64) -> Vec<Pt> {
        let mut out = vec![None];
        for x in 0..p {
            let r = (x * x % p * x + a * x + b) % p;
            for y in 0..p {
                if y * y % p == r {
                    out.push(Some((x, y)));
                }
            }
        }
        out
    }

    /// One model per F_p-isomorphism class, with its trace.
    fn census(p: u64) -> Vec<FieldCurve> {
        let tab = FieldTables::new(p);
        let mut seen = BTreeSet::new();
        let mut out = Vec::new();
        for a in 0..p {
            for b in 0..p {
                if !is_singular(p, a, b) && seen.insert(tab.iso_key(a, b)) {
                    out.push(FieldCurve { a, b, t: tab.trace(a, b) });
                }
            }
        }
        out
    }

    fn phi(l: u64, p: u64) -> Vec<Vec<u64>> {
        let text = include_str!("../../../src/cryptanalysis/curve_traits/modular_polynomials.txt");
        let phis = crate::import::enrich::modular_polynomials(text).unwrap();
        phi_mod_p(&phis[&(l as u32)], l, p)
    }

    /// Kernel polynomials of the F_p-rational subgroups of order l of E(F_p), by brute force.
    fn brute_rational_point_kernels(p: u64, a: u64, b: u64, l: u64) -> BTreeSet<Poly> {
        let pts = points(p, a, b);
        let mut out = BTreeSet::new();
        for &pt in &pts[1..] {
            let mut q = pt;
            let mut xs = Vec::new();
            let mut order = 1;
            while q.is_some() {
                xs.push(q.unwrap().0);
                q = add(p, a, q, pt);
                order += 1;
                if order > l {
                    break;
                }
            }
            if q.is_none() && order == l {
                let mut k: Poly = vec![1];
                let mut xs: Vec<u64> = xs.into_iter().collect::<BTreeSet<_>>().into_iter().collect();
                xs.sort();
                for x in xs {
                    k = pmul(&k, &[(p - x) % p, 1], p);
                }
                out.insert(k);
            }
        }
        out
    }

    #[test]
    fn class_counts() {
        for p in [5u64, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
            assert_eq!(census(p).len() as u64, class_count(p), "p = {p}");
        }
    }

    #[test]
    fn kohel_matches_the_census_codomain_code() {
        // ecdb's l = 2, 3, 5 importer (generated.rs, pinned against Sage) evaluates the same odd-degree
        // Kohel formula; it must agree on every l = 7, 11, 13 kernel of the F_101 census
        use num_bigint::BigInt;
        let p = 101u64;
        let mut n = 0;
        for c in census(p) {
            for l in ELLS {
                let (_, ks) = rational_kernels(p, c.a, c.b, c.t, l).unwrap();
                for k in ks {
                    let mine = kohel_codomain(&k, c.a, c.b, p);
                    let big: Vec<BigInt> = k.iter().map(|&x| BigInt::from(x)).collect();
                    let theirs = crate::import::generated::velu_codomain(
                        l,
                        &BigInt::from(c.a),
                        &BigInt::from(c.b),
                        &big,
                        &BigInt::from(p),
                    );
                    assert_eq!((BigInt::from(mine.0), BigInt::from(mine.1)), theirs);
                    n += 1;
                }
            }
        }
        assert!(n > 100, "only {n} kernels compared");
    }

    /// Brute-force kernels of eigenvalue ±1: subgroups of order l of E(F_p) (λ = 1) and of the
    /// quadratic twist E^d: y^2 = x^3 + A d^2 x + B d^3 (λ = -1), pulled back by x -> x / d.
    fn brute_pm1_kernels(p: u64, a: u64, b: u64, l: u64) -> BTreeSet<Poly> {
        let mut out = brute_rational_point_kernels(p, a, b, l);
        let d = (2..p).find(|&d| powmod(d, (p - 1) / 2, p) == p - 1).unwrap();
        let di = inv(d, p);
        for k in brute_rational_point_kernels(p, a * d % p * d % p, b * powmod(d, 3, p) % p, l) {
            // roots x' of k become x = x'/d: k(d x) / d^deg, monic
            let deg = k.len() - 1;
            let pulled: Poly = k.iter().enumerate().map(|(i, &c)| c * powmod(d, i as u64, p) % p).collect();
            out.insert(pscale(&pulled, powmod(di, deg as u64, p), p));
        }
        out
    }

    #[test]
    fn rational_point_kernels_agree_with_brute_force() {
        // the kernels of eigenvalue ±1 are exactly the subgroups of order l of E(F_p) and of its twist
        let mut compared = 0;
        for &(p, l) in &[(29u64, 7u64), (43, 7), (71, 7), (113, 7), (23, 11), (67, 11), (89, 11), (53, 13), (79, 13)] {
            for c in census(p) {
                let brute = brute_pm1_kernels(p, c.a, c.b, l);
                let (_, ks) = rational_kernels(p, c.a, c.b, c.t, l).unwrap();
                let lam: Vec<i64> =
                    (1..l as i64).filter(|&x| (x * x - c.t * x + p as i64).rem_euclid(l as i64) == 0).collect();
                let mine: BTreeSet<Poly> = ks.iter().cloned().collect();
                assert_eq!(mine.len(), ks.len(), "repeated kernel");
                assert!(brute.is_subset(&mine), "p = {p}, l = {l}, ({}, {})", c.a, c.b);
                let pm1 = lam.iter().any(|&x| x == 1 || x == l as i64 - 1);
                if pm1 {
                    // when λ = ±1 is an eigenvalue, its eigenline (or all of E[l]) is in the brute-force set
                    assert!(!brute.is_empty(), "p = {p}, l = {l}, ({}, {}): eigenvalue ±1 but no subgroup", c.a, c.b);
                    if lam.iter().all(|&x| x == 1 || x == l as i64 - 1) {
                        assert_eq!(brute, mine, "p = {p}, l = {l}, ({}, {})", c.a, c.b);
                    }
                    compared += 1;
                } else {
                    assert!(brute.is_empty(), "p = {p}, l = {l}, ({}, {}): subgroup without eigenvalue ±1", c.a, c.b);
                }
            }
        }
        assert!(compared > 50, "only {compared} curves had an eigenvalue ±1");
    }

    #[test]
    fn scalar_case_has_all_l_plus_1_kernels() {
        // E[7] ⊂ E(F_p) (p = 1 mod 7, pi = 1 on E[7]): the eight kernels are the eight subgroups of E(F_p)[7]
        let mut found = 0;
        for p in [29u64, 43, 71, 113, 127, 197, 211, 239, 281] {
            for c in census(p) {
                let n = (p as i64 + 1 - c.t) as u64;
                if n.is_multiple_of(49) && brute_rational_point_kernels(p, c.a, c.b, 7).len() == 8 {
                    let (action, ks) = rational_kernels(p, c.a, c.b, c.t, 7).unwrap();
                    assert_eq!(action, Action::Scalar);
                    assert_eq!(ks.into_iter().collect::<BTreeSet<_>>(), brute_rational_point_kernels(p, c.a, c.b, 7));
                    found += 1;
                }
            }
        }
        assert!(found > 0, "no E[7] ⊂ E(F_p) curve found");
    }

    #[test]
    fn complete_graphs_on_small_fields() {
        // every field 5 <= p < 140 for each l: all checks pass, and the count matches the Frobenius action
        let mut scalar_seen = 0;
        for p in (5u64..140).filter(|&p| is_prime(p)) {
            let curves = census(p);
            for l in ELLS {
                if l == p {
                    continue;
                }
                let g = field_graph(p, l, &curves, &phi(l, p)).unwrap_or_else(|e| panic!("p = {p}, l = {l}: {e}"));
                for (c, (action, n)) in curves.iter().zip(&g.curves) {
                    let want = match action.unwrap() {
                        Action::Inert => 0,
                        Action::Split => 2,
                        Action::Ramified | Action::Jordan => 1,
                        Action::Scalar => {
                            scalar_seen += 1;
                            l as usize + 1
                        }
                    };
                    assert_eq!(*n, want, "p = {p}, l = {l}, ({}, {})", c.a, c.b);
                }
            }
        }
        assert!(scalar_seen > 0, "no scalar case exercised");
    }

    /// LMFDB 26.b: 26.b1 [1,-1,1,-3,3] and 26.b2 [1,-1,1,-213,-1257] are 7-isogenous over Q.
    fn short_model(ai: [i64; 5], p: u64) -> (u64, u64) {
        let [a1, a2, a3, a4, a6] = ai;
        let b2 = a1 * a1 + 4 * a2;
        let b4 = 2 * a4 + a1 * a3;
        let b6 = a3 * a3 + 4 * a6;
        let c4 = b2 * b2 - 24 * b4;
        let c6 = -b2 * b2 * b2 + 36 * b2 * b4 - 216 * b6;
        let m = |v: i64| v.rem_euclid(p as i64) as u64;
        (m(-27 * c4), m(-54 * c6))
    }

    #[test]
    fn pinned_lmfdb_26b_7_isogeny() {
        let tab_check = |p: u64| {
            let (a, b) = short_model([1, -1, 1, -3, 3], p);
            let (a2, b2) = short_model([1, -1, 1, -213, -1257], p);
            let tab = FieldTables::new(p);
            let t = tab.trace(a, b);
            assert_eq!(tab.trace(a2, b2), t, "isogenous curves share the trace (p = {p})");
            let (_, ks) = rational_kernels(p, a, b, t, 7).unwrap();
            let target = tab.iso_key(a2, b2);
            assert!(
                ks.iter().any(|k| {
                    let c = kohel_codomain(k, a, b, p);
                    tab.iso_key(c.0, c.1) == target
                }),
                "p = {p}: no 7-isogeny 26.b1 -> 26.b2"
            );
            // and the j-invariants satisfy Phi_7
            let j1 = j_invariant(p, a, b);
            let j2 = j_invariant(p, a2, b2);
            assert_eq!(peval(&phi_at(&phi(7, p), j1, p), j2, p), 0);
        };
        for p in [29u64, 31, 37, 41, 43, 101, 211, 1009] {
            tab_check(p);
        }
    }

    #[test]
    fn corrupted_inputs_are_refused() {
        let p = 101u64;
        let mut curves = census(p);
        // a wrong stored trace
        let mut bad = curves.clone();
        bad[3].t += 1;
        assert!(field_graph(p, 7, &bad, &phi(7, p)).unwrap_err().contains("stored trace"));
        // a missing isomorphism class
        let mut short = curves.clone();
        short.pop();
        assert!(field_graph(p, 7, &short, &phi(7, p)).unwrap_err().contains("isomorphism classes"));
        // a corrupted modular polynomial: Phi_7 with one coefficient changed
        let mut phi_bad = phi(7, p);
        phi_bad[0][1] = (phi_bad[0][1] + 1) % p;
        phi_bad[1][0] = (phi_bad[1][0] + 1) % p;
        assert!(field_graph(p, 7, &curves, &phi_bad).is_err());
        // the wrong l's polynomial
        assert!(field_graph(p, 7, &curves, &phi(11, p)).is_err());
        // a swapped trace sign on one curve (its twist's trace): both a trace and a graph problem
        let k = curves.iter().position(|c| c.t != 0).unwrap();
        curves[k].t = -curves[k].t;
        assert!(field_graph(p, 7, &curves, &phi(7, p)).is_err());
    }

    /// The stored-row comparison: agreeing rows are kept (not inserted twice, not rewritten), NULL counts
    /// are filled, and any conflicting stored edge or curve_ell_torsion value refuses the field.
    #[test]
    fn reconcile_refuses_conflicts_and_never_rewrites() {
        let (p, l) = (43u64, 7u64);
        let curves = census(p);
        let g = field_graph(p, l, &curves, &phi(l, p)).unwrap();
        let rows: Vec<Row> = curves
            .iter()
            .enumerate()
            .map(|(i, c)| Row {
                id: 1000 + i as i64,
                census: true,
                slug: Some(format!("s{i}")),
                a: c.a,
                b: c.b,
                t: c.t,
                torsion: Vec::new(),
            })
            .collect();
        let chosen: Vec<&Row> = rows.iter().collect();
        let none = BTreeMap::new();
        // nothing stored: every curve gets a new row, every edge is inserted
        let plan = reconcile(l, &chosen, &none, &g).unwrap();
        assert_eq!((plan.inserts.len(), plan.fills.len(), plan.agree, plan.skip.len()), (curves.len(), 0, 0, 0));

        let e = &g.edges[0];
        let (src, tgt) = (rows[e.source].id, rows[e.target].id);
        // a stored edge equal to a computed one is recognised and not inserted again (also with no kernel)
        for kernel in [Some(e.kernel.clone()), None] {
            let stored = BTreeMap::from([((src, l), vec![(tgt, kernel)])]);
            let plan = reconcile(l, &chosen, &stored, &g).unwrap();
            assert_eq!(plan.skip.len(), 1);
            let k = *plan.skip.iter().next().unwrap();
            assert_eq!((g.edges[k].source, g.edges[k].target), (e.source, e.target));
        }
        // a stored edge with a corrupted kernel, a wrong target, or stored twice is refused
        let mut bad_kernel = e.kernel.clone();
        bad_kernel[0] = (bad_kernel[0] + 1) % p;
        let wrong_target =
            rows.iter().find(|r| g.edges.iter().all(|x| x.source != e.source || rows[x.target].id != r.id));
        let mut corrupt = vec![
            BTreeMap::from([((src, l), vec![(tgt, Some(bad_kernel))])]),
            BTreeMap::from([((src, l), vec![(tgt, Some(e.kernel.clone())), (tgt, Some(e.kernel.clone()))])]),
        ];
        corrupt.push(BTreeMap::from([((src, l), vec![(wrong_target.unwrap().id, None)])]));
        for stored in &corrupt {
            assert!(reconcile(l, &chosen, stored, &g).unwrap_err().contains("not a computed edge"));
        }

        // curve_ell_torsion: agree, fill a NULL count, refuse a wrong count or action
        let (action, n) = g.curves[e.source];
        let action = action.unwrap().as_str().to_string();
        let with = |tors: (u64, Option<i32>, Option<String>)| {
            let mut rows2: Vec<Row> =
                rows.iter().map(|r| Row { torsion: Vec::new(), slug: r.slug.clone(), ..*r }).collect();
            rows2[e.source].torsion = vec![tors];
            rows2
        };
        let r = with((l, Some(n as i32), Some(action.clone())));
        let plan = reconcile(l, &r.iter().collect::<Vec<_>>(), &none, &g).unwrap();
        assert_eq!((plan.agree, plan.fills.len(), plan.inserts.len()), (1, 0, curves.len() - 1));
        let r = with((l, None, None));
        let plan = reconcile(l, &r.iter().collect::<Vec<_>>(), &none, &g).unwrap();
        assert_eq!((plan.agree, plan.fills.len()), (0, 1));
        assert_eq!(plan.fills[0]["n"], json!(n));
        // a row for another l is not this l's row
        let r = with((11, Some(99), Some("bogus".into())));
        let plan = reconcile(l, &r.iter().collect::<Vec<_>>(), &none, &g).unwrap();
        assert_eq!(plan.inserts.len(), curves.len());
        for bad in [(l, Some(n as i32 + 1), None), (l, None, Some("inert-or-other".to_string())), (l, Some(-1), None)] {
            let r = with(bad);
            assert!(reconcile(l, &r.iter().collect::<Vec<_>>(), &none, &g)
                .unwrap_err()
                .contains("stored curve_ell_torsion"));
        }
    }

    #[test]
    fn database_shortfall_is_a_problem() {
        assert_eq!(shortfall_problem("rows", 5, 5), None);
        assert_eq!(shortfall_problem("rows", 0, 0), None);
        assert!(shortfall_problem("rows", 5, 3).unwrap().starts_with("2 of 5 rows"));
    }

    #[test]
    fn modular_polynomial_roots_cover_the_targets() {
        let p = 107u64;
        let curves = census(p);
        for l in ELLS {
            let ph = phi(l, p);
            let g = field_graph(p, l, &curves, &ph).unwrap();
            for e in &g.edges {
                let c = &curves[e.source];
                let t = &curves[e.target];
                assert_eq!(peval(&phi_at(&ph, j_invariant(p, c.a, c.b), p), j_invariant(p, t.a, t.b), p), 0);
                assert_eq!(c.t, t.t);
            }
        }
    }

    /// Every field of the census (p < 2^10). Slow in debug builds:
    /// `cargo test --release ell_isogenies::tests::full_census_sweep -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn full_census_sweep() {
        let text = include_str!("../../../src/cryptanalysis/curve_traits/modular_polynomials.txt");
        let phis = crate::import::enrich::modular_polynomials(text).unwrap();
        let max: u64 = std::env::var("ECDB_SWEEP_MAX_P").ok().and_then(|s| s.parse().ok()).unwrap_or(MAX_P);
        let mut totals: BTreeMap<String, u64> = BTreeMap::new();
        for p in (5u64..max).filter(|&p| is_prime(p)) {
            let curves = census(p);
            for l in ELLS {
                if l == p {
                    continue;
                }
                let g = field_graph(p, l, &curves, &phi_mod_p(&phis[&(l as u32)], l, p))
                    .unwrap_or_else(|e| panic!("p = {p}, l = {l}: {e}"));
                *totals.entry(format!("curves_l{l}")).or_default() += curves.len() as u64;
                *totals.entry(format!("edges_l{l}")).or_default() += g.edges.len() as u64;
                *totals.entry(format!("phi_excess_curves_l{l}")).or_default() += g.phi_excess_curves;
                for (a, _) in &g.curves {
                    *totals.entry(format!("action_l{l}_{}", a.unwrap().as_str())).or_default() += 1;
                }
            }
        }
        println!("{totals:#?}");
    }
}
