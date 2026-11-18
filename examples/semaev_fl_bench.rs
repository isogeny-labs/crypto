//! Matched on/off benchmark for the Boolean Faugere--Lachartre split.
//!
//! The systems are fixed Weil restrictions of Semaev's third summation
//! polynomial.  System construction and input hashing happen before the timed
//! region.  `--arm fl` selects the optimized implementation,
//! `--arm full_width` selects the exact same-binary control, and `--arm both`
//! alternates the matched arms between repeats.

use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::koblitz_groebner::{
    build_decomposition_system, permute_poly, FieldStructure,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_frobenius_factor_base, KoblitzCurve,
};
use crypto_lib::cryptanalysis::pq_f4_f2::{groebner_basis_f4, groebner_basis_f4_full_width};
use crypto_lib::cryptanalysis::pq_groebner_f2::{mask_low64, F2BoolPoly};
use num_bigint::BigUint;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::time::Instant;

#[derive(Clone, Copy)]
struct Case {
    name: &'static str,
    a: u8,
    n: u32,
    factor_index: usize,
    m: usize,
    planted: usize,
    random: u32,
    seed: u64,
    interleave: bool,
}

const CASES: &[Case] = &[
    Case {
        name: "k1_n17_m2",
        a: 1,
        n: 17,
        factor_index: 0,
        m: 2,
        planted: 2,
        random: 2,
        seed: 17,
        interleave: false,
    },
    Case {
        name: "k0_n15_f1_m3",
        a: 0,
        n: 15,
        factor_index: 1,
        m: 3,
        planted: 1,
        random: 1,
        seed: 15,
        interleave: true,
    },
];

fn target_scalar(i: u32) -> BigUint {
    BigUint::from(1u64 + (i as u64).wrapping_mul(2_654_435_761) % 1_000_003)
}

fn systems(case: Case) -> Vec<Vec<F2BoolPoly>> {
    let curve = KoblitzCurve::new(case.a, case.n).expect("Koblitz curve exists");
    let factor_base =
        build_frobenius_factor_base(&curve, case.factor_index).expect("factor base exists");
    assert!(factor_base.m_can_decompose(&curve, case.m));
    let structure = FieldStructure::new(case.n, &curve.curve.irreducible);
    let mut rng = StdRng::seed_from_u64(case.seed);
    let mut targets = Vec::new();
    while targets.len() < case.planted {
        let target = (0..case.m).fold(BinaryPoint::Infinity, |acc, _| {
            curve.add(
                &acc,
                &factor_base.points[rng.gen_range(0..factor_base.points.len())],
            )
        });
        if matches!(target, BinaryPoint::Affine { .. }) {
            targets.push(target);
        }
    }
    for i in 0..case.random {
        targets.push(curve.mul(curve.generator(), &target_scalar(i)));
    }
    targets
        .iter()
        .map(|target| {
            let BinaryPoint::Affine { x, .. } = target else {
                panic!("frozen target is infinity")
            };
            let system = build_decomposition_system(
                &factor_base.subspace_basis,
                x,
                &curve.curve.b,
                case.m,
                &structure,
            )
            .expect("system fits the Boolean mask");
            if case.interleave && case.m >= 3 {
                let permutation = system.interleaved_order(case.n);
                system
                    .equations
                    .iter()
                    .map(|poly| permute_poly(poly, &permutation))
                    .collect()
            } else {
                system.equations
            }
        })
        .collect()
}

fn hash_polys(systems: &[Vec<F2BoolPoly>]) -> String {
    let mut hash = Sha256::new();
    hash.update((systems.len() as u64).to_le_bytes());
    for system in systems {
        hash.update((system.len() as u64).to_le_bytes());
        for poly in system {
            hash.update((poly.n_vars as u64).to_le_bytes());
            hash.update((poly.terms.len() as u64).to_le_bytes());
            for term in &poly.terms {
                hash.update(mask_low64(term.mask).to_le_bytes());
            }
        }
    }
    hex::encode(hash.finalize())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let case_name = args
        .windows(2)
        .find(|pair| pair[0] == "--case")
        .map(|pair| pair[1].as_str())
        .unwrap_or("k1_n17_m2");
    let repeats: usize = args
        .windows(2)
        .find(|pair| pair[0] == "--repeats")
        .map(|pair| pair[1].parse().expect("--repeats integer"))
        .unwrap_or(1);
    let arm = args
        .windows(2)
        .find(|pair| pair[0] == "--arm")
        .map(|pair| pair[1].as_str())
        .unwrap_or("fl");
    assert!(
        matches!(arm, "fl" | "full_width" | "both"),
        "invalid --arm {arm}"
    );
    let case = *CASES
        .iter()
        .find(|case| case.name == case_name)
        .unwrap_or_else(|| panic!("unknown --case {case_name}"));
    let systems = systems(case);
    let input_sha256 = hash_polys(&systems);
    let n_vars = systems
        .first()
        .and_then(|system| system.first())
        .map_or(0, |poly| poly.n_vars);
    for repeat in 0..repeats {
        let selected_arms = match arm {
            "both" if repeat % 2 == 0 => vec!["full_width", "fl"],
            "both" => vec!["fl", "full_width"],
            selected => vec![selected],
        };
        for (order_index, selected_arm) in selected_arms.into_iter().enumerate() {
            let outer = Instant::now();
            let mut basis_sets = Vec::with_capacity(systems.len());
            let mut stats = Vec::with_capacity(systems.len());
            for system in &systems {
                let (basis, report) = if selected_arm == "full_width" {
                    groebner_basis_f4_full_width(system.clone(), n_vars, None)
                } else {
                    groebner_basis_f4(system.clone(), n_vars, None)
                };
                basis_sets.push(basis);
                stats.push(report);
            }
            let outer_ns = outer.elapsed().as_nanos() as u64;
            let basis_sha256 = hash_polys(&basis_sets);
            let solver_wall_ns: u64 = stats.iter().map(|report| report.wall_ns).sum();
            let fl_matrices: u64 = stats.iter().map(|report| report.fl_matrices).sum();
            let timed_out = stats.iter().any(|report| report.timed_out);
            let oversize = stats.iter().any(|report| report.oversize);
            println!(
                "{}",
                json!({
                    "schema_version": "1.0",
                    "kind": "semaev_boolean_fl_benchmark",
                    "case": case.name,
                    "arm": selected_arm,
                    "repeat": repeat,
                    "order_index": order_index,
                    "curve_a": case.a,
                    "field_degree": case.n,
                    "factor_index": case.factor_index,
                    "summands": case.m,
                    "interleaved": case.interleave,
                    "systems": systems.len(),
                    "n_vars": n_vars,
                    "input_sha256": input_sha256,
                    "basis_sha256": basis_sha256,
                    "outer_ns": outer_ns,
                    "solver_wall_ns": solver_wall_ns,
                    "fl_matrices": fl_matrices,
                    "timed_out": timed_out,
                    "oversize": oversize,
                    "stats": stats,
                    "claim_boundary": "Boolean Groebner stage on fixed toy Semaev systems; not an end-to-end ECDLP or sub-rho claim"
                })
            );
        }
    }
}
