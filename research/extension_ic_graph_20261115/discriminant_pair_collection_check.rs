//! Independent Cantor replay and column-keyed u128 rank/counter qualification.
//! This does not call the producer's certificate constructor or graph filter.
use crate::{
    pair_driver::{self as driver, Record, Source},
    remainder,
};
#[allow(dead_code)]
#[path = "native_analysis/src/bin/two-lp-component-analysis.rs"]
pub(crate) mod accounting;
use accounting::{add, mul, pow};
use crypto_lib::cryptanalysis::{
    jv_cover::{self as cover, BaseEl, Ctx, Fq, Hyp, E2, E6},
    jv_sieve::{self as sieve},
};
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

fn ensure(ok: bool, reason: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(reason.into())
    }
}
#[derive(Default)]
pub struct Rank {
    pub basis: BTreeMap<usize, Vec<u64>>,
    order: Vec<usize>,
    pub products: u64,
}
impl Rank {
    pub fn insert(&mut self, mut row: Vec<u64>, l: u64) -> Result<(), String> {
        ensure(row.iter().all(|&v| v < l), "rank noncanonical input")?;
        let columns = row.len();
        // Column-keyed storage and ordinary u128 remainder arithmetic; the
        // producer uses fixed-factor arithmetic and an insertion-order vector.
        for &pivot in &self.order {
            let factor = row[pivot];
            if factor != 0 {
                for (v, &w) in row.iter_mut().zip(&self.basis[&pivot]) {
                    if w != 0 {
                        *v = add(*v, l - mul(factor, w, l), l);
                        self.products += 1;
                    }
                }
                self.products += 1; // homogeneous RHS multiply is still executed
            }
        }
        let Some(pivot) = row.iter().position(|&x| x != 0) else {
            return Ok(());
        };
        let inv = pow(row[pivot], l - 2, l);
        for v in &mut row {
            *v = mul(*v, inv, l);
        }
        self.products += columns as u64 + 1;
        for old in self.basis.values_mut() {
            let factor = old[pivot];
            if factor != 0 {
                for (v, &w) in old.iter_mut().zip(&row) {
                    if w != 0 {
                        *v = add(*v, l - mul(factor, w, l), l);
                        self.products += 1;
                    }
                }
                self.products += 1;
            }
        }
        self.basis.insert(pivot, row);
        self.order.push(pivot);
        Ok(())
    }
}

