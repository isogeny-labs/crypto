//! Six-summand M31 panel, with complete outputs and setup-inclusive timings.
use super::*;

fn multiplicity(key: &[u8; 6]) -> u64 {
    let mut denominator = 1;
    let mut run = 1;
    for i in 1..6 {
        if key[i] == key[i - 1] {
            run += 1;
        } else {
            denominator *= (1..=run).product::<u64>();
            run = 1;
        }
    }
    denominator *= (1..=run).product::<u64>();
    720 / denominator
}

fn cached_polynomial<'a>(
    c: Curve,
    base: &[Point],
    key: &[u8],
    cache: &'a mut HashMap<Vec<u8>, Poly>,
    steps: &mut HashMap<(u8, usize), ResultantStep>,
) -> &'a [u64] {
    if !cache.contains_key(key) {
        let (&last, prefix) = key.split_last().unwrap();
        let row = if prefix.is_empty() {
            vec![c.f.neg(base[last as usize].0), 1]
        } else {
            let previous = cached_polynomial(c, base, prefix, cache, steps).to_vec();
            let degree = previous.len() - 1;
            let step = steps
                .entry((last, degree))
                .or_insert_with(|| ResultantStep::new(c, base[last as usize].0, degree));
            resultant_with_step(c.f, &previous, step)
        };
        cache.insert(key.to_vec(), row);
    }
    &cache[key]
}

fn direct_six(
    c: Curve,
    base: &[Point],
    target: Point,
    cached_steps: bool,
) -> (BTreeSet<usize>, u64, usize) {
    let mut cache = HashMap::new();
    let mut steps = HashMap::new();
    let mut roots = BTreeSet::new();
    let mut top = 0;
    let mut evaluations = 0;
    m31_single_target::multisets::<6>(|key| {
        let row = if cached_steps {
            cached_polynomial(c, base, key, &mut cache, &mut steps)
        } else {
            polynomial(c, base, key, &mut cache)
        };
        let value = evaluate(c.f, row, target.0);
        evaluations += 1;
        if value == 0 {
            m31_single_target::ordered_labels(key, &mut roots);
        }
        let weighted = c.f.mul(value, multiplicity(key));
        if key.iter().map(|v| v.count_ones()).sum::<u32>() & 1 == 0 {
            top = c.f.add(top, weighted);
        } else {
            top = c.f.sub(top, weighted);
        }
    });
    (roots, top, evaluations)
}

fn indexed_six(
    c: Curve,
    base: &[Point],
    target: Point,
    output: Option<BufWriter<File>>,
) -> (BTreeSet<usize>, usize, usize, usize, usize) {
    let mut pair_index: HashMap<u64, Vec<[u8; 2]>> = HashMap::new();
    let mut pair_charts = 0;
    for i in 0..SIZE {
        let row = coefficients(c, base[i].0);
        for j in i..SIZE {
            let data = row.each_ref().map(|poly| evaluate(c.f, poly, base[j].0));
            pair_charts += 1;
            for root in quadratic_roots(c.f, data).into_iter().flatten() {
                pair_index.entry(root).or_default().push([i as u8, j as u8]);
            }
        }
    }
    let mut found = BTreeSet::new();
    let mut prefix_sums = 0;
    m31_single_target::multisets::<4>(|prefix| {
        for sum in signed_sum(c, prefix.iter().map(|&i| base[i as usize])) {
            prefix_sums += 1;
            let residual = group_add(c, Some(target), sum.map(|point| neg(c.f, point)));
            if let Some(residual) = residual {
                if let Some(pairs) = pair_index.get(&residual.0) {
                    for &[i, j] in pairs {
                        if signed_sum(c, [base[i as usize], base[j as usize]])
                            .contains(&Some(residual))
                        {
                            let mut key = prefix.to_vec();
                            key.extend([i, j]);
                            key.sort_unstable();
                            found.insert(key);
                        }
                    }
                }
            } else {
                for i in 0..SIZE {
                    let mut key = prefix.to_vec();
                    key.extend([i as u8, i as u8]);
                    key.sort_unstable();
                    found.insert(key);
                }
            }
        }
    });
    let mut roots = BTreeSet::new();
    for key in found {
        m31_single_target::ordered_labels(&key, &mut roots);
    }
    let charts: Vec<_> = roots
        .iter()
        .map(|&origin| Chart {
            origin,
            flips: vec![],
        })
        .collect();
    let (reconstructed, auxiliaries, basis_rows) = assemble(c, &charts, 24, output);
    assert_eq!(roots, reconstructed);
    (roots, pair_charts, prefix_sums, auxiliaries, basis_rows)
}

