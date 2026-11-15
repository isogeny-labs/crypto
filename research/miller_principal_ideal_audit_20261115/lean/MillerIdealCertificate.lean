module
public import GuardControls

@[expose] public section
namespace MillerPrincipalIdealAudit
noncomputable section
open WeierstrassCurve.Affine
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing F := (inferInstance : Field F).toCommRing

/-- Affine principal-ideal/function/evaluation bridge and actual guard controls.
No assertion about the projective infinity valuation or canonical pairing is included. -/
theorem miller_affine_ideal_certificate :
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K)
      (x₁ x₂ y₁ y₂ : K) (_ : W.Nonsingular x₁ y₁) (_ : W.Nonsingular x₂ y₂),
      ¬ (x₁ = x₂ ∧ y₁ = W.negY x₂ y₂) →
      pointIdeal W x₁ y₁ * pointIdeal W x₂ y₂ *
        pointIdeal W (W.addX x₁ x₂ (W.slope x₁ x₂ y₁ y₂))
          (W.negY (W.addX x₁ x₂ (W.slope x₁ x₂ y₁ y₂))
            (W.addY x₁ x₂ y₁ (W.slope x₁ x₂ y₁ y₂))) =
        lineIdeal W x₁ y₁ (W.slope x₁ x₂ y₁ y₂)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K)
      (x y : K) (hN : W.Nonsingular x y),
      (3 : ℕ) • Point.some x y hN = 0 →
      pointIdeal W x y ^ 3 = lineIdeal W x y (W.slope x x y y)) ∧
    (∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (x y slope : K),
      lineFunction W x y slope ≠ 0) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K)
      (_ : W.a₁ = 0) (_ : W.a₂ = 0) (_ : W.a₃ = 0)
      (x y xr yr : K) (hN : W.Nonsingular x y)
      (_ : (3 : ℕ) • Point.some x y hN = 0) (hR : W.Equation xr yr),
      pointEval W xr yr hR (lineClass W x y (W.slope x x y y)) =
        yr - y - ((3 * x ^ 2 + W.a₄) / (2 * y)) * (xr - x)) ∧
    controlCurve.Δ ≠ 0 ∧
    (3 : ℕ) • Point.some 0 1 control_positive_nonsingular = 0 ∧
    pointIdeal controlCurve 0 1 ^ 3 =
      lineIdeal controlCurve 0 1 (controlCurve.slope 0 0 1 1) ∧
    controlCurve.Nonsingular 2 2 ∧
    (2 : F) ≠ controlCurve.negY 2 2 ∧
    (3 : ℕ) • Point.some 2 2 control_negative_nonsingular ≠ 0 ∧
    pointIdeal controlCurve 2 2 ^ 3 ≠
      lineIdeal controlCurve 2 2 (controlCurve.slope 2 2 2 2) := by
  refine ⟨?_, ?_, ?_, ?_, control_delta, control_positive_three,
    control_positive_ideal, control_negative_nonsingular,
    control_negative_nonvertical, control_negative_not_three, control_negative_ideal⟩
  · intro K _ _ W x₁ x₂ y₁ y₂ h₁ h₂ hxy
    exact chord_principal_ideal W h₁ h₂ hxy
  · intro K _ _ W x y hN h3
    exact tangent_three_principal_ideal W hN h3
  · intro K _ W x y slope
    exact line_function_nonzero W x y slope
  · intro K _ _ W ha1 ha2 ha3 x y xr yr hN h3 hR
    exact short_three_miller_evaluation W ha1 ha2 ha3 hN h3 hR

#print axioms miller_affine_ideal_certificate
end
end MillerPrincipalIdealAudit
