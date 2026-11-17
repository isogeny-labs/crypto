//! One process constructs and independently verifies the complete certificate.
//! It emits proof bytes for separate replay; parent owns resource/isolation caps.
use certified_relations::{replay, System};
use serde_json::json;
use std::{
    error::Error,
    fs,
    io::{Read, Write},
    path::Path,
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn save(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(bytes)?;
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage:fused-native compact|compact-sparse|compact-dag|packed-constructor-sparse|packed-constructor-dag|expanded|cubic-reference|cubic-dag|cubic-dag-pruned legacy|packed legacy|frontier|packed-products|packed-batch NEW_PROOF_DIR; original System on stdin".into());
    }
    let start = Instant::now();
    let mut original = Vec::new();
    std::io::stdin()
        .lock()
        .take((4 << 20) + 1)
        .read_to_end(&mut original)?;
    if original.len() > 4 << 20 {
        return Err("original byte cap".into());
    }
    let system: System = serde_json::from_slice(&original)?;
    let kernel = match args[3].as_str() {
        "legacy" => replay::NativeKernels::Legacy,
        "frontier" => replay::NativeKernels::BothUpdatesFrontier,
        "packed-products" => replay::NativeKernels::FrontierPackedProducts,
        "packed-batch" => replay::NativeKernels::FrontierPackedBatch,
        _ => return Err("kernel enum".into()),
    };
    if !["legacy", "packed"].contains(&args[2].as_str()) {
        return Err("checker enum".into());
    }
    if args[2] == "legacy" && system.prime != 2147483647 {
        return Err("legacy consumer supports only p31".into());
    }
    let construct = Instant::now();
    let (wire, constructor) = if args[1] == "packed-constructor-dag" {
        certified_relations::packed_constructor::produce_dag(&system, kernel)?
    } else if args[1] == "packed-constructor-sparse" {
        certified_relations::packed_constructor::produce_sparse(&system, kernel)?
    } else if args[1] == "compact-dag" {
        replay::produce_compact_dag_for_packed(&system, kernel)?
    } else if args[1] == "cubic-dag-pruned" {
        replay::produce_cubic_dag_pruned_for_packed(&system, kernel)?
    } else if args[1] == "cubic-dag" {
        (
            replay::produce_cubic_dag_for_packed(&system, kernel)?,
            json!({"method":"shared-cubic-Buchberger-DAG"}),
        )
    } else if args[1] == "cubic-reference" {
        (
            replay::produce_cubic_for_packed(&system, kernel)?,
            json!({"method":"two-direct-cubics-provenance-Buchberger"}),
        )
    } else {
        replay::produce_native_for_packed(&system, &args[1], kernel)?
    };
    let construct_ns = construct.elapsed().as_nanos();
    let verify = Instant::now();
    let checked = if args[1].starts_with("cubic-dag")
        || (args[1] == "compact-dag" || args[1] == "packed-constructor-dag")
    {
        if args[2] != "packed" {
            return Err("DAG requires explicit packed consumer".into());
        }
        relation_replay_checker::packed::verify_dag_json(&original, &wire)?
    } else if args[2] == "packed" {
        relation_replay_checker::packed::verify_json(&original, &wire)?
    } else {
        relation_replay_checker::verify_json(&original, &wire)?
    };
    let verify_ns = verify.elapsed().as_nanos();
    let out = Path::new(&args[4]);
    fs::create_dir(out)?;
    save(&out.join("original.json"), &original)?;
    save(&out.join("witness.json"), &wire)?;
    let whole_ns = start.elapsed().as_nanos();
    println!(
        "{}",
        json!({"status":"complete","prime":system.prime,"variables":system.variables.len(),"constructor":constructor,"checker":args[2],"kernel":args[3],"basis":checked.basis,"consumer_stats":checked.stats,"phases_diagnostic_ns":{"construct_and_wire":construct_ns.to_string(),"independent_replay":verify_ns.to_string(),"full_worker_including_proof_write":whole_ns.to_string()},"S_pair_and_input_containment_checks":true})
    );
    Ok(())
}
