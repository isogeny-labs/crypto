//! Optional full-base graph IC with principal quadratic remainder classes.
use super::homogeneous_quotient::{ExtraCounts, Quotient};
use super::principal_relation as remainder;
use crate::cryptanalysis::{
    jv_bitops::{self as work, Phase},
    jv_cover::{self as cover, BaseEl, Ctx, EllE, Fld, Spec, E2},
    jv_sieve::{
        self as sieve,
        two_large_prime::{CountedRelationSystem, TwoLargePrimeSieve},
        Counters, FpRing, Line,
    },
    residual_walk::is_prime_u64,
};
use rand::{rngs::StdRng, SeedableRng};
use remainder::Certificate;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
const SOURCE_CAP: usize = 65_536;
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
    pub b_index: u64,
    pub a0: Vec<u64>,
    pub a1: Vec<u64>,
    pub b: Vec<[u64; 2]>,
    pub s: u64,
    pub certificate: Certificate,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Projection {
    pub terms: Vec<(usize, u64)>,
    pub witness: Vec<(usize, i8)>,
    pub rank: usize,
    pub products: u64,
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

/// Retain one anchor per signed canonical remainder. Each collision carries a
/// two-source integer combination whose remainder coefficient cancels exactly.
pub struct Graph {
    anchors: BTreeMap<[u64; 6], usize>,
    seen: BTreeSet<(Vec<(usize, i8)>, Option<[u64; 6]>)>,
}
impl Graph {
    pub fn new() -> Self {
        Self {
            anchors: BTreeMap::new(),
            seen: BTreeSet::new(),
        }
    }
    pub fn accept(
        &mut self,
        row: &Certificate,
        id: usize,
        old: &[Source],
        p: u64,
    ) -> Option<Option<Projection>> {
        let orientation = row
            .remainder
            .as_ref()
            .map_or(row.terms[0].1, |r| r.coefficient);
        let key = (
            row.terms
                .iter()
                .map(|&(i, s)| (i, s * orientation))
                .collect::<Vec<_>>(),
            row.remainder.as_ref().map(|r| r.key),
        );
        work::probe((row.terms.len() * 2 + 6) as u64);
        if !self.seen.insert(key) {
            return None;
        }
        let witness = if let Some(r) = &row.remainder {
            work::probe(6);
            if let Some(&anchor) = self.anchors.get(&r.key) {
                let c0 = old[anchor]
                    .certificate
                    .remainder
                    .as_ref()
                    .expect("partial anchor")
                    .coefficient;
                vec![(anchor, -r.coefficient), (id, c0)]
            } else {
                self.anchors.insert(r.key, id);
                return Some(None);
            }
        } else {
            vec![(id, 1)]
        };
        let mut sum = BTreeMap::new();
        for &(source, coefficient) in &witness {
            let c = if source == id {
                row
            } else {
                &old[source].certificate
            };
            for &(i, e) in &c.terms {
                let value = signed(coefficient * e, p);
                let entry = sum.entry(i).or_insert(0);
                *entry = add_mod(*entry, value, p);
                work::words(4);
                work::probe(1);
            }
        }
        Some(Some(Projection {
            terms: sum.into_iter().filter(|&(_, v)| v != 0).collect(),
            witness,
            rank: 0,
            products: 0,
        }))
    }
}

pub fn candidate(
    ctx: &Ctx,
    ring: &FpRing,
    config: &TwoLargePrimeSieve,
    line: &Line,
    s: u64,
) -> Result<Certificate, &'static str> {
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
    for (i, b) in config.base().iter().enumerate() {
        work::words(2);
        if ring.eval(&norm, b.x) != 0 {
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
    let checked = remainder::check_equation(
        f,
        &ctx.cov.hx,
        config.base(),
        &remainder::trim(a),
        &remainder::trim(c),
        &norm,
        &terms,
        &tail,
    )?;
    if checked.certificate().remainder.is_none()
        && !remainder::replay_group(&ctx.jac(), config.base(), checked.certificate())
    {
        return Err("full Jacobian check");
    }
    Ok(checked.into_certificate())
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct ExtensionRemainderDlpOptions {
    pub quadratic_remainders: bool,
    pub b_count: Option<u128>,
    pub max_descent_residuals: u64,
    pub max_accepted_descents: usize,
}
impl Default for ExtensionRemainderDlpOptions {
    fn default() -> Self {
        Self {
            quadratic_remainders: true,
            b_count: None,
            max_descent_residuals: 8192,
            max_accepted_descents: 32,
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
pub fn run_extension_remainder_dlp_on(
    spec: &Spec,
    seed: u64,
    options: ExtensionRemainderDlpOptions,
) -> Result<(Value, Option<u64>), String> {
    if !(101..=1511).contains(&spec.p)
        || !(3..1u64 << 63).contains(&spec.l)
        || !is_prime_u64(spec.p)
        || !is_prime_u64(spec.l)
        || options.max_descent_residuals == 0
        || options.max_descent_residuals > 1_000_000
        || !(2..=32).contains(&options.max_accepted_descents)
    {
        return Err("invalid extension-remainder DLP bounds".into());
    }
    let mode = if options.quadratic_remainders {
        "quadratic-remainders"
    } else {
        "full-smooth"
    };
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
    let mut ctr = Counters::new(spec.p);
    let mut enum_rng = StdRng::seed_from_u64(seed ^ 0xA17E_2E17);
    let mut extraction_rng = StdRng::seed_from_u64(seed ^ 0xE17A_C710);
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
    let mut graph = Graph::new();
    let mut system = CountedRelationSystem::new(spec.l, base.len());
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
                let hits = if mode == "full-smooth" {
                    sieve::sieve_line(
                        &ring,
                        config.setup(),
                        &line,
                        &mut ctr,
                        None,
                        shared.as_ref(),
                    )
                    .0
                } else {
                    config.sieve_line(&ring, &line, &mut ctr, shared.as_ref()).0
                };
                for s in hits {
                    counts.candidates += 1;
                    let certificate = if mode == "full-smooth" {
                        config
                            .verified_full_smooth_relation(
                                &ctx,
                                &ring,
                                &line,
                                s,
                                sources.len() as u64,
                                spec.l,
                                &mut extraction_rng,
                            )
                            .map(|r| Certificate {
                                terms: r.full().terms.clone(),
                                remainder: None,
                                a: vec![],
                                c: vec![],
                                norm: vec![],
                            })
                    } else {
                        candidate(&ctx, &ring, &config, &line, s).ok()
                    };
                    let Some(certificate) = certificate else {
                        counts.refused += 1;
                        continue;
                    };
                    if sources.len() >= SOURCE_CAP {
                        stop = "source resource cap".into();
                        counts.b_count = begin + offset as u64;
                        break 'collect;
                    }
                    let Some(projected) =
                        graph.accept(&certificate, sources.len(), &sources, spec.l)
                    else {
                        counts.duplicates += 1;
                        continue;
                    };
                    if certificate.remainder.is_some() {
                        counts.partial_rows += 1;
                    } else {
                        counts.full_rows += 1;
                    }
                    sources.push(Source {
                        b_index: begin + offset as u64,
                        a0: line.a0.clone(),
                        a1: line.a1.clone(),
                        b: remainder::encode(&line.b),
                        s,
                        certificate,
                    });
                    if let Some(mut projection) = projected {
                        if projection.witness.len() == 2 {
                            counts.collisions += 1;
                        }
                        let _la = work::enter(Phase::LinearAlgebra);
                        let mut dense = vec![0; base.len()];
                        work::words(base.len() as u64);
                        for &(i, v) in &projection.terms {
                            dense[i] = v;
                            work::words(2);
                        }
                        system.insert(dense, 0);
                        projection.rank = system.rank();
                        projection.products = system.ops();
                        projections.push(projection);
                    }
                }
            }
        }
        counts.b_count = end;
        checkpoints.push(Checkpoint {
            b_count: end,
            lines: counts.lines,
            cohort_sha256: format!("{:x}", hash.clone().finalize()),
            sources: sources.len(),
            projections: projections.len(),
            rank: system.rank(),
        });
        if system.rank() >= rank_target {
            stop = "homogeneous rank target at complete batch".into();
            break;
        }
    }
    counts.vertices = graph.anchors.len();
    counts.rank = system.rank();
    counts.products = system.ops();
    counts.normalization_products = system.normalization_products();
    counts.subtractions = system.subtraction_ops();
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
        "foreign_hook_calls":work::foreign(),"other_modulus_ops":other,"known_uncounted":["native allocations/containers/scans/normalization and control flow","PRNG internals and incompletely hooked sampling","incremental RREF scans/fixed-factor internals","cohort hashing/serialization"],
        "excluded":"caller public domain validation and curve transfer; charged separately by full worker"});
    let ready = system.rank() == rank_target;
    let mut basis = if ready {
        let _la = work::enter(Phase::LinearAlgebra);
        let q = Quotient::from_rref(spec.l, base.len(), system.pivot_rows())?;
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
        let opts = cover::opts_for(24, 600.0, Some(64));
        for i in 0..options.max_descent_residuals {
            let mut rng = StdRng::seed_from_u64(seed ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let (decompositions, cost) =
                cover::nagao_decompose(&ctx, &base, &by_x, &residual, &opts, &mut rng);
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
    let collection = json!({"schema":"cover.extension-remainder.collection/v1","mode":mode,"seed":seed,"p":spec.p,"l":spec.l,
        "base":points(&base),"h":remainder::encode(&ctx.cov.hx),"sources":sources,"projections":projections,"checkpoints":checkpoints,"counts":counts,"rank_target":rank_target,"stop":stop,"work_native":collection_native});
    let report = json!({"schema":"cover.extension-remainder.dlp/v1","p":spec.p,"l":spec.l,"seed":seed,"options":options,
        "collection":collection,"collection_rank_ready":ready,"initial_basis":initial_basis,"final_extra_counts":basis.as_ref().map(|q|&q.counts),
        "descents":descents,"affine_rows":affine_rows,"projected_rows":projected_rows,"descent_residuals":residuals,"descent_stream_advances":advances,
        "descent_timeouts":timeouts,"descent_incomplete":incomplete,"invalid_descents":invalid,"last_rank":affine.rank(),"solver_modular_muls":affine.ops(),
        "actual_scalar":scalar,"verified":scalar.is_some(),"capped":scalar.is_none(),"resource_refusal":refusal,"fallback_solves":0,
        "rank_scope":"kernel-scale and protected-target coordinates only; full-base homogeneous rank is in collection",
        "solver_product_scope":"two-column affine RREF only; collection/kernel/projection work recorded separately",
        "complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"known_uncounted":["native containers/control/normalization/scans","PRNG internals","fixed-factor and Euclidean internals","hashing/serialization"]});
    Ok((report, scalar))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn source(certificate: Certificate) -> Source {
        Source {
            b_index: 0,
            a0: vec![],
            a1: vec![],
            b: vec![],
            s: 1,
            certificate,
        }
    }
    fn partial(terms: Vec<(usize, i8)>, coefficient: i8) -> Certificate {
        Certificate {
            terms,
            remainder: Some(remainder::Remainder {
                key: [3, 4, 5, 6, 7, 8],
                coefficient,
            }),
            a: vec![],
            c: vec![],
            norm: vec![],
        }
    }
    #[test]
    fn signed_class_anchor_and_negative_duplicate_controls() {
        let first = partial(vec![(0, 1), (1, -1)], -1);
        let old = vec![source(first.clone())];
        let mut graph = Graph::new();
        assert!(graph.accept(&first, 0, &[], 101).unwrap().is_none());
        assert!(graph
            .accept(&partial(vec![(0, -1), (1, 1)], 1), 1, &old, 101)
            .is_none());
        let row = graph
            .accept(&partial(vec![(0, 1), (2, 1)], 1), 1, &old, 101)
            .unwrap()
            .unwrap();
        assert_eq!(row.witness, vec![(0, -1), (1, -1)]);
        assert_eq!(row.terms, vec![(0, 99), (1, 1), (2, 100)]);
    }
    #[test]
    fn protected_affine_sign_and_actual_two_column_recovery() {
        work::claim();
        work::set_moduli(103, 101);
        work::enable(true);
        let _la = work::enter(Phase::LinearAlgebra);
        let mut quotient = Quotient::from_rref(101, 2, &[(0, vec![1, 2], 0)]).unwrap();
        let mut system = CountedRelationSystem::new(101, 2);
        for (row, rhs) in [(vec![1, 1, 98], 2), (vec![1, 0, 96], 11)] {
            let (projected, rhs) = quotient.project(&row, rhs).unwrap();
            system.insert(projected.to_vec(), rhs);
        }
        assert_eq!(system.rank(), 2);
        assert_eq!(system.solved(0), Some(78));
        assert_eq!(system.solved(1), Some(7));
        work::enable(false);
    }
    #[test]
    fn invalid_caps_refused_before_relation_collection() {
        let spec = cover::generate_spec(101, 17);
        for options in [
            ExtensionRemainderDlpOptions {
                max_descent_residuals: 0,
                ..Default::default()
            },
            ExtensionRemainderDlpOptions {
                max_descent_residuals: 1_000_001,
                ..Default::default()
            },
            ExtensionRemainderDlpOptions {
                max_accepted_descents: 1,
                ..Default::default()
            },
        ] {
            assert!(run_extension_remainder_dlp_on(&spec, 389, options).is_err());
        }
    }
}
