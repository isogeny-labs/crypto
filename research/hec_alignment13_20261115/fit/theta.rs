struct Sample {
    n: u64,
    base: [Vec<E>; 2],
    minus: [Vec<E>; 2],
    shifted: [Vec<Vec<E>>; 2],
}
fn quotient(a: E, b: E, f: &Field) -> E {
    assert_ne!(b, Z);
    f.mul(a, f.inv(b))
}
fn load_inputs(args: &[String]) -> (Curve, jac::Curve, jac::Div, [Vec<Div>; 2]) {
    let input = fs::read_to_string(&args[2]).unwrap();
    let ns: Vec<u64> = input
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(&ns[..3], &[3, 127, 0]);
    assert_eq!(ns[4], 1028893);
    let mut at = 5;
    let mut ps = Vec::new();
    for _ in 0..3 {
        let n = ns[at] as usize;
        at += 1;
        ps.push(Poly::from_coeffs(&ns[at..at + n]));
        at += n
    }
    let refc = jac::Curve::new(Fp::new(P), ps[0]);
    let d1 = jac::Div { u: ps[1], v: ps[2] };
    assert!(refc.is_valid(&d1));
    assert!(refc.is_zero(&refc.mul(&d1, 1028893)));
    let text = fs::read_to_string(&args[1]).unwrap();
    assert!(text.lines().any(|l| l == "FIELD 127 13 10"));
    let field = Field {
        c: 10,
        q: (P as u128).pow(13),
    };
    assert!(field.irreducible());
    let c = Curve {
        field,
        f: Pol::embed(&refc.f),
    };
    let mut ks = [Vec::new(), Vec::new()];
    for line in text.lines().filter(|l| l.starts_with("POINT ")) {
        let ns: Vec<u64> = line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();
        let k = ns[0] as usize;
        assert!(k < 2);
        assert_eq!(ns[1] as usize, ks[k].len());
        let mut at = 2;
        let mut ps = Vec::new();
        for _ in 0..2 {
            let n = ns[at] as usize;
            assert!(n <= 4);
            at += 1;
            let mut v = Vec::new();
            for _ in 0..n {
                let a = std::array::from_fn(|i| ns[at + i]);
                at += 13;
                assert!(a.iter().all(|&x| x < P));
                v.push(a)
            }
            let pol = Pol::new(v);
            assert_eq!(pol.0.len(), n);
            ps.push(pol)
        }
        assert_eq!(at, ns.len());
        let p = Div {
            u: ps[0].clone(),
            v: ps[1].clone(),
        };
        assert!(c.valid(&p));
        assert!(c.is_zero(&c.small(&p, 3)));
        ks[k].push(p);
    }
    assert!(ks.iter().all(|v| v.len() == 27));
    (c, refc, d1, ks)
}
fn values(c: &Curve, ks: &[Div], hs: &[H], neg: &[usize], x: &Div) -> Result<Vec<E>, String> {
    let mut out = Vec::new();
    for i in 0..27 {
        let j = neg[i];
        let v =
            evaluate(c, &ks[j], &hs[j], x).ok_or_else(|| format!("unsupported_function_{i}"))?;
        if v == Z {
            return Err(format!("zero_function_{i}"));
        }
        out.push(v)
    }
    Ok(out)
}
fn sample(
    c: &Curve,
    ks: &[Vec<Div>; 2],
    hs: &[Vec<H>; 2],
    neg: &[Vec<usize>; 2],
    n: u64,
    x: &Div,
) -> Result<Sample, String> {
    let mut s = Sample {
        n,
        base: [Vec::new(), Vec::new()],
        minus: [Vec::new(), Vec::new()],
        shifted: [Vec::new(), Vec::new()],
    };
    for k in 0..2 {
        s.base[k] =
            values(c, &ks[k], &hs[k], &neg[k], x).map_err(|e| format!("kernel{k}_base_{e}"))?;
        s.minus[k] = values(c, &ks[k], &hs[k], &neg[k], &c.neg(x))
            .map_err(|e| format!("kernel{k}_minus_{e}"))?;
        for i in 0..27 {
            let y = c.add(x, &ks[k][i]);
            s.shifted[k].push(
                values(c, &ks[k], &hs[k], &neg[k], &y)
                    .map_err(|e| format!("kernel{k}_shift{i}_{e}"))?,
            )
        }
    }
    Ok(s)
}
pub fn run_theta() {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(600));
        eprintln!("CAP aggregate600seconds");
        std::process::exit(124)
    });
    let args: Vec<_> = std::env::args().collect();
    let (c, refc, d1, ks) = load_inputs(&args);
    let f = &c.field;
    let hs: [Vec<H>; 2] = std::array::from_fn(|k| {
        ks[k]
            .iter()
            .map(|p| construct(&c, p).expect("principal function failure"))
            .collect()
    });
    let indices: [HashMap<Div, usize>; 2] = std::array::from_fn(|k| {
        ks[k]
            .iter()
            .enumerate()
            .map(|(i, p)| (p.clone(), i))
            .collect()
    });
    assert!(indices.iter().all(|m| m.len() == 27));
    let zeros: [usize; 2] = std::array::from_fn(|k| indices[k][&c.zero()]);
    let neg: [Vec<usize>; 2] =
        std::array::from_fn(|k| ks[k].iter().map(|p| indices[k][&c.neg(p)]).collect());
    let add: [Vec<Vec<usize>>; 2] = std::array::from_fn(|k| {
        ks[k]
            .iter()
            .map(|p| ks[k].iter().map(|q| indices[k][&c.add(p, q)]).collect())
            .collect()
    });
    let mut samples = Vec::new();
    let mut rejected = 0;
    for n in 1..=32 {
        let d = refc.mul(&d1, n);
        let x = Div {
            u: Pol::embed(&d.u),
            v: Pol::embed(&d.v),
        };
        match sample(&c, &ks, &hs, &neg, n, &x) {
            Ok(s) => {
                println!("ATTEMPT n={n} regular=true");
                samples.push(s)
            }
            Err(e) => {
                println!("ATTEMPT n={n} regular=false reason={e}");
                rejected += 1
            }
        }
        if samples.len() == 4 {
            break;
        }
    }
    assert_eq!(samples.len(), 4, "CAP four regular samples not reached");
    let mut cocycles: [Vec<Vec<E>>; 2] = std::array::from_fn(|_| vec![vec![Z; 27]; 27]);
    let mut constant_checks = 0;
    for k in 0..2 {
        for i in 0..27 {
            for j in 0..27 {
                for (t, s) in samples.iter().enumerate() {
                    let v = quotient(
                        f.mul(s.base[k][i], s.shifted[k][i][j]),
                        s.base[k][add[k][i][j]],
                        f,
                    );
                    if t == 0 {
                        cocycles[k][i][j] = v
                    } else {
                        assert_eq!(
                            v, cocycles[k][i][j],
                            "nonconstant cocycle k{k} i{i} j{j} n{}",
                            s.n
                        );
                        constant_checks += 1
                    }
                }
            }
        }
    }
    let mut symmetric = 0;
    let mut associative = 0;
    for k in 0..2 {
        let cc = &cocycles[k];
        for i in 0..27 {
            assert_eq!(cc[zeros[k]][i], O);
            assert_eq!(cc[i][zeros[k]], O);
            for j in 0..27 {
                assert_eq!(cc[i][j], cc[j][i]);
                symmetric += 1;
                for l in 0..27 {
                    assert_eq!(
                        f.mul(cc[i][j], cc[add[k][i][j]][l]),
                        f.mul(cc[j][l], cc[i][add[k][j][l]])
                    );
                    associative += 1
                }
            }
        }
    }
    let mut alpha: [Vec<E>; 2] = std::array::from_fn(|_| vec![Z; 27]);
    let mut normality = 0;
    let mut symmetry = 0;
    for k in 0..2 {
        for (t, s) in samples.iter().enumerate() {
            let mut a = Vec::new();
            for i in 0..27 {
                let cp = cocycles[k][i][neg[k][i]];
                let a3 = f.inv(f.mul(cocycles[k][i][i], cp));
                let sp = quotient(s.base[k][neg[k][i]], s.minus[k][i], f);
                let a2 = quotient(sp, cp, f);
                let value = quotient(a3, a2, f);
                assert_eq!(f.pow(value, 2), a2);
                assert_eq!(f.pow(value, 3), a3);
                a.push(value);
            }
            assert_eq!(a[zeros[k]], O);
            if t == 0 {
                alpha[k] = a.clone()
            } else {
                assert_eq!(a, alpha[k])
            }
            for i in 0..27 {
                assert_eq!(
                    f.mul(a[i], s.minus[k][i]),
                    f.mul(a[neg[k][i]], s.base[k][neg[k][i]])
                );
                symmetry += 1;
                for j in 0..27 {
                    assert_eq!(f.mul(f.mul(a[i], a[j]), cocycles[k][i][j]), a[add[k][i][j]]);
                    normality += 1
                }
            }
        }
    }
    let mut frobenius = 0;
    for k in 0..2 {
        for i in 0..27 {
            let j = indices[k][&c.frob(&ks[k][i])];
            assert_eq!(f.frob(alpha[k][i]), alpha[k][j]);
            frobenius += 1;
            print!("ALPHA {k} {i}");
            for a in alpha[k][i] {
                print!(" {a}")
            }
            println!()
        }
    }
    let mut actions = 0;
    let mut theta_zeros = 0;
    let mut theta_nonzero = [0; 2];
    let mut corrupt_normality = 0;
    let mut corrupt_actions = 0;
    for k in 0..2 {
        let ni = (0..27).find(|&i| i != zeros[k]).unwrap();
        let mut bad = alpha[k].clone();
        bad[ni] = f.mul(base(2), bad[ni]);
        for i in 0..27 {
            for j in 0..27 {
                if f.mul(f.mul(bad[i], bad[j]), cocycles[k][i][j]) != bad[add[k][i][j]] {
                    corrupt_normality += 1
                }
            }
        }
        for s in &samples {
            let sum = |weights: &[E], v: &[E]| {
                weights
                    .iter()
                    .zip(v)
                    .fold(Z, |o, (&a, &v)| f.add(o, f.mul(a, v)))
            };
            let theta = sum(&alpha[k], &s.base[k]);
            assert_eq!(f.frob(theta), theta);
            if theta == Z {
                theta_zeros += 1
            } else {
                theta_nonzero[k] += 1
            }
            let tm = sum(&alpha[k], &s.minus[k]);
            assert_eq!(theta, tm);
            print!("THETA kernel={k} n={} value", s.n);
            for a in theta {
                print!(" {a}")
            }
            println!();
            for i in 0..27 {
                let ts = sum(&alpha[k], &s.shifted[k][i]);
                assert_eq!(f.mul(f.mul(alpha[k][i], s.base[k][i]), ts), theta);
                actions += 1;
                let bs = sum(&bad, &s.shifted[k][i]);
                let bt = sum(&bad, &s.base[k]);
                if f.mul(f.mul(bad[i], s.base[k][i]), bs) != bt {
                    corrupt_actions += 1
                }
            }
        }
    }
    assert!(theta_nonzero.iter().all(|&n| n > 0));
    assert!(corrupt_normality > 0 && corrupt_actions > 0);
    println!("TERMINAL kernels=2 regular_samples=4 rejected_multiples={rejected} cocycle_constancy={constant_checks} symmetric_pairs={symmetric} associative_triples={associative} normality_pairs={normality} symmetry_values={symmetry} frobenius_scalars={frobenius} section_actions={actions} theta_zeros={theta_zeros} nonzero_by_kernel={theta_nonzero:?} corrupt_normality_failures={corrupt_normality} corrupt_action_failures={corrupt_actions} pass=true");
}
