//! Separate known-assignment control: also pin orbit representative and phase.
//! This is an encoding control, not a natural relation search.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::binary_semaev_chain_sat::{
    frobenius_orbit_domain_size, ChainedS3Encoding,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use crypto_lib::cryptanalysis::sat::SolveResult;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use std::time::Instant;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const FIXTURE: &[u8] = include_bytes!("../research/koblitz_n83_factor_base_sweep_20261008/verification/mixed-orbit-known-witness-source-20261012/fixture.json");
const FIXTURE_SHA256: &str = "fc203db28eed0164e47b0c755c3d9a09f80df1acd5e4c7e09dfe172fa5f0a538";
const FIXTURE_BLAKE3: &str = "7e257cf3f278051094e27bd6df30ac8e78a0f55739b11f8b6837f23e5637703d";

fn decimal(value: &Value) -> Result<BigUint> {
    let raw = value.as_str().ok_or("decimal string required")?;
    if raw.is_empty() || !raw.bytes().all(|c| c.is_ascii_digit()) {
        return Err("invalid decimal string".into());
    }
    let parsed: BigUint = raw.parse()?;
    if parsed.to_string() != raw || parsed.bits() > 83 {
        return Err("noncanonical or out-of-field integer".into());
    }
    Ok(parsed)
}

fn numbers(value: &Value, count: usize) -> Result<Vec<BigUint>> {
    let array = value.as_array().ok_or("array required")?;
    if array.len() != count {
        return Err("array length differs".into());
    }
    array.iter().map(decimal).collect()
}

fn pin_block(enc: &mut ChainedS3Encoding, block: usize, value: &BigUint, n: usize) {
    for bit in 0..n {
        let wire = (block * n + bit + 1) as i32;
        enc.solver
            .add_clause(vec![if value.bit(bit as u64) { wire } else { -wire }]);
    }
}

fn locate(reps: &[BigUint], x: &BigUint, curve: &KoblitzCurve) -> Result<(usize, u32)> {
    let mut found = None;
    for (i, rep) in reps.iter().enumerate() {
        let mut value = F2mElement::from_biguint(rep, curve.n);
        for phase in 0..curve.n {
            if value.to_biguint() == *x {
                if found.is_some() {
                    return Err("ambiguous orbit witness".into());
                }
                found = Some((i, phase));
            }
            value = value.square_k_times(1, &curve.curve.irreducible);
        }
    }
    found.ok_or_else(|| "witness absent from orbit domain".into())
}

// Coupled explicitly to add_frobenius_orbit_domain's frozen allocation layout:
// selectors(r), prefix(r-1), root(n), phase(ceil(log2 n)), shifted/output blocks.
fn pin_selectors(
    enc: &mut ChainedS3Encoding,
    domains: &[Vec<BigUint>],
    xs: &[BigUint],
    curve: &KoblitzCurve,
    first: u32,
    end: u32,
) -> Result<Vec<Value>> {
    let mut next = first;
    let mut assignments = Vec::new();
    let phase_bits = u32::BITS - (curve.n - 1).leading_zeros();
    for (domain, x) in domains.iter().zip(xs) {
        let mut reps = domain.clone();
        reps.sort_unstable();
        reps.dedup();
        if reps.len() != domain.len() {
            return Err("duplicate domain representatives".into());
        }
        let (selected, phase) = locate(&reps, x, curve)?;
        for i in 0..reps.len() {
            let wire = (next + i as u32) as i32;
            enc.solver
                .add_clause(vec![if i == selected { wire } else { -wire }]);
        }
        let phase_first = next + 2 * reps.len() as u32 - 1 + curve.n;
        for bit in 0..phase_bits {
            let wire = (phase_first + bit) as i32;
            enc.solver
                .add_clause(vec![if phase & (1 << bit) != 0 { wire } else { -wire }]);
        }
        assignments.push(json!({"representative":reps[selected].to_string(),"index":selected,"phase":phase,"domain_size":reps.len(),"selector_first_wire":next,"phase_first_wire":phase_first}));
        next += u32::try_from(
            frobenius_orbit_domain_size(curve.n, 1, reps.len())
                .ok_or("domain size")?
                .variables,
        )?;
    }
    if next != end + 1 {
        return Err("selector allocation layout differs from frozen encoder".into());
    }
    Ok(assignments)
}

