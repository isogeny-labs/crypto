//! EXP-R31-RR9: classical elliptic Riemann--Roch L(9O) determinant gate.
//!
//! This is a **relation predicate on GIVEN signed leaves**, not a point
//! extractor, not an unknown-scalar ECDLP solver, and not new complexity.
//! Nine inputs MUST be pairwise distinct finite points. Repetition without
//! elliptic Hermite jets yields false determinant-zero positives.
//!
//! Native pinned-source run:
//! cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_rr9_evaluation
//! cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_rr9_evaluation
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const P: u64 = 2_147_483_647;
const NCASES: usize = 32;
const CAP: usize = 10_000;
const SEED: u64 = 0x5252_395f_4445_5447;

#[inline]
fn fp_add(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= P { s - P } else { s }
}
#[inline]
fn fp_sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + P - b }
}
#[inline]
fn fp_mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
fn fp_pow(mut a: u64, mut e: u64) -> u64 {
    let mut v = 1;
    while e > 0 {
        if e & 1 == 1 { v = fp_mul(v, a); }
        a = fp_mul(a, a);
        e >>= 1;
    }
    v
}
fn neg(q: Point) -> Point {
    q.map(|(x, y)| (x, if y == 0 { 0 } else { P - y }))
}
fn splitmix64(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Basis for L(9 O), ordered by pole orders 0,2,3,4,5,6,7,8,9.
fn evaluation_row(q: Point) -> Result<[u64; 9], String> {
    let (x, y) = q.ok_or("infinity has no affine evaluation row")?;
    if x >= P || y >= P { return Err("noncanonical affine coordinates".into()); }
    let x2 = fp_mul(x, x);
    let x3 = fp_mul(x2, x);
    let x4 = fp_mul(x3, x);
    Ok([1, x, y, x2, fp_mul(x, y), x3, fp_mul(x2, y), x4, fp_mul(x3, y)])
}

/// Modular Gaussian determinant, no elliptic group arithmetic or secret scalars.
fn determinant(mut a: [[u64; 9]; 9]) -> u64 {
    let mut d = 1;
    for j in 0..9 {
        let pivot = (j..9).find(|&i| a[i][j] != 0);
        let Some(i) = pivot else { return 0; };
        if i != j {
            a.swap(j, i);
            d = fp_sub(0, d);
        }
        let pj = a[j][j];
        d = fp_mul(d, pj);
        let inv = fp_pow(pj, P - 2);
        for r in j + 1..9 {
            if a[r][j] == 0 { continue; }
            let scale = fp_mul(a[r][j], inv);
            for c in j + 1..9 {
                a[r][c] = fp_sub(a[r][c], fp_mul(scale, a[j][c]));
            }
            a[r][j] = 0;
        }
    }
    d
}

/// Return a false-zero risk explicitly, never interpret a diagonal determinant.
fn relation_determinant(leaves: &[Point; 8], residual: Point) -> Result<u64, String> {
    let mut seen = HashSet::new();
    for q in leaves.iter().copied().chain(std::iter::once(neg(residual))) {
        if q.is_none() || !seen.insert(q) {
            return Err("non-distinct or infinite evaluation divisor: Hermite jets required".into());
        }
    }
    let mut a = [[0u64; 9]; 9];
    for (i, &q) in leaves.iter().enumerate() { a[i] = evaluation_row(q)?; }
    a[8] = evaluation_row(neg(residual))?;
    Ok(determinant(a))
}

/// Order-one jet along the elliptic curve in local parameter x-x(P).
/// Valid at affine P with y != 0: dy/dx = (3x^2+a)/(2y).
fn evaluation_tangent_jet(q: Point) -> Result<[u64; 9], String> {
    let (x, y) = q.ok_or("infinity has no finite local x parameter")?;
    if x >= P || y >= P || y == 0 {
        return Err("noncanonical point or local x not uniformizer at y=0".into());
    }
    let x2 = fp_mul(x, x);
    let x3 = fp_mul(x2, x);
    let dy = fp_mul(
        fp_add(fp_mul(3, x2), PRIME31.a),
        fp_pow(fp_mul(2, y), P - 2),
    );
    Ok([
        0, 1, dy,
        fp_mul(2, x),
        fp_add(y, fp_mul(x, dy)),
        fp_mul(3, x2),
        fp_add(fp_mul(fp_mul(2, x), y), fp_mul(x2, dy)),
        fp_mul(4, x3),
        fp_add(fp_mul(fp_mul(3, x2), y), fp_mul(x3, dy)),
    ])
}

/// Confluent evaluation matrix for at most two occurrences of each signed
/// point. Each pair yields value row followed by first tangent jet.
fn relation_determinant_hermite(
    leaves: &[Point; 8], residual: Point,
) -> Result<u64, String> {
    let mut a = [[0u64; 9]; 9];
    let mut seen: HashMap<Point, u8> = HashMap::new();
    for (i, q) in leaves.iter().copied()
        .chain(std::iter::once(neg(residual))).enumerate()
    {
        if q.is_none() { return Err("effective divisor includes infinity".into()); }
        let entry = seen.entry(q).or_insert(0);
        *entry += 1;
        a[i] = match *entry {
            1 => evaluation_row(q)?,
            2 => evaluation_tangent_jet(q)?,
            _ => return Err("third occurrence: higher elliptic Hermite jet required".into()),
        };
    }
    Ok(determinant(a))
}

fn first_rational_points(ar: &mut Arithmetic) -> Vec<(u64, u64)> {
    let mut out = Vec::with_capacity(31);
    for x in 0..PRIME31.p {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            if y == 0 { continue; }
            out.push((x, y.min(PRIME31.p - y)));
            if out.len() == 31 { break; }
        }
    }
    assert_eq!(out.len(), 31);
    out
}

