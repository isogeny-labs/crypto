//! Variable-time fixed-width Montgomery arithmetic mod an odd `p < 2^(64·N)`,
//! for the public-input Jacobian ladder of [`Point::scalar_mul`].
//!
//! [`FieldElement`]'s operations run on `num-bigint`: every one allocates,
//! multiplies in a general loop and reduces by a long division, and an
//! inversion is a 3-multiplications-a-bit exponentiation.  Here an element
//! is `N` words in Montgomery form (`a·R mod p`, `R = 2^(64·N)`), the
//! product is one CIOS pass, and every value is kept in `[0, p)`, so the
//! Montgomery form is canonical: zero tests and equality tests are the ones
//! on the integers.  Nothing here depends on `p` being prime: the formulas
//! are ring operations, and the inversion is the Fermat power `a^(p−2)`
//! that [`FieldElement::inv`] computes, so results equal the `BigUint`
//! path's for every odd modulus.  Variable time: for public values only.
//!
//! [`Point::scalar_mul`]: super::point::Point::scalar_mul
//! [`FieldElement`]: super::field::FieldElement
//! [`FieldElement::inv`]: super::field::FieldElement::inv

use num_bigint::BigUint;

/// The constants of an odd modulus `p` of at most `N` words.
#[derive(Clone, Copy)]
pub(crate) struct Mont<const N: usize> {
    p: [u64; N],
    /// `-p⁻¹ mod 2^64`.
    n0: u64,
    /// `R² mod p`.
    r2: [u64; N],
    /// `R mod p`: one, in Montgomery form.
    one: [u64; N],
    /// `R³ mod p`: converts a plain inverse `(aR)⁻¹` back to `a⁻¹R`.
    r3: [u64; N],
}

fn limbs<const N: usize>(v: &BigUint) -> Option<[u64; N]> {
    let mut out = [0u64; N];
    let mut it = v.iter_u64_digits();
    for o in out.iter_mut() {
        match it.next() {
            Some(d) => *o = d,
            None => return Some(out),
        }
    }
    it.next().is_none().then_some(out)
}

fn to_big<const N: usize>(a: &[u64; N]) -> BigUint {
    BigUint::from_slice(
        &a.iter()
            .flat_map(|w| [*w as u32, (*w >> 32) as u32])
            .collect::<Vec<u32>>(),
    )
}

/// `a ≥ b`.
#[inline(always)]
fn geq<const N: usize>(a: &[u64; N], b: &[u64; N]) -> bool {
    for i in (0..N).rev() {
        if a[i] != b[i] {
            return a[i] > b[i];
        }
    }
    true
}

/// `a − b` in place, returning the borrow.
#[inline(always)]
fn sub_in_place<const N: usize>(a: &mut [u64; N], b: &[u64; N]) -> bool {
    let mut borrow = false;
    for i in 0..N {
        let (d, b1) = a[i].overflowing_sub(b[i]);
        let (d, b2) = d.overflowing_sub(borrow as u64);
        a[i] = d;
        borrow = b1 | b2;
    }
    borrow
}

impl<const N: usize> Mont<N> {
    /// The context for `modulus`, or `None` if it is even, below 3, or
    /// wider than `N` words.
    pub(crate) fn new(modulus: &BigUint) -> Option<Self> {
        let p = limbs::<N>(modulus)?;
        if p[0] & 1 == 0 || *modulus < BigUint::from(3u32) {
            return None;
        }
        // n0 = -p⁻¹ mod 2^64 by Newton iteration.
        let mut inv = 1u64;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(p[0].wrapping_mul(inv)));
        }
        let r = BigUint::from(1u8) << (64 * N);
        let one = limbs::<N>(&(&r % modulus))?;
        let r2 = limbs::<N>(&((&r * &r) % modulus))?;
        let r3 = limbs::<N>(&((&r * &r * &r) % modulus))?;
        Some(Mont {
            p,
            n0: inv.wrapping_neg(),
            r2,
            one,
            r3,
        })
    }

    /// `a·b·R⁻¹ mod p` for `a, b < p` (CIOS), in `[0, p)`.
    #[inline(always)]
    pub(crate) fn mul(&self, a: &[u64; N], b: &[u64; N]) -> [u64; N] {
        let mut t = [0u64; N];
        let mut t_n: u64 = 0;
        for i in 0..N {
            let bi = b[i] as u128;
            let mut carry: u128 = 0;
            for j in 0..N {
                let s = t[j] as u128 + a[j] as u128 * bi + carry;
                t[j] = s as u64;
                carry = s >> 64;
            }
            let s = t_n as u128 + carry;
            t_n = s as u64;
            let t_n1 = (s >> 64) as u64;

            let m = t[0].wrapping_mul(self.n0) as u128;
            let s = t[0] as u128 + m * self.p[0] as u128;
            let mut carry = s >> 64;
            for j in 1..N {
                let s = t[j] as u128 + m * self.p[j] as u128 + carry;
                t[j - 1] = s as u64;
                carry = s >> 64;
            }
            let s = t_n as u128 + carry;
            t[N - 1] = s as u64;
            t_n = t_n1 + (s >> 64) as u64;
        }
        if t_n != 0 || geq(&t, &self.p) {
            sub_in_place(&mut t, &self.p);
        }
        t
    }

    #[inline(always)]
    pub(crate) fn add(&self, a: &[u64; N], b: &[u64; N]) -> [u64; N] {
        let mut s = [0u64; N];
        let mut carry = false;
        for i in 0..N {
            let (d, c1) = a[i].overflowing_add(b[i]);
            let (d, c2) = d.overflowing_add(carry as u64);
            s[i] = d;
            carry = c1 | c2;
        }
        if carry || geq(&s, &self.p) {
            sub_in_place(&mut s, &self.p);
        }
        s
    }

    #[inline(always)]
    pub(crate) fn sub(&self, a: &[u64; N], b: &[u64; N]) -> [u64; N] {
        let mut d = *a;
        if sub_in_place(&mut d, b) {
            let mut carry = false;
            for i in 0..N {
                let (v, c1) = d[i].overflowing_add(self.p[i]);
                let (v, c2) = v.overflowing_add(carry as u64);
                d[i] = v;
                carry = c1 | c2;
            }
        }
        d
    }

    /// Canonical integer `v < p` to Montgomery form.
    pub(crate) fn enter(&self, v: &BigUint) -> Option<[u64; N]> {
        let w = limbs::<N>(v)?;
        if !geq(&w, &self.p) {
            Some(self.mul(&w, &self.r2))
        } else {
            None
        }
    }

    pub(crate) fn one(&self) -> [u64; N] {
        self.one
    }

    /// Montgomery form back to the integer in `[0, p)`.
    pub(crate) fn leave(&self, a: &[u64; N]) -> BigUint {
        let mut unit = [0u64; N];
        unit[0] = 1;
        to_big(&self.mul(a, &unit))
    }

    /// `a^e` for a Montgomery-form `a` and an integer exponent, square and
    /// multiply from the top bit (as `FieldElement::pow` computes the same
    /// power); the result is in Montgomery form.
    pub(crate) fn pow(&self, a: &[u64; N], e: &BigUint) -> [u64; N] {
        self.pow_k::<ProdKernel>(a, e)
    }

    /// [`pow`](Self::pow) on the kernel `K`.
    pub(crate) fn pow_k<K: MulKernel>(&self, a: &[u64; N], e: &BigUint) -> [u64; N] {
        let mut acc = self.one;
        for i in (0..e.bits()).rev() {
            acc = K::sqr(self, &acc);
            if e.bit(i) {
                acc = K::mul(self, &acc, a);
            }
        }
        acc
    }
}

