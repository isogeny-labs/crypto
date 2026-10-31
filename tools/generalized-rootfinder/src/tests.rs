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
fn optimized_evaluation_preserves_zero_one_and_higher_power_semantics() {
    for f in [p(2), p(5), binary(3, 11)] {
        let q = f.order.to_u64_digits()[0];
        let equations = vec![
            Term {
                coefficient: N::from(q - 1),
                powers: vec![0, 3, 2],
            },
            Term {
                coefficient: N::one(),
                powers: vec![5, 0, 1],
            },
            Term {
                coefficient: N::from(2u8) % &f.order,
                powers: vec![1, 4, 0],
            },
            Term {
                coefficient: N::one(),
                powers: vec![0, 0, 0],
            },
        ];
        for a in 0..q {
            for b in 0..q {
                for c in 0..q {
                    let x = [N::from(a), N::from(b), N::from(c)];
                    // Independent field-power definition, including 0^0 = 1.
                    let mut reference = N::zero();
                    for t in &equations {
                        let mut v = t.coefficient.clone();
                        for (&power, x) in t.powers.iter().zip(&x) {
                            v = f.mul(&v, &f.pow(x, &N::from(power)));
                        }
                        reference = f.add(&reference, &v);
                    }
                    assert_eq!(solver::evaluate(&f, &equations, &x), reference);
                }
            }
        }
    }
}

#[test]
fn typed_boolean_entry_preserves_complete_roots_exact_counts_and_error_boundaries() {
    use crypto_lib::cryptanalysis::pq_groebner_f2::{F2BoolMono, F2BoolPoly};
    let poly = |n, masks: &[u64]| {
        F2BoolPoly::from_monos(
            masks.iter().copied().map(F2BoolMono::from_mask).collect(),
            n,
        )
    };
    let equations = vec![
        poly(10, &[0, 3, 4]),
        poly(10, &[8, 16]),
        poly(10, &[0, 1, 512]),
    ];
    let expected = (0..1u64 << 10)
        .filter(|&x| equations.iter().all(|p| p.eval(x) == 0))
        .collect::<Vec<_>>();
    let limits = Limits {
        max_work: 1 << 10,
        max_solutions: 3,
        ..Default::default()
    };
    let result = solver::solve_boolean_polynomials(&equations, 10, &limits).unwrap();
    assert_eq!(result.roots, expected[..3]);
    assert_eq!(result.count_exact, expected.len() as u64);
    assert!(result.output_truncated);
    assert!(solver::solve_boolean_polynomials(
        &equations,
        10,
        &Limits {
            max_work: 1,
            ..limits.clone()
        }
    )
    .is_err());
    assert!(solver::solve_boolean_polynomials(&[poly(10, &[7])], 10, &limits).is_err());
    assert!(solver::solve_boolean_polynomials(&[poly(10, &[1 << 11])], 10, &limits).is_err());
    let zero = solver::solve_boolean_polynomials(
        &[poly(32, &[])],
        32,
        &Limits {
            max_work: 0,
            ..limits.clone()
        },
    )
    .unwrap();
    assert_eq!(zero.roots, vec![0, 1, 2]);
    assert_eq!(zero.count_exact, 1u64 << 32);
    assert_eq!(zero.work, 0);
    assert!(zero.output_truncated);
    let empty = solver::solve_boolean_polynomials(
        &[poly(32, &[0])],
        32,
        &Limits {
            max_work: 0,
            ..limits
        },
    )
    .unwrap();
    assert_eq!(empty.count_exact, 0);
    assert!(empty.roots.is_empty());
    assert!(!empty.output_truncated);
    // Independent linear equations pin one exact22-coordinate assignment;
    // this also exercises wide dispatch at the measured large-size boundary.
    let target = 0x2a9537u64;
    let linear = (0..22)
        .map(|i| {
            poly(
                22,
                &if target >> i & 1 == 1 {
                    vec![1 << i, 0]
                } else {
                    vec![1 << i]
                },
            )
        })
        .collect::<Vec<_>>();
    let pinned = solver::solve_boolean_polynomials(
        &linear,
        22,
        &Limits {
            max_work: 1 << 22,
            max_solutions: 1,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(pinned.roots, vec![target]);
    assert_eq!(pinned.count_exact, 1);
    assert!(!pinned.output_truncated);
}

#[test]
fn wide_dispatch_preserves_all_roots_and_checks_unpacked_equations() {
    for n in [8usize, 10, 12] {
        for many in [false, true] {
            let f = binary(3, 11);
            let mut equations = Vec::new();
            for j in 0..if many { 70 } else { 7 } {
                let mut powers = vec![0; n];
                if many {
                    // First64 packed equations constrain only x0. Later
                    // equations constrain x1 and must still be verified.
                    powers[usize::from(j >= 64)] = 1 + (j % 3) as u32;
                } else {
                    powers[j % n] = 1 + (j % 3) as u32;
                    powers[(j + 3) % n] = 2;
                }
                let coefficient = 1u8 << (j % 3);
                let planted = 0b1010_0110_1101u64;
                let constant = if powers
                    .iter()
                    .enumerate()
                    .all(|(i, &p)| p == 0 || planted >> i & 1 == 1)
                {
                    coefficient
                } else {
                    0
                };
                equations.push(vec![
                    Term {
                        coefficient: N::from(coefficient),
                        powers,
                    },
                    Term {
                        coefficient: N::from(constant),
                        powers: vec![0; n],
                    },
                ]);
            }
            // A Boolean polynomial can cancel terms with different exponents.
            let mut a = vec![0; n];
            a[0] = 1;
            let mut b = a.clone();
            b[0] = 2;
            equations.push(vec![
                Term {
                    coefficient: N::one(),
                    powers: a,
                },
                Term {
                    coefficient: N::one(),
                    powers: b,
                },
            ]);
            let mut expected = Vec::new();
            for bits in 0..1u64 << n {
                if equations.iter().all(|e| {
                    let mut coefficient = 0u64;
                    for t in e {
                        if t.powers
                            .iter()
                            .enumerate()
                            .all(|(i, &p)| p == 0 || bits >> i & 1 == 1)
                        {
                            coefficient ^=
                                t.coefficient.to_u64_digits().first().copied().unwrap_or(0);
                        }
                    }
                    coefficient == 0
                }) {
                    expected.push((0..n).map(|i| N::from(bits >> i & 1)).collect::<Vec<_>>());
                }
            }
            expected.sort();
            assert!(!expected.is_empty());
            let limits = Limits {
                max_work: 1 << n,
                max_solutions: usize::MAX,
                ..Default::default()
            };
            for method in ["auto", "native_boolean"] {
                let got = solver::search(&f, &equations, n, &domain(2), &limits, method).unwrap();
                assert!(got.complete);
                assert_eq!(got.count, Some(expected.len() as u64));
                assert_eq!(got.solutions, expected);
                let capped = solver::search(
                    &f,
                    &equations,
                    n,
                    &domain(2),
                    &Limits {
                        max_solutions: 3,
                        ..limits
                    },
                    method,
                )
                .unwrap();
                assert_eq!(capped.count, Some(expected.len() as u64));
                assert_eq!(capped.solutions.len(), 3.min(expected.len()));
                assert!(capped.solutions.iter().all(|x| expected.contains(x)));
            }
        }
    }
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
