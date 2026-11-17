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

/// Multiply truncated power series over Fp; coefficients, not derivatives.
fn series_product(a: &[u64], b: &[u64]) -> Vec<u64> {
    assert_eq!(a.len(), b.len());
    let mut out = vec![0u64; a.len()];
    for i in 0..a.len() {
        for j in 0..(a.len()-i) {
            out[i+j] = fp_add(out[i+j], fp_mul(a[i], b[j]));
        }
    }
    out
}

/// The coefficient of t^order in each L(9O) basis monomial, where
/// t=x-x(P), y(t)^2=(x(P)+t)^3+a(x(P)+t)+b.
/// Valid for any affine P with y(P) != 0 and 0 <= order <= 8.
fn evaluation_higher_jet(q: Point, order: usize) -> Result<[u64; 9], String> {
    if order >= 9 { return Err("RR9 has only nine rows".into()); }
    if order == 0 { return evaluation_row(q); }
    let (x, y) = q.ok_or("infinity needs an O-local parameter")?;
    if x >= P || y >= P || y == 0 {
        return Err("noncanonical point or x is not uniformizer at y=0".into());
    }
    let len = order+1;
    let mut one = vec![0u64; len]; one[0] = 1;
    let mut xs = vec![0u64; len]; xs[0] = x; xs[1] = 1;
    let mut ys = vec![0u64; len]; ys[0] = y;
    let x2 = fp_mul(x, x);
    let inv_two_y = fp_pow(fp_mul(2, y), P-2);
    for k in 1..len {
        let c = match k {
            1 => fp_add(fp_mul(3, x2), PRIME31.a),
            2 => fp_mul(3, x),
            3 => 1,
            _ => 0,
        };
        let mut cross = 0u64;
        for j in 1..k {
            cross = fp_add(cross, fp_mul(ys[j], ys[k-j]));
        }
        ys[k] = fp_mul(fp_sub(c, cross), inv_two_y);
    }
    let xx2 = series_product(&xs, &xs);
    let xx3 = series_product(&xx2, &xs);
    let xx4 = series_product(&xx3, &xs);
    let xy = series_product(&xs, &ys);
    let x2y = series_product(&xx2, &ys);
    let x3y = series_product(&xx3, &ys);
    let cols = [one, xs, ys, xx2, xy, xx3, x2y, xx4, x3y];
    Ok(std::array::from_fn(|i| cols[i][order]))
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

/// Confluent L(9O) matrix for all rational affine multiplicities <=9.
/// On this prime-order, odd-order curve there are no rational y=0 points:
/// such a point would be nontrivial rational 2-torsion.
/// Infinity remains a separate unsupported branch, not a false positive.
fn relation_determinant_hermite(
    leaves: &[Point; 8], residual: Point,
) -> Result<u64, String> {
    let mut a = [[0u64; 9]; 9];
    let mut seen: HashMap<Point, usize> = HashMap::new();
    for (i, q) in leaves.iter().copied()
        .chain(std::iter::once(neg(residual))).enumerate()
    {
        if q.is_none() { return Err("effective divisor includes infinity".into()); }
        let entry = seen.entry(q).or_insert(0);
        *entry += 1;
        let order = *entry - 1;
        a[i] = if order == 1 {
            evaluation_tangent_jet(q)?
        } else {
            evaluation_higher_jet(q, order)?
        };
    }
    Ok(determinant(a))
}

/// Compute the unique (projective) F=U(x)+y V(x) in L(9O) vanishing
/// on the GIVEN length-eight affine divisor, including Hermite multiplicity.
/// This is coefficient interpolation, NOT target-conditioned factor extraction.
fn rr9_leaves_kernel(leaves: &[Point; 8]) -> Result<[u64; 9], String> {
    let mut rows = [[0u64; 9]; 8];
    let mut seen: HashMap<Point, usize> = HashMap::new();
    for (i, &q) in leaves.iter().enumerate() {
        if q.is_none() { return Err("kernel divisor includes infinity".into()); }
        let order = seen.entry(q).or_insert(0);
        rows[i] = evaluation_higher_jet(q, *order)?;
        *order += 1;
    }
    let mut pivots = [usize::MAX; 8];
    let mut rank = 0usize;
    for col in 0..9 {
        if rank == 8 { break; }
        let Some(pivot) = (rank..8).find(|&i| rows[i][col] != 0) else {
            continue;
        };
        rows.swap(rank, pivot);
        let inv = fp_pow(rows[rank][col], P - 2);
        for j in col..9 { rows[rank][j] = fp_mul(rows[rank][j], inv); }
        for i in 0..8 {
            if i == rank { continue; }
            let scale = rows[i][col];
            for j in col..9 {
                rows[i][j] = fp_sub(rows[i][j], fp_mul(scale, rows[rank][j]));
            }
        }
        pivots[rank] = col;
        rank += 1;
    }
    if rank != 8 { return Err(format!("Riemann-Roch length-eight rank is {rank}, not 8")); }
    let free = (0..9).find(|j| !pivots.contains(j))
        .ok_or("missing free coefficient")?;
    let mut out = [0u64; 9];
    out[free] = 1;
    for i in 0..8 {
        out[pivots[i]] = fp_sub(0, rows[i][free]);
    }
    let mut again: HashMap<Point, usize> = HashMap::new();
    for &q in leaves {
        let order = again.entry(q).or_insert(0);
        let row = evaluation_higher_jet(q, *order)?;
        let v = (0..9).fold(0, |acc, i| fp_add(acc, fp_mul(row[i], out[i])));
        if v != 0 { return Err("interpolated kernel fails its supplied leaf jet".into()); }
        *order += 1;
    }
    Ok(out)
}

/// Small original-field polynomial convolution, not group arithmetic.
fn rr9_poly_mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    assert!(!a.is_empty() && !b.is_empty());
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        for (j, &bj) in b.iter().enumerate() {
            c[i+j] = fp_add(c[i+j], fp_mul(ai, bj));
        }
    }
    c
}
fn rr9_function_at(coeff: &[u64; 9], q: Point) -> Result<u64, String> {
    let row = evaluation_row(q)?;
    Ok((0..9).fold(0, |acc, i| fp_add(acc, fp_mul(row[i], coeff[i]))))
}

