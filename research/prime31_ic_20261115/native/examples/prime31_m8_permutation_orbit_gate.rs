//! Prime31 balanced S4/S3 m8 orbit/solution-multiplicity gate.
//!
//! Reuses the native Rust Arithmetic and Base and the first *previously
//! collected* public 8-sum witness. It does not extract a new relation or
//! solve an ECDLP. A single positive eight-distinct-index signed witness
//! produces 8! distinct ordered rational roots of the balanced m8 ideal.
//! Two different index sets for the same public mask give at least 2*8! roots.
//! [pcc:unverified]
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use serde_json::json;
use std::collections::BTreeMap;

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const Q: (u64, u64) = (1_547_953_785, 1_667_575_595);
const A_MASK: u64 = 806_206_853;
const B_MASK: u64 = 542_783_019;
const CODES: [[u64; 2]; 2] = [[18_903_234, 258_036_944], [37_012_034, 107_578_577]];

fn add(x: u64, y: u64) -> u64 {
    let s = x + y;
    if s >= P { s - P } else { s }
}
fn sub(x: u64, y: u64) -> u64 {
    if x >= y { x - y } else { x + P - y }
}
fn mul(x: u64, y: u64) -> u64 {
    ((x as u128 * y as u128) % P as u128) as u64
}
fn sq(x: u64) -> u64 { mul(x, x) }
fn s3_coeff(x: u64, y: u64) -> (u64, u64, u64) {
    let sm = add(x, y);
    let prod = mul(x, y);
    (
        sq(sub(x, y)),
        sub(0, mul(2, add(mul(sm, add(prod, A)), mul(2, B)))),
        sub(sq(sub(prod, A)), mul(mul(4, B), sm)),
    )
}
fn s3(x: u64, y: u64, z: u64) -> u64 {
    let (a, b, c) = s3_coeff(x, y);
    add(add(mul(a, sq(z)), mul(b, z)), c)
}
fn s4(x: u64, y: u64, z: u64, w: u64) -> u64 {
    let (a, b, c) = s3_coeff(x, y);
    let (d, e, f) = s3_coeff(z, w);
    sub(sq(sub(mul(a, f), mul(c, d))),
        mul(sub(mul(a, e), mul(b, d)), sub(mul(b, f), mul(c, e))))
}
fn four(code: u64) -> [(usize, i8); 4] {
    std::array::from_fn(|j| (
        ((code >> (6 * j)) & 63) as usize,
        if ((code >> (24 + j)) & 1) == 0 { 1 } else { -1 },
    ))
}
fn sum(ar: &mut Arithmetic, ps: &[(u64, u64)], indices: &[usize]) -> Point {
    let mut acc = None;
    for &i in indices { acc = ar.add(PRIME31, acc, Some(ps[i])); }
    acc
}
fn next_permutation(a: &mut [usize; 8]) -> bool {
    let mut i = 6;
    while a[i] > a[i + 1] {
        if i == 0 { return false; }
        i -= 1;
    }
    let mut j = 7;
    while a[j] < a[i] { j -= 1; }
    a.swap(i, j);
    a[i + 1..].reverse();
    true
}
// S3 on each three-leaf block, S2 on the pair, and swap the two S3
// blocks: (3!)(3!)(2!)(2) = 144 elementary block relabellings.
fn orbit_key(a: &[usize; 8]) -> [u8; 8] {
    let mut l = [a[0] as u8, a[1] as u8, a[2] as u8];
    let mut r = [a[3] as u8, a[4] as u8, a[5] as u8];
    let mut t = [a[6] as u8, a[7] as u8];
    l.sort(); r.sort(); t.sort();
    if r < l { std::mem::swap(&mut l, &mut r); }
    [l[0], l[1], l[2], r[0], r[1], r[2], t[0], t[1]]
}
fn main() -> Result<(), String> {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    let base = Base::build(PRIME31, 6, &mut ar)?;
    if PRIME31.p != P || PRIME31.a != A || PRIME31.b != B
        || base.points.len() != 31 || base.terminal != 933_677_148 {
        return Err("registered ordinary prime31 curve/base changed".into());
    }
    let ag = ar.scalar(PRIME31, A_MASK, Some(PRIME31.g));
    let bq = ar.scalar(PRIME31, B_MASK, Some(Q));
    let target = ar.add(PRIME31, ag, bq)
        .ok_or("registered public mask is infinity")?;
    let mut checked: usize = 0;
    let mut orbit_counts: BTreeMap<(usize, [u8;8]), usize> = BTreeMap::new();
    let mut witness_sets = std::collections::HashSet::new();
    for (witness_index, &codes) in CODES.iter().enumerate() {
        let labels: Vec<_> = codes.iter().flat_map(|&x| four(x)).collect();
        let distinct: std::collections::HashSet<_> = labels.iter().map(|&(i, _)| i).collect();
        if labels.len() != 8 || distinct.len() != 8 {
            return Err("expected eight distinct factor-base indices".into());
        }
        let mut sorted_set: Vec<_> = distinct.into_iter().collect();
        sorted_set.sort_unstable();
        if !witness_sets.insert(sorted_set) {
            return Err("same index set repeated under distinct witness codes".into());
        }
        let ps: Vec<(u64, u64)> = labels.iter().map(|&(i, sign)| {
            let pt = Some(base.points[i]);
            if sign < 0 { ar.neg(PRIME31, pt).unwrap() } else { pt.unwrap() }
        }).collect();
        if sum(&mut ar, &ps, &[0,1,2,3,4,5,6,7]) != Some(target) {
            return Err("prior signed relation no longer checks".into());
        }
        let mut a: [usize;8] = std::array::from_fn(|i| i);
        loop {
            let u = sum(&mut ar, &ps, &a[..3]).ok_or("u=O, special case")?;
            let v = sum(&mut ar, &ps, &a[3..6]).ok_or("v=O, special case")?;
            let w = sum(&mut ar, &ps, &a[6..]).ok_or("w=O, special case")?;
            let uv = ar.add(PRIME31, Some(u), Some(v));
            if ar.add(PRIME31, uv, Some(w)) != Some(target) {
                return Err("permuted eight-summand equality failed".into());
            }
            if s4(ps[a[0]].0, ps[a[1]].0, ps[a[2]].0, u.0) != 0
                || s4(ps[a[3]].0, ps[a[4]].0, ps[a[5]].0, v.0) != 0
                || s3(ps[a[6]].0, ps[a[7]].0, w.0) != 0
                || s4(u.0, v.0, w.0, target.0) != 0 {
                return Err("balanced S4/S3 polynomial residual nonzero".into());
            }
            *orbit_counts.entry((witness_index, orbit_key(&a))).or_default() += 1;
            checked += 1;
            if !next_permutation(&mut a) { break; }
        }
    }
    if checked != 80_640 || orbit_counts.len() != 560
        || orbit_counts.values().any(|&count| count != 144)
        || witness_sets.len() != 2 {
        return Err("permutation/orbit certificate changed".into());
    }
    println!("{}", json!({
        "standing":"[pcc:unverified]",
        "curve_p":P,"curve_n":PRIME31.n,"factor_base_points":base.points.len(),
        "source":"two different previously recorded natural-positive m8 witnesses for the same public mask",
        "witness_mask":[A_MASK,B_MASK],
        "leaf_x_distinct_per_witness":8,"distinct_witness_index_sets":witness_sets.len(),
        "distinct_balanced_affine_roots_lower_bound":checked,
        "zero_polynomial_residuals_checked":4*checked,
        "relabel_symmetry_order":144,"orbits_per_witness":280,
        "symmetry_quotient_orbits":orbit_counts.len(),
        "orbit_size_min":orbit_counts.values().min(),
        "orbit_size_max":orbit_counts.values().max(),
        "claim":"solution-multiplicity lower bound, not F4 execution or ECDLP cost"
    }));
    Ok(())
}
