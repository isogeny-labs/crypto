//! Frozen-stream evaluator for the large-prime graph algorithms (catalog §8).
//!
//! One call runs every integrated algorithm, and both existing filters, on the
//! **same** relation stream and returns a serializable record with counts and
//! wall times. Rules from the catalog that this record keeps:
//!
//! * metadata timing alone never passes: the pivot and MST filters are run to
//!   completion and every emitted relation is replayed from its source witness;
//! * the exact gain-DSU rank is audited against dense elimination where the
//!   matrix is small enough, and against the conservative
//!   `StructuralRankBound` where its input contract holds;
//! * `N` and `Z` are recorded after filtering, with minimum-degree fill and
//!   width on the residual column graph;
//! * partition quality is reported as `Σ (λ_j − 1)` next to the contiguous
//!   baseline; it is a communication model, not a measured product time.
//!
//! Wall times are single-run local timings for orientation only. They are not
//! benchmark results and carry no claim about complete ECDLP cost.

use super::*;
use crate::cryptanalysis::large_prime_filter::{
    filter_large_primes, filter_large_primes_mst, replay_complete_relation, CompleteRelation,
    LargePrimeFilterOptions, LargePrimeFilterReport, MstLimits, MstStats, PartialRelation,
};
use std::time::Instant;

#[path = "../jv_large_prime_rank_bound.rs"]
#[allow(dead_code)]
mod structural_rank_bound;
use structural_rank_bound::StructuralRankBound;

pub const SCHEMA: &str = "lp-graph-eval/v1";

/// Evaluation knobs. Every field has a default.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct EvalParams {
    /// Dense audits run only when `R · C` is at most this.
    pub dense_audit_max_cells: usize,
    pub low_stretch: LowStretchParams,
    pub fm: FmParams,
    /// Partition the residual matrix into each of these part counts.
    pub partition_parts: Vec<usize>,
    pub max_fill: usize,
    /// Rows wider than this are skipped when building the column graph.
    pub column_graph_max_row: usize,
    pub filter_max_small_weight: usize,
    pub filter_max_sources: usize,
    pub mst_max_rows: usize,
    pub mst_max_column_degree: usize,
    pub mst_max_edge_evaluations: usize,
    pub mst_max_active_coefficients: usize,
}

