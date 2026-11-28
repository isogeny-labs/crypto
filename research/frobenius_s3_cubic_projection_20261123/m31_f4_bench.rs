//! Cold single-target comparison against the byte-for-byte native F4 engine.
use super::*;
use crypto_lib::cryptanalysis::f4_fp::{
    solve, solve_visit, F4Options, F4ResourceLimits, Ordering, Poly, Verdict,
};
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::time::Duration;

fn boolean_input(c: Curve, base: &[Point], t: u64, summands: usize) -> Vec<Poly> {
    let width = K * summands;
    let f = c.f;
    let mut cache = HashMap::new();
    let mut evaluations = HashMap::new();
    let mut values: Vec<_> = (0..1usize << width)
        .map(|word| {
            let mut key = [0u8; 5];
            for (i, bit) in key[..summands].iter_mut().enumerate() {
                *bit = ((word >> (K * i)) & (SIZE - 1)) as u8;
            }
            key[..summands].sort_unstable();
            *evaluations.entry(key).or_insert_with(|| {
                evaluate(f, polynomial(c, base, &key[..summands], &mut cache), t)
            })
        })
        .collect();
    for bit in 0..width {
        let stride = 1 << bit;
        for chunk in values.chunks_exact_mut(2 * stride) {
            let (left, right) = chunk.split_at_mut(stride);
            for (output, &value) in right.iter_mut().zip(left.iter()) {
                *output = f.sub(*output, value);
            }
        }
    }
    let mut equations = vec![values
        .into_iter()
        .enumerate()
        .filter_map(|(mask, coefficient)| {
            (coefficient != 0).then(|| {
                (
                    (0..width).map(|i| ((mask >> i) & 1) as u32).collect(),
                    coefficient,
                )
            })
        })
        .collect::<Poly>()];
    for i in 0..width {
        let mut exponent = vec![0; width];
        exponent[i] = 2;
        let mut linear = exponent.clone();
        linear[i] = 1;
        equations.push(vec![(exponent, 1), (linear, P - 1)]);
    }
    equations
}

fn indexed_charts(c: Curve, base: &[Point], t: Point) -> BTreeSet<usize> {
    let mut index = HashMap::<_, BTreeSet<_>>::new();
    for (i, p) in base.iter().enumerate() {
        let row = coefficients(c, p.0);
        for (j, q) in base.iter().enumerate().skip(i) {
            let data = row.each_ref().map(|poly| evaluate(c.f, poly, q.0));
            for root in quadratic_roots(c.f, data).into_iter().flatten() {
                index.entry(root).or_default().insert((i, j));
            }
        }
    }
    let (charts, _) = charted_with(c, base, t, |w| {
        Some(index.get(&w.0).cloned().unwrap_or_default())
    })
    .unwrap();
    assemble(c, &charts, None).0
}

fn suffix_membership(c: Curve, base: &[Point]) -> [Poly; 2] {
    let factors = base.iter().fold(vec![1], |poly, &point| {
        mul(c.f, &poly, &[c.f.neg(point.0), 1])
    });
    [0, 1].map(|axis| {
        factors
            .iter()
            .enumerate()
            .filter_map(|(i, &value)| {
                let mut exponents = vec![0; 2];
                exponents[axis] = i as u32;
                (value != 0).then_some((exponents, value))
            })
            .collect()
    })
}

fn suffix_s3(c: Curve, t: u64) -> Poly {
    let f = c.f;
    let t2 = f.mul(t, t);
    let linear = f.neg(f.add(f.mul(f.mul(2, c.a), t), f.mul(4, c.b)));
    [
        (vec![2, 2], 1),
        (vec![2, 1], f.neg(f.mul(2, t))),
        (vec![1, 2], f.neg(f.mul(2, t))),
        (vec![2, 0], t2),
        (vec![0, 2], t2),
        (vec![1, 1], f.neg(f.mul(2, f.add(t2, c.a)))),
        (vec![1, 0], linear),
        (vec![0, 1], linear),
        (vec![0, 0], f.sub(f.mul(c.a, c.a), f.mul(f.mul(4, c.b), t))),
    ]
    .into_iter()
    .filter(|&(_, x)| x != 0)
    .collect()
}

