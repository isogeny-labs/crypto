import Init

/-! Finite arithmetic attached to the measured W1 genus-fourteen L-polynomial.
The zeta computation, isogeny decomposition, and geometric maps are external
inputs, not claims formalized by this file. -/
namespace PrymFactorArithmetic

theorem middle_coefficient : 2 * 128 - 9^2 = 175 := by decide
theorem top_coefficient : 128^2 = 16384 := by decide
theorem class_number_factors : 16560 = 138 * 120 := by decide

def pMinus (u : Fin 23) : Nat := (u.val^2 + 9*u.val + 128) % 23
def pPlus (u : Fin 23) : Nat := (u.val^2 + 14*u.val + 128) % 23
def eMinus (u : Fin 23) : Nat := (6*u.val^3 + 6*u.val + 12) % 23

theorem minus_roots :
    ∀ u : Fin 23, pMinus u = 0 ↔ u.val = 1 ∨ u.val = 13 := by decide
theorem plus_roots :
    ∀ u : Fin 23, pPlus u = 0 ↔ u.val = 10 ∨ u.val = 22 := by decide
theorem projector_on_roots :
    ∀ u : Fin 23,
      (pMinus u = 0 → eMinus u = 1) ∧
      (pPlus u = 0 → eMinus u = 0) := by decide

end PrymFactorArithmetic
