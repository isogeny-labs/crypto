//! The two base fields of `isoclass-rho`, both on `u64` words: `F_p` for a
//! prime `p < 2^62`, and `F_{2^m}` (`m ≤ 62`, odd) in polynomial basis with
//! carry-less multiplication.  Each field also carries the group law of the
//! curve form that lives over it (short Weierstrass `y² = x³ + ax + b` over
//! `F_p`; `y² + xy = x³ + ax² + b` over `F_{2^m}`), so the walk, the rho
//! walk and the statistics are written once, generically over [`Fq`].

use rand::rngs::SmallRng;
use rand::Rng;

/// A curve model: `y² = x³ + ax + b` over `F_p`, `y² + xy = x³ + ax² + b`
/// over `F_{2^m}`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Curve {
    pub a: u64,
    pub b: u64,
}

pub type Pt = Option<(u64, u64)>;

/// A commutative ring with its element type: the base field (`u64`) or an
/// extension `F_q[y]/(g)` (`Vec<u64>`).  The x-only ladders are written
/// over this trait, so a kernel's `x`-coordinates can be enumerated in the
/// extension field that contains them.
pub trait Ring {
    type El: Clone + PartialEq + std::fmt::Debug;
    fn zero(&self) -> Self::El;
    fn one(&self) -> Self::El;
    /// The integer `k` as a ring element (`k·1`).
    fn small(&self, k: u64) -> Self::El;
    /// A base-field element lifted into the ring.
    fn lift(&self, v: u64) -> Self::El;
    fn is_zero(&self, a: &Self::El) -> bool;
    fn add(&self, a: &Self::El, b: &Self::El) -> Self::El;
    fn sub(&self, a: &Self::El, b: &Self::El) -> Self::El;
    fn mul(&self, a: &Self::El, b: &Self::El) -> Self::El;
    fn sqr(&self, a: &Self::El) -> Self::El {
        self.mul(a, a)
    }
    fn inv(&self, a: &Self::El) -> Option<Self::El>;
}

/// A base field on `u64` words, with the curve form that lives over it.
pub trait Fq: Copy + Send + Sync + 'static {
    fn char2(&self) -> bool;
    /// `⌊log₂ q⌋ + 1`.
    fn bits(&self) -> u32;
    /// `q` in base-2 digits, least significant first, for powering by `q`.
    fn q_bits(&self) -> Vec<bool>;
    fn fadd(&self, a: u64, b: u64) -> u64;
    fn fsub(&self, a: u64, b: u64) -> u64;
    fn fneg(&self, a: u64) -> u64;
    fn fmul(&self, a: u64, b: u64) -> u64;
    fn fsqr(&self, a: u64) -> u64 {
        self.fmul(a, a)
    }
    fn finv(&self, a: u64) -> u64;
    fn fpow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = self.fmul(r, a);
            }
            a = self.fmul(a, a);
            e >>= 1;
        }
        r
    }
    /// The integer `k` as a field element.
    fn fsmall(&self, k: u64) -> u64;
    fn frandom(&self, rng: &mut SmallRng) -> u64;

    // ----- the curve form over this field
    fn ec_j(&self, c: &Curve) -> Option<u64>;
    fn ec_on(&self, c: &Curve, p: Pt) -> bool;
    fn ec_neg(&self, c: &Curve, p: Pt) -> Pt;
    fn ec_add(&self, c: &Curve, p: Pt, q: Pt) -> Pt;
    fn ec_dbl(&self, c: &Curve, p: Pt) -> Pt {
        self.ec_add(c, p, p)
    }
    fn ec_mul(&self, c: &Curve, p: Pt, k: u64) -> Pt {
        let (mut r, mut b, mut k) = (None, p, k);
        while k > 0 {
            if k & 1 == 1 {
                r = self.ec_add(c, r, b);
            }
            b = self.ec_dbl(c, b);
            k >>= 1;
        }
        r
    }
    /// A random affine point other than `O` (its `y` may be anything).
    fn ec_random(&self, c: &Curve, rng: &mut SmallRng) -> (u64, u64);
    /// An isomorphic model drawn from `rng` with the map on points: a
    /// change of coordinates, the same curve.
    fn ec_iso_model(
        &self,
        c: &Curve,
        rng: &mut SmallRng,
    ) -> (Curve, Box<dyn Fn((u64, u64)) -> (u64, u64) + Send + Sync>);
    /// `x(2P)` from `x(P)` in any ring over this field; `None` if `2P = O`.
    fn x_dbl<R: Ring>(&self, r: &R, c: &Curve, x: &R::El) -> Option<R::El>;
    /// `x(P + Q)` from `x(P)`, `x(Q)` and `x(P − Q)`; `None` if `x(P) = x(Q)`.
    fn x_sum<R: Ring>(&self, r: &R, c: &Curve, x1: &R::El, x2: &R::El, xd: &R::El)
        -> Option<R::El>;
    /// `x(P)` as a packed word used by the rho walk's partition and
    /// distinguished-point test.
    fn hash_x(&self, x: u64) -> u64 {
        x
    }
}

