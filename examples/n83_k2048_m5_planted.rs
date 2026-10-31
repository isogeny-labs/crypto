//! Exact N83 five-summand positive control on the two K2048 bases.
//! This constructs its own target from five subgroup representatives.
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::binary_semaev_chain_sat::ChainedS3Encoding;
use crypto_lib::cryptanalysis::koblitz_fast_arith::FastBinaryCurve128;
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use crypto_lib::cryptanalysis::sat::{RootPropagationResult, SolveResult};
use flate2::read::GzDecoder;
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::{Duration, Instant};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const ORDER: &str = "2417851639230796216685689";
const U_SHA256: &str = "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a";
const D_SHA256: &str = "2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7";
const INDICES: [usize; 5] = [0, 17, 53, 97, 211];

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn decimal(value: &str) -> Result<BigUint> {
    let number = BigUint::parse_bytes(value.as_bytes(), 10).ok_or("invalid decimal")?;
    if number.bits() > 83 || number.to_string() != value {
        return Err("noncanonical 83-bit decimal".into());
    }
    Ok(number)
}

fn coordinates(point: &BinaryPoint) -> Result<Value> {
    let BinaryPoint::Affine { x, y } = point else {
        return Err("expected affine point".into());
    };
    Ok(json!([
        x.to_biguint().to_string(),
        y.to_biguint().to_string()
    ]))
}

fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: n83_k2048_m5_planted ARM OBJECT OUTPUT".into());
    }
    let arm = args[1].as_str();
    let expected_hash = match arm {
        "U" => U_SHA256,
        "D" => D_SHA256,
        _ => return Err("arm must be U or D".into()),
    };
    let output = Path::new(&args[3]);
    if output.exists() {
        return Err("output already exists".into());
    }
    let started = Instant::now();
    let compressed = fs::read(&args[2])?;
    if sha256(&compressed) != expected_hash {
        return Err("factor-base object hash differs".into());
    }
    let mut reader = BufReader::new(GzDecoder::new(&compressed[..]));
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let header: Value = serde_json::from_str(&line)?;
    let policy = if arm == "U" {
        "public_x_hash"
    } else {
        "direct_subgroup_x_d18_prefix2048"
    };
    if header["curve_slug"] != CURVE
        || header["policy"] != policy
        || header["closure"] != "signed_frobenius"
        || header["orbit_columns"] != 2048
        || header["point_count"] != 339_968
    {
        return Err("factor-base header differs".into());
    }
    let representatives = header["representatives"]
        .as_array()
        .ok_or("representatives missing")?;
    if representatives.len() != 2048 {
        return Err("representative count differs".into());
    }
    let kc = KoblitzCurve::known_n83_k0().ok_or("N83 curve unavailable")?;
    if kc.label() != CURVE || kc.subgroup_order.to_string() != ORDER {
        return Err("curve identity differs".into());
    }
    let fast =
        FastBinaryCurve128::new(&kc.curve.irreducible, 0).ok_or("fast N83 curve unavailable")?;
    let mut chosen_x = Vec::with_capacity(5);
    let mut points = Vec::with_capacity(5);
    let mut sum = BinaryPoint::Infinity;
    let mut fast_sum = None;
    for index in INDICES {
        let rep = representatives[index]
            .as_array()
            .ok_or("representative is not a point")?;
        if rep.len() != 2 {
            return Err("representative width differs".into());
        }
        let x = decimal(rep[0].as_str().ok_or("representative x absent")?)?;
        let x_word: u128 = x.clone().try_into()?;
        let selected = fast
            .points_with_x(1, x_word)
            .into_iter()
            .find(|point| fast.scalar_mul(*point, &kc.subgroup_order).is_none())
            .ok_or("representative lacks subgroup lift")?;
        let (sx, sy) = selected.ok_or("representative lift is infinity")?;
        let point = BinaryPoint::Affine {
            x: fast.element(sx),
            y: fast.element(sy),
        };
        if !kc.curve.is_on_curve(&point)
            || kc.mul(&point, &kc.subgroup_order) != BinaryPoint::Infinity
        {
            return Err("generic subgroup replay failed".into());
        }
        chosen_x.push(F2mElement::from_biguint(&x, 83));
        points.push(coordinates(&point)?);
        sum = kc.add(&sum, &point);
        fast_sum = fast.add(fast_sum, selected);
    }
    let BinaryPoint::Affine { x: target_x, .. } = &sum else {
        return Err("planted target is infinity".into());
    };
    let (fast_target_x, fast_target_y) = fast_sum.ok_or("fast sum is infinity")?;
    if fast_target_x != fast.word(target_x)
        || coordinates(&sum)? != json!([fast_target_x.to_string(), fast_target_y.to_string()])
    {
        return Err("fast and generic target sums differ".into());
    }
    if !kc.curve.is_on_curve(&sum) || kc.mul(&sum, &kc.subgroup_order) != BinaryPoint::Infinity {
        return Err("planted target subgroup replay failed".into());
    }
    let fixture_ms = started.elapsed().as_secs_f64() * 1000.0;
    let model_start = Instant::now();
    let mut encoding = ChainedS3Encoding::build(
        5,
        &kc.curve.irreducible,
        &kc.curve.b,
        Some(target_x),
        0,
        300_000,
    )
    .map_err(|_| "chained-S3 model build failed")?;
    let all_x: Vec<_> = representatives
        .iter()
        .map(|rep| {
            let pair = rep.as_array().ok_or("representative shape differs")?;
            let x = decimal(pair[0].as_str().ok_or("representative x absent")?)?;
            Ok(x)
        })
        .collect::<Result<_>>()?;
    encoding
        .constrain_frobenius_orbit_summands(&all_x, 300_000, 8_000_000)
        .map_err(|_| "orbit-domain install failed")?;
    let unpinned_variables = encoding.solver.n_vars();
    let unpinned_clauses = encoding.solver.n_clauses();
    let unpinned_xors = encoding.solver.n_xors();
    for (block, x) in chosen_x.iter().enumerate() {
        encoding
            .pin_x_block(block, x)
            .map_err(|_| "summand pin failed")?;
    }
    let model_ms = model_start.elapsed().as_secs_f64() * 1000.0;
    encoding.solver.conflict_budget = 5_000;
    encoding.solver.deadline = Some(started + Duration::from_secs(240));
    let search_start = Instant::now();
    let root = encoding.solver.propagate_root();
    let answer = if root == RootPropagationResult::Unsat {
        SolveResult::Unsat
    } else {
        encoding.solver.solve()
    };
    let search_ms = search_start.elapsed().as_secs_f64() * 1000.0;
    let model_verified = answer == SolveResult::Sat
        && encoding.verify_model()
        && encoding.decode_x_blocks().len() == 9
        && encoding.decode_x_blocks()[..5] == chosen_x[..]
        && encoding.decode_x_blocks()[8] == *target_x;
    let receipt = json!({
        "schema":"n83.k2048-m5-planted/v1","arm":arm,"curve_id":"EC1N83Ckb0h252dacc102a4",
        "object_sha256":expected_hash,"policy":policy,"indices":INDICES,
        "summands":points,"target":coordinates(&sum)?,
        "group_sum_verified":true,"subgroup_verified":true,
        "pinned_summand_x":chosen_x.iter().map(|x| x.to_biguint().to_string()).collect::<Vec<_>>(),
        "unpinned_variables":unpinned_variables,"unpinned_clauses":unpinned_clauses,
        "unpinned_xor_rows":unpinned_xors,
        "root_status":format!("{root:?}"),"solve_status":format!("{answer:?}"),
        "model_verified":model_verified,"conflicts":encoding.solver.conflicts(),
        "fixture_ms":fixture_ms,"model_ms":model_ms,"search_ms":search_ms,
        "process_wall_ms":started.elapsed().as_secs_f64()*1000.0,
        "source_blake3":blake3::hash(include_bytes!("n83_k2048_m5_planted.rs")).to_hex().to_string(),
        "binary_sha256":sha256(&fs::read(std::env::current_exe()?)?),
        "factor_base_selection":Value::Null,"single_target_online_speedup":Value::Null
    });
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    serde_json::to_writer_pretty(&mut file, &receipt)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!("{}", serde_json::to_string(&receipt)?);
    if !model_verified {
        return Err("planted SAT model did not verify under cap".into());
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("N83 K2048 planted control: {error}");
        std::process::exit(1);
    }
}
