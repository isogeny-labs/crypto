//! Screened two-quadratic IC with bounded deferred-edge activation and sparse rank.
//! Declared successor prototype; older grounded collectors and wire remain intact.
#[path = "jv_nagao_unit.rs"]
mod nagao_unit;
#[path = "jv_nagao_unit_elimination.rs"]
mod nagao_unit_elimination;
use super::deferred_pair_graph as deferred;
use super::homogeneous_quotient::{ExtraCounts, Quotient};
use super::quadratic_pair_relation::{self as pair, Certificate, SplitOptions, SplitReport};
use super::quartic_discriminant as discriminant;
use super::sparse_collection_rank::Tracker;
use super::{principal_relation as remainder, quadratic_pair_graph as graph_core};
use crate::cryptanalysis::{
    jv_bitops::{self as work, Phase},
    jv_cover::{self as cover, BaseEl, Ctx, EllE, Fld, Spec, E2},
    jv_sieve::{
        self as sieve,
        two_large_prime::{CountedRelationSystem, TwoLargePrimeSieve},
        Counters, FpRing, Line, SharedB, SieveCost,
    },
    residual_walk::is_prime_u64,
};
use rand::{rngs::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: u64,
    pub y: [u64; 2],
}
pub fn points(base: &[BaseEl]) -> Vec<Point> {
    base.iter().map(|b| Point { x: b.x, y: b.y.0 }).collect()
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Source {
    pub candidate_index: u64,
    pub b_index: u64,
    pub a0: Vec<u64>,
    pub a1: Vec<u64>,
    pub b: Vec<[u64; 2]>,
    pub s: u64,
    pub certificate: Certificate,
    /// Complete ordered events caused by this source, including older edges.
    pub graph: Vec<deferred::Event>,
}
/// Every quartic request is retained, including failed splits and candidates
/// rejected after a successful split. Enumeration and splitting use separate RNGs.
#[derive(Clone, Serialize, Deserialize)]
pub struct SplitRecord {
    pub candidate_index: u64,
    pub b_index: u64,
    pub a0: Vec<u64>,
    pub a1: Vec<u64>,
    pub b: Vec<[u64; 2]>,
    pub s: u64,
    pub tail: Vec<u64>,
    pub report: SplitReport,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ScreenRefusal {
    pub candidate_index: u64,
    pub assessment: discriminant::Screen,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Projection {
    pub terms: Vec<(usize, u64)>,
    pub witness: Vec<(usize, i8)>,
    pub rank: usize,
    pub products: u64,
    /// Arrival id that made this row available; may cancel from final witness.
    pub activation_source: usize,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Checkpoint {
    pub b_count: u64,
    pub lines: u64,
    pub cohort_sha256: String,
    pub sources: usize,
    pub projections: usize,
    pub rank: usize,
}
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Counts {
    pub b_count: u64,
    pub lines: u64,
    pub candidates: u64,
    pub refused: u64,
    pub duplicates: u64,
    pub full_rows: u64,
    pub partial_rows: u64,
    pub one_class_rows: u64,
    pub two_class_rows: u64,
    pub vertices: usize,
    pub collisions: u64,
    pub rank: usize,
    pub products: u64,
    pub normalization_products: u64,
    pub subtractions: u64,
}
pub fn bind_line(hash: &mut Sha256, index: u64, line: &Line) {
    hash.update(index.to_le_bytes());
    for p in [&line.a0, &line.a1] {
        hash.update((p.len() as u64).to_le_bytes());
        for x in p {
            hash.update(x.to_le_bytes());
        }
    }
    hash.update((line.b.len() as u64).to_le_bytes());
    for x in &line.b {
        for y in x.0 {
            hash.update(y.to_le_bytes());
        }
    }
}
fn mul_mod(a: u64, b: u64, p: u64) -> u64 {
    work::hook_mul(p);
    ((a as u128 * b as u128) % p as u128) as u64
}
fn add_mod(a: u64, b: u64, p: u64) -> u64 {
    work::hook_add(p);
    ((a as u128 + b as u128) % p as u128) as u64
}
pub fn signed(sign: i8, p: u64) -> u64 {
    if sign == 1 {
        1
    } else {
        p - 1
    }
}

/// The existing sieve produces every final root counter. Select once in
/// ascending s, retaining the original norm degree/setup and charged arithmetic.
pub fn sieve_line_quadratic_pairs(
    ring: &FpRing,
    config: &TwoLargePrimeSieve,
    line: &Line,
    ctr: &mut Counters,
    shared: Option<&SharedB>,
) -> (Vec<u64>, SieveCost) {
    assert_eq!(config.setup().m, 9);
    let mut counts = Vec::new();
    let (_, cost) = sieve::sieve_line(ring, config.setup(), line, ctr, Some(&mut counts), shared);
    // Counter copying initializes p slots, tests p tags, and writes at most p
    // counts; selection then loads/tests p values. This is a bounded word-event
    // charge, not a complete native allocation/control-flow model.
    work::words(6 * ring.p);
    let hits = counts
        .into_iter()
        .enumerate()
        .skip(1)
        .filter_map(|(s, count)| (count >= 5).then_some(s as u64))
        .collect();
    (hits, cost)
}

type SourceKey = (Vec<(usize, i8)>, Vec<([u64; 6], i8)>);
fn source_key(row: &Certificate) -> SourceKey {
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

/// Expand source coefficients as integers before reducing the small columns.
/// The graph core cannot bypass this final exact cofactor-cancellation check.
fn project(
    sources: &[Source],
    witness: graph_core::Witness,
    activation_source: usize,
    l: u64,
) -> Result<Projection, String> {
    if activation_source >= sources.len() || witness.iter().any(|&(id, _)| id > activation_source) {
        return Err("projection activation/source range".into());
    }
    let mut small = BTreeMap::<usize, i64>::new();
    let mut large = BTreeMap::<[u64; 6], i64>::new();
    for &(id, coefficient) in &witness {
        let row = &sources.get(id).ok_or("witness source range")?.certificate;
        if ![-2, -1, 1, 2].contains(&coefficient) {
            return Err("witness coefficient".into());
        }
        for &(i, sign) in &row.terms {
            *small.entry(i).or_default() += coefficient as i64 * sign as i64;
            work::words(4);
            work::probe(1);
        }
        for r in &row.remainders {
            *large.entry(r.key).or_default() += coefficient as i64 * r.coefficient as i64;
            work::words(4);
            work::probe(6);
        }
    }
    if large.values().any(|&x| x != 0) {
        return Err("nonzero integer cofactor incidence".into());
    }
    let terms = small
        .into_iter()
        .filter_map(|(i, x)| {
            work::words(2);
            let v = (x as i128).rem_euclid(l as i128) as u64;
            (v != 0).then_some((i, v))
        })
        .collect();
    Ok(Projection {
        terms,
        witness,
        rank: 0,
        products: 0,
        activation_source,
    })
}

/// Return the split trace even when later principal admission fails.
pub struct Extraction<'a> {
    pub options: SplitOptions,
    pub rng: &'a mut StdRng,
    pub trace: Option<(Vec<u64>, SplitReport)>,
}
struct Screening<'context, 'ring> {
    context: &'context discriminant::Context<'ring>,
    refusal: Option<discriminant::Screen>,
    performed: bool,
}
pub fn candidate(
    ctx: &Ctx,
    ring: &FpRing,
    config: &TwoLargePrimeSieve,
    line: &Line,
    s: u64,
    extraction: &mut Extraction<'_>,
) -> Result<(Certificate, Vec<graph_core::LargeTerm>), &'static str> {
    candidate_with_screen(ctx, ring, config, line, s, extraction, None)
}
fn candidate_with_screen(
    ctx: &Ctx,
    ring: &FpRing,
    config: &TwoLargePrimeSieve,
    line: &Line,
    s: u64,
    extraction: &mut Extraction<'_>,
    mut screening: Option<&mut Screening<'_, '_>>,
) -> Result<(Certificate, Vec<graph_core::LargeTerm>), &'static str> {
    extraction.trace = None;
    if let Some(state) = screening.as_deref_mut() {
        state.refusal = None;
        state.performed = false;
    }
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
        let is_root = {
            let found = roots.iter().position(|&x| x == b.x);
            // Aggregate the exact number of comparisons performed, including
            // unsuccessful membership tests, once per base abscissa.
            work::words(found.map_or(roots.len(), |index| index + 1) as u64);
            found.is_some()
        };
        if !is_root {
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
        1 => vec![],
        3 => vec![ring.monic(&tail)],
        5 => {
            if let Some(state) = screening {
                state.performed = true;
                let assessment = state.context.screen(&tail)?;
                if assessment.rejects() {
                    state.refusal = Some(assessment);
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

use super::extension_quadratic_pair_dlp::{DiscriminantPairDlpOptions, QuadraticPairDlpOptions};
#[derive(Clone, Copy, Debug, Serialize)]
pub struct DeferredPairDlpOptions {
    pub collection: DiscriminantPairDlpOptions,
    pub max_deferred_edges: usize,
}
impl Default for DeferredPairDlpOptions {
    fn default() -> Self {
        let mut collection = DiscriminantPairDlpOptions::default();
        collection.collection.graph.max_vertices = 262144;
        collection.collection.max_split_records = 524288;
        Self {
            collection,
            max_deferred_edges: 262144,
        }
    }
}
fn charge_extra(before: &ExtraCounts, after: &ExtraCounts, l: u64) {
    work::hook_muls(l, after.products - before.products);
    work::hook_adds(l, after.additions - before.additions);
    work::words(after.word_events - before.word_events);
}
/// Cold full-base collection, checked homogeneous kernel, verified Nagao
/// descents and protected affine target solve. Spec.d is never read.
pub fn run_extension_deferred_pair_dlp_on(
    spec: &Spec,
    seed: u64,
    options: DeferredPairDlpOptions,
) -> Result<(Value, Option<u64>), String> {
    if !(1..=1_000_000).contains(&options.collection.max_screen_refusals) {
        return Err("bounded discriminant refusal history required".into());
    }
    run_quadratic_pair(
        spec,
        seed,
        options.collection.collection,
        options.collection.max_screen_refusals,
        options.max_deferred_edges,
        DescentBackend::Original,
    )
}
/// Same collection and protected affine solve, with an audited five-variable
/// Nagao backend. Every residual attempt is retained, including empty/capped.
#[allow(
    dead_code,
    reason = "Retained public backend is used by sibling comparison binaries; this example includes the shared source directly"
)]
pub fn run_extension_unit_deferred_pair_dlp_on(
    spec: &Spec,
    seed: u64,
    options: DeferredPairDlpOptions,
) -> Result<(Value, Option<u64>), String> {
    if !(1..=1_000_000).contains(&options.collection.max_screen_refusals)
        || options.collection.collection.max_descent_residuals > 8192
    {
        return Err("bounded unit descent/refusal history required".into());
    }
    run_quadratic_pair(
        spec,
        seed,
        options.collection.collection,
        options.collection.max_screen_refusals,
        options.max_deferred_edges,
        DescentBackend::UnitFive,
    )
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum DescentBackend {
    Original,
    #[allow(
        dead_code,
        reason = "Retained backend selected by the sibling adapted comparison entry point"
    )]
    AdaptedSix,
    UnitFive,
}

/// Matched root-materialization adapter control, retaining six coordinates.
#[allow(
    dead_code,
    reason = "Retained public backend is used by sibling comparison binaries; this example includes the shared source directly"
)]
pub fn run_extension_adapted_deferred_pair_dlp_on(
    spec: &Spec,
    seed: u64,
    options: DeferredPairDlpOptions,
) -> Result<(Value, Option<u64>), String> {
    if !(1..=1_000_000).contains(&options.collection.max_screen_refusals) {
        return Err("bounded discriminant refusal history required".into());
    }
    run_quadratic_pair(
        spec,
        seed,
        options.collection.collection,
        options.collection.max_screen_refusals,
        options.max_deferred_edges,
        DescentBackend::AdaptedSix,
    )
}
fn run_quadratic_pair(
    spec: &Spec,
    seed: u64,
    options: QuadraticPairDlpOptions,
    screen_cap: usize,
    max_deferred_edges: usize,
    backend: DescentBackend,
) -> Result<(Value, Option<u64>), String> {
    let max_screen_refusals = Some(screen_cap);
    options.sparse_rank.validate()?;
    let graph_options = deferred::Options {
        max_sources: options.graph.max_sources,
        max_vertices: options.graph.max_vertices,
        max_witness_terms: options.graph.max_witness_terms,
        max_deferred_edges,
    };
    let mut graph = deferred::Graph::new(graph_options)?;
    if !(101..=1511).contains(&spec.p)
        || !(3..1u64 << 63).contains(&spec.l)
        || !is_prime_u64(spec.p)
        || !is_prime_u64(spec.l)
        || options.max_descent_residuals == 0
        || options.max_descent_residuals > 1_000_000
        || !(2..=32).contains(&options.max_accepted_descents)
        || !(1..=16).contains(&options.split.max_attempts)
        || !(1..=1_000_000).contains(&options.max_split_records)
    {
        return Err("invalid extension-remainder DLP bounds".into());
    }
    let mode = "deferred-discriminant-pairs";
    let collection_before = work::snapshot();
    let uniform_before = work::uniform_snapshot();
    let other_before = work::snapshot_other();
    let ctx = {
        let _model = work::enter(Phase::Model);
        Ctx::new(spec)
    };
    let base_phase = work::enter(Phase::FactorBase);
    let mut base_rng = StdRng::seed_from_u64(seed ^ 0xBACE_0001);
    let base = cover::factor_base(&ctx, &mut base_rng);
    let config = TwoLargePrimeSieve::new(&ctx, &base, base.len(), 9)?;
    drop(base_phase);
    let relation_phase = work::enter(Phase::Relations);
    let ring = FpRing::new(spec.p);
    let screen_context = max_screen_refusals
        .map(|_| discriminant::Context::new(&ring))
        .transpose()?;
    let mut ctr = Counters::new(spec.p);
    let mut enum_rng = StdRng::seed_from_u64(seed ^ 0xA17E_2E17);
    let mut split_rng = StdRng::seed_from_u64(seed ^ 0x5155_4144_5041_4952);
    let count = options
        .b_count
        .unwrap_or_else(|| sieve::b_space(config.setup()));
    if count == 0 || count > sieve::b_space(config.setup()) {
        return Err("B count exceeds finite space".into());
    }
    let scramble = sieve::coprime_scramble(sieve::b_space(config.setup()), &mut enum_rng);
    let nu = sieve::nu_lines(config.setup());
    let mut slot = Vec::new();
    let mut hash = Sha256::new();
    let mut counts = Counts::default();
    let mut seen = BTreeSet::new();
    let mut splits = Vec::new();
    let mut screen_refusals = Vec::new();
    let mut screen_calls = 0u64;
    let mut rank = Tracker::new(
        spec.l,
        base.len(),
        seed ^ 0x5A46_4B45_524E_454C,
        options.sparse_rank,
    )?;
    let mut sources: Vec<Source> = Vec::new();
    let mut projections = Vec::new();
    let mut checkpoints = Vec::new();
    let rank_target = base.len().checked_sub(1).ok_or("empty factor base")?;
    let mut stop = "B budget exhausted".to_string();
    'collect: while (counts.b_count as u128) < count {
        let begin = counts.b_count;
        let end = ((begin as u128 + 1024).min(count)) as u64;
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
                counts.lines += 1;
                bind_line(&mut hash, begin + offset as u64, &line);
                let hits =
                    sieve_line_quadratic_pairs(&ring, &config, &line, &mut ctr, shared.as_ref()).0;
                for s in hits {
                    counts.candidates += 1;
                    if splits.len() >= options.max_split_records {
                        stop = "split trace resource cap".into();
                        counts.b_count = begin + offset as u64;
                        break 'collect;
                    }
                    if max_screen_refusals.is_some_and(|cap| screen_refusals.len() >= cap) {
                        stop = "discriminant refusal resource cap".into();
                        counts.b_count = begin + offset as u64;
                        break 'collect;
                    }
                    let mut extraction = Extraction {
                        options: options.split,
                        rng: &mut split_rng,
                        trace: None,
                    };
                    let mut screening = screen_context.as_ref().map(|context| Screening {
                        context,
                        refusal: None,
                        performed: false,
                    });
                    let certificate = if let Some(state) = screening.as_mut() {
                        candidate_with_screen(
                            &ctx,
                            &ring,
                            &config,
                            &line,
                            s,
                            &mut extraction,
                            Some(state),
                        )
                        .ok()
                    } else {
                        candidate(&ctx, &ring, &config, &line, s, &mut extraction).ok()
                    };
                    if let Some(state) = screening {
                        screen_calls += u64::from(state.performed);
                        if let Some(assessment) = state.refusal {
                            screen_refusals.push(ScreenRefusal {
                                candidate_index: counts.candidates,
                                assessment,
                            });
                        }
                    }
                    if let Some((tail, report)) = extraction.trace {
                        splits.push(SplitRecord {
                            candidate_index: counts.candidates,
                            b_index: begin + offset as u64,
                            a0: line.a0.clone(),
                            a1: line.a1.clone(),
                            b: remainder::encode(&line.b),
                            s,
                            tail,
                            report,
                        });
                    }
                    let Some((certificate, large)) = certificate else {
                        counts.refused += 1;
                        continue;
                    };
                    if sources.len() >= options.graph.max_sources {
                        stop = "source resource cap".into();
                        counts.b_count = begin + offset as u64;
                        break 'collect;
                    }
                    if !seen.insert(source_key(&certificate)) {
                        counts.duplicates += 1;
                        continue;
                    }
                    if !certificate.remainders.is_empty() {
                        counts.partial_rows += 1;
                        if certificate.remainders.len() == 1 {
                            counts.one_class_rows += 1;
                        } else {
                            counts.two_class_rows += 1;
                        }
                    } else {
                        counts.full_rows += 1;
                    }
                    let activation_source = sources.len();
                    let outcome = graph.accept(activation_source, &large)?;
                    sources.push(Source {
                        candidate_index: counts.candidates,
                        b_index: begin + offset as u64,
                        a0: line.a0.clone(),
                        a1: line.a1.clone(),
                        b: remainder::encode(&line.b),
                        s,
                        certificate,
                        graph: outcome.clone(),
                    });
                    for event in outcome {
                        if let deferred::Event::Full { witness, .. } = event {
                            if witness.len() > 1 {
                                counts.collisions += 1;
                            }
                            // Rank is admitted only by a complete-batch sparse solve.
                            projections.push(project(
                                &sources,
                                witness,
                                activation_source,
                                spec.l,
                            )?);
                        }
                    }
                }
            }
        }
        counts.b_count = end;
        if rank.due(projections.len()) {
            let _la = work::enter(Phase::LinearAlgebra);
            let rows = projections.iter().map(|p| p.terms.clone()).collect();
            if end as u128 == count {
                rank.check_at_budget_end(end, rows)?;
            } else {
                rank.check(end, rows)?;
            }
        }
        checkpoints.push(Checkpoint {
            b_count: end,
            lines: counts.lines,
            cohort_sha256: format!("{:x}", hash.clone().finalize()),
            sources: sources.len(),
            projections: projections.len(),
            rank: rank.rank_lower_bound(),
        });
        if rank.kernel.is_some() {
            stop = "sparse homogeneous rank certified at complete batch".into();
            break;
        }
        if rank.exhausted() {
            stop = "sparse kernel checkpoint cap".into();
            break;
        }
    }
    counts.vertices = graph.stats().vertices;
    counts.rank = rank.rank_lower_bound();
    drop(relation_phase);
    let collection_tally = work::snapshot().since(&collection_before);
    let widths = work::widths(spec.p, spec.l);
    let phases:Vec<_>=Phase::IC.iter().map(|&p|json!({"phase":p.name(),"primitives":work::prims_json(collection_tally.phase(p)),"diagnostic_priced_bitops":work::price(collection_tally.phase(p),&widths).expect("registered widths").to_string()})).collect();
    let uniforms = work::uniform_snapshot()
        .since(&uniform_before)
        .ok_or("uniform regression")?;
    let other: Vec<_> = work::snapshot_other()
        .iter()
        .zip(other_before)
        .map(|(&a, b)| a.checked_sub(b))
        .collect();
    let collection_native = json!({"complete":false,"total_bitops":null,"comparative_eligibility":false,"ratio":null,"band":null,
        "phases":phases,"observed_primitives":work::prims_json(&collection_tally.sum(&Phase::IC)),"uniform_sampling":work::uniform_json(&uniforms.sum(&Phase::IC).ok_or("uniform overflow")?),
        "foreign_hook_calls":work::foreign(),"other_modulus_ops":other,"known_uncounted":["native allocations/containers/scans/normalization and control flow","PRNG internals and incompletely hooked sampling","sparse preconditioner setup/divisions and recurrence control","cohort hashing/serialization"],
        "excluded":"caller public domain validation and curve transfer; charged separately by full worker"});
    let ready = rank.kernel.is_some();
    let mut basis = if ready {
        let _la = work::enter(Phase::LinearAlgebra);
        let q = Quotient::from_rank_certified_kernel(
            spec.l,
            rank.kernel.clone().ok_or("missing certified kernel")?,
            rank_target,
        )?;
        charge_extra(&ExtraCounts::default(), &q.counts, spec.l);
        Some(q)
    } else {
        None
    };
    let initial_basis = basis
        .as_ref()
        .map(|q| serde_json::to_value(q).expect("quotient serialization"));
    let jac = ctx.jac();
    let by_x: HashMap<_, _> = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
    let mut descents = Vec::new();
    let mut nagao_attempts = Vec::new();
    let mut affine_rows = Vec::new();
    let mut projected_rows = Vec::new();
    let mut affine = CountedRelationSystem::new(spec.l, 2);
    let (mut residuals, mut advances, mut timeouts, mut incomplete, mut invalid) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut scalar = None;
    let mut refusal = if ready {
        None
    } else {
        Some("homogeneous collection not ready")
    };
    if let Some(quotient) = basis.as_mut() {
        let _descent = work::enter(Phase::Descent);
        let mut stream = StdRng::seed_from_u64(seed ^ 0x51E7E);
        let al = work::uniform_mod_u64(&mut stream, 1, spec.l);
        let be = work::uniform_mod_u64(&mut stream, 1, spec.l);
        let step = jac.add(
            &jac.mul(&spec.gj, al as u128),
            &jac.mul(&spec.qj, be as u128),
        );
        let mut a = work::uniform_mod_u64(&mut stream, 0, spec.l);
        let mut b = work::uniform_mod_u64(&mut stream, 1, spec.l);
        let mut residual = jac.add(&jac.mul(&spec.gj, a as u128), &jac.mul(&spec.qj, b as u128));
        let opts = cover::opts_for_mode(24, 600.0, cover::StopMode::Staircase(64));
        for i in 0..options.max_descent_residuals {
            let mut rng = StdRng::seed_from_u64(seed ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let (decompositions, cost) = if backend != DescentBackend::Original {
                let attempt = nagao_unit::decompose(
                    &ctx,
                    &base,
                    &by_x,
                    &residual,
                    &opts,
                    &mut rng,
                    backend == DescentBackend::UnitFive,
                );
                let (rows, cost, audit) = match attempt {
                    Ok(value) => value,
                    Err(error) => {
                        let mut v = residual.v.clone();
                        v.resize(3, cover::E2::ZERO);
                        let input = cover::weil_forms(&ctx.f.f, &ctx.cov.hx, &residual.u, &v)
                            .polys(&ctx.reim, spec.p);
                        nagao_attempts.push(json!({"residual_index":i,"u":remainder::encode(&residual.u),"v":remainder::encode(&residual.v),"original":input,"refusal":error}));
                        residuals += 1;
                        refusal = Some(error);
                        break;
                    }
                };
                if backend == DescentBackend::UnitFive {
                    nagao_attempts.push(json!({"residual_index":i,"u":remainder::encode(&residual.u),"v":remainder::encode(&residual.v),"audit":audit}));
                }
                (rows, cost)
            } else {
                cover::nagao_decompose(&ctx, &base, &by_x, &residual, &opts, &mut rng)
            };
            residuals += 1;
            timeouts += u64::from(cost.timed_out);
            incomplete += u64::from(cost.incomplete);
            for mut decomposition in decompositions {
                decomposition.terms.sort_unstable();
                if config
                    .verified_descent(&ctx, spec, a, b, decomposition)
                    .is_none()
                {
                    invalid += 1;
                    continue;
                }
                descents.push(json!({"residual_index":i,"a":a,"b":b,"decomposition":decomposition,"verified":true}));
                let _la = work::enter(Phase::LinearAlgebra);
                let mut dense = vec![0; base.len() + 1];
                work::words(dense.len() as u64);
                for &(index, sign) in &decomposition.terms {
                    dense[index] = signed(sign, spec.l);
                    work::words(2);
                }
                dense[base.len()] = if b == 0 { 0 } else { spec.l - b };
                work::hook_add(spec.l);
                work::words(1);
                let before = quotient.counts.clone();
                let (projected, rhs) = quotient.project(&dense, a)?;
                charge_extra(&before, &quotient.counts, spec.l);
                affine_rows.push(json!({"coefficients":dense,"rhs":a}));
                work::words(2);
                affine.insert(projected.to_vec(), rhs);
                let candidate = affine.solved(1);
                projected_rows.push(json!({"coefficients":projected,"rhs":rhs,"rank":affine.rank(),"products":affine.ops(),"normalization_products":affine.normalization_products(),"subtractions":affine.subtraction_ops(),"candidate_scalar":candidate,"projection_extra_counts":ExtraCounts{products:quotient.counts.products-before.products,additions:quotient.counts.additions-before.additions,word_events:quotient.counts.word_events-before.word_events}}));
                if let Some(d) = candidate {
                    let _verification = work::enter(Phase::Verification);
                    let ec = EllE::new(&ctx.f, &spec.alpha);
                    if d > 0 && d < spec.l && ec.mul(&spec.g, d) == spec.q {
                        scalar = Some(d);
                    } else {
                        refusal = Some("candidate elliptic replay failed");
                    }
                }
                break;
            }
            if scalar.is_some() || refusal.is_some() {
                break;
            }
            if descents.len() >= options.max_accepted_descents {
                refusal = Some("accepted descent cap");
                break;
            }
            residual = jac.add(&residual, &step);
            a = add_mod(a, al, spec.l);
            b = add_mod(b, be, spec.l);
            advances += 1;
        }
    }
    let mut collection = json!({"schema":"cover.deferred-pair.collection/v1","mode":mode,"seed":seed,"p":spec.p,"l":spec.l,
        "base":points(&base),"h":remainder::encode(&ctx.cov.hx),"sources":sources,"projections":projections,"checkpoints":checkpoints,"counts":counts,"rank_target":rank_target,"stop":stop,"work_native":collection_native,
        "rank_checks":rank.checkpoints,"rank_options":options.sparse_rank,"rank_semantics":"certified lower bound; zero before success; no incremental RREF products",
        "graph_stats":graph.stats(),"graph_options":options.graph,"deferred_graph_options":graph_options,"split_options":options.split,"max_split_records":options.max_split_records,
        "splits":splits,"split_seed_xor":0x5155_4144_5041_4952u64,"root_finder":"forward-differences","sieve_selection":"counter-table-five-ascending"});
    if let Some(cap) = max_screen_refusals {
        collection["discriminant_screen"] = json!({"method":"euclidean-discriminant-character","max_refusals":cap,"calls":screen_calls,"refusals":screen_refusals});
    }
    let mut report = json!({"schema":"cover.deferred-pair.dlp/v1","p":spec.p,"l":spec.l,"seed":seed,"options":options,"max_deferred_edges":max_deferred_edges,"max_screen_refusals":screen_cap,
        "collection":collection,"collection_rank_ready":ready,"initial_basis":initial_basis,"final_extra_counts":basis.as_ref().map(|q|&q.counts),
        "descents":descents,"affine_rows":affine_rows,"projected_rows":projected_rows,"descent_residuals":residuals,"descent_stream_advances":advances,
        "descent_timeouts":timeouts,"descent_incomplete":incomplete,"invalid_descents":invalid,"last_rank":affine.rank(),"solver_modular_muls":affine.ops(),
        "actual_scalar":scalar,"verified":scalar.is_some(),"capped":scalar.is_none(),"resource_refusal":refusal,"fallback_solves":0,
        "rank_scope":"kernel-scale and protected-target coordinates only; full-base homogeneous rank is in collection",
        "solver_product_scope":"two-column affine RREF only; collection/kernel/projection work recorded separately",
        "complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"known_uncounted":["native containers/control/normalization/scans","PRNG internals","fixed-factor and Euclidean internals","hashing/serialization"]});
    if backend != DescentBackend::Original {
        report["nagao_backend"] = json!(if backend == DescentBackend::UnitFive {
            "exact-unit-five"
        } else {
            "adapted-six"
        });
        report["nagao_attempts"] = json!(nagao_attempts);
    }
    Ok((report, scalar))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn source(terms: Vec<(usize, i8)>, large: i8) -> Source {
        Source {
            candidate_index: 1,
            b_index: 0,
            a0: vec![],
            a1: vec![],
            b: vec![],
            s: 1,
            certificate: Certificate {
                terms,
                remainders: vec![remainder::Remainder {
                    key: [3, 4, 5, 6, 7, 8],
                    coefficient: large,
                }],
                a: vec![],
                c: vec![],
                norm: vec![],
            },
            graph: vec![],
        }
    }
    #[test]
    fn exact_projection_checks_causal_binding_even_when_anchor_cancels() {
        let sources = vec![
            source(vec![(0, 1), (1, -1)], -1),
            source(vec![(0, 1), (2, 1)], 1),
            source(vec![], 1),
        ];
        let row = project(&sources, vec![(0, -2), (1, -2)], 2, 101).unwrap();
        assert_eq!(row.activation_source, 2);
        assert_eq!(row.terms, vec![(0, 97), (1, 2), (2, 99)]);
        assert!(project(&sources, vec![(0, 1), (1, -1)], 2, 101).is_err());
        assert!(project(&sources, vec![(0, -2), (1, -2)], 0, 101).is_err());
        assert!(project(&sources, vec![(0, -2), (1, -2)], 3, 101).is_err());
    }
    #[test]
    fn invalid_edge_screen_and_collection_caps_fail_before_collection() {
        let spec = cover::generate_spec(101, 17);
        for max_deferred_edges in [0, 1_000_001] {
            assert!(run_extension_deferred_pair_dlp_on(
                &spec,
                389,
                DeferredPairDlpOptions {
                    max_deferred_edges,
                    ..Default::default()
                }
            )
            .is_err());
        }
        for max_screen_refusals in [0, 1_000_001] {
            let mut o = DeferredPairDlpOptions::default();
            o.collection.max_screen_refusals = max_screen_refusals;
            assert!(run_extension_deferred_pair_dlp_on(&spec, 389, o).is_err());
        }
        let mut o = DeferredPairDlpOptions::default();
        o.collection.collection.max_descent_residuals = 0;
        assert!(run_extension_deferred_pair_dlp_on(&spec, 389, o).is_err());
    }
}
