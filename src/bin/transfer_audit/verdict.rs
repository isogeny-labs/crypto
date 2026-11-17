//! The verdict rule of the protocol (section 7), in one function.
//!
//! A weakness verdict needs all of: a subgroup-preservation status that is
//! `verified` (executed on the generator) or `arithmetic` with every
//! antecedent checked; a complete price (no unpriced required phase) below
//! the matched baseline *by at least the model margin*; and a map that is
//! executed or given by a cited theorem whose antecedents were checked (not
//! mere existence).  A formula-level lead below the baseline whose map is not
//! constructed, or whose price is below the baseline by less than the margin,
//! stays `no weakness found in scope`, with the lead class recorded beside it.
//!
//! The margin is the frozen model's own `advantage_margin_bits` (8 bits): the
//! bits a route must save over the reference to be classed a formula
//! advantage.  The prices are formulas with `o(1)` terms dropped, "can move it
//! by many bits" (`cost.rs`), so a sub-margin sign is not evidence.  Without
//! it the sign of a three-bit difference decided a weakness verdict
//! (Amendment 3).

use crate::certificate::{Evidence, TransferType, Verdict};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapBasis {
    /// The map was evaluated on the actual generator.
    Executed,
    /// The map is given by a cited theorem (for example the Tate pairing)
    /// whose antecedents the tool checked.
    Theorem,
    /// Only existence is known, or the construction is missing.
    Existence,
}

pub struct VerdictInput {
    pub transfer: TransferType,
    pub subgroup: Evidence,
    /// Every antecedent of an arithmetic subgroup argument passed.
    pub antecedents_ok: bool,
    pub map_basis: MapBasis,
    /// `log2(route / baseline)`; `None` when a required phase is unpriced.
    pub log2_cost_ratio: Option<f64>,
    /// Bits the route must undercut the baseline by before its price is
    /// evidence of anything (`CostModel::advantage_margin_bits`).
    pub margin_bits: f64,
    /// The advantage would hold for every curve of the isogeny class
    /// (embedding degree, order structure).
    pub class_invariant: bool,
    /// The weakness sits at an isogenous representative, not at the source.
    pub at_isogenous_representative: bool,
}

