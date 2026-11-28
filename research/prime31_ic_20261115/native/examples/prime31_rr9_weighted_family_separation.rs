//! Exact relation-family discriminator on ORIGINAL Prime31 Arithmetic.
//!
//! Fixed squarefree, unweighted RR9/M8=H_B is NOT the slot-weighted W8 language.
//! Every fixture is deliberately planted and uses a generic 256-sum grader.
//! This is NOT public-only extraction or a complete ECDLP benchmark.
//! Protocol: ../rr9/RR9_WEIGHTED_FAMILY_SEPARATION_PROTOCOL_20261119.md.
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashSet;

const SEED: u64 = 0x5252_395f_5745_4947;
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

fn base31(ar: &mut Arithmetic) -> Vec<(u64, u64)> {
    let mut base = Vec::new();
    for x in 0..10_000 {
        let y2 = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(y2, PRIME31.p) {
            if y != 0 {
                base.push((x, y.min(PRIME31.p - y)));
                if base.len() == 31 {
                    break;
                }
            }
        }
    }
    assert_eq!(base.len(), 31, "unchanged original base required");
    base
}

fn squarefree_support(base: &[(u64, u64)], ar: &mut Arithmetic) -> HashSet<Point> {
    assert_eq!(base.len(), 8);
    let mut out = HashSet::new();
    for mask in 0..256u32 {
        let mut r: Point = None;
        for (i, &pt) in base.iter().enumerate() {
            let leaf = if (mask >> i) & 1 == 0 {
                Some(pt)
            } else {
                ar.neg(PRIME31, Some(pt))
            };
            r = ar.add(PRIME31, r, leaf);
        }
        out.insert(r);
    }
    out
}

fn run(cases_per_window: usize) -> Value {
    assert!((1..=128).contains(&cases_per_window));
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("original full prime-order curve");
    let base = base31(&mut ar);
    let mut state = SEED;
    let mut total = 0u64;
    let mut weighted_in_squarefree_support = 0u64;
    let mut weighted_equal_same_signed_unweighted = 0u64;
    let mut transformed_x_equal_fixed_hb = 0u64;
    let mut weighted_identity = 0u64;
    let mut examples = Vec::new();
    let mut by_window = Vec::new();

    for (wi, win) in WINDOWS.iter().enumerate() {
        let fb: Vec<(u64, u64)> = win.iter().map(|&j| base[j]).collect();
        let support = squarefree_support(&fb, &mut ar);
        assert_eq!(support.len(), 256, "squarefree support unexpectedly collides");
        let fixed_x: Vec<u64> = fb.iter().map(|p| p.0).collect();
        let mut matched = 0u64;
        for case in 0..cases_per_window {
            let mut perm = [0usize, 1, 2, 3, 4, 5, 6, 7];
            for i in (1..8).rev() {
                let j = (splitmix64(&mut state) % ((i + 1) as u64)) as usize;
                perm.swap(i, j);
            }
            let signed: [(u64, u64); 8] = std::array::from_fn(|i| {
                let point = fb[perm[i]];
                if splitmix64(&mut state) & 1 == 0 {
                    point
                } else {
                    ar.neg(PRIME31, Some(point)).expect("ordinary leaf")
                }
            });
            let mut plain: Point = None;
            let mut weighted: Point = None;
            let mut transformed_x = Vec::new();
            for (i, &leaf) in signed.iter().enumerate() {
                let doubled = ar.scalar(PRIME31, 1u64 << i, Some(leaf))
                    .expect("prime-order group has no 2-power torsion");
                transformed_x.push(doubled.0);
                weighted = ar.add(PRIME31, weighted, Some(doubled));
                plain = ar.add(PRIME31, plain, Some(leaf));
            }
            assert!(support.contains(&plain), "unweighted planted control failed");
            if support.contains(&weighted) {
                weighted_in_squarefree_support += 1;
                matched += 1;
            }
            weighted_equal_same_signed_unweighted += u64::from(weighted == plain);
            transformed_x.sort_unstable();
            transformed_x_equal_fixed_hb += u64::from(transformed_x == fixed_x);
            weighted_identity += u64::from(weighted.is_none());
            assert!(ar.on_curve(PRIME31, weighted), "weighted result off curve");
            if case == 0 {
                examples.push(json!({
                    "window": wi,
                    "signed": signed,
                    "unweighted_target": plain,
                    "weighted_target": weighted,
                    "transformed_x_sorted": transformed_x,
                }));
            }
            total += 1;
        }
        by_window.push(json!({
            "window": wi,
            "squarefree_support_size": support.len(),
            "planted_weighted_fixtures": cases_per_window,
            "weighted_targets_in_squarefree_support": matched,
        }));
    }
    json!({
        "schema": "prime31.rr9.weighted_family_separation.v1",
        "curve": PRIME31,
        "seed": format!("0x{SEED:016x}"),
        "cases_per_window": cases_per_window,
        "planted_weighted_total": total,
        "weighted_targets_in_squarefree_support": weighted_in_squarefree_support,
        "weighted_equal_same_signed_unweighted": weighted_equal_same_signed_unweighted,
        "transformed_x_equal_fixed_hb": transformed_x_equal_fixed_hb,
        "weighted_identity": weighted_identity,
        "per_window": by_window,
        "first_case_per_window": examples,
        "fixed_squarefree_support_bound": 256,
        "weighted_ordered_witnesses": 4_294_967_296u64,
        "group_order": PRIME31.n,
        "generic_squarefree_grader_used": true,
        "public_only_unknown_index_extraction": false,
        "complete_ecdlp_solved": false,
        "independent_signed_pcc_verified": false,
        "original_group_additions_including_grader": ar.counts.group_additions,
    })
}

fn main() {
    println!("{}", serde_json::to_string_pretty(&run(128)).expect("valid JSON"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squarefree_vs_weighted_original_curve_controls() {
        let result = run(8);
        assert_eq!(result["planted_weighted_total"].as_u64(), Some(32));
        assert_eq!(result["fixed_squarefree_support_bound"].as_u64(), Some(256));
        assert_eq!(result["generic_squarefree_grader_used"].as_bool(), Some(true));
        assert_eq!(result["public_only_unknown_index_extraction"].as_bool(), Some(false));
        assert_eq!(result["complete_ecdlp_solved"].as_bool(), Some(false));
        assert_eq!(result["transformed_x_equal_fixed_hb"].as_u64(), Some(0));
    }
}