// ================================================================== F_p

#[derive(Clone, Copy, Debug)]
pub struct Fp {
    pub p: u64,
}

pub fn inv_mod(a: u64, m: u64) -> u64 {
    let (mut t, mut nt, mut r, mut nr) = (0i128, 1i128, m as i128, (a % m) as i128);
    while nr != 0 {
        let q = r / nr;
        (t, nt) = (nt, t - q * nt);
        (r, nr) = (nr, r - q * nr);
    }
    assert_eq!(r, 1, "inverse of {a} mod {m} does not exist");
    ((t % m as i128 + m as i128) % m as i128) as u64
}

pub fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

pub fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
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

pub fn is_prime(n: u64) -> bool {
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

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn brent(n: u64) -> u64 {
    if n.is_multiple_of(2) {
        return 2;
    }
    let mut c = 1u64;
    loop {
        let (mut y, mut r, mut q, mut g) = (2u64, 1u64, 1u64, 1u64);
        let (mut x, mut ys) = (0u64, 0u64);
        let f = |v: u64| (mulmod(v, v, n) + c) % n;
        while g == 1 {
            x = y;
            for _ in 0..r {
                y = f(y);
            }
            let mut k = 0;
            while k < r && g == 1 {
                ys = y;
                for _ in 0..r.min(128).min(r - k) {
                    y = f(y);
                    q = mulmod(q, x.abs_diff(y), n);
                }
                g = gcd(q, n);
                k += 128;
            }
            r *= 2;
        }
        if g == n {
            loop {
                ys = f(ys);
                g = gcd(x.abs_diff(ys), n);
                if g > 1 {
                    break;
                }
            }
        }
        if g != n {
            return g;
        }
        c += 1;
    }
}

/// Prime factorisation `[(q, e)]`, ascending.
pub fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out: Vec<(u64, u32)> = Vec::new();
    let mut push = |q: u64| match out.iter_mut().find(|(r, _)| *r == q) {
        Some((_, e)) => *e += 1,
        None => out.push((q, 1)),
    };
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47] {
        while n.is_multiple_of(q) {
            push(q);
            n /= q;
        }
    }
    let mut stack = vec![n];
    while let Some(m) = stack.pop() {
        if m == 1 {
            continue;
        }
        if is_prime(m) {
            push(m);
            continue;
        }
        let d = brent(m);
        stack.push(d);
        stack.push(m / d);
    }
    out.sort();
    out
}

