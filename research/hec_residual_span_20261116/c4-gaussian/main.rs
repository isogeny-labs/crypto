mod core;
use hec_native_ic::{counted, fp::Fp, instance::factor_base, jac::Curve, poly::Poly};
use std::fs;
fn main() {
    let root = "research/hec_residual_span_20261116";
    let mut models = 0;
    println!("p\tmodel\ti\tC1\ttwisted_count\tnonsquare\ttrace_sigma_pi\tcandidates\tA\tB\tlambda\tmodulus\tcounts");
    for l in fs::read_to_string(format!("{root}/order4-strong/SOURCES.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        let n = |j: usize| v[j].parse::<u64>().unwrap();
        let p = n(0);
        let code = n(1);
        let order = n(2);
        let ell = n(3);
        let original_i = n(5);
        let original_lambda = n(6);
        let c = Curve::new(
            Fp::new(p),
            Poly::from_coeffs(&[0, 2, 0, code % 4, 0, (code / 4) % 4, 0, code / 16, 0, 1]),
        );
        let weil = fs::read_to_string(format!("{root}/order4-strong/WEIL.tsv")).unwrap();
        let row = weil
            .lines()
            .skip(1)
            .find(|l| {
                let v: Vec<_> = l.split('\t').collect();
                v[0].parse::<u64>().unwrap() == p
            })
            .unwrap();
        let coeff: Vec<i128> = row
            .split('\t')
            .nth(2)
            .unwrap()
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect();
        assert_ne!(coeff[4] % p as i128, 0);
        for i in [original_i, p - original_i] {
            let mut previous = None;
            for _rep in 0..2 {
                counted::reset();
                let (t, choices) = core::derive(&c, order, ell, i);
                let cs = counted::snapshot();
                counted::disable();
                let sig = (choices.clone(), cs.clone());
                if let Some(prev) = &previous {
                    assert_eq!(prev, &sig)
                } else {
                    previous = Some(sig)
                }
                assert_eq!(choices.len(), 1, "ambiguity must remain explicit");
                let x = &choices[0];
                assert_eq!(core::norm_polynomial(&x.g), coeff);
                assert_eq!(
                    x.lambda,
                    if i == original_i {
                        original_lambda
                    } else {
                        ell - original_lambda
                    }
                );
                let fb = factor_base(&c);
                assert_eq!(
                    t.ordinary_count,
                    1 + fb.iter().map(|p| if p.1 == 0 { 1 } else { 2 }).sum::<u64>()
                );
                for &(px, py) in &fb {
                    let d = c.mul(&c.place(px, py), 2);
                    let image = c.mul(&c.place(c.fp.neg(px), c.fp.mul(i, py)), 2);
                    assert_eq!(image, c.mul(&d, x.lambda));
                }
                for (modulus, values) in cs {
                    let s = values
                        .iter()
                        .map(u64::to_string)
                        .collect::<Vec<_>>()
                        .join(",");
                    println!(
                        "{p}\t{code}\t{i}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{modulus}\t{s}",
                        t.ordinary_count,
                        t.twisted_count,
                        t.nonsquare,
                        t.imag_trace,
                        choices.len(),
                        x.value.r,
                        x.value.i,
                        x.lambda
                    );
                }
            }
        }
        models += 1;
    }
    assert_eq!(models, 3);
    eprintln!("models=3 orientations=6 repetitions=2 unique_candidates=true complete_Weil_norm_matches=true all_catalogue_scalar_actions=true conditional_det_unit_p_squared=true");
}
