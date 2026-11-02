//! `curve25519-family` — Curve25519's structure, and scaled-down analogues
//! built by Curve25519's own selection rule, for index-calculus baselines.
//!
//! Curve25519 (Bernstein, PKC 2006; RFC 7748) is the Montgomery curve
//! `E_A : y² = x³ + A x² + x` over `F_p`, `p = 2^255 − 19`, `A = 486662`,
//! with `#E = 8ℓ`, `#E' = 4ℓ'` (twist), `ℓ`, `ℓ'` prime.  No index-calculus
//! pipeline runs at 255 bits, so baselines are measured on miniatures that
//! keep every structural property the 255-bit curve has and that an
//! index calculus could see:
//!
//! * `p` the largest prime below `2^k` with `p ≡ 5 (mod 8)` (as `2^255 − 19`);
//! * Montgomery form with `B = 1` and `A ≡ 2 (mod 4)`, so `a24 = (A − 2)/4`
//!   is an integer (Curve25519: 121665);
//! * `#E = 8r`, `#E' = 4r'`, `r`, `r'` prime, `r > 2^(k−3)` (Curve25519
//!   rejected `A = 358990` and `464586` for a prime below `2^252`);
//! * `A` the **smallest** such value (Curve25519's rule; 486662 at k = 255);
//! * base point: the smallest `u ≥ 2` of order `r` (Curve25519: `u = 9`).
//!
//! `audit` re-derives the 255-bit curve's facts from first principles with
//! big integers (primality, `[ℓ](9) = O` by the x-only ladder, twist order,
//! 2-Sylow shape, 4-torsion, embedding degree bound, trace, `j`), so the
//! report does not rest on recalled constants.  `family` generates the
//! miniatures, checks every property, verifies the Montgomery summation
//! polynomial `S₃` on random point triples, and writes an `ecbench.spec/v1`
//! curve list in short Weierstrass form (`x_W = x_M + A/3`), the form
//! `ecbench`'s `prime_explicit` workload takes.
//!
//! Native Rust, no Python (AGENTS.md "Implementation language").

use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::{json, Value};
use std::collections::HashMap;

// ── u64 prime-field helpers (p < 2^63) ───────────────────────────────

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn addm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}
fn subm(a: u64, b: u64, p: u64) -> u64 {
    addm(a, p - b % p, p)
}
fn powm(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, b, p);
        }
        b = mulm(b, b, p);
        e >>= 1;
    }
    r
}
fn invm(a: u64, p: u64) -> u64 {
    assert!(!a.is_multiple_of(p), "inverse of zero");
    powm(a, p - 2, p)
}
/// Legendre symbol as −1, 0, 1.
fn legendre(a: u64, p: u64) -> i32 {
    let a = a % p;
    if a == 0 {
        return 0;
    }
    if powm(a, (p - 1) / 2, p) == 1 {
        1
    } else {
        -1
    }
}
/// Square root for `p ≡ 5 (mod 8)` (Atkin), the case every curve here has.
fn sqrtm(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if legendre(a, p) != 1 {
        return None;
    }
    assert_eq!(p % 8, 5);
    let two_a = addm(a, a, p);
    let v = powm(two_a, (p - 5) / 8, p);
    let i = mulm(mulm(two_a, v, p), v, p);
    let x = mulm(mulm(a, v, p), subm(i, 1, p), p);
    debug_assert_eq!(mulm(x, x, p), a);
    Some(x)
}

fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n.is_multiple_of(q) {
            return n == q;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d.is_multiple_of(2) {
        d /= 2;
        s += 1;
    }
    // Deterministic for n < 3.3·10^24 with these bases.
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powm(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulm(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

// ── short Weierstrass affine arithmetic (for counting and checks) ────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pt {
    x: u64,
    y: u64,
    inf: bool,
}
const INF: Pt = Pt {
    x: 0,
    y: 0,
    inf: true,
};

#[derive(Clone, Copy, Debug)]
struct Weier {
    p: u64,
    a: u64,
    b: u64,
}

impl Weier {
    fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        addm(
            addm(mulm(mulm(x, x, p), x, p), mulm(self.a, x, p), p),
            self.b,
            p,
        )
    }
    fn on(&self, q: Pt) -> bool {
        q.inf || mulm(q.y, q.y, self.p) == self.rhs(q.x)
    }
    fn neg(&self, q: Pt) -> Pt {
        if q.inf {
            q
        } else {
            Pt {
                x: q.x,
                y: (self.p - q.y) % self.p,
                inf: false,
            }
        }
    }
    fn add(&self, s: Pt, t: Pt) -> Pt {
        let p = self.p;
        if s.inf {
            return t;
        }
        if t.inf {
            return s;
        }
        let lam = if s.x == t.x {
            if addm(s.y, t.y, p) == 0 {
                return INF;
            }
            let num = addm(mulm(3, mulm(s.x, s.x, p), p), self.a, p);
            mulm(num, invm(addm(s.y, s.y, p), p), p)
        } else {
            mulm(subm(t.y, s.y, p), invm(subm(t.x, s.x, p), p), p)
        };
        let x3 = subm(subm(mulm(lam, lam, p), s.x, p), t.x, p);
        let y3 = subm(mulm(lam, subm(s.x, x3, p), p), s.y, p);
        Pt {
            x: x3,
            y: y3,
            inf: false,
        }
    }
    fn mul(&self, q: Pt, mut k: u64) -> Pt {
        let mut acc = INF;
        let mut base = q;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(acc, base);
            }
            base = self.add(base, base);
            k >>= 1;
        }
        acc
    }
    /// The `m` in the Hasse interval with `[m]q = O`, by baby-step
    /// giant-step: `O(p^{1/4})` additions.
    fn hasse_annihilators(&self, q: Pt) -> Vec<u64> {
        let p = self.p;
        let w = 2 * ((4 * p as u128).isqrt() as u64) + 1; // ⌊2√p⌋·2 + 1
        let lo = p + 1 - (4 * p as u128).isqrt() as u64;
        let s = (w as f64).sqrt().ceil() as u64 + 1;
        let mut baby: HashMap<u64, Vec<(u64, u64)>> = HashMap::new();
        let mut zero_js = Vec::new(); // j with [j]q = O (small-order q)
        let mut jp = INF;
        for j in 0..s {
            if jp.inf {
                zero_js.push(j);
            } else {
                baby.entry(jp.x).or_default().push((j, jp.y));
            }
            jp = self.add(jp, q);
        }
        let step = self.mul(q, s);
        let mut r = self.mul(q, lo);
        let mut hits = Vec::new();
        let mut i = 0u64;
        while i * s < w + s {
            let base = lo + i * s;
            if r.inf {
                hits.extend(zero_js.iter().map(|j| base + j));
            } else if let Some(v) = baby.get(&r.x) {
                for &(j, y) in v {
                    if addm(y, r.y, p) == 0 {
                        hits.push(base + j);
                    }
                }
            }
            r = self.add(r, step);
            i += 1;
        }
        hits.retain(|&m| m >= lo && m < lo + w);
        hits.sort_unstable();
        hits.dedup();
        hits
    }
    /// `#E(F_p)` by intersecting annihilator sets of points `x = 1, 2, …`.
    fn order(&self) -> u64 {
        let mut cand: Option<Vec<u64>> = None;
        let mut x = 1u64;
        loop {
            if let Some(y) = sqrtm(self.rhs(x), self.p) {
                let h = self.hasse_annihilators(Pt { x, y, inf: false });
                cand = Some(match cand {
                    None => h,
                    Some(c) => c.into_iter().filter(|m| h.contains(m)).collect(),
                });
                if cand.as_ref().unwrap().len() == 1 {
                    return cand.unwrap()[0];
                }
            }
            x += 1;
            assert!(
                x < 10_000,
                "order did not resolve: p={} a={} b={}",
                self.p,
                self.a,
                self.b
            );
        }
    }
}

// ── Montgomery curve y² = x³ + A x² + x (B = 1) ─────────────────────

