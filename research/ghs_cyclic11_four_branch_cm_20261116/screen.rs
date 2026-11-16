use std::collections::{BTreeMap, BTreeSet};

const R: [i64; 5] = [1, 3, 4, 5, 9];
const N: [i64; 5] = [2, 6, 7, 8, 10];

fn signature(exponents: &[i64; 4], i: i64) -> i64 {
    let sum: i64 = exponents
        .iter()
        .map(|&e| (-i * e).rem_euclid(11))
        .sum();
    assert_eq!(sum % 11, 0);
    sum / 11 - 1
}

fn bound(exponents: &[i64; 4]) -> i64 {
    let r = R.iter().map(|&i| signature(exponents, i)).min().unwrap();
    let n = N.iter().map(|&i| signature(exponents, i)).min().unwrap();
    5 * (r + n)
}

fn opposite_pairs(exponents: &[i64; 4]) -> bool {
    (1..4).any(|j| {
        let other: Vec<_> = (1..4).filter(|&k| k != j).collect();
        (exponents[0] + exponents[j]) % 11 == 0
            && (exponents[other[0]] + exponents[other[1]]) % 11 == 0
    })
}

fn fifth_trace(s: i64, p: i64) -> i64 {
    s.pow(5) - 5 * p * s.pow(3) + 5 * p * p * s
}

fn cm_traces(p: i64) -> Vec<(i64, i64, bool)> {
    let q = p.pow(5);
    let mut out = Vec::new();
    for t in (-2 * q)..=(2 * q) {
        if t * t > 4 * q || t % p == 0 {
            continue;
        }
        for m in 1..=(2 * q) {
            let lhs = t * t + 11 * m * m;
            if lhs == 4 * q {
                let descended = (-2 * p..=2 * p)
                    .any(|s| s * s <= 4 * p && fifth_trace(s, p) == t);
                out.push((t, m, descended));
            }
            if lhs >= 4 * q {
                break;
            }
        }
    }
    out
}

fn main() {
    let mut histogram = BTreeMap::<i64, usize>::new();
    let mut paired_by_bound = BTreeMap::<i64, usize>::new();
    let mut mismatches = Vec::new();
    let mut bound_ten_patterns = BTreeSet::<[i64; 4]>::new();
    let mut rows = 0_usize;
    for a in 1..=10 {
        for b in 1..=10 {
            for c in 1..=10 {
                let d = (-(a + b + c)).rem_euclid(11);
                if d == 0 {
                    continue;
                }
                let exponents = [a, b, c, d];
                let v = bound(&exponents);
                let paired = opposite_pairs(&exponents);
                *histogram.entry(v).or_default() += 1;
                if paired {
                    *paired_by_bound.entry(v).or_default() += 1;
                }
                if (v == 10) != paired {
                    mismatches.push(exponents);
                }
                if v == 10 {
                    let mut sorted = exponents;
                    sorted.sort();
                    bound_ten_patterns.insert(sorted);
                }
                rows += 1;
            }
        }
    }
    println!("schema=ghs.cyclic11-four-branch-cm/v1");
    println!("ordered_inertia_rows={rows}");
    println!("bouw_bound_histogram={histogram:?}");
    println!("opposite_pairs_by_bound={paired_by_bound:?}");
    println!("bound_ten_iff_opposite_pairs={}", mismatches.is_empty());
    println!("counterexamples={mismatches:?}");
    println!("bound_ten_sorted_patterns={bound_ten_patterns:?}");
    for p in [3, 5] {
        println!("p={p} ordinary_cm_traces_t_m_descended={:?}", cm_traces(p));
    }
}
