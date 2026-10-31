#[cfg(test)]
mod exhaustive_small_field {
    use super::*;
    use std::collections::BTreeMap;

    // This is a finite gate, not a universal group-law theorem. The chosen
    // monic quintic has derivative 5x^4+1 and is square-free over F_7 and F_11.
    fn all_reduced(j: &Jacobian) -> Vec<Divisor> {
        let p = j.f.p;
        let mut out = vec![Divisor::O];
        for x in 0..p {
            for y in 0..p {
                if j.f.sq(y) == evaluate(&j.f, &j.polynomial, x) {
                    out.push(j.point(x, y));
                }
            }
        }
        for q in 0..p {
            for r in 0..p {
                for s in 0..p {
                    for t in 0..p {
                        let d = Divisor {
                            degree: 2,
                            u: [r, q, 1],
                            v: [t, s],
                        };
                        if j.valid(d) {
                            out.push(d);
                        }
                    }
                }
            }
        }
        out.sort();
        out.dedup();
        assert!(out.iter().all(|&d| j.valid(d)));
        out
    }

    fn weighted_add(
        j: &Jacobian,
        a: Divisor,
        b: Divisor,
        za: u64,
        wa: u64,
        zb: u64,
        wb: u64,
    ) -> (Option<ResultPoint>, &'static str) {
        let d1 = |d: Divisor, z: u64, w: u64| D1 {
            x: j.f.mul(j.f.neg(d.u[0]), j.f.sq(z)),
            y: j.f.mul(d.v[0], j.f.mul(j.f.pow(z, 5), w)),
            z,
            w,
        };
        match (a.degree, b.degree) {
            (0, _) => (Some(from_affine(b, &j.f)), "identity_left"),
            (_, 0) => (Some(from_affine(a, &j.f)), "identity_right"),
            (1, 1) => {
                let (v, branch) = add11(d1(a, za, wa), d1(b, zb, wb), &j.polynomial, &j.f);
                (Some(v), branch)
            }
            (1, 2) => add12_weighted_complete(
                d1(a, za, wa),
                weight_degree_two(b, zb, wb, &j.f),
                &j.polynomial,
                &j.f,
            ),
            (2, 1) => add12_weighted_complete(
                d1(b, zb, wb),
                weight_degree_two(a, za, wa, &j.f),
                &j.polynomial,
                &j.f,
            ),
            (2, 2) => add22_weighted(
                weight_degree_two(a, za, wa, &j.f),
                weight_degree_two(b, zb, wb, &j.f),
                &j.polynomial,
                &j.f,
            ),
            _ => unreachable!(),
        }
    }

    #[test]
    fn all_valid_reduced_pairs_on_two_smooth_small_models() {
        for p in [7u64, 11] {
            let j = Jacobian {
                f: Field::new(p, 1, 0),
                polynomial: vec![2, 1, 0, 0, 0, 1],
            };
            let divisors = all_reduced(&j);
            let mut labels = BTreeMap::<String, u64>::new();
            let mut checked = 0u64;
            let mut unresolved = 0u64;
            let weights = [(1, 1, 1, 1), (2, 3, 3, 5)];
            for &a in &divisors {
                for &b in &divisors {
                    let expected = j.add(a, b);
                    for &(za, wa, zb, wb) in &weights {
                        let (out, label) = weighted_add(&j, a, b, za, wa, zb, wb);
                        *labels.entry(label.to_string()).or_default() += 1;
                        if let Some(out) = out {
                            assert_eq!(
                                normalize(out, &j.f),
                                expected,
                                "p={p} a={a:?} b={b:?} weights=({za},{wa},{zb},{wb}) label={label}"
                            );
                            checked += 1;
                        } else {
                            unresolved += 1;
                        }
                    }
                }
            }
            println!(
                "EXHAUSTIVE_GATE_JSON {}",
                serde_json::json!({
                    "p": p, "curve": [2,1,0,0,0,1], "divisors": divisors.len(),
                    "ordered_pairs": divisors.len() * divisors.len(),
                    "weight_variants": weights.len(), "checked": checked,
                    "unresolved": unresolved, "labels": labels,
                })
            );
            assert_eq!(unresolved, 0, "p={p}: unresolved weighted law branches");
        }
    }
}
