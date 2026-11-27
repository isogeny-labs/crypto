//! Additive partial-run replay: no solver calls, no replacement observations.
#[path = "../check.rs"]
mod check;
use crypto_lib::cryptanalysis::f4_fp::Poly;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};
fn read(p: &Path) -> Result<Vec<u8>, String> {
    fs::read(p).map_err(|e| format!("{}: {e}", p.display()))
}
fn load(p: &Path) -> Result<Value, String> {
    serde_json::from_slice(&read(p)?).map_err(|e| e.to_string())
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn outcome(
    p: u64,
    exit: i64,
    stderr: &str,
    observations_exist: bool,
) -> Result<&'static str, String> {
    if p == 9007199254740881
        && exit == 101
        && stderr.contains("f4_fp: the prime must be below 2^32")
        && !observations_exist
    {
        Ok("unsupported-prime: first arm panicked; remaining arms unexecuted")
    } else {
        Err("unsupported classification does not match retained execution evidence".into())
    }
}
fn run(study: &Path, out: &Path) -> Result<(), String> {
    let data = study.join("data");
    let roster = load(&data.join("ROSTER.json"))?;
    let mut cases = vec![];
    let mut completed = 0;
    let mut unsupported = 0;
    let mut lower = 0;
    let mut equal = 0;
    let mut higher = 0;
    let mut failed = 0;
    for c in roster["cases"].as_array().ok_or("cases")? {
        let id = c["id"].as_str().ok_or("id")?;
        let p = c["prime"].as_u64().ok_or("prime")?;
        let n = c["n"].as_u64().ok_or("n")? as usize;
        let dir = data.join(id);
        if load(&dir.join("case.json"))? != *c
            || Some(hash(&read(&dir.join("original.json"))?).as_str())
                != c["original_sha256"].as_str()
        {
            return Err("input/roster drift".into());
        }
        let refmeta = load(&dir.join("reference.json"))?;
        let checked = relation_replay_checker::packed::verify_dag_json(
            &read(&dir.join("reference-original.json"))?,
            &read(&dir.join("reference-proof.json"))?,
        )?;
        let rb: Vec<Poly> =
            serde_json::from_value(refmeta["basis"].clone()).map_err(|e| e.to_string())?;
        let mut independent: Vec<Poly> = checked
            .basis
            .iter()
            .map(|g| {
                g.iter()
                    .map(|(m, c)| (m.iter().map(|e| *e as u32).collect(), *c))
                    .collect()
            })
            .collect();
        let mut native = rb.clone();
        for row in &mut independent {
            row.sort()
        }
        for row in &mut native {
            row.sort()
        }
        independent.sort();
        native.sort();
        if independent != native {
            return Err("reference replay export".into());
        }
        check::validate(&rb, &rb, n, p, 5_000_000)?;
        let exit = std::str::from_utf8(&read(&study.join(format!("raw/{id}.exit")))?)
            .map_err(|e| e.to_string())?
            .trim()
            .parse::<i64>()
            .map_err(|e| e.to_string())?;
        if p == 9007199254740881 {
            let any = c["arms"].as_array().ok_or("arms")?.iter().any(|a| {
                dir.join(format!("{}.json", a["id"].as_str().unwrap()))
                    .exists()
            });
            let stderr = String::from_utf8(read(&study.join(format!("raw/{id}.stderr")))?)
                .map_err(|e| e.to_string())?;
            let standing = outcome(p, exit, &stderr, any)?;
            let count = c["arms"].as_array().unwrap().len();
            unsupported += count;
            cases.push(json!({"case":id,"status":standing,"first_arm_panicked":true,
                "unexecuted_later_arms":count-1,"planned_arms":c["arms"],"observed_degrees":null,
                "reference_replay_valid":true,"original_ideal_auxiliary_certificates":c["auxiliary_count"]}));
            continue;
        }
        if p != 2147483647 || exit != 0 {
            return Err("unexpected execution failure; retain and investigate".into());
        }
        let base = load(&dir.join("original-a.json"))?;
        let mut aa = load(&dir.join("original-aa.json"))?;
        aa["arm"] = base["arm"].clone();
        let aa_equal = base == aa;
        if !aa_equal {
            return Err("deterministic A/A algebraic mismatch".into());
        }
        let mut observations = vec![];
        for a in c["arms"].as_array().ok_or("arms")? {
            let arm = a["id"].as_str().ok_or("arm id")?;
            let v = load(&dir.join(format!("{arm}.json")))?;
            if v["arm"] != *a
                || v["case"] != id
                || v["timing_claim"] != false
                || v["all_originals_retained"] != true
            {
                return Err("record drift".into());
            }
            let valid = v["independent_complete_basis_valid"] == true && v["uncensored"] == true;
            if valid {
                let basis: Vec<Poly> =
                    serde_json::from_value(v["basis"].clone()).map_err(|e| e.to_string())?;
                check::validate(&basis, &rb, n, p, 5_000_000)?;
            } else {
                failed += 1;
            }
            let comparable = valid
                && base["independent_complete_basis_valid"] == true
                && base["uncensored"] == true
                && v["productive_degree_observation"].is_u64()
                && base["productive_degree_observation"].is_u64();
            let delta = if comparable {
                Some(
                    v["productive_degree_observation"].as_i64().unwrap()
                        - base["productive_degree_observation"].as_i64().unwrap(),
                )
            } else {
                None
            };
            if !arm.starts_with("original-") {
                match delta {
                    Some(d) if d < 0 => lower += 1,
                    Some(0) => equal += 1,
                    Some(_) => higher += 1,
                    None => (),
                }
            }
            observations.push(json!({"arm":a,"complete_and_valid":valid,"productive_degree":v["productive_degree_observation"],
                "degree_difference_vs_original":delta,"max_rows":v["max_rows"],"max_cols":v["max_cols"],"steps":v["steps"],
                "row_reduction_multiplications_only":v["row_reduction_multiplications_only"],"resource_limit":v["resource_limit"],
                "timed_out":v["timed_out"],"pairs_above_bound":v["pairs_above_bound"]}));
            completed += 1;
        }
        cases.push(
            json!({"case":id,"status":"observed","aa_exact_algebraic_records_equal":aa_equal,
            "reference_replay_valid":true,"observations":observations}),
        );
    }
    if completed + unsupported != 98 {
        return Err("roster count".into());
    }
    let result = json!({"schema":"fixed-f4-consequence-partial-summary/v1","planned_arms":98,
        "actual_f4_entries":completed+3,"completed_observations":completed,"unsupported_prime_arms":unsupported,
        "unsupported_prime_panics":3,"later_arms_not_executed":unsupported-3,"failed_or_censored_completed_observations":failed,
        "candidate_degree_effect_counts":{"lower":lower,"equal":equal,"higher":higher},"cases":cases,
        "timing_observations":0,"speedup_established":false,"original_15x_achieved":false,
        "intrinsic_regularity":null,"group_scalar_labels_present":false,
        "retained_prior_attempt":"attempt-v1: six reference export-order setup failures, zero F4 entries",
        "total_reference_constructions_including_failed_setup":12,
        "interpretation":"fixed existing fixtures; degree/matrix diagnostics only; unsupported53-bit cases remain uncomputed"});
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out)
        .map_err(|e| e.to_string())?;
    file.write_all(&serde_json::to_vec_pretty(&result).unwrap())
        .and_then(|_| file.sync_all())
        .map_err(|e| e.to_string())?;
    println!("completed={completed}, unsupported={unsupported}, candidate degree lower/equal/higher={lower}/{equal}/{higher}, failed_or_censored={failed}");
    for c in result["cases"].as_array().unwrap() {
        if let Some(obs) = c["observations"].as_array() {
            for v in obs
                .iter()
                .filter(|v| ["original-a", "full"].contains(&v["arm"]["id"].as_str().unwrap()))
            {
                println!(
                    "{} {} degree={} rows={} cols={} steps={} row-mults={}",
                    c["case"].as_str().unwrap(),
                    v["arm"]["id"].as_str().unwrap(),
                    v["productive_degree"],
                    v["max_rows"],
                    v["max_cols"],
                    v["steps"],
                    v["row_reduction_multiplications_only"]
                );
            }
        }
    }
    Ok(())
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let r = if args.len() == 3 {
        run(Path::new(&args[1]), Path::new(&args[2]))
    } else {
        Err("usage: summarize STUDY OUT".into())
    };
    if let Err(e) = r {
        eprintln!("ERROR: {e}");
        std::process::exit(1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unsupported_requires_exact_field_exit_diagnostic_and_no_output() {
        let message = "f4_fp: the prime must be below 2^32";
        assert!(outcome(9007199254740881, 101, message, false).is_ok());
        for (p, e, m, o) in [
            (2147483647, 101, message, false),
            (9007199254740881, 0, message, false),
            (9007199254740881, 101, "allocation error", false),
            (9007199254740881, 101, message, true),
        ] {
            assert!(outcome(p, e, m, o).is_err());
        }
    }
}
