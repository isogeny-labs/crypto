//! Affine descent and a protected target column for adapted two-LP IC.
use super::{
    filter_and_replay, CountedRelationSystem, FilteredTwoLargePrimeRelations,
    TwoLargePrimeRelation, TwoLargePrimeSieve,
};
use crate::cryptanalysis::gaudry_cubic::am;
use crate::cryptanalysis::gaudry_cubic::sm;
use crate::cryptanalysis::jv_bitops::{self as work, Phase};
use crate::cryptanalysis::jv_cover::{factor_base, nagao_decompose, opts_for, EllE};
use crate::cryptanalysis::jv_cover::{verify_dec, Ctx, Dec6, Spec};
use crate::cryptanalysis::jv_sieve::{
    b_from_index, b_space, coprime_scramble, lines_for_b_hint, nu_lines, roots_by_incidence,
    shared_for_b, Counters, FpRing,
};
use crate::cryptanalysis::large_prime_filter::{LargePrimeFilterOptions, PartialRelation};
use rand::{rngs::StdRng, SeedableRng};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashSet;

#[path = "jv_large_prime_rank_bound.rs"]
mod structural_rank_bound;
use structural_rank_bound::{BoundSnapshot, StructuralRankBound};

use super::homogeneous_quotient::{ExtraCounts, Quotient};

fn charge_quotient_delta(before: &ExtraCounts, after: &ExtraCounts, modulus: u64) {
    work::hook_muls(modulus, after.products - before.products);
    work::hook_adds(modulus, after.additions - before.additions);
    work::words(after.word_events - before.word_events);
}

fn structural_snapshot(s: &BoundSnapshot) -> Value {
    json!({"sources":s.sources,"vertices":s.vertices,"components":s.components,
        "anchored_components":s.anchored_components,"large_rank_lower_bound":s.large_rank_lower_bound,
        "small_rank_upper_bound":s.small_rank_upper_bound,"find_calls":s.find_calls,
        "parent_probes":s.parent_probes,"successful_unions":s.successful_unions})
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct TwoLargePrimeDlpOptions {
    pub small_percent: usize,
    /// None enumerates the whole finite degree-nine B space once.
    pub b_count: Option<u128>,
    /// Opt-in certified homogeneous-rank checks after this many new sources.
    /// Checks occur only at complete B-batch boundaries. None keeps the
    /// original fixed-cohort collector and its RNG consumption unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_rank_stride: Option<usize>,
    pub max_descent_residuals: u64,
    pub max_accepted_descents: usize,
}
impl Default for TwoLargePrimeDlpOptions {
    fn default() -> Self {
        Self {
            small_percent: 75,
            b_count: None,
            collection_rank_stride: None,
            max_descent_residuals: 8192,
            max_accepted_descents: 32,
        }
    }
}

/// Exact rank of replayed homogeneous rows. Row count alone is insufficient:
/// dependent rows must not trigger the collection readiness heuristic.
fn homogeneous_rank(
    projection: &FilteredTwoLargePrimeRelations,
    modulus: u64,
    columns: usize,
) -> Result<(usize, u64), &'static str> {
    let system = homogeneous_system(projection, modulus, columns)?;
    Ok((system.rank(), system.ops()))
}

fn homogeneous_system(
    projection: &FilteredTwoLargePrimeRelations,
    modulus: u64,
    columns: usize,
) -> Result<CountedRelationSystem, &'static str> {
    let mut system = CountedRelationSystem::new(modulus, columns);
    for row in &projection.rows {
        if row.rhs != 0 || row.cols.iter().any(|&(c, v)| c >= columns || v >= modulus) {
            return Err("invalid homogeneous rank row");
        }
        let mut dense = vec![0; columns];
        work::words(columns as u64);
        for &(c, v) in &row.cols {
            dense[c] = v;
            work::words(2);
        }
        system.insert(dense, 0);
    }
    Ok(system)
}

fn bind_line(h: &mut blake3::Hasher, k: u128, line: &crate::cryptanalysis::jv_sieve::Line) {
    h.update(&k.to_le_bytes());
    for p in [&line.a0, &line.a1, &line.p1, &line.p2, &line.p3, &line.p4] {
        h.update(&(p.len() as u64).to_le_bytes());
        for c in p {
            h.update(&c.to_le_bytes());
        }
    }
    h.update(&(line.b.len() as u64).to_le_bytes());
    for c in &line.b {
        for v in c.0 {
            h.update(&v.to_le_bytes());
        }
    }
}

