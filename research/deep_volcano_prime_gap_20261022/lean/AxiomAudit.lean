import DeepGap
import Lean.Util.CollectAxioms
import Lean.Elab.Command

/-!
Axiom audit for `DeepGap.lean`. Run `lake build && lake env lean AxiomAudit.lean`
in this directory. The `run_cmd` block fails if any listed declaration depends
on an axiom outside `propext`, `Classical.choice` and `Quot.sound` (for
example `sorryAx`). The repository proof gate (`tools/lean/proof-gate.sh`)
checks the same thing for every theorem of the project.
-/

open Lean Elab Command

run_cmd do
  let allowed : Array Name := #[`propext, `Classical.choice, `Quot.sound]
  for name in #[`DeepGap.Hom.comp, `DeepGap.Isogeny.comp, `DeepGap.comp_cost,
      `DeepGap.transfer_comp_cost, `DeepGap.transfer_comp_answer, `DeepGap.chain_equivalence,
      `DeepGap.twist_order_identity, `DeepGap.twist_order_sq_dvd, `DeepGap.order_mod,
      `DeepGap.class_discriminant, `DeepGap.gap_prime_divides_t_plus_2,
      `DeepGap.class_twist_order, `DeepGap.gap_prime_mod_11,
      `DeepGap.volcanoSize_closed, `DeepGap.depth6_size] do
    let axioms ← Lean.collectAxioms name
    let forbidden := axioms.filter fun ax => !allowed.contains ax
    unless forbidden.isEmpty do
      throwError "Forbidden proof dependency in {name}: {forbidden}"
    logInfo m!"AXIOM_AUDIT|{name}|{axioms}"
