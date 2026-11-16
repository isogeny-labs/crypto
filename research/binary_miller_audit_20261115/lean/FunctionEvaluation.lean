module
public import FiniteRegular

@[expose] public section
namespace FinitePointEvaluationAudit
noncomputable section
open MillerFinitePointAudit MillerInfinityChartAudit MillerNormValuationAudit
open scoped WithZero
variable {K : Type*} [Field K]

def infinityRegularEval (W : WeierstrassCurve.Affine K) : valuationRing W →+* K :=
  (localOriginEval W).comp (chartLocalEquiv W).symm.toRingHom

theorem infinity_regular_surjective (W : WeierstrassCurve.Affine K) :
    Function.Surjective (infinityRegularEval W) :=
  (local_origin_surjective W).comp (chartLocalEquiv W).symm.surjective

/-- Actual normalized local valuation, using the proved norm/official-adic equality at infinity. -/
def localFunctionValuation (W : WeierstrassCurve.Affine K) :
    W.Point → Valuation W.FunctionField ℤᵐ⁰
  | .zero => functionValuation W
  | .some _ _ h => finiteAdic W h

abbrev RegularRing (W : WeierstrassCurve.Affine K) (P : W.Point) :=
  (localFunctionValuation W P).valuationSubring

/-- Actual residue evaluation is defined on the actual regular-function ring only. -/
def regularEvaluation (W : WeierstrassCurve.Affine K) (P : W.Point) : RegularRing W P →+* K :=
  match P with
  | .zero => infinityRegularEval W
  | .some _ _ h => finiteRegularEval W h

theorem regular_evaluation_surjective (W : WeierstrassCurve.Affine K) (P : W.Point) :
    Function.Surjective (regularEvaluation W P) := by
  cases P with
  | zero => exact infinity_regular_surjective W
  | some _ _ h => exact finite_regular_surjective W h

theorem regular_evaluation_kernel (W : WeierstrassCurve.Affine K) (P : W.Point) :
    RingHom.ker (regularEvaluation W P) = IsLocalRing.maximalIdeal (RegularRing W P) :=
  IsLocalRing.eq_maximalIdeal
    (RingHom.ker_isMaximal_of_surjective _ (regular_evaluation_surjective W P))

theorem regular_evaluation_value_iff (W : WeierstrassCurve.Affine K) (P : W.Point)
    (a : RegularRing W P) : regularEvaluation W P a ≠ 0 ↔ localFunctionValuation W P a = 1 := by
  have hz : regularEvaluation W P a = 0 ↔ a ∈ IsLocalRing.maximalIdeal (RegularRing W P) := by
    change a ∈ RingHom.ker (regularEvaluation W P) ↔ _
    rw [regular_evaluation_kernel]
  rw [ne_eq, hz, Valuation.mem_maximalIdeal_iff]
  constructor
  · intro hn
    exact le_antisymm a.property (le_of_not_gt hn)
  · intro he
    simp only [he, lt_self_iff_false, not_false_eq_true]

theorem point_order_formula (W : WeierstrassCurve.Affine K) (f : W.FunctionField) (P : W.Point) :
    pointOrder W f P = -WithZero.log (localFunctionValuation W P f) := by
  cases P with
  | zero =>
    change -WithZero.log (chartAdicValuation W f) = -WithZero.log (functionValuation W f)
    rw [chart_adic_eq_norm]
  | some _ _ _ => rfl

theorem point_order_zero_iff_value_one (W : WeierstrassCurve.Affine K)
    (f : W.FunctionField) (hf : f ≠ 0) (P : W.Point) :
    pointOrder W f P = 0 ↔ localFunctionValuation W P f = 1 := by
  have hv : localFunctionValuation W P f ≠ 0 := (Valuation.ne_zero_iff _).mpr hf
  rw [point_order_formula, neg_eq_zero]
  constructor
  · intro hz
    rw [← WithZero.exp_log hv, hz, WithZero.exp_zero]
  · intro he
    rw [he, WithZero.log_one]

#print axioms regular_evaluation_kernel
#print axioms regular_evaluation_value_iff
#print axioms point_order_zero_iff_value_one
end
end FinitePointEvaluationAudit