fn sliced(seed: u64, kind: &str, cap: u64, batched: bool) {
    let started = Instant::now();
    let c = Curve::new(P, A, B);
    let base = factor_base(c);
    let target = m31_single_target::target(c, &base, seed, kind, 5);
    let setup = started.elapsed().as_secs_f64();
    let expected = indexed_charts(c, &base, target);
    let candidate = started.elapsed().as_secs_f64();
    assert_eq!(
        expected,
        m31_single_target::signed_group_solve::<3>(c, &base, target)
    );

    let started = Instant::now();
    let c = Curve::new(P, A, B);
    let same_base = factor_base(c);
    let same_target = m31_single_target::target(c, &same_base, seed, kind, 5);
    assert_eq!(base, same_base);
    assert_eq!(target, same_target);
    let f4_setup = started.elapsed().as_secs_f64();
    let membership = suffix_membership(c, &same_base);
    let domain: Vec<_> = base.iter().map(|point| point.0).collect();
    let domains = [Some(domain.clone()), Some(domain)];
    let range_start = base[0].0;
    let mut options = F4Options::new(Ordering::Grevlex, 34);
    options.stop_staircase = Some(16);
    options.sibling_batch = batched;
    options.deadline = Some(Instant::now() + Duration::from_secs(cap));
    let mut abscissae = Vec::new();
    charted_with(c, &same_base, same_target, |w| {
        abscissae.push(w.0);
        Some(BTreeSet::new())
    })
    .unwrap();
    let progress = AtomicUsize::new(0);
    let outcomes: Vec<_> = abscissae
        .par_iter()
        .map(|&t| {
            let completed = progress.fetch_add(1, AtomicOrdering::Relaxed);
            if completed != 0 && completed % 2048 == 0 {
                eprintln!("F4 residuals started: {completed}");
            }
            let equations = vec![
                suffix_s3(c, t),
                membership[0].clone(),
                membership[1].clone(),
            ];
            let entered = Instant::now();
            let label = |point: &[u64]| {
                assert_eq!(point.len(), 2);
                assert!(point
                    .iter()
                    .all(|x| (range_start..range_start + SIZE as u64).contains(x)));
                let i = (point[0] - range_start) as usize;
                let j = (point[1] - range_start) as usize;
                (i.min(j), i.max(j))
            };
            let mut found = BTreeSet::new();
            let report = solve_visit(&equations, 2, P, &options, &domains, |point| {
                found.insert(label(point));
                false
            })
            .unwrap();
            (
                t,
                report.complete.then_some(found),
                entered.elapsed().as_secs_f64(),
                report.f4_runs,
                report.timed_out,
            )
        })
        .collect();
    let f4_runs: usize = outcomes.iter().map(|row| row.3).sum();
    let f4_seconds: f64 = outcomes.iter().map(|row| row.2).sum();
    let timed_out = outcomes.iter().any(|row| row.4);
    let residuals = outcomes.iter().filter(|row| row.1.is_some()).count();
    let lookup: HashMap<_, _> = outcomes.into_iter().map(|row| (row.0, row.1)).collect();
    let actual = charted_with(c, &same_base, same_target, |w| lookup[&w.0].clone())
        .map(|(charts, _)| assemble(c, &charts, None).0);
    let total = started.elapsed().as_secs_f64();
    if let Some(ref roots) = actual {
        assert_eq!(roots, &expected);
    }
    println!(concat!(
        "{{\"seed\":{},\"summands\":5,\"kind\":\"{}\",\"baseline\":\"{}\",",
        "\"target_x\":{},\"target_y\":{},\"roots\":{},\"f4_complete\":{},\"timed_out\":{},",
        "\"residuals\":{},\"f4_runs\":{},\"candidate_setup_seconds\":{:.9},\"candidate_total_seconds\":{:.9},",
        "\"f4_setup_seconds\":{:.9},\"f4_worker_seconds\":{:.9},\"f4_total_seconds\":{:.9}}}"
    ),seed,kind,if batched {"batched_domain_sliced_f4"} else {"domain_sliced_f4"},target.0,target.1,expected.len(),actual.is_some(),timed_out,residuals,f4_runs,
       setup,candidate,f4_setup,f4_seconds,total);
}

