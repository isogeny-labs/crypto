import NormTransport

namespace EllipticNormTransportAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- The official rational-function infinity place, with a classical equality decision. -/
def baseInfinityValuation (K : Type*) [Field K] : Valuation (RatFunc K) ℤᵐ⁰ := by
  classical
  exact RatFunc.inftyValuation K

def baseInfinityOrder (r : RatFunc K) : ℤ := -WithZero.log (baseInfinityValuation K r)

theorem base_infinity_polynomial (p : K[X]) :
    baseInfinityValuation K (algebraMap K[X] (RatFunc K) p)=degreeValue p.degree := by
  classical
  by_cases hp : p=0
  · rw [hp,map_zero,map_zero,degree_zero]
    rfl
  · change RatFunc.inftyValuationDef K _=_
    rw [RatFunc.inftyValuation.polynomial K hp,degree_eq_natDegree hp]
    rfl

/-- Actual norm down to K(x), evaluated at the actual base infinity valuation.
The residue-degree-one formula is proved by actual fractions; not assumed. -/
theorem actual_norm_infinity_valuation (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    baseInfinityValuation K (Algebra.norm (RatFunc K) f)=functionValuation W f := by
  obtain ⟨a,b,_hb,hab⟩ := IsFractionRing.div_surjective W.CoordinateRing f
  rw [←hab,norm_fraction,norm_coordinate,norm_coordinate,map_div₀,map_div₀,
    function_valuation_map,function_valuation_map,base_infinity_polynomial,base_infinity_polynomial]
  rfl

/-- Base order of the actual norm equals the elliptic order of f, not twice it. -/
theorem actual_norm_base_infinity_order (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    baseInfinityOrder (Algebra.norm (RatFunc K) f)=pointOrder W f 0 := by
  rw [baseInfinityOrder,actual_norm_infinity_valuation,point_order_formula]
  rfl

theorem actual_norm_upstairs_infinity_order (W : WeierstrassCurve.Affine K)
    (f : W.FunctionFieldˣ) :
    pointOrder W (embeddedNorm W f) 0=2*pointOrder W f 0 :=
  actual_norm_fixed_point_order W f 0 (by rfl)

/-- Actual ramification factor on every nonzero rational-base function at infinity. -/
theorem actual_base_infinity_pullback (W : WeierstrassCurve.Affine K) (c : (RatFunc K)ˣ) :
    pointOrder W (Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom c) 0=
      2*baseInfinityOrder (c : RatFunc K) := by
  rw [←actual_norm_base_infinity_order]
  change baseInfinityOrder (Algebra.norm (RatFunc K)
    (algebraMap (RatFunc K) W.FunctionField (c : RatFunc K)))=_
  rw [norm_base_scalar]
  simp only [baseInfinityOrder,map_pow,WithZero.log_pow]
  ring

#print axioms actual_norm_infinity_valuation
#print axioms actual_norm_base_infinity_order
#print axioms actual_base_infinity_pullback
end
end EllipticNormTransportAudit
