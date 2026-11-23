module
public import InfinityMonic

@[expose] public section
namespace MillerRecursionAudit
noncomputable section
open WeierstrassCurve.Affine MillerFinitePointAudit MillerPrincipalIdealAudit
open MillerNormValuationAudit MillerInfinityChartAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing F := (inferInstance : Field F).toCommRing

def twoPoint : controlCurve.Point := Point.some 4 0 control_two_nonsingular

theorem two_point_nonzero : twoPoint ≠ 0 := Point.some_ne_zero control_two_nonsingular

theorem two_point_double : twoPoint + twoPoint = 0 :=
  Point.add_self_of_Y_eq control_two_neg.symm

theorem two_point_step : stepFunction controlCurve twoPoint twoPoint =
    verticalFunction controlCurve 4 := by
  change (if (4 : F) = 4 ∧ (0 : F) = controlCurve.negY 4 0 then _ else _) = _
  rw [ite_eq_left ⟨rfl, control_two_neg.symm⟩]

theorem two_point_step_infinity_order :
    pointOrder controlCurve (stepFunction controlCurve twoPoint twoPoint) 0 = (-2 : ℤ) := by
  rw [two_point_step]
  change -WithZero.log (chartAdicValuation controlCurve _) = _
  rw [chart_adic_vertical, WithZero.log_exp]

/-- Treating a vertical tangent as an ordinary line violates the actual divisor formula.
The source explicitly asks for this exceptional branch; its qualified algorithm survives. -/
theorem unsafe_two_point_chord_divisor :
    ¬ HasPointDivisor controlCurve
      (MillerNormValuationAudit.lineFunction controlCurve 4 0 (controlCurve.slope 4 4 0 0))
      (chordDivisor controlCurve twoPoint twoPoint) := by
  intro h
  have ho : pointOrder controlCurve
      (MillerNormValuationAudit.lineFunction controlCurve 4 0 (controlCurve.slope 4 4 0 0)) 0 =
      (-3 : ℤ) := by
    change -WithZero.log (chartAdicValuation controlCurve _) = _
    rw [chart_adic_line, WithZero.log_exp]
  have hv := h.2 0
  rw [ho] at hv
  simp [chordDivisor, two_point_double, two_point_nonzero, eq_comm] at hv

#print axioms unsafe_two_point_chord_divisor
end
end MillerRecursionAudit
