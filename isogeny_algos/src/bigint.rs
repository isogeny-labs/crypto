//! Minimal unsigned big integers (little-endian u64 limbs) for exponents, field sizes and moduli.
//! Only what the field code needs; not performance critical.
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Big(pub Vec<u64>);

impl Big {
    pub fn zero() -> Big {
        Big(vec![])
    }
    pub fn from_u64(v: u64) -> Big {
        Big::norm(vec![v])
    }
    pub fn from_u128(v: u128) -> Big {
        Big::norm(vec![v as u64, (v >> 64) as u64])
    }
    pub fn from_limbs(l: &[u64]) -> Big {
        Big::norm(l.to_vec())
    }
    fn norm(mut v: Vec<u64>) -> Big {
        while v.last() == Some(&0) {
            v.pop();
        }
        Big(v)
    }
    pub fn is_zero(&self) -> bool {
        self.0.is_empty()
    }
    pub fn bits(&self) -> usize {
        match self.0.last() {
            None => 0,
            Some(&t) => 64 * (self.0.len() - 1) + (64 - t.leading_zeros() as usize),
        }
    }
    pub fn bit(&self, i: usize) -> bool {
        let (w, b) = (i / 64, i % 64);
        w < self.0.len() && (self.0[w] >> b) & 1 == 1
    }
    pub fn limbs(&self, n: usize) -> Vec<u64> {
        let mut v = self.0.clone();
        v.resize(n, 0);
        v
    }
    pub fn to_u128(&self) -> Option<u128> {
        if self.0.len() > 2 {
            return None;
        }
        let l = self.limbs(2);
        Some(l[0] as u128 | (l[1] as u128) << 64)
    }
    pub fn add(&self, o: &Big) -> Big {
        let n = self.0.len().max(o.0.len());
        let (a, b) = (self.limbs(n), o.limbs(n));
        let mut r = Vec::with_capacity(n + 1);
        let mut c = 0u128;
        for i in 0..n {
            let s = a[i] as u128 + b[i] as u128 + c;
            r.push(s as u64);
            c = s >> 64;
        }
        r.push(c as u64);
        Big::norm(r)
    }
    /// self - o, requires self >= o.
    pub fn sub(&self, o: &Big) -> Big {
        assert!(self.cmp(o) != Ordering::Less, "Big::sub underflow");
        let n = self.0.len();
        let b = o.limbs(n);
        let mut r = Vec::with_capacity(n);
        let mut borrow = 0u64;
        for i in 0..n {
            let (d1, o1) = self.0[i].overflowing_sub(b[i]);
            let (d2, o2) = d1.overflowing_sub(borrow);
            r.push(d2);
            borrow = (o1 || o2) as u64;
        }
        Big::norm(r)
    }
    pub fn mul(&self, o: &Big) -> Big {
        if self.is_zero() || o.is_zero() {
            return Big::zero();
        }
        Big::norm(mul_limbs(&self.0, &o.0))
    }
    /// Quadratic product, kept as an independent oracle for the Karatsuba path.
    pub fn mul_schoolbook(&self, o: &Big) -> Big {
        if self.is_zero() || o.is_zero() {
            return Big::zero();
        }
        let mut r = vec![0u64; self.0.len() + o.0.len()];
        schoolbook_into(&mut r, &self.0, &o.0);
        Big::norm(r)
    }
    pub fn mul_small(&self, m: u64) -> Big {
        self.mul(&Big::from_u64(m))
    }
    pub fn add_small(&self, m: u64) -> Big {
        self.add(&Big::from_u64(m))
    }
    pub fn sub_small(&self, m: u64) -> Big {
        self.sub(&Big::from_u64(m))
    }
    pub fn shr(&self, k: usize) -> Big {
        let (w, b) = (k / 64, k % 64);
        if w >= self.0.len() {
            return Big::zero();
        }
        let src = &self.0[w..];
        let mut r = vec![0u64; src.len()];
        for i in 0..src.len() {
            r[i] = src[i] >> b;
            if b > 0 && i + 1 < src.len() {
                r[i] |= src[i + 1] << (64 - b);
            }
        }
        Big::norm(r)
    }
    /// (quotient, remainder) by a small divisor.
    pub fn divrem_small(&self, d: u64) -> (Big, u64) {
        let mut q = vec![0u64; self.0.len()];
        let mut r = 0u128;
        for i in (0..self.0.len()).rev() {
            let cur = (r << 64) | self.0[i] as u128;
            q[i] = (cur / d as u128) as u64;
            r = cur % d as u128;
        }
        (Big::norm(q), r as u64)
    }
    pub fn rem_small(&self, d: u64) -> u64 {
        self.divrem_small(d).1
    }
    pub fn shl(&self, k: usize) -> Big {
        if self.is_zero() {
            return Big::zero();
        }
        let (w, b) = (k / 64, k % 64);
        let mut r = vec![0u64; self.0.len() + w + 1];
        for (i, &x) in self.0.iter().enumerate() {
            r[i + w] |= x << b;
            if b > 0 {
                r[i + w + 1] |= x >> (64 - b);
            }
        }
        Big::norm(r)
    }
    /// (quotient, remainder) by Knuth's Algorithm D (64-bit limbs, normalised divisor).
    pub fn divrem(&self, d: &Big) -> (Big, Big) {
        assert!(!d.is_zero(), "division by zero");
        if self < d {
            return (Big::zero(), self.clone());
        }
        if d.0.len() == 1 {
            let (q, r) = self.divrem_small(d.0[0]);
            return (q, Big::from_u64(r));
        }
        let n = d.0.len();
        let m = self.0.len() - n;
        let s = d.0[n - 1].leading_zeros();
        let shift = |v: &[u64], len: usize| -> Vec<u64> {
            let mut r = vec![0u64; len];
            for i in 0..v.len() {
                r[i] |= v[i] << s;
                if s > 0 && i + 1 < len {
                    r[i + 1] |= v[i] >> (64 - s);
                }
            }
            r
        };
        let vn = shift(&d.0, n);
        let mut un = shift(&self.0, m + n + 1);
        let mut q = vec![0u64; m + 1];
        let b: u128 = 1 << 64;
        for j in (0..=m).rev() {
            let num = ((un[j + n] as u128) << 64) | un[j + n - 1] as u128;
            let mut qhat = num / vn[n - 1] as u128;
            let mut rhat = num % vn[n - 1] as u128;
            while qhat >= b || qhat * vn[n - 2] as u128 > ((rhat << 64) | un[j + n - 2] as u128) {
                qhat -= 1;
                rhat += vn[n - 1] as u128;
                if rhat >= b {
                    break;
                }
            }
            // un[j..j+n+1] -= qhat * vn
            let mut k: i128 = 0;
            for i in 0..n {
                let p = qhat * vn[i] as u128;
                let t = un[i + j] as i128 - k - (p as u64) as i128;
                un[i + j] = t as u64;
                k = (p >> 64) as i128 - (t >> 64);
            }
            let t = un[j + n] as i128 - k;
            un[j + n] = t as u64;
            if t < 0 {
                qhat -= 1;
                let mut c = 0u128;
                for i in 0..n {
                    let s2 = un[i + j] as u128 + vn[i] as u128 + c;
                    un[i + j] = s2 as u64;
                    c = s2 >> 64;
                }
                un[j + n] = un[j + n].wrapping_add(c as u64);
            }
            q[j] = qhat as u64;
        }
        let mut r = vec![0u64; n];
        for i in 0..n {
            r[i] = un[i] >> s;
            if s > 0 {
                r[i] |= un[i + 1] << (64 - s);
            }
        }
        (Big::norm(q), Big::norm(r))
    }
    /// floor(sqrt(self)) by Newton's iteration.
    pub fn isqrt(&self) -> Big {
        if self.is_zero() {
            return Big::zero();
        }
        let mut x = Big::from_u64(1).shl(self.bits().div_ceil(2));
        loop {
            let y = x.add(&self.divrem(&x).0).shr(1);
            if y >= x {
                return x;
            }
            x = y;
        }
    }
    pub fn from_dec(s: &str) -> Big {
        let mut r = Big::zero();
        for ch in s.chars().filter(|c| !c.is_whitespace() && *c != '_') {
            let d = ch.to_digit(10).expect("decimal digit") as u64;
            r = r.mul_small(10).add_small(d);
        }
        r
    }
    pub fn to_dec(&self) -> String {
        if self.is_zero() {
            return "0".into();
        }
        let mut digits = vec![];
        let mut cur = self.clone();
        while !cur.is_zero() {
            let (q, r) = cur.divrem_small(10_000_000_000_000_000_000);
            digits.push(r);
            cur = q;
        }
        let mut s = digits.last().unwrap().to_string();
        for d in digits.iter().rev().skip(1) {
            s.push_str(&format!("{:019}", d));
        }
        s
    }
}

