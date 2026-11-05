//! Public-data arithmetic; no constant-time claim.
pub const MASK: u128 = (1u128 << 83) - 1;
pub const LOW: u128 = (1u128 << 45) | 7;
pub const P: u128 = 4835703278761981908090881;
pub const B: u128 = (1u128 << 82) | 0x51b;

pub fn slow_bmul(mut a: u128, mut b: u128) -> u128 {
    let mut r = 0;
    while b != 0 {
        if b & 1 != 0 {
            r ^= a;
        }
        b >>= 1;
        let carry = a >> 82;
        a = (a << 1) & MASK;
        if carry != 0 {
            a ^= LOW;
        }
    }
    r
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
unsafe fn clmul(a: u128, b: u128) -> u128 {
    use std::arch::x86_64::*;
    let aa: __m128i = std::mem::transmute(a);
    let bb: __m128i = std::mem::transmute(b);
    let ll: u128 = std::mem::transmute(_mm_clmulepi64_si128(aa, bb, 0));
    let hh: u128 = std::mem::transmute(_mm_clmulepi64_si128(aa, bb, 17));
    let lh: u128 = std::mem::transmute(_mm_xor_si128(
        _mm_clmulepi64_si128(aa, bb, 1),
        _mm_clmulepi64_si128(aa, bb, 16),
    ));
    let lo = ll ^ (lh << 64);
    let hi = hh ^ (lh >> 64);
    // Product degree <=164. Three folds give degrees <=126, <=88, <=50.
    let h = (lo >> 83) ^ (hi << 45);
    let r = (lo & MASK) ^ h ^ (h << 1) ^ (h << 2) ^ (h << 45);
    let h = r >> 83;
    let r = (r & MASK) ^ h ^ (h << 1) ^ (h << 2) ^ (h << 45);
    let h = r >> 83;
    (r & MASK) ^ h ^ (h << 1) ^ (h << 2) ^ (h << 45)
}

#[inline]
pub fn bmul(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "x86_64")]
    {
        unsafe { clmul(a, b) }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        slow_bmul(a, b)
    }
}
pub fn check_cpu() {
    #[cfg(target_arch = "x86_64")]
    assert!(
        std::is_x86_feature_detected!("pclmulqdq"),
        "PCLMULQDQ required"
    );
}
pub fn bpow(mut a: u128, mut e: u128) -> u128 {
    let mut r = 1;
    while e != 0 {
        if e & 1 != 0 {
            r = bmul(r, a);
        }
        a = bmul(a, a);
        e >>= 1;
    }
    r
}
pub fn half_trace(c: u128) -> u128 {
    let (mut z, mut t) = (c, c);
    for _ in 1..42 {
        t = bmul(t, t);
        t = bmul(t, t);
        z ^= t;
    }
    z
}
pub struct BinaryTables {
    half: [[u128; 256]; 11],
    trace_mask: u128,
}
impl BinaryTables {
    pub fn new() -> Self {
        let mut half = [[0; 256]; 11];
        let mut trace_mask = 0;
        for i in 0..83 {
            let x = 1u128 << i;
            let h = half_trace(x);
            let trace = bmul(h, h) ^ h ^ x;
            assert!(trace <= 1);
            trace_mask |= trace << i;
            for v in 0..256 {
                if v & (1 << (i % 8)) != 0 {
                    half[i / 8][v] ^= h;
                }
            }
        }
        Self { half, trace_mask }
    }
    pub fn trace(&self, a: u128) -> u32 {
        (a & self.trace_mask).count_ones() & 1
    }
    pub fn half(&self, mut a: u128) -> u128 {
        let mut h = 0;
        for t in &self.half {
            h ^= t[(a & 255) as usize];
            a >>= 8;
        }
        h
    }
}
pub fn batch_inverse(start: u64, end: u64, coordinate_bit: u8) -> Vec<u128> {
    let mut prefix = Vec::with_capacity((end - start) as usize);
    let mut acc = 1;
    for x in start..end {
        prefix.push(acc);
        if x != 0 {
            acc = bmul(acc, super::domain_x(x, coordinate_bit) as u128);
        }
    }
    let mut inv = bpow(acc, MASK - 1);
    for x in (start..end).rev() {
        let i = (x - start) as usize;
        if x == 0 {
            prefix[i] = 0;
        } else {
            prefix[i] = bmul(prefix[i], inv);
            inv = bmul(inv, super::domain_x(x, coordinate_bit) as u128);
        }
    }
    assert_eq!(inv, 1);
    prefix
}

