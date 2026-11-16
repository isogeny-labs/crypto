module
public import DivisorEvaluation

@[expose] public section
namespace FinitePointEvaluationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerFinitePointAudit MillerNormValuationAudit MillerInfinityChartAudit
open FunctionDivisorFinitenessAudit MillerRecursionAudit
variable {K : Type*} [Field K]

/-- A nonzero vertical function cannot be killed by a field homomorphism. -/
theorem no_total_point_evaluation (W : WeierstrassCurve.Affine K) (x : K)
    (e : W.FunctionField →+* K) (hc : ∀ c : K, e (constantHom W c) = c)
    (hx : e (xFunction W) = x) : False := by
  have hz : e (verticalFunction W x) = 0 := by
    rw [vertical_formula, map_sub, hx, hc, sub_self]
  apply vertical_nonzero W x
  apply e.injective
  rw [map_zero]
  exact hz

theorem point_u_field (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y) :
    pointToField W h (localU W h) = verticalFunction W x := by
  rw [localU, point_to_field_map]
  rfl

/-- The actual inverse of the vertical function has no representative in this local ring. -/
theorem inverse_vertical_not_local (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) :
    ¬ ∃ a : PointLocalRing W x y h, pointToField W h a = (verticalFunction W x)⁻¹ := by
  rintro ⟨a, ha⟩
  have he : localU W h * a = 1 := by
    apply point_to_field_injective W h
    rw [map_mul, point_u_field, ha, map_one]
    exact mul_inv_cancel₀ (vertical_nonzero W x)
  have hv := congrArg (localPointEval W h) he
  rw [map_mul, local_point_u_zero, zero_mul, map_one] at hv
  exact zero_ne_one hv

theorem inverse_vertical_has_pole (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) : 1 < finiteAdic W h (verticalFunction W x)⁻¹ := by
  apply lt_of_not_ge
  intro hv
  obtain ⟨a, ha⟩ := local_to_finite_surjective W h ⟨(verticalFunction W x)⁻¹, hv⟩
  exact inverse_vertical_not_local W h.1 ⟨a, congrArg Subtype.val ha⟩

theorem vertical_regular_value_zero (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) :
    finiteRegularEval W h (finiteLocalEquiv W h (localU W h.1)) = 0 := by
  rw [finite_regular_eval_local, local_point_u_zero]

/-- A deliberately totalized singleton-supported product; only that support is used in controls. -/
def totalizedPointDivisorValue (W : WeierstrassCurve.Affine K) (P : W.Point) (z : K)
    (D : W.Point →₀ ℤ) : K := by
  classical
  exact D.prod fun Q n => if Q = P then z ^ n else 1

theorem totalized_single (W : WeierstrassCurve.Affine K) (P : W.Point) (z : K) (n : ℤ) :
    totalizedPointDivisorValue W P z (Finsupp.single P n) = z ^ n := by
  classical
  rw [totalizedPointDivisorValue, Finsupp.prod_single_index]
  · simp
  · simp

theorem totalized_add_failure (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) :
    totalizedPointDivisorValue W (.some x y h) (localPointEval W h.1 (localU W h.1))
        (Finsupp.single (.some x y h) 1 + Finsupp.single (.some x y h) (-1)) ≠
      totalizedPointDivisorValue W (.some x y h) (localPointEval W h.1 (localU W h.1))
        (Finsupp.single (.some x y h) 1) *
      totalizedPointDivisorValue W (.some x y h) (localPointEval W h.1 (localU W h.1))
        (Finsupp.single (.some x y h) (-1)) := by
  classical
  have hc : (Finsupp.single (.some x y h) 1 + Finsupp.single (.some x y h) (-1) :
      W.Point →₀ ℤ) = 0 := by
    rw [← Finsupp.single_add]
    simp
  rw [hc, totalized_single, totalized_single]
  simp [totalizedPointDivisorValue, local_point_u_zero]

local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
local instance : CommRing F3 := (inferInstance : Field F3).toCommRing

theorem control_nonsingular : rationalCurve.Nonsingular (0 : F3) 1 := by
  apply (equation_iff_nonsingular_of_Δ_ne_zero rational_delta).mp
  rw [equation_iff]
  decide +kernel

def controlPoint : rationalCurve.Point := .some 0 1 control_nonsingular

def controlDivisor : rationalCurve.Point →₀ ℤ := Finsupp.single controlPoint 1

def controlPrincipalDivisor : rationalCurve.Point →₀ ℤ :=
  functionDivisor rationalCurve (verticalFunction rationalCurve 0) (vertical_nonzero rationalCurve 0)

theorem control_point_order : pointOrder rationalCurve (verticalFunction rationalCurve 0) controlPoint = 1 := by
  rw [vertical_point_order rationalCurve control_nonsingular]
  change verticalDivisor rationalCurve controlPoint controlPoint = 1
  have h0 : controlPoint ≠ 0 := Point.some_ne_zero control_nonsingular
  have hn : -controlPoint ≠ controlPoint := by decide +kernel
  simp [verticalDivisor, h0, hn]

theorem control_infinity_order : pointOrder rationalCurve (verticalFunction rationalCurve 0) 0 = -2 := by
  change -WithZero.log (chartAdicValuation rationalCurve (verticalFunction rationalCurve 0)) = -2
  rw [chart_adic_vertical, WithZero.log_exp]

/-- Unequal support sets are insufficient: the singleton intersects the actual principal support. -/
theorem control_support_overlap : ¬ Disjoint controlDivisor.support controlPrincipalDivisor.support := by
  intro hd
  have hpD : controlPoint ∈ controlDivisor.support := by simp [controlDivisor]
  have hpF : controlPoint ∈ controlPrincipalDivisor.support := by
    simp only [controlPrincipalDivisor, Finsupp.mem_support_iff, function_divisor_apply,
      control_point_order, ne_eq, one_ne_zero, not_false_eq_true]
  exact Finset.disjoint_left.mp hd hpD hpF

theorem control_support_unequal : controlDivisor.support ≠ controlPrincipalDivisor.support := by
  intro he
  have hi : (0 : rationalCurve.Point) ∈ controlPrincipalDivisor.support := by
    simp only [controlPrincipalDivisor, Finsupp.mem_support_iff, function_divisor_apply,
      control_infinity_order]
    norm_num
  have h0 : controlPoint ≠ 0 := Point.some_ne_zero control_nonsingular
  have hz : (0 : rationalCurve.Point) ∉ controlDivisor.support := by
    simp [controlDivisor, Ne.symm h0]
  exact hz (he.symm ▸ hi)

theorem control_inverse_pole : 1 < finiteAdic rationalCurve control_nonsingular
    (verticalFunction rationalCurve 0)⁻¹ := inverse_vertical_has_pole rationalCurve control_nonsingular

#print axioms no_total_point_evaluation
#print axioms inverse_vertical_not_local
#print axioms inverse_vertical_has_pole
#print axioms totalized_add_failure
#print axioms control_support_overlap
#print axioms control_support_unequal
end
end FinitePointEvaluationAudit
