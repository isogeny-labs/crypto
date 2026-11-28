// Verbatim extract of ec_index_calculus.rs sqrt_mod_p and sqrt_mod_p_u64 (jacobi_u64 from fp_backend.rs).
use num_bigint::BigUint;
use num_traits::{One, Zero};
use crate::jacobi_u64;
pub fn sqrt_mod_p(n: &BigUint, p: &BigUint) -> Option<BigUint> {
    let n_mod = n % p;
    if n_mod.is_zero() {
        return Some(BigUint::zero());
    }
    // Word-sized `p`: the same algorithm on `u64`s, with the residue
    // tests as Jacobi symbols.  Same root: the shortcut and the
    // Tonelli–Shanks loop below are deterministic in `n`, `p` and the
    // first non-residue, and the word path finds the same `z`.
    if let ([n], [m]) = (
        n_mod.to_u64_digits().as_slice(),
        p.to_u64_digits().as_slice(),
    ) {
        if *m > 2 && *m % 2 == 1 && *m < 1 << 63 {
            return sqrt_mod_p_u64(*n, *m).map(BigUint::from);
        }
    }
    // Euler's criterion: n^((p-1)/2) must be 1 (QR) or p-1 (NQR).
    let p_minus_one = p - BigUint::one();
    let euler = n_mod.modpow(&(&p_minus_one >> 1), p);
    if euler != BigUint::one() {
        return None;
    }
    // Shortcut for p ≡ 3 (mod 4):  x = n^((p+1)/4) mod p.
    let four = BigUint::from(4u32);
    if (p % &four) == BigUint::from(3u32) {
        let exp = (p + BigUint::one()) >> 2;
        return Some(n_mod.modpow(&exp, p));
    }
    // General Tonelli–Shanks.  Write p − 1 = Q · 2^S with Q odd.
    let mut q = p_minus_one.clone();
    let mut s = 0u32;
    while (&q & BigUint::one()).is_zero() {
        q >>= 1;
        s += 1;
    }
    // Find a quadratic non-residue z.
    let mut z = BigUint::from(2u32);
    while z.modpow(&(&p_minus_one >> 1), p) != p_minus_one {
        z += BigUint::one();
    }
    let mut m = s;
    let mut c = z.modpow(&q, p);
    let mut t = n_mod.modpow(&q, p);
    let mut r = n_mod.modpow(&((&q + BigUint::one()) >> 1), p);
    loop {
        if t == BigUint::one() {
            return Some(r);
        }
        // Find least i, 0 < i < m, with t^(2^i) = 1.
        let mut i = 0u32;
        let mut tmp = t.clone();
        while tmp != BigUint::one() {
            tmp = (&tmp * &tmp) % p;
            i += 1;
            if i >= m {
                // n should be a QR by the Euler check; reach here only
                // if p is composite or our randomness is unlucky.  Bail.
                return None;
            }
        }
        let exp = BigUint::one() << (m - i - 1);
        let b = c.modpow(&exp, p);
        m = i;
        c = (&b * &b) % p;
        t = (t * &c) % p;
        r = (r * b) % p;
    }
}

/// [`sqrt_mod_p`] on words: `0 < n < p`, `p` an odd prime below `2^63`.
///
/// The residue tests are Jacobi symbols, a gcd-style loop, and the
/// exponentiations plain square-and-multiply: what `bitop/v1` prices a
/// `legendre` and a `field_sqrt` as.  The big-integer path's Euler
/// criterion goes through `num-bigint`'s windowed `modpow`, whose cost
/// is a fixed ~90 products whatever the size of `p`.
fn sqrt_mod_p_u64(n: u64, p: u64) -> Option<u64> {
    let mul = |a: u64, b: u64| ((a as u128 * b as u128) % p as u128) as u64;
    let pow = |mut b: u64, mut e: u64| {
        let mut acc = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                acc = mul(acc, b);
            }
            b = mul(b, b);
            e >>= 1;
        }
        acc
    };
    if jacobi_u64(n, p) != 1 {
        return None;
    }
    // Shortcut for p ≡ 3 (mod 4):  x = n^((p+1)/4) mod p.
    if p % 4 == 3 {
        return Some(pow(n, p / 4 + 1));
    }
    // General Tonelli–Shanks.  Write p − 1 = Q · 2^S with Q odd.
    let mut q = p - 1;
    let mut s = 0u32;
    while q.is_multiple_of(2) {
        q /= 2;
        s += 1;
    }
    // Find a quadratic non-residue z.
    let mut z = 2u64;
    while jacobi_u64(z, p) != -1 {
        z += 1;
    }
    let mut m = s;
    let mut c = pow(z, q);
    let mut t = pow(n, q);
    let mut r = pow(n, q.div_ceil(2));
    loop {
        if t == 1 {
            return Some(r);
        }
        // Find least i, 0 < i < m, with t^(2^i) = 1.
        let mut i = 0u32;
        let mut tmp = t;
        while tmp != 1 {
            tmp = mul(tmp, tmp);
            i += 1;
            if i >= m {
                return None;
            }
        }
        let b = pow(c, 1u64 << (m - i - 1));
        m = i;
        c = mul(b, b);
        t = mul(t, c);
        r = mul(r, b);
    }
}

