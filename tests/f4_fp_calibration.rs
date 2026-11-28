//! Permanent calibration guard for `cryptanalysis::f4_fp::solving_degree`.
//!
//! For a homogeneous sequence of `m` quadrics in `n` variables (m > n), the
//! degree of regularity is the first degree at which the coefficient of
//! `(1 - t^2)^m / (1 - t)^n` becomes non-positive (Bardet–Faugère–Salvy).
//! `f4_fp::solving_degree` must track that number, or every downstream
//! comparison that uses it is meaningless.
//!
//! The cell table below was measured by hand on 2026-11-21 (8/8 exact, see
//! `research_notes/ic_gb_semaev_cell_20261121.md`); this file pins it as a
//! regression test so a change to the selection strategy, the stopping rule,
//! or the degree accounting cannot silently move the metric.

use crypto_lib::cryptanalysis::f4_fp::{f4, normalise, F4Options, Ordering, Poly};
use std::time::Duration;

// ── deterministic RNG (xorshift64*) ─────────────────────────────────
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

/// `C(k + n - 1, n - 1)`, the coefficient of `t^k` in `1/(1-t)^n`.
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

/// First `k >= 1` with `[t^k] (1-t^2)^m/(1-t)^n <= 0`.
fn semireg_dreg(n: usize, m: usize, prec: usize) -> Option<usize> {
    let mut num = vec![0i128; prec + 1];
    let mut binom = 1i128;
    for j in 0..=m {
        if 2 * j <= prec {
            num[2 * j] = if j % 2 == 0 { binom } else { -binom };
        }
        binom = binom * (m - j) as i128 / (j + 1) as i128;
    }
    for k in 1..=prec {
        let mut s = 0i128;
        for j in 0..=k {
            s += num[j] * mult_choose(k - j, n);
        }
        if s <= 0 {
            return Some(k);
        }
    }
    None
}

fn random_homogeneous_quadric(rng: &mut Rng, n: usize, p: u64) -> Poly {
    let mut terms: Vec<(Vec<u32>, u64)> = Vec::new();
    for i in 0..n {
        for j in i..n {
            if rng.below(4) == 0 {
                continue;
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

#[test]
fn solving_degree_matches_semiregular_prediction() {
    let p: u64 = 65521;
    let mut checked = 0usize;
    for n in 3..=6usize {
        for extra in 1..=2usize {
            let m = n + extra;
            let pred = semireg_dreg(n, m, 60).expect("prediction exists for m > n");
            let mut rng = Rng(0x9E37_79B9_7F4A_7C15 ^ ((n as u64) << 8) ^ extra as u64);
            let polys: Vec<Poly> = (0..m)
                .map(|_| random_homogeneous_quadric(&mut rng, n, p))
                .collect();
            let opts =
                F4Options::new(Ordering::Grevlex, 30).with_budget(Duration::from_secs(60));
            let rep = f4(&polys, n, p, &opts);
            assert!(
                !rep.timed_out,
                "n={n} m={m}: calibration cell timed out; the budget or the machine changed"
            );
            assert_eq!(
                rep.solving_degree as usize, pred,
                "n={n} m={m}: solving_degree {} != semi-regular prediction {} \
                 (reached {}, rows {}, cols {})",
                rep.solving_degree, pred, rep.degree_reached, rep.max_rows, rep.max_cols
            );
            checked += 1;
        }
    }
    assert_eq!(checked, 8, "expected the full 8-cell calibration table");
}
