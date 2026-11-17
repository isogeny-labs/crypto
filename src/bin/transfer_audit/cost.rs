//! Pricing a transfer in the phases the transfer discipline requires.
//!
//! Every formula that the repository already has is called, not copied:
//! the frozen [`CostModel`] prices the elliptic baseline, a Jacobian
//! operation and every destination solver.  What this module adds is (i)
//! the split of the hyperelliptic index-calculus cost into relation
//! collection and linear algebra, which `CostModel` returns only as a sum
//! (a test pins the sum), (ii) the Hasse-Weil capacity gate, (iii) the
//! `L[1/3]` price of index calculus in `F_(q^k)` for the pairing route and
//! (iv) the lead classification.

use crypto_lib::cryptanalysis::jacobian_transfer::cost::{
    log2_factorial, CostModel, DestinationSolver,
};

use crate::certificate::{round3, CostRow, CostStatus, CostTable, LeadClass};

/// Smallest published `L[1/3]` constant, `(32/9)^(1/3)` (FFS/SNFS family).
/// An attacker-favourable lower bound: no generic constant is smaller.
pub const LFS_C: f64 = 1.526_294_3;

/// The generic number-field-sieve constant `(64/9)^(1/3)`: the conservative
/// end of the pairing price band.
pub const LNFS_C: f64 = 1.923_001_7;

/// Field multiplications in `F_(q^k)` per bit of `r` in one pairing
/// evaluation.  An order-of-magnitude placeholder that never decides a
/// verdict: the discrete logarithm in `F_(q^k)` dominates by many bits.
pub const MILLER_MULTS_PER_BIT: f64 = 12.0;

/// Field multiplications per kernel point in one Velu evaluation
/// (placeholder; an isogeny leaves the destination solver unchanged).
pub const VELU_MULTS_PER_KERNEL_POINT: f64 = 4.0;

/// Lead class of a route from its advantage over the matched baseline in
/// bits (`baseline - route`, positive means cheaper than rho): at least
/// `CostModel::advantage_margin_bits` is `formula_advantage`, positive below
/// that `formula_marginal`, otherwise `boundary`.
pub fn classify(model: &CostModel, carried: bool, advantage_bits: Option<f64>) -> LeadClass {
    if !carried {
        return LeadClass::NotCarried;
    }
    match advantage_bits {
        None => LeadClass::NotPriced,
        Some(r) if r >= model.advantage_margin_bits => LeadClass::FormulaAdvantage,
        Some(r) if r > 0.0 => LeadClass::FormulaMarginal,
        Some(_) => LeadClass::Boundary,
    }
}

/// `log2` of the Hasse-Weil upper bound on `#Jac(F_(2^l))` per unit of
/// genus: `2 log2(sqrt(q) + 1)`.
pub fn weil_bits_per_genus(base_field_bits: f64) -> f64 {
    2.0 * ((0.5 * base_field_bits).exp2() + 1.0).log2()
}

/// Least genus `g` with `(sqrt q + 1)^(2g) >= r`: below it the Jacobian over
/// the descent field cannot contain a subgroup of order `r`.
pub fn min_genus_to_carry(log2_r: f64, base_field_bits: f64) -> f64 {
    (log2_r / weil_bits_per_genus(base_field_bits))
        .ceil()
        .max(1.0)
}

/// Whether a genus-`2^log2_genus` Jacobian over `F_(2^l)` can hold `r`.
pub fn jacobian_can_carry(log2_genus: f64, base_field_bits: f64, log2_r: f64) -> bool {
    log2_genus.exp2() * weil_bits_per_genus(base_field_bits) >= log2_r
}

