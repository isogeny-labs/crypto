//! Paired native inverse and completed-map unit; all counters and checks are Rust.
use super::*;
use std::collections::BTreeSet;
use std::panic::{catch_unwind, AssertUnwindSafe};

type Counts15 = [u64; 5];

fn reset15(field: &Field) {
    field.k.mul_calls15.set(0);
    field.k.nonzero_mul_calls15.set(0);
    field.k.inverse_calls15.set(0);
    field.shortcut_calls15.set(0);
    field.general_calls15.set(0);
}

fn counts15(field: &Field) -> Counts15 {
    [
        field.k.mul_calls15.get(),
        field.k.nonzero_mul_calls15.get(),
        field.k.inverse_calls15.get(),
        field.shortcut_calls15.get(),
        field.general_calls15.get(),
    ]
}

fn encoding15(field: &Field, value: &Element) -> [u64; 2] {
    [field.k.encoding(&value.a), field.k.encoding(&value.b)]
}

fn paired_map15(
    baseline: &RichelotMap,
    candidate: &RichelotMap,
    input: &Class,
    reverse: bool,
    label: &str,
) -> Result<(Class, closed_fibres::ClosedRoute, u64, u64)> {
    reset15(&baseline.k);
    reset15(&candidate.k);
    let (expected, old_route) = baseline.evaluate_closed(input, reverse)?;
    let old = counts15(&baseline.k);
    let (actual, new_route) = candidate.evaluate_closed(input, reverse)?;
    let new = counts15(&candidate.k);
    assert_eq!(expected, actual, "paired class {label}, reverse={reverse}");
    assert_eq!(old_route, new_route);
    assert_eq!(old[2], new[2], "inner inverse calls");
    assert_eq!(old[0], new[0] + 5 * new[3], "all cubic boundary calls");
    assert_eq!(old[1], new[1] + 2 * new[3], "nonzero cubic boundary calls");
    assert_eq!(old[4], new[3] + new[4], "inverse route accounting");
    println!("CT15|map|label={label}|reverse={reverse}|route={old_route:?}|input={:?}|baseline={:?}|candidate={:?}|base_counts={old:?}|fast_counts={new:?}",input.components(),expected.components(),actual.components());
    Ok((actual, old_route, new[3], new[4]))
}

