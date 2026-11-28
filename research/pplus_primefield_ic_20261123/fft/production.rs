#![allow(dead_code)]
use std::collections::HashSet;
#[inline]
pub(crate) fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    if m <= u64::from(u32::MAX) && a < m && b < m {
        mulmod_u32(a, b, m)
    } else {
        ((a as u128 * b as u128) % m as u128) as u64
    }
}

/// `a·b mod m` for `a, b < m < 2³²`.  The product fits 64 bits and its
/// quotient by `m` is below `m`, so on x86-64 one 64-by-32-bit `div`
/// gives the remainder.  The compiler's `(a * b) % m` divides by a 64-bit
/// register instead, after testing at run time whether both operands fit
/// 32 bits (they rarely do).  The same remainder either way.
#[inline]
fn mulmod_u32(a: u64, b: u64, m: u64) -> u64 {
    let prod = a * b;
    #[cfg(target_arch = "x86_64")]
    {
        let rem: u32;
        // SAFETY: `div` faults only on a zero divisor or a quotient that
        // overflows 32 bits; `m > a ≥ 0` is nonzero and the quotient
        // `a·b / m < b < m < 2³²`.
        unsafe {
            std::arch::asm!(
                "div {m:e}",
                m = in(reg) m as u32,
                inout("eax") prod as u32 => _,
                inout("edx") (prod >> 32) as u32 => rem,
                options(pure, nomem, nostack),
            );
        }
        u64::from(rem)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        prod % m
    }
}

/// `a + b mod m`, the value of `(a + b) % m` on 128-bit integers exactly.
/// Reduced operands need one compare-and-subtract instead of the
/// 128-bit remainder's library call; the carry handles a modulus above
/// `2^63`.
#[inline]
pub(crate) fn addmod(a: u64, b: u64, m: u64) -> u64 {
    if a < m && b < m {
        let (s, carry) = a.overflowing_add(b);
        if carry || s >= m {
            s.wrapping_sub(m)
        } else {
            s
        }
    } else {
        ((a as u128 + b as u128) % m as u128) as u64
    }
}

#[inline]
fn submod(a: u64, b: u64, m: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        m - (b - a)
    }
}

fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64 % m;
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

/// [`powmod`] adding its multiplications to `muls`.
fn powmod_counted(mut a: u64, mut e: u64, m: u64, muls: &mut u64) -> u64 {
    let mut r = 1u64 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            *muls += 1;
            r = mulmod(r, a, m);
        }
        *muls += 1;
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}

pub struct PrimeCurve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}

impl PrimeCurve {
    /// `x³ + ax + b`.
    pub fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        addmod(
            addmod(mulmod(mulmod(x, x, p), x, p), mulmod(self.a, x, p), p),
            self.b,
            p,
        )
    }

    /// A square root modulo `p` (Tonelli–Shanks), or `None` for a
    /// non-residue.
    pub fn sqrt(&self, n: u64) -> Option<u64> {
        self.sqrt_counted(n, &mut 0)
    }

    /// [`PrimeCurve::sqrt`], adding to `muls` the modular multiplications it
    /// performs (squarings included: every `mulmod` call, the ones inside
    /// its exponentiations too). The result is `sqrt`'s, step for step.
    pub fn sqrt_counted(&self, n: u64, muls: &mut u64) -> Option<u64> {
        let p = self.p;
        let n = n % p;
        if n == 0 {
            return Some(0);
        }
        if p % 4 == 3 {
            let r = powmod_counted(n, (p + 1) / 4, p, muls);
            *muls += 1;
            return (mulmod(r, r, p) == n).then_some(r);
        }
        if powmod_counted(n, (p - 1) / 2, p, muls) != 1 {
            return None;
        }
        let mut q = p - 1;
        let mut s = 0u32;
        while q.is_multiple_of(2) {
            q /= 2;
            s += 1;
        }
        let mut z = 2u64;
        while powmod_counted(z, (p - 1) / 2, p, muls) != p - 1 {
            z += 1;
        }
        let mut m = s;
        let mut c = powmod_counted(z, q, p, muls);
        let mut t = powmod_counted(n, q, p, muls);
        let mut r = powmod_counted(n, q.div_ceil(2), p, muls);
        loop {
            if t == 1 {
                return Some(r);
            }
            let mut i = 0u32;
            let mut tt = t;
            while tt != 1 {
                *muls += 1;
                tt = mulmod(tt, tt, p);
                i += 1;
                if i == m {
                    return None;
                }
            }
            let b = powmod_counted(c, 1u64 << (m - i - 1), p, muls);
            m = i;
            *muls += 3;
            c = mulmod(b, b, p);
            t = mulmod(t, c, p);
            r = mulmod(r, b, p);
        }
    }

}
fn addm(a: u64, b: u64, p: u64) -> u64 {
    let s = a + b;
    if s >= p {
        s - p
    } else {
        s
    }
}
/// Abscissae of a Dickson / `T_2`-torus 2-power tower over `F_p`:
/// `y_{t-1}² = c_top + 2`, `y_j² = y_{j+1} + 2`, `x = (y_0 + lo) mod p`.
/// Requires `2^t` distinct `F_p` roots (e.g. `p ≡ 3 (mod 4)` with `2^{t+2} | p + 1`
/// and `c_top = 0`, such as `M31 = 2^31 − 1` for `1 ≤ t ≤ 29`).
pub fn dickson_abscissae(p: u64, t: u32, lo: u64, c_top: u64) -> Result<Vec<u64>, String> {
    if t == 0 || t >= 30 {
        return Err(format!(
            "pkm-semaev: dickson tower height t = {t} must be in 1..30"
        ));
    }
    let dummy_curve = PrimeCurve { p, a: 0, b: 1 };
    let mut layer = vec![c_top % p];
    for step in (0..t).rev() {
        let mut next = Vec::with_capacity(layer.len() * 2);
        for &u in &layer {
            let rhs = addm(u, 2 % p, p);
            if rhs == 0 {
                return Err(format!(
                    "pkm-semaev: dickson tower ramifies (rhs = 0) at level {step} mod {p}"
                ));
            }
            let Some(r) = dummy_curve.sqrt(rhs) else {
                return Err(format!(
                    "pkm-semaev: dickson tower fails to split in F_{p} at level {step}"
                ));
            };
            let r0 = r.min(p - r);
            let r1 = p - r0;
            next.push(r0);
            if r1 != r0 {
                next.push(r1);
            }
        }
        layer = next;
    }
    let xs: Vec<u64> = layer.into_iter().map(|y0| addm(y0, lo % p, p)).collect();
    let uniq: HashSet<u64> = xs.iter().copied().collect();
    if uniq.len() != (1usize << t) {
        return Err(format!(
            "pkm-semaev: dickson tower produced {} distinct roots instead of 2^{t}",
            uniq.len()
        ));
    }
    Ok(xs)
}