/// Scratch width for a double-length product: `2·N ≤ 18` words (`N ≤ 9`).
const WIDE: usize = 18;

/// `out[..a.len()+b.len()] = a·b`, schoolbook (operand-scanning), for the
/// half-size products of [`Mont::mul_wide_karatsuba`].  `out` is overwritten.
#[inline(always)]
fn mul_slices(a: &[u64], b: &[u64], out: &mut [u64]) {
    for o in out[..a.len() + b.len()].iter_mut() {
        *o = 0;
    }
    for (i, &bi) in b.iter().enumerate() {
        let mut c: u128 = 0;
        for (j, &aj) in a.iter().enumerate() {
            let s = out[i + j] as u128 + aj as u128 * bi as u128 + c;
            out[i + j] = s as u64;
            c = s >> 64;
        }
        out[i + a.len()] = c as u64;
    }
}

/// `out = |x − y|` for equal-length `x`, `y`; returns `x < y`.
#[inline(always)]
fn abs_diff(x: &[u64], y: &[u64], out: &mut [u64]) -> bool {
    let mut borrow = false;
    for i in 0..x.len() {
        let (d, b1) = x[i].overflowing_sub(y[i]);
        let (d, b2) = d.overflowing_sub(borrow as u64);
        out[i] = d;
        borrow = b1 | b2;
    }
    if borrow {
        // Two's-complement negate: out = 2^(64·len) − out.
        let mut c = true;
        for o in out[..x.len()].iter_mut() {
            let (v, c1) = (!*o).overflowing_add(c as u64);
            *o = v;
            c = c1;
        }
    }
    borrow
}

/// `acc[..] += x[..]` with carry propagated to the end of `acc`; returns the
/// carry out of `acc`.
#[inline(always)]
fn add_into(acc: &mut [u64], x: &[u64]) -> bool {
    let mut c = false;
    for i in 0..acc.len() {
        let xi = if i < x.len() { x[i] } else { 0 };
        if i >= x.len() && !c {
            break;
        }
        let (v, c1) = acc[i].overflowing_add(xi);
        let (v, c2) = v.overflowing_add(c as u64);
        acc[i] = v;
        c = c1 | c2;
    }
    c
}

/// `acc[..] −= x[..]` with borrow propagated; returns the borrow out.
#[inline(always)]
fn sub_into(acc: &mut [u64], x: &[u64]) -> bool {
    let mut b = false;
    for i in 0..acc.len() {
        let xi = if i < x.len() { x[i] } else { 0 };
        if i >= x.len() && !b {
            break;
        }
        let (v, b1) = acc[i].overflowing_sub(xi);
        let (v, b2) = v.overflowing_sub(b as u64);
        acc[i] = v;
        b = b1 | b2;
    }
    b
}

impl<const N: usize> Mont<N> {
    /// The full `2N`-word product `a·b` (schoolbook), in `t[..2N]`.
    #[inline(always)]
    pub(crate) fn mul_wide(a: &[u64; N], b: &[u64; N]) -> [u64; WIDE] {
        let mut t = [0u64; WIDE];
        for i in 0..N {
            let bi = b[i] as u128;
            let mut c: u128 = 0;
            for j in 0..N {
                let s = t[i + j] as u128 + a[j] as u128 * bi + c;
                t[i + j] = s as u64;
                c = s >> 64;
            }
            t[i + N] = c as u64;
        }
        t
    }

    /// The full `2N`-word square `a²`: each cross product `aᵢ·aⱼ` (`i < j`)
    /// once, the sum doubled by a shift, then the diagonal `aᵢ²` added —
    /// `N(N+1)/2` word products instead of the `N²` of [`mul_wide`].
    ///
    /// [`mul_wide`]: Self::mul_wide
    #[inline(always)]
    pub(crate) fn sqr_wide(a: &[u64; N]) -> [u64; WIDE] {
        let mut t = [0u64; WIDE];
        for i in 0..N {
            let ai = a[i] as u128;
            let mut c: u128 = 0;
            for j in (i + 1)..N {
                let s = t[i + j] as u128 + ai * a[j] as u128 + c;
                t[i + j] = s as u64;
                c = s >> 64;
            }
            if i + N < 2 * N {
                t[i + N] = c as u64;
            }
        }
        // Σ_{i<j} aᵢaⱼ·2^(64(i+j)) < 2^(128N−1), so the doubling cannot overflow.
        let mut top = 0u64;
        for w in t.iter_mut().take(2 * N) {
            let v = *w;
            *w = (v << 1) | top;
            top = v >> 63;
        }
        let mut c: u128 = 0;
        for i in 0..N {
            let d = a[i] as u128 * a[i] as u128;
            let s = t[2 * i] as u128 + (d as u64) as u128 + c;
            t[2 * i] = s as u64;
            let s2 = t[2 * i + 1] as u128 + (d >> 64) + (s >> 64);
            t[2 * i + 1] = s2 as u64;
            c = s2 >> 64;
        }
        t
    }