fn run(path: &str, conflicts: u64) -> Result<Value> {
    if conflicts == 0 || conflicts > 1_000_000 {
        return Err("invalid conflict cap".into());
    }
    let raw = fs::read(path)?;
    let input: Value = serde_json::from_slice(&raw)?;
    if input["schema"] != "n83.fixed-witness-domains/v1"
        || input["fixture_sha256"] != FIXTURE_SHA256
        || blake3::hash(FIXTURE).to_hex().as_str() != FIXTURE_BLAKE3
        || input["identity_mask"] != 0
    {
        return Err("input identity differs".into());
    }
    let fixture: Value = serde_json::from_slice(FIXTURE)?;
    let ordered_indices = [1, 0, 2, 3, 4];
    let terms = fixture["terms"].as_array().ok_or("fixture terms")?;
    if terms.len() != 5 {
        return Err("fixture arity differs".into());
    }
    let expected_xs: Vec<BigUint> = ordered_indices
        .iter()
        .map(|&i| decimal(&terms[i]["point"][0]))
        .collect::<Result<_>>()?;
    let xs = numbers(&input["ordered_xs"], 5)?;
    if xs != expected_xs || xs[0] >= xs[1] {
        return Err("ordered witness differs".into());
    }
    let target_x = decimal(&fixture["equation"]["target"][0])?;
    if decimal(&input["target_x"])? != target_x {
        return Err("target x differs".into());
    }
    let partial_xs = numbers(&input["partial_xs"], 3)?;
    let small = input["small_reps"]
        .as_array()
        .ok_or("small representatives")?;
    let residual = input["residual_reps"]
        .as_array()
        .ok_or("residual representatives")?;
    if small.len() != 1182 || residual.len() != 866 {
        return Err("factor-base orbit count differs".into());
    }
    let small: Vec<BigUint> = small.iter().map(decimal).collect::<Result<_>>()?;
    let residual: Vec<BigUint> = residual.iter().map(decimal).collect::<Result<_>>()?;
    let domains = vec![
        small.clone(),
        small.clone(),
        small,
        residual.clone(),
        residual,
    ];
    let curve = KoblitzCurve::known_n83_k0().ok_or("pinned N83 curve")?;
    let target = F2mElement::from_biguint(&target_x, 83);
    let started = Instant::now();
    eprintln!(
        "{}",
        json!({"phase":"build_started","elapsed_wall_ms_L0":0.0})
    );
    let mut enc = ChainedS3Encoding::build(
        5,
        &curve.curve.irreducible,
        &curve.curve.b,
        Some(&target),
        0,
        150_000,
    )
    .map_err(|error| format!("circuit build: {error:?}"))?;
    eprintln!(
        "{}",
        json!({
            "phase":"circuit_built", "elapsed_wall_ms_L0":started.elapsed().as_secs_f64()*1000.0,
            "variables":enc.solver.n_vars(), "clauses":enc.solver.n_clauses(),
            "xors":enc.solver.n_xors(),
        })
    );
    let domain_first = enc.solver.n_vars() + 1;
    let size = enc
        .constrain_frobenius_orbit_domains(&domains, 150_000, 12_000_000)
        .map_err(|error| format!("domain build: {error:?}"))?;
    if size.clauses != 465_738 {
        return Err("domain clause count differs from v7 worker".into());
    }
    let domain_end = enc.solver.n_vars();
    enc.order_first_pair(150_000)
        .map_err(|error| format!("first-pair order: {error:?}"))?;
    eprintln!(
        "{}",
        json!({
            "phase":"domains_built", "elapsed_wall_ms_L0":started.elapsed().as_secs_f64()*1000.0,
            "variables":enc.solver.n_vars(), "clauses":enc.solver.n_clauses(),
            "xors":enc.solver.n_xors(),
        })
    );
    let before_units = json!({
        "variables":enc.solver.n_vars(), "clauses":enc.solver.n_clauses(),
        "xors":enc.solver.n_xors(), "domain_clauses":size.clauses,
    });
    for (i, x) in xs.iter().enumerate() {
        pin_block(&mut enc, i, x, 83);
    }
    for (i, x) in partial_xs.iter().enumerate() {
        pin_block(&mut enc, 5 + i, x, 83);
    }
    let variables = enc.solver.n_vars();
    let clauses = enc.solver.n_clauses();
    eprintln!(
        "{}",
        json!({
            "phase":"fixed_units_installed", "elapsed_wall_ms_L0":started.elapsed().as_secs_f64()*1000.0,
            "variables":variables, "clauses":clauses, "xors":enc.solver.n_xors(),
        })
    );
    let selector_assignments =
        pin_selectors(&mut enc, &domains, &xs, &curve, domain_first, domain_end)?;
    eprintln!(
        "{}",
        json!({"phase":"selector_units_installed","elapsed_wall_ms_L0":started.elapsed().as_secs_f64()*1000.0,"assignments":selector_assignments})
    );
    enc.solver.conflict_budget = conflicts;
    let outcome = enc.solver.solve();
    let model_verified = if outcome == SolveResult::Sat {
        enc.verify_model()
            && enc
                .decode_summands()
                .iter()
                .zip(&xs)
                .all(|(a, b)| a.to_biguint() == *b)
    } else {
        false
    };
    Ok(json!({
        "schema":"n83.fixed-witness-sat-control/v1",
        "scope":"known_answer_encoding_control_only",
        "selector_assignments":selector_assignments,
        "control_variant":"fixed_coordinates_representatives_and_phases",
        "status":match outcome {
            SolveResult::Sat if model_verified => "SAT_verified_witness_x",
            SolveResult::Sat => "PRODUCER_FAILURE_invalid_model",
            SolveResult::Unsat => "UNSAT_fixed_witness_encoding",
            SolveResult::Unknown => "UNKNOWN_conflict_cap",
        },
        "input_blake3":blake3::hash(&raw).to_hex().to_string(),
        "fixture_sha256":FIXTURE_SHA256,
        "fixture_blake3":FIXTURE_BLAKE3,
        "identity_mask":0,
        "before_fixed_units":before_units,
        "fixed_x_blocks":8,
        "variables":variables, "clauses":enc.solver.n_clauses(), "xors":enc.solver.n_xors(),
        "conflict_cap":conflicts, "conflicts":enc.solver.conflicts(),
        "model_verified":model_verified,
        "elapsed_wall_ms_L0":started.elapsed().as_secs_f64()*1000.0,
        "natural_relations":Value::Null,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null,
    }))
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: koblitz_n83_selector_witness_control DOMAINS_JSON CONFLICT_CAP".into());
    }
    println!(
        "{}",
        serde_json::to_string(&run(&args[1], args[2].parse()?)?)?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn locator_recovers_phase_and_rejects_ambiguous_or_absent() {
        let curve = KoblitzCurve::new(0, 7).unwrap();
        let reps = vec![BigUint::from(2u8), BigUint::from(3u8)];
        let x = F2mElement::from_biguint(&reps[0], 7)
            .square_k_times(4, &curve.curve.irreducible)
            .to_biguint();
        assert_eq!(locate(&reps, &x, &curve).unwrap(), (0, 4));
        assert!(locate(&[], &x, &curve).is_err());
        assert!(locate(&[BigUint::from(2u8), BigUint::from(4u8)], &x, &curve).is_err());
    }
}
