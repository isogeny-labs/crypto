import NormLocal

namespace EllipticNormTransportAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit
open scoped EllipticFieldNormAudit
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- Honest finite-support product is invariant under the actual negation permutation. -/
theorem global_negation_covariance (W : WeierstrassCurve.Affine K) (f g : W.FunctionFieldˣ) :
    globalTameProduct W (negatedUnit W f) (negatedUnit W g)=globalTameProduct W f g := by
  unfold globalTameProduct
  calc
    _ = ∏ᶠ P,tameSymbol W (-P) f g := finprod_congr (fun P=>tame_negation_covariance W P f g)
    _ = _ := finprod_comp_equiv (pointNegEquiv W) (f:=fun P=>tameSymbol W P f g)

/-- This is a square identity, not a proof that either side is1. -/
theorem actual_global_norm_square (W : WeierstrassCurve.Affine K) (f g : W.FunctionFieldˣ)
    (hg : negatedUnit W g=g) :
    globalTameProduct W (embeddedNorm W f) g=(globalTameProduct W f g)^2 := by
  rw [embedded_norm_product,global_mul_left]
  have he:=global_negation_covariance W f g
  rw [hg] at he
  rw [he,pow_two]

theorem actual_rational_global_norm_square (W : WeierstrassCurve.Affine K)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    globalTameProduct W (embeddedNorm W f)
      (Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom c)=
    (globalTameProduct W f (Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom c))^2 :=
  actual_global_norm_square W f _ (rational_unit_negation W c)

/-- Fixed-point local formula, including wildly ramified characteristic2 points. -/
theorem actual_fixed_point_norm_tame (W : WeierstrassCurve.Affine K) (P : W.Point)
    (hP : -P=P) (f g : W.FunctionFieldˣ) (hg : negatedUnit W g=g) :
    tameSymbol W P (embeddedNorm W f) g=(tameSymbol W P f g)^2 := by
  rw [actual_norm_tame_orbit W P f g hg,hP,pow_two]

#print axioms global_negation_covariance
#print axioms actual_rational_global_norm_square
#print axioms actual_fixed_point_norm_tame
end
end EllipticNormTransportAudit