/// Norm under y -> -y: U(X)^2 - (X^3+aX+b)*V(X)^2.
fn rr9_norm(coeff: &[u64; 9]) -> [u64; 10] {
    let u = [coeff[0], coeff[1], coeff[3], coeff[5], coeff[7]];
    let v = [coeff[2], coeff[4], coeff[6], coeff[8]];
    let uu = rr9_poly_mul(&u, &u);
    let cc = [PRIME31.b, PRIME31.a, 0u64, 1u64];
    let cvv = rr9_poly_mul(&cc, &rr9_poly_mul(&v, &v));
    assert_eq!(uu.len(), 9);
    assert_eq!(cvv.len(), 10);
    std::array::from_fn(|i| fp_sub(*uu.get(i).unwrap_or(&0), cvv[i]))
}

/// Exact degree-nine coefficient identity, even for repeated leaves.
/// The signed F(-R) evaluation is separately checked: x-norm alone
/// deliberately forgets the orientation.
fn rr9_norm_certificate(
    leaves: &[Point; 8], residual: Point, wrong_residual: Point,
) -> Result<Value, String> {
    let coeff = rr9_leaves_kernel(leaves)?;
    let (rx, _) = residual.ok_or("norm certificate R=O needs infinity branch")?;
    let mut product = vec![1u64];
    for &q in leaves.iter().chain(std::iter::once(&residual)) {
        let (x, _) = q.ok_or("norm certificate has infinity among nine points")?;
        product = rr9_poly_mul(&product, &[fp_sub(0, x), 1]);
    }
    if product.len() != 10 { return Err("norm factor polynomial degree is not nine".into()); }
    let norm = rr9_norm(&coeff);
    let leading = fp_sub(0, fp_mul(coeff[8], coeff[8]));
    if leading == 0 { return Err("V_3=0; ninth zero is at infinity".into()); }
    if norm[9] != leading { return Err("norm polynomial leading coefficient mismatch".into()); }
    let factored: [u64; 10] = std::array::from_fn(|i| fp_mul(product[i], leading));
    if norm != factored {
        return Err("coefficient norm factorization fails supplied 9-point divisor".into());
    }
    if rr9_function_at(&coeff, neg(residual))? != 0 {
        return Err("norm x-root has wrong F(-R) orientation".into());
    }
    if rr9_function_at(&coeff, neg(wrong_residual))? == 0 {
        return Err("F unexpectedly vanishes at known wrong residual".into());
    }
    Ok(json!({
        "verified":true, "residual_x":rx,
        "kernel_coefficients_L9O":coeff,
        "norm_coefficients_ascending":norm,
        "factorization_leading_coefficient":leading,
        "signed_orientation_true":true, "signed_orientation_wrong":false
    }))
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
        let norm_certificate = rr9_norm_certificate(&leaves, r, not_r)
            .expect("RR9 original-curve norm certificate for 8 distinct leaves");
        records.push(json!({
            "case":records.len(), "eight_signed_leaves":leaves, "sum_residual":r,
            "negative_residual_plus_G":not_r, "positive_det":d_yes,
            "negative_det":d_no, "no_repeated_or_infinite_points":true,
            "original_curve_sum_witness":true,
            "norm_certificate":norm_certificate
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

    // Preregistered diagonal tests: third, fourth and eighth occurrence.
    // Construction with original EC arithmetic is *outside* the determinant.
    let mut higher_controls = Vec::new();
    for multiplicity in [3usize, 4, 8] {
        let mut repeated_leaves: [Point; 8] = [Some(base[1]); 8];
        for leaf in repeated_leaves.iter_mut().take(multiplicity) {
            *leaf = Some(base[0]);
        }
        let true_r = group_sum(ar, &repeated_leaves);
        let false_r = ar.add(PRIME31, true_r, Some(PRIME31.g));
        assert!(true_r.is_some() && false_r.is_some());
        assert_ne!(true_r, false_r);
        let true_det = relation_determinant_hermite(&repeated_leaves, true_r)
            .expect("positive high-multiplicity divisor");
        let false_det = relation_determinant_hermite(&repeated_leaves, false_r)
            .expect("negative high-multiplicity divisor");
        assert_eq!(true_det, 0, "higher jet rejects valid signed relation");
        assert_ne!(false_det, 0, "higher jet admits wrong public residual");
        let higher_norm = rr9_norm_certificate(&repeated_leaves, true_r, false_r)
            .expect("RR9 norm polynomial identity with repeated Hermite leaves");
        higher_controls.push(json!({
            "norm_certificate":higher_norm,
            "multiplicity":multiplicity,
            "leaves":repeated_leaves, "true_residual":true_r,
            "wrong_residual_plus_G":false_r,
            "true_det":true_det, "wrong_det":false_det,
            "curve_witness_outside_determinant":true
        }));
    }

    // Ninth-jet falsifier: nine copies of P cannot sum to infinity,
    // since P != O and the group has prime order not dividing nine.
    let all_one: [Point; 8] = [Some(base[0]); 8];
    let fake_residual = neg(Some(base[0]));
    let real_residual = group_sum(ar, &all_one);
    assert_ne!(real_residual, fake_residual);
    let ninth_jet_wrong_det = relation_determinant_hermite(&all_one, fake_residual)
        .expect("nine identical affine points require Taylor orders zero through eight");
    assert_ne!(ninth_jet_wrong_det, 0, "nine equal points falsely satisfy RR9");

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
        "confluent_hermite_higher_controls":higher_controls,
        "ninth_jet_wrong_det":ninth_jet_wrong_det,
        "ninth_jet_original_curve_negative":true,
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
    fn rr9_taylor_first_two_coefficients_match_independent_rows() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        let points = first_rational_points(&mut ar);
        for &point in points.iter().take(16) {
            let q = Some(point);
            assert_eq!(evaluation_higher_jet(q, 0).unwrap(), evaluation_row(q).unwrap());
            assert_eq!(evaluation_higher_jet(q, 1).unwrap(),
                       evaluation_tangent_jet(q).unwrap());
        }
        assert!(evaluation_higher_jet(Some(points[0]), 9).is_err());
    }
    #[test]
    fn rr9_original_prime31_bases_and_duplicate_control() {
        let data = run();
        assert_eq!(data["positive_count"], 64);
        assert_eq!(data["negative_count"], 64);
        for panel in data["panels"].as_array().unwrap() {
            assert_eq!(panel["accepted_positive"], 32);
            assert_eq!(panel["accepted_negative"], 32);
            for case in panel["cases"].as_array().unwrap() {
                assert_eq!(case["norm_certificate"]["verified"], true);
                assert_eq!(
                    case["norm_certificate"]["norm_coefficients_ascending"]
                        .as_array().unwrap().len(), 10);
            }
            assert_eq!(panel["naive_repeated_row_false_zero"], true);
            assert_eq!(panel["safe_repeated_row_rejected"], true);
            assert_eq!(panel["confluent_hermite_repeated_true_det"], 0);
            assert_ne!(panel["confluent_hermite_repeated_false_det"], 0);
            assert_eq!(panel["confluent_hermite_double_control"], true);
            let h = panel["confluent_hermite_higher_controls"].as_array().unwrap();
            assert_eq!(h.len(), 3);
            for (i, &m) in [3, 4, 8].iter().enumerate() {
                assert_eq!(h[i]["multiplicity"], m);
                assert_eq!(h[i]["true_det"], 0);
                assert_ne!(h[i]["wrong_det"], 0);
                assert_eq!(h[i]["norm_certificate"]["verified"], true);
            }
            assert_ne!(panel["ninth_jet_wrong_det"], 0);
            assert_eq!(panel["ninth_jet_original_curve_negative"], true);
        }
    }
}