fn group_sum(ar: &mut Arithmetic, pts: &[Point; 8]) -> Point {
    let mut acc: Point = None;
    for &q in pts { acc = ar.add(PRIME31, acc, q); }
    acc
}

/// Generate pairwise-distinct signed leaves without consulting any discrete-log
/// label. The public residual is constructed outside the determinant oracle.
fn panel(ar: &mut Arithmetic, base: &[(u64, u64)], label: &str) -> Value {
    assert_eq!(base.len(), 31);
    let mut seed = SEED ^ if label.starts_with("dickson") { 0x1234_5678 } else { 0 };
    let mut attempts = 0usize;
    let mut pos_ns = 0u128;
    let mut neg_ns = 0u128;
    let mut records = Vec::new();
    let base_additions_before = ar.counts.group_additions;

    while records.len() < NCASES {
        attempts += 1;
        assert!(attempts <= CAP, "failed deterministic admissible trial cap");
        let mut choices: Vec<usize> = (0..31).collect();
        let mut leaves: [Point; 8] = [None; 8];
        for i in 0..8 {
            let j = i + (splitmix64(&mut seed) as usize % (31 - i));
            choices.swap(i, j);
            let q = Some(base[choices[i]]);
            leaves[i] = if splitmix64(&mut seed) & 1 == 1 { neg(q) } else { q };
        }
        let r = group_sum(ar, &leaves);
        let not_r = ar.add(PRIME31, r, Some(PRIME31.g));
        // Require two NONDEGENERATE full nine-point divisors. Rejections are
        // counted and happen before looking at determinant values.
        if r.is_none() || not_r.is_none() { continue; }
        let unique_pos = {
            let mut s = HashSet::new();
            leaves.iter().copied().chain(std::iter::once(neg(r))).all(|q| s.insert(q))
        };
        let unique_neg = {
            let mut s = HashSet::new();
            leaves.iter().copied().chain(std::iter::once(neg(not_r))).all(|q| s.insert(q))
        };
        if !unique_pos || !unique_neg { continue; }

        let t = Instant::now();
        let d_yes = relation_determinant(&leaves, r).expect("prechecked distinct positives");
        pos_ns += t.elapsed().as_nanos();
        let t = Instant::now();
        let d_no = relation_determinant(&leaves, not_r).expect("prechecked distinct negatives");
        neg_ns += t.elapsed().as_nanos();

        assert_eq!(d_yes, 0, "elliptic RR9 determinant misses a positive");
        assert_ne!(d_no, 0, "elliptic RR9 determinant false positive on distinct nine points");
        assert_ne!(r, not_r, "G is unexpectedly identity");
        records.push(json!({
            "case":records.len(), "eight_signed_leaves":leaves, "sum_residual":r,
            "negative_residual_plus_G":not_r, "positive_det":d_yes,
            "negative_det":d_no, "no_repeated_or_infinite_points":true,
            "original_curve_sum_witness":true
        }));
    }

    // Intentionally violate pairwise distinctness to falsify the naive
    // determinant, retaining a wrong target (correct sum plus nonzero G).
    let first = &records[0];
    let original: Vec<Point> = serde_json::from_value(first["eight_signed_leaves"].clone())
        .expect("serialize/deserialize eight public points");
    let mut repeated = [None; 8];
    repeated.copy_from_slice(&original);
    repeated[1] = repeated[0];
    let repeated_sum = group_sum(ar, &repeated);
    let wrong_target = ar.add(PRIME31, repeated_sum, Some(PRIME31.g));
    assert_ne!(wrong_target, repeated_sum);
    let mut naive = [[0u64; 9]; 9];
    for i in 0..8 { naive[i] = evaluation_row(repeated[i]).unwrap(); }
    // If wrong target happens to be infinity, use another nonzero offset.
    let wrong_target = if wrong_target.is_none() {
        {
            let double_g = ar.add(PRIME31, Some(PRIME31.g), Some(PRIME31.g));
            ar.add(PRIME31, repeated_sum, double_g)
        }
    } else { wrong_target };
    naive[8] = evaluation_row(neg(wrong_target)).unwrap();
    let false_zero = determinant(naive);
    assert_eq!(false_zero, 0, "duplicate matrix rows must cause a false zero");
    assert!(relation_determinant(&repeated, wrong_target).is_err(),
        "safe squarefree oracle must reject repeated signed points");
    // Repair the diagonal failure via the TRUE elliptic tangent jet rather
    // than ignoring the duplicated point or admitting the false positive.
    let hermite_true = relation_determinant_hermite(&repeated, repeated_sum)
        .expect("double divisor positive with x-local uniformizer");
    let hermite_false = relation_determinant_hermite(&repeated, wrong_target)
        .expect("double divisor negative with x-local uniformizer");
    assert_eq!(hermite_true, 0, "confluent RR9 misses a true repeated-leaf sum");
    assert_ne!(hermite_false, 0, "confluent RR9 admits a false repeated-leaf sum");

    json!({
        "base":label, "factor_base":base, "seed":format!("{SEED:#x}"),
        "accepted_positive":NCASES, "accepted_negative":NCASES,
        "draws_including_rejections":attempts, "positive_det_total_ns_diagnostic":pos_ns.to_string(),
        "negative_det_total_ns_diagnostic":neg_ns.to_string(),
        "source_group_additions_outside_determinant":ar.counts.group_additions-base_additions_before,
        "naive_repeated_row_false_zero":false_zero==0,
        "safe_repeated_row_rejected":true,
        "confluent_hermite_repeated_true_det":hermite_true,
        "confluent_hermite_repeated_false_det":hermite_false,
        "confluent_hermite_double_control":true,
        "no_unknown_ecdlp_solved":true, "cases":records
    })
}

