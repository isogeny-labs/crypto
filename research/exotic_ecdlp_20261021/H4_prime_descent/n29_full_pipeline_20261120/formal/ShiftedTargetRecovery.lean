import Mathlib

/-! Algebraic correctness of known-scalar target shifting. -/

namespace H4ShiftedTargetRecovery

variable {R G : Type*} [Ring R] [AddCommGroup G] [Module R G]

/-- A verified log for `Q + t • G₀` yields the log `s - t` for `Q`. -/
theorem recover_from_shift (Q G₀ : G) (s t : R)
    (hshifted : Q + t • G₀ = s • G₀) :
    Q = (s - t) • G₀ := by
  calc
    Q = (Q + t • G₀) - t • G₀ := by simp
    _ = s • G₀ - t • G₀ := by rw [hshifted]
    _ = (s - t) • G₀ := (sub_smul s t G₀).symm

#print axioms H4ShiftedTargetRecovery.recover_from_shift

end H4ShiftedTargetRecovery
