use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use crypto_lib::cryptanalysis::relation_yield_pruning::{self, PruningInput};

/// Analyze one frozen pruning checkpoint and write a new report.
///
/// Refusing to overwrite matters here: a later look is a new evidence object,
/// not a replacement for the decision the earlier checkpoint produced.
pub fn run(input: &Path, out: &Path) -> Result<String, String> {
    let bytes = fs::read(input).map_err(|e| format!("{}: {e}", input.display()))?;
    let parsed: PruningInput =
        serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", input.display()))?;
    let report = relation_yield_pruning::analyse(&parsed)?;
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(out)
        .map_err(|e| format!("{}: {e}", out.display()))?;
    serde_json::to_writer_pretty(&mut file, &report).map_err(|e| e.to_string())?;
    file.write_all(b"\n").map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    serde_json::to_string(&serde_json::json!({
        "schema": report.schema,
        "look": report.look,
        "incumbent": report.incumbent,
        "candidates": report.candidates.len(),
        "out": out,
    }))
    .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refuses_to_replace_an_existing_report() {
        let root =
            std::env::temp_dir().join(format!("icprog-relation-pruning-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let input = root.join("input.json");
        let out = root.join("report.json");
        fs::write(
            &input,
            r#"{
              "schema":"ic.relation-pruning.input/v1",
              "look":1,
              "policy":{
                "familywise_alpha":0.05,
                "planned_candidates":1,
                "planned_episodes":1,
                "checkpoint_trials":[10],
                "min_trials_for_futility":10,
                "min_terminal_episodes":1,
                "min_full_rank_success":0.0,
                "dominance_margin":0.0
              },
              "observations":[{
                "candidate_id":"c0","episode_id":"s0","trials":10,
                "relation_hit_trials":2,"verified_relations":2,
                "relation_cap_per_trial":1,"independent_relations":2,
                "initial_rank":0,"rank":2,"target_rank":2,
                "factor_base_columns":2,"covered_columns":2,
                "reachable_rank_upper":2,"factor_hits":[1,1],
                "charged_cost":100,"terminal":true,"full_rank_verified":true,
                "invalid_relations":0,"inconsistent_relations":0,
                "verification_failures":0
              }]
            }"#,
        )
        .unwrap();
        run(&input, &out).unwrap();
        let error = run(&input, &out).unwrap_err();
        assert!(error.contains(&out.display().to_string()));
        fs::remove_dir_all(root).unwrap();
    }
}
