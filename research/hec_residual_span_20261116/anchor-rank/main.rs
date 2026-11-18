#[path = "../production/qualified.rs"]
mod qualified;
use hec_native_ic::{
    fp::Fp,
    poly::Poly,
    solve::{solve_for_last, Row, SolveStats, Solver},
};
use std::{fs, io::Write};
fn rank(mut a: Vec<Vec<u64>>, f: &Fp) -> usize {
    if a.is_empty() {
        return 0;
    }
    let mut r = 0;
    for j in 0..a[0].len() {
        let Some(i) = (r..a.len()).find(|&i| a[i][j] != 0) else {
            continue;
        };
        a.swap(r, i);
        let z = f.inv(a[r][j]);
        for k in j..a[r].len() {
            a[r][k] = f.mul(a[r][k], z)
        }
        for i in r + 1..a.len() {
            let z = a[i][j];
            for k in j..a[i].len() {
                a[i][k] = f.sub(a[i][k], f.mul(z, a[r][k]))
            }
        }
        r += 1;
        if r == a.len() {
            break;
        }
    }
    r
}
fn main() {
    let mut fixtures =
        fs::File::create_new("research/hec_residual_span_20261116/anchor-rank/FIXTURES.tsv")
            .unwrap();
    let mut prefix =
        fs::File::create_new("research/hec_residual_span_20261116/anchor-rank/PREFIX.tsv").unwrap();
    writeln!(prefix, "g\tp\tsm\tcase\tanchor\tt\trows\trank\tk\tgroup_ok").unwrap();
    println!("g\tp\tsm\tcase\tanchors\tparameters\tskipped_degree\tshared\trepeated\taccepted\tbucket_checks\trank\tfirst_anchor\tk\tcorrect");
    let mut failures = 0;
    let mut single_checks = 0;
    let mut positive_checks = 0;
    let mut rank_receipt =
        fs::File::create_new("research/hec_residual_span_20261116/anchor-rank/RANKS.tsv").unwrap();
    writeln!(rank_receipt,"g\tp\tsm\tcase\tanchor\tt\trows\trank_coeff\trank_drop_target\trank_augmented\tnative_determined\tpublic_source_normalization_available").unwrap();
    for (g, p) in [(2usize, 101u64), (3, 113), (4, 109)] {
        let x = qualified::fixture(g, p, &mut fixtures).expect("fixed qualified model");
        let c = &x.c;
        let f = Fp::new(x.ell);
        for sm in [1u64, 17] {
            let source = c.mul(&x.source, sm);
            for (case, expected) in [37u64, 2026, 7001].into_iter().enumerate() {
                let expected = expected % x.ell;
                let target = c.mul(&source, expected);
                // Normalize only when the PUBLIC source Mumford divisor splits.
                let mut source_u = source.u;
                let mut source_coeff = vec![0; x.reps.len()];
                let mut source_sum = c.zero();
                for xx in 0..p {
                    while source_u.deg() > 0 && source_u.eval(xx, &c.fp) == 0 {
                        source_u = source_u.div_exact(&Poly::x_minus(xx, &c.fp), &c.fp);
                        let yy = source.v.eval(xx, &c.fp);
                        source_sum = c.add_place(&source_sum, xx, yy);
                        let j = x.fb.iter().position(|v| v.0 == xx).unwrap();
                        let (r, w) = x.map[j];
                        if r != usize::MAX {
                            let w = if yy == x.fb[j].1 {
                                w
                            } else {
                                assert_eq!(yy, c.fp.neg(x.fb[j].1));
                                f.neg(w)
                            };
                            source_coeff[r] = f.add(source_coeff[r], w)
                        }
                    }
                }
                let normalization = if source_u.deg() == 0 {
                    assert_eq!(source_sum, source);
                    Some((
                        source_coeff
                            .iter()
                            .enumerate()
                            .filter_map(|(j, &v)| (v != 0).then_some((j as u32, v)))
                            .collect::<Row>(),
                        1u64,
                    ))
                } else {
                    None
                };
                let mut rows: Vec<(Row, u64)> = vec![];
                let mut dense = vec![];
                let (mut params, mut skipped, mut shared, mut repeated, mut accepted, mut checks) =
                    (0, 0, 0, 0, 0, 0);
                let (mut first, mut found) = (0, None);
                for a in 1..=16u64 {
                    let mut single: Vec<(Row, u64)> = vec![];
                    let mut single_dense: Vec<Vec<u64>> = vec![];
                    let anchor = c.add(&c.mul(&source, a), &target);
                    if anchor.u.deg() != g as isize {
                        skipped += 1;
                        continue;
                    }
                    let mut buckets = vec![Vec::<(u64, u64)>::new(); p as usize];
                    for &(xx, yy) in &x.fb {
                        let u = anchor.u.eval(xx, &c.fp);
                        if u == 0 {
                            continue;
                        }
                        let inv = c.fp.inv(u);
                        let v = anchor.v.eval(xx, &c.fp);
                        let signs = if yy == 0 {
                            vec![0]
                        } else {
                            vec![yy, c.fp.neg(yy)]
                        };
                        for y in signs {
                            let t = c.fp.mul(c.fp.sub(y, v), inv);
                            buckets[t as usize].push((xx, y))
                        }
                    }
                    for t in 0..p {
                        params += 1;
                        let vt = anchor.v.add(&anchor.u.scale(t, &c.fp), &c.fp);
                        let (h, rem) = c.f.sub(&vt.mul(&vt, &c.fp), &c.fp).divrem(&anchor.u, &c.fp);
                        assert_eq!(rem, Poly::zero());
                        assert_eq!(h.deg(), (g + 1) as isize);
                        // Independent exhaustive linear division, including multiplicities and shared roots.
                        let mut left = h;
                        let mut roots = vec![];
                        for xx in 0..p {
                            let lin = Poly::x_minus(xx, &c.fp);
                            loop {
                                let (q, r) = left.divrem(&lin, &c.fp);
                                if r != Poly::zero() {
                                    break;
                                }
                                roots.push(xx);
                                left = q;
                                if left.deg() == 0 {
                                    break;
                                }
                            }
                        }
                        let mut direct: Vec<_> = roots
                            .iter()
                            .copied()
                            .filter(|&xx| anchor.u.eval(xx, &c.fp) != 0)
                            .collect();
                        direct.sort_unstable();
                        direct.dedup();
                        let mut bucket: Vec<_> = buckets[t as usize].iter().map(|v| v.0).collect();
                        bucket.sort_unstable();
                        assert_eq!(direct, bucket);
                        checks += 1;
                        if roots.iter().any(|&xx| anchor.u.eval(xx, &c.fp) == 0) {
                            shared += 1;
                            continue;
                        }
                        if roots.windows(2).any(|w| w[0] == w[1]) {
                            repeated += 1;
                            continue;
                        }
                        if left.deg() != 0 {
                            continue;
                        }
                        assert_eq!(roots.len(), g + 1);
                        let mut reconstruction = Poly::constant(h.lc());
                        let mut sum = c.zero();
                        let mut coeff = vec![0; x.reps.len() + 1];
                        for xx in roots {
                            reconstruction = reconstruction.mul(&Poly::x_minus(xx, &c.fp), &c.fp);
                            let y = vt.eval(xx, &c.fp);
                            assert_eq!(c.fp.mul(y, y), c.f.eval(xx, &c.fp));
                            sum = c.add_place(&sum, xx, y);
                            let j = x.fb.iter().position(|v| v.0 == xx).unwrap();
                            let (r, w) = x.map[j];
                            if r != usize::MAX {
                                let signed = if y == x.fb[j].1 {
                                    w
                                } else {
                                    assert_eq!(y, c.fp.neg(x.fb[j].1));
                                    f.neg(w)
                                };
                                coeff[r] = f.add(coeff[r], signed)
                            }
                        }
                        assert_eq!(reconstruction, h);
                        assert!(c.is_zero(&c.add(&anchor, &sum)));
                        coeff[x.reps.len()] = 1;
                        let row = coeff
                            .iter()
                            .enumerate()
                            .filter_map(|(j, &v)| (v != 0).then_some((j as u32, v)))
                            .collect();
                        rows.push((row, f.neg(a % x.ell)));
                        dense.push(coeff);
                        single.push(rows.last().unwrap().clone());
                        single_dense.push(dense.last().unwrap().clone());
                        assert!(!c.is_zero(&anchor));
                        let sr = rank(single_dense.clone(), &f);
                        let dropped = single_dense
                            .iter()
                            .map(|v| v[..x.reps.len()].to_vec())
                            .collect();
                        assert_eq!(sr, rank(dropped, &f));
                        let augmented = single_dense
                            .iter()
                            .map(|v| {
                                let mut v = v.clone();
                                v.push(f.neg(a % x.ell));
                                v
                            })
                            .collect();
                        assert_eq!(sr, rank(augmented, &f));
                        let mut one_stats = SolveStats::default();
                        assert_eq!(
                            solve_for_last(
                                &single,
                                x.reps.len(),
                                x.ell,
                                Solver::Fast,
                                &[],
                                &mut one_stats
                            ),
                            None
                        );
                        single_checks += 1;
                        writeln!(
                            rank_receipt,
                            "{g}\t{p}\t{sm}\t{case}\t{a}\t{t}\t{}\t{sr}\t{sr}\t{sr}\tfalse\t{}",
                            single.len(),
                            normalization.is_some()
                        )
                        .unwrap();
                        if let Some(norm) = &normalization {
                            let mut normalized = single.clone();
                            normalized.push(norm.clone());
                            let mut ns = SolveStats::default();
                            if let Some(k) = solve_for_last(
                                &normalized,
                                x.reps.len(),
                                x.ell,
                                Solver::Fast,
                                &[],
                                &mut ns,
                            ) {
                                assert_eq!(c.mul(&source, k), target);
                                assert_eq!(k, expected);
                                positive_checks += 1
                            }
                        }

                        accepted += 1;
                        let mut stats = SolveStats::default();
                        let k = solve_for_last(
                            &rows,
                            x.reps.len(),
                            x.ell,
                            Solver::Fast,
                            &[],
                            &mut stats,
                        );
                        if let Some(k) = k {
                            assert_eq!(c.mul(&source, k), target);
                            if found.is_none() {
                                first = a;
                                found = Some(k)
                            }
                        }
                        writeln!(
                            prefix,
                            "{g}\t{p}\t{sm}\t{case}\t{a}\t{t}\t{}\t{}\t{}\ttrue",
                            rows.len(),
                            rank(dense.clone(), &f),
                            k.map(|v| v.to_string()).unwrap_or("none".into())
                        )
                        .unwrap();
                    }
                }
                let correct = found == Some(expected);
                if !correct {
                    failures += 1
                }
                println!("{g}\t{p}\t{sm}\t{case}\t16\t{params}\t{skipped}\t{shared}\t{repeated}\t{accepted}\t{checks}\t{}\t{first}\t{}\t{correct}",rank(dense,&f),found.map(|v|v.to_string()).unwrap_or("none".into()));
            }
        }
    }
    assert_eq!(single_checks, 892);
    assert!(positive_checks > 0);
    eprintln!("single_anchor_prefix_checks={single_checks} source_normalized_determining_prefixes={positive_checks}");
    if failures != 0 {
        eprintln!("incomplete_target_cases={failures}");
        std::process::exit(2)
    }
}
