//! Export the pinned primary-N83 chained-S3 known-witness control as exact XCNF.
//! A separate solver and verifier consume the immutable exported bytes.

use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::binary_semaev_chain_sat::ChainedS3Encoding;
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use crypto_lib::cryptanalysis::sat::to_dimacs_xor;
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::Instant;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const FIXTURE: &[u8] = include_bytes!("../research/koblitz_n83_factor_base_sweep_20261008/verification/mixed-orbit-known-witness-source-20261012/fixture.json");
const FIXTURE_SHA256: &str = "fc203db28eed0164e47b0c755c3d9a09f80df1acd5e4c7e09dfe172fa5f0a538";
const FIXTURE_BLAKE3: &str = "7e257cf3f278051094e27bd6df30ac8e78a0f55739b11f8b6837f23e5637703d";
const NATURAL_PLAN: &[u8] = include_bytes!("../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-gray-hash-paired-protocol-20261014/plan.json");
const NATURAL_PLAN_SHA256: &str =
    "fbd3fb943bc0f181797d106c41ee9961b8d70768b5c65a5584002e3c1b5e488f";

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

fn run(path: &str, output: &str) -> Result<Value> {
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
    let mut enc = ChainedS3Encoding::build(
        5,
        &curve.curve.irreducible,
        &curve.curve.b,
        Some(&target),
        0,
        150_000,
    )
    .map_err(|error| format!("circuit build: {error:?}"))?;
    let size = enc
        .constrain_frobenius_orbit_domains(&domains, 150_000, 12_000_000)
        .map_err(|error| format!("domain build: {error:?}"))?;
    if size.clauses != 465_738 {
        return Err("domain clause count differs from v7 worker".into());
    }
    enc.order_first_pair(150_000)
        .map_err(|error| format!("first-pair order: {error:?}"))?;
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
    let xors = enc.solver.n_xors();
    let xcnf = to_dimacs_xor(&enc.solver);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    file.write_all(xcnf.as_bytes())?;
    file.sync_all()?;
    Ok(json!({
        "schema":"n83.fixed-witness-xcnf-export/v1",
        "scope":"known_answer_encoding_control_only",
        "status":"EXPORTED_XCNF",
        "input_blake3":blake3::hash(&raw).to_hex().to_string(),
        "fixture_sha256":FIXTURE_SHA256,
        "fixture_blake3":FIXTURE_BLAKE3,
        "identity_mask":0,
        "before_fixed_units":before_units,
        "fixed_x_blocks":8,
        "variables":variables, "clauses":clauses, "xors":xors,
        "xcnf_bytes":xcnf.len(),
        "xcnf_blake3":blake3::hash(xcnf.as_bytes()).to_hex().to_string(),
        "build_export_wall_ms_L0":started.elapsed().as_secs_f64()*1000.0,
        "natural_relations":Value::Null,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null,
    }))
}

