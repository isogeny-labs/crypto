//! `volcano-gap` — two deep 2-volcanoes on either side of a large prime
//! conductor gap, joined by an explicit, verified degree-`N` isogeny.
//!
//! The class: `t² − 4p = D₀·(2^d·N)²` with `D₀` a class-number-one
//! fundamental discriminant, `N` a large prime and `d` the depth of the
//! 2-volcano.  It splits into
//!
//! - `S1`, the curves whose endomorphism conductor is a power of 2 (one
//!   2-volcano, `j(D₀)` on its crater), and
//! - `S2`, the curves whose conductor is divisible by `N` (`h(O_N)` further
//!   2-volcanoes of the same shape),
//!
//! and every isogeny between `S1` and `S2` has degree divisible by `N`
//! (Kohel).  The search also asks for `t ≡ −2 (mod N)`.  Then Frobenius acts
//! as `−1` on the `N`-torsion of every `S1` curve, so all `N + 1` kernels of
//! degree `N` are rational on the quadratic twist, and the cross-gap isogeny
//! is Vélu's formula over `F_p` with `(N − 1)/2` kernel abscissae: `O(N)`
//! field operations, against `√p` for rho.  On an `S2` curve Frobenius is a
//! nontrivial Jordan block mod `N`, so its twist has a cyclic `N`-part.  That
//! is the side certificate: the twist's `N`-torsion has rank 2 on `S1` and
//! rank 1 on `S2`.
//!
//!   volcano-gap construct --bits B --depth D [--d0 11] [--max-cofactor H] [--seed S] --out pair.json
//!   volcano-gap certify --pair pair.json --curves S1.jsonl,S2.jsonl --out certificates.json
//!   volcano-gap spec --pair pair.json --curves S1.jsonl,S2.jsonl --targets T --seed S --out DIR
//!                    [--aa-targets T2] [--bsgs-targets T3]
//!
//! `construct` is deterministic in `(B, D, D₀, H)`: it walks `N` down from
//! the largest admissible prime and takes the first accepted candidate.
//! `--seed` drives only the random points used for proofs and the instance.

use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

// ---------------------------------------------------------------- F_p, p < 2^62

#[derive(Clone, Copy)]
struct Fp {
    p: u64,
}

impl Fp {
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
    fn mul(&self, a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % self.p as u128) as u64
    }
    fn sm(&self, k: u64, a: u64) -> u64 {
        self.mul(k % self.p, a)
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
        inv_mod(a, self.p)
    }
    fn reduce_i128(&self, v: i128) -> u64 {
        v.rem_euclid(self.p as i128) as u64
    }
    fn is_square(&self, a: u64) -> bool {
        a == 0 || self.pow(a, (self.p - 1) / 2) == 1
    }
    /// Tonelli–Shanks; `None` for a non-residue.
    fn sqrt(&self, a: u64) -> Option<u64> {
        if a == 0 {
            return Some(0);
        }
        if !self.is_square(a) {
            return None;
        }
        if self.p % 4 == 3 {
            return Some(self.pow(a, (self.p + 1) / 4));
        }
        let s = (self.p - 1).trailing_zeros();
        let q = (self.p - 1) >> s;
        let mut z = 2;
        while self.is_square(z) {
            z += 1;
        }
        let (mut m, mut c) = (s, self.pow(z, q));
        let (mut t, mut r) = (self.pow(a, q), self.pow(a, q.div_ceil(2)));
        while t != 1 {
            let (mut i, mut t2) = (0, t);
            while t2 != 1 {
                t2 = self.mul(t2, t2);
                i += 1;
            }
            let b = self.pow(c, 1 << (m - i - 1));
            m = i;
            c = self.mul(b, b);
            t = self.mul(t, c);
            r = self.mul(r, b);
        }
        Some(r)
    }
    /// The least quadratic non-residue.
    fn non_residue(&self) -> u64 {
        (2..).find(|&c| !self.is_square(c)).unwrap()
    }
}

/// `a⁻¹ mod m` by word-sized extended Euclid: coefficient magnitudes with
/// alternating signs, one hardware division per step (as `ic_boundary::invmod`).
fn inv_mod(a: u64, m: u64) -> u64 {
    let (mut r0, mut r1) = (m, a % m);
    let (mut t0, mut t1) = (0u64, 1u64);
    let mut neg0 = true;
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 + q * t1);
        neg0 = !neg0;
    }
    assert_eq!(r0, 1, "not invertible");
    if neg0 {
        m - t0
    } else {
        t0
    }
}

/// Montgomery arithmetic modulo an odd `p < 2^62`, `R = 2^64`: REDC needs
/// two word products and no division, against the 128-bit remainder (a
/// library call) of `Fp::mul`.  Used for the degree-`N` isogeny's inner
/// loops; values cross in and out with `to`/`from`.
#[derive(Clone, Copy)]
struct Mont {
    p: u64,
    /// `−p⁻¹ mod 2^64`.
    pinv: u64,
    /// `R² mod p`.
    r2: u64,
}

impl Mont {
    fn new(p: u64) -> Mont {
        let mut inv = 1u64;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(inv)));
        }
        let r = ((1u128 << 64) % p as u128) as u64;
        Mont {
            p,
            pinv: inv.wrapping_neg(),
            r2: ((r as u128 * r as u128) % p as u128) as u64,
        }
    }
    #[inline]
    fn redc(&self, t: u128) -> u64 {
        let m = (t as u64).wrapping_mul(self.pinv);
        let u = ((t + m as u128 * self.p as u128) >> 64) as u64;
        if u >= self.p {
            u - self.p
        } else {
            u
        }
    }
    #[inline]
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.redc(a as u128 * b as u128)
    }
    #[inline]
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline]
    fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    fn to(&self, a: u64) -> u64 {
        self.mul(a % self.p, self.r2)
    }
    fn from(&self, a: u64) -> u64 {
        self.redc(a as u128)
    }
    /// The inverse of a Montgomery-form value, in Montgomery form.
    fn inv(&self, a: u64) -> u64 {
        self.to(inv_mod(self.from(a), self.p))
    }
    /// Replaces each value by its inverse with one inversion
    /// (Montgomery's trick); every value must be nonzero.
    fn batch_inv(&self, v: &mut [u64]) {
        let mut pre = Vec::with_capacity(v.len());
        let mut acc = self.to(1);
        for &x in v.iter() {
            pre.push(acc);
            acc = self.mul(acc, x);
        }
        let mut inv = self.inv(acc);
        for i in (0..v.len()).rev() {
            let x = v[i];
            v[i] = self.mul(inv, pre[i]);
            inv = self.mul(inv, x);
        }
    }
}

