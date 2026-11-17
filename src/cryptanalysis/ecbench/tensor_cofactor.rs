//! `ic.tensor_contraction_cofactor_v1`: the toy tensor-contraction index
//! calculus of `crates/ecdl-tensor-ic` on a fixed-cofactor prime curve
//! (#E = c·r, r prime, c > 1, target and generator in the order-r subgroup).
//!
//! A separate id from `ic.tensor_contraction_v1`, which keeps refusing
//! c > 1 and whose charge is unchanged (ecbench rule: never change what a
//! registered id counts). The charge is the same: every non-trivial point
//! addition and doubling the solver performs, phase by phase from the crate's
//! tally, including the [c] projections evaluated inside the contraction;
//! contraction rows, factor-base listing, the once-per-draw preimage scan
//! and elimination are counted `_uncharged`.

use std::collections::BTreeMap;
use std::time::Instant;

use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::json;

use super::exact_scalar::ExactScalar;
use super::methods::{
    ops_phase, param_u64, unpriced_of, ParamDecl, ResolvedMethod, SolveReport, TENSOR_PHASES,
};
use crate::cryptanalysis::ic_boundary::{GroupOps, PrimeInstance, PrimePoint};

pub const ID: &str = "ic.tensor_contraction_cofactor_v1";

pub const SUMMARY: &str = "toy index calculus on a fixed-cofactor curve (#E = c·r, c > 1): m keyed SHA-256 hashed factor bases over the order-r subgroup G (|F_i| ≈ (2r)^(1/m)), free source points S_i ∈ E with A_i = [c]S_i, each relation found by exact sparse tensor contraction of the whole-point cofactor network (crates/ecdl-tensor-ic, preimages of each F_i listed once per draw); exponential; contraction rows, factor-base listing, preimage scan and elimination counted unpriced";

pub const PARAMS: &[ParamDecl] = &[
    ParamDecl {
        name: "m",
        default: None,
        help: "hashed factor bases, at least 2",
    },
    ParamDecl {
        name: "max_restarts",
        default: Some("8"),
        help: "factor-base redraws before the run counts as exhausted",
    },
];

/// The parameter and setting refusals `solve` raises on every run.
pub fn applicable(m: &ResolvedMethod, inst: &PrimeInstance) -> Result<(), String> {
    if usize::try_from(param_u64(m, "m")?).map_or(true, |v| v < 2) {
        return Err(format!("{ID} needs m ≥ 2"));
    }
    if usize::try_from(param_u64(m, "max_restarts")?).map_or(true, |v| v == 0) {
        return Err(format!("{ID} needs max_restarts ≥ 1"));
    }
    if inst.cofactor < 2 || inst.group_order != inst.cofactor * inst.r || inst.r <= inst.cofactor {
        return Err(format!(
            "{ID} needs #E = c·r with c > 1 and r > c (ic.tensor_contraction_v1 covers c = 1); {} has #E = {}, r = {}, cofactor {}",
            inst.name, inst.group_order, inst.r, inst.cofactor
        ));
    }
    Ok(())
}

