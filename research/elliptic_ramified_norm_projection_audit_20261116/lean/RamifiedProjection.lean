import RamifiedSymbol

namespace EllipticRamifiedNormProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

theorem finite_base_field_unit_order (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) (c : (RatFunc K)ˣ) :
    pointOrder W (baseFieldUnit W c) (Point.some x y h)=2*finiteBaseOrder x (c : RatFunc K) :=
  fixed_base_order W h hP c

/-- Exact comparison of the two actual proper balanced local units. -/
theorem actual_ramified_projection_balanced (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    finiteBaseUnitEmbedding W h hP (finiteBaseBalancedLocalUnit x (normUnit W f) c)=
      balancedLocalUnit W (Point.some x y h) f (baseFieldUnit W c)*
        (fixedRatioLocalUnit W (Point.some x y h) hP f)^finiteBaseOrder x (c : RatFunc K) := by
  apply Subtype.ext
  change baseFieldUnit W (finiteBaseBalancedUnit x (normUnit W f) c)=
    (f^pointOrder W (baseFieldUnit W c) (Point.some x y h)/
      (baseFieldUnit W c)^pointOrder W f (Point.some x y h))*
      (negatedUnit W f/f)^finiteBaseOrder x (c : RatFunc K)
  rw [finiteBaseBalancedUnit,map_div,map_zpow,map_zpow,fixed_norm_base_order W h hP,
    finite_base_field_unit_order W h hP,base_field_norm_unit,embedded_norm_product]
  exact balanced_ratio_word f (negatedUnit W f) (baseFieldUnit W c)
    (pointOrder W f (Point.some x y h)) (finiteBaseOrder x (c : RatFunc K))

/-- Normalized finite ramified norm/tame projection, proved without cancelling symbol squares. -/
theorem actual_ramified_norm_projection (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    finiteBaseTameSymbol W h hP (normUnit W f) c=tameSymbol W (Point.some x y h) f (baseFieldUnit W c) := by
  let n:=pointOrder W f (Point.some x y h)
  let m:=finiteBaseOrder x (c : RatFunc K)
  rw [finiteBaseTameSymbol,fixed_norm_base_order W h hP,finite_base_unit_eval_compatibility,
    actual_ramified_projection_balanced,map_mul,map_zpow,tameSymbol,finite_base_field_unit_order W h hP]
  change (-1 : Kˣ)^(n*m)*
    (pointUnitEvaluation W (Point.some x y h) (balancedLocalUnit W (Point.some x y h) f (baseFieldUnit W c))*
      (fixedInertiaCharacter W (Point.some x y h) hP f)^m)=
      (-1 : Kˣ)^(n*(2*m))*pointUnitEvaluation W (Point.some x y h)
        (balancedLocalUnit W (Point.some x y h) f (baseFieldUnit W c))
  have hc : (fixedInertiaCharacter W (Point.some x y h) hP f)^m=(-1 : Kˣ)^(n*m) := by
    rw [actual_fixed_inertia_character,←zpow_mul]
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
      pointUnitEvaluation W (Point.some x y h) (balancedLocalUnit W (Point.some x y h) f (baseFieldUnit W c)) := by ac_rfl
    _ = _ := by rw [hh,one_mul]

theorem actual_ramified_inverse_projection (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    (finiteBaseTameSymbol W h hP (normUnit W f) c)⁻¹=milnorSymbol W (Point.some x y h) f (baseFieldUnit W c) := by
  rw [actual_ramified_norm_projection,milnorSymbol]

/-- The norm of the actual simple parameter separates normalized from raw restricted symbols. -/
theorem actual_ramified_norm_parameter_values (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) :
    finiteBaseOrder x (normUnit W (finiteParameterUnit W y) : RatFunc K)=1 ∧
    finiteBaseTameSymbol W h hP (normUnit W (finiteParameterUnit W y)) (normUnit W (finiteParameterUnit W y))= -1 ∧
    tameSymbol W (Point.some x y h) (finiteParameterUnit W y) (embeddedNorm W (finiteParameterUnit W y))= -1 ∧
    pointBaseTameSymbol W (Point.some x y h) (normUnit W (finiteParameterUnit W y))
      (normUnit W (finiteParameterUnit W y))=1 := by
  have ho : finiteBaseOrder x (normUnit W (finiteParameterUnit W y) : RatFunc K)=1 := by
    rw [fixed_norm_base_order W h hP,fixed_parameter_order W h hP]
  have hb : finiteBaseTameSymbol W h hP (normUnit W (finiteParameterUnit W y))
      (normUnit W (finiteParameterUnit W y))= -1 := by
    rw [finite_base_tame_self,ho]
    norm_num
  have ht : tameSymbol W (Point.some x y h) (finiteParameterUnit W y)
      (embeddedNorm W (finiteParameterUnit W y))= -1 := by
    rw [←base_field_norm_unit,←actual_ramified_norm_projection W h hP]
    exact hb
  refine ⟨ho,hb,ht,?_⟩
  rw [point_base_norm_orbit,←hP,base_field_norm_unit,ht]
  norm_num

#print axioms actual_ramified_projection_balanced
#print axioms actual_ramified_norm_projection
#print axioms actual_ramified_inverse_projection
#print axioms actual_ramified_norm_parameter_values
end
end EllipticRamifiedNormProjectionAudit
