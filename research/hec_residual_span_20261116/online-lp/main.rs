#[path = "../gain-graph/graph.rs"]
mod graph;
mod online;
use hec_native_ic::counted;
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
        fs::File::create_new("research/hec_residual_span_20261116/online-lp/FIXTURES.tsv").unwrap();
    let mut prefix =
        fs::File::create_new("research/hec_residual_span_20261116/online-lp/PREFIX.tsv").unwrap();
    writeln!(prefix, "g\tp\tsm\tcase\tanchor\tt\trows\trank\tk\tgroup_ok").unwrap();
    println!("g\tp\tsm\tcase\tanchors\tparameters\tskipped_degree\tshared\trepeated\taccepted\tbucket_checks\trank\tfirst_anchor\tk\tcorrect");
    let mut failures = 0;
    let mut graph_receipt =
        fs::File::create_new("research/hec_residual_span_20261116/online-lp/GRAPH.tsv").unwrap();
    writeln!(graph_receipt,"g\tp\tsm\tcase\tsmall_orbits\tanchor\tt\tpartials\trejected\tlp_rank\tgraph_rank\tnaive_rank\tlabel_relations\tlabel_rank\tk").unwrap();
    let mut arithmetic =
        fs::File::create_new("research/hec_residual_span_20261116/online-lp/ARITH.tsv").unwrap();
    writeln!(arithmetic,"g\tp\tsm\tcase\tcut\tanchor\tt\tpartials\tlp_rank\tlabel_rank\tonline_cells\tinput_cells\tmax_lp_width\tonline_mul\tonline_inv\tonline_redc\tref_mul\tref_inv\tref_redc\tk").unwrap();
    let mut full_receipt =
        fs::File::create_new("research/hec_residual_span_20261116/online-lp/FULL.tsv").unwrap();
    writeln!(full_receipt,"g\tp\tsm\tcase\tcut\tanchor\tt\tpartials\tfull_rank\tfull_cells\tfull_mul\tfull_inv\tfull_redc\tk").unwrap();
    let mut graph_checks = 0;

    let mut false_cycle_prefixes = 0;

    for (g, p) in [(2usize, 101u64), (3, 113), (4, 109)] {
        let x = qualified::fixture(g, p, &mut fixtures).expect("fixed qualified model");
        let c = &x.c;
        let f = Fp::new(x.ell); // Two parallel gain edges: a graph cycle but full-rank LP block.
        let negative = vec![vec![f.neg(1), 1], vec![f.neg(x.lambda), 1]];
        assert_ne!(x.lambda, 1);
        assert_eq!(rank(negative.clone(), &f), 2);
        assert_eq!(graph::ranks(&negative, 2, &f), (2, 1));

        for sm in [1u64, 17] {
            let source = c.mul(&x.source, sm);
            for (case, expected) in [37u64, 2026, 7001].into_iter().enumerate() {
                let expected = expected % x.ell;
                let target = c.mul(&source, expected);
                let cuts = vec![(x.reps.len() + 3) / 4, (x.reps.len() + 1) / 2];
                let mut partials: Vec<Vec<(Row, u64)>> = vec![vec![]; 2];
                let mut lp_rows: Vec<Vec<Vec<u64>>> = vec![vec![]; 2];
                let mut label_rows: Vec<Vec<Vec<u64>>> = vec![vec![]; 2];
                let mut rejected = vec![0usize; 2];
                let mut incremental: Vec<_> = cuts
                    .iter()
                    .map(|&cut| online::Online::new(cut, x.reps.len() - cut, x.ell))
                    .collect();
                let mut full_incremental: Vec<_> = cuts
                    .iter()
                    .map(|_| online::Full::new(x.reps.len() + 1, x.ell))
                    .collect();
                let mut full_stats = vec![[0u64; 16]; 2];
                let mut own_stats = vec![[0u64; 16]; 2];
                let mut ref_stats = vec![[0u64; 16]; 2];

                let mut rows: Vec<(Row, u64)> = vec![];
                let mut dense = vec![];
                let (mut params, mut skipped, mut shared, mut repeated, mut accepted, mut checks) =
                    (0, 0, 0, 0, 0, 0);
                let (mut first, mut found) = (0, None);
                for a in 1..=16u64 {
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
                        for (ci, &cut) in cuts.iter().enumerate() {
                            let v = dense.last().unwrap();
                            let lp = v[cut..x.reps.len()].to_vec();
                            if lp.iter().filter(|&&x| x != 0).count() > 2 {
                                rejected[ci] += 1;
                                continue;
                            }
                            lp_rows[ci].push(lp);
                            partials[ci].push(rows.last().unwrap().clone());
                            label_rows[ci].push(v.clone());
                            let exact = rank(lp_rows[ci].clone(), &f);
                            let (gr, naive) = graph::ranks(&lp_rows[ci], x.reps.len() - cut, &f);
                            assert_eq!(exact, gr);
                            assert!(naive <= gr);
                            if naive < gr {
                                false_cycle_prefixes += 1
                            }
                            let surviving = partials[ci].len() - gr;
                            let labelrank = rank(label_rows[ci].clone(), &f) - gr;
                            assert!(labelrank <= surviving);
                            counted::reset();
                            incremental[ci].insert(v, f.neg(a % x.ell));
                            let own = counted::snapshot().get(&x.ell).copied().unwrap_or([0; 16]);
                            for j in 0..16 {
                                own_stats[ci][j] += own[j]
                            }
                            assert_eq!(incremental[ci].lp_rank, exact);
                            assert_eq!(incremental[ci].label_rank, labelrank);
                            counted::reset();
                            let mut ps = SolveStats::default();
                            let k = solve_for_last(
                                &partials[ci],
                                x.reps.len(),
                                x.ell,
                                Solver::Fast,
                                &[],
                                &mut ps,
                            );
                            let measured =
                                counted::snapshot().get(&x.ell).copied().unwrap_or([0; 16]);
                            for j in 0..16 {
                                ref_stats[ci][j] += measured[j]
                            }
                            assert_eq!(incremental[ci].k(), k);
                            counted::reset();
                            full_incremental[ci].insert(v, f.neg(a % x.ell));
                            let cost = counted::snapshot().get(&x.ell).copied().unwrap_or([0; 16]);
                            for j in 0..16 {
                                full_stats[ci][j] += cost[j]
                            }
                            assert_eq!(full_incremental[ci].k(), k);
                            assert_eq!(full_incremental[ci].rank, exact + labelrank);
                            let fc = full_stats[ci];
                            assert_eq!(fc[8], 2 * fc[7] + fc[5] + fc[6]);
                            writeln!(full_receipt,"{g}\t{p}\t{sm}\t{case}\t{cut}\t{a}\t{t}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",partials[ci].len(),full_incremental[ci].rank,full_incremental[ci].cells(),fc[4],fc[9],fc[8],k.map(|v|v.to_string()).unwrap_or("none".into())).unwrap();
                            let own = own_stats[ci];
                            let reference = ref_stats[ci];
                            assert_eq!(own[8], 2 * own[7] + own[5] + own[6]);
                            assert_eq!(
                                reference[8],
                                2 * reference[7] + reference[5] + reference[6]
                            );
                            let input_cells: usize =
                                partials[ci].iter().map(|(v, _)| v.len() + 1).sum();
                            writeln!(arithmetic,"{g}\t{p}\t{sm}\t{case}\t{cut}\t{a}\t{t}\t{}\t{}\t{}\t{}\t{input_cells}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",partials[ci].len(),exact,labelrank,incremental[ci].cells(),incremental[ci].max_lp_width,own[4],own[9],own[8],reference[4],reference[9],reference[8],k.map(|v|v.to_string()).unwrap_or("none".into())).unwrap();
                            if let Some(k) = k {
                                assert_eq!(c.mul(&source, k), target);
                                assert_eq!(k, expected)
                            }
                            graph_checks += 1;
                            writeln!(graph_receipt,"{g}\t{p}\t{sm}\t{case}\t{cut}\t{a}\t{t}\t{}\t{}\t{exact}\t{gr}\t{naive}\t{surviving}\t{labelrank}\t{}",partials[ci].len(),rejected[ci],k.map(|v|v.to_string()).unwrap_or("none".into())).unwrap();
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
    assert!(graph_checks > 0);
    eprintln!("graph_prefix_checks={graph_checks} naive_false_cycle_prefixes={false_cycle_prefixes} negative_controls=3");
    if failures != 0 {
        eprintln!("incomplete_target_cases={failures}");
        std::process::exit(2)
    }
}
