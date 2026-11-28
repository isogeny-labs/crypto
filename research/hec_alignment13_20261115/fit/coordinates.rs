fn digits(mut i: usize) -> [usize; 3] {
    let mut a = [0; 3];
    for x in a.iter_mut().rev() {
        *x = i % 3;
        i /= 3
    }
    assert_eq!(i, 0);
    a
}
fn negchar(a: usize) -> usize {
    let a = digits(a);
    ((3 - a[0]) % 3) * 9 + ((3 - a[1]) % 3) * 3 + (3 - a[2]) % 3
}
fn sumchar(a: usize, b: usize) -> usize {
    let a = digits(a);
    let b = digits(b);
    ((a[0] + b[0]) % 3) * 9 + ((a[1] + b[1]) % 3) * 3 + (a[2] + b[2]) % 3
}
fn projections(v: &[E], alpha: &[E], characters: &[Vec<E>], f: &Field) -> Vec<E> {
    characters
        .iter()
        .map(|row| {
            row.iter()
                .zip(v)
                .zip(alpha)
                .fold(Z, |o, ((&w, &v), &a)| f.add(o, f.mul(w, f.mul(v, a))))
        })
        .collect()
}
fn products(phi: &[E], representatives: &[usize], f: &Field) -> Vec<E> {
    assert_ne!(phi[0], Z);
    let inv = f.inv(f.mul(phi[0], phi[0]));
    representatives
        .iter()
        .map(|&a| f.mul(f.mul(phi[a], phi[negchar(a)]), inv))
        .collect()
}
fn trace_candidates(g: &[E], sigma: &[usize], f: &Field) -> Vec<E> {
    let z = {
        let mut a = Z;
        a[1] = 1;
        a
    };
    let mut result = Vec::new();
    for a in 0..g.len() {
        for j in 0..13 {
            let mut w = f.pow(z, j);
            let mut index = a;
            let mut value = Z;
            for _ in 0..13 {
                value = f.add(value, f.mul(w, g[index]));
                w = f.frob(w);
                index = sigma[index]
            }
            assert_eq!(index, a);
            result.push(value)
        }
    }
    result
}
fn field_trace(a: E, f: &Field) -> E {
    let mut v = a;
    let mut out = Z;
    for _ in 0..13 {
        out = f.add(out, v);
        v = f.frob(v)
    }
    assert_eq!(v, a);
    out
}
fn pivots_fp(mut rows: Vec<Vec<u64>>) -> Vec<usize> {
    let n = rows.len();
    let cols = rows[0].len();
    let fp = Fp::new(P);
    let mut out = Vec::new();
    for j in 0..cols {
        let r = out.len();
        if r == n {
            break;
        }
        let Some(i) = (r..n).find(|&i| rows[i][j] != 0) else {
            continue;
        };
        rows.swap(i, r);
        let inv = fp.inv(rows[r][j]);
        for k in j..cols {
            rows[r][k] = fp.mul(rows[r][k], inv)
        }
        for i in 0..n {
            if i != r {
                let v = rows[i][j];
                for k in j..cols {
                    rows[i][k] = fp.sub(rows[i][k], fp.mul(v, rows[r][k]))
                }
            }
        }
        out.push(j)
    }
    out
}
pub fn run_coordinates() {
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(600));
        eprintln!("CAP aggregate600seconds");
        std::process::exit(124)
    });
    let args: Vec<_> = std::env::args().collect();
    let (c, refc, d1, ks) = load_inputs(&args);
    let f = &c.field;
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
    assert!(indices.iter().all(|v| v.len() == 27));
    let neg: [Vec<usize>; 2] =
        std::array::from_fn(|k| ks[k].iter().map(|p| indices[k][&c.neg(p)]).collect());
    let fr: [Vec<usize>; 2] =
        std::array::from_fn(|k| ks[k].iter().map(|p| indices[k][&c.frob(p)]).collect());
    for k in 0..2 {
        assert_eq!(ks[k][0], c.zero());
        for i in 0..27 {
            assert_eq!(neg[k][i], negchar(i));
            assert_eq!(f.frob(alpha[k][i]), alpha[k][fr[k][i]]);
            for j in 0..27 {
                assert_eq!(indices[k][&c.add(&ks[k][i], &ks[k][j])], sumchar(i, j))
            }
        }
    }
    let zeta = (2..P)
        .map(base)
        .find(|&a| f.pow(a, 3) == O && a != O)
        .expect("cube root");
    println!("ZETA {}", zeta[0]);
    let characters: Vec<Vec<E>> = (0..27)
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
    let mut character_checks = 0;
    for a in 0..27 {
        let total = characters[a].iter().fold(Z, |o, &w| f.add(o, w));
        assert_eq!(total, if a == 0 { base(27) } else { Z });
        for b in 0..27 {
            for p in 0..27 {
                assert_eq!(
                    f.mul(characters[a][p], characters[b][p]),
                    characters[sumchar(a, b)][p]
                );
                character_checks += 1
            }
        }
    }
    let representatives: Vec<_> = (0..27).filter(|&a| a <= negchar(a)).collect();
    assert_eq!(representatives.len(), 14);
    let sigmas: [Vec<usize>; 2] = std::array::from_fn(|k| {
        let mut inv = vec![0; 27];
        for i in 0..27 {
            inv[fr[k][i]] = i
        }
        representatives
            .iter()
            .map(|&a| {
                let row: Vec<_> = (0..27).map(|i| characters[a][inv[i]]).collect();
                let b = characters.iter().position(|r| *r == row).unwrap();
                representatives
                    .iter()
                    .position(|&r| r == b.min(negchar(b)))
                    .unwrap()
            })
            .collect()
    });
    let mut samples = Vec::new();
    let mut rejected = 0;
    for n in 1..=128 {
        let d = refc.mul(&d1, n);
        let x = Div {
            u: Pol::embed(&d.u),
            v: Pol::embed(&d.v),
        };
        let attempt = sample(&c, &ks, &hs, &neg, n, &x);
        match attempt {
            Ok(s) => {
                if (0..2).any(|k| projections(&s.base[k], &alpha[k], &characters, f)[0] == Z) {
                    println!("ATTEMPT n={n} regular=false reason=theta_zero");
                    rejected += 1
                } else {
                    println!("ATTEMPT n={n} regular=true");
                    samples.push(s)
                }
            }
            Err(e) => {
                println!("ATTEMPT n={n} regular=false reason={e}");
                rejected += 1
            }
        }
        if samples.len() == 16 {
            break;
        }
    }
    assert_eq!(samples.len(), 16, "CAP sixteen regular samples not reached");
    let mut tables: [Vec<Vec<u64>>; 2] = [Vec::new(), Vec::new()];
    let mut nonzero_phi = [[false; 27]; 2];
    let mut eigen_checks = 0;
    let mut product_checks = 0;
    let mut trace_checks = 0;
    let mut symmetry_checks = 0;
    let mut bad_eigen_failures = 0;
    for s in &samples {
        for k in 0..2 {
            let phi = projections(&s.base[k], &alpha[k], &characters, f);
            for i in 0..27 {
                nonzero_phi[k][i] |= phi[i] != Z
            }
            let g = products(&phi, &representatives, f);
            let t = trace_candidates(&g, &sigmas[k], f);
            let mut z = Z;
            z[1] = 1;
            for a in 0..14 {
                for j in 0..13 {
                    assert_eq!(
                        t[a * 13 + j],
                        field_trace(f.mul(f.pow(z, j as u128), g[a]), f)
                    );
                    trace_checks += 1
                }
            }
            assert!(t.iter().all(|v| v[1..].iter().all(|&x| x == 0)));
            let row: Vec<_> = t.iter().map(|v| v[0]).collect();
            print!("TABLE kernel={k} n={}", s.n);
            for v in &row {
                print!(" {v}")
            }
            println!();
            tables[k].push(row);
            let mp = projections(&s.minus[k], &alpha[k], &characters, f);
            for a in 0..27 {
                assert_eq!(mp[a], phi[negchar(a)])
            }
            assert_eq!(g, products(&mp, &representatives, f));
            symmetry_checks += 14;
            for p in 0..27 {
                let pp = projections(&s.shifted[k][p], &alpha[k], &characters, f);
                let fp = f.mul(alpha[k][p], s.base[k][p]);
                for a in 0..27 {
                    assert_eq!(f.mul(fp, pp[a]), f.mul(characters[a][negchar(p)], phi[a]));
                    eigen_checks += 1
                }
                let gg = products(&pp, &representatives, f);
                assert_eq!(g, gg);
                product_checks += 14;
            }
            let mut bad = characters.clone();
            bad[1][1] = f.mul(base(2), bad[1][1]);
            let bp = projections(&s.base[k], &alpha[k], &bad, f);
            for p in 0..27 {
                let bpp = projections(&s.shifted[k][p], &alpha[k], &bad, f);
                if f.mul(f.mul(alpha[k][p], s.base[k][p]), bpp[1])
                    != f.mul(characters[1][negchar(p)], bp[1])
                {
                    bad_eigen_failures += 1
                }
            }
        }
    }
    assert!(nonzero_phi.iter().all(|v| v.iter().all(|&b| b)));
    assert!(bad_eigen_failures > 0);
    let mut selected = [Vec::new(), Vec::new()];
    for k in 0..2 {
        selected[k] = pivots_fp(tables[k][..12].to_vec());
        println!(
            "RANK kernel={k} training_points=12 trace_candidates=182 rank={}",
            selected[k].len()
        );
        assert_eq!(selected[k].len(), 8, "coordinate span not eight");
        println!(
            "BASIS kernel={k} descriptors={:?}",
            selected[k]
                .iter()
                .map(|&i| (representatives[i / 13], i % 13))
                .collect::<Vec<_>>()
        );
        let basis_rows: Vec<Vec<u64>> = tables[k][..12]
            .iter()
            .map(|r| selected[k].iter().map(|&i| r[i]).collect())
            .collect();
        let mut duplicate = basis_rows.clone();
        for r in &mut duplicate {
            r[7] = r[0]
        }
        assert_eq!(pivots_fp(duplicate).len(), 7);
    }
    let mut coordinate_checks = 0;
    let mut coordinate_symmetry = 0;
    for s in &samples {
        for k in 0..2 {
            let phi = projections(&s.base[k], &alpha[k], &characters, f);
            let t = trace_candidates(&products(&phi, &representatives, f), &sigmas[k], f);
            let chosen: Vec<_> = selected[k].iter().map(|&i| t[i]).collect();
            assert!(chosen.iter().any(|&x| x != Z));
            print!(
                "COORD kernel={k} n={} selected_by_first12={} values",
                s.n,
                samples[..12].iter().any(|r| r.n == s.n)
            );
            for v in &chosen {
                print!(" {}", v[0])
            }
            println!();
            let mp = projections(&s.minus[k], &alpha[k], &characters, f);
            let mt = trace_candidates(&products(&mp, &representatives, f), &sigmas[k], f);
            for &i in &selected[k] {
                assert_eq!(mt[i], t[i]);
                coordinate_symmetry += 1
            }
            for p in 0..27 {
                let pp = projections(&s.shifted[k][p], &alpha[k], &characters, f);
                let tt = trace_candidates(&products(&pp, &representatives, f), &sigmas[k], f);
                for &i in &selected[k] {
                    assert_eq!(tt[i], t[i]);
                    assert_eq!(f.frob(tt[i]), tt[i]);
                    coordinate_checks += 1
                }
            }
        }
    }
    println!("TERMINAL kernels=2 regular_samples=16 rejected_multiples={rejected} character_products={character_checks} nonzero_character_sections=54 eigenactions={eigen_checks} opposite_product_actions={product_checks} coefficient_field_traces={trace_checks} product_symmetry={symmetry_checks} descended_rank=[8,8] coordinate_translations={coordinate_checks} coordinate_symmetry={coordinate_symmetry} duplicate_basis_rank=7 bad_character_failures={bad_eigen_failures} pass=true");
}
