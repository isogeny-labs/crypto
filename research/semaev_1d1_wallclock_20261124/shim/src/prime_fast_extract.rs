// Verbatim extract of crypto/src/cryptanalysis/prime_fast.rs lines 57-518 (Fp64 Montgomery layer and helpers).
use num_bigint as _;

/// Largest field size the Montgomery layer supports (`p < 2^62` keeps
/// `a + b` and `t + m·p` inside `u64` / `u128`).
pub const MAX_BITS: u32 = 62;

// ── Montgomery F_p on u64 ─────────────────────────────────────────────

/// Montgomery field context for an odd prime `p < 2^62`, `R = 2^64`.
/// Values are held as `a·R mod p`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp64 {
    pub p: u64,
    /// `−p⁻¹ mod 2^64`.
    n_inv: u64,
    /// `R mod p` — the Montgomery form of 1.
    pub one: u64,
    /// `R² mod p`.
    r2: u64,
    /// `R³ mod p` (turns a canonical inverse of a Montgomery value back
    /// into Montgomery form in one multiplication).
    r3: u64,
    /// `(p − 1) / 2`, the sign boundary for the negation map.
    pub half: u64,
}

impl Fp64 {
    pub fn new(p: u64) -> Self {
        assert!(p & 1 == 1 && p > 3, "Fp64 needs an odd prime > 3");
        assert!(p < (1u64 << MAX_BITS), "Fp64 supports p < 2^{MAX_BITS}");
        // Newton iteration for p⁻¹ mod 2^64 (each step doubles the
        // number of correct bits; 6 steps from 1 correct bit is enough).
        let mut inv: u64 = 1;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(inv)));
        }
        let n_inv = inv.wrapping_neg();
        let r = ((1u128 << 64) % p as u128) as u64;
        let r2 = ((r as u128 * r as u128) % p as u128) as u64;
        let mut f = Fp64 {
            p,
            n_inv,
            one: r,
            r2,
            r3: 0,
            half: (p - 1) / 2,
        };
        f.r3 = f.mul(r2, r2);
        f
    }

    /// `u mod p` for `u < 2p`.  A select, not a branch: on field data the
    /// comparison is a coin flip a branch predictor cannot learn (see
    /// [`crate::cryptanalysis::fp_backend::Fp64`] and
    /// `research/hwcost_round2_20261016`, addendum 1).
    #[inline(always)]
    fn csub(&self, u: u64) -> u64 {
        let (d, borrow) = u.overflowing_sub(self.p);
        std::hint::select_unpredictable(borrow, u, d)
    }

    /// Montgomery reduction of `t < p·2^64`: returns `t·R⁻¹ mod p`.
    #[inline(always)]
    pub fn reduce(&self, t: u128) -> u64 {
        let m = (t as u64).wrapping_mul(self.n_inv);
        let u = ((t + (m as u128) * (self.p as u128)) >> 64) as u64;
        self.csub(u)
    }

    #[inline(always)]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        self.reduce(a as u128 * b as u128)
    }

    #[inline(always)]
    pub fn sqr(&self, a: u64) -> u64 {
        self.mul(a, a)
    }

    #[inline(always)]
    pub fn add(&self, a: u64, b: u64) -> u64 {
        self.csub(a + b)
    }

    #[inline(always)]
    pub fn sub(&self, a: u64, b: u64) -> u64 {
        let (d, borrow) = a.overflowing_sub(b);
        std::hint::select_unpredictable(borrow, d.wrapping_add(self.p), d)
    }

    #[inline(always)]
    pub fn neg(&self, a: u64) -> u64 {
        // a == 0 is rare on data, so this branch predicts: kept.
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }

    /// Canonical → Montgomery.
    #[inline]
    pub fn to_mont(&self, a: u64) -> u64 {
        self.mul(a % self.p, self.r2)
    }

    /// Montgomery → canonical.
    #[inline(always)]
    pub fn from_mont(&self, a: u64) -> u64 {
        // Field elements in this module are stored reduced (`a < p`); the
        // fast path skips the final conditional subtract in `reduce`.
        if a < self.p {
            self.from_mont_reduced(a)
        } else {
            self.reduce(a as u128)
        }
    }

    /// Montgomery → canonical for an already-reduced operand `a < p`.
    /// Because `a <= p - 1` and `m <= 2^64 - 1`, `a + m·p <= 2^64·p - 1`,
    /// so `(a + m·p) >> 64 <= p - 1` without a final conditional subtract.
    #[inline(always)]
    #[allow(clippy::wrong_self_convention)]
    fn from_mont_reduced(&self, a: u64) -> u64 {
        debug_assert!(a < self.p);
        let m = a.wrapping_mul(self.n_inv);
        (((a as u128) + (m as u128) * (self.p as u128)) >> 64) as u64
    }

    /// Inverse of a non-zero Montgomery value, in Montgomery form.
    #[inline]
    pub fn inv(&self, a: u64) -> u64 {
        // inv_mod gives (a·R)⁻¹ = a⁻¹·R⁻¹; times R³ through one
        // Montgomery multiplication (which divides by R) is a⁻¹·R.
        let c = inv_mod_u64(a, self.p);
        self.mul(c, self.r3)
    }

    /// `a^e` for a Montgomery value `a` (result in Montgomery form).
    pub fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut acc = self.one;
        while e > 0 {
            if e & 1 == 1 {
                acc = self.mul(acc, a);
            }
            a = self.sqr(a);
            e >>= 1;
        }
        acc
    }

    /// Montgomery's trick: invert every (non-zero) element of `xs` in
    /// place with a single field inversion and `3(k − 1)` multiplications.
    pub fn batch_inv(&self, xs: &mut [u64], scratch: &mut Vec<u64>) {
        let k = xs.len();
        if k == 0 {
            return;
        }
        static INTERLEAVE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        if k >= 32 && *INTERLEAVE.get_or_init(|| std::env::var_os("NOINTERLEAVE").is_none()) {
            return self.batch_inv_x4(xs, scratch);
        }
        if scratch.len() < k {
            scratch.resize(k, 0);
        }
        let mut acc = self.one;
        for i in 0..k {
            scratch[i] = acc;
            acc = self.mul(acc, xs[i]);
        }
        let mut inv_acc = self.inv(acc);
        for i in (0..k).rev() {
            let xi = xs[i];
            xs[i] = self.mul(inv_acc, scratch[i]);
            inv_acc = self.mul(inv_acc, xi);
        }
    }

    /// Montgomery's trick over four interleaved chains: the prefix-product
    /// and back-substitution chains are latency-bound, so four independent
    /// chains keep the multiplier busy. One inversion, same exact results.
    fn batch_inv_x4(&self, xs: &mut [u64], scratch: &mut Vec<u64>) {
        let k = xs.len();
        if scratch.len() < k {
            scratch.resize(k, 0);
        }
        let base = k / 4;
        let rem = k % 4;
        let mut start = [0usize; 4];
        let mut s = 0usize;
        for (c, st) in start.iter_mut().enumerate() {
            *st = s;
            s += base + usize::from(c < rem);
        }
        let mut acc = [self.one; 4];
        for t in 0..base {
            for c in 0..4 {
                let i = start[c] + t;
                scratch[i] = acc[c];
                acc[c] = self.mul(acc[c], xs[i]);
            }
        }
        for c in 0..rem {
            let i = start[c] + base;
            scratch[i] = acc[c];
            acc[c] = self.mul(acc[c], xs[i]);
        }
        let p01 = self.mul(acc[0], acc[1]);
        let p012 = self.mul(p01, acc[2]);
        let p0123 = self.mul(p012, acc[3]);
        let mut inv = self.inv(p0123);
        let i3 = self.mul(inv, p012);
        inv = self.mul(inv, acc[3]);
        let i2 = self.mul(inv, p01);
        inv = self.mul(inv, acc[2]);
        let i1 = self.mul(inv, acc[0]);
        let i0 = self.mul(inv, acc[1]);
        let mut ia = [i0, i1, i2, i3];
        for c in 0..rem {
            let i = start[c] + base;
            let xi = xs[i];
            xs[i] = self.mul(ia[c], scratch[i]);
            ia[c] = self.mul(ia[c], xi);
        }
        for t in (0..base).rev() {
            for c in 0..4 {
                let i = start[c] + t;
                let xi = xs[i];
                xs[i] = self.mul(ia[c], scratch[i]);
                ia[c] = self.mul(ia[c], xi);
            }
        }
    }

    /// Square root of a Montgomery value (Tonelli–Shanks), in Montgomery
    /// form.  `None` if `a` is a non-residue.
    pub fn sqrt(&self, a: u64) -> Option<u64> {
        if a == 0 {
            return Some(0);
        }
        let p = self.p;
        if self.pow(a, (p - 1) / 2) != self.one {
            return None;
        }
        if p % 4 == 3 {
            return Some(self.pow(a, (p + 1) / 4));
        }
        let mut q = p - 1;
        let mut s = 0u32;
        while q & 1 == 0 {
            q >>= 1;
            s += 1;
        }
        let minus_one = self.neg(self.one);
        let mut z = 2u64;
        while self.pow(self.to_mont(z), (p - 1) / 2) != minus_one {
            z += 1;
        }
        let mut m = s;
        let mut c = self.pow(self.to_mont(z), q);
        let mut t = self.pow(a, q);
        let mut r = self.pow(a, q.div_ceil(2));
        loop {
            if t == self.one {
                return Some(r);
            }
            let mut i = 0u32;
            let mut tt = t;
            while tt != self.one {
                tt = self.sqr(tt);
                i += 1;
                if i == m {
                    return None;
                }
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = self.sqr(b);
            }
            m = i;
            c = self.sqr(b);
            t = self.mul(t, c);
            r = self.mul(r, b);
        }
    }
}

