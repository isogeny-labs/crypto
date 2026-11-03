//! Exact Fp Rosenhain normalization gate; no Kummer coordinate formulas.
use serde_json::{json, Value};
use std::fs;

fn add(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    a * b % p
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut out = 1;
    while n > 0 {
        if n & 1 == 1 {
            out = mul(out, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    out
}
fn poly_mul_linear(poly: &[u64], c0: u64, c1: u64, p: u64) -> Vec<u64> {
    let mut out = vec![0; poly.len() + 1];
    for (i, &c) in poly.iter().enumerate() {
        out[i] = add(out[i], mul(c, c0, p), p);
        out[i + 1] = add(out[i + 1], mul(c, c1, p), p);
    }
    out
}
fn normalize(coeff: &[u64], r0: u64, delta: u64, p: u64) -> Vec<u64> {
    let mut out = vec![0];
    for &c in coeff.iter().rev() {
        out = poly_mul_linear(&out, r0, delta, p);
        out[0] = add(out[0], c, p);
    }
    while out.len() > 6 {
        assert_eq!(out.pop(), Some(0));
    }
    out
}
fn main() {
    let src = "research/four_followups_20261021/results/kummer-model-gate.jsonl";
    let dst = "research/kummer_walk_20261022/results/rosenhain-models.jsonl";
    let mut out = String::new();
    let mut count = 0;
    for line in fs::read_to_string(src).unwrap().lines() {
        let row: Value = serde_json::from_str(line).unwrap();
        let p = row["p"].as_u64().unwrap();
        let roots: Vec<u64> = row["finite_rational_weierstrass_roots"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect();
        let coeff: Vec<u64> = row["quintic_coefficients"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect();
        assert_eq!(roots.len(), 5);
        assert_eq!(coeff.len(), 6);
        assert_eq!(p % 4, 3);
        let mut selected = None;
        let mut square_pairs = 0;
        let mut pairs_tried = 0;
        for &r0 in &roots {
            for &r1 in &roots {
                if r0 == r1 {
                    continue;
                }
                pairs_tried += 1;
                let delta = sub(r1, r0, p);
                let scale = mul(coeff[5], pow(delta, 5, p), p);
                if scale == 0 || pow(scale, (p - 1) / 2, p) != 1 {
                    continue;
                }
                square_pairs += 1;
                if selected.is_some() {
                    continue;
                }
                let y_scale = pow(scale, (p + 1) / 4, p);
                assert_eq!(mul(y_scale, y_scale, p), scale);
                let inv_delta = pow(delta, p - 2, p);
                let transformed: Vec<u64> = roots
                    .iter()
                    .filter(|&&r| r != r0 && r != r1)
                    .map(|&r| mul(sub(r, r0, p), inv_delta, p))
                    .collect();
                let mut expected = vec![1];
                for &root in &[0, 1, transformed[0], transformed[1], transformed[2]] {
                    expected = poly_mul_linear(&expected, sub(0, root, p), 1, p);
                }
                let observed: Vec<u64> = normalize(&coeff, r0, delta, p)
                    .iter()
                    .map(|&v| mul(v, pow(scale, p - 2, p), p))
                    .collect();
                selected = Some(json!({"r0":r0,"r1":r1,"delta":delta,"scale":scale,
                    "sqrt_scale":y_scale,"lambda_mu_nu":transformed,
                    "observed_coefficients":observed,"reconstructed_coefficients":expected,
                    "coefficient_identity":observed==expected}));
            }
        }
        let all_correct = selected
            .as_ref()
            .is_some_and(|v| v["coefficient_identity"] == true);
        out.push_str(&format!(
            "{}\n",
            json!({"schema":"kummer-rosenhain-model/v1","p":p,
            "input_factorization_verified":row["factorization_verified"],
            "pairs_tried":pairs_tried,"square_pairs":square_pairs,
            "fp_rosenhain_map_found":selected.is_some(),"all_correct":all_correct,
            "selected":selected})
        ));
        count += 1;
    }
    assert_eq!(count, 21);
    fs::write(dst, out).unwrap();
}
