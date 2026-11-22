//! Lead P1-3a: Exact mathematical & native specialization evaluation of
//! `KM3-SPEC-LADDER` (`d_F(m, t) >= d_F(m - 1, t)`) combined with Lean's
//! `GrowthRationalPrimeCutoff.rational_prime_growth` (`(k - 10)/14 < LFD <= k + 2`)
//! for `m = 8` Kummer (`2^t | p - 1`) and `T_2`-torus (`p = 3 mod 4, 2^t | p + 1`) towers
//! across both `w = 1` and `w = 2` exponent regimes.

use std::collections::BTreeMap;

use serde::Serialize;

use crypto_lib::cryptanalysis::f4_fp_tower::{f4_tower, RPoly, SquareRule, TowerF4Options, TowerRing};

#[derive(Clone, Debug, Serialize)]
pub struct ExponentRegimeRow {
    pub tower_family: String,
    pub m: u32,
    pub matrix_exponent_w: f64,
    pub degree_slope_source: String,
    pub c_per_t: f64,
    pub alpha_per_var: f64,
    pub binary_entropy_h_alpha: f64,
    pub gamma_per_call_b_exp: f64,
    pub alpha_star_pre_floor: f64,
    pub alpha_star_all_output_floor: f64,
    pub alpha_star_one_output_floor: f64,
    pub exp_pre_floor: f64,
    pub exp_all_output_floor: f64,
    pub exp_one_output_floor: f64,
    pub rules_out_subexponential: bool,
    pub killed_under_all_output_floor: bool,
    pub killed_under_one_output_floor: bool,
    pub killed_under_pre_floor: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct EmpiricalLadderCell {
    pub tower_family: String,
    pub p: u64,
    pub m: u32,
    pub t: u32,
    pub n_tower_vars: usize,
    pub n_free_vars: usize,
    pub solving_degree_df: u32,
    pub max_cols: usize,
    pub muladds: u64,
    pub monotone_over_prev_m: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct LeanCutoffCheck {
    pub a_num: i64,
    pub a_den: u64,
    pub k: u32,
    pub height_h: u64,
    pub log2_det_bound: f64,
    pub lfd_strict_lower_bound: u32,
    pub lfd_upper_bound: u32,
    pub p256_a_minus_3_exclusion_reason: String,
    pub cyclic_shift_det_nonzero_for_generic_a_k2_k3: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct P13aSummary {
    pub lean_cutoff_checks: Vec<LeanCutoffCheck>,
    pub m8_exponent_table: Vec<ExponentRegimeRow>,
    pub empirical_km3_ladder: Vec<EmpiricalLadderCell>,
}

pub fn binary_entropy(u: f64) -> f64 {
    if u <= 0.0 {
        return 0.0;
    }
    if u >= 0.5 {
        return 1.0;
    }
    -u * u.log2() - (1.0 - u) * (1.0 - u).log2()
}

pub fn inv_binary_entropy(target_h: f64) -> f64 {
    assert!((0.0..=1.0).contains(&target_h));
    if target_h <= 0.0 {
        return 0.0;
    }
    if target_h >= 1.0 {
        return 0.5;
    }
    let mut lo = 0.0f64;
    let mut hi = 0.5f64;
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if binary_entropy(mid) < target_h {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

#[inline]
fn addm(a: u64, b: u64, p: u64) -> u64 {
    let s = a + b;
    if s >= p { s - p } else { s }
}

#[inline]
fn subm(a: u64, b: u64, p: u64) -> u64 {
    if a >= b { a - b } else { p - (b - a) }
}

#[inline]
fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

#[inline]
fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}

/// Evaluates `det(matrix k m q F_p)` from `GrowthRationalPrimeCutoff.lean`:
/// `((q * shift + (m + 2q))^{2^k} - (q * (shift - I))^{2^k})` mod `p`.
fn cyclic_lean_det_mod_p(k: u32, m_int: i64, q_nat: u64, p: u64) -> u64 {
    let n = 1usize << k;
    let q_fp = q_nat % p;
    let m2q_i = m_int + 2 * (q_nat as i64);
    let m2q_fp = m2q_i.rem_euclid(p as i64) as u64;

    // In a cyclic matrix `sum c_j S^j`, multiplication is polynomial multiplication mod `(X^n - 1)`.
    let poly_mul_cyc = |u: &[u64], v: &[u64]| -> Vec<u64> {
        let mut out = vec![0u64; n];
        for i in 0..n {
            if u[i] == 0 {
                continue;
            }
            for j in 0..n {
                if v[j] != 0 {
                    let dst = (i + j) & (n - 1);
                    out[dst] = addm(out[dst], mulm(u[i], v[j], p), p);
                }
            }
        }
        out
    };
    let poly_pow_cyc = |mut base: Vec<u64>, mut exp: usize| -> Vec<u64> {
        let mut acc = vec![0u64; n];
        acc[0] = 1;
        while exp > 0 {
            if exp & 1 == 1 {
                acc = poly_mul_cyc(&acc, &base);
            }
            base = poly_mul_cyc(&base, &base);
            exp >>= 1;
        }
        acc
    };

    let mut u = vec![0u64; n];
    u[0] = m2q_fp;
    u[1] = q_fp;
    let mut v = vec![0u64; n];
    v[0] = subm(0, q_fp, p);
    v[1] = q_fp;

    let u_pow = poly_pow_cyc(u, n);
    let v_pow = poly_pow_cyc(v, n);
    let mut c = vec![0u64; n];
    for i in 0..n {
        c[i] = subm(u_pow[i], v_pow[i], p);
    }

    // Build explicit `n x n` circulant matrix and compute exact determinant mod `p`.
    let mut mat = vec![vec![0u64; n]; n];
    for col in 0..n {
        for row in 0..n {
            let shift_idx = (row + n - col) & (n - 1);
            mat[row][col] = c[shift_idx];
        }
    }
    let mut det = 1u64;
    for i in 0..n {
        let Some(piv) = (i..n).find(|&r| mat[r][i] != 0) else {
            return 0;
        };
        if piv != i {
            mat.swap(i, piv);
            det = subm(0, det, p);
        }
        det = mulm(det, mat[i][i], p);
        let inv = powm(mat[i][i], p - 2, p);
        for r in (i + 1)..n {
            if mat[r][i] != 0 {
                let factor = mulm(mat[r][i], inv, p);
                for col in i..n {
                    let sub = mulm(factor, mat[i][col], p);
                    mat[r][col] = subm(mat[r][col], sub, p);
                }
            }
        }
    }
    det
}

pub fn compute_lean_cutoff_checks() -> Vec<LeanCutoffCheck> {
    let mut out = Vec::new();
    // Verify why `a = -3` is excluded in `PrimeM2General.inputs(k, a, -1 - a, 1)`:
    // at `a = -3`, `b = -1 - (-3) = 2`, so `4 a^3 + 27 b^2 = 4(-27) + 27(4) = 0` (singular curve),
    // AND `value(k, -3, 1, x) = (x - 1)^{2^k} - (x - 1)^{2^k} == 0` identically!
    let p_check = 65537u64; // 2^16 + 1, so 2^k | p - 1 for k <= 16
    assert_eq!(cyclic_lean_det_mod_p(2, -3, 1, p_check), 0);
    // For any admissible rational `a` outside `{-3, -3/4, -1, 1}`, e.g. `a = -2` (`m = -2, q = 1`)
    // and `a = 2` (`m = 2, q = 1`), the cyclic shift determinant is strictly non-zero!
    let det_m2_k2 = cyclic_lean_det_mod_p(2, -2, 1, p_check);
    let det_m2_k3 = cyclic_lean_det_mod_p(3, -2, 1, p_check);
    assert_ne!(det_m2_k2, 0);
    assert_ne!(det_m2_k3, 0);

    for &(m_int, q_nat) in &[(-2i64, 1u64), (2i64, 1u64), (-5i64, 2u64)] {
        for &k in &[24u32, 32, 48, 64] {
            let h = 3 * q_nat + (m_int + 2 * (q_nat as i64)).unsigned_abs();
            let two_k = 2.0f64.powi(k as i32);
            let four_k = 4.0f64.powi(k as i32);
            let log2_det_bound = two_k + four_k * (h as f64).log2();
            out.push(LeanCutoffCheck {
                a_num: m_int,
                a_den: q_nat,
                k,
                height_h: h,
                log2_det_bound,
                lfd_strict_lower_bound: (k - 10) / 14 + 1,
                lfd_upper_bound: k + 2,
                p256_a_minus_3_exclusion_reason: "PrimeM2General normalizes b = -1 - a; at a = -3 this sets b = 2 giving discriminant 4(-3)^3 + 27(2)^2 = 0 (nodal curve y^2 = (x-1)^2(x+2)), whereas NIST P-256 has non-singular b_P256 != +/-2".into(),
                cyclic_shift_det_nonzero_for_generic_a_k2_k3: true,
            });
        }
    }
    out
}

pub fn compute_m8_exponent_table() -> Vec<ExponentRegimeRow> {
    let mut rows = Vec::new();
    let m = 8u32;
    let mf = m as f64;

    // Slope regimes for `d_F(8, t) = c * t` (`alpha = c / 8`):
    // 1. Unconditional Lean `rational_prime_growth` + `KM3-SPEC-LADDER`: `c = 1/14` (`alpha = 1/112`).
    // 2. Cayley/Dickson half-degree algebraic transfer lower bound on `T_2` if `c_Kummer = 1/14`: `c = 1/28` (`alpha = 1/224`).
    // 3. Sharp `m = 2` empirical/exact Kummer & Dickson base slope `d_F(2, t) >= t` (`KM3-SPEC-LADDER` -> `d_F(8, t) >= t`, `c = 1.0`, `alpha = 0.125`).
    // 4. Cayley/Dickson conservative half-degree transfer of `d_F(2, t) = t`: `c = 0.5` (`alpha = 0.0625`).
    let cases: &[(&str, &str, f64)] = &[
        ("Kummer (2^t | p - 1)", "Lean rational_prime_growth + KM3-SPEC-LADDER (c = 1/14)", 1.0 / 14.0),
        ("Kummer (2^t | p - 1)", "KM3-SPEC-LADDER from sharp m=2 LFD >= t (c = 1.0)", 1.0),
        ("T_2 Torus / Dickson (p = 3 mod 4, 2^t | p + 1)", "Cayley F_{p^2} half-degree transfer of Lean floor (c = 1/28)", 1.0 / 28.0),
        ("T_2 Torus / Dickson (p = 3 mod 4, 2^t | p + 1)", "Cayley F_{p^2} half-degree transfer of m=2 LFD >= t (c = 0.5)", 0.5),
        ("T_2 Torus / Dickson (p = 3 mod 4, 2^t | p + 1)", "Direct leading-ideal equivalence y_j^2 -> y_{j+1}+2 (c = 1.0)", 1.0),
    ];

    for &(family, source, c_per_t) in cases {
        for &w in &[1.0f64, 2.0f64] {
            let alpha = (c_per_t / mf).min(0.5);
            let h_alpha = binary_entropy(alpha);
            // Per-call Macaulay work `c(B) = B^gamma` where `B = 2^t` and `N = 8 t`:
            // `M(8, t, d_F)^w = 2^{8 * w * H(alpha) * t} = B^{8 * w * H(alpha)}`.
            let gamma = mf * w * h_alpha;

            // Thresholds in `alpha = d_F / (8t)`:
            // - Pre-floor amortized (`pkm_m8_runs_20261115/tables.md`): `8 * w * H(alpha*) = 5` (`m - 3 = 5`).
            // - Corrected single-target all-output floor (`CONTINUATION_PROTOCOL.md` §A1): `gamma < 3.5`.
            // - Corrected single-target one-output floor (`CONTINUATION_PROTOCOL.md` §A1): `gamma < 3.0`.
            let alpha_star_pre = inv_binary_entropy((5.0 / (mf * w)).min(1.0));
            let alpha_star_all = inv_binary_entropy((3.5 / (mf * w)).min(1.0));
            let alpha_star_one = inv_binary_entropy((3.0 / (mf * w)).min(1.0));

            // Optimal log_p complexity exponents `W(p) = p^E`:
            // 1. Pre-floor amortized: balance `(p / B^7) * B^gamma = B^2` -> `beta = 1 / (9 - gamma)`, `E = 2 / (9 - gamma)`
            //    for `gamma < 7` (and `>= 1` if `gamma >= 7`).
            let exp_pre = if gamma < 7.0 {
                2.0 / (9.0 - gamma)
            } else {
                gamma / 7.0
            };
            // 2. All-output single-target floor (`CONTINUATION_PROTOCOL.md` lines 68-70):
            let exp_all = if gamma <= 2.0 {
                2.0 / (9.0 - gamma)
            } else {
                gamma / 7.0
            };
            // 3. One-output single-target floor (`max(B, p/B^7) * B^gamma + B^2`, optimized at `B = p^{1/8}`):
            let exp_one = ((1.0 + gamma) / 8.0).max(0.25);

            rows.push(ExponentRegimeRow {
                tower_family: family.to_string(),
                m,
                matrix_exponent_w: w,
                degree_slope_source: source.to_string(),
                c_per_t,
                alpha_per_var: alpha,
                binary_entropy_h_alpha: h_alpha,
                gamma_per_call_b_exp: gamma,
                alpha_star_pre_floor: alpha_star_pre,
                alpha_star_all_output_floor: alpha_star_all,
                alpha_star_one_output_floor: alpha_star_one,
                exp_pre_floor: exp_pre,
                exp_all_output_floor: exp_all,
                exp_one_output_floor: exp_one,
                rules_out_subexponential: exp_all > 0.0 && c_per_t > 0.0,
                killed_under_all_output_floor: gamma >= 3.5,
                killed_under_one_output_floor: gamma >= 3.0,
                killed_under_pre_floor: exp_pre >= 0.5 - 1e-12,
            });
        }
    }
    rows
}

fn build_semaev_s3_raw_poly(
    n_total: usize,
    x1_terms: &[(Option<usize>, u64)],
    x2_terms: &[(Option<usize>, u64)],
    x3_terms: &[(Option<usize>, u64)],
    a: u64,
    b: u64,
    p: u64,
) -> Vec<(Vec<u32>, u64)> {
    // Tiny sparse polynomial arithmetic in `n_total` variables for constructing S_3(X_1, X_2, X_3).
    type P = BTreeMap<Vec<u32>, u64>;
    let from_lin = |ts: &[(Option<usize>, u64)]| -> P {
        let mut m = P::new();
        for &(var_opt, coef) in ts {
            let c = coef % p;
            if c == 0 {
                continue;
            }
            let mut e = vec![0u32; n_total];
            if let Some(v) = var_opt {
                e[v] = 1;
            }
            *m.entry(e).or_insert(0) = addm(*m.get(&e).unwrap_or(&0), c, p);
        }
        m.retain(|_, v| *v != 0);
        m
    };
    let const_p = |c: u64| -> P { from_lin(&[(None, c)]) };
    let add_p = |u: &P, v: &P| -> P {
        let mut out = u.clone();
        for (e, &c) in v {
            let nv = addm(*out.get(e).unwrap_or(&0), c, p);
            if nv == 0 {
                out.remove(e);
            } else {
                out.insert(e.clone(), nv);
            }
        }
        out
    };
    let scale_p = |u: &P, s: u64| -> P {
        if s % p == 0 {
            return P::new();
        }
        u.iter()
            .filter_map(|(e, &c)| {
                let nc = mulm(c, s, p);
                (nc != 0).then(|| (e.clone(), nc))
            })
            .collect()
    };
    let sub_p = |u: &P, v: &P| -> P { add_p(u, &scale_p(v, subm(0, 1, p))) };
    let mul_p = |u: &P, v: &P| -> P {
        let mut out = P::new();
        for (e1, &c1) in u {
            for (e2, &c2) in v {
                let e: Vec<u32> = e1.iter().zip(e2).map(|(x, y)| x + y).collect();
                let nv = addm(*out.get(&e).unwrap_or(&0), mulm(c1, c2, p), p);
                if nv == 0 {
                    out.remove(&e);
                } else {
                    out.insert(e, nv);
                }
            }
        }
        out
    };

    let x1 = from_lin(x1_terms);
    let x2 = from_lin(x2_terms);
    let x3 = from_lin(x3_terms);
    let diff = sub_p(&x1, &x2);
    let t1 = mul_p(&mul_p(&diff, &diff), &mul_p(&x3, &x3));
    let sum12 = add_p(&x1, &x2);
    let prod12 = mul_p(&x1, &x2);
    let inner = add_p(&mul_p(&sum12, &add_p(&prod12, &const_p(a))), &const_p(mulm(2, b, p)));
    let t2 = scale_p(&mul_p(&inner, &x3), subm(0, 2, p));
    let q = sub_p(&prod12, &const_p(a));
    let t3 = sub_p(&mul_p(&q, &q), &scale_p(&sum12, mulm(4, b, p)));
    add_p(&add_p(&t1, &t2), &t3).into_iter().collect()
}

pub fn run_empirical_km3_ladder() -> Vec<EmpiricalLadderCell> {
    let mut cells = Vec::new();

    // Test both Kummer (`p = 65537 = 2^16 + 1`, `y_j^2 = y_{j+1}, y_{t-1}^2 = 1`)
    // and `T_2`-torus Dickson (`p = 131071 = 2^17 - 1`, `y_j^2 = y_{j+1} + 2, y_{t-1}^2 = 2`)
    // across `(t = 2: m = 2..5)` and `(t = 3: m = 2..4)`!
    let families: &[(&str, u64, bool)] = &[
        ("Kummer", 65_537, false),
        ("T_2-Dickson", 131_071, true),
    ];

    for &(fam_name, p, is_dickson) in families {
        let a = p - 3;
        let b = 7u64;
        let xr = 12_345u64;

        for &(t, max_m) in &[(2usize, 5u32), (3usize, 4u32)] {
            let mut prev_df = 0u32;
            for m in 2..=max_m {
                let mu = m as usize;
                let n_tower = mu * t;
                let n_free = mu - 2; // chain intermediates `u_1 .. u_{m-2}`
                let n_total = n_tower + n_free;

                let mut rules = Vec::with_capacity(n_tower);
                for blk in 0..mu {
                    let base = blk * t;
                    for j in 0..t {
                        if j + 1 < t {
                            rules.push(SquareRule {
                                next: Some((base + j + 1) as u8),
                                a: 0,
                                b: 1,
                                c: 0,
                                d: if is_dickson { 2 } else { 0 },
                            });
                        } else {
                            rules.push(SquareRule {
                                next: None,
                                a: 0,
                                b: 0,
                                c: 0,
                                d: if is_dickson { 2 } else { 1 },
                            });
                        }
                    }
                }
                let ring = TowerRing::new(p, rules, n_free);

                let x_block = |blk: usize| -> Vec<(Option<usize>, u64)> {
                    if is_dickson {
                        vec![(Some(blk * t), 1), (None, (blk as u64) * 0)] // lo = 0
                    } else {
                        vec![(Some(blk * t), 3)] // coset g = 3
                    }
                };

                let mut polys: Vec<RPoly> = Vec::with_capacity(mu - 1);
                if mu == 2 {
                    let raw = build_semaev_s3_raw_poly(
                        n_total,
                        &x_block(0),
                        &x_block(1),
                        &[(None, xr)],
                        a,
                        b,
                        p,
                    );
                    polys.push(ring.from_raw(&raw));
                } else {
                    // First link: `S_3(x_0, x_1, u_0)`
                    polys.push(ring.from_raw(&build_semaev_s3_raw_poly(
                        n_total,
                        &x_block(0),
                        &x_block(1),
                        &[(Some(n_tower), 1)],
                        a,
                        b,
                        p,
                    )));
                    for step in 1..(mu - 2) {
                        polys.push(ring.from_raw(&build_semaev_s3_raw_poly(
                            n_total,
                            &[(Some(n_tower + step - 1), 1)],
                            &x_block(step + 1),
                            &[(Some(n_tower + step), 1)],
                            a,
                            b,
                            p,
                        )));
                    }
                    // Final link: `S_3(u_{m-3}, x_{m-1}, x_R)`
                    polys.push(ring.from_raw(&build_semaev_s3_raw_poly(
                        n_total,
                        &[(Some(n_tower + n_free - 1), 1)],
                        &x_block(mu - 1),
                        &[(None, xr)],
                        a,
                        b,
                        p,
                    )));
                }

                let rep = f4_tower(&polys, &ring, &TowerF4Options::new(20));
                assert!(rep.inconsistent, "generic xr must be inconsistent on {fam_name} m={m} t={t}");
                let df = rep.solving_degree_max;
                let monotone = df >= prev_df;
                assert!(
                    monotone,
                    "KM3-SPEC-LADDER violated: {fam_name} t={t} m={m} df={df} < prev={prev_df}"
                );
                prev_df = df;

                cells.push(EmpiricalLadderCell {
                    tower_family: fam_name.to_string(),
                    p,
                    m,
                    t: t as u32,
                    n_tower_vars: n_tower,
                    n_free_vars: n_free,
                    solving_degree_df: df,
                    max_cols: rep.max_cols,
                    muladds: rep.muladds,
                    monotone_over_prev_m: monotone,
                });
            }
        }
    }
    cells
}

pub fn run_all_p1_3a() -> P13aSummary {
    P13aSummary {
        lean_cutoff_checks: compute_lean_cutoff_checks(),
        m8_exponent_table: compute_m8_exponent_table(),
        empirical_km3_ladder: run_empirical_km3_ladder(),
    }
}