/// `a⁻¹ mod p` for odd `p < 2^63` and `0 < a < p`, by the binary
/// extended Euclidean algorithm (HAC 14.61): no divisions.
pub fn inv_mod_u64(a: u64, p: u64) -> u64 {
    debug_assert!(a != 0 && a < p && p & 1 == 1);
    let (mut u, mut v) = (a, p);
    // Invariants: x1·a ≡ u, x2·a ≡ v (mod p).
    let (mut x1, mut x2) = (1u64, 0u64);
    #[inline(always)]
    fn halve(x: u64, p: u64) -> u64 {
        if x & 1 == 0 {
            x >> 1
        } else {
            ((x as u128 + p as u128) >> 1) as u64
        }
    }
    while u != 1 && v != 1 {
        while u & 1 == 0 {
            u >>= 1;
            x1 = halve(x1, p);
        }
        while v & 1 == 0 {
            v >>= 1;
            x2 = halve(x2, p);
        }
        if u >= v {
            u -= v;
            x1 = if x1 >= x2 { x1 - x2 } else { x1 + p - x2 };
        } else {
            v -= u;
            x2 = if x2 >= x1 { x2 - x1 } else { x2 + p - x1 };
        }
    }
    if u == 1 {
        x1
    } else {
        x2
    }
}

