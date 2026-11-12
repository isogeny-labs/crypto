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
///
/// The value is Fermat's `a^(m-2) mod m` (the Go prototype's formula) for
/// every input. When `m` is an odd prime and `a` is not a multiple of it,
/// that power is the unique inverse, so the extended Euclidean algorithm
/// returns the same value in about `log m` word divisions instead of
/// `2 log m` 128-bit remainders (assessment 2026-11-10, R11). Primality is
/// tested once per modulus and cached per thread; any other input (even or
/// composite `m`, `a ≡ 0`) takes the Fermat power as before. `opcount`
/// prices no step of an inversion, so no tally changes.
pub fn inv_mod(a: u64, m: u64) -> u64 {
    if m > 2 && m & 1 == 1 && prime_cached(m) {
        let r = a % m;
        if r != 0 {
            return inv_euclid(r, m);
        }
    }
    pow_mod(a, m - 2, m)
}

/// `is_prime(m)`, remembered for the last two moduli seen on this thread
/// (a solve alternates between the field prime and the group order).
fn prime_cached(m: u64) -> bool {
    use std::cell::Cell;
    thread_local! {
        static SEEN: Cell<[(u64, bool); 2]> = const { Cell::new([(0, false); 2]) };
    }
    SEEN.with(|seen| {
        let s = seen.get();
        if s[0].0 == m {
            return s[0].1;
        }
        if s[1].0 == m {
            seen.set([s[1], s[0]]);
            return s[1].1;
        }
        let p = is_prime(m);
        seen.set([(m, p), s[0]]);
        p
    })
}

/// The inverse of `a` modulo `m` for `0 < a < m` with `gcd(a, m) = 1`, by
/// the extended Euclidean algorithm on words. Successive Bézout
/// coefficients alternate in sign, so magnitudes are kept with the sign of
/// the current one tracked separately; every magnitude stays at most `m`.
fn inv_euclid(a: u64, m: u64) -> u64 {
    let (mut r0, mut r1) = (m, a);
    let (mut t0, mut t1) = (0u64, 1u64);
    let mut neg0 = true;
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 + q * t1);
        neg0 = !neg0;
    }
    debug_assert_eq!(r0, 1, "a prime modulus inverts every non-zero residue");
    if neg0 {
        (m - t0) % m
    } else {
        t0 % m
    }
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

#[cfg(test)]
mod tests {
    use super::{inv_mod, pow_mod};

    /// `inv_mod` returns Fermat's power for every input: primes of several
    /// sizes (Euclid path), composites, 2, and multiples of the modulus.
    #[test]
    fn inv_mod_equals_the_fermat_power() {
        let mut s = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s
        };
        for &m in &[
            2u64,
            3,
            5,
            7,
            9,
            15,
            101,
            65_521,
            1_000_003,
            67_108_859,
            4_294_967_291,
            4_294_967_297,
            (1 << 61) - 1,
        ] {
            let mut cases = vec![0, 1, m - 1, m, m + 1, 2 * m];
            for _ in 0..500 {
                cases.push(next() % m);
                cases.push(next());
            }
            for a in cases {
                assert_eq!(inv_mod(a, m), pow_mod(a, m - 2, m), "a = {a}, m = {m}");
            }
        }
    }
}
