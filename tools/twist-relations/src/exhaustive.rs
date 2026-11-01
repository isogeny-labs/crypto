use crate::core::{norm, prime, Curve, Element, Parameters, Point};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

type Hist = BTreeMap<usize, u64>;
fn increment(hist: &mut Hist, key: usize) {
    *hist.entry(key).or_default() += 1;
}
fn sumset(base: &[u64], n: u64) -> usize {
    let mut sums = HashSet::new();
    for (i, &u) in base.iter().enumerate() {
        for &v in &base[i..] {
            sums.insert((u + v) % n);
        }
    }
    sums.len()
}

pub fn f3(e: &Curve, x: u64, y: u64, z: u64) -> u64 {
    let (x, y, z, a, b) = (
        x as i128,
        y as i128,
        z as i128,
        e.params.a as i128,
        e.params.b as i128,
    );
    norm(
        (x - y).pow(2) * z * z - 2 * ((x + y) * (x * y + a) + 2 * b) * z + (x * y - a).pow(2)
            - 4 * b * (x + y),
        e.params.p,
    )
}

fn group_triples(e: &mut Curve, points: &[Point], twist: bool) -> HashSet<[u64; 3]> {
    let mut triples = HashSet::new();
    let p = e.params.p;
    let transform = |x| if twist { (p - x) % p } else { x };
    for (i, &left) in points.iter().enumerate() {
        for &right in &points[i..] {
            let sums = [
                e.add(Some(left), Some(right)),
                e.add(Some(left), e.neg(Some(right))),
            ];
            for sum in sums.into_iter().flatten() {
                let mut xs = [transform(left.x), transform(right.x), transform(sum.x)];
                xs.sort();
                triples.insert(xs);
            }
        }
    }
    triples
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurveAudit {
    pub parameters: Parameters,
    pub group_order: u64,
    pub twist_order: u64,
    pub factor_bases: u64,
    pub plus_coverage_hist: Hist,
    pub signed_coverage_hist: Hist,
    pub best_plus_x: [u64; 3],
    pub best_signed_x: [u64; 3],
    pub max_twist_run_score: usize,
    pub twist_run_optimal_plus_hist: Hist,
    pub twist_run_optimal_signed_hist: Hist,
    pub score_vs_plus_hist: BTreeMap<usize, Hist>,
    pub f3_triples: u64,
    pub f3_zero_counts_by_original_points: Hist,
    pub f3_mixed_zero_count: u64,
}

fn audit_curve(e: &mut Curve, chars: &[i64], n: u64) -> CurveAudit {
    let p = e.params.p;
    assert!(!chars.contains(&0));
    let xs: Vec<_> = (0..p).filter(|&x| chars[x as usize] == 1).collect();
    let points: Vec<_> = xs.iter().map(|&x| e.point(x, true).unwrap()).collect();
    assert_eq!(points.len() as u64, (n - 1) / 2);
    // Oracle labels for exhaustive analysis only; the benchmark never sees them.
    let g = Some(points[0]);
    let mut log = HashMap::new();
    let mut current: Element = None;
    for k in 0..n {
        assert!(log.insert(current, k).is_none());
        current = e.add(current, g);
    }
    assert!(current.is_none());
    let logs: Vec<_> = points.iter().map(|point| log[&Some(*point)]).collect();
    for (i, &left) in points.iter().enumerate() {
        for &right in &points[i..] {
            assert_eq!(
                log[&e.add(Some(left), Some(right))],
                (log[&Some(left)] + log[&Some(right)]) % n
            );
        }
    }
    let scores: Vec<_> = xs
        .iter()
        .map(|&x| {
            (1..=8)
                .take_while(|j| chars[((x + j) % p) as usize] == -1)
                .count()
        })
        .collect();
    let mut ranked = scores.clone();
    ranked.sort_by(|a, b| b.cmp(a));
    let max_score = ranked[..3].iter().sum();
    let (mut plus_hist, mut signed_hist, mut top_plus, mut top_signed) =
        (Hist::new(), Hist::new(), Hist::new(), Hist::new());
    let mut score_bins = BTreeMap::<usize, Hist>::new();
    let (mut best_plus, mut best_signed) = (0, 0);
    let (mut best_plus_x, mut best_signed_x) = ([0; 3], [0; 3]);
    for i in 0..logs.len() {
        for j in i + 1..logs.len() {
            for k in j + 1..logs.len() {
                let base = [logs[i], logs[j], logs[k]];
                let plus = sumset(&base, n);
                let signed = sumset(
                    &[
                        base[0],
                        base[1],
                        base[2],
                        n - base[0],
                        n - base[1],
                        n - base[2],
                    ],
                    n,
                );
                let score = scores[i] + scores[j] + scores[k];
                increment(&mut plus_hist, plus);
                increment(&mut signed_hist, signed);
                increment(score_bins.entry(score).or_default(), plus);
                if score == max_score {
                    increment(&mut top_plus, plus);
                    increment(&mut top_signed, signed);
                }
                if plus > best_plus {
                    best_plus = plus;
                    best_plus_x = [xs[i], xs[j], xs[k]];
                }
                if signed > best_signed {
                    best_signed = signed;
                    best_signed_x = [xs[i], xs[j], xs[k]];
                }
            }
        }
    }
    let count = points.len() as u64;
    let bases = count * (count - 1) * (count - 2) / 6;
    assert_eq!(plus_hist.values().sum::<u64>(), bases);
    assert_eq!(signed_hist.values().sum::<u64>(), bases);
    assert!(best_plus <= 6.min(n as usize));
    assert!(best_signed <= 19.min(n as usize));
    let mut zeros = HashSet::new();
    let mut zero_counts = Hist::new();
    let mut mixed = 0;
    for x in 0..p {
        for y in x..p {
            for z in y..p {
                if f3(e, x, y, z) == 0 {
                    let original_count = [x, y, z]
                        .iter()
                        .filter(|&&v| chars[v as usize] == 1)
                        .count();
                    increment(&mut zero_counts, original_count);
                    if original_count == 1 || original_count == 2 {
                        mixed += 1;
                    }
                    zeros.insert([x, y, z]);
                }
            }
        }
    }
    let mut twist = Curve::new(p, e.params.a, (p - e.params.b) % p);
    let twist_points: Vec<_> = (0..p)
        .filter(|&x| chars[x as usize] == -1)
        .map(|x| {
            let x = (p - x) % p;
            assert_eq!(twist.character(x), 1);
            twist.point(x, true).unwrap()
        })
        .collect();
    let twist_order = 1 + 2 * twist_points.len() as u64;
    assert_eq!(n + twist_order, 2 * p + 2);
    let mut expected = group_triples(e, &points, false);
    expected.extend(group_triples(&mut twist, &twist_points, true));
    assert_eq!(
        zeros, expected,
        "S3 zero set disagrees with signed curve/twist group addition"
    );
    assert_eq!(mixed, 0);
    CurveAudit {
        parameters: e.params,
        group_order: n,
        twist_order,
        factor_bases: bases,
        plus_coverage_hist: plus_hist,
        signed_coverage_hist: signed_hist,
        best_plus_x,
        best_signed_x,
        max_twist_run_score: max_score,
        twist_run_optimal_plus_hist: top_plus,
        twist_run_optimal_signed_hist: top_signed,
        score_vs_plus_hist: score_bins,
        f3_triples: p * (p + 1) * (p + 2) / 6,
        f3_zero_counts_by_original_points: zero_counts,
        f3_mixed_zero_count: mixed,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelCount {
    pub p: u64,
    pub nonsingular: u64,
    pub audited: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Audit {
    pub schema: String,
    pub primes: Vec<u64>,
    pub models: Vec<ModelCount>,
    pub curves: Vec<CurveAudit>,
    pub all_nonzero_three_point_base_hist_by_order: BTreeMap<u64, Hist>,
}

pub fn run(primes: &[u64]) -> Audit {
    let mut result = Audit {
        schema: "twist-relations/exhaustive/v1".into(),
        primes: primes.into(),
        models: Vec::new(),
        curves: Vec::new(),
        all_nonzero_three_point_base_hist_by_order: BTreeMap::new(),
    };
    for &p in primes {
        let (mut nonsingular, start) = (0, result.curves.len());
        for a in 0..p {
            for b in 0..p {
                let mut e = Curve::new(p, a, b);
                if !e.nonsingular() {
                    continue;
                }
                nonsingular += 1;
                let chars: Vec<_> = (0..p).map(|x| e.character(x)).collect();
                let n = (p as i64 + 1 + chars.iter().sum::<i64>()) as u64;
                if n < 7 || !prime(n) {
                    continue;
                }
                result.curves.push(audit_curve(&mut e, &chars, n));
                result
                    .all_nonzero_three_point_base_hist_by_order
                    .entry(n)
                    .or_insert_with(|| {
                        let mut hist = Hist::new();
                        for i in 1..n {
                            for j in i + 1..n {
                                for k in j + 1..n {
                                    increment(&mut hist, sumset(&[i, j, k], n));
                                }
                            }
                        }
                        assert_eq!(hist.values().sum::<u64>(), (n - 1) * (n - 2) * (n - 3) / 6);
                        hist
                    });
            }
        }
        assert_eq!(nonsingular, p * (p - 1));
        result.models.push(ModelCount {
            p,
            nonsingular,
            audited: result.curves.len() - start,
        });
        eprintln!(
            "Audited all {} eligible coefficient models over F_{p}",
            result.curves.len() - start
        );
    }
    result
}
