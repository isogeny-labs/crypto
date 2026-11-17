module
public import LocalOrderAlgebra

@[expose] public section
namespace MillerRecursionAudit
noncomputable section
open WeierstrassCurve.Affine MillerFinitePointAudit
variable {K : Type*} [Field K] [DecidableEq K]

def additionDivisor (W : WeierstrassCurve.Affine K) (P Q : W.Point) : W.Point →₀ ℤ :=
  Finsupp.single P 1 + Finsupp.single Q 1 - Finsupp.single (P + Q) 1 - Finsupp.single 0 1

theorem addition_divisor_zero_left (W : WeierstrassCurve.Affine K) (Q : W.Point) :
    additionDivisor W 0 Q = 0 := by
  simp [additionDivisor]

theorem addition_divisor_zero_right (W : WeierstrassCurve.Affine K) (P : W.Point) :
    additionDivisor W P 0 = 0 := by
  simp [additionDivisor]

theorem addition_divisor_of_sum_zero (W : WeierstrassCurve.Affine K) {P Q : W.Point}
    (h : P + Q = 0) : additionDivisor W P Q = verticalDivisor W P := by
  have hq : Q = -P := eq_neg_iff_add_eq_zero.mpr (by rwa [add_comm])
  simp only [additionDivisor, verticalDivisor, hq]
  have ho : (Finsupp.single (0 : W.Point) (2 : ℤ)) =
      Finsupp.single 0 1 + Finsupp.single 0 1 := by rw [← Finsupp.single_add]; norm_num
  rw [ho]
  abel_nf

theorem chord_minus_vertical (W : WeierstrassCurve.Affine K) (P Q : W.Point) :
    chordDivisor W P Q - verticalDivisor W (P + Q) = additionDivisor W P Q := by
  simp only [chordDivisor, verticalDivisor, additionDivisor]
  have htwo : (Finsupp.single (0 : W.Point) (2 : ℤ)) =
      Finsupp.single 0 1 + Finsupp.single 0 1 := by rw [← Finsupp.single_add]; norm_num
  have hthree : (Finsupp.single (0 : W.Point) (3 : ℤ)) =
      Finsupp.single 0 1 + Finsupp.single 0 1 + Finsupp.single 0 1 := by
    rw [← Finsupp.single_add, ← Finsupp.single_add]
    norm_num
  rw [htwo, hthree]
  abel

/-- Actual source addition-step function, including both zero-point and vertical exceptions. -/
def stepFunction (W : WeierstrassCurve.Affine K) : W.Point → W.Point → W.FunctionField
  | .zero, _ => 1
  | .some _ _ _, .zero => 1
  | .some x₁ y₁ _, .some x₂ y₂ _ =>
    if x₁ = x₂ ∧ y₁ = W.negY x₂ y₂ then MillerNormValuationAudit.verticalFunction W x₁
    else MillerNormValuationAudit.lineFunction W x₁ y₁ (W.slope x₁ x₂ y₁ y₂) /
      MillerNormValuationAudit.verticalFunction W (W.addX x₁ x₂ (W.slope x₁ x₂ y₁ y₂))

/-- The actual function has divisor (P)+(Q)-(P+Q)-(O) for every actual point pair. -/
theorem step_has_point_divisor (W : WeierstrassCurve.Affine K) (P Q : W.Point) :
    HasPointDivisor W (stepFunction W P Q) (additionDivisor W P Q) := by
  cases P with
  | zero =>
    change HasPointDivisor W 1 (additionDivisor W 0 Q)
    rw [addition_divisor_zero_left]
    exact has_point_divisor_one W
  | some x₁ y₁ h₁ =>
    cases Q with
    | zero =>
      change HasPointDivisor W 1 (additionDivisor W (Point.some x₁ y₁ h₁) 0)
      rw [addition_divisor_zero_right]
      exact has_point_divisor_one W
    | some x₂ y₂ h₂ =>
      by_cases hxy : x₁ = x₂ ∧ y₁ = W.negY x₂ y₂
      · simp only [stepFunction, ite_eq_left hxy]
        rw [addition_divisor_of_sum_zero W (Point.add_of_Y_eq hxy.1 hxy.2)]
        exact has_point_divisor_vertical W h₁
      · simp only [stepFunction, ite_eq_right hxy]
        have hs := W.nonsingular_add h₁ h₂ hxy
        have hd := (has_point_divisor_chord W h₁ h₂ hxy).div W
          (has_point_divisor_vertical W hs)
        rw [← Point.add_some (h₁ := h₁) (h₂ := h₂) hxy, chord_minus_vertical] at hd
        exact hd

theorem step_function_nonzero (W : WeierstrassCurve.Affine K) (P Q : W.Point) :
    stepFunction W P Q ≠ 0 := (step_has_point_divisor W P Q).1

#print axioms step_has_point_divisor
end
end MillerRecursionAudit
