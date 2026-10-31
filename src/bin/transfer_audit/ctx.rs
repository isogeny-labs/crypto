//! The per-curve context and the assembly of a certificate.

use serde_json::{Map, Value};

use crypto_lib::cryptanalysis::curve_traits::{self, CurveTraits, Factoring};
use crypto_lib::cryptanalysis::jacobian_transfer::cost::CostModel;

use crate::certificate::{
    candidate_value, obligation_state, Certificate, Check, CostTable, Evaluation, Evidence,
    LeadClass, Obligations, RouteStatus, SubgroupPreservation, TransferType, Verdict, SCHEMA, TOOL,
};
use crate::cost::{classify, ConstructionCost, Route};
use crate::spec::{baseline, rho_class_size, validate, Baseline, Instance};
use crate::verdict::{decide, MapBasis, VerdictInput};

pub struct Ctx<'a> {
    pub inst: &'a Instance,
    pub traits: CurveTraits,
    pub model: &'a CostModel,
    pub fx: &'a Factoring,
    pub base: Baseline,
    pub seed: u64,
    pub instance_checks: Vec<Check>,
}

impl<'a> Ctx<'a> {
    /// Validate the instance (`INVALID_INSTANCE` on failure), compute the
    /// repository's curve traits and the matched baseline.
    pub fn new(
        inst: &'a Instance,
        model: &'a CostModel,
        fx: &'a Factoring,
        seed: u64,
    ) -> Result<Self, String> {
        Self::with_traits(inst, model, fx, seed, None)
    }

    /// As [`Ctx::new`], taking the trait record from the committed
    /// `docs/curves/traits.json` when supplied.  Only for a curve whose live
    /// `curve_traits::compute` is too slow to replay; the record must agree
    /// with the instance on order and trace, and the certificate says so.
    pub fn with_traits(
        inst: &'a Instance,
        model: &'a CostModel,
        fx: &'a Factoring,
        seed: u64,
        committed: Option<CurveTraits>,
    ) -> Result<Self, String> {
        let t0 = std::time::Instant::now();
        let mut instance_checks = validate(inst, seed)?;
        let t1 = t0.elapsed();
        let traits = match committed {
            Some(t) => {
                if t.order != inst.rc.order.to_string() || t.trace != inst.rc.trace.to_string() {
                    return Err(format!(
                        "INVALID_INSTANCE: {}: the committed trait record disagrees with the instance on order or trace",
                        inst.name
                    ));
                }
                instance_checks.push(Check::new(
                    "traits_record",
                    crate::certificate::CheckResult::Pass,
                    "curve_traits record read from the committed docs/curves/traits.json (order and trace equal the instance's); a live curve_traits::compute of this curve takes minutes",
                ));
                t
            }
            None => {
                let t = curve_traits::compute(&inst.rc, fx)
                    .map_err(|e| format!("INVALID_INSTANCE: {}: curve_traits: {e}", inst.name))?;
                instance_checks.push(Check::new(
                    "traits_record",
                    crate::certificate::CheckResult::Pass,
                    "curve_traits::compute run live on this instance",
                ));
                t
            }
        };
        if std::env::var_os("TRANSFER_AUDIT_TRACE").is_some() {
            eprintln!(
                "trace {}: validate {:?}, traits {:?}",
                inst.name,
                t1,
                t0.elapsed() - t1
            );
        }
        let (class, reason) = rho_class_size(inst, &traits);
        let base = baseline(model, inst, class, reason);
        Ok(Ctx {
            inst,
            traits,
            model,
            fx,
            base,
            seed,
            instance_checks,
        })
    }

    /// A route skeleton with this curve's baseline filled in.
    pub fn route(&self, log2_genus: f64, field_bits: f64) -> Route {
        Route {
            log2_genus,
            field_bits,
            source_field_bits: self.inst.field_bits(),
            log2_r: self.inst.log2_r(),
            construction: ConstructionCost::Unpriced(String::new()),
            transfer_eval_override: None,
            baseline_log2: self.base.log2_total,
            reference_no_aut_log2: self.base.log2_no_automorphisms,
            image_class: 2,
        }
    }
}

