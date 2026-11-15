//! Development-only terminal pairing. No production LFD certificate.
#[allow(dead_code)]
#[path = "../prime_first_fall_donor.rs"]
mod donor;
#[path = "../prime_terminal_math.rs"]
mod math;
use math::*;
use serde_json::{json, Value};

fn functional(w: &[u128], t: u128, p: u128) -> (Value, Poly, Poly) {
    let n = w.len();
    assert!(n >= 3 && n <= 27 && n % 2 == 1 && p > n as u128 && t < p);
    assert!(w.iter().all(|&x| x > 0 && x < p));
    let h = h_poly(n, p);
    let a1 = multiply_x(&h, w, p);
    let a2 = multiply_x(&a1, w, p);
    let mut c = a2.clone();
    scaled_add(&mut c, &a1, neg(mul(2, t, p), p), p);
    scaled_add(&mut c, &h, mul(t, t, p), p);
    let sum = w.iter().fold(0, |s, &v| add(s, v, p));
    let delta = add(sum, neg(mul(2, t, p), p), p);
    let power = (0..n / 2).fold(1, |s, _| mul(s, p - 2, p));
    let actual = pairing(&h, &c, n, p);
    let expected = mul(power, delta, p);
    assert_eq!(
        actual, expected,
        "direct coefficient pairing versus formula"
    );
    assert_eq!(pairing(&h, &a1, n, p), power);
    assert_eq!(pairing(&h, &a2, n, p), mul(power, sum, p));
    (
        json!({"schema":"n83.terminal-functional-development/v1",
          "ell":n,"field":p.to_string(),"target_x":t.to_string(),
          "weights":w.iter().map(|x|x.to_string()).collect::<Vec<_>>(),
          "delta":delta.to_string(),"pairing":actual.to_string(),
          "formula":expected.to_string(),"pairing_nonzero":actual!=0,
          "h_terms":h.len(),"u2_coefficient_terms":c.len(),
          "coefficient_sha256":sha(&bytes(&c,n,p)),
          "source_bound":false,"filtered_closure_computed":false,
          "strict_filtered_fall_verified":false,"last_fall_lower_bound":null,
          "claim":"Synthetic development fixture; functional arithmetic only"}),
        h,
        c,
    )
}

fn main() {
    eprintln!("Development only: use cargo test --bin prime_terminal_functional; no production entry point");
    std::process::exit(2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path};

    fn retain(name: &str, v: &Value) {
        if let Ok(dir) = std::env::var("N83_FUNCTIONAL_DEV_OUT") {
            let path = Path::new(&dir).join(name);
            assert!(!path.exists(), "never overwrite development records");
            fs::write(path, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap();
        }
    }

    #[test]
    fn small_pairings_independent_product_and_exception() {
        let mut rows = vec![];
        for p in [101u128, 1009] {
            for n in [3usize, 5, 7] {
                let w: Vec<_> = (1..=n).map(|i| (3 * i + 1) as u128).collect();
                let sum = w.iter().fold(0, |s, &v| add(s, v, p));
                let exception = mul(sum, (p + 1) / 2, p);
                for t in [add(exception, 1, p), exception] {
                    let (v, h, c) = functional(&w, t, p);
                    let expanded = product(&h, &c, &vec![0; n], p);
                    let independent = *expanded.get(&((1u64 << n) - 1)).unwrap_or(&0);
                    assert_eq!(v["pairing"], independent.to_string());
                    assert_eq!(independent == 0, t == exception);
                    rows.push(v);
                }
            }
        }
        retain("small-pairings.json", &json!({"cases":rows}));
    }

    #[test]
    fn terminal_top_image_annihilation_all_small_multipliers() {
        let mut rows = vec![];
        for n in [3usize, 5, 7] {
            let p = 101;
            let h = h_poly(n, p);
            let x = linear(n);
            let zero = vec![0; n];
            let x2 = product(&x, &x, &zero, p);
            let mut count = 0;
            for mask in 0..(1u64 << n) {
                if mask.count_ones() as usize != n / 2 - 1 {
                    continue;
                }
                let q: Poly = [(mask, 1)].into();
                let f = product(&x2, &q, &zero, p);
                assert_eq!(pairing(&h, &f, n, p), 0);
                count += 1;
            }
            assert!(count > 0);
            rows.push(json!({"ell":n,"multipliers_checked":count,"all_pairings_zero":true}));
        }
        retain(
            "top-image-small.json",
            &json!({"cases":rows,
            "scope":"last-block multiplier enumeration only; not complete filtered closure"}),
        );
    }

    #[test]
    fn complementary_coefficient_mutation_is_detected() {
        let p = 101;
        let (_, h, mut c) = functional(&[1, 2, 4, 8, 16], 7, p);
        let old = pairing(&h, &c, 5, p);
        let (&mask, &coefficient) = h.iter().next().unwrap();
        accum(&mut c, 31 ^ mask, 1, p);
        assert_eq!(pairing(&h, &c, 5, p), add(old, coefficient, p));
        assert_ne!(pairing(&h, &c, 5, p), old);
    }

    #[test]
    fn n83_synthetic_fixture_not_source_bound() {
        let p = donor::field();
        let mut rows = vec![];
        for spike in [false, true] {
            let w: Vec<_> = (0..27)
                .map(|i| 1u128 << if spike && i == 26 { 40 } else { i })
                .collect();
            let sum = w.iter().fold(0, |s, &v| add(s, v, p));
            let exception = mul(sum, (p + 1) / 2, p);
            for t in [7, exception] {
                let (v, _, _) = functional(&w, t, p);
                assert_eq!(v["pairing_nonzero"], t != exception);
                assert_eq!(v["source_bound"], false);
                assert!(v["last_fall_lower_bound"].is_null());
                rows.push(v);
            }
        }
        retain("n83-synthetic.json", &json!({"cases":rows}));
    }

    #[test]
    #[should_panic]
    fn reject_zero_normalization_weight() {
        functional(&[1, 0, 4], 7, 101);
    }

    #[test]
    #[should_panic]
    fn reject_even_width() {
        functional(&[1, 2, 4, 8], 7, 101);
    }
}
