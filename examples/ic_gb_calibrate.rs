//! Calibration of `f4_fp::solving_degree` against the semi-regular prediction.
//!
//! For a homogeneous semi-regular sequence of `m` quadrics in `n` variables
//! (m > n), the degree of regularity is the first degree at which the
//! coefficient of  (1 - t^2)^m / (1 - t)^n  becomes non-positive.  This is the
//! standard Bardet–Faugère–Salvy prediction.  If the instrument is calibrated,
//! `solving_degree` should track that number across (n, m).
//!
//! Systems are homogeneous, so x = 0 is always a solution and the ideal is
//! never the unit ideal — the degenerate "no solutions over F_p" case that
//! ruins affine random systems is excluded by construction.

use crypto_lib::cryptanalysis::f4_fp::{f4, normalise, F4Options, Ordering, Poly};
use std::time::Duration;

// ── tiny deterministic RNG (xorshift64*) ────────────────────────────
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

#[allow(dead_code)]
fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

/// Semi-regular d_reg: first k >= 1 with [t^k] (1-t^2)^m/(1-t)^n <= 0.
/// Computed with exact integer arithmetic on the power series.
fn semireg_dreg(n: usize, m: usize, prec: usize) -> Option<usize> {
    // numerator coefficients of (1 - t^2)^m : sum_j (-1)^j C(m,j) t^{2j}
    let mut num = vec![0i128; prec + 1];
    let mut binom = 1i128;
    for j in 0..=m {
        if 2 * j <= prec {
            num[2 * j] = if j % 2 == 0 { binom } else { -binom };
        }
        binom = binom * (m - j) as i128 / (j + 1) as i128;
    }
    // series = num / (1-t)^n, and 1/(1-t)^n has all coefficients C(k+n-1, n-1)
    let mut coeff = vec![0i128; prec + 1];
    let mut mult = 1i128; // C(k+n-1, n-1) for k = 0
    for k in 0..=prec {
        if k > 0 {
            mult = mult * (k + n - 1) as i128 / k as i128;
        }
        let mut s = 0i128;
        for j in 0..=k {
            s += num[j] * mult_choose(k - j, n);
        }
        coeff[k] = s;
    }
    for (k, c) in coeff.iter().enumerate().skip(1) {
        if *c <= 0 {
            return Some(k);
        }
    }
    None
}

/// C(k + n - 1, n - 1), i.e. coefficient of t^k in 1/(1-t)^n.
fn mult_choose(k: usize, n: usize) -> i128 {
    if n == 0 {
        return if k == 0 { 1 } else { 0 };
    }
    let mut r = 1i128;
    for i in 1..=k {
        r = r * (n + i - 1) as i128 / i as i128;
    }
    r
}

fn random_homogeneous_quadric(rng: &mut Rng, n: usize, p: u64) -> Poly {
    let mut terms: Vec<(Vec<u32>, u64)> = Vec::new();
    for i in 0..n {
        for j in i..n {
            if rng.below(4) == 0 {
                continue; // ~75% density, like a dense random system
            }
            let c = 1 + rng.below(p - 1);
            let mut e = vec![0u32; n];
            e[i] += 1;
            e[j] += 1;
            terms.push((e, c));
        }
    }
    if terms.is_empty() {
        let mut e = vec![0u32; n];
        e[0] += 2;
        terms.push((e, 1));
    }
    normalise(&terms, p, Ordering::Grevlex)
}

fn main() {
    let p: u64 = 65521;
    println!("p={p}  target: solving_degree == semi-regular d_reg for m = n+1, n+2");
    println!(
        "{:>2} {:>2} {:>8} {:>9} {:>9} {:>8} {:>7} {:>6}",
        "n", "m", "pred", "solving", "reached", "rows", "cols", "ms"
    );
    let mut ok = 0;
    let mut total = 0;
    for n in 3..=6usize {
        for extra in 1..=2usize {
            let m = n + extra;
            let pred = semireg_dreg(n, m, 60);
            let mut rng = Rng(0x9E3779B97F4A7C15 ^ ((n as u64) << 8) ^ extra as u64);
            let polys: Vec<Poly> = (0..m)
                .map(|_| random_homogeneous_quadric(&mut rng, n, p))
                .collect();
            let opts = F4Options::new(Ordering::Grevlex, 30).with_budget(Duration::from_secs(60));
            let rep = f4(&polys, n, p, &opts);
            let pred_s = pred.map(|v| v.to_string()).unwrap_or("-".into());
            println!(
                "{:>2} {:>2} {:>8} {:>9} {:>9} {:>8} {:>7} {:>6.1}",
                n,
                m,
                pred_s,
                rep.solving_degree,
                rep.degree_reached,
                rep.max_rows,
                rep.max_cols,
                rep.ms
            );
            if let Some(pr) = pred {
                total += 1;
                if rep.solving_degree as usize == pr {
                    ok += 1;
                }
            }
        }
    }
    println!("\ncalibration: {ok}/{total} cells match the semi-regular prediction exactly");
}
