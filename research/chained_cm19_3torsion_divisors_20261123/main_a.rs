#[path = "algebra_a.rs"]
mod algebra;
#[path = "divisors_a.rs"]
mod divisors;
#[path = "elliptic_a.rs"]
mod elliptic;
#[path = "field_a.rs"]
mod field;
#[path = "linear_a.rs"]
mod linear;
#[path = "polynomial_a.rs"]
mod polynomial;

use std::collections::BTreeSet;

fn main() {
    println!("PROFILE\tactual_C7_E7_3torsion_conic_divisor_certificates_A\tunsigned_development_not_benchmark");
    field::qualify();
    let points = elliptic::torsion();
    let curve = polynomial::quartic();
    divisors::controls(&curve);
    let mut cells = Vec::new();
    for &point in points.iter().filter(|p| p.is_some()) {
        for sign in [1, -1] {
            cells.push(divisors::cell(point, sign, &curve));
        }
    }
    assert_eq!(cells.len(), 16);
    for cell in &cells {
        let conjugate = cells
            .iter()
            .find(|c| c.point == elliptic::frobenius(cell.point) && c.sign == cell.sign)
            .expect("complete paired Frobenius cell");
        if let (Some(a), Some(b)) = (cell.plus, conjugate.plus) {
            assert_eq!(a.map(|v| v.power(7)), b);
        }
        if let (Some(a), Some(b)) = (cell.minus, conjugate.minus) {
            assert_eq!(a.map(|v| v.power(7)), b);
        }
        assert_eq!(cell.certified, conjugate.certified);
    }
    let mut full_kernel_signs = Vec::new();
    for sign in [1, -1] {
        let certified: Vec<_> = cells
            .iter()
            .filter(|c| c.sign == sign && c.certified)
            .collect();
        let mut basis = None;
        for a in &certified {
            for b in &certified {
                let span: BTreeSet<_> = (0..3)
                    .flat_map(|i| {
                        (0..3).map(move |j| {
                            elliptic::add(
                                elliptic::multiple(a.point, i),
                                elliptic::multiple(b.point, j),
                            )
                        })
                    })
                    .collect();
                if span.len() == 9 {
                    assert_eq!(span, points.iter().copied().collect::<BTreeSet<_>>());
                    basis.get_or_insert((a.point, b.point));
                }
            }
        }
        println!("SIGN_RESULT\tsign={}\tcertified_cells={}\tretained_other_cells={}\tfull_kernel_basis={:?}", sign, certified.len(), 8 - certified.len(), basis);
        if basis.is_some() {
            full_kernel_signs.push(sign);
        }
    }
    assert!(
        full_kernel_signs.len() <= 1,
        "both signs conflict with norm3 pullback/descent; retain and diagnose"
    );
    if let Some(&sign) = full_kernel_signs.first() {
        println!("GEOMETRIC_KERNEL_CERTIFICATE\tsign={}\tcomplete_E3_basis_covered\tconditional_written_[3]_descent_and_H_basis\tPASS", sign);
    } else {
        println!("INCONCLUSIVE\tno_sign_has_sufficient_principal_divisor_basis\tno_divided_integral_map_claimed");
    }
    println!("SUMMARY\tfield_scan=2401\tactual_E3_points=9\tcells=16\tcertified_plus={}\tcertified_minus={}\tfull_kernel_signs={:?}\tall_cells_and_degeneracies_retained\tPASS", cells.iter().filter(|c| c.sign == 1 && c.certified).count(), cells.iter().filter(|c| c.sign == -1 && c.certified).count(), full_kernel_signs);
    println!("SCOPE\tactual_finite_fibre_algebra_and_conic_divisor_certificates\tglobal_kernel_descent_is_written_argument\tno_generic_Lean_or_independent_signed_verdict_or_full_S_J4_target_engine_or_ECDLP_cost_gain");
}
