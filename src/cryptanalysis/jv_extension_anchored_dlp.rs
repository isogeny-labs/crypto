//! Unified extension-field graph-IC discrete-logarithm pipeline supporting:
//! - 1-LP (`SparseTable`, `SparseTableFast`) and 2-LP (`DeferredPairs`, `DeferredPairsFast`)
//!   relation collectors with certificate-preserving `O(1)` root-index and inversion-free
//!   ordinate extraction;
//! - optional frustrated odd-cycle half-anchor harvesting (`scale = 2`);
//! - cold per-instance F4 trace replay composed with graph-anchored multi-pattern
//!   quadratic descent (`(1^6) + (1^4, 2) + (1^2, 2^2) + (2^3)`);
//! - compact (`< 64 KiB`) self-certifying proof-cone bundles (`AnchoredSolveBundle`)
//!   verifiable from first principles without re-running Phase-1 sieving; and
//! - matched negation-map Pollard rho references under both schoolbook and
//!   Karatsuba + Chung-Hasan `F_{p^6}` towers.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::time::Instant;

use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::deferred_pair_graph as deferred;
use super::extension_deferred_pair_dlp::{self as deferred_dlp, Extraction};
use super::extension_sparse_dlp as sparse_dlp;
use super::homogeneous_quotient::{ExtraCounts, Quotient};
use super::principal_relation as remainder;
use super::quadratic_pair_graph as graph_core;
use super::quadratic_pair_relation::{self as pair, Certificate as PairCert, SplitOptions};
use super::quartic_discriminant as discriminant;
use super::sparse_collection_rank::{Options as RankOptions, Tracker};
use crate::cryptanalysis::jv_anchored_descent::{
    count_quadratic_universe, decompose_residual_anchored_ref, harvest_frustrated_cycle_anchors,
    AnchorRecord, AnchorTable, AnchoredDescentCert, CycleHarvestStats, DescentMode,
    LocalTraceState, PatternHistogram, UnanchoredEdge, UniverseCensus,
};
use crate::cryptanalysis::jv_bitops::{self as work, Kind, OpTally, Phase};
use crate::cryptanalysis::jv_cover::{self as cover, Ctx, EllE, Fld, Fq, Spec, E2};
use crate::cryptanalysis::jv_rho_strong::{
    self as rho_strong, CurveInstance, Fq3Prims, StrongOpts,
};
use crate::cryptanalysis::jv_sieve::{
    self as sieve,
    two_large_prime::{CountedRelationSystem, TwoLargePrimeSieve},
    Counters, FpRing, HitBuckets, Line,
};
use crate::cryptanalysis::jv_signed_matroid_graph::{
    MatroidOptions, MatroidStats, SignedMatroidGraph,
};
use crate::cryptanalysis::large_prime_filter::{
    filter_large_primes, filter_large_primes_mst_with, LargePrimeFilterOptions, MstLimits,
    MstStrategy, PartialRelation,
};
use crate::cryptanalysis::large_prime_graph::{witness::collect_witnesses_lazy, LpIndex};
use crate::cryptanalysis::residual_walk::is_prime_u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CollectorArm {
    SparseTable,
    SparseTableFast,
    DeferredPairs,
    DeferredPairsFast,
    #[serde(rename = "signed-matroid-2lp")]
    SignedMatroid2lp,
    #[serde(rename = "signed-matroid-3lp")]
    SignedMatroid3lp,
}

impl CollectorArm {
    pub fn is_two_lp(self) -> bool {
        matches!(self, Self::DeferredPairs | Self::DeferredPairsFast)
    }

    pub fn is_matroid(self) -> bool {
        matches!(self, Self::SignedMatroid2lp | Self::SignedMatroid3lp)
    }

    pub fn max_arity(self) -> usize {
        match self {
            Self::SparseTable | Self::SparseTableFast => 1,
            Self::DeferredPairs | Self::DeferredPairsFast | Self::SignedMatroid2lp => 2,
            Self::SignedMatroid3lp => 3,
        }
    }