/// How to read a candidate's price (`Spec::price_basis`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PriceBasis {
    /// The total prices this route.  A total at or above the baseline (a
    /// lower bound, where phases are unpriced) excludes the route.
    Route,
    /// The destination is the source's own cost: the identity path.  The
    /// total is the baseline plus one edge by construction, so it says
    /// nothing about the representatives that were not searched.
    IdentityPath,
    /// A formula sweep or heuristic estimate for a map that is not
    /// constructed for this curve.
    Hypothetical,
    /// A price band whose ends lie on both sides of the baseline: the
    /// formula cannot decide.
    Straddles,
}

/// Everything a candidate module supplies; [`Ctx::finish`] adds the
/// verdict, lead class, evaluation and obligation states.
#[derive(Clone)]
pub struct Spec {
    pub transfer: TransferType,
    pub id: String,
    pub label: String,
    pub map: Value,
    pub subgroup: SubgroupPreservation,
    pub costs: CostTable,
    /// The subgroup is (or may be) carried: not refuted, obstructed or absent.
    pub carried: bool,
    pub map_basis: MapBasis,
    /// How to read `costs` (see [`PriceBasis`]).
    pub price_basis: PriceBasis,
    pub class_invariant: bool,
    pub at_isogenous_representative: bool,
    pub existence: &'static str,
    pub explicit_construction: &'static str,
    pub map_correctness: &'static str,
    pub scaling_support: &'static str,
    /// Override the derived `computational_advantage` state.
    pub advantage: Option<&'static str>,
    /// Override the derived lead class (a price band that straddles the baseline).
    pub lead_override: Option<LeadClass>,
    pub checks: Vec<Check>,
    pub invariants: Map<String, Value>,
    pub executed: Option<Value>,
    pub open_obligations: Vec<String>,
    pub notes: Vec<String>,
    pub references: Vec<String>,
}

impl Spec {
    pub fn new(transfer: TransferType, id: &str, label: &str, costs: CostTable) -> Self {
        Spec {
            transfer,
            id: id.to_string(),
            label: label.to_string(),
            map: Value::Null,
            subgroup: SubgroupPreservation::not_evaluated("none", "not assessed"),
            costs,
            carried: false,
            map_basis: MapBasis::Existence,
            price_basis: PriceBasis::Route,
            class_invariant: false,
            at_isogenous_representative: false,
            existence: "unknown",
            explicit_construction: "unknown",
            map_correctness: "unknown",
            scaling_support: "unknown",
            advantage: None,
            lead_override: None,
            checks: Vec::new(),
            invariants: Map::new(),
            executed: None,
            open_obligations: Vec::new(),
            notes: Vec::new(),
            references: Vec::new(),
        }
    }
}

/// What the protocol's decision rules say about one [`Spec`].
pub struct Judgement {
    pub lead: LeadClass,
    pub verdict: Verdict,
    pub basis: Vec<String>,
    pub evaluation: Evaluation,
    /// `obligations.computational_advantage`.
    pub advantage: &'static str,
}

fn evidence_label(e: Evidence) -> &'static str {
    match e {
        Evidence::Verified => "verified",
        Evidence::Arithmetic => "arithmetic",
        Evidence::NotEvaluated => "not_evaluated",
        Evidence::Refuted => "refuted",
        Evidence::NotApplicable => "not_applicable",
    }
}

