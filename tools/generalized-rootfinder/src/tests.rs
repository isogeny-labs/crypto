use super::solver::{
    self,
    field::{Field, N},
    poly, Equation, Limits, Term,
};
use num_traits::{One, Zero};
use serde_json::json;

fn p(n: u64) -> Field {
    Field::prime(N::from(n), None).unwrap()
}
fn binary(d: usize, mask: u64) -> Field {
    Field::binary(d, N::from(mask)).unwrap()
}
fn domain(q: usize) -> Vec<N> {
    (0..q).map(N::from).collect()
}
fn brute(f: &Field, e: &[Equation], n: usize, d: &[N]) -> Vec<Vec<N>> {
    let mut out = Vec::new();
    for mut code in 0..d.len().pow(n as u32) {
        let mut x = Vec::new();
        for _ in 0..n {
            x.push(d[code % d.len()].clone());
            code /= d.len();
        }
        if e.iter().all(|e| solver::evaluate(f, e, &x).is_zero()) {
            out.push(x);
        }
    }
    out.sort();
    out
}
#[test]
fn certified_fields_reject_composites_and_reducible_moduli() {
    for n in [0u64, 1, 4, 9, 341, 561, 3215031751, 3825123056546413051] {
        assert!(Field::prime(N::from(n), None).is_err());
    }
    for mask in [5u64, 9, 15] {
        assert!(Field::binary(if mask < 8 { 2 } else { 3 }, N::from(mask)).is_err());
    }
    assert!(Field::extension(N::from(3u8), vec![N::from(2u8), N::zero(), N::one()], None).is_err());
    assert!(Field::prime((N::one() << 89) - N::one(), None).is_err());
    for f in [
        p(2),
        p(3),
        p(5),
        binary(2, 7),
        binary(3, 11),
        Field::extension(N::from(3u8), vec![N::one(), N::zero(), N::one()], None).unwrap(),
    ] {
        for a in domain(if f.degree == 3 {
            8
        } else if f.degree == 2 {
            if f.p == N::from(3u8) {
                9
            } else {
                4
            }
        } else {
            f.p.to_u64_digits()[0] as usize
        }) {
            if !a.is_zero() {
                assert_eq!(f.mul(&a, &f.inv(&a)), N::one());
            }
        }
    }
}
#[test]
fn pocklington_proves_a_wide_prime_and_rejects_tampering() {
    let n = (N::one() << 127) - N::one();
    let qs = [
        (2u64, 1u32),
        (3, 3),
        (7, 2),
        (19, 1),
        (43, 1),
        (73, 1),
        (127, 1),
        (337, 1),
        (5419, 1),
        (92737, 1),
        (649657, 1),
        (77158673929, 1),
    ];
    let mut factors = Vec::new();
    let mut product = N::one();
    for (q, e) in qs {
        let q = N::from(q);
        product *= q.pow(e);
        let witness = (2u64..1000)
            .map(N::from)
            .find(|a| {
                use num_integer::Integer;
                let z = a.modpow(&((&n - N::one()) / &q), &n);
                !z.is_zero() && (z - N::one()).gcd(&n) == N::one()
            })
            .unwrap();
        factors.push(json!({"q":q.to_str_radix(10),"e":e,"a":witness.to_str_radix(10)}));
    }
    assert_eq!(product, &n - N::one());
    let mut cert = json!({"factors":factors});
    assert!(Field::prime(n.clone(), Some(&cert)).is_ok());
    cert["factors"][0]["a"] = json!(1);
    assert!(Field::prime(n, Some(&cert)).is_err());
}
#[test]
fn all_small_monic_polynomials_include_repeated_and_inseparable_roots() {
    for f in [p(2), p(3), binary(2, 7)] {
        let q = f.order.to_u64_digits()[0] as usize;
        for degree in 0..=5 {
            for mut code in 0..q.pow(degree as u32) {
                let mut polynomial = Vec::new();
                for _ in 0..degree {
                    polynomial.push(N::from(code % q));
                    code /= q;
                }
                polynomial.push(N::one());
                let expected = domain(q)
                    .into_iter()
                    .filter(|x| poly::eval(&f, &polynomial, x).is_zero())
                    .collect::<Vec<_>>();
                let mut budget = poly::SplitBudget {
                    remaining: 1000,
                    state: 7,
                };
                let got = poly::roots(&f, polynomial, q, 8, &mut budget).unwrap();
                assert_eq!(got, expected);
            }
        }
    }
}
#[test]
fn large_binary_field_factorization_is_complete_with_repeated_inputs() {
    let f = Field::binary(83, (N::one() << 83) | (N::one() << 45) | N::from(7u8)).unwrap();
    let roots = vec![
        N::one(),
        (N::one() << 80) + N::from(13u8),
        (N::one() << 70) + N::from(9u8),
    ];
    let mut polynomial = vec![N::one()];
    for x in roots.iter().chain(roots.iter()) {
        polynomial = poly::mul(&f, &polynomial, &[f.neg(x), N::one()]);
    }
    let mut budget = poly::SplitBudget {
        remaining: 1000,
        state: 23,
    };
    let mut expected = roots;
    expected.sort();
    assert_eq!(
        poly::roots(&f, polynomial, 16, 16, &mut budget).unwrap(),
        expected
    );
}
#[test]
fn rational_compositions_exclude_literal_intermediate_poles() {
    let input = json!({"field":{"kind":"prime","characteristic":5},"variables":["x"],"equations":[[]],"L":{"semantics":"partial_circuit","composition":[{"coefficients":[0,1],"denominator":[0,1]},{"coefficients":[4,1]}]}});
    let answer = solver::solve_json(&input, &Limits::default()).unwrap();
    assert_eq!(answer["status"], "complete");
    assert_eq!(answer["solution_count_exact"], 4);
    assert_eq!(
        answer["solutions"],
        json!([["0x1"], ["0x2"], ["0x3"], ["0x4"]])
    );
    let mut bad = input;
    bad["L"]["semantics"] = json!("rational_function");
    let answer = solver::solve_json(&bad, &Limits::default()).unwrap();
    assert_eq!(answer["certificate"]["search_complete"], false);
}
#[test]
fn linearized_domains_match_literal_enumeration_in_both_characteristics() {
    for f in [
        binary(2, 7),
        binary(3, 11),
        Field::extension(N::from(3u8), vec![N::one(), N::zero(), N::one()], None).unwrap(),
    ] {
        let q = f.order.to_u64_digits()[0] as usize;
        for a in domain(q) {
            for b in domain(q) {
                let terms = vec![(0, a.clone()), (1, b.clone()), (f.degree + 1, N::one())];
                let c = N::one();
                let expected = domain(q)
                    .into_iter()
                    .filter(|x| {
                        terms
                            .iter()
                            .fold(c.clone(), |v, (power, a)| {
                                f.add(
                                    &v,
                                    &f.mul(a, &f.pow(x, &f.p.pow((power % f.degree) as u32))),
                                )
                            })
                            .is_zero()
                    })
                    .collect::<Vec<_>>();
                assert_eq!(
                    solver::linearized_domain(&f, &terms, &c, 100).unwrap(),
                    expected
                );
            }
        }
    }
}
#[test]
fn native_boolean_and_component_mitm_return_every_original_tuple() {
    for f in [p(2), p(3), binary(2, 7)] {
        let q = f.order.to_u64_digits()[0] as usize;
        let d = domain(q);
        let mut state = 7u64;
        for _ in 0..30 {
            let mut equations = vec![vec![], vec![]];
            for e in &mut equations {
                for powers in [
                    [0, 0, 0, 0],
                    [1, 0, 0, 0],
                    [0, 1, 0, 0],
                    [1, 1, 0, 0],
                    [0, 0, 1, 0],
                    [0, 0, 0, 1],
                    [0, 0, 1, 1],
                ] {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    e.push(Term {
                        coefficient: N::from(state % q as u64),
                        powers: powers.to_vec(),
                    });
                }
            }
            let expected = brute(&f, &equations, 4, &d);
            for method in ["auto", "mitm", "enumerate"] {
                let got =
                    solver::search(&f, &equations, 4, &d, &Limits::default(), method).unwrap();
                assert!(got.complete);
                assert_eq!(got.count, Some(expected.len() as u64));
                assert_eq!(got.solutions, expected);
            }
        }
    }
    // Non-prime coefficients become multiple binary equations, preserving
    // the full GF4 equation rather than only a parity projection.
    let f = binary(2, 7);
    let e = vec![vec![
        Term {
            coefficient: N::from(2u8),
            powers: vec![1, 0],
        },
        Term {
            coefficient: N::one(),
            powers: vec![0, 1],
        },
    ]];
    assert_eq!(
        solver::search(&f, &e, 2, &domain(2), &Limits::default(), "native_boolean")
            .unwrap()
            .solutions,
        vec![vec![N::zero(), N::zero()]]
    );
}
#[test]
fn truncation_and_budget_failures_cannot_masquerade_as_complete_empty_sets() {
    let input = json!({"field":{"kind":"prime","characteristic":3},"variables":["x","y"],"equations":[[]],"L":{"coefficients":[0]}});
    let mut limits = Limits {
        max_solutions: 2,
        ..Default::default()
    };
    let out = solver::solve_json(&input, &limits).unwrap();
    assert_eq!(out["status"], "output_truncated");
    assert_eq!(out["solution_count_exact"], 9);
    assert_eq!(out["certificate"]["all_solutions_returned"], false);
    limits.max_work = 1;
    let out = solver::solve_json(&input, &limits).unwrap();
    assert!(out["solution_count_exact"].is_null());
    assert_eq!(out["certificate"]["search_complete"], false);
    limits.max_domain = 1;
    let out = solver::solve_json(&input, &limits).unwrap();
    assert_eq!(out["certificate"]["domain_complete"], false);
}

