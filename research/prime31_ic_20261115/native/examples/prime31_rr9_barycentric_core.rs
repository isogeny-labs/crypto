//! B8 public-R RR9 signed-fibre quotient: exact barycentric elimination.
//! Child of the original native prime31_rr9_public_macaulay.rs example.
//! Signed masks are EXPLICITLY enumerated (256): this is a generic-equivalent
//! completeness/control oracle, never a non-enumerative index calculus advance.
use super::{
    add, sub, mul, pow, factor_polynomial, independent_direct_m, first_eight,
    check_public_coefficients, P, NV, PRIME31, Arithmetic, Point,
};
use serde_json::{json, Value};
use std::time::Instant;

fn eval_cubic(a: &[u64; 4], x: u64) -> u64 {
    let mut out = a[3];
    for i in (0..3).rev() { out = add(mul(out, x), a[i]); }
    out
}

// Newton divided differences on the first four distinct public abscissas,
// converted into the ascending power basis. Independently check all eight.
fn interpolate_a(xs: &[u64; 8], c: &[u64; 8], v: &[u64; 4]) -> [u64; 4] {
    let mut dd = [0u64; 4];
    for i in 0..4 {
        dd[i] = sub(0, mul(c[i], eval_cubic(v, xs[i])));
    }
    for j in 1..4 {
        for i in (j..4).rev() {
            dd[i] = mul(sub(dd[i], dd[i - 1]), pow(sub(xs[i], xs[i-j]), P - 2));
        }
    }
    let mut ans = [0u64; 4];
    let mut basis = [0u64; 4];
    basis[0] = 1;
    for j in 0..4 {
        for k in 0..4 { ans[k] = add(ans[k], mul(dd[j], basis[k])); }
        if j < 3 {
            let mut next = [0u64; 4];
            for k in 0..3 {
                next[k] = sub(next[k], mul(xs[j], basis[k]));
                next[k+1] = add(next[k+1], basis[k]);
            }
            basis = next;
        }
    }
    ans
}

fn solve_three_unknowns(mut rows: [[u64; 4]; 4]) -> Option<([u64; 3], usize)> {
    let mut pivot_row = 0usize;
    let mut pivot_of_column = [None; 3];
    for col in 0..3 {
        let Some(at) = (pivot_row..4).find(|&i| rows[i][col] != 0) else { continue; };
        rows.swap(at, pivot_row);
        let inv = pow(rows[pivot_row][col], P - 2);
        for j in col..4 { rows[pivot_row][j] = mul(rows[pivot_row][j], inv); }
        for i in 0..4 {
            if i == pivot_row { continue; }
            let t = rows[i][col];
            if t == 0 { continue; }
            for j in col..4 {
                rows[i][j] = sub(rows[i][j], mul(t, rows[pivot_row][j]));
            }
        }
        pivot_of_column[col] = Some(pivot_row);
        pivot_row += 1;
    }
    if rows.iter().any(|r| r[..3].iter().all(|&z| z == 0) && r[3] != 0) {
        return None;
    }
    let mut v = [0u64; 3];
    // Free coordinates are fixed to 0; this gives one valid chart whenever
    // the affine solution set is nonempty, including deficient-rank cases.
    for j in 0..3 {
        if let Some(i) = pivot_of_column[j] { v[j] = rows[i][3]; }
    }
    Some((v, pivot_row))
}

#[derive(Clone)]
struct Chart {
    mask: u16,
    coeff: [u64; NV],
    rank: usize,
}

