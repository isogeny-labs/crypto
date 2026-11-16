module
public import Mathlib.NumberTheory.Padics.PadicVal.Basic
public import Mathlib.Tactic.FieldSimp
public import Mathlib.Tactic.Ring
public section
namespace NormalizedSquare
instance : Fact (Nat.Prime 2) := ⟨Nat.prime_two⟩

/-- Actual normalized-square node identity, with explicit node alternatives. -/
theorem node (w r : ℚ) (hr : r ≠ 0) (hw : w = 0 ∨ w = r) :
    w*w/r = w := by
  rcases hw with h | h
  · simp [h]
  · rw [h]
    field_simp

/-- Exact valuation of the actual candidate, conditional on a unit normalizer.
The Cauchy-specific normalizer's unit property is not assumed to be proved. -/
theorem valuation (w r : ℚ) (hw : w ≠ 0) (hr : r ≠ 0)
    (hu : padicValRat 2 r = 0) :
    padicValRat 2 (w*w/r) = 2 * padicValRat 2 w := by
  rw [padicValRat.div (mul_ne_zero hw hw) hr, padicValRat.mul hw hw, hu]
  ring

end NormalizedSquare
#print axioms NormalizedSquare.node
#print axioms NormalizedSquare.valuation
