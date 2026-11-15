//! Number theory for curve construction: Miller-Rabin, BSGS point counting,
//! discriminant/conductor check, analytic class-number estimate.
use crate::{Curve, Fp, Rng};
use std::collections::HashMap;

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
/// deterministic Miller-Rabin for u64
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 {
            return n == q;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
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
pub fn isqrt(n: u128) -> u128 {
    if n == 0 {
        return 0;
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

/// affine point helpers (None = infinity), Montgomery-form coordinates
fn aff_add(c: &Curve, p: Option<(u64, u64)>, q: Option<(u64, u64)>) -> Option<(u64, u64)> {
    let f = &c.f;
    match (p, q) {
        (None, x) | (x, None) => x,
        (Some(p), Some(q)) => {
            let lam = if p.0 == q.0 {
                if f.add(p.1, q.1) == 0 {
                    return None;
                }
                // doubling
                let num = f.add(f.mul(f.small(3), f.mul(p.0, p.0)), c.a);
                f.mul(num, f.inv(f.add(p.1, p.1)))
            } else {
                f.mul(f.sub(q.1, p.1), f.inv(f.sub(q.0, p.0)))
            };
            let x3 = f.sub(f.sub(f.mul(lam, lam), p.0), q.0);
            let y3 = f.sub(f.mul(lam, f.sub(p.0, x3)), p.1);
            Some((x3, y3))
        }
    }
}
fn aff_mul(c: &Curve, k: u64, p: (u64, u64)) -> Option<(u64, u64)> {
    let j = {
        let mut acc = (c.f.one, c.f.one, 0u64);
        for i in (0..64).rev() {
            acc = c.jdbl(acc);
            if (k >> i) & 1 == 1 {
                acc = c.jmadd(acc, p);
            }
        }
        acc
    };
    c.to_affine(j)
}

/// #E(F_p) by BSGS on random points: the unique N in the Hasse interval with
/// N*P = O for points P whose order exceeds the interval width. Returns None on failure.
pub fn card(p: u64, a: u64, b: u64, rng: &mut Rng) -> Option<u64> {
    let f = Fp::new(p);
    let c = Curve { f, a: f.to_m(a), b: f.to_m(b) };
    let s = isqrt(p as u128) as u64;
    let lo = p + 1 - 2 * s - 2;
    let width = 4 * s + 4;
    let m = isqrt(width as u128) as u64 + 1;
    for _attempt in 0..8 {
        let pt = c.random_point(rng);
        // baby steps j*P, j = 0..m
        let mut baby: HashMap<u64, Vec<u64>> = HashMap::new();
        let mut cur: Option<(u64, u64)> = None;
        for j in 0..=m {
            let key = cur.map(|q| q.0).unwrap_or(u64::MAX);
            baby.entry(key).or_default().push(j);
            cur = aff_add(&c, cur, Some(pt));
        }
        let mp = aff_mul(&c, m, pt);
        let neg_mp = mp.map(|q| (q.0, f.neg(q.1)));
        // giant: -(lo*P) - i*m*P ; solve k = i*m + j with (lo + k) P = O
        let lp = aff_mul(&c, lo, pt);
        let mut g = lp.map(|q| (q.0, f.neg(q.1)));
        let mut sols: Vec<u64> = vec![];
        for i in 0..=m {
            let key = g.map(|q| q.0).unwrap_or(u64::MAX);
            if let Some(js) = baby.get(&key) {
                for &j in js {
                    let k = i * m + j;
                    if k <= width && aff_mul(&c, lo + k, pt).is_none() {
                        sols.push(lo + k);
                    }
                }
            }
            g = aff_add(&c, g, neg_mp);
        }
        sols.sort();
        sols.dedup();
        if sols.len() == 1 {
            let n = sols[0];
            // confirm with fresh points
            let mut ok = true;
            for _ in 0..4 {
                let q = c.random_point(rng);
                if aff_mul(&c, n, q).is_some() {
                    ok = false;
                }
            }
            if ok {
                return Some(n);
            }
        }
    }
    None
}

/// For D = t^2 - 4p < 0, return (fundamental discriminant d, conductor f) if f <= 2, else None.
pub fn disc_conductor_le2(d_abs: u64) -> Option<(i64, u64)> {
    // D = -d_abs.  Remove square factors of odd primes: none allowed.
    let mut n = d_abs;
    let mut twos = 0;
    while n % 2 == 0 {
        n /= 2;
        twos += 1;
    }
    // odd part must be squarefree
    let lim = (n as f64).cbrt() as u64 + 2;
    let mut m = n;
    let mut q = 3u64;
    while q <= lim && q * q <= m {
        if m % q == 0 {
            m /= q;
            if m % q == 0 {
                return None;
            }
        }
        q += 2;
    }
    if m > 1 {
        let r = isqrt(m as u128) as u64;
        if r * r == m {
            return None;
        }
    }
    // now D = -2^twos * n with n odd squarefree; -n mod 4
    let negn_mod4 = (4 - (n % 4)) % 4; // (-n) mod 4
    match twos {
        0 => {
            if negn_mod4 == 1 {
                Some((-(n as i64), 1))
            } else {
                None
            }
        }
        2 => {
            // D = 4 * (-n): fundamental if -n = 2,3 mod 4; else conductor 2 with d = -n
            if negn_mod4 == 3 {
                Some((-(4 * n as i64), 1))
            } else {
                Some((-(n as i64), 2))
            }
        }
        3 => Some((-(8 * n as i64), 1)),
        4 => {
            // D = 16 * (-n): f = 2 with d = -4n if -n = 3 mod 4 (fundamental -4n)
            if negn_mod4 == 3 {
                Some((-(4 * n as i64), 2))
            } else {
                None
            }
        }
        5 => Some((-(8 * n as i64), 2)),
        _ => None,
    }
}

/// Kronecker symbol (d / q) for odd prime q
fn kron(d: i64, q: u64) -> i32 {
    let r = ((d % q as i64) + q as i64) as u64 % q;
    crate::jacobi(r, q)
}
/// analytic estimate h = sqrt(|d|)/pi * L(1, chi_d), Euler product over primes < 2e6
pub fn class_number_estimate(d: i64) -> f64 {
    let lim = 2_000_000usize;
    let mut sieve = vec![true; lim];
    let mut l = 1.0f64;
    // q = 2: chi(2) = 0 if d even, 1 if d = 1 mod 8, -1 if d = 5 mod 8
    let dm8 = ((d % 8) + 8) % 8;
    let c2 = if d % 2 == 0 { 0.0 } else if dm8 == 1 { 1.0 } else { -1.0 };
    l *= 1.0 / (1.0 - c2 / 2.0);
    for i in 2..lim {
        if !sieve[i] {
            continue;
        }
        let mut j = i * i;
        while j < lim {
            sieve[j] = false;
            j += i;
        }
        if i == 2 {
            continue;
        }
        let ch = kron(d, i as u64) as f64;
        l *= 1.0 / (1.0 - ch / i as f64);
    }
    ((-d) as f64).sqrt() / std::f64::consts::PI * l
}