/// `log2` of `L_N[1/3, c]` with `o(1)` dropped, `N = q^k`
/// (`log2_n_bits = k log2 q`).  Indicative: an operation count, not a bit
/// count, and the dropped `o(1)` can move it by many bits.
pub fn log2_l13(log2_n_bits: f64, c: f64) -> f64 {
    let ln_n = log2_n_bits * std::f64::consts::LN_2;
    c * ln_n.cbrt() * ln_n.ln().powf(2.0 / 3.0) / std::f64::consts::LN_2
}

/// The relation-collection and linear-algebra terms of Gaudry's
/// double-large-prime price, each including one Jacobian operation, as
/// `CostModel::log2_hyperelliptic_ic` forms them.  Their `log2_add` equals
/// that function's value (tested).
pub fn ic_phases(model: &CostModel, log2_genus: f64, field_bits: f64) -> (f64, f64) {
    let genus = log2_genus.exp2();
    let op = model.log2_jacobian_op(log2_genus, field_bits);
    let la = (2.0 - 2.0 / genus) * field_bits
        + model.hyperelliptic_ic_genus_polynomial_exponent * log2_genus
        + op;
    let rel = log2_factorial(genus) + field_bits + op;
    (rel, la)
}

/// Cost of constructing the map (or finding it, for an isogeny walk).
#[derive(Clone, Debug)]
pub enum ConstructionCost {
    Priced(f64, String),
    /// Needed, and nothing in the repository prices it: the route total is
    /// then a lower bound.
    Unpriced(String),
}

/// What a Jacobian-destination route needs to be priced.
pub struct Route {
    pub log2_genus: f64,
    /// `log2` of the field the destination is defined over.
    pub field_bits: f64,
    /// `log2` of the field `E` is defined over.
    pub source_field_bits: f64,
    pub log2_r: f64,
    pub construction: ConstructionCost,
    /// Replace the default one-evaluation price `log2 g + log2(op)`.
    pub transfer_eval_override: Option<(f64, String)>,
    pub baseline_log2: f64,
    /// The cost model's own rho reference `sqrt(pi r / 4)` on `E`: negation
    /// included, no further automorphism.
    pub reference_no_aut_log2: f64,
    /// Size `c` of the equivalence class (negation included) that rho can
    /// walk on the image inside the destination Jacobian; `2` is negation
    /// only, which is all the frozen model's rho-on-Jacobian row assumes.
    /// Greater than 2 only where the map is defined over the subfield that
    /// gives `E` its extra class (Frobenius), so the class lifts to the image.
    pub image_class: u64,
}

/// Bits Pollard rho gains from an equivalence class of `class_size` elements
/// (negation included) over the cost model's reference, which is the
/// `c = 2` case: the expected number of steps is `sqrt(pi r / (2c))`, and
/// `CostModel::log2_rho_elliptic` is `sqrt(pi r / 4)`, so the class enters as
/// `sqrt(c / 2)`.  (Dividing the reference by `sqrt(c)` would count negation
/// twice: Amendment 3.)
pub fn class_discount_bits(class_size: u64) -> f64 {
    0.5 * (class_size.max(2) as f64 / 2.0).log2()
}

/// `log2` of rho on `E` with an equivalence class of `class_size` elements.
pub fn log2_rho_with_class(
    model: &CostModel,
    subgroup_bits: f64,
    field_bits: f64,
    class_size: u64,
) -> f64 {
    model.log2_rho_elliptic(subgroup_bits, field_bits) - class_discount_bits(class_size)
}

/// Price a route into a genus-`g` Jacobian.  Rows: map construction,
/// transfer evaluation, relation collection, linear algebra, an unsplit
/// destination row for solvers whose phases `CostModel` does not split,
/// and recovery/verification.  When the image has an equivalence class
/// beyond negation (`Route::image_class`), rho on the Jacobian is
/// discounted by it, and the total under the frozen model alone is kept in
/// `log2_total_frozen_model` for the cross-check with the repository.
pub fn price_jacobian_route(model: &CostModel, rt: &Route) -> CostTable {
    let discount = class_discount_bits(rt.image_class);
    let mut t = price_with_discount(model, rt, discount);
    if discount > 0.0 {
        t.log2_total_frozen_model = price_with_discount(model, rt, 0.0).log2_total;
    }
    t
}