/// Operand length (in limbs) below which `mul_limbs` uses the quadratic loop.
pub const KARATSUBA_LIMBS: usize = 32;

/// `r = a*b` for zeroed `r` of length `a.len()+b.len()`.
fn schoolbook_into(r: &mut [u64], a: &[u64], b: &[u64]) {
    debug_assert_eq!(r.len(), a.len() + b.len());
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        let row = &mut r[i..i + b.len() + 1];
        let mut c = 0u64;
        for (rj, &y) in row.iter_mut().zip(b) {
            let s = *rj as u128 + x as u128 * y as u128 + c as u128;
            *rj = s as u64;
            c = (s >> 64) as u64;
        }
        row[b.len()] = c;
    }
}

/// `acc += x`, returning the carry out of `acc`.
fn add_into(acc: &mut [u64], x: &[u64]) -> u64 {
    let mut c = 0u64;
    for (i, a) in acc.iter_mut().enumerate() {
        let y = x.get(i).copied().unwrap_or(0);
        if i >= x.len() && c == 0 {
            break;
        }
        let (s1, o1) = a.overflowing_add(y);
        let (s2, o2) = s1.overflowing_add(c);
        *a = s2;
        c = (o1 | o2) as u64;
    }
    c
}

/// `acc -= x` where the result is known to be nonnegative.
fn sub_into(acc: &mut [u64], x: &[u64]) {
    let mut bw = 0u64;
    for (i, a) in acc.iter_mut().enumerate() {
        let y = x.get(i).copied().unwrap_or(0);
        if i >= x.len() && bw == 0 {
            break;
        }
        let (d1, o1) = a.overflowing_sub(y);
        let (d2, o2) = d1.overflowing_sub(bw);
        *a = d2;
        bw = (o1 | o2) as u64;
    }
    debug_assert_eq!(bw, 0, "sub_into underflow");
}