impl Default for EvalParams {
    fn default() -> Self {
        let f = LargePrimeFilterOptions::default();
        let m = MstLimits::default();
        Self {
            dense_audit_max_cells: 4_000_000,
            low_stretch: LowStretchParams::default(),
            fm: FmParams::default(),
            partition_parts: vec![2, 4, 8],
            max_fill: 5_000_000,
            column_graph_max_row: 256,
            filter_max_small_weight: f.max_small_weight,
            filter_max_sources: f.max_sources,
            mst_max_rows: m.max_rows,
            mst_max_column_degree: m.max_column_degree,
            mst_max_edge_evaluations: m.max_edge_evaluations,
            mst_max_active_coefficients: m.max_active_coefficients,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankAudit {
    /// Rows with at most two LP entries (the gain-DSU domain).
    pub two_entry_rows: usize,
    /// `true` when every row is in the domain, so the DSU rank is the rank of `A_L`.
    pub exact_for_stream: bool,
    pub gain_dsu: GainDsuSnapshot,
    pub gain_dsu_ns: u128,
    /// Dense rank of the same rows, when small enough.
    pub dense_rank: Option<usize>,
    pub dense_matches: Option<bool>,
    /// `StructuralRankBound` lower bound on the same rows, when its contract holds.
    pub structural_lower_bound: Option<usize>,
    /// Hopcroft–Karp structural rank of the same rows.
    pub matching_rank: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoreSummary {
    pub lp_graph_max_core: usize,
    pub lp_graph_two_core_vertices: usize,
    pub incidence_two_core_rows: usize,
    pub incidence_two_core_columns: usize,
    pub ns: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatchingSummary {
    pub structural_rank: usize,
    pub phases: usize,
    pub hopcroft_karp_ns: u128,
    pub dm: DmDecomposition,
    pub dm_ns: u128,
    pub fine_block_count: usize,
    pub largest_fine_block: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TreeSummary {
    pub vertices: usize,
    pub edges: usize,
    pub bfs: CycleStats,
    pub bfs_ns: u128,
    pub low_stretch: CycleStats,
    pub low_stretch_ns: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilterSummary {
    pub report: LargePrimeFilterReport,
    pub mst_stats: Option<MstStats>,
    /// MST refused the projection atomically (`relations: None`).
    pub refused: bool,
    pub ns: u128,
    /// `N`: distinct factor-base columns in the output.
    pub residual_columns: usize,
    /// `Z`: factor-base nonzeros in the output.
    pub residual_nonzeros: usize,
    pub source_terms: usize,
    pub max_row_weight: usize,
    pub replayed: usize,
    pub replay_failures: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartitionSummary {
    pub parts: usize,
    pub block_connectivity_minus_one: usize,
    pub fm_connectivity_minus_one: usize,
    pub fm: Partition,
    pub fm_ns: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LpGraphEvaluation {
    pub schema: String,
    pub modulus: u64,
    pub label: String,
    /// `R`, `C`, `I` of the LP block.
    pub rows: usize,
    pub lp_columns: usize,
    pub incidences: usize,
    /// Rows by LP arity after canonicalization.
    pub arity_histogram: BTreeMap<usize, usize>,
    pub rank: RankAudit,
    pub cores: CoreSummary,
    pub matching: MatchingSummary,
    pub trees: TreeSummary,
    pub pivot_filter: FilterSummary,
    pub mst_filter: FilterSummary,
    /// Minimum-degree ordering on the residual column graph of the pivot filter's output.
    pub residual_elimination: EliminationStats,
    pub residual_elimination_ns: u128,
    pub residual_rows_skipped_for_width: usize,
    pub partitions: Vec<PartitionSummary>,
}

fn ns(t: Instant) -> u128 {
    t.elapsed().as_nanos()
}

fn summarize_output(
    out: &[CompleteRelation],
    inputs: &[PartialRelation],
    modulus: u64,
) -> (usize, usize, usize, usize, usize, usize) {
    let cols: BTreeSet<u32> = out.iter().flat_map(|r| r.small.iter().map(|&(c, _)| c)).collect();
    let z = out.iter().map(|r| r.small.len()).sum();
    let src = out.iter().map(|r| r.sources.len()).sum();
    let wmax = out.iter().map(|r| r.small.len()).max().unwrap_or(0);
    let mut ok = 0;
    let mut bad = 0;
    for r in out {
        match replay_complete_relation(r, inputs, modulus) {
            Some(replayed) if replayed.small == r.small && replayed.rhs == r.rhs => ok += 1,
            _ => bad += 1,
        }
    }
    (cols.len(), z, src, wmax, ok, bad)
}

/// Run every integrated algorithm on one frozen stream.
pub fn evaluate_stream(
    label: &str,
    relations: &[PartialRelation],
    modulus: u64,
    params: &EvalParams,
) -> Result<LpGraphEvaluation, LpGraphError> {
    let index = LpIndex::build(relations, modulus)?;
    let r = index.num_rows();
    let c = index.num_columns();
    let mut arity_histogram = BTreeMap::new();
    for row in &index.rows {
        *arity_histogram.entry(row.len()).or_insert(0) += 1;
    }

    // --- Rank: gain DSU on the two-entry domain, with audits.
    let domain: Vec<Vec<(usize, u64)>> = index.rows.iter().filter(|row| row.len() <= 2).cloned().collect();
    let t = Instant::now();
    let mut dsu = GainDsu::new(c, modulus)?;
    for row in &domain {
        dsu.add_row(row)?;
    }
    let gain = dsu.snapshot();
    let gain_ns = ns(t);
    let dense = (domain.len().saturating_mul(c) <= params.dense_audit_max_cells)
        .then(|| dense_rank(&domain, c, modulus))
        .transpose()?;
    let structural_lower_bound = (domain.len() <= 4096 && c <= 65536)
        .then(|| {
            let mut b = StructuralRankBound::new(c.max(1), modulus).ok()?;
            for row in &domain {
                let large: Vec<(u64, u64)> = row.iter().map(|&(col, a)| (col as u64, a)).collect();
                b.add_row(0, &large).ok()?;
            }
            Some(b.snapshot().large_rank_lower_bound)
        })
        .flatten();
    let domain_support: Vec<Vec<usize>> = domain.iter().map(|row| row.iter().map(|&(col, _)| col).collect()).collect();
    let matching_rank = hopcroft_karp(domain.len(), c, &domain_support).size;
    let rank = RankAudit {
        two_entry_rows: domain.len(),
        exact_for_stream: domain.len() == r,
        gain_dsu: gain,
        gain_dsu_ns: gain_ns,
        dense_rank: dense,
        dense_matches: dense.map(|d| d == gain.rank),
        structural_lower_bound,
        matching_rank,
    };

    // --- Cores.
    let t = Instant::now();
    let edges = index.two_entry_edges();
    let lp_adj = adjacency(c, &edges);
    let lp_core = core_numbers(&lp_adj);
    let inc_core = core_numbers(&incidence_adjacency(&index));
    let cores = CoreSummary {
        lp_graph_max_core: lp_core.iter().copied().max().unwrap_or(0),
        lp_graph_two_core_vertices: lp_core.iter().filter(|&&k| k >= 2).count(),
        incidence_two_core_rows: inc_core[..r].iter().filter(|&&k| k >= 2).count(),
        incidence_two_core_columns: inc_core[r..].iter().filter(|&&k| k >= 2).count(),
        ns: ns(t),
    };

    // --- Matching and Dulmage–Mendelsohn on the whole LP block.
    let support = index.row_supports();
    let t = Instant::now();
    let m = hopcroft_karp(r, c, &support);
    let hk_ns = ns(t);
    let t = Instant::now();
    let dm = dulmage_mendelsohn(&support, c, &m);
    let dm_ns = ns(t);
    let matching = MatchingSummary {
        structural_rank: m.size,
        phases: m.phases,
        hopcroft_karp_ns: hk_ns,
        fine_block_count: dm.fine_blocks.len(),
        largest_fine_block: dm.fine_blocks.iter().copied().max().unwrap_or(0),
        dm,
        dm_ns,
    };

    // --- Spanning trees of the LP graph and their fundamental cycles.
    let t = Instant::now();
    let bfs = bfs_forest(c, &edges);
    let bfs_stats = fundamental_cycle_length(c, &edges, &bfs);
    let bfs_ns = ns(t);
    let t = Instant::now();
    let ls = low_stretch_forest(c, &edges, params.low_stretch);
    let ls_stats = fundamental_cycle_length(c, &edges, &ls);
    let ls_ns = ns(t);
    let trees = TreeSummary {
        vertices: c,
        edges: edges.len(),
        bfs: bfs_stats,
        bfs_ns,
        low_stretch: ls_stats,
        low_stretch_ns: ls_ns,
    };

    // --- Existing filters on identical input.
    let options = LargePrimeFilterOptions {
        modulus,
        max_small_weight: params.filter_max_small_weight,
        max_sources: params.filter_max_sources,
    };
    let t = Instant::now();
    let (pivot_out, pivot_report) =
        filter_large_primes(relations.to_vec(), &options).map_err(|_| LpGraphError::InvalidModulus)?;
    let pivot_ns = ns(t);
    let (n_res, z_res, src, wmax, ok, bad) = summarize_output(&pivot_out, relations, modulus);
    let pivot_filter = FilterSummary {
        report: pivot_report,
        mst_stats: None,
        refused: false,
        ns: pivot_ns,
        residual_columns: n_res,
        residual_nonzeros: z_res,
        source_terms: src,
        max_row_weight: wmax,
        replayed: ok,
        replay_failures: bad,
    };
    let limits = MstLimits {
        max_rows: params.mst_max_rows,
        max_column_degree: params.mst_max_column_degree,
        max_edge_evaluations: params.mst_max_edge_evaluations,
        max_active_coefficients: params.mst_max_active_coefficients,
    };
    let t = Instant::now();
    let attempt =
        filter_large_primes_mst(relations.to_vec(), &options, &limits).map_err(|_| LpGraphError::InvalidModulus)?;
    let mst_ns = ns(t);
    let mst_out = attempt.relations.clone().unwrap_or_default();
    let (n2, z2, src2, w2, ok2, bad2) = summarize_output(&mst_out, relations, modulus);
    let mst_filter = FilterSummary {
        report: attempt.report,
        mst_stats: Some(attempt.stats),
        refused: attempt.relations.is_none(),
        ns: mst_ns,
        residual_columns: n2,
        residual_nonzeros: z2,
        source_terms: src2,
        max_row_weight: w2,
        replayed: ok2,
        replay_failures: bad2,
    };

    // --- Residual structure: minimum degree and partitioning.
    let col_of: BTreeMap<u32, usize> = pivot_out
        .iter()
        .flat_map(|row| row.small.iter().map(|&(col, _)| col))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, col)| (col, i))
        .collect();
    let residual_rows: Vec<Vec<usize>> = pivot_out
        .iter()
        .map(|row| row.small.iter().map(|&(col, _)| col_of[&col]).collect())
        .collect();
    let t = Instant::now();
    let (col_graph, skipped) = column_interaction_graph(&residual_rows, col_of.len(), params.column_graph_max_row);
    let residual_elimination = minimum_degree(&col_graph, params.max_fill);
    let residual_elimination_ns = ns(t);
    let partitions = params
        .partition_parts
        .iter()
        .filter(|&&k| k >= 2 && residual_rows.len() >= k)
        .map(|&k| {
            let block = block_partition(&residual_rows, col_of.len(), k);
            let t = Instant::now();
            let fm = fm_partition(&residual_rows, col_of.len(), FmParams { parts: k, ..params.fm });
            PartitionSummary {
                parts: k,
                block_connectivity_minus_one: block.connectivity_minus_one,
                fm_connectivity_minus_one: fm.connectivity_minus_one,
                fm,
                fm_ns: ns(t),
            }
        })
        .collect();

    Ok(LpGraphEvaluation {
        schema: SCHEMA.to_string(),
        modulus,
        label: label.to_string(),
        rows: r,
        lp_columns: c,
        incidences: index.incidences(),
        arity_histogram,
        rank,
        cores,
        matching,
        trees,
        pivot_filter,
        mst_filter,
        residual_elimination,
        residual_elimination_ns,
        residual_rows_skipped_for_width: skipped,
        partitions,
    })
}

/// A seeded synthetic relation stream. Large primes are drawn log-uniformly
/// from `1..=lp_columns` (density `∝ 1/q`, the usual large-prime shape); LP
/// arity follows `arity_weights`; coefficients are uniform nonzero mod `ℓ`.
/// Synthetic streams test the algorithms, not ECDLP relation statistics.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct SyntheticParams {
    pub seed: u64,
    pub rows: usize,
    pub lp_columns: u64,
    pub fb_columns: u32,
    pub fb_terms: usize,
    /// Weights of LP arity 0, 1, 2, 3, ...
    pub arity_weights: Vec<u64>,
    /// Use coefficient 1 on every LP entry (unweighted graph) instead of random.
    pub unit_coefficients: bool,
}

impl Default for SyntheticParams {
    fn default() -> Self {
        Self {
            seed: 1,
            rows: 4000,
            lp_columns: 20_000,
            fb_columns: 2000,
            fb_terms: 4,
            arity_weights: vec![1, 6, 12, 2],
            unit_coefficients: false,
        }
    }
}

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

pub fn synthetic_stream(params: &SyntheticParams, modulus: u64) -> Vec<PartialRelation> {
    let mut rng = SplitMix(params.seed);
    let total: u64 = params.arity_weights.iter().sum::<u64>().max(1);
    (0..params.rows)
        .map(|i| {
            let mut x = rng.next() % total;
            let mut arity = 0;
            for (a, &w) in params.arity_weights.iter().enumerate() {
                if x < w {
                    arity = a;
                    break;
                }
                x -= w;
            }
            let large = (0..arity)
                .map(|_| {
                    let q = (params.lp_columns as f64).powf(rng.unit()).floor().max(1.0) as u64;
                    let coef = if params.unit_coefficients { 1 } else { 1 + rng.next() % (modulus - 1) };
                    (q, coef)
                })
                .collect();
            let small = (0..params.fb_terms)
                .map(|_| ((rng.next() % params.fb_columns.max(1) as u64) as u32, 1 + rng.next() % (modulus - 1)))
                .collect();
            PartialRelation {
                id: i as u64,
                small,
                large,
                rhs: rng.next() % modulus,
            }
        })
        .collect()
}
