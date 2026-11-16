import Lean

open Lean

/-- Export elaborated proof dependencies; these are syntactic edges, not a claim of minimality. -/
def main : IO Unit := do
  initSearchPath (← findSysroot)
  let env ← importModules #[{ module := `FiniteFieldHandbook }] {} (trustLevel := 0)
  let entries := env.constants.fold (init := (#[] : Array (Name × Expr × Expr))) fun acc name ci =>
    match ci with
    | .thmInfo info =>
      if (`FiniteFieldHandbook).isPrefixOf name && !name.isInternal then
        acc.push (name, info.type, info.value)
      else acc
    | _ => acc
  let entries := entries.qsort (fun a b => a.1.toString < b.1.toString)
  let json := entries.map fun (name, type, value) => Json.mkObj [
    ("declaration", toJson name.toString),
    ("statement_constants", toJson (type.getUsedConstants.map Name.toString |>.qsort (· < ·))),
    ("proof_constants", toJson (value.getUsedConstants.map Name.toString |>.qsort (· < ·)))]
  IO.println (Json.pretty (Json.mkObj [
    ("schema", toJson "finite-field-kb.dependencies/v1"),
    ("scope", toJson "Direct constants in elaborated theorem types and proof expressions; not minimal dependencies or an independent semantic review."),
    ("declarations", Json.arr json)]))
