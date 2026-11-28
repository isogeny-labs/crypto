//! Prime31 W8: exact slot-root resultant membership on the unchanged original curve.
//! Eight public slot supports, each with eight roots. This only verifies
//! unsigned transformed-root membership, NOT target-conditioned RR9 extraction.
//! Frozen protocol: ../rr9/WEIGHTED_SLOT_RESULTANT_PROTOCOL_20261119.md.
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashSet;

const SEED: u64 = 0x534c_4f54_5f52_4553;
const WINDOWS: [[usize; 8]; 4] = [
    [0, 1, 2, 3, 4, 5, 6, 7],
    [8, 9, 10, 11, 12, 13, 14, 15],
    [16, 17, 18, 19, 20, 21, 22, 23],
    [23, 24, 25, 26, 27, 28, 29, 30],
];

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn add_mod(x: u64, y: u64) -> u64 {
    (x + y) % PRIME31.p
}
fn sub_mod(x: u64, y: u64) -> u64 {
    if x >= y { x - y } else { PRIME31.p + x - y }
}
fn mul_mod(x: u64, y: u64) -> u64 {
    ((x as u128 * y as u128) % PRIME31.p as u128) as u64
}
fn monic_from_roots(roots: &[u64; 8]) -> Vec<u64> {
    // Coefficient order: constant term first; leading coefficient 1.
    let mut c = vec![1u64];
    for &root in roots {
        let mut n = vec![0u64; c.len() + 1];
        for (j, &v) in c.iter().enumerate() {
            n[j] = sub_mod(n[j], mul_mod(v, root));
            n[j + 1] = add_mod(n[j + 1], v);
        }
        c = n;
    }
    assert_eq!(c.len(), 9);
    assert_eq!(c[8], 1);
    c
}
fn eval(c: &[u64], x: u64) -> u64 {
    c.iter().rev().fold(0u64, |acc, &v| add_mod(mul_mod(acc, x), v))
}
fn resultant(c: &[u64], slot: &[u64; 8]) -> u64 {
    // Res_X(H_i,M) = product_{alpha root H_i} M(alpha)
    slot.iter().fold(1u64, |acc, &x| mul_mod(acc, eval(c, x)))
}
fn jacobian_det(c: &[u64], slot: &[[u64; 8]; 8], roots: &[u64; 8]) -> u64 {
    // Differentiating product_t M(t) at its unique zero t_i gives
    // row i = (product_{t != t_i} M(t)) * [1,t_i,...,t_i^7].
    // Thus the determinant is a row-scale product times Vandermonde.
    let mut det = 1u64;
    for i in 0..8 {
        for &t in &slot[i] {
            if t != roots[i] {
                det = mul_mod(det, eval(c, t));
            }
        }
    }
    for i in 0..8 {
        for j in (i + 1)..8 {
            det = mul_mod(det, sub_mod(roots[j], roots[i]));
        }
    }
    det
}
fn base31(ar: &mut Arithmetic) -> Vec<(u64, u64)> {
    let mut points = Vec::new();
    for x in 0..10_000 {
        let y2 = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(y2, PRIME31.p) {
            if y != 0 {
                points.push((x, y.min(PRIME31.p - y)));
                if points.len() == 31 { break; }
            }
        }
    }
    assert_eq!(points.len(), 31);
    points
}
fn run(cases_per_window: usize) -> Value {
    assert!((1..=128).contains(&cases_per_window));
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("unchanged original Prime31");
    let base = base31(&mut ar);
    let mut state = SEED;
    let mut positives = 0u64;
    let mut negatives = 0u64;
    let mut positive_zero_resultants = 0u64;
    let mut negative_only_slot0_failed = 0u64;
    let mut jacobian_nonsingular = 0u64;
    let mut first = Vec::new();
    let mut windows_json = Vec::new();

    for (wi, window) in WINDOWS.iter().enumerate() {
        let fb: Vec<(u64, u64)> = window.iter().map(|&j| base[j]).collect();
        let points: [[(u64, u64); 8]; 8] = std::array::from_fn(|i| {
            std::array::from_fn(|j| {
                ar.scalar(PRIME31, 1u64 << i, Some(fb[j]))
                    .expect("prime order excludes doubling to infinity")
            })
        });
        let xsets: [[u64; 8]; 8] =
            std::array::from_fn(|i| std::array::from_fn(|j| points[i][j].0));
        let distinct: HashSet<u64> = xsets.iter().flat_map(|slot| slot.iter().copied()).collect();
        assert_eq!(distinct.len(), 64, "slot supports overlap on original curve");
        for slot in &xsets {
            assert_eq!(slot.iter().copied().collect::<HashSet<_>>().len(), 8);
        }
        let mut window_pos = 0u64;
        let mut window_neg = 0u64;
        for case in 0..cases_per_window {
            let mut choices = [0usize; 8];
            let mut signs = [0u64; 8];
            let mut roots = [0u64; 8];
            let mut q: Point = None;
            for i in 0..8 {
                choices[i] = (splitmix64(&mut state) % 8) as usize;
                signs[i] = splitmix64(&mut state) & 1;
                roots[i] = xsets[i][choices[i]];
                let p = Some(points[i][choices[i]]);
                let signed = if signs[i] == 1 { ar.neg(PRIME31, p) } else { p };
                q = ar.add(PRIME31, q, signed);
            }
            assert!(ar.on_curve(PRIME31, q));
            assert_eq!(roots.iter().copied().collect::<HashSet<_>>().len(), 8);
            let m = monic_from_roots(&roots);
            let pos: [u64; 8] = std::array::from_fn(|i| resultant(&m, &xsets[i]));
            assert_eq!(pos, [0u64; 8], "planted M not in each slot support");
            let jac_det = jacobian_det(&m, &xsets, &roots);
            assert_ne!(jac_det, 0, "radical complete-intersection Jacobian singular");
            jacobian_nonsingular += 1;
            positives += 1;
            window_pos += 1;
            positive_zero_resultants += pos.iter().filter(|&&r| r == 0).count() as u64;

            // Omit the slot-0 root; insert a different slot-1 root instead.
            let mut bad_roots = roots;
            bad_roots[0] = xsets[1][(choices[1] + 1) % 8];
            assert_eq!(bad_roots.iter().copied().collect::<HashSet<_>>().len(), 8);
            let bad = monic_from_roots(&bad_roots);
            let neg: [u64; 8] = std::array::from_fn(|i| resultant(&bad, &xsets[i]));
            assert_ne!(neg[0], 0, "missing slot-0 root incorrectly passed");
            assert_eq!(&neg[1..], &[0u64; 7], "unaffected slot unexpectedly failed");
            negatives += 1;
            window_neg += 1;
            negative_only_slot0_failed += 1;
            if case == 0 {
                first.push(json!({
                    "window": wi,
                    "choices": choices,
                    "signs": signs,
                    "positive_coefficients_c0_to_c8": m,
                    "negative_coefficients_c0_to_c8": bad,
                    "negative_slot0_resultant": neg[0],
                    "jacobian_det": jac_det,
                    "weighted_target": q,
                }));
            }
        }
        windows_json.push(json!({
            "window": wi, "slot_root_count": 8,
            "distinct_slot_roots": distinct.len(),
            "positive": window_pos, "negative": window_neg,
            "first_base_x": fb.iter().map(|pt| pt.0).collect::<Vec<_>>(),
        }));
    }
    json!({
        "schema": "prime31.weighted_slot_resultants.v1",
        "curve": PRIME31,
        "seed": format!("0x{SEED:016x}"),
        "cases_per_window": cases_per_window,
        "positive": positives,
        "negative": negatives,
        "positive_zero_resultants": positive_zero_resultants,
        "negative_only_slot0_failed": negative_only_slot0_failed,
        "jacobian_nonsingular": jacobian_nonsingular,
        "membership_geometric_quotient_dimension": 16_777_216u64,
        "windows": windows_json,
        "first_case_per_window": first,
        "candidate_monic_degree": 8,
        "slot_membership_equations": 8,
        "slot_supports_pairwise_disjoint": true,
        "generic_sum_table_calls": 0,
        "public_only_unknown_index_extraction": false,
        "target_conditioned_rr9_solved": false,
        "complete_ecdlp_solved": false,
        "independent_signed_pcc_verified": false,
        "original_group_additions": ar.counts.group_additions,
    })
}
fn main() {
    println!("{}", serde_json::to_string_pretty(&run(128)).expect("JSON"));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transformed_slot_root_resultants_on_original_curve() {
        let r = run(2);
        assert_eq!(r["positive"].as_u64(), Some(8));
        assert_eq!(r["negative"].as_u64(), Some(8));
        assert_eq!(r["positive_zero_resultants"].as_u64(), Some(64));
        assert_eq!(r["negative_only_slot0_failed"].as_u64(), Some(8));
        assert_eq!(r["jacobian_nonsingular"].as_u64(), Some(8));
        assert_eq!(r["slot_supports_pairwise_disjoint"].as_bool(), Some(true));
        assert_eq!(r["public_only_unknown_index_extraction"].as_bool(), Some(false));
    }
}