#[derive(Clone, Copy, Debug)]
struct Mont {
    p: u64,
    a: u64,
}

impl Mont {
    fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        // x (x² + A x + 1)
        mulm(x, addm(addm(mulm(x, x, p), mulm(self.a, x, p), p), 1, p), p)
    }
    /// The short Weierstrass model `x_W = x + A/3`, `y_W = y`.
    fn weier(&self) -> (Weier, u64) {
        let p = self.p;
        let a = self.a;
        let i3 = invm(3, p);
        let a2 = mulm(a, a, p);
        let wa = mulm(subm(3, a2, p), i3, p);
        let wb = mulm(
            subm(mulm(2, mulm(a2, a, p), p), mulm(9, a, p), p),
            invm(27, p),
            p,
        );
        (Weier { p, a: wa, b: wb }, mulm(a, i3, p))
    }
    /// The quadratic twist's model: `d·y² = rhs(x)`, `d` a non-square;
    /// as a Weierstrass curve `y² = x³ + d²·aW x + d³·bW` (scaling by d).
    fn twist_weier(&self) -> Weier {
        let (w, _) = self.weier();
        let p = self.p;
        let d = (2..p).find(|&d| legendre(d, p) == -1).unwrap();
        Weier {
            p,
            a: mulm(mulm(d, d, p), w.a, p),
            b: mulm(mulm(mulm(d, d, p), d, p), w.b, p),
        }
    }
    /// Montgomery `S₃(x1, x2, x3)`: zero iff `±P1 ± P2 ± P3 = O` for points
    /// with these abscissae (from the differential-addition identities
    /// `x₊ + x₋ = 2((x1x2 + 1)(x1 + x2) + 2A x1x2)/(x1 − x2)²` and
    /// `x₊·x₋ = (x1x2 − 1)²/(x1 − x2)²`):
    /// `S₃ = (x1 − x2)² x3² − 2((x1x2 + 1)(x1 + x2) + 2A x1x2) x3 + (x1x2 − 1)²`.
    fn s3(&self, x1: u64, x2: u64, x3: u64) -> u64 {
        let p = self.p;
        let x12 = mulm(x1, x2, p);
        let d = subm(x1, x2, p);
        let t0 = mulm(mulm(d, d, p), mulm(x3, x3, p), p);
        let inner = addm(
            mulm(addm(x12, 1, p), addm(x1, x2, p), p),
            mulm(mulm(2, self.a, p), x12, p),
            p,
        );
        let t1 = mulm(mulm(2, inner, p), x3, p);
        let t2 = mulm(subm(x12, 1, p), subm(x12, 1, p), p);
        addm(subm(t0, t1, p), t2, p)
    }
}

#[derive(Clone, Debug)]
struct Miniature {
    k: u32,
    p: u64,
    a_mont: u64,
    order: u64,
    r: u64,
    twist_order: u64,
    r_twist: u64,
    candidates_tried: u64,
    u_base: u64,
    weier: Weier,
    shift: u64,
    g: Pt,
    t0_weier: Pt,
    order4_x_mont: u64,
    trace: i64,
    cm_disc_fundamental: i128,
    cm_conductor: u64,
    embedding_degree_gt: u64,
    s3_checks: u64,
}

fn largest_prime_5mod8_below(k: u32) -> u64 {
    let mut p = (1u64 << k) - 1;
    while p % 8 != 5 {
        p -= 1;
    }
    while !is_prime_u64(p) {
        p -= 8;
    }
    p
}

/// `(D₀, f)` with `t² − 4p = D₀ f²`, `D₀` a fundamental discriminant.
fn cm_discriminant(t: i64, p: u64) -> (i128, u64) {
    let mut d: i128 = (t as i128) * (t as i128) - 4 * p as i128;
    let mut f: u64 = 1;
    let mut q: i128 = 2;
    // Remove square factors by trial division (|d| < 2^66 here).
    while q * q <= d.abs() {
        while d % (q * q) == 0 {
            d /= q * q;
            f *= q as u64;
        }
        q += if q == 2 { 1 } else { 2 };
        if q > 3_000_000 {
            break; // cofactor left: report the partial reduction
        }
    }
    // A fundamental discriminant is ≡ 1 (mod 4) or 4·(≡ 2, 3 mod 4).
    if d.rem_euclid(4) != 1 {
        d *= 4;
        f /= 2;
    }
    (d, f)
}

fn build_miniature(k: u32) -> Miniature {
    let p = largest_prime_5mod8_below(k);
    let mut tried = 0;
    let mut a = 6u64; // A ≡ 2 (mod 4), A ≠ ±2
    loop {
        tried += 1;
        let m = Mont { p, a };
        // A² − 4 a non-square: one rational 2-torsion point, cyclic 2-Sylow.
        if legendre(subm(mulm(a, a, p), 4, p), p) == -1 {
            let (w, shift) = m.weier();
            let n = w.order();
            let nt = 2 * p + 2 - n;
            if n % 8 == 0
                && nt.is_multiple_of(4)
                && is_prime_u64(n / 8)
                && is_prime_u64(nt / 4)
                && n / 8 > (1u64 << (k - 3))
            {
                // Cross-check the twist count independently.
                assert_eq!(m.twist_weier().order(), nt, "twist order mismatch");
                let r = n / 8;
                // Smallest u ≥ 2 on E of order r.
                let (u, g) = (2..p)
                    .find_map(|u| {
                        let y = sqrtm(m.rhs(u), p)?;
                        let q = Pt {
                            x: addm(u, shift, p),
                            y: y.min(p - y),
                            inf: false,
                        };
                        (w.mul(q, r).inf).then_some((u, q))
                    })
                    .unwrap();
                assert!(w.on(g) && !g.inf && w.mul(g, r).inf);
                let t0 = Pt {
                    x: shift,
                    y: 0,
                    inf: false,
                };
                assert!(w.on(t0));
                // A rational point of order 4 has Montgomery x = ±1.
                let o4 = if legendre(addm(a, 2, p), p) == 1 {
                    1
                } else {
                    p - 1
                };
                let trace = (p + 1) as i64 - n as i64;
                let (d0, f) = cm_discriminant(trace, p);
                // Embedding degree: smallest e with p^e ≡ 1 (mod r).
                let mut e = 1u64;
                let mut pe = p % r;
                while pe != 1 && e < 1000 {
                    pe = mulm(pe, p % r, r);
                    e += 1;
                }
                // Summation polynomial checks on random triples P1 + P2 + P3 = O.
                let mut s3_checks = 0;
                let mut seed = 0x2551_9u64 ^ p;
                for _ in 0..200 {
                    seed = seed
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    let s1 = w.mul(g, (seed >> 11) % r);
                    seed = seed
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    let s2 = w.mul(g, (seed >> 11) % r);
                    let s3 = w.neg(w.add(s1, s2));
                    if s1.inf || s2.inf || s3.inf {
                        continue;
                    }
                    let mx = |q: Pt| subm(q.x, shift, p);
                    assert_eq!(m.s3(mx(s1), mx(s2), mx(s3)), 0, "S3 must vanish");
                    // And a perturbed abscissa must not (overwhelmingly).
                    let bad = addm(mx(s3), 1, p);
                    let other = w.add(s1, w.neg(s2));
                    if !other.inf && mx(other) != bad {
                        assert_ne!(m.s3(mx(s1), mx(s2), bad), 0);
                    }
                    s3_checks += 1;
                }
                return Miniature {
                    k,
                    p,
                    a_mont: a,
                    order: n,
                    r,
                    twist_order: nt,
                    r_twist: nt / 4,
                    candidates_tried: tried,
                    u_base: u,
                    weier: w,
                    shift,
                    g,
                    t0_weier: t0,
                    order4_x_mont: o4,
                    trace,
                    cm_disc_fundamental: d0,
                    cm_conductor: f,
                    embedding_degree_gt: e,
                    s3_checks,
                };
            }
        }
        a += 4;
        assert!(a < p, "no A found");
    }
}

