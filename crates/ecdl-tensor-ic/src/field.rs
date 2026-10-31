//! Word-sized modular arithmetic (Go: field.go).

use rand::rngs::StdRng;
use rand::Rng;

/// Returns a*b mod m for a, b < m.
pub fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

pub fn add_mod(a: u64, b: u64, m: u64) -> u64 {
    let (s, carry) = a.overflowing_add(b);
    if s >= m || carry {
        s.wrapping_sub(m)
    } else {
        s
    }
}

pub fn sub_mod(a: u64, b: u64, m: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + (m - b)
    }
}

pub fn pow_mod(a: u64, e: u64, m: u64) -> u64 {
    let mut r = 1u64 % m;
    let mut a = a % m;
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            r = mul_mod(r, a, m);
        }
        a = mul_mod(a, a, m);
        e >>= 1;
    }
    r
}

/// Returns a^-1 mod a prime m. a must be nonzero mod m.
pub fn inv_mod(a: u64, m: u64) -> u64 {
    pow_mod(a, m - 2, m)
}

/// Returns y with y^2 = a mod p, where p is an odd prime, via Tonelli-Shanks.
/// `None` is Go's `ok == false`: a is a non-residue.
pub fn sqrt_mod(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if pow_mod(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(pow_mod(a, (p + 1) / 4, p));
    }
    let (mut q, mut s) = (p - 1, 0i32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2u64;
    while pow_mod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = pow_mod(z, q, p);
    let mut t = pow_mod(a, q, p);
    let mut r = pow_mod(a, q.div_ceil(2), p);
    while t != 1 {
        let mut i = 0i32;
        let mut t2 = t;
        while t2 != 1 {
            t2 = mul_mod(t2, t2, p);
            i += 1;
        }
        let mut b = c;
        let mut j = 0i32;
        while j + 1 < m - i {
            b = mul_mod(b, b, p);
            j += 1;
        }
        m = i;
        c = mul_mod(b, b, p);
        t = mul_mod(t, c, p);
        r = mul_mod(r, b, p);
    }
    Some(r)
}

/// Deterministic Miller-Rabin test for 64-bit integers.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    const BASES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &q in BASES.iter() {
        if n.is_multiple_of(q) {
            return n == q;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    for &a in BASES.iter() {
        let mut x = pow_mod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut composite = true;
        for _ in 1..s {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

/// Replacement for Go's `uint64(rng.Int63n(int64(n)))`: uniform in [0, n),
/// n >= 1 (panics on 0 like Go's Int63n). Shared helper (not in Go).
pub fn rand_below(rng: &mut StdRng, n: u64) -> u64 {
    assert!(n >= 1, "invalid argument to rand_below");
    rng.gen_range(0..n)
}
