module
public import LineEvaluation

@[expose] public section
namespace MillerPrincipalIdealAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev F := ZMod 5
local instance : CommRing F := (inferInstance : Field F).toCommRing

def controlCurve : WeierstrassCurve.Affine F := ⟨0, 0, 0, 0, 1⟩
theorem control_delta : controlCurve.Δ ≠ 0 := by decide +kernel

theorem control_positive_nonsingular : controlCurve.Nonsingular 0 1 := by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero control_delta).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel

theorem control_negative_nonsingular : controlCurve.Nonsingular 2 2 := by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero control_delta).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  decide +kernel

theorem control_positive_three :
    (3 : ℕ) • Point.some 0 1 control_positive_nonsingular = 0 := by decide +kernel

theorem control_negative_not_three :
    (3 : ℕ) • Point.some 2 2 control_negative_nonsingular ≠ 0 := by decide +kernel

theorem control_negative_nonvertical : (2 : F) ≠ controlCurve.negY 2 2 := by decide +kernel

theorem control_negative_slope : controlCurve.slope 2 2 2 2 = 3 := by decide +kernel

theorem control_positive_ideal :
    pointIdeal controlCurve 0 1 ^ 3 =
      lineIdeal controlCurve 0 1 (controlCurve.slope 0 0 1 1) :=
  tangent_three_principal_ideal controlCurve control_positive_nonsingular control_positive_three

/-- Removing actual three-torsion makes the tangent cube assertion false,
while nonsingularity and the nonvertical tangent guard are retained. -/
theorem control_negative_ideal :
    pointIdeal controlCurve 2 2 ^ 3 ≠
      lineIdeal controlCurve 2 2 (controlCurve.slope 2 2 2 2) := by
  let ev := pointEval controlCurve 0 1 control_positive_nonsingular.1
  have hzero : ev (lineClass controlCurve 2 2 (controlCurve.slope 2 2 2 2)) = 0 := by
    change pointEval controlCurve 0 1 control_positive_nonsingular.1 _ = 0
    rw [line_evaluation, control_negative_slope]
    decide +kernel
  have hle : lineIdeal controlCurve 2 2 (controlCurve.slope 2 2 2 2) ≤ RingHom.ker ev := by
    apply Ideal.span_le.mpr
    intro z hz
    rcases Set.mem_singleton_iff.mp hz with rfl
    exact hzero
  have hx : XClass controlCurve 2 ∈ pointIdeal controlCurve 2 2 :=
    Ideal.subset_span (by simp)
  have hc := Ideal.pow_mem_pow hx 3
  intro he
  rw [he] at hc
  have hval := hle hc
  change ev ((XClass controlCurve 2) ^ 3) = 0 at hval
  rw [map_pow] at hval
  change (pointEval controlCurve 0 1 control_positive_nonsingular.1 (XClass controlCurve 2)) ^ 3 = 0 at hval
  rw [point_eval_XClass] at hval
  exact (by decide +kernel : ((0 : F) - 2) ^ 3 ≠ 0) hval

#print axioms control_negative_ideal
#print axioms control_positive_ideal
end
end MillerPrincipalIdealAudit