/// Worker threads for the isogeny's inner loops.
fn threads() -> usize {
    std::thread::available_parallelism().map_or(1, |n| n.get())
}

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}

/// Deterministic Miller–Rabin for `n < 2^64`.
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n.is_multiple_of(q) {
            return n == q;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

/// The largest prime factor of `n < 2^64`: trial division to 2^16, then
/// Pollard–Brent on what is left.
fn largest_prime_factor(mut n: u64) -> u64 {
    let mut best = 1;
    let mut d = 2u64;
    while d < 1 << 16 && d * d <= n {
        while n.is_multiple_of(d) {
            n /= d;
            best = d;
        }
        d += if d == 2 { 1 } else { 2 };
    }
    let mut stack = vec![n];
    while let Some(m) = stack.pop() {
        if m == 1 {
            continue;
        }
        if is_prime(m) {
            best = best.max(m);
            continue;
        }
        let g = brent(m);
        stack.push(g);
        stack.push(m / g);
    }
    best
}

fn brent(n: u64) -> u64 {
    for c in 1u64.. {
        let f = |x: u64| (mulmod(x, x, n) + c) % n;
        let (mut x, mut y, mut g) = (2u64, 2u64, 1u64);
        while g == 1 {
            x = f(x);
            y = f(f(y));
            g = gcd(x.abs_diff(y), n);
        }
        if g != n {
            return g;
        }
    }
    unreachable!()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn isqrt(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let mut x = (n as f64).sqrt() as u128;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

fn vl(mut x: u64, l: u64) -> u32 {
    let mut v = 0;
    while x > 0 && x.is_multiple_of(l) {
        x /= l;
        v += 1;
    }
    v
}

// ---------------------------------------------------------------- curves

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Curve {
    a: u64,
    b: u64,
}

type Pt = Option<(u64, u64)>;

impl Curve {
    fn rhs(&self, f: &Fp, x: u64) -> u64 {
        f.add(f.mul(f.add(f.mul(x, x), self.a), x), self.b)
    }
    fn on(&self, f: &Fp, p: Pt) -> bool {
        p.is_none_or(|(x, y)| f.mul(y, y) == self.rhs(f, x))
    }
    fn j(&self, f: &Fp) -> Option<u64> {
        let a3 = f.sm(4, f.mul(f.mul(self.a, self.a), self.a));
        let den = f.add(a3, f.sm(27, f.mul(self.b, self.b)));
        (den != 0).then(|| f.mul(f.sm(1728, a3), f.inv(den)))
    }
    fn add(&self, f: &Fp, p: Pt, q: Pt) -> Pt {
        let (Some((x1, y1)), Some((x2, y2))) = (p, q) else {
            return p.or(q);
        };
        let l = if x1 == x2 {
            if f.add(y1, y2) == 0 {
                return None;
            }
            f.mul(f.add(f.sm(3, f.mul(x1, x1)), self.a), f.inv(f.add(y1, y1)))
        } else {
            f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1)))
        };
        let x3 = f.sub(f.sub(f.mul(l, l), x1), x2);
        Some((x3, f.sub(f.mul(l, f.sub(x1, x3)), y1)))
    }
    fn mul(&self, f: &Fp, p: Pt, k: u64) -> Pt {
        let (mut r, mut b, mut k) = (None, p, k);
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(f, r, b);
            }
            b = self.add(f, b, b);
            k >>= 1;
        }
        r
    }
    fn random_point(&self, f: &Fp, rng: &mut SmallRng) -> (u64, u64) {
        loop {
            let x = rng.gen_range(0..f.p);
            if let Some(y) = f.sqrt(self.rhs(f, x)) {
                if y != 0 {
                    return (x, y);
                }
            }
        }
    }
    /// The quadratic twist by `c`: `y² = x³ + c²a·x + c³b`.  Its points
    /// `(x, y)` are the points `(x/c, y/c^{3/2})` of this curve over `F_p²`.
    fn twist(&self, f: &Fp, c: u64) -> Curve {
        Curve {
            a: f.mul(f.mul(c, c), self.a),
            b: f.mul(f.mul(f.mul(c, c), c), self.b),
        }
    }
}

/// `#E = h·n` proved: a point `P ≠ O` with `[n]P = O`, `n` prime, has
/// order `n`; with `n > 4√p` the Hasse interval holds one multiple of `n`.
fn prove_order(f: &Fp, e: &Curve, h: u64, n: u64, rng: &mut SmallRng) -> Option<(u64, u64)> {
    assert!(
        (n as u128) * (n as u128) > 16 * f.p as u128,
        "n must exceed 4·sqrt(p)"
    );
    for _ in 0..64 {
        let r = e.random_point(f, rng);
        let pt = e.mul(f, Some(r), h);
        if let Some(pt) = pt {
            return e.mul(f, Some(pt), n).is_none().then_some(pt);
        }
    }
    None
}

/// Rank of the `N`-part of `E(F_p)` when `N² | #E`: 1 if some point has an
/// `N`-component of full order (one witness proves it), otherwise 2 with
/// error at most `N^(−trials)`.
fn n_rank(
    f: &Fp,
    e: &Curve,
    order: u64,
    big_n: u64,
    trials: usize,
    rng: &mut SmallRng,
) -> (u32, Option<(u64, u64)>) {
    let cof = order / big_n;
    for _ in 0..trials {
        let r = e.random_point(f, rng);
        if e.mul(f, Some(r), cof).is_some() {
            return (1, Some(r));
        }
    }
    (2, None)
}

/// A point of order exactly `N` on `E` (`N | #E`).
fn point_of_order(f: &Fp, e: &Curve, order: u64, big_n: u64, rng: &mut SmallRng) -> (u64, u64) {
    let v = vl(order, big_n);
    let m = order / big_n.pow(v);
    loop {
        let r = e.random_point(f, rng);
        let Some(mut s) = e.mul(f, Some(r), m) else {
            continue;
        };
        while let Some(t) = e.mul(f, Some(s), big_n) {
            s = t;
        }
        return s;
    }
}

