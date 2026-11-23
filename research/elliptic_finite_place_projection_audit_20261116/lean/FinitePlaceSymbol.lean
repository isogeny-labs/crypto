import FinitePlaceResidue

namespace EllipticFinitePlaceProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- Actual normalized finite base symbol at every actual finite nonsingular point. -/
def finitePlaceTameSymbol (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y)
    (a b : (RatFunc K)ˣ) : Kˣ :=
  (-1 : Kˣ)^(finiteBaseOrder x (a : RatFunc K)*finiteBaseOrder x (b : RatFunc K))*
    finitePlaceUnitEvaluation W h (finiteBaseBalancedLocalUnit x a b)

theorem finite_place_tame_model_independent (W₁ W₂ : WeierstrassCurve.Affine K) {x y₁ y₂ : K}
    (h₁ : W₁.Nonsingular x y₁) (h₂ : W₂.Nonsingular x y₂) (a b : (RatFunc K)ˣ) :
    finitePlaceTameSymbol W₁ h₁ a b=finitePlaceTameSymbol W₂ h₂ a b := by
  rw [finitePlaceTameSymbol,finitePlaceTameSymbol,finite_place_unit_model_independent W₁ W₂ h₁ h₂]

theorem finite_place_tame_fixed (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) (a b : (RatFunc K)ˣ) :
    finitePlaceTameSymbol W h a b=finiteBaseTameSymbol W h hP a b := rfl

theorem finite_place_split_field_unit_order (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h) (c : (RatFunc K)ˣ) :
    pointOrder W (baseFieldUnit W c) (Point.some x y h)=finiteBaseOrder x (c : RatFunc K) := by
  change pointOrder W (algebraMap (RatFunc K) W.FunctionField (c : RatFunc K)) (Point.some x y h)=_
  rw [←point_base_order_embedding]
  exact split_restricted_order_official W h hP c

theorem finite_place_balanced_embedding_split (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h) (a b : (RatFunc K)ˣ) :
    finitePlaceUnitEmbedding W h (finiteBaseBalancedLocalUnit x a b)=
      balancedLocalUnit W (Point.some x y h) (baseFieldUnit W a) (baseFieldUnit W b) := by
  apply Subtype.ext
  change baseFieldUnit W (finiteBaseBalancedUnit x a b)=
    (baseFieldUnit W a)^pointOrder W (baseFieldUnit W b) (Point.some x y h)/
      (baseFieldUnit W b)^pointOrder W (baseFieldUnit W a) (Point.some x y h)
  rw [finiteBaseBalancedUnit,map_div,map_zpow,map_zpow,
    finite_place_split_field_unit_order W h hP,finite_place_split_field_unit_order W h hP]

theorem finite_place_tame_split_embedding (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h) (a b : (RatFunc K)ˣ) :
    finitePlaceTameSymbol W h a b=tameSymbol W (Point.some x y h) (baseFieldUnit W a) (baseFieldUnit W b) := by
  rw [finitePlaceTameSymbol,finite_place_unit_eval_compatibility,finite_place_balanced_embedding_split W h hP,
    tameSymbol,finite_place_split_field_unit_order W h hP,finite_place_split_field_unit_order W h hP]

#print axioms finite_place_tame_model_independent
#print axioms finite_place_balanced_embedding_split
#print axioms finite_place_tame_split_embedding
end
end EllipticFinitePlaceProjectionAudit