fn run() -> Value {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("prime31 curve certificate");
    let b1 = first_rational_points(&mut ar);
    let b2 = Base::build(PRIME31, 6, &mut ar).expect("real Dickson31 depth-six base");
    assert_eq!(b2.points.len(), 31);
    let first = panel(&mut ar, &b1, "first31_rational_x");
    let dickson = panel(&mut ar, &b2.points, "dickson_depth6_31_rational_points");
    json!({
        "schema":"prime31.rr9.evaluation.v1", "p":PRIME31.p, "n":PRIME31.n,
        "curve":PRIME31, "method":"L(9O) evaluation determinant on nine distinct affine points",
        "positive_count":64, "negative_count":64,
        "panels":[first,dickson], "source_only_pcc":"unverified",
        "nonenumerative_factor_extraction":false,
        "complete_unknown_target_ecdlp_recoveries":0,
        "matched_optimized_rho":false
    })
}
fn main() { println!("{}", run()); }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rr9_pole_basis_and_mod_arithmetic() {
        assert_eq!(fp_add(P - 1, 1), 0);
        assert_eq!(fp_sub(0, 1), P - 1);
        assert_eq!(fp_mul(P - 1, P - 1), 1);
        assert_eq!(fp_pow(7, P - 2), 1_840_700_269); // checked against Fermat inverse
        let mut a = [[0u64; 9]; 9];
        for i in 0..9 { a[i][i] = 1; }
        assert_eq!(determinant(a), 1);
        a[2] = a[1];
        assert_eq!(determinant(a), 0);
    }
    #[test]
    fn rr9_original_prime31_bases_and_duplicate_control() {
        let data = run();
        assert_eq!(data["positive_count"], 64);
        assert_eq!(data["negative_count"], 64);
        for panel in data["panels"].as_array().unwrap() {
            assert_eq!(panel["accepted_positive"], 32);
            assert_eq!(panel["accepted_negative"], 32);
            assert_eq!(panel["naive_repeated_row_false_zero"], true);
            assert_eq!(panel["safe_repeated_row_rejected"], true);
            assert_eq!(panel["confluent_hermite_repeated_true_det"], 0);
            assert_ne!(panel["confluent_hermite_repeated_false_det"], 0);
            assert_eq!(panel["confluent_hermite_double_control"], true);
        }
    }
}