// ── the 255-bit audit (big integers) ──────────────────────────────────

fn big(s: &str) -> BigUint {
    BigUint::parse_bytes(s.as_bytes(), 10).unwrap()
}

fn mr_big(n: &BigUint) -> bool {
    let one = BigUint::one();
    let two = &one + &one;
    if *n < two {
        return false;
    }
    let nm1 = n - &one;
    let mut d = nm1.clone();
    let mut s = 0;
    while (&d % &two).is_zero() {
        d /= &two;
        s += 1;
    }
    'w: for a in [
        2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53,
    ] {
        let a = BigUint::from(a);
        let mut x = a.modpow(&d, n);
        if x == one || x == nm1 {
            continue;
        }
        for _ in 1..s {
            x = x.modpow(&two, n);
            if x == nm1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

/// x-only Montgomery ladder; returns `(X : Z)` of `[k](u : 1)`.
fn ladder(u: &BigUint, k: &BigUint, a24: &BigUint, p: &BigUint) -> (BigUint, BigUint) {
    let sub = |a: &BigUint, b: &BigUint| ((a + p) - b) % p;
    let (mut x2, mut z2) = (BigUint::one(), BigUint::zero());
    let (mut x3, mut z3) = (u.clone(), BigUint::one());
    for i in (0..k.bits()).rev() {
        let bit = k.bit(i);
        if bit {
            std::mem::swap(&mut x2, &mut x3);
            std::mem::swap(&mut z2, &mut z3);
        }
        let a = (&x2 + &z2) % p;
        let aa = (&a * &a) % p;
        let b = sub(&x2, &z2);
        let bb = (&b * &b) % p;
        let e = sub(&aa, &bb);
        let c = (&x3 + &z3) % p;
        let d = sub(&x3, &z3);
        let da = (&d * &a) % p;
        let cb = (&c * &b) % p;
        let s = (&da + &cb) % p;
        let t = sub(&da, &cb);
        x3 = (&s * &s) % p;
        z3 = (u * ((&t * &t) % p)) % p;
        x2 = (&aa * &bb) % p;
        z2 = (&e * ((&aa + (a24 * &e)) % p)) % p;
        if bit {
            std::mem::swap(&mut x2, &mut x3);
            std::mem::swap(&mut z2, &mut z3);
        }
    }
    (x2, z2)
}

fn audit_255() -> Value {
    let p = (BigUint::one() << 255u32) - BigUint::from(19u32);
    let a = BigUint::from(486662u32);
    let l = (BigUint::one() << 252u32) + big("27742317777372353535851937790883648493");
    let n = &l * 8u32;
    let one = BigUint::one();
    let two_p2 = (&p + &one) * 2u32;
    let nt = &two_p2 - &n;
    let lt = &nt / 4u32;
    let a24 = (&a - 2u32) / 4u32; // RFC 7748: a24 = (A − 2)/4 = 121665 with z2 = E(AA + a24·E)
    let (_, z_l) = ladder(&BigUint::from(9u32), &l, &a24, &p);
    let (_, z_lm1) = ladder(&BigUint::from(9u32), &(&l - &one), &a24, &p);
    let pm = |e: &BigUint, x: &BigUint| x.modpow(e, &p);
    let half = (&p - &one) / 2u32;
    let leg = |x: &BigUint| {
        let v = pm(&half, &(x % &p));
        if v.is_zero() {
            0
        } else if v == one {
            1
        } else {
            -1
        }
    };
    let a2m4 = (&a * &a - 4u32) % &p;
    // Embedding degree: smallest e ≤ 10^4 with p^e ≡ 1 (mod ℓ), if any.
    let pl = &p % &l;
    let mut acc = pl.clone();
    let mut emb = None;
    for e in 1..=10_000u32 {
        if acc == one {
            emb = Some(e);
            break;
        }
        acc = (&acc * &pl) % &l;
    }
    // t = p + 1 − 8ℓ is negative for Curve25519: store |t| and the sign.
    let (trace_neg, trace_big) = if n > &p + &one {
        (true, &n - (&p + &one))
    } else {
        (false, (&p + &one) - &n)
    };
    // j = 256 (A² − 3)³ / (A² − 4)
    let a2 = (&a * &a) % &p;
    let num =
        (BigUint::from(256u32) * ((&a2 + &p - 3u32) % &p).modpow(&BigUint::from(3u32), &p)) % &p;
    let j = (num * a2m4.modpow(&(&p - 2u32), &p)) % &p;
    // #E = 8ℓ from first principles: ℓ | ord(9) and 8ℓ is the only multiple of
    // ℓ in the Hasse interval (|t| ≤ 2√p and ℓ > 4√p).
    let sqrt4p = (&p * 4u32).sqrt();
    let hasse_ok = trace_big <= sqrt4p && l > &sqrt4p * 2u32;
    // Twist: smallest u with u(u² + Au + 1) a non-square lies on E'; the same
    // x-only ladder runs there.  Check [4ℓ']u = O and [4ℓ' / ℓ']u ≠ O.
    let rhs = |u: &BigUint| (u * ((u * u + &a * u + 1u32) % &p)) % &p;
    let ut = (2u32..)
        .map(BigUint::from)
        .find(|u| leg(&rhs(u)) == -1)
        .unwrap();
    let (_, zt) = ladder(&ut, &nt, &a24, &p);
    let (_, zt4) = ladder(&ut, &BigUint::from(4u32), &a24, &p);
    let twist_ok = zt.is_zero() && !zt4.is_zero();
    json!({
        "p": p.to_string(), "p_bits": p.bits(), "p_mod_8": (&p % 8u32).to_u32(),
        "p_prime": mr_big(&p),
        "A": 486662, "a24_rfc7748": a24.to_string(),
        "l": l.to_string(), "l_bits": l.bits(), "l_prime": mr_big(&l),
        "order_E": n.to_string(), "cofactor": 8,
        "order_twist": nt.to_string(), "twist_cofactor": 4,
        "l_twist": lt.to_string(), "l_twist_prime": mr_big(&lt),
        "twist_order_divisible_by_4": (&nt % 4u32).is_zero(),
        "base_u9_order_l": z_l.is_zero() && !z_lm1.is_zero(),
        "order_E_from_hasse_uniqueness": hasse_ok,
        "twist_point_u": ut.to_string(), "twist_point_killed_by_order_twist": twist_ok,
        "A2_minus_4_legendre": leg(&a2m4),
        "two_sylow": if leg(&a2m4) == -1 { "cyclic Z/8 (one rational 2-torsion point T0=(0,0))" } else { "non-cyclic" },
        "A_plus_2_legendre": leg(&((&a + 2u32) % &p)),
        "A_minus_2_legendre": leg(&((&a - 2u32) % &p)),
        "order4_points_x": if leg(&((&a + 2u32) % &p)) == 1 { "x = 1" } else { "x = -1" },
        "minus1_is_square": leg(&(&p - &one)) == 1,
        "two_is_square": leg(&BigUint::from(2u32)) == 1,
        "trace": format!("{}{}", if trace_neg { "-" } else { "" }, trace_big), "trace_abs_bits": trace_big.bits(),
        "anomalous": !trace_neg && trace_big == one,
        "supersingular": trace_big.is_zero(),
        "embedding_degree_le_10000": emb,
        "j_invariant": j.to_string(),
        "j_is_0_or_1728": j.is_zero() || j == BigUint::from(1728u32),
    })
}

// ── summarise: per-phase and P1 analysis of the baseline sessions ─────

/// Per (session, curve, arm): measured, verified runs only (warm-ups out).
#[derive(Default, Clone)]
struct Agg {
    n: u64,
    s_sum: f64,
    gae: std::collections::BTreeMap<String, f64>,
    trials: f64,
    relations: f64,
}

fn summarise(dirs: &[String]) -> (Value, String, String) {
    use std::collections::BTreeMap;
    let mut out_rows = Vec::new();
    let mut md = String::from(
        "| k | curve | cofactor | arm | N | runs | mean S (w=0) | S / rho | trials / relation | phase GAE share: base / table / relations / LA+verify |\n|--:|:--|--:|:--|--:|--:|--:|--:|--:|:--|\n",
    );
    // For the figure: (k, cofactor, arm) -> mean S / rho.
    let mut fig: Vec<(u32, u64, String, f64)> = Vec::new();
    let mut p1 = Vec::new();
    for dir in dirs {
        let k: u32 = dir
            .trim_end_matches('/')
            .rsplit('k')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let text = std::fs::read_to_string(format!("{dir}/records.jsonl")).unwrap();
        let mut agg: BTreeMap<(u64, String, String), Agg> = BTreeMap::new();
        let mut cof: BTreeMap<String, u64> = BTreeMap::new();
        let mut size: BTreeMap<String, String> = BTreeMap::new();
        for line in text.lines() {
            let r: Value = serde_json::from_str(line).unwrap();
            if r["warmup"].as_bool().unwrap_or(false) || r["outcome"]["status"] != "verified" {
                continue;
            }
            let curve = r["workload"]["curve"]["slug"].as_str().unwrap().to_string();
            let go = r["workload"]["curve"]["group_order"].as_u64().unwrap();
            let rr = r["workload"]["curve"]["r"].as_u64().unwrap();
            let h = go / rr;
            cof.insert(curve.clone(), h);
            let arm = r["arm"].as_str().unwrap().to_string();
            if let Some(fb) = r["method"]["params"]["factor_base"].as_str() {
                let n = fb.split("size=").nth(1).unwrap().split(',').next().unwrap();
                size.insert(arm.clone(), n.to_string());
            }
            let a = agg.entry((h, curve, arm)).or_default();
            a.n += 1;
            a.s_sum += r["cost"]["s"].as_f64().unwrap();
            if let Some(ph) = r["phases"].as_array() {
                for p in ph {
                    let name = p["name"].as_str().unwrap().to_string();
                    *a.gae.entry(name.clone()).or_default() += p["gae"].as_f64().unwrap_or(0.0);
                    if name == "relations" {
                        a.trials += p["native"]["trials"].as_f64().unwrap_or(0.0);
                        a.relations += p["native"]["relations"].as_f64().unwrap_or(0.0);
                    }
                }
            }
        }
        // Order: cofactor 8 first.
        let mut keys: Vec<_> = agg.keys().cloned().collect();
        keys.sort_by(|x, y| y.0.cmp(&x.0).then(x.2.cmp(&y.2)));
        let mut tpr: BTreeMap<(u64, String), f64> = BTreeMap::new();
        for key in keys {
            let (h, curve, arm) = &key;
            let a = &agg[&key];
            let rho = &agg[&(*h, curve.clone(), "rho-neg".to_string())];
            let s = a.s_sum / a.n as f64;
            let srho = rho.s_sum / rho.n as f64;
            let total: f64 = a.gae.values().sum();
            let share = |n: &str| 100.0 * a.gae.get(n).copied().unwrap_or(0.0) / total.max(1.0);
            let t_per_rel = if a.relations > 0.0 {
                a.trials / a.relations
            } else {
                f64::NAN
            };
            if arm.starts_with("abs-") || arm.starts_with("proj-") {
                tpr.insert((*h, arm.clone()), t_per_rel);
            }
            let n = size.get(arm).cloned().unwrap_or_else(|| "-".into());
            let ph = if arm.starts_with("rho") {
                "-".to_string()
            } else {
                format!(
                    "{:.0}% / {:.0}% / {:.0}% / {:.0}%",
                    share("factor_base"),
                    share("oracle_setup"),
                    share("relations"),
                    share("linear_algebra") + share("verify")
                )
            };
            md += &format!(
                "| {k} | `{curve}` | {h} | {arm} | {n} | {} | {s:.3} | {:.2} | {} | {ph} |\n",
                a.n,
                s / srho,
                if t_per_rel.is_nan() {
                    "-".to_string()
                } else {
                    format!("{t_per_rel:.0}")
                },
            );
            if !arm.ends_with("-aa") {
                fig.push((k, *h, arm.clone(), s / srho));
            }
            out_rows.push(json!({
                "k": k, "curve": curve, "cofactor": h, "arm": arm, "N": n, "runs": a.n,
                "mean_s_w0": s, "s_over_rho": s / srho,
                "trials": a.trials, "relations": a.relations, "trials_per_relation": if t_per_rel.is_nan() { Value::Null } else { json!(t_per_rel) },
                "phase_gae": a.gae,
            }));
        }
        for arm in ["a", "2a"] {
            let get = |h: u64, base: &str| {
                tpr.get(&(h, format!("{base}-{arm}")))
                    .copied()
                    .unwrap_or(f64::NAN)
            };
            let r8 = get(8, "abs") / get(8, "proj");
            let r1 = get(1, "abs") / get(1, "proj");
            p1.push(
                json!({"k": k, "N": arm, "abs_over_proj_trials_per_relation_cofactor8": r8,
                           "abs_over_proj_trials_per_relation_cofactor1": r1,
                           "P1_holds": r8 >= 4.0 && (0.67..=1.5).contains(&r1)}),
            );
        }
    }
    // SVG: S / rho against k, one line per (cofactor, arm), log y.
    let (w, hgt, ml, mr, mt, mb) = (860.0, 450.0, 70.0, 270.0, 30.0, 75.0);
    let ks = [20.0, 24.0, 28.0, 32.0];
    let xs = |k: f64| ml + (k - 20.0) / 12.0 * (w - ml - mr);
    let (ylo, yhi) = (1.0f64.log2(), 128.0f64.log2());
    let ys = |v: f64| mt + (yhi - v.log2()) / (yhi - ylo) * (hgt - mt - mb);
    let mut svg = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{hgt}' viewBox='0 0 {w} {hgt}' font-family='Helvetica,Arial,sans-serif' font-size='12'>\n<rect width='100%' height='100%' fill='white'/>\n<text x='{ml}' y='18' font-size='14' font-weight='bold'>Index calculus cost relative to matched rho (S_IC / S_rho, w = 0)</text>\n"
    );
    for v in [1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0] {
        let y = ys(v);
        svg += &format!("<line x1='{ml}' x2='{}' y1='{y:.1}' y2='{y:.1}' stroke='#ddd'/><text x='{}' y='{:.1}' text-anchor='end'>{v}</text>\n", w - mr, ml - 6.0, y + 4.0);
    }
    for k in ks {
        let x = xs(k);
        svg += &format!(
            "<text x='{x:.1}' y='{}' text-anchor='middle'>k = {k} (r ≈ 2^{})</text>\n",
            hgt - mb + 18.0,
            k as u32 - 3
        );
    }
    svg += &format!("<text x='{}' y='{}' text-anchor='middle'>field size k (bits of p for the Curve25519 miniature)</text>\n", (ml + w - mr) / 2.0, hgt - 30.0);
    let series = [
        (
            8u64,
            "abs-a",
            "#c0392b",
            "",
            "Curve25519 mini, small-x base (blind)",
        ),
        (
            8,
            "proj-a",
            "#1f77b4",
            "",
            "Curve25519 mini, cofactor-projected base",
        ),
        (
            1,
            "abs-a",
            "#555",
            "6,4",
            "cofactor-1 control, small-x base",
        ),
        (
            1,
            "proj-a",
            "#7fb3d5",
            "2,3",
            "cofactor-1 control, projected base",
        ),
    ];
    for (i, (h, arm, col, dash, label)) in series.iter().enumerate() {
        let pts: Vec<(f64, f64)> = fig
            .iter()
            .filter(|(_, hh, a, _)| hh == h && a == arm)
            .map(|(k, _, _, v)| (xs(*k as f64), ys(*v)))
            .collect();
        let path: Vec<String> = pts.iter().map(|(x, y)| format!("{x:.1},{y:.1}")).collect();
        svg += &format!("<polyline fill='none' stroke='{col}' stroke-width='2' stroke-dasharray='{dash}' points='{}'/>\n", path.join(" "));
        for (x, y) in &pts {
            svg += &format!("<circle cx='{x:.1}' cy='{y:.1}' r='3.5' fill='{col}'/>\n");
        }
        let ly = mt + 20.0 + 22.0 * i as f64;
        svg += &format!("<line x1='{}' x2='{}' y1='{ly}' y2='{ly}' stroke='{col}' stroke-width='2' stroke-dasharray='{dash}'/><text x='{}' y='{}' font-size='11'>{label}</text>\n", w - mr + 8.0, w - mr + 30.0, w - mr + 34.0, ly + 4.0);
    }
    let y1 = ys(1.0);
    svg += &format!(
        "<text x='{}' y='{:.1}' font-size='11' fill='#27ae60'>rho = 1</text>\n",
        ml + 4.0,
        y1 - 4.0
    );
    svg += &format!("<text x='{}' y='{}' font-size='10' fill='#666'>ecbench counted GAE, 24 targets × 2 rounds per curve; N = round(0.63·r^(1/3)); all runs verified</text>\n</svg>\n", ml, hgt - 8.0);
    (
        json!({"schema": "curve25519-ic-baseline.summary/v1", "rows": out_rows, "p1": p1}),
        md,
        svg,
    )
}

// ── t0: the 2-torsion symmetry x ↦ 1/x in m = 2 relation collection ────
//
// research/curve25519_t0_symmetry_20261102/PROTOCOL.md.  Exact coverage of
// three pair-table oracles with the same N columns (blind, T₀-keyed, and
// cofactor-projected), and a Monte Carlo of the blind and T₀-keyed oracles
// whose every hit is verified by point arithmetic.

/// `x ↦ x + 1/x` on Montgomery abscissae: invariant under translation by `T₀`.
fn kappa(xm: u64, p: u64) -> u64 {
    addm(xm, invm(xm, p), p)
}

fn t0_experiment(k: u32, n: usize, samples: u64, seed: u64) -> Value {
    let m = build_miniature(k);
    let (w, shift, p, r) = (m.weier, m.shift, m.p, m.r);
    let mx = |q: &Pt| subm(q.x, shift, p);
    let t0 = m.t0_weier;
    // A point of order 8 spanning E[8]: [r]Q for some Q, [4]([r]Q) ≠ O.
    let t8 = (2..p)
        .find_map(|u| {
            let y = sqrtm(Mont { p, a: m.a_mont }.rhs(u), p)?;
            let q = w.mul(
                Pt {
                    x: addm(u, shift, p),
                    y,
                    inf: false,
                },
                r,
            );
            (!w.mul(q, 4).inf).then_some(q)
        })
        .unwrap();
    assert_eq!(w.mul(t8, 4), t0, "[4]T8 must be T0");
    let e8: Vec<Pt> = (0..8).map(|j| w.mul(t8, j)).collect();
    let label = |q: &Pt| -> u64 {
        let t = w.mul(*q, r);
        e8.iter().position(|e| *e == t).expect("[r]P in E[8]") as u64
    };
    // Base: smallest Montgomery u ≥ 2 on E, not torsion, one point per u.
    let mont = Mont { p, a: m.a_mont };
    let mut base: Vec<(u64, Pt, u64)> = Vec::new();
    let mut u = 2u64;
    while base.len() < n {
        if let Some(y) = sqrtm(mont.rhs(u), p) {
            let q = Pt {
                x: addm(u, shift, p),
                y: y.min(p - y),
                inf: false,
            };
            if !w.mul(q, 8).inf {
                let l = label(&q);
                base.push((u, q, l));
            }
        }
        u += 1;
    }
    let max_u = u - 1;
    // Exact coverage: distinct target abscissae D for each oracle.
    use std::collections::HashSet;
    let (mut blind, mut t0set, mut proj) = (HashSet::new(), HashSet::new(), HashSet::new());
    let mut label_hist = [0u64; 8];
    for i in 0..n {
        for j in i..n {
            for sgn in [1i8, -1] {
                if i == j && sgn < 0 {
                    continue;
                }
                let (pj, lj) = if sgn > 0 {
                    (base[j].1, base[j].2)
                } else {
                    (w.neg(base[j].1), (8 - base[j].2) % 8)
                };
                let s = w.add(base[i].1, pj);
                if s.inf {
                    continue;
                }
                let l = (base[i].2 + lj) % 8;
                label_hist[l as usize] += 1;
                let xs = mx(&s);
                if l == 0 {
                    blind.insert(xs);
                    t0set.insert(xs);
                } else if l == 4 && xs != 0 {
                    t0set.insert(invm(xs, p)); // R = S + T₀ has x = 1/x(S)
                }
            }
        }
    }
    let pb: Vec<Pt> = {
        let mut seen = HashSet::new();
        base.iter()
            .map(|b| w.mul(b.1, 8))
            .filter(|q| !q.inf && seen.insert(q.x))
            .collect()
    };
    for i in 0..pb.len() {
        for j in i..pb.len() {
            for s in [w.add(pb[i], pb[j]), w.add(pb[i], w.neg(pb[j]))] {
                if !s.inf {
                    proj.insert(mx(&s));
                }
            }
        }
    }
    let y = |d: usize| 2.0 * d as f64 / (r - 1) as f64;
    let (yb, yt, yp) = (y(blind.len()), y(t0set.len()), y(proj.len()));

    // Monte Carlo: blind (key x) and t0 (key κ) tables, verified hits.
    let mut tab_b: HashMap<u64, (usize, usize, i8)> = HashMap::new();
    let mut tab_t: HashMap<u64, (usize, usize, i8)> = HashMap::new();
    for i in 0..n {
        for j in i..n {
            for sgn in [1i8, -1] {
                if i == j && sgn < 0 {
                    continue;
                }
                let pj = if sgn > 0 { base[j].1 } else { w.neg(base[j].1) };
                let s = w.add(base[i].1, pj);
                if s.inf || mx(&s) == 0 {
                    continue;
                }
                let lj = if sgn > 0 {
                    base[j].2
                } else {
                    (8 - base[j].2) % 8
                };
                let l = (base[i].2 + lj) % 8;
                if l == 0 {
                    tab_b.entry(mx(&s)).or_insert((i, j, sgn));
                }
                if l == 0 || l == 4 {
                    tab_t.entry(kappa(mx(&s), p)).or_insert((i, j, sgn));
                }
            }
        }
    }
    let mut st = seed;
    let (mut hb, mut ht, mut vb, mut vt, mut t0_only) = (0u64, 0u64, 0u64, 0u64, 0u64);
    for _ in 0..samples {
        st = st
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let a = 1 + (st >> 11) % (r - 1);
        let rt = w.mul(m.g, a);
        let xr = mx(&rt);
        if xr == 0 {
            continue;
        }
        let check = |(i, j, sgn): (usize, usize, i8)| -> bool {
            let pj = if sgn > 0 { base[j].1 } else { w.neg(base[j].1) };
            let s = w.add(base[i].1, pj);
            let st0 = w.add(s, t0);
            [s, w.neg(s), st0, w.neg(st0)].contains(&rt)
        };
        let b_hit = tab_b.get(&xr).copied();
        if let Some(e) = b_hit {
            hb += 1;
            let pj = if e.2 > 0 {
                base[e.1].1
            } else {
                w.neg(base[e.1].1)
            };
            let s = w.add(base[e.0].1, pj);
            if rt == s || rt == w.neg(s) {
                vb += 1;
            }
        }
        if let Some(e) = tab_t.get(&kappa(xr, p)).copied() {
            ht += 1;
            if check(e) {
                vt += 1;
            }
            if b_hit.is_none() {
                t0_only += 1;
            }
        }
    }
    let z = |hits: u64, yv: f64| {
        let e = samples as f64 * yv;
        (hits as f64 - e) / (e * (1.0 - yv)).sqrt().max(1e-9)
    };
    json!({
        "k": k, "p": p, "A": m.a_mont, "r": r, "N": n, "max_u": max_u,
        "coset_histogram_of_pair_sums": label_hist,
        "exact": {
            "D_blind": blind.len(), "D_t0": t0set.len(), "D_proj": proj.len(),
            "yield_blind": yb, "yield_t0": yt, "yield_proj": yp,
            "t0_over_blind": yt / yb, "proj_over_t0": yp / yt, "proj_over_blind": yp / yb,
            "Q1_holds": (1.8..=2.2).contains(&(yt / yb)),
            "Q2_holds": (3.6..=4.4).contains(&(yp / yt)),
        },
        "monte_carlo": {
            "samples": samples, "seed": seed,
            "blind_hits": hb, "blind_verified": vb, "blind_z": z(hb, yb),
            "t0_hits": ht, "t0_verified": vt, "t0_only_hits": t0_only, "t0_z": z(ht, yt),
            "Q3_holds": vt == ht && vb == hb && z(ht, yt).abs() <= 4.0 && z(hb, yb).abs() <= 4.0,
        }
    })
}

/// Figure for the T₀ study: covered-target ratios to the blind oracle, per
/// (k, N) cell, from the `t0` JSON lines.
fn t0_figure(jsonl: &str) -> String {
    let rows: Vec<Value> = jsonl
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let (w, h, ml, mr, mt, mb) = (820.0, 400.0, 60.0, 230.0, 34.0, 70.0);
    let n = rows.len() as f64;
    let xs = |i: f64| ml + (i + 0.5) / n * (w - ml - mr);
    let ys = |v: f64| mt + (1.0 - v / 9.0) * (h - mt - mb);
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}' font-family='Helvetica,Arial,sans-serif' font-size='12'>\n<rect width='100%' height='100%' fill='white'/>\n<text x='{ml}' y='20' font-size='14' font-weight='bold'>Targets covered per pair table, relative to the blind table (same N columns, exact counts)</text>\n");
    for v in [1.0, 2.0, 4.0, 8.0] {
        let y = ys(v);
        s += &format!("<line x1='{ml}' x2='{}' y1='{y:.1}' y2='{y:.1}' stroke='#ddd'/><text x='{}' y='{:.1}' text-anchor='end'>{v}×</text>\n", w - mr, ml - 6.0, y + 4.0);
    }
    let bw = (w - ml - mr) / n / 4.0;
    for (i, r) in rows.iter().enumerate() {
        let e = &r["exact"];
        let vals = [
            1.0,
            e["t0_over_blind"].as_f64().unwrap(),
            e["proj_over_blind"].as_f64().unwrap(),
        ];
        for (j, (v, c)) in vals
            .iter()
            .zip(["#c0392b", "#e67e22", "#1f77b4"])
            .enumerate()
        {
            let x = xs(i as f64) - 1.5 * bw + j as f64 * bw;
            let y = ys(*v);
            s += &format!(
                "<rect x='{x:.1}' y='{y:.1}' width='{:.1}' height='{:.1}' fill='{c}'/>\n",
                bw * 0.9,
                ys(0.0) - y
            );
        }
        s += &format!(
            "<text x='{:.1}' y='{}' text-anchor='middle' font-size='11'>k={} N={}</text>\n",
            xs(i as f64),
            h - mb + 16.0,
            r["k"],
            r["N"]
        );
    }
    for (j, (c, lab)) in [
        ("#c0392b", "blind (coset 0 of 8)"),
        ("#e67e22", "T₀-keyed, x + 1/x (cosets 0, 4)"),
        ("#1f77b4", "cofactor-projected = baseline (all 8)"),
    ]
    .iter()
    .enumerate()
    {
        let y = mt + 20.0 + 22.0 * j as f64;
        s += &format!("<rect x='{}' y='{}' width='14' height='10' fill='{c}'/><text x='{}' y='{}' font-size='11'>{lab}</text>\n", w - mr + 10.0, y - 9.0, w - mr + 30.0, y);
    }
    s += &format!("<text x='{ml}' y='{}' font-size='10' fill='#666'>Curve25519 miniatures; ratios of distinct covered target abscissae D (yield = 2D/(r−1)); theory 1 : 2 : 8</text>\n</svg>\n", h - 12.0);
    s
}