    /// The full product by one level of subtractive Karatsuba for even
    /// `N ≥ 4` (`a₀b₁ + a₁b₀ = a₀b₀ + a₁b₁ + (a₀ − a₁)(b₁ − b₀)`): three
    /// `N/2`-word products, `3N²/4` word products against `N²`.  Odd or
    /// small `N` falls back to [`mul_wide`](Self::mul_wide).
    #[inline(always)]
    pub(crate) fn mul_wide_karatsuba(a: &[u64; N], b: &[u64; N]) -> [u64; WIDE] {
        if N % 2 != 0 || N < 4 {
            return Self::mul_wide(a, b);
        }
        let h = N / 2;
        let mut t = [0u64; WIDE];
        // z0 = a0·b0 into t[0..N], z2 = a1·b1 into t[N..2N].
        let (lo, hi) = t.split_at_mut(N);
        mul_slices(&a[..h], &b[..h], lo);
        mul_slices(&a[h..], &b[h..], &mut hi[..N]);
        let mut da = [0u64; 5];
        let mut db = [0u64; 5];
        let neg_a = abs_diff(&a[..h], &a[h..], &mut da[..h]);
        let neg_b = abs_diff(&b[h..], &b[..h], &mut db[..h]);
        let mut m = [0u64; 10];
        mul_slices(&da[..h], &db[..h], &mut m[..N]);
        // mid = z0 + z2 ± m, in N+1 words (non-negative: it is a0b1 + a1b0).
        let mut mid = [0u64; 10];
        mid[..N].copy_from_slice(&t[..N]);
        let mut z2 = [0u64; 9];
        z2[..N].copy_from_slice(&t[N..2 * N]);
        add_into(&mut mid[..N + 1], &z2[..N]);
        if neg_a == neg_b {
            add_into(&mut mid[..N + 1], &m[..N]);
        } else {
            sub_into(&mut mid[..N + 1], &m[..N]);
        }
        add_into(&mut t[h..2 * N], &mid[..N + 1]);
        t
    }

    /// Montgomery reduction (SOS) of a `2N`-word `t < p·R`: `t·R⁻¹ mod p`,
    /// in `[0, p)`.
    #[inline(always)]
    pub(crate) fn redc(&self, mut t: [u64; WIDE]) -> [u64; N] {
        let mut carry_hi: u64 = 0;
        for i in 0..N {
            let m = t[i].wrapping_mul(self.n0) as u128;
            let mut c: u128 = 0;
            for j in 0..N {
                let s = t[i + j] as u128 + m * self.p[j] as u128 + c;
                t[i + j] = s as u64;
                c = s >> 64;
            }
            let s = t[i + N] as u128 + c + carry_hi as u128;
            t[i + N] = s as u64;
            carry_hi = (s >> 64) as u64;
        }
        let mut r = [0u64; N];
        r.copy_from_slice(&t[N..2 * N]);
        if carry_hi != 0 || geq(&r, &self.p) {
            sub_in_place(&mut r, &self.p);
        }
        r
    }

    /// `a²·R⁻¹ mod p` for `a < p`: [`sqr_wide`](Self::sqr_wide) then
    /// [`redc`](Self::redc).  Equal to `self.mul(a, a)` for every `a < p`.
    #[inline(always)]
    pub(crate) fn sqr(&self, a: &[u64; N]) -> [u64; N] {
        self.redc(Self::sqr_wide(a))
    }

    /// `a·b·R⁻¹ mod p` through [`mul_wide_karatsuba`](Self::mul_wide_karatsuba)
    /// and [`redc`](Self::redc).  Equal to `self.mul(a, b)`.
    #[inline(always)]
    pub(crate) fn mul_karatsuba(&self, a: &[u64; N], b: &[u64; N]) -> [u64; N] {
        self.redc(Self::mul_wide_karatsuba(a, b))
    }

    /// The inverse of a Montgomery-form `a` in Montgomery form, by the
    /// binary extended Euclidean algorithm on `N` words, in **variable
    /// time**; `None` when `a` is not a unit.  About `2·log₂p` shift/subtract
    /// steps on fixed-width words, no allocation.  Over a prime this is the
    /// Fermat inverse [`pow`](Self::pow)`(a, p − 2)`; on a composite modulus
    /// the two differ (as for `FieldElement::inv_vartime`).
    pub(crate) fn inv_bingcd(&self, a: &[u64; N]) -> Option<[u64; N]> {
        #[inline(always)]
        fn is_one<const N: usize>(x: &[u64; N]) -> bool {
            x[0] == 1 && x[1..].iter().all(|&w| w == 0)
        }
        #[inline(always)]
        fn shr1<const N: usize>(x: &mut [u64; N], top: u64) {
            for i in 0..N - 1 {
                x[i] = (x[i] >> 1) | (x[i + 1] << 63);
            }
            x[N - 1] = (x[N - 1] >> 1) | (top << 63);
        }
        // x ← x/2 mod p for x < p.
        let half = |x: &mut [u64; N]| {
            if x[0] & 1 == 0 {
                shr1(x, 0);
            } else {
                let mut c = false;
                for i in 0..N {
                    let (v, c1) = x[i].overflowing_add(self.p[i]);
                    let (v, c2) = v.overflowing_add(c as u64);
                    x[i] = v;
                    c = c1 | c2;
                }
                shr1(x, c as u64);
            }
        };
        if Self::is_zero(a) {
            return None;
        }
        let mut u = *a;
        let mut v = self.p;
        let mut x1 = [0u64; N];
        x1[0] = 1;
        let mut x2 = [0u64; N];
        while !is_one(&u) && !is_one(&v) {
            if Self::is_zero(&u) || Self::is_zero(&v) {
                return None;
            }
            while u[0] & 1 == 0 {
                shr1(&mut u, 0);
                half(&mut x1);
            }
            while v[0] & 1 == 0 {
                shr1(&mut v, 0);
                half(&mut x2);
            }
            if geq(&u, &v) {
                sub_in_place(&mut u, &v);
                x1 = self.sub(&x1, &x2);
            } else {
                sub_in_place(&mut v, &u);
                x2 = self.sub(&x2, &x1);
            }
        }
        let inv = if is_one(&u) { x1 } else { x2 };
        // inv = (aR)⁻¹; (aR)⁻¹·R³·R⁻¹ = a⁻¹·R.
        Some(self.mul(&inv, &self.r3))
    }
}