/// Balanced Karatsuba, `r = a*b`, `a.len() == b.len()`, `r` zeroed.
fn karatsuba_into(r: &mut [u64], a: &[u64], b: &[u64]) {
    let n = a.len();
    debug_assert_eq!(n, b.len());
    if n < KARATSUBA_LIMBS {
        schoolbook_into(r, a, b);
        return;
    }
    let h = n / 2;
    let (a0, a1) = a.split_at(h);
    let (b0, b1) = b.split_at(h);
    let (low, high) = r.split_at_mut(2 * h);
    karatsuba_into(low, a0, b0);
    karatsuba_into(high, a1, b1);
    // Sums have at most n-h+1 limbs; their product at most 2(n-h+1).
    let mut sa = a1.to_vec();
    let ca = add_into(&mut sa, a0);
    sa.push(ca);
    let mut sb = b1.to_vec();
    let cb = add_into(&mut sb, b0);
    sb.push(cb);
    let mut mid = vec![0u64; sa.len() + sb.len()];
    let (sa, sb) = (trim(&sa), trim(&sb));
    if sa.len() == sb.len() {
        karatsuba_into(&mut mid[..sa.len() + sb.len()], sa, sb);
    } else if !sa.is_empty() && !sb.is_empty() {
        mul_into(&mut mid[..sa.len() + sb.len()], sa, sb);
    }
    sub_into(&mut mid, &r[..2 * h]);
    sub_into(&mut mid, &r[2 * h..]);
    let carry = add_into(&mut r[h..], trim(&mid));
    debug_assert_eq!(carry, 0);
}

fn trim(x: &[u64]) -> &[u64] {
    let mut n = x.len();
    while n > 0 && x[n - 1] == 0 {
        n -= 1;
    }
    &x[..n]
}

