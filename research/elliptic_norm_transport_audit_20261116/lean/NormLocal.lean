import NormInfinity

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

theorem norm_local_order_zero (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (hf : pointOrder W f P=0) (hn : pointOrder W f (-P)=0) :
    pointOrder W (embeddedNorm W f) P=0 := by
  rw [actual_norm_point_order,hf,hn,add_zero]

/-- Actual local-unit norm residue, derived from both actual local unit guards. -/
theorem actual_norm_local_unit_residue (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (hf : pointOrder W f P=0) (hn : pointOrder W f (-P)=0) :
    localUnitValue W P (embeddedNorm W f) (by rw [actual_norm_point_order,hf,hn];rfl)=
      localUnitValue W P f hf*localUnitValue W (-P) f hn := by
  have hneg : pointOrder W (negatedUnit W f) P=0 := by
    change pointOrder W (fieldNeg W (f : W.FunctionField)) P=0
    rw [point_order_pullback,hn]
  have he:=local_unit_value_mul W P f (negatedUnit W f) hf hneg
  rw [local_unit_value_negation W P f hn] at he
  simpa only [embedded_norm_product] using he

/-- Balanced local words are transported by the actual local unit map. -/
theorem balanced_negation (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) :
    balancedLocalUnit W P (negatedUnit W f) (negatedUnit W g)=
      pointUnitNegation W P (balancedLocalUnit W (-P) f g) := by
  apply Subtype.ext
  change (negatedUnit W f)^pointOrder W (fieldNeg W (g : W.FunctionField)) P/
      (negatedUnit W g)^pointOrder W (fieldNeg W (f : W.FunctionField)) P=
    negatedUnit W (f^pointOrder W g (-P)/g^pointOrder W f (-P))
  rw [point_order_pullback,point_order_pullback,map_div,map_zpow,map_zpow]

/-- Actual tame symbol covariance, including its sign at common zeros and poles. -/
theorem tame_negation_covariance (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) :
    tameSymbol W P (negatedUnit W f) (negatedUnit W g)=tameSymbol W (-P) f g := by
  unfold tameSymbol
  change (-1 : Kˣ)^(pointOrder W (fieldNeg W (f : W.FunctionField)) P*
    pointOrder W (fieldNeg W (g : W.FunctionField)) P)*_= _
  rw [point_order_pullback,point_order_pullback,balanced_negation,point_unit_negation_value]

/-- Actual local norm orbit identity. At a fixed point its RHS is a square. -/
theorem actual_norm_tame_orbit (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) (hg : negatedUnit W g=g) :
    tameSymbol W P (embeddedNorm W f) g=tameSymbol W P f g*tameSymbol W (-P) f g := by
  rw [embedded_norm_product,tame_mul_left]
  have he:=tame_negation_covariance W P f g
  rw [hg] at he
  rw [he]

/-- Invariance is derived on every actual rational-base unit. -/
theorem rational_unit_negation (W : WeierstrassCurve.Affine K) (c : (RatFunc K)ˣ) :
    negatedUnit W (Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom c)=
      Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom c := by
  apply Units.ext
  exact conjugation_fixes_base W c

theorem actual_rational_norm_tame_orbit (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    tameSymbol W P (embeddedNorm W f)
      (Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom c)=
    tameSymbol W P f (Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom c)*
      tameSymbol W (-P) f (Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom c) :=
  actual_norm_tame_orbit W P f _ (rational_unit_negation W c)

#print axioms actual_norm_local_unit_residue
#print axioms tame_negation_covariance
#print axioms actual_rational_norm_tame_orbit
end
end EllipticNormTransportAudit