/// The candidate accepts ONLY public R and public B8 points; it constructs
/// and enumerates every sign mask itself. No scalar, selected mask, or planted
/// coefficients enter the candidate. This still costs exponential support work.
fn extract_public_b8(r: Point, base: &[(u64,u64); 8]) -> Vec<Chart> {
    let (rx, ry) = r.expect("bounded finite B8 fixture");
    let xs = base.map(|p| p.0);
    assert!(base.iter().all(|&(x,y)| x != rx && y != 0),
        "exceptional x=r or order-two leaf requires separate branch");
    let h = factor_polynomial(&xs);
    let mut weight = [0u64; 8];
    let mut inverse_xr = [0u64; 8];
    let mut xp = [[1u64; 7]; 8];
    for i in 0..8 {
        let mut product = 1;
        for j in 0..8 {
            if j != i { product = mul(product, sub(xs[i], xs[j])); }
        }
        assert_ne!(product, 0, "factor-base abscissas must be distinct");
        weight[i] = pow(product, P - 2);
        inverse_xr[i] = pow(sub(xs[i], rx), P - 2);
        for k in 1..7 { xp[i][k] = mul(xp[i][k-1], xs[i]); }
    }
    let mut charts = Vec::new();
    for mask in 0..256u16 {
        let mut c = [0u64; 8];
        let mut equations = [[0u64; 4]; 4];
        for i in 0..8 {
            let leaf_y = if mask & (1 << i) == 0 { base[i].1 } else { sub(0, base[i].1) };
            c[i] = mul(add(ry, leaf_y), inverse_xr[i]);
            let wc = mul(weight[i], c[i]);
            for k in 0..4 {
                for j in 0..4 {
                    equations[k][j] = add(equations[k][j], mul(wc, xp[i][k+j]));
                }
            }
        }
        // Barycentric moment equations: sum_i w_i x_i^k c_i V(x_i)=0
        // for k=0..3. The monic x^3 term is moved to the RHS.
        for row in &mut equations { row[3] = sub(0, row[3]); }
        let Some((vsmall, rank)) = solve_three_unknowns(equations) else { continue; };
        let v = [vsmall[0], vsmall[1], vsmall[2], 1u64];
        let a = interpolate_a(&xs, &c, &v);
        // Independent exact polynomial and all-point checks, not just the
        // four moment identities used during elimination.
        if (0..8).any(|i| add(eval_cubic(&a, xs[i]),
                              mul(c[i], eval_cubic(&v, xs[i]))) != 0) {
            continue;
        }
        let coeff = [a[0], a[1], a[2], a[3], v[0], v[1], v[2]];
        assert_eq!(independent_direct_m(&coeff, rx, ry), h,
            "linear chart fails the original quadratic coefficient ideal");
        charts.push(Chart { mask, coeff, rank });
    }
    charts
}

pub(super) fn run() -> Value {
    let mut ar = Arithmetic::default();
    super::check_curve(PRIME31, &mut ar).expect("original curve");
    let base = first_eight(&mut ar);
    let xs = base.map(|p| p.0);
    assert_eq!(xs, [0,1,2,3,5,6,8,9]);
    let positive:Point = Some((1_911_579_686, 548_820_096));
    let negatives:[Point;2] = [
        Some((625_037_715, 2_100_627_251)),
        Some((1_923_055_775, 307_731_458))
    ];
    // Freeze the exact public fixtures; the independent sign/group-sum
    // oracle is used ONLY AFTER candidate extraction, never for selection.
    let (oracle_positive, oracle_negatives, masks) = super::fixtures(&mut ar, &base);
    assert_eq!(oracle_positive, positive);
    assert_eq!(oracle_negatives.as_slice(), &negatives);
    assert_eq!(masks, 256);

    let now = Instant::now();
    let pos = extract_public_b8(positive, &base);
    let pos_ns = now.elapsed().as_nanos().to_string();
    assert_eq!(pos.len(), 1, "frozen B8 residual has one normalized chart");
    assert_eq!(pos[0].rank, 3);
    assert!(check_public_coefficients(&mut ar, positive, &base, &pos[0].coeff),
        "independent original-curve signed group-sum replay failed");
    let mut negs = Vec::new();
    for r in negatives {
        let start = Instant::now();
        let solutions = extract_public_b8(r, &base);
        let ns = start.elapsed().as_nanos().to_string();
        assert!(solutions.is_empty(), "outside-support negative must have no B8 chart");
        negs.push(json!({"residual":r,"charts":solutions.len(),"search_wall_ns":ns}));
    }
    json!({
        "schema":"prime31.rr9.public-barycentric-b8.v1",
        "p":P,"n":PRIME31.n,"curve_a":PRIME31.a,"curve_b":PRIME31.b,
        "base_x":xs,"positive_residual":positive,
        "positive_chart_count":pos.len(),"positive_first_chart_coefficients":pos[0].coeff,
        "positive_mask_discovered_by_enumeration":pos[0].mask,
        "positive_four_by_three_system_rank":pos[0].rank,
        "positive_search_wall_ns":pos_ns,
        "positive_original_curve_replay":true,
        "negatives":negs,"candidate_inputs":"public R and public B8 points only",
        "sign_masks_enumerated_per_residual":256,
        "method_is_support_enumeration":true,
        "nonenumerative_extraction":false,
        "full_b31_unknown_ecdlp_solves":0,
        "pcc":"unverified"
    })
}
