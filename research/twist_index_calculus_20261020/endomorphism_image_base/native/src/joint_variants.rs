//! Additional representations, separate from the frozen global constructor.
use super::*;

pub(super) fn quartic_discriminant_terms() -> Vec<(i64, Exponent)> {
    vec![
        (256, [0, 0, 0, 3, 0]),
        (-192, [1, 0, 1, 2, 0]),
        (-128, [0, 2, 0, 2, 0]),
        (144, [0, 1, 2, 1, 0]),
        (-27, [0, 0, 4, 0, 0]),
        (144, [2, 1, 0, 2, 0]),
        (-6, [2, 0, 2, 1, 0]),
        (-80, [1, 2, 1, 1, 0]),
        (18, [1, 1, 3, 0, 0]),
        (16, [0, 4, 0, 1, 0]),
        (-4, [0, 3, 2, 0, 0]),
        (-27, [4, 0, 0, 2, 0]),
        (18, [3, 1, 1, 1, 0]),
        (-4, [3, 0, 3, 0, 0]),
        (-4, [2, 3, 0, 1, 0]),
        (1, [2, 2, 2, 0, 0]),
    ]
}
fn quartic_discriminant(f: Field) -> Polynomial {
    let mut p = Polynomial::default();
    for (c, e) in quartic_discriminant_terms() {
        p.term(f, e, f.scalar(c.rem_euclid(f.p as i64) as u64));
    }
    p
}

pub(super) fn integer_quartic_discriminant() -> (integer::ZPoly, integer::ZPoly) {
    use integer::ZPoly as Z;
    let roots: [Z; 4] = std::array::from_fn(Z::var);
    let mut es = vec![
        Z::constant(1),
        Z::default(),
        Z::default(),
        Z::default(),
        Z::default(),
    ];
    for x in &roots {
        for k in (1..=4).rev() {
            es[k] = es[k].add(&es[k - 1].mul(x));
        }
    }
    let mut discriminant = Z::default();
    for (c, e) in quartic_discriminant_terms() {
        let mut term = Z::constant(c as i128);
        for i in 0..4 {
            term = term.mul(&es[i + 1].pow(e[i] as u8));
        }
        discriminant = discriminant.add(&term);
    }
    let mut vandermonde = Z::constant(1);
    for i in 0..4 {
        for j in i + 1..4 {
            vandermonde = vandermonde.mul(&roots[i].sub(&roots[j]).pow(2));
        }
    }
    (discriminant, vandermonde)
}

fn implicit(case: &Case, arm: usize) -> (Polynomial, Polynomial) {
    let f = case.c.f;
    let u = Polynomial::var(f, 3);
    let w = Polynomial::var(f, 4);
    let one = Polynomial::scalar(f, 1);
    let b = case.c.b;
    if arm == 7 {
        let a = f.mul(
            f.neg(f.scalar(case.omega * case.omega % case.p)),
            f.scalar(fpow(3, case.p - 2, case.p)),
        );
        let z = u.add(f, &w.scale(f, f.mul(f.scalar(4), b))).scale(f, a);
        (z, u.pow(f, 2).mul(f, &w).sub(f, &one))
    } else {
        assert_eq!(arm, 9);
        let c = f.pow(b, 4);
        assert_eq!(c[1..], [0, 0, 0]);
        let inv4 = f.scalar(fpow(4, case.p - 2, case.p));
        let middle = u
            .scale(f, c)
            .sub(f, &u.pow(f, 10).scale(f, b))
            .add(f, &u.pow(f, 7).scale(f, f.pow(b, 2)))
            .sub(f, &u.pow(f, 4).scale(f, f.pow(b, 3)));
        let z = u
            .scale(f, inv4)
            .add(f, &w.mul(f, &middle).scale(f, f.mul(f.scalar(9), inv4)));
        (
            z,
            u.pow(f, 12)
                .sub(f, &Polynomial::constant(c))
                .mul(f, &w)
                .sub(f, &one),
        )
    }
}

