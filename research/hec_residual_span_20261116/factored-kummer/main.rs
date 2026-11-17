mod kernel;
use hec_native_ic::{
    counted,
    fp::Fp,
    instance::factor_base,
    jac::Curve,
    oracle::{split_reference, OracleStats},
    poly::Poly,
    rng::Rng,
};
use std::{fs, io::Write};
fn main() {
    let root = "research/hec_residual_span_20261116/factored-kummer";
    let mut setup = fs::File::create_new(format!("{root}/SETUP.tsv")).unwrap();
    writeln!(setup, "p\tsm\tcase\trep\tcounts").unwrap();
    println!("p\tsm\tcase\trep\tstep\tarm\tchart\tsmooth\tcounts");
    let mut checked = 0;
    let mut recovered = 0;
    let mut exceptional = 0;
    for (p, cc, ell) in [(101u64, 2u64, 7481u64), (251, 18, 67931), (601, 7, 411491)] {
        counted::disable();
        let c = Curve::new(Fp::new(p), Poly::from_coeffs(&[cc, 0, 0, 0, 0, 1]));
        let fb = factor_base(&c);
        let base = c.place(fb[0].0, fb[0].1);
        assert!(c.is_zero(&c.mul(&base, ell)));
        let mut rng = Rng::new(2026112300 ^ p);
        for sm in [1u64, 17] {
            let d = c.mul(&base, sm);
            for case in 0..3 {
                let start = if case < 2 {
                    c.mul(&d, 1 + rng.below(ell - 1))
                } else {
                    c.zero()
                };
                let edges = vec![
                    c.zero(),
                    d,
                    c.neg(&d),
                    c.mul(&d, 2),
                    base,
                    c.neg(&base),
                    c.mul(&base, 2),
                    c.neg(&c.mul(&base, 2)),
                ];
                let count = if case < 2 { 128 } else { edges.len() };
                let mut prior = None;
                for rep in 0..2 {
                    counted::reset();
                    let compiled = kernel::Compiled::new(&c, &d);
                    let mut current = kernel::project(&c, &start);
                    let mut previous = kernel::project(&c, &c.add(&start, &c.neg(&d)));
                    let setup_counts = counted::snapshot();
                    counted::disable();
                    let mut signature = vec![];
                    writeln!(
                        setup,
                        "{p}\t{sm}\t{case}\t{rep}\t{}",
                        setup_counts
                            .get(&p)
                            .copied()
                            .unwrap_or([0; 16])
                            .iter()
                            .map(u64::to_string)
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                    .unwrap();
                    let mut point = start;
                    for step in 0..count {
                        if case == 2 {
                            point = edges[step];
                            previous = kernel::project(&c, &c.add(&point, &c.neg(&d)));
                            current = kernel::project(&c, &point)
                        }
                        counted::reset();
                        let reference_next = kernel::best_add(&c, &point, &d);
                        let mut stats = OracleStats::default();
                        let roots = split_reference(&point.u, &c.fp, &mut stats);
                        let reference_points = roots
                            .as_ref()
                            .map(|r| kernel::points(&c, &point, r.as_slice()));
                        let reference_counts = counted::snapshot();
                        counted::disable();
                        counted::reset();
                        let (next, chart) = compiled.step(&c.fp, &current, &previous);
                        let kernel_counts = counted::snapshot();
                        let decoded = kernel::decode(&c, &current);
                        let candidate = decoded.as_ref().map(|a| {
                            let r = kernel::recover(&c, &d, &next, a);
                            let points = kernel::points(&c, &r, &a.roots);
                            (r, points)
                        });
                        let candidate_counts = counted::snapshot();
                        counted::disable();
                        assert_eq!(reference_next, c.add(&point, &d));
                        assert!(kernel::same(
                            &c.fp,
                            &next,
                            &kernel::project(&c, &reference_next)
                        ));
                        compiled.check_diagonal(&c.fp, &current);
                        assert_eq!(reference_points.is_some(), candidate.is_some());
                        if let Some((r, points)) = &candidate {
                            assert_eq!(*r, point);
                            assert!(c.is_valid(r));
                            assert_eq!(Some(points), reference_points.as_ref());
                            recovered += 1
                        }
                        if chart != 0 {
                            exceptional += 1
                        }
                        signature.push((
                            kernel_counts.clone(),
                            reference_counts.clone(),
                            candidate_counts.clone(),
                            chart,
                            candidate.is_some(),
                        ));
                        for (arm, counts) in [
                            (0, reference_counts),
                            (1, candidate_counts),
                            (2, kernel_counts),
                        ] {
                            let values = counts
                                .get(&p)
                                .copied()
                                .unwrap_or([0; 16])
                                .iter()
                                .map(u64::to_string)
                                .collect::<Vec<_>>()
                                .join(",");
                            println!(
                                "{p}\t{sm}\t{case}\t{rep}\t{step}\t{arm}\t{chart}\t{}\t{values}",
                                candidate.is_some()
                            );
                        }
                        checked += 1;
                        previous = current;
                        current = next;
                        point = reference_next;
                    }
                    let sig = (setup_counts, signature);
                    if let Some(p) = &prior {
                        assert_eq!(p, &sig)
                    } else {
                        prior = Some(sig)
                    }
                }
            }
        }
    }
    assert_eq!(checked, 3168);
    eprintln!("checked_steps={checked} signed_recovered={recovered} nonfirst_charts={exceptional} repeats_exact=true original_group_and_points_equal=true models=3 complete_solve=false");
}
