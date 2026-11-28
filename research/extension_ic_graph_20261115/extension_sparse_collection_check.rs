//! Independent Cantor replay and column-keyed u128 rank/counter qualification.
//! This does not call the producer's certificate constructor or graph filter.
use crate::{
    driver::{self, Record, Source},
    remainder,
};
#[allow(dead_code)]
#[path = "native_analysis/src/bin/two-lp-component-analysis.rs"]
pub(crate) mod accounting;
use accounting::{add, mul, pow};
use crypto_lib::cryptanalysis::{
    jv_cover::{self as cover, BaseEl, Ctx, Fld, Fq, Hyp, E2, E6},
    jv_sieve::{self as sieve, SieveSetup},
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

fn source_proof(
    jac: &Hyp<Fq>,
    setup: &SieveSetup,
    base: &[BaseEl],
    source: &Source,
    mode: &str,
    expected_b: &[E2],
) -> Result<(), String> {
    let f = jac.f;
    let p = f.p;
    let row = &source.certificate;
    ensure(
        source.s > 0
            && source.s < p
            && !source.a0.is_empty()
            && source.a0.len() <= 5
            && source.a0.last() == Some(&1)
            && source.a1.len() <= 5
            && source.b.len() == 2,
        "source degrees/parameter",
    )?;
    ensure(
        source.a0.iter().chain(&source.a1).all(|&v| v < p)
            && source.b.iter().flatten().all(|&v| v < p),
        "source canonical coordinates",
    )?;
    let b = remainder::decode(&source.b);
    ensure(b == expected_b, "enumerated B binding")?;
    let b2h = remainder::mul(f, &remainder::mul(f, &b, &b), &jac.h);
    let twice = remainder::mul(
        f,
        &remainder::embed(&source.a0),
        &remainder::embed(&source.a1),
    )
    .iter()
    .map(|x| f.scale(x, 2))
    .collect::<Vec<_>>();
    let im = remainder::trim(b2h.iter().map(|x| E2([x.0[1], 0])).collect());
    ensure(twice == im, "line imaginary constraint")?;
    let sq = if setup.sqrt[source.s as usize] != u64::MAX {
        E2([setup.sqrt[source.s as usize], 0])
    } else {
        let root = setup.sqrt[mul(source.s, setup.winv, p) as usize];
        ensure(root != u64::MAX, "sqrt")?;
        E2([0, root])
    };
    let a = remainder::trim(
        (0..5)
            .map(|i| {
                E2([
                    *source.a0.get(i).unwrap_or(&0),
                    mul(source.s, *source.a1.get(i).unwrap_or(&0), p),
                ])
            })
            .collect(),
    );
    let c = remainder::trim(b.iter().map(|x| f.mul(&sq, x)).collect());
    let norm = remainder::sub(
        f,
        &remainder::mul(f, &a, &a),
        &remainder::mul(f, &remainder::mul(f, &c, &c), &jac.h),
    );
    ensure(
        norm.len() == 10 && norm.iter().all(|x| x.in_fp()),
        "norm degree/base field",
    )?;
    if mode == "full-smooth" {
        ensure(
            row.a.is_empty() && row.c.is_empty() && row.norm.is_empty() && row.remainder.is_none(),
            "baseline shape",
        )?;
    } else {
        ensure(
            row.a == remainder::encode(&a)
                && row.c == remainder::encode(&c)
                && row.norm == norm.iter().map(|x| x.0[0]).collect::<Vec<_>>(),
            "candidate exact principal binding",
        )?;
    }
    let mut product = vec![E2::ONE];
    ensure(
        row.terms.windows(2).all(|w| w[0].0 < w[1].0),
        "support ordering",
    )?;
    for &(i, sign) in &row.terms {
        let point = base.get(i).ok_or("point index")?;
        ensure([-1, 1].contains(&sign), "point sign")?;
        let y = if sign == 1 { point.y } else { f.neg(&point.y) };
        let den = remainder::eval(f, &c, point.x);
        ensure(
            den != E2::ZERO
                && f.add(&remainder::eval(f, &a, point.x), &f.mul(&den, &y)) == E2::ZERO,
            "principal small ordinate",
        )?;
        product = remainder::mul(f, &product, &[f.neg(&f.from_fp(point.x)), E2::ONE]);
    }
    if let Some(r) = &row.remainder {
        ensure(
            row.terms.len() == 7
                && [-1, 1].contains(&r.coefficient)
                && r.key.iter().all(|&x| x < p),
            "remainder shape",
        )?;
        let neg = [
            r.key[0],
            r.key[1],
            (p - r.key[2]) % p,
            (p - r.key[3]) % p,
            (p - r.key[4]) % p,
            (p - r.key[5]) % p,
        ];
        ensure(r.key <= neg, "remainder canonical orientation")?;
        let discriminant = add(mul(r.key[1], r.key[1], p), p - mul(4, r.key[0], p), p);
        ensure(
            discriminant != 0 && pow(discriminant, (p - 1) / 2, p) == p - 1,
            "irreducible quadratic",
        )?;
        let d = r.divisor();
        let resultant = f.add(
            &f.sub(
                &f.mul(&c[0], &c[0]),
                &f.scale(&f.mul(&c[0], &c[1]), r.key[1]),
            ),
            &f.scale(&f.mul(&c[1], &c[1]), r.key[0]),
        );
        ensure(resultant != E2::ZERO, "independent denominator resultant")?;
        let v = if r.coefficient == 1 {
            d.v.clone()
        } else {
            d.v.iter().map(|x| f.neg(x)).collect()
        };
        ensure(
            remainder::modulo(
                f,
                &remainder::sub(
                    f,
                    &a,
                    &remainder::mul(f, &c, &v)
                        .iter()
                        .map(|x| f.neg(x))
                        .collect::<Vec<_>>(),
                ),
                &d.u,
            )?
            .is_empty(),
            "principal remainder ordinate",
        )?;
        product = remainder::mul(f, &product, &d.u);
    } else {
        ensure(row.terms.len() == 9, "full support cardinality")?;
    }
    ensure(
        remainder::mul(f, &product, &[*norm.last().ok_or("norm leading")?]) == norm,
        "exact squarefree support",
    )?;
    // Different mathematical backend from norm/support construction.
    ensure(
        remainder::replay_group(jac, base, row),
        "independent Cantor identity",
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
        ["full-smooth", "quadratic-remainders"].contains(&record.mode.as_str()),
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
    let mut seen = BTreeSet::new();
    let mut anchors = BTreeMap::new();
    let mut expected = Vec::new();
    let mut partials = 0;
    let mut full = 0;
    for (id, s) in record.sources.iter().enumerate() {
        ensure(
            s.b_index < c.b_count || record.stop == "source resource cap" && s.b_index == c.b_count,
            "source B range",
        )?;
        let b = sieve::b_from_index(&setup, s.b_index as u128, scramble);
        source_proof(&ctx.jac(), &setup, &base, s, &record.mode, &b)?;
        let row = &s.certificate;
        let orient = row
            .remainder
            .as_ref()
            .map_or(row.terms[0].1, |r| r.coefficient);
        let key = (
            row.terms
                .iter()
                .map(|&(i, c)| (i, c * orient))
                .collect::<Vec<_>>(),
            row.remainder.as_ref().map(|r| r.key),
        );
        ensure(seen.insert(key), "duplicate original relation")?;
        if let Some(r) = &row.remainder {
            partials += 1;
            if let Some(&old) = anchors.get(&r.key) {
                let old: usize = old;
                let c0 = record.sources[old]
                    .certificate
                    .remainder
                    .as_ref()
                    .ok_or("anchor")?
                    .coefficient;
                expected.push(vec![(old, -r.coefficient), (id, c0)]);
            } else {
                anchors.insert(r.key, id);
            }
        } else {
            full += 1;
            expected.push(vec![(id, 1)]);
        }
    }
    ensure(
        expected.len() == record.projections.len(),
        "projection completeness",
    )?;
    let mut rank = Rank::default();
    let mut prefix_ranks = vec![0usize];
    for (projection, witness) in record.projections.iter().zip(&expected) {
        ensure(&projection.witness == witness, "anchor witness")?;
        let mut small = BTreeMap::new();
        let mut large = BTreeMap::new();
        for &(id, coefficient) in witness {
            let row = &record.sources[id].certificate;
            for &(i, c) in &row.terms {
                let entry = small.entry(i).or_insert(0);
                *entry = add(*entry, driver::signed(c * coefficient, curve.l), curve.l);
            }
            if let Some(r) = &row.remainder {
                *large.entry(r.key).or_insert(0i64) += (r.coefficient * coefficient) as i64;
            }
        }
        ensure(large.values().all(|&v| v == 0), "remainder cancellation")?;
        let terms: Vec<_> = small.into_iter().filter(|&(_, v)| v != 0).collect();
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
            && c.vertices == anchors.len()
            && c.collisions as usize == record.projections.len() - full as usize,
        "source/graph counts",
    )?;
    ensure(
        c.candidates
            == c.refused
                + c.duplicates
                + record.sources.len() as u64
                + u64::from(record.stop == "source resource cap"),
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
    Ok(
        json!({"mode":record.mode,"counts":c,"rank_target":record.rank_target,"stop":record.stop,"verified_originals":record.sources.len(),"verified_projections":record.projections.len(),"every_rank_product_prefix_checked":sparse.is_none(),"every_sparse_checkpoint_checked":sparse.is_some(),"sparse_checkpoint_proofs":sparse_proofs,"independent_cantor_replay":true,"observed_incomplete_bitops":bitops.to_string(),"recorded_inner_wall_ns_practicality_only":record.inner_wall_ns,"complete_cost":false,"total_bitops":null,"ratios":null}),
    )
}

pub fn pair(a: &Record, b: &Record, seed: u64, input_sha: &str) -> Result<Value, String> {
    ensure(
        a.mode == "full-smooth"
            && b.mode == "quadratic-remainders"
            && a.source_revision == b.source_revision
            && a.base == b.base
            && a.h == b.h
            && serde_json::to_value(&a.public).map_err(|e| e.to_string())?
                == serde_json::to_value(&b.public).map_err(|e| e.to_string())?,
        "paired model identity",
    )?;
    let mut shared = 0;
    for (x, y) in a.checkpoints.iter().zip(&b.checkpoints) {
        ensure(
            x.b_count == y.b_count && x.lines == y.lines && x.cohort_sha256 == y.cohort_sha256,
            "common cohort prefix",
        )?;
        shared += 1;
    }
    ensure(shared > 0, "missing paired complete cohort")?;
    let outcomes = [qualify(a, seed, input_sha)?, qualify(b, seed, input_sha)?];
    let common_b = a.checkpoints[shared - 1].b_count;
    let full_key = |r: &Record| {
        r.sources
            .iter()
            .filter(|s| s.b_index < common_b && s.certificate.remainder.is_none())
            .map(|s| {
                let sign = s.certificate.terms[0].1;
                (
                    s.b_index,
                    s.certificate
                        .terms
                        .iter()
                        .map(|&(i, e)| (i, e * sign))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<BTreeSet<_>>()
    };
    ensure(
        full_key(a) == full_key(b),
        "common-prefix full-smooth source equality",
    )?;
    Ok(
        json!({"schema":"cover.extension-pipeline.collection-analysis/v1","source_revision":a.source_revision,"outcomes":outcomes,"common_complete_batch_prefixes":shared,"flow":{"planned":2,"attempted":2,"completed":2,"excluded":0,"automatic_retries":0},"component_only":true,"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"rho_parity":false}),
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
        let certificate = remainder::check_equation(&f, &h, &base, &a, &c, &norm, &terms, &tail)
            .unwrap()
            .into_certificate();
        let setup = sieve::sieve_setup(&f, &h, &base, 9);
        let source = Source {
            b_index: 0,
            a0: a.iter().map(|x| x.0[0]).collect(),
            a1: a.iter().map(|x| x.0[1]).collect(),
            b: remainder::encode(&c),
            s: 1,
            certificate,
        };
        let jac = Hyp { f: &f, h, g: 3 };
        assert!(source_proof(&jac, &setup, &base, &source, "quadratic-remainders", &c).is_ok());
        let mut bad = source.clone();
        bad.a0[0] = (bad.a0[0] + 1) % f.p;
        assert!(source_proof(&jac, &setup, &base, &bad, "quadratic-remainders", &c).is_err());
        let mut bad = source.clone();
        bad.b[0][0] = 1;
        assert!(source_proof(&jac, &setup, &base, &bad, "quadratic-remainders", &c).is_err());
        let mut bad = source;
        bad.certificate.norm[0] = (bad.certificate.norm[0] + 1) % f.p;
        assert!(source_proof(&jac, &setup, &base, &bad, "quadratic-remainders", &c).is_err());
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
