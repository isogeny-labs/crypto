//! Native cold relation-collection component; public curve input only.
use crate::remainder::{self, Certificate};
use crypto_lib::cryptanalysis::{
    jv_bitops::{self as work, Phase},
    jv_cover::{self as cover, BaseEl, Ctx, EllE, Fld, Fq3, PtE6, E2, E6},
    jv_sieve::{
        self as sieve,
        two_large_prime::{CountedRelationSystem, TwoLargePrimeSieve},
        Counters, FpRing, Line,
    },
    residual_walk::is_prime_u64,
};
use rand::{rngs::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    time::Instant,
};

pub const SEED: u64 = 379;
pub const SOURCE_CAP: usize = 65_536;
pub const INPUT_SHA: &str = "07915bb6d6cb22867f5c36b7cf6ffbfd6f10902025488ad074df94e7448c1159";
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicPoint {
    pub x: [[u64; 2]; 3],
    pub y: [[u64; 2]; 3],
    pub inf: bool,
}
impl PublicPoint {
    pub fn native(&self) -> PtE6 {
        PtE6 {
            x: E6(self.x.map(E2)),
            y: E6(self.y.map(E2)),
            inf: self.inf,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicCurve {
    pub p: u64,
    pub l: u64,
    pub alpha: [[u64; 2]; 3],
    pub g: PublicPoint,
    pub q: PublicPoint,
}
impl PublicCurve {
    pub fn validate(&self) -> Result<(), String> {
        if self.p != 1009
            || self.l != 263807419864521289
            || !is_prime_u64(self.p)
            || !is_prime_u64(self.l)
            || self.g.inf
            || self.q.inf
            || self
                .alpha
                .iter()
                .flatten()
                .chain(self.g.x.iter().flatten())
                .chain(self.g.y.iter().flatten())
                .chain(self.q.x.iter().flatten())
                .chain(self.q.y.iter().flatten())
                .any(|&x| x >= self.p)
        {
            return Err("invalid registered public domain".into());
        }
        let f = Fq3::new(self.p);
        let ec = EllE::new(&f, &E6(self.alpha.map(E2)));
        let (g, q) = (self.g.native(), self.q.native());
        if !ec.on_curve(&g)
            || !ec.on_curve(&q)
            || !ec.mul(&g, self.l).inf
            || !ec.mul(&q, self.l).inf
        {
            return Err("invalid public subgroup".into());
        }
        Ok(())
    }
}
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
#[derive(Serialize, Deserialize)]
pub struct Record {
    pub schema: String,
    pub source_revision: String,
    pub input_sha256: String,
    pub mode: String,
    pub seed: u64,
    pub public: PublicCurve,
    pub base: Vec<Point>,
    pub h: Vec<[u64; 2]>,
    pub sources: Vec<Source>,
    pub projections: Vec<Projection>,
    pub checkpoints: Vec<Checkpoint>,
    pub counts: Counts,
    pub rank_target: usize,
    pub stop: String,
    pub inner_wall_ns: u128,
    pub work_native: Value,
}
pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
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

pub fn run(curve: PublicCurve, mode: &str, source_revision: String) -> Result<Record, String> {
    if !["full-smooth", "quadratic-remainders"].contains(&mode) {
        return Err("unregistered mode".into());
    }
    curve.validate()?;
    work::claim();
    work::set_moduli(curve.p, curve.l);
    work::enable(true);
    let before = work::snapshot();
    let uniform_before = work::uniform_snapshot();
    let other_before = work::snapshot_other();
    let started = Instant::now();
    let model = work::enter(Phase::Model);
    let spec = cover::spec_from_curve(
        curve.p,
        E6(curve.alpha.map(E2)),
        curve.l,
        curve.g.native(),
        curve.q.native(),
        0,
    )
    .ok_or("cover transfer")?;
    let ctx = Ctx::new(&spec);
    drop(model);
    let base_phase = work::enter(Phase::FactorBase);
    let mut base_rng = StdRng::seed_from_u64(SEED ^ 0xBACE_0001);
    let base = cover::factor_base(&ctx, &mut base_rng);
    let config = TwoLargePrimeSieve::new(&ctx, &base, base.len(), 9)?;
    drop(base_phase);
    let relation_phase = work::enter(Phase::Relations);
    let ring = FpRing::new(curve.p);
    let mut ctr = Counters::new(curve.p);
    let mut enum_rng = StdRng::seed_from_u64(SEED ^ 0xA17E_2E17);
    let mut extraction_rng = StdRng::seed_from_u64(SEED ^ 0xE17A_C710);
    let count = sieve::b_space(config.setup());
    let scramble = sieve::coprime_scramble(count, &mut enum_rng);
    let nu = sieve::nu_lines(config.setup());
    let mut slot = Vec::new();
    let mut hash = Sha256::new();
    let mut counts = Counts::default();
    let mut graph = Graph::new();
    let mut system = CountedRelationSystem::new(curve.l, base.len());
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
        let hints = sieve::roots_by_incidence(curve.p, &nu, &ab, &mut slot).0;
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
                                curve.l,
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
                        graph.accept(&certificate, sources.len(), &sources, curve.l)
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
    let elapsed = started.elapsed().as_nanos();
    let tally = work::snapshot().since(&before);
    let widths = work::widths(curve.p, curve.l);
    let phases:Vec<_>=Phase::IC.iter().map(|&p|json!({"phase":p.name(),"primitives":work::prims_json(tally.phase(p)),"diagnostic_priced_bitops":work::price(tally.phase(p),&widths).expect("widths").to_string()})).collect();
    let uniforms = work::uniform_snapshot()
        .since(&uniform_before)
        .ok_or("uniform regression")?;
    let other: Vec<_> = work::snapshot_other()
        .iter()
        .zip(other_before)
        .map(|(&a, b)| a.checked_sub(b))
        .collect();
    let work_native = json!({"complete":false,"total_bitops":null,"comparative_eligibility":false,"ratio":null,"band":null,
        "phases":phases,"observed_primitives":work::prims_json(&tally.sum(&Phase::IC)),"uniform_sampling":work::uniform_json(&uniforms.sum(&Phase::IC).ok_or("uniform overflow")?),
        "foreign_hook_calls":work::foreign(),"other_modulus_ops":other,"known_uncounted":["native allocations/containers/scans/normalization and control flow","PRNG internals and incompletely hooked sampling","incremental RREF scans/fixed-factor internals","cohort hashing/serialization"],
        "excluded":"public input domain validation; process wall includes it; no reusable attack precomputation"});
    work::enable(false);
    Ok(Record {
        schema: "cover.extension-remainder.component/v1".into(),
        source_revision,
        input_sha256: INPUT_SHA.into(),
        mode: mode.into(),
        seed: SEED,
        public: curve,
        base: points(&base),
        h: remainder::encode(&ctx.cov.hx),
        sources,
        projections,
        checkpoints,
        counts,
        rank_target,
        stop,
        inner_wall_ns: elapsed,
        work_native,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
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
    #[test]
    fn signed_anchor_collisions_cancel_and_negative_duplicates_are_removed() {
        let first = partial(vec![(0, 1), (1, -1)], -1);
        let old = vec![source(first.clone())];
        let mut graph = Graph::new();
        assert!(graph.accept(&first, 0, &[], 101).unwrap().is_none());
        let duplicate = partial(vec![(0, -1), (1, 1)], 1);
        assert!(graph.accept(&duplicate, 1, &old, 101).is_none());
        let next = partial(vec![(0, 1), (2, 1)], 1);
        let projected = graph.accept(&next, 1, &old, 101).unwrap().unwrap();
        assert_eq!(projected.witness, vec![(0, -1), (1, -1)]);
        assert_eq!(projected.terms, vec![(0, 99), (1, 1), (2, 100)]);
        assert_eq!(graph.anchors.len(), 1);
    }
    #[test]
    fn full_rows_pass_through_and_canonical_duplicates_are_removed() {
        let row = Certificate {
            terms: vec![(0, -1), (2, 1)],
            remainder: None,
            a: vec![],
            c: vec![],
            norm: vec![],
        };
        let mut graph = Graph::new();
        let out = graph.accept(&row, 0, &[], 101).unwrap().unwrap();
        assert_eq!(out.witness, vec![(0, 1)]);
        assert_eq!(out.terms, vec![(0, 100), (2, 1)]);
        let negative = Certificate {
            terms: vec![(0, 1), (2, -1)],
            ..row.clone()
        };
        assert!(graph.accept(&negative, 1, &[source(row)], 101).is_none());
    }
}
