use std::collections::BTreeMap;

/// Return a cost mean only for a nonempty, completely verified cohort.
fn complete_means(rows: &BTreeMap<String, ([f64; 3], u64, u64)>, arm: &str) -> Option<[f64; 3]> {
    let (s, runs, verified) = rows.get(arm)?;
    (*runs > 0 && runs == verified && s.iter().all(|v| v.is_finite() && *v >= 0.0)).then_some(*s)
}

/// Missing/zero/corrupt reference costs cannot define a finite ratio.
fn checked_distance(s: &[f64; 3], rho: &[f64; 3]) -> Option<[f64; 3]> {
    if s.iter().any(|v| !v.is_finite() || *v < 0.0)
        || rho.iter().any(|v| !v.is_finite() || *v <= 0.0)
    {
        return None;
    }
    let ratio = [s[0] / rho[0], s[1] / rho[1], s[2] / rho[2]];
    ratio.iter().all(|v| v.is_finite()).then_some(ratio)
}

#[test]
fn partial_reference_does_not_supply_a_distance() {
    let rows = BTreeMap::from([
        ("sel".into(), ([1.0, 2.0, 3.0], 4, 4)),
        ("rho-neg".into(), ([2.0, 3.0, 4.0], 4, 1)),
    ]);
    assert!(complete_means(&rows, "sel").is_some());
    let ratio = complete_means(&rows, "sel")
        .zip(complete_means(&rows, "rho-neg"))
        .and_then(|(s, rho)| checked_distance(&s, &rho));
    assert_eq!(ratio, None);
}

#[test]
fn complete_cohorts_keep_the_original_ratios() {
    let rows = BTreeMap::from([
        ("sel".into(), ([1.0, 2.0, 3.0], 4, 4)),
        ("rho-neg".into(), ([2.0, 4.0, 6.0], 4, 4)),
    ]);
    assert_eq!(
        complete_means(&rows, "sel")
            .zip(complete_means(&rows, "rho-neg"))
            .and_then(|(s, rho)| checked_distance(&s, &rho)),
        Some([0.5; 3])
    );
}

#[test]
fn empty_incomplete_and_nonfinite_cohorts_are_not_eligible() {
    for (means, runs, verified) in [
        ([1.0; 3], 0, 0),
        ([1.0; 3], 4, 0),
        ([1.0; 3], 4, 3),
        ([f64::NAN; 3], 4, 4),
        ([-1.0; 3], 4, 4),
        ([f64::INFINITY; 3], 4, 4),
    ] {
        let rows = BTreeMap::from([("inc".into(), (means, runs, verified))]);
        assert!(complete_means(&rows, "inc").is_none());
    }
}

#[test]
fn invalid_denominators_and_ratio_overflow_are_not_distances() {
    assert_eq!(checked_distance(&[1.0; 3], &[0.0; 3]), None);
    assert_eq!(checked_distance(&[1.0; 3], &[-1.0; 3]), None);
    assert_eq!(checked_distance(&[1.0; 3], &[f64::NAN; 3]), None);
    assert_eq!(
        checked_distance(&[f64::MAX; 3], &[f64::MIN_POSITIVE; 3]),
        None
    );
    assert_eq!(checked_distance(&[0.0; 3], &[1.0; 3]), Some([0.0; 3]));
}
