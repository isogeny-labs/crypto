use crypto_lib::cryptanalysis::aps_single_relation::LabeledBase;
use serde::Serialize;

#[derive(Serialize)]
pub struct Draw {
    pub index: usize,
    pub slot: usize,
    pub u: u64,
    pub v: u64,
    pub point_x: Option<u64>,
    pub outcome: &'static str,
}

#[derive(Serialize)]
pub struct TupleRow {
    pub coordinates: Vec<u64>,
    pub labels: Vec<u64>,
    pub signed_b_residues: Vec<u64>,
    pub valuation_safe: bool,
    pub exact_sign_safe: bool,
    pub informative_group_incidents: usize,
    pub zero_b_group_incidents: usize,
}

#[derive(Serialize)]
pub struct Profile {
    pub seed: u64,
    pub weights: [u64; 4],
    pub fixture_setup_draws: usize,
    pub generation_error: Option<String>,
    pub base: LabeledBase,
    pub draws: Vec<Draw>,
    pub tuples: Vec<TupleRow>,
    pub valuation_safe_tuples: usize,
    pub exact_sign_safe_tuples: usize,
    pub informative_group_incidents: usize,
    pub zero_b_group_incidents: usize,
    pub recovered_logarithm: Option<u64>,
    pub recovery_error: Option<String>,
    pub corrupted_label_refused: bool,
    pub complete_and_equal: bool,
}

#[derive(Serialize)]
pub struct Arm {
    pub seed: u64,
    pub weights: [u64; 4],
    pub report: Option<Profile>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct Report {
    pub candidate_u_stream: &'static str,
    pub coordinate_sets_matched: bool,
    pub polynomial_solver_tested: bool,
    pub performance_trial: bool,
    pub planned_arms: usize,
    pub attempted_arms: usize,
    pub arms: Vec<Arm>,
    pub modular_wrap_control_refused: bool,
    pub complete_and_equal: bool,
}

pub(super) struct Generation {
    pub base: LabeledBase,
    pub draws: Vec<Draw>,
    pub error: Option<String>,
}

impl Profile {
    pub(super) fn incomplete(
        seed: u64,
        weights: [u64; 4],
        fixture_setup_draws: usize,
        g: Generation,
    ) -> Self {
        Self {
            seed,
            weights,
            fixture_setup_draws,
            generation_error: g.error,
            base: g.base,
            draws: g.draws,
            tuples: Vec::new(),
            valuation_safe_tuples: 0,
            exact_sign_safe_tuples: 0,
            informative_group_incidents: 0,
            zero_b_group_incidents: 0,
            recovered_logarithm: None,
            recovery_error: None,
            corrupted_label_refused: false,
            complete_and_equal: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{fresh_base, run, signs};
    use crate::{sampler, valuation};

    #[test]
    fn labels_are_fresh_valid_and_all_profiles_and_signs_retained() {
        let r = run().unwrap();
        assert!(r.complete_and_equal);
        assert_eq!(r.arms.len(), 4);
        for arm in r.arms {
            assert!(arm.error.is_none());
            let p = arm.report.unwrap();
            assert_eq!(p.tuples.len(), 70);
            assert!(p.tuples.iter().all(|t| t.signed_b_residues.len() == 16));
            assert_eq!(p.draws.len(), p.base.draws);
            assert!(p.corrupted_label_refused);
            assert_eq!(
                p.base.parts.iter().map(Vec::len).collect::<Vec<_>>(),
                [2, 2, 2, 2]
            );
        }
    }

    #[test]
    fn wrap_and_repeated_label_controls_prevent_false_safety() {
        assert!(signs(&[1, 2, 2, 2], 7).unwrap().contains(&0));
        assert!(!valuation::guaranteed_nonzero(&[1, 2, 2, 2], 7));
        for labels in [[1, 1, 3, 3], [9, 9, 27, 27]] {
            assert!(signs(&labels, 107).unwrap().contains(&0));
        }
        assert!(signs(&[1, 2, 3, 107], 107).is_err());
    }
    #[test]
    fn capped_generation_preserves_every_draw_and_partial_base() {
        let f = sampler::even_fixture(sampler::SEEDS[0]).unwrap();
        let g = fresh_base(&f, sampler::SEEDS[0], [1, 3, 9, 27], 3).unwrap();
        assert!(g.error.is_some());
        assert_eq!(g.base.draws, 3);
        assert_eq!(g.draws.len(), 3);
        assert!(g.base.points.len() <= 3);
    }
}
