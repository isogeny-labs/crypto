//! Rational degree-2 isogenies on a depth-one ordinary volcano.
//!
//! This executable uses the native isogeny module on the toy curve
//!
//! ```text
//! E/F_17: y^2 = x^3 + 9x + 15,
//! ```
//!
//! which is the short-Weierstrass form of `y^2 = x(x - 1)(x - 3)`.
//! It checks the concrete facts behind the floor/crater picture:
//!
//! - `#E(F_17) = 24 = 8 * 3` and all three nonzero points of `E[2]` are rational;
//! - the three rational degree-2 quotients have j-invariants 6, 10, and 1;
//! - the `j = 10` quotient is explicitly F_17-isomorphic to `E`;
//! - the other two quotients have only their dual kernel rational;
//! - no rational non-backtracking chain reaches five steps (cyclic degree 32).
//!
//! Taking an isogeny and then its dual is deliberately excluded after the first
//! step. Such backtracking can produce arbitrarily long rational chains, but
//! two consecutive steps then have non-cyclic composite kernel. See Vélu's
//! formulas (1971), Sutherland's ANTS X volcano survey (2013), and the
//! repository's `research/isogeny_vs_mu_factor_base_20261102/` study.

use std::collections::BTreeMap;

use crypto_lib::ecc::point::Point;
use crypto_lib::isogeny::cm::cm_discriminant;
use crypto_lib::isogeny::velu::velu_isogeny_2;
use crypto_lib::isogeny::volcano::j_invariant;
use crypto_lib::isogeny::SmallCurve;
use num_bigint::BigUint;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Chain {
    kernels: Vec<u64>,
    j_path: Vec<u64>,
}

fn example_curve() -> SmallCurve {
    SmallCurve {
        name: "toy-depth-one-2-volcano",
        p: 17,
        a: 9,
        b: 15,
    }
}

fn rational_two_torsion_x(curve: &SmallCurve) -> Vec<u64> {
    (0..curve.p).filter(|&x| curve.rhs(x) == 0).collect()
}

/// For the short-Weierstrass Vélu model used by `velu_isogeny_2`, the
/// codomain's dual kernel has abscissa `-2*x_t`.
fn dual_kernel_x(p: u64, x_t: u64) -> u64 {
    (p - (2 * x_t) % p) % p
}

/// Find `u` for the F_p-isomorphism `(x, y) -> (u^2*x, u^3*y)`.
fn isomorphism_scale(source: &SmallCurve, target: &SmallCurve) -> Option<u64> {
    if source.p != target.p {
        return None;
    }
    let p = source.p as u128;
    (1..source.p).find(|&u| {
        let u = u as u128;
        let u2 = u * u % p;
        let u4 = u2 * u2 % p;
        let u6 = u4 * u2 % p;
        source.a as u128 * u4 % p == target.a as u128 % p
            && source.b as u128 * u6 % p == target.b as u128 % p
    })
}

fn maximal_nonbacktracking_chains(curve: SmallCurve, limit: usize) -> Vec<Chain> {
    fn visit(
        curve: SmallCurve,
        excluded_dual: Option<u64>,
        limit: usize,
        kernels: &mut Vec<u64>,
        j_path: &mut Vec<u64>,
        result: &mut Vec<Chain>,
    ) {
        if kernels.len() == limit {
            result.push(Chain {
                kernels: kernels.clone(),
                j_path: j_path.clone(),
            });
            return;
        }

        let candidates: Vec<u64> = rational_two_torsion_x(&curve)
            .into_iter()
            .filter(|x| Some(*x) != excluded_dual)
            .collect();
        if candidates.is_empty() {
            result.push(Chain {
                kernels: kernels.clone(),
                j_path: j_path.clone(),
            });
            return;
        }

        for kernel_x in candidates {
            let isogeny = velu_isogeny_2(&curve, kernel_x).expect("enumerated root is 2-torsion");
            kernels.push(kernel_x);
            j_path.push(j_invariant(&isogeny.codomain));
            visit(
                isogeny.codomain,
                Some(dual_kernel_x(curve.p, kernel_x)),
                limit,
                kernels,
                j_path,
                result,
            );
            kernels.pop();
            j_path.pop();
        }
    }

    let mut result = Vec::new();
    visit(
        curve,
        None,
        limit,
        &mut Vec::new(),
        &mut vec![j_invariant(&curve)],
        &mut result,
    );
    result
}

fn rational_points(curve: &SmallCurve) -> Vec<Point> {
    let params = curve.to_curve_params();
    let mut points = vec![Point::Infinity];
    for x in 0..curve.p {
        for y in 0..curve.p {
            if y as u128 * y as u128 % curve.p as u128 == curve.rhs(x) as u128 {
                points.push(Point::Affine {
                    x: params.fe(BigUint::from(x)),
                    y: params.fe(BigUint::from(y)),
                });
            }
        }
    }
    points
}

fn point_order_histogram(curve: &SmallCurve) -> BTreeMap<u64, usize> {
    let params = curve.to_curve_params();
    let a = params.a_fe();
    let group_order = cm_discriminant(curve).order as u64;
    let mut histogram = BTreeMap::new();

    for point in rational_points(curve) {
        let order = if matches!(point, Point::Infinity) {
            1
        } else {
            let mut sum = Point::Infinity;
            let mut found = None;
            for n in 1..=group_order {
                sum = sum.add_vartime(&point, &a);
                if matches!(sum, Point::Infinity) {
                    found = Some(n);
                    break;
                }
            }
            found.expect("point order divides #E(F_p)")
        };
        *histogram.entry(order).or_insert(0) += 1;
    }
    histogram
}

