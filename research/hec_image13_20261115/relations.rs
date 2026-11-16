fn monomials(degree: u8) -> Vec<[u8; 8]> {
    fn rec(i: usize, d: u8, e: &mut [u8; 8], out: &mut Vec<[u8; 8]>) {
        if i == 7 {
            e[i] = d;
            out.push(*e);
            return;
        }
        for n in 0..=d {
            e[i] = n;
            rec(i + 1, d - n, e, out)
        }
    }
    let mut out = Vec::new();
    rec(0, degree, &mut [0; 8], &mut out);
    out
}
fn mono(e: &[u8; 8], x: &[u64; 8]) -> u64 {
    let mut out = 1;
    for i in 0..8 {
        for _ in 0..e[i] {
            out = out * x[i] % P
        }
    }
    out
}
fn row_at(ms: &[[u8; 8]], x: &[u64; 8]) -> Vec<u64> {
    ms.iter().map(|e| mono(e, x)).collect()
}
fn derivative_row(ms: &[[u8; 8]], x: &[u64; 8], j: usize) -> Vec<u64> {
    ms.iter()
        .map(|e| {
            if e[j] == 0 {
                return 0;
            }
            let mut ee = *e;
            ee[j] -= 1;
            (e[j] as u64) * mono(&ee, x) % P
        })
        .collect()
}
fn dot_fp(a: &[u64], b: &[u64]) -> u64 {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).fold(0, |s, (&x, &y)| (s + x * y) % P)
}
fn nullspace_fp(original: &[Vec<u64>], cols: usize) -> Vec<Vec<u64>> {
    assert!(original.iter().all(|r| r.len() == cols));
    let mut m = original.to_vec();
    let mut pivots = Vec::new();
    let fp = Fp::new(P);
    for j in 0..cols {
        let row = pivots.len();
        if row == m.len() {
            break;
        }
        let Some(i) = (row..m.len()).find(|&i| m[i][j] != 0) else {
            continue;
        };
        m.swap(i, row);
        let inv = fp.inv(m[row][j]);
        for k in j..cols {
            m[row][k] = m[row][k] * inv % P
        }
        for i in 0..m.len() {
            if i != row {
                let t = m[i][j];
                if t != 0 {
                    for k in j..cols {
                        m[i][k] = (m[i][k] + P - t * m[row][k] % P) % P
                    }
                }
            }
        }
        pivots.push(j)
    }
    let mut out = Vec::new();
    for free in 0..cols {
        if pivots.contains(&free) {
            continue;
        }
        let mut v = vec![0; cols];
        v[free] = 1;
        for (i, &p) in pivots.iter().enumerate() {
            v[p] = (P - m[i][free]) % P
        }
        for r in original {
            assert_eq!(dot_fp(r, &v), 0)
        }
        out.push(v)
    }
    out
}
fn partial_coefficients(q: &[u64], m4: &[[u8; 8]], m3: &[[u8; 8]], j: usize) -> Vec<u64> {
    let mut out = vec![0; m3.len()];
    for (&a, e) in q.iter().zip(m4) {
        if a == 0 || e[j] == 0 {
            continue;
        }
        let mut ee = *e;
        ee[j] -= 1;
        let i = m3.iter().position(|r| *r == ee).unwrap();
        out[i] = (out[i] + a * (e[j] as u64)) % P
    }
    out
}
fn controls_image(m2: &[[u8; 8]], m3: &[[u8; 8]], m4: &[[u8; 8]]) {
    let rational = |t: u64| {
        let mut x = [1; 8];
        for i in 1..8 {
            x[i] = x[i - 1] * t % P
        }
        x
    };
    for (ms, expected) in [(m2, 21), (m3, 98)] {
        let rows: Vec<_> = (0..50).map(|t| row_at(ms, &rational(t))).collect();
        let ns = nullspace_fp(&rows, ms.len());
        assert_eq!(ns.len(), expected);
        for t in 60..70 {
            for v in &ns {
                assert_eq!(dot_fp(v, &row_at(ms, &rational(t))), 0)
            }
        }
    }
    let mut square = vec![0; m4.len()];
    for (e, a) in [
        ([2, 2, 0, 0, 0, 0, 0, 0], 1),
        ([1, 1, 1, 1, 0, 0, 0, 0], P - 2),
        ([0, 0, 2, 2, 0, 0, 0, 0], 1),
    ] {
        square[m4.iter().position(|x| *x == e).unwrap()] = a
    }
    let badindex = m4
        .iter()
        .position(|x| *x == [4, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let mut bad = square.clone();
    bad[badindex] = 1;
    let mut failures = 0;
    for t in 0..20 {
        let b = t + 2;
        let d = t + 5;
        let x = [1, b * d % P, b, d, t, t * t % P, (t + 1) % P, (t + 9) % P];
        assert_eq!(dot_fp(&square, &row_at(m4, &x)), 0);
        for j in 0..8 {
            assert_eq!(dot_fp(&square, &derivative_row(m4, &x, j)), 0)
        }
        if dot_fp(&bad, &derivative_row(m4, &x, 0)) != 0 {
            failures += 1
        }
    }
    assert_eq!(failures, 20);
    println!("CONTROLS rational_normal_quadric_nullity=21 rational_normal_cubic_nullity=98 square_quadric_points=20 corruption_failures={failures} pass=true");
}
pub fn run_image() {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(600));
        eprintln!("CAP aggregate600seconds");
        std::process::exit(124)
    });
    let args: Vec<_> = std::env::args().collect();
    let (c, refc, d1, ks) = load_inputs(&args);
    let f = &c.field;
    let basis = fs::read_to_string(&args[4]).unwrap();
    for k in 0..2 {
        assert!(basis.lines().any(|l|l==format!("BASIS kernel={k} descriptors=[(0, 0), (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6)]")))
    }
    let text = fs::read_to_string(&args[3]).unwrap();
    let mut alpha = [Vec::new(), Vec::new()];
    for line in text.lines().filter(|s| s.starts_with("ALPHA ")) {
        let ns: Vec<u64> = line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(ns.len(), 15);
        let k = ns[0] as usize;
        assert!(k < 2);
        assert_eq!(ns[1] as usize, alpha[k].len());
        let a = std::array::from_fn(|i| ns[i + 2]);
        assert!(a.iter().all(|&x| x < P));
        assert_ne!(a, Z);
        alpha[k].push(a)
    }
    assert!(alpha.iter().all(|a| a.len() == 27 && a[0] == O));
    let hs: [Vec<H>; 2] =
        std::array::from_fn(|k| ks[k].iter().map(|p| construct(&c, p).unwrap()).collect());
    let ix: [HashMap<_, _>; 2] = std::array::from_fn(|k| {
        ks[k]
            .iter()
            .enumerate()
            .map(|(i, p)| (p.clone(), i))
            .collect()
    });
    let neg: [Vec<usize>; 2] =
        std::array::from_fn(|k| ks[k].iter().map(|p| ix[k][&c.neg(p)]).collect());
    let fr: [Vec<usize>; 2] =
        std::array::from_fn(|k| ks[k].iter().map(|p| ix[k][&c.frob(p)]).collect());
    for k in 0..2 {
        for i in 0..27 {
            assert_eq!(neg[k][i], negchar(i));
            assert_eq!(f.frob(alpha[k][i]), alpha[k][fr[k][i]])
        }
    }
    let zeta = base(19);
    assert_eq!(f.pow(zeta, 3), O);
    assert_ne!(zeta, O);
    let chars: Vec<Vec<E>> = (0..27)
        .map(|a| {
            (0..27)
                .map(|p| {
                    let a = digits(a);
                    let p = digits(p);
                    f.pow(
                        zeta,
                        ((0..3).map(|i| a[i] * p[i]).sum::<usize>() % 3) as u128,
                    )
                })
                .collect()
        })
        .collect();
    let reps: Vec<_> = (0..27).filter(|&a| a <= negchar(a)).collect();
    let sig: [Vec<usize>; 2] = std::array::from_fn(|k| {
        let mut inv = vec![0; 27];
        for i in 0..27 {
            inv[fr[k][i]] = i
        }
        reps.iter()
            .map(|&a| {
                let row: Vec<_> = (0..27).map(|i| chars[a][inv[i]]).collect();
                let b = chars.iter().position(|r| *r == row).unwrap();
                reps.iter().position(|&r| r == b.min(negchar(b))).unwrap()
            })
            .collect()
    });
    let m2 = monomials(2);
    let m3 = monomials(3);
    let m4 = monomials(4);
    assert_eq!((m2.len(), m3.len(), m4.len()), (36, 120, 330));
    controls_image(&m2, &m3, &m4);
    for (d, ms) in [(2, &m2), (3, &m3), (4, &m4)] {
        for (i, e) in ms.iter().enumerate() {
            println!("MONOMIAL degree={d} index={i} exponents={e:?}")
        }
    }
    let mut points: [Vec<[u64; 8]>; 2] = [Vec::new(), Vec::new()];
    let mut ids = Vec::new();
    let mut excluded = 0;
    for n in 1001..=4000 {
        let d = refc.mul(&d1, n);
        let x = Div {
            u: Pol::embed(&d.u),
            v: Pol::embed(&d.v),
        };
        let mut pair = Vec::new();
        let mut reason = None;
        for k in 0..2 {
            match values(&c, &ks[k], &hs[k], &neg[k], &x) {
                Ok(v) => {
                    let ph = projections(&v, &alpha[k], &chars, f);
                    if ph[0] == Z {
                        reason = Some(format!("kernel{k}_theta_zero"));
                        break;
                    }
                    let tr = trace_candidates(&products(&ph, &reps, f), &sig[k], f);
                    let indices = [0, 13, 14, 15, 16, 17, 18, 19];
                    let q = std::array::from_fn(|j| {
                        let t = tr[indices[j]];
                        assert!(t[1..].iter().all(|&v| v == 0));
                        t[0]
                    });
                    pair.push(q)
                }
                Err(e) => {
                    reason = Some(format!("kernel{k}_{e}"));
                    break;
                }
            }
        }
        if let Some(r) = reason {
            println!("ATTEMPT n={n} regular=false reason={r}");
            excluded += 1;
            continue;
        }
        let holdout = ids.len() >= 160;
        println!("ATTEMPT n={n} regular=true holdout={holdout}");
        ids.push(n);
        for k in 0..2 {
            points[k].push(pair[k]);
            println!(
                "POINT kernel={k} n={n} holdout={holdout} values={:?}",
                pair[k]
            )
        }
        if ids.len() == 192 {
            break;
        }
    }
    assert_eq!(ids.len(), 192, "CAP sample count");
    let mut counts = Vec::new();
    let mut holdoutchecks = 0;
    let mut corrupt_failures = 0;
    for k in 0..2 {
        let train = &points[k][..160];
        let r2: Vec<_> = train.iter().map(|x| row_at(&m2, x)).collect();
        let r3: Vec<_> = train.iter().map(|x| row_at(&m3, x)).collect();
        let r4: Vec<_> = train
            .iter()
            .flat_map(|x| (0..8).map(|j| derivative_row(&m4, x, j)))
            .collect();
        let q2 = nullspace_fp(&r2, 36);
        let q3 = nullspace_fp(&r3, 120);
        let q4 = nullspace_fp(&r4, 330);
        println!(
            "DIMENSIONS kernel={k} quadric={} cubic={} quartic_gradient={}",
            q2.len(),
            q3.len(),
            q4.len()
        );
        for (d, ms, polys) in [(2, &m2, &q2), (3, &m3, &q3), (4, &m4, &q4)] {
            for (i, q) in polys.iter().enumerate() {
                print!("POLYNOMIAL kernel={k} degree={d} index={i} coefficients");
                for a in q {
                    print!(" {a}")
                }
                println!();
                for (t, x) in points[k].iter().enumerate() {
                    assert_eq!(
                        dot_fp(q, &row_at(ms, x)),
                        0,
                        "failed degree{d} k{k} sample{t}"
                    );
                    if t >= 160 {
                        holdoutchecks += 1
                    }
                    if d == 4 {
                        for j in 0..8 {
                            assert_eq!(dot_fp(q, &derivative_row(ms, x, j)), 0);
                            if t >= 160 {
                                holdoutchecks += 1
                            }
                        }
                    }
                }
            }
        }
        let base_rank = if q3.is_empty() {
            0
        } else {
            pivots_fp(q3.clone()).len()
        };
        let mut gradientranks = Vec::new();
        for (i, q) in q4.iter().enumerate() {
            let gradients: Vec<_> = (0..8)
                .map(|j| partial_coefficients(q, &m4, &m3, j))
                .collect();
            for g in &gradients {
                let mut rows = q3.clone();
                rows.push(g.clone());
                assert_eq!(
                    pivots_fp(rows).len(),
                    base_rank,
                    "gradient outside cubic space"
                )
            }
            let rank = pivots_fp(gradients).len();
            gradientranks.push(rank);
            println!(
                "GRADIENT kernel={k} quartic={i} span_rank={rank} cubic_space_rank={base_rank}"
            );
            let mut bad = q.clone();
            let at = m4
                .iter()
                .position(|e| *e == [4, 0, 0, 0, 0, 0, 0, 0])
                .unwrap();
            bad[at] = (bad[at] + 1) % P;
            let failures = points[k][160..]
                .iter()
                .filter(|x| dot_fp(&bad, &derivative_row(&m4, x, 0)) != 0)
                .count();
            assert!(failures > 0);
            corrupt_failures += failures;
        }
        let pattern = q2.is_empty() && q3.len() == 8 && q4.len() == 1 && gradientranks == [8];
        println!(
            "PATTERN kernel={k} coble_type_candidate={pattern} global_identity_certified=false"
        );
        counts.push((q2.len(), q3.len(), q4.len()));
    }
    println!("TERMINAL kernels=2 training_each=160 holdout_each=32 excluded_multiples={excluded} dimensions={counts:?} holdout_polynomial_gradient_checks={holdoutchecks} corrupted_quartic_failures={corrupt_failures} global_identity_certified=false pass=true");
}