#[test]
fn scalar_coefficients_and_zero_output_caps_preserve_exact_counts() {
    let input = json!({"field":{"kind":"prime","characteristic":5},"variables":["x"],"equations":[[{"powers":[1],"coefficient":1},{"powers":[0],"coefficient":{"scalar":-1}}]],"L":{"coefficients":[0]}});
    let out = solver::solve_json(&input, &Limits::default()).unwrap();
    assert_eq!(out["solutions"], json!([["0x1"]]));
    let f = p(2);
    let limits = Limits {
        max_work: 1 << 31,
        max_solutions: 0,
        ..Default::default()
    };
    let out = solver::search(&f, &[vec![]], 31, &domain(2), &limits, "auto").unwrap();
    assert_eq!(out.count, Some(1 << 31));
    assert!(out.solutions.is_empty());
    let f = p(3);
    let limits = Limits {
        max_solutions: 0,
        ..Default::default()
    };
    let out = solver::search(&f, &[vec![]], 4, &domain(3), &limits, "mitm").unwrap();
    assert_eq!(out.count, Some(81));
    assert!(out.solutions.is_empty());
}
#[test]
fn s3_separator_matches_all_eight_coordinate_brute_force_including_diagonals() {
    for f in [p(3), p(5), binary(2, 7)] {
        let q = f.order.to_u64_digits()[0] as usize;
        let d = domain(2);
        let limits = Limits {
            max_solutions: 100_000,
            max_trials: 100_000,
            ..Default::default()
        };
        let mut separator =
            solver::Separator::new(&f, d.clone(), &json!({"a4":1,"a6":1,"b":1}), &limits).unwrap();
        for target in domain(q) {
            let mut expected = Vec::new();
            for mut code in 0..32 {
                let mut x = Vec::new();
                for _ in 0..5 {
                    x.push(d[code % 2].clone());
                    code /= 2;
                }
                for u1 in domain(q) {
                    if !separator.evaluate(&x[0], &x[1], &u1).is_zero() {
                        continue;
                    }
                    for u2 in domain(q) {
                        if !separator.evaluate(&u1, &x[2], &u2).is_zero() {
                            continue;
                        }
                        for u3 in domain(q) {
                            if separator.evaluate(&u2, &x[3], &u3).is_zero()
                                && separator.evaluate(&u3, &x[4], &target).is_zero()
                            {
                                let mut row = x.clone();
                                row.extend([u1.clone(), u2.clone(), u3]);
                                expected.push(row);
                            }
                        }
                    }
                }
            }
            expected.sort();
            let got = separator.solve(&target).unwrap();
            assert_eq!(got.count, Some(expected.len() as u64));
            assert_eq!(got.solutions, expected);
        }
    }
}