pub(crate) fn source_proof(
    jac: &Hyp<Fq>,
    base: &[BaseEl],
    source: &Source,
    expected_b: &[E2],
    domain: &crate::pair_math_check::Domain,
) -> Result<(), String> {
    ensure(
        remainder::decode(&source.b) == expected_b,
        "enumerated B binding",
    )?;
    let proof = domain.line(&serde_json::to_value(source).map_err(|e| e.to_string())?)?;
    let row = &source.certificate;
    ensure(
        row.a == proof.a && row.c == proof.c && row.norm == proof.norm && row.terms == proof.terms,
        "independent complete line/certificate binding",
    )?;
    domain.check(&serde_json::to_value(row).map_err(|e| e.to_string())?)?;
    // Separate Cantor backend: no producer replay helper or constructor.
    let mut sum = jac.identity();
    for &(i, sign) in &row.terms {
        let d = &base.get(i).ok_or("group base range")?.d;
        ensure(jac.is_valid(d), "small group divisor")?;
        sum = jac.add(&sum, &if sign == 1 { d.clone() } else { jac.neg(d) });
    }
    for q in &row.remainders {
        let d = cover::Div {
            u: vec![E2([q.key[0], 0]), E2([q.key[1], 0]), E2::ONE],
            v: remainder::trim(vec![E2([q.key[2], q.key[3]]), E2([q.key[4], q.key[5]])]),
        };
        ensure(jac.is_valid(&d), "quadratic group divisor")?;
        sum = jac.add(&sum, &if q.coefficient == 1 { d } else { jac.neg(&d) });
    }
    ensure(
        jac.is_identity(&sum),
        "independent original Cantor identity",
    )
}
pub fn qualify(record: &Record, seed: u64, input_sha: &str) -> Result<Value, String> {
    qualify_impl(record, seed, input_sha, None)
}
pub fn qualify_sparse(
    record: &Record,
    seed: u64,
    input_sha: &str,
    evidence: &Value,
) -> Result<Value, String> {
    qualify_impl(record, seed, input_sha, Some(evidence))
}
fn qualify_impl(
    record: &Record,
    seed: u64,
    input_sha: &str,
    sparse: Option<&Value>,
) -> Result<Value, String> {
    ensure(
        record.schema == "cover.extension-pipeline.collection-projection/v1"
            && record.seed == seed
            && record.input_sha256 == input_sha,
        "registered record",
    )?;
    ensure(
        ["grounded-quadratic-pairs", "discriminant-quadratic-pairs"]
            .contains(&record.mode.as_str()),
        "mode",
    )?;
    ensure(
        record.source_revision.len() == 40
            && record
                .source_revision
                .bytes()
                .all(|c| c.is_ascii_hexdigit()),
        "source revision",
    )?;
    validate_public(&record.public)?;
    let curve = &record.public;
    let spec = cover::spec_from_curve(
        curve.p,
        E6(curve.alpha.map(E2)),
        curve.l,
        curve.g.native(),
        curve.q.native(),
        0,
    )
    .ok_or("cover")?;
    let ctx = Ctx::new(&spec);
    let f = &ctx.f.f;
    let mut rng = StdRng::seed_from_u64(record.seed ^ 0xBACE_0001);
    let base = cover::factor_base(&ctx, &mut rng);
    ensure(
        record.base == driver::points(&base) && record.h == remainder::encode(&ctx.cov.hx),
        "cold model/base binding",
    )?;
    let setup = sieve::sieve_setup(f, &ctx.cov.hx, &base, 9);
    let mut enum_rng = StdRng::seed_from_u64(record.seed ^ 0xA17E_2E17);
    let space = sieve::b_space(&setup);
    let scramble = sieve::coprime_scramble(space, &mut enum_rng);
    let c = &record.counts;
    ensure(
        c.b_count as u128 <= space
            && record.sources.len() <= driver::SOURCE_CAP
            && record.rank_target + 1 == base.len(),
        "resource/rank bounds",
    )?;
    ensure(
        record
            .sources
            .windows(2)
            .all(|w| w[0].b_index <= w[1].b_index),
        "source enumeration order",
    )?;
    let domain = crate::pair_math_check::Domain::new(
        curve.p,
        f.w,
        record.h.clone(),
        base.iter().map(|b| (b.x, b.y.0)).collect(),
    )?;
    ensure(
        record.graph_options
            == json!({"max_sources":262144,"max_vertices":65536,"max_witness_terms":256})
            && record.split_options == json!({"max_attempts":8})
            && record.max_split_records == 524288
            && record.splits.len() <= record.max_split_records
            && record.split_seed_xor == 0x5155_4144_5041_4952
            && record.root_finder == "forward-differences"
            && record.sieve_selection == "counter-table-five-ascending",
        "pair algorithm registration",
    )?;
    let mut seen = BTreeSet::new();
    let mut graph = crate::graph_math_check::Replay::default();
    let mut expected = Vec::new();
    let (mut partials, mut full, mut one, mut two) = (0u64, 0u64, 0u64, 0u64);
    let mut split_keys = BTreeSet::new();
    let mut split_successes = BTreeSet::new();
    for split in &record.splits {
        ensure(
            split.b_index < c.b_count
                || record.stop.ends_with("resource cap") && split.b_index == c.b_count,
            "split B range",
        )?;
        let expected_b = sieve::b_from_index(&setup, split.b_index as u128, scramble);
        ensure(
            split.b == remainder::encode(&expected_b),
            "split enumerated B binding",
        )?;
        let proof = domain.line(&serde_json::to_value(split).map_err(|e| e.to_string())?)?;
        ensure(
            proof.tail == split.tail && proof.tail.len() == 5 && proof.terms.len() == 5,
            "quartic original tail binding",
        )?;
        crate::pair_math_check::check_split(&split.report, &split.tail, 8)?;
        let key = split.candidate_index;
        ensure(split_keys.insert(key), "duplicate quartic invocation")?;
        if split.report["factors"].is_array() {
            split_successes.insert(key);
        }
    }
    for (id, s) in record.sources.iter().enumerate() {
        ensure(
            s.b_index < c.b_count
                || record.stop.ends_with("resource cap") && s.b_index == c.b_count,
            "source B range",
        )?;
        let b = sieve::b_from_index(&setup, s.b_index as u128, scramble);
        source_proof(&ctx.jac(), &base, s, &b, &domain)?;
        let row = &s.certificate;
        let orient = row
            .remainders
            .first()
            .map_or(row.terms[0].1, |r| r.coefficient);
        let key = (
            row.terms
                .iter()
                .map(|&(i, c)| (i, c * orient))
                .collect::<Vec<_>>(),
            row.remainders
                .iter()
                .map(|r| (r.key, r.coefficient * orient))
                .collect::<Vec<_>>(),
        );
        ensure(seen.insert(key), "duplicate original relation")?;
        match row.remainders.len() {
            0 => full += 1,
            1 => {
                partials += 1;
                one += 1;
            }
            2 => {
                partials += 1;
                two += 1;
                ensure(
                    split_successes.contains(&s.candidate_index),
                    "two-class source missing successful split history",
                )?;
            }
            _ => return Err("class cardinality".into()),
        }
        if let Some(witness) = graph.accept(&record.sources[..=id])? {
            expected.push(witness);
        }
    }
    ensure(
        graph.stats() == record.graph_stats,
        "all grounded graph statistics",
    )?;
    ensure(
        expected.len() == record.projections.len(),
        "projection completeness",
    )?;
    let mut rank = Rank::default();
    let mut prefix_ranks = vec![0usize];
    for (projection, witness) in record.projections.iter().zip(&expected) {
        ensure(&projection.witness == witness, "anchor witness")?;
        let mut small = BTreeMap::<usize, i64>::new();
        let mut large = BTreeMap::new();
        for &(id, coefficient) in witness {
            let row = &record.sources[id].certificate;
            for &(i, c) in &row.terms {
                *small.entry(i).or_default() += c as i64 * coefficient as i64;
            }
            for r in &row.remainders {
                *large.entry(r.key).or_insert(0i64) += (r.coefficient * coefficient) as i64;
            }
        }
        ensure(large.values().all(|&v| v == 0), "remainder cancellation")?;
        let terms: Vec<_> = small
            .into_iter()
            .map(|(i, v)| (i, (v as i128).rem_euclid(curve.l as i128) as u64))
            .filter(|&(_, v)| v != 0)
            .collect();
        ensure(terms == projection.terms, "projected small replay")?;
        let mut dense = vec![0; base.len()];
        for &(i, v) in &terms {
            dense[i] = v;
        }
        rank.insert(dense, curve.l)?;
        prefix_ranks.push(rank.basis.len());
        ensure(
            if sparse.is_some() {
                projection.rank == 0 && projection.products == 0
            } else {
                projection.rank == rank.basis.len() && projection.products == rank.products
            },
            "every rank/product prefix",
        )?;
    }
    ensure(
        if sparse.is_some() {
            c.rank <= rank.basis.len()
                && c.products == 0
                && c.normalization_products == 0
                && c.subtractions == 0
        } else {
            c.rank == rank.basis.len()
                && c.products == rank.products
                && c.normalization_products == c.rank as u64 * (base.len() as u64 + 1)
                && c.subtractions == c.products - c.normalization_products
        },
        "rank/count endpoint",
    )?;
    ensure(
        c.partial_rows == partials
            && c.full_rows == full
            && c.one_class_rows == one
            && c.two_class_rows == two
            && record.graph_stats["vertices"] == c.vertices
            && c.collisions as usize == record.projections.len() - full as usize,
        "source/graph counts",
    )?;
    ensure(
        c.candidates
            == c.refused
                + c.duplicates
                + record.sources.len() as u64
                + u64::from(record.stop.ends_with("resource cap")),
        "candidate flow",
    )?;
    let mut old_b = 0;
    let mut old_lines = 0;
    let evidence_checks = if let Some(e) = sparse {
        ensure(
            e["rank_options"]
                == json!({"surplus":8,"failed_checkpoint_stride":16,"max_checkpoints":8,"max_preconditioners":3})
                && e["rank_semantics"]
                    == "certified lower bound; zero before success; no incremental RREF products",
            "sparse registration",
        )?;
        Some(e["rank_checks"].as_array().ok_or("sparse checks")?)
    } else {
        None
    };
    let mut sparse_index = 0usize;
    let mut next_rows = record.rank_target + 8;
    let mut certified = 0usize;
    let mut sparse_proofs = Vec::new();
    let mut sparse_primitives: BTreeMap<String, u64> = accounting::KINDS
        .into_iter()
        .map(|k| (k.into(), 0))
        .collect();
    for checkpoint in &record.checkpoints {
        ensure(
            checkpoint.b_count > old_b
                && checkpoint.b_count <= c.b_count
                && checkpoint.b_count as u128 <= space
                && (checkpoint.b_count % 1024 == 0 || checkpoint.b_count as u128 == space),
            "complete checkpoint batch",
        )?;
        ensure(
            checkpoint.lines >= old_lines
                && checkpoint.cohort_sha256.len() == 64
                && checkpoint
                    .cohort_sha256
                    .bytes()
                    .all(|b| b.is_ascii_hexdigit()),
            "cohort fingerprint",
        )?;
        let sources = record
            .sources
            .iter()
            .take_while(|s| s.b_index < checkpoint.b_count)
            .count();
        let projections = record
            .projections
            .iter()
            .take_while(|p| p.witness.last().is_some_and(|&(id, _)| id < sources))
            .count();
        let mut expected_rank = record
            .projections
            .get(projections.wrapping_sub(1))
            .map_or(0, |p| p.rank);
        if let Some(checks) = evidence_checks {
            if projections >= next_rows && certified == 0 && sparse_index < 8 {
                let check = checks
                    .get(sparse_index)
                    .ok_or("missing scheduled sparse check")?;
                ensure(
                    check["b_count"] == checkpoint.b_count && check["rows"] == projections,
                    "sparse batch binding",
                )?;
                let matrix = json!({"modulus":curve.l,"columns":base.len(),"rows":record.projections[..projections].iter().map(|p|p.terms.clone()).collect::<Vec<_>>()});
                let proof = crate::sparse_math_check::qualify_checkpoint(
                    &matrix,
                    check,
                    prefix_ranks[projections],
                )?;
                for (k, v) in accounting::counts(&check["native"]["primitives"])? {
                    let out = sparse_primitives
                        .get_mut(&k)
                        .ok_or("checkpoint primitive")?;
                    *out = out.checked_add(v).ok_or("checkpoint count overflow")?;
                }
                if proof["verified_kernel"] == true {
                    certified = record.rank_target;
                }
                sparse_proofs.push(proof);
                sparse_index += 1;
                next_rows = projections + 16;
            }
            expected_rank = certified;
        }
        ensure(
            checkpoint.sources == sources
                && checkpoint.projections == projections
                && checkpoint.rank == expected_rank,
            "checkpoint row/rank prefix",
        )?;
        if checkpoint.rank >= record.rank_target {
            ensure(
                checkpoint.b_count == c.b_count
                    && record.stop
                        == if sparse.is_some() {
                            "sparse homogeneous rank certified at complete batch"
                        } else {
                            "homogeneous rank target at complete batch"
                        },
                "rank stop schedule",
            )?;
        }
        old_b = checkpoint.b_count;
        old_lines = checkpoint.lines;
    }
    if let Some(checks) = evidence_checks {
        ensure(
            sparse_index == checks.len() && checks.len() <= 8 && c.rank == certified,
            "all sparse checkpoints consumed",
        )?;
    }
    match record.stop.as_str() {
        "sparse homogeneous rank certified at complete batch" => ensure(
            sparse.is_some()
                && certified == record.rank_target
                && rank.basis.len() == record.rank_target
                && old_b == c.b_count
                && old_lines == c.lines,
            "sparse rank stop",
        )?,
        "sparse kernel checkpoint cap" => ensure(
            sparse.is_some()
                && sparse_index == 8
                && certified == 0
                && old_b == c.b_count
                && old_lines == c.lines,
            "sparse checkpoint cap",
        )?,
        "homogeneous rank target at complete batch" => ensure(
            c.rank >= record.rank_target && old_b == c.b_count && old_lines == c.lines,
            "rank stop",
        )?,
        "B budget exhausted" => ensure(
            c.b_count as u128 == space
                && old_b == c.b_count
                && old_lines == c.lines
                && c.rank < record.rank_target,
            "budget stop",
        )?,
        "source resource cap" => ensure(
            record.sources.len() == driver::SOURCE_CAP,
            "source cap stop",
        )?,
        "split trace resource cap" => ensure(
            record.splits.len() == record.max_split_records,
            "split trace cap stop",
        )?,
        "discriminant refusal resource cap" => {
            let evidence = sparse.ok_or("screened sparse evidence")?;
            ensure(
                record.mode == "discriminant-quadratic-pairs"
                    && evidence["discriminant_screen"]["refusals"]
                        .as_array()
                        .is_some_and(|x| x.len() == 1_000_000),
                "discriminant refusal cap",
            )?;
        }
        _ => return Err("unknown endpoint".into()),
    }
    let work = &record.work_native;
    ensure(
        work["known_uncounted"]
            .as_array()
            .is_some_and(|a| !a.is_empty()),
        "explicit collection coverage gaps",
    )?;
    ensure(
        work["complete"] == false
            && work["comparative_eligibility"] == false
            && work["total_bitops"].is_null()
            && work["ratio"].is_null()
            && work["band"].is_null(),
        "cost scope",
    )?;
    ensure(
        work["foreign_hook_calls"] == 0
            && work["other_modulus_ops"]
                .as_array()
                .is_some_and(|a| a.len() == 15 && a.iter().all(|v| *v == 0)),
        "counter isolation",
    )?;
    let widths = accounting::Widths {
        field: accounting::Field::Prime {
            bits: 64 - curve.p.leading_zeros(),
        },
        r_bits: 64 - curve.l.leading_zeros(),
    };
    let mut total: BTreeMap<String, u64> = accounting::KINDS
        .into_iter()
        .map(|k| (k.into(), 0))
        .collect();
    let mut bitops = 0u128;
    let phases = work["phases"].as_array().ok_or("phases")?;
    ensure(
        phases.len() == accounting::PHASES.len(),
        "phase cardinality",
    )?;
    for (row, name) in phases.iter().zip(accounting::PHASES) {
        ensure(row["phase"] == name, "phase order")?;
        let counts = accounting::counts(&row["primitives"])?;
        let price = accounting::price(&counts, &widths)?;
        ensure(
            row["diagnostic_priced_bitops"] == price.to_string(),
            "finite price",
        )?;
        bitops += price;
        for (k, v) in counts {
            let out = total.get_mut(&k).ok_or("primitive")?;
            *out = out.checked_add(v).ok_or("count overflow")?;
        }
        if name == "linear_algebra" {
            ensure(
                if sparse.is_some() {
                    accounting::counts(&row["primitives"])? == sparse_primitives
                } else {
                    row["primitives"]["modr_mul"] == c.products
                        && row["primitives"]["modr_add"] == c.subtractions
                        && row["primitives"]["modr_inv"] == c.rank
                },
                "executed rank arithmetic counts",
            )?;
        }
    }
    ensure(
        accounting::counts(&work["observed_primitives"])? == total,
        "phase primitive sum",
    )?;
    let uniform = &work["uniform_sampling"];
    let calls = accounting::n(uniform, "calls")?;
    let draws = accounting::n(uniform, "raw_draws")?;
    let rejections = accounting::n(uniform, "rejections")?;
    ensure(
        uniform["schema"] == "modular-uniform-events/v1"
            && draws == calls + rejections
            && draws <= total["rng_words"]
            && uniform["complete"] == false
            && uniform["other_modulus_calls"] == 0,
        "sampling events",
    )?;
    ensure(
        uniform["priced_native"]["integer_mul64"] == draws
            && uniform["priced_native"]["integer_add64"] == calls
            && uniform["known_native_priced_bitops"]
                == (draws as u128 * accounting::bitop_mul64()
                    + calls as u128 * accounting::bitop_add64())
                .to_string(),
        "sampling known price",
    )?;
    ensure(
        uniform["unpriced_native"]["integer_sub64"] == 2 * calls
            && uniform["unpriced_native"]["integer_clz64"] == calls
            && uniform["unpriced_native"]["integer_shl64"] == calls,
        "sampling explicit unpriced events",
    )?;
    let raw_screen = sparse
        .ok_or("registered sparse evidence")?
        .get("discriminant_screen");
    ensure(
        raw_screen.is_some() == (record.mode == "discriminant-quadratic-pairs"),
        "screen evidence/mode binding",
    )?;
    let enumeration =
        crate::enumeration_math_check::qualify(record, &ctx, &setup, &domain, raw_screen)?;
    Ok(
        json!({"mode":record.mode,"counts":c,"rank_target":record.rank_target,"stop":record.stop,"verified_originals":record.sources.len(),"verified_projections":record.projections.len(),"every_rank_product_prefix_checked":sparse.is_none(),"every_sparse_checkpoint_checked":sparse.is_some(),"sparse_checkpoint_proofs":sparse_proofs,"independent_cantor_replay":true,"enumeration":enumeration,"observed_incomplete_bitops":bitops.to_string(),"recorded_inner_wall_ns_practicality_only":record.inner_wall_ns,"complete_cost":false,"total_bitops":null,"ratios":null}),
    )
}