// ---------------------------------------------------------------- Vélu, odd degree

/// Vélu's isogeny of odd degree `ℓ` from `(ℓ − 1)/2` kernel abscissae in
/// `F_p` (the points themselves may lie over `F_p²`).
struct Velu {
    ell: u64,
    xs: Vec<u64>,
    /// Plain forms of `tq`, `uq`: read by `map_reference` in the tests.
    #[cfg_attr(not(test), allow(dead_code))]
    tq: Vec<u64>,
    #[cfg_attr(not(test), allow(dead_code))]
    uq: Vec<u64>,
    /// `xs`, `tq`, `uq` in Montgomery form, for [`Velu::map`].
    mont: Mont,
    xs_m: Vec<u64>,
    tq_m: Vec<u64>,
    uq_m: Vec<u64>,
    codomain: Curve,
}

/// Field multiplications charged, approximately: one per `Fp::mul`, three
/// per element of a batch inversion, and 128 per inversion.
#[derive(Default, Clone, Copy)]
struct Cost {
    mults: u64,
}

impl Velu {
    fn new(f: &Fp, e: &Curve, ell: u64, xs: Vec<u64>, cost: &mut Cost) -> Velu {
        assert_eq!(xs.len() as u64, (ell - 1) / 2);
        let (mut v, mut w) = (0u64, 0u64);
        let mut tq = Vec::with_capacity(xs.len());
        let mut uq = Vec::with_capacity(xs.len());
        for &x in &xs {
            let gx = f.add(f.sm(3, f.mul(x, x)), e.a);
            let t = f.add(gx, gx);
            let u = f.sm(4, e.rhs(f, x));
            v = f.add(v, t);
            w = f.add(w, f.add(u, f.mul(x, t)));
            tq.push(t);
            uq.push(u);
        }
        cost.mults += 7 * xs.len() as u64;
        let mont = Mont::new(f.p);
        let conv = |v: &[u64]| v.iter().map(|&a| mont.to(a)).collect::<Vec<u64>>();
        let (xs_m, tq_m, uq_m) = (conv(&xs), conv(&tq), conv(&uq));
        cost.mults += 3 * xs.len() as u64;
        Velu {
            ell,
            xs,
            tq,
            uq,
            mont,
            xs_m,
            tq_m,
            uq_m,
            codomain: Curve {
                a: f.sub(e.a, f.sm(5, v)),
                b: f.sub(e.b, f.sm(7, w)),
            },
        }
    }
    /// `φ(x, y) = (x + Σ t/(x−x_Q) + u/(x−x_Q)², y·(1 − Σ t/(x−x_Q)² + 2u/(x−x_Q)³))`,
    /// in Montgomery form, the kernel split into one chunk per thread, each
    /// with one batch inversion of its `x − x_Q`.
    fn map(&self, f: &Fp, p: Pt, cost: &mut Cost) -> Pt {
        let (x, y) = p?;
        if self.xs.contains(&x) {
            return None;
        }
        let m = &self.mont;
        let xm = m.to(x);
        let n = self.xs_m.len();
        let chunks = threads().min(n.div_ceil(4096)).max(1);
        let step = n.div_ceil(chunks);
        let partial = |lo: usize, hi: usize| -> (u64, u64) {
            let mut d: Vec<u64> = self.xs_m[lo..hi].iter().map(|&xq| m.sub(xm, xq)).collect();
            m.batch_inv(&mut d);
            let (mut sx, mut sy) = (0u64, 0u64);
            for (k, &i1) in d.iter().enumerate() {
                let (t, u) = (self.tq_m[lo + k], self.uq_m[lo + k]);
                let i2 = m.mul(i1, i1);
                let i3 = m.mul(i2, i1);
                let u2 = m.mul(u, i2);
                sx = m.add(sx, m.add(m.mul(t, i1), u2));
                let u3 = m.mul(u, i3);
                sy = m.add(sy, m.add(m.mul(t, i2), m.add(u3, u3)));
            }
            (sx, sy)
        };
        let parts: Vec<(u64, u64)> = if chunks == 1 {
            vec![partial(0, n)]
        } else {
            std::thread::scope(|sc| {
                let hs: Vec<_> = (0..chunks)
                    .map(|c| {
                        let (lo, hi) = (c * step, ((c + 1) * step).min(n));
                        sc.spawn(move || partial(lo, hi))
                    })
                    .collect();
                hs.into_iter().map(|h| h.join().unwrap()).collect()
            })
        };
        let (sx, sy) = parts
            .iter()
            .fold((0, 0), |(a, b), &(c, d)| (m.add(a, c), m.add(b, d)));
        cost.mults += 3 * n as u64 + 128 * chunks as u64 + 6 * n as u64 + 4;
        let (sx, sy) = (m.from(sx), m.from(sy));
        Some((f.add(x, sx), f.sub(y, f.mul(y, sy))))
    }

    /// The map as first written (plain `Fp`, one batch inversion), kept as
    /// the reference the Montgomery form is tested against.
    #[cfg(test)]
    fn map_reference(&self, f: &Fp, p: Pt) -> Pt {
        let (x, y) = p?;
        let d: Vec<u64> = self.xs.iter().map(|&xq| f.sub(x, xq)).collect();
        if d.contains(&0) {
            return None;
        }
        let mut pre = Vec::with_capacity(d.len());
        let mut acc = 1u64;
        for &di in &d {
            pre.push(acc);
            acc = f.mul(acc, di);
        }
        let mut inv = f.inv(acc);
        let mut invs = vec![0u64; d.len()];
        for i in (0..d.len()).rev() {
            invs[i] = f.mul(inv, pre[i]);
            inv = f.mul(inv, d[i]);
        }
        let (mut sx, mut sy) = (0u64, 0u64);
        for i in 0..d.len() {
            let i1 = invs[i];
            let i2 = f.mul(i1, i1);
            let i3 = f.mul(i2, i1);
            sx = f.add(sx, f.add(f.mul(self.tq[i], i1), f.mul(self.uq[i], i2)));
            sy = f.add(
                sy,
                f.add(f.mul(self.tq[i], i2), f.sm(2, f.mul(self.uq[i], i3))),
            );
        }
        Some((f.add(x, sx), f.sub(y, f.mul(y, sy))))
    }
}

