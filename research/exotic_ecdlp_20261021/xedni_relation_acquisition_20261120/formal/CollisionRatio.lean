import Mathlib.Tactic

namespace XedniRelationAcquisition

/-- A verified nonzero coefficient difference `da*G + db*R = 0` in a
prime-order scalar group determines the reduced discrete logarithm of `R`
to base `G` when `db` is invertible. The curve-to-scalar-coordinate bridge
is checked separately for the measured reductions. -/
theorem collision_determines_ratio (q : ℕ) [Fact q.Prime]
    (da db G R : ZMod q) (hdb : db ≠ 0)
    (hrel : da * G + db * R = 0) :
    R = (-da / db) * G := by
  have h : db * R = -da * G := by
    linear_combination hrel
  calc
    R = db⁻¹ * (db * R) := by field_simp
    _ = db⁻¹ * (-da * G) := by rw [h]
    _ = (-da / db) * G := by rw [div_eq_mul_inv]; ring

#print axioms collision_determines_ratio

end XedniRelationAcquisition
