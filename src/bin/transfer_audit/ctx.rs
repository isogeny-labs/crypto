//! The per-curve context and the assembly of a certificate.

use serde_json::{Map, Value};

use crypto_lib::cryptanalysis::curve_traits::{self, CurveTraits, Factoring};
use crypto_lib::cryptanalysis::jacobian_transfer::cost::CostModel;

use crate::certificate::{
    candidate_value, obligation_state, Certificate, Check, CostTable, LeadClass, Obligations,
    SubgroupPreservation, TransferType, Verdict, SCHEMA, TOOL,
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
        }
    }
}

/// Everything a candidate module supplies; [`Ctx::finish`] adds the
/// verdict, lead class and obligation states.
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

impl Ctx<'_> {
    pub fn finish(&self, s: Spec) -> Certificate {
        let lead = if s.costs.log2_total.is_none() && s.carried {
            LeadClass::NotPriced
        } else {
            classify(self.model, s.carried, s.costs.log2_cost_ratio.map(|r| -r))
        };
        let lead = s.lead_override.unwrap_or(lead);
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
        let advantage = match (verdict, s.costs.log2_cost_ratio) {
            _ if s.advantage.is_some() => s.advantage.unwrap_or("unknown"),
            (Verdict::NoWeaknessFoundInScope, Some(r)) if r >= 0.0 && s.carried => "refuted",
            (Verdict::NoWeaknessFoundInScope, _) if !s.carried => "refuted",
            (Verdict::NoWeaknessFoundInScope, _) => "unknown",
            _ => "supported",
        };
        let obligations = Obligations {
            existence: s.existence,
            explicit_construction: s.explicit_construction,
            map_correctness: s.map_correctness,
            subgroup_preservation: obligation_state(s.subgroup.status),
            computational_advantage: advantage,
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
            lead_class: lead,
            verdict,
            verdict_basis: basis,
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