pub fn validate_public(curve: &driver::PublicCurve) -> Result<(), String> {
    ensure(
        (101..=1511).contains(&curve.p)
            && (3..1u64 << 63).contains(&curve.l)
            && accounting::prime(curve.p)
            && accounting::prime(curve.l)
            && !curve.g.inf
            && !curve.q.inf,
        "public prime domain",
    )?;
    ensure(
        curve
            .alpha
            .iter()
            .flatten()
            .chain(curve.g.x.iter().flatten())
            .chain(curve.g.y.iter().flatten())
            .chain(curve.q.x.iter().flatten())
            .chain(curve.q.y.iter().flatten())
            .all(|&x| x < curve.p),
        "public canonical coordinates",
    )?;
    let f = cover::Fq3::new(curve.p);
    let ec = cover::EllE::new(&f, &E6(curve.alpha.map(E2)));
    let (g, q) = (curve.g.native(), curve.q.native());
    ensure(
        ec.on_curve(&g) && ec.on_curve(&q) && ec.mul(&g, curve.l).inf && ec.mul(&q, curve.l).inf,
        "public subgroup",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::jv_sieve::two_large_prime::CountedRelationSystem;
    #[test]
    fn independent_rank_and_actual_product_prefixes_match_wide_field() {
        for p in [101, 263807419864521289] {
            let mut oracle = Rank::default();
            let mut actual = CountedRelationSystem::new(p, 4);
            for row in [
                vec![1, 2, 0, 4],
                vec![0, 1, 1, 0],
                vec![3, 7, 1, 12],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
            ] {
                oracle.insert(row.clone(), p).unwrap();
                actual.insert(row, 0);
                assert_eq!(oracle.basis.len(), actual.rank());
                assert_eq!(oracle.products, actual.ops());
            }
        }
    }
    #[test]
    fn noncanonical_rank_input_refused() {
        assert!(Rank::default().insert(vec![101], 101).is_err());
    }
    #[test]
    fn independent_principal_source_binding_accepts_fixture_and_refuses_forgery() {
        let (f, h, base, a, c, norm, terms, tail) = remainder::tests::hand();
        let old = remainder::check_equation(&f, &h, &base, &a, &c, &norm, &terms, &tail)
            .unwrap()
            .into_certificate();
        let certificate = crate::quadratic_pair_relation::Certificate {
            terms: old.terms,
            remainders: old
                .remainder
                .into_iter()
                .map(|q| crate::quadratic_pair_relation::QuadraticClass {
                    key: q.key,
                    coefficient: q.coefficient,
                })
                .collect(),
            a: old.a,
            c: old.c,
            norm: old.norm,
        };
        let domain = crate::pair_math_check::Domain::new(
            f.p,
            f.w,
            remainder::encode(&h),
            base.iter().map(|b| (b.x, b.y.0)).collect(),
        )
        .unwrap();
        let source = Source {
            candidate_index: 1,
            b_index: 0,
            a0: a.iter().map(|x| x.0[0]).collect(),
            a1: a.iter().map(|x| x.0[1]).collect(),
            b: remainder::encode(&c),
            s: 1,
            certificate,
            graph: json!({"kind":"ungrounded"}),
        };
        let jac = Hyp { f: &f, h, g: 3 };
        assert!(source_proof(&jac, &base, &source, &c, &domain).is_ok());
        let mut bad = source.clone();
        bad.a0[0] = (bad.a0[0] + 1) % f.p;
        assert!(source_proof(&jac, &base, &bad, &c, &domain).is_err());
        let mut bad = source.clone();
        bad.b[0][0] = 1;
        assert!(source_proof(&jac, &base, &bad, &c, &domain).is_err());
        let mut bad = source;
        bad.certificate.norm[0] = (bad.certificate.norm[0] + 1) % f.p;
        assert!(source_proof(&jac, &base, &bad, &c, &domain).is_err());
    }
    #[test]
    fn independent_group_check_refuses_forged_remainder_and_small_signs() {
        let (f, h, base, a, c, norm, terms, tail) = remainder::tests::hand();
        let row = remainder::check_equation(&f, &h, &base, &a, &c, &norm, &terms, &tail)
            .unwrap()
            .into_certificate();
        let jac = cover::Hyp { f: &f, h, g: 3 };
        assert!(remainder::replay_group(&jac, &base, &row));
        let mut bad = row.clone();
        bad.remainder.as_mut().unwrap().key[2] = (bad.remainder.as_ref().unwrap().key[2] + 1) % f.p;
        assert!(!remainder::replay_group(&jac, &base, &bad));
        let mut bad = row.clone();
        bad.remainder.as_mut().unwrap().coefficient *= -1;
        assert!(!remainder::replay_group(&jac, &base, &bad));
        let mut bad = row.clone();
        bad.terms[0].1 *= -1;
        assert!(!remainder::replay_group(&jac, &base, &bad));
        let mut bad = row.clone();
        bad.terms[1].0 = bad.terms[0].0;
        assert!(!remainder::replay_group(&jac, &base, &bad));
        let mut bad = row;
        bad.remainder.as_mut().unwrap().key[0] = f.p;
        assert!(!remainder::replay_group(&jac, &base, &bad));
    }
    #[test]
    fn bad_domain_and_unknown_input_fields_refused() {
        let text = r#"{"p":1009,"l":263807419864521289,"alpha":[[0,0],[0,0],[0,0]],"g":{"x":[[0,0],[0,0],[0,0]],"y":[[0,0],[0,0],[0,0]],"inf":false},"q":{"x":[[0,0],[0,0],[0,0]],"y":[[0,0],[0,0],[0,0]],"inf":false},"expected_scalar":1}"#;
        assert!(serde_json::from_str::<driver::PublicCurve>(text).is_err());
    }
}
