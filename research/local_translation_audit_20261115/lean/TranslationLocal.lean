module
public import TranslationIntegral

@[expose] public section
namespace LocalTranslationAudit
noncomputable section
open WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open EllipticTranslationAudit MillerNormValuationAudit MillerFinitePointAudit
open MillerPrincipalIdealAudit FinitePointEvaluationAudit
open scoped WithZero
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

local instance pointMonoid (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Equation b c) : Monoid (PointLocalRing W b c h) :=
  (inferInstance : CommRing (PointLocalRing W b c h)).toMonoid

local instance pointFractionRing (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Equation b c) : IsFractionRing (PointLocalRing W b c h) W.FunctionField :=
  pointFunctionFractionRing W h

theorem translation_denominator_unit (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) (s : (evaluationIdeal W b c h.1).primeCompl) :
    IsUnit (coordinateInfinity W hΔ h s) := by
  by_contra hn
  have hm : coordinateInfinity W hΔ h s ∈ IsLocalRing.maximalIdeal (RegularRing W Point.zero) :=
    (IsLocalRing.mem_maximalIdeal _).mpr hn
  have hz : regularEvaluation W Point.zero (coordinateInfinity W hΔ h s)=0 := by
    change coordinateInfinity W hΔ h s ∈ RingHom.ker (regularEvaluation W Point.zero)
    rw [regular_evaluation_kernel]
    exact hm
  rw [coordinate_infinity_residue] at hz
  exact s.property hz

/-- Actual localization map from Q's local ring to infinity's regular ring. -/
def localInfinity (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    PointLocalRing W b c h.1 →+* RegularRing W Point.zero :=
  IsLocalization.lift (translation_denominator_unit W hΔ h)

theorem local_infinity_map (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) (a : W.CoordinateRing) :
    localInfinity W hΔ h (algebraMap W.CoordinateRing (PointLocalRing W b c h.1) a)=
      coordinateInfinity W hΔ h a := IsLocalization.lift_eq _ a

theorem local_infinity_field (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) (a : PointLocalRing W b c h.1) :
    (localInfinity W hΔ h a : W.FunctionField)=
      fieldTranslation W hΔ (.some b c h) (pointToField W h.1 a) := by
  have he : (RegularRing W Point.zero).subtype.comp (localInfinity W hΔ h)=
      (fieldTranslation W hΔ (.some b c h)).comp (pointToField W h.1) := by
    apply IsLocalization.ringHom_ext (evaluationIdeal W b c h.1).primeCompl
    apply RingHom.ext
    intro d
    change (localInfinity W hΔ h (algebraMap W.CoordinateRing (PointLocalRing W b c h.1) d) :
      W.FunctionField)=fieldTranslation W hΔ (.some b c h)
        (pointToField W h.1 (algebraMap W.CoordinateRing (PointLocalRing W b c h.1) d))
    rw [local_infinity_map,point_to_field_map,coordinate_infinity_coe]
  exact RingHom.congr_fun he a

theorem local_infinity_residue (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) (a : PointLocalRing W b c h.1) :
    regularEvaluation W Point.zero (localInfinity W hΔ h a)=localPointEval W h.1 a := by
  have he : (regularEvaluation W Point.zero).comp (localInfinity W hΔ h)=localPointEval W h.1 := by
    apply IsLocalization.ringHom_ext (evaluationIdeal W b c h.1).primeCompl
    apply RingHom.ext
    intro d
    change regularEvaluation W Point.zero
      (localInfinity W hΔ h (algebraMap W.CoordinateRing (PointLocalRing W b c h.1) d))=
      localPointEval W h.1 (algebraMap W.CoordinateRing (PointLocalRing W b c h.1) d)
    rw [local_infinity_map,coordinate_infinity_residue,local_point_map]
  exact RingHom.congr_fun he a

theorem translation_parameter_value (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) :
    functionValuation W
      (fieldTranslation W hΔ (.some b c h) (pointToField W h.1 (pointParameter W h)))=
      WithZero.exp (-1 : ℤ) := by
  classical
  by_cases ht : derivativeY W b c≠0
  · simp only [pointParameter,ite_eq_left ht,localU,point_to_field_map,XClass,map_sub]
    have hx : algebraMap W.CoordinateRing W.FunctionField (mk W (Polynomial.C Polynomial.X))=
        xFunction W := by simp only [xFunction,XClass,map_zero,sub_zero]
    rw [hx]
    change functionValuation W (fieldTranslation W hΔ (.some b c h) (xFunction W)-
      fieldTranslation W hΔ (.some b c h) (constantHom W b))=WithZero.exp (-1 : ℤ)
    rw [field_translation_constant]
    exact delta_x_parameter_value W hΔ h ht
  · simp only [pointParameter,ite_eq_right ht,localV,point_to_field_map,YClass,map_sub]
    have hy : algebraMap W.CoordinateRing W.FunctionField (mk W Polynomial.X)=
        yFunction W := by simp only [yFunction,YClass,map_zero,sub_zero]
    rw [hy]
    change functionValuation W (fieldTranslation W hΔ (.some b c h) (yFunction W)-
      fieldTranslation W hΔ (.some b c h) (constantHom W c))=WithZero.exp (-1 : ℤ)
    rw [field_translation_constant]
    exact delta_y_parameter_value W hΔ h (not_ne_iff.mp ht)

theorem translation_unit_value (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) (u : (PointLocalRing W b c h.1)ˣ) :
    functionValuation W (fieldTranslation W hΔ (.some b c h) (pointToField W h.1 u))=1 := by
  have hv : regularEvaluation W Point.zero (localInfinity W hΔ h u)≠0 := by
    rw [local_infinity_residue]
    exact (local_point_unit_iff W h.1 u).mp u.isUnit
  have he := (regular_evaluation_value_iff W Point.zero (localInfinity W hΔ h u)).mp hv
  change functionValuation W (localInfinity W hΔ h u : W.FunctionField)=1 at he
  rw [local_infinity_field] at he
  exact he

/-- Exact normalized equality on every actual function, derived from actual units/uniformizer. -/
theorem finite_infinity_translation_valuation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    {b c : K} (h : W.Nonsingular b c) (f : W.FunctionField) :
    functionValuation W (fieldTranslation W hΔ (.some b c h) f)=finiteAdic W h f := by
  let := pointLocalDVR W h
  by_cases hf : f=0
  · rw [hf,map_zero,map_zero,map_zero]
  have hπ : Irreducible (pointParameter W h) :=
    (IsDiscreteValuationRing.irreducible_iff_uniformizer _).mpr (point_parameter_maximal W h)
  obtain ⟨n,u,hu⟩ := IsDiscreteValuationRing.exists_units_eq_smul_zpow_of_irreducible hπ hf
  have he : f=pointToField W h.1 u*pointToField W h.1 (pointParameter W h)^n := by
    simpa only [Units.smul_def,Algebra.smul_def,NegationPullbackAudit.point_field_map] using hu
  rw [he]
  simp only [map_mul,map_zpow₀,translation_unit_value,translation_parameter_value,
    finite_adic_unit,finite_adic_parameter]

#print axioms local_infinity_field
#print axioms local_infinity_residue
#print axioms finite_infinity_translation_valuation
end
end LocalTranslationAudit
