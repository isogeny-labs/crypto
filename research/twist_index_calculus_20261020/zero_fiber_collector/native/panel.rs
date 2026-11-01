//! Fixed 21-case, two-arm result reduction. No incomplete run supplies a zero.
use super::*;

fn read_json(path: &Path) -> Option<(Value, String)> {
    let bytes = std::fs::read(path).ok()?;
    Some((
        serde_json::from_slice(&bytes).ok()?,
        hex::encode(hash::sha256::sha256(&bytes)),
    ))
}

pub(super) fn summarize(run: &Path, out: &Path) -> bool {
    assert!(!out.exists());
    let mut records = Vec::new();
    let mut totals = BTreeMap::<String, BTreeMap<&str, u64>>::new();
    let mut complete = 0;
    let mut missing = Vec::new();
    for p in PRIMES {
        for seed in 0..3 {
            for arm in ["phi-l0-active", "double-l0-active"] {
                let id = format!("p{p}-seed{seed}");
                let tag = format!("{id}-{arm}");
                let root = run.join("cases").join(&id).join(arm);
                let first = read_json(&root.join("first/record.json"));
                let audit = read_json(&root.join("audit/summary.json"));
                let public = std::fs::read(run.join("inputs").join(format!("{id}.json"))).ok();
                let public_sha = public
                    .as_ref()
                    .map(|b| hex::encode(hash::sha256::sha256(b)));
                let rc = |phase: &str| {
                    std::fs::read_to_string(
                        run.join("metadata").join(format!("{tag}-{phase}.exit")),
                    )
                    .ok()
                    .map(|s| s.trim().to_owned())
                };
                let collector_exit = rc("collect");
                let audit_exit = rc("audit");
                let mut item = json!({"case":id,"p":p,"seed":seed,"arm":arm,
            "collector_exit":collector_exit,"audit_exit":audit_exit,"public_input_sha256":public_sha});
                if let (Some((v, sha)), Some((a, asha)), Some(psha)) = (&first, &audit, &public_sha)
                {
                    let valid = collector_exit.as_deref() == Some("0")
                        && audit_exit.as_deref() == Some("0")
                        && v["schema"] == "twist-zero-five-polynomial/v1"
                        && v["status"] == "complete-polynomial-enumeration"
                        && a["status"] == "pass"
                        && a["candidate_sha256"] == *sha
                        && a["public_input_sha256"] == *psha
                        && v["public_input_sha256"] == *psha
                        && v["p"] == p
                        && v["seed"] == seed
                        && v["arm"] == arm
                        && a["p"] == p
                        && a["seed"] == seed
                        && a["arm"] == arm;
                    item["candidate_sha256"] = json!(sha);
                    item["audit_sha256"] = json!(asha);
                    item["status"] = json!(if valid {
                        "complete-independent-physical-replay"
                    } else {
                        "unverified-or-partial"
                    });
                    if valid {
                        complete += 1;
                        for key in [
                            "signed_pairs",
                            "pair_representatives",
                            "systems_attempted",
                            "signed_physical_relations",
                            "zero_augmented_rows",
                            "unique_projective_augmented_rows",
                            "f4_row_reduction_fp_muls",
                        ] {
                            let n = v[key].as_u64().expect("audited counter type");
                            let current = totals
                                .entry(arm.to_owned())
                                .or_default()
                                .entry(key)
                                .or_default();
                            *current = current.checked_add(n).unwrap();
                            item[key] = json!(n);
                        }
                        for key in [
                            "factor_log_rank",
                            "unanchored_factor_columns",
                            "factor_log_solution_point_verified",
                            "setup_work",
                            "collection_and_matrix_work",
                            "memory",
                        ] {
                            item[key] = v[key].clone();
                        }
                        item["independent_checks"] = a["independent_checks"].clone();
                        item["corruption_controls"] = a["corruption_controls"].clone();
                    } else {
                        missing.push(tag);
                    }
                } else {
                    item["status"] = json!("missing-terminal-candidate-or-audit");
                    missing.push(tag);
                }
                records.push(item);
            }
        }
    }
    let pass = complete == 42 && missing.is_empty();
    write(
        out,
        &json!({"schema":"twist-zero-five-polynomial-panel/v1",
        "status":if pass {"complete-independent-physical-replay"} else {"partial"},
        "expected_case_arms":42,"audited_complete_case_arms":complete,"unverified_case_arms":missing,
        "audited_case_local_count_sums":totals,"records":records,
        "scope":"All seven curves in three public scalar-271 presentations, two matched image arms. Sums are case-local diagnostics; ranks remain per case. Full physical enumeration and recorded pair/work replay only. No target recovered by zero-only equations, no admitted timing, no calibrated IC or rho speedup."}),
    );
    pass
}
