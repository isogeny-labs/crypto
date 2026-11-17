//! Opt-in projection of the existing, verified cover/sieve linear system.
//! The target is always retained. Every graph output is replayed from the
//! selected input rows before it reaches Wiedemann. Unsupported projections
//! return None so the caller can charge the original solve as fallback.
use super::*;
use crate::cryptanalysis::jv_bitops;
use crate::cryptanalysis::large_prime_filter::{
    filter_large_primes, replay_complete_relation, CompleteRelation, LargePrimeFilterOptions,
    LargePrimeFilterReport, PartialRelation,
};
use crate::cryptanalysis::large_prime_filter::{filter_large_primes_mst, MstLimits, MstStats};

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
    /// Last MST attempt only, including a refused attempt. Absent in old mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mst: Option<MstStats>,
    /// Refused MST output whose dimension * nonzeros exceeds the input.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub cost_guard_refusals: u64,
    /// Observed primitive subtotals per graph call, present only with hooks on.
    /// These do not assert complete native-bookkeeping coverage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_native: Option<Vec<serde_json::Value>>,
    pub counter_scope: &'static str,
}

struct ProjectionMeter {
    before: jv_bitops::JvTally,
    phase: &'static str,
    stages: Vec<serde_json::Value>,
    input_rows: usize,
    input_nonzeros: usize,
    candidate: Option<(usize, usize)>,
    call_index: u64,
}
impl ProjectionMeter {
    fn new(rows: &[SparseRel], call_index: u64) -> Option<Self> {
        jv_bitops::enabled().then(|| Self {
            before: jv_bitops::snapshot(),
            phase: "mapping",
            stages: Vec::new(),
            input_rows: rows.len(),
            input_nonzeros: rows.iter().map(|r| r.cols.len()).sum(),
            candidate: None,
            call_index,
        })
    }
    fn advance(&mut self, next: &'static str) {
        let after = jv_bitops::snapshot();
        self.stages.push(serde_json::json!({
            "stage":self.phase,
            "primitives":jv_bitops::prims_json(&after.since(&self.before).total())
        }));
        self.before = after;
        self.phase = next;
    }
    fn candidate(&mut self, rows: &[CompleteRelation]) {
        self.candidate = Some((rows.len(), rows.iter().map(|r| r.small.len()).sum()));
    }
    fn finish(mut self, accepted: bool) -> serde_json::Value {
        self.advance("finished");
        serde_json::json!({
            "schema":"cover.graph.projection-work/v1", "complete":false,
            "marker":"counted:graph_projection_subtotals",
            "call_index":self.call_index,
            "input_rows":self.input_rows, "input_nonzeros":self.input_nonzeros,
            "candidate_rows":self.candidate.map(|c|c.0),
            "candidate_nonzeros":self.candidate.map(|c|c.1),
            "accepted":accepted, "stages":self.stages,
            "known_uncounted":"graph native bookkeeping remains incomplete"
        })
    }
}
fn is_zero(v: &u64) -> bool {
    *v == 0
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
    project_mode(rows, n, options, stats, false)
}

pub(super) fn project_mst(
    rows: &[SparseRel],
    n: u64,
    options: &SieveGraphOptions,
    stats: &mut SieveGraphStats,
) -> Option<(Vec<SparseRel>, usize)> {
    project_mode(rows, n, options, stats, true)
}

