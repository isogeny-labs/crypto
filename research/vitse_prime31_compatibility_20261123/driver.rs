//! Fixed-input native compatibility audit; no timing or scalar solver.
//! The parent prepares generated/pkm_prefix.rs as an exact prefix of the
//! pinned production source, with a separately recorded source digest.

#![allow(dead_code)]

#[path = "generated/pkm_prefix.rs"]
mod production;
#[path = "compatibility.rs"]
mod compatibility;

use compatibility::*;
use production::{Curve, Pt};

const SOURCE_COMMIT: &str = "7d61e3b45de6e93d235345a0061dae08db1f8b44";
const SELECTOR_DEGREES: [u64; 4] = [2, 3, 7, 31];
const SELECTOR_DEGREE: u64 = 1_302;
const SEED: u64 = 20_261_123;

fn require(condition: bool, message: &str) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message.into()) }
}

fn json_string(value: &str) -> String {
    let mut result = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c < '\u{20}' => result.push_str(&format!("\\u{:04x}", c as u32)),
            c => result.push(c),
        }
    }
    result.push('"');
    result
}

fn optional_json(value: Option<u64>) -> String {
    value.map_or_else(|| "null".into(), |x| x.to_string())
}

fn point_json(point: Pt) -> String {
    match point {
        None => "null".into(),
        Some((x, y)) => format!("[{x},{y}]"),
    }
}

fn points_json(points: &[Pt]) -> String {
    format!("[{}]", points.iter().map(|&p| point_json(p)).collect::<Vec<_>>().join(","))
}

fn domain_json(audit: &DomainAudit) -> String {
    let escaped: Vec<[u64; 2]> = audit.escaped.iter().map(|&(x, y)| [x, y]).collect();
    let first_escaped = escaped.first().map_or_else(|| "null".into(), |pair| format!("{pair:?}"));
    format!(
        "{{\"cardinality\":{},\"set_closed\":{},\"nonvacuous_bijection\":{},\"pole_count\":{},\"poles\":{:?},\"escaped_count\":{},\"escaped\":{:?},\"first_pole\":{},\"first_escape\":{}}}",
        audit.cardinality, audit.set_closed, audit.nonvacuous_bijection,
        audit.poles.len(), audit.poles, audit.escaped.len(), escaped,
        optional_json(audit.poles.first().copied()), first_escaped,
    )
}

/// Production-field evaluator at the seam, reusing the existing arithmetic.
/// This cross-checks the std-only audit's small map evaluator on EVERY root.
fn production_image(map: Mobius, x: u64) -> Option<u64> {
    let [a, b, c, d] = map.coefficients();
    let p = PRIME31_P;
    let numerator = production::addm(production::mulm(a, x, p), b, p);
    let denominator = production::addm(production::mulm(c, x, p), d, p);
    if denominator == 0 {
        None
    } else {
        Some(production::mulm(numerator, production::invm(denominator, p), p))
    }
}

fn emit_leaf_case(label: &str, audit: &LeafPermutationAudit) {
    let mismatches: Vec<String> = audit.mismatches.iter().map(|m| format!(
        "{{\"destination\":{},\"source\":{},\"source_only\":{:?},\"destination_only\":{:?}}}",
        m.destination, m.source, m.source_only, m.destination_only,
    )).collect();
    println!(
        "{{\"type\":\"leaf_domain_audit\",\"case\":{},\"source_for_destination\":{:?},\"leafwise_equal\":{},\"cartesian_domain_equal\":{},\"nonvacuous_domain_compatible\":{},\"empty_leaves\":{:?},\"mismatches\":[{}],\"scope\":\"finite_coordinate_sets_only\",\"chain_automorphism_verified\":false}}",
        json_string(label), audit.source_for_destination, audit.leafwise_equal,
        audit.cartesian_domain_equal, audit.nonvacuous_domain_compatible,
        audit.empty_leaves, mismatches.join(","),
    );
}

fn sum(curve: &Curve, points: &[Pt]) -> Pt {
    points.iter().fold(None, |acc, &point| curve.add(acc, point))
}