    pub fn is_fast(self) -> bool {
        matches!(
            self,
            Self::SparseTableFast
                | Self::DeferredPairsFast
                | Self::SignedMatroid2lp
                | Self::SignedMatroid3lp
        )
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::SparseTable => "sparse-table",
            Self::SparseTableFast => "sparse-table-fast",
            Self::DeferredPairs => "deferred-pairs",
            Self::DeferredPairsFast => "deferred-pairs-fast",
            Self::SignedMatroid2lp => "signed-matroid-2lp",
            Self::SignedMatroid3lp => "signed-matroid-3lp",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AnchoredPipelineOptions {
    pub collector: CollectorArm,
    pub descent: DescentMode,
    pub harvest_odd_cycles: bool,
    pub b_count: Option<u128>,
    pub max_sources: usize,
    pub max_vertices: usize,
    pub max_deferred_edges: usize,
    pub max_witness_terms: usize,
    pub max_screen_refusals: usize,
    pub max_descent_residuals: u64,
    pub max_accepted_descents: usize,
    pub sparse_rank: RankOptions,
    pub split: SplitOptions,
}

impl Default for AnchoredPipelineOptions {
    fn default() -> Self {
        Self {
            collector: CollectorArm::SignedMatroid2lp,
            descent: DescentMode::TracedAnchored,
            harvest_odd_cycles: false,
            b_count: None,
            max_sources: 262_144,
            max_vertices: 262_144,
            max_deferred_edges: 262_144,
            max_witness_terms: 256,
            max_screen_refusals: 500_000,
            max_descent_residuals: 8_192,
            max_accepted_descents: 16,
            sparse_rank: RankOptions::default(),
            split: SplitOptions::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReferencedSource {
    pub source_id: usize,
    pub terms: Vec<(usize, i8)>,
    /// Up to two canonical degree-2 remainders `(key, coefficient)`.
    pub remainders: Vec<([u64; 6], i8)>,
    pub a: Vec<[u64; 2]>,
    pub c: Vec<[u64; 2]>,
    pub norm: Vec<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompactAffineRow {
    pub nonzeros: Vec<(usize, u64)>,
    pub target_coeff: u64,
    pub rhs: u64,
    pub projected: [u64; 2],
    pub projected_rhs: u64,
    pub solver_rank: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhaseAccounting {
    pub phase: String,
    pub field_mul: u64,
    pub field_add: u64,
    pub field_inv: u64,
    pub modr_mul: u64,
    pub modr_add: u64,
    pub modr_inv: u64,
    pub word_op: u64,
    pub hash_probe: u64,
    pub rng_words: u64,
    pub bitops: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SensitivityBand {
    pub gae_bitops: u128,
    pub w0_bitops: u128,
    pub w0_1_bitops: u128,
    pub w1_bitops: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountingReport {
    pub phases: Vec<PhaseAccounting>,
    pub observed_bitops: u128,
    pub uniform_priced_bitops: u128,
    pub uniform_unpriced_ops: u64,
    pub foreign_hook_calls: u64,
    pub schoolbook_band: SensitivityBand,
    pub karatsuba_band: SensitivityBand,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollectionSummary {
    pub b_space_total: u128,
    pub b_count_scanned: u64,
    pub lines_tested: u64,
    pub candidates_tested: u64,
    pub candidates_refused: u64,
    pub duplicates_skipped: u64,
    pub sources_admitted: usize,
    pub full_sources: u64,
    pub one_class_sources: u64,
    pub two_class_sources: u64,
    #[serde(default)]
    pub three_class_sources: u64,
    pub discriminant_screen_calls: u64,
    pub discriminant_screen_refusals: u64,
    pub projections_emitted: usize,
    pub rank_achieved: usize,
    pub rank_target: usize,
    pub rank_ready: bool,
    pub stop_reason: String,
    pub graph_vertices: usize,
    pub unit_anchors: usize,
    pub half_anchors: usize,
    pub total_anchors: usize,
    pub live_deferred_edges: usize,
    pub cycle_harvest: Option<CycleHarvestStats>,
    #[serde(default)]
    pub matroid_stats: Option<MatroidStats>,
    pub sources_sha256: String,
    pub projections_sha256: String,
    pub kernel_orthogonality_verified_rows: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DescentSummary {
    pub mode: DescentMode,
    pub residuals_tried: u64,
    pub stream_advances: u64,
    pub accepted_rows: usize,
    pub trace_recorded: u64,
    pub trace_replayed: u64,
    pub trace_fallbacks: u64,
    pub timeouts: u64,
    pub incomplete: u64,
    pub f4_muls_total: u64,
    pub lin_muls_total: u64,
    pub weil_muls_total: u64,
    pub root_muls_total: u64,
    pub anchor_coverage_f: f64,
    pub predicted_multiplier_m: f64,
    pub pattern_histogram: PatternHistogram,
}

/// Compact (`< 64 KiB`) self-certifying bundle for one cold discrete-logarithm run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnchoredSolveBundle {
    pub schema: String,
    pub p: u64,
    pub l: u64,
    pub seed: u64,
    pub arm_label: String,
    pub options: AnchoredPipelineOptions,
    pub instance_alpha: [[u64; 2]; 3],
    pub instance_g: ([[u64; 2]; 3], [[u64; 2]; 3]),
    pub instance_q: ([[u64; 2]; 3], [[u64; 2]; 3]),
    pub cover_hx: Vec<[u64; 2]>,
    pub gj_u: Vec<[u64; 2]>,
    pub gj_v: Vec<[u64; 2]>,
    pub qj_u: Vec<[u64; 2]>,
    pub qj_v: Vec<[u64; 2]>,
    /// Compact factor-base points `[x, y_0, y_1]`.
    pub base_points: Vec<[u64; 3]>,
    pub universe_census: UniverseCensus,
    pub collection: CollectionSummary,
    pub initial_kernel: Option<Vec<u64>>,
    pub descent: DescentSummary,
    pub referenced_sources: BTreeMap<usize, ReferencedSource>,
    pub descent_certs: Vec<AnchoredDescentCert>,
    pub affine_rows: Vec<CompactAffineRow>,
    pub recovered_scalar: Option<u64>,
    pub verified: bool,
    pub refusal: Option<String>,
    pub wall_ms_indicative: f64,
    pub phase_wall_ms_indicative: BTreeMap<String, f64>,
    pub accounting: AccountingReport,
}

fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    work::hook_mul(m);
    ((a as u128 * b as u128) % m as u128) as u64
}

fn add_mod(a: u64, b: u64, m: u64) -> u64 {
    work::hook_add(m);
    ((a as u128 + b as u128) % m as u128) as u64
}

fn encode_e6(x: &cover::E6) -> [[u64; 2]; 3] {
    [x.0[0].0, x.0[1].0, x.0[2].0]
}

fn gae_bitops_from_prims(pr: &Fq3Prims, p: u64, l: u64) -> u128 {
    // 1 affine E(F_{p^6}) addition: 2 F_{q^3} muls + 1 F_{q^3} sq + 1 F_{q^3} inv + 42 F_p adds.
    let mut t = OpTally::default();
    for k in Kind::ALL {
        let i = k as usize;
        t.counts[i] = 2 * pr.mul.counts[i] + pr.sq.counts[i] + pr.inv.counts[i];
    }
    t.counts[Kind::FieldAdd as usize] += 42;
    let widths = work::widths(p, l);
    work::price(&t, &widths).expect("registered widths")
}

fn build_band(lower: u128, unpriced_ops: u64, gae: u128) -> SensitivityBand {
    let u = unpriced_ops as u128;
    let w0_1 = lower + (u * gae) / 10;
    let w1 = lower + u * gae;
    SensitivityBand {
        gae_bitops: gae,
        w0_bitops: lower,
        w0_1_bitops: w0_1,
        w1_bitops: w1,
    }
}

/// Fast `O(1)` root-index + inversion-free ordinate extractor for 1-LP (`SparseTableFast`).
fn candidate_sparse_fast(
    ctx: &Ctx,
    ring: &FpRing,
    config: &TwoLargePrimeSieve,
    base_by_x: &[u32],
    line: &Line,
    s: u64,
) -> Result<remainder::Certificate, &'static str> {
    let f = &ctx.f.f;
    if s == 0 || s >= f.p {
        return Err("zero/out-of-range sieve parameter");
    }
    let norm = sieve::line_poly(ring, line, s);
    if norm.len() != 10 {
        return Err("degree nine required");
    }
    let roots = ring.roots_by_table(&norm);
    let mut hit_indices: Vec<usize> = Vec::with_capacity(roots.len());
    for &x in &roots {
        work::words(1);
        let idx = *base_by_x.get(x as usize).unwrap_or(&u32::MAX);
        if idx != u32::MAX {
            hit_indices.push(idx as usize);
        }
    }
    if !matches!(hit_indices.len(), 7 | 9) {
        return Err("degree/canonical/support bounds");
    }
    hit_indices.sort_unstable();

    let base = config.base();
    let mut tail = norm.clone();
    for &i in &hit_indices {
        let bx = base[i].x;
        let divisor = vec![if bx == 0 { 0 } else { f.p - bx }, 1];
        let (quotient, residue) = ring.divrem(&tail, &divisor);
        if !residue.is_empty() {
            return Err("root division");
        }
        tail = quotient;
        if remainder::eval(f, &line.b, bx) == E2::ZERO {
            return Err("zero denominator");
        }
    }

    // Screen quadratic tail irreducibility over F_p BEFORE constructing F_{p^2} polynomials `a, c`.
    if tail.len() == 3 {
        if tail[2] == 0 {
            return Err("quadratic tail");
        }
        let p = f.p;
        work::hook_mul(p);
        work::hook_mul(p);
        work::hook_add(p);
        let t1_sq = (tail[1] as u128 * tail[1] as u128) % p as u128;
        let four_t2_t0 = (4u128 * ((tail[2] as u128 * tail[0] as u128) % p as u128)) % p as u128;
        let disc = ((t1_sq + p as u128 - four_t2_t0) % p as u128) as u64;
        if disc == 0 {
            return Err("irreducible Fp quadratic required");
        }
        let leg = f.pow(&f.from_fp(disc), (p - 1) as u128 / 2);
        if leg != f.from_fp(p - 1) {
            return Err("irreducible Fp quadratic required");
        }
    } else if tail.len() != 1 || tail[0] == 0 {
        return Err("full support tail");
    }

    let setup = config.setup();
    let sq = if setup.sqrt[s as usize] != u64::MAX {
        E2([setup.sqrt[s as usize], 0])
    } else {
        let r = setup.sqrt[mul_mod(s, setup.winv, f.p) as usize];
        if r == u64::MAX {
            return Err("square root table");
        }
        E2([0, r])
    };
    let a: Vec<E2> = (0..line.a0.len().max(line.a1.len()))
        .map(|i| {
            E2([
                *line.a0.get(i).unwrap_or(&0),
                mul_mod(s, *line.a1.get(i).unwrap_or(&0), f.p),
            ])
        })
        .collect();
    let c: Vec<E2> = line.b.iter().map(|b| f.mul(&sq, b)).collect();

    let mut terms = Vec::with_capacity(hit_indices.len());
    for &i in &hit_indices {
        let b = &base[i];
        let den = remainder::eval(f, &c, b.x);
        if den == E2::ZERO {
            return Err("zero denominator");
        }
        let ax = remainder::eval(f, &a, b.x);
        let dy = f.mul(&den, &b.y);
        let sign = if f.add(&ax, &dy) == E2::ZERO {
            1
        } else if f.sub(&ax, &dy) == E2::ZERO {
            -1
        } else {
            return Err("ordinate mismatch");
        };
        terms.push((i, sign));
    }

    let checked = remainder::check_equation(
        f,
        &ctx.cov.hx,
        base,
        &remainder::trim(a),
        &remainder::trim(c),
        &norm,
        &terms,
        &tail,
    )?;
    if checked.certificate().remainder.is_none()
        && !remainder::replay_group(&ctx.jac(), base, checked.certificate())
    {
        return Err("full Jacobian check");
    }
    Ok(checked.into_certificate())
}

/// Fast `O(1)` root-index + tail-first screening + inversion-free ordinate extractor
/// for 2-LP (`DeferredPairsFast`), preserving exact RNG synchronization with `DeferredPairs`.
#[allow(clippy::too_many_arguments)]
fn candidate_deferred_fast(
    ctx: &Ctx,
    ring: &FpRing,
    config: &TwoLargePrimeSieve,
    base_by_x: &[u32],
    line: &Line,
    s: u64,
    extraction: &mut Extraction<'_>,
    screen_ctx: Option<&discriminant::Context<'_>>,
    screen_calls: &mut u64,
    screen_refusals: &mut u64,
) -> Result<(PairCert, Vec<graph_core::LargeTerm>), &'static str> {
    extraction.trace = None;
    let f = &ctx.f.f;
    if s == 0 || s >= f.p {
        return Err("zero/out-of-range sieve parameter");
    }
    let norm = sieve::line_poly(ring, line, s);
    if norm.len() != 10 {
        return Err("degree nine required");
    }
    let roots = ring.roots_by_table(&norm);
    let mut hit_indices: Vec<usize> = Vec::with_capacity(roots.len());
    for &x in &roots {
        work::words(1);
        let idx = *base_by_x.get(x as usize).unwrap_or(&u32::MAX);
        if idx != u32::MAX {
            hit_indices.push(idx as usize);
        }
    }
    if !matches!(hit_indices.len(), 5 | 7 | 9) {
        return Err("norm support is not nine/seven/five distinct small points");
    }
    hit_indices.sort_unstable();

    let base = config.base();
    let mut tail = norm.clone();
    for &i in &hit_indices {
        let bx = base[i].x;
        let divisor = vec![if bx == 0 { 0 } else { f.p - bx }, 1];
        let (quotient, residue) = ring.divrem(&tail, &divisor);
        if !residue.is_empty() {
            return Err("root division");
        }
        tail = quotient;
        // Exact pre-screen check of `C(bx) = sqrt(s) * B(bx) == 0` so `split_rng`
        // advances on the exact same candidates as `DeferredPairs`.
        if remainder::eval(f, &line.b, bx) == E2::ZERO {
            return Err("zero denominator");
        }
    }

    let factors = match tail.len() {
        1 => Vec::new(),
        3 => vec![ring.monic(&tail)],
        5 => {
            if let Some(sc) = screen_ctx {
                *screen_calls += 1;
                let assessment = sc.screen(&tail)?;
                if assessment.rejects() {
                    *screen_refusals += 1;
                    return Err("quartic discriminant necessary condition");
                }
            }
            let report = pair::split_quartic(ring, &tail, extraction.options, extraction.rng)?;
            let factors = report.factors.clone();
            extraction.trace = Some((tail, report));
            factors
                .ok_or("quartic split refused")?
                .into_iter()
                .collect()
        }
        _ => return Err("norm support is not nine/seven/five distinct small points"),
    };

    // Only accepted tails (~30% of hits) build `a`, `c`, and evaluate ordinates!
    let setup = config.setup();
    let sq = if setup.sqrt[s as usize] != u64::MAX {
        E2([setup.sqrt[s as usize], 0])
    } else {
        let r = setup.sqrt[mul_mod(s, setup.winv, f.p) as usize];
        if r == u64::MAX {
            return Err("square root table");
        }
        E2([0, r])
    };
    let a: Vec<E2> = (0..line.a0.len().max(line.a1.len()))
        .map(|i| {
            E2([
                *line.a0.get(i).unwrap_or(&0),
                mul_mod(s, *line.a1.get(i).unwrap_or(&0), f.p),
            ])
        })
        .collect();
    let c: Vec<E2> = line.b.iter().map(|b| f.mul(&sq, b)).collect();

    let mut terms = Vec::with_capacity(hit_indices.len());
    for &i in &hit_indices {
        let b = &base[i];
        let den = remainder::eval(f, &c, b.x);
        if den == E2::ZERO {
            return Err("zero denominator");
        }
        let ax = remainder::eval(f, &a, b.x);
        let dy = f.mul(&den, &b.y);
        let sign = if f.add(&ax, &dy) == E2::ZERO {
            1
        } else if f.sub(&ax, &dy) == E2::ZERO {
            -1
        } else {
            return Err("ordinate mismatch");
        };
        terms.push((i, sign));
    }

    let checked = pair::check_equation(
        f,
        &ctx.cov.hx,
        base,
        &remainder::trim(a),
        &remainder::trim(c),
        &norm,
        &terms,
        &factors,
    )?;
    if checked.certificate().remainders.is_empty()
        && !pair::replay_group(&ctx.jac(), base, checked.certificate())
    {
        return Err("full Jacobian check");
    }
    let large = checked.large_terms();
    Ok((checked.into_certificate(), large))
}

type PairKey = (Vec<(usize, i8)>, Vec<([u64; 6], i8)>);

fn pair_source_key(row: &PairCert) -> PairKey {
    let orientation = row
        .remainders
        .first()
        .map_or(row.terms[0].1, |r| r.coefficient);
    work::probe((row.terms.len() * 2 + row.remainders.len() * 7) as u64);
    (
        row.terms
            .iter()
            .map(|&(i, s)| (i, s * orientation))
            .collect(),
        row.remainders
            .iter()
            .map(|r| (r.key, r.coefficient * orientation))
            .collect(),
    )
}

fn project_sources(
    sources: &[ReferencedSource],
    witness: &[(usize, i8)],
    l: u64,
) -> Result<Vec<(usize, u64)>, String> {
    if witness.is_empty() {
        return Err("empty witness".into());
    }
    let mut small = BTreeMap::<usize, i64>::new();
    let mut large = BTreeMap::<[u64; 6], i64>::new();
    for &(id, coeff) in witness {
        let src = sources.get(id).ok_or("witness source index")?;
        for &(i, sign) in &src.terms {
            *small.entry(i).or_default() += coeff as i64 * sign as i64;
            work::words(4);
            work::probe(1);
        }
        for &(key, r_coeff) in &src.remainders {
            *large.entry(key).or_default() += coeff as i64 * r_coeff as i64;
            work::words(4);
            work::probe(6);
        }
    }
    if large.values().any(|&v| v != 0) {
        return Err("nonzero integer cofactor incidence".into());
    }
    Ok(small
        .into_iter()
        .filter_map(|(i, x)| {
            work::words(2);
            let v = (x as i128).rem_euclid(l as i128) as u64;
            (v != 0).then_some((i, v))
        })
        .collect())
}

fn project_sources_i16(
    sources: &[ReferencedSource],
    witness: &[(usize, i16)],
    l: u64,
    small_buf: &mut Vec<(usize, i64)>,
    large_buf: &mut Vec<([u64; 6], i64)>,
) -> Result<Vec<(usize, u64)>, String> {
    if witness.is_empty() {
        return Err("empty witness".into());
    }
    small_buf.clear();
    large_buf.clear();
    for &(id, coeff) in witness {
        let src = sources.get(id).ok_or("witness source index")?;
        let c64 = coeff as i64;
        for &(i, sign) in &src.terms {
            small_buf.push((i, c64 * (sign as i64)));
            work::words(3);
        }
        for &(key, r_coeff) in &src.remainders {
            large_buf.push((key, c64 * (r_coeff as i64)));
            work::words(4);
        }
    }
    // Verify exact integer cancellation of all large classes over Z.
    large_buf.sort_unstable_by_key(|a| a.0);
    work::words((large_buf.len() * 4) as u64);
    let mut idx = 0usize;
    while idx < large_buf.len() {
        let key = large_buf[idx].0;
        let mut acc = 0i64;
        while idx < large_buf.len() && large_buf[idx].0 == key {
            acc += large_buf[idx].1;
            idx += 1;
            work::words(2);
        }
        if acc != 0 {
            return Err("nonzero integer cofactor incidence".into());
        }
    }
    // Fold small factor-base columns modulo `l`.
    small_buf.sort_unstable_by_key(|&(i, _)| i);
    work::words((small_buf.len() * 3) as u64);
    let mut out = Vec::with_capacity(small_buf.len());
    let mut s_idx = 0usize;
    while s_idx < small_buf.len() {
        let col = small_buf[s_idx].0;
        let mut acc = 0i128;
        while s_idx < small_buf.len() && small_buf[s_idx].0 == col {
            acc += small_buf[s_idx].1 as i128;
            s_idx += 1;
            work::words(2);
        }
        let v = acc.rem_euclid(l as i128) as u64;
        work::words(2);
        if v != 0 {
            out.push((col, v));
        }
    }
    Ok(out)
}

#[inline]
fn add_mod_fp(a: u64, b: u64, p: u64) -> u64 {
    work::hook_add(p);
    let s = a + b;
    if s >= p {
        s - p
    } else {
        s
    }
}

#[inline]
fn sub_mod_fp(a: u64, b: u64, p: u64) -> u64 {
    work::hook_add(p);
    if a >= b {
        a - b
    } else {
        a + p - b
    }
}

fn inv_mod_fp(a: u64, p: u64) -> u64 {
    work::hook_inv(p);
    let (mut old_r, mut r) = (a as i128, p as i128);
    let (mut old_s, mut s) = (1i128, 0i128);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    old_s.rem_euclid(p as i128) as u64
}

/// In-place synthetic division of `tail[0..=*deg]` by `(x - r)` over `F_p`.
/// Returns `true` iff the remainder is zero, decrementing `*deg`.
#[inline]
fn synthetic_div_linear(tail: &mut [u64; 10], deg: &mut usize, r: u64, p: u64) -> bool {
    let d = *deg;
    if d == 0 {
        return false;
    }
    let mut carry = tail[d];
    tail[d] = 0;
    for k in (0..d).rev() {
        let cur = tail[k];
        tail[k] = carry;
        let prod = mul_mod(carry, r, p);
        carry = add_mod_fp(cur, prod, p);
    }
    if carry != 0 {
        return false;
    }
    *deg = d - 1;
    true
}

/// Horner modular composition `f_poly(arg) mod modulus` in `F_p[x]`.
fn eval_poly_mod_ring(ring: &FpRing, f_poly: &[u64], arg: &[u64], modulus: &[u64]) -> Vec<u64> {
    if f_poly.is_empty() {
        return Vec::new();
    }
    let mut acc = vec![*f_poly.last().unwrap()];
    for &c in f_poly.iter().rev().skip(1) {
        let prod = ring.rem(&ring.mul(&acc, arg), modulus);
        acc = if c == 0 { prod } else { ring.add(&prod, &[c]) };
    }
    ring.rem(&acc, modulus)
}

#[inline]
fn is_irreducible_quadratic_table(u: &[u64], p: u64, sqrt_table: &[u64]) -> bool {
    if u.len() != 3 || u[2] != 1 {
        return false;
    }
    let b2 = mul_mod(u[1], u[1], p);
    let four_c = mul_mod(4 % p, u[0], p);
    let disc = sub_mod_fp(b2, four_c, p);
    work::words(2);
    disc != 0 && sqrt_table[disc as usize] == u64::MAX
}

/// Deterministic zero-RNG Frobenius-norm resolvent splitter for a monic quartic
/// `q` given `xp = x^p mod q` when `q` is already certified to split into two
/// distinct monic irreducible quadratics over `F_p`.
fn split_quartic_from_xp_resolvent(
    ring: &FpRing,
    q: &[u64],
    xp: &[u64],
    sqrt_table: &[u64],
) -> Result<[Vec<u64>; 2], &'static str> {
    let p = ring.p;
    let inv2 = p.div_ceil(2);
    for alpha in [0u64, 1u64] {
        let xp_a = if alpha == 0 {
            xp.to_vec()
        } else {
            ring.add(xp, &[alpha])
        };
        let n_poly = ring.rem(&ring.mul(&[alpha, 1], &xp_a), q);
        if n_poly.len() < 3 {
            continue;
        }
        let d = n_poly.len() - 1;
        let s_poly = ring.rem(&ring.mul(&n_poly, &n_poly), q);
        let sd = s_poly.get(d).copied().unwrap_or(0);
        let t_tr = mul_mod(sd, inv_mod_fp(n_poly[d], p), p);
        let n0 = n_poly.first().copied().unwrap_or(0);
        let s0 = s_poly.first().copied().unwrap_or(0);
        let d_nm = sub_mod_fp(mul_mod(t_tr, n0, p), s0, p);
        let disc = sub_mod_fp(mul_mod(t_tr, t_tr, p), mul_mod(4 % p, d_nm, p), p);
        work::words(2);
        let r = sqrt_table[disc as usize];
        if disc == 0 || r == u64::MAX {
            return Err("resolvent discriminant not nonzero square");
        }
        let lam1 = mul_mod(add_mod_fp(t_tr, r, p), inv2, p);
        let u1 = ring.gcd(q, &ring.sub(&n_poly, &[lam1]));
        if u1.len() != 3 {
            continue;
        }
        let (u2, rem) = ring.divrem(q, &u1);
        if !rem.is_empty() || u2.len() != 3 {
            return Err("resolvent exact division failed");
        }
        let mut out = [u1, u2];
        out.sort();
        if out[0] == out[1]
            || !is_irreducible_quadratic_table(&out[0], p, sqrt_table)
            || !is_irreducible_quadratic_table(&out[1], p, sqrt_table)
        {
            return Err("resolvent factor irreducibility failed");
        }
        return Ok(out);
    }
    Err("deterministic resolvent failed on both alpha=0 and alpha=1")
}

fn split_quartic_frob_resolvent(
    ring: &FpRing,
    tail_raw: &[u64],
    sqrt_table: &[u64],
) -> Result<[Vec<u64>; 2], &'static str> {
    let q = ring.monic(tail_raw);
    if q.len() != 5 {
        return Err("canonical quartic required");
    }
    let xp = ring.powmod_x(ring.p as u128, &q);
    let xp2 = eval_poly_mod_ring(ring, &xp, &xp, &q);
    if xp2 != [0, 1] {
        return Err("quartic not a product of degree-1/2 factors");
    }
    if ring.gcd(&q, &ring.sub(&xp, &[0, 1])) != [1] {
        return Err("quartic has F_p linear root");
    }
    split_quartic_from_xp_resolvent(ring, &q, &xp, sqrt_table)
}

fn split_sextic_frob_norm(
    ring: &FpRing,
    tail_raw: &[u64],
    sqrt_table: &[u64],
    max_attempts: u8,
    rng: &mut StdRng,
) -> Result<[Vec<u64>; 3], &'static str> {
    let q = ring.monic(tail_raw);
    if q.len() != 7 {
        return Err("canonical sextic required");
    }
    let xp = ring.powmod_x(ring.p as u128, &q);
    let xp2 = eval_poly_mod_ring(ring, &xp, &xp, &q);
    if xp2 != [0, 1] {
        return Err("sextic not a product of degree-1/2 factors");
    }
    if ring.gcd(&q, &ring.sub(&xp, &[0, 1])) != [1] {
        return Err("sextic has F_p linear root");
    }
    let exp = (ring.p as u128 - 1) / 2;
    for _ in 0..max_attempts.max(8) {
        let alpha = work::uniform_mod_u64(rng, 0, ring.p);
        let xp_a = ring.add(&xp, &[alpha]);
        let n_poly = ring.rem(&ring.mul(&[alpha, 1], &xp_a), &q);
        if n_poly.len() <= 1 {
            continue;
        }
        let w = ring.powmod(&n_poly, exp, &q);
        let g = ring.gcd(&q, &ring.sub(&w, &[1]));
        let (quad, quartic) = if g.len() == 3 {
            let (qt, rem) = ring.divrem(&q, &g);
            if !rem.is_empty() {
                return Err("sextic division");
            }
            (g, qt)
        } else if g.len() == 5 {
            let (qd, rem) = ring.divrem(&q, &g);
            if !rem.is_empty() {
                return Err("sextic division");
            }
            (qd, g)
        } else {
            continue;
        };
        let xp_q4 = ring.rem(&xp, &quartic);
        let [u2, u3] = split_quartic_from_xp_resolvent(ring, &quartic, &xp_q4, sqrt_table)?;
        if !is_irreducible_quadratic_table(&quad, ring.p, sqrt_table) {
            return Err("sextic quadratic factor reducible");
        }
        let mut out = [quad, u2, u3];
        out.sort();
        if out[0] == out[1] || out[1] == out[2] {
            return Err("sextic duplicate quadratic");
        }
        return Ok(out);
    }
    Err("sextic split attempt cap")
}

#[inline]
fn canonical_mumford_class(f: &Fq, u_fp: &[u64], v: &[E2]) -> ([u64; 6], i8) {
    let a = v.first().copied().unwrap_or(E2::ZERO);
    let b = v.get(1).copied().unwrap_or(E2::ZERO);
    let na = f.neg(&a);
    let nb = f.neg(&b);
    let key = [u_fp[0], u_fp[1], a.0[0], a.0[1], b.0[0], b.0[1]];
    let neg = [u_fp[0], u_fp[1], na.0[0], na.0[1], nb.0[0], nb.0[1]];
    work::words(18);
    if neg < key {
        (neg, -1)
    } else {
        (key, 1)
    }
}

#[allow(clippy::too_many_arguments)]
fn extract_partial_source_matroid(
    ctx: &Ctx,
    ring: &FpRing,
    config: &TwoLargePrimeSieve,
    base_by_x: &[u32],
    line: &Line,
    s: u64,
    raw_xs: &[u16],
    max_arity: usize,
    split_options: SplitOptions,
    split_rng: &mut StdRng,
    screen_ctx: Option<&discriminant::Context<'_>>,
    screen_calls: &mut u64,
    screen_refusals: &mut u64,
) -> Result<ReferencedSource, &'static str> {
    let cnt = raw_xs.len();
    let allowed = if max_arity >= 3 {
        matches!(cnt, 3 | 5 | 7 | 9)
    } else {
        matches!(cnt, 5 | 7 | 9)
    };
    if !allowed {
        return Err("hit count outside odd arity window");
    }
    let f = &ctx.f.f;
    let p = f.p;
    if s == 0 || s >= p {
        return Err("zero/out-of-range sieve parameter");
    }

    let mut xs_sorted = [0u16; 9];
    xs_sorted[..cnt].copy_from_slice(raw_xs);
    xs_sorted[..cnt].sort_unstable();
    work::words((cnt * 3) as u64);

    let mut hit_indices = [0usize; 9];
    for k in 0..cnt {
        work::words(1);
        let x = xs_sorted[k] as usize;
        let idx = *base_by_x.get(x).unwrap_or(&u32::MAX);
        if idx == u32::MAX {
            return Err("hit x not in factor base");
        }
        hit_indices[k] = idx as usize;
    }

    let norm = sieve::line_poly(ring, line, s);
    if norm.len() != 10 {
        return Err("degree nine required");
    }

    let base = config.base();
    let mut tail_arr = [0u64; 10];
    tail_arr.copy_from_slice(&norm);
    let mut deg = 9usize;
    for &idx in &hit_indices[..cnt] {
        let bx = base[idx].x;
        if !synthetic_div_linear(&mut tail_arr, &mut deg, bx, p) {
            return Err("root division");
        }
        if remainder::eval(f, &line.b, bx) == E2::ZERO {
            return Err("zero denominator");
        }
    }

    let setup = config.setup();
    let tail_slice = &tail_arr[..=deg];
    let factors: Vec<Vec<u64>> = match deg {
        0 => Vec::new(),
        2 => {
            if tail_slice[2] == 0 {
                return Err("degenerate quadratic");
            }
            let q2 = ring.monic(tail_slice);
            if !is_irreducible_quadratic_table(&q2, p, &setup.sqrt) {
                return Err("reducible quadratic");
            }
            vec![q2]
        }
        4 => {
            if tail_slice[4] == 0 {
                return Err("degenerate quartic");
            }
            if let Some(sc) = screen_ctx {
                *screen_calls += 1;
                let assessment = sc.screen(tail_slice)?;
                if assessment.rejects() {
                    *screen_refusals += 1;
                    return Err("quartic discriminant necessary condition");
                }
            }
            let [u1, u2] = split_quartic_frob_resolvent(ring, tail_slice, &setup.sqrt)?;
            vec![u1, u2]
        }
        6 if max_arity >= 3 => {
            if tail_slice[6] == 0 {
                return Err("degenerate sextic");
            }
            let [u1, u2, u3] = split_sextic_frob_norm(
                ring,
                tail_slice,
                &setup.sqrt,
                split_options.max_attempts,
                split_rng,
            )?;
            vec![u1, u2, u3]
        }
        _ => return Err("unexpected tail degree"),
    };

    let sq = if setup.sqrt[s as usize] != u64::MAX {
        E2([setup.sqrt[s as usize], 0])
    } else {
        let r = setup.sqrt[mul_mod(s, setup.winv, p) as usize];
        if r == u64::MAX {
            return Err("square root table");
        }
        E2([0, r])
    };
    let a_poly: Vec<E2> = (0..line.a0.len().max(line.a1.len()))
        .map(|i| {
            E2([
                *line.a0.get(i).unwrap_or(&0),
                mul_mod(s, *line.a1.get(i).unwrap_or(&0), p),
            ])
        })
        .collect();
    let c_poly: Vec<E2> = line.b.iter().map(|b| f.mul(&sq, b)).collect();
    let a_trim = remainder::trim(a_poly);
    let c_trim = remainder::trim(c_poly);
    if c_trim.len() != 2 || c_trim[1] == E2::ZERO {
        return Err("non-linear denominator");
    }

    let mut terms = Vec::with_capacity(cnt);
    for &idx in &hit_indices[..cnt] {
        let b = &base[idx];
        let den = remainder::eval(f, &c_trim, b.x);
        if den == E2::ZERO {
            return Err("zero denominator");
        }
        let ax = remainder::eval(f, &a_trim, b.x);
        let dy = f.mul(&den, &b.y);
        let sign = if f.add(&ax, &dy) == E2::ZERO {
            1
        } else if f.sub(&ax, &dy) == E2::ZERO {
            -1
        } else {
            return Err("ordinate mismatch");
        };
        terms.push((idx, sign));
    }

    let mut remainders = Vec::with_capacity(factors.len());
    for factor in &factors {
        let u = remainder::embed(factor);
        let inv_c = remainder::inverse_linear(f, &c_trim, &u)?;
        let a_mod = remainder::modulo(f, &a_trim, &u)?;
        let prod = remainder::mul(f, &a_mod, &inv_c);
        let v: Vec<E2> = remainder::modulo(f, &prod, &u)?
            .into_iter()
            .map(|x| f.neg(&x))
            .collect();
        remainders.push(canonical_mumford_class(f, factor, &v));
    }
    if remainders.windows(2).any(|w| w[0].0 >= w[1].0) {
        return Err("non-distinct canonical Mumford classes");
    }

    let src = ReferencedSource {
        source_id: 0,
        terms,
        remainders,
        a: remainder::encode(&a_trim),
        c: remainder::encode(&c_trim),
        norm,
    };

    if src.remainders.is_empty() {
        let jac = ctx.jac();
        let mut acc = jac.identity();
        for &(idx, sign) in &src.terms {
            let d = if sign == 1 {
                base[idx].d.clone()
            } else {
                jac.neg(&base[idx].d)
            };
            acc = jac.add(&acc, &d);
        }
        if !jac.is_identity(&acc) {
            return Err("full Jacobian check");
        }
    }

    Ok(src)
}

fn matroid_dedup_key(src: &ReferencedSource) -> PairKey {
    let orientation = src
        .remainders
        .first()
        .map_or(src.terms[0].1, |&(_, coeff)| coeff);
    work::probe((src.terms.len() * 2 + src.remainders.len() * 7) as u64);
    (
        src.terms
            .iter()
            .map(|&(i, s)| (i, s * orientation))
            .collect(),
        src.remainders
            .iter()
            .map(|&(k, c)| (k, c * orientation))
            .collect(),
    )
}

fn hash_source(hasher: &mut Sha256, src: &ReferencedSource) {
    hasher.update((src.source_id as u64).to_le_bytes());
    hasher.update((src.terms.len() as u64).to_le_bytes());
    for &(idx, s) in &src.terms {
        hasher.update((idx as u64).to_le_bytes());
        hasher.update([s as u8]);
    }
    hasher.update((src.remainders.len() as u64).to_le_bytes());
    for &(key, c) in &src.remainders {
        for w in key {
            hasher.update(w.to_le_bytes());
        }
        hasher.update([c as u8]);
    }
    for x in &src.norm {
        hasher.update(x.to_le_bytes());
    }
}

/// Execute one cold, end-to-end extension-field graph-IC discrete-logarithm solve.
///
/// `spec.d` is never read anywhere in this function.
pub fn run_anchored_ic_dlp_on(
    spec: &Spec,
    seed: u64,
    options: AnchoredPipelineOptions,
) -> Result<AnchoredSolveBundle, String> {
    options.sparse_rank.validate()?;
    if !(101..=1511).contains(&spec.p)
        || !(3..1u64 << 63).contains(&spec.l)
        || !is_prime_u64(spec.p)
        || !is_prime_u64(spec.l)
    {
        return Err("invalid prime/subgroup bounds".into());
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .map_err(|e| format!("rayon 1-thread pool: {e}"))?;

    pool.install(|| {
        // Calibrate GAE prices cleanly before resetting `jv_bitops` for the cold IC run.
        work::enable(true);
        work::claim();
        work::set_moduli(spec.p, spec.l);
        let schoolbook_prims = rho_strong::measure_native_fq3_prims(spec.p, spec.l);
        let karatsuba_prims = rho_strong::measure_karatsuba_fq3_prims(spec.p, spec.l);
        let schoolbook_gae = gae_bitops_from_prims(&schoolbook_prims, spec.p, spec.l);
        let karatsuba_gae = gae_bitops_from_prims(&karatsuba_prims, spec.p, spec.l);

        // Start fresh cold counting window for the IC pipeline itself.
        work::enable(false);
        work::enable(true);
        work::claim();
        work::set_moduli(spec.p, spec.l);
        let foreign_before = work::foreign();
        let bitops_before = work::snapshot();
        let uniform_before = work::uniform_snapshot();

        let t_total = Instant::now();
        let mut phase_wall = BTreeMap::<String, f64>::new();

        // 1. Model & exact universe census (excluded diagnostic metadata done before Relations).
        let t_model = Instant::now();
        let ctx = {
            let _m = work::enter(Phase::Model);
            Ctx::new(spec)
        };
        phase_wall.insert("model".into(), t_model.elapsed().as_secs_f64() * 1e3);
        // Compute exact O(p^2) census with counting paused so diagnostic census does not pollute IC.
        work::enable(false);
        let census = count_quadratic_universe(&ctx);
        work::enable(true);
        work::claim();
        work::set_moduli(spec.p, spec.l);

        // 2. Factor base.
        let t_fb = Instant::now();
        let (base, config, base_by_x, by_x_map) = {
            let _fb = work::enter(Phase::FactorBase);
            let mut base_rng = StdRng::seed_from_u64(seed ^ 0xBACE_0001);
            let base = cover::factor_base(&ctx, &mut base_rng);
            let config = TwoLargePrimeSieve::new(&ctx, &base, base.len(), 9)?;
            let mut base_by_x = vec![u32::MAX; spec.p as usize];
            let mut by_x_map = HashMap::with_capacity(base.len());
            for (i, b) in base.iter().enumerate() {
                base_by_x[b.x as usize] = i as u32;
                by_x_map.insert(b.x, i);
                work::words(2);
            }
            (base, config, base_by_x, by_x_map)
        };
        phase_wall.insert("factor_base".into(), t_fb.elapsed().as_secs_f64() * 1e3);

        // 3. Phase-1 Relation collection & sparse homogeneous rank certification.
        let t_rel = Instant::now();
        let relation_guard = work::enter(Phase::Relations);
        let ring = FpRing::new(spec.p);
        let screen_ctx = (options.collector.is_two_lp() || options.collector.is_matroid())
            .then(|| discriminant::Context::new(&ring))
            .transpose()?;
        let mut ctr = Counters::new(spec.p);
        let mut hit_buckets = options
            .collector
            .is_matroid()
            .then(|| HitBuckets::new(spec.p));
        let mut counts_buf = Vec::<u8>::with_capacity(spec.p as usize);
        let mut enum_rng = StdRng::seed_from_u64(seed ^ 0xA17E_2E17);
        let mut split_rng = StdRng::seed_from_u64(seed ^ 0x5155_4144_5041_4952);
        let b_space_total = sieve::b_space(config.setup());
        let b_budget = options.b_count.unwrap_or(b_space_total);
        if b_budget == 0 || b_budget > b_space_total {
            work::enable(false);
            return Err("B count exceeds finite space".into());
        }
        let scramble = sieve::coprime_scramble(b_space_total, &mut enum_rng);
        let nu = sieve::nu_lines(config.setup());
        let mut slot = Vec::new();
        let rank_target = base.len().checked_sub(1).ok_or("empty factor base")?;
        let mut rank = Tracker::new(
            spec.l,
            base.len(),
            seed ^ 0x5A46_4B45_524E_454C,
            options.sparse_rank,
        )?;

        let mut all_sources: Vec<ReferencedSource> = Vec::new();
        let mut projections: Vec<Vec<(usize, u64)>> = Vec::new();
        let mut b_scanned = 0u64;
        let mut lines_tested = 0u64;
        let mut candidates_tested = 0u64;
        let mut candidates_refused = 0u64;
        let mut duplicates_skipped = 0u64;
        let mut full_sources = 0u64;
        let mut one_class_sources = 0u64;
        let mut two_class_sources = 0u64;
        let mut three_class_sources = 0u64;
        let mut screen_calls = 0u64;
        let mut screen_refusals = 0u64;
        let mut la_wall_during_rel = 0.0f64;
        let mut stop_reason = "B budget exhausted".to_string();

        // Collector-specific graph state.
        let mut sparse_anchors = BTreeMap::<[u64; 6], usize>::new();
        let mut sparse_seen = BTreeSet::<(Vec<(usize, i8)>, Option<[u64; 6]>)>::new();
        let mut deferred_graph = deferred::Graph::new(deferred::Options {
            max_sources: options.max_sources,
            max_vertices: options.max_vertices,
            max_deferred_edges: options.max_deferred_edges,
            max_witness_terms: options.max_witness_terms,
        })
        .map_err(String::from)?;
        let mut deferred_seen = BTreeSet::<PairKey>::new();
        let mut matroid_graph = SignedMatroidGraph::new(MatroidOptions {
            max_sources: options.max_sources,
            max_vertices: options.max_vertices,
            max_tree_edges: options.max_deferred_edges,
            max_hyperedges: options.max_deferred_edges,
            max_witness_terms: options.max_witness_terms,
            max_abs_coeff: 4096,
        })
        .map_err(String::from)?;
        let mut matroid_seen = HashSet::<PairKey>::new();
        let mut proj_small_buf = Vec::<(usize, i64)>::with_capacity(256);
        let mut proj_large_buf = Vec::<([u64; 6], i64)>::with_capacity(64);

        'collect: while (b_scanned as u128) < b_budget {
            let begin = b_scanned;
            let end = ((begin as u128 + 1024).min(b_budget)) as u64;
            let batch: Vec<_> = (begin..end)
                .map(|k| sieve::b_from_index(config.setup(), k as u128, scramble))
                .collect();
            let ab: Vec<_> = batch.iter().map(|b| (b[0].0[0], b[0].0[1])).collect();
            let hints = sieve::roots_by_incidence(spec.p, &nu, &ab, &mut slot).0;

            for (offset, b) in batch.iter().enumerate() {
                let found = sieve::lines_for_b_hint(
                    &ring,
                    &ctx.f.f,
                    config.setup(),
                    b,
                    &mut enum_rng,
                    true,
                    hints[offset].as_deref(),
                );
                let shared = if found.len() >= 2 {
                    Some(sieve::shared_for_b(&ring, &found[0], &mut 0))
                } else {
                    None
                };
                for line in found {
                    lines_tested += 1;
                    if options.collector.is_matroid() {
                        let max_arity = options.collector.max_arity();
                        let min_hits = if max_arity >= 3 { 3u8 } else { 5u8 };
                        let buckets = hit_buckets.as_mut().expect("matroid hit buckets");
                        let (mut hits, _) = sieve::sieve_line_buckets(
                            &ring,
                            config.setup(),
                            &line,
                            buckets,
                            shared.as_ref(),
                            min_hits,
                        );
                        hits.sort_unstable();
                        work::words((hits.len() * 2) as u64);
                        let mut three_lp_tested_on_line = 0usize;
                        for pass in 0..2 {
                            for &s in &hits {
                                let raw_xs = buckets.hits_for(s);
                                let cnt = raw_xs.len();
                                if pass == 0 {
                                    if !matches!(cnt, 5 | 7 | 9) {
                                        continue;
                                    }
                                } else {
                                    if max_arity < 3 || cnt != 3 || three_lp_tested_on_line >= 2 {
                                        continue;
                                    }
                                    three_lp_tested_on_line += 1;
                                }
                                candidates_tested += 1;
                                if screen_refusals as usize >= options.max_screen_refusals {
                                    stop_reason = "discriminant refusal resource cap".into();
                                    b_scanned = begin + offset as u64;
                                    break 'collect;
                                }
                                let Ok(mut src) = extract_partial_source_matroid(
                                    &ctx,
                                    &ring,
                                    &config,
                                    &base_by_x,
                                    &line,
                                    s,
                                    raw_xs,
                                    max_arity,
                                    options.split,
                                    &mut split_rng,
                                    screen_ctx.as_ref(),
                                    &mut screen_calls,
                                    &mut screen_refusals,
                                ) else {
                                    candidates_refused += 1;
                                    continue;
                                };
                                if all_sources.len() >= options.max_sources {
                                    stop_reason = "source resource cap".into();
                                    b_scanned = begin + offset as u64;
                                    break 'collect;
                                }
                                if !matroid_seen.insert(matroid_dedup_key(&src)) {
                                    duplicates_skipped += 1;
                                    continue;
                                }
                                match src.remainders.len() {
                                    0 => full_sources += 1,
                                    1 => one_class_sources += 1,
                                    2 => two_class_sources += 1,
                                    _ => three_class_sources += 1,
                                }
                                let src_id = all_sources.len();
                                src.source_id = src_id;
                                let large_terms: Vec<graph_core::LargeTerm> = src
                                    .remainders
                                    .iter()
                                    .map(|&(class, coefficient)| graph_core::LargeTerm {
                                        class,
                                        coefficient,
                                    })
                                    .collect();
                                let emitted = matroid_graph
                                    .accept(src_id, &large_terms)
                                    .map_err(String::from)?;
                                all_sources.push(src);
                                for zw in &emitted {
                                    projections.push(project_sources_i16(
                                        &all_sources,
                                        zw,
                                        spec.l,
                                        &mut proj_small_buf,
                                        &mut proj_large_buf,
                                    )?);
                                }
                            }
                        }
                    } else if options.collector.is_two_lp() {
                        let _ = sieve::sieve_line(
                            &ring,
                            config.setup(),
                            &line,
                            &mut ctr,
                            Some(&mut counts_buf),
                            shared.as_ref(),
                        );
                        work::words(if options.collector.is_fast() {
                            ring.p
                        } else {
                            6 * ring.p
                        });
                        for (s_idx, &cnt) in counts_buf.iter().enumerate().skip(1) {
                            if cnt < 5 {
                                continue;
                            }
                            let s = s_idx as u64;
                            candidates_tested += 1;
                            if screen_refusals as usize >= options.max_screen_refusals {
                                stop_reason = "discriminant refusal resource cap".into();
                                b_scanned = begin + offset as u64;
                                break 'collect;
                            }
                            let mut extraction = Extraction {
                                options: options.split,
                                rng: &mut split_rng,
                                trace: None,
                            };
                            let res = if options.collector.is_fast() {
                                candidate_deferred_fast(
                                    &ctx,
                                    &ring,
                                    &config,
                                    &base_by_x,
                                    &line,
                                    s,
                                    &mut extraction,
                                    screen_ctx.as_ref(),
                                    &mut screen_calls,
                                    &mut screen_refusals,
                                )
                            } else {
                                // Exact baseline `DeferredPairs` check.
                                if let Some(sc) = screen_ctx.as_ref() {
                                    deferred_baseline_candidate_with_screen(
                                        &ctx,
                                        &ring,
                                        &config,
                                        &line,
                                        s,
                                        &mut extraction,
                                        sc,
                                        &mut screen_calls,
                                        &mut screen_refusals,
                                    )
                                } else {
                                    deferred_dlp::candidate(
                                        &ctx,
                                        &ring,
                                        &config,
                                        &line,
                                        s,
                                        &mut extraction,
                                    )
                                }
                            };
                            let Ok((cert, large)) = res else {
                                candidates_refused += 1;
                                continue;
                            };
                            if all_sources.len() >= options.max_sources {
                                stop_reason = "source resource cap".into();
                                b_scanned = begin + offset as u64;
                                break 'collect;
                            }
                            if !deferred_seen.insert(pair_source_key(&cert)) {
                                duplicates_skipped += 1;
                                continue;
                            }
                            match cert.remainders.len() {
                                0 => full_sources += 1,
                                1 => one_class_sources += 1,
                                _ => two_class_sources += 1,
                            }
                            let src_id = all_sources.len();
                            let outcome = deferred_graph
                                .accept(src_id, &large)
                                .map_err(String::from)?;
                            all_sources.push(ReferencedSource {
                                source_id: src_id,
                                terms: cert.terms,
                                remainders: cert
                                    .remainders
                                    .into_iter()
                                    .map(|r| (r.key, r.coefficient))
                                    .collect(),
                                a: cert.a,
                                c: cert.c,
                                norm: cert.norm,
                            });
                            for event in outcome {
                                if let deferred::Event::Full { witness, .. } = event {
                                    projections.push(project_sources(
                                        &all_sources,
                                        &witness,
                                        spec.l,
                                    )?);
                                }
                            }
                        }
                    } else {
                        // 1-LP (`SparseTable` / `SparseTableFast`): threshold >= 7.
                        let (hits, _) = config.sieve_line(&ring, &line, &mut ctr, shared.as_ref());
                        for s in hits {
                            candidates_tested += 1;
                            let res = if options.collector.is_fast() {
                                candidate_sparse_fast(&ctx, &ring, &config, &base_by_x, &line, s)
                            } else {
                                sparse_dlp::candidate_with_root_table(
                                    &ctx, &ring, &config, &line, s, true,
                                )
                            };
                            let Ok(cert) = res else {
                                candidates_refused += 1;
                                continue;
                            };
                            let orientation = cert
                                .remainder
                                .as_ref()
                                .map_or(cert.terms[0].1, |r| r.coefficient);
                            let dedup_key = (
                                cert.terms
                                    .iter()
                                    .map(|&(i, sgn)| (i, sgn * orientation))
                                    .collect::<Vec<_>>(),
                                cert.remainder.as_ref().map(|r| r.key),
                            );
                            work::probe((cert.terms.len() * 2 + 6) as u64);
                            if !sparse_seen.insert(dedup_key) {
                                duplicates_skipped += 1;
                                continue;
                            }
                            let src_id = all_sources.len();
                            let rems = cert
                                .remainder
                                .as_ref()
                                .map(|r| vec![(r.key, r.coefficient)])
                                .unwrap_or_default();
                            if rems.is_empty() {
                                full_sources += 1;
                            } else {
                                one_class_sources += 1;
                            }
                            all_sources.push(ReferencedSource {
                                source_id: src_id,
                                terms: cert.terms.clone(),
                                remainders: rems,
                                a: cert.a,
                                c: cert.c,
                                norm: cert.norm,
                            });
                            if let Some(r) = &cert.remainder {
                                work::probe(6);
                                if let Some(&anc_id) = sparse_anchors.get(&r.key) {
                                    let c0 = all_sources[anc_id].remainders[0].1;
                                    let witness = vec![(anc_id, -r.coefficient), (src_id, c0)];
                                    projections.push(project_sources(
                                        &all_sources,
                                        &witness,
                                        spec.l,
                                    )?);
                                } else {
                                    sparse_anchors.insert(r.key, src_id);
                                }
                            } else {
                                let witness = vec![(src_id, 1i8)];
                                projections.push(project_sources(&all_sources, &witness, spec.l)?);
                            }
                        }
                    }
                }
            }
            b_scanned = end;
            if rank.due(projections.len()) {
                let t_la_chk = Instant::now();
                let _la = work::enter(Phase::LinearAlgebra);
                let rows = projections.clone();
                if end as u128 == b_budget {
                    rank.check_at_budget_end(end, rows)?;
                } else {
                    rank.check(end, rows)?;
                }
                la_wall_during_rel += t_la_chk.elapsed().as_secs_f64() * 1e3;
            }
            if rank.kernel.is_some() {
                stop_reason = "sparse homogeneous rank certified at complete batch".into();
                break;
            }
            if rank.exhausted() {
                stop_reason = "sparse kernel checkpoint cap".into();
                break;
            }
        }

        // Build unified `AnchorTable` from Phase-1 state (+ optional frustrated-cycle half-anchors).
        let mut anchor_table = if options.collector.is_matroid() {
            matroid_graph.export_anchor_table().0
        } else {
            let mut tbl = AnchorTable::with_capacity(if options.collector.is_two_lp() {
                deferred_graph.stats().anchors + 1024
            } else {
                sparse_anchors.len() + 64
            });
            if options.collector.is_two_lp() {
                for (class, witness) in deferred_graph.anchors() {
                    tbl.insert(AnchorRecord {
                        class: *class,
                        witness: witness.iter().map(|&(id, c)| (id, c as i16)).collect(),
                        scale: 1,
                    });
                }
            } else {
                for (&class, &src_id) in &sparse_anchors {
                    let c0 = all_sources[src_id].remainders[0].1;
                    tbl.insert(AnchorRecord {
                        class,
                        witness: vec![(src_id, c0 as i16)],
                        scale: 1,
                    });
                }
            }
            tbl
        };

        let cycle_harvest = if options.collector.is_two_lp() && options.harvest_odd_cycles {
            let unanchored: Vec<UnanchoredEdge> = all_sources
                .iter()
                .filter_map(|s| {
                    if s.remainders.len() == 2 {
                        Some((
                            s.source_id,
                            s.remainders[0].0,
                            s.remainders[0].1,
                            s.remainders[1].0,
                            s.remainders[1].1,
                        ))
                    } else {
                        None
                    }
                })
                .collect();
            Some(harvest_frustrated_cycle_anchors(
                &unanchored,
                &mut anchor_table,
                options.max_witness_terms,
            ))
        } else {
            None
        };

        drop(relation_guard);
        let rel_wall_only = (t_rel.elapsed().as_secs_f64() * 1e3 - la_wall_during_rel).max(0.0);
        phase_wall.insert("relations".into(), rel_wall_only);

        // Compute deterministic SHA-256 digests of full source and projection streams.
        let mut src_hasher = Sha256::new();
        for s in &all_sources {
            hash_source(&mut src_hasher, s);
        }
        let sources_sha256 = format!("{:x}", src_hasher.finalize());
        let mut proj_hasher = Sha256::new();
        for row in &projections {
            proj_hasher.update((row.len() as u64).to_le_bytes());
            for &(idx, val) in row {
                proj_hasher.update((idx as u64).to_le_bytes());
                proj_hasher.update(val.to_le_bytes());
            }
        }
        let projections_sha256 = format!("{:x}", proj_hasher.finalize());

        // 4. Homogeneous quotient construction & orthogonality verification.
        let t_la_post = Instant::now();
        let ready = rank.kernel.is_some();
        let mut kernel_orthogonality_verified_rows = 0usize;
        let mut basis = if ready {
            let _la = work::enter(Phase::LinearAlgebra);
            let q = Quotient::from_rank_certified_kernel(
                spec.l,
                rank.kernel.clone().ok_or("missing certified kernel")?,
                rank_target,
            )
            .map_err(String::from)?;
            work::hook_muls(spec.l, q.counts.products);
            work::hook_adds(spec.l, q.counts.additions);
            work::words(q.counts.word_events);
            // Check orthogonality of every Phase-1 homogeneous projection against `q.kernel`.
            for row in &projections {
                let mut dot = 0u128;
                for &(idx, coef) in row {
                    dot = (dot + (coef as u128) * (q.kernel[idx] as u128)) % (spec.l as u128);
                }
                if dot != 0 {
                    work::enable(false);
                    return Err(
                        "certified homogeneous kernel failed projection orthogonality".into(),
                    );
                }
                kernel_orthogonality_verified_rows += 1;
            }
            Some(q)
        } else {
            None
        };
        let initial_kernel = basis.as_ref().map(|q| q.kernel.clone());
        let mut total_la_wall = la_wall_during_rel + t_la_post.elapsed().as_secs_f64() * 1e3;

        // 5. Descent + protected 2-column affine solve + elliptic curve verification.
        let mut descent_certs = Vec::<AnchoredDescentCert>::new();
        let mut affine_rows = Vec::<CompactAffineRow>::new();
        let mut affine_solver = CountedRelationSystem::new(spec.l, 2);
        let mut residuals_tried = 0u64;
        let mut stream_advances = 0u64;
        let mut timeouts = 0u64;
        let mut incomplete = 0u64;
        let mut f4_muls_total = 0u64;
        let mut lin_muls_total = 0u64;
        let mut weil_muls_total = 0u64;
        let mut root_muls_total = 0u64;
        let mut pattern_hist = PatternHistogram::default();
        let mut trace_state = LocalTraceState::new();
        let mut recovered_scalar = None;
        let mut refusal: Option<String> = if ready {
            None
        } else {
            Some("homogeneous collection not ready".into())
        };

        let t_desc = Instant::now();
        let mut ver_wall = 0.0f64;
        if let Some(quotient) = basis.as_mut() {
            let _desc = work::enter(Phase::Descent);
            let jac = ctx.jac();
            let mut stream = StdRng::seed_from_u64(seed ^ 0x51E7E);
            let al = work::uniform_mod_u64(&mut stream, 1, spec.l);
            let be = work::uniform_mod_u64(&mut stream, 1, spec.l);
            let step = jac.add(
                &jac.mul(&spec.gj, al as u128),
                &jac.mul(&spec.qj, be as u128),
            );
            let mut a = work::uniform_mod_u64(&mut stream, 0, spec.l);
            let mut b = work::uniform_mod_u64(&mut stream, 1, spec.l);
            let mut residual =
                jac.add(&jac.mul(&spec.gj, a as u128), &jac.mul(&spec.qj, b as u128));
            let opts = cover::opts_for_mode(24, 600.0, cover::StopMode::Staircase(64));

            for i in 0..options.max_descent_residuals {
                let mut rng = StdRng::seed_from_u64(seed ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15));
                let (maybe_cert, cost, hist) = decompose_residual_anchored_ref(
                    &ctx,
                    &base,
                    &base_by_x,
                    &by_x_map,
                    &anchor_table,
                    &|src_id| all_sources[src_id].terms.as_slice(),
                    i,
                    a,
                    b,
                    &residual,
                    &opts,
                    options.descent,
                    &mut trace_state,
                    &mut rng,
                );
                residuals_tried += 1;
                timeouts += u64::from(cost.timed_out);
                incomplete += u64::from(cost.incomplete);
                f4_muls_total += cost.f4_muls;
                lin_muls_total += cost.lin_muls;
                weil_muls_total += cost.weil_muls;
                root_muls_total += cost.post_muls;
                pattern_hist.merge(&hist);

                if let Some(cert) = maybe_cert {
                    let t_la_row = Instant::now();
                    let _la = work::enter(Phase::LinearAlgebra);
                    let l_i128 = spec.l as i128;
                    let mut dense = vec![0u64; base.len() + 1];
                    let mut nonzeros = Vec::with_capacity(cert.expanded_row.len());
                    work::words(dense.len() as u64);
                    for &(idx, coef) in &cert.expanded_row {
                        let v = (coef as i128).rem_euclid(l_i128) as u64;
                        if v != 0 {
                            dense[idx] = v;
                            nonzeros.push((idx, v));
                        }
                        work::words(2);
                    }
                    let scale_u128 = cert.scale as u128;
                    let neg_sb = if b == 0 {
                        0
                    } else {
                        (((spec.l - b) as u128 * scale_u128) % (spec.l as u128)) as u64
                    };
                    let rhs_sa = (((a as u128) * scale_u128) % (spec.l as u128)) as u64;
                    dense[base.len()] = neg_sb;
                    work::hook_add(spec.l);
                    work::words(2);

                    let before_q: ExtraCounts = quotient.counts.clone();
                    let (projected, projected_rhs) =
                        quotient.project(&dense, rhs_sa).map_err(String::from)?;
                    work::hook_muls(spec.l, quotient.counts.products - before_q.products);
                    work::hook_adds(spec.l, quotient.counts.additions - before_q.additions);
                    work::words(quotient.counts.word_events - before_q.word_events);

                    affine_solver.insert(projected.to_vec(), projected_rhs);
                    let candidate = affine_solver.solved(1);
                    affine_rows.push(CompactAffineRow {
                        nonzeros,
                        target_coeff: neg_sb,
                        rhs: rhs_sa,
                        projected,
                        projected_rhs,
                        solver_rank: affine_solver.rank(),
                    });
                    descent_certs.push(cert);
                    total_la_wall += t_la_row.elapsed().as_secs_f64() * 1e3;

                    if let Some(d) = candidate {
                        let t_v = Instant::now();
                        let _ver = work::enter(Phase::Verification);
                        let ec = EllE::new(&ctx.f, &spec.alpha);
                        if d > 0 && d < spec.l && ec.mul(&spec.g, d) == spec.q {
                            recovered_scalar = Some(d);
                        } else {
                            refusal = Some("candidate elliptic replay failed".into());
                        }
                        ver_wall += t_v.elapsed().as_secs_f64() * 1e3;
                    }
                }

                if recovered_scalar.is_some() || refusal.is_some() {
                    break;
                }
                if descent_certs.len() >= options.max_accepted_descents {
                    refusal = Some("accepted descent cap".into());
                    break;
                }
                residual = jac.add(&residual, &step);
                a = add_mod(a, al, spec.l);
                b = add_mod(b, be, spec.l);
                stream_advances += 1;
            }
            if recovered_scalar.is_none() && refusal.is_none() {
                refusal = Some("descent residual budget exhausted".into());
            }
        }
        let desc_wall = (t_desc.elapsed().as_secs_f64() * 1e3 - ver_wall).max(0.0);
        phase_wall.insert("linear_algebra".into(), total_la_wall);
        phase_wall.insert("descent".into(), desc_wall);
        phase_wall.insert("verification".into(), ver_wall);
        let wall_ms_indicative = t_total.elapsed().as_secs_f64() * 1e3;

        // Collect proof-cone `referenced_sources` for all accepted descent certificates.
        let mut referenced_sources = BTreeMap::<usize, ReferencedSource>::new();
        for cert in &descent_certs {
            for (_, _, _, witness) in &cert.quadratic_terms {
                for &(src_id, _) in witness {
                    referenced_sources
                        .entry(src_id)
                        .or_insert_with(|| all_sources[src_id].clone());
                }
            }
        }

        // Finalize `bitop/v1` accounting and turn `jv_bitops` OFF cleanly so `foreign == 0`.
        let tally = work::snapshot().since(&bitops_before);
        let uniforms = work::uniform_snapshot()
            .since(&uniform_before)
            .ok_or("uniform counter regression")?;
        let foreign_hook_calls = work::foreign().saturating_sub(foreign_before);
        work::enable(false);

        let widths = work::widths(spec.p, spec.l);
        let mut phase_reports = Vec::with_capacity(Phase::IC.len());
        let mut observed_bitops = 0u128;
        for &ph in &Phase::IC {
            let pt = tally.phase(ph);
            let b_cost = work::price(pt, &widths).expect("registered widths");
            observed_bitops += b_cost;
            phase_reports.push(PhaseAccounting {
                phase: ph.name().into(),
                field_mul: pt.get(Kind::FieldMul),
                field_add: pt.get(Kind::FieldAdd),
                field_inv: pt.get(Kind::FieldInv),
                modr_mul: pt.get(Kind::ModrMul),
                modr_add: pt.get(Kind::ModrAdd),
                modr_inv: pt.get(Kind::ModrInv),
                word_op: pt.get(Kind::WordOp),
                hash_probe: pt.get(Kind::HashProbe),
                rng_words: pt.rng_words,
                bitops: b_cost,
            });
        }
        let uni_ic = uniforms.sum(&Phase::IC).ok_or("uniform sum overflow")?;
        let uni_json = work::uniform_json(&uni_ic).ok_or("uniform json overflow")?;
        let uniform_priced_bitops: u128 = uni_json["known_native_priced_bitops"]
            .as_str()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let uniform_unpriced_ops: u64 = ["integer_sub64", "integer_clz64", "integer_shl64"]
            .iter()
            .map(|k| uni_json["unpriced_native"][*k].as_u64().unwrap_or(0))
            .sum();
        let lower_w0 = observed_bitops + uniform_priced_bitops;

        let f_cov = if census.total_unsigned_classes > 0 {
            anchor_table.len() as f64 / census.total_unsigned_classes as f64
        } else {
            0.0
        };
        let predicted_m = if options.descent.uses_anchors() {
            1.0 + 15.0 * f_cov + 45.0 * f_cov * f_cov + 15.0 * f_cov * f_cov * f_cov
        } else {
            1.0
        };

        let arm_label = if options.harvest_odd_cycles {
            format!(
                "{}-cycle:{}",
                options.collector.label(),
                options.descent.label()
            )
        } else {
            format!("{}:{}", options.collector.label(), options.descent.label())
        };

        Ok(AnchoredSolveBundle {
            schema: "cover.anchored-multi-pattern.dlp/v1".into(),
            p: spec.p,
            l: spec.l,
            seed,
            arm_label,
            options,
            instance_alpha: encode_e6(&spec.alpha),
            instance_g: (encode_e6(&spec.g.x), encode_e6(&spec.g.y)),
            instance_q: (encode_e6(&spec.q.x), encode_e6(&spec.q.y)),
            cover_hx: remainder::encode(&ctx.cov.hx),
            gj_u: remainder::encode(&spec.gj.u),
            gj_v: remainder::encode(&spec.gj.v),
            qj_u: remainder::encode(&spec.qj.u),
            qj_v: remainder::encode(&spec.qj.v),
            base_points: base.iter().map(|b| [b.x, b.y.0[0], b.y.0[1]]).collect(),
            universe_census: census,
            collection: CollectionSummary {
                b_space_total,
                b_count_scanned: b_scanned,
                lines_tested,
                candidates_tested,
                candidates_refused,
                duplicates_skipped,
                sources_admitted: all_sources.len(),
                full_sources,
                one_class_sources,
                two_class_sources,
                three_class_sources,
                discriminant_screen_calls: screen_calls,
                discriminant_screen_refusals: screen_refusals,
                projections_emitted: projections.len(),
                rank_achieved: rank.rank_lower_bound(),
                rank_target,
                rank_ready: ready,
                stop_reason,
                graph_vertices: if options.collector.is_matroid() {
                    matroid_graph.stats().vertices
                } else if options.collector.is_two_lp() {
                    deferred_graph.stats().vertices
                } else {
                    sparse_anchors.len()
                },
                unit_anchors: anchor_table.unit_anchors,
                half_anchors: anchor_table.half_anchors,
                total_anchors: anchor_table.len(),
                live_deferred_edges: if options.collector.is_matroid() {
                    matroid_graph
                        .stats()
                        .hyper3_parked
                        .saturating_sub(matroid_graph.stats().hyper3_woken)
                        as usize
                } else if options.collector.is_two_lp() {
                    deferred_graph.stats().live_deferred
                } else {
                    0
                },
                cycle_harvest,
                matroid_stats: options
                    .collector
                    .is_matroid()
                    .then(|| matroid_graph.stats().clone()),
                sources_sha256,
                projections_sha256,
                kernel_orthogonality_verified_rows,
            },
            initial_kernel,
            descent: DescentSummary {
                mode: options.descent,
                residuals_tried,
                stream_advances,
                accepted_rows: descent_certs.len(),
                trace_recorded: trace_state.recorded,
                trace_replayed: trace_state.replayed,
                trace_fallbacks: trace_state.fallbacks,
                timeouts,
                incomplete,
                f4_muls_total,
                lin_muls_total,
                weil_muls_total,
                root_muls_total,
                anchor_coverage_f: f_cov,
                predicted_multiplier_m: predicted_m,
                pattern_histogram: pattern_hist,
            },
            referenced_sources,
            descent_certs,
            affine_rows,
            recovered_scalar,
            verified: recovered_scalar.is_some(),
            refusal,
            wall_ms_indicative,
            phase_wall_ms_indicative: phase_wall,
            accounting: AccountingReport {
                phases: phase_reports,
                observed_bitops,
                uniform_priced_bitops,
                uniform_unpriced_ops,
                foreign_hook_calls,
                schoolbook_band: build_band(lower_w0, uniform_unpriced_ops, schoolbook_gae),
                karatsuba_band: build_band(lower_w0, uniform_unpriced_ops, karatsuba_gae),
            },
        })
    })
}

#[allow(clippy::too_many_arguments)]
fn deferred_baseline_candidate_with_screen(
    ctx: &Ctx,
    ring: &FpRing,
    config: &TwoLargePrimeSieve,
    line: &Line,
    s: u64,
    extraction: &mut Extraction<'_>,
    screen_ctx: &discriminant::Context<'_>,
    screen_calls: &mut u64,
    screen_refusals: &mut u64,
) -> Result<(PairCert, Vec<graph_core::LargeTerm>), &'static str> {
    extraction.trace = None;
    let f = &ctx.f.f;
    if s == 0 || s >= f.p {
        return Err("zero/out-of-range sieve parameter");
    }
    let norm = sieve::line_poly(ring, line, s);
    if norm.len() != 10 {
        return Err("degree nine required");
    }
    let setup = config.setup();
    let sq = if setup.sqrt[s as usize] != u64::MAX {
        E2([setup.sqrt[s as usize], 0])
    } else {
        let r = setup.sqrt[mul_mod(s, setup.winv, f.p) as usize];
        if r == u64::MAX {
            return Err("square root table");
        }
        E2([0, r])
    };
    let a: Vec<_> = (0..line.a0.len().max(line.a1.len()))
        .map(|i| {
            E2([
                *line.a0.get(i).unwrap_or(&0),
                mul_mod(s, *line.a1.get(i).unwrap_or(&0), f.p),
            ])
        })
        .collect();
    let c: Vec<_> = line.b.iter().map(|b| f.mul(&sq, b)).collect();
    let mut tail = norm.clone();
    let mut terms = Vec::new();
    let roots = ring.roots_by_table(&norm);
    for (i, b) in config.base().iter().enumerate() {
        work::words(2);
        let found = roots.iter().position(|&x| x == b.x);
        work::words(found.map_or(roots.len(), |idx| idx + 1) as u64);
        if found.is_none() {
            continue;
        }
        let divisor = vec![if b.x == 0 { 0 } else { f.p - b.x }, 1];
        let (quotient, residue) = ring.divrem(&tail, &divisor);
        if !residue.is_empty() {
            return Err("root division");
        }
        tail = quotient;
        let den = remainder::eval(f, &c, b.x);
        if den == E2::ZERO {
            return Err("zero denominator");
        }
        let y = f.neg(&f.mul(&remainder::eval(f, &a, b.x), &f.inv(&den)));
        let sign = if y == b.y {
            1
        } else if y == f.neg(&b.y) {
            -1
        } else {
            return Err("ordinate mismatch");
        };
        terms.push((i, sign));
    }
    let factors = match tail.len() {
        1 => Vec::new(),
        3 => vec![ring.monic(&tail)],
        5 => {
            *screen_calls += 1;
            let assessment = screen_ctx.screen(&tail)?;
            if assessment.rejects() {
                *screen_refusals += 1;
                return Err("quartic discriminant necessary condition");
            }
            let report = pair::split_quartic(ring, &tail, extraction.options, extraction.rng)?;
            let factors = report.factors.clone();
            extraction.trace = Some((tail, report));
            factors
                .ok_or("quartic split refused")?
                .into_iter()
                .collect()
        }
        _ => return Err("norm support is not nine/seven/five distinct small points"),
    };
    let checked = pair::check_equation(
        f,
        &ctx.cov.hx,
        config.base(),
        &remainder::trim(a),
        &remainder::trim(c),
        &norm,
        &terms,
        &factors,
    )?;
    if checked.certificate().remainders.is_empty()
        && !pair::replay_group(&ctx.jac(), config.base(), checked.certificate())
    {
        return Err("full Jacobian check");
    }
    let large = checked.large_terms();
    Ok((checked.into_certificate(), large))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RhoWalkSample {
    pub run_index: u64,
    pub batch: usize,
    pub steps: u64,
    pub wall_ms_indicative: f64,
    pub verified: bool,
    pub schoolbook_bitops: u128,
    pub karatsuba_bitops: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatchedRhoReference {
    pub p: u64,
    pub l: u64,
    pub expected_negation_steps: f64,
    pub analytical_b1_schoolbook_bitops: u128,
    pub analytical_b1_karatsuba_bitops: u128,
    pub analytical_b16_schoolbook_bitops: u128,
    pub analytical_b16_karatsuba_bitops: u128,
    pub empirical_walks: Vec<RhoWalkSample>,
    pub empirical_b16_median_steps: u64,
    pub empirical_b16_mean_schoolbook_bitops: u128,
    pub empirical_b16_mean_karatsuba_bitops: u128,
    pub empirical_b16_median_schoolbook_bitops: u128,
    pub empirical_b16_median_karatsuba_bitops: u128,
}

/// Analytical expectation (`sqrt(pi * l / 4)` steps under Bos-Kleinjung-Lenstra negation
/// quotient with Montgomery batch `B`) plus multi-walk empirical `strong_rho` runs,
/// priced symmetrically under both schoolbook and Karatsuba + Chung-Hasan `F_{q^3}` towers.
pub fn run_matched_rho_reference(
    spec: &Spec,
    expected_scalar_for_check: Option<u64>,
    seed: u64,
    num_b16_walks: u64,
    run_empirical: bool,
) -> MatchedRhoReference {
    work::enable(true);
    work::claim();
    work::set_moduli(spec.p, spec.l);
    let sb_prims = rho_strong::measure_native_fq3_prims(spec.p, spec.l);
    let ka_prims = rho_strong::measure_karatsuba_fq3_prims(spec.p, spec.l);
    work::enable(false);

    let widths = work::widths(spec.p, spec.l);
    let exp_steps = (std::f64::consts::PI * (spec.l as f64) / 4.0).sqrt();

    let analytical_for = |batch: u64, pr: &Fq3Prims| -> u128 {
        // Per negation-map step at Montgomery batch `B`:
        // - `batch == 1`: 2 F_{q^3} muls + 1 sq + 1 inv + 42 F_p adds + 6 canon F_p adds + 14 word_ops + 1 modr_add
        // - `batch > 1`: (5*B - 3)/B muls + 1 sq + (1/B) inv + 48 F_p adds + 14 word_ops + 1 modr_add
        let steps = exp_steps.round() as u64;
        let batches = steps.div_ceil(batch);
        let muls = if batch == 1 {
            2 * steps
        } else {
            (5 * batch - 3) * batches
        };
        let sqs = batch * batches;
        let invs = batches;
        let mut t = OpTally::default();
        for k in Kind::ALL {
            let i = k as usize;
            t.counts[i] = muls * pr.mul.counts[i] + sqs * pr.sq.counts[i] + invs * pr.inv.counts[i];
        }
        t.counts[Kind::FieldAdd as usize] += 48 * steps;
        t.counts[Kind::WordOp as usize] += 14 * steps;
        t.counts[Kind::ModrAdd as usize] += steps;
        work::price(&t, &widths).expect("registered widths")
    };

    let analytical_b1_sb = analytical_for(1, &sb_prims);
    let analytical_b1_ka = analytical_for(1, &ka_prims);
    let analytical_b16_sb = analytical_for(16, &sb_prims);
    let analytical_b16_ka = analytical_for(16, &ka_prims);

    let mut walks = Vec::new();
    if run_empirical && num_b16_walks > 0 {
        let inst = CurveInstance::from_spec(spec);
        let dp_bits = (((spec.l as f64).sqrt().log2() - 6.0).round() as i32).clamp(4, 14) as u32;
        for run_idx in 0..num_b16_walks {
            let opts = StrongOpts {
                negation: true,
                batch: 16,
                dp_bits,
                r: 64,
            };
            let t0 = Instant::now();
            let (cr, recovered_d) = rho_strong::strong_rho_counted_with_scalar(
                &inst,
                seed ^ 0x5248_4F00,
                run_idx,
                opts,
            );
            let wall_ms = t0.elapsed().as_secs_f64() * 1e3;
            let tally_sb = rho_strong::rho_tally(&cr, &sb_prims);
            let tally_ka = rho_strong::rho_tally(&cr, &ka_prims);
            let sb_cost = Phase::RHO
                .iter()
                .map(|&ph| work::price(tally_sb.phase(ph), &widths).unwrap())
                .sum();
            let ka_cost = Phase::RHO
                .iter()
                .map(|&ph| work::price(tally_ka.phase(ph), &widths).unwrap())
                .sum();
            walks.push(RhoWalkSample {
                run_index: run_idx,
                batch: 16,
                steps: cr.run.steps,
                wall_ms_indicative: wall_ms,
                verified: cr.verified
                    && expected_scalar_for_check.is_none_or(|want| recovered_d == Some(want)),
                schoolbook_bitops: sb_cost,
                karatsuba_bitops: ka_cost,
            });
        }
    }

    let (med_steps, mean_sb, mean_ka, med_sb, med_ka) = if walks.is_empty() {
        (0, 0, 0, 0, 0)
    } else {
        let n = walks.len() as u128;
        let mut steps_sorted: Vec<u64> = walks.iter().map(|w| w.steps).collect();
        steps_sorted.sort_unstable();
        let mut sb_sorted: Vec<u128> = walks.iter().map(|w| w.schoolbook_bitops).collect();
        sb_sorted.sort_unstable();
        let mut ka_sorted: Vec<u128> = walks.iter().map(|w| w.karatsuba_bitops).collect();
        ka_sorted.sort_unstable();
        let mid = walks.len() / 2;
        (
            steps_sorted[mid],
            walks.iter().map(|w| w.schoolbook_bitops).sum::<u128>() / n,
            walks.iter().map(|w| w.karatsuba_bitops).sum::<u128>() / n,
            sb_sorted[mid],
            ka_sorted[mid],
        )
    };

    MatchedRhoReference {
        p: spec.p,
        l: spec.l,
        expected_negation_steps: exp_steps,
        analytical_b1_schoolbook_bitops: analytical_b1_sb,
        analytical_b1_karatsuba_bitops: analytical_b1_ka,
        analytical_b16_schoolbook_bitops: analytical_b16_sb,
        analytical_b16_karatsuba_bitops: analytical_b16_ka,
        empirical_walks: walks,
        empirical_b16_median_steps: med_steps,
        empirical_b16_mean_schoolbook_bitops: mean_sb,
        empirical_b16_mean_karatsuba_bitops: mean_ka,
        empirical_b16_median_schoolbook_bitops: med_sb,
        empirical_b16_median_karatsuba_bitops: med_ka,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphEngineRow {
    pub engine: String,
    pub max_arity: usize,
    pub online_streaming: bool,
    pub exact_z_cofactor: bool,
    pub input_sources_used: usize,
    pub complete_rows_emitted: usize,
    pub certified_fb_rank: usize,
    pub rank_target: usize,
    pub full_rank_reached: bool,
    pub matrix_nonzeros: usize,
    pub mean_row_weight: f64,
    pub max_row_weight: usize,
    pub exported_unit_anchors: usize,
    pub exported_half_anchors: usize,
    pub word_ops: u64,
    pub hash_probes: u64,
    pub modr_muls: u64,
    pub modr_invs: u64,
    pub graph_bitops: u128,
    pub wall_ms_indicative: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphEngineStreamComparison {
    pub p: u64,
    pub l: u64,
    pub seed: u64,
    pub base_len: usize,
    pub total_stream_sources: usize,
    pub arity_0_sources: usize,
    pub arity_1_sources: usize,
    pub arity_2_sources: usize,
    pub arity_3_sources: usize,
    pub engines: Vec<GraphEngineRow>,
}

/// Collect a canonical stream of verified `0/1/2/3-LP` partial relations on `b_count`
/// planes so multiple partial-relation graph algorithms can be compared head-to-head
/// on the exact same input stream.
pub fn collect_partial_relation_stream(
    spec: &Spec,
    seed: u64,
    b_count: u64,
    max_arity: usize,
) -> Result<(usize, Vec<ReferencedSource>), String> {
    let ctx = Ctx::new(spec);
    let mut base_rng = StdRng::seed_from_u64(seed ^ 0xBACE_0001);
    let base = cover::factor_base(&ctx, &mut base_rng);
    let config = TwoLargePrimeSieve::new(&ctx, &base, base.len(), 9)?;
    let mut base_by_x = vec![u32::MAX; spec.p as usize];
    for (i, b) in base.iter().enumerate() {
        base_by_x[b.x as usize] = i as u32;
    }
    let ring = FpRing::new(spec.p);
    let screen_ctx = discriminant::Context::new(&ring)?;
    let mut buckets = HitBuckets::new(spec.p);
    let mut enum_rng = StdRng::seed_from_u64(seed ^ 0xA17E_2E17);
    let mut split_rng = StdRng::seed_from_u64(seed ^ 0x5155_4144_5041_4952);
    let b_space_total = sieve::b_space(config.setup());
    let b_limit = (b_count as u128).min(b_space_total) as u64;
    let scramble = sieve::coprime_scramble(b_space_total, &mut enum_rng);
    let nu = sieve::nu_lines(config.setup());
    let mut slot = Vec::new();
    let mut seen = HashSet::<PairKey>::new();
    let mut out = Vec::<ReferencedSource>::new();
    let mut screen_calls = 0u64;
    let mut screen_refusals = 0u64;
    let min_hits = if max_arity >= 3 { 3u8 } else { 5u8 };

    let mut b_scanned = 0u64;
    while b_scanned < b_limit {
        let begin = b_scanned;
        let end = (begin + 1024).min(b_limit);
        let batch: Vec<_> = (begin..end)
            .map(|k| sieve::b_from_index(config.setup(), k as u128, scramble))
            .collect();
        let ab: Vec<_> = batch.iter().map(|b| (b[0].0[0], b[0].0[1])).collect();
        let hints = sieve::roots_by_incidence(spec.p, &nu, &ab, &mut slot).0;
        for (offset, b) in batch.iter().enumerate() {
            let found = sieve::lines_for_b_hint(
                &ring,
                &ctx.f.f,
                config.setup(),
                b,
                &mut enum_rng,
                true,
                hints[offset].as_deref(),
            );
            let shared = if found.len() >= 2 {
                Some(sieve::shared_for_b(&ring, &found[0], &mut 0))
            } else {
                None
            };
            for line in found {
                let (mut hits, _) = sieve::sieve_line_buckets(
                    &ring,
                    config.setup(),
                    &line,
                    &mut buckets,
                    shared.as_ref(),
                    min_hits,
                );
                hits.sort_unstable();
                let mut three_lp_on_line = 0usize;
                for pass in 0..2 {
                    for &s in &hits {
                        let raw_xs = buckets.hits_for(s);
                        let cnt = raw_xs.len();
                        if pass == 0 {
                            if !matches!(cnt, 5 | 7 | 9) {
                                continue;
                            }
                        } else {
                            if max_arity < 3 || cnt != 3 || three_lp_on_line >= 2 {
                                continue;
                            }
                            three_lp_on_line += 1;
                        }
                        let Ok(mut src) = extract_partial_source_matroid(
                            &ctx,
                            &ring,
                            &config,
                            &base_by_x,
                            &line,
                            s,
                            raw_xs,
                            max_arity,
                            SplitOptions::default(),
                            &mut split_rng,
                            Some(&screen_ctx),
                            &mut screen_calls,
                            &mut screen_refusals,
                        ) else {
                            continue;
                        };
                        if !seen.insert(matroid_dedup_key(&src)) {
                            continue;
                        }
                        src.source_id = out.len();
                        out.push(src);
                    }
                }
            }
        }
        b_scanned = end;
    }
    Ok((base.len(), out))
}

fn certify_projection_rows_rank(
    l: u64,
    base_len: usize,
    _seed: u64,
    rows: &[Vec<(usize, u64)>],
) -> Result<usize, String> {
    if rows.is_empty() || base_len <= 1 {
        return Ok(0);
    }
    work::enable(false);
    let mut sys = CountedRelationSystem::new(l, base_len);
    for r in rows {
        let mut dense = vec![0u64; base_len];
        for &(col, val) in r {
            if col >= base_len {
                return Err("projected column out of range".into());
            }
            dense[col] = val % l;
        }
        sys.insert(dense, 0);
        if sys.rank() + 1 >= base_len {
            break;
        }
    }
    Ok(sys.rank())
}

/// Compare 7 partial-relation graph algorithms side-by-side on the exact same
/// collected `ReferencedSource` stream under `bitop/v1` accounting + certified
/// factor-base rank + matrix sparsity + exported descent anchors.
pub fn benchmark_graph_engines_on_stream(
    p: u64,
    l: u64,
    seed: u64,
    base_len: usize,
    sources: &[ReferencedSource],
) -> Result<GraphEngineStreamComparison, String> {
    let rank_target = base_len.saturating_sub(1);
    let widths = work::widths(p, l);
    let arity_0 = sources.iter().filter(|s| s.remainders.is_empty()).count();
    let arity_1 = sources.iter().filter(|s| s.remainders.len() == 1).count();
    let arity_2 = sources.iter().filter(|s| s.remainders.len() == 2).count();
    let arity_3 = sources.iter().filter(|s| s.remainders.len() == 3).count();

    // Consecutive-id slice for `arity <= 2` engines so `accept(0..M)` is contiguous.
    let sources_2lp: Vec<ReferencedSource> = sources
        .iter()
        .filter(|s| s.remainders.len() <= 2)
        .enumerate()
        .map(|(i, s)| {
            let mut c = s.clone();
            c.source_id = i;
            c
        })
        .collect();

    // Map `[u64; 6]` large classes to dense `u64` ids for `PartialRelation`-based batch filters.
    let mut lp_id_map = HashMap::<[u64; 6], u64>::with_capacity(sources.len() * 2);
    let mut next_lp_id = 1u64;
    for s in sources {
        for &(key, _) in &s.remainders {
            lp_id_map.entry(key).or_insert_with(|| {
                let id = next_lp_id;
                next_lp_id += 1;
                id
            });
        }
    }

    let to_partial = |src: &ReferencedSource| -> PartialRelation {
        PartialRelation {
            id: src.source_id as u64,
            small: src
                .terms
                .iter()
                .map(|&(col, sgn)| (col as u32, (sgn as i64).rem_euclid(l as i64) as u64))
                .collect(),
            large: src
                .remainders
                .iter()
                .map(|&(key, sgn)| (lp_id_map[&key], (sgn as i64).rem_euclid(l as i64) as u64))
                .collect(),
            rhs: 0,
        }
    };

    let partials_2lp: Vec<PartialRelation> = sources_2lp.iter().map(to_partial).collect();
    let partials_3lp: Vec<PartialRelation> = sources.iter().map(to_partial).collect();

    let summarize = |engine: &str,
                     max_arity: usize,
                     online_streaming: bool,
                     exact_z_cofactor: bool,
                     input_sources_used: usize,
                     rows: Vec<Vec<(usize, u64)>>,
                     unit_anchors: usize,
                     half_anchors: usize,
                     tally: OpTally,
                     wall_ms: f64|
     -> Result<GraphEngineRow, String> {
        let fb_rank = certify_projection_rows_rank(l, base_len, seed, &rows)?;
        let nonzeros: usize = rows.iter().map(|r| r.len()).sum();
        let max_w = rows.iter().map(|r| r.len()).max().unwrap_or(0);
        let mean_w = if rows.is_empty() {
            0.0
        } else {
            nonzeros as f64 / rows.len() as f64
        };
        let bitops = work::price(&tally, &widths).map_err(|_| "unregistered widths")?;
        Ok(GraphEngineRow {
            engine: engine.to_string(),
            max_arity,
            online_streaming,
            exact_z_cofactor,
            input_sources_used,
            complete_rows_emitted: rows.len(),
            certified_fb_rank: fb_rank,
            rank_target,
            full_rank_reached: fb_rank >= rank_target,
            matrix_nonzeros: nonzeros,
            mean_row_weight: mean_w,
            max_row_weight: max_w,
            exported_unit_anchors: unit_anchors,
            exported_half_anchors: half_anchors,
            word_ops: tally.counts[Kind::WordOp as usize],
            hash_probes: tally.counts[Kind::HashProbe as usize],
            modr_muls: tally.counts[Kind::ModrMul as usize],
            modr_invs: tally.counts[Kind::ModrInv as usize],
            graph_bitops: bitops,
            wall_ms_indicative: wall_ms,
        })
    };

    let mut engines = Vec::with_capacity(7);

    // 1. 1-LP Singleton Hash Table (`arity <= 1`).
    {
        work::enable(false);
        work::enable(true);
        work::claim();
        work::set_moduli(p, l);
        let snap0 = work::snapshot();
        let t0 = Instant::now();
        let (rows, anchors_cnt, used) = {
            let _g = work::enter(Phase::Relations);
            let mut tbl = HashMap::<[u64; 6], usize>::new();
            let mut rows = Vec::new();
            let mut sb = Vec::new();
            let mut lb = Vec::new();
            let mut used = 0usize;
            for s in sources.iter().filter(|s| s.remainders.len() <= 1) {
                used += 1;
                if s.remainders.is_empty() {
                    rows.push(project_sources_i16(
                        sources,
                        &[(s.source_id, 1)],
                        l,
                        &mut sb,
                        &mut lb,
                    )?);
                } else {
                    let (key, coeff) = s.remainders[0];
                    work::probe(1);
                    if let Some(&anc_id) = tbl.get(&key) {
                        let c0 = sources[anc_id].remainders[0].1 as i16;
                        let mut zw = vec![(anc_id, -(coeff as i16)), (s.source_id, c0)];
                        zw.sort_unstable_by_key(|&(id, _)| id);
                        rows.push(project_sources_i16(sources, &zw, l, &mut sb, &mut lb)?);
                    } else {
                        tbl.insert(key, s.source_id);
                    }
                }
            }
            (rows, tbl.len(), used)
        };
        let wall = t0.elapsed().as_secs_f64() * 1e3;
        let tally = *work::snapshot().since(&snap0).phase(Phase::Relations);
        work::enable(false);
        engines.push(summarize(
            "1lp-singleton-table",
            1,
            true,
            true,
            used,
            rows,
            anchors_cnt,
            0,
            tally,
            wall,
        )?);
    }

    // 2. Legacy 2-LP Deferred-Pair BFS (`jv_deferred_pair_graph::Graph`, `arity <= 2`).
    {
        work::enable(false);
        work::enable(true);
        work::claim();
        work::set_moduli(p, l);
        let snap0 = work::snapshot();
        let t0 = Instant::now();
        let (rows, anchors_cnt, used) = {
            let _g = work::enter(Phase::Relations);
            let mut dg = deferred::Graph::new(deferred::Options {
                max_sources: sources_2lp.len() + 16,
                max_vertices: sources_2lp.len() * 2 + 16,
                max_deferred_edges: sources_2lp.len() + 16,
                max_witness_terms: 512,
            })
            .map_err(String::from)?;
            let mut rows = Vec::new();
            for s in &sources_2lp {
                let large: Vec<graph_core::LargeTerm> = s
                    .remainders
                    .iter()
                    .map(|&(class, coefficient)| graph_core::LargeTerm { class, coefficient })
                    .collect();
                let evs = dg.accept(s.source_id, &large).map_err(String::from)?;
                for ev in evs {
                    if let deferred::Event::Full { witness, .. } = ev {
                        rows.push(project_sources(&sources_2lp, &witness, l)?);
                    }
                }
            }
            (rows, dg.stats().anchors, sources_2lp.len())
        };
        let wall = t0.elapsed().as_secs_f64() * 1e3;
        let tally = *work::snapshot().since(&snap0).phase(Phase::Relations);
        work::enable(false);
        engines.push(summarize(
            "2lp-deferred-bfs",
            2,
            true,
            true,
            used,
            rows,
            anchors_cnt,
            0,
            tally,
            wall,
        )?);
    }

    // 3. `F_ell` Frame-Matroid Forest Witness Extractor (`collect_witnesses_lazy`, `arity <= 2`).
    {
        work::enable(false);
        work::enable(true);
        work::claim();
        work::set_moduli(p, l);
        let snap0 = work::snapshot();
        let t0 = Instant::now();
        let (rows, used) = {
            let _g = work::enter(Phase::Relations);
            let lp_idx =
                LpIndex::build(&partials_2lp, l).map_err(|e| format!("LpIndex::build: {e:?}"))?;
            work::words((partials_2lp.len() * 8) as u64);
            work::probe((partials_2lp.len() * 2) as u64);
            let batch = collect_witnesses_lazy(&lp_idx)
                .map_err(|e| format!("collect_witnesses_lazy: {e:?}"))?;
            for _ in 0..batch.stats.inverse_calls {
                work::hook_inv(l);
            }
            work::hook_muls(
                l,
                2 * batch.stats.path_steps + batch.stats.source_terms as u64,
            );
            work::words(4 * (batch.stats.parent_path_reads + batch.stats.reroot_steps));
            let mut rows = Vec::with_capacity(batch.witnesses.len());
            for w in &batch.witnesses {
                let mut acc = BTreeMap::<usize, u128>::new();
                for &(row_pos, weight) in &w.sources {
                    let pr = &partials_2lp[row_pos];
                    for &(col, sval) in &pr.small {
                        *acc.entry(col as usize).or_default() += (sval as u128) * (weight as u128);
                        work::words(3);
                        work::hook_mul(l);
                    }
                }
                rows.push(
                    acc.into_iter()
                        .filter_map(|(c, v)| {
                            let r = (v % (l as u128)) as u64;
                            (r != 0).then_some((c, r))
                        })
                        .collect(),
                );
            }
            (rows, partials_2lp.len())
        };
        let wall = t0.elapsed().as_secs_f64() * 1e3;
        let tally = *work::snapshot().since(&snap0).phase(Phase::Relations);
        work::enable(false);
        engines.push(summarize(
            "2lp-forest-witness-fl",
            2,
            true,
            false,
            used,
            rows,
            0,
            0,
            tally,
            wall,
        )?);
    }

    // 4. Online Exact-Z Signed Frame-Matroid DSU (`SignedMatroidGraph`, `arity <= 2`).
    {
        work::enable(false);
        work::enable(true);
        work::claim();
        work::set_moduli(p, l);
        let snap0 = work::snapshot();
        let t0 = Instant::now();
        let (rows, unit_anc, half_anc, used) = {
            let _g = work::enter(Phase::Relations);
            let mut mg = SignedMatroidGraph::new(MatroidOptions {
                max_sources: sources_2lp.len() + 16,
                max_vertices: sources_2lp.len() * 2 + 16,
                max_tree_edges: sources_2lp.len() + 16,
                max_hyperedges: sources_2lp.len() + 16,
                max_witness_terms: 512,
                max_abs_coeff: 4096,
            })
            .map_err(String::from)?;
            let mut rows = Vec::new();
            let mut sb = Vec::new();
            let mut lb = Vec::new();
            for s in &sources_2lp {
                let large: Vec<graph_core::LargeTerm> = s
                    .remainders
                    .iter()
                    .map(|&(class, coefficient)| graph_core::LargeTerm { class, coefficient })
                    .collect();
                let emitted = mg.accept(s.source_id, &large).map_err(String::from)?;
                for zw in &emitted {
                    rows.push(project_sources_i16(&sources_2lp, zw, l, &mut sb, &mut lb)?);
                }
            }
            let (tbl, _, _) = mg.export_anchor_table();
            (rows, tbl.unit_anchors, tbl.half_anchors, sources_2lp.len())
        };
        let wall = t0.elapsed().as_secs_f64() * 1e3;
        let tally = *work::snapshot().since(&snap0).phase(Phase::Relations);
        work::enable(false);
        engines.push(summarize(
            "2lp-signed-matroid-dsu",
            2,
            true,
            true,
            used,
            rows,
            unit_anc,
            half_anc,
            tally,
            wall,
        )?);
    }

    // 5. Offline 2-Core Peeling + Markowitz Minimum-Degree Elimination (`arity <= 3`).
    {
        let input_clone = partials_3lp.clone();
        work::enable(false);
        work::enable(true);
        work::claim();
        work::set_moduli(p, l);
        let snap0 = work::snapshot();
        let t0 = Instant::now();
        let rows = {
            let _g = work::enter(Phase::Relations);
            let (complete, _) = filter_large_primes(
                input_clone,
                &LargePrimeFilterOptions {
                    modulus: l,
                    max_small_weight: base_len + 64,
                    max_sources: 4096,
                },
            )
            .map_err(|e| format!("filter_large_primes: {e:?}"))?;
            complete
                .into_iter()
                .map(|r| r.small.into_iter().map(|(c, v)| (c as usize, v)).collect())
                .collect::<Vec<_>>()
        };
        let wall = t0.elapsed().as_secs_f64() * 1e3;
        let tally = *work::snapshot().since(&snap0).phase(Phase::Relations);
        work::enable(false);
        engines.push(summarize(
            "klp-markowitz-2core",
            3,
            false,
            false,
            partials_3lp.len(),
            rows,
            0,
            0,
            tally,
            wall,
        )?);
    }

    // 6. Offline 2-Core Peeling + Shared-Key Prim MST Elimination (`arity <= 3`).
    {
        let input_clone = partials_3lp.clone();
        work::enable(false);
        work::enable(true);
        work::claim();
        work::set_moduli(p, l);
        let snap0 = work::snapshot();
        let t0 = Instant::now();
        let rows = {
            let _g = work::enter(Phase::Relations);
            let attempt = filter_large_primes_mst_with(
                input_clone,
                &LargePrimeFilterOptions {
                    modulus: l,
                    max_small_weight: base_len + 64,
                    max_sources: 4096,
                },
                &MstLimits {
                    max_rows: partials_3lp.len() + 64,
                    max_column_degree: 256,
                    max_edge_evaluations: 4_000_000,
                    max_active_coefficients: 4_000_000,
                },
                MstStrategy::SharedKey,
            )
            .map_err(|e| format!("filter_large_primes_mst_with: {e:?}"))?;
            attempt
                .relations
                .ok_or("MST filter refused budget")?
                .into_iter()
                .map(|r| r.small.into_iter().map(|(c, v)| (c as usize, v)).collect())
                .collect::<Vec<_>>()
        };
        let wall = t0.elapsed().as_secs_f64() * 1e3;
        let tally = *work::snapshot().since(&snap0).phase(Phase::Relations);
        work::enable(false);
        engines.push(summarize(
            "klp-sharedkey-prim-mst",
            3,
            false,
            false,
            partials_3lp.len(),
            rows,
            0,
            0,
            tally,
            wall,
        )?);
    }

    // 7. Online Exact-Z Signed Frame-Matroid DSU + Watched 3-LP Hypergraph (`arity <= 3`).
    {
        work::enable(false);
        work::enable(true);
        work::claim();
        work::set_moduli(p, l);
        let snap0 = work::snapshot();
        let t0 = Instant::now();
        let (rows, unit_anc, half_anc) = {
            let _g = work::enter(Phase::Relations);
            let mut mg = SignedMatroidGraph::new(MatroidOptions {
                max_sources: sources.len() + 16,
                max_vertices: sources.len() * 3 + 16,
                max_tree_edges: sources.len() + 16,
                max_hyperedges: sources.len() + 16,
                max_witness_terms: 512,
                max_abs_coeff: 4096,
            })
            .map_err(String::from)?;
            let mut rows = Vec::new();
            let mut sb = Vec::new();
            let mut lb = Vec::new();
            for s in sources {
                let large: Vec<graph_core::LargeTerm> = s
                    .remainders
                    .iter()
                    .map(|&(class, coefficient)| graph_core::LargeTerm { class, coefficient })
                    .collect();
                let emitted = mg.accept(s.source_id, &large).map_err(String::from)?;
                for zw in &emitted {
                    rows.push(project_sources_i16(sources, zw, l, &mut sb, &mut lb)?);
                }
            }
            let (tbl, _, _) = mg.export_anchor_table();
            (rows, tbl.unit_anchors, tbl.half_anchors)
        };
        let wall = t0.elapsed().as_secs_f64() * 1e3;
        let tally = *work::snapshot().since(&snap0).phase(Phase::Relations);
        work::enable(false);
        engines.push(summarize(
            "3lp-signed-matroid-hyper",
            3,
            true,
            true,
            sources.len(),
            rows,
            unit_anc,
            half_anc,
            tally,
            wall,
        )?);
    }

    Ok(GraphEngineStreamComparison {
        p,
        l,
        seed,
        base_len,
        total_stream_sources: sources.len(),
        arity_0_sources: arity_0,
        arity_1_sources: arity_1,
        arity_2_sources: arity_2,
        arity_3_sources: arity_3,
        engines,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fast_deferred_extractor_preserves_exact_certificates_and_reduces_relations_bitops() {
        // Test on a small bounded B slice at p=307 to verify byte-identical Phase-1 certificates
        // and strictly smaller Relations bitops (`field_inv` and `word_op`).
        let mut spec = cover::generate_spec(307, 19);
        spec.d = 0; // wipe secret scalar
        let opts_slow = AnchoredPipelineOptions {
            collector: CollectorArm::DeferredPairs,
            descent: DescentMode::TracedAnchored,
            b_count: Some(2048),
            max_descent_residuals: 1,
            ..Default::default()
        };
        let mut opts_fast = opts_slow;
        opts_fast.collector = CollectorArm::DeferredPairsFast;

        let b_slow = run_anchored_ic_dlp_on(&spec, 19, opts_slow).expect("slow");
        let b_fast = run_anchored_ic_dlp_on(&spec, 19, opts_fast).expect("fast");

        assert_eq!(
            b_slow.collection.sources_admitted,
            b_fast.collection.sources_admitted
        );
        assert_eq!(
            b_slow.collection.sources_sha256,
            b_fast.collection.sources_sha256
        );
        assert_eq!(
            b_slow.collection.projections_sha256,
            b_fast.collection.projections_sha256
        );
        assert_eq!(
            b_slow.collection.unit_anchors,
            b_fast.collection.unit_anchors
        );
        assert_eq!(b_slow.accounting.foreign_hook_calls, 0);
        assert_eq!(b_fast.accounting.foreign_hook_calls, 0);

        let rel_slow = b_slow
            .accounting
            .phases
            .iter()
            .find(|p| p.phase == "relations")
            .unwrap();
        let rel_fast = b_fast
            .accounting
            .phases
            .iter()
            .find(|p| p.phase == "relations")
            .unwrap();
        assert!(rel_fast.field_inv < rel_slow.field_inv);
        assert!(rel_fast.word_op < rel_slow.word_op);
        assert!(rel_fast.bitops < rel_slow.bitops);
    }

    #[test]
    fn signed_matroid_2lp_and_7_engine_stream_benchmark_work_and_beat_legacy_bfs() {
        let mut spec = cover::generate_spec(307, 101);
        let secret_d = spec.d;
        spec.d = 0;

        let b_matroid = run_anchored_ic_dlp_on(
            &spec,
            101,
            AnchoredPipelineOptions {
                collector: CollectorArm::SignedMatroid2lp,
                descent: DescentMode::TracedAnchored,
                ..Default::default()
            },
        )
        .expect("signed-matroid-2lp solve");
        assert!(b_matroid.verified);
        assert_eq!(b_matroid.recovered_scalar, Some(secret_d));
        assert_eq!(b_matroid.accounting.foreign_hook_calls, 0);

        let (base_len, stream) =
            collect_partial_relation_stream(&spec, 101, 24_576, 3).expect("stream");
        let cmp = benchmark_graph_engines_on_stream(spec.p, spec.l, 101, base_len, &stream)
            .expect("7-engine benchmark");
        assert_eq!(cmp.engines.len(), 7);
        let legacy = cmp
            .engines
            .iter()
            .find(|e| e.engine == "2lp-deferred-bfs")
            .unwrap();
        let matroid2 = cmp
            .engines
            .iter()
            .find(|e| e.engine == "2lp-signed-matroid-dsu")
            .unwrap();
        let matroid3 = cmp
            .engines
            .iter()
            .find(|e| e.engine == "3lp-signed-matroid-hyper")
            .unwrap();
        assert!(matroid2.complete_rows_emitted > 0);
        assert!(
            matroid2.certified_fb_rank >= legacy.certified_fb_rank,
            "matroid DSU harvests unanchored cycles so rank >= legacy BFS"
        );
        assert!(
            matroid3.complete_rows_emitted >= matroid2.complete_rows_emitted,
            "3-LP hypergraph cascade emits >= 2-LP rows"
        );
        assert!(
            matroid2.graph_bitops < legacy.graph_bitops,
            "matroid DSU graph bitops ({}) must beat legacy BTreeMap BFS ({})",
            matroid2.graph_bitops,
            legacy.graph_bitops
        );
    }
}