impl Fp {
    pub fn sqrt(&self, a: u64) -> Option<u64> {
        let p = self.p;
        if a == 0 {
            return Some(0);
        }
        if powmod(a, (p - 1) / 2, p) != 1 {
            return None;
        }
        if p % 4 == 3 {
            return Some(powmod(a, (p + 1) / 4, p));
        }
        let s = (p - 1).trailing_zeros();
        let q = (p - 1) >> s;
        let mut z = 2;
        while powmod(z, (p - 1) / 2, p) != p - 1 {
            z += 1;
        }
        let (mut m, mut c) = (s, powmod(z, q, p));
        let (mut t, mut r) = (powmod(a, q, p), powmod(a, q.div_ceil(2), p));
        while t != 1 {
            let (mut i, mut t2) = (0, t);
            while t2 != 1 {
                t2 = mulmod(t2, t2, p);
                i += 1;
            }
            let b = powmod(c, 1 << (m - i - 1), p);
            m = i;
            c = mulmod(b, b, p);
            t = mulmod(t, c, p);
            r = mulmod(r, b, p);
        }
        Some(r)
    }
    pub fn rhs(&self, c: &Curve, x: u64) -> u64 {
        self.fadd(self.fmul(self.fadd(self.fmul(x, x), c.a), x), c.b)
    }
    /// A `k`-th root of `c` for a prime `k` (Adleman–Manders–Miller), or
    /// `None` if `c` is not a `k`-th power (or the Sylow subgroup is too
    /// large to search).
    pub fn kth_root(&self, c: u64, k: u64) -> Option<u64> {
        let p = self.p;
        if c == 0 {
            return Some(0);
        }
        let (mut t, mut s) = (p - 1, 0u32);
        while t.is_multiple_of(k) {
            t /= k;
            s += 1;
        }
        if s == 0 {
            return Some(powmod(c, inv_mod(k % (p - 1), p - 1), p));
        }
        if powmod(c, (p - 1) / k, p) != 1 {
            return None;
        }
        let mut g = 2u64;
        while powmod(g, (p - 1) / k, p) == 1 {
            g += 1;
        }
        let rho = powmod(g, t, p); // order k^s
        let ks = k.pow(s);
        if ks > 1 << 22 {
            return None;
        }
        let (x, w) = if t == 1 {
            (1u64, c)
        } else {
            let alpha = inv_mod(k % t, t);
            // k·alpha = 1 + m·t
            let m = ((k as u128 * alpha as u128 - 1) / t as u128) as u64;
            (powmod(c, alpha, p), powmod(powmod(c, t, p), m, p))
        };
        // w = rho^j in the Sylow subgroup; x^k = c·w
        let mut pw = 1u64;
        for j in 0..ks {
            if pw == w {
                if j % k != 0 {
                    return None;
                }
                let r = mulmod(x, inv_mod(powmod(rho, j / k, p), p), p);
                debug_assert_eq!(powmod(r, k, p), c);
                return Some(r);
            }
            pw = mulmod(pw, rho, p);
        }
        None
    }
    /// `u` with `u^6 = c`, if any.
    pub fn sixth_root(&self, c: u64) -> Option<u64> {
        let z = self.kth_root(c, 3)?;
        let s3 = self.sqrt(self.fneg(3))?;
        let omega = self.fmul(self.fsub(s3, 1), self.finv(2));
        let mut zz = z;
        for _ in 0..3 {
            if let Some(u) = self.sqrt(zz) {
                return Some(u);
            }
            zz = self.fmul(zz, omega);
        }
        None
    }
}

