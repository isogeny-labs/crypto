//! Cold, target-specialized m=7 PKM/Semaev elimination over a p+1 Dickson
//! tower, with an exact affine dynamic-programming control.
//!
//! This is the finite correctness ladder frozen in
//! `research/pplus_dickson_m7_elimination_20261127/PROTOCOL.md`. Every target
//! receives a fresh factor base and oracle. No trace, learned basis, or state
//! is shared across targets.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use crypto_lib::cryptanalysis::ic_boundary::{
    CountedGroup, FactorBase, GroupOps, OracleCounters, PrimeCurve, PrimeInstance, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::pkm_semaev::{dickson_factor_base, PkmSemaevOracle};
use crypto_lib::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};
use serde_json::{json, Value};

const ARITY: usize = 7;

#[derive(Clone)]
struct Fixture {
    label: &'static str,
    instance: PrimeInstance,
    depth: u32,
}

fn f7_depth1() -> Fixture {
    Fixture {
        label: "f7-depth1-full-carrier",
        instance: PrimeInstance {
            name: "m7-f7-d1".into(),
            curve: PrimeCurve { p: 7, a: 0, b: 3 },
            group_order: 13,
            r: 13,
            cofactor: 1,
            generator: (4, 2),
            representation: None,
        },
        depth: 1,
    }
}

fn f31_depth2_sparse() -> Fixture {
    Fixture {
        label: "f31-depth2-sparse-carrier",
        instance: PrimeInstance {
            name: "m7-f31-d2".into(),
            curve: PrimeCurve { p: 31, a: 0, b: 12 },
            group_order: 43,
            r: 43,
            cofactor: 1,
            generator: (2, 12),
            representation: None,
        },
        depth: 2,
    }
}

fn context(instance: &PrimeInstance) -> InstanceCtx<'_, PrimeCurve> {
    InstanceCtx {
        group: &instance.curve,
        generator: instance.generator_point(),
        target: instance.generator_point(),
        r: instance.r,
        cofactor: instance.cofactor,
        group_order: instance.group_order,
        name: instance.name.clone(),
        field_degree: None,
    }
}

fn sum_indices(curve: &PrimeCurve, base: &FactorBase<PrimePoint>, indices: &[usize]) -> PrimePoint {
    indices.iter().fold(curve.identity(), |sum, &index| {
        curve.add(&mut GroupOps::default(), sum, base.points[index])
    })
}

/// Exact seven-level affine enumeration with duplicate group states merged.
/// Merging is lossless for the existence predicate and retains one complete
/// witness for every reachable public point.
fn exhaustive_affine(
    curve: &PrimeCurve,
    base: &FactorBase<PrimePoint>,
) -> HashMap<u64, Vec<usize>> {
    let mut layer = HashMap::from([(curve.key(&curve.identity()), Vec::new())]);
    for _ in 0..ARITY {
        let mut next = HashMap::new();
        for (key, witness) in layer {
            let point = if key == curve.key(&curve.identity()) {
                curve.identity()
            } else {
                sum_indices(curve, base, &witness)
            };
            for (index, &leaf) in base.points.iter().enumerate() {
                let sum = curve.add(&mut GroupOps::default(), point, leaf);
                next.entry(curve.key(&sum)).or_insert_with(|| {
                    let mut extended = witness.clone();
                    extended.push(index);
                    extended
                });
            }
        }
        layer = next;
    }
    layer
}

fn point_json(point: PrimePoint) -> Value {
    json!({"x":point.x,"y":point.y,"infinity":point.infinity})
}

