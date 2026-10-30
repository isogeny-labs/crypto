import ProjectiveLawBoundary
import RuntimeSemantics
import Lean.Util.CollectAxioms

open Lean Elab Command

private def audited : Array Name := #[
  `ProjectiveLawBoundary.equal_or_opposite_of_sq_eq_sq,
  `ProjectiveLawBoundary.same_u_zero_slope_delta_is_inverse,
  `ProjectiveLawBoundary.shared_zero_q_delta_unreachable,
  `ProjectiveLawBoundary.generic12_output_v_clearing,
  `ProjectiveLawBoundary.generic22_interpolates_first,
  `ProjectiveLawBoundary.generic22_interpolates_second,
  `ProjectiveLawBoundary.runtime_certificate_mapping_total,
  `ProjectiveLawBoundary.formula_to_divisor_class,
  `ProjectiveLawRuntime.classify12_ne_mismatch_of_valid,
  `ProjectiveLawRuntime.generic12Runtime_cleared,
  `ProjectiveLawRuntime.generic22Runtime_mod_first,
  `ProjectiveLawRuntime.generic22Runtime_mod_second,
  `ProjectiveLawRuntime.residual_represents_input
]

run_cmd do
  for theoremName in audited do
    let axioms ← Lean.collectAxioms theoremName
    let permitted : Array Name := #[`propext, `Classical.choice, `Quot.sound]
    let unexpected := axioms.filter fun ax ↦ !permitted.contains ax
    unless unexpected.isEmpty do
      throwError m!"Forbidden proof dependency in {theoremName}: {unexpected}"
    logInfo m!"AXIOM_AUDIT|{theoremName}|{axioms}"

#print axioms ProjectiveLawBoundary.equal_or_opposite_of_sq_eq_sq
#print axioms ProjectiveLawBoundary.same_u_zero_slope_delta_is_inverse
#print axioms ProjectiveLawBoundary.shared_zero_q_delta_unreachable
#print axioms ProjectiveLawBoundary.generic12_output_v_clearing
#print axioms ProjectiveLawBoundary.generic22_interpolates_first
#print axioms ProjectiveLawBoundary.generic22_interpolates_second
#print axioms ProjectiveLawBoundary.runtime_certificate_mapping_total
#print axioms ProjectiveLawBoundary.formula_to_divisor_class
#print axioms ProjectiveLawRuntime.classify12_ne_mismatch_of_valid
#print axioms ProjectiveLawRuntime.generic12Runtime_cleared
#print axioms ProjectiveLawRuntime.generic22Runtime_mod_first
#print axioms ProjectiveLawRuntime.generic22Runtime_mod_second
#print axioms ProjectiveLawRuntime.residual_represents_input