fn reduce_inverse(f: Field, p: &Polynomial, arm: usize, c: E) -> Polynomial {
    let step = if arm == 7 {
        2
    } else {
        assert_eq!(arm, 9);
        12
    };
    let mut pending = p.clone();
    let mut reduced = Polynomial::default();
    while let Some((e, coefficient)) = pending.0.pop_last() {
        if e[3] >= step && e[4] > 0 {
            let mut next = e;
            next[3] -= step;
            next[4] -= 1;
            pending.term(f, next, coefficient);
            if arm == 9 {
                next[4] += 1;
                pending.term(f, next, f.mul(coefficient, c));
            }
        } else {
            reduced.term(f, e, coefficient);
        }
    }
    reduced
}

fn fully_symmetric(f: Field, generic: &Polynomial) -> Polynomial {
    // Auxiliary U moves to index four; zero..three become S1..S4.
    let s1 = Polynomial::var(f, 0);
    let s2 = Polynomial::var(f, 1);
    let s3 = Polynomial::var(f, 2);
    let s4 = Polynomial::var(f, 3);
    let u = Polynomial::var(f, 4);
    let e1 = s1.sub(f, &u);
    let e2 = s2.sub(f, &u.mul(f, &e1));
    let e3 = s3.sub(f, &u.mul(f, &e2));
    let substitutions = [e1, e2, e3, u.clone()];
    let maxima: Vec<_> = (0..4)
        .map(|i| generic.0.keys().map(|e| e[i]).max().unwrap_or(0))
        .collect();
    let powers: Vec<Vec<_>> = (0..4)
        .map(|i| {
            (0..=maxima[i])
                .map(|k| substitutions[i].pow(f, k))
                .collect()
        })
        .collect();
    let mut expanded = Polynomial::default();
    for (e, &c) in &generic.0 {
        assert_eq!(e[4], 0);
        let mut term = Polynomial::constant(c);
        for i in 0..4 {
            term = term.mul(f, &powers[i][e[i] as usize]);
        }
        expanded = expanded.add(f, &term);
    }
    // U^4 = S1 U^3 - S2 U^2 + S3 U - S4.
    let replacements = [s4.neg(f), s3, s2.neg(f), s1];
    let degree = expanded.0.keys().map(|e| e[4]).max().unwrap_or(0);
    for k in (4..=degree).rev() {
        let selected: Vec<_> = expanded
            .0
            .iter()
            .filter(|(e, _)| e[4] == k)
            .map(|(&e, &c)| (e, c))
            .collect();
        for (e, c) in selected {
            expanded.0.remove(&e);
            for j in 0..4 {
                let mut base = e;
                base[4] = k - 4 + j as u16;
                let term = Polynomial(BTreeMap::from([(base, c)])).mul(f, &replacements[j]);
                expanded = expanded.add(f, &term);
            }
        }
    }
    assert!(
        expanded.0.keys().all(|e| e[4] == 0),
        "S5 four-factor symmetry failed its exact coefficient check"
    );
    expanded
}