/// How a ladder multiplies and squares in [`Mont`]: every kernel returns the
/// same residues, so the choice changes speed only.
pub(crate) trait MulKernel {
    fn mul<const N: usize>(ctx: &Mont<N>, a: &[u64; N], b: &[u64; N]) -> [u64; N];
    fn sqr<const N: usize>(ctx: &Mont<N>, a: &[u64; N]) -> [u64; N];
}

/// The pre-change kernel: interleaved CIOS for products and squares alike.
pub(crate) struct CiosKernel;
/// CIOS products, dedicated squaring ([`Mont::sqr`]).
pub(crate) struct SqrKernel;
/// One-level Karatsuba products ([`Mont::mul_karatsuba`]) and dedicated squaring.
pub(crate) struct KaratsubaKernel;

impl MulKernel for CiosKernel {
    #[inline(always)]
    fn mul<const N: usize>(ctx: &Mont<N>, a: &[u64; N], b: &[u64; N]) -> [u64; N] {
        ctx.mul(a, b)
    }
    #[inline(always)]
    fn sqr<const N: usize>(ctx: &Mont<N>, a: &[u64; N]) -> [u64; N] {
        ctx.mul(a, a)
    }
}

impl MulKernel for SqrKernel {
    #[inline(always)]
    fn mul<const N: usize>(ctx: &Mont<N>, a: &[u64; N], b: &[u64; N]) -> [u64; N] {
        ctx.mul(a, b)
    }
    #[inline(always)]
    fn sqr<const N: usize>(ctx: &Mont<N>, a: &[u64; N]) -> [u64; N] {
        ctx.sqr(a)
    }
}

impl MulKernel for KaratsubaKernel {
    #[inline(always)]
    fn mul<const N: usize>(ctx: &Mont<N>, a: &[u64; N], b: &[u64; N]) -> [u64; N] {
        ctx.mul_karatsuba(a, b)
    }
    #[inline(always)]
    fn sqr<const N: usize>(ctx: &Mont<N>, a: &[u64; N]) -> [u64; N] {
        ctx.sqr(a)
    }
}

/// Width-tuned kernel from the 2026-11-25 ablation (`docs/notes/
/// mont-mul-kernels-20261125/`, local Xeon, single core): the separated
/// schoolbook + SOS product beat CIOS by 2–5% at `N ≤ 5` and lost 7–24% at
/// `N ≥ 6`; dedicated squaring beat CIOS `a·a` by 3–13% at `N ≥ 4` and lost
/// 12% at `N = 3`.  `N` is a constant, so the choice compiles away.
pub(crate) struct TunedKernel;

impl MulKernel for TunedKernel {
    #[inline(always)]
    fn mul<const N: usize>(ctx: &Mont<N>, a: &[u64; N], b: &[u64; N]) -> [u64; N] {
        if N <= 5 {
            ctx.redc(Mont::<N>::mul_wide(a, b))
        } else {
            ctx.mul(a, b)
        }
    }
    #[inline(always)]
    fn sqr<const N: usize>(ctx: &Mont<N>, a: &[u64; N]) -> [u64; N] {
        if N >= 4 {
            ctx.sqr(a)
        } else {
            ctx.mul(a, a)
        }
    }
}

/// The kernel [`Point::scalar_mul`](super::point::Point::scalar_mul) uses.
pub(crate) type ProdKernel = TunedKernel;

/// A Jacobian point `(X : Y : Z)` in Montgomery form, `Z ≠ 0` not assumed.
#[derive(Clone, Copy)]
pub(crate) struct Jac<const N: usize> {
    pub x: [u64; N],
    pub y: [u64; N],
    pub z: [u64; N],
}

impl<const N: usize> Mont<N> {
    #[inline(always)]
    fn is_zero(a: &[u64; N]) -> bool {
        a.iter().all(|&w| w == 0)
    }

    /// `2P` as `Jacobian::double` of `point.rs` (dbl-2007-bl) on the kernel
    /// `K` (squares go to `K::sqr`): `None` when `Y = 0`.
    #[inline(always)]
    pub(crate) fn double_k<K: MulKernel>(&self, j: &Jac<N>, a: &[u64; N]) -> Option<Jac<N>> {
        if Self::is_zero(&j.y) {
            return None;
        }
        let xx = K::sqr(self, &j.x);
        let yy = K::sqr(self, &j.y);
        let yyyy = K::sqr(self, &yy);
        let zz = K::sqr(self, &j.z);
        let t = self.add(&j.x, &yy);
        let s = self.sub(&self.sub(&K::sqr(self, &t), &xx), &yyyy);
        let s = self.add(&s, &s);
        let m = self.add(
            &self.add(&self.add(&xx, &xx), &xx),
            &K::mul(self, a, &K::sqr(self, &zz)),
        );
        let x3 = self.sub(&self.sub(&K::sqr(self, &m), &s), &s);
        let yyyy8 = {
            let d = self.add(&yyyy, &yyyy);
            let q = self.add(&d, &d);
            self.add(&q, &q)
        };
        let y3 = self.sub(&K::mul(self, &m, &self.sub(&s, &x3)), &yyyy8);
        let yz = self.add(&j.y, &j.z);
        let z3 = self.sub(&self.sub(&K::sqr(self, &yz), &yy), &zz);
        Some(Jac {
            x: x3,
            y: y3,
            z: z3,
        })
    }