pub(super) fn run() {
    let seed = std::env::args()
        .nth(2)
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap_or(11_781_661);
    let c = Curve::new(P, A, B);
    let started = Instant::now();
    let base = factor_base(c);
    let base_seconds = started.elapsed().as_secs_f64();
    let output_dir = std::env::var_os("S3_BASIS_DIR").map(PathBuf::from);
    let cached_steps = std::env::var_os("S3_DISABLE_RESULTANT_STEP_CACHE").is_none();
    let kind = std::env::args().nth(3);
    let kinds = match kind.as_deref() {
        Some("planted") => &["planted"][..],
        Some("unplanted") => &["unplanted"][..],
        None => &["planted", "unplanted"][..],
        _ => panic!("kind must be planted or unplanted"),
    };
    for &kind in kinds {
        let target = m31_single_target::target(c, &base, seed, kind, 6);
        let output = output_dir.as_ref().map(|dir| {
            BufWriter::new(
                File::create(dir.join(format!("target-{}.basis.txt", target.0))).unwrap(),
            )
        });
        let direct_first = std::env::var_os("S3_DIRECT_FIRST").is_some();
        let (indexed, pair_charts, prefix_sums, auxiliaries, basis_rows);
        let (direct, top, evaluations);
        let indexed_seconds;
        let direct_seconds;
        if direct_first {
            let started = Instant::now();
            (direct, top, evaluations) = direct_six(c, &base, target, cached_steps);
            direct_seconds = started.elapsed().as_secs_f64() + base_seconds;
            let started = Instant::now();
            (indexed, pair_charts, prefix_sums, auxiliaries, basis_rows) =
                indexed_six(c, &base, target, output);
            indexed_seconds = started.elapsed().as_secs_f64() + base_seconds;
        } else {
            let started = Instant::now();
            (indexed, pair_charts, prefix_sums, auxiliaries, basis_rows) =
                indexed_six(c, &base, target, output);
            indexed_seconds = started.elapsed().as_secs_f64() + base_seconds;
            let started = Instant::now();
            (direct, top, evaluations) = direct_six(c, &base, target, cached_steps);
            direct_seconds = started.elapsed().as_secs_f64() + base_seconds;
        }
        let signed = m31_single_target::signed_group_solve::<4>(c, &base, target);
        assert_eq!(indexed, direct);
        assert_eq!(indexed, signed);
        assert_ne!(top, 0, "the direct degree-24 certificate must be nonzero");
        let multisets: BTreeSet<_> = indexed
            .iter()
            .map(|&word| {
                let mut key = [0u8; 6];
                for (i, index) in key.iter_mut().enumerate() {
                    *index = ((word >> (K * i)) & (SIZE - 1)) as u8;
                }
                key.sort_unstable();
                key
            })
            .collect();
        let mut relation_rows = BTreeSet::new();
        for key in &multisets {
            for mask in 0..64 {
                let sum = key.iter().enumerate().fold(None, |sum, (i, &index)| {
                    let point = base[index as usize];
                    let point = if mask >> i & 1 == 0 {
                        point
                    } else {
                        neg(c.f, point)
                    };
                    group_add(c, sum, Some(point))
                });
                if sum == Some(target) {
                    let mut row = [0i32; SIZE];
                    for (i, &index) in key.iter().enumerate() {
                        row[index as usize] += if mask >> i & 1 == 0 { 1 } else { -1 };
                    }
                    relation_rows.insert(row);
                }
            }
        }
        let signed_witnesses = relation_rows.len();
        println!(
            "{{\"seed\":{seed},\"kind\":\"{kind}\",\"summands\":6,\"width\":24,\"direct_degree_lower_bound\":24,\"structured_degree_cap\":2,\"target\":{},\"target_x\":{},\"target_y\":{},\"roots\":{},\"distinct_abscissa_multisets\":{},\"signed_witnesses\":{signed_witnesses},\"relation_rows\":{:?},\"pair_charts\":{pair_charts},\"prefix_sums\":{prefix_sums},\"auxiliary_variables\":{auxiliaries},\"basis_rows\":{basis_rows},\"quadratic_rows\":{},\"direct_multisets\":{evaluations},\"indexed_total_seconds\":{indexed_seconds},\"direct_total_seconds\":{direct_seconds},\"top_coefficient\":{top}}}",
            target.0,
            target.0,
            target.1,
            indexed.len(),
            multisets.len(),
            relation_rows.iter().collect::<Vec<_>>(),
            if indexed.is_empty() { 0 } else { basis_rows - auxiliaries - 24 },
        );
    }
}
