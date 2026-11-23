//! Ownership-corrected native entrypoint; frozen trial42 science stays unchanged.
#![allow(clippy::duplicate_mod)]
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/owned_baseline/research/extension_ic_graph_20261115/anchored_descent.rs"]
mod anchored;
#[allow(dead_code)] // Frozen shared projector also serves the separate input-preparation tool.
#[path = "../research/extension_ic_graph_20261115/owned_baseline/research/extension_ic_graph_20261115/anchored_input.rs"]
mod anchored_input;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/owned_baseline/research/extension_ic_graph_20261115/extension_pipeline_collection_check.rs"]
mod collection_check;
#[path = "../research/extension_ic_graph_20261115/owned_baseline/research/extension_ic_graph_20261115/anchored_driver.rs"]
mod component;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/owned_baseline/research/extension_ic_graph_20261115/extension_remainder_driver.rs"]
mod driver;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/owned_baseline/research/extension_ic_graph_20261115/anchored_nagao.rs"]
mod nagao;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/owned_baseline/research/extension_ic_graph_20261115/extension_remainder.rs"]
mod remainder;
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        return Err("usage: OWNED_WORKER MODE INPUT NEW_OUTPUT".into());
    }
    let source = std::env::var("IC_SOURCE_REVISION")?;
    let bytes = std::fs::read(&args[1])?;
    // Same pattern as the existing production comparator: claim and count on
    // the Rayon worker that actually executes the nested F4 parallel paths.
    // Pool construction is common harness setup, outside the component interval.
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build()?;
    let result = pool.install(|| component::run(&bytes, &args[0], &source))?;
    let mut out = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args[2])?,
    );
    serde_json::to_writer(&mut out, &result)?;
    out.write_all(b"\n")?;
    out.flush()?;
    println!(
        "{}",
        serde_json::json!({"mode":args[0],"residuals":result["trials"].as_array().map(Vec::len),"rank":result["rank"],"verified":result["verified"],"foreign_thread_hooks":result["foreign_thread_hooks"]})
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn counted_owner_is_the_single_rayon_worker_for_nested_parallel_hooks() {
        use crypto_lib::cryptanalysis::jv_bitops as work;
        use rayon::prelude::*;
        let caller = std::thread::current().id();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .unwrap();
        pool.install(|| {
            assert_ne!(std::thread::current().id(), caller);
            work::claim();
            work::set_moduli(101, 103);
            work::enable(true);
            let before = work::snapshot();
            (0..17).into_par_iter().for_each(|_| {
                work::hook_mul(101);
                work::hook_add(103);
            });
            let delta = work::snapshot().since(&before);
            work::enable(false);
            let counters = work::prims_json(&delta.total());
            assert_eq!(counters["field_mul"], 17);
            assert_eq!(counters["modr_add"], 17);
            assert_eq!(work::foreign(), 0);
        });
    }
}
