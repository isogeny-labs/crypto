use crate::{
    quartic_solver::{self, Route},
    sampler,
};
use crypto_lib::cryptanalysis::aps_single_relation::Encoding;
use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Key {
    pub seed: u64,
    pub encoding: Encoding,
    pub shared: bool,
}

#[derive(Serialize)]
pub struct Arm {
    pub key: Key,
    pub report: Option<Route>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct Flow {
    pub planned: usize,
    pub attempted: usize,
    pub completed: usize,
    pub failed_or_incomplete: usize,
    pub excluded_by_rule: usize,
}

#[derive(Serialize)]
pub struct Panel {
    pub arms: Vec<Arm>,
    pub flow: Flow,
    pub complete_and_equal: bool,
}

/// An arm error is retained and cannot silently remove later planned arms.
pub fn collect(mut call: impl FnMut(Key) -> Result<Route, String>) -> Panel {
    let mut arms = Vec::new();
    let mut complete = 0;
    let mut correct = true;
    for seed in sampler::SEEDS {
        for encoding in [Encoding::Expanded4, Encoding::Split4] {
            for shared in [false, true] {
                let key = Key {
                    seed,
                    encoding,
                    shared,
                };
                let (report, error) = match call(key) {
                    Ok(report) => {
                        complete += usize::from(report.complete_and_equal);
                        correct &= report.complete_and_equal;
                        (Some(report), None)
                    }
                    Err(error) => {
                        correct = false;
                        (None, Some(error))
                    }
                };
                arms.push(Arm { key, report, error });
            }
        }
    }
    for seed in sampler::SEEDS {
        let routes: Vec<_> = arms
            .iter()
            .filter(|a| a.key.seed == seed)
            .filter_map(|a| a.report.as_ref())
            .collect();
        correct &= routes.len() == 4
            && routes
                .windows(2)
                .all(|w| w[0].observed_roots == w[1].observed_roots);
    }
    Panel {
        flow: Flow {
            planned: 8,
            attempted: arms.len(),
            completed: complete,
            failed_or_incomplete: arms.len() - complete,
            excluded_by_rule: 0,
        },
        arms,
        complete_and_equal: correct,
    }
}

pub fn run() -> Panel {
    collect(|key| {
        let f = sampler::even_fixture(key.seed)?;
        quartic_solver::run_encoding(&f, key.seed, key.shared, key.encoding)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_encodings_and_cache_modes_match_projected_group_roots() {
        let panel = run();
        assert!(
            panel.complete_and_equal,
            "{}",
            serde_json::to_string(&panel).unwrap()
        );
        assert_eq!(panel.flow.attempted, 8);
        for arm in panel.arms {
            let report = arm.report.unwrap();
            for cell in report.cells {
                let width = if arm.key.encoding == Encoding::Split4 {
                    5
                } else {
                    4
                };
                assert!(cell.raw_witnesses.iter().all(|w| w.len() == width));
            }
        }
    }

    #[test]
    fn transport_control_keeps_all_failed_arms_in_fixed_order() {
        let mut calls = 0;
        let panel = collect(|_| {
            calls += 1;
            Err(format!("transport-only arm{calls}"))
        });
        assert_eq!(calls, 8);
        assert_eq!(panel.flow.attempted, 8);
        assert_eq!(panel.flow.failed_or_incomplete, 8);
        assert_eq!(panel.flow.excluded_by_rule, 0);
        assert!(!panel.complete_and_equal);
        assert!(panel.arms[7].error.as_ref().unwrap().ends_with("arm8"));
    }
}