/// Actual scalar recovery from a PUBLIC transferred Spec (d is never read).
/// Cold transfer itself belongs to the caller's Model phase. All original
/// rows and accepted descents are retained, including unsuccessful attempts.
/// This bounded degree-nine implementation uses incremental modular RREF;
/// it does not call the original full-smooth solve as a fallback.
pub fn run_two_large_prime_dlp_on(
    spec: &Spec,
    seed: u64,
    options: TwoLargePrimeDlpOptions,
) -> Result<(Value, Option<u64>), String> {
    run_two_large_prime_dlp_impl(spec, seed, options, false, false)
}

/// Optional graph bound defers exact rank checkpoints until enough
/// small-only equations are structurally possible. Exact rank, affine
/// target determination and elliptic verification are still required.
pub fn run_two_large_prime_dlp_on_with_structural_gate(
    spec: &Spec,
    seed: u64,
    options: TwoLargePrimeDlpOptions,
) -> Result<(Value, Option<u64>), String> {
    if options.collection_rank_stride.is_none() {
        return Err("structural gate requires exact rank checkpoints".into());
    }
    run_two_large_prime_dlp_impl(spec, seed, options, true, false)
}

/// Retain a ready homogeneous RREF and solve affine rows in its quotient.
/// This bounded option requires a checked codimension-one homogeneous basis.
pub fn run_two_large_prime_dlp_on_with_quotient_reuse(
    spec: &Spec,
    seed: u64,
    options: TwoLargePrimeDlpOptions,
) -> Result<(Value, Option<u64>), String> {
    if options.collection_rank_stride.is_none() {
        return Err("quotient reuse requires exact rank checkpoints".into());
    }
    run_two_large_prime_dlp_impl(spec, seed, options, true, true)
}

