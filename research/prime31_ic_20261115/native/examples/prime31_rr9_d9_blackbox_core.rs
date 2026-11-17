//! Sparse matrix-free rank-certification attempt for the B8 RR9 public D9 ideal.
//! Standard Wiedemann/Berlekamp–Massey: one-sided FULL-RANK witness only.
//! Deliberately does NOT enumerate signed supports or solve a discrete log.
use super::*;
use std::mem;

type SparseRow = Vec<(usize, u64)>;

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn sparse_macaulay(r: u64, s: u64, h: &[u64; 9], degree: u8) -> (usize, Vec<SparseRow>) {
    assert!(degree >= 2);
    let qs = equations(r, s, h);
    let basis = monomials(degree);
    let multiplier_basis = monomials(degree - 2);
    let index: HashMap<Exp, usize> =
        basis.iter().copied().enumerate().map(|(i, e)| (e, i)).collect();
    let mut rows = Vec::with_capacity(8 * multiplier_basis.len());
    for q in &qs {
        for m in &multiplier_basis {
            let mut row = Vec::with_capacity(q.len());
            for (e, &v) in q {
                let col = *index.get(&add_exp(e, m)).expect("Macaulay degree overflow");
                if v != 0 { row.push((col, v)); }
            }
            row.sort_unstable_by_key(|x| x.0);
            assert!(row.windows(2).all(|pair| pair[0].0 < pair[1].0));
            rows.push(row);
        }
    }
    (basis.len(), rows)
}

// A = M^T D M, D nonsingular diagonal. In Fp a singular A does not imply
// a singular M (isotropic Gram matrices exist); the converse direction is safe.
fn gram_apply(rows: &[SparseRow], diag: &[u64], x: &[u64], y: &mut [u64]) {
    assert_eq!(rows.len(), diag.len());
    assert_eq!(x.len(), y.len());
    y.fill(0);
    for (row, &d) in rows.iter().zip(diag) {
        assert_ne!(d, 0);
        let mut z = 0;
        for &(i, coeff) in row { z = add(z, mul(coeff, x[i])); }
        let scaled = mul(d, z);
        for &(i, coeff) in row {
            y[i] = add(y[i], mul(coeff, scaled));
        }
    }
}

fn dot(x: &[u64], y: &[u64]) -> u64 {
    assert_eq!(x.len(), y.len());
    let mut total = 0;
    for (&a, &b) in x.iter().zip(y) { total = add(total, mul(a, b)); }
    total
}

// Classic scalar Berlekamp–Massey, C[0]=1:
// s_t + sum_{i=1}^L C[i] s_{t-i} = 0 for all tested t >= L.
fn berlekamp_massey(s: &[u64]) -> (usize, Vec<u64>) {
    let mut c = vec![1];
    let mut b = vec![1];
    let mut l = 0usize;
    let mut shift = 1usize;
    let mut prev_discrepancy = 1u64;
    for (t, &value) in s.iter().enumerate() {
        let mut discrepancy = value;
        for i in 1..=l {
            discrepancy = add(discrepancy, mul(c[i], s[t - i]));
        }
        if discrepancy == 0 { shift += 1; continue; }
        let old_c = c.clone();
        let scale = mul(discrepancy, pow(prev_discrepancy, P - 2));
        if c.len() < b.len() + shift { c.resize(b.len() + shift, 0); }
        for (j, &bj) in b.iter().enumerate() {
            c[j + shift] = sub(c[j + shift], mul(scale, bj));
        }
        if 2 * l <= t {
            l = t + 1 - l;
            b = old_c;
            prev_discrepancy = discrepancy;
            shift = 1;
        } else { shift += 1; }
    }
    c.resize(l + 1, 0);
    (l, c)
}

fn random_nonzero(seed: &mut u64) -> u64 { 1 + splitmix64(seed) % (P - 1) }

fn moments(rows: &[SparseRow], cols: usize, seed: u64, count: usize)
           -> (Vec<u64>, usize, u64) {
    let mut rng = seed;
    let diag: Vec<u64> = (0..rows.len()).map(|_| random_nonzero(&mut rng)).collect();
    let u: Vec<u64> = (0..cols).map(|_| random_nonzero(&mut rng)).collect();
    let mut current: Vec<u64> = (0..cols).map(|_| random_nonzero(&mut rng)).collect();
    let mut next = vec![0; cols];
    let nnz: usize = rows.iter().map(Vec::len).sum();
    let mut seq = Vec::with_capacity(count);
    for _ in 0..count {
        seq.push(dot(&u, &current));
        gram_apply(rows, &diag, &current, &mut next);
        mem::swap(&mut current, &mut next);
    }
    // Exactly the field multiplications in the displayed M^T D M + dot loop.
    let per_iteration = (2 * nnz + rows.len() + cols) as u64;
    (seq, nnz, per_iteration * count as u64)
}

fn full_rank_certified(seq: &[u64], n: usize) -> (bool, usize, u64) {
    assert_eq!(seq.len(), 2 * n);
    let (l, c) = berlekamp_massey(seq);
    let end = c[l];
    (l == n && end != 0, l, end)
}

