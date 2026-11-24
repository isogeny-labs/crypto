//! Signed prefix charts with exact subfield S3 recovery and a quadratic graph basis.
use super::higher_degree::{evaluation, polynomial, signed_sum};
use super::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

const K: usize = 4;
const WIDTH: usize = 5 * K;
const SIZE: u64 = 1 << K;
const ORIGINAL_MODULUS: u64 = 0x1009;

fn quadratic_extension(original: &Coset) -> (Coset, Vec<u64>) {
    let f = Gf2::new(&IrreduciblePoly {
        degree: 24,
        low_terms: vec![0, 1, 2, 7],
    });
    let fixed = BinaryMap::new((0..24).map(|i| f.sqr_k(1 << i, 12) ^ (1 << i)));
    assert_eq!(fixed.free.len(), 12);
    let coefficients: Vec<_> = (0..=12).map(|j| (ORIGINAL_MODULUS >> j) & 1).collect();
    let generator = span(&fixed.free)
        .into_iter()
        .find(|&x| evaluation(&f, &coefficients, x) == 0)
        .unwrap();
    let mut embedding = vec![1];
    while embedding.len() < 12 {
        embedding.push(f.mul(*embedding.last().unwrap(), generator));
    }
    let basis = original
        .basis
        .iter()
        .map(|&v| apply(&embedding, v))
        .collect();
    let shift = apply(&embedding, original.shift);
    (Coset::with_geometry(f, basis, shift), embedding)
}

fn upper_triples() -> impl Iterator<Item = [u64; 3]> {
    (0..SIZE).flat_map(|i| (i..SIZE).flat_map(move |j| (j..SIZE).map(move |k| [i, j, k])))
}

fn prefix_permutations([a, b, c]: [u64; 3]) -> BTreeSet<u64> {
    [
        [a, b, c],
        [a, c, b],
        [b, a, c],
        [b, c, a],
        [c, a, b],
        [c, b, a],
    ]
    .map(|[i, j, k]| i | (j << K) | (k << (2 * K)))
    .into_iter()
    .collect()
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Chart {
    origin: u64,
    directions: Vec<u64>,
}

fn canonical_chart(mut chart: Chart) -> Chart {
    let mut pivots = [0; WIDTH];
    for mut row in chart.directions {
        for (i, &pivot) in pivots.iter().enumerate().rev() {
            if row >> i & 1 != 0 && pivot != 0 {
                row ^= pivot;
            }
        }
        assert_ne!(row, 0);
        let lead = row.ilog2() as usize;
        for pivot in &mut pivots {
            if *pivot >> lead & 1 != 0 {
                *pivot ^= row;
            }
        }
        pivots[lead] = row;
    }
    for (i, &row) in pivots.iter().enumerate().rev() {
        if chart.origin >> i & 1 != 0 {
            chart.origin ^= row;
        }
    }
    chart.directions = pivots.into_iter().filter(|&v| v != 0).collect();
    chart
}

fn merge_parallel(charts: Vec<Chart>) -> Vec<Chart> {
    let mut charts: Vec<_> = charts.into_iter().map(canonical_chart).collect();
    loop {
        let previous = charts.len();
        let mut by_direction = BTreeMap::<Vec<u64>, Vec<u64>>::new();
        for chart in charts {
            by_direction
                .entry(chart.directions)
                .or_default()
                .push(chart.origin);
        }
        charts = Vec::new();
        for (directions, mut origins) in by_direction {
            origins.sort_unstable();
            for pair in origins.chunks(2) {
                let mut dirs = directions.clone();
                if let [a, b] = *pair {
                    dirs.push(a ^ b);
                }
                charts.push(canonical_chart(Chart {
                    origin: pair[0],
                    directions: dirs,
                }));
            }
        }
        if charts.len() == previous {
            charts.sort_unstable();
            return charts;
        }
    }
}

#[derive(Default)]
struct BranchWork {
    signed_prefix_sums: usize,
    intermediate_abscissae: usize,
    parameter_candidates: usize,
    diagonal_prefixes: usize,
}

fn find_charts(c: &Coset, t: u64, values: &[u64], lifts: &[Point]) -> (Vec<Chart>, BranchWork) {
    let target = c.lift(t).unwrap();
    let indices: HashMap<_, _> = values
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i as u64))
        .collect();
    let mut cache = HashMap::new();
    let mut charts = BTreeSet::new();
    let mut work = BranchWork::default();
    for prefix in upper_triples() {
        let sums = signed_sum(c, &prefix.map(|i| lifts[i as usize]));
        work.signed_prefix_sums += sums.len();
        let mut diagonal = false;
        let mut pairs = BTreeSet::new();
        for w in sums {
            let residual = w.map_or(Some(target), |p| c.add(target, negate(p), None));
            let Some(residual) = residual else {
                diagonal = true;
                continue;
            };
            let candidates = cache.entry(residual.0).or_insert_with(|| {
                let (pairs, count, _) = c.pairs(residual.0, true);
                work.parameter_candidates += count;
                pairs
            });
            for &(a, b) in candidates.iter() {
                let (i, j) = (indices[&a], indices[&b]);
                let p = lifts[i as usize];
                let q = lifts[j as usize];
                if [p, negate(p)].into_iter().any(|p| {
                    [q, negate(q)]
                        .into_iter()
                        .any(|q| c.add(p, q, None) == Some(residual))
                }) {
                    pairs.insert((i.min(j), i.max(j)));
                }
            }
        }
        work.diagonal_prefixes += usize::from(diagonal);
        for bits in prefix_permutations(prefix) {
            if diagonal {
                charts.insert(Chart {
                    origin: bits,
                    directions: (0..K)
                        .map(|i| (1 << (3 * K + i)) | (1 << (4 * K + i)))
                        .collect(),
                });
            }
            for &(i, j) in &pairs {
                if diagonal && i == j {
                    continue;
                }
                let swap = i ^ j;
                let directions = if swap == 0 {
                    Vec::new()
                } else {
                    vec![(swap << (3 * K)) | (swap << (4 * K))]
                };
                charts.insert(Chart {
                    origin: bits | (i << (3 * K)) | (j << (4 * K)),
                    directions,
                });
            }
        }
    }
    work.intermediate_abscissae = cache.len();
    (charts.into_iter().collect(), work)
}

