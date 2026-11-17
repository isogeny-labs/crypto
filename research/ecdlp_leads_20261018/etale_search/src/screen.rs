//! Necessary-condition screen for étale covers of genus-g0 bases at deployed sizes.
//!
//! An F_2-rational cover D -> B that is geometrically Z/n-Galois, with Frobenius acting on
//! the Galois group by lambda, exists iff lambda is an eigenvalue of Frobenius on J_B[n],
//! that is iff n | P_B(lambda) with P_B(T) = T^(2g) L_B(1/T). If the Prym contains T_n then
//! ord(lambda) <= g0 - 1. This module only counts which bases pass that condition; it does
//! not test characters.

use crate::{lpoly, Curve};
use std::collections::BTreeMap;

/// lambda in F_n with lambda^k = 1 for some k <= kmax, grouped by order.
pub fn small_order_units(n: u64, kmax: u64) -> Vec<(u64, u64)> {
    let pw = |a: u64, e: u64| (0..e).fold(1u64, |acc, _| acc * a % n);
    (1..n)
        .filter_map(|l| (1..=kmax).find(|&k| pw(l, k) == 1).map(|k| (l, k)))
        .collect()
}

/// P(lambda) mod n with P(T) = sum_i a_i T^(2g - i).
pub fn p_at(lp: &[i64], lambda: u64, n: u64) -> u64 {
    let n = n as i128;
    let d = lp.len() - 1;
    let mut acc = 0i128;
    for (i, &a) in lp.iter().enumerate() {
        let mut t = 1i128;
        for _ in 0..(d - i) {
            t = t * lambda as i128 % n;
        }
        acc = (acc + a as i128 * t).rem_euclid(n);
    }
    acc as u64
}

/// Hyperelliptic models y^2 + h y = f of genus g over F_2; for each target n, the
/// L-polynomials (with model counts) for which n | P(lambda) for some lambda of order
/// <= kmax.
/// Returns the L-polynomials found, with model counts.
pub fn run_hyperelliptic(g: i32, targets: &[u64], kmax: u64) -> BTreeMap<Vec<i64>, u64> {
    let units: Vec<Vec<(u64, u64)>> = targets
        .iter()
        .map(|&n| small_order_units(n, kmax))
        .collect();
    // (n, order) -> L -> models
    let mut cand: BTreeMap<(u64, u64), BTreeMap<Vec<i64>, u64>> = BTreeMap::new();
    let mut lcount: BTreeMap<Vec<i64>, u64> = BTreeMap::new();
    let mut smooth = 0u64;
    for h in 1..(1u64 << (g + 2)) {
        for f in 0..(1u64 << (2 * g + 3)) {
            let c = Curve { g, h, f };
            if !c.is_smooth_genus() {
                continue;
            }
            smooth += 1;
            let lp = lpoly(&c);
            *lcount.entry(lp.clone()).or_default() += 1;
            for (t, &n) in targets.iter().enumerate() {
                // smallest order of a lambda with n | P(lambda)
                let kmin = units[t]
                    .iter()
                    .filter(|&&(l, _)| p_at(&lp, l, n) == 0)
                    .map(|&(_, k)| k)
                    .min();
                if let Some(k) = kmin {
                    *cand
                        .entry((n, k))
                        .or_default()
                        .entry(lp.clone())
                        .or_default() += 1;
                }
            }
        }
    }
    print_candidates("hyperelliptic", g, &cand);
    print_summary(
        "hyperelliptic",
        g,
        smooth,
        &lcount,
        targets,
        kmax,
        &cand,
        "",
    );
    lcount
}

/// L(-1) = sum_k (-1)^k a_k.
pub fn l_at_minus_one(lp: &[i64]) -> i64 {
    lp.iter()
        .enumerate()
        .map(|(k, &a)| if k % 2 == 0 { a } else { -a })
        .sum()
}

/// (n, order of lambda) -> L-polynomial -> number of models.
pub type Candidates = BTreeMap<(u64, u64), BTreeMap<Vec<i64>, u64>>;

pub fn print_candidates(family: &str, g: i32, cand: &Candidates) {
    for ((n, k), m) in cand {
        for (lp, models) in m {
            let l1: i64 = lp.iter().sum();
            println!(
                "{{\"kind\":\"candidate\",\"family\":\"{}\",\"g\":{},\"n\":{},\"ord_lambda\":{},\"L\":{:?},\"L1\":{},\"models\":{}}}",
                family, g, n, k, lp, l1, models
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn print_summary(
    family: &str,
    g: i32,
    smooth: u64,
    lcount: &BTreeMap<Vec<i64>, u64>,
    targets: &[u64],
    kmax: u64,
    cand: &Candidates,
    extra: &str,
) {
    let primes = |f: &dyn Fn(&Vec<i64>) -> i64| -> Vec<u64> {
        let set: std::collections::BTreeSet<u64> = lcount
            .keys()
            .flat_map(|lp| crate::odd_primes(f(lp)))
            .collect();
        set.into_iter().collect()
    };
    let per: Vec<String> = targets
        .iter()
        .map(|&n| {
            let by_k: Vec<String> = (1..=kmax)
                .map(|k| {
                    let m = cand.get(&(n, k));
                    format!(
                        "\"{}\":{{\"lpolys\":{},\"models\":{}}}",
                        k,
                        m.map_or(0, |m| m.len()),
                        m.map_or(0, |m| m.values().sum::<u64>())
                    )
                })
                .collect();
            format!("\"{}\":{{{}}}", n, by_k.join(","))
        })
        .collect();
    println!(
        "{{\"kind\":\"screen_summary\",\"family\":\"{}\",\"g\":{},\"smooth_models\":{},\"distinct_L\":{},{}\"max_L1\":{},\"max_abs_Lm1\":{},\"max_N1\":{},\"odd_primes_L1\":{:?},\"odd_primes_Lm1\":{:?},\"by_n_and_ord_lambda\":{{{}}}}}",
        family,
        g,
        smooth,
        lcount.len(),
        extra,
        lcount.keys().map(|lp| lp.iter().sum::<i64>()).max().unwrap_or(0),
        lcount.keys().map(|lp| l_at_minus_one(lp).abs()).max().unwrap_or(0),
        lcount.keys().map(|lp| 3 + lp[1]).max().unwrap_or(0),
        primes(&|lp| lp.iter().sum::<i64>()),
        primes(&|lp| l_at_minus_one(lp)),
        per.join(",")
    );
}
