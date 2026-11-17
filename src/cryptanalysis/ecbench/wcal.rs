//! Measure the calibrated weight `w_cal` of AGENTS.md §5a (2026-11-12): the
//! time of one native base-field mulmod divided by the time of one group
//! addition on the same curve and host.
//!
//! This helper is a **best-of-N wall-clock timing** and labels itself so
//! ([`BEST_OF_N`]).  It never reports `perf-ipc`: a perf-ipc reading needs
//! hardware counters on an isolated host (skill `perf-ipc`) and its result is
//! passed to the band as a [`WCal`] with that method.  Both operations are
//! timed in the same process, on affine short-Weierstrass points of a prime
//! curve `y² = x³ + a·x + b` over `F_p` (`p < 2^63`), the group addition
//! including its field inversion.  Taking the minimum round of each removes
//! scheduler noise, not frequency drift; the spread of the rounds is
//! recorded so a reader can judge it.

use std::hint::black_box;
use std::time::Instant;

use super::band::WCal;

/// The method label of a reading this module produced.
pub const BEST_OF_N: &str = "best-of-N timing";

/// One measurement.
#[derive(Clone, Debug, PartialEq)]
pub struct WCalMeasurement {
    pub p: u64,
    pub rounds: usize,
    pub ops_per_round: u64,
    /// Fastest round, nanoseconds per operation.
    pub mul_ns: f64,
    pub add_ns: f64,
    /// Slowest round over fastest, per operation (1 = no spread).
    pub mul_spread: f64,
    pub add_spread: f64,
    pub w_cal: f64,
}

impl WCalMeasurement {
    /// The band input, labelled as a timing taken on `host`.
    pub fn to_wcal(&self, host: &str) -> WCal {
        WCal {
            w_cal: self.w_cal,
            method: BEST_OF_N.to_string(),
            host: host.to_string(),
        }
    }
}

fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn powmod(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, b, p);
        }
        b = mulmod(b, b, p);
        e >>= 1;
    }
    r
}

fn invmod(a: u64, p: u64) -> u64 {
    let (mut t, mut nt) = (0i128, 1i128);
    let (mut r, mut nr) = (p as i128, a as i128 % p as i128);
    while nr != 0 {
        let q = r / nr;
        (t, nt) = (nt, t - q * nt);
        (r, nr) = (nr, r - q * nr);
    }
    t.rem_euclid(p as i128) as u64
}

/// Affine addition of distinct points with `x1 != x2`.
fn add(p: u64, a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
    let num = (b.1 + p - a.1) % p;
    let den = (b.0 + p - a.0) % p;
    let l = mulmod(num, invmod(den, p), p);
    let x3 = (mulmod(l, l, p) + 2 * p - a.0 - b.0) % p;
    let y3 = (mulmod(l, (a.0 + p - x3) % p, p) + p - a.1) % p;
    (x3, y3)
}

/// The first point of `y² = x³ + a x + b` at `x ≥ x0` (p an odd prime).
fn find_point(p: u64, a: u64, b: u64, x0: u64) -> Option<(u64, u64)> {
    for x in x0..p.min(x0 + 10_000) {
        let rhs = (mulmod(mulmod(x, x, p), x, p) + mulmod(a % p, x, p) + b % p) % p;
        if rhs != 0 && powmod(rhs, (p - 1) / 2, p) == 1 {
            // p ≡ 3 (mod 4): a root is rhs^((p+1)/4); otherwise search by brute force.
            let y = if p % 4 == 3 {
                powmod(rhs, p.div_ceil(4), p)
            } else {
                (1..p).find(|&y| mulmod(y, y, p) == rhs)?
            };
            return Some((x, y));
        }
    }
    None
}

/// Best-of-`rounds` time of a mulmod chain and of a group-addition chain on
/// `y² = x³ + a x + b` over `F_p`, `ops_per_round` operations a round.
/// `Err` on a bad modulus or when no curve point is found.
pub fn measure(
    p: u64,
    a: u64,
    b: u64,
    rounds: usize,
    ops_per_round: u64,
) -> Result<WCalMeasurement, String> {
    if !(5..1u64 << 63).contains(&p) || rounds == 0 || ops_per_round == 0 {
        return Err("need 5 <= p < 2^63, rounds >= 1, ops_per_round >= 1".into());
    }
    let g = find_point(p, a, b, 1).ok_or("no curve point found")?;
    let h = find_point(p, a, b, g.0 + 1).ok_or("no second curve point found")?;
    let time = |f: &mut dyn FnMut()| -> (f64, f64) {
        let (mut best, mut worst) = (f64::MAX, 0.0f64);
        for _ in 0..rounds {
            let t = Instant::now();
            f();
            let ns = t.elapsed().as_nanos() as f64 / ops_per_round as f64;
            best = best.min(ns);
            worst = worst.max(ns);
        }
        (best, worst)
    };
    let mut x = g.0 | 1;
    let (mul_ns, mul_worst) = time(&mut || {
        for _ in 0..ops_per_round {
            x = mulmod(black_box(x), black_box(h.0 | 1), p);
        }
        black_box(x);
    });
    // Walk acc += h (or += g where the x-coordinates meet) so every addition
    // is a distinct-point addition on the curve.
    let mut acc = g;
    let (add_ns, add_worst) = time(&mut || {
        for _ in 0..ops_per_round {
            let q = if acc.0 == h.0 { g } else { h };
            if acc.0 == q.0 {
                acc = black_box(find_point(p, a, b, acc.0 + 1).unwrap_or(g));
                continue;
            }
            acc = add(p, black_box(acc), q);
        }
        black_box(acc);
    });
    Ok(WCalMeasurement {
        p,
        rounds,
        ops_per_round,
        mul_ns,
        add_ns,
        mul_spread: mul_worst / mul_ns,
        add_spread: add_worst / add_ns,
        w_cal: mul_ns / add_ns,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic_matches_a_known_curve_and_the_label_is_a_timing() {
        // y² = x³ + 2x + 3 over F_97: (3, 6) + (80, 10) = (80, 87).
        assert_eq!(add(97, (3, 6), (80, 10)), (80, 87));
        let m = measure(2_147_483_647, 2, 3, 3, 2_000).unwrap();
        assert!(m.w_cal.is_finite() && m.w_cal > 0.0, "{m:?}");
        assert!(m.mul_ns > 0.0 && m.add_ns > 0.0);
        let w = m.to_wcal("test");
        assert_eq!(w.method, "best-of-N timing");
        assert!(w.validate().is_ok());
        assert!(measure(4, 2, 3, 1, 1).is_err());
    }
}
