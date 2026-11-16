//! Replay an immutable recorded affine matrix; no relation search/descent.
use clap::Parser;
use crypto_lib::cryptanalysis::{
    jv_bitops::{self as work, Kind, Phase},
    jv_sieve::two_large_prime::CountedRelationSystem,
};
use serde_json::{json, Value};
use std::{fs::OpenOptions, path::PathBuf};
#[derive(Parser)]
struct Args {
    #[arg(long)]
    input: PathBuf,
    #[arg(long)]
    out: PathBuf,
    #[arg(long)]
    source_revision: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if args.source_revision.len() != 40
        || !args.source_revision.bytes().all(|c| c.is_ascii_hexdigit())
    {
        return Err("source SHA required".into());
    }
    let out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args.out)?;
    let bytes = std::fs::read(&args.input)?;
    if bytes.len() > 32 * 1024 * 1024 {
        return Err("input bound".into());
    }
    let original: Value = serde_json::from_slice(&bytes)?;
    let w = &original["work"];
    let proof = &w["final_projection"];
    let p = original["public"]["p"].as_u64().ok_or("p")?;
    let l = original["public"]["l"].as_u64().ok_or("l")?;
    let target = proof["target_column"].as_u64().ok_or("target")? as usize;
    if original["verified"] != true || target > 4096 || !(3..1u64 << 63).contains(&l) {
        return Err("invalid original replay source".into());
    }
    let expected = original["actual_scalar"]
        .as_u64()
        .ok_or("original scalar")?;
    let certificates = proof["certificates"].as_array().ok_or("certificates")?;
    if certificates.len() > 4096 {
        return Err("matrix row bound".into());
    }
    let mut rows = Vec::new();
    for cert in certificates {
        let sparse: Vec<(usize, u64)> = serde_json::from_value(cert["small"].clone())?;
        if sparse.iter().any(|&(c, v)| c > target || v == 0 || v >= l)
            || sparse.windows(2).any(|t| t[0].0 >= t[1].0)
        {
            return Err("canonical matrix row".into());
        }
        let mut dense = vec![0; target + 1];
        for (c, v) in sparse {
            dense[c] = v;
        }
        let rhs = cert["rhs"].as_u64().filter(|&v| v < l).ok_or("RHS")?;
        rows.push((dense, rhs));
    }
    work::enable(true);
    work::claim();
    work::set_moduli(p, l);
    let other_before = work::snapshot_other();
    let before = work::snapshot();
    let mut rows_used = 0;
    let mut system = CountedRelationSystem::new(l, target + 1);
    {
        let _g = work::enter(Phase::LinearAlgebra);
        for (row, rhs) in rows {
            system.insert(row, rhs);
            rows_used += 1;
            if system.solved(target).is_some() {
                break;
            }
        }
    }
    let tally = work::snapshot().since(&before).total();
    let scalar = system.solved(target);
    let widths = work::widths(p, l);
    let result = json!({"schema":"cover.recorded-rref-counter-replay/v1","source_revision":args.source_revision,"original_source_revision":original["source_revision"],
        "original_raw_blake3":blake3::hash(&bytes).to_hex().to_string(),"p":p,"l":l,"rows_available":certificates.len(),"rows_used":rows_used,"columns":target+1,
        "target_column":target,"expected_scalar":expected,"actual_scalar":scalar,"rank":system.rank(),"legacy_products":system.ops(),"normalization_products":system.normalization_products(),"modular_subtractions":system.subtraction_ops(),
        "observed_primitives":work::prims_json(&tally),"observed_primitive_subtotal_bitops":work::price(&tally,&widths).map_err(|e|format!("primitive price: {e:?}"))?.to_string(),
        "counter_conservation":tally.get(Kind::ModrMul)==system.ops()&&tally.get(Kind::ModrAdd)==system.subtraction_ops()&&tally.get(Kind::ModrInv)==system.rank() as u64,
        "same_original_target":scalar==Some(expected),"same_original_legacy_products":w["solver_modular_muls"]==system.ops(),"same_original_rank":w["last_rank"]==system.rank() as u64,
        "other_modulus_ops":work::snapshot_other().iter().zip(other_before).map(|(&a,b)|a.checked_sub(b)).collect::<Vec<_>>(),"foreign_hook_calls":work::foreign(),
        "complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"new_full_dlp_trial":false,"rho_parity":false,
        "excluded":"original relation search/descent/group replay and JSON/matrix mapping; immutable29proof replay only",
        "known_uncounted":["RREF native coefficient scans","fixed-factor quotient/preparation internals","native allocations/copies/solver-control"]});
    serde_json::to_writer(out, &result)?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}
