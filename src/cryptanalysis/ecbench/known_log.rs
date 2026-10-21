//! A target-blind known-log factor base and a two-summand recovery experiment.
//!
//! Preparation sees only the generator. With H = [u]G it samples distinct
//! signed pairs ±[s_i]H, paying for every scalar multiplication, and builds
//! the existing negation-folded pair table. Search probes R = Q + [a]H.
//! A checked witness R = F_i + F_j immediately gives
//! log_G(Q) = u (s_i + s_j - a) mod r. No unknown base logs or matrix remain.
//!
//! This is a generic table/search trade, not a complexity claim. The table,
//! failed queries, recovery and internal verification all have separate
//! ledgers; the ecbench runner independently verifies the returned answer.

use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::cryptanalysis::ic_boundary::{
    addmod, mulmod, CountedGroup, FactorBase, GroupOps, PairTable,
};
use crate::cryptanalysis::ic_measurement::{self as measurement, Phase};

#[derive(Clone, Copy, Debug)]
pub struct Parameters {
    /// Number of unsigned points; the signed base has twice this size.
    pub size: usize,
    pub base_seed: u64,
    /// H = [base_multiplier]G. Allows a transported-base generator control.
    pub base_multiplier: u64,
    /// Bound on pair-construction attempts, checked before allocating a base.
    pub max_pair_work: u64,
}

/// Reusable preparation. There is deliberately no target in its constructor.
pub struct Prepared<E> {
    pub base: FactorBase<E>,
    pub base_ops: GroupOps,
    pub table_ops: GroupOps,
    pub base_wall_ns: u64,
    pub table_wall_ns: u64,
    pub counters: BTreeMap<String, u64>,
    anchor: E,
    generator: E,
    logs: Vec<u64>,
    r: u64,
    multiplier: u64,
    table: PairTable,
}

#[derive(Clone, Debug, Default)]
pub struct Recovery {
    pub recovered: Option<u64>,
    pub query_ops: GroupOps,
    pub decomposition_ops: GroupOps,
    pub relation_ops: GroupOps,
    pub recovery_ops: GroupOps,
    pub verify_ops: GroupOps,
    pub counters: BTreeMap<String, u64>,
}

fn count(counters: &mut BTreeMap<String, u64>, name: &str, n: u64) {
    *counters.entry(name.into()).or_default() += n;
}

pub fn prepare<G: CountedGroup>(
    g: &G,
    generator: G::Elt,
    r: u64,
    params: Parameters,
) -> Result<Prepared<G::Elt>, String> {
    let size = u64::try_from(params.size).map_err(|_| "base size does not fit u64")?;
    if r < 3 || size == 0 || size > (r - 1) / 2 {
        return Err("known-log size must lie in 1..=(r-1)/2 for an odd prime r".into());
    }
    if params.base_multiplier == 0 || params.base_multiplier >= r {
        return Err("known-log base_multiplier must lie in 1..r".into());
    }
    if g.is_identity(&generator) {
        return Err("known-log generator must be nonidentity".into());
    }
    let pair_work = size
        .checked_add(1)
        .and_then(|next| size.checked_mul(next))
        .ok_or("known-log pair work overflows u64")?;
    if pair_work > params.max_pair_work {
        return Err(format!(
            "known-log pair construction needs {pair_work} attempts, above max_pair_work={}",
            params.max_pair_work
        ));
    }
    // PairTable stores witness indices as u32.
    if size > u64::from(u32::MAX) / 2 {
        return Err("known-log signed base exceeds the pair-table index width".into());
    }

    measurement::mark(Phase::FactorBase);
    let started = Instant::now();
    let mut ops = GroupOps::default();
    let anchor = if params.base_multiplier == 1 {
        generator
    } else {
        g.mul(&mut ops, generator, params.base_multiplier)
    };
    let mut rng = StdRng::seed_from_u64(params.base_seed);
    let mut used = BTreeSet::new();
    let mut points = Vec::with_capacity(params.size * 2);
    let mut logs = Vec::with_capacity(params.size * 2);
    let mut columns = Vec::with_capacity(params.size * 2);
    let mut coefficients = Vec::with_capacity(params.size * 2);
    let mut counters = BTreeMap::new();
    while used.len() < params.size {
        let scalar = rng.gen_range(1..r);
        let scalar = scalar.min(r - scalar);
        count(&mut counters, "base_scalar_draws_uncharged", 1);
        count(&mut counters, "base_dedup_probes_uncharged", 1);
        if !used.insert(scalar) {
            count(&mut counters, "base_duplicate_draws", 1);
            continue;
        }
        let column = points.len() / 2;
        let point = g.mul(&mut ops, anchor, scalar);
        points.extend([point, g.neg(point)]);
        logs.extend([scalar, r - scalar]);
        columns.extend([column, column]);
        coefficients.extend([1, r - 1]);
    }
    let base = FactorBase::from_column_map(
        format!("{size} seeded known-log signed pairs; target-blind scalar construction"),
        points,
        columns,
        coefficients,
        params.size,
        |p| g.key(p),
        |p| g.key(&g.neg(*p)),
        |p| g.key(p) >> 1,
    )?;
    count(&mut counters, "base_index_entries_uncharged", size * 2);
    count(&mut counters, "base_negations_uncharged", size * 3);
    let base_wall_ns = started.elapsed().as_nanos() as u64;

    measurement::mark(Phase::Precompute);
    let table = PairTable::build_negation_folded(g, &base);
    // Exactly one inverse pair per signed pair maps to O and is skipped.
    // The remaining size^2 attempts cover ± sums; entries are distinct classes.
    count(&mut counters, "pair_attempts", pair_work);
    count(&mut counters, "pair_identity_sums", size);
    count(&mut counters, "pair_distinct_signed_classes", table.entries);
    count(
        &mut counters,
        "pair_colliding_attempts",
        size * size - table.entries,
    );
    count(&mut counters, "table_inserts_uncharged", size * size);
    count(&mut counters, "unknown_base_logs", 0);
    Ok(Prepared {
        base,
        base_ops: ops,
        table_ops: table.build_ops,
        base_wall_ns,
        table_wall_ns: table.build_wall_ns,
        counters,
        anchor,
        generator,
        logs,
        r,
        multiplier: params.base_multiplier,
        table,
    })
}