pub fn solve(
    m: &ResolvedMethod,
    inst: &PrimeInstance,
    target: PrimePoint,
    seed: u64,
) -> Result<SolveReport, String> {
    use ecdl_tensor_ic::{opcount, Curve as TCurve, Point as TPoint, Solver, SparseContractor};

    applicable(m, inst)?;
    let fb_count = usize::try_from(param_u64(m, "m")?).map_err(|_| "m exceeds usize")?;
    let max_restarts =
        usize::try_from(param_u64(m, "max_restarts")?).map_err(|_| "max_restarts exceeds usize")?;
    let curve = TCurve {
        p: inst.curve.p,
        a: inst.curve.a,
        b: inst.curve.b,
        order: inst.r,
        g: TPoint {
            x: inst.generator.0,
            y: inst.generator.1,
            inf: false,
        },
    };
    let q = TPoint {
        x: target.x,
        y: target.y,
        inf: target.infinity,
    };
    let mut solver = Solver::new(
        curve,
        fb_count,
        Box::new(SparseContractor::default()),
        StdRng::seed_from_u64(seed),
    );
    solver.max_restarts = max_restarts;
    solver.cofactor = inst.cofactor;
    let started = Instant::now();
    opcount::start();
    let (result, stats) = solver.solve_with_stats(q);
    let tally = opcount::take();
    let wall = started.elapsed().as_nanos() as u64;

    let mut phases = Vec::with_capacity(TENSOR_PHASES.len());
    let mut charged = 0usize;
    for name in TENSOR_PHASES {
        let mut get = |k: &str| match tally.get(&format!("{name}.{k}")) {
            Some(&n) => {
                charged += 1;
                n
            }
            None => 0,
        };
        let ops = GroupOps {
            adds: get("adds"),
            doubles: get("doubles"),
            ..GroupOps::default()
        };
        phases.push(ops_phase(name, ops));
    }
    let mut counters: BTreeMap<String, u64> = tally
        .iter()
        .filter(|(k, _)| k.ends_with("_uncharged"))
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    // Every tally entry is either charged above or kept as unpriced work.
    if charged + counters.len() != tally.len() {
        return Err(format!(
            "{ID}: tally has entries outside the known phases: {:?}",
            tally.keys().collect::<Vec<_>>()
        ));
    }
    for (key, value) in [
        ("cofactor", inst.cofactor as usize),
        ("factor_base_size", stats.factor_base_size),
        ("relations", stats.relations),
        ("attempts", stats.attempts),
        ("restarts", stats.restarts),
        ("network_edges", stats.network.edges),
        ("network_vertices", stats.network.vertices),
        ("network_max_degree", stats.network.max_degree),
    ] {
        counters.insert(key.into(), value as u64);
    }
    let (recovered, error) = match result {
        Ok(xi) => (Some(u128::from(xi)), None),
        Err(e) => (None, Some(e.to_string())),
    };
    Ok(SolveReport {
        recovered: recovered.map(ExactScalar::from),
        exhausted: recovered.is_none(),
        total_gae: phases.iter().map(|p| p.gae).sum(),
        phases,
        automorphisms_used: 1,
        unpriced: unpriced_of(&counters),
        counters,
        deterministic: true,
        solve_wall_ns: wall,
        detail: json!({
            "implementation": "crates/ecdl-tensor-ic cofactor.rs (Rust only; no Go original)",
            "factor_base": "m keyed SHA-256 membership predicates on A_i ∈ G, listed by enumerating the group (toy shortcut); preimages under [c] listed once per draw",
            "relation_network": "whole-point cofactor network: free S_i ∈ E, projection A_i = [c]S_i filtered by F_i membership, sums to R; counted by SparseContractor, witness by self-reduction; count = c^m × the subgroup network's",
            "linear_algebra": "sparse left kernel mod r",
            "charged": "every non-trivial point addition and doubling (neither input O), including the [c] projections inside the contraction",
            "error": error,
            "cost_scope": "cold, one target; no [Ope26] solver, every counter exponential",
        }),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::super::methods::{applicable as plan, resolve, solve as run, MethodSpec};
    use super::super::workload::{CurveSpec, Instance};
    use super::*;
    use crate::cryptanalysis::ic_boundary::CountedGroup;

    fn method(m: &str) -> ResolvedMethod {
        resolve(&MethodSpec {
            id: ID.into(),
            params: [("m".to_string(), m.to_string())].into_iter().collect(),
        })
        .unwrap()
    }

    /// A spec from a toy cofactor curve of the crate (naive counting).
    fn spec(bits: usize, c: u64, seed: u64) -> CurveSpec {
        let mut rng = StdRng::seed_from_u64(seed);
        let cc = ecdl_tensor_ic::random_cofactor_curve(&mut rng, bits, c).unwrap();
        CurveSpec::PrimeExplicit {
            p: cc.curve.p,
            a: cc.curve.a,
            b: cc.curve.b,
            group_order: cc.group_order(),
            r: cc.curve.order,
            gx: cc.curve.g.x,
            gy: cc.curve.g.y,
        }
    }

    /// The registered 16-bit c = 4 curve, `toy_curve_with_cofactor(16, 20261021, 4)`.
    const C4_16: CurveSpec = CurveSpec::PrimeExplicit {
        p: 65519,
        a: 65516,
        b: 45453,
        group_order: 65644,
        r: 16411,
        gx: 64994,
        gy: 58144,
    };

    #[test]
    fn plan_time_refusals() {
        let inst = C4_16.build().unwrap();
        let facts = inst.facts(&C4_16);
        plan(&method("2"), &inst, &facts).unwrap();
        assert!(plan(&method("1"), &inst, &facts).is_err());
        // The prime-order arm still refuses this curve, and this arm refuses
        // a prime-order curve.
        let v1 = resolve(&MethodSpec {
            id: "ic.tensor_contraction_v1".into(),
            params: [("m".to_string(), "2".to_string())].into_iter().collect(),
        })
        .unwrap();
        assert!(plan(&v1, &inst, &facts).is_err());
        let prime = CurveSpec::PrimeSearch { bits: 10, seed: 1 };
        let pinst = prime.build().unwrap();
        let pfacts = pinst.facts(&prime);
        assert!(plan(&method("2"), &pinst, &pfacts).is_err());
    }

    #[test]
    fn recovers_known_logs_and_charges_every_phase() {
        for (c, seed) in [(2u64, 1u64), (4, 2)] {
            let spec = spec(10, c, seed);
            let inst = spec.build().unwrap();
            let facts = inst.facts(&spec);
            let m = method("2");
            plan(&m, &inst, &facts).unwrap();
            let Instance::Prime(pi) = &inst else {
                unreachable!()
            };
            assert_eq!(pi.cofactor, c);
            for (k, s) in [(5u64, 1u64), (pi.r - 2, 2)] {
                let t = pi
                    .curve
                    .mul(&mut GroupOps::default(), pi.generator_point(), k);
                let target = [format!("0x{:x}", t.x), format!("0x{:x}", t.y)];
                let rep = run(&m, &inst, &facts, &target, s).unwrap();
                assert_eq!(
                    rep.recovered,
                    Some(ExactScalar::from(u128::from(k))),
                    "c={c}"
                );
                let names: Vec<&str> = rep.phases.iter().map(|p| p.name.as_str()).collect();
                assert_eq!(names, TENSOR_PHASES);
                assert!(rep
                    .phases
                    .iter()
                    .any(|p| p.name == "relation_search" && p.gae > 0.0));
                assert_eq!(rep.counters["cofactor"], c);
                assert!(rep
                    .unpriced
                    .iter()
                    .any(|u| u.ends_with("fb_preimage_scan_uncharged")));
                // Deterministic: the same seed replays to the same report.
                let again = run(&m, &inst, &facts, &target, s).unwrap();
                assert_eq!(again.counters, rep.counters);
                assert_eq!(again.total_gae, rep.total_gae);
            }
        }
    }
}
