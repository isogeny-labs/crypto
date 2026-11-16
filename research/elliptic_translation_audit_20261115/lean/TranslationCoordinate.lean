module
public import TranslationTranscendence

@[expose] public section
namespace EllipticTranslationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit NegationPullbackAudit FunctionDivisorFinitenessAudit MillerBasechangeAudit
variable {K : Type*} [Field K] [DecidableEq K]

def translatedX (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (Q : W.Point) :
    W.FunctionField := genericX W (translatedGeneric W hΔ Q)

def translatedY (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (Q : W.Point) :
    W.FunctionField := genericY W (translatedGeneric W hΔ Q)

/-- Actual translated generic coordinates satisfy the actual coefficient polynomial. -/
theorem translated_root_equation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) :
    W.polynomial.eval₂ (aeval (translatedX W hΔ Q)).toRingHom (translatedY W hΔ Q)=0 := by
  have he := (generic_coordinates_nonsingular W (translatedGeneric W hΔ Q)
    (translated_generic_nonzero W hΔ Q)).1
  rw [(genericCurve W).equation_iff] at he
  simp only [genericCurve,extendedCurve,WeierstrassCurve.map,function_coefficient_map] at he
  change translatedY W hΔ Q^2+constantHom W W.a₁*translatedX W hΔ Q*translatedY W hΔ Q+
    constantHom W W.a₃*translatedY W hΔ Q = translatedX W hΔ Q^3+
    constantHom W W.a₂*translatedX W hΔ Q^2+constantHom W W.a₄*translatedX W hΔ Q+
    constantHom W W.a₆ at he
  simp only [WeierstrassCurve.Affine.polynomial,eval₂_sub,eval₂_add,eval₂_mul,
    eval₂_pow,eval₂_X,eval₂_C,AlgHom.toRingHom_eq_coe,AlgHom.coe_toRingHom,
    map_add,map_mul,aeval_C,aeval_X,map_pow,function_coefficient_map]
  linear_combination he

/-- Actual coordinate substitution into the actual function field; no affine-ring automorphism assumed. -/
def translationCoordinate (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (Q : W.Point) :
    W.CoordinateRing →+* W.FunctionField :=
  AdjoinRoot.lift (aeval (translatedX W hΔ Q)).toRingHom (translatedY W hΔ Q)
    (translated_root_equation W hΔ Q)

theorem translation_coordinate_polynomial (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) (p : K[X]) :
    translationCoordinate W hΔ Q (mk W (C p)) = aeval (translatedX W hΔ Q) p :=
  AdjoinRoot.lift_of (translated_root_equation W hΔ Q)

theorem translation_coordinate_root (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) : translationCoordinate W hΔ Q (mk W X) = translatedY W hΔ Q :=
  AdjoinRoot.lift_root (translated_root_equation W hΔ Q)

variable [IsAlgClosed K]

/-- Actual nonzero norm polynomial and derived transcendence force the substitution injective. -/
theorem translation_coordinate_injective (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) : Function.Injective (translationCoordinate W hΔ Q) := by
  apply (injective_iff_map_eq_zero _).mpr
  intro a ha
  by_contra hn
  have he := congrArg (translationCoordinate W hΔ Q) (coordinate_norm_product W a)
  rw [translation_coordinate_polynomial,map_mul,ha,zero_mul] at he
  have hi : Function.Injective (aeval (translatedX W hΔ Q) : K[X] →ₐ[K] W.FunctionField) :=
    transcendental_iff_injective.mp (translated_x_transcendental W hΔ Q)
  have hz : Algebra.norm K[X] a=0 := hi (he.trans (map_zero _).symm)
  exact norm_polynomial_nonzero W hn hz

/-- The actual fraction-field lift, with injectivity proved rather than supplied. -/
def fieldTranslation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (Q : W.Point) :
    W.FunctionField →+* W.FunctionField :=
  IsFractionRing.lift (translation_coordinate_injective W hΔ Q)

theorem field_translation_map (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) (a : W.CoordinateRing) :
    fieldTranslation W hΔ Q (algebraMap W.CoordinateRing W.FunctionField a)=
      translationCoordinate W hΔ Q a := IsFractionRing.lift_algebraMap _ a

theorem field_translation_constant (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) (c : K) : fieldTranslation W hΔ Q (constantHom W c)=constantHom W c := by
  change fieldTranslation W hΔ Q
    (algebraMap W.CoordinateRing W.FunctionField (mk W (C (C c)))) = constantHom W c
  rw [field_translation_map,translation_coordinate_polynomial,aeval_C,function_coefficient_map]

theorem field_translation_x (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) : fieldTranslation W hΔ Q (xFunction W)=translatedX W hΔ Q := by
  have he := field_translation_map W hΔ Q (mk W (C X))
  rw [translation_coordinate_polynomial,aeval_X] at he
  simpa only [xFunction,XClass,map_zero,sub_zero] using he

theorem field_translation_y (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (Q : W.Point) : fieldTranslation W hΔ Q (yFunction W)=translatedY W hΔ Q := by
  have he := field_translation_map W hΔ Q (mk W X)
  rw [translation_coordinate_root] at he
  simpa only [yFunction,YClass,map_zero,sub_zero] using he

#print axioms translation_coordinate_injective
#print axioms field_translation_x
end
end EllipticTranslationAudit
