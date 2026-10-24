//! Standalone native input check; no SAT construction or solver timing.
//! The complete library-backed construction check remains required.
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;

pub fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: root_inventory SELECTED_CSV NEW_JSON".into());
    }
    let input = fs::read(&args[1])?;
    let input_hash = format!("{:x}", Sha256::digest(&input));
    if input_hash != "534e58ced979e05ec747eb462aa1d63135d8004813d094c522a4d7160c03c4a4" {
        return Err("representative CSV digest differs".into());
    }
    let mut lines = std::str::from_utf8(&input)?.lines();
    if lines.next() != Some("orbit_key_decimal,source_x_decimal,source_x_multiplicity") {
        return Err("CSV header differs".into());
    }
    let mut keys = BTreeSet::new();
    let mut roots = BTreeSet::new();
    for line in lines {
        let fields: Vec<_> = line.split(',').collect();
        if fields.len() != 3 {
            return Err("CSV width differs".into());
        }
        let key: u128 = fields[0].parse()?;
        let root: u128 = fields[1].parse()?;
        let multiplicity: u32 = fields[2].parse()?;
        if key >= 1 << 83
            || root >= 1 << 18
            || multiplicity == 0
            || !keys.insert(key)
            || !roots.insert(root)
        {
            return Err("invalid or duplicate CSV row".into());
        }
    }
    if roots.len() != 16_627 {
        return Err("representative count differs".into());
    }
    let first = *roots.first().ok_or("empty source")?;
    // Independent u128 XOR-mask implementation of the BigUint/all-bit check
    // used by the full construction tool and native worker.
    let varying = roots.iter().fold(0u128, |mask, root| mask | (first ^ root));
    let shared: Vec<_> = (0..83)
        .filter(|bit| varying & (1u128 << bit) == 0)
        .map(|bit| json!({"bit":bit,"value":first & (1u128 << bit) != 0}))
        .collect();
    let reductions: Vec<_> = (2..=4)
        .map(|m| json!({"m":m,"derived_removed_root_implications":m*shared.len()*(roots.len()-1)}))
        .collect();
    let receipt = json!({
        "schema":"n83.shared-root-inventory/v1","status":"PASS_ROOT_INVENTORY",
        "representatives_sha256":input_hash,"columns":roots.len(),
        "minimum_root":roots.first().unwrap().to_string(),
        "maximum_root":roots.last().unwrap().to_string(),
        "shared_bits":shared,"varying_mask_decimal":varying.to_string(),
        "varying_bit_count":varying.count_ones(),"derived_clause_reductions":reductions,
        "source_sha256":format!("{:x}",Sha256::digest(include_bytes!("root_inventory.rs"))),
        "sat_construction_performed":false,"solver_search_performed":false,
        "actual_model_counts":null,"timing_comparison":null,
        "candidate_id":null,"selected_best_total_runtime":null
    });
    let mut output = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&args[2])?;
    serde_json::to_writer_pretty(&mut output, &receipt)?;
    output.write_all(b"\n")?;
    output.sync_all()?;
    Ok(())
}