pub(super) fn run() -> Result<()> {
    let baseline = Field::new(CubicField::with_cubic(P14, [3, 26, 3])?);
    let mut candidate = Field::new(CubicField::with_cubic(P14, [3, 26, 3])?);
    candidate.cubic_first15 = true;
    assert_eq!(baseline.nonresidue, candidate.nonresidue);
    println!("CT15|scope|p=101|cubic=[3,26,3,1]|nonresidue_encoding={}|counter_order=mul,nonzero_mul,inner_inverse,shortcut,general|inverse_intervals_exclude_setup_and_product_checks|no_timing_ratio|unsigned",baseline.k.encoding(&baseline.nonresidue));
    let mut encodings = BTreeSet::new();
    let mut sign_controls = 0;
    for i in 0_u64..64 {
        let n = 1 + (65537 * i % 1_030_300);
        assert!(encodings.insert(n));
        let x = baseline.lift3(&baseline.k.element(n));
        assert!(!baseline.zero(&x) && x.b.is_zero());
        reset15(&baseline);
        reset15(&candidate);
        let old = baseline.inv(&x);
        let old_counts = counts15(&baseline);
        let new = candidate.inv(&x);
        let new_counts = counts15(&candidate);
        assert_eq!(old, new);
        assert_eq!(old_counts, [5, 2, 1, 0, 1]);
        assert_eq!(new_counts, [0, 0, 1, 1, 0]);
        assert_eq!(baseline.mul(&x, &old), baseline.c(1));
        assert_eq!(candidate.mul(&x, &new), candidate.c(1));
        println!("CT15|primitive|kind=cubic|i={i}|input={:?}|baseline={:?}|candidate={:?}|base_counts={old_counts:?}|fast_counts={new_counts:?}",encoding15(&baseline,&x),encoding15(&baseline,&old),encoding15(&candidate,&new));
        if i < 8 {
            let wrong = candidate.neg(&new);
            assert_ne!(candidate.mul(&x, &wrong), candidate.c(1));
            println!(
                "CT15|control|wrong_sign|i={i}|wrong_inverse={:?}|detected=true",
                encoding15(&candidate, &wrong)
            );
            sign_controls += 1;
        }
    }
    assert_eq!(encodings.len(), 64);
    assert_eq!(sign_controls, 8);
    for i in 0_u64..8 {
        let x = Element {
            a: baseline.k.element(i + 1),
            b: baseline.k.element(3 * i + 1),
        };
        assert!(!x.b.is_zero());
        reset15(&baseline);
        reset15(&candidate);
        let old = baseline.inv(&x);
        let old_counts = counts15(&baseline);
        let new = candidate.inv(&x);
        let new_counts = counts15(&candidate);
        assert_eq!(old, new);
        assert_eq!(old_counts, [5, 5, 1, 0, 1]);
        assert_eq!(new_counts, old_counts);
        assert_eq!(baseline.mul(&x, &old), baseline.c(1));
        assert_eq!(candidate.mul(&x, &new), candidate.c(1));
        println!("CT15|primitive|kind=noncubic|i={i}|input={:?}|baseline={:?}|candidate={:?}|base_counts={old_counts:?}|fast_counts={new_counts:?}",encoding15(&baseline,&x),encoding15(&baseline,&old),encoding15(&candidate,&new));
    }
    let zero = baseline.c(0);
    assert!(catch_unwind(AssertUnwindSafe(|| baseline.inv(&zero))).is_err());
    assert!(catch_unwind(AssertUnwindSafe(|| candidate.inv(&zero))).is_err());
    println!(
        "CT15|control|zero_original_and_candidate|rejections=2|panic_hook_not_suppressed=true"
    );
    let mut constructor_controls = 0;
    for (label, p, coefficients) in [
        ("reducible_zero", 101, [0, 0, 0]),
        ("reducible_unit", 101, [100, 0, 0]),
        ("composite_modulus", 15, [1, 1, 1]),
    ] {
        match CubicField::with_cubic(p, coefficients) {
            Ok(_) => return Err(format!("invalid field control accepted: {label}")),
            Err(error) => {
                println!("CT15|control|constructor|label={label}|detected=true|error={error}")
            }
        }
        constructor_controls += 1;
    }
    assert_eq!(constructor_controls, 3);
    let old_map =
        RichelotMap::from_selected_pairing(assessment14(false, Mutation14::None)?, PAIRING14)?;
    let mut new_map =
        RichelotMap::from_selected_pairing(assessment14(false, Mutation14::None)?, PAIRING14)?;
    assert_model14(&old_map.source, &SOURCE14);
    assert_model14(&old_map.target, &TARGET14);
    assert_model14(&new_map.source, &SOURCE14);
    assert_model14(&new_map.target, &TARGET14);
    assert_eq!(old_map.target_scale, 13);
    assert_eq!(new_map.target_scale, 13);
    new_map.k.cubic_first15 = true;
    println!("CT15|models|source={SOURCE14:?}|target={TARGET14:?}|target_scale=13");
    let witnesses = crate::balanced::witnesses14(&old_map.source)?;
    assert_eq!(witnesses.len(), 17);
    let mut routes = [0_usize; 2];
    let mut shortcuts = 0;
    let mut general = 0;
    let mut closed_orbits = 0;
    for (label, input) in &witnesses {
        println!("CT15|attempt|map|label={label}");
        let (image, route, fast, fallback) = paired_map15(&old_map, &new_map, input, false, label)?;
        shortcuts += fast;
        general += fallback;
        match route {
            closed_fibres::ClosedRoute::LowerDegree => routes[0] += 1,
            closed_fibres::ClosedRoute::Quadratic => routes[1] += 1,
            other => return Err(format!("unexpected forward route {label}: {other:?}")),
        }
        if label == "quadratic_orbit" || label == "opposite_quadratic_orbit" {
            assert_eq!(route, closed_fibres::ClosedRoute::Quadratic);
            closed_orbits += 1;
        }
        let (_, _, fast, fallback) = paired_map15(&old_map, &new_map, &image, true, label)?;
        shortcuts += fast;
        general += fallback;
    }
    assert_eq!(routes, [9, 8]);
    assert_eq!(closed_orbits, 2);
    assert!(shortcuts > 0);
    println!("CT15|map_totals|forward=17|dual=17|routes={routes:?}|shortcuts={shortcuts}|general={general}|all_classes_and_routes_equal=true|wrapper_count_relations_checked=true");
    // Preserve the protocol's full criterion. Accumulate through all34 calls
    // so a counterexample retains the entire declared population, not a subset.
    assert_eq!(
        general, 0,
        "prospective cubic-only criterion on all34 observations"
    );
    println!("CT15|accepted|cubic=64|noncubic=8|zero_rejections=2|wrong_sign=8|constructor_controls=3|forward=17|dual=17|closed_orbits=2|shortcuts={shortcuts}|general={general}|no_timing_ratio|unsigned");
    Ok(())
}