    /// `P + (x₂, y₂)` as `Jacobian::add_affine` (madd-2007-bl) on the
    /// kernel `K`.
    #[inline(always)]
    pub(crate) fn add_affine_k<K: MulKernel>(
        &self,
        j: &Jac<N>,
        x2: &[u64; N],
        y2: &[u64; N],
        a: &[u64; N],
    ) -> Option<Jac<N>> {
        let z1z1 = K::sqr(self, &j.z);
        let u2 = K::mul(self, x2, &z1z1);
        let s2 = K::mul(self, &K::mul(self, y2, &j.z), &z1z1);
        let h = self.sub(&u2, &j.x);
        let r = self.sub(&s2, &j.y);
        if Self::is_zero(&h) {
            return if Self::is_zero(&r) {
                self.double_k::<K>(j, a)
            } else {
                None
            };
        }
        let r = self.add(&r, &r);
        let hh = K::sqr(self, &h);
        let i = self.add(&hh, &hh);
        let i = self.add(&i, &i);
        let jj = K::mul(self, &h, &i);
        let v = K::mul(self, &j.x, &i);
        let x3 = self.sub(&self.sub(&self.sub(&K::sqr(self, &r), &jj), &v), &v);
        let y1j = K::mul(self, &j.y, &jj);
        let y3 = self.sub(&self.sub(&K::mul(self, &r, &self.sub(&v, &x3)), &y1j), &y1j);
        let zh = self.add(&j.z, &h);
        let z3 = self.sub(&self.sub(&K::sqr(self, &zh), &z1z1), &hh);
        Some(Jac {
            x: x3,
            y: y3,
            z: z3,
        })
    }

    /// `k·(x, y)` by left-to-right binary double-and-add on the kernel `K`,
    /// as a Jacobian point (`None` is the identity).  The operation
    /// sequence is the one of the `BigUint` ladder in `Point::scalar_mul`.
    pub(crate) fn ladder_binary<K: MulKernel>(
        &self,
        xm: &[u64; N],
        ym: &[u64; N],
        am: &[u64; N],
        k: &BigUint,
    ) -> Option<Jac<N>> {
        let mut acc: Option<Jac<N>> = None;
        let digits: Vec<u64> = k.iter_u64_digits().collect();
        for i in (0..k.bits()).rev() {
            if let Some(j) = &acc {
                acc = self.double_k::<K>(j, am);
            }
            if (digits[(i / 64) as usize] >> (i % 64)) & 1 == 1 {
                acc = match &acc {
                    None => Some(Jac {
                        x: *xm,
                        y: *ym,
                        z: self.one(),
                    }),
                    Some(j) => self.add_affine_k::<K>(j, xm, ym, am),
                };
            }
        }
        acc
    }

    /// `k·(x, y)` by width-`w` NAF (`2 ≤ w ≤ 6`) on the kernel `K`, with
    /// about `bits/(w+1)` mixed additions in place of the binary ladder's
    /// `bits/2`.  Variable time.
    ///
    /// The odd multiples `P, 3P, …, (2^(w−1) − 1)P` are built without
    /// inverting `2P`: with `2P = (X₂ : Y₂ : Z₂)`, the isomorphism
    /// `φ(x, y) = (Z₂²x, Z₂³y)` to `y² = x³ + a·Z₂⁴·x + b·Z₂⁶` sends `2P` to
    /// the affine `(X₂, Y₂)`, so the table is built by mixed additions on
    /// the image curve and made affine by one batch (Montgomery-trick)
    /// inversion; the ladder runs on the image curve, and its result
    /// `(X : Y : Z)` is `(X : Y : Z·Z₂)` on the original one.  Returns the
    /// Jacobian result (`Some(None)` is the identity); declines (`None`)
    /// when `2P` or a table entry is the identity or a `Z` is not a unit
    /// (a small-order base, a composite modulus), where the caller keeps
    /// the binary ladder.  Over a prime field the point equals
    /// [`ladder_binary`](Self::ladder_binary)'s.
    pub(crate) fn ladder_wnaf<K: MulKernel>(
        &self,
        xm: &[u64; N],
        ym: &[u64; N],
        am: &[u64; N],
        k: &BigUint,
        w: usize,
    ) -> Option<Option<Jac<N>>> {
        debug_assert!((2..=6).contains(&w));
        let naf = wnaf_digits(k, w);
        let tsize = 1usize << (w - 2);
        let p1 = Jac {
            x: *xm,
            y: *ym,
            z: self.one(),
        };
        let d = self.double_k::<K>(&p1, am)?;
        if Self::is_zero(&d.z) {
            return None;
        }
        // Image curve: a' = a·Z₂⁴, φ(P) = (Z₂²x, Z₂³y), φ(2P) = (X₂, Y₂).
        let z2sq = K::sqr(self, &d.z);
        let z2cu = K::mul(self, &z2sq, &d.z);
        let a_img = K::mul(self, am, &K::sqr(self, &z2sq));
        let mut tx = vec![K::mul(self, xm, &z2sq); tsize];
        let mut ty = vec![K::mul(self, ym, &z2cu); tsize];
        if tsize > 1 {
            let mut jacs: Vec<Jac<N>> = Vec::with_capacity(tsize - 1);
            let mut cur = Jac {
                x: tx[0],
                y: ty[0],
                z: self.one(),
            };
            for _ in 1..tsize {
                cur = self.add_affine_k::<K>(&cur, &d.x, &d.y, &a_img)?;
                jacs.push(cur);
            }
            let mut prefix = Vec::with_capacity(jacs.len());
            let mut accz = self.one();
            for j in &jacs {
                accz = K::mul(self, &accz, &j.z);
                prefix.push(accz);
            }
            let mut inv = self.inv_bingcd(&accz)?;
            for i in (0..jacs.len()).rev() {
                let zi = if i == 0 {
                    inv
                } else {
                    K::mul(self, &inv, &prefix[i - 1])
                };
                inv = K::mul(self, &inv, &jacs[i].z);
                let zi2 = K::sqr(self, &zi);
                let zi3 = K::mul(self, &zi2, &zi);
                tx[i + 1] = K::mul(self, &jacs[i].x, &zi2);
                ty[i + 1] = K::mul(self, &jacs[i].y, &zi3);
            }
        }
        let zero = [0u64; N];
        let mut acc: Option<Jac<N>> = None;
        for &dg in naf.iter().rev() {
            if let Some(j) = &acc {
                acc = self.double_k::<K>(j, &a_img);
            }
            if dg != 0 {
                let idx = (dg.unsigned_abs() as usize) >> 1;
                let x = &tx[idx];
                let y = if dg > 0 {
                    ty[idx]
                } else {
                    self.sub(&zero, &ty[idx])
                };
                acc = match &acc {
                    None => Some(Jac {
                        x: *x,
                        y,
                        z: self.one(),
                    }),
                    Some(j) => self.add_affine_k::<K>(j, x, &y, &a_img),
                };
            }
        }
        Some(acc.map(|j| Jac {
            z: K::mul(self, &j.z, &d.z),
            ..j
        }))
    }
}