/// `r = a*b` for zeroed `r` of length `a.len()+b.len()`, any shapes.
fn mul_into(r: &mut [u64], a: &[u64], b: &[u64]) {
    let (a, b) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    if b.len() < KARATSUBA_LIMBS {
        schoolbook_into(r, a, b);
        return;
    }
    if a.len() == b.len() {
        karatsuba_into(r, a, b);
        return;
    }
    // Unbalanced: balanced blocks of the shorter length, accumulated.
    let k = b.len();
    let mut block = vec![0u64; 2 * k];
    let mut start = 0;
    while start < a.len() {
        let piece = &a[start..(start + k).min(a.len())];
        let width = piece.len() + k;
        block[..width].fill(0);
        mul_into(&mut block[..width], piece, b);
        let carry = add_into(&mut r[start..], &block[..width]);
        debug_assert_eq!(carry, 0);
        start += k;
    }
}

/// Operand length (in limbs) below which `sqr_limbs` uses the quadratic loop.
pub const SQR_KARATSUBA_LIMBS: usize = 48;

/// `r = a^2` for zeroed `r` of length `2*a.len()`: off-diagonal products
/// once, doubled, plus the diagonal squares.
fn sqr_schoolbook_into(r: &mut [u64], a: &[u64]) {
    let n = a.len();
    debug_assert_eq!(r.len(), 2 * n);
    for i in 0..n.saturating_sub(1) {
        let x = a[i];
        if x == 0 {
            continue;
        }
        let row = &mut r[2 * i + 1..i + n + 1];
        let mut c = 0u64;
        for (rj, &y) in row.iter_mut().zip(&a[i + 1..]) {
            let s = *rj as u128 + x as u128 * y as u128 + c as u128;
            *rj = s as u64;
            c = (s >> 64) as u64;
        }
        r[i + n] = c;
    }
    let mut top = 0u64;
    for v in r.iter_mut() {
        let next = *v >> 63;
        *v = (*v << 1) | top;
        top = next;
    }
    debug_assert_eq!(top, 0);
    let mut c = 0u64;
    for (i, &x) in a.iter().enumerate() {
        let sq = x as u128 * x as u128;
        let lo = r[2 * i] as u128 + (sq as u64) as u128 + c as u128;
        r[2 * i] = lo as u64;
        let hi = r[2 * i + 1] as u128 + (sq >> 64) + (lo >> 64);
        r[2 * i + 1] = hi as u64;
        c = (hi >> 64) as u64;
    }
    debug_assert_eq!(c, 0);
}

/// Karatsuba squaring, `r = a^2`, `r` zeroed of length `2*a.len()`.
fn sqr_karatsuba_into(r: &mut [u64], a: &[u64]) {
    let n = a.len();
    if n < SQR_KARATSUBA_LIMBS {
        sqr_schoolbook_into(r, a);
        return;
    }
    let h = n / 2;
    let (a0, a1) = a.split_at(h);
    let (low, high) = r.split_at_mut(2 * h);
    sqr_karatsuba_into(low, a0);
    sqr_karatsuba_into(high, a1);
    let mut sum = a1.to_vec();
    let carry = add_into(&mut sum, a0);
    sum.push(carry);
    let sum = trim(&sum);
    let mut mid = vec![0u64; 2 * sum.len()];
    sqr_karatsuba_into(&mut mid, sum);
    sub_into(&mut mid, &r[..2 * h]);
    sub_into(&mut mid, &r[2 * h..]);
    let carry = add_into(&mut r[h..], trim(&mid));
    debug_assert_eq!(carry, 0);
}

/// Exact square of a little-endian limb slice (length `2*a.len()`, not
/// normalized).
pub fn sqr_limbs(a: &[u64]) -> Vec<u64> {
    let mut r = vec![0u64; 2 * a.len()];
    if !a.is_empty() {
        sqr_karatsuba_into(&mut r, a);
    }
    r
}

/// Exact product of little-endian limb slices (length `a.len()+b.len()`,
/// not normalized). Karatsuba above `KARATSUBA_LIMBS`; equal operands use
/// `sqr_limbs`.
pub fn mul_limbs(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a == b {
        return sqr_limbs(a);
    }
    let mut r = vec![0u64; a.len() + b.len()];
    if !a.is_empty() && !b.is_empty() {
        mul_into(&mut r, a, b);
    }
    r
}

