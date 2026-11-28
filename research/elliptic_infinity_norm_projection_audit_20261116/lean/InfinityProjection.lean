import InfinityRatio

namespace EllipticInfinityNormProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

def baseFieldUnit (W : WeierstrassCurve.Affine K) : (RatFunc K)ˣ →* W.FunctionFieldˣ :=
  Units.map (algebraMap (RatFunc K) W.FunctionField).toMonoidHom

theorem norm_unit_base_order (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ) :
    baseInfinityOrder (normUnit W f : RatFunc K)=pointOrder W f 0 :=
  actual_norm_base_infinity_order W f

theorem base_field_unit_order (W : WeierstrassCurve.Affine K) (c : (RatFunc K)ˣ) :
    pointOrder W (baseFieldUnit W c) 0=2*baseInfinityOrder (c : RatFunc K) :=
  actual_base_infinity_pullback W c

theorem base_field_norm_unit (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ) :
    baseFieldUnit W (normUnit W f)=embeddedNorm W f := rfl

/-- Group arithmetic used only after both sides have been actualized in the actual local unit group. -/
theorem balanced_ratio_word {G : Type*} [CommGroup G] (f h c : G) (n m : ℤ) :
    (f*h)^m/c^n=(f^(2*m)/c^n)*(h/f)^m := by
  have ha (a b d : G) : a*b/d=(a*a/d)*(b/a) := by
    rw [div_mul_div_comm]
    rw [show a*a*b=(a*b)*a by ac_rfl,mul_div_mul_right_eq_div]
  have hp : f^(2*m)=f^m*f^m := by rw [two_mul,zpow_add]
  rw [mul_zpow,div_zpow,hp]
  exact ha _ _ _

/-- Exact comparison of two actual balanced local units, not an equality of squares. -/
theorem actual_projection_balanced (W : WeierstrassCurve.Affine K)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    basePointUnit W (baseBalancedLocalUnit (normUnit W f) c)=
      balancedLocalUnit W 0 f (baseFieldUnit W c)*(ratioLocalUnit W f)^baseInfinityOrder (c : RatFunc K) := by
  apply Subtype.ext
  change baseFieldUnit W (baseBalancedUnit (normUnit W f) c)=
    (f^pointOrder W (baseFieldUnit W c) 0/(baseFieldUnit W c)^pointOrder W f 0)*
      (negatedUnit W f/f)^baseInfinityOrder (c : RatFunc K)
  rw [baseBalancedUnit,map_div,map_zpow,map_zpow,norm_unit_base_order,
    base_field_unit_order,base_field_norm_unit,embedded_norm_product]
  exact balanced_ratio_word f (negatedUnit W f) (baseFieldUnit W c)
    (pointOrder W f 0) (baseInfinityOrder (c : RatFunc K))

/-- Actual infinity norm/tame projection. The exact actual conjugation sign supplies the
missing information beyond the previously proved norm/square identity. -/
theorem actual_infinity_norm_projection (W : WeierstrassCurve.Affine K)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    baseTameSymbol W (normUnit W f) c=tameSymbol W 0 f (baseFieldUnit W c) := by
  let n:=pointOrder W f 0
  let m:=baseInfinityOrder (c : RatFunc K)
  rw [baseTameSymbol,norm_unit_base_order,base_unit_evaluation_compatibility,
    actual_projection_balanced,map_mul,map_zpow,tameSymbol,base_field_unit_order]
  change (-1 : Kˣ)^(n*m)*
    (pointUnitEvaluation W 0 (balancedLocalUnit W 0 f (baseFieldUnit W c))*(inertiaCharacter W f)^m)=
      (-1 : Kˣ)^(n*(2*m))*pointUnitEvaluation W 0 (balancedLocalUnit W 0 f (baseFieldUnit W c))
  have hc : (inertiaCharacter W f)^m=(-1 : Kˣ)^(n*m) := by
    rw [actual_inertia_character,←zpow_mul]
  rw [hc]
  have hs (k : ℤ) : (-1 : Kˣ)^(2*k)=1 := by
    rw [zpow_mul]
    norm_num
  have he : n*(2*m)=2*(n*m) := by ring
  have hh : (-1 : Kˣ)^(n*m)*(-1 : Kˣ)^(n*m)=1 := by
    rw [←zpow_add,show n*m+n*m=2*(n*m) by ring,hs]
  rw [he,hs,one_mul]
  calc
    _ = ((-1 : Kˣ)^(n*m)*(-1 : Kˣ)^(n*m))*
      pointUnitEvaluation W 0 (balancedLocalUnit W 0 f (baseFieldUnit W c)) := by ac_rfl
    _ = _ := by rw [hh,one_mul]

/-- Corrected inverse convention inherits the same actual projection. -/
theorem actual_infinity_inverse_projection (W : WeierstrassCurve.Affine K)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    (baseTameSymbol W (normUnit W f) c)⁻¹=milnorSymbol W 0 f (baseFieldUnit W c) := by
  rw [actual_infinity_norm_projection,milnorSymbol]

#print axioms actual_projection_balanced
#print axioms actual_infinity_norm_projection
#print axioms actual_infinity_inverse_projection
end
end EllipticInfinityNormProjectionAudit
