// New scan source, closed over every preceding implementation. No timing.
mod previous {
    include!("../historical/pre_scan_native_roster.rs");
    pub(super) fn all_arms() -> Vec<(String, &'static str, &'static str, usize, bool)> {
        supervised_roster("AB")
            .unwrap()
            .into_iter()
            .map(|a| (a.name, a.kind, a.kernel, a.worker, a.native))
            .collect()
    }
    pub(super) fn root_forms(
        graph: bool,
    ) -> Vec<(String, &'static str, &'static str, usize, bool)> {
        root_arms(graph)
            .into_iter()
            .map(|a| (a.name, a.kind, a.kernel, a.worker, a.native))
            .collect()
    }
}
const PRIMARY: &str = "scan-root-pairgraph-flat-dag-487-native";
const ROUNDS: usize = 5;
const PLANNED_CALLS: usize = 15120;
#[derive(Clone, Debug, PartialEq, Eq)]
struct SupervisedArm {
    name: String,
    kind: &'static str,
    kernel: &'static str,
    worker: usize,
    native: bool,
}
fn root_arms(graph: bool) -> Vec<SupervisedArm> {
    previous::root_forms(graph)
        .into_iter()
        .map(|(name, kind, kernel, _, native)| SupervisedArm {
            name: format!("scan-{name}"),
            kind,
            kernel,
            worker: 4,
            native,
        })
        .collect()
}
fn supervised_controls() -> Vec<SupervisedArm> {
    let mut out: Vec<_> = previous::all_arms()
        .into_iter()
        .map(|(name, kind, kernel, worker, native)| SupervisedArm {
            name,
            kind,
            kernel,
            worker,
            native,
        })
        .collect();
    out.extend(root_arms(false));
    out
}
fn supervised_roster(stage: &str) -> std::result::Result<Vec<SupervisedArm>, &'static str> {
    match stage {
        "AA" => Ok(supervised_controls()
            .into_iter()
            .flat_map(|a| {
                let mut b = a.clone();
                b.name += "-copy";
                [a, b]
            })
            .collect()),
        "AB" => {
            let mut out = supervised_controls();
            out.extend(root_arms(true));
            Ok(out)
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
mod scan_control_tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn every_previous_candidate_and_control_keeps_its_exact_worker_and_launcher() {
        let controls = supervised_controls();
        assert_eq!(controls.len(), 156);
        let old = previous::all_arms();
        assert_eq!(old.len(), 120);
        for (name, kind, kernel, worker, native) in old {
            assert!(controls.contains(&SupervisedArm {
                name,
                kind,
                kernel,
                worker,
                native
            }));
        }
        for name in [
            "packed-dag-487-native",
            "flat-dag-487-native",
            "root-pairgraph-flat-dag-487-native",
            "scan-root-exhaustive-flat-dag-487-native",
        ] {
            assert!(controls.iter().any(|a| a.name == name));
        }
        let ab = supervised_roster("AB").unwrap();
        assert_eq!(ab.len(), 192);
        assert_eq!(
            ab.iter().map(|a| &a.name).collect::<BTreeSet<_>>().len(),
            192
        );
        for a in root_arms(true) {
            let name = a.name.replacen("pairgraph", "exhaustive", 1);
            let b = controls.iter().find(|b| b.name == name).unwrap();
            assert_eq!(
                (a.worker, a.kernel, a.native),
                (b.worker, b.kernel, b.native)
            );
        }
    }
    #[test]
    fn aa_copies_are_adjacent_and_the_new_primary_is_never_a_control() {
        let aa = supervised_roster("AA").unwrap();
        assert_eq!(aa.len(), 312);
        for pair in aa.chunks_exact(2) {
            let mut b = pair[0].clone();
            b.name += "-copy";
            assert_eq!(pair[1], b);
        }
        assert_eq!(
            (aa.len() + supervised_roster("AB").unwrap().len()) * ROUNDS * 6,
            PLANNED_CALLS
        );
        assert_eq!(PRIMARY, "scan-root-pairgraph-flat-dag-487-native");
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
    fn row_scan_does_not_relax_root_checker_identity() {
        let a = root_arms(true).pop().unwrap();
        let v = serde_json::json!({"status":"complete","prime":2147483647u64,"variables":3,"checker":"packed","kernel":a.kernel,"constructor":a.kind,"root_api_contains_wire_replay_and_native_checks":true,"native_check_kind":"pairgraph","S_pair_and_input_containment_checks":true});
        assert!(worker_identity_valid(&v, &a, 2147483647, 3));
        let mut bad = v;
        bad["native_check_kind"] = serde_json::json!("exhaustive");
        assert!(!worker_identity_valid(&bad, &a, 2147483647, 3));
    }
}
