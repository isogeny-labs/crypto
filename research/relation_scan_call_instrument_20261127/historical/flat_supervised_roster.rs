// New flat draft: every previous packed/generic arm is now an eligible control.
// Existing packed/supervised source and evidence remain immutable.
include!("packed_roster.rs");

const PRIMARY: &str = "flat-dag-487-native";
const ROUNDS: usize = 5;
const PLANNED_CALLS: usize = 3600;

#[derive(Clone, Debug, PartialEq, Eq)]
struct SupervisedArm {
    name: String,
    kind: &'static str,
    kernel: &'static str,
    worker: usize,
    native: bool,
}
fn with_launchers(arms: Vec<Arm>) -> Vec<SupervisedArm> {
    arms.into_iter()
        .flat_map(|arm| {
            [false, true].map(|native| SupervisedArm {
                name: format!("{}-{}", arm.name, if native { "native" } else { "wrapped" }),
                kind: arm.kind,
                kernel: arm.kernel,
                worker: arm.worker,
                native,
            })
        })
        .collect()
}
fn supervised_controls() -> Vec<SupervisedArm> {
    with_launchers(roster("AB"))
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
            for (form, kind) in [("sparse", "flat-constructor-sparse"), ("dag", "flat-constructor-dag")] {
                for (key, kernel) in [("103", "frontier"), ("231", "packed-products"), ("487", "packed-batch")] {
                    arms.extend(with_launchers(vec![Arm {
                        name: format!("flat-{form}-{key}"), kind, kernel, worker: 2,
                    }]));
                }
            }
            Ok(arms)
        },
        _ => Err("unknown comparison stage"),
    }
}

#[cfg(test)]
mod supervised_roster_tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn closed_roster_covers_both_launchers_for_every_original_arm() {
        let all = supervised_roster("AB").unwrap();
        assert_eq!(all.len(), 48);
        assert_eq!(
            all.iter().map(|a| &a.name).collect::<BTreeSet<_>>().len(),
            48
        );
        for old in roster("AB") {
            for native in [false, true] {
                let name = format!("{}-{}", old.name, if native { "native" } else { "wrapped" });
                let arm = all.iter().find(|a| a.name == name).unwrap();
                assert_eq!(
                    (arm.kind, arm.kernel, arm.worker, arm.native),
                    (old.kind, old.kernel, old.worker, native)
                );
            }
        }
        for previous in with_launchers(roster("AB")) {
            assert!(supervised_controls().contains(&previous), "previous implementation must be eligible");
        }
        assert!(supervised_controls().iter().any(|a| a.name == "packed-dag-487-native"));
        for form in ["sparse", "dag"] {
            for kernel in ["103", "231", "487"] {
                for launcher in ["native", "wrapped"] {
                    assert!(all.iter().any(|a| a.name == format!("flat-{form}-{kernel}-{launcher}")));
                }
            }
        }
        assert_eq!(supervised_controls().len(), 36);
        assert_eq!(
            supervised_controls().iter().filter(|a| a.native).count(),
            18
        );
    }

    #[test]
    fn every_control_has_an_identical_adjacent_negative_control() {
        let aa = supervised_roster("AA").unwrap();
        assert_eq!(aa.len(), 72);
        for pair in aa.chunks_exact(2) {
            let mut expected = pair[0].clone();
            expected.name += "-copy";
            assert_eq!(pair[1], expected);
        }
        assert_eq!(
            (aa.len() + supervised_roster("AB").unwrap().len()) * ROUNDS * 6,
            PLANNED_CALLS
        );
    }

    #[test]
    fn primary_is_fixed_and_never_an_eligible_control() {
        assert_eq!(PRIMARY, "flat-dag-487-native");
        assert_eq!(
            supervised_roster("AB")
                .unwrap()
                .iter()
                .filter(|a| a.name == PRIMARY)
                .count(),
            1
        );
        assert!(!supervised_controls().iter().any(|a| a.name == PRIMARY));
        assert!(supervised_roster("typo").is_err());
    }
}
