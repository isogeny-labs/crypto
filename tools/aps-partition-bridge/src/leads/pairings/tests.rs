use super::*;

#[test]
fn projection_excludes_auxiliary_and_input_rule_has_fixed_ties() {
    for p in PAIRINGS {
        let original = [17, 23, 31, 47];
        let mut w: Vec<_> = p.iter().map(|&i| original[i]).collect();
        w.push(2);
        let domains: Vec<_> = original.iter().map(|&x| vec![x]).collect();
        assert_eq!(project(&w, p, &domains).unwrap(), original);
        let mut swapped = w.clone();
        swapped.swap(0, 1);
        assert!(project(&swapped, p, &domains).is_err());
        assert_ne!(
            w[..4]
                .iter()
                .copied()
                .chain([w[4]])
                .collect::<BTreeSet<_>>(),
            original.into_iter().collect()
        );
    }
    assert!(project(
        &[1, 2, 3, 4],
        PAIRINGS[0],
        &[vec![1], vec![2], vec![3], vec![4]]
    )
    .is_err());
    assert!(project(
        &[1, 2, 3, 4, 5],
        [0, 0, 2, 3],
        &[vec![1], vec![2], vec![3], vec![4]]
    )
    .is_err());
    assert_eq!(
        select(&[vec![1], vec![2], vec![3], vec![4]], 0)
            .unwrap()
            .selected_pairing,
        0
    );
    let s = select(&[vec![1, 2], vec![3, 4], vec![5], vec![6]], 0).unwrap();
    assert_eq!(s.candidate_product_scores, [5, 4, 4]);
    assert_eq!(s.selected_pairing, 1);
}

#[test]
fn all_pairings_keep_complete_projected_roots_and_selected_cold_union() {
    let r = run().unwrap();
    assert!(
        r.complete_and_equal,
        "{}",
        serde_json::to_string(&r).unwrap()
    );
    assert!(r.actual_selected_route_executed);
    assert_eq!(r.planned_control_routes, 12);
    assert_eq!(r.planned_selected_routes, 2);
    assert!(r.samples.iter().all(|s| s.routes.len() == 6
        && s.selected_cold_replay_complete
        && s.selected_route.as_ref().unwrap().complete_and_equal));
}

#[test]
fn failed_first_seed_keeps_later_planned_routes() {
    let report = panel::collect(|seed| {
        if seed == sampler::SEEDS[0] {
            Err("deliberate fixture failure".into())
        } else {
            sampler::even_fixture(seed)
        }
    });
    assert!(!report.complete_and_equal);
    assert_eq!(report.attempted_routes, 14);
    assert_eq!(report.failed_or_incomplete_routes, 7);
    assert_eq!(report.samples.len(), 2);
    assert!(report.samples[0].fixture_error.is_some());
    assert_eq!(report.samples[1].routes.len(), 6);
}