/// The decision rules, as a pure function of the cost model and a
/// candidate: lead class, verdict (the closed vocabulary), whether the
/// route was evaluated, excluded or is not evaluable, and the
/// `computational_advantage` state.  `Ctx::finish` and the F10 control both
/// call it, so a control exercises the rules production runs.
///
/// `computational_advantage` (Amendment 3):
/// * `supported`: a weakness verdict;
/// * `refuted`: the route is carried, priced as a route and its priced total
///   (a lower bound where phases are unpriced; they only add cost) is not
///   below the matched baseline;
/// * `not_applicable`: the route does not carry `<G>` or does not exist for
///   this curve, so there is no advantage claim to refute;
/// * `unknown`: everything else (identity-path price, formula sweep or
///   heuristic, unconstructed map with a sub-baseline lower bound, a price
///   inside the model margin, a band that straddles the baseline).
pub fn judge(model: &CostModel, s: &Spec) -> Judgement {
    let lead = if s.costs.log2_total.is_none() && s.carried {
        LeadClass::NotPriced
    } else {
        classify(model, s.carried, s.costs.log2_cost_ratio.map(|r| -r))
    };
    let lead = s.lead_override.unwrap_or(lead);
    let margin = model.advantage_margin_bits;
    let (verdict, mut basis) = decide(&VerdictInput {
        transfer: s.transfer,
        subgroup: s.subgroup.status,
        antecedents_ok: s
            .subgroup
            .antecedents
            .iter()
            .all(|c| c.result == crate::certificate::CheckResult::Pass),
        map_basis: s.map_basis,
        // An incomplete price is a lower bound on the route: it can
        // show the route is too expensive, never that it is cheap.
        log2_cost_ratio: match s.costs.log2_cost_ratio {
            Some(r) if s.costs.total_is_lower_bound && r < 0.0 => None,
            other => other,
        },
        margin_bits: margin,
        class_invariant: s.class_invariant,
        at_isogenous_representative: s.at_isogenous_representative,
    });
    if verdict == Verdict::NoWeaknessFoundInScope
        && matches!(
            lead,
            LeadClass::FormulaAdvantage | LeadClass::FormulaMarginal
        )
    {
        basis.push(format!(
            "formula-level lead ({lead:?}) below the baseline, recorded as a lead only: a lead is not a weakness"
        ));
    }
    let weak = verdict != Verdict::NoWeaknessFoundInScope;
    let ratio = s.costs.log2_cost_ratio;
    let lower_bound = if s.costs.total_is_lower_bound {
        " (a lower bound: the unpriced phases only add cost)"
    } else {
        ""
    };
    let subgroup_lost = matches!(
        s.subgroup.status,
        Evidence::Refuted | Evidence::NotApplicable
    );
    let evaluation = if weak {
        Evaluation {
            status: RouteStatus::Evaluated,
            reason: format!(
                "map {:?}, subgroup {}, priced {:.1} bits below the matched baseline, past the {margin:.0}-bit margin",
                s.map_basis,
                evidence_label(s.subgroup.status),
                -ratio.unwrap_or(0.0)
            ),
        }
    } else if subgroup_lost {
        Evaluation {
            status: RouteStatus::Excluded,
            reason: format!(
                "the subgroup is not carried (subgroup preservation {}): the route solves a different problem or does not exist, so there is no attack cost to compare with the baseline",
                evidence_label(s.subgroup.status)
            ),
        }
    } else if s.carried && s.price_basis == PriceBasis::Route && ratio.is_some_and(|r| r >= 0.0) {
        Evaluation {
            status: RouteStatus::Excluded,
            reason: format!(
                "the priced total is {:.1} bits above the matched baseline{lower_bound}",
                ratio.unwrap_or(0.0)
            ),
        }
    } else {
        match s.price_basis {
            PriceBasis::IdentityPath => Evaluation {
                status: RouteStatus::NotEvaluable,
                reason: "the price is the identity path (the matched baseline plus one edge, by construction): it says nothing about the representatives that were not searched, and the search for a cheaper one is not priced".into(),
            },
            PriceBasis::Hypothetical => Evaluation {
                status: RouteStatus::NotEvaluable,
                reason: "the price is a formula sweep or heuristic estimate for a map that is not constructed for this curve".into(),
            },
            PriceBasis::Straddles => Evaluation {
                status: RouteStatus::Evaluated,
                reason: "the price band has ends on both sides of the matched baseline: the formula cannot decide (a marginal lead)".into(),
            },
            PriceBasis::Route => {
                if s.map_basis == MapBasis::Existence
                    || s.costs.total_is_lower_bound
                    || ratio.is_none()
                {
                    Evaluation {
                        status: RouteStatus::NotEvaluable,
                        reason: format!(
                            "the map is not constructed or a required phase is unpriced, and the priced part ({}) does not exclude the route",
                            match ratio {
                                Some(r) => format!("{r:+.1} bits against the matched baseline{lower_bound}"),
                                None => "nothing is priced".to_string(),
                            }
                        ),
                    }
                } else {
                    Evaluation {
                        status: RouteStatus::Evaluated,
                        reason: format!(
                            "priced {:+.1} bits against the matched baseline, inside the {margin:.0}-bit margin",
                            ratio.unwrap_or(0.0)
                        ),
                    }
                }
            }
        }
    };
    let advantage = if let Some(o) = s.advantage {
        o
    } else if weak {
        "supported"
    } else {
        match (evaluation.status, subgroup_lost) {
            (RouteStatus::Excluded, true) => "not_applicable",
            (RouteStatus::Excluded, false) => "refuted",
            _ => "unknown",
        }
    };
    Judgement {
        lead,
        verdict,
        basis,
        evaluation,
        advantage,
    }
}