impl Fq for Fp {
    fn char2(&self) -> bool {
        false
    }
    fn bits(&self) -> u32 {
        64 - self.p.leading_zeros()
    }
    fn q_bits(&self) -> Vec<bool> {
        (0..self.bits()).map(|i| (self.p >> i) & 1 == 1).collect()
    }
    fn fadd(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    fn fsub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    fn fneg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    fn fmul(&self, a: u64, b: u64) -> u64 {
        mulmod(a, b, self.p)
    }
    fn finv(&self, a: u64) -> u64 {
        inv_mod(a, self.p)
    }
    fn fsmall(&self, k: u64) -> u64 {
        k % self.p
    }
    fn frandom(&self, rng: &mut SmallRng) -> u64 {
        rng.gen_range(0..self.p)
    }
    fn ec_j(&self, c: &Curve) -> Option<u64> {
        let a3 = self
            .fsmall(4)
            .pipe(|k| self.fmul(k, self.fmul(self.fmul(c.a, c.a), c.a)));
        let den = self.fadd(a3, self.fmul(self.fsmall(27), self.fmul(c.b, c.b)));
        (den != 0).then(|| self.fmul(self.fmul(self.fsmall(1728), a3), self.finv(den)))
    }
    fn ec_on(&self, c: &Curve, p: Pt) -> bool {
        p.is_none_or(|(x, y)| self.fmul(y, y) == self.rhs(c, x))
    }
    fn ec_neg(&self, _c: &Curve, p: Pt) -> Pt {
        p.map(|(x, y)| (x, self.fneg(y)))
    }
    fn ec_add(&self, c: &Curve, p: Pt, q: Pt) -> Pt {
        let (Some((x1, y1)), Some((x2, y2))) = (p, q) else {
            return p.or(q);
        };
        let l = if x1 == x2 {
            if self.fadd(y1, y2) == 0 {
                return None;
            }
            self.fmul(
                self.fadd(self.fmul(self.fsmall(3), self.fmul(x1, x1)), c.a),
                self.finv(self.fadd(y1, y1)),
            )
        } else {
            self.fmul(self.fsub(y2, y1), self.finv(self.fsub(x2, x1)))
        };
        let x3 = self.fsub(self.fsub(self.fmul(l, l), x1), x2);
        Some((x3, self.fsub(self.fmul(l, self.fsub(x1, x3)), y1)))
    }
    fn ec_random(&self, c: &Curve, rng: &mut SmallRng) -> (u64, u64) {
        loop {
            let x = rng.gen_range(0..self.p);
            if let Some(y) = self.sqrt(self.rhs(c, x)) {
                if y != 0 {
                    return (x, y);
                }
            }
        }
    }
    fn ec_iso_model(
        &self,
        c: &Curve,
        rng: &mut SmallRng,
    ) -> (Curve, Box<dyn Fn((u64, u64)) -> (u64, u64) + Send + Sync>) {
        // (u⁴a, u⁶b), (x, y) ↦ (u²x, u³y).
        let u = rng.gen_range(2..self.p);
        let (u2, u3) = (self.fmul(u, u), self.fmul(self.fmul(u, u), u));
        let m = Curve {
            a: self.fmul(self.fmul(u2, u2), c.a),
            b: self.fmul(self.fmul(u3, u3), c.b),
        };
        let f = *self;
        (m, Box::new(move |(x, y)| (f.fmul(u2, x), f.fmul(u3, y))))
    }
    fn x_dbl<R: Ring>(&self, r: &R, c: &Curve, x: &R::El) -> Option<R::El> {
        // x(2P) = ((x² − a)² − 8bx) / (4(x³ + ax + b))
        let (a, b) = (r.lift(c.a), r.lift(c.b));
        let x2 = r.sqr(x);
        let num = r.sub(&r.sqr(&r.sub(&x2, &a)), &r.mul(&r.small(8), &r.mul(&b, x)));
        let f = r.add(&r.mul(&r.add(&x2, &a), x), &b);
        let den = r.inv(&r.mul(&r.small(4), &f))?;
        Some(r.mul(&num, &den))
    }
    fn x_sum<R: Ring>(
        &self,
        r: &R,
        c: &Curve,
        x1: &R::El,
        x2: &R::El,
        xd: &R::El,
    ) -> Option<R::El> {
        // x(P+Q) + x(P−Q) = 2[(x1x2 + a)(x1 + x2) + 2b] / (x1 − x2)²
        let (a, b) = (r.lift(c.a), r.lift(c.b));
        let d = r.sub(x1, x2);
        let inv = r.inv(&r.sqr(&d))?;
        let num = r.mul(
            &r.small(2),
            &r.add(
                &r.mul(&r.add(&r.mul(x1, x2), &a), &r.add(x1, x2)),
                &r.mul(&r.small(2), &b),
            ),
        );
        Some(r.sub(&r.mul(&num, &inv), xd))
    }
}

trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}
impl Pipe for u64 {}

impl Ring for Fp {
    type El = u64;
    fn zero(&self) -> u64 {
        0
    }
    fn one(&self) -> u64 {
        1
    }
    fn small(&self, k: u64) -> u64 {
        k % self.p
    }
    fn lift(&self, v: u64) -> u64 {
        v
    }
    fn is_zero(&self, a: &u64) -> bool {
        *a == 0
    }
    fn add(&self, a: &u64, b: &u64) -> u64 {
        self.fadd(*a, *b)
    }
    fn sub(&self, a: &u64, b: &u64) -> u64 {
        self.fsub(*a, *b)
    }
    fn mul(&self, a: &u64, b: &u64) -> u64 {
        self.fmul(*a, *b)
    }
    fn inv(&self, a: &u64) -> Option<u64> {
        (*a != 0).then(|| self.finv(*a))
    }
}

