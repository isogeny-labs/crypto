import CMRecovery42
import Mathlib.Algebra.BigOperators.Ring.Finset

open scoped BigOperators

namespace CMLeastScalar42

variable {m : ℕ}

/-- Primitive coordinates force every proportional integral solution's scalar
to be divisible by the primitive denominator. Matrix nonsingularity must
separately supply `proportional`; it is not assumed to follow from a checker. -/
theorem scalar_divisibility (d h w0 : ℤ) (a w z : Fin m → ℤ)
    (primitive : w0*d + ∑ i, w i*a i = 1)
    (proportional : ∀ i, d*z i = h*a i) : d ∣ h := by
  have h0 : d ∣ h*(w0*d) := ⟨h*w0, by ring⟩
  have hi : ∀ i, d ∣ h*(w i*a i) := by
    intro i
    refine ⟨w i*z i, ?_⟩
    calc
      h*(w i*a i) = w i*(h*a i) := by ring
      _ = w i*(d*z i) := by rw [proportional i]
      _ = d*(w i*z i) := by ring
  have hs : d ∣ ∑ i, h*(w i*a i) := Finset.dvd_sum (fun i _ => hi i)
  have hh := dvd_add h0 hs
  rw [← Finset.mul_sum, ← mul_add, primitive, mul_one] at hh
  exact hh

end CMLeastScalar42
