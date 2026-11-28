use crypto_lib::cryptanalysis::prime_root_trace::{
    solve, verify, RootEvent, RootLimits, MAX_DEGREE, MAX_EVENTS, MAX_WORK,
};

#[test]
fn public_verifier_replays_power_and_gcd_chains() {
    let p = 13;
    let polynomial = [6, 8, 1]; // (x - 2)(x - 3) over F_13.
    let limits = RootLimits {
        field_work: MAX_WORK,
        split_attempts: 1000,
        events: MAX_EVENTS,
        degree: MAX_DEGREE,
    };
    let report = solve(&polynomial, p, 17, limits, &mut |_| Ok(())).unwrap();
    assert!(report.complete());
    assert_eq!(report.roots, vec![2, 3]);
    verify(&report).unwrap();

    let mut bad_power = report.clone();
    let power_value = bad_power.events.iter_mut().find_map(|event| match event {
        RootEvent::PowerEnd { value, .. } => Some(value),
        _ => None,
    });
    let power_value = power_value.expect("power chain present");
    power_value[0] = (power_value[0] + 1) % p;
    assert!(verify(&bad_power).is_err());

    let mut bad_gcd = report.clone();
    let gcd_value = bad_gcd.events.iter_mut().find_map(|event| match event {
        RootEvent::GcdEnd { value, .. } => Some(value),
        _ => None,
    });
    let gcd_value = gcd_value.expect("gcd chain present");
    gcd_value[0] = (gcd_value[0] + 1) % p;
    assert!(verify(&bad_gcd).is_err());
}