/// The kernel abscissae on `E` of `⟨K⟩`, `K` a point of order `N` on the
/// twist `E^c`: `x(iK)/c` for `i = 1 … (N − 1)/2`.  The multiples are
/// accumulated in Jacobian coordinates by mixed additions of `K` (no
/// inversion per step: `jK ≠ ±K` for `2 ≤ j < N − 1`), then normalised
/// with one batch inversion of the `Z²`, all in Montgomery form.
fn kernel_xs(
    f: &Fp,
    twist: &Curve,
    k: (u64, u64),
    c: u64,
    big_n: u64,
    cost: &mut Cost,
) -> Vec<u64> {
    let m = Mont::new(f.p);
    let count = ((big_n - 1) / 2) as usize;
    let mut xs = Vec::with_capacity(count);
    if count == 0 {
        return xs;
    }
    let (kx, ky) = (m.to(k.0), m.to(k.1));
    let one = m.to(1);
    let mut jx = Vec::with_capacity(count);
    let mut jz2 = Vec::with_capacity(count);
    jx.push(kx);
    jz2.push(one);
    if count >= 2 {
        let (x2, y2) = twist
            .add(f, Some(k), Some(k))
            .expect("2K vanished: N would be 2");
        let (mut x1, mut y1, mut z1) = (m.to(x2), m.to(y2), one);
        jx.push(x1);
        jz2.push(one);
        for _ in 3..=count {
            // madd-2007-bl: (X1:Y1:Z1) + (kx, ky).
            let z1z1 = m.mul(z1, z1);
            let u2 = m.mul(kx, z1z1);
            let s2 = m.mul(m.mul(ky, z1), z1z1);
            let h = m.sub(u2, x1);
            let hh = m.mul(h, h);
            let i4 = m.add(m.add(hh, hh), m.add(hh, hh));
            let j = m.mul(h, i4);
            let r = m.add(m.sub(s2, y1), m.sub(s2, y1));
            let v = m.mul(x1, i4);
            let x3 = m.sub(m.sub(m.mul(r, r), j), m.add(v, v));
            let y1j = m.mul(y1, j);
            let y3 = m.sub(m.mul(r, m.sub(v, x3)), m.add(y1j, y1j));
            let zh = m.add(z1, h);
            let z3 = m.sub(m.sub(m.mul(zh, zh), z1z1), hh);
            (x1, y1, z1) = (x3, y3, z3);
            jx.push(x1);
            jz2.push(m.mul(z1, z1));
        }
    }
    m.batch_inv(&mut jz2);
    let cinv = m.to(f.inv(c));
    for (&x, &iz2) in jx.iter().zip(&jz2) {
        xs.push(m.from(m.mul(m.mul(x, iz2), cinv)));
    }
    // A mixed addition is 7M + 4S; then Z², the batch inversion's three
    // and the two products of the normalisation.
    cost.mults += count as u64 * (11 + 1 + 3 + 2) + 128 * 2;
    xs
}

/// The affine enumeration first written, kept as the reference the
/// Jacobian form is tested against.
#[cfg(test)]
fn kernel_xs_reference(f: &Fp, twist: &Curve, k: (u64, u64), c: u64, big_n: u64) -> Vec<u64> {
    let cinv = f.inv(c);
    let mut xs = Vec::with_capacity(((big_n - 1) / 2) as usize);
    let mut cur = Some(k);
    for i in 1..=(big_n - 1) / 2 {
        let (x, _) = cur.expect("a multiple of K below N vanished");
        xs.push(f.mul(x, cinv));
        if i < (big_n - 1) / 2 {
            cur = twist.add(f, cur, Some(k));
        }
    }
    xs
}

// ---------------------------------------------------------------- CM data

/// `j(D₀)` for the class-number-one discriminants with `D₀ ≡ 1 (mod 4)`
/// and `D₀ ≠ −3` (no extra automorphisms anywhere in the class).
fn cm_j(d0: u64) -> Option<i128> {
    Some(match d0 {
        7 => -3375,
        11 => -32768,
        19 => -884736,
        43 => -884736000,
        67 => -147197952000,
        163 => -262537412640768000,
        _ => return None,
    })
}

struct Candidate {
    p: u64,
    t: i64,
    big_n: u64,
    order: u64,
    n: u64,
    h: u64,
    examined: u64,
}

/// `p = t'² + |D₀|·4^{d−1}·N²`, `t = 2t'`, `t' ≡ −1 (mod N)`, `p` prime in
/// `[2^{B−1}, 2^B)`, `#E = p + 1 − t = h·n` with `n` prime and `h ≤ H`.
fn search(bits: u32, depth: u32, d0: u64, max_h: u64) -> Candidate {
    let lo = 1u128 << (bits - 1);
    let hi = 1u128 << bits;
    let c = d0 as u128 * 4u128.pow(depth - 1);
    let mut big_n = isqrt((hi - 1) / c) as u64;
    let mut examined = 0u64;
    while big_n > 3 {
        if is_prime(big_n) && big_n != d0 {
            let base = c * big_n as u128 * big_n as u128;
            let tmax = isqrt(hi - 1 - base.min(hi - 1)) as i128;
            // t' = mN − 1 over m in order of |t'|: smallest traces first.
            let mut ts: Vec<i128> = Vec::new();
            let mmax = tmax / big_n as i128 + 2;
            for m in -mmax..=mmax {
                let tp = m * big_n as i128 - 1;
                if tp.abs() <= tmax && tp.rem_euclid(2) == 1 {
                    ts.push(tp);
                }
            }
            ts.sort_by_key(|t| (t.abs(), *t));
            for tp in ts {
                examined += 1;
                let p = (tp * tp) as u128 + base;
                if p < lo || p >= hi || !is_prime(p as u64) {
                    continue;
                }
                let p = p as u64;
                let order = (p as i128 + 1 - 2 * tp) as u64;
                let n = largest_prime_factor(order);
                let h = order / n;
                if h > max_h || n == 2 || n == big_n || (n as u128).pow(2) <= 16 * p as u128 {
                    continue;
                }
                return Candidate {
                    p,
                    t: (2 * tp) as i64,
                    big_n,
                    order,
                    n,
                    h,
                    examined,
                };
            }
        }
        big_n -= 1;
    }
    die("no candidate: lower --depth or raise --bits")
}