/// Width-`w` non-adjacent form of `k`, least significant digit first: every
/// nonzero digit is odd with `|d| < 2^(w−1)`, any two nonzero digits are at
/// least `w` apart, and `Σ dᵢ·2^i = k`.
pub(crate) fn wnaf_digits(k: &BigUint, w: usize) -> Vec<i8> {
    let mut x: Vec<u64> = k.iter_u64_digits().collect();
    let n = x.len();
    x.push(0);
    x.push(0);
    let limit = 64 * (n + 1);
    let mut naf = vec![0i8; limit];
    let width = 1u64 << w;
    let mask = width - 1;
    let mut pos = 0usize;
    let mut carry = 0u64;
    while pos < limit {
        let idx = pos / 64;
        let bit = pos % 64;
        let buf = if bit < 64 - w {
            x[idx] >> bit
        } else {
            (x[idx] >> bit) | (x[idx + 1] << (64 - bit))
        };
        let window = carry + (buf & mask);
        if window & 1 == 0 {
            pos += 1;
            continue;
        }
        if window < width / 2 {
            carry = 0;
            naf[pos] = window as i8;
        } else {
            carry = 1;
            naf[pos] = (window as i64 - width as i64) as i8;
        }
        pos += w;
    }
    while naf.last() == Some(&0) {
        naf.pop();
    }
    naf
}

/// Constant-time Montgomery product of four-word residues: `a·b·R⁻¹ mod p`
/// for `a, b < p`, `R = 2^256`, `n0 = -p⁻¹ mod 2^64`, in `[0, p)` — the
/// value of [`Uint::mont_mul`](crate::ct_bignum::Uint::mont_mul) for every
/// such input, by one interleaved (CIOS) pass instead of a full product
/// followed by a reduction, with every loop bound fixed and the final
/// subtraction selected by a mask (no branch and no table index depends
/// on a value), as in `ct_bignum`.
#[inline(always)]
pub(crate) fn ct_mont_mul4(a: &[u64; 4], b: &[u64; 4], p: &[u64; 4], n0: u64) -> [u64; 4] {
    let mut t = [0u64; 4];
    let mut t4: u64 = 0;
    for i in 0..4 {
        let bi = b[i] as u128;
        let mut carry: u128 = 0;
        for j in 0..4 {
            let s = t[j] as u128 + a[j] as u128 * bi + carry;
            t[j] = s as u64;
            carry = s >> 64;
        }
        let s = t4 as u128 + carry;
        t4 = s as u64;
        let t5 = (s >> 64) as u64;

        let m = t[0].wrapping_mul(n0) as u128;
        let s = t[0] as u128 + m * p[0] as u128;
        let mut carry = s >> 64;
        for j in 1..4 {
            let s = t[j] as u128 + m * p[j] as u128 + carry;
            t[j - 1] = s as u64;
            carry = s >> 64;
        }
        let s = t4 as u128 + carry;
        t[3] = s as u64;
        t4 = t5 + (s >> 64) as u64;
    }
    // t4·2^256 + t < 2p: subtract p when t4 = 1 or t >= p.
    let mut d = [0u64; 4];
    let mut borrow = 0u64;
    for i in 0..4 {
        let (v, b1) = t[i].overflowing_sub(p[i]);
        let (v, b2) = v.overflowing_sub(borrow);
        d[i] = v;
        borrow = (b1 | b2) as u64;
    }
    let need = t4 | (borrow ^ 1);
    let mask = 0u64.wrapping_sub(need & 1);
    [
        (t[0] & !mask) | (d[0] & mask),
        (t[1] & !mask) | (d[1] & mask),
        (t[2] & !mask) | (d[2] & mask),
        (t[3] & !mask) | (d[3] & mask),
    ]
}

/// `base^exp mod p` for a canonical `base < p` and a 256-bit `exp`, by the
/// fixed 256-iteration square-and-always-multiply ladder of
/// [`mont_pow_ct`](crate::ct_bignum::mont_pow_ct) (the product is always
/// computed and kept or dropped by a mask) on [`ct_mont_mul4`]; `r2` is
/// `R² mod p` and `one` is `R mod p`.  The result is the canonical power.
pub(crate) fn ct_pow4(
    base: &[u64; 4],
    exp: &[u64; 4],
    p: &[u64; 4],
    n0: u64,
    r2: &[u64; 4],
    one: &[u64; 4],
) -> [u64; 4] {
    let base_m = ct_mont_mul4(base, r2, p, n0);
    let mut acc = *one;
    for i in (0..256).rev() {
        acc = ct_mont_mul4(&acc, &acc, p, n0);
        let prod = ct_mont_mul4(&acc, &base_m, p, n0);
        let mask = 0u64.wrapping_sub((exp[i / 64] >> (i % 64)) & 1);
        for k in 0..4 {
            acc[k] = (acc[k] & !mask) | (prod[k] & mask);
        }
    }
    ct_mont_mul4(&acc, &[1, 0, 0, 0], p, n0)
}

