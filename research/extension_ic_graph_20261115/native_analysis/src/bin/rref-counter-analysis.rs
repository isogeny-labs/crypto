//! Recorded-matrix arithmetic conservation; never an end-to-end repricing.
#[allow(dead_code)]
#[path = "two-lp-component-analysis.rs"]
mod shared;
use serde_json::{json, Value};
use shared::{counts, n, price, Field, Widths};
use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};
fn analyze(result: &Value, original: &Value, bytes: &[u8], plan: &Value) -> Result<Value, String> {
    let p = n(&original["public"], "p")?;
    let l = n(&original["public"], "l")?;
    let target = n(&original["work"]["final_projection"], "target_column")?;
    let rank = n(&original["work"], "last_rank")?;
    let products = n(&original["work"], "solver_modular_muls")?;
    let normalization = rank
        .checked_mul(target.checked_add(2).ok_or("column overflow")?)
        .ok_or("normalization overflow")?;
    let subtractions = products
        .checked_sub(normalization)
        .ok_or("normalization exceeds products")?;
    if !(101..=1511).contains(&p)
        || !(3..1u64 << 63).contains(&l)
        || target > 4096
        || rank > target + 1
        || plan["driver_kind"] != "recorded-rref-counter-validation"
        || plan["allow_comparative_claims"] != false
        || plan["automatic_retries"] != 0
        || original["source_revision"] != plan["original_source_revision"]
        || original["actual_scalar"] != plan["expected_scalar"]
        || original["verified"] != true
        || products != n(plan, "expected_legacy_products")?
        || rank != n(plan, "expected_rank")?
        || target + 1 != n(plan, "expected_columns")?
        || result["schema"] != "cover.recorded-rref-counter-replay/v1"
        || result["source_revision"] != plan["source_revision"]
        || result["original_source_revision"] != original["source_revision"]
        || result["original_raw_blake3"] != blake3::hash(bytes).to_hex().to_string()
        || result["p"] != p
        || result["l"] != l
        || result["columns"] != target + 1
        || result["target_column"] != target
        || result["rank"] != rank
        || result["legacy_products"] != products
        || result["normalization_products"] != normalization
        || result["modular_subtractions"] != subtractions
        || result["actual_scalar"] != plan["expected_scalar"]
        || result["expected_scalar"] != plan["expected_scalar"]
        || [
            "counter_conservation",
            "same_original_target",
            "same_original_legacy_products",
            "same_original_rank",
        ]
        .iter()
        .any(|k| result[*k] != true)
        || result["complete_cost"] != false
        || !result["total_bitops"].is_null()
        || result["comparative_eligibility"] != false
        || result["new_full_dlp_trial"] != false
        || result["rho_parity"] != false
        || result["foreign_hook_calls"] != 0
        || result["other_modulus_ops"]
            .as_array()
            .is_none_or(|a| a.len() != 15 || a.iter().any(|v| v != 0))
        || result["known_uncounted"]
            .as_array()
            .is_none_or(|a| a.is_empty())
    {
        return Err("recorded replay identity/conservation/coverage".into());
    }
    let primitives = counts(&result["observed_primitives"])?;
    for (key, &count) in &primitives {
        let expected = match key.as_str() {
            "modr_mul" => products,
            "modr_add" => subtractions,
            "modr_inv" => rank,
            _ => 0,
        };
        if count != expected {
            return Err(format!("primitive {key}: {count} != {expected}"));
        }
    }
    let widths = Widths {
        field: Field::Prime {
            bits: 64 - p.leading_zeros(),
        },
        r_bits: 64 - l.leading_zeros(),
    };
    let subtotal = price(&primitives, &widths)?;
    if result["observed_primitive_subtotal_bitops"] != subtotal.to_string() {
        return Err("arithmetic repricing".into());
    }
    Ok(
        json!({"schema":"cover.recorded-rref-counter-analysis/v1","source_revision":plan["source_revision"],"original_source_revision":original["source_revision"],"original_raw_blake3":result["original_raw_blake3"],"actual_scalar":result["actual_scalar"],"rank":rank,"columns":target+1,"rows_used":result["rows_used"],"rows_available":result["rows_available"],"legacy_products":products,"normalization_products":normalization,"modular_subtractions":subtractions,"observed_primitives":primitives,"observed_arithmetic_subtotal_bitops":subtotal.to_string(),"recorded_counter_validation_passed":true,"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"new_full_dlp_trial":false,"rho_parity":false,"independent_signed_verdict":false,"no_end_to_end_composition":true,"flow":{"planned":1,"attempted":1,"completed":1,"excluded":0,"automatic_retries":0}}),
    )
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: rref-counter-analysis RUN-DIR NEW-SUMMARY.json".into());
    }
    let run = Path::new(&args[1]);
    if fs::read_to_string(run.join("exit-code.txt"))?.trim() != "0" {
        return Err("failed/censored replay".into());
    }
    let plan: Value = serde_json::from_reader(File::open(run.join("runtime-plan.json"))?)?;
    let result: Value = serde_json::from_reader(File::open(run.join("raw.json"))?)?;
    let bytes = fs::read(run.join("input.raw.json"))?;
    if bytes.len() > 32 * 1024 * 1024 {
        return Err("input bound".into());
    }
    let original = serde_json::from_slice(&bytes)?;
    let flow: Vec<Value> = fs::read_to_string(run.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    if flow.len() != 2
        || flow[0]["record_type"] != "execution_start"
        || flow[1]["record_type"] != "execution"
        || flow[1]["exit_code"] != 0
        || flow
            .iter()
            .any(|v| v["source_revision"] != plan["source_revision"])
    {
        return Err("flow/source".into());
    }
    let summary = analyze(&result, &original, &bytes, &plan)?;
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    serde_json::to_writer_pretty(&mut out, &summary)?;
    writeln!(out)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_hand_counts_keep_normalizations_and_inversions_separate() {
        let original = json!({"source_revision":"1".repeat(40),"public":{"p":103,"l":101},"actual_scalar":1,"verified":true,"work":{"last_rank":2,"solver_modular_muls":11,"final_projection":{"target_column":1}}});
        let bytes = serde_json::to_vec(&original).unwrap();
        let plan = json!({"driver_kind":"recorded-rref-counter-validation","allow_comparative_claims":false,"automatic_retries":0,"source_revision":"0".repeat(40),"original_source_revision":"1".repeat(40),"expected_scalar":1,"expected_legacy_products":11,"expected_rank":2,"expected_columns":2});
        let mut prim: BTreeMap<String, u64> =
            shared::KINDS.into_iter().map(|k| (k.into(), 0)).collect();
        prim.insert("modr_mul".into(), 11);
        prim.insert("modr_add".into(), 5);
        prim.insert("modr_inv".into(), 2);
        let widths = Widths {
            field: Field::Prime { bits: 7 },
            r_bits: 7,
        };
        let priced = price(&prim, &widths).unwrap().to_string();
        let result = json!({"schema":"cover.recorded-rref-counter-replay/v1","source_revision":"0".repeat(40),"original_source_revision":"1".repeat(40),"original_raw_blake3":blake3::hash(&bytes).to_hex().to_string(),"p":103,"l":101,"columns":2,"target_column":1,"rank":2,"legacy_products":11,"normalization_products":6,"modular_subtractions":5,"actual_scalar":1,"expected_scalar":1,"counter_conservation":true,"same_original_target":true,"same_original_legacy_products":true,"same_original_rank":true,"observed_primitives":prim,"observed_primitive_subtotal_bitops":priced,"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"new_full_dlp_trial":false,"rho_parity":false,"foreign_hook_calls":0,"other_modulus_ops":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"known_uncounted":["native scans"]});
        assert_eq!(
            analyze(&result, &original, &bytes, &plan).unwrap()["modular_subtractions"],
            5
        );
        for key in ["modr_mul", "modr_add", "modr_inv"] {
            let mut bad = result.clone();
            bad["observed_primitives"][key] = json!(0);
            assert!(analyze(&bad, &original, &bytes, &plan).is_err());
        }
        let mut promoted = result;
        promoted["complete_cost"] = json!(true);
        assert!(analyze(&promoted, &original, &bytes, &plan).is_err());
    }
    use std::collections::BTreeMap;
}
