//! The verdict rule of the protocol (section 7), in one function.
//!
//! A weakness verdict needs all three of: a subgroup-preservation status
//! that is `verified` (executed on the generator) or `arithmetic` with every
//! antecedent checked; a complete price (no unpriced required phase) below
//! the matched baseline; and a map that is executed or given by a cited
//! theorem whose antecedents were checked (not mere existence).  A
//! formula-level lead below the baseline whose map is not constructed stays
//! `no weakness found in scope`, with the lead class recorded beside it.

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
    let below = i.log2_cost_ratio.is_some_and(|r| r < 0.0);
    match i.log2_cost_ratio {
        None => why.push("a required cost phase is unpriced".into()),
        Some(r) if r >= 0.0 => why.push(format!(
            "priced route costs 2^{r:.1} times the matched baseline or more"
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
        return (
            v,
            vec![format!(
                "{}: subgroup carried, map {:?}, total below the matched baseline",
                i.transfer.label(),
                i.map_basis
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
            log2_cost_ratio: Some(-5.0),
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
}
