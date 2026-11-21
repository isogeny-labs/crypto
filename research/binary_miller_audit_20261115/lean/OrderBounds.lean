module
public import GeometricDegree

@[expose] public section
namespace GeometricPrincipalDivisorAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit
variable {K : Type*} [Field K]
open scoped WithZero

/-- The actual previously constructed normalized local valuation at each actual point. -/
def pointValuation (W : WeierstrassCurve.Affine K) : W.Point → Valuation W.FunctionField ℤᵐ⁰
  | .zero => chartAdicValuation W
  | .some _ _ h => finiteAdic W h

theorem point_order_valuation (W : WeierstrassCurve.Affine K) (f : W.FunctionField) (P : W.Point) :
    pointOrder W f P = -WithZero.log (pointValuation W P f) := by cases P <;> rfl

theorem point_order_nonnegative_iff (W : WeierstrassCurve.Affine K) {f : W.FunctionField}
    (hf : f ≠ 0) (P : W.Point) : 0 ≤ pointOrder W f P ↔ pointValuation W P f ≤ 1 := by
  rw [point_order_valuation, neg_nonneg,
    WithZero.log_le_iff_le_exp ((pointValuation W P).ne_zero_iff.mpr hf), WithZero.exp_zero]

theorem point_order_positive_iff (W : WeierstrassCurve.Affine K) {f : W.FunctionField}
    (hf : f ≠ 0) (P : W.Point) : 0 < pointOrder W f P ↔ pointValuation W P f < 1 := by
  rw [point_order_valuation, neg_pos,
    WithZero.log_lt_iff_lt_exp ((pointValuation W P).ne_zero_iff.mpr hf), WithZero.exp_zero]

theorem constant_point_value_le (W : WeierstrassCurve.Affine K) (c : K) (P : W.Point) :
    pointValuation W P (constantHom W c) ≤ 1 := by
  by_cases hc : c = 0
  · simp [hc]
  · exact (point_order_nonnegative_iff W (constant_function_nonzero W hc) P).mp
      (by rw [constant_point_order W hc P])

def NoPoles (W : WeierstrassCurve.Affine K) (f : W.FunctionField) : Prop :=
  ∀ P : W.Point, 0 ≤ pointOrder W f P

theorem no_poles_sub_constant (W : WeierstrassCurve.Affine K) {f : W.FunctionField}
    (hf : f ≠ 0) (hn : NoPoles W f) (c : K) (hg : f - constantHom W c ≠ 0) :
    NoPoles W (f - constantHom W c) := by
  intro P
  apply (point_order_nonnegative_iff W hg P).mpr
  exact (pointValuation W P).map_sub_le
    ((point_order_nonnegative_iff W hf P).mp (hn P)) (constant_point_value_le W c P)

#print axioms no_poles_sub_constant
end
end GeometricPrincipalDivisorAudit