fn die(msg: &str) -> ! {
    eprintln!("volcano-gap: {msg}");
    std::process::exit(2)
}

fn get(argv: &[String], k: &str) -> Option<String> {
    argv.iter()
        .position(|a| a == k)
        .and_then(|i| argv.get(i + 1))
        .cloned()
}

fn num<T: std::str::FromStr>(argv: &[String], k: &str, default: Option<T>) -> T {
    match get(argv, k) {
        Some(v) => v
            .parse()
            .unwrap_or_else(|_| die(&format!("{k} takes a number"))),
        None => default.unwrap_or_else(|| die(&format!("{k} is required"))),
    }
}

// ---------------------------------------------------------------- construct

fn construct(argv: &[String]) {
    let bits: u32 = num(argv, "--bits", Some(42));
    let depth: u32 = num(argv, "--depth", Some(6));
    let d0: u64 = num(argv, "--d0", Some(11));
    let max_h: u64 = num(argv, "--max-cofactor", Some(64));
    let seed: u64 = num(argv, "--seed", Some(20261022));
    let out = PathBuf::from(get(argv, "--out").unwrap_or_else(|| die("--out is required")));
    if !(20..=61).contains(&bits) || depth < 1 {
        die("--bits must be in 20..=61 and --depth at least 1");
    }
    let Some(j_cm) = cm_j(d0) else {
        die("--d0 must be one of 7, 11, 19, 43, 67, 163")
    };
    let mut rng = SmallRng::seed_from_u64(seed);

    let t0 = Instant::now();
    let cand = search(bits, depth, d0, max_h);
    let search_s = t0.elapsed().as_secs_f64();
    let (p, t, big_n, order, n, h) = (cand.p, cand.t, cand.big_n, cand.order, cand.n, cand.h);
    let f = Fp { p };
    let fcond = (1u64 << depth) * big_n;
    let disc = t as i128 * t as i128 - 4 * p as i128;
    assert_eq!(
        disc,
        -(d0 as i128) * (fcond as i128).pow(2),
        "t² − 4p ≠ D₀ f²"
    );
    assert_eq!((t as i128 + 2).rem_euclid(big_n as i128), 0, "t ≢ −2 mod N");
    eprintln!(
        "class: p={p} t={t} t^2-4p = -{d0}*(2^{depth}*{big_n})^2  #E={order} = {h}*{n}  ({} candidates, {search_s:.2}s)",
        cand.examined
    );

    // The crater curve of S1: j = j(D₀), in the twist with trace t.
    let j1 = f.reduce_i128(j_cm);
    let k = f.mul(j1, f.inv(f.sub(1728 % p, j1)));
    let base = Curve {
        a: f.sm(3, k),
        b: f.sm(2, k),
    };
    let c = f.non_residue();
    let (e1, p1) = match prove_order(&f, &base, h, n, &mut rng) {
        Some(pt) => (base, pt),
        None => {
            let tw = base.twist(&f, c);
            let pt = prove_order(&f, &tw, h, n, &mut rng)
                .unwrap_or_else(|| die("neither j(D0) twist has order h·n"));
            (tw, pt)
        }
    };
    assert_eq!(e1.j(&f), Some(j1));
    // The order is the unique multiple of n in the Hasse interval.
    let lo = p as i128 + 1 - 2 * (isqrt(p as u128) as i128 + 1);
    let mult = (lo + n as i128 - 1) / n as i128 * n as i128;
    assert_eq!(
        mult as u64, order,
        "order proof: the multiple of n is not h·n"
    );
    assert!(mult + (n as i128) > p as i128 + 1 + 2 * (isqrt(p as u128) as i128 + 1));

    // Twist orders: N² divides #E^t = ((t+2)² − f²D₀)/4.
    let order_t = 2 * p + 2 - order;
    let v_t = vl(order_t, big_n);
    assert!(v_t >= 2, "N^2 does not divide the twist order");
    assert_eq!(order % big_n, 4 % big_n, "#E mod N should be 4");

    // Side certificate of the crater: full N-torsion on the twist.
    let e1t = e1.twist(&f, c);
    let (rank1, _) = n_rank(&f, &e1t, order_t, big_n, 16, &mut rng);
    assert_eq!(rank1, 2, "the j(D0) crater must lie on S1");

    // The descending N-isogeny: a random line of E1[N] (twist-rational);
    // a horizontal line returns to j(D₀), so draw again.
    let mut construct_cost = Cost::default();
    let t_iso = Instant::now();
    let mut draws = 0u32;
    let (kt, phi) = loop {
        draws += 1;
        let kt = point_of_order(&f, &e1t, order_t, big_n, &mut rng);
        let xs = kernel_xs(&f, &e1t, kt, c, big_n, &mut construct_cost);
        let phi = Velu::new(&f, &e1, big_n, xs, &mut construct_cost);
        match phi.codomain.j(&f) {
            Some(j2) if j2 != j1 => break (kt, phi),
            Some(_) => continue,
            None => die("singular Vélu codomain"),
        }
    };
    let construct_s = t_iso.elapsed().as_secs_f64();
    let e2 = phi.codomain;
    let j2 = e2.j(&f).unwrap();

    // Verify φ on the generator and on random pairs.
    let mut eval_cost = Cost::default();
    let t_eval = Instant::now();
    let p2 = phi.map(&f, Some(p1), &mut eval_cost).expect("φ(P) = O");
    let eval_s = t_eval.elapsed().as_secs_f64();
    let per_point_mults = eval_cost.mults;
    assert!(e2.on(&f, Some(p2)), "φ(P) off the codomain");
    assert!(e2.mul(&f, Some(p2), n).is_none(), "nφ(P) ≠ O");
    let mut hom_checks = 0;
    for _ in 0..8 {
        let r = Some(e1.random_point(&f, &mut rng));
        let s = Some(e1.random_point(&f, &mut rng));
        let (fr, fs) = (
            phi.map(&f, r, &mut eval_cost),
            phi.map(&f, s, &mut eval_cost),
        );
        let frs = phi.map(&f, e1.add(&f, r, s), &mut eval_cost);
        assert!(e2.on(&f, fr) && e2.on(&f, fs), "image off the codomain");
        assert_eq!(e2.add(&f, fr, fs), frs, "φ is not a homomorphism");
        hom_checks += 1;
    }
    // #E2 = #E1 (isogenous), checked on random points.
    for _ in 0..8 {
        let r = e2.random_point(&f, &mut rng);
        assert!(e2.mul(&f, Some(r), order).is_none(), "#E2 ≠ #E1");
    }
    let p2_order = prove_order(&f, &e2, h, n, &mut rng).is_some();
    assert!(p2_order);
    // Side certificate of the codomain: cyclic N-part of its twist.
    let e2t = e2.twist(&f, c);
    let (rank2, witness2) = n_rank(&f, &e2t, order_t, big_n, 16, &mut rng);
    assert_eq!(rank2, 1, "the codomain must lie on S2");

    // The dual direction: E2's one twist-rational N-line ascends to j(D₀),
    // and ψ∘φ = ±[N] after the model isomorphism.
    let mut dual_cost = Cost::default();
    let k2 = point_of_order(&f, &e2t, order_t, big_n, &mut rng);
    let xs2 = kernel_xs(&f, &e2t, k2, c, big_n, &mut dual_cost);
    let psi = Velu::new(&f, &e2, big_n, xs2, &mut dual_cost);
    let e1p = psi.codomain;
    assert_eq!(
        e1p.j(&f),
        Some(j1),
        "the ascending N-isogeny does not reach j(D0)"
    );
    // E1' = (u⁴a, u⁶b): u² = b'a/(a'b).
    let u2 = f.mul(f.mul(e1p.b, e1.a), f.inv(f.mul(e1p.a, e1.b)));
    let back = phi.map(&f, Some(p1), &mut dual_cost);
    let pp = psi.map(&f, back, &mut dual_cost).expect("ψφ(P) = O");
    let np = e1.mul(&f, Some(p1), big_n).unwrap();
    assert_eq!(f.mul(pp.0, f.inv(u2)), np.0, "ψ∘φ ≠ ±[N] on P");
    let mut dual_checks = 1;
    for _ in 0..4 {
        let r = Some(e1.random_point(&f, &mut rng));
        let q = psi.map(&f, phi.map(&f, r, &mut dual_cost), &mut dual_cost);
        let nr = e1.mul(&f, r, big_n);
        assert_eq!(
            q.map(|q| f.mul(q.0, f.inv(u2))),
            nr.map(|q| q.0),
            "ψ∘φ ≠ ±[N]"
        );
        dual_checks += 1;
    }

    let sqrt_p = (p as f64).sqrt();
    let rho_neg = (std::f64::consts::PI * n as f64 / 4.0).sqrt();
    let out_v = json!({
        "schema": "volcano-gap/pair/v1",
        "command": std::env::args().collect::<Vec<_>>().join(" "),
        "params": {"bits": bits, "depth": depth, "d0": -(d0 as i64), "max_cofactor": max_h, "seed": seed},
        "class": {
            "p": p, "t": t, "disc": format!("{disc}"), "fundamental_disc": -(d0 as i64),
            "frobenius_conductor": fcond, "two_depth": depth, "gap_prime": big_n,
            "gap_prime_over_p_quarter": big_n as f64 / sqrt_p.sqrt(),
            "log_p_gap_prime": (big_n as f64).ln() / (p as f64).ln(),
            "order": order, "cofactor": h, "n": n, "twist_order": order_t, "v_N_twist_order": v_t,
            "order_proof": "a point P != O with [n]P = O, n prime, n > 4 sqrt(p): the Hasse interval holds one multiple of n, and it is h*n",
            "t_plus_2_mod_N": 0, "order_mod_N": order % big_n,
            "search_candidates_examined": cand.examined, "search_s": search_s,
            "expected_levels_per_side": (0..=depth).map(|l| if l == 0 { 1 } else { 3u64 << (l - 1) }).collect::<Vec<_>>(),
            "rho_negation_expected_gae": rho_neg,
        },
        "twist_nonresidue": c,
        "s1_root": {"side": "S1", "conductor": 1, "a": e1.a, "b": e1.b, "j": j1,
                    "generator": [p1.0, p1.1], "twist_n_rank": rank1},
        "s2_root": {"side": "S2", "conductor": big_n, "a": e2.a, "b": e2.b, "j": j2,
                    "generator": [p2.0, p2.1], "twist_n_rank": rank2,
                    "twist_cyclic_witness": witness2.map(|w| [w.0, w.1])},
        "isogeny": {
            "degree": phi.ell, "direction": "descending S1 -> S2",
            "kernel": "<K>, K of order N on the quadratic twist by the non-residue, abscissae x(iK)/c for i = 1..(N-1)/2",
            "kernel_generator_on_twist": [kt.0, kt.1], "kernel_draws": draws,
            "construct_field_mults": construct_cost.mults, "construct_s": construct_s,
            "eval_field_mults_per_point": per_point_mults, "eval_s_per_point": eval_s,
            "transfer_field_mults_per_instance": 2 * per_point_mults,
            "homomorphism_checks": hom_checks, "generator_order_preserved": true,
            "codomain_order_checks": 8,
        },
        "dual": {
            "degree": psi.ell, "direction": "ascending S2 -> S1",
            "kernel_generator_on_twist": [k2.0, k2.1], "codomain_j_is_j_d0": true, "u_squared": u2,
            "psi_phi_is_plus_minus_N_checks": dual_checks, "field_mults": dual_cost.mults,
        },
    });
    if let Some(dir) = out.parent() {
        fs::create_dir_all(dir).ok();
    }
    fs::write(&out, serde_json::to_string_pretty(&out_v).unwrap()).unwrap();
    eprintln!(
        "S1 root: a={} b={} j={j1} | S2 root (phi codomain): a={} b={} j={j2}\nN-isogeny: {} draws, construct {:.3}s ({} mults), eval {:.4}s/point ({} mults); dual reaches j(D0); psi.phi = ±[N] on {} points",
        e1.a, e1.b, e2.a, e2.b, draws, construct_s, construct_cost.mults, eval_s, per_point_mults, dual_checks
    );
    println!("{}", out.display());
}