pub(super) fn run() {
    let args: Vec<_> = std::env::args().skip(2).collect();
    let seed = args[0].parse().unwrap();
    let summands = args[1].parse().unwrap();
    let cap: u64 = args[2].parse().unwrap();
    let kind = args.get(3).map(String::as_str).unwrap_or("planted");
    assert!(matches!(summands, 3..=5) && cap > 0 && matches!(kind, "planted" | "unplanted"));
    if args
        .get(4)
        .is_some_and(|v| matches!(v.as_str(), "sliced-visit" | "sliced-batched"))
    {
        assert_eq!(summands, 5);
        sliced(seed, kind, cap, args[4] == "sliced-batched");
        return;
    }
    let width = K * summands;

    let started = Instant::now();
    let c = Curve::new(P, A, B);
    let base = factor_base(c);
    let t = m31_single_target::target(c, &base, seed, kind, summands);
    let candidate_setup = started.elapsed().as_secs_f64();
    let candidate_start = Instant::now();
    let expected = match summands {
        3 => m31_single_target::signed_group_solve::<1>(c, &base, t),
        4 => m31_single_target::signed_group_solve::<2>(c, &base, t),
        5 => m31_single_target::signed_group_solve::<3>(c, &base, t),
        _ => unreachable!(),
    };
    let candidate_solve = candidate_start.elapsed().as_secs_f64();
    let candidate_total = started.elapsed().as_secs_f64();

    let started = Instant::now();
    let field = Curve::new(P, A, B);
    let same_base = factor_base(field);
    let same_t = m31_single_target::target(field, &same_base, seed, kind, summands);
    let baseline_setup = started.elapsed().as_secs_f64();
    assert_eq!(base, same_base);
    assert_eq!(t, same_t);
    let encoding = Instant::now();
    let equations = boolean_input(field, &same_base, t.0, summands);
    let encoding_seconds = encoding.elapsed().as_secs_f64();
    eprintln!(
        "Native F4 input ready with {} nonzero Semaev terms",
        equations[0].len()
    );
    let mut options = F4Options::new(Ordering::Grevlex, (2 * width + 2) as u32);
    options.max_pairs_per_step = Some(128);
    options.resource_limits = (summands >= 4).then_some(F4ResourceLimits {
        max_pairs: 1_000_000,
        max_symbolic_monomials: 3_000_000,
        max_matrix_entries: 25_000_000,
    });
    options.deadline = Some(Instant::now() + Duration::from_secs(cap));
    let entered = Instant::now();
    let result = solve(&equations, width, P, &options);
    let f4_seconds = entered.elapsed().as_secs_f64();
    let actual = match result.verdict {
        Verdict::Solutions(points) => Some(
            points
                .into_iter()
                .map(|point| {
                    assert_eq!(point.len(), width);
                    point.into_iter().enumerate().fold(0, |mask, (i, bit)| {
                        assert!(bit <= 1);
                        mask | ((bit as usize) << i)
                    })
                })
                .collect::<BTreeSet<_>>(),
        ),
        Verdict::Inconsistent => Some(BTreeSet::new()),
        Verdict::Undetermined => None,
    };
    let baseline_total = started.elapsed().as_secs_f64();
    if let Some(set) = &actual {
        assert_eq!(set, &expected);
    }
    println!(
        concat!(
            "{{\"seed\":{},\"summands\":{},\"kind\":\"{}\",\"target_x\":{},\"target_y\":{},",
            "\"roots\":{},\"f4_complete\":{},\"timed_out\":{},\"f4_runs\":{},",
            "\"degree_reached\":{},\"max_rows\":{},\"max_cols\":{},\"input_terms\":{},",
            "\"candidate_setup_seconds\":{:.9},\"candidate_solve_seconds\":{:.9},",
            "\"candidate_total_seconds\":{:.9},\"f4_setup_seconds\":{:.9},",
            "\"f4_encoding_seconds\":{:.9},\"f4_solve_seconds\":{:.9},\"f4_total_seconds\":{:.9}}}"
        ),
        seed,
        summands,
        kind,
        t.0,
        t.1,
        expected.len(),
        actual.is_some(),
        result.timed_out,
        result.f4_runs,
        result.degree_reached,
        result.max_rows,
        result.max_cols,
        equations[0].len(),
        candidate_setup,
        candidate_solve,
        candidate_total,
        baseline_setup,
        encoding_seconds,
        f4_seconds,
        baseline_total
    );
}