fn run() -> Result<(), String> {
    require(std::env::args().count() == 1, "the audit accepts no variable inputs")?;
    let curve = Curve { p: PRIME31_P, a: PRIME31_A, b: PRIME31_B };
    let g = Some(PRIME31_GENERATOR);
    let p_prime = production::is_prime(PRIME31_P);
    let n_prime = production::is_prime(PRIME31_ORDER);
    let nonsingular = !curve.is_singular();
    let g_nonzero = g.is_some();
    let g_on_curve = curve.on_curve(g);
    let ng_zero = curve.mul(PRIME31_ORDER, g).is_none();
    let sqrt_upper = 46_341_u64;
    let sqrt_upper_square_gt_p = sqrt_upper as u128 * sqrt_upper as u128 > PRIME31_P as u128;
    let hasse_upper = PRIME31_P as u128 + 1 + 2 * sqrt_upper as u128;
    let hasse_upper_lt_twice_n = hasse_upper < 2 * PRIME31_ORDER as u128;
    let order_admitted = p_prime && n_prime && nonsingular && g_nonzero && g_on_curve
        && ng_zero && sqrt_upper_square_gt_p && hasse_upper_lt_twice_n;
    println!(
        "{{\"type\":\"fixed_curve_admission\",\"source_commit\":{},\"p\":{},\"a\":{},\"b\":{},\"n\":{},\"g\":{},\"p_prime\":{},\"n_prime\":{},\"nonsingular\":{},\"g_nonzero\":{},\"g_on_curve\":{},\"n_times_g_is_identity\":{},\"sqrt_upper\":{},\"sqrt_upper_square_gt_p\":{},\"hasse_upper\":{},\"hasse_upper_lt_twice_n\":{},\"order_admitted_by_prime_order_point_and_hasse\":{},\"independent_point_count_performed\":false,\"new_curve_selected\":false}}",
        json_string(SOURCE_COMMIT), PRIME31_P, PRIME31_A, PRIME31_B, PRIME31_ORDER,
        point_json(g), p_prime, n_prime, nonsingular, g_nonzero, g_on_curve, ng_zero,
        sqrt_upper, sqrt_upper_square_gt_p, hasse_upper, hasse_upper_lt_twice_n, order_admitted,
    );
    require(order_admitted, "fixed Prime31 admission checks failed")?;

    let tower = production::kummer_tower(PRIME31_P, &SELECTOR_DEGREES, Some(1), SEED)?;
    tower.verify()?;
    let mut selector = tower.v.clone();
    let listed_count = selector.len();
    selector.sort_unstable();
    selector.dedup();
    require(listed_count == selector.len(), "production selector listed repeated roots")?;
    require(selector.len() as u64 == SELECTOR_DEGREE, "production selector cardinality is not d")?;
    require(SELECTOR_DEGREES.iter().product::<u64>() == SELECTOR_DEGREE, "degree manifest mismatch")?;
    require((PRIME31_P - 1) % SELECTOR_DEGREE == 0, "d does not divide p-1")?;
    require(selector.iter().all(|&x| production::powm(x, SELECTOR_DEGREE, PRIME31_P) == 1),
        "a selector coordinate is not a root of X^d-1")?;

    let mut retained = Vec::new();
    let mut retained_points: Vec<[u64; 2]> = Vec::new();
    let mut liftable_count = 0_usize;
    let mut subgroup_rejections = Vec::new();
    let mut nonresidues = Vec::new();
    for &x in &selector {
        match production::sqrtm(curve.rhs(x), PRIME31_P) {
            None => nonresidues.push(x),
            Some(y) => {
                liftable_count += 1;
                let point = Some((x, y));
                require(curve.on_curve(point), "production square root did not give a curve point")?;
                if curve.mul(PRIME31_ORDER, point).is_none() {
                    let opposite = curve.neg(point);
                    require(curve.on_curve(opposite) && curve.mul(PRIME31_ORDER, opposite).is_none(),
                        "opposite point sign failed curve or subgroup checks")?;
                    retained.push(x);
                    retained_points.push([x, y]);
                } else {
                    subgroup_rejections.push(x);
                }
            }
        }
    }
    println!(
        "{{\"type\":\"complete_supplied_selector_and_retained_support\",\"selector_kind\":\"production_kummer_X_pow_d_minus_1\",\"degrees\":{:?},\"d\":{},\"g\":1,\"seed\":{},\"tower_verify_passed\":true,\"d_distinct_roots_verified\":true,\"root_completeness_reason\":\"d_distinct_roots_of_monic_degree_d_polynomial\",\"selector_count\":{},\"liftable_count\":{},\"retained_count\":{},\"nonresidue_count\":{},\"subgroup_rejection_count\":{},\"selector\":{:?},\"retained\":{:?},\"retained_point_representatives\":{:?},\"nonresidues\":{:?},\"subgroup_rejections\":{:?},\"both_point_signs_supported\":true,\"relation_extraction_performed\":false}}",
        SELECTOR_DEGREES, SELECTOR_DEGREE, SEED, selector.len(), liftable_count,
        retained.len(), nonresidues.len(), subgroup_rejections.len(), selector,
        retained, retained_points, nonresidues, subgroup_rejections,
    );

    let maps = [
        ("identity", Mobius::identity()),
        ("coordinate_negation", Mobius::coordinate_negation()),
        ("reciprocal", Mobius::reciprocal()),
        ("negative_reciprocal", Mobius::new(0, PRIME31_P - 1, 1, 0).map_err(|e| e.to_string())?),
    ];
    for &(name, map) in &maps {
        for &x in &selector {
            require(map.apply(x).map_err(|e| e.to_string())? == production_image(map, x),
                "map evaluator disagrees with the pinned production arithmetic")?;
        }
        let audit = audit_selector_support(&selector, &retained, map).map_err(|e| e.to_string())?;
        println!(
            "{{\"type\":\"selector_map_audit\",\"map\":{},\"coefficients\":{:?},\"production_evaluator_crosschecks\":{},\"selector\":{},\"retained\":{},\"both_nonvacuously_closed\":{},\"scope\":\"finite_coordinate_sets_only\",\"geometric_point_action_verified\":false,\"relation_preservation_verified\":false}}",
            json_string(name), map.coefficients(), selector.len(),
            domain_json(&audit.selector), domain_json(&audit.retained), audit.both_nonvacuously_closed,
        );
    }

    let equal_leaves: [&[u64]; 4] = [&retained, &retained, &retained, &retained];
    let equal = audit_leaf_permutation(&equal_leaves, &[3, 1, 0, 2]).map_err(|e| e.to_string())?;
    emit_leaf_case("four_equal_retained_domains", &equal);
    if retained.len() >= 2 {
        let prefix_length = retained.len() / 2;
        let prefix = &retained[..prefix_length];
        println!(
            "{{\"type\":\"heterogeneous_domain_control\",\"construction\":\"first_floor_half_of_sorted_actual_retained_coordinates\",\"full_size\":{},\"fourth_leaf_size\":{},\"fourth_leaf\":{:?},\"scope\":\"domain_guard_only_no_relation_solution_claim\"}}",
            retained.len(), prefix_length, prefix,
        );
        let different: [&[u64]; 4] = [&retained, &retained, &retained, prefix];
        let first_three = audit_leaf_permutation(&different, &[2, 0, 1, 3]).map_err(|e| e.to_string())?;
        let cross = audit_leaf_permutation(&different, &[3, 1, 2, 0]).map_err(|e| e.to_string())?;
        require(first_three.nonvacuous_domain_compatible, "equal first-three-domain guard failed")?;
        require(!cross.cartesian_domain_equal, "strict smaller fourth domain incorrectly exchangeable")?;
        emit_leaf_case("permute_equal_first_three_keep_fourth", &first_three);
        emit_leaf_case("swap_full_domain_with_smaller_fourth", &cross);
    } else {
        println!("{{\"type\":\"heterogeneous_domain_control\",\"performed\":false,\"reason\":\"retained_support_has_fewer_than_two_coordinates\"}}");
    }

    // Public control R=G, P=2G, Q=-G. It is NOT a claimed decomposition
    // over the audited selector and is not passed to any relation solver.
    let p_point = curve.mul(2, g);
    let q_point = curve.neg(g);
    require(p_point.is_some() && q_point.is_some(), "public pair contains infinity")?;
    require(curve.on_curve(p_point) && curve.on_curve(q_point), "public pair is off curve")?;
    require(curve.add(p_point, q_point) == g, "public pair does not add to fixed R=G")?;
    let xp = p_point.unwrap().0;
    let xq = q_point.unwrap().0;
    let xt = PRIME31_GENERATOR.0;
    let s3 = production::s3(&curve, xp, xq, xt);
    let swapped_s3 = production::s3(&curve, xq, xp, xt);
    require(s3 == 0 && swapped_s3 == 0, "production S3 or pair-permutation control failed")?;
    let negative_sum = curve.add(curve.neg(p_point), curve.neg(q_point));
    require(curve.neg(p_point).map(|point| point.0) == Some(xp)
        && curve.neg(q_point).map(|point| point.0) == Some(xq),
        "point negation changed an abscissa")?;
    require(negative_sum == curve.neg(g) && negative_sum != g,
        "signed point-negation fixed-target control failed")?;
    println!(
        "{{\"type\":\"public_fixed_target_pair_control\",\"r\":{},\"p_point\":{},\"q_point\":{},\"p_plus_q\":{},\"s3\":{},\"swapped_s3\":{},\"both_points_negated_sum\":{},\"point_negation_preserves_fixed_signed_target\":{},\"point_negation_keeps_x_coordinates\":true,\"selector_constrained_decomposition_claim\":false,\"scope\":\"public_group_and_polynomial_correctness_control\"}}",
        point_json(g), point_json(p_point), point_json(q_point),
        point_json(curve.add(p_point, q_point)), s3, swapped_s3,
        point_json(negative_sum), negative_sum == g,
    );
    for &(name, map) in &maps {
        let mxp = map.apply(xp).map_err(|e| e.to_string())?;
        let mxq = map.apply(xq).map_err(|e| e.to_string())?;
        let residual = match (mxp, mxq) {
            (Some(x), Some(y)) => Some(production::s3(&curve, x, y, xt)),
            _ => None,
        };
        let rejected_by_s3 = residual.map(|value| value != 0);
        let rejected_json = rejected_by_s3.map_or_else(|| "null".into(), |b| b.to_string());
        println!(
            "{{\"type\":\"coordinate_map_fixed_target_s3_control\",\"map\":{},\"original_x\":[{},{}],\"mapped_x\":[{},{}],\"fixed_target_x\":{},\"s3_residual\":{},\"rejected_as_s3_solution\":{},\"geometric_signed_relation_verified\":false,\"zero_residual_alone_is_not_signed_relation\":true,\"selector_constrained_decomposition_claim\":false}}",
            json_string(name), xp, xq, optional_json(mxp), optional_json(mxq),
            xt, optional_json(residual), rejected_json,
        );
    }

    // The affine intermediate chart depends on ordering even when the sum
    // relation and all leaf domains are permutation-invariant.
    let original = [g, g, curve.neg(g), g];
    let swap_first_two = [original[1], original[0], original[2], original[3]];
    let swap_slots_one_two = [original[0], original[2], original[1], original[3]];
    let original_prefix = curve.add(original[0], original[1]);
    let first_swap_prefix = curve.add(swap_first_two[0], swap_first_two[1]);
    let crossed_prefix = curve.add(swap_slots_one_two[0], swap_slots_one_two[1]);
    let original_second_prefix = curve.add(original_prefix, original[2]);
    let first_swap_second_prefix = curve.add(first_swap_prefix, swap_first_two[2]);
    let crossed_second_prefix = curve.add(crossed_prefix, swap_slots_one_two[2]);
    let original_total = sum(&curve, &original);
    let first_swap_total = sum(&curve, &swap_first_two);
    let crossed_total = sum(&curve, &swap_slots_one_two);
    require(original_prefix == p_point && original_prefix.is_some(), "original chart prefix is not 2G")?;
    require(first_swap_prefix == original_prefix, "first-two swap changed first prefix")?;
    require(crossed_prefix.is_none(), "crossed chart control did not produce infinity")?;
    require(original_second_prefix == g && first_swap_second_prefix == g && crossed_second_prefix == g,
        "second-prefix chart controls did not return to G")?;
    require(original_total == p_point && first_swap_total == p_point && crossed_total == p_point,
        "chart permutation changed the public total 2G")?;
    println!(
        "{{\"type\":\"affine_chain_chart_control\",\"original_points\":{},\"swap_first_two_points\":{},\"swap_zero_based_slots_one_two_points\":{},\"original_first_prefix\":{},\"first_two_swap_prefix\":{},\"crossed_first_prefix\":{},\"original_second_prefix\":{},\"first_two_swap_second_prefix\":{},\"crossed_second_prefix\":{},\"original_total\":{},\"first_two_swap_total\":{},\"crossed_total\":{},\"original_first_prefix_is_finite\":{},\"crossed_first_prefix_is_identity\":{},\"all_totals_equal_2g\":{},\"selector_constrained_solution_claim\":false,\"scope\":\"public_chart_fixture_only\"}}",
        points_json(&original), points_json(&swap_first_two), points_json(&swap_slots_one_two),
        point_json(original_prefix), point_json(first_swap_prefix), point_json(crossed_prefix),
        point_json(original_second_prefix), point_json(first_swap_second_prefix), point_json(crossed_second_prefix),
        point_json(original_total), point_json(first_swap_total), point_json(crossed_total),
        original_prefix.is_some(), crossed_prefix.is_none(),
        original_total == p_point && first_swap_total == p_point && crossed_total == p_point,
    );
    println!("{{\"type\":\"completion\",\"status\":\"complete\",\"scope\":\"finite_domain_checks_and_public_correctness_controls\",\"timed_benchmark\":false,\"scalar_solver_executed\":false,\"asymptotic_speedup_claim\":false,\"new_curve_selected\":false}}");
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        println!("{{\"type\":\"failure\",\"error\":{}}}", json_string(&error));
        std::process::exit(1);
    }
}