/// `a·b mod m` for a fixed modulus, with the value of
/// `((a as u128 * b as u128) % m as u128) as u64` for every input.
///
/// For an odd `m < 2^63` and reduced operands the product goes through two
/// Montgomery reductions (`a·b·R⁻¹`, then `·R²·R⁻¹`), which return the
/// canonical remainder in `[0, m)` — the same value the 128-bit `%`
/// (`__umodti3`, a library call) returns, without a division.  Anything
/// else (even modulus, `m ≥ 2^63`, unreduced operand) takes the 128-bit
/// path unchanged, so no input changes its result.  Replay class WALL:
/// callers that swap their `u128 %` for this keep every value and every
/// counted unit (assessment 2026-11-10, R1/R2).
#[derive(Clone, Copy, Debug)]
pub struct MulModN {
    pub m: u64,
    /// `−m⁻¹ mod 2^64` (0 when the Montgomery path is off).
    n_inv: u64,
    /// `2^128 mod m`.
    r2: u64,
    mont: bool,
}

impl MulModN {
    pub fn new(m: u64) -> Self {
        let mont = m & 1 == 1 && m > 1 && m < (1u64 << 63);
        if !mont {
            return MulModN {
                m,
                n_inv: 0,
                r2: 0,
                mont,
            };
        }
        let mut inv: u64 = 1;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(m.wrapping_mul(inv)));
        }
        let r = ((1u128 << 64) % m as u128) as u64;
        let r2 = ((r as u128 * r as u128) % m as u128) as u64;
        MulModN {
            m,
            n_inv: inv.wrapping_neg(),
            r2,
            mont,
        }
    }

    /// Montgomery reduction of `t < m·2^64`: `t·2^-64 mod m` in `[0, m)`.
    #[inline(always)]
    fn redc(&self, t: u128) -> u64 {
        let q = (t as u64).wrapping_mul(self.n_inv);
        // t + q·m < 2m·2^64 ≤ 2^128 for m < 2^63, and the quotient by 2^64
        // is below 2m, so one conditional subtraction makes it canonical.
        let u = ((t + q as u128 * self.m as u128) >> 64) as u64;
        // A select, not a branch: on field data the comparison is a coin
        // flip a branch predictor cannot learn.
        let (d, borrow) = u.overflowing_sub(self.m);
        std::hint::select_unpredictable(borrow, u, d)
    }

    #[inline(always)]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        if self.mont && a < self.m && b < self.m {
            self.redc(self.redc(a as u128 * b as u128) as u128 * self.r2 as u128)
        } else {
            ((a as u128 * b as u128) % self.m as u128) as u64
        }
    }
}