fn price_with_discount(model: &CostModel, rt: &Route, rho_discount_bits: f64) -> CostTable {
    let dest = model.jacobian_destination(rt.log2_r, rt.log2_genus, rt.field_bits);
    // Rho on the image walks the image's equivalence class; the other
    // destination solvers are unaffected.
    let mut considered: Vec<(DestinationSolver, f64)> = dest.considered.clone();
    for (solver, cost) in considered.iter_mut() {
        if *solver == DestinationSolver::RhoOnJacobian {
            *cost -= rho_discount_bits;
        }
    }
    let (dest_solver, dest_cost) = considered
        .iter()
        .copied()
        .fold(
            None,
            |best: Option<(DestinationSolver, f64)>, x| match best {
                Some(b) if b.1 <= x.1 => Some(b),
                _ => Some(x),
            },
        )
        .expect("at least rho");
    // One evaluation writes a genus-g divisor: at least g source-field
    // elliptic operations (the repository's `jacobian_route` convention).
    let eval_default = rt.log2_genus + model.log2_elliptic_op(rt.source_field_bits);
    let verify = (1.5 * rt.log2_r).log2() + model.log2_elliptic_op(rt.source_field_bits);
    let mut rows = Vec::new();
    rows.push(match &rt.construction {
        ConstructionCost::Priced(c, f) => CostRow::modeled("map_construction", *c, f.clone()),
        ConstructionCost::Unpriced(f) => {
            CostRow::absent("map_construction", CostStatus::NotEvaluated, f.clone())
        }
    });
    rows.push(match &rt.transfer_eval_override {
        Some((v, f)) => CostRow::modeled("transfer_evaluation", *v, f.clone()),
        None => CostRow::modeled(
            "transfer_evaluation",
            eval_default,
            "g elliptic operations over the source field: log2 g + log2(10 * M(source_bits)); a lower bound, the conorm-norm formulas are not implemented",
        ),
    });
    let (rel, la) = ic_phases(model, rt.log2_genus, rt.field_bits);
    if dest_solver == DestinationSolver::HyperellipticIndexCalculus {
        rows.push(CostRow::modeled(
            "relation_collection",
            rel,
            "g! * q Jacobian operations (random divisor splits over the degree-1 factor base with probability 1/g!)",
        ));
        rows.push(CostRow::modeled(
            "linear_algebra",
            la,
            "g^2 * q^(2 - 2/g) Jacobian operations (double large primes, Gaudry-Thome-Theriault-Diem)",
        ));
        rows.push(CostRow::absent(
            "destination_solve_unsplit",
            CostStatus::NotApplicable,
            "phases split above",
        ));
    } else {
        rows.push(CostRow::absent(
            "relation_collection",
            CostStatus::NotApplicable,
            "the cheapest destination solver is not index calculus on the Jacobian",
        ));
        rows.push(CostRow::absent(
            "linear_algebra",
            CostStatus::NotApplicable,
            "see relation_collection",
        ));
        rows.push(CostRow::modeled(
            "destination_solve_unsplit",
            dest_cost,
            format!(
                "{:?}: CostModel::jacobian_destination; phases not split by the model{}",
                dest_solver,
                if rho_discount_bits > 0.0 {
                    format!("; rho on the image discounted by {rho_discount_bits:.3} bits = 0.5 log2(c/2) for its equivalence class c = {} (the map is defined over the subfield that gives E its class, so the class lifts to the image)", rt.image_class)
                } else {
                    String::new()
                }
            ),
        ));
    }
    rows.push(CostRow::modeled(
        "descent_back_mapping",
        verify,
        "the discrete logarithm of the image is the logarithm of the source (homomorphism), so recovery is free; one scalar multiplication on E verifies it: log2(1.5 log2 r) + log2(op)",
    ));
    let mut t = CostTable::with_rows(rows, rt.baseline_log2, rt.reference_no_aut_log2);
    t.destination_solver = Some(format!("{dest_solver:?}"));
    t.solvers_considered = considered
        .iter()
        .map(|(s, c)| (format!("{s:?}"), round3(*c)))
        .collect();
    t
}

