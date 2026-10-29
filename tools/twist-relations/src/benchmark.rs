use crate::core::{discover, norm, power, Counts, Curve, Parameters, Point, Rng};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::Instant;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Phase {
    pub ns: u64,
    pub counts: Counts,
}
impl Phase {
    fn add(&mut self, other: Self) {
        self.ns += other.ns;
        self.counts.accumulate(other.counts);
    }
}
fn measured<T>(e: &mut Curve, f: impl FnOnce(&mut Curve) -> T) -> (T, Phase) {
    let before = e.counts;
    let clock = Instant::now();
    let result = f(e);
    (
        result,
        Phase {
            ns: clock.elapsed().as_nanos() as u64,
            counts: e.counts.difference(before),
        },
    )
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PointLog {
    pub point: Point,
    pub scalar: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Record {
    pub parameters: Parameters,
    pub group_order: u64,
    pub g: Point,
    pub q: Point,
    pub seed: u64,
    pub method: String,
    pub recovered_scalar: u64,
    pub factor_base_logs: Vec<PointLog>,
    pub factor_base_size: usize,
    pub total_ns: u64,
    pub discovery_ns_shared_excluded: u64,
    pub phases: BTreeMap<String, Phase>,
    pub counts: Counts,
    pub attempts: u64,
    pub relation_hits: u64,
    pub rank: usize,
    pub table_entries: usize,
}

fn make_base(e: &mut Curve, size: usize, method: &str, rng: &mut Rng) -> Vec<Point> {
    let p = e.params.p;
    if method == "twist_runs" {
        let chars: Vec<_> = (0..p).map(|x| e.character(x)).collect();
        let mut xs: Vec<_> = (0..p).filter(|&x| chars[x as usize] == 1).collect();
        rng.shuffle(&mut xs);
        xs.sort_by_key(|&x| {
            std::cmp::Reverse(
                (1..=8)
                    .take_while(|j| chars[((x + j) % p) as usize] == -1)
                    .count(),
            )
        });
        return xs[..size]
            .iter()
            .map(|&x| e.point(x, true).unwrap())
            .collect();
    }
    let mut seen = HashSet::new();
    let mut points = Vec::new();
    let mut x = rng.below(p);
    let step = rng.below(p - 1) + 1;
    while points.len() < size {
        if method == "random" {
            x = rng.below(p);
        }
        if !seen.contains(&x) {
            if let Some(point) = e.point(x, false) {
                seen.insert(x);
                points.push(point);
            }
        }
        if method == "progression" {
            x = (x + step) % p;
        }
    }
    points
}

fn bsgs(e: &mut Curve, g: Point, q: Point, n: u64) -> (u64, usize) {
    let width = (n as f64).sqrt() as u64 + 1;
    let mut table = HashMap::new();
    let mut point = None;
    for j in 0..width {
        table.entry(point).or_insert(j);
        point = e.add(point, Some(g));
    }
    let stride = e.mul(width, Some(g));
    let step = e.neg(stride);
    let mut current = Some(q);
    for i in 0..=width {
        e.counts.hash_lookups += 1;
        if let Some(&j) = table.get(&current) {
            let k = (i * width + j) % n;
            assert_eq!(e.mul(k, Some(g)), Some(q));
            return (k, table.len());
        }
        current = e.add(current, step);
    }
    panic!("BSGS failed on a verified prime-order fixture");
}

struct RelationResult {
    logs: Vec<u64>,
    phases: BTreeMap<String, Phase>,
    attempts: u64,
    hits: u64,
    rank: usize,
    entries: usize,
}
fn relations(
    e: &mut Curve,
    g: Point,
    q: Point,
    n: u64,
    base: &[Point],
    rng: &mut Rng,
) -> RelationResult {
    let size = base.len();
    let (pairs, setup) = measured(e, |e| {
        let mut pairs = HashMap::new();
        for i in 0..size {
            for j in i..size {
                pairs
                    .entry(e.add(Some(base[i]), Some(base[j])))
                    .or_insert((i, j));
            }
        }
        pairs
    });
    let (mut a, mut b) = (rng.below(n), rng.below(n));
    let (mut current, mut search) = measured(e, |e| {
        let left = e.mul(a, Some(g));
        let right = e.mul(b, Some(q));
        e.add(left, right)
    });
    let mut pivots: Vec<Option<Vec<u64>>> = vec![None; size + 1];
    let mut linear = Phase::default();
    let (mut attempts, mut hits, mut rank) = (0, 0, 0);
    while rank < size + 1 {
        attempts += 1;
        assert!(
            attempts <= 1000 * n,
            "Relation rank exceeded its declared attempt budget"
        );
        let step = if rng.below(2) == 1 {
            a = (a + 1) % n;
            g
        } else {
            b = (b + 1) % n;
            q
        };
        let (pair, phase) = measured(e, |e| {
            current = e.add(current, Some(step));
            e.counts.hash_lookups += 1;
            pairs.get(&current).copied()
        });
        search.add(phase);
        let Some((i, j)) = pair else {
            continue;
        };
        hits += 1;
        let clock = Instant::now();
        let mut row = vec![0; size + 2];
        row[i] += 1;
        row[j] += 1;
        row[size] = (n - b) % n;
        row[size + 1] = a;
        let mut inserted = false;
        for col in 0..=size {
            if row[col] == 0 {
                continue;
            }
            if let Some(old) = &pivots[col] {
                let scale = row[col];
                for k in col..size + 2 {
                    row[k] = norm(row[k] as i128 - scale as i128 * old[k] as i128, n);
                    linear.counts.linear_cells += 1;
                }
            } else {
                let inverse = power(row[col], n - 2, n);
                linear.counts.linear_inversions += 1;
                for value in &mut row[col..] {
                    *value = *value * inverse % n;
                    linear.counts.linear_cells += 1;
                }
                pivots[col] = Some(row.clone());
                rank += 1;
                inserted = true;
                break;
            }
        }
        if !inserted {
            assert_eq!(row[size + 1], 0, "Inconsistent relation");
        }
        linear.ns += clock.elapsed().as_nanos() as u64;
    }
    let clock = Instant::now();
    let mut logs = vec![0; size + 1];
    for col in (0..=size).rev() {
        let row = pivots[col].as_ref().unwrap();
        let sum: u64 = (col + 1..=size).map(|j| row[j] * logs[j] % n).sum();
        logs[col] = norm(row[size + 1] as i128 - sum as i128, n);
        linear.counts.linear_cells += (size - col) as u64;
    }
    linear.ns += clock.elapsed().as_nanos() as u64;
    let (_, verification) = measured(e, |e| {
        assert_eq!(e.mul(logs[size], Some(g)), Some(q));
        for (point, &k) in base.iter().zip(&logs) {
            assert_eq!(e.mul(k, Some(g)), Some(*point));
        }
    });
    RelationResult {
        logs,
        phases: BTreeMap::from([
            ("pair_setup".into(), setup),
            ("relation_search".into(), search),
            ("joint_linear_algebra".into(), linear),
            ("verification".into(), verification),
        ]),
        attempts,
        hits,
        rank,
        entries: pairs.len(),
    }
}

pub fn run(primes: &[u64], seeds: u64, seed_start: u64) -> Vec<Record> {
    let mut records = Vec::new();
    for &p in primes {
        let discovery = Instant::now();
        let (params, n, g) = discover(p);
        let discovery_ns = discovery.elapsed().as_nanos() as u64;
        let size = (n as f64).cbrt().round() as usize;
        for seed in seed_start..seed_start + seeds {
            let k = Rng(seed.wrapping_add(p).wrapping_add(10000)).below(n - 1) + 1;
            let q = Curve::new(p, params.a, params.b).mul(k, Some(g)).unwrap();
            for method in ["bsgs", "random", "progression", "twist_runs"] {
                let mut e = Curve::new(p, params.a, params.b);
                let clock = Instant::now();
                let (recovered, logs, phases, attempts, hits, rank, entries) = if method == "bsgs" {
                    let ((scalar, entries), phase) = measured(&mut e, |e| bsgs(e, g, q, n));
                    (
                        scalar,
                        Vec::new(),
                        BTreeMap::from([("direct_solve".into(), phase)]),
                        0,
                        0,
                        0,
                        entries,
                    )
                } else {
                    let (base, selection) = measured(&mut e, |e| {
                        make_base(e, size, method, &mut Rng(seed.wrapping_add(20000)))
                    });
                    let mut result =
                        relations(&mut e, g, q, n, &base, &mut Rng(seed.wrapping_add(30000)));
                    result.phases.insert("base_selection".into(), selection);
                    let logs = base
                        .iter()
                        .zip(&result.logs)
                        .map(|(&point, &scalar)| PointLog { point, scalar })
                        .collect();
                    (
                        result.logs[size],
                        logs,
                        result.phases,
                        result.attempts,
                        result.hits,
                        result.rank,
                        result.entries,
                    )
                };
                assert_eq!(recovered, k);
                let counts = phases.values().fold(Counts::default(), |mut c, phase| {
                    c.accumulate(phase.counts);
                    c
                });
                // Linear algebra works over Z/n separately from the curve counter.
                let mut expected = e.counts;
                expected.linear_cells = counts.linear_cells;
                expected.linear_inversions = counts.linear_inversions;
                assert_eq!(counts, expected);
                records.push(Record {
                    parameters: params,
                    group_order: n,
                    g,
                    q,
                    seed,
                    method: method.into(),
                    recovered_scalar: recovered,
                    factor_base_logs: logs,
                    factor_base_size: if method == "bsgs" { 0 } else { size },
                    total_ns: clock.elapsed().as_nanos() as u64,
                    discovery_ns_shared_excluded: discovery_ns,
                    phases,
                    counts,
                    attempts,
                    relation_hits: hits,
                    rank,
                    table_entries: entries,
                });
            }
        }
        eprintln!("Completed benchmark correctness checks for p={p}, n={n}");
    }
    records
}

pub fn verify(records: &[Record]) -> Result<(), String> {
    for row in records {
        let p = row.parameters;
        if !crate::valid_prime(p.p, 1_000_003) || p.a >= p.p || p.b >= p.p {
            return Err("Invalid curve parameters".into());
        }
        let mut e = Curve::new(p.p, p.a, p.b);
        if !e.nonsingular() {
            return Err("Invalid group".into());
        }
        let actual_order = (p.p as i64 + 1 + (0..p.p).map(|x| e.character(x)).sum::<i64>()) as u64;
        if actual_order != row.group_order
            || !crate::core::prime(actual_order)
            || !e.on_curve(Some(row.g))
            || !e.on_curve(Some(row.q))
            || row.recovered_scalar >= row.group_order
            || e.mul(row.recovered_scalar, Some(row.g)) != Some(row.q)
        {
            return Err("Target scalar replay failed".into());
        }
        if !["bsgs", "random", "progression", "twist_runs"].contains(&row.method.as_str())
            || row.factor_base_logs.len() != row.factor_base_size
        {
            return Err("Invalid method or base size".into());
        }
        if row.method == "bsgs" {
            if row.factor_base_size != 0 {
                return Err("Unexpected direct-solver base".into());
            }
        } else if row.factor_base_size == 0
            || row.factor_base_size > 101
            || row.rank != row.factor_base_size + 1
            || row.table_entries > row.factor_base_size * (row.factor_base_size + 1) / 2
        {
            return Err("Invalid relation/base metadata".into());
        }
        for log in &row.factor_base_logs {
            if !e.on_curve(Some(log.point))
                || log.scalar >= row.group_order
                || e.mul(log.scalar, Some(row.g)) != Some(log.point)
            {
                return Err("Base log replay failed".into());
            }
        }
        let mut total = Counts::default();
        for phase in row.phases.values() {
            total.accumulate(phase.counts);
        }
        if total != row.counts {
            return Err("Phase accounting does not reconcile".into());
        }
    }
    Ok(())
}