pub(super) fn export(public: &Path, expected: &str, arm_name: &str, qi: usize, out: &Path) {
    let raw = checked_public(public, expected);
    let arm = arm_index(arm_name);
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let case = Case::new(raw, out);
    let f = case.c.f;
    let residual = case.residual(&case.raw["query_trace"][qi]);
    let (_, h) = integer::derive();
    let (generic, matrix, power) = generic(f, case.c.b, &h, residual);
    probes(f, &generic, &matrix);
    let (joined, constraints, variables, encoding, root_encoding, checks, unreduced) = if arm == 8 {
        let joined = fully_symmetric(f, &generic);
        let vars: Vec<_> = (0..4).map(|i| Polynomial::var(f, i)).collect();
        let constraints = remainder(
            f,
            &domain(&case),
            &[
                vars[3].clone(),
                vars[2].neg(f),
                vars[1].clone(),
                vars[0].neg(f),
                Polynomial::scalar(f, 1),
            ],
        );
        let ordinary: Vec<_> = case.pool[..case.l0].iter().map(|v| v.p.x[0]).collect();
        let mut checks = 0;
        let mut nonzero = 0;
        for offset in 0..ordinary.len() {
            let roots: Vec<_> = (0..4)
                .map(|j| ordinary[(offset + j) % ordinary.len()])
                .collect();
            let mut es = [1u64, 0, 0, 0, 0];
            for &x in &roots {
                for k in (1..=4).rev() {
                    es[k] = (es[k] + es[k - 1] * x) % case.p;
                }
            }
            let s = [
                f.scalar(es[1]),
                f.scalar(es[2]),
                f.scalar(es[3]),
                f.scalar(es[4]),
                [0; 4],
            ];
            let t = [
                f.scalar((roots[0] + roots[1] + roots[2]) % case.p),
                f.scalar(
                    (roots[0] * roots[1] + roots[0] * roots[2] + roots[1] * roots[2]) % case.p,
                ),
                f.scalar(roots[0] * roots[1] % case.p * roots[2] % case.p),
                f.scalar(roots[3]),
                [0; 4],
            ];
            assert_eq!(joined.eval(f, &s), generic.eval(f, &t));
            nonzero += usize::from(joined.eval(f, &s) != [0; 4]);
            checks += 1;
        }
        assert!(nonzero > 0, "four-factor evaluation checks were all zero");
        (
            joined,
            constraints,
            4,
            "finite-domain-full-four-symmetric",
            "elementary-four",
            checks,
            generic.stats(),
        )
    } else {
        let (z, inverse_constraint) = implicit(&case, arm);
        let raw_joined = compose(f, &generic, &z, &Polynomial::scalar(f, 1), power);
        let joined = reduce_inverse(f, &raw_joined, arm, f.pow(case.c.b, 4));
        let mut checks = 0;
        let mut nonzero = 0;
        for &id in &case.peels[arm] {
            let u = case.pool[case.pool[id].source_id].p.x[0];
            let denominator = if arm == 7 {
                u * u % case.p
            } else {
                (fpow(u, 12, case.p) + case.p - f.pow(case.c.b, 4)[0]) % case.p
            };
            assert_ne!(denominator, 0);
            let w = fpow(denominator, case.p - 2, case.p);
            let x = [
                f.scalar(3),
                f.scalar(7),
                f.scalar(11),
                f.scalar(u),
                f.scalar(w),
            ];
            assert_eq!(inverse_constraint.eval(f, &x), [0; 4]);
            assert_eq!(z.eval(f, &x), case.pool[id].p.x);
            let mut direct = x;
            direct[3] = case.pool[id].p.x;
            direct[4] = [0; 4];
            assert_eq!(joined.eval(f, &x), generic.eval(f, &direct));
            assert_eq!(joined.eval(f, &x), raw_joined.eval(f, &x));
            nonzero += usize::from(joined.eval(f, &x) != [0; 4]);
            checks += 1;
        }
        assert!(nonzero > 0, "implicit map evaluation checks were all zero");
        let mut constraints = finite_constraints(f, arm, &domain(&case));
        constraints.push(inverse_constraint);
        (
            joined,
            constraints,
            5,
            "finite-domain-implicit-image",
            "elementary-three-and-source",
            checks,
            raw_joined.stats(),
        )
    };
    let mut equations = joined.components(f, variables);
    for c in &constraints {
        equations.push(c.fp_equation(f, variables));
    }
    let stats: Vec<_> = equations
        .iter()
        .map(|p| {
            json!({"terms":p.len(),
        "degree":p.iter().map(|(e,_)| e.iter().sum::<u32>()).max().unwrap_or(0)})
        })
        .collect();
    write(
        &out.join("system.json"),
        &json!({"schema":"joint-image-system/v1",
        "p":case.p,"seed":case.seed,"arm":arm_name,"query_index":qi,
        "variables":variables,"equations":input_json(&equations),
        "root_encoding":root_encoding,"encoding":encoding}),
    );
    std::fs::write(
        out.join("system.ms"),
        msolve_input(&equations, case.p, variables),
    )
    .unwrap();
    let geometric_variables = if arm == 8 { 5 } else { 6 };
    let mut geometric = joined.components(f, variables);
    if arm == 8 {
        for equation in &mut geometric {
            for (e, _) in equation {
                e.resize(5, 0);
            }
        }
        let guard = quartic_discriminant(f)
            .mul(f, &Polynomial::var(f, 4))
            .sub(f, &Polynomial::scalar(f, 1));
        geometric.push(guard.fp_equation(f, 5));
    } else {
        geometric.push(constraints.last().unwrap().fp_equation(f, 5));
        for equation in &mut geometric {
            for (e, _) in equation {
                e.resize(6, 0);
            }
        }
        let mut guard = discriminant(f).fp_equation(f, 5);
        for (e, _) in &mut guard {
            e.resize(6, 0);
            e[5] = 1;
        }
        guard.push((vec![0; 6], case.p - 1));
        geometric.push(f4_fp::normalise(&guard, case.p, f4_fp::Ordering::Grevlex));
    }
    write(
        &out.join("geometric.json"),
        &json!({"schema":"joint-image-system/v1",
        "p":case.p,"seed":case.seed,"arm":arm_name,"query_index":qi,
        "variables":geometric_variables,"equations":input_json(&geometric),
        "root_encoding":root_encoding,"encoding":if arm==8 {"geometric-full-four-symmetric"} else {"geometric-implicit-image"}}),
    );
    std::fs::write(
        out.join("geometric.ms"),
        msolve_input(&geometric, case.p, geometric_variables),
    )
    .unwrap();
    write(
        &out.join("summary.json"),
        &json!({"schema":"joint-image-variant-export/v1",
        "status":"exported-not-solved","p":case.p,"seed":case.seed,"arm":arm_name,
        "query_index":qi,"public_input_sha256":expected,"encoding":encoding,
        "root_encoding":root_encoding,"relation_stats":joined.stats(),
        "unreduced_relation_stats":unreduced,
        "geometric_variables":geometric_variables,"geometric_equations":geometric.len(),
        "equation_stats":stats,"map_or_symmetry_checks":checks,
        "four_factor_coefficient_independence":arm==8,
        "cost_scope":"Unadmitted construction only; no root completeness or speedup"}),
    );
    println!(
        "{}",
        json!({"arm":arm_name,"encoding":encoding,
        "relation_stats":joined.stats(),"equations":equations.len()})
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_symmetric_regression_export_has_consistent_geometric_arity() {
        let tape =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../quartic_twist_mixed_v1.ndjson");
        let scratch =
            std::env::temp_dir().join(format!("joint-four-export-check-{}", std::process::id()));
        assert!(!scratch.exists());
        std::fs::create_dir(&scratch).unwrap();
        let public = public_record(&load(&tape, INPUT_SHA, 37, 0));
        let bytes = serde_json::to_vec(&public).unwrap();
        let sha = hex::encode(hash::sha256::sha256(&bytes));
        let path = scratch.join("public.json");
        std::fs::write(&path, bytes).unwrap();
        let out = scratch.join("export");
        export(&path, &sha, "l0-peel", 2, &out);
        for (name, n) in [("system", 4), ("geometric", 5)] {
            let v: Value =
                serde_json::from_slice(&std::fs::read(out.join(format!("{name}.json"))).unwrap())
                    .unwrap();
            let equations: Vec<f4_fp::Poly> =
                serde_json::from_value(v["equations"].clone()).unwrap();
            assert_eq!(v["variables"], n);
            assert!(equations.iter().flatten().all(|(e, _)| e.len() == n));
            assert_eq!(v["root_encoding"], "elementary-four");
            assert!(out.join(format!("{name}.ms")).is_file());
        }
        std::fs::remove_dir_all(scratch).unwrap();
    }
    #[test]
    fn implicit_coordinates_match_every_registered_group_image() {
        let tape =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../quartic_twist_mixed_v1.ndjson");
        let scratch =
            std::env::temp_dir().join(format!("joint-implicit-check-{}", std::process::id()));
        assert!(!scratch.exists());
        std::fs::create_dir(&scratch).unwrap();
        for p in PRIMES {
            for seed in 0..3 {
                let directory = scratch.join(format!("p{p}-s{seed}"));
                std::fs::create_dir(&directory).unwrap();
                let case = Case::new(public_record(&load(&tape, INPUT_SHA, p, seed)), &directory);
                let f = case.c.f;
                for arm in [7, 9] {
                    let (z, constraint) = implicit(&case, arm);
                    let normalized = reduce_inverse(f, &z, arm, f.pow(case.c.b, 4));
                    for &id in &case.peels[arm] {
                        let u = case.pool[case.pool[id].source_id].p.x[0];
                        let d = if arm == 7 {
                            u * u % p
                        } else {
                            (fpow(u, 12, p) + p - f.pow(case.c.b, 4)[0]) % p
                        };
                        assert_ne!(d, 0);
                        let x = [
                            [0; 4],
                            [0; 4],
                            [0; 4],
                            f.scalar(u),
                            f.scalar(fpow(d, p - 2, p)),
                        ];
                        assert_eq!(constraint.eval(f, &x), [0; 4]);
                        assert_eq!(normalized.eval(f, &x), case.pool[id].p.x);
                    }
                }
            }
        }
        std::fs::remove_dir_all(scratch).unwrap();
    }
    #[test]
    fn four_factor_rewrite_uses_all_four_elementary_coordinates() {
        let f = Field { p: 37, d: 2 };
        let a = Polynomial::var(f, 0);
        let b = Polynomial::var(f, 1);
        let c = Polynomial::var(f, 2);
        let u = Polynomial::var(f, 3);
        for (partial, expected) in [
            (a.add(f, &u), Polynomial::var(f, 0)),
            (b.add(f, &a.mul(f, &u)), Polynomial::var(f, 1)),
            (c.add(f, &b.mul(f, &u)), Polynomial::var(f, 2)),
            (c.mul(f, &u), Polynomial::var(f, 3)),
        ] {
            assert_eq!(fully_symmetric(f, &partial), expected);
        }
    }
    #[test]
    fn inverse_normal_form_respects_both_inverse_equations() {
        let f = Field::new(37);
        let u = Polynomial::var(f, 3);
        let w = Polynomial::var(f, 4);
        let one = Polynomial::scalar(f, 1);
        let c = f.pow([0, 0, 0, 2], 4);
        let value = u
            .pow(f, 30)
            .mul(f, &w.pow(f, 3))
            .add(f, &u.pow(f, 9).mul(f, &w.pow(f, 7)));
        for arm in [7, 9] {
            let inverse = if arm == 7 {
                u.pow(f, 2).mul(f, &w).sub(f, &one)
            } else {
                u.pow(f, 12)
                    .sub(f, &Polynomial::constant(c))
                    .mul(f, &w)
                    .sub(f, &one)
            };
            assert_eq!(reduce_inverse(f, &inverse, arm, c), Polynomial::default());
            let normalized = reduce_inverse(f, &value, arm, c);
            assert!(normalized
                .0
                .keys()
                .all(|e| e[3] < (if arm == 7 { 2 } else { 12 }) || e[4] == 0));
            for x in 1..37 {
                let d = if arm == 7 {
                    x * x % 37
                } else {
                    (fpow(x, 12, 37) + 37 - c[0]) % 37
                };
                if d == 0 {
                    continue;
                }
                let point = [
                    [0; 4],
                    [0; 4],
                    [0; 4],
                    f.scalar(x),
                    f.scalar(fpow(d, 35, 37)),
                ];
                assert_eq!(value.eval(f, &point), normalized.eval(f, &point));
            }
        }
    }
    #[test]
    fn quartic_discriminant_matches_exact_root_differences() {
        let (a, b) = integer_quartic_discriminant();
        assert_eq!(a, b);
        assert_ne!(a.add(&integer::ZPoly::constant(1)), b);
    }
}
