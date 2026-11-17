//! Child of kummer08: explicit dual-base Frobenius circuits and exact checks.
use super::*;
#[path = "polynomial10.rs"]
pub(crate) mod polynomial10;
fn negate_fn(c: &Curve, p: &Fn) -> Fn {
    c.reduce(add(&p.a, &mul(c.f, &p.b, &c.h())), p.b.clone(), p.d.clone())
}
fn sigma_fn(f: &F, p: &Fn) -> Fn {
    Fn {
        a: p.a.iter().map(|v| f.sq(v)).collect(),
        b: p.b.iter().map(|v| f.sq(v)).collect(),
        d: p.d.iter().map(|v| f.sq(v)).collect(),
    }
}
fn signed(c: &Curve, p: &P, n: i64) -> P {
    let q = c.times(p, n.unsigned_abs());
    if n < 0 {
        c.neg(&q)
    } else {
        q
    }
}
fn vertical(c: &Curve, p: &P) -> Fn {
    let P::Affine { x, .. } = p else {
        return c.one();
    };
    Fn {
        a: vec![x.clone(), c.f.o()],
        b: vec![],
        d: vec![c.f.o()],
    }
}
fn combination(c: &Curve, points: &[P], weights: &[i64]) -> (P, Fn) {
    let mut sum = P::Infinity;
    let mut correction = c.one();
    let mut product = c.one();
    for (t, n) in points.iter().zip(weights) {
        let ft = c.miller(t).0;
        let factor = if *n < 0 { c.inverse_fn(&ft) } else { ft };
        product = c.mul(&product, &c.power_fn(&factor, n.unsigned_abs()));
        let step = if *n < 0 { c.neg(t) } else { t.clone() };
        for _ in 0..n.unsigned_abs() {
            let mut line = c.line(&sum, &step);
            if *n < 0 {
                line = c.mul(&line, &c.inverse_fn(&vertical(c, t)));
            }
            correction = c.mul(&correction, &line);
            sum = c.add(&sum, &step);
        }
    }
    assert!(c.equal_fn(
        &product,
        &c.mul(&c.miller(&sum).0, &c.power_fn(&correction, 19))
    ));
    (sum, correction)
}
fn h_fixed(c: &Curve, t: &P, r: &P) -> Fn {
    c.mul(
        &c.line(&c.add(t, &c.neg(r)), r),
        &c.inverse_fn(&vertical(c, r)),
    )
}
fn h_reverse(c: &Curve, t: &P, r: &P) -> Fn {
    let f = c.f;
    let P::Affine { x: rx, y: ry } = r else {
        panic!("generic fixed point")
    };
    let P::Affine { x: tx, .. } = t else {
        panic!("nonzero graph component")
    };
    let x = Fn {
        a: vec![f.z(), f.o()],
        b: vec![],
        d: vec![f.o()],
    };
    let y = Fn {
        a: vec![],
        b: vec![f.o()],
        d: vec![f.o()],
    };
    let (a, b) = c.translation(t);
    let a = negate_fn(c, &a);
    let b = negate_fn(c, &b); // T-P
    let lambda = c.mul(&c.sum_fn(&b, &y), &c.inverse_fn(&c.sum_fn(&a, &x)));
    let numerator = c.sum_fn(
        &c.sum_fn(&c.constant(ry), &y),
        &c.mul(&lambda, &c.sum_fn(&c.constant(rx), &x)),
    );
    let denominator = c.mul(&c.constant(&rx.add(tx)), &c.sum_fn(&c.constant(rx), &x));
    c.mul(&numerator, &c.inverse_fn(&denominator))
}
fn value(c: &Curve, p: &Fn, x: &P) -> E {
    c.eval(p, x).expect("declared Frobenius chart pole")
}
fn beta(c: &Curve, p: &[P]) -> Vec<P> {
    let mut q = vec![c.neg(&p[3])];
    for x in &p[..3] {
        q.push(c.add(x, &c.neg(&p[3])))
    }
    q.push(p[4].clone());
    q
}
fn eval_pair(c: &Curve, p: &Fn, x: &Fn, y: &Fn) -> Fn {
    let a = c.evaluate_poly_fn(&p.a, x);
    let b = c.evaluate_poly_fn(&p.b, x);
    let d = c.evaluate_poly_fn(&p.d, x);
    c.mul(&c.sum_fn(&a, &c.mul(&b, y)), &c.inverse_fn(&d))
}
fn const_pair(c: &Curve, p: &P) -> (Fn, Fn) {
    let P::Affine { x, y } = p else {
        panic!("affine fixture")
    };
    (c.constant(x), c.constant(y))
}
struct Lift {
    t: Vec<Vec<P>>,
    functions: Vec<Vec<Fn>>,
    sigma: Vec<Vec<Fn>>,
    classes: Vec<Vec<Fn>>,
    sums: Vec<Fn>,
    trace: i64,
}
fn curves<'a>(f: &'a F, t: i64) -> [Curve<'a>; 2] {
    [
        Curve {
            f,
            ss: false,
            c: if t == 1 { 1 } else { 0 },
        },
        Curve {
            f,
            ss: true,
            c: if t == 1 { 1 } else { 0 },
        },
    ]
}
fn h_value(c: &[Curve; 2], l: &Lift, i: usize, p: &[P]) -> E {
    let f = c[0].f;
    let mut z = value(&c[0], &l.sums[i], &p[3]);
    for j in 1..4 {
        z = f.mul(
            &z,
            &value(
                &c[0],
                &h_fixed(&c[0], &l.t[i][j], &c[0].neg(&p[3])),
                &p[j - 1],
            ),
        );
    }
    for j in 0..5 {
        z = f.mul(
            &z,
            &f.inv(&value(&c[usize::from(j == 4)], &l.classes[i][j], &p[j])),
        );
    }
    z
}
fn f_value(c: &[Curve; 2], fs: &[Fn], p: &[P]) -> E {
    (0..5).fold(c[0].f.o(), |z, j| {
        c[0].f
            .mul(&z, &value(&c[usize::from(j == 4)], &fs[j], &p[j]))
    })
}
fn h_slice(c: &[Curve; 2], l: &Lift, i: usize, variable: usize, p: &[P]) -> Fn {
    let cv = &c[usize::from(variable == 4)];
    let lift = |fun: &Fn, j: usize| {
        if variable == j {
            fun.clone()
        } else {
            cv.constant(&value(&c[usize::from(j == 4)], fun, &p[j]))
        }
    };
    let mut z = lift(&l.sums[i], 3);
    for j in 1..4 {
        let h = if variable == j - 1 {
            h_fixed(&c[0], &l.t[i][j], &c[0].neg(&p[3]))
        } else if variable == 3 {
            negate_fn(&c[0], &h_reverse(&c[0], &l.t[i][j], &p[j - 1]))
        } else {
            cv.constant(&value(
                &c[0],
                &h_fixed(&c[0], &l.t[i][j], &c[0].neg(&p[3])),
                &p[j - 1],
            ))
        };
        z = cv.mul(&z, &h);
    }
    for j in 0..5 {
        z = cv.mul(&z, &cv.inverse_fn(&lift(&l.classes[i][j], j)));
    }
    z
}
fn make_fixture(c: &[Curve; 2], seed: u64) -> Vec<P> {
    let mut p = vec![];
    for j in 0..5 {
        let cc = &c[usize::from(j == 4)];
        let point = (seed * 131 + j as u64 * 17 + 2..4096)
            .filter_map(|x| cc.point(&cc.f.e(x)))
            .map(|x| cc.times(&x, 19))
            .find(|x| cc.times(x, 19) != P::Infinity)
            .expect("generic point budget");
        p.push(point);
    }
    for n in 0..5 {
        let pp = (0..n).fold(p.clone(), |p, _| beta(&c[0], &p));
        for q in &pp[..4] {
            assert_ne!(c[0].times(q, 19), P::Infinity);
        }
        for j in 0..3 {
            for sign in [false, true] {
                let q = if sign {
                    c[0].neg(&pp[3])
                } else {
                    pp[3].clone()
                };
                assert_ne!(c[0].times(&c[0].add(&pp[j], &q), 19), P::Infinity);
            }
        }
    }
    p
}
fn root19(f: &F, a: &E) -> E {
    use num_bigint::BigUint;
    use num_traits::{One, Zero};
    let m = ((BigUint::one() << 360) - BigUint::one()) / 19u64;
    assert_eq!(f.pow(a, &m), f.o());
    let k = (0..19u64)
        .find(|k| (BigUint::one() + &m * *k) % 19u64 == BigUint::zero())
        .unwrap();
    let z = f.pow(a, &((BigUint::one() + m * k) / 19u64));
    assert_eq!(f.powu(&z, 19), *a);
    z
}
pub fn run(dir: &Path) {
    let input = round04::read(&dir.join("q6-intrinsic-Kummer-charts.json"));
    let oldglue = round04::read(&dir.join("q6c-global-chart-gluing.json"));
    let f = F::new(360, 45);
    let mut records = vec![];
    let mut univariate = 0;
    let mut slices = 0;
    let mut sigma_checks = 0;
    let mut iteration_checks = 0;
    let mut transition_checks = 0;
    for (ri, row) in input["records"].as_array().unwrap().iter().enumerate() {
        let trace = row["target_trace"].as_i64().unwrap();
        let c = curves(&f, trace);
        let mut ts = vec![];
        let mut fs = vec![];
        let mut sig = vec![];
        for gen in row["generators"].as_array().unwrap() {
            let mut t = vec![];
            let mut funcs = vec![];
            let mut ss = vec![];
            for (j, factor) in gen["factors"].as_array().unwrap().iter().enumerate() {
                let p = dp(&f, &factor["torsion_point"]);
                let fun = parsed(&f, &factor["function"]);
                let sigma = sigma_fn(&f, &fun);
                let ff = c[usize::from(j == 4)].miller(&f.frobp(&p, 1)).0;
                assert!(c[usize::from(j == 4)].equal_fn(&sigma, &ff));
                t.push(p);
                funcs.push(fun);
                ss.push(sigma);
                sigma_checks += 1;
            }
            ts.push(t);
            fs.push(funcs);
            sig.push(ss);
        }
        // D*sigma(T)=T*M for the actual graph, with generators(Frob S,S).
        for i in 0..2 {
            let sigma: Vec<_> = ts[i].iter().map(|p| f.frobp(p, 1)).collect();
            let mut dest =
                vec![c[0].neg(&sigma[..4].iter().fold(P::Infinity, |z, p| c[0].add(&z, p)))];
            dest.extend(sigma[..3].iter().cloned());
            dest.push(sigma[4].clone());
            for j in 0..5 {
                let cc = &c[usize::from(j == 4)];
                let expected = if i == 0 {
                    cc.add(
                        &signed(cc, &ts[0][j], 2 * trace),
                        &signed(cc, &ts[1][j], -2),
                    )
                } else {
                    ts[0][j].clone()
                };
                assert_eq!(dest[j], expected);
            }
        }
        let mut classes = vec![];
        let mut sums = vec![];
        for i in 0..2 {
            let weights = if i == 0 { [0, 1] } else { [9, trace] };
            let mut cs = vec![];
            let negpoints: Vec<_> = ts[i][..4].iter().map(|p| c[0].neg(p)).collect();
            let (sum, gs) = combination(&c[0], &negpoints, &[1, 1, 1, 1]);
            univariate += 1;
            for j in 0..5 {
                let cc = &c[usize::from(j == 4)];
                let points: Vec<_> = ts.iter().map(|t| f.frobp(&t[j], 1)).collect();
                let (w, g) = combination(cc, &points, &weights);
                let expected = if j < 3 {
                    ts[i][j + 1].clone()
                } else if j == 3 {
                    sum.clone()
                } else {
                    ts[i][4].clone()
                };
                assert_eq!(w, expected, "dual-base Picard class");
                cs.push(g);
                univariate += 1;
            }
            classes.push(cs);
            sums.push(gs);
        }
        let l = Lift {
            t: ts,
            functions: fs,
            sigma: sig,
            classes,
            sums,
            trace,
        };
        let mut fixtures = vec![];
        let mut slice_records = vec![];
        let mut cocycles = vec![];
        for fixture in 0..2 {
            let p = make_fixture(&c, fixture + 1);
            let bp = beta(&c[0], &p);
            for i in 0..2 {
                for j in 1..4 {
                    assert!(
                        c[0].equal_fn(
                            &h_fixed(&c[0], &l.t[i][j], &p[j - 1]),
                            &h_reverse(&c[0], &l.t[i][j], &p[j - 1])
                        ),
                        "Miller addition correction symmetry"
                    );
                }
            }
            for variable in 0..5 {
                let cv = &c[usize::from(variable == 4)];
                let x = Fn {
                    a: vec![f.z(), f.o()],
                    b: vec![],
                    d: vec![f.o()],
                };
                let y = Fn {
                    a: vec![],
                    b: vec![f.o()],
                    d: vec![f.o()],
                };
                let mut images: Vec<_> = bp.iter().map(|p| const_pair(cv, p)).collect();
                if variable < 3 {
                    images[variable + 1] = cv.translation(&c[0].neg(&p[3]));
                } else if variable == 3 {
                    images[0] = (x.clone(), negate_fn(cv, &y));
                    for j in 1..4 {
                        let (a, b) = cv.translation(&p[j - 1]);
                        images[j] = (negate_fn(cv, &a), negate_fn(cv, &b));
                    }
                } else {
                    images[4] = (x.clone(), y.clone());
                }
                for i in 0..2 {
                    let mut lhs = cv.one();
                    for j in 0..5 {
                        lhs = cv.mul(
                            &lhs,
                            &eval_pair(cv, &l.functions[i][j], &images[j].0, &images[j].1),
                        );
                    }
                    let mut source = vec![];
                    for k in 0..2 {
                        let z = (0..5).fold(cv.one(), |z, j| {
                            cv.mul(
                                &z,
                                &if j == variable {
                                    l.sigma[k][j].clone()
                                } else {
                                    cv.constant(&value(
                                        &c[usize::from(j == 4)],
                                        &l.sigma[k][j],
                                        &p[j],
                                    ))
                                },
                            )
                        });
                        source.push(z);
                    }
                    let h = h_slice(&c, &l, i, variable, &p);
                    let rhs = if i == 0 {
                        cv.mul(&source[1], &cv.power_fn(&h, 19))
                    } else {
                        let a = cv.power_fn(&source[0], 9);
                        let b = if trace < 0 {
                            cv.inverse_fn(&source[1])
                        } else {
                            source[1].clone()
                        };
                        cv.mul(&cv.mul(&a, &b), &cv.power_fn(&h, 19))
                    };
                    assert!(cv.equal_fn(&lhs, &rhs), "full Frobenius identity on slice");
                    slice_records.push(json!({"fixture":fixture,"variable":variable,"root":i,"H":fn_json(&h),"exact_identity":true}));
                    slices += 1;
                }
            }
            let z0 = [
                root19(&f, &f_value(&c, &l.functions[0], &p)),
                root19(&f, &f_value(&c, &l.functions[1], &p)),
            ];
            let mut pp = p.clone();
            let mut zz = z0.clone();
            for step in 0..360 {
                let sp: Vec<_> = pp.iter().map(|p| f.frobp(p, 1)).collect();
                let h = [h_value(&c, &l, 0, &sp), h_value(&c, &l, 1, &sp)];
                let nz0 = f.mul(&f.sq(&zz[1]), &h[0]);
                let mut nz1 = f.mul(
                    &f.powu(&f.sq(&zz[0]), 9),
                    &if trace < 0 {
                        f.inv(&f.sq(&zz[1]))
                    } else {
                        f.sq(&zz[1])
                    },
                );
                nz1 = f.mul(&nz1, &h[1]);
                pp = beta(&c[0], &sp);
                zz = [nz0, nz1];
                for i in 0..2 {
                    assert_eq!(
                        f.powu(&zz[i], 19),
                        f_value(&c, &l.functions[i], &pp),
                        "radicand after Frobenius step{step}"
                    );
                }
                iteration_checks += 1;
            }
            assert_eq!(pp, p);
            assert_eq!(zz, z0, "360-step deck cocycle");
            cocycles.push(json!({"fixture":fixture,"steps":360,"base_returns":true,"roots_return":true,"deck_phase":["1","1"]}));
            // Every old transition factor commutes with coefficient Frobenius.
            for rec in oldglue["records"][ri]["transition_factors"]
                .as_array()
                .unwrap()
            {
                let j = rec["factor"].as_u64().unwrap() as usize;
                let cc = &c[usize::from(j == 4)];
                let rho = parsed(&f, &rec["transition_from_zero"]);
                let sigma = sigma_fn(&f, &rho);
                assert_eq!(
                    value(cc, &sigma, &f.frobp(&p[j], 1)),
                    f.sq(&value(cc, &rho, &p[j]))
                );
                transition_checks += 1;
            }
            fixtures.push(json!({"point":p.iter().map(pj).collect::<Vec<_>>(),"root_coordinates":z0.iter().map(ej).collect::<Vec<_>>()}));
        }
        records.push(json!({"graph_record":ri,"trace":trace,"graph_Frobenius":[[2*trace,1],[-2,0]],"signed_inverse_mod19":[[0,9],[1,trace]],"base_action":"(-sigma P3,sigma P0-sigma P3,sigma P1-sigma P3,sigma P2-sigma P3,sigma P4)","root_action":"z0'=sigma(z1)*H0;z1'=sigma(z0)^9*sigma(z1)^trace*H1","class_corrections":l.classes.iter().map(|a|a.iter().map(fn_json).collect::<Vec<_>>()).collect::<Vec<_>>(),"sum_corrections":l.sums.iter().map(fn_json).collect::<Vec<_>>(),"mixed_factors":(0..2).map(|i|(1..4).map(|j|json!({"T":pj(&l.t[i][j]),"X_factor":j-1,"Y_factor":3,"Y_negated":true})).collect::<Vec<_>>()).collect::<Vec<_>>(),"H_formula":"sum_correction_i(P3)*product_(j=1..3) h_Tij(P_(j-1),-P3)/product_(j=0..4) class_correction_ij(Pj)","h_formula":"g_(T-Y,Y)(X)/(x(X)+x(Y));normalized Miller line quotient","fixtures":fixtures,"exact_slices":slice_records,"cocycles":cocycles,"chart_action":"z_i,l'=product_j sigma(z_j,k)^N_ji * H_i(sigma P)*rho_i,l(B sigma P)/product_j sigma(rho_j,k(P))^N_ji;rho_i,k is product of the five retained factor transitions","scope":"factored explicit semilinear coordinate circuits;global mixed identity proved by written divisor/normalization argument with exact univariate and slice certificates;effective invariant F2 equations remain open"}));
        println!("Q7 graph{ri}:Frobenius circuits,20 exact slices,two360-step cocycles pass");
    }
    assert_eq!(univariate, 48);
    assert_eq!(slices, 80);
    assert_eq!(sigma_checks, 40);
    assert_eq!(iteration_checks, 2880);
    assert_eq!(transition_checks, 400);
    round02::save(
        dir,
        "q7-Frobenius-coordinate-circuits.json",
        &json!({"schema":"koblitz.round09/q7-v1","records":records,"univariate_class_identities":univariate,"exact_slice_identities":slices,"sigma_Miller_identities":sigma_checks,"semilinear_iterations":iteration_checks,"old_transition_sigma_checks":transition_checks,"F2_invariant_equations":false,"scope":"explicit dual-base Frobenius lifts and cocycle checks,not yet an effective F2 polynomial model"}),
    );
}