/// Disjoint point charts, each with an optional selector and local axes.
/// Chart zero is the default when all explicit selectors are zero.
struct Assembly {
    selectors: Vec<Option<usize>>,
    axes: Vec<Vec<usize>>,
    owners: Vec<usize>,
}

impl Assembly {
    fn new(charts: &[Chart]) -> Self {
        let mut selectors = Vec::new();
        let mut axes = Vec::new();
        let mut owners = Vec::new();
        for (i, chart) in charts.iter().enumerate() {
            let selector = (i != 0).then(|| {
                let var = owners.len();
                owners.push(i);
                var
            });
            selectors.push(selector);
            axes.push(
                chart
                    .directions
                    .iter()
                    .map(|_| {
                        let var = owners.len();
                        owners.push(i);
                        var
                    })
                    .collect(),
            );
        }
        Self {
            selectors,
            axes,
            owners,
        }
    }

    fn graph_rows(&self, charts: &[Chart]) -> Vec<(bool, Vec<usize>)> {
        (0..WIDTH)
            .map(|bit| {
                let default = charts[0].origin >> bit & 1 != 0;
                let mut support = Vec::new();
                for (i, chart) in charts.iter().enumerate() {
                    if let Some(selector) = self.selectors[i] {
                        if (chart.origin ^ charts[0].origin) >> bit & 1 != 0 {
                            support.push(selector);
                        }
                    }
                    for (&axis, &direction) in self.axes[i].iter().zip(&chart.directions) {
                        if direction >> bit & 1 != 0 {
                            support.push(axis);
                        }
                    }
                }
                (default, support)
            })
            .collect()
    }

    fn emit_quadratics(&self, output: &mut Option<BufWriter<File>>) -> (usize, u64) {
        let mut rows = 0;
        let mut checksum = 0u64;
        // This streams the explicit sparse rows without retaining a dense matrix.
        for (a, &owner) in self.owners.iter().enumerate() {
            for (b, &other) in self.owners.iter().enumerate().skip(a + 1) {
                let gate = self.selectors[owner] == Some(a) && owner == other;
                if gate || owner != other {
                    rows += 1;
                    let tail = if gate { b as u64 + 1 } else { 0 };
                    checksum = checksum.rotate_left(7)
                        ^ ((a as u64 + 1) << 32)
                        ^ (((b as u64 + 1) << 1) | tail);
                    if let Some(file) = output {
                        write!(file, "v{a}*v{b}").unwrap();
                        if gate {
                            write!(file, " + v{b}").unwrap();
                        }
                        writeln!(file).unwrap();
                    }
                }
            }
        }
        (rows, checksum)
    }

    fn emit_other_rows(&self, graph: &[(bool, Vec<usize>)], output: &mut Option<BufWriter<File>>) {
        if let Some(file) = output {
            for j in 0..self.owners.len() {
                writeln!(file, "v{j}^2 + v{j}").unwrap();
            }
            for (i, (constant, support)) in graph.iter().enumerate() {
                write!(file, "x{i}").unwrap();
                if *constant {
                    write!(file, " + 1").unwrap();
                }
                for j in support {
                    write!(file, " + v{j}").unwrap();
                }
                writeln!(file).unwrap();
            }
            file.flush().unwrap();
        }
    }

