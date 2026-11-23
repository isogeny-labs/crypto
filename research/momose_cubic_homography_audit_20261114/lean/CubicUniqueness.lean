module
public import CubicExistence

@[expose] public section
namespace MomoseCubicHomographyAudit
noncomputable section
open Matrix
variable (K L : Type*) [Field K] [Field L] [Algebra K L]

theorem numerator_mul (A B : Mat2 K) (z : L) :
    numerator K L (A * B) z = algebraMap K L (A 0 0) * numerator K L B z +
      algebraMap K L (A 0 1) * denominator K L B z := by
  simp [numerator, denominator, Matrix.mul_apply, Fin.sum_univ_two, map_add, map_mul]
  ring

theorem denominator_mul (A B : Mat2 K) (z : L) :
    denominator K L (A * B) z = algebraMap K L (A 1 0) * numerator K L B z +
      algebraMap K L (A 1 1) * denominator K L B z := by
  simp [numerator, denominator, Matrix.mul_apply, Fin.sum_univ_two, map_add, map_mul]
  ring

variable [FiniteDimensional K L]

/-- The actual cubic-point stabilizer in GL2 consists exactly of scalar matrices. -/
theorem cubic_stabilizer_scalar (hd : Module.finrank K L = 3) {z : L}
    (hz : outsideBase K L z) (A : GL2 K) (h : sends K L A.val z z) :
    ∃ u : Kˣ, A = Matrix.GeneralLinearGroup.scalar (Fin 2) u := by
  have hr : algebraMap K L (A 1 0)*z^2 +
      algebraMap K L (A 1 1-A 0 0)*z + algebraMap K L (-A 0 1) = 0 := by
    dsimp [sends, numerator, denominator] at h
    simp only [map_sub, map_neg]
    linear_combination -h
  have hh := quadratic_relation_zero K L hd hz _ _ _ hr
  have hdiag : A 1 1 = A 0 0 := sub_eq_zero.mp hh.2.1
  have hoff : A 0 1 = 0 := neg_eq_zero.mp hh.2.2
  have hn : A 0 0 ≠ 0 := by
    intro he
    apply A.det_ne_zero
    simp [det_fin_two, hdiag, hoff, hh.1, he]
  refine ⟨Units.mk0 (A 0 0) hn, ?_⟩
  apply Matrix.GeneralLinearGroup.ext
  intro i j
  fin_cases i <;> fin_cases j <;>
    simp [Matrix.GeneralLinearGroup.coe_scalar, Matrix.scalar_apply, hdiag, hoff, hh.1]

/-- Uniqueness of the actual projective matrix, retaining both non-base source and target. -/
theorem cubic_homography_unique (hd : Module.finrank K L = 3) {z w : L}
    (hz : outsideBase K L z) (hw : outsideBase K L w) (A B : GL2 K)
    (ha : sends K L A.val z w) (hb : sends K L B.val z w) :
    Matrix.ProjGenLinGroup.mk A = Matrix.ProjGenLinGroup.mk B := by
  let H : GL2 K := B⁻¹ * A
  have he : B.val * H.val = A.val := by
    change (B * (B⁻¹ * A)).val = A.val
    simp [← mul_assoc]
  have hcoef : algebraMap K L (B 0 0) - w*algebraMap K L (B 1 0) ≠ 0 := by
    intro hc
    have hh := linear_relation_zero K L hw (B 1 0) (-B 0 0) (by
      rw [map_neg]
      linear_combination -hc)
    apply B.det_ne_zero
    simp [det_fin_two, hh.1, neg_eq_zero.mp hh.2]
  have hha := ha
  unfold sends at hha hb
  rw [← he, numerator_mul, denominator_mul] at hha
  have hm : (algebraMap K L (B 0 0) - w*algebraMap K L (B 1 0)) *
      (numerator K L H.val z - z * denominator K L H.val z) = 0 := by
    dsimp [numerator, denominator] at hb
    linear_combination hha - denominator K L H.val z * hb
  have hs : sends K L H.val z z := sub_eq_zero.mp ((mul_eq_zero.mp hm).resolve_left hcoef)
  obtain ⟨u, hu⟩ := cubic_stabilizer_scalar K L hd hz H hs
  have hh : Matrix.ProjGenLinGroup.mk H = 1 := by
    rw [hu]
    exact Matrix.ProjGenLinGroup.mk_scalar u
  have hx : (Matrix.ProjGenLinGroup.mk B)⁻¹ * Matrix.ProjGenLinGroup.mk A = 1 := by
    simpa [H] using hh
  exact (inv_mul_eq_one.mp hx).symm

theorem cubic_homography_unique_up_to_scalar (hd : Module.finrank K L = 3) {z w : L}
    (hz : outsideBase K L z) (hw : outsideBase K L w) (A B : GL2 K)
    (ha : sends K L A.val z w) (hb : sends K L B.val z w) :
    ∃ u : Kˣ, A * Matrix.GeneralLinearGroup.scalar (Fin 2) u = B :=
  Matrix.ProjGenLinGroup.mk_eq_mk_iff.mp (cubic_homography_unique K L hd hz hw A B ha hb)

#print axioms cubic_stabilizer_scalar
#print axioms cubic_homography_unique
#print axioms cubic_homography_unique_up_to_scalar
end
end MomoseCubicHomographyAudit
