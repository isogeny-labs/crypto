use super::*;

#[test]
fn singleton_and_quadratic_remainders_preserve_values_and_scalar() {
    let parts = vec![vec![2, 5], vec![7], vec![9, 11], vec![13]];
    let poly = vec![(vec![4, 3, 2, 1], 7), (vec![0, 0, 0, 0], 19)];
    let (r, _) = remainder(&poly, &parts, 101).unwrap();
    assert!(r
        .iter()
        .all(|(e, _)| e.iter().zip(&parts).all(|(&d, p)| d < p.len() as u32)));
    for &a in &parts[0] {
        for &c in &parts[2] {
            let x = [a, 7, c, 13];
            assert_eq!(f4_fp::eval(&poly, &x, 101), f4_fp::eval(&r, &x, 101));
        }
    }
    let constant = vec![(vec![0; 4], 19)];
    assert_eq!(remainder(&constant, &parts, 101).unwrap().0, constant);
    assert!(remainder(&vec![], &parts, 101).unwrap().0.is_empty());
}

#[test]
fn repeated_or_invalid_domains_are_refused() {
    for parts in [
        vec![vec![1, 1], vec![2], vec![3], vec![4]],
        vec![vec![1], vec![1], vec![3], vec![4]],
        vec![vec![], vec![2], vec![3], vec![4]],
        vec![vec![101], vec![2], vec![3], vec![4]],
    ] {
        assert!(remainder(&vec![], &parts, 101).is_err());
    }
}

#[test]
fn zero_budget_is_retained_as_incomplete_not_an_empty_root_pass() {
    let f = sampler::even_fixture(sampler::SEEDS[0]).unwrap();
    let cell = quartic_cells::plan(&f).unwrap().remove(0);
    let system = aps::build_system(&f.curve, &cell.parts, Encoding::Expanded4).unwrap();
    let result = visit(
        system.equations(),
        &cell.parts,
        system.modulus(),
        Duration::ZERO,
    );
    assert!(!result.complete && result.timed_out);
}

#[test]
fn all_cell_remainders_and_both_native_routes_match_the_affine_oracle() {
    let r = run().unwrap();
    assert!(
        r.complete_and_equal,
        "{}",
        serde_json::to_string(&r).unwrap()
    );
    assert!(r
        .samples
        .iter()
        .all(|s| s["cells"].as_array().unwrap().len() == 19));
}
