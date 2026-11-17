module
public import NegationDivisor

@[expose] public section
namespace NegationPullbackAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerPrincipalIdealAudit MillerFinitePointAudit MillerNormValuationAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing F := (inferInstance : Field F).toCommRing

def controlPoint : controlCurve.Point := Point.some 0 1 control_positive_nonsingular

def controlFunction : controlCurve.FunctionField :=
  MillerNormValuationAudit.lineFunction controlCurve 0 1 (controlCurve.slope 0 0 1 1)

theorem control_function_nonzero : controlFunction ≠ 0 :=
  MillerPrincipalIdealAudit.line_function_nonzero controlCurve 0 1 _

theorem control_divisor :
    principalPointDivisor controlCurve controlFunction
      (chord_support_finite controlCurve control_positive_nonsingular control_positive_nonsingular
        (fun hn => three_tangent_nonvertical controlCurve control_positive_nonsingular
          control_positive_three hn.2)) =
      Finsupp.single controlPoint 3 - Finsupp.single 0 3 :=
  three_tangent_principal_point_divisor controlCurve control_positive_nonsingular control_positive_three

theorem control_point_ne_negative : controlPoint ≠ -controlPoint := by decide +kernel

theorem control_order_three : pointOrder controlCurve controlFunction controlPoint = 3 := by
  have he := congrArg (fun D : controlCurve.Point →₀ ℤ => D controlPoint) control_divisor
  change pointOrder controlCurve controlFunction controlPoint = _ at he
  simpa [controlPoint] using he

theorem control_negative_order_zero :
    pointOrder controlCurve controlFunction (-controlPoint) = 0 := by
  have he := congrArg (fun D : controlCurve.Point →₀ ℤ => D (-controlPoint)) control_divisor
  change pointOrder controlCurve controlFunction (-controlPoint) = _ at he
  have h0 : (0 : controlCurve.Point) ≠ -controlPoint := by
    exact (neg_ne_zero.mpr (Point.some_ne_zero control_positive_nonsingular)).symm
  simpa [control_point_ne_negative, h0] using he

/-- Actual nonzero function disproves deleting the function pullback from the source lemma. -/
theorem missing_function_pullback_counter :
    pointOrder controlCurve controlFunction controlPoint ≠
      pointOrder controlCurve controlFunction (-controlPoint) := by
  rw [control_order_three, control_negative_order_zero]
  decide

theorem control_correct_pullback :
    pointOrder controlCurve (fieldNeg controlCurve controlFunction) controlPoint = 0 := by
  rw [point_order_pullback, control_negative_order_zero]

#print axioms missing_function_pullback_counter
#print axioms control_correct_pullback
end
end NegationPullbackAudit