pub fn decide(i: &VerdictInput) -> (Verdict, Vec<String>) {
    let mut why = Vec::new();
    let preserved = match i.subgroup {
        Evidence::Verified => true,
        Evidence::Arithmetic => i.antecedents_ok,
        _ => false,
    };
    if !preserved {
        why.push(format!(
            "subgroup preservation is {:?}{}: the transfer is not shown to carry <G>",
            i.subgroup,
            if i.subgroup == Evidence::Arithmetic && !i.antecedents_ok {
                " with an unchecked antecedent"
            } else {
                ""
            }
        ));
    }
    let map_ok = matches!(i.map_basis, MapBasis::Executed | MapBasis::Theorem);
    if !map_ok {
        why.push("the map is not constructed (existence or construction missing)".into());
    }
    let margin = i.margin_bits.max(0.0);
    let below = i.log2_cost_ratio.is_some_and(|r| r < 0.0 && r <= -margin);
    match i.log2_cost_ratio {
        None => why.push("a required cost phase is unpriced".into()),
        Some(r) if r >= 0.0 => why.push(format!(
            "priced route costs 2^{r:.1} times the matched baseline or more"
        )),
        Some(r) if !below => why.push(format!(
            "priced route is {:.1} bits below the matched baseline, inside the {margin:.0}-bit margin of the frozen model: a formula-marginal lead, not a weakness",
            -r
        )),
        Some(_) => {}
    }
    if preserved && map_ok && below {
        let v = if i.class_invariant {
            Verdict::ClassWeak
        } else if i.at_isogenous_representative {
            Verdict::WeakRepresentativeExists
        } else {
            Verdict::SourceTransferWeak
        };
        let r = i.log2_cost_ratio.unwrap_or(0.0);
        return (
            v,
            vec![format!(
                "{}: subgroup carried, map {:?}, priced total {:.1} bits below the matched baseline (margin {margin:.0} bits); a statement relative to rho on <G> under the frozen formula model, not a claim about absolute security",
                i.transfer.label(),
                i.map_basis,
                -r
            )],
        );
    }
    (Verdict::NoWeaknessFoundInScope, why)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> VerdictInput {
        VerdictInput {
            transfer: TransferType::GhsWeilDescent,
            subgroup: Evidence::Verified,
            antecedents_ok: true,
            map_basis: MapBasis::Executed,
            log2_cost_ratio: Some(-20.0),
            margin_bits: 8.0,
            class_invariant: false,
            at_isogenous_representative: false,
        }
    }

    #[test]
    fn a_complete_cheaper_transfer_is_a_weakness_of_the_right_kind() {
        assert_eq!(decide(&input()).0, Verdict::SourceTransferWeak);
        let mut i = input();
        i.class_invariant = true;
        i.transfer = TransferType::PairingTransfer;
        i.subgroup = Evidence::Arithmetic;
        i.map_basis = MapBasis::Theorem;
        assert_eq!(decide(&i).0, Verdict::ClassWeak);
        let mut i = input();
        i.at_isogenous_representative = true;
        assert_eq!(decide(&i).0, Verdict::WeakRepresentativeExists);
    }

    #[test]
    fn each_missing_ingredient_downgrades_to_no_weakness() {
        let mut i = input();
        i.subgroup = Evidence::NotEvaluated;
        assert_eq!(decide(&i).0, Verdict::NoWeaknessFoundInScope);
        let mut i = input();
        i.subgroup = Evidence::Arithmetic;
        i.antecedents_ok = false;
        assert_eq!(decide(&i).0, Verdict::NoWeaknessFoundInScope);
        let mut i = input();
        i.map_basis = MapBasis::Existence;
        assert_eq!(decide(&i).0, Verdict::NoWeaknessFoundInScope);
        let mut i = input();
        i.log2_cost_ratio = None;
        assert_eq!(decide(&i).0, Verdict::NoWeaknessFoundInScope);
        let mut i = input();
        i.log2_cost_ratio = Some(0.0);
        assert_eq!(decide(&i).0, Verdict::NoWeaknessFoundInScope);
        let mut i = input();
        i.subgroup = Evidence::Refuted;
        assert_eq!(decide(&i).0, Verdict::NoWeaknessFoundInScope);
    }

    /// A price below the baseline by less than the margin is a lead, not a
    /// weakness: the boundary is exactly the margin, and the explanation
    /// names it.  (fp318 read `class-weak` on a -2.987-bit margin before.)
    #[test]
    fn a_price_inside_the_margin_is_not_a_weakness() {
        for ratio in [-0.001, -2.987, -7.999] {
            let mut i = input();
            i.log2_cost_ratio = Some(ratio);
            let (v, why) = decide(&i);
            assert_eq!(v, Verdict::NoWeaknessFoundInScope, "ratio {ratio}");
            assert!(
                why.iter().any(|w| w.contains("inside the 8-bit margin")),
                "{why:?}"
            );
        }
        for ratio in [-8.0, -8.001, -112.0] {
            let mut i = input();
            i.log2_cost_ratio = Some(ratio);
            assert_eq!(decide(&i).0, Verdict::SourceTransferWeak, "ratio {ratio}");
        }
        // The margin is a parameter of the model, not a constant of this rule.
        let mut i = input();
        i.log2_cost_ratio = Some(-3.0);
        i.margin_bits = 2.0;
        assert_eq!(decide(&i).0, Verdict::SourceTransferWeak);
        // A zero margin still needs a strictly negative ratio.
        let mut i = input();
        i.log2_cost_ratio = Some(0.0);
        i.margin_bits = 0.0;
        assert_eq!(decide(&i).0, Verdict::NoWeaknessFoundInScope);
    }

    /// The weakness explanation says what the verdict is relative to.
    #[test]
    fn a_weakness_explanation_is_relative_to_rho() {
        let (v, why) = decide(&input());
        assert_eq!(v, Verdict::SourceTransferWeak);
        assert!(why[0].contains("relative to rho on <G>"), "{why:?}");
        assert!(
            why[0].contains("not a claim about absolute security"),
            "{why:?}"
        );
    }
}
