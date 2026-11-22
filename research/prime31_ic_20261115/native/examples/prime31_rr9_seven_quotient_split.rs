//! Seven-of-eight RR9 quotient: exact B8 split-fibre diagnostic.
//! Reuses the original prime31-algebraic-ic Arithmetic and actual curve.
//! NOT public-only nonenumerative relation extraction and NOT a DLP solver.
//! Frozen protocol: RR9_SEVEN_QUOTIENT_SPLIT_PROTOCOL_20261117.md.
//!
//! cargo +1.100.0 test --release --locked --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_rr9_seven_quotient_split
//! cargo +1.100.0 run --release --locked --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_rr9_seven_quotient_split

use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::time::Instant;

const X: [u64; 8] = [0, 1, 2, 3, 5, 6, 8, 9];
const POSITIVE: (u64, u64) = (1_911_579_686, 548_820_096);
const NEGATIVE_2: (u64, u64) = (1_923_055_775, 307_731_458);
const P: u64 = 2_147_483_647;

fn add(a: u64, b: u64) -> u64 {
    (a + b) % P
}
fn sub(a: u64, b: u64) -> u64 {
    (a + P - b) % P
}
fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
fn power(mut a: u64, mut e: u64) -> u64 {
    let mut out = 1;
    while e != 0 {
        if e & 1 != 0 {
            out = mul(out, a);
        }
        a = mul(a, a);
        e >>= 1;
    }
    out
}
fn inverse(a: u64) -> u64 {
    assert_ne!(a, 0);
    power(a, P - 2)
}

// Order of seven unknowns: a0,a1,a2,a3,v0,v1,v2.
// V(X) = X^3 + v2 X^2 + v1 X + v0.
// L_i^sign = (x_i-r) A(x_i) + (s+sign*y_i) V(x_i).
fn signed_row(x: u64, y: u64, r: u64, s: u64, positive: bool) -> ([u64; 7], u64) {
    let z = sub(x, r);
    let sy = add(s, if positive { y } else { sub(0, y) });
    let x2 = mul(x, x);
    let x3 = mul(x2, x);
    (
        [z, mul(z, x), mul(z, x2), mul(z, x3),
         sy, mul(sy, x), mul(sy, x2)],
        sub(0, mul(sy, x3)),
    )
}

fn solve_7x7(rows: [[u64; 8]; 7]) -> Option<[u64; 7]> {
    let mut m = rows;
    for col in 0..7 {
        let pivot = (col..7).find(|&i| m[i][col] != 0)?;
        m.swap(col, pivot);
        let inv = inverse(m[col][col]);
        for j in col..8 {
            m[col][j] = mul(m[col][j], inv);
        }
        for i in 0..7 {
            if i == col {
                continue;
            }
            let c = m[i][col];
            if c != 0 {
                for j in col..8 {
                    m[i][j] = sub(m[i][j], mul(c, m[col][j]));
                }
            }
        }
    }
    let mut out = [0; 7];
    for (i, item) in out.iter_mut().enumerate() {
        *item = m[i][7];
    }
    Some(out)
}

fn affine_zero(x: u64, y: u64, r: u64, s: u64, positive: bool, coeff: &[u64; 7]) -> bool {
    let (row, rhs) = signed_row(x, y, r, s, positive);
    let mut dot = 0;
    for i in 0..7 {
        dot = add(dot, mul(row[i], coeff[i]));
    }
    dot == rhs
}

// Independently evaluates the UNFACTORED norm M(X).
fn norm_at(x: u64, r: u64, s: u64, c: &[u64; 7]) -> u64 {
    let x2 = mul(x, x);
    let x3 = mul(x2, x);
    let a = add(add(add(c[0], mul(c[1], x)), mul(c[2], x2)), mul(c[3], x3));
    let v = add(add(add(x3, c[4]), mul(c[5], x)), mul(c[6], x2));
    let d = add(add(add(x2, mul(r, x)), mul(r, r)), PRIME31.a);
    sub(
        sub(mul(d, mul(v, v)), mul(sub(x, r), mul(a, a))),
        mul(mul(add(s, s), a), v),
    )
}

fn base(ar: &mut Arithmetic) -> [(u64, u64); 8] {
    assert_eq!(P, PRIME31.p);
    let mut pts = [(0, 0); 8];
    for (i, &x) in X.iter().enumerate() {
        let rhs = ar.rhs(PRIME31, x);
        let y = ar.sqrt(rhs, P).expect("B8 x has an Fp point");
        assert_ne!(y, 0);
        pts[i] = (x, y.min(P - y));
        assert!(ar.on_curve(PRIME31, Some(pts[i])));
    }
    pts
}