fn project_mode(
    rows: &[SparseRel],
    n: u64,
    options: &SieveGraphOptions,
    stats: &mut SieveGraphStats,
    mst: bool,
) -> Option<(Vec<SparseRel>, usize)> {
    let started = Instant::now();
    let mut meter = ProjectionMeter::new(rows, stats.calls + 1);
    if mst {
        stats.counter_scope = "complete wall includes MST planning/normalization/filter/cost guard/replay/drop; legacy la_ops/la_muls/total_muls omit graph modular and native work";
    }
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
        let filter_options = LargePrimeFilterOptions {
            modulus: n,
            max_small_weight: options.max_small_weight,
            max_sources: options.max_sources,
        };
        if let Some(meter) = &mut meter {
            meter.advance("filter");
        }
        let complete = if mst {
            let attempt =
                filter_large_primes_mst(inputs.clone(), &filter_options, &MstLimits::default())
                    .ok()?;
            stats.last_report = Some(attempt.report);
            stats.mst = Some(attempt.stats);
            let projected = attempt.relations?;
            if let Some(meter) = &mut meter {
                meter.candidate(&projected);
            }
            let input_work =
                width as u128 * rows.iter().map(|r| r.cols.len() as u128).sum::<u128>();
            let projected_work = projected.len() as u128
                * projected
                    .iter()
                    .map(|r| r.small.len() as u128)
                    .sum::<u128>();
            if projected_work > input_work {
                stats.cost_guard_refusals += 1;
                return None;
            }
            projected
        } else {
            let (rows, report) = filter_large_primes(inputs.clone(), &filter_options).ok()?;
            stats.last_report = Some(report);
            if let Some(meter) = &mut meter {
                meter.candidate(&rows);
            }
            rows
        };
        if let Some(meter) = &mut meter {
            meter.advance("replay");
        }
        if complete.len() != keep + 1 || !replays_match(&complete, &inputs, n) {
            return None;
        }
        if let Some(meter) = &mut meter {
            meter.advance("output");
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
    if let Some(meter) = meter {
        stats
            .work_native
            .get_or_insert_with(Vec::new)
            .push(meter.finish(result.is_some()));
    }
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
    fn projection_subtotals_cover_the_call_and_refusals() {
        use jv_bitops::Phase;
        jv_bitops::enable(true);
        jv_bitops::claim();
        jv_bitops::set_moduli(103, 101);
        let rows = vec![
            SparseRel {
                cols: vec![(0, 1), (2, 2)],
                rhs: 12,
            },
            SparseRel {
                cols: vec![(1, 1), (2, 3)],
                rhs: 18,
            },
            SparseRel {
                cols: vec![(2, 7)],
                rhs: 35,
            },
            SparseRel {
                cols: vec![(3, 1)],
                rhs: 7,
            },
        ];
        let before = jv_bitops::snapshot();
        let _phase = jv_bitops::enter(Phase::LinearAlgebra);
        let mut stats = SieveGraphStats::new();
        assert!(project(&rows, 101, &SieveGraphOptions::default(), &mut stats).is_some());
        let expected = jv_bitops::prims_json(&jv_bitops::snapshot().since(&before).total());
        let call = &stats.work_native.as_ref().unwrap()[0];
        assert_eq!(call["complete"], false);
        assert_eq!(call["input_nonzeros"], 6);
        assert_eq!(call["candidate_rows"], 3);
        assert_eq!(call["candidate_nonzeros"], 3);
        let stages = call["stages"].as_array().unwrap();
        assert_eq!(
            stages
                .iter()
                .map(|s| s["stage"].as_str().unwrap())
                .collect::<Vec<_>>(),
            ["mapping", "filter", "replay", "output"]
        );
        for (k, value) in expected.as_object().unwrap() {
            let actual: u64 = stages
                .iter()
                .map(|s| s["primitives"][k].as_u64().unwrap())
                .sum();
            assert_eq!(actual, value.as_u64().unwrap(), "{k}");
        }
        assert!(stages[1]["primitives"]["modr_mul"].as_u64().unwrap() > 0);
        assert!(stages[2]["primitives"]["modr_mul"].as_u64().unwrap() > 0);
        assert!(project(&[], 101, &SieveGraphOptions::default(), &mut stats).is_none());
        let rejected = &stats.work_native.as_ref().unwrap()[1];
        assert_eq!(rejected["accepted"], false);
        assert!(rejected["candidate_rows"].is_null());
        assert_eq!(rejected["stages"].as_array().unwrap().len(), 1);
        assert_eq!(rejected["stages"][0]["stage"], "mapping");
    }

    #[test]
    fn mst_projects_a_weighted_system_and_keeps_the_target() {
        let p = 101;
        let rows = vec![
            SparseRel {
                cols: vec![(0, 1), (2, 2)],
                rhs: 12,
            },
            SparseRel {
                cols: vec![(1, 1), (2, 3)],
                rhs: 18,
            },
            SparseRel {
                cols: vec![(2, 7)],
                rhs: 35,
            },
            SparseRel {
                cols: vec![(3, 1)],
                rhs: 7,
            },
        ];
        let mut stats = SieveGraphStats::new();
        let (out, target) =
            project_mst(&rows, p, &SieveGraphOptions::default(), &mut stats).unwrap();
        assert_eq!(target, 2);
        assert_eq!(out.len(), 3);
        assert_eq!(stats.replayed_rows, 3);
        assert_eq!(stats.projected, 1);
        assert_eq!(stats.cost_guard_refusals, 0);
        let mst = stats.mst.as_ref().unwrap();
        assert_eq!(mst.trees, 1);
        assert_eq!(mst.selected_edges, 2);
        assert_eq!(mst.normalized_rows, 3);
        let values = [2, 3, 7];
        for row in &out {
            let rhs = row.cols.iter().fold(0u64, |s, &(i, v)| {
                ((s as u128 + v as u128 * values[i] as u128) % p as u128) as u64
            });
            assert_eq!(rhs, row.rhs);
        }
    }

    #[test]
    fn unused_mst_fields_preserve_original_default_report_serialization() {
        let value = serde_json::to_value(SieveGraphStats::new()).unwrap();
        assert!(value.get("mst").is_none());
        assert!(value.get("cost_guard_refusals").is_none());
        assert!(value.get("work_native").is_none());
    }

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
