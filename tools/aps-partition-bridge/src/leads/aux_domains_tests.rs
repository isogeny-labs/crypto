use super::*;

#[test]
fn exact_domain_and_join_keep_all_native_roots_and_empty_cells() {
    let report = run().unwrap();
    assert!(
        report.complete_and_equal,
        "{}",
        serde_json::to_string(&report).unwrap()
    );
    for sample in report.samples {
        assert_eq!(sample.cells.len(), 19);
        assert!(sample.cells.iter().any(|c| c.intersection.is_empty()));
        for cell in sample.cells {
            assert_eq!(
                cell.constrained.solver_called,
                !cell.intersection.is_empty()
            );
            for x in 0..101 {
                assert_eq!(
                    f4_fp::eval(&cell.auxiliary_vanishing, &[0, 0, 0, 0, x], 101) == 0,
                    cell.intersection.contains(&x)
                );
            }
        }
    }
}

#[test]
fn wrong_empty_domain_loses_a_known_root_and_pair_overlap_is_refused() {
    let f = sampler::even_fixture(sampler::SEEDS[0]).unwrap();
    let cells = quartic_cells::plan(&f).unwrap();
    let cell = cells
        .iter()
        .find(|c| {
            !quartic_oracle::roots(&f.base, &quartic_cells::product(&c.parts).unwrap())
                .unwrap()
                .is_empty()
        })
        .unwrap();
    let incorrect = solve(&f, &cell.parts, Some(&[]));
    assert!(incorrect.roots.is_empty() && !incorrect.solver_called);
    assert!(
        !quartic_oracle::roots(&f.base, &quartic_cells::product(&cell.parts).unwrap())
            .unwrap()
            .is_empty()
    );
    assert!(pairs(&f, &cell.parts[0], &cell.parts[0]).is_err());
}

#[test]
fn a_middle_cell_setup_error_keeps_prior_and_later_rows() {
    let seed = sampler::SEEDS[0];
    let f = sampler::even_fixture(seed).unwrap();
    let mut cells = quartic_cells::plan(&f).unwrap();
    cells[9].parts[0] = vec![999];
    let sample = run_cells(&f, seed, cells);
    assert_eq!(sample.planned_cells, 19);
    assert_eq!(sample.attempted_cells, 19);
    assert_eq!(sample.completed_cells, 18);
    assert_eq!(sample.failed_or_incomplete_cells, 1);
    assert_eq!(sample.cell_errors.len(), 1);
    assert_eq!(sample.cell_errors[0].cell_index, 9);
    assert_eq!(sample.cells.first().unwrap().cell_index, 0);
    assert_eq!(sample.cells.last().unwrap().cell_index, 18);
    assert!(!sample.complete_and_equal);
}

#[test]
fn union_oracle_failure_retains_the_cell_results_and_error() {
    let seed = sampler::SEEDS[0];
    let mut f = sampler::even_fixture(seed).unwrap();
    let cells = quartic_cells::plan(&f).unwrap();
    f.base.points.values_mut().next().unwrap().y = 0;
    let sample = run_cells(&f, seed, cells);
    assert_eq!(sample.attempted_cells, 19);
    assert_eq!(sample.cells.len() + sample.cell_errors.len(), 19);
    assert!(sample.expected_union.is_none());
    assert!(sample.union_error.is_some());
    assert!(!sample.cells.is_empty());
    assert!(!sample.complete_and_equal);
}