// ---------------------------------------------------------------- certify

fn read_lines(path: &str) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap_or_else(|e| die(&format!("{path}: {e}")))
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

fn pair_of(argv: &[String]) -> Value {
    let path = get(argv, "--pair").unwrap_or_else(|| die("--pair is required"));
    serde_json::from_str(
        &fs::read_to_string(&path).unwrap_or_else(|e| die(&format!("{path}: {e}"))),
    )
    .unwrap()
}

fn sides(argv: &[String]) -> Vec<(String, Vec<Value>)> {
    let list =
        get(argv, "--curves").unwrap_or_else(|| die("--curves S1.jsonl,S2.jsonl is required"));
    let files: Vec<&str> = list.split(',').collect();
    if files.len() != 2 {
        die("--curves takes exactly two files, S1 then S2");
    }
    vec![
        ("S1".to_string(), read_lines(files[0])),
        ("S2".to_string(), read_lines(files[1])),
    ]
}

fn certify(argv: &[String]) {
    let pair = pair_of(argv);
    let out = get(argv, "--out").unwrap_or_else(|| die("--out is required"));
    let cl = &pair["class"];
    let u = |v: &Value| v.as_u64().unwrap();
    let (p, order, n, h, big_n, order_t) = (
        u(&cl["p"]),
        u(&cl["order"]),
        u(&cl["n"]),
        u(&cl["cofactor"]),
        u(&cl["gap_prime"]),
        u(&cl["twist_order"]),
    );
    let c = u(&pair["twist_nonresidue"]);
    let f = Fp { p };
    let mut rng = SmallRng::seed_from_u64(num(argv, "--seed", Some(20261022u64)) ^ 0xce57);
    let mut rows = Vec::new();
    let mut js: [std::collections::BTreeSet<u64>; 2] = Default::default();
    let mut bad = 0usize;
    for (si, (side, lines)) in sides(argv).iter().enumerate() {
        for l in lines {
            let e = Curve {
                a: u(&l["a"]),
                b: u(&l["b"]),
            };
            let g = (u(&l["gx"]), u(&l["gy"]));
            let on = e.on(&f, Some(g));
            let g_order = e.mul(&f, Some(g), n).is_none() && e.mul(&f, Some(g), 1).is_some();
            let killed = (0..4).all(|_| {
                e.mul(&f, Some(e.random_point(&f, &mut rng)), order)
                    .is_none()
            });
            let (rank, wit) = n_rank(&f, &e.twist(&f, c), order_t, big_n, 16, &mut rng);
            let want = if si == 0 { 2 } else { 1 };
            let ok = on && g_order && killed && rank == want;
            if !ok {
                bad += 1;
            }
            let j = e.j(&f).unwrap();
            if u(&l["model"]) == 0 {
                js[si].insert(j);
            }
            rows.push(json!({
                "side": side, "vertex": l["vertex"], "model": l["model"], "level": l["level"], "j": j,
                "generator_on_curve": on, "generator_order_n": g_order, "order_kills_random_points": killed,
                "twist_n_rank": rank, "twist_cyclic_witness": wit.map(|w| [w.0, w.1]), "ok": ok,
            }));
        }
    }
    let shared = js[0].intersection(&js[1]).count();
    let summary = json!({
        "schema": "volcano-gap/certificates/v1",
        "rule": "S1 (conductor prime to N): Frobenius = -1 on E[N], the twist has (Z/N)^2 (rank 2, error <= N^-16 per curve). S2 (N | conductor): the twist's N-part is cyclic, proved by a witness point R with [#E^t/N]R != O.",
        "p": p, "n": n, "cofactor": h, "gap_prime": big_n,
        "curves": rows.len(), "failed": bad, "distinct_j": [js[0].len(), js[1].len()],
        "j_shared_between_sides": shared, "all_ok": bad == 0 && shared == 0,
        "rows": rows,
    });
    fs::write(&out, serde_json::to_string_pretty(&summary).unwrap()).unwrap();
    eprintln!(
        "certify: {} curve models, {} failed; distinct j S1 {} / S2 {}; shared {}",
        summary["curves"],
        bad,
        js[0].len(),
        js[1].len(),
        shared
    );
    if bad > 0 || shared > 0 {
        std::process::exit(1);
    }
}

