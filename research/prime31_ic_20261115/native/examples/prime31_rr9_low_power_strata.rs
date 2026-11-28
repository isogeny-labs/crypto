//! Exact RR9 bounded-power support-strata control on the *original* Prime31 curve.
//!
//! Reuses prime31_algebraic_ic's pinned original-curve arithmetic.
//! It does NOT extract an unknown M, solve a discrete logarithm, or claim
//! a non-generic speedup. See RR9_LOW_POWER_STRATA_PROTOCOL_20261118.md.
use prime31_algebraic_ic::{check_curve, Arithmetic, Curve, Point, PRIME31};
use serde_json::{json, Value};

const SLOTS: usize = 8;
const B: usize = 31;

fn choose(n: usize, k: usize) -> u128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut v = 1u128;
    for i in 0..k {
        v = v * (n - i) as u128 / (i + 1) as u128;
    }
    v
}

/// Number of ordered eight-slot x-label words where no label occurs > cap times.
/// DP selects which still-unassigned labelled positions belong to each x-label.
fn occupancy_count(labels: usize, cap: usize) -> u128 {
    let mut dp = [0u128; SLOTS + 1];
    dp[0] = 1;
    for _ in 0..labels {
        let mut next = [0u128; SLOTS + 1];
        for used in 0..=SLOTS {
            for take in 0..=cap.min(SLOTS - used) {
                next[used + take] += dp[used] * choose(SLOTS - used, take);
            }
        }
        dp = next;
    }
    dp[SLOTS]
}

fn poly_mul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut v = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            let z = v[i + j] as u128 + (x as u128) * (y as u128);
            v[i + j] = (z % p as u128) as u64;
        }
    }
    trim(v)
}

fn trim(mut v: Vec<u64>) -> Vec<u64> {
    while v.last() == Some(&0) {
        v.pop();
    }
    v
}

/// Degree < deg(m) remainder, m must be monic. No inversions or EC enumeration.
fn poly_mod_monic(f: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    assert!(m.len() >= 2 && m.last() == Some(&1));
    let mut r = trim(f.to_vec());
    while r.len() >= m.len() {
        let coefficient = *r.last().unwrap();
        let offset = r.len() - m.len();
        for (j, &mj) in m.iter().enumerate() {
            let prod = ((coefficient as u128 * mj as u128) % p as u128) as u64;
            r[offset + j] = (r[offset + j] + p - prod) % p;
        }
        r = trim(r);
    }
    r
}

fn poly_from_roots(roots: &[u64], p: u64) -> Vec<u64> {
    let mut v = vec![1u64];
    for &x in roots {
        assert!(x < p);
        v = poly_mul(&v, &[(p - x) % p, 1], p);
    }
    v
}

/// P_1,P_2,P_4,P_8 computed by the three frozen modular squarings.
fn power_gates(h: &[u64], m: &[u64], p: u64) -> [bool; 4] {
    let mut r = poly_mod_monic(h, m, p);
    let mut out = [r.is_empty(), false, false, false];
    for hit in out.iter_mut().skip(1) {
        r = poly_mod_monic(&poly_mul(&r, &r, p), m, p);
        *hit = r.is_empty();
    }
    out
}

/// First 31 rational liftable abscissae; exactly the established first-x B31.
fn first_x_lifts(e: Curve, ar: &mut Arithmetic) -> Vec<(u64, u64)> {
    let mut points = vec![];
    for x in 0..10000 {
        let rhs = ar.rhs(e, x);
        if let Some(y) = ar.sqrt(rhs, e.p) {
            if y != 0 {
                let y = y.min(e.p - y);
                assert!(ar.on_curve(e, Some((x, y))));
                points.push((x, y));
            }
        }
        if points.len() == B + 1 {
            break;
        }
    }
    assert_eq!(points.len(), B + 1, "first-x base needs a rational omitted x");
    points
}

fn check_partition(
    partition: &[usize],
    base: &[(u64, u64)],
    h: &[u64],
    ar: &mut Arithmetic,
) -> Value {
    assert_eq!(partition.iter().sum::<usize>(), SLOTS);
    let e = PRIME31;
    let mut roots = vec![];
    let mut sum: Point = None;
    for (&multiplicity, &(x, y)) in partition.iter().zip(base.iter()) {
        assert!(ar.on_curve(e, Some((x, y))));
        for _ in 0..multiplicity {
            roots.push(x);
            sum = ar.add(e, sum, Some((x, y)));
        }
    }
    assert_eq!(roots.len(), SLOTS);
    assert!(ar.on_curve(e, sum));
    let m = poly_from_roots(&roots, e.p);
    assert_eq!(m.len(), SLOTS + 1);
    let flags = power_gates(h, &m, e.p);
    let max_multiplicity = *partition.iter().max().unwrap();
    for (flag, k) in flags.iter().zip([1usize, 2, 4, 8]) {
        assert_eq!(*flag, max_multiplicity <= k, "false support gate");
    }
    json!({
        "multiplicities": partition,
        "maximum_multiplicity": max_multiplicity,
        "first_admitting_power": [1, 2, 4, 8].into_iter().find(|&k| k >= max_multiplicity).unwrap(),
        "p1_p2_p4_p8": flags,
        "replayed_public_eight_leaf_sum": sum,
        "full_support_accepted": flags[3],
    })
}

