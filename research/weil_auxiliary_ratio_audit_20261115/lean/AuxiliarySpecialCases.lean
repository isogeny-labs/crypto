import AuxiliaryDiagonal

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open NegationPullbackAudit GeneralTranslationAudit
open BKThreeNormalizationAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- Actual opposite-point function relation makes the quotient one by cancellation. -/
theorem opposite_function_quotient_one (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P : W.Point) (f g : W.FunctionFieldˣ) (a : K)
    (he : (g : W.FunctionField)=constantHom W a*fieldNeg W f) :
    auxiliaryQuotient W hΔ P (-P) f g=1 := by
  apply (div_eq_one_iff_eq (auxiliary_denominator_nonzero W hΔ P f.ne_zero g.ne_zero)).mpr
  unfold auxiliaryNumerator auxiliaryDenominator
  rw [he,map_mul,negation_constant,field_neg_square,map_mul,general_translation_constant]
  ring

/-- Actual geometric divisor comparison derives the opposite-point function relation. -/
theorem opposite_auxiliary_one [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P : W.Point) (n : ℕ)
    (f g : W.FunctionFieldˣ)
    (hDf : functionDivisor W f f.ne_zero=torsionDivisor W P n)
    (hDg : functionDivisor W g g.ne_zero=torsionDivisor W (-P) n) :
    auxiliaryQuotient W hΔ P (-P) f g=1 := by
  have hd : functionDivisor W g g.ne_zero=
      functionDivisor W (fieldNeg W f) (field_neg_nonzero W f.ne_zero) := by
    rw [function_divisor_pullback W (f : W.FunctionField) f.ne_zero,hDg,hDf]
    simp only [torsionDivisor,map_sub,map_zsmul,divisor_neg_single,neg_zero]
  obtain ⟨a,ha,he⟩ := (same_divisor_iff_scalar W hΔ g.ne_zero
    (field_neg_nonzero W f.ne_zero)).mp hd
  exact opposite_function_quotient_one W hΔ P f g a he

theorem chosen_opposite_one [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P : W.Point) (n : ℕ) (ht : n • P=0) :
    auxiliaryQuotient W hΔ P (-P) (chosenFunctionUnit W P n) (chosenFunctionUnit W (-P) n)=1 := by
  have hn : n • (-P)=0 := by rw [smul_neg,ht,neg_zero]
  exact opposite_auxiliary_one W hΔ P n _ _ (chosen_unit_divisor W P n ht)
    (chosen_unit_divisor W (-P) n hn)

theorem miller_zero_point (W : WeierstrassCurve.Affine K) (n : ℕ) :
    millerFunction W 0 n=1 := by
  induction n with
  | zero => rfl
  | succ n ih =>
    rw [millerFunction,ih,nsmul_zero]
    simp [stepFunction]

theorem auxiliary_zero_left (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) {g : W.FunctionField} (hg : g≠0) :
    auxiliaryQuotient W hΔ 0 Q 1 g=1 := by
  simpa only [auxiliaryQuotient,auxiliaryNumerator,auxiliaryDenominator,map_one,
    neg_zero,general_translation_zero,RingHom.id_apply,one_mul] using div_self (field_neg_nonzero W hg)

theorem auxiliary_zero_right (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P : W.Point) {f : W.FunctionField} (hf : f≠0) :
    auxiliaryQuotient W hΔ P 0 f 1=1 := by
  simpa only [auxiliaryQuotient,auxiliaryNumerator,auxiliaryDenominator,map_one,
    general_translation_zero,RingHom.id_apply,mul_one] using div_self hf

/-- Actual chosen normalized Miller function at three is the actual y-monic tangent. -/
theorem chosen_three_tangent [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) {x y : K} (hP : W.Nonsingular x y)
    (ht : (3 : ℕ) • Point.some x y hP=0) :
    (chosenFunctionUnit W (.some x y hP) 3 : W.FunctionField)=tangentFunction W x y := by
  apply normalized_divisor_unique W hΔ (chosenFunctionUnit W (.some x y hP) 3).ne_zero
    (tangent_function_nonzero W x y) ?_ (miller_infinity_monic W _ 3)
    (infinity_monic_line W x y (tangentSlope W x y))
  exact (chosen_unit_divisor W (.some x y hP) 3 ht).trans (tangent_function_divisor W hP ht).symm

#print axioms opposite_auxiliary_one
#print axioms chosen_opposite_one
#print axioms miller_zero_point
#print axioms chosen_three_tangent
end
end WeilAuxiliaryRatioAudit