#[inline(always)]
fn add_mod(a: u64, b: u64, n: u64) -> u64 {
    let s = a + b;
    if s >= n {
        s - n
    } else {
        s
    }
}

#[inline(always)]
fn neg_mod(a: u64, n: u64) -> u64 {
    if a == 0 {
        0
    } else {
        n - a
    }
}

// ── Curve and points ──────────────────────────────────────────────────

/// Finite affine point with Montgomery-form coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pt {
    pub x: u64,
    pub y: u64,
}

/// A Jacobian point `(X : Y : Z)`, `x = X/Z²`, `y = Y/Z³`, Montgomery
/// coordinates; `Z = 0` is `O`.  Only the scalar multiplications use it.
#[derive(Clone, Copy, Debug)]
struct Jac {
    x: u64,
    y: u64,
    z: u64,
}

const JAC_INFINITY: Jac = Jac { x: 0, y: 0, z: 0 };

/// Short-Weierstrass curve `y² = x³ + ax + b` over `F_p`, `p < 2^62`,
/// with a prime-order generator subgroup of order `n`.
#[derive(Clone, Debug)]
pub struct FastCurve {
    pub f: Fp64,
    /// `a`, `b` in Montgomery form.
    pub a: u64,
    pub b: u64,
    /// Subgroup order (prime, `< 2^62`).
    pub n: u64,
    pub name: String,
}

/// Reused storage for simultaneous additions from independent walk lanes.
/// Each active ordinary lane contributes one denominator; exceptional lanes
/// use the ordinary group law after the shared inversion.
#[derive(Default)]
pub struct BatchAddScratch {
    denominators: Vec<u64>,
    prefix: Vec<u64>,
    ordinary: Vec<Option<usize>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BatchAddStats {
    pub ordinary: usize,
    pub exceptional: usize,
    pub inversions: usize,
}