fn run_natural(path: &str, output: &str) -> Result<Value> {
    let started = Instant::now();
    let raw = fs::read(path)?;
    let input: Value = serde_json::from_slice(&raw)?;
    if format!("{:x}", Sha256::digest(NATURAL_PLAN)) != NATURAL_PLAN_SHA256
        || input["schema"] != "n83.k600-natural-xcnf-input/v1"
        || input["plan_sha256"] != NATURAL_PLAN_SHA256
        || input["trial"] != 0
    {
        return Err("natural input or plan identity differs".into());
    }
    let plan: Value = serde_json::from_slice(NATURAL_PLAN)?;
    let role = input["role"].as_str().ok_or("role required")?;
    let arm = plan["arms"]
        .as_array()
        .ok_or("plan arms")?
        .iter()
        .find(|arm| arm["role"] == role)
        .ok_or("unknown plan role")?;
    for field in [
        "base_object_blake3",
        "base_compressed_sha256",
        "pool_object_blake3",
        "pool_compressed_sha256",
        "shared_signed_orbits",
    ] {
        if input[field] != arm[field] {
            return Err(format!("natural input {field} differs from plan").into());
        }
    }
    let residual_count = arm["residual_signed_orbits"]
        .as_u64()
        .ok_or("residual orbit count")? as usize;
    let small = numbers(&input["small_reps"], 600)?;
    let residual = numbers(&input["residual_reps"], residual_count)?;
    let domains = vec![
        small.clone(),
        small.clone(),
        small,
        residual.clone(),
        residual,
    ];
    let curve = KoblitzCurve::known_n83_k0().ok_or("pinned N83 curve")?;
    let public = numbers(&plan["public_target"], 2)?;
    let q = BinaryPoint::Affine {
        x: F2mElement::from_biguint(&public[0], 83),
        y: F2mElement::from_biguint(&public[1], 83),
    };
    if !curve.curve.is_on_curve(&q) || curve.mul(&q, &curve.subgroup_order) != BinaryPoint::Infinity
    {
        return Err("public target subgroup differs".into());
    }
    let a = decimal(&plan["first_trial_a"])?;
    let b = decimal(&plan["first_trial_b"])?;
    if a >= curve.subgroup_order || b >= curve.subgroup_order {
        return Err("query coefficient exceeds subgroup order".into());
    }
    let query = curve.add(&curve.mul(curve.generator(), &a), &curve.mul(&q, &b));
    let BinaryPoint::Affine {
        x: query_x,
        y: query_y,
    } = &query
    else {
        return Err("ordinary query is identity".into());
    };
    let mut enc = ChainedS3Encoding::build(
        5,
        &curve.curve.irreducible,
        &curve.curve.b,
        Some(query_x),
        0,
        150_000,
    )
    .map_err(|error| format!("circuit build: {error:?}"))?;
    let size = enc
        .constrain_frobenius_orbit_domains(&domains, 150_000, 12_000_000)
        .map_err(|error| format!("domain build: {error:?}"))?;
    let expected_clauses = if role == "gray_pool" {
        183_314
    } else {
        269_830
    };
    let expected_variables = if role == "gray_pool" { 95_068 } else { 97_080 };
    if size.clauses != expected_clauses {
        return Err("natural domain clause count differs from hosted pilot".into());
    }
    enc.order_first_pair(150_000)
        .map_err(|error| format!("first-pair order: {error:?}"))?;
    if enc.solver.n_vars() != expected_variables || enc.solver.n_xors() != 4_980 {
        return Err("natural formula shape differs from hosted pilot".into());
    }
    let xcnf = to_dimacs_xor(&enc.solver);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    file.write_all(xcnf.as_bytes())?;
    file.sync_all()?;
    Ok(json!({
        "schema":"n83.k600-natural-xcnf-export/v1",
        "scope":"frozen_ordinary_first_query_stage_only",
        "status":"EXPORTED_NATURAL_XCNF",
        "role":role, "trial":0,
        "plan_sha256":NATURAL_PLAN_SHA256,
        "input_blake3":blake3::hash(&raw).to_hex().to_string(),
        "small_orbits":600, "residual_orbits":residual_count,
        "query_coefficient_a":a.to_string(), "query_coefficient_b":b.to_string(),
        "public_target":[public[0].to_string(),public[1].to_string()],
        "query_point":[query_x.to_biguint().to_string(),query_y.to_biguint().to_string()],
        "identity_mask":0, "order_first_pair":true,
        "variables":enc.solver.n_vars(), "clauses":enc.solver.n_clauses(),
        "xors":enc.solver.n_xors(), "domain_clauses":size.clauses,
        "xcnf_bytes":xcnf.len(),
        "xcnf_blake3":blake3::hash(xcnf.as_bytes()).to_hex().to_string(),
        "build_export_wall_ms_L0":started.elapsed().as_secs_f64()*1000.0,
        "verified_relations":Value::Null,
        "useful_rank":Value::Null,
        "selected_best_total_runtime":Value::Null,
    }))
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, path, output] => run(path, output)?,
        [_, mode, path, output] if mode == "--natural" => run_natural(path, output)?,
        _ => {
            return Err(
                "usage: n83_chain_xcnf_control [--natural] DOMAINS_JSON FRESH_OUTPUT_XCNF".into(),
            )
        }
    };
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}
