//! Original free-selector witness solve with post-solve model export only.
//! This is an encoding control, not a natural relation search.

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::binary_semaev_chain_sat::frobenius_orbit_domain_size;
use crypto_lib::cryptanalysis::binary_semaev_chain_sat::ChainedS3Encoding;
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

fn decode_selector(
    model: &[bool],
    first: u32,
    phase_first: u32,
    count: usize,
) -> Result<(usize, u32)> {
    let read = |wire: u32| -> Result<bool> {
        let i = wire.checked_sub(1).ok_or("zero wire")? as usize;
        model
            .get(i)
            .copied()
            .ok_or_else(|| "wire outside model".into())
    };
    let mut selected = None;
    for i in 0..count {
        if read(first + u32::try_from(i)?)? {
            if selected.replace(i).is_some() {
                return Err("multiple selected representatives".into());
            }
        }
    }
    let index = selected.ok_or("no selected representative")?;
    let mut phase = 0;
    for bit in 0..7 {
        if read(phase_first + bit)? {
            phase |= 1 << bit;
        }
    }
    if phase >= 83 {
        return Err("model phase outside83".into());
    }
    Ok((index, phase))
}

fn export_selectors(
    enc: &ChainedS3Encoding,
    domains: &[Vec<BigUint>],
    first: u32,
    end: u32,
) -> Result<Vec<Value>> {
    let model = enc.solver.model();
    let mut next = first;
    let mut assignments = Vec::new();
    for domain in domains {
        let mut reps = domain.clone();
        reps.sort_unstable();
        reps.dedup();
        if reps.len() != domain.len() {
            return Err("duplicate representatives".into());
        }
        let phase_first = next + 2 * reps.len() as u32 - 1 + 83;
        let (index, phase) = decode_selector(&model, next, phase_first, reps.len())?;
        assignments.push(json!({"representative":reps[index].to_string(),"index":index,"phase":phase,"domain_size":reps.len(),"selector_first_wire":next,"phase_first_wire":phase_first}));
        next += u32::try_from(
            frobenius_orbit_domain_size(83, 1, reps.len())
                .ok_or("domain size")?
                .variables,
        )?;
    }
    if next != end + 1 {
        return Err("domain wire layout differs".into());
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
    enc.solver.conflict_budget = conflicts;
    let outcome = enc.solver.solve();
    // Decode only after solve returns; never install selector assumptions.
    let selector_assignments = if outcome == SolveResult::Sat {
        export_selectors(&enc, &domains, domain_first, domain_end)?
    } else {
        Vec::new()
    };
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
        "control_variant":"fixed_coordinates_free_selectors",
        "selector_assignments":selector_assignments,
        "selector_export":"decoded_from_sat_model_after_solve",
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
        "variables":variables, "clauses":clauses, "xors":enc.solver.n_xors(),
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
        return Err(
            "usage: koblitz_n83_free_selector_export_control DOMAINS_JSON CONFLICT_CAP".into(),
        );
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
    fn selector_decoder_rejects_absent_multiple_bad_phase_and_bounds() {
        let mut model = vec![false; 10];
        model[1] = true;
        model[4] = true; // phase bit1 at wire5: phase=2
        assert_eq!(decode_selector(&model, 1, 4, 3).unwrap(), (1, 2));
        model[0] = true;
        assert!(decode_selector(&model, 1, 4, 3).is_err());
        model[0] = false;
        model[1] = false;
        assert!(decode_selector(&model, 1, 4, 3).is_err());
        model[1] = true;
        for bit in 0..7 {
            model[3 + bit] = (83 & (1 << bit)) != 0;
        }
        assert!(decode_selector(&model, 1, 4, 3).is_err());
        assert!(decode_selector(&model, 0, 4, 3).is_err());
        assert!(decode_selector(&model[..5], 1, 4, 3).is_err());
    }
}