// ================================================================== F_{2^m}

/// `F_{2^m} = F_2[z]/(poly)`, elements as `u64` bit vectors, `m ≤ 62`.
#[derive(Clone, Copy, Debug)]
pub struct F2m {
    pub m: u32,
    /// The irreducible polynomial with its `z^m` bit set.
    pub poly: u64,
    tail: u64,
    mask: u64,
    clmul: bool,
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
unsafe fn clmul_hw(a: u64, b: u64) -> u128 {
    use std::arch::x86_64::*;
    let x = _mm_set_epi64x(0, a as i64);
    let y = _mm_set_epi64x(0, b as i64);
    let z = _mm_clmulepi64_si128::<0x00>(x, y);
    let lo = _mm_cvtsi128_si64(z) as u64;
    let hi = _mm_cvtsi128_si64(_mm_srli_si128::<8>(z)) as u64;
    ((hi as u128) << 64) | lo as u128
}

fn clmul_sw(a: u64, b: u64) -> u128 {
    let mut r = 0u128;
    let mut bb = b;
    let mut i = 0;
    while bb != 0 {
        if bb & 1 == 1 {
            r ^= (a as u128) << i;
        }
        bb >>= 1;
        i += 1;
    }
    r
}

/// Degree of a non-zero `F_2[z]` polynomial in a word.
fn deg(a: u128) -> i32 {
    127 - a.leading_zeros() as i32
}

/// `a mod f` for `F_2[z]` polynomials in words.
fn pmod(mut a: u128, f: u128) -> u128 {
    let df = deg(f);
    while a != 0 && deg(a) >= df {
        a ^= f << (deg(a) - df);
    }
    a
}

fn pgcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, pmod(a, b));
    }
    a
}

