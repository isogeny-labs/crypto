module
public import TranslationInfinity
public import EvaluationCertificate

@[expose] public section
namespace LocalTranslationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open EllipticTranslationAudit MillerNormValuationAudit MillerFinitePointAudit
open FinitePointEvaluationAudit FunctionDivisorFinitenessAudit MillerPrincipalIdealAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

theorem translated_x_integral (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    functionValuation W (translatedX W hΔ (.some b c h))≤1 := by
  have he := (functionValuation W).map_add_le (delta_x_small W hΔ h).le
    (function_valuation_constant_le W b)
  simpa only [deltaX,sub_add_cancel,field_translation_x] using he

theorem translated_y_integral (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    functionValuation W (translatedY W hΔ (.some b c h))≤1 := by
  have he := (functionValuation W).map_add_le (delta_y_small W hΔ h).le
    (function_valuation_constant_le W c)
  simpa only [deltaY,sub_add_cancel,field_translation_y] using he

/-- Every actual affine coordinate function becomes regular at infinity after pullback by Q. -/
theorem translation_coordinate_integral (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) (a : W.CoordinateRing) :
    translationCoordinate W hΔ (.some b c h) a ∈ RegularRing W Point.zero := by
  have hp (p : K[X]) : aeval (translatedX W hΔ (.some b c h)) p ∈ RegularRing W Point.zero := by
    induction p using Polynomial.induction_on' with
    | add p q ihp ihq =>
      rw [map_add]
      exact (RegularRing W Point.zero).add_mem _ _ ihp ihq
    | monomial n d =>
      rw [aeval_monomial]
      exact (RegularRing W Point.zero).mul_mem _ _ (function_valuation_constant_le W d)
        ((RegularRing W Point.zero).toSubring.pow_mem (translated_x_integral W hΔ h) n)
  obtain ⟨p,q,rfl⟩ := exists_smul_basis_eq a
  have he := (RegularRing W Point.zero).add_mem _ _ (hp p)
    ((RegularRing W Point.zero).mul_mem _ _ (hp q) (translated_y_integral W hΔ h))
  simpa only [smul,map_add,map_mul,map_one,mul_one,translation_coordinate_polynomial,
    translation_coordinate_root] using he

def coordinateInfinity (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) : W.CoordinateRing →+* RegularRing W Point.zero where
  toFun a := ⟨translationCoordinate W hΔ (.some b c h) a,
    translation_coordinate_integral W hΔ h a⟩
  map_zero' := Subtype.ext (map_zero _)
  map_one' := Subtype.ext (map_one _)
  map_add' a b := Subtype.ext (map_add _ a b)
  map_mul' a b := Subtype.ext (map_mul _ a b)

theorem coordinate_infinity_coe (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) (a : W.CoordinateRing) :
    (coordinateInfinity W hΔ h a : W.FunctionField)=
      fieldTranslation W hΔ (.some b c h) (algebraMap W.CoordinateRing W.FunctionField a) :=
  (field_translation_map W hΔ (.some b c h) a).symm

omit [DecidableEq K] [IsAlgClosed K] in
theorem infinity_evaluation_of_small (W : WeierstrassCurve.Affine K)
    (r : RegularRing W Point.zero) (d : K)
    (h : functionValuation W ((r : W.FunctionField)-constantHom W d)<1) :
    regularEvaluation W .zero r=d := by
  have hm : r-regularConstant W .zero d ∈ IsLocalRing.maximalIdeal (RegularRing W .zero) :=
    (Valuation.mem_maximalIdeal_iff (K := W.FunctionField) (v := functionValuation W)).mpr h
  have hz : regularEvaluation W .zero (r-regularConstant W .zero d)=0 := by
    change r-regularConstant W .zero d ∈ RingHom.ker (regularEvaluation W .zero)
    rw [regular_evaluation_kernel]
    exact hm
  simpa only [map_sub,regular_eval_constant,sub_eq_zero] using hz

theorem coordinate_infinity_polynomial (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) (p : K[X]) :
    regularEvaluation W .zero (coordinateInfinity W hΔ h (mk W (C p)))=p.eval b := by
  have hX : regularEvaluation W .zero (coordinateInfinity W hΔ h (mk W (C X)))=b := by
    apply infinity_evaluation_of_small W
    change functionValuation W
      (translationCoordinate W hΔ (.some b c h) (mk W (C X))-constantHom W b)<1
    rw [translation_coordinate_polynomial,aeval_X,← field_translation_x]
    exact delta_x_small W hΔ h
  have hd (d : K) : coordinateInfinity W hΔ h (mk W (C (C d)))=regularConstant W .zero d := by
    apply Subtype.ext
    change translationCoordinate W hΔ (.some b c h) (mk W (C (C d)))=constantHom W d
    rw [translation_coordinate_polynomial,aeval_C]
    rfl
  have he : ((regularEvaluation W .zero).comp (coordinateInfinity W hΔ h)).comp ((mk W).comp C)=
      evalRingHom b := by
    apply Polynomial.ringHom_ext
    · intro d
      simp only [RingHom.comp_apply,coe_evalRingHom,eval_C]
      rw [hd,regular_eval_constant]
    · simpa only [RingHom.comp_apply,coe_evalRingHom,eval_X] using hX
  exact RingHom.congr_fun he p

theorem coordinate_infinity_residue (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) (a : W.CoordinateRing) :
    regularEvaluation W .zero (coordinateInfinity W hΔ h a)=pointEval W b c h.1 a := by
  have hY : regularEvaluation W .zero (coordinateInfinity W hΔ h (mk W X))=c := by
    apply infinity_evaluation_of_small W
    change functionValuation W
      (translationCoordinate W hΔ (.some b c h) (mk W X)-constantHom W c)<1
    rw [translation_coordinate_root,← field_translation_y]
    exact delta_y_small W hΔ h
  have hy : pointEval W b c h.1 (mk W X)=c := by
    simpa only [YClass,map_sub,map_zero,sub_zero,eval_zero] using
      point_eval_YClass W h.1 (0 : K[X])
  obtain ⟨p,q,rfl⟩ := exists_smul_basis_eq a
  simp only [smul,map_add,map_mul,mul_one,coordinate_infinity_polynomial,
    point_eval_polynomial,hY,hy]

#print axioms translation_coordinate_integral
#print axioms coordinate_infinity_residue
end
end LocalTranslationAudit
