//! Independent flat-index replay of newly projected, already validated sources.
//! Original bytes are read-only; no relation finder, DLP or timing is launched.
pub use crypto_lib::cryptanalysis;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_deferred_pair_graph.rs"]
mod deferred_pair_graph;
#[path = "../research/extension_ic_graph_20261115/deferred_pair_graph_check.rs"]
mod independent;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_driver.rs"]
mod pair_driver;
#[allow(dead_code, clippy::too_many_arguments)]
#[path = "../src/cryptanalysis/jv_principal_relation.rs"]
mod principal_relation;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_quadratic_pair_graph.rs"]
mod quadratic_pair_graph;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_quadratic_pair_relation.rs"]
mod quadratic_pair_relation;
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{fs, io::Write};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Deserialize)]
struct Raw {
    work: Work,
}
#[derive(Deserialize)]
struct Work {
    collection: Collection,
}
#[derive(Deserialize)]
struct Collection {
    sources: Vec<pair_driver::Source>,
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: RECORDED_CHECK ORIGINAL49_RAW NEW_OUTPUT".into());
    }
    let bytes = fs::read(&a[0])?;
    let sha = format!("{:x}", Sha256::digest(&bytes));
    if sha != "e767e8a175a475645b3a899a32c2f0643ce20a750693aaa8797697934eb4bc56" {
        return Err("original49hash".into());
    }
    let mut raw: Raw = serde_json::from_slice(&bytes)?;
    drop(bytes);
    let sources = &mut raw.work.collection.sources;
    let mut producer = deferred_pair_graph::Graph::new(deferred_pair_graph::Options::default())?;
    let mut replay = independent::Replay::default();
    let mut full = 0;
    let mut first = None;
    for id in 0..sources.len() {
        let large: Vec<_> = sources[id]
            .certificate
            .remainders
            .iter()
            .map(|r| quadratic_pair_graph::LargeTerm {
                class: r.key,
                coefficient: r.coefficient,
            })
            .collect();
        // This derivative exists only in memory; never rewrites the raw input.
        sources[id].graph = serde_json::to_value(producer.accept(id, &large)?)?;
        full += replay.accept(&sources[..=id])?.len();
        if full >= 516 && first.is_none() {
            first = Some(json!({"activation_source":id,"b_index":sources[id].b_index,"rows":full}));
        }
    }
    if full != 865 || replay.stats() != serde_json::to_value(producer.stats())? {
        return Err("full queue state disagreement".into());
    }
    let result = json!({"schema":"cover.deferred-pair.independent-recorded-graph/v1","original_sha256":sha,
        "sources":sources.len(),"independent_full_incidence_witnesses":full,"first_516_rows":first,
        "producer_and_flat_index_stats_identical":true,"every_event_order_and_incidence_checked":true,
        "graph_stats":replay.stats(),"original_inputs_unchanged":true,"recorded_source_projection_only":true,
        "natural_yield":null,"elapsed_performance":null,"complete_cost":false,"rho_parity":false});
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[1])?;
    serde_json::to_writer_pretty(&mut f, &result)?;
    f.write_all(b"\n")?;
    Ok(())
}