fn study() -> Value {
    let e = PRIME31;
    let mut ar = Arithmetic::default();
    check_curve(e, &mut ar).expect("the fixed ordinary cofactor-one curve must verify");
    let points = first_x_lifts(e, &mut ar);
    assert_eq!(
        points.iter().take(8).map(|(x, _)| *x).collect::<Vec<_>>(),
        [0, 1, 2, 3, 5, 6, 8, 9]
    );
    let xs: Vec<u64> = points[..B].iter().map(|&(x, _)| x).collect();
    let h = poly_from_roots(&xs, e.p);
    assert_eq!(h.len(), B + 1);

    let partitions: [&[usize]; 6] = [
        &[8],
        &[4, 4],
        &[3, 2, 1, 1, 1],
        &[2, 2, 2, 2],
        &[2, 1, 1, 1, 1, 1, 1],
        &[1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let rows: Vec<Value> = partitions
        .iter()
        .map(|&partition| check_partition(partition, &points, &h, &mut ar))
        .collect();
    let mut missing: Vec<u64> = points[..7].iter().map(|&(x, _)| x).collect();
    missing.push(points[B].0);
    let bad_m = poly_from_roots(&missing, e.p);
    let bad = power_gates(&h, &bad_m, e.p);
    assert_eq!(bad, [false; 4], "omitted on-curve root was accepted");

    let total = (B as u128).pow(SLOTS as u32);
    let caps: Vec<Value> = [1usize, 2, 4, 8]
        .into_iter()
        .map(|cap| {
            let count = occupancy_count(B, cap);
            json!({"max_x_multiplicity": cap, "ordered_words": count.to_string(),
                   "total_words": total.to_string(), "fraction": count as f64 / total as f64})
        })
        .collect();

    json!({
        "schema": "prime31.rr9.low-power-strata.v1",
        "classification": "exact-supplied-polynomial-support-only",
        "p": e.p, "a": e.a, "b": e.b, "n": e.n, "cofactor": 1,
        "base": "first31-liftable-rational-x", "base_x": xs,
        "group_relation_replay": "public-planted-only",
        "partitions": rows, "omitted_rational_root_rejected": bad == [false; 4],
        "occupancy": caps,
        "complete_unknown_target_ecdlp_recoveries": 0,
        "nonenumerative_unknown_M_extractions": 0,
        "timed_solver_measurements": 0,
        "producer_validation": "rust-tests-required",
        "arithmetic": ar.counts,
    })
}

fn main() {
    println!("{}", serde_json::to_string_pretty(&study()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_ordered_occupancy_counts() {
        assert_eq!(occupancy_count(B, 1), 318_073_392_000);
        assert_eq!(occupancy_count(B, 2), 809_127_509_400);
        assert_eq!(occupancy_count(B, 4), 852_843_376_770);
        assert_eq!(occupancy_count(B, 8), 852_891_037_441);
        assert_eq!(occupancy_count(8, 1), 40_320);
        assert_eq!(occupancy_count(1, 8), 1);
        assert_eq!(occupancy_count(1, 7), 0);
    }

    #[test]
    fn planted_actual_prime31_fibres_and_omitted_on_curve_root() {
        let d = study();
        assert_eq!(d["p"], 2147483647u64);
        assert_eq!(d["n"], 2147446831u64);
        assert_eq!(d["partitions"].as_array().unwrap().len(), 6);
        assert_eq!(d["omitted_rational_root_rejected"], true);
        assert_eq!(d["complete_unknown_target_ecdlp_recoveries"], 0);
        assert_eq!(d["nonenumerative_unknown_M_extractions"], 0);
        for (row, expected) in d["partitions"].as_array().unwrap().iter().zip([8, 4, 4, 2, 2, 1]) {
            assert_eq!(row["first_admitting_power"], expected);
            assert_eq!(row["full_support_accepted"], true);
        }
    }

    #[test]
    fn modular_power_agrees_with_direct_polynomial_exponentiation() {
        let p = PRIME31.p;
        let mut ar = Arithmetic::default();
        let points = first_x_lifts(PRIME31, &mut ar);
        let h = poly_from_roots(
            &points[..B].iter().map(|&(x, _)| x).collect::<Vec<_>>(), p
        );
        for multiplicities in [
            vec![8], vec![4,4], vec![3,2,1,1,1],
            vec![2,2,2,2], vec![1,1,1,1,1,1,1,1],
        ] {
            let roots: Vec<u64> = multiplicities.iter().zip(&points)
                .flat_map(|(&count, &(x, _))| std::iter::repeat(x).take(count))
                .collect();
            let m = poly_from_roots(&roots, p);
            let flags = power_gates(&h, &m, p);
            let mut direct = vec![1u64];
            for exponent in 1..=8 {
                direct = poly_mul(&direct, &h, p);
                if let Some(i) = [1,2,4,8].iter().position(|&v| v == exponent) {
                    assert_eq!(poly_mod_monic(&direct, &m, p).is_empty(), flags[i]);
                }
            }
        }
    }
}