// ---------------------------------------------------------------- spec

fn spec(argv: &[String]) {
    let pair = pair_of(argv);
    let out = PathBuf::from(get(argv, "--out").unwrap_or_else(|| die("--out is required")));
    let targets: u64 = num(argv, "--targets", None);
    let aa_targets: u64 = num(argv, "--aa-targets", Some(targets));
    let bsgs_targets: u64 = num(argv, "--bsgs-targets", Some(2));
    let seed: u64 = num(argv, "--seed", Some(20261022));
    let cl = &pair["class"];
    let (order, n) = (cl["order"].as_u64().unwrap(), cl["n"].as_u64().unwrap());
    let side_lists = sides(argv);
    let curve = |l: &Value| {
        json!({"kind": "prime_explicit", "p": l["p"], "a": l["a"], "b": l["b"],
               "group_order": order, "r": n, "gx": l["gx"], "gy": l["gy"]})
    };
    let all: Vec<Value> = side_lists
        .iter()
        .flat_map(|(_, ls)| ls.iter().map(curve))
        .collect();
    let model0: Vec<Value> = side_lists
        .iter()
        .flat_map(|(_, ls)| ls.iter().filter(|l| l["model"] == 0).map(curve))
        .collect();
    let desc = format!(
        "volcano-gap: both sides of a prime conductor gap N = {} (S1: conductor 2^a, S2: conductor N*2^a), \
         depth-{} 2-volcanoes over F_{}, generators carried by verified Velu maps (one N-isogeny, then 2-isogenies), \
         planted_shared targets; written by {}",
        cl["gap_prime"], cl["two_depth"], cl["p"], std::env::args().collect::<Vec<_>>().join(" ")
    );
    let mk = |label: &str, curves: &[Value], t: u64, arms: Value, mseed: u64| {
        json!({
            "schema": "ecbench.spec/v1", "label": label, "description": desc,
            "workloads": {"curves": curves, "targets_per_curve": t, "target_seed": seed, "target_kind": "planted_shared"},
            "arms": arms,
            "measurement": {"rounds": 1, "warmup": 0, "order": "shuffle", "seed": mseed,
                            "isolation_required": "L0", "timeout_seconds": 600},
        })
    };
    fs::create_dir_all(&out).unwrap();
    let main = mk(
        "prime conductor gap: rho.negation on every curve of both sides",
        &all,
        targets,
        json!([{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}]),
        seed,
    );
    let aa = mk(
        "prime conductor gap: independent A/A rho.negation (model 0)",
        &model0,
        aa_targets,
        json!([{"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}}]),
        seed ^ 0xA_A000_0000,
    );
    let bsgs = mk(
        "prime conductor gap: BSGS exactness (model 0)",
        &model0,
        bsgs_targets,
        json!([{"name": "bsgs-neg", "role": "baseline", "method": {"id": "bsgs.negation"}}]),
        seed,
    );
    for (name, v) in [
        ("spec.json", &main),
        ("spec-aa.json", &aa),
        ("spec-bsgs.json", &bsgs),
    ] {
        fs::write(out.join(name), serde_json::to_string_pretty(v).unwrap()).unwrap();
    }
    eprintln!(
        "spec: {} workload curves x {targets} targets (A/A {} x {aa_targets}, BSGS {} x {bsgs_targets}) -> {}",
        all.len(),
        model0.len(),
        model0.len(),
        out.display()
    );
}

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    match argv.first().map(String::as_str) {
        Some("construct") => construct(&argv),
        Some("certify") => certify(&argv),
        Some("spec") => spec(&argv),
        _ => {
            println!("usage: volcano-gap construct|certify|spec ... (see the module comment in src/bin/volcano_gap.rs)");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A small class end to end: the search, the order proof, both side
    /// certificates and ψ∘φ = ±[N].
    #[test]
    fn small_gap_pair() {
        let cand = search(30, 3, 11, 64);
        let f = Fp { p: cand.p };
        assert_eq!(
            cand.t as i128 * cand.t as i128 - 4 * cand.p as i128,
            -11 * ((8 * cand.big_n) as i128).pow(2)
        );
        assert_eq!((cand.t as i128 + 2).rem_euclid(cand.big_n as i128), 0);
        assert_eq!(cand.order, cand.h * cand.n);
        let order_t = 2 * cand.p + 2 - cand.order;
        assert!(order_t.is_multiple_of(cand.big_n * cand.big_n));
        let mut rng = SmallRng::seed_from_u64(1);
        let c = f.non_residue();
        let j1 = f.reduce_i128(-32768);
        let k = f.mul(j1, f.inv(f.sub(1728, j1)));
        let base = Curve {
            a: f.sm(3, k),
            b: f.sm(2, k),
        };
        let e1 = if prove_order(&f, &base, cand.h, cand.n, &mut rng).is_some() {
            base
        } else {
            base.twist(&f, c)
        };
        let p1 = prove_order(&f, &e1, cand.h, cand.n, &mut rng).unwrap();
        let e1t = e1.twist(&f, c);
        assert_eq!(n_rank(&f, &e1t, order_t, cand.big_n, 16, &mut rng).0, 2);
        let mut cost = Cost::default();
        let phi = loop {
            let kt = point_of_order(&f, &e1t, order_t, cand.big_n, &mut rng);
            let xs = kernel_xs(&f, &e1t, kt, c, cand.big_n, &mut cost);
            let phi = Velu::new(&f, &e1, cand.big_n, xs, &mut cost);
            if phi.codomain.j(&f) != Some(j1) {
                break phi;
            }
        };
        let e2 = phi.codomain;
        assert_eq!(
            n_rank(&f, &e2.twist(&f, c), order_t, cand.big_n, 16, &mut rng).0,
            1
        );
        let p2 = phi.map(&f, Some(p1), &mut cost).unwrap();
        assert!(e2.on(&f, Some(p2)) && e2.mul(&f, Some(p2), cand.n).is_none());
        let k2 = point_of_order(&f, &e2.twist(&f, c), order_t, cand.big_n, &mut rng);
        let psi = Velu::new(
            &f,
            &e2,
            cand.big_n,
            kernel_xs(&f, &e2.twist(&f, c), k2, c, cand.big_n, &mut cost),
            &mut cost,
        );
        assert_eq!(psi.codomain.j(&f), Some(j1));
        let u2 = f.mul(
            f.mul(psi.codomain.b, e1.a),
            f.inv(f.mul(psi.codomain.a, e1.b)),
        );
        let q = psi.map(&f, Some(p2), &mut cost).unwrap();
        assert_eq!(
            f.mul(q.0, f.inv(u2)),
            e1.mul(&f, Some(p1), cand.big_n).unwrap().0
        );
    }

    /// The Montgomery/Jacobian kernel and the threaded map give exactly the
    /// values of the affine reference, on the study's own 52-bit class.
    #[test]
    fn fast_isogeny_matches_reference() {
        let cand = search(52, 6, 11, 16384);
        assert_eq!(cand.big_n, 631409);
        let f = Fp { p: cand.p };
        let mut rng = SmallRng::seed_from_u64(3);
        let c = f.non_residue();
        let j1 = f.reduce_i128(-32768);
        let k = f.mul(j1, f.inv(f.sub(1728, j1)));
        let base = Curve {
            a: f.sm(3, k),
            b: f.sm(2, k),
        };
        let e1 = if prove_order(&f, &base, cand.h, cand.n, &mut rng).is_some() {
            base
        } else {
            base.twist(&f, c)
        };
        let order_t = 2 * cand.p + 2 - cand.order;
        let e1t = e1.twist(&f, c);
        let kt = point_of_order(&f, &e1t, order_t, cand.big_n, &mut rng);
        let mut cost = Cost::default();
        let fast = kernel_xs(&f, &e1t, kt, c, cand.big_n, &mut cost);
        assert_eq!(fast, kernel_xs_reference(&f, &e1t, kt, c, cand.big_n));
        let phi = Velu::new(&f, &e1, cand.big_n, fast, &mut cost);
        for _ in 0..3 {
            let r = Some(e1.random_point(&f, &mut rng));
            assert_eq!(phi.map(&f, r, &mut cost), phi.map_reference(&f, r));
        }
    }
}