/// Closed cofactor fiber on a prime curve (ratchet round 1, Part 2): exact
/// covered targets and table entries for the blind small-x table, the closed
/// fiber (every pair sum shifted by its own torsion component, i.e. the
/// table of `P_i ± P_j + K`, `K ∈ E[8]`, restricted to the entries that land
/// in `G`), and the projected base, at the same N columns.
fn fiber_experiment(k: u32, n: usize) -> Value {
    use std::collections::HashSet;
    let m = build_miniature(k);
    let (w, shift, p, r) = (m.weier, m.shift, m.p, m.r);
    let mx = |q: &Pt| subm(q.x, shift, p);
    let mont = Mont { p, a: m.a_mont };
    let t8 = (2..p)
        .find_map(|u| {
            let y = sqrtm(mont.rhs(u), p)?;
            let q = w.mul(
                Pt {
                    x: addm(u, shift, p),
                    y,
                    inf: false,
                },
                r,
            );
            (!w.mul(q, 4).inf).then_some(q)
        })
        .unwrap();
    let e8: Vec<Pt> = (0..8).map(|j| w.mul(t8, j)).collect();
    let rinv8 = (1..8u64).find(|c| (c * (r % 8)) % 8 == 1).unwrap();
    let label = |q: &Pt| -> u64 {
        let t = w.mul(*q, r);
        e8.iter().position(|e| *e == t).unwrap() as u64
    };
    let mut base = Vec::new();
    let mut u = 2u64;
    while base.len() < n {
        if let Some(y) = sqrtm(mont.rhs(u), p) {
            let q = Pt {
                x: addm(u, shift, p),
                y: y.min(p - y),
                inf: false,
            };
            if !w.mul(q, 8).inf {
                base.push((q, label(&q)));
            }
        }
        u += 1;
    }
    let (mut blind, mut fiber, mut proj) = (HashSet::new(), HashSet::new(), HashSet::new());
    let (mut entries_blind, mut checked) = (0u64, 0u64);
    for i in 0..n {
        for j in i..n {
            for sgn in [1i8, -1] {
                if i == j && sgn < 0 {
                    continue;
                }
                let (pj, lj) = if sgn > 0 {
                    (base[j].0, base[j].1)
                } else {
                    (w.neg(base[j].0), (8 - base[j].1) % 8)
                };
                let s = w.add(base[i].0, pj);
                if s.inf {
                    continue;
                }
                entries_blind += 1;
                let l = (base[i].1 + lj) % 8;
                if l == 0 {
                    blind.insert(mx(&s));
                }
                // S − t(S), t(S) = (L · r⁻¹ mod 8)·T₈: the lift of S in G.
                let rs = w.add(s, w.neg(e8[((l * rinv8) % 8) as usize]));
                if !rs.inf {
                    if checked < 2000 {
                        assert!(w.mul(rs, r).inf, "fiber representative must lie in G");
                        checked += 1;
                    }
                    fiber.insert(mx(&rs));
                }
            }
        }
    }
    let pb: Vec<Pt> = {
        let mut seen = HashSet::new();
        base.iter()
            .map(|b| w.mul(b.0, 8))
            .filter(|q| !q.inf && seen.insert(q.x))
            .collect()
    };
    let mut entries_proj = 0u64;
    for i in 0..pb.len() {
        for j in i..pb.len() {
            for (idx, s) in [w.add(pb[i], pb[j]), w.add(pb[i], w.neg(pb[j]))]
                .into_iter()
                .enumerate()
            {
                if idx == 1 && i == j {
                    continue;
                }
                if !s.inf {
                    entries_proj += 1;
                    proj.insert(mx(&s));
                }
            }
        }
    }
    // The closed table holds every sum shifted by every K ∈ E[8].
    let entries_fiber = 8 * entries_blind;
    let (db, df, dp) = (blind.len() as f64, fiber.len() as f64, proj.len() as f64);
    json!({
        "k": k, "r": r, "N": n,
        "D_blind": blind.len(), "D_fiber": fiber.len(), "D_proj": proj.len(),
        "entries_blind": entries_blind, "entries_fiber": entries_fiber, "entries_proj": entries_proj,
        "fiber_over_blind_D": df / db, "fiber_over_proj_D": df / dp,
        "entries_fiber_over_proj": entries_fiber as f64 / entries_proj as f64,
        "coverage_per_entry_fiber_over_proj": (df / entries_fiber as f64) / (dp / entries_proj as f64),
        "F1_holds": (0.95..=1.05).contains(&(df / dp)),
        "F2_holds": (7.5..=8.5).contains(&(entries_fiber as f64 / entries_proj as f64)),
        "membership_checks": checked,
    })
}

