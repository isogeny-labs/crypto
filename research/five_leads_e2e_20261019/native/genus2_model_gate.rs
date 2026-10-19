//! Check the exact Scholten quintics against the complete projective
//! genus-2 formula model. No projective addition is implemented here.
use five_leads_study::three_leads::{Divisor, Field, Jacobian, Point, Scholten};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;

fn choose(n: usize, k: usize) -> u64 {
    let mut x = 1;
    for i in 0..k {
        x = x * (n - i) as u64 / (i + 1) as u64;
    }
    x
}
fn transform(poly: &[u64], alpha: u64, beta: u64, scale: u64, f: &Field) -> Vec<u64> {
    let mut out = vec![0; poly.len()];
    let inv = f.inv(scale);
    for (k, &coefficient) in poly.iter().enumerate() {
        for (j, slot) in out.iter_mut().enumerate().take(k + 1) {
            let term = f.mul(
                coefficient,
                f.mul(
                    choose(k, j) % f.p,
                    f.mul(f.pow(alpha, j as u64), f.pow(beta, (k - j) as u64)),
                ),
            );
            *slot = f.add(*slot, f.mul(term, inv));
        }
    }
    while out.last() == Some(&0) {
        out.pop();
    }
    out
}
fn map_divisor(d: Divisor, alpha: u64, beta: u64, f: &Field) -> Divisor {
    if d.degree == 0 {
        return d;
    }
    let u = transform(
        &d.u[..=d.degree],
        alpha,
        beta,
        f.pow(alpha, d.degree as u64),
        f,
    );
    let v = transform(&d.v[..d.degree], alpha, beta, f.pow(alpha, 3), f);
    let mut result = Divisor {
        degree: d.degree,
        u: [0; 3],
        v: [0; 2],
    };
    result.u[..u.len()].copy_from_slice(&u);
    result.v[..v.len()].copy_from_slice(&v);
    result
}
fn main() {
    let mut seen = BTreeSet::new();
    for dir in ["genus-fixtures-v2", "genus-order-fixtures-v1"] {
        for bits in [16, 20, 24, 28, 32, 36, 40] {
            let path = format!(
                "research/three_leads_e2e_20261018/{dir}/{}genus-{bits}.json",
                if dir.starts_with("genus-order") {
                    "order-"
                } else {
                    ""
                }
            );
            let spec: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
            for row in spec["workloads"]["curves"].as_array().unwrap() {
                let p = row["p"].as_u64().unwrap();
                let key = (dir, bits, p);
                assert!(seen.insert(key));
                let s = Scholten::new(p);
                assert_eq!(s.short_curve().c[0], row["b"].as_u64().unwrap());
                assert_eq!(s.short_curve().c[1], row["a"].as_u64().unwrap());
                let f = Field::new(p, 1, 0);
                let old = &s.j.polynomial;
                let lead = old[5];
                assert_ne!(lead, 0);
                let shift = f.neg(f.div(old[4], f.mul(5, lead)));
                let normalized = transform(old, lead, shift, f.pow(lead, 6), &f);
                assert_eq!(normalized.len(), 6);
                assert_eq!(normalized[5], 1);
                assert_eq!(normalized[4], 0);
                let new = Jacobian {
                    f: f.fresh(),
                    polynomial: normalized.clone(),
                };
                let generator =
                    Point::affine(row["gx"].as_u64().unwrap(), row["gy"].as_u64().unwrap());
                assert!(s.short_curve().valid(generator));
                let generator_divisor = s.tau(s.from_short(generator));
                let mapped_generator = map_divisor(generator_divisor, lead, shift, &f);
                assert!(new.valid(mapped_generator));
                let mut checked = 0;
                let mut group_law_checked = 0;
                for k in 1..=100u64 {
                    let source = s.short_curve().mul(generator, k);
                    if source.inf {
                        continue;
                    }
                    let d = s.tau(s.from_short(source));
                    assert!(s.j.valid(d));
                    let mapped = map_divisor(d, lead, shift, &f);
                    assert!(new.valid(mapped), "invalid normalized divisor p={p} k={k}");
                    checked += 1;
                    let left = map_divisor(s.j.add(d, generator_divisor), lead, shift, &f);
                    let right = new.add(mapped, mapped_generator);
                    assert_eq!(left, right, "coordinate map not additive p={p} k={k}");
                    group_law_checked += 1;
                }
                println!(
                    "{}",
                    serde_json::to_string(&json!({
                        "schema":"genus2-projective-model-gate/v1","cohort":dir,"bits":bits,"p":p,
                        "original_polynomial":old,"normalized_polynomial":normalized,
                        "x_scale":lead,"x_shift":shift,"y_scale":f.pow(lead,3),
                        "sampled_transfer_divisors_valid":checked,
                        "sampled_group_law_pairs_valid":group_law_checked,
                        "formula_model_y2_monic_quintic_no_x4":true,
                        "projective_addition_implemented":false
                    }))
                    .unwrap()
                );
            }
        }
    }
    assert_eq!(seen.len(), 42);
}
