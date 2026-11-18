//! Separate source/graph/rank/full-affine qualifier for the new three-arm trial.
#![allow(clippy::duplicate_mod)]
#[cfg(test)]
#[allow(dead_code, unused_imports)] // Standalone example's reexport when nested for unit fixtures.
#[path = "jv_sieve_quadratic_pair_relation_controls.rs"]
mod admission_controls;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_pipeline_check.rs"]
mod check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_collection_check.rs"]
mod collection_check;
#[allow(dead_code, clippy::type_complexity)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder_driver.rs"]
mod driver;
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_enumeration_check.rs"]
mod enumeration_math_check;
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_graph_check.rs"]
mod graph_math_check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_pipeline_check.rs"]
mod pair_check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_collection_check.rs"]
mod pair_collection_check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_driver.rs"]
mod pair_driver;
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_certificate_check.rs"]
mod pair_math_check;
#[allow(dead_code, clippy::too_many_arguments)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder.rs"]
mod remainder;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_math_check.rs"]
mod sparse_math_check;
// The production wire type is deserialized only. Independent arithmetic and
// graph replay do not invoke its constructors, splitter or replay helpers.
pub use crypto_lib::cryptanalysis;
#[allow(dead_code, clippy::too_many_arguments)]
#[path = "../src/cryptanalysis/jv_principal_relation.rs"]
mod principal_relation;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_quadratic_pair_graph.rs"]
mod quadratic_pair_graph;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_quadratic_pair_relation.rs"]
mod quadratic_pair_relation;
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: CAP_CHECK COLLECTED_STAGE NEW_OUTPUT".into());
    }
    let stage = Path::new(&a[0]);
    let run = stage.join("returned/task/run");
    let read =
        |name: &str| -> Result<Value> { Ok(serde_json::from_slice(&fs::read(run.join(name))?)?) };
    let deletion: Value = serde_json::from_slice(&fs::read(stage.join("deletion.json"))?)?;
    if deletion["delete_http"] != 204
        || deletion["independent_get_http"] != 404
        || fs::read_to_string(run.join("analysis-original.exit-code.txt"))?.trim() != "1"
    {
        return Err("original failed qualification/cleanup".into());
    }
    let public = read("public.json")?;
    let fixture = read("fixture.json")?;
    let sha = driver::digest(&fs::read(run.join("public.json"))?);
    let answer = fixture["expected_scalar"]
        .as_u64()
        .ok_or("fixture answer")?;
    let mut proofs = Vec::new();
    for (i, role) in ["original-ic", "quadratic-table", "grounded-pairs"]
        .iter()
        .enumerate()
    {
        if fs::read_to_string(run.join(format!("{role}.exit-code.txt")))?.trim() != "0" {
            return Err("worker process failure".into());
        }
        let bytes = fs::read(run.join(format!("{role}.raw.json")))?;
        let w: Value = serde_json::from_slice(&bytes)?;
        if w["source_revision"] != "f58a7eb96be9fe47c40f5ef5dcaabe74e2392854"
            || w["seed"] != 389
            || w["public"] != public
            || w["solver_answer_placeholder"] != 0
        {
            return Err("original worker identity".into());
        }
        let proof = match i {
            0 => {
                if w["actual_scalar"] != answer || w["verified"] != true || w["capped"] != false {
                    return Err("original IC solve".into());
                }
                json!({"accounting":check::full::counter_check(&w,1009,263807419614139553)?})
            }
            1 => {
                let c = check::collection(&w, &sha)?;
                let cp = collection_check::qualify_sparse(&c, 389, &sha, &w["work"]["collection"])?;
                check::pipeline(&w, &c, &cp, answer, 8192, 32)?
            }
            _ => {
                if w["method"] != "quadratic-pairs"
                    || w["verified"] != false
                    || w["capped"] != true
                    || !w["actual_scalar"].is_null()
                    || w["work"]["collection_rank_ready"] != false
                    || w["work"]["collection"]["stop"] != "split trace resource cap"
                    || w["work"]["descent_residuals"] != 0
                {
                    return Err("capped pair endpoint".into());
                }
                let c = pair_check::collection(&w, &sha)?;
                let cp =
                    pair_collection_check::qualify_sparse(&c, 389, &sha, &w["work"]["collection"])?;
                let mut rank = pair_collection_check::Rank::default();
                for projection in &c.projections {
                    let mut row = vec![0; c.base.len()];
                    for &(j, v) in &projection.terms {
                        row[j] = v;
                    }
                    rank.insert(row, c.public.l)?;
                }
                json!({"component_collection":cp,"actual_independent_row_rank":rank.basis.len(),
                    "producer_sparse_rank_lower_bound":c.counts.rank,"target_rank":c.rank_target,
                    "no_candidate_scalar":true,"full_candidate_pipeline_qualified":false})
            }
        };
        proofs.push(json!({"mode":role,"original_sha256":driver::digest(&bytes),"proof":proof}));
    }
    let output = json!({"schema":"cover.quadratic-pair.cap-component-check/v1",
        "source_revision":"f58a7eb96be9fe47c40f5ef5dcaabe74e2392854","outcomes":proofs,
        "original_frozen_qualifier_exit_code":1,"original_failure_preserved":true,
        "postexecution_mathematics_only":true,"scientific_worker_invoked":false,
        "candidate_scalar_recovery":false,"complete_cost":false,"total_bitops":null,
        "cost_ratios":null,"rho_parity":false,"independent_signed_verdict":false});
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[1])?;
    serde_json::to_writer_pretty(&mut f, &output)?;
    f.write_all(b"\n")?;
    Ok(())
}
