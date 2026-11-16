//! Opt-in projection of the existing, verified cover/sieve linear system.
//! The target is always retained. Every graph output is replayed from the
//! selected input rows before it reaches Wiedemann. Unsupported projections
//! return None so the caller can charge the original solve as fallback.
use super::*;
use crate::cryptanalysis::large_prime_filter::{
    filter_large_primes, replay_complete_relation, CompleteRelation, LargePrimeFilterOptions,
    LargePrimeFilterReport, PartialRelation,
};

#[derive(Clone, Debug, Serialize)]
pub struct SieveGraphOptions {
    pub retained_percent: usize,
    pub max_small_weight: usize,
    pub max_sources: usize,
}
impl Default for SieveGraphOptions {
    fn default() -> Self {
        Self {
            retained_percent: 75,
            max_small_weight: 4096,
            max_sources: 4096,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SieveGraphStats {
    pub calls: u64,
    pub projected: u64,
    pub refusals: u64,
    pub fallback_solves: u64,
    pub replayed_rows: u64,
    pub input_rows: u64,
    pub output_rows: u64,
    pub filter_call_wall_ns: u128,
    pub input_digest: String,
    pub last_report: Option<LargePrimeFilterReport>,
    pub counter_scope: &'static str,
}
impl SieveGraphStats {
    pub(super) fn new() -> Self {
        Self { counter_scope: "complete wall includes graph mapping/filter/replay/drop; legacy la_ops/la_muls/total_muls omit graph modular and native work", ..Self::default() }
    }
}

fn replays_match(rows: &[CompleteRelation], inputs: &[PartialRelation], n: u64) -> bool {
    rows.iter().all(|row| {
        replay_complete_relation(row, inputs, n)
            .is_some_and(|replayed| replayed.small == row.small && replayed.rhs == row.rhs)
    })
}

pub(super) fn project(
    rows: &[SparseRel],
    n: u64,
    options: &SieveGraphOptions,
    stats: &mut SieveGraphStats,
) -> Option<(Vec<SparseRel>, usize)> {
    let started = Instant::now();
    stats.calls += 1;
    stats.input_rows += rows.len() as u64;
    let result = (|| {
        let width = rows.len();
        if width == 0
            || width > 4096
            || !(3..1u64 << 63).contains(&n)
            || n.is_multiple_of(2)
            || !(1..=100).contains(&options.retained_percent)
            || !(1..=4096).contains(&options.max_small_weight)
            || !(1..=4096).contains(&options.max_sources)
            || rows.iter().any(|r| r.cols.iter().any(|&(c, _)| c >= width))
            || rows.iter().map(|r| r.cols.len()).sum::<usize>() > 1_000_000
        {
            return None;
        }
        let target = width - 1;
        let keep = (target * options.retained_percent / 100)
            .max(usize::from(target > 0))
            .min(target);
        let mut digest = blake3::Hasher::new();
        digest.update(stats.input_digest.as_bytes());
        digest.update(&n.to_le_bytes());
        digest.update(&(width as u64).to_le_bytes());
        let inputs: Vec<_> = rows
            .iter()
            .enumerate()
            .map(|(id, row)| {
                digest.update(&(id as u64).to_le_bytes());
                digest.update(&row.rhs.to_le_bytes());
                digest.update(&(row.cols.len() as u64).to_le_bytes());
                let mut small = Vec::new();
                let mut large = Vec::new();
                for &(c, v) in &row.cols {
                    digest.update(&(c as u64).to_le_bytes());
                    digest.update(&v.to_le_bytes());
                    if c == target {
                        small.push((keep as u32, v));
                    } else if c < keep {
                        small.push((c as u32, v));
                    } else {
                        large.push(((c - keep) as u64, v));
                    }
                }
                PartialRelation {
                    id: id as u64,
                    small,
                    large,
                    rhs: row.rhs,
                }
            })
            .collect();
        stats.input_digest = digest.finalize().to_hex().to_string();
        let (complete, report) = filter_large_primes(
            inputs.clone(),
            &LargePrimeFilterOptions {
                modulus: n,
                max_small_weight: options.max_small_weight,
                max_sources: options.max_sources,
            },
        )
        .ok()?;
        stats.last_report = Some(report);
        if complete.len() != keep + 1 || !replays_match(&complete, &inputs, n) {
            return None;
        }
        stats.replayed_rows += complete.len() as u64;
        stats.output_rows += complete.len() as u64;
        Some((
            complete
                .into_iter()
                .map(|r| SparseRel {
                    cols: r.small.into_iter().map(|(c, v)| (c as usize, v)).collect(),
                    rhs: r.rhs,
                })
                .collect(),
            keep,
        ))
    })();
    stats.filter_call_wall_ns += started.elapsed().as_nanos();
    if result.is_some() {
        stats.projected += 1;
    } else {
        stats.refusals += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_keeps_the_target_and_unique_retained_solution() {
        for n in [7, 101, (1u64 << 61) - 1] {
            for width in [1, 4, 11, 32] {
                for retained_percent in [25, 75, 100] {
                    let mut rng = StdRng::seed_from_u64(719);
                    let values: Vec<_> = (0..width).map(|_| rng.gen_range(0..n)).collect();
                    let rows: Vec<_> = (0..width)
                        .map(|i| {
                            let mut cols = vec![(i, 1)];
                            for j in i + 1..width {
                                if rng.gen_bool(0.2) {
                                    cols.push((j, rng.gen_range(1..n)));
                                }
                            }
                            let rhs = cols.iter().fold(0, |s, &(c, v)| {
                                ((s as u128 + v as u128 * values[c] as u128) % n as u128) as u64
                            });
                            SparseRel { cols, rhs }
                        })
                        .collect();
                    let mut stats = SieveGraphStats::new();
                    let (projected, target) = project(
                        &rows,
                        n,
                        &SieveGraphOptions {
                            retained_percent,
                            ..Default::default()
                        },
                        &mut stats,
                    )
                    .unwrap();
                    let mut dense: Vec<_> = projected
                        .iter()
                        .map(|r| {
                            let mut row = vec![0; target + 2];
                            for &(c, v) in &r.cols {
                                row[c] = v;
                            }
                            row[target + 1] = r.rhs;
                            row
                        })
                        .collect();
                    let pivots =
                        crate::cryptanalysis::gaudry_cubic::rref_mod_p(&mut dense, n, &mut 0);
                    assert_eq!(pivots, (0..=target).collect::<Vec<_>>());
                    assert_eq!(dense[target][target + 1], values[width - 1]);
                    for i in 0..target {
                        assert_eq!(dense[i][target + 1], values[i]);
                    }
                    assert_eq!(stats.replayed_rows, projected.len() as u64);
                    assert_eq!(stats.refusals, 0);
                }
            }
        }
    }

    #[test]
    fn altered_output_cannot_pass_original_input_replay() {
        let inputs = vec![PartialRelation {
            id: 0,
            small: vec![(0, 1)],
            large: vec![],
            rhs: 3,
        }];
        let (mut complete, _) = filter_large_primes(
            inputs.clone(),
            &LargePrimeFilterOptions {
                modulus: 7,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(replays_match(&complete, &inputs, 7));
        complete[0].rhs = 4;
        assert!(!replays_match(&complete, &inputs, 7));
    }

    #[test]
    fn bounds_and_fill_refusals_return_to_the_original_solve() {
        let rows = vec![
            SparseRel {
                cols: vec![(0, 1), (1, 1), (2, 1)],
                rhs: 0,
            },
            SparseRel {
                cols: vec![(1, 1), (3, 1)],
                rhs: 0,
            },
            SparseRel {
                cols: vec![(2, 1), (3, 1)],
                rhs: 0,
            },
            SparseRel {
                cols: vec![(3, 1)],
                rhs: 0,
            },
        ];
        let mut stats = SieveGraphStats::new();
        assert!(project(
            &rows,
            7,
            &SieveGraphOptions {
                retained_percent: 25,
                max_sources: 1,
                ..Default::default()
            },
            &mut stats
        )
        .is_none());
        assert!(stats.last_report.as_ref().unwrap().rejected_fill > 0);
        assert!(project(&rows, 1u64 << 63, &SieveGraphOptions::default(), &mut stats).is_none());
        assert!(project(
            &rows,
            7,
            &SieveGraphOptions {
                retained_percent: 0,
                ..Default::default()
            },
            &mut stats
        )
        .is_none());
    }
}