/// `log2` of the cost of finding `d` when `rho` is run on the Jacobian
/// image only; used by the genus sweep to show the leading exponents.
pub fn ic_leading_exponent(g: f64) -> f64 {
    2.0 - 2.0 / g
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::certificate::log2_add;

    #[test]
    fn ic_phase_split_sums_to_the_models_total() {
        let m = CostModel::default();
        for (g, q) in [(2.0f64, 192.0), (3.0, 113.0), (4.0, 40.0), (8.0, 20.0)] {
            let lg = g.log2();
            let (rel, la) = ic_phases(&m, lg, q);
            let total = m.log2_hyperelliptic_ic(lg, q).unwrap();
            assert!((log2_add(rel, la) - total).abs() < 1e-9, "g={g} q={q}");
        }
    }

    #[test]
    fn weil_capacity_gate_reproduces_the_repository_threshold() {
        // tests/sect113r1_ghs_evidence.rs proves with exact Pell arithmetic
        // that no genus <= 44 Jacobian over F_2 holds the 112-bit subgroup
        // of sect113r1; genus 45 is the first that can.
        let r = num_bigint::BigUint::from(5192296858534827689835882578830703u128);
        let log2_r = crypto_lib::cryptanalysis::jacobian_transfer::log2_biguint(&r);
        assert!((log2_r - 112.0).abs() < 1e-9);
        assert_eq!(min_genus_to_carry(log2_r, 1.0), 45.0);
        assert!(!jacobian_can_carry(44f64.log2(), 1.0, log2_r));
        assert!(jacobian_can_carry(45f64.log2(), 1.0, log2_r));
    }

    #[test]
    fn lead_classes_follow_the_model_margin() {
        let m = CostModel::default();
        assert_eq!(classify(&m, false, Some(100.0)), LeadClass::NotCarried);
        assert_eq!(classify(&m, true, None), LeadClass::NotPriced);
        assert_eq!(classify(&m, true, Some(8.0)), LeadClass::FormulaAdvantage);
        assert_eq!(classify(&m, true, Some(0.5)), LeadClass::FormulaMarginal);
        assert_eq!(classify(&m, true, Some(-3.0)), LeadClass::Boundary);
    }

    /// Positive control (protocol F10): a synthetic GHS instance of the
    /// kind the attack is known to work on -- genus 4 over F_(2^40), a
    /// source subgroup of 158 bits -- must be flagged by the classifier.
    /// The lead is classified on the production quantity, `log2_cost_ratio`
    /// against the matched baseline (not on `repo_ratio_log2`); the full
    /// path through `judge` is tested in `controls.rs`.
    #[test]
    fn synthetic_ghs_instance_is_a_formula_advantage() {
        let m = CostModel::default();
        let log2_r = 158.0;
        let source_bits = 160.0;
        let baseline = log2_rho_with_class(&m, log2_r, source_bits, 2);
        let rt = Route {
            log2_genus: 2.0,
            field_bits: 40.0,
            source_field_bits: source_bits,
            log2_r,
            construction: ConstructionCost::Priced(0.0, "synthetic".into()),
            transfer_eval_override: None,
            baseline_log2: baseline,
            reference_no_aut_log2: m.log2_rho_elliptic(log2_r, source_bits),
            image_class: 2,
        };
        let t = price_jacobian_route(&m, &rt);
        assert_eq!(
            t.destination_solver.as_deref(),
            Some("HyperellipticIndexCalculus")
        );
        assert!(t.log2_total_frozen_model.is_none());
        let class = classify(&m, true, t.log2_cost_ratio.map(|r| -r));
        assert_eq!(class, LeadClass::FormulaAdvantage, "{t:?}");
    }

    /// The cost model's `sqrt(pi r / 4)` already contains negation, so the
    /// class `c` enters as `sqrt(c / 2)`: no discount for `c = 2`, and the
    /// discount for `c = 2n` is `0.5 log2 n`.
    #[test]
    fn class_discount_counts_negation_once() {
        assert_eq!(class_discount_bits(2), 0.0);
        assert_eq!(class_discount_bits(0), 0.0);
        assert!((class_discount_bits(326) - 0.5 * 163f64.log2()).abs() < 1e-12);
        assert!((class_discount_bits(6) - 0.5 * 3f64.log2()).abs() < 1e-12);
        let m = CostModel::default();
        let base = m.log2_rho_elliptic(162.0, 163.0);
        assert_eq!(log2_rho_with_class(&m, 162.0, 163.0, 2), base);
        assert!(
            (log2_rho_with_class(&m, 162.0, 163.0, 326) - (base - 0.5 * 163f64.log2())).abs()
                < 1e-12
        );
    }

    /// A route whose image has an equivalence class beyond negation prices
    /// rho on the image with it, and keeps the frozen-model total beside it.
    #[test]
    fn image_class_discounts_rho_on_the_jacobian_only() {
        let m = CostModel::default();
        let (log2_r, bits) = (162.0, 163.0);
        let mk = |image_class| Route {
            log2_genus: 3f64.log2(),
            field_bits: bits,
            source_field_bits: bits,
            log2_r,
            construction: ConstructionCost::Priced(0.0, "x".into()),
            transfer_eval_override: None,
            baseline_log2: log2_rho_with_class(&m, log2_r, bits, 326),
            reference_no_aut_log2: m.log2_rho_elliptic(log2_r, bits),
            image_class,
        };
        let plain = price_jacobian_route(&m, &mk(2));
        let classed = price_jacobian_route(&m, &mk(326));
        assert_eq!(plain.destination_solver.as_deref(), Some("RhoOnJacobian"));
        assert_eq!(classed.destination_solver.as_deref(), Some("RhoOnJacobian"));
        assert!(plain.log2_total_frozen_model.is_none());
        // The frozen-model total of the classed route is the plain total.
        assert_eq!(classed.log2_total_frozen_model, plain.log2_total);
        let (a, b) = (classed.log2_total.unwrap(), plain.log2_total.unwrap());
        assert!(a < b && b - a <= class_discount_bits(326) + 1e-9, "{a} {b}");
        // Index calculus is not discounted: at genus 2 over a small field it
        // is the cheapest solver and the two totals coincide.
        let ic = |image_class| Route {
            log2_genus: 2f64.log2(),
            field_bits: 20.0,
            source_field_bits: 20.0,
            log2_r: 60.0,
            construction: ConstructionCost::Priced(0.0, "x".into()),
            transfer_eval_override: None,
            baseline_log2: 0.0,
            reference_no_aut_log2: 0.0,
            image_class,
        };
        let (p, c) = (
            price_jacobian_route(&m, &ic(2)),
            price_jacobian_route(&m, &ic(326)),
        );
        assert_eq!(
            p.destination_solver.as_deref(),
            Some("HyperellipticIndexCalculus")
        );
        assert_eq!(p.log2_total, c.log2_total);
    }

    #[test]
    fn l13_is_monotone_and_attacker_favourable() {
        let a = log2_l13(2.0 * 192.0, LFS_C);
        let b = log2_l13(12.0 * 192.0, LFS_C);
        assert!(b > a);
        // The generic NFS constant is larger.
        assert!(log2_l13(2.0 * 192.0, (64.0f64 / 9.0).cbrt()) > a);
    }
}