impl PartialOrd for Big {
    fn partial_cmp(&self, o: &Big) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Big {
    fn cmp(&self, o: &Big) -> Ordering {
        if self.0.len() != o.0.len() {
            return self.0.len().cmp(&o.0.len());
        }
        for i in (0..self.0.len()).rev() {
            if self.0[i] != o.0[i] {
                return self.0[i].cmp(&o.0[i]);
            }
        }
        Ordering::Equal
    }
}

/// Montgomery arithmetic with a runtime limb count (for primality tests on candidates of any size).
struct MontRt {
    n: Vec<u64>,
    ninv: u64,
}
impl MontRt {
    fn new(n: &Big) -> Self {
        let mut inv = 1u64;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(n.0[0].wrapping_mul(inv)));
        }
        MontRt {
            n: n.0.clone(),
            ninv: inv.wrapping_neg(),
        }
    }
    fn geq(a: &[u64], b: &[u64]) -> bool {
        for i in (0..b.len()).rev() {
            if a[i] != b[i] {
                return a[i] > b[i];
            }
        }
        true
    }
    fn sub_in(a: &mut [u64], b: &[u64]) {
        let mut br = false;
        for i in 0..b.len() {
            let (d1, o1) = a[i].overflowing_sub(b[i]);
            let (d2, o2) = d1.overflowing_sub(br as u64);
            a[i] = d2;
            br = o1 || o2;
        }
    }
    /// 2a mod n for a < n.
    fn dbl(&self, a: &mut [u64]) {
        let mut c = 0u64;
        for x in a.iter_mut() {
            let nc = *x >> 63;
            *x = (*x << 1) | c;
            c = nc;
        }
        if c == 1 || Self::geq(a, &self.n) {
            Self::sub_in(a, &self.n);
        }
    }
    fn mul(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        let k = self.n.len();
        let mut t = vec![0u64; k + 2];
        for i in 0..k {
            let mut c: u128 = 0;
            for j in 0..k {
                let s = t[j] as u128 + a[j] as u128 * b[i] as u128 + c;
                t[j] = s as u64;
                c = s >> 64;
            }
            let s = t[k] as u128 + c;
            t[k] = s as u64;
            t[k + 1] = (s >> 64) as u64;
            let m = t[0].wrapping_mul(self.ninv);
            let mut c = (t[0] as u128 + m as u128 * self.n[0] as u128) >> 64;
            for j in 1..k {
                let s = t[j] as u128 + m as u128 * self.n[j] as u128 + c;
                t[j - 1] = s as u64;
                c = s >> 64;
            }
            let s = t[k] as u128 + c;
            t[k - 1] = s as u64;
            t[k] = t[k + 1] + (s >> 64) as u64;
        }
        let top = t[k];
        t.truncate(k);
        if top != 0 || Self::geq(&t, &self.n) {
            Self::sub_in(&mut t, &self.n);
        }
        t
    }
}

/// Small odd primes for trial division before Miller-Rabin.
fn small_odd_primes() -> &'static [u64] {
    use std::sync::OnceLock;
    static P: OnceLock<Vec<u64>> = OnceLock::new();
    P.get_or_init(|| {
        (3u64..2000)
            .step_by(2)
            .filter(|&q| crate::field::is_prime(q))
            .collect()
    })
}

