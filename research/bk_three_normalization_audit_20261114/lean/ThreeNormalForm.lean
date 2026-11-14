import ShiftPointGroup

namespace BKThreeNormalizationAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General
variable {K : Type*} [Field K]

def tangentSlope (W : WeierstrassCurve.Affine K) (x y : K) : K :=
  numerator W x y/denominator W x y

def normalized (W : WeierstrassCurve.Affine K) (x y : K) : WeierstrassCurve.Affine K :=
  shifted W x (tangentSlope W x y) y

theorem normalized_discriminant (W : WeierstrassCurve.Affine K) (x y : K) :
    (normalized W x y).Δ=W.Δ := shifted_discriminant W x (tangentSlope W x y) y

theorem normalized_a6 (W : WeierstrassCurve.Affine K) (x y : K) (hN : W.Nonsingular x y) :
    (normalized W x y).a₆=0 := by
  have he := hN.1
  rw [WeierstrassCurve.Affine.equation_iff'] at he
  simp only [normalized,shifted]
  linear_combination -he

theorem normalized_a4 (W : WeierstrassCurve.Affine K) (x y : K) (hD : denominator W x y≠0) :
    (normalized W x y).a₄=0 := by
  simp only [normalized,shifted,tangentSlope]
  field_simp
  simp only [numerator,denominator]
  ring

variable [DecidableEq K]

theorem normalized_coefficients (W : WeierstrassCurve.Affine K) (x y : K)
    (hN : W.Nonsingular x y) (ht : (3:ℕ) • Point.some x y hN=0) :
    (normalized W x y).a₂=0 ∧ (normalized W x y).a₄=0 ∧ (normalized W x y).a₆=0 := by
  have hp := (three_point_iff W hN).mp ht
  have hD := psi_root_denominator_nonzero W hN hp
  refine ⟨?_,normalized_a4 W x y hD,normalized_a6 W x y hN⟩
  have he : W.Ψ₃.eval x=0 := hp
  rw [psi_at_point W hN.1] at he
  simp only [normalized,shifted,tangentSlope]
  field_simp
  linear_combination he

theorem actual_normalization (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (x y : K) (hN : W.Nonsingular x y) (ht : (3:ℕ) • Point.some x y hN=0) :
    (normalized W x y).Δ≠0 ∧
    (normalized W x y).a₂=0 ∧ (normalized W x y).a₄=0 ∧ (normalized W x y).a₆=0 ∧
    Nonempty ((normalized W x y).Point ≃+ W.Point) := by
  obtain ⟨h2,h4,h6⟩ := normalized_coefficients W x y hN ht
  exact ⟨by rwa [normalized_discriminant],h2,h4,h6,⟨shiftEquiv W hΔ x (tangentSlope W x y) y⟩⟩

#print axioms normalized_coefficients
#print axioms actual_normalization
end
end BKThreeNormalizationAudit
