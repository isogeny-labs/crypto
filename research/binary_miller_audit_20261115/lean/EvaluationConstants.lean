module
public import FunctionEvaluation

@[expose] public section
namespace FinitePointEvaluationAudit
noncomputable section
open MillerFinitePointAudit MillerInfinityChartAudit MillerNormValuationAudit
variable {K : Type*} [Field K]

theorem local_constant_integral (W : WeierstrassCurve.Affine K) (P : W.Point) (c : K) :
    localFunctionValuation W P (constantHom W c) ≤ 1 := by
  cases P with
  | zero => exact function_valuation_constant_le W c
  | some x y h =>
    have hi := finite_local_integral W h
      (algebraMap W.CoordinateRing (PointLocalRing W x y h.1)
        (algebraMap K W.CoordinateRing c))
    rw [point_to_field_map] at hi
    exact hi

/-- Actual coefficient embedding into each actual regular-function ring. -/
def regularConstant (W : WeierstrassCurve.Affine K) (P : W.Point) : K →+* RegularRing W P where
  toFun c := ⟨constantHom W c, local_constant_integral W P c⟩
  map_zero' := Subtype.ext (map_zero _)
  map_one' := Subtype.ext (map_one _)
  map_add' a b := Subtype.ext (map_add _ a b)
  map_mul' a b := Subtype.ext (map_mul _ a b)

theorem regular_constant_coe (W : WeierstrassCurve.Affine K) (P : W.Point) (c : K) :
    (regularConstant W P c : W.FunctionField) = constantHom W c := rfl

theorem regular_eval_constant (W : WeierstrassCurve.Affine K) (P : W.Point) (c : K) :
    regularEvaluation W P (regularConstant W P c) = c := by
  cases P with
  | zero =>
    let a : ChartLocalRing W := algebraMap (ChartRing W) (ChartLocalRing W)
      (algebraMap K (ChartRing W) c)
    have he : regularConstant W .zero c = chartLocalEquiv W a := by
      apply Subtype.ext
      change constantHom W c = localToField W a
      rw [local_to_field_map, chart_to_field_constant]
    change infinityRegularEval W (regularConstant W .zero c) = c
    rw [he]
    change localOriginEval W ((chartLocalEquiv W).symm (chartLocalEquiv W a)) = c
    rw [RingEquiv.symm_apply_apply]
    exact local_origin_constant W c
  | some x y h =>
    let a : PointLocalRing W x y h.1 :=
      algebraMap W.CoordinateRing (PointLocalRing W x y h.1)
        (algebraMap K W.CoordinateRing c)
    have he : regularConstant W (.some x y h) c = finiteLocalEquiv W h a := by
      apply Subtype.ext
      change constantHom W c = pointToField W h.1 a
      rw [point_to_field_map]
      rfl
    change finiteRegularEval W h (regularConstant W (.some x y h) c) = c
    rw [he, finite_regular_eval_local]
    exact local_point_constant W h.1 c

theorem local_constant_value (W : WeierstrassCurve.Affine K) (P : W.Point)
    {c : K} (hc : c ≠ 0) : localFunctionValuation W P (constantHom W c) = 1 := by
  apply (regular_evaluation_value_iff W P (regularConstant W P c)).mp
  rw [regular_eval_constant]
  exact hc

#print axioms regular_eval_constant
#print axioms local_constant_value
end
end FinitePointEvaluationAudit
