module
public import BasechangePoints

@[expose] public section
namespace MillerBasechangeAudit
noncomputable section
open WeierstrassCurve.Affine MillerRecursionAudit
variable {K L : Type*} [Field K] [Field L] [Algebra K L] [DecidableEq K] [DecidableEq L]

theorem field_base_step (W : WeierstrassCurve.Affine K) (P Q : W.Point) :
    fieldBaseChange (L := L) W (stepFunction W P Q) =
      stepFunction (extendedCurve (L := L) W)
        (pointBaseChange (L := L) W P) (pointBaseChange (L := L) W Q) := by
  cases P with
  | zero =>
    change fieldBaseChange (L := L) W 1 = 1
    exact map_one _
  | some x₁ y₁ h₁ =>
    cases Q with
    | zero =>
      change fieldBaseChange (L := L) W 1 = 1
      exact map_one _
    | some x₂ y₂ h₂ =>
      rw [point_base_some, point_base_some]
      have hc : (algebraMap K L x₁ = algebraMap K L x₂ ∧
          algebraMap K L y₁ = (extendedCurve (L := L) W).negY (algebraMap K L x₂) (algebraMap K L y₂)) ↔
          (x₁ = x₂ ∧ y₁ = W.negY x₂ y₂) := by
        rw [WeierstrassCurve.Affine.map_negY (W' := W) (algebraMap K L)]
        exact and_congr (algebraMap K L).injective.eq_iff (algebraMap K L).injective.eq_iff
      by_cases hxy : x₁ = x₂ ∧ y₁ = W.negY x₂ y₂
      · simp only [stepFunction, ite_eq_left hxy, ite_eq_left (hc.mpr hxy), field_base_vertical]
      · simp only [stepFunction, ite_eq_right hxy, ite_eq_right (fun h => hxy (hc.mp h)),
          map_div₀, field_base_line, field_base_vertical]
        rw [WeierstrassCurve.Affine.map_slope, WeierstrassCurve.Affine.map_addX]

/-- The actual chosen recursive functions commute with the actual field/point embeddings. -/
theorem miller_function_basechange (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) :
    fieldBaseChange (L := L) W (millerFunction W P n) =
      millerFunction (extendedCurve (L := L) W) (pointBaseChange (L := L) W P) n := by
  induction n with
  | zero => simp only [millerFunction, map_one]
  | succ n ih =>
    simp only [millerFunction, map_mul, ih, field_base_step, point_base_nsmul]

#print axioms field_base_step
#print axioms miller_function_basechange
end
end MillerBasechangeAudit