fn cold_target(
    fixture: &Fixture,
    target: PrimePoint,
    expected: Option<&[usize]>,
    budget_seconds: u64,
) -> Value {
    let cold_started = Instant::now();
    let base = dickson_factor_base(&fixture.instance, fixture.depth, 0, 0)
        .expect("production Dickson factor base");
    let setup_started = Instant::now();
    let (_, params) = Params::parse_spec(
        "pkm-semaev:m=7,mode=chain,engine=f4,descent=1,grid_crossover=0,trace=0,degree=12",
    )
    .expect("fixed PKM parameters");
    let mut oracle = PkmSemaevOracle::new_dickson(ARITY as u32, fixture.depth, 0, 0)
        .with_budget(Some(Duration::from_secs(budget_seconds)));
    let mut setup_ops = GroupOps::default();
    oracle
        .prepare(&context(&fixture.instance), &base, &params, &mut setup_ops)
        .expect("prepare cold m=7 oracle");
    let setup_elapsed = setup_started.elapsed();

    let query_started = Instant::now();
    let mut query_ops = GroupOps::default();
    let mut counters = OracleCounters::default();
    let got = oracle.decompose(
        &context(&fixture.instance),
        &base,
        &mut query_ops,
        &mut counters,
        target,
    );
    let query_elapsed = query_started.elapsed();
    let totals = oracle.solver_totals().expect("algebraic solver totals");
    let native = oracle.relation_counters();
    let expected_hit = expected.is_some();
    assert_eq!(
        got.is_some(),
        expected_hit,
        "{}: algebraic truth differs from affine control for {:?}",
        fixture.label,
        target
    );
    if let Some(indices) = &got {
        assert_eq!(indices.len(), ARITY);
        assert_eq!(sum_indices(&fixture.instance.curve, &base, indices), target);
    }
    assert_eq!(
        native
            .get("pkm_descent_deferred_to_grid")
            .copied()
            .unwrap_or(0),
        0,
        "m=7 must never use grid extraction"
    );
    assert_eq!(
        totals.budget_exceeded, 0,
        "{}: the bounded correctness row was unresolved",
        fixture.label
    );

    json!({
        "schema":"isogeny-labs/pplus-dickson-m7-elimination/v1",
        "fixture":fixture.label,
        "p":fixture.instance.curve.p,
        "curve_a":fixture.instance.curve.a,
        "curve_b":fixture.instance.curve.b,
        "group_order":fixture.instance.group_order,
        "depth":fixture.depth,
        "arity":ARITY,
        "factor_base_abscissae":base.abscissae,
        "factor_base_points":base.points.len(),
        "target":point_json(target),
        "expected_hit":expected_hit,
        "control_witness":expected,
        "candidate_witness":got,
        "cold_wall_ns":cold_started.elapsed().as_nanos() as u64,
        "setup_wall_ns":setup_elapsed.as_nanos() as u64,
        "query_wall_ns":query_elapsed.as_nanos() as u64,
        "setup_group_ops":setup_ops,
        "setup_field_ops":{"muls":setup_ops.field_ops.muls,"sqrs":setup_ops.field_ops.sqrs,"invs":setup_ops.field_ops.invs},
        "query_group_ops":query_ops,
        "query_field_ops":{"muls":query_ops.field_ops.muls,"sqrs":query_ops.field_ops.sqrs,"invs":query_ops.field_ops.invs},
        "oracle_counters":{"lift_failures":counters.lift_failures,"unliftable_systems":counters.unliftable_systems},
        "setup_native":oracle.setup_native(),
        "relation_counters":native,
        "system_shape":oracle.last_system(),
        "solver_totals":totals,
        "budget_seconds":budget_seconds,
        "cross_target_state_reused":false,
    })
}

fn run_fixture(fixture: Fixture, include_miss: bool) -> Vec<Value> {
    let base = dickson_factor_base(&fixture.instance, fixture.depth, 0, 0)
        .expect("control Dickson factor base");
    assert!(!base.points.is_empty());
    let truth = exhaustive_affine(&fixture.instance.curve, &base);

    let planted_indices = vec![0; ARITY];
    let planted = sum_indices(&fixture.instance.curve, &base, &planted_indices);
    assert!(!planted.infinity);
    let planted_control = truth
        .get(&fixture.instance.curve.key(&planted))
        .expect("planted target is reachable");
    let mut rows = vec![cold_target(&fixture, planted, Some(planted_control), 60)];

    if include_miss {
        let generator = fixture.instance.generator_point();
        let miss = (1..fixture.instance.r)
            .map(|scalar| {
                fixture
                    .instance
                    .curve
                    .mul(&mut GroupOps::default(), generator, scalar)
            })
            .find(|point| {
                !point.infinity && !truth.contains_key(&fixture.instance.curve.key(point))
            })
            .expect("sparse carrier has an empty public target");
        rows.push(cold_target(&fixture, miss, None, 60));
    }

    // The affine Semaev chain deliberately excludes an infinite public root.
    // Keep the exact control value visible without turning it into a solver
    // hit/miss comparison outside the candidate's declared chart.
    rows.push(json!({
        "schema":"isogeny-labs/pplus-dickson-m7-chart-control/v1",
        "fixture":fixture.label,
        "target":point_json(fixture.instance.curve.identity()),
        "exact_affine_reachable":truth.contains_key(&fixture.instance.curve.key(&fixture.instance.curve.identity())),
        "candidate_chart":"finite nonidentity public target",
        "candidate_run":false,
    }));
    rows
}

fn main() {
    let selection = std::env::args().nth(1).unwrap_or_else(|| "all".into());
    let rows = match selection.as_str() {
        "depth1" => run_fixture(f7_depth1(), false),
        "depth2" => run_fixture(f31_depth2_sparse(), true),
        "all" => run_fixture(f7_depth1(), false)
            .into_iter()
            .chain(run_fixture(f31_depth2_sparse(), true))
            .collect(),
        _ => panic!("usage: pplus_dickson_m7_elimination [depth1|depth2|all]"),
    };
    for row in rows {
        println!("{}", serde_json::to_string(&row).expect("serialize result"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_controls_have_the_frozen_carriers_and_a_depth2_miss() {
        let d1 = f7_depth1();
        let b1 = dickson_factor_base(&d1.instance, d1.depth, 0, 0).unwrap();
        assert_eq!((b1.abscissae, b1.points.len()), (2, 4));

        let d2 = f31_depth2_sparse();
        let b2 = dickson_factor_base(&d2.instance, d2.depth, 0, 0).unwrap();
        assert_eq!((b2.abscissae, b2.points.len()), (1, 2));
        let truth = exhaustive_affine(&d2.instance.curve, &b2);
        assert!(truth.len() < (d2.instance.group_order - 1) as usize);
        for witness in truth.values() {
            assert_eq!(witness.len(), ARITY);
        }
    }
}