/// [`ct_pow4`] for a **public** exponent: the same power, by a fixed
/// four-bit window (about 256 squarings and 64 products against the
/// ladder's 256 and 256).  The sequence of operations follows the bits of
/// `exp`, so `exp` must not be secret; it does not depend on `base`.
pub(crate) fn ct_pow4_public(
    base: &[u64; 4],
    exp: &[u64; 4],
    p: &[u64; 4],
    n0: u64,
    r2: &[u64; 4],
    one: &[u64; 4],
) -> [u64; 4] {
    let base_m = ct_mont_mul4(base, r2, p, n0);
    let mut table = [*one; 16];
    table[1] = base_m;
    for i in 2..16 {
        table[i] = ct_mont_mul4(&table[i - 1], &base_m, p, n0);
    }
    let mut acc = *one;
    let mut started = false;
    for w in (0..64).rev() {
        let nib = ((exp[w / 16] >> (4 * (w % 16))) & 15) as usize;
        if started {
            for _ in 0..4 {
                acc = ct_mont_mul4(&acc, &acc, p, n0);
            }
        }
        if nib != 0 {
            acc = if started {
                ct_mont_mul4(&acc, &table[nib], p, n0)
            } else {
                table[nib]
            };
            started = true;
        }
    }
    ct_mont_mul4(&acc, &[1, 0, 0, 0], p, n0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ct_bignum::{compute_minv64, Uint, U256};
    use num_bigint::RandBigInt;
    use num_traits::Zero;
    use rand::{rngs::StdRng, SeedableRng};

    fn moduli() -> Vec<BigUint> {
        let two = |e: u32| BigUint::from(1u8) << e;
        vec![
            crate::ecc::curve::CurveParams::p256().p.clone(),
            crate::ecc::curve::CurveParams::secp256k1().p.clone(),
            two(256) - 189u32,
            two(256) - 1u32,
            two(255) + 95u32,
            two(192) + 133u32,
            BigUint::from(3u32),
        ]
    }

    /// `ct_mont_mul4` is `Uint::mont_mul`, on random and edge operands.
    #[test]
    fn ct_mont_mul4_matches_ct_bignum() {
        let mut rng = StdRng::seed_from_u64(0x6d75_6c34);
        for m in moduli() {
            let p = U256::from_biguint(&m);
            let n0 = compute_minv64(p.0[0]);
            let mut vals = vec![BigUint::from(0u8), BigUint::from(1u8), &m - 1u32, &m - 2u32];
            for _ in 0..200 {
                vals.push(rng.gen_biguint_below(&m));
            }
            for a in &vals {
                for b in vals.iter().take(24) {
                    let (ua, ub) = (U256::from_biguint(a), U256::from_biguint(b));
                    assert_eq!(
                        ct_mont_mul4(&ua.0, &ub.0, &p.0, n0),
                        Uint::mont_mul(&ua, &ub, &p, n0).0,
                        "m={m} a={a} b={b}"
                    );
                }
            }
        }
    }

    /// `ct_pow4_public` is `ct_pow4`, on random, sparse and edge exponents.
    #[test]
    fn ct_pow4_public_matches_ct_pow4() {
        let mut rng = StdRng::seed_from_u64(0x7075_6232);
        for m in moduli().into_iter().take(4) {
            let p = U256::from_biguint(&m);
            let n0 = compute_minv64(p.0[0]);
            let one = U256::from_biguint(&((BigUint::from(1u8) << 256) % &m));
            let r2 = U256::from_biguint(&((BigUint::from(1u8) << 512) % &m));
            let mut exps = vec![
                BigUint::from(0u8),
                BigUint::from(1u8),
                BigUint::from(15u8),
                BigUint::from(16u8),
                &m - 2u32,
                &m - 1u32,
                (BigUint::from(1u8) << 256) - 1u32,
                BigUint::from(1u8) << 255,
            ];
            for _ in 0..20 {
                exps.push(rng.gen_biguint(256));
            }
            for e in exps {
                let b = U256::from_biguint(&rng.gen_biguint_below(&m));
                let e = U256::from_biguint(&e);
                assert_eq!(
                    ct_pow4_public(&b.0, &e.0, &p.0, n0, &r2.0, &one.0),
                    ct_pow4(&b.0, &e.0, &p.0, n0, &r2.0, &one.0)
                );
            }
        }
    }

    /// `ct_pow4` is `mont_pow_ct`.
    #[test]
    fn ct_pow4_matches_mont_pow_ct() {
        use crate::ct_bignum::{mont_pow_ct, MontgomeryContext};
        let mut rng = StdRng::seed_from_u64(0x706f_7734);
        for m in moduli().into_iter().take(4) {
            let ctx = MontgomeryContext::new(U256::from_biguint(&m)).unwrap();
            for _ in 0..24 {
                let b = U256::from_biguint(&rng.gen_biguint_below(&m));
                let e = U256::from_biguint(&rng.gen_biguint(256));
                assert_eq!(
                    ct_pow4(
                        &b.0,
                        &e.0,
                        &ctx.n.0,
                        ctx.n_inv_low,
                        &ctx.r2_mod_n.0,
                        &ctx.r_mod_n.0
                    ),
                    mont_pow_ct(&b, &e, &ctx).0
                );
            }
        }
    }

    /// The generic variable-time product against `BigUint`, every width.
    fn generic_mul_matches<const N: usize>(m: &BigUint, rng: &mut StdRng) {
        let ctx = Mont::<N>::new(m).unwrap();
        for _ in 0..200 {
            let (a, b) = (rng.gen_biguint_below(m), rng.gen_biguint_below(m));
            let (am, bm) = (ctx.enter(&a).unwrap(), ctx.enter(&b).unwrap());
            assert_eq!(ctx.leave(&ctx.mul(&am, &bm)), (&a * &b) % m);
            assert_eq!(ctx.leave(&ctx.add(&am, &bm)), (&a + &b) % m);
            assert_eq!(ctx.leave(&ctx.sub(&am, &bm)), (m + &a - &b) % m);
        }
    }

    /// Dedicated squaring and Karatsuba products equal CIOS on random and
    /// edge residues, every width 1..=9 (Karatsuba runs for even N ≥ 4 and
    /// falls back elsewhere).
    fn kernels_match<const N: usize>(m: &BigUint, rng: &mut StdRng) {
        let ctx = Mont::<N>::new(m).unwrap();
        let mut vals = vec![
            BigUint::from(0u8),
            BigUint::from(1u8),
            m - 1u32,
            m - 2u32,
            m >> 1,
        ];
        for _ in 0..300 {
            vals.push(rng.gen_biguint_below(m));
        }
        let ms: Vec<_> = vals.iter().map(|v| ctx.enter(v).unwrap()).collect();
        for (i, a) in ms.iter().enumerate() {
            assert_eq!(ctx.sqr(a), ctx.mul(a, a), "sqr N={N} m={m} a={}", vals[i]);
            for b in ms.iter().take(12) {
                assert_eq!(ctx.mul_karatsuba(a, b), ctx.mul(a, b), "kara N={N} m={m}");
            }
            let wide = Mont::<N>::mul_wide_karatsuba(a, &ms[i / 2]);
            assert_eq!(wide, Mont::<N>::mul_wide(a, &ms[i / 2]), "wide N={N}");
            assert_eq!(
                Mont::<N>::sqr_wide(a),
                Mont::<N>::mul_wide(a, a),
                "sqr_wide N={N}"
            );
        }
        // All-ones words exercise every carry chain of the wide products.
        let ones = [u64::MAX; N];
        assert_eq!(
            Mont::<N>::sqr_wide(&ones),
            Mont::<N>::mul_wide(&ones, &ones)
        );
        assert_eq!(
            Mont::<N>::mul_wide_karatsuba(&ones, &ones),
            Mont::<N>::mul_wide(&ones, &ones)
        );
    }

    #[test]
    fn squaring_and_karatsuba_match_cios() {
        let mut rng = StdRng::seed_from_u64(0x6b61_7261);
        let two = |e: u32| BigUint::from(1u8) << e;
        kernels_match::<1>(&BigUint::from(u64::MAX - 58), &mut rng);
        kernels_match::<2>(&(two(127) - 1u32), &mut rng);
        kernels_match::<2>(&(two(128) - 159u32), &mut rng);
        kernels_match::<3>(&(two(192) - two(64) - 1u32), &mut rng);
        kernels_match::<4>(&(two(256) - 189u32), &mut rng);
        kernels_match::<4>(&(two(255) - 19u32), &mut rng);
        kernels_match::<4>(&(two(200) + 235u32), &mut rng);
        kernels_match::<5>(&(two(320) - 197u32), &mut rng);
        kernels_match::<6>(&(two(384) - two(128) - two(96) + two(32) - 1u32), &mut rng);
        kernels_match::<6>(&(two(321) + 9u32), &mut rng);
        kernels_match::<7>(&(two(448) - two(224) - 1u32), &mut rng);
        kernels_match::<8>(&(two(512) - 569u32), &mut rng);
        kernels_match::<8>(&(two(512) - 1u32), &mut rng);
        kernels_match::<9>(&(two(521) - 1u32), &mut rng);
        kernels_match::<9>(&(two(576) - 3u32), &mut rng);
    }

    /// The binary-GCD inverse is the Fermat inverse on primes, and `None`
    /// exactly on non-units of a composite modulus.
    #[test]
    fn bingcd_inverse_matches_fermat() {
        let mut rng = StdRng::seed_from_u64(0x6269_6e67);
        fn check<const N: usize>(m: &BigUint, rng: &mut StdRng) {
            let ctx = Mont::<N>::new(m).unwrap();
            let e = m - 2u32;
            let mut vals = vec![BigUint::from(1u8), BigUint::from(2u8), m - 1u32, m >> 1];
            for _ in 0..60 {
                vals.push(rng.gen_biguint_below(m));
            }
            for v in vals.iter().filter(|v| !v.is_zero()) {
                let am = ctx.enter(v).unwrap();
                assert_eq!(ctx.inv_bingcd(&am), Some(ctx.pow(&am, &e)), "m={m} v={v}");
            }
            assert_eq!(ctx.inv_bingcd(&[0u64; N]), None);
        }
        let two = |e: u32| BigUint::from(1u8) << e;
        check::<2>(&(two(127) - 1u32), &mut rng);
        check::<3>(&(two(192) - two(64) - 1u32), &mut rng);
        check::<4>(&(two(255) - 19u32), &mut rng);
        check::<4>(&crate::ecc::curve::CurveParams::p256().p.clone(), &mut rng);
        check::<5>(&(two(320) - 197u32), &mut rng);
        check::<7>(&(two(448) - two(224) - 1u32), &mut rng);
        check::<8>(&(two(512) - 569u32), &mut rng);
        check::<9>(&(two(521) - 1u32), &mut rng);
        // Composite 3·(2^127 − 1): multiples of 3 have no inverse.
        let m = BigUint::from(3u8) * (two(127) - 1u32);
        let ctx = Mont::<2>::new(&m).unwrap();
        let am = ctx.enter(&BigUint::from(6u8)).unwrap();
        assert_eq!(ctx.inv_bingcd(&am), None);
        let bm = ctx.enter(&BigUint::from(5u8)).unwrap();
        let inv = ctx.inv_bingcd(&bm).unwrap();
        assert_eq!(ctx.leave(&ctx.mul(&inv, &bm)), BigUint::from(1u8));
    }

    /// wNAF digits reconstruct the scalar and satisfy the width rules.
    #[test]
    fn wnaf_digits_reconstruct() {
        use num_bigint::BigInt;
        let mut rng = StdRng::seed_from_u64(0x776e_6166);
        let mut ks: Vec<BigUint> = (0u32..300).map(BigUint::from).collect();
        ks.push((BigUint::from(1u8) << 521) - 1u32);
        ks.push(BigUint::from(u64::MAX));
        for bits in [63u64, 64, 65, 127, 128, 255, 256, 521, 576] {
            for _ in 0..20 {
                ks.push(rng.gen_biguint(bits));
            }
        }
        for w in 2..=6usize {
            for k in &ks {
                let naf = wnaf_digits(k, w);
                let mut acc = BigInt::from(0);
                let mut last: Option<usize> = None;
                for (i, &d) in naf.iter().enumerate().rev() {
                    acc = (acc << 1) + BigInt::from(d);
                    if d != 0 {
                        assert!(d % 2 != 0 && (d.unsigned_abs() as u64) < (1u64 << (w - 1)));
                        if let Some(l) = last {
                            assert!(l - i >= w, "w={w} k={k}");
                        }
                        last = Some(i);
                    }
                }
                assert_eq!(acc, BigInt::from(k.clone()), "w={w}");
            }
        }
    }

    #[test]
    fn generic_arithmetic_matches_biguint() {
        let mut rng = StdRng::seed_from_u64(0x6765_6e31);
        let two = |e: u32| BigUint::from(1u8) << e;
        generic_mul_matches::<1>(&BigUint::from(u64::MAX), &mut rng);
        generic_mul_matches::<1>(&BigUint::from(97u32), &mut rng);
        generic_mul_matches::<2>(&(two(128) - 159u32), &mut rng);
        generic_mul_matches::<4>(&(two(256) - 189u32), &mut rng);
        generic_mul_matches::<4>(&(two(200) + 235u32), &mut rng);
        generic_mul_matches::<9>(&(two(521) - 1u32), &mut rng);
        generic_mul_matches::<9>(&(two(576) - 3u32), &mut rng);
    }
}
