//! Post-run structural control. Reads saved zero sets, never repeats their oracle.
use super::*;

pub(super) fn audit(run: &Path, public_root: &Path, out: &Path) -> bool {
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let mut records = Vec::new();
    let mut missing = Vec::new();
    for p in PRIMES {
        for seed in 0..3 {
            let id = format!("p{p}-seed{seed}");
            let file = run.join("zero").join(&id).join("double-l0-active.json");
            if !file.is_file() {
                missing.push(id);
                continue;
            }
            let bytes = std::fs::read(&file).unwrap();
            let saved: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(saved["arm"], "double-l0-active");
            assert_eq!(saved["status"], "pass");
            let observed: BTreeSet<Terms> =
                serde_json::from_value(saved["signed_physical_set"].clone()).unwrap();
            assert_eq!(
                observed.len() as u64,
                saved["signed_physical_relations"].as_u64().unwrap()
            );
            let public = public_root.join(format!("{id}.json"));
            let public_bytes = std::fs::read(&public).unwrap();
            let sha = hex::encode(hash::sha256::sha256(&public_bytes));
            let raw = checked_public(&public, &sha);
            let directory = out.join(&id);
            std::fs::create_dir(&directory).unwrap();
            let case = Case::new(raw, &directory);
            assert_eq!(case.p, p);
            assert_eq!(case.seed, seed);
            assert_eq!(case.peels[9].len(), case.l0);
            let by_x: BTreeMap<_, _> = case.pool[..case.l0]
                .iter()
                .enumerate()
                .map(|(i, f)| (f.p.x, i))
                .collect();
            let mut predicted = BTreeSet::new();
            for &image_id in &case.peels[9] {
                let source = case.pool[image_id].source_id;
                let point: Point = case.pool[source].p.into();
                let w = case.raw["omega"].as_u64().unwrap();
                let first = psi(&case.c, point, w);
                let second = psi(&case.c, first, w);
                assert_eq!(case.c.add(case.c.add(point, first), second), INF);
                let first_id = by_x[&first.x];
                let second_id = by_x[&second.x];
                let double_sign =
                    matches_sign(&case.c, case.pool[image_id].p.into(), case.c.mul(point, 2));
                let first_sign = matches_sign(&case.c, case.pool[first_id].p.into(), first);
                let second_sign = matches_sign(&case.c, case.pool[second_id].p.into(), second);
                let mut terms = vec![
                    (image_id, double_sign),
                    (source, -1),
                    (first_id, first_sign),
                    (second_id, second_sign),
                ];
                terms.sort();
                for relation in [terms.clone(), terms.iter().map(|&(i, s)| (i, -s)).collect()] {
                    case.verify_terms(&relation, INF);
                    assert!(
                        observed.contains(&relation),
                        "complete zero oracle omitted a doubled triangle"
                    );
                    let row = case.row(&relation, &json!([0, 0]), 9);
                    assert!(
                        row.iter().all(|&x| x == 0),
                        "doubled triangle added a nonzero folded row"
                    );
                    assert!(
                        predicted.insert(relation),
                        "double-image injectivity failed"
                    );
                }
            }
            assert_eq!(predicted.len(), 2 * case.l0);
            assert!(saved["zero_augmented_rows"].as_u64().unwrap() >= predicted.len() as u64);
            write(
                &directory.join("structural-set.json"),
                &json!({"predicted":predicted,"status":"pass"}),
            );
            records.push(json!({"p":p,"seed":seed,"l0":case.l0,
            "predicted_signed_relations":predicted.len(),"all_predicted_augmented_rows_zero":true,
            "observed_signed_relations":observed.len(),"additional_physical_relations":observed.len()-predicted.len(),
            "observed_zero_rows":saved["zero_augmented_rows"],"observed_rank":saved["rank"],
            "factor_log_unknowns":saved["factor_log_unknowns"],"public_input_sha256":sha,
            "saved_zero_file":file,"saved_zero_sha256":hex::encode(hash::sha256::sha256(&bytes)),"status":"pass"}));
        }
    }
    let pass = missing.is_empty();
    write(
        &out.join("summary.json"),
        &json!({"schema":"joint-image-zero-doubling-control/v1",
        "status":if pass {"pass"} else {"partial"},"cases_requested":21,"cases":records,"missing":missing,
        "scope":"Independent post-run reconstruction and actual group/row check of 2L doubled triangles per case. Reads retained zero sets; no oracle repeated and no target recovery or cost claim."}),
    );
    pass
}