impl F2m {
    pub fn new(m: u32, poly: u64) -> Self {
        assert!(
            (3..=62).contains(&m) && poly >> m == 1,
            "F2m: need 3 <= m <= 62 and a degree-m polynomial"
        );
        #[cfg(target_arch = "x86_64")]
        let clmul = is_x86_feature_detected!("pclmulqdq");
        #[cfg(not(target_arch = "x86_64"))]
        let clmul = false;
        F2m {
            m,
            poly,
            tail: poly ^ (1 << m),
            mask: (1u64 << m) - 1,
            clmul,
        }
    }
    /// Whether `f` (degree `m`, bit `m` set) is irreducible over `F_2`
    /// (Ben-Or: `gcd(z^{2^i} − z, f) = 1` for `i ≤ m/2`).
    pub fn irreducible(f: u64) -> bool {
        let m = 63 - f.leading_zeros();
        let fm = f as u128;
        let mut x = 2u128; // z
        for _ in 1..=m / 2 {
            // x ← x² mod f
            x = pmod(clmul_sw(x as u64, x as u64), fm);
            if pgcd(x ^ 2, fm) != 1 {
                return false;
            }
        }
        true
    }
    /// The `k`-th irreducible polynomial of degree `m` in the order
    /// trinomials `z^m + z^t + 1` (`t` ascending) then pentanomials
    /// `z^m + z^{t3} + z^{t2} + z^{t1} + 1` (lexicographic).
    pub fn irreducible_poly(m: u32, k: usize) -> u64 {
        let mut seen = 0;
        for t in 1..m {
            let f = (1u64 << m) | (1 << t) | 1;
            if Self::irreducible(f) {
                if seen == k {
                    return f;
                }
                seen += 1;
            }
        }
        for t3 in 3..m {
            for t2 in 2..t3 {
                for t1 in 1..t2 {
                    let f = (1u64 << m) | (1 << t3) | (1 << t2) | (1 << t1) | 1;
                    if Self::irreducible(f) {
                        if seen == k {
                            return f;
                        }
                        seen += 1;
                    }
                }
            }
        }
        panic!("no irreducible polynomial of degree {m} at index {k}")
    }
    #[inline]
    fn clmul(&self, a: u64, b: u64) -> u128 {
        #[cfg(target_arch = "x86_64")]
        if self.clmul {
            return unsafe { clmul_hw(a, b) };
        }
        clmul_sw(a, b)
    }
    #[inline]
    fn reduce(&self, mut p: u128) -> u64 {
        let m = self.m;
        loop {
            let hi = (p >> m) as u64;
            if hi == 0 {
                return p as u64 & self.mask;
            }
            p = (p & self.mask as u128) ^ self.clmul(hi, self.tail);
        }
    }
    pub fn trace(&self, a: u64) -> u64 {
        let (mut t, mut x) = (0u64, a);
        for _ in 0..self.m {
            t ^= x;
            x = self.fsqr(x);
        }
        t
    }
    /// A solution `z` of `z² + z = c` (`m` odd; needs `Tr(c) = 0`).
    pub fn half_trace(&self, c: u64) -> Option<u64> {
        if self.trace(c) != 0 {
            return None;
        }
        let (mut h, mut x) = (0u64, c);
        for _ in 0..=(self.m - 1) / 2 {
            h ^= x;
            x = self.fsqr(self.fsqr(x));
        }
        debug_assert_eq!(self.fadd(self.fsqr(h), h), c);
        Some(h)
    }
    pub fn sqrt(&self, a: u64) -> u64 {
        // a^{2^{m−1}}
        let mut x = a;
        for _ in 0..self.m - 1 {
            x = self.fsqr(x);
        }
        x
    }
    /// `x³ + ax² + b`.
    pub fn rhs(&self, c: &Curve, x: u64) -> u64 {
        let x2 = self.fsqr(x);
        self.fadd(self.fadd(self.fmul(x2, x), self.fmul(c.a, x2)), c.b)
    }
    /// `y` with `(x, y)` on the curve, if any.
    pub fn lift_x(&self, c: &Curve, x: u64) -> Option<u64> {
        if x == 0 {
            return Some(self.sqrt(c.b));
        }
        let cc = self.fmul(self.rhs(c, x), self.finv(self.fsqr(x)));
        self.half_trace(cc).map(|z| self.fmul(z, x))
    }
    /// The map `F_2[z]/(poly) → F_2[z]/(other)` sending `z` to `root`, a
    /// root of `poly` in the other field.
    pub fn embed(&self, other: &F2m, root: u64, a: u64) -> u64 {
        let mut acc = 0u64;
        for i in (0..self.m).rev() {
            acc = other.fmul(acc, root);
            if (a >> i) & 1 == 1 {
                acc ^= 1;
            }
        }
        acc
    }
}

