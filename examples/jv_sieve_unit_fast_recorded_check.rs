//! Algebraic replay only: no F4 solve, fresh curve, yield or elapsed measurement.
pub use crypto_lib::cryptanalysis;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_nagao_unit_fast_elimination.rs"]
mod fast;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_nagao_unit_elimination.rs"]
mod nagao_unit_elimination;
use crypto_lib::cryptanalysis::f4_fp::Poly;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write};
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: REPLAY ORIGINAL_UNIT_RAW NEW_PROOF".into());
    }
    let bytes = fs::read(&a[0])?;
    let raw: Value = serde_json::from_slice(&bytes)?;
    let p = raw["public"]["p"].as_u64().ok_or("field")?;
    let attempts = raw["work"]["nagao_attempts"].as_array().ok_or("attempts")?;
    if attempts.len() != 3415 {
        return Err("registered recorded cohort".into());
    }
    let mut roots = 0;
    for (i, t) in attempts.iter().enumerate() {
        if t["residual_index"] != json!(i) {
            return Err("complete prefix".into());
        }
        let input: Vec<Poly> = serde_json::from_value(t["audit"]["original"].clone())?;
        let old = nagao_unit_elimination::UnitReduction::new(&input, p)?;
        let new = fast::UnitReduction::new(&input, p)?;
        if old.polynomials() != new.polynomials()
            || t["audit"]["reduced_sha256"] != digest(&serde_json::to_vec(new.polynomials())?)
        {
            return Err(format!("different reduced system at{i}").into());
        }
        let lifted: Vec<Vec<u64>> = serde_json::from_value(t["audit"]["lifted_roots"].clone())?;
        for x in lifted {
            if x.len() != 6 || old.lift(&x[..5])? != x || new.lift(&x[..5])? != x {
                return Err("lift equality".into());
            }
            roots += 1;
        }
    }
    if roots != 3419 {
        return Err("recorded root count".into());
    }
    let result = json!({"schema":"cover.unit-fast.recorded-algebra/v1","input_sha256":digest(&bytes),"p":p,
        "systems":attempts.len(),"roots":roots,"reduced_polynomials_byte_equal":true,"every_captured_digest_equal":true,
        "all_lifts_and_original_equations_equal":true,"f4_solves":0,"new_natural_yield_calls":0,"elapsed_claim":false,
        "complete_cost":false,"rho_parity":false,"scope":"exact algebraic replay of every retained Run51 input; baseline geometry already bound by separate original-binary replay"});
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&a[1])?;
    serde_json::to_writer_pretty(&mut file, &result)?;
    file.write_all(b"\n")?;
    println!("3415 reduced systems and3419 lifts agree exactly; zero F4 solves");
    Ok(())
}