fn run_two_large_prime_dlp_impl(
    spec: &Spec,
    seed: u64,
    options: TwoLargePrimeDlpOptions,
    structural_gate: bool,
    quotient_reuse: bool,
) -> Result<(Value, Option<u64>), String> {
    if !(101..=1511).contains(&spec.p)
        || !(3..1u64 << 63).contains(&spec.l)
        || !crate::cryptanalysis::residual_walk::is_prime_u64(spec.p)
        || !crate::cryptanalysis::residual_walk::is_prime_u64(spec.l)
        || !(1..100).contains(&options.small_percent)
        || options.max_descent_residuals == 0
        || options.max_descent_residuals > 1_000_000
        || !(2..=32).contains(&options.max_accepted_descents)
        || options
            .collection_rank_stride
            .is_some_and(|s| !(1..=4096).contains(&s))
    {
        return Err("invalid bounded two-LP DLP options".into());
    }
    let ctx = {
        let _g = work::enter(Phase::Model);
        Ctx::new(spec)
    };
    let mut stream_rng = StdRng::seed_from_u64(seed ^ 0x51E7E);
    let sieve = {
        let _g = work::enter(Phase::FactorBase);
        let base = factor_base(&ctx, &mut stream_rng);
        let small = (base.len() * options.small_percent / 100).max(1);
        TwoLargePrimeSieve::new(&ctx, &base, small, 9)?
    };
    let count = options.b_count.unwrap_or_else(|| b_space(sieve.setup()));
    if count == 0 || count > b_space(sieve.setup()) {
        return Err("B count exceeds unique finite space".into());
    }
    let mut enum_rng = StdRng::seed_from_u64(seed ^ 0xA17E_2E17);
    let ring = FpRing::new(spec.p);
    let mut ctr = Counters::new(spec.p);
    let mut relations = Vec::new();
    let mut seen = HashSet::new();
    let mut fingerprint = blake3::Hasher::new();
    let (mut bs, mut lines, mut candidates, mut refused, mut duplicates) =
        (0u128, 0u64, 0u64, 0u64, 0u64);
    let mut histogram = [0u64; 3];
    let mut refusal: Option<&str> = None;
    let rank_target = sieve.small.saturating_sub(1);
    let mut next_rank_check = rank_target.max(1);
    let mut rank_checks = Vec::new();
    let mut rank_ready = false;
    let mut retained_quotient: Option<Quotient> = None;
    let mut initial_quotient = Value::Null;
    let mut quotient_attempts = Vec::new();
    let mut gate_words = 0;
    let mut gate = if structural_gate {
        let _la = work::enter(Phase::LinearAlgebra);
        // Three initialized dense state arrays; other native allocation and
        // scheduling work remains in the explicit incomplete-coverage gap.
        gate_words = 3 * spec.p;
        work::words(gate_words);
        Some(StructuralRankBound::new(spec.p as usize, spec.l)?)
    } else {
        None
    };
    let mut gate_checks = Vec::new();
    {
        let _g = work::enter(Phase::Relations);
        let scramble = coprime_scramble(b_space(sieve.setup()), &mut enum_rng);
        let nu = nu_lines(sieve.setup());
        let mut slot = Vec::new();
        'collect: while bs < count {
            let end = (bs + 1024).min(count);
            let batch: Vec<_> = (bs..end)
                .map(|k| b_from_index(sieve.setup(), k, scramble))
                .collect();
            let ab: Vec<_> = batch.iter().map(|b| (b[0].0[0], b[0].0[1])).collect();
            let hints = roots_by_incidence(spec.p, &nu, &ab, &mut slot).0;
            for (offset, b) in batch.iter().enumerate() {
                let found = lines_for_b_hint(
                    &ring,
                    &ctx.f.f,
                    sieve.setup(),
                    b,
                    &mut enum_rng,
                    true,
                    hints[offset].as_deref(),
                );
                let shared = if found.len() >= 2 {
                    Some(shared_for_b(&ring, &found[0], &mut 0))
                } else {
                    None
                };
                for line in found {
                    lines += 1;
                    bind_line(&mut fingerprint, bs + offset as u128, &line);
                    for s in sieve.sieve_line(&ring, &line, &mut ctr, shared.as_ref()).0 {
                        candidates += 1;
                        let Some(r) = sieve.verified_relation(
                            &ctx,
                            &ring,
                            &line,
                            s,
                            relations.len() as u64,
                            spec.l,
                        ) else {
                            refused += 1;
                            continue;
                        };
                        let neg: Vec<_> = r.full().terms.iter().map(|&(i, e)| (i, -e)).collect();
                        let key_words = r.full().terms.len() as u64;
                        work::probe(key_words);
                        let duplicate = seen.contains(&r.full().terms) || {
                            work::probe(key_words);
                            seen.contains(&neg)
                        };
                        if duplicate {
                            duplicates += 1;
                            continue;
                        }
                        if relations.len() >= 4096 - options.max_accepted_descents {
                            refused += 1;
                            bs += offset as u128;
                            refusal = Some("relation resource cap");
                            break 'collect;
                        }
                        work::probe(key_words);
                        seen.insert(r.full().terms.clone());
                        histogram[r.partial().large.len()] += 1;
                        if let Some(gate) = gate.as_mut() {
                            let _la = work::enter(Phase::LinearAlgebra);
                            let probes_before = gate.snapshot().parent_probes;
                            gate.add_row(r.partial().rhs, &r.partial().large)?;
                            let probes = gate.snapshot().parent_probes - probes_before;
                            // Known lower bound: read each support identity and
                            // coefficient, and every parent probe actually made.
                            // Do not call this a complete native bookkeeping price.
                            let words = 2 * r.partial().large.len() as u64 + probes;
                            work::words(words);
                            gate_words += words;
                        }
                        relations.push(r);
                    }
                }
            }
            bs = end;
            eprintln!(
                "[two-lp collection] B={bs}/{count} lines={lines} verified={}",
                relations.len()
            );
            if let Some(stride) = options.collection_rank_stride {
                if relations.len() >= next_rank_check {
                    let _la = work::enter(Phase::LinearAlgebra);
                    if let Some(gate) = gate.as_ref() {
                        let state = gate.snapshot();
                        work::words(1);
                        gate_words += 1;
                        let permits_exact_check = state.small_rank_upper_bound >= rank_target;
                        gate_checks.push(json!({"b_count":bs,"source_rows":relations.len(),"rank_target":rank_target,
                            "permits_exact_check":permits_exact_check,"state":structural_snapshot(&state)}));
                        if !permits_exact_check {
                            // Keep the baseline's source-stride schedule. This
                            // skips a doomed checkpoint without changing when
                            // later checks consume the original relation stream.
                            next_rank_check = relations.len().saturating_add(stride);
                            continue 'collect;
                        }
                    }
                    let before = work::snapshot();
                    let projection = match sieve.filter_checked(
                        &ctx,
                        &relations,
                        &LargePrimeFilterOptions {
                            modulus: spec.l,
                            ..Default::default()
                        },
                    ) {
                        Ok(p) => p,
                        Err(error) => {
                            rank_checks.push(json!({"b_count":bs,"source_rows":relations.len(),"projection_error":error,"observed_primitives":work::prims_json(&work::snapshot().since(&before).total())}));
                            refusal = Some(error);
                            break 'collect;
                        }
                    };
                    let (rank, products) = if quotient_reuse {
                        let system = homogeneous_system(&projection, spec.l, sieve.small)?;
                        if system.rank() >= rank_target {
                            let quotient =
                                Quotient::from_rref(spec.l, sieve.small, system.pivot_rows())?;
                            charge_quotient_delta(
                                &ExtraCounts::default(),
                                &quotient.counts,
                                spec.l,
                            );
                            initial_quotient =
                                serde_json::to_value(&quotient).map_err(|e| e.to_string())?;
                            retained_quotient = Some(quotient);
                        }
                        (system.rank(), system.ops())
                    } else {
                        homogeneous_rank(&projection, spec.l, sieve.small)?
                    };
                    rank_ready = rank >= rank_target;
                    rank_checks.push(json!({"b_count":bs,"lines":lines,"cohort_digest":fingerprint.clone().finalize().to_hex().to_string(),"source_rows":relations.len(),"complete_rows":projection.rows.len(),"rank":rank,"rank_target":rank_target,"ready":rank_ready,"solver_modular_muls":products,"graph":projection.report,"replayed_rows":projection.replayed_rows,"certificates":projection.certificates,"observed_primitives":work::prims_json(&work::snapshot().since(&before).total())}));
                    next_rank_check = relations.len().saturating_add(stride);
                    eprintln!(
                        "[two-lp rank] sources={} rank={rank}/{rank_target} ready={rank_ready}",
                        relations.len()
                    );
                    if rank_ready {
                        break 'collect;
                    }
                }
            }
        }
    }
    if quotient_reuse && retained_quotient.is_none() {
        return Err("collection did not produce a reusable codimension-one basis".into());
    }
    let by_x = &sieve.by_x;
    let jac = ctx.jac();
    let mut descents = Vec::new();
    let mut attempts = Vec::new();
    let mut last_projection = None;
    let (mut descent_residuals, mut descent_timeouts, mut descent_incomplete, mut invalid_descents) =
        (0u64, 0u64, 0u64, 0u64);
    let mut stream_advances = 0u64;
    let (mut scalar, mut last_rank, mut solve_ops) = (None, 0usize, 0u64);
    if refusal.is_none() {
        let _g = work::enter(Phase::Descent);
        let al = work::uniform_mod_u64(&mut stream_rng, 1, spec.l);
        let be = work::uniform_mod_u64(&mut stream_rng, 1, spec.l);
        let step = jac.add(
            &jac.mul(&spec.gj, al as u128),
            &jac.mul(&spec.qj, be as u128),
        );
        let mut a = work::uniform_mod_u64(&mut stream_rng, 0, spec.l);
        let mut b = work::uniform_mod_u64(&mut stream_rng, 1, spec.l);
        let mut residual = jac.add(&jac.mul(&spec.gj, a as u128), &jac.mul(&spec.qj, b as u128));
        let opts = opts_for(24, 600.0, Some(64));
        for i in 0..options.max_descent_residuals {
            let mut rng = StdRng::seed_from_u64(seed ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let (decompositions, cost) =
                nagao_decompose(&ctx, &sieve.base, by_x, &residual, &opts, &mut rng);
            descent_residuals += 1;
            descent_timeouts += cost.timed_out as u64;
            descent_incomplete += cost.incomplete as u64;
            let mut added = false;
            for d in decompositions {
                if let Some(checked) = sieve.verified_descent(&ctx, spec, a, b, d) {
                    descents.push(checked);
                    added = true;
                    break;
                } else {
                    invalid_descents += 1;
                }
            }
            if added && descents.len() >= 2 {
                let _la = work::enter(Phase::LinearAlgebra);
                let before = work::snapshot();
                match sieve.filter_with_descent(
                    &ctx,
                    spec,
                    &relations,
                    &descents,
                    &LargePrimeFilterOptions {
                        modulus: spec.l,
                        ..Default::default()
                    },
                ) {
                    Err(error) => {
                        attempts.push(
                            json!({"accepted_descents":descents.len(),"projection_error":error,"observed_primitives":work::prims_json(&work::snapshot().since(&before).total())}),
                        );
                        refusal = Some(error);
                        break;
                    }
                    Ok(projection) => {
                        let columns = sieve.small + 1;
                        let solver_columns = if quotient_reuse { 2 } else { columns };
                        let solver_target = if quotient_reuse {
                            1
                        } else {
                            projection.target_column
                        };
                        let mut system = CountedRelationSystem::new(spec.l, solver_columns);
                        let mut projected_rows = Vec::new();
                        let projection_before =
                            retained_quotient.as_ref().map(|q| q.counts.clone());
                        for (row_index, row) in projection.projection.rows.iter().enumerate() {
                            let mut dense = vec![0u64; columns];
                            work::words(columns as u64);
                            for &(c, v) in &row.cols {
                                dense[c] = v;
                                work::words(2);
                            }
                            if let Some(quotient) = retained_quotient.as_mut() {
                                let before = quotient.counts.clone();
                                let (small, rhs) = quotient.project(&dense, row.rhs)?;
                                charge_quotient_delta(&before, &quotient.counts, spec.l);
                                projected_rows
                                    .push(json!({"row":row_index,"coefficients":small,"rhs":rhs}));
                                system.insert(small.to_vec(), rhs);
                            } else {
                                system.insert(dense, row.rhs);
                            }
                            if system.solved(solver_target).is_some() {
                                break;
                            }
                        }
                        last_rank = system.rank();
                        solve_ops = solve_ops
                            .checked_add(system.ops())
                            .ok_or("solver op overflow")?;
                        let candidate = system.solved(solver_target);
                        let verified = {
                            let _v = work::enter(Phase::Verification);
                            let ec = EllE::new(&ctx.f, &spec.alpha);
                            candidate.is_some_and(|d| {
                                d > 0 && d < spec.l && ec.mul(&spec.g, d) == spec.q
                            })
                        };
                        attempts.push(json!({"accepted_descents":descents.len(),"input_rows":projection.inputs.len(),"complete_rows":projection.projection.rows.len(),"nonzeros":projection.projection.rows.iter().map(|r|r.cols.len()).sum::<usize>(),"graph":projection.projection.report,"replayed_rows":projection.projection.replayed_rows,"rank":system.rank(),"solver_modular_muls":system.ops(),"candidate_scalar":candidate,"verified":verified,"observed_primitives":work::prims_json(&work::snapshot().since(&before).total())}));
                        if let (Some(quotient), Some(before)) =
                            (retained_quotient.as_ref(), projection_before)
                        {
                            let extra = ExtraCounts {
                                products: quotient.counts.products - before.products,
                                additions: quotient.counts.additions - before.additions,
                                word_events: quotient.counts.word_events - before.word_events,
                            };
                            quotient_attempts.push(json!({"accepted_descents":descents.len(),"solver_columns":solver_columns,
                                "target_column":solver_target,"rank":system.rank(),"solver_modular_muls":system.ops(),
                                "normalization_products":system.normalization_products(),"subtraction_ops":system.subtraction_ops(),
                                "projected_rows":projected_rows,"extra_counts":extra,"actual_scalar":candidate}));
                        }
                        last_projection = Some(projection);
                        if verified {
                            scalar = candidate;
                            break;
                        }
                        if candidate.is_some() {
                            refusal = Some("candidate elliptic replay failed");
                            break;
                        }
                    }
                }
            }
            if descents.len() >= options.max_accepted_descents {
                refusal = Some("accepted descent cap");
                break;
            }
            residual = jac.add(&residual, &step);
            a = am(a, al, spec.l);
            b = am(b, be, spec.l);
            stream_advances += 1;
            if descent_residuals.is_multiple_of(64) {
                eprintln!(
                    "[two-lp descent] residuals={descent_residuals} accepted={}",
                    descents.len()
                );
            }
        }
    }
    let source_rows: Vec<_> = relations
        .iter()
        .map(|r| json!({"full":r.full(),"partial":r.partial()}))
        .collect();
    let proof=last_projection.as_ref().map(|p|json!({"target_column":p.target_column,"inputs":p.inputs,"certificates":p.projection.certificates,"report":p.projection.report,"replayed_rows":p.projection.replayed_rows}));
    let report = json!({"schema":"cover.two-large-prime.dlp/v1","p":spec.p,"l":spec.l,"seed":seed,"options":options,"m":9,"full_base":sieve.base.len(),"small_base":sieve.small,"b_count_planned":count,"b_count_completed":bs,"lines":lines,"cohort_digest":fingerprint.finalize().to_hex().to_string(),"candidates":candidates,"refused":refused,"duplicates":duplicates,"verified_source_rows":relations.len(),"by_large_count":histogram,"source_relations":source_rows,"descents":descents,"descent_residuals":descent_residuals,"descent_timeouts":descent_timeouts,"descent_incomplete":descent_incomplete,"invalid_descents":invalid_descents,"projection_solve_attempts":attempts,"final_projection":proof,"last_rank":last_rank,"solver_modular_muls":solve_ops,"actual_scalar":scalar,"verified":scalar.is_some(),"capped":scalar.is_none(),"resource_refusal":refusal,"fallback_solves":0,"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"known_uncounted":["native containers and normalization","PRNG internals and other sampling paths","incremental RREF native scans and fixed-factor internals","cohort hashing and serialization"]});
    let mut output = report;
    output["descent_stream_advances"] = json!(stream_advances);
    if options.collection_rank_stride.is_some() {
        output["collection_rank_checks"] = json!(rank_checks);
        output["collection_rank_ready"] = json!(rank_ready);
        output["collection_rank_target"] = json!(rank_target);
    }
    if let Some(gate) = gate {
        output["collection_structural_gate"] = json!({"schema":"two-large-prime.structural-rank-gate/v1",
            "checks":gate_checks,"final_state":structural_snapshot(&gate.snapshot()),"charged_word_events_lower_bound":gate_words,
            "word_event_scope":"dense state initialization, support identity/coefficient reads, parent probes and checkpoint bound reads",
            "native_bookkeeping_complete":false,"known_uncounted":"DSU scalar updates, comparisons and allocation/scheduling beyond counted state events"});
    }
    if quotient_reuse {
        output["homogeneous_quotient_reuse"] = json!({"schema":"two-large-prime.homogeneous-quotient-reuse/v1",
            "initial_basis":initial_quotient,"attempts":quotient_attempts,
            "final_extra_counts":retained_quotient.as_ref().map(|q|&q.counts),
            "rank_scope":"kernel-scale-and-target coordinates only; independent full-matrix rank belongs to replay",
            "solver_product_scope":"two-column RREF products only; kernel/projection products recorded separately",
            "native_bookkeeping_complete":false,"known_uncounted":"native RREF, Euclidean, fixed-factor and kernel/projection control beyond emitted state events"});
    }
    Ok((output, scalar))
}

/// Verified `a G' + b Q' = sum epsilon_i B_i` in the cover Jacobian.
/// The caller cannot construct or modify an unchecked descent record.
#[derive(Clone, Debug, Serialize)]
pub struct VerifiedDescentRelation {
    a: u64,
    b: u64,
    decomposition: Dec6,
    modulus: u64,
}

pub struct ProtectedTargetProjection {
    pub target_column: usize,
    pub inputs: Vec<PartialRelation>,
    pub projection: FilteredTwoLargePrimeRelations,
}

impl TwoLargePrimeSieve {
    /// Validate an actual Jacobian equation; the public elliptic transport is
    /// the caller's separate responsibility (the DLP worker rebuilds it).
    pub fn verified_descent(
        &self,
        ctx: &Ctx,
        spec: &Spec,
        a: u64,
        b: u64,
        mut decomposition: Dec6,
    ) -> Option<VerifiedDescentRelation> {
        if ctx.f.f.p != self.setup.p
            || ctx.f.f.w != self.setup.w
            || ctx.cov.hx != self.setup.h
            || spec.p != self.setup.p
            || !(3..1u64 << 63).contains(&spec.l)
            || spec.l.is_multiple_of(2)
            || a >= spec.l
            || b == 0
            || b >= spec.l
        {
            return None;
        }
        decomposition.terms.sort_unstable();
        if decomposition
            .terms
            .iter()
            .any(|&(i, e)| i >= self.base.len() || ![-1, 1].contains(&e))
            || decomposition.terms.windows(2).any(|w| w[0].0 >= w[1].0)
        {
            return None;
        }
        let jac = ctx.jac();
        let residual = jac.add(&jac.mul(&spec.gj, a as u128), &jac.mul(&spec.qj, b as u128));
        verify_dec(&jac, &self.base, &residual, &decomposition).then_some(VerifiedDescentRelation {
            a,
            b,
            decomposition,
            modulus: spec.l,
        })
    }

    /// Eliminate actual LPs from original relations AND affine descent rows.
    /// Log(Q) is a protected small column: coefficient -b, RHS a. Descent
    /// rows may contain up to six LPs, supported by the existing hypergraph
    /// filter. Every output is replayed with its RHS before solver use.
    pub fn filter_with_descent(
        &self,
        ctx: &Ctx,
        spec: &Spec,
        relations: &[TwoLargePrimeRelation],
        descents: &[VerifiedDescentRelation],
        options: &LargePrimeFilterOptions,
    ) -> Result<ProtectedTargetProjection, &'static str> {
        if options.modulus != spec.l
            || relations
                .len()
                .checked_add(descents.len())
                .is_none_or(|n| n > 4096)
        {
            return Err("descent modulus or input cap");
        }
        let mut inputs = self.checked_inputs(ctx, relations, options)?;
        let mut next = match inputs.iter().map(|r| r.id).max() {
            Some(id) => id.checked_add(1).ok_or("source id overflow")?,
            None => 0,
        };
        for d in descents {
            if d.modulus != spec.l
                || self
                    .verified_descent(ctx, spec, d.a, d.b, d.decomposition)
                    .is_none()
            {
                return Err("descent Jacobian or modulus binding failed");
            }
            let (mut small, mut large) = (Vec::new(), Vec::new());
            for &(i, e) in &d.decomposition.terms {
                let v = if e == 1 { 1 } else { spec.l - 1 };
                if i < self.small {
                    small.push((i as u32, v));
                } else {
                    large.push((self.base[i].x, v));
                }
            }
            small.push((self.small as u32, sm(0, d.b, spec.l)));
            inputs.push(PartialRelation {
                id: next,
                small,
                large,
                rhs: d.a,
            });
            next = next.checked_add(1).ok_or("source id overflow")?;
        }
        let projection = filter_and_replay(&inputs, options)?;
        if projection
            .rows
            .iter()
            .any(|r| r.cols.iter().any(|&(c, _)| c > self.small))
        {
            return Err("unprotected target or invalid output column");
        }
        Ok(ProtectedTargetProjection {
            target_column: self.small,
            inputs,
            projection,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::jv_cover::{factor_base, generate_spec};
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn retained_basis_projection_charges_hand_arithmetic_once() {
        use crate::cryptanalysis::gaudry_cubic::SparseRel;
        work::claim();
        work::enable(true);
        work::set_moduli(103, 101);
        let _phase = work::enter(Phase::LinearAlgebra);
        let projection = FilteredTwoLargePrimeRelations {
            rows: vec![SparseRel {
                cols: vec![(0, 1), (1, 2)],
                rhs: 0,
            }],
            certificates: Vec::new(),
            report: Default::default(),
            replayed_rows: 1,
        };
        let homogeneous = homogeneous_system(&projection, 101, 2).unwrap();
        let before = work::snapshot();
        let mut quotient = Quotient::from_rref(101, 2, homogeneous.pivot_rows()).unwrap();
        charge_quotient_delta(&ExtraCounts::default(), &quotient.counts, 101);
        let mut system = CountedRelationSystem::new(101, 2);
        for (row, rhs) in [(vec![3, 4, 1], 17), (vec![1, 0, 2], 9)] {
            let old = quotient.counts.clone();
            let (small, rhs) = quotient.project(&row, rhs).unwrap();
            charge_quotient_delta(&old, &quotient.counts, 101);
            system.insert(small.to_vec(), rhs);
        }
        assert_eq!(system.solved(1), Some(93));
        let delta = work::snapshot().since(&before).total();
        assert_eq!(delta.get(work::Kind::ModrMul), 14);
        assert_eq!(delta.get(work::Kind::ModrAdd), 9);
        assert_eq!(delta.get(work::Kind::ModrInv), 2);
        assert_eq!(delta.get(work::Kind::WordOp), 53);
    }

    #[test]
    fn homogeneous_checkpoint_requires_independent_rows_and_zero_rhs() {
        use crate::cryptanalysis::gaudry_cubic::SparseRel;
        let mut projection = FilteredTwoLargePrimeRelations {
            rows: vec![
                SparseRel {
                    cols: vec![(0, 1), (1, 2)],
                    rhs: 0,
                },
                SparseRel {
                    cols: vec![(0, 2), (1, 4)],
                    rhs: 0,
                },
                SparseRel {
                    cols: vec![(0, 3), (1, 6)],
                    rhs: 0,
                },
            ],
            certificates: Vec::new(),
            report: Default::default(),
            replayed_rows: 3,
        };
        // Three equations, but rank1 cannot reach the3-column target2.
        assert_eq!(homogeneous_rank(&projection, 101, 3).unwrap().0, 1);
        projection.rows.push(SparseRel {
            cols: vec![(1, 1), (2, 1)],
            rhs: 0,
        });
        assert_eq!(homogeneous_rank(&projection, 101, 3).unwrap().0, 2);
        projection.rows[0].rhs = 1;
        assert!(homogeneous_rank(&projection, 101, 3).is_err());
        projection.rows[0].rhs = 0;
        projection.rows[0].cols[0].0 = 3;
        assert!(homogeneous_rank(&projection, 101, 3).is_err());
    }

    #[test]
    fn actual_jacobian_affine_equation_preserves_target_and_nonzero_rhs() {
        // A Jacobian equation fixture, independent of elliptic transport.
        // Full ECDLP recovery must also check Q against the elliptic G.
        let mut spec = generate_spec(251, 2);
        let ctx = Ctx::new(&spec);
        let mut rng = StdRng::seed_from_u64(41);
        let base = factor_base(&ctx, &mut rng);
        let sieve = TwoLargePrimeSieve::new(&ctx, &base, base.len() * 3 / 4, 9).unwrap();
        let decomposition = Dec6 {
            terms: std::array::from_fn(|i| (i, 1)),
        };
        let jac = ctx.jac();
        let mut residual = jac.identity();
        for b in &base[..6] {
            residual = jac.add(&residual, &b.d);
        }
        spec.qj = jac.add(&residual, &jac.neg(&spec.gj));
        let descent = sieve
            .verified_descent(&ctx, &spec, 1, 1, decomposition)
            .unwrap();
        let opts = LargePrimeFilterOptions {
            modulus: spec.l,
            ..Default::default()
        };
        let projected = sieve
            .filter_with_descent(&ctx, &spec, &[], std::slice::from_ref(&descent), &opts)
            .unwrap();
        assert_eq!(projected.projection.replayed_rows, 1);
        assert_eq!(projected.projection.rows[0].rhs, 1);
        assert_eq!(
            projected.projection.rows[0].cols.last(),
            Some(&(sieve.small_columns(), spec.l - 1))
        );
        let mut wrong = descent.clone();
        wrong.a = 2;
        assert!(sieve
            .filter_with_descent(&ctx, &spec, &[], &[wrong], &opts)
            .is_err());
        assert!(sieve
            .verified_descent(&ctx, &spec, 1, 0, decomposition)
            .is_none());
        let mut invalid = decomposition;
        invalid.terms[0].0 = base.len();
        assert!(sieve.verified_descent(&ctx, &spec, 1, 1, invalid).is_none());
    }
}
