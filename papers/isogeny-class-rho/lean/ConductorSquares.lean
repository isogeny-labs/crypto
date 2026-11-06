import Init.Grind.Ring.Field

namespace ConductorSquares

/-- The algebraic square-factor cancellation used in the conductor argument.
It does not prove the number-theoretic characterization of the quadratic
character of 2 or instantiate a residue field. -/
theorem cancelSquare {F : Type} [Lean.Grind.Field F]
    (a u v : F) (hv : v ≠ 0) (h : u ^ 2 = a * v ^ 2) :
    (u / v) ^ 2 = a := by
  have hc := Lean.Grind.Field.mul_inv_cancel hv
  simp only [Lean.Grind.Field.div_eq_mul_inv]
  grind

/-- An odd power of a nonzero field element being square makes that element
square. The exponent is written 2k+1 as k+k+1. The conductor/discriminant
bridge and the ±1 mod8 quadratic-reciprocity step remain separate. -/
theorem oddPowerSquare {F : Type} [Lean.Grind.Field F]
    (a u : F) (ha : a ≠ 0) (k : Nat) (h : u ^ 2 = a ^ (k + k + 1)) :
    ∃ w : F, w ^ 2 = a := by
  have hv : a ^ k ≠ 0 := by
    intro hz
    exact ha (Lean.Grind.Field.of_pow_eq_zero a k hz)
  refine ⟨u / a ^ k, cancelSquare a u (a ^ k) hv ?_⟩
  rw [Lean.Grind.Semiring.pow_add, Lean.Grind.Semiring.pow_add] at h
  grind

#print axioms ConductorSquares.cancelSquare
#print axioms ConductorSquares.oddPowerSquare

end ConductorSquares
