import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

namespace XedniWitnessRows

/-- Signed relation row for the first positive source-pair family, with
columns ordered as the 21 negation orbits in the frozen census. -/
def rowA : Fin 21 → ZMod 43 := fun i =>
  if i = 2 then 1 else if i = 19 then 2 else 0

/-- Signed relation row for the second positive source-pair family. -/
def rowB : Fin 21 → ZMod 43 := fun i =>
  if i = 5 then 1 else if i = 8 then 2 else 0

/-- The two observed row patterns are linearly independent over `F₄₃`.
The finite census separately checks that every one of its 40 rows is
`rowA`, `-rowA`, `rowB`, or `-rowB`. -/
theorem independent_coefficients (a b : ZMod 43)
    (h : a • rowA + b • rowB = 0) : a = 0 ∧ b = 0 := by
  have h₂ := congrArg (fun f : Fin 21 → ZMod 43 => f 2) h
  have h₅ := congrArg (fun f : Fin 21 → ZMod 43 => f 5) h
  constructor
  · simpa [rowA, rowB] using h₂
  · simpa [rowA, rowB] using h₅

#print axioms independent_coefficients

end XedniWitnessRows
