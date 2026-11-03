import Band
import KeyedInvariance
import Certificates
import Lean.Util.CollectAxioms
import Lean.Elab.Command

/-!
Axiom audit for `Band.lean`, `KeyedInvariance.lean` and the generated
`Certificates.lean`. Run `lake build && lake env lean AxiomAudit.lean` in this
directory. Every theorem in the three namespaces is collected from the
environment (no hand-kept list, so a new certificate cannot be skipped), and
the run fails if any depends on an axiom outside `propext`,
`Classical.choice` and `Quot.sound` (for example `sorryAx`, or
`Lean.ofReduceBool` from compiled evaluation). The last line,
`AXIOM_AUDIT_COUNT|<namespace>|<n>`, is what CI compares with
`certificates.json`.
-/

open Lean Elab Command

run_cmd do
  let allowed : Array Name := #[`propext, `Classical.choice, `Quot.sound]
  let env ← getEnv
  for ns in #[`Band, `KeyedInvariance, `Certificates] do
    let mut names : Array Name := #[]
    for (n, info) in env.constants.map₁.toList do
      if ns.isPrefixOf n && !n.isInternal then
        if let .thmInfo _ := info then names := names.push n
    names := names.qsort Name.lt
    for name in names do
      let axioms ← Lean.collectAxioms name
      let forbidden := axioms.filter fun ax => !allowed.contains ax
      unless forbidden.isEmpty do
        throwError "Forbidden proof dependency in {name}: {forbidden}"
      if ns != `Certificates then
        logInfo m!"AXIOM_AUDIT|{name}|{axioms}"
    logInfo m!"AXIOM_AUDIT_COUNT|{ns}|{names.size}"
