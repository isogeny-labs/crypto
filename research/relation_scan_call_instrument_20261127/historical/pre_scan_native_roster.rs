// Measurement preparation only. Historical sources and acceptance stay frozen.
mod previous {
    include!("../historical/flat_supervised_roster.rs");
    pub(super) fn all_previous_arms() -> Vec<(String, &'static str, &'static str, usize, bool)> {
        supervised_roster("AB")
            .unwrap()
            .into_iter()
            .map(|a| (a.name, a.kind, a.kernel, a.worker, a.native))
            .collect()
    }
}
const PRIMARY: &str = "root-pairgraph-flat-dag-487-native";
const ROUNDS: usize = 5;
const PLANNED_CALLS: usize = 8640;
#[derive(Clone, Debug, PartialEq, Eq)]
struct SupervisedArm {
    name: String,
    kind: &'static str,
    kernel: &'static str,
    worker: usize,
    native: bool,
}
fn root_arms(pair_graph: bool) -> Vec<SupervisedArm> {
    let forms = if pair_graph {
        [
            ("compact-sparse", "root-pairgraph-compact-sparse"),
            ("compact-dag", "root-pairgraph-compact-dag"),
            ("packed-sparse", "root-pairgraph-packed-sparse"),
            ("packed-dag", "root-pairgraph-packed-dag"),
            ("flat-sparse", "root-pairgraph-flat-sparse"),
            ("flat-dag", "root-pairgraph-flat-dag"),
        ]
    } else {
        [
            ("compact-sparse", "root-exhaustive-compact-sparse"),
            ("compact-dag", "root-exhaustive-compact-dag"),
            ("packed-sparse", "root-exhaustive-packed-sparse"),
            ("packed-dag", "root-exhaustive-packed-dag"),
            ("flat-sparse", "root-exhaustive-flat-sparse"),
            ("flat-dag", "root-exhaustive-flat-dag"),
        ]
    };
    let mut arms = Vec::new();
    for (form, kind) in forms {
        for (key, kernel) in [
            ("103", "frontier"),
            ("231", "packed-products"),
            ("487", "packed-batch"),
        ] {
            for native in [false, true] {
                arms.push(SupervisedArm {
                    name: format!(
                        "root-{}-{form}-{key}-{}",
                        if pair_graph {
                            "pairgraph"
                        } else {
                            "exhaustive"
                        },
                        if native { "native" } else { "wrapped" }
                    ),
                    kind,
                    kernel,
                    worker: 3,
                    native,
                });
            }
        }
    }
    arms
}
fn supervised_controls() -> Vec<SupervisedArm> {
    let mut arms: Vec<_> = previous::all_previous_arms()
        .into_iter()
        .map(|(name, kind, kernel, worker, native)| SupervisedArm {
            name,
            kind,
            kernel,
            worker,
            native,
        })
        .collect();
    arms.extend(root_arms(false));
    arms
}
fn supervised_roster(stage: &str) -> std::result::Result<Vec<SupervisedArm>, &'static str> {
    match stage {
        "AA" => Ok(supervised_controls()
            .into_iter()
            .flat_map(|arm| {
                let mut copy = arm.clone();
                copy.name += "-copy";
                [arm, copy]
            })
            .collect()),
        "AB" => {
            let mut arms = supervised_controls();
            arms.extend(root_arms(true));
            Ok(arms)
        }
        _ => Err("unknown comparison stage"),
    }
}
fn worker_identity_valid(v: &serde_json::Value, arm: &SupervisedArm, p: u64, n: usize) -> bool {
    v["status"] == "complete"
        && v["prime"] == p
        && v["variables"] == n
        && v["checker"] == "packed"
        && v["kernel"] == arm.kernel
        && v["S_pair_and_input_containment_checks"] == true
        && (!arm.kind.starts_with("root-")
            || (v["constructor"] == arm.kind
                && v["root_api_contains_wire_replay_and_native_checks"] == true
                && v["native_check_kind"]
                    == if arm.kind.starts_with("root-pairgraph-") {
                        "pairgraph"
                    } else {
                        "exhaustive"
                    }))
}
fn root_native_basis_matches(
    v: &serde_json::Value,
    arm: &SupervisedArm,
    checked: &relation_replay_checker::Checked,
    p: u64,
    n: usize,
) -> bool {
    if !arm.kind.starts_with("root-") {
        return true;
    }
    let Ok(native) =
        serde_json::from_value::<Vec<Vec<(Vec<usize>, u64)>>>(v["native_basis"].clone())
    else {
        return false;
    };
    if native.len() != checked.basis.len() {
        return false;
    }
    native.iter().zip(&checked.basis).all(|(row, expected)| {
        let map: std::collections::BTreeMap<_, _> = row.iter().cloned().collect();
        map.len() == row.len()
            && row.iter().all(|(m, c)| m.len() == n && *c > 0 && *c < p)
            && map == expected.iter().cloned().collect()
    })
}
#[cfg(test)]
mod native_roster_tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn every_previous_arm_is_eligible_without_identity_or_launcher_changes() {
        let controls = supervised_controls();
        assert_eq!(controls.len(), 84);
        for (name, kind, kernel, worker, native) in previous::all_previous_arms() {
            assert!(controls.contains(&SupervisedArm {
                name,
                kind,
                kernel,
                worker,
                native
            }));
        }
        for old in ["packed-dag-487-native", "flat-dag-487-native"] {
            assert!(controls.iter().any(|a| a.name == old));
        }
        let ab = supervised_roster("AB").unwrap();
        assert_eq!(ab.len(), 120);
        assert_eq!(
            ab.iter().map(|a| &a.name).collect::<BTreeSet<_>>().len(),
            120
        );
        for a in root_arms(true) {
            let expected = a.name.replacen("root-pairgraph-", "root-exhaustive-", 1);
            let control = controls.iter().find(|x| x.name == expected).unwrap();
            assert_eq!(
                (a.worker, a.kernel, a.native),
                (control.worker, control.kernel, control.native)
            );
        }
    }
    #[test]
    fn every_control_has_identical_adjacent_aa_copy_and_primary_is_fixed() {
        let aa = supervised_roster("AA").unwrap();
        assert_eq!(aa.len(), 168);
        for pair in aa.chunks_exact(2) {
            let mut expected = pair[0].clone();
            expected.name += "-copy";
            assert_eq!(pair[1], expected);
        }
        assert_eq!(
            (aa.len() + supervised_roster("AB").unwrap().len()) * ROUNDS * 6,
            PLANNED_CALLS
        );
        assert_eq!(PRIMARY, "root-pairgraph-flat-dag-487-native");
        assert!(!supervised_controls().iter().any(|a| a.name == PRIMARY));
        assert_eq!(
            supervised_roster("AB")
                .unwrap()
                .iter()
                .filter(|a| a.name == PRIMARY)
                .count(),
            1
        );
        assert!(supervised_roster("typo").is_err());
    }
    #[test]
    fn root_receipts_cannot_mislabel_the_checker_or_skip_native_obligations() {
        let a = root_arms(true).pop().unwrap();
        let v = serde_json::json!({"status":"complete","prime":2147483647u64,"variables":3,"checker":"packed","kernel":a.kernel,"constructor":a.kind,"root_api_contains_wire_replay_and_native_checks":true,"native_check_kind":"pairgraph","S_pair_and_input_containment_checks":true});
        assert!(worker_identity_valid(&v, &a, 2147483647, 3));
        for (key, value) in [
            ("native_check_kind", serde_json::json!("exhaustive")),
            (
                "root_api_contains_wire_replay_and_native_checks",
                serde_json::json!(false),
            ),
            ("constructor", serde_json::json!("flat-dag")),
        ] {
            let mut bad = v.clone();
            bad[key] = value;
            assert!(!worker_identity_valid(&bad, &a, 2147483647, 3));
        }
    }
}