    fn validate_graph(&self, charts: &[Chart], graph: &[(bool, Vec<usize>)]) -> BTreeSet<u64> {
        let mut roots = BTreeSet::new();
        let mut active = vec![false; self.owners.len()];
        for (i, chart) in charts.iter().enumerate() {
            if let Some(selector) = self.selectors[i] {
                active[selector] = true;
            }
            for mask in 0..(1 << chart.directions.len()) {
                for (j, &axis) in self.axes[i].iter().enumerate() {
                    active[axis] = mask >> j & 1 != 0;
                }
                let candidate =
                    graph
                        .iter()
                        .enumerate()
                        .fold(0, |word, (bit, (constant, support))| {
                            let value = support
                                .iter()
                                .fold(*constant, |value, &j| value ^ active[j]);
                            word | ((value as u64) << bit)
                        });
                assert_eq!(candidate, chart.origin ^ apply(&chart.directions, mask));
                assert!(
                    roots.insert(candidate),
                    "charts have overlapping projections"
                );
            }
            for &axis in &self.axes[i] {
                active[axis] = false;
            }
            if let Some(selector) = self.selectors[i] {
                active[selector] = false;
            }
        }
        roots
    }
}

fn exact_reference(
    c: &Coset,
    values: &[u64],
    t: u64,
    cache: &mut HashMap<Vec<u8>, Vec<u64>>,
) -> BTreeSet<u64> {
    let mut zeros = BTreeSet::new();
    let mut evaluations = HashMap::new();
    for bits in 0..(1 << WIDTH) {
        let mut key = [0; 5];
        for (j, slot) in key.iter_mut().enumerate() {
            *slot = ((bits >> (K * j)) & (SIZE - 1)) as u8;
        }
        key.sort_unstable();
        let zero = evaluations
            .entry(key)
            .or_insert_with(|| evaluation(&c.f, polynomial(&c.f, values, &key, cache), t) == 0);
        if *zero {
            zeros.insert(bits);
        }
    }
    zeros
}

pub(super) fn run(output_directory: Option<PathBuf>) {
    let started = Instant::now();
    let original = Coset::new(12, K as u32, ORIGINAL_MODULUS);
    let (extended, embedding) = quadratic_extension(&original);
    let values: Vec<_> = span(&original.basis)
        .into_iter()
        .map(|u| u ^ original.shift)
        .collect();
    let lifted: Vec<_> = values.iter().map(|&x| apply(&embedding, x)).collect();
    let lifts: Vec<_> = lifted.iter().map(|&x| extended.lift(x).unwrap()).collect();
    let setup = started.elapsed().as_secs_f64();
    let mut cache = HashMap::new();
    for t in [13, 24, 34, 82] {
        let started = Instant::now();
        let (raw_charts, work) = find_charts(&extended, apply(&embedding, t), &lifted, &lifts);
        let unmerged_count = raw_charts.len();
        let charts = merge_parallel(raw_charts);
        let chart_seconds = started.elapsed().as_secs_f64();
        let started = Instant::now();
        let assembly = Assembly::new(&charts);
        let graph = assembly.graph_rows(&charts);
        let mut output = output_directory.as_ref().map(|dir| {
            BufWriter::new(File::create(dir.join(format!("target-{t}.basis.txt"))).unwrap())
        });
        let (quadratics, checksum) = assembly.emit_quadratics(&mut output);
        assembly.emit_other_rows(&graph, &mut output);
        let assembly_seconds = started.elapsed().as_secs_f64();
        let started = Instant::now();
        let roots = assembly.validate_graph(&charts, &graph);
        let recovery_seconds = started.elapsed().as_secs_f64();
        let started = Instant::now();
        let expected = exact_reference(&original, &values, t, &mut cache);
        let reference_seconds = started.elapsed().as_secs_f64();
        assert_eq!(roots, expected);
        let standard_count: usize = charts
            .iter()
            .map(|chart| 1usize << chart.directions.len())
            .sum();
        assert_eq!(standard_count, roots.len());
        let basis_rows = quadratics + assembly.owners.len() + WIDTH;
        assert!(quadratics > 0);
        println!(concat!(
            "{{\"n\":12,\"summands\":5,\"target\":{},\"roots\":{},\"charts_before_merge\":{},\"charts\":{},",
            "\"auxiliary_variables\":{},\"quadratic_rows\":{},\"basis_rows\":{},\"row_checksum\":{},",
            "\"signed_prefix_sums\":{},\"intermediate_abscissae\":{},\"parameter_candidates\":{},",
            "\"diagonal_prefixes\":{},\"solving_degree\":2,\"setup_seconds\":{:.9},",
            "\"chart_seconds\":{:.9},\"assembly_seconds\":{:.9},\"recovery_seconds\":{:.9},",
            "\"reference_seconds\":{:.9}}}"
        ),t, roots.len(), unmerged_count, charts.len(), assembly.owners.len(), quadratics, basis_rows, checksum,
          work.signed_prefix_sums, work.intermediate_abscissae, work.parameter_candidates,
          work.diagonal_prefixes, setup, chart_seconds, assembly_seconds, recovery_seconds, reference_seconds);
    }
}