/// Ratchet round 1 figure: mean S (w = 0) of the projected base against
/// N / N_a, one line per k, from the train sessions' records.
fn sweep_figure(dirs: &[String]) -> String {
    use std::collections::BTreeMap;
    let (w, h, ml, mr, mt, mb) = (760.0, 400.0, 60.0, 150.0, 34.0, 60.0);
    let xs = |f: f64| ml + (f - 0.3) / 0.75 * (w - ml - mr);
    let ys = |v: f64| mt + (1.0 - (v - 0.0) / 45.0) * (h - mt - mb);
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}' font-family='Helvetica,Arial,sans-serif' font-size='12'>\n<rect width='100%' height='100%' fill='white'/>\n<text x='{ml}' y='20' font-size='14' font-weight='bold'>Base-size retune (train): mean S at w = 0, projected base, mitm:negation_folded=1</text>\n");
    for v in [0.0, 10.0, 20.0, 30.0, 40.0] {
        let y = ys(v);
        s += &format!("<line x1='{ml}' x2='{}' y1='{y:.1}' y2='{y:.1}' stroke='#ddd'/><text x='{}' y='{:.1}' text-anchor='end'>{v}</text>\n", w - mr, ml - 6.0, y + 4.0);
    }
    for f in [0.35, 0.5, 0.7, 1.0] {
        s += &format!(
            "<text x='{:.1}' y='{}' text-anchor='middle'>{f}·N_a</text>\n",
            xs(f),
            h - mb + 18.0
        );
    }
    s += &format!("<text x='{}' y='{}' text-anchor='middle'>factor-base size N relative to the incumbent N_a = round(0.63·r^(1/3))</text>\n", (ml + w - mr) / 2.0, h - 14.0);
    let cols = ["#1f77b4", "#2ca02c", "#e67e22", "#c0392b"];
    for (ci, dir) in dirs.iter().enumerate() {
        let k: u32 = dir
            .trim_end_matches('/')
            .rsplit('k')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let mut agg: BTreeMap<u64, (f64, f64)> = BTreeMap::new();
        for line in std::fs::read_to_string(format!("{dir}/records.jsonl"))
            .unwrap()
            .lines()
        {
            let r: Value = serde_json::from_str(line).unwrap();
            if r["warmup"].as_bool().unwrap_or(false) || r["outcome"]["status"] != "verified" {
                continue;
            }
            let Some(n) = r["arm"].as_str().unwrap().strip_prefix("proj-") else {
                continue;
            };
            let e = agg.entry(n.parse().unwrap()).or_default();
            e.0 += r["cost"]["s"].as_f64().unwrap();
            e.1 += 1.0;
        }
        let na = *agg.keys().max().unwrap() as f64;
        let pts: Vec<(f64, f64)> = agg
            .iter()
            .map(|(n, (t, c))| (xs(*n as f64 / na), ys(t / c)))
            .collect();
        let path: Vec<String> = pts.iter().map(|(x, y)| format!("{x:.1},{y:.1}")).collect();
        s += &format!(
            "<polyline fill='none' stroke='{}' stroke-width='2' points='{}'/>\n",
            cols[ci % 4],
            path.join(" ")
        );
        for (x, y) in &pts {
            s += &format!(
                "<circle cx='{x:.1}' cy='{y:.1}' r='3.5' fill='{}'/>\n",
                cols[ci % 4]
            );
        }
        let ly = mt + 20.0 + 22.0 * ci as f64;
        s += &format!("<line x1='{}' x2='{}' y1='{ly}' y2='{ly}' stroke='{}' stroke-width='2'/><text x='{}' y='{}' font-size='11'>k = {k}</text>\n", w - mr + 10.0, w - mr + 32.0, cols[ci % 4], w - mr + 36.0, ly + 4.0);
    }
    s += "</svg>\n";
    s
}

