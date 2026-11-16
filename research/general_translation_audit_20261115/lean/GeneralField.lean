module
public import GeneralTranscendence

@[expose] public section
namespace GeneralTranslationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open EllipticTranslationAudit MillerNormValuationAudit MillerInfinityChartAudit
open FunctionDivisorFinitenessAudit NegationPullbackAudit
variable {K : Type*} [Field K] [DecidableEq K]

theorem general_coordinate_injective (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) : Function.Injective (translationCoordinate W hΔ Q) := by
  apply (injective_iff_map_eq_zero _).mpr
  intro a ha
  by_contra hn
  have he := congrArg (translationCoordinate W hΔ Q) (coordinate_norm_product W a)
  rw [translation_coordinate_polynomial,map_mul,ha,zero_mul] at he
  have hi : Function.Injective (aeval (translatedX W hΔ Q) : K[X] →ₐ[K] W.FunctionField) :=
    transcendental_iff_injective.mp (general_translated_x_transcendental W hΔ Q)
  have hz : Algebra.norm K[X] a=0 := hi (he.trans (map_zero _).symm)
  exact norm_polynomial_nonzero W hn hz

/-- Actual coefficient-field function-field pullback, without algebraic closure. -/
def generalTranslation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (Q : W.Point) :
    W.FunctionField →+* W.FunctionField := IsFractionRing.lift (general_coordinate_injective W hΔ Q)

theorem general_translation_map (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (a : W.CoordinateRing) :
    generalTranslation W hΔ Q (algebraMap W.CoordinateRing W.FunctionField a)=
      translationCoordinate W hΔ Q a := IsFractionRing.lift_algebraMap _ a

theorem general_translation_constant (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (c : K) :
    generalTranslation W hΔ Q (constantHom W c)=constantHom W c := by
  change generalTranslation W hΔ Q
    (algebraMap W.CoordinateRing W.FunctionField (mk W (C (C c))))=constantHom W c
  rw [general_translation_map,translation_coordinate_polynomial,aeval_C,function_coefficient_map]

theorem general_translation_x (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) : generalTranslation W hΔ Q (xFunction W)=translatedX W hΔ Q := by
  have he := general_translation_map W hΔ Q (mk W (C X))
  rw [translation_coordinate_polynomial,aeval_X] at he
  simpa only [xFunction,XClass,map_zero,sub_zero] using he

theorem general_translation_y (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) : generalTranslation W hΔ Q (yFunction W)=translatedY W hΔ Q := by
  have he := general_translation_map W hΔ Q (mk W X)
  rw [translation_coordinate_root] at he
  simpa only [yFunction,YClass,map_zero,sub_zero] using he

def generalTranslationAlgHom (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (Q : W.Point) :
    W.FunctionField →ₐ[K] W.FunctionField where
  __ := generalTranslation W hΔ Q
  toFun := fun f => generalTranslation W hΔ Q f
  commutes' c := by
    rw [function_coefficient_map]
    exact general_translation_constant W hΔ Q c

/-- The new unrestricted map agrees with the frozen construction on its old domain. -/
theorem general_translation_agrees [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) : generalTranslation W hΔ Q=fieldTranslation W hΔ Q := rfl

#print axioms general_coordinate_injective
#print axioms general_translation_agrees
end
end GeneralTranslationAudit