/// One arbitrary target; the prepared base/table may be reused explicitly.
/// Queries never repeat: a advances by one and stops after at most r trials.
/// Neither the planted answer nor another target's search state is accepted.
pub fn recover<G: CountedGroup>(
    g: &G,
    prepared: &Prepared<G::Elt>,
    target: G::Elt,
    query_seed: u64,
    max_trials: u64,
) -> Result<Recovery, String> {
    if max_trials == 0 {
        return Err("known-log max_trials must be positive".into());
    }
    measurement::begin_online(Phase::TargetQuery);
    let result = (|| {
        let mut out = Recovery::default();
        let r = prepared.r;
        let mut rng = StdRng::seed_from_u64(query_seed);
        let mut a = rng.gen_range(0..r);
        count(&mut out.counters, "query_scalar_draws_uncharged", 1);
        let offset = g.mul(&mut out.query_ops, prepared.anchor, a);
        let mut query = g.add(&mut out.query_ops, target, offset);
        for trial in 0..max_trials.min(r) {
            count(&mut out.counters, "targets_tried", 1);
            measurement::mark(Phase::TargetPdp);
            let witness = if g.is_identity(&query) {
                count(&mut out.counters, "identity_relations", 1);
                Some((None, 0))
            } else {
                let mut probes = 0;
                let found = prepared.table.probe(
                    g,
                    &prepared.base,
                    &mut out.decomposition_ops,
                    &mut probes,
                    query,
                );
                count(&mut out.counters, "lookups_uncharged", probes);
                found.map(|(i, j)| (Some((i, j)), addmod(prepared.logs[i], prepared.logs[j], r)))
            };
            if let Some((indices, sum_log)) = witness {
                measurement::mark(Phase::TargetRelationCheck);
                if let Some((i, j)) = indices {
                    let sum = g.add(
                        &mut out.relation_ops,
                        prepared.base.points[i],
                        prepared.base.points[j],
                    );
                    if sum != query {
                        return Err("known-log pair witness failed its point check".into());
                    }
                    count(&mut out.counters, "relation_point_checks", 1);
                    count(&mut out.counters, "scalar_additions_uncharged", 1);
                }
                measurement::mark(Phase::TargetDescent);
                let log_anchor = addmod(sum_log, r - a, r);
                let answer = mulmod(prepared.multiplier, log_anchor, r);
                count(&mut out.counters, "scalar_additions_uncharged", 1);
                count(&mut out.counters, "scalar_multiplications_uncharged", 1);
                count(&mut out.counters, "relations_found", 1);
                count(&mut out.counters, "useful_target_relations", 1);
                count(&mut out.counters, "target_equation_rank", 1);
                measurement::mark(Phase::RecoveryCheck);
                if g.mul(&mut out.verify_ops, prepared.generator, answer) != target {
                    return Err("known-log scalar failed internal point verification".into());
                }
                out.recovered = Some(answer);
                return Ok(out);
            }
            if trial + 1 < max_trials.min(r) {
                measurement::mark(Phase::TargetQuery);
                query = g.add(&mut out.query_ops, query, prepared.anchor);
                a = addmod(a, 1, r);
                count(&mut out.counters, "scalar_additions_uncharged", 1);
            }
        }
        Ok(out)
    })();
    measurement::end_online();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::{
        invmod, koblitz_instance, roster_prime_instance, BinaryGroup,
    };

    fn parameters() -> Parameters {
        Parameters {
            size: 4,
            base_seed: 20261021,
            base_multiplier: 1,
            max_pair_work: 20,
        }
    }

    fn every_target<G: CountedGroup>(g: &G, generator: G::Elt, r: u64) {
        let prepared = prepare(g, generator, r, parameters()).unwrap();
        assert_eq!(prepared.counters["pair_attempts"], 20);
        assert_eq!(prepared.base.points.len(), 8);
        assert_eq!(prepared.base_ops.scalar_mults, 4);
        for k in 1..r {
            let target = g.mul(&mut GroupOps::default(), generator, k);
            let out = recover(g, &prepared, target, 7, r).unwrap();
            assert_eq!(out.recovered, Some(k));
            assert_eq!(out.counters["target_equation_rank"], 1);
            assert_eq!(out.counters["useful_target_relations"], 1);
            assert!(out.counters["targets_tried"] <= r);
        }
    }

    #[test]
    fn recovers_every_nonzero_prime_and_koblitz_log_without_answer_input() {
        let prime = roster_prime_instance(10).unwrap();
        every_target(&prime.curve, prime.generator_point(), prime.r);
        let binary = koblitz_instance(0, 7).unwrap();
        every_target(&BinaryGroup(&binary.fast), binary.generator, binary.r);
    }

    #[test]
    fn generator_transport_keeps_the_base_and_query_trajectory_fixed() {
        let i = roster_prime_instance(10).unwrap();
        let g = &i.curve;
        let original = i.generator_point();
        let changed = g.mul(&mut GroupOps::default(), original, 7);
        let inverse = invmod(7, i.r).unwrap();
        let a = prepare(g, original, i.r, parameters()).unwrap();
        let b = prepare(
            g,
            changed,
            i.r,
            Parameters {
                base_multiplier: inverse,
                ..parameters()
            },
        )
        .unwrap();
        assert_eq!(a.base.points, b.base.points);
        assert_eq!(a.logs, b.logs);
        assert_eq!(a.table.entries, b.table.entries);
        for k in [1, 17, i.r - 1] {
            let q = g.mul(&mut GroupOps::default(), original, k);
            let ar = recover(g, &a, q, 31, i.r).unwrap();
            let br = recover(g, &b, q, 31, i.r).unwrap();
            assert_eq!(ar.recovered, Some(k));
            assert_eq!(br.recovered, Some(mulmod(k, inverse, i.r)));
            assert_eq!(ar.counters, br.counters);
            assert_eq!(ar.query_ops, br.query_ops);
            assert!(b.base_ops.gae() > a.base_ops.gae());
        }
    }

    #[test]
    fn deterministic_counts_and_hard_trial_limit() {
        let i = roster_prime_instance(10).unwrap();
        let g = &i.curve;
        let prepared = prepare(g, i.generator_point(), i.r, parameters()).unwrap();
        let mut saw_exhaustion = false;
        for k in 1..i.r {
            let q = g.mul(&mut GroupOps::default(), i.generator_point(), k);
            let a = recover(g, &prepared, q, 7, 1).unwrap();
            let b = recover(g, &prepared, q, 7, 1).unwrap();
            assert_eq!(a.recovered, b.recovered);
            assert_eq!(a.counters, b.counters);
            assert_eq!(a.query_ops, b.query_ops);
            assert_eq!(a.counters["targets_tried"], 1);
            if a.recovered.is_none() {
                saw_exhaustion = true;
                assert_eq!(a.verify_ops.scalar_mults, 0);
                break;
            }
        }
        assert!(saw_exhaustion);
        assert!(recover(g, &prepared, i.generator_point(), 7, 0).is_err());
    }

    #[test]
    fn refuses_invalid_sizes_multipliers_and_pair_budgets_before_work() {
        let i = roster_prime_instance(10).unwrap();
        for params in [
            Parameters {
                size: 0,
                ..parameters()
            },
            Parameters {
                size: i.r as usize,
                ..parameters()
            },
            Parameters {
                base_multiplier: 0,
                ..parameters()
            },
            Parameters {
                base_multiplier: i.r,
                ..parameters()
            },
            Parameters {
                max_pair_work: 19,
                ..parameters()
            },
        ] {
            assert!(prepare(&i.curve, i.generator_point(), i.r, params).is_err());
        }
    }
}
