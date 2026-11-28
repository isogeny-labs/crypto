import Mathlib.Algebra.CharP.Lemmas
import Mathlib.Tactic.NormNum

/-! Algebraic identities used by the closed-point coefficient lift. This file
checks identities under explicit residue-field and nilpotence hypotheses.
It does not import native arrays or formalize normalization/Riemann--Roch. -/
namespace ClosedPointAlgebra35
variable {R : Type*} [CommRing R] [CharP R 2]

theorem frobenius_1024_of_32 (c : R) (hc : c ^ 32 = c) : c ^ 1024 = c := by
  calc
    c ^ 1024 = (c ^ 32) ^ 32 := by rw [← pow_mul]
    _ = c := by rw [hc, hc]

theorem coefficient_lift (c z : R) (hc : c ^ 32 = c) (hz : z ^ 1024 = 0) :
    (c + z) ^ 1024 = c := by
  have h := add_pow_char_pow c z 2 10
  norm_num only at h
  rw [h, hz, add_zero]
  exact frobenius_1024_of_32 c hc

theorem nilpotent_precision (z : R) (hz : z ^ 154 = 0) : z ^ 1024 = 0 := by
  have h : z ^ (154 + 870) = z ^ 154 * z ^ 870 := pow_add z 154 870
  simpa only [hz, zero_mul] using h

theorem coefficient_lift_precision154 (c z : R) (hc : c ^ 32 = c)
    (hz : z ^ 154 = 0) : (c + z) ^ 1024 = c :=
  coefficient_lift c z hc (nilpotent_precision z hz)

end ClosedPointAlgebra35
