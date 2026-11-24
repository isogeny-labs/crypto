//! Cold, matched single-target timings with an independent signed-group control.
use super::*;

fn random(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn target(c: Curve, base: &[Point], seed: u64, kind: &str) -> Point {
    let mut rng = seed;
    if kind == "unplanted" {
        loop {
            let word = random(&mut rng);
            if let Some(point) = lift(c, word % P) {
                return if word >> 63 != 0 {
                    neg(c.f, point)
                } else {
                    point
                };
            }
        }
    }
    let mut indices: Vec<_> = (0..SIZE).collect();
    let mut sum = None;
    for i in 0..5 {
        let draw = random(&mut rng);
        indices.swap(i, i + (draw % (SIZE - i) as u64) as usize);
        let point = base[indices[i]];
        let point = if draw >> 63 != 0 {
            neg(c.f, point)
        } else {
            point
        };
        sum = group_add(c, sum, Some(point));
    }
    sum.expect("fixture generated a finite target")
}

fn multisets<const N: usize>(mut visit: impl FnMut(&[u8; N])) {
    fn visit_tail<const N: usize>(
        state: &mut [u8; N],
        position: usize,
        lower: u8,
        visit: &mut impl FnMut(&[u8; N]),
    ) {
        if position == N {
            visit(state);
            return;
        }
        for value in lower..SIZE as u8 {
            state[position] = value;
            visit_tail(state, position + 1, value, visit);
        }
    }

    visit_tail(&mut [0; N], 0, 0, &mut visit);
}

fn ordered_labels(sorted: &[u8; 5], output: &mut BTreeSet<usize>) {
    fn permutations(
        values: &[u8; 5],
        used: u32,
        position: usize,
        label: usize,
        output: &mut BTreeSet<usize>,
    ) {
        if position == values.len() {
            output.insert(label);
            return;
        }
        let mut previous = None;
        for (i, &value) in values.iter().enumerate() {
            if used >> i & 1 == 0 && previous != Some(value) {
                previous = Some(value);
                permutations(
                    values,
                    used | 1 << i,
                    position + 1,
                    label | ((value as usize) << (K * position)),
                    output,
                );
            }
        }
    }

    permutations(sorted, 0, 0, 0, output);
}

fn direct_solve(c: Curve, base: &[Point], target: Point) -> BTreeSet<usize> {
    let mut solutions = BTreeSet::new();
    let mut polynomials = HashMap::new();
    multisets::<5>(|key| {
        if evaluate(c.f, polynomial(c, base, key, &mut polynomials), target.0) == 0 {
            ordered_labels(key, &mut solutions);
        }
    });
    solutions
}

fn signed_group_solve(c: Curve, base: &[Point], target: Point) -> BTreeSet<usize> {
    let mut pairs = HashMap::<Option<Point>, Vec<[u8; 2]>>::new();
    multisets::<2>(|indices| {
        for sum in signed_sum(c, indices.iter().map(|&i| base[i as usize])) {
            pairs.entry(sum).or_default().push(*indices);
        }
    });
    let mut multisets_found = BTreeSet::new();
    multisets::<3>(|[i, j, k]| {
        for sum in signed_sum(c, [base[*i as usize], base[*j as usize], base[*k as usize]]) {
            let complement = group_add(c, Some(target), sum.map(|point| neg(c.f, point)));
            if let Some(matches) = pairs.get(&complement) {
                for &[a, b] in matches {
                    let mut candidate = [*i, *j, *k, a, b];
                    candidate.sort_unstable();
                    multisets_found.insert(candidate);
                }
            }
        }
    });
    let mut solutions = BTreeSet::new();
    for key in multisets_found {
        ordered_labels(&key, &mut solutions);
    }
    solutions
}

pub(super) fn run() {
    let mut args = std::env::args().skip(2);
    let seed = args.next().map_or(11781661, |v| v.parse().unwrap());
    let repetitions = args.next().map_or(9, |v| v.parse::<usize>().unwrap());
    let kind = args.next().unwrap_or_else(|| "planted".to_string());
    assert!(
        seed != 0
            && (1..=64).contains(&repetitions)
            && matches!(kind.as_str(), "planted" | "unplanted")
    );
    let arms = ["direct_resultant", "charted_resultant", "signed_group"];
    let mut expected = None;
    for trial in 0..repetitions {
        for offset in 0..arms.len() {
            let arm = arms[(trial + offset) % arms.len()];
            let started = Instant::now();
            let c = Curve::new(P, A, B);
            let base = factor_base(c);
            let input = target(c, &base, seed, &kind);
            let setup = started.elapsed().as_secs_f64();
            let entered = Instant::now();
            let roots = match arm {
                "direct_resultant" => direct_solve(c, &base, input),
                "charted_resultant" => {
                    let (charts, _) = charted(c, &base, input);
                    assemble(c, &charts, None).0
                }
                "signed_group" => signed_group_solve(c, &base, input),
                _ => unreachable!(),
            };
            let solve = entered.elapsed().as_secs_f64();
            let total = started.elapsed().as_secs_f64();
            if kind == "planted" {
                assert!(!roots.is_empty());
            }
            if let Some((prior_base, prior_input, prior_roots)) = &expected {
                assert_eq!(&base, prior_base);
                assert_eq!(input, *prior_input);
                assert_eq!(&roots, prior_roots);
            } else {
                expected = Some((base.clone(), input, roots.clone()));
            }
            println!(
                concat!(
                    "{{\"seed\":{},\"kind\":\"{}\",\"trial\":{},\"arm\":\"{}\",",
                    "\"base_start\":{},\"target_x\":{},\"target_y\":{},\"roots\":{},",
                    "\"setup_seconds\":{:.9},\"solve_seconds\":{:.9},\"total_seconds\":{:.9}}}"
                ),
                seed,
                kind,
                trial + 1,
                arm,
                base[0].0,
                input.0,
                input.1,
                roots.len(),
                setup,
                solve,
                total
            );
        }
    }
}
