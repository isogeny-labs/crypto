import IcNegative
import Lean.Util.CollectAxioms
import Lean.Elab.Command

/-!
Axiom audit for the `IcNegative` modules.  Run `lake exe cache get && lake build
&& lake env lean AxiomAudit.lean` in this directory.  The `run_cmd` block fails
the run if any listed declaration depends on an axiom outside `propext`,
`Classical.choice` and `Quot.sound`, for example `sorryAx`.  The repository-wide
proof gate (`tools/lean/proof-gate.sh`) performs the same audit on every theorem.
-/

open Lean Elab Command

run_cmd do
  let allowed : Array Name := #[`propext, `Classical.choice, `Quot.sound]
  for name in #[`IcNegative.agree_card_le_one,
      `IcNegative.badSecrets_card_le,
      `IcNegative.badSecrets_card_le_sq,
      `IcNegative.injOn_of_not_bad,
      `IcNegative.sum_yield,
      `IcNegative.sum_yield_cast,
      `IcNegative.total_yield_eq,
      `IcNegative.coverage_le_card_dom,
      `IcNegative.coverage_le_card_cod] do
    let axioms ← Lean.collectAxioms name
    let forbidden := axioms.filter fun ax => !allowed.contains ax
    unless forbidden.isEmpty do
      throwError "Forbidden proof dependency in {name}: {forbidden}"
    logInfo m!"AXIOM_AUDIT|{name}|{axioms}"