pub fn smoke() -> Value {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("actual prime31 curve");
    let base = first_eight(&mut ar);
    let xs = base.map(|q| q.0);
    assert_eq!(xs, [0, 1, 2, 3, 5, 6, 8, 9]);
    let h = factor_polynomial(&xs);
    let (r, s) = PRIME31.g;
    let (n3, rows3) = sparse_macaulay(r, s, &h, 3);
    assert_eq!(n3, 120);
    assert_eq!(rows3.len(), 64);
    let basis = monomials(3);
    let mult_basis = monomials(1);
    let index: HashMap<Exp, usize> =
        basis.iter().copied().enumerate().map(|(i, e)| (e, i)).collect();
    let eqs = equations(r, s, &h);
    let diag: Vec<u64> = (0..rows3.len()).map(|i| i as u64 + 1).collect();
    let x: Vec<u64> = (0..n3).map(|i| ((i * 7919 + 47) as u64) % P).collect();
    let mut sparse_out = vec![0; n3];
    gram_apply(&rows3, &diag, &x, &mut sparse_out);
    let mut dense_out = vec![0; n3];
    for (qi, q) in eqs.iter().enumerate() {
        for (mi, m) in mult_basis.iter().enumerate() {
            let row = raw_row(q, m, &index, n3);
            let val = mul(diag[qi * mult_basis.len() + mi], dot(&row, &x));
            for (j, &coefficient) in row.iter().enumerate() {
                dense_out[j] = add(dense_out[j], mul(coefficient, val));
            }
        }
    }
    assert_eq!(sparse_out, dense_out, "D3 sparse operator differs from independent dense form");

    // A two-by-two full-rank diagonal A has a scalar sequence with degree 2
    // when projected nondegenerately; a singular A must not pass the gate.
    let invertible = vec![vec![(0usize, 1u64)], vec![(1usize, 1u64)]];
    let full_diag = vec![1, 2];
    let mut vv = vec![1, 1];
    let mut next = vec![0, 0];
    let mut seq = Vec::new();
    for _ in 0..4 {
        seq.push(dot(&[1, 1], &vv));
        gram_apply(&invertible, &full_diag, &vv, &mut next);
        mem::swap(&mut vv, &mut next);
    }
    let (full, l, _) = full_rank_certified(&seq, 2);
    assert!(full && l == 2);
    let singular = vec![vec![(0usize, 1u64)]];
    let mut vv = vec![1, 1];
    let mut seq = Vec::new();
    for _ in 0..4 {
        seq.push(dot(&[1, 1], &vv));
        gram_apply(&singular, &[1], &vv, &mut next);
        mem::swap(&mut vv, &mut next);
    }
    assert!(!full_rank_certified(&seq, 2).0);

    // Reuse the independent test-only positive D9 witness from #4550.
    // A positive has a kernel; no algorithm that certifies it full-rank is sound.
    let pos = projective_degree_nine_witness_controls();
    assert_eq!(pos["macaulay_rows_checked"], 27_456);
    assert_eq!(pos["all_D9_rows_annihilated"], true);

    let (n9, rows9) = sparse_macaulay(r, s, &h, 9);
    assert_eq!(n9, 11_440);
    assert_eq!(rows9.len(), 27_456);
    let (_, nnz9, _) = moments(&rows9, n9, 0xd99_0001, 2);
    json!({
        "schema": "prime31.rr9.d9-blackbox-smoke.v1",
        "curve_p": P, "order_n": PRIME31.n, "macaulay_d9_columns": n9,
        "macaulay_d9_rows": rows9.len(), "d9_nnz": nnz9,
        "d3_sparse_equals_independent_dense": true,
        "rank2_positive_gate": true, "singular_negative_gate": true,
        "original_positive_d9_kernel_test": true,
        "d9_smoke_matvecs": 2,
        "complete_unknown_ecdlp_solves": 0,
        "pcc": "unverified"
    })
}

pub fn run(case: usize) -> Value {
    assert!(case < 2, "frozen negative case indices 0 and 1 only");
    let started = Instant::now();
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("actual cofactor-one prime31 curve");
    let xs = first_eight(&mut ar).map(|q| q.0);
    assert_eq!(xs, [0, 1, 2, 3, 5, 6, 8, 9]);
    let h = factor_polynomial(&xs);
    let residuals = [PRIME31.g, (1_923_055_775, 307_731_458)];
    let (r, s) = residuals[case];
    assert!(ar.on_curve(PRIME31, Some((r, s))));
    assert!(!xs.contains(&r));
    let seed = 0x0d99_0001 + case as u64;
    let (cols, rows) = sparse_macaulay(r, s, &h, 9);
    assert_eq!(cols, 11_440);
    assert_eq!(rows.len(), 27_456);
    let start = Instant::now();
    let (seq, nnz, matvec_muls) = moments(&rows, cols, seed, 2 * cols);
    let moment_elapsed_ns = start.elapsed().as_nanos().to_string();
    let (certified, complexity, last) = full_rank_certified(&seq, cols);
    json!({
        "schema": "prime31.rr9.d9-blackbox-negative.v1",
        "p": P, "n": PRIME31.n, "a": PRIME31.a, "b": PRIME31.b,
        "base_x": xs, "public_residual": [r, s],
        "negative_index": case, "seed": seed, "degree": 9,
        "rows": rows.len(), "columns": cols, "nnz": nnz,
        "matrix_operator": "M^T D M over Fp; D nonzero",
        "krylov_moments": seq.len(), "gram_applications": seq.len(),
        "operator_field_multiplications": matvec_muls.to_string(),
        "scalar_bm_linear_complexity": complexity,
        "scalar_bm_last_coefficient": last,
        "one_sided_full_rank_certified": certified,
        "absence_over_algebraic_closure_only_if_certified": certified,
        "inconclusive_if_not_certified": !certified,
        "moment_elapsed_ns": moment_elapsed_ns,
        "all_charged_elapsed_ns": started.elapsed().as_nanos().to_string(),
        "no_sign_mask_or_secret_scalar_passed": true,
        "public_only_positive_chart_recovered": false,
        "complete_unknown_ecdlp_solves": 0,
        "pcc": "unverified"
    })
}