fn case(ar: &mut Arithmetic, pts: &[(u64, u64); 8],
        public_r: (u64, u64), label: &str) -> Value {
    let (r, s) = public_r;
    assert!(ar.on_curve(PRIME31, Some(public_r)));
    assert!(!X.contains(&r), "exceptional x=r requires a separate branch");
    let start = Instant::now();
    let mut seen = HashSet::<[u64; 7]>::new();
    let mut first_seven_norm_checks = 0_usize;
    let mut selected = Vec::<Value>::new();
    let mut correct_group_replays = 0_usize;
    let mut double_sign_ambiguities = 0_usize;
    for mask in 0..128_u16 {
        let mut matrix = [[0_u64; 8]; 7];
        for i in 0..7 {
            let (row, rhs) = signed_row(pts[i].0, pts[i].1, r, s, mask & (1 << i) != 0);
            matrix[i][..7].copy_from_slice(&row);
            matrix[i][7] = rhs;
        }
        let c = solve_7x7(matrix).expect("registered 7x7 system must have rank 7");
        assert!(seen.insert(c), "two sign branches cannot share one chart point");
        for i in 0..7 {
            let sign = mask & (1 << i) != 0;
            assert!(affine_zero(pts[i].0, pts[i].1, r, s, sign, &c));
            if affine_zero(pts[i].0, pts[i].1, r, s, !sign, &c) {
                double_sign_ambiguities += 1;
            }
            assert_eq!(norm_at(pts[i].0, r, s, &c), 0,
                       "check unfactored norm, not just a chosen factor");
            first_seven_norm_checks += 1;
        }
        let last_plus = affine_zero(pts[7].0, pts[7].1, r, s, true, &c);
        let last_minus = affine_zero(pts[7].0, pts[7].1, r, s, false, &c);
        assert_eq!(norm_at(pts[7].0, r, s, &c) == 0, last_plus || last_minus);
        if last_plus || last_minus {
            assert_ne!(last_plus, last_minus, "last root orientation must be unambiguous");
            let mut sum: Point = None;
            for i in 0..8 {
                let positive = if i == 7 { last_plus } else { mask & (1 << i) != 0 };
                let pt = if positive {
                    Some(pts[i])
                } else {
                    ar.neg(PRIME31, Some(pts[i]))
                };
                sum = ar.add(PRIME31, sum, pt);
            }
            assert_eq!(sum, Some(public_r), "independent original group-law replay");
            correct_group_replays += 1;
            selected.push(json!({
                "first_seven_sign_mask": mask,
                "eighth_positive": last_plus,
                "coefficients": c,
                "original_group_replay": true,
            }));
        }
    }
    assert_eq!(seen.len(), 128);
    assert_eq!(double_sign_ambiguities, 0);
    assert_eq!(first_seven_norm_checks, 896);
    json!({
        "label": label,
        "public_r": public_r,
        "seven_sign_branches": 128,
        "full_rank_7x7": 128,
        "distinct_affine_points": seen.len(),
        "first_seven_unfactored_norm_checks": first_seven_norm_checks,
        "double_sign_ambiguities": double_sign_ambiguities,
        "eighth_selector_charts": selected.len(),
        "eighth_selector_original_group_replays": correct_group_replays,
        "selected": selected,
        "diagnostic_elapsed_ns": start.elapsed().as_nanos().to_string(),
    })
}

fn run() -> Value {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("original ordinary cofactor-one prime31");
    let pts = base(&mut ar);
    let cases = [
        case(&mut ar, &pts, POSITIVE, "frozen_positive"),
        case(&mut ar, &pts, PRIME31.g, "frozen_negative_generator"),
        case(&mut ar, &pts, NEGATIVE_2, "frozen_negative_2"),
    ];
    assert_eq!(cases[0]["eighth_selector_charts"], 1);
    assert_eq!(cases[1]["eighth_selector_charts"], 0);
    assert_eq!(cases[2]["eighth_selector_charts"], 0);
    json!({
        "schema": "prime31.rr9.seven-quotient-split.v1",
        "p": PRIME31.p,
        "n": PRIME31.n,
        "a": PRIME31.a,
        "b": PRIME31.b,
        "base_x": X,
        "base_points_canonical_y": pts,
        "cases": cases,
        "theorem_scope": "seven B8 factor-root quadrics, x_i != x(R), all 128 sign intersections transverse and rational",
        "exact_seven_quadric_quotient_dimension": 128,
        "quotient_is_split_product_of_128_fp": true,
        "is_public_only_nonenumerative_extraction": false,
        "complete_unknown_prime31_logs": 0,
        "pcc": "unverified",
    })
}

fn main() {
    println!("{}", run());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prime31_exact_split_quotient_and_eighth_selector() {
        let r = run();
        assert_eq!(r["cases"][0]["distinct_affine_points"], 128);
        assert_eq!(r["cases"][0]["eighth_selector_charts"], 1);
        assert_eq!(r["cases"][1]["eighth_selector_charts"], 0);
        assert_eq!(r["cases"][2]["eighth_selector_charts"], 0);
        assert_eq!(r["complete_unknown_prime31_logs"], 0);
    }
}
