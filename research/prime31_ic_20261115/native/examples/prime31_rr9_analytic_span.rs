//! Exact eight-row certificate for RR9 exterior-span full rank on original Prime31.
//! Reuses prime31_algebraic_ic::Arithmetic, not an independent replacement crate.
//! The exterior rank-70 consequence follows from a multilinearity theorem;
//! this binary tests only its four-signed-pair sufficient condition.
//! Companion report: rr9/RR9_SPAN_FULLRANK_SHORTCUT_20261119.md
use prime31_algebraic_ic::{Arithmetic, Point, PRIME31};
use serde_json::json;
use std::collections::HashSet;

const P: u64 = 2_147_483_647;
const G: Point = Some((625_037_715, 2_100_627_251));
const W: [u64; 8] = [1, 3, 5, 7, 11, 13, 17, 19];
const DN: [u64; 8] = [
    1_468_457_909,
    1_667_486_837,
    936_410_919,
    886_543_102,
    2_137_754_017,
    1_290_111_997,
    1_235_269_058,
    1_541_396_348,
];
const DP: [u64; 8] = [
    1_418_252_579,
    1_982_109_171,
    1_921_836_591,
    1_045_673_972,
    1_683_787_312,
    2_146_239_327,
    1_923_764_810,
    754_634_012,
];
#[inline]
fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
#[inline]
fn sub(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + P - b
    }
}
fn pow(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1;
    while e > 0 {
        if e & 1 != 0 {
            r = mul(r, a);
        }
        a = mul(a, a);
        e >>= 1;
    }
    r
}
fn inverse(a: u64) -> u64 {
    assert_ne!(a, 0);
    pow(a, P - 2)
}
fn row(q: (u64, u64), target: (u64, u64)) -> [u64; 8] {
    let (x, y) = q;
    let (r, s) = target;
    let mut a = [0; 8];
    let mut xk = 1;
    for k in 0..4 {
        a[k] = mul(sub(x, r), xk);
        a[k + 4] = mul((y + s) % P, xk);
        xk = mul(xk, x);
    }
    a
}
fn det8(mut a: [[u64; 8]; 8]) -> u64 {
    let mut ans = 1u64;
    for k in 0..8 {
        let Some(pivot_idx) = (k..8).find(|&i| a[i][k] != 0) else {
            return 0;
        };
        if pivot_idx != k {
            a.swap(k, pivot_idx);
            ans = sub(0, ans);
        }
        let pivot = a[k][k];
        ans = mul(ans, pivot);
        let pivot_inverse = inverse(pivot);
        for i in (k + 1)..8 {
            let factor = mul(a[i][k], pivot_inverse);
            for j in (k + 1)..8 {
                a[i][j] = sub(a[i][j], mul(factor, a[k][j]));
            }
            a[i][k] = 0;
        }
    }
    ans
}
fn formula(four: &[(u64, u64); 4], r: u64) -> u64 {
    let mut d = 16;
    for &(x, y) in four {
        assert_ne!(y, 0, "finite point with y=0");
        assert_ne!(x, r, "target x collides with a selected leaf x");
        d = mul(d, mul(sub(x, r), y));
    }
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(four[i].0, four[j].0, "distinct abscissae required");
            let t = sub(four[j].0, four[i].0);
            d = mul(d, mul(t, t));
        }
    }
    d
}
fn canonical_four(ar: &mut Arithmetic) -> [(u64, u64); 4] {
    let mut pts = Vec::new();
    for x in 0..20 {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, P) {
            if y != 0 {
                let p = (x, y.min(P - y));
                assert!(ar.on_curve(PRIME31, Some(p)));
                pts.push(p);
            }
            if pts.len() == 4 {
                break;
            }
        }
    }
    assert_eq!(pts.len(), 4);
    assert_eq!(
        pts.iter().map(|p| p.0).collect::<Vec<_>>(),
        vec![0, 1, 2, 3]
    );
    [pts[0], pts[1], pts[2], pts[3]]
}
fn audit() -> serde_json::Value {
    let mut ar = Arithmetic::default();
    let base = canonical_four(&mut ar);
    let mut planted: Point = None;
    for &w in &W {
        let term = ar.scalar(PRIME31, w, Some(base[0]));
        planted = ar.add(PRIME31, planted, term);
    }
    assert_eq!(planted, Some((1_991_076_510, 718_644_930)));
    let mut forbidden_x = HashSet::new();
    for &w in &W {
        for &q in &base {
            let weighted = ar
                .scalar(PRIME31, w, Some(q))
                .expect("non-infinite weighted point");
            forbidden_x.insert(weighted.0);
        }
    }
    assert_eq!(
        forbidden_x.len(),
        32,
        "exact 32 distinct forbidden target x-coordinates"
    );
    assert!(!forbidden_x.contains(&G.expect("finite G").0));
    assert!(!forbidden_x.contains(&planted.expect("finite P").0));
    let mut cases = Vec::new();
    for (name, target, expectation) in [("N", G, DN), ("P", planted, DP)] {
        let target_xy = target.expect("finite residual");
        let mut determinants = Vec::new();
        for (slot, &w) in W.iter().enumerate() {
            let four = core::array::from_fn(|i| {
                ar.scalar(PRIME31, w, Some(base[i]))
                    .expect("non-infinite leaf")
            });
            let mut rows = [[0u64; 8]; 8];
            for i in 0..4 {
                rows[i] = row(four[i], target_xy);
                rows[i + 4] = row((four[i].0, sub(0, four[i].1)), target_xy);
            }
            let computed = det8(rows);
            assert_eq!(computed, formula(&four, target_xy.0), "formula slot {slot}");
            assert_ne!(computed, 0, "full-rank slot {slot}");
            assert_eq!(computed, expectation[slot], "frozen residue slot {slot}");
            determinants.push(computed);
        }
        cases.push(json!({"name":name,"determinants":determinants,
            "B4_rank_pairing_by_theorem":70,
            "B8_rank_pairing_by_theorem":70}));
    }
    json!({"p":P,"curve":"original ordinary cofactor-one Prime31",
        "weight_count":W.len(),"selected_base_x":[0,1,2,3],
        "forbidden_target_x_count":forbidden_x.len(),
        "max_unproved_target_points_for_this_certificate":2*forbidden_x.len(),
        "proof":"8x8 slot det nonzero => every slot spans Fp^8 => wedge^4 of each half spans dimension 70 => complement pairing rank 70",
        "scope":"N4,P4,N8,P8 frozen cases; no full extractor or ECDLP claim",
        "cases":cases,"pcc":"unverified"})
}
fn main() {
    println!("{}", serde_json::to_string_pretty(&audit()).unwrap());
}
#[cfg(test)]
mod tests {
    #[test]
    fn fixed_prime31_slot_certificates() {
        let result = super::audit();
        assert_eq!(result["cases"].as_array().unwrap().len(), 2);
    }
}