fn main() {
    let curve = example_curve();
    let cm = cm_discriminant(&curve);
    let roots = rational_two_torsion_x(&curve);

    println!("Base curve");
    println!("  E/F_17: y^2 = x^3 + 9*x + 15");
    println!("  #E(F_17) = {} = 8 * {}", cm.order, cm.order / 8);
    println!(
        "  Frobenius trace = {}; ordinary = {}",
        cm.trace, !cm.supersingular
    );
    println!(
        "  Frobenius discriminant = {} = {} * {}^2 (maximum rational 2-volcano depth 1)",
        cm.frobenius_disc, cm.fundamental_disc, cm.frobenius_order_conductor
    );
    println!("  j(E) = {}", j_invariant(&curve));
    println!("  nonzero rational E[2] abscissae = {roots:?}");

    println!("\nThe three rational degree-2 Velu quotients");
    for kernel_x in roots {
        let isogeny = velu_isogeny_2(&curve, kernel_x).expect("root defines a 2-isogeny");
        let quotient = isogeny.codomain;
        let quotient_roots = rational_two_torsion_x(&quotient);
        let dual_x = dual_kernel_x(curve.p, kernel_x);
        let relation = if let Some(scale) = isomorphism_scale(&curve, &quotient) {
            format!("F_17-isomorphic to E (u={scale})")
        } else if quotient_roots == [dual_x] {
            "only the dual rational kernel remains".to_owned()
        } else {
            "additional rational kernels remain".to_owned()
        };
        println!(
            "  ker <({kernel_x},0)> -> y^2=x^3+{}x+{}, j={}, roots={quotient_roots:?}, dual x={dual_x} [{relation}]",
            quotient.a,
            quotient.b,
            j_invariant(&quotient),
        );
    }

    let orders = point_order_histogram(&curve);
    println!("\nRational point orders");
    println!("  {orders:?}");
    println!(
        "  rational point of order 8 exists: {}",
        orders.contains_key(&8)
    );

    let requested_steps = 5;
    let chains = maximal_nonbacktracking_chains(curve, requested_steps);
    println!(
        "\nNon-backtracking search (requested r={requested_steps}, degree 2^{requested_steps}=32)"
    );
    for chain in &chains {
        println!(
            "  kernels x={:?}; j path={:?}; stopped after {} step(s)",
            chain.kernels,
            chain.j_path,
            chain.kernels.len()
        );
    }
    let full_length = chains
        .iter()
        .any(|chain| chain.kernels.len() == requested_steps);
    println!("  full rational cyclic chain found: {full_length}");
    println!(
        "  The missing forward kernels at the terminal vertices are defined over F_17^2, not F_17."
    );
    println!(
        "  Caveat: allowing each dual edge permits arbitrary backtracking chains, but their composite kernels are not cyclic."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_curve_has_order_24_and_full_rational_two_torsion() {
        let curve = example_curve();
        let cm = cm_discriminant(&curve);
        assert_eq!(cm.order, 24);
        assert_eq!(cm.trace, -6);
        assert!(!cm.supersingular);
        assert_eq!(cm.frobenius_disc, -32);
        assert_eq!(cm.fundamental_disc, -8);
        assert_eq!(cm.frobenius_order_conductor, 2);
        assert_eq!(j_invariant(&curve), 10);
        assert_eq!(rational_two_torsion_x(&curve), [10, 11, 13]);
    }

    #[test]
    fn three_quotients_have_the_depth_one_shape() {
        let curve = example_curve();
        let quotients: Vec<_> = rational_two_torsion_x(&curve)
            .into_iter()
            .map(|x| velu_isogeny_2(&curve, x).unwrap().codomain)
            .collect();

        assert_eq!(
            quotients.iter().map(j_invariant).collect::<Vec<_>>(),
            [6, 10, 1]
        );
        assert_eq!(
            quotients
                .iter()
                .map(|q| rational_two_torsion_x(q).len())
                .collect::<Vec<_>>(),
            [1, 3, 1]
        );
        assert!(quotients.iter().all(|q| cm_discriminant(q).order == 24));
        assert_eq!(isomorphism_scale(&curve, &quotients[1]), Some(7));
    }

    #[test]
    fn terminal_rational_kernel_is_exactly_the_dual() {
        let curve = example_curve();
        for kernel_x in [10, 13] {
            let quotient = velu_isogeny_2(&curve, kernel_x).unwrap().codomain;
            assert_eq!(
                rational_two_torsion_x(&quotient),
                [dual_kernel_x(curve.p, kernel_x)]
            );
        }
    }

    #[test]
    fn no_rational_point_has_order_eight() {
        let histogram = point_order_histogram(&example_curve());
        assert_eq!(
            histogram,
            BTreeMap::from([(1, 1), (2, 3), (3, 2), (4, 4), (6, 6), (12, 8)])
        );
        assert!(!histogram.contains_key(&8));
    }

    #[test]
    fn no_five_step_nonbacktracking_chain_exists() {
        let chains = maximal_nonbacktracking_chains(example_curve(), 5);
        let mut paths: Vec<Vec<u64>> = chains.iter().map(|chain| chain.kernels.clone()).collect();
        paths.sort();
        assert_eq!(paths, [vec![10], vec![11, 8], vec![11, 14], vec![13]]);
        assert!(chains.iter().all(|chain| chain.kernels.len() < 5));
    }
}