// Two-limb Montgomery reduction. Inputs and outputs canonical modulo P.
const fn minv() -> u64 {
    let n = P as u64;
    let mut x = 1u64;
    let mut i = 0;
    while i < 6 {
        x = x.wrapping_mul(2u64.wrapping_sub(n.wrapping_mul(x)));
        i += 1;
    }
    x.wrapping_neg()
}
const INV: u64 = minv();
const fn pow2(k: usize) -> u128 {
    let mut r = 1;
    let mut i = 0;
    while i < k {
        r = (2 * r) % P;
        i += 1;
    }
    r
}
pub const ONE: u128 = pow2(128);
const R2: u128 = pow2(256);
#[inline]
pub fn pmul(a: u128, b: u128) -> u128 {
    let aa = [a as u64, (a >> 64) as u64];
    let bb = [b as u64, (b >> 64) as u64];
    let pp = [P as u64, (P >> 64) as u64];
    let mut t = [0u64; 5];
    for i in 0..2 {
        let mut carry = 0u128;
        for j in 0..2 {
            let z = aa[i] as u128 * bb[j] as u128 + t[i + j] as u128 + carry;
            t[i + j] = z as u64;
            carry = z >> 64;
        }
        t[i + 2] = carry as u64;
    }
    for i in 0..2 {
        let m = t[i].wrapping_mul(INV);
        let mut carry = 0u128;
        for j in 0..2 {
            let z = m as u128 * pp[j] as u128 + t[i + j] as u128 + carry;
            t[i + j] = z as u64;
            carry = z >> 64;
        }
        for item in t.iter_mut().skip(i + 2) {
            let z = *item as u128 + carry;
            *item = z as u64;
            carry = z >> 64;
        }
    }
    debug_assert_eq!(t[4], 0);
    let r = t[2] as u128 | ((t[3] as u128) << 64);
    if r >= P {
        r - P
    } else {
        r
    }
}
pub fn to_mont(a: u128) -> u128 {
    pmul(a, R2)
}
pub fn from_mont(a: u128) -> u128 {
    pmul(a, 1)
}
pub fn ppow(mut a: u128, mut e: u128) -> u128 {
    let mut r = ONE;
    while e != 0 {
        if e & 1 != 0 {
            r = pmul(r, a);
        }
        a = pmul(a, a);
        e >>= 1;
    }
    r
}
pub fn slow_pmul(mut a: u128, mut b: u128) -> u128 {
    let mut r = 0;
    while b != 0 {
        if b & 1 != 0 {
            r = (r + a) % P;
        }
        a = (a + a) % P;
        b >>= 1;
    }
    r
}
pub fn slow_ppow(mut a: u128, mut e: u128) -> u128 {
    let mut r = 1;
    while e != 0 {
        if e & 1 != 0 {
            r = slow_pmul(r, a);
        }
        a = slow_pmul(a, a);
        e >>= 1;
    }
    r
}
// Tonelli-Shanks expressed via six 7-bit digits in the fixed 2-Sylow group.
// P-1=q*2^42, and 3 is a certified quadratic nonresidue.
pub struct PrimeSqrt {
    log: std::collections::HashMap<u128, u64>,
    correction: [[u128; 128]; 6],
}
impl PrimeSqrt {
    pub fn new() -> Self {
        let q = (P - 1) >> 42;
        let c = ppow(to_mont(3), q);
        assert_eq!(ppow(c, 1u128 << 41), to_mont(P - 1));
        let g = ppow(c, 1u128 << 35);
        let mut log = std::collections::HashMap::new();
        let mut z = ONE;
        for d in 0..128 {
            assert!(log.insert(z, d).is_none());
            z = pmul(z, g);
        }
        assert_eq!(z, ONE);
        let mut correction = [[ONE; 128]; 6];
        let mut g = ppow(c, P - 2);
        for row in &mut correction {
            for d in 1..128 {
                row[d] = pmul(row[d - 1], g);
            }
            for _ in 0..7 {
                g = pmul(g, g);
            }
        }
        Self { log, correction }
    }
    pub fn root(&self, a: u128) -> Option<u128> {
        if a == 0 {
            return Some(0);
        }
        let q = (P - 1) >> 42;
        let a = to_mont(a);
        let w = ppow(a, (q - 1) / 2);
        let mut root = pmul(a, w);
        let mut t = pmul(root, w);
        let mut e = 0u64;
        for k in 0..6 {
            let mut z = t;
            for _ in 0..(35 - 7 * k) {
                z = pmul(z, z);
            }
            let d = *self.log.get(&z).expect("2-Sylow digit");
            if k == 0 && d & 1 != 0 {
                return None;
            }
            e |= d << (7 * k);
            t = pmul(t, self.correction[k][d as usize]);
        }
        assert_eq!(t, ONE);
        e >>= 1;
        for k in 0..6 {
            root = pmul(root, self.correction[k][((e >> (7 * k)) & 127) as usize]);
        }
        assert_eq!(pmul(root, root), a);
        Some(from_mont(root))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn next(s: &mut u128) -> u128 {
        *s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *s
    }
    #[test]
    fn arithmetic_reference_and_boundaries() {
        check_cpu();
        let mut s = 20261028u128;
        for i in 0..5000 {
            let a = if i < 83 { 1 << i } else { next(&mut s) & MASK };
            let b = next(&mut s) & MASK;
            assert_eq!(bmul(a, b), slow_bmul(a, b));
            let a = a % P;
            let b = b % P;
            assert_eq!(from_mont(to_mont(a)), a);
            assert_eq!(from_mont(pmul(to_mont(a), to_mont(b))), slow_pmul(a, b));
        }
        for a in [0, 1, P - 1, P - 2, 1 << 64] {
            for b in [0, 1, P - 1, P - 2, 1 << 64] {
                assert_eq!(from_mont(pmul(to_mont(a), to_mont(b))), slow_pmul(a, b));
            }
        }
    }
    #[test]
    fn all_sqrt_classes_and_binary_linear_map() {
        check_cpu();
        let ps = PrimeSqrt::new();
        let bs = BinaryTables::new();
        let mut s = 20261028;
        for _ in 0..512 {
            let a = next(&mut s) % P;
            let root = ps.root(a);
            let legendre = slow_ppow(a, (P - 1) / 2);
            assert_eq!(root.is_some(), legendre != P - 1);
            if let Some(r) = root {
                assert_eq!(slow_pmul(r, r), a);
            }
            let c = next(&mut s) & MASK;
            let h = bs.half(c);
            assert_eq!(h, half_trace(c));
            assert_eq!(slow_bmul(h, h) ^ h ^ c, bs.trace(c) as u128);
        }
        assert_eq!(ps.root(0), Some(0));
        for bit in 27..=40 {
            for (start, end) in [(0, 256), (12345, 12401), (134217700, 134217728)] {
                let v = batch_inverse(start, end, bit);
                for (i, x) in (start..end).enumerate() {
                    if x != 0 {
                        assert_eq!(slow_bmul(crate::domain_x(x, bit) as u128, v[i]), 1);
                    }
                }
            }
        }
    }
}