/// Miller-Rabin for big integers (bases 2..=37) after trial division by the primes below 2000;
/// Montgomery arithmetic with the candidate's own limb count. `n` odd > 3.
pub fn is_probable_prime(n: &Big) -> bool {
    if let Some(v) = n.to_u128() {
        if v < (1u128 << 63) {
            return crate::field::is_prime(v as u64);
        }
    }
    if !n.bit(0) {
        return false;
    }
    for &q in small_odd_primes() {
        if n.rem_small(q) == 0 {
            return false;
        }
    }
    let mt = MontRt::new(n);
    let k = n.0.len();
    // R mod n (Montgomery one) by doubling 1 64k times
    let mut one = vec![0u64; k];
    one[0] = 1;
    for _ in 0..64 * k {
        mt.dbl(&mut one);
    }
    // n - 1 = d 2^s; in Montgomery form, -1 is n - R
    let nm1 = n.sub_small(1);
    let mut s = 0;
    while !nm1.bit(s) {
        s += 1;
    }
    let d = nm1.shr(s);
    let mut minus_one = n.0.clone();
    MontRt::sub_in(&mut minus_one, &one);
    'outer: for &base in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        // base R mod n = base * (R mod n) by additions
        let mut b = vec![0u64; k];
        for _ in 0..base {
            let mut c = false;
            for i in 0..k {
                let (s1, o1) = b[i].overflowing_add(one[i]);
                let (s2, o2) = s1.overflowing_add(c as u64);
                b[i] = s2;
                c = o1 || o2;
            }
            if c || MontRt::geq(&b, &mt.n) {
                MontRt::sub_in(&mut b, &mt.n);
            }
        }
        let mut x = one.clone();
        for i in (0..d.bits()).rev() {
            x = mt.mul(&x, &x);
            if d.bit(i) {
                x = mt.mul(&x, &b);
            }
        }
        if x == one || x == minus_one {
            continue;
        }
        for _ in 0..s - 1 {
            x = mt.mul(&x, &x);
            if x == minus_one {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

/// a mod n by shift-and-subtract on preallocated limbs (setup only).
pub fn big_mod(a: &Big, n: &Big) -> Big {
    if a < n {
        return a.clone();
    }
    let k = n.0.len();
    let mut r = vec![0u64; k + 1];
    let mut nn = n.0.clone();
    nn.push(0);
    for i in (0..a.bits()).rev() {
        let mut c = a.bit(i) as u64;
        for x in r.iter_mut() {
            let nc = *x >> 63;
            *x = (*x << 1) | c;
            c = nc;
        }
        if MontRt::geq(&r, &nn) {
            MontRt::sub_in(&mut r, &nn);
        }
    }
    Big::from_limbs(&r)
}

#[cfg(test)]
mod mul_tests {
    use super::*;

    fn fixture(limbs: usize, mut s: u64, dense: bool) -> Big {
        let v: Vec<u64> = (0..limbs)
            .map(|_| {
                s ^= s << 13;
                s ^= s >> 7;
                s ^= s << 17;
                if dense {
                    u64::MAX - (s & 3)
                } else {
                    s
                }
            })
            .collect();
        Big::from_limbs(&v)
    }

    #[test]
    fn karatsuba_matches_schoolbook_on_all_shapes() {
        let sizes = [0, 1, 2, 31, 32, 33, 63, 64, 65, 97, 128, 200, 257, 513];
        for &x in &sizes {
            for &y in &sizes {
                for dense in [false, true] {
                    let a = fixture(x, 0x9e37_79b9_7f4a_7c15 ^ x as u64, dense);
                    let b = fixture(y, 0x2545_f491_4f6c_dd1d ^ y as u64, dense);
                    assert_eq!(a.mul(&b), a.mul_schoolbook(&b), "{x}x{y} dense={dense}");
                }
            }
        }
    }

    #[test]
    fn squaring_matches_schoolbook() {
        for n in [0, 1, 2, 3, 47, 48, 49, 95, 96, 97, 130, 255, 256, 257, 700] {
            for dense in [false, true] {
                let a = fixture(n, 0x1234_5678 ^ n as u64, dense);
                let mut r = vec![0u64; 2 * a.0.len()];
                sqr_schoolbook_into(&mut r, &a.0);
                assert_eq!(Big::norm(r), a.mul_schoolbook(&a), "schoolbook {n} {dense}");
                assert_eq!(
                    Big::norm(sqr_limbs(&a.0)),
                    a.mul_schoolbook(&a),
                    "{n} {dense}"
                );
                assert_eq!(a.mul(&a.clone()), a.mul_schoolbook(&a), "mul {n} {dense}");
            }
        }
    }

    #[test]
    fn karatsuba_all_ones_square() {
        for n in [32, 33, 64, 100, 1000] {
            let a = Big::from_limbs(&vec![u64::MAX; n]);
            assert_eq!(a.mul(&a), a.mul_schoolbook(&a), "{n}");
        }
    }
}
