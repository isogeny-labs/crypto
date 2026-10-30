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