/// Base size `N_a = round(0.63·r^{1/3})`: the lab's validated prime-order
/// optimum (`docs/ic/LEADERBOARD.md`, prime-field baseline: N = 162, 407,
/// 1024 at 24, 28, 32 bits is 0.63·n^{1/3}) scaled to the subgroup.
fn base_size(r: u64) -> u64 {
    (0.63 * (r as f64).cbrt()).round() as u64
}

fn baseline_spec(k: u32, targets: u64, seed: u64) -> Value {
    let m = build_miniature(k);
    let na = base_size(m.r);
    let ic = |base: String| json!({"id": "ic.pipeline", "params": {"factor_base": base, "oracle": "mitm:negation_folded=1"}});
    let abs = |n: u64| ic(format!("prime-abscissa:size={n}"));
    let proj = |n: u64| {
        ic(format!(
            "prime-projected-abscissa:size={n},scan_cap={}",
            16 * n
        ))
    };
    json!({
        "schema": "ecbench.spec/v1",
        "label": format!("Curve25519-family index-calculus baseline, k = {k}"),
        "description": "research/curve25519_ic_baseline_20261102/PROTOCOL.md (frozen before data). Curve 1: the k-bit Curve25519 miniature (Montgomery y^2 = x^3 + A x^2 + x, p = 5 mod 8, #E = 8r, #E' = 4r', smallest A by Curve25519's rule) in short Weierstrass form. Curve 2: cofactor-1 control, prime_search at the subgroup's bit length. Public synthetic curves and targets; no security claim.",
        "workloads": {
            "curves": [
                {"kind": "prime_explicit", "p": m.p, "a": m.weier.a, "b": m.weier.b,
                 "group_order": m.order, "r": m.r, "gx": m.g.x, "gy": m.g.y},
                {"kind": "prime_search", "bits": 64 - m.r.leading_zeros(), "seed": seed ^ 0x2551_9}
            ],
            "targets_per_curve": targets,
            "target_seed": seed + 1,
            "target_kind": "public"
        },
        "arms": [
            {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
            {"name": "abs-a", "role": "baseline", "method": abs(na)},
            {"name": "abs-2a", "role": "baseline", "method": abs(2 * na)},
            {"name": "proj-a", "role": "baseline", "method": proj(na)},
            {"name": "proj-2a", "role": "baseline", "method": proj(2 * na)},
            {"name": "abs-2a-aa", "role": "control", "method": abs(2 * na)},
            {"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}}
        ],
        "measurement": {"rounds": 2, "warmup": 1, "order": "alternate", "seed": seed + 2,
                        "isolation_required": "L0", "timeout_seconds": 1800}
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("help");
    match cmd {
        "audit" => {
            println!("{}", serde_json::to_string_pretty(&audit_255()).unwrap());
        }
        "family" => {
            let ks: Vec<u32> = args
                .get(2)
                .map(|s| s.split(',').map(|v| v.parse().unwrap()).collect())
                .unwrap_or(vec![20, 24, 28, 32]);
            let mut rows = Vec::new();
            let mut curves = Vec::new();
            for k in ks {
                assert!((12..=40).contains(&k), "k in 12..=40");
                let m = build_miniature(k);
                rows.push(json!({
                    "k": m.k, "p": m.p, "p_mod_8": m.p % 8, "A": m.a_mont,
                    "a24": (m.a_mont - 2) / 4,
                    "candidates_tried": m.candidates_tried,
                    "order_E": m.order, "cofactor": 8, "r": m.r, "r_bits": 64 - m.r.leading_zeros(),
                    "order_twist": m.twist_order, "twist_cofactor": 4, "r_twist": m.r_twist,
                    "base_u": m.u_base,
                    "weierstrass": {"a": m.weier.a, "b": m.weier.b, "x_shift_A_over_3": m.shift},
                    "generator_weierstrass": {"x": m.g.x, "y": m.g.y},
                    "t0_weierstrass": {"x": m.t0_weier.x, "y": 0},
                    "order4_x_mont": if m.order4_x_mont == 1 { "1".to_string() } else { "-1".to_string() },
                    "trace": m.trace,
                    "cm_fundamental_discriminant": m.cm_disc_fundamental.to_string(),
                    "cm_conductor": m.cm_conductor,
                    "embedding_degree": if m.embedding_degree_gt >= 1000 { json!(">=1000") } else { json!(m.embedding_degree_gt) },
                    "s3_identity_checks": m.s3_checks,
                }));
                curves.push(json!({
                    "kind": "prime_explicit", "p": m.p, "a": m.weier.a, "b": m.weier.b,
                    "group_order": m.order, "r": m.r, "gx": m.g.x, "gy": m.g.y,
                }));
                eprintln!(
                    "k={k}: p={} A={} #E=8·{} #E'=4·{} u={}",
                    m.p, m.a_mont, m.r, m.r_twist, m.u_base
                );
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "schema": "curve25519-family/v1",
                    "rule": "p = largest prime < 2^k with p = 5 mod 8; smallest A = 2 mod 4 (A >= 6) with A^2-4 non-square, #E = 8r, #E' = 4r', r, r' prime, r > 2^(k-3); base = smallest u >= 2 of order r",
                    "miniatures": rows,
                    "ecbench_curves": curves,
                }))
                .unwrap()
            );
        }
        "spec" => {
            // spec <k> <targets_per_curve> <seed>: the frozen baseline panel of
            // research/curve25519_ic_baseline_20261102/PROTOCOL.md for one size.
            let k: u32 = args[2].parse().unwrap();
            let targets: u64 = args[3].parse().unwrap();
            let seed: u64 = args[4].parse().unwrap();
            println!(
                "{}",
                serde_json::to_string_pretty(&baseline_spec(k, targets, seed)).unwrap()
            );
        }
        "t0" => {
            // t0 <k> <N> <samples> <seed>
            let k: u32 = args[2].parse().unwrap();
            let n: usize = args[3].parse().unwrap();
            let samples: u64 = args[4].parse().unwrap();
            let seed: u64 = args[5].parse().unwrap();
            println!(
                "{}",
                serde_json::to_string(&t0_experiment(k, n, samples, seed)).unwrap()
            );
        }
        "t0-figure" => {
            // t0-figure <t0.jsonl> <out.svg>
            let text = std::fs::read_to_string(&args[2]).unwrap();
            std::fs::write(&args[3], t0_figure(&text)).unwrap();
        }
        "fiber" => {
            // fiber <k> <N>
            let k: u32 = args[2].parse().unwrap();
            let n: usize = args[3].parse().unwrap();
            println!(
                "{}",
                serde_json::to_string(&fiber_experiment(k, n)).unwrap()
            );
        }
        "sweep-figure" => {
            // sweep-figure <out.svg> <train_session_dir>...
            std::fs::write(&args[2], sweep_figure(&args[3..])).unwrap();
        }
        "summarise" => {
            // summarise <out_prefix> <session_dir>...
            let (j, md, svg) = summarise(&args[3..]);
            let pre = &args[2];
            std::fs::write(
                format!("{pre}.json"),
                serde_json::to_string_pretty(&j).unwrap(),
            )
            .unwrap();
            std::fs::write(format!("{pre}.md"), md).unwrap();
            std::fs::write(format!("{pre}.svg"), svg).unwrap();
            println!("{}", serde_json::to_string_pretty(&j["p1"]).unwrap());
        }
        _ => {
            eprintln!(
                "usage: curve25519-family audit | family [k1,k2,...] | spec <k> <targets> <seed>"
            );
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// BSGS order of curve and twist against the Legendre-sum count, every
    /// `A ≡ 2 (mod 4)` below 200 on three small `p ≡ 5 (mod 8)` (this
    /// covers curves whose small-abscissa points are 2- and 4-torsion).
    #[test]
    fn order_matches_bruteforce() {
        for p in [65437u64, 1013, 4093] {
            assert!(p % 8 == 5 && is_prime_u64(p));
            for a in (6..200u64).step_by(4) {
                let m = Mont { p, a };
                if legendre(subm(mulm(a, a, p), 4, p), p) == 0 {
                    continue;
                }
                let (w, _) = m.weier();
                let mut n = 1u64;
                for x in 0..p {
                    n += (1 + legendre(w.rhs(x), p)) as u64;
                }
                assert_eq!(w.order(), n, "p={p} A={a}");
                assert_eq!(m.twist_weier().order(), 2 * p + 2 - n, "twist p={p} A={a}");
            }
        }
    }

    /// The 2-torsion symmetry: `T₀ = (0, 0)` translates `x ↦ 1/x`, and
    /// `(P1 + T₀) + (P2 + T₀) = P1 + P2`, so inverting two abscissae scales
    /// `S₃` by `(x1·x2)^{-2}` (the Montgomery analogue of Faugère–Gaudry–
    /// Huot–Renault's torsion symmetries).
    #[test]
    fn s3_inversion_symmetry_from_t0() {
        let p = 1048573u64;
        let m = Mont { p, a: 926 };
        let mut s = 7u64;
        for _ in 0..1000 {
            let mut nx = || {
                s = s
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                1 + (s >> 20) % (p - 1)
            };
            let (x1, x2, x3) = (nx(), nx(), nx());
            let lhs = m.s3(invm(x1, p), invm(x2, p), x3);
            let rhs = mulm(
                m.s3(x1, x2, x3),
                invm(mulm(mulm(x1, x2, p), mulm(x1, x2, p), p), p),
                p,
            );
            assert_eq!(lhs, rhs);
        }
        // And T₀ really maps x to 1/x on the curve.
        let (w, shift) = m.weier();
        let t0 = Pt {
            x: shift,
            y: 0,
            inf: false,
        };
        for x in 2..50u64 {
            if let Some(y) = sqrtm(m.rhs(x), p) {
                let q = w.add(
                    Pt {
                        x: addm(x, shift, p),
                        y,
                        inf: false,
                    },
                    t0,
                );
                assert_eq!(subm(q.x, shift, p), invm(x, p));
            }
        }
    }

    /// `S₃` vanishes on every `x(P ± Q)` and on nothing else.
    #[test]
    fn s3_roots_are_exactly_sum_and_difference() {
        let p = 1013u64;
        let m = Mont { p, a: 6 + 4 * 7 };
        let (w, shift) = m.weier();
        let pts: Vec<Pt> = (0..p)
            .filter_map(|x| {
                sqrtm(m.rhs(x), p).map(|y| Pt {
                    x: addm(x, shift, p),
                    y,
                    inf: false,
                })
            })
            .take(12)
            .collect();
        let mx = |q: Pt| subm(q.x, shift, p);
        for &s in &pts {
            for &t in &pts {
                if s.x == t.x {
                    continue;
                }
                let plus = w.add(s, t);
                let minus = w.add(s, w.neg(t));
                for x3 in 0..p {
                    let root = m.s3(mx(s), mx(t), x3) == 0;
                    let expect = (!plus.inf && mx(plus) == x3) || (!minus.inf && mx(minus) == x3);
                    assert_eq!(root, expect, "x1={} x2={} x3={x3}", mx(s), mx(t));
                }
            }
        }
    }
}