impl Ctx<'_> {
    pub fn finish(&self, s: Spec) -> Certificate {
        let j = judge(self.model, &s);
        let obligations = Obligations {
            existence: s.existence,
            explicit_construction: s.explicit_construction,
            map_correctness: s.map_correctness,
            subgroup_preservation: obligation_state(s.subgroup.status),
            computational_advantage: j.advantage,
            scaling_support: s.scaling_support,
        };
        Certificate {
            schema: SCHEMA,
            tool: TOOL,
            curve: self.inst.curve_value(&self.instance_checks),
            candidate: candidate_value(&s.id, s.transfer, &s.label),
            map: s.map,
            obligations,
            subgroup_preservation: s.subgroup,
            baseline: self.base.value(self.inst.log2_r()),
            carried: s.carried,
            costs: s.costs,
            lead_class: j.lead,
            verdict: j.verdict,
            verdict_basis: j.basis,
            evaluation: j.evaluation,
            checks: s.checks,
            invariants: s.invariants,
            executed: s.executed,
            open_obligations: s.open_obligations,
            notes: s.notes,
            references: s.references,
        }
    }
}

pub fn jmap(pairs: Vec<(&str, Value)>) -> Map<String, Value> {
    pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::certificate::{CostRow, CostStatus};

    /// A cost table whose one priced row puts the total `ratio` bits away
    /// from a baseline of 100.
    fn costs(ratio: f64, lower_bound: bool) -> CostTable {
        let mut rows = vec![CostRow::modeled(
            "destination_solve_unsplit",
            100.0 + ratio,
            "x",
        )];
        if lower_bound {
            rows.push(CostRow::absent(
                "map_construction",
                CostStatus::NotEvaluated,
                "unpriced",
            ));
        }
        CostTable::with_rows(rows, 100.0, 100.0)
    }

    fn spec(ratio: f64, lower_bound: bool) -> Spec {
        let mut s = Spec::new(
            TransferType::GenusCover,
            "t",
            "t",
            costs(ratio, lower_bound),
        );
        s.carried = true;
        s.subgroup = SubgroupPreservation {
            status: Evidence::Verified,
            ..SubgroupPreservation::not_evaluated("m", "a")
        };
        s.map_basis = MapBasis::Executed;
        s
    }

    fn j(s: &Spec) -> Judgement {
        judge(&CostModel::default(), s)
    }

    #[test]
    fn refuted_only_for_a_carried_priced_route_at_or_above_the_baseline() {
        let r = j(&spec(5.6, false));
        assert_eq!(r.evaluation.status, RouteStatus::Excluded);
        assert_eq!(r.advantage, "refuted");
        // A lower bound above the baseline refutes too: missing phases add cost.
        let mut s = spec(3.0, true);
        s.map_basis = MapBasis::Existence;
        let r = j(&s);
        assert_eq!(r.evaluation.status, RouteStatus::Excluded);
        assert_eq!(r.advantage, "refuted");
        assert!(
            r.evaluation.reason.contains("lower bound"),
            "{}",
            r.evaluation.reason
        );
    }

    #[test]
    fn identity_path_and_hypothetical_prices_never_refute() {
        for basis in [PriceBasis::IdentityPath, PriceBasis::Hypothetical] {
            let mut s = spec(0.3, false);
            s.price_basis = basis;
            let r = j(&s);
            assert_eq!(r.evaluation.status, RouteStatus::NotEvaluable, "{basis:?}");
            assert_eq!(r.advantage, "unknown", "{basis:?}");
            assert_eq!(r.verdict, Verdict::NoWeaknessFoundInScope);
        }
        // Not carried and nothing known about the subgroup: unknown, not refuted.
        let mut s = spec(30.0, false);
        s.carried = false;
        s.price_basis = PriceBasis::Hypothetical;
        s.subgroup = SubgroupPreservation::not_evaluated("none", "x");
        let r = j(&s);
        assert_eq!(r.evaluation.status, RouteStatus::NotEvaluable);
        assert_eq!(r.advantage, "unknown");
    }

    #[test]
    fn a_lost_or_absent_subgroup_is_excluded_and_not_applicable() {
        for st in [Evidence::Refuted, Evidence::NotApplicable] {
            let mut s = spec(-30.0, false);
            s.carried = false;
            s.subgroup.status = st;
            let r = j(&s);
            assert_eq!(r.evaluation.status, RouteStatus::Excluded, "{st:?}");
            assert_eq!(r.advantage, "not_applicable", "{st:?}");
            assert_eq!(r.verdict, Verdict::NoWeaknessFoundInScope);
        }
    }

    /// The composite-degree GHS case: the map is not constructed and the
    /// priced part is below the baseline, so the "no weakness" verdict is
    /// vacuous and the certificate says so.
    #[test]
    fn an_unconstructed_route_below_the_baseline_is_not_evaluable() {
        let mut s = spec(-12.0, true);
        s.map_basis = MapBasis::Existence;
        s.subgroup = SubgroupPreservation::not_evaluated("none", "x");
        let r = j(&s);
        assert_eq!(r.verdict, Verdict::NoWeaknessFoundInScope);
        assert_eq!(r.evaluation.status, RouteStatus::NotEvaluable);
        assert_eq!(r.advantage, "unknown");
        assert_eq!(r.lead, LeadClass::FormulaAdvantage);
    }

    #[test]
    fn a_complete_price_inside_the_margin_is_evaluated_and_unknown() {
        let mut s = spec(-3.0, false);
        s.map_basis = MapBasis::Theorem;
        s.subgroup.status = Evidence::Arithmetic;
        s.subgroup.antecedents = vec![Check::new("a", crate::certificate::CheckResult::Pass, "x")];
        let r = j(&s);
        assert_eq!(r.verdict, Verdict::NoWeaknessFoundInScope);
        assert_eq!(r.lead, LeadClass::FormulaMarginal);
        assert_eq!(r.evaluation.status, RouteStatus::Evaluated);
        assert_eq!(r.advantage, "unknown");
        // Past the margin it is a weakness.
        let mut s = spec(-9.0, false);
        s.map_basis = MapBasis::Theorem;
        s.class_invariant = true;
        let r = j(&s);
        assert_eq!(r.verdict, Verdict::ClassWeak);
        assert_eq!(r.evaluation.status, RouteStatus::Evaluated);
        assert_eq!(r.advantage, "supported");
    }

    #[test]
    fn a_straddling_band_is_evaluated_and_not_refuted() {
        let mut s = spec(3.7, false);
        s.price_basis = PriceBasis::Straddles;
        s.lead_override = Some(LeadClass::FormulaMarginal);
        let r = j(&s);
        assert_eq!(r.evaluation.status, RouteStatus::Evaluated);
        assert_eq!(r.advantage, "unknown");
    }
}