pub fn verify(dir: &Path) {
    let data = round04::read(&dir.join("q7-Frobenius-coordinate-circuits.json"));
    let base = round04::read(&dir.join("q6-intrinsic-Kummer-charts.json"));
    let gluing = round04::read(&dir.join("q6c-global-chart-gluing.json"));
    let f = F::new(360, 45);
    let mut identities = 0;
    let mut chart_checks = 0;
    let mut iterations = 0;
    for (ri, row) in data["records"].as_array().unwrap().iter().enumerate() {
        let trace = row["trace"].as_i64().unwrap();
        let c = curves(&f, trace);
        let t: Vec<Vec<_>> = base["records"][ri]["generators"]
            .as_array()
            .unwrap()
            .iter()
            .map(|g| {
                g["factors"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|r| dp(&f, &r["torsion_point"]))
                    .collect()
            })
            .collect();
        let functions: Vec<Vec<_>> = base["records"][ri]["generators"]
            .as_array()
            .unwrap()
            .iter()
            .map(|g| {
                g["factors"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|r| parsed(&f, &r["function"]))
                    .collect()
            })
            .collect();
        let sigma = functions
            .iter()
            .map(|a| a.iter().map(|p| sigma_fn(&f, p)).collect())
            .collect();
        let classes = row["class_corrections"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| {
                a.as_array()
                    .unwrap()
                    .iter()
                    .map(|r| parsed(&f, r))
                    .collect()
            })
            .collect();
        let sums = row["sum_corrections"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| parsed(&f, r))
            .collect();
        let l = Lift {
            t,
            functions,
            sigma,
            classes,
            sums,
            trace,
        };
        for i in 0..2 {
            let sum = l.t[i][..4]
                .iter()
                .fold(P::Infinity, |z, p| c[0].add(&z, &c[0].neg(p)));
            let prod = l.t[i][..4].iter().fold(c[0].one(), |z, p| {
                c[0].mul(&z, &c[0].miller(&c[0].neg(p)).0)
            });
            assert!(c[0].equal_fn(
                &prod,
                &c[0].mul(&c[0].miller(&sum).0, &c[0].power_fn(&l.sums[i], 19))
            ));
            identities += 1;
            for j in 0..5 {
                let cc = &c[usize::from(j == 4)];
                let w = if j < 3 {
                    l.t[i][j + 1].clone()
                } else if j == 3 {
                    sum.clone()
                } else {
                    l.t[i][4].clone()
                };
                let prod = if i == 0 {
                    l.sigma[1][j].clone()
                } else {
                    cc.mul(
                        &cc.power_fn(&l.sigma[0][j], 9),
                        &if trace < 0 {
                            cc.inverse_fn(&l.sigma[1][j])
                        } else {
                            l.sigma[1][j].clone()
                        },
                    )
                };
                assert!(cc.equal_fn(
                    &prod,
                    &cc.mul(&cc.miller(&w).0, &cc.power_fn(&l.classes[i][j], 19))
                ));
                identities += 1;
            }
        }
        let rho = |i: usize, k: usize, p: &[P]| {
            if k == 0 {
                return f.o();
            }
            gluing["records"][ri]["transition_factors"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|r| r["generator"] == i && r["chart"] == k)
                .fold(f.o(), |z, r| {
                    let j = r["factor"].as_u64().unwrap() as usize;
                    f.mul(
                        &z,
                        &value(
                            &c[usize::from(j == 4)],
                            &parsed(&f, &r["transition_from_zero"]),
                            &p[j],
                        ),
                    )
                })
        };
        let monomial = |z: &[E; 2], i: usize| {
            if i == 0 {
                z[1].clone()
            } else {
                f.mul(
                    &f.powu(&z[0], 9),
                    &if trace < 0 {
                        f.inv(&z[1])
                    } else {
                        z[1].clone()
                    },
                )
            }
        };
        for fixture in row["fixtures"].as_array().unwrap() {
            let p: Vec<_> = fixture["point"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| dp(&f, p))
                .collect();
            let z: Vec<_> = fixture["root_coordinates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| crate::reverse06::decode(&f, v))
                .collect();
            let original = [z[0].clone(), z[1].clone()];
            let sp: Vec<_> = p.iter().map(|p| f.frobp(p, 1)).collect();
            let tp = beta(&c[0], &sp);
            let hs = [h_value(&c, &l, 0, &sp), h_value(&c, &l, 1, &sp)];
            let zs = [f.sq(&z[0]), f.sq(&z[1])];
            let out = [
                f.mul(&monomial(&zs, 0), &hs[0]),
                f.mul(&monomial(&zs, 1), &hs[1]),
            ];
            for k in 0..6 {
                let sr = [rho(0, k, &p), rho(1, k, &p)];
                let sr2 = [f.sq(&sr[0]), f.sq(&sr[1])];
                let chart = [f.sq(&f.mul(&z[0], &sr[0])), f.sq(&f.mul(&z[1], &sr[1]))];
                for ll in 0..6 {
                    for i in 0..2 {
                        let rr = rho(i, ll, &tp);
                        let predicted = f.mul(
                            &f.mul(&f.mul(&monomial(&chart, i), &hs[i]), &rr),
                            &f.inv(&monomial(&sr2, i)),
                        );
                        assert_eq!(predicted, f.mul(&out[i], &rr));
                        chart_checks += 1;
                    }
                }
            }
            let (mut pp, mut zz) = (p.clone(), original.clone());
            for _ in 0..360 {
                let sp: Vec<_> = pp.iter().map(|p| f.frobp(p, 1)).collect();
                let zs = [f.sq(&zz[0]), f.sq(&zz[1])];
                zz = [
                    f.mul(&monomial(&zs, 0), &h_value(&c, &l, 0, &sp)),
                    f.mul(&monomial(&zs, 1), &h_value(&c, &l, 1, &sp)),
                ];
                pp = beta(&c[0], &sp);
                iterations += 1;
            }
            assert_eq!(pp, p);
            assert_eq!(zz, original);
        }
    }
    assert_eq!(identities, 48);
    assert_eq!(chart_checks, 576);
    assert_eq!(iterations, 2880);
    round02::save(
        dir,
        "q7b-parsed-descent-chart-check.json",
        &json!({"schema":"koblitz.round09/q7b-v1","parsed_univariate_identities":identities,"chart_pair_checks":chart_checks,"parsed_semilinear_iterations":iterations,"scope":"parse actual correction functions;exact univariate checks;every36 chart pair on both fixtures/roots;independent recomposition of360-step cocycles with shared primitives"}),
    );
    println!("Q7b:48 parsed identities,576 chart squares,2880 parsed Frobenius steps checked");
}

fn parsed_lift(f: &F, data: &Value, base: &Value) -> Lift {
    let trace = data["trace"].as_i64().unwrap();
    let t = base["generators"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| {
            g["factors"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| dp(f, &r["torsion_point"]))
                .collect()
        })
        .collect();
    let functions: Vec<Vec<_>> = base["generators"]
        .as_array()
        .unwrap()
        .iter()
        .map(|g| {
            g["factors"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| parsed(f, &r["function"]))
                .collect()
        })
        .collect();
    let sigma = functions
        .iter()
        .map(|r| r.iter().map(|p| sigma_fn(f, p)).collect())
        .collect();
    let classes = data["class_corrections"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r.as_array().unwrap().iter().map(|p| parsed(f, p)).collect())
        .collect();
    let sums = data["sum_corrections"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| parsed(f, p))
        .collect();
    Lift {
        trace,
        t,
        functions,
        sigma,
        classes,
        sums,
    }
}
fn field_trace(f: &F, x: &E) -> bool {
    let mut z = f.z();
    let mut a = x.clone();
    for _ in 0..360 {
        z = z.add(&a);
        a = f.sq(&a);
    }
    assert_eq!(a, *x);
    assert!(z == f.z() || z == f.o());
    z == f.o()
}
pub fn traces(dir: &Path) {
    use num_bigint::BigUint;
    use num_traits::{One, Zero};
    let f = F::new(360, 45);
    assert!(f.irreducible());
    let n = 360usize;
    let mut powers = vec![f.o()];
    for _ in 1..719 {
        powers.push(f.mul(powers.last().unwrap(), &f.e(2)));
    }
    let tr: Vec<_> = powers.iter().map(|p| field_trace(&f, p)).collect();
    let mut rows = vec![];
    for i in 0..n {
        let mut row = BigUint::one() << (n + i);
        for j in 0..n {
            if tr[i + j] {
                row |= BigUint::one() << j;
            }
        }
        rows.push(row);
    }
    for j in 0..n {
        let p = (j..n)
            .find(|i| rows[*i].bit(j as u64))
            .expect("nondegenerate trace pairing");
        rows.swap(j, p);
        for i in 0..n {
            if i != j && rows[i].bit(j as u64) {
                let v = rows[j].clone();
                rows[i] ^= v;
            }
        }
    }
    let dual: Vec<_> = (0..n)
        .map(|j| {
            let mut bits = BigUint::zero();
            for i in 0..n {
                if rows[i].bit((n + j) as u64) {
                    bits |= BigUint::one() << i;
                }
            }
            E::from_biguint(&bits, 360)
        })
        .collect();
    let mut mask = BigUint::zero();
    for (i, t) in tr[..n].iter().enumerate() {
        if *t {
            mask |= BigUint::one() << i;
        }
    }
    let fast = |x: &E| {
        (x.to_biguint() & &mask)
            .to_u64_digits()
            .iter()
            .map(|w| w.count_ones())
            .sum::<u32>()
            % 2
            == 1
    };
    for i in 0..n {
        for j in 0..n {
            assert_eq!(fast(&f.mul(&powers[i], &dual[j])), i == j);
        }
    }
    for i in 0..64 {
        let x = f.mul(&powers[i * 5 % n], &dual[(i * 17 + 3) % n]);
        assert_eq!(fast(&x), field_trace(&f, &x));
    }
    let mut conjugates = powers[..n].to_vec();
    let mut idempotents = vec![];
    for j in 0..n {
        let z = (0..n).fold(f.z(), |z, i| z.add(&f.mul(&dual[i], &conjugates[i])));
        assert_eq!(z, if j == 0 { f.o() } else { f.z() });
        idempotents.push(ej(&z));
        for v in &mut conjugates {
            *v = f.sq(v);
        }
    }
    let descent = round04::read(&dir.join("q7-Frobenius-coordinate-circuits.json"));
    let base = round04::read(&dir.join("q6-intrinsic-Kummer-charts.json"));
    let mut tests = vec![];
    let mut equivariance = 0;
    let mut steps = 0;
    for (ri, row) in descent["records"].as_array().unwrap().iter().enumerate() {
        let l = parsed_lift(&f, row, &base["records"][ri]);
        let c = curves(&f, l.trace);
        let fixture = &row["fixtures"][0];
        let mut p: Vec<_> = fixture["point"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| dp(&f, p))
            .collect();
        let roots: Vec<_> = fixture["root_coordinates"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| crate::reverse06::decode(&f, x))
            .collect();
        let mut z = [roots[0].clone(), roots[1].clone()];
        let mut orbit = vec![];
        for _ in 0..360 {
            let P::Affine { x, y } = &p[0] else {
                unreachable!()
            };
            orbit.push(vec![x.clone(), y.clone(), z[0].clone(), z[1].clone()]);
            let sp: Vec<_> = p.iter().map(|p| f.frobp(p, 1)).collect();
            let mut b = f.mul(
                &f.powu(&f.sq(&z[0]), 9),
                &if l.trace < 0 {
                    f.inv(&f.sq(&z[1]))
                } else {
                    f.sq(&z[1])
                },
            );
            b = f.mul(&b, &h_value(&c, &l, 1, &sp));
            z = [f.mul(&f.sq(&z[1]), &h_value(&c, &l, 0, &sp)), b];
            p = beta(&c[0], &sp);
            steps += 1;
        }
        let mut coeff = powers[..n].to_vec();
        let mut values = vec![vec![f.z(); n]; 4];
        let mut next = vec![vec![f.z(); 8]; 4];
        for j in 0..n {
            for k in 0..4 {
                let v = f.frob(&orbit[(n - j) % n][k], j as u32);
                let w = f.frob(&orbit[(n + 1 - j) % n][k], j as u32);
                for i in 0..n {
                    values[k][i] = values[k][i].add(&f.mul(&coeff[i], &v));
                    if i < 8 {
                        next[k][i] = next[k][i].add(&f.mul(&coeff[i], &w));
                    }
                }
            }
            for x in &mut coeff {
                *x = f.sq(x);
            }
        }
        for k in 0..4 {
            let recovered = (0..n).fold(f.z(), |z, i| z.add(&f.mul(&dual[i], &values[k][i])));
            assert_eq!(recovered, orbit[0][k]);
            for i in 0..8 {
                assert_eq!(next[k][i], f.sq(&values[k][i]));
                equivariance += 1;
            }
        }
        tests.push(json!({"graph_record":ri,"sampled_coordinates":["x0","y0","z0","z1"],"invariant_values":values.iter().map(|row|row.iter().map(ej).collect::<Vec<_>>()).collect::<Vec<_>>(),"coordinate_recovery":true}));
        println!("Q7c graph{ri}:1440 invariant values,four recoveries,32 equivariance checks");
    }
    assert_eq!(steps, 1440);
    assert_eq!(equivariance, 128);
    round02::save(
        dir,
        "q7c-invariant-rational-generators.json",
        &json!({"schema":"koblitz.round09/q7c-v1","basis":"1,z,...,z^359;z^360+z^5+z^3+z^2+1=0","dual_basis":dual.iter().map(ej).collect::<Vec<_>>(),"trace_pairing_entries_checked":129600,"separability_idempotents":idempotents,"direct_trace_controls":64,"coordinate_functions":["x0","y0","x1","y1","x2","y2","x3","y3","x4","y4","z0","z1"],"invariant_rational_generators":4320,"generator_formula":"I_(f,k)=sum_(j=0..359) rho^j(b_k*f);rho^j(f)(P)=sigma^j(f(alpha^(-j)P))","recovery_formula":"f=sum_(k=0..359) dual_basis[k]*I_(f,k)","sampled_values":tests,"sampled_values_count":5760,"coordinate_recoveries":16,"equivariance_checks":equivariance,"additional_point_steps":steps,"effective_F2_polynomial_relations":false,"scope":"explicit invariant rational generating circuits and exact trace-dual recovery;not yet polynomial equations,saturation or a closed embedding"}),
    );
}