impl Fq for F2m {
    fn char2(&self) -> bool {
        true
    }
    fn bits(&self) -> u32 {
        self.m + 1
    }
    fn q_bits(&self) -> Vec<bool> {
        let mut v = vec![false; self.m as usize + 1];
        v[self.m as usize] = true;
        v
    }
    fn fadd(&self, a: u64, b: u64) -> u64 {
        a ^ b
    }
    fn fsub(&self, a: u64, b: u64) -> u64 {
        a ^ b
    }
    fn fneg(&self, a: u64) -> u64 {
        a
    }
    fn fmul(&self, a: u64, b: u64) -> u64 {
        self.reduce(self.clmul(a, b))
    }
    fn finv(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of 0 in F_2^m");
        // Extended Euclid in F_2[z].
        let (mut u, mut v) = (a as u128, self.poly as u128);
        let (mut g1, mut g2) = (1u128, 0u128);
        while u != 1 {
            let mut j = deg(u) - deg(v);
            if j < 0 {
                std::mem::swap(&mut u, &mut v);
                std::mem::swap(&mut g1, &mut g2);
                j = -j;
            }
            u ^= v << j;
            g1 ^= g2 << j;
        }
        self.reduce(g1)
    }
    fn fsmall(&self, k: u64) -> u64 {
        k & 1
    }
    fn frandom(&self, rng: &mut SmallRng) -> u64 {
        rng.gen::<u64>() & self.mask
    }
    fn ec_j(&self, c: &Curve) -> Option<u64> {
        (c.b != 0).then(|| self.finv(c.b))
    }
    fn ec_on(&self, c: &Curve, p: Pt) -> bool {
        p.is_none_or(|(x, y)| self.fadd(self.fsqr(y), self.fmul(x, y)) == self.rhs(c, x))
    }
    fn ec_neg(&self, _c: &Curve, p: Pt) -> Pt {
        p.map(|(x, y)| (x, x ^ y))
    }
    fn ec_add(&self, c: &Curve, p: Pt, q: Pt) -> Pt {
        let (Some((x1, y1)), Some((x2, y2))) = (p, q) else {
            return p.or(q);
        };
        if x1 == x2 {
            if y1 != y2 || x1 == 0 {
                return None; // P = −Q, or a 2-torsion point doubled
            }
            // λ = x + y/x; x3 = λ² + λ + a; y3 = x² + (λ + 1)x3
            let l = x1 ^ self.fmul(y1, self.finv(x1));
            let x3 = self.fsqr(l) ^ l ^ c.a;
            let y3 = self.fsqr(x1) ^ self.fmul(l ^ 1, x3);
            return Some((x3, y3));
        }
        let l = self.fmul(y1 ^ y2, self.finv(x1 ^ x2));
        let x3 = self.fsqr(l) ^ l ^ x1 ^ x2 ^ c.a;
        let y3 = self.fmul(l, x1 ^ x3) ^ x3 ^ y1;
        Some((x3, y3))
    }
    fn ec_random(&self, c: &Curve, rng: &mut SmallRng) -> (u64, u64) {
        loop {
            let x = self.frandom(rng);
            if x == 0 {
                continue;
            }
            if let Some(y) = self.lift_x(c, x) {
                return (x, y);
            }
        }
    }
    fn ec_iso_model(
        &self,
        c: &Curve,
        rng: &mut SmallRng,
    ) -> (Curve, Box<dyn Fn((u64, u64)) -> (u64, u64) + Send + Sync>) {
        // (x, y) ↦ (x, y + sx) sends y² + xy = x³ + ax² + b to a' = a + s + s².
        let s = self.frandom(rng);
        let m = Curve {
            a: c.a ^ s ^ self.fsqr(s),
            b: c.b,
        };
        let f = *self;
        (m, Box::new(move |(x, y)| (x, y ^ f.fmul(s, x))))
    }
    fn x_dbl<R: Ring>(&self, r: &R, c: &Curve, x: &R::El) -> Option<R::El> {
        // x(2P) = x² + b/x²
        let x2 = r.sqr(x);
        let inv = r.inv(&x2)?;
        Some(r.add(&x2, &r.mul(&r.lift(c.b), &inv)))
    }
    fn x_sum<R: Ring>(
        &self,
        r: &R,
        _c: &Curve,
        x1: &R::El,
        x2: &R::El,
        xd: &R::El,
    ) -> Option<R::El> {
        // x(P+Q) + x(P−Q) = x1x2 / (x1 + x2)²
        let s = r.add(x1, x2);
        let inv = r.inv(&r.sqr(&s))?;
        Some(r.add(&r.mul(&r.mul(x1, x2), &inv), xd))
    }
}

impl Ring for F2m {
    type El = u64;
    fn zero(&self) -> u64 {
        0
    }
    fn one(&self) -> u64 {
        1
    }
    fn small(&self, k: u64) -> u64 {
        k & 1
    }
    fn lift(&self, v: u64) -> u64 {
        v
    }
    fn is_zero(&self, a: &u64) -> bool {
        *a == 0
    }
    fn add(&self, a: &u64, b: &u64) -> u64 {
        a ^ b
    }
    fn sub(&self, a: &u64, b: &u64) -> u64 {
        a ^ b
    }
    fn mul(&self, a: &u64, b: &u64) -> u64 {
        self.fmul(*a, *b)
    }
    fn inv(&self, a: &u64) -> Option<u64> {
        (*a != 0).then(|| self.finv(*a))
    }
}
