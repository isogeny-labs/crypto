module
public import FiniteControls

@[expose] public section
namespace MillerFinitePointAudit
noncomputable section
open WeierstrassCurve.Affine MillerPrincipalIdealAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing F := (inferInstance : Field F).toCommRing

/-- Actual finite-point local DVRs/full function fields, actual line/vertical/three-torsion
principal point divisors, and actual characteristic-five guard countercontrols.
For a smooth elliptic curve over an algebraically closed field the points are all geometric points.
No separate Mathlib projective Scheme.localRing/Divisor interoperability or pairing is asserted. -/
theorem miller_finite_point_certificate :
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K),
      (∀ (x y : K) (h : W.Nonsingular x y),
        IsDiscreteValuationRing (PointLocalRing W x y h.1) ∧
        IsFractionRing (PointLocalRing W x y h.1) W.FunctionField ∧
        IsLocalRing.maximalIdeal (PointLocalRing W x y h.1) = Ideal.span {pointParameter W h}) ∧
      (∀ (x₁ y₁ x₂ y₂ : K) (h₁ : W.Nonsingular x₁ y₁) (h₂ : W.Nonsingular x₂ y₂)
        (hxy : ¬ (x₁ = x₂ ∧ y₁ = W.negY x₂ y₂)),
        principalPointDivisor W
          (MillerNormValuationAudit.lineFunction W x₁ y₁ (W.slope x₁ x₂ y₁ y₂))
          (chord_support_finite W h₁ h₂ hxy) =
          chordDivisor W (Point.some x₁ y₁ h₁) (Point.some x₂ y₂ h₂)) ∧
      (∀ (x y : K) (h : W.Nonsingular x y),
        principalPointDivisor W (MillerNormValuationAudit.verticalFunction W x)
          (vertical_support_finite W h) = verticalDivisor W (Point.some x y h)) ∧
      (∀ (x y : K) (h : W.Nonsingular x y) (h3 : (3 : ℕ) • Point.some x y h = 0),
        principalPointDivisor W (MillerNormValuationAudit.lineFunction W x y (W.slope x x y y))
          (chord_support_finite W h h (fun hn => three_tangent_nonvertical W h h3 hn.2)) =
          Finsupp.single (Point.some x y h) 3 - Finsupp.single 0 3)) ∧
    controlCurve.Δ ≠ 0 ∧
    IsLocalRing.maximalIdeal (PointLocalRing controlCurve 4 0 control_two_nonsingular.1) ≠
      Ideal.span {localU controlCurve control_two_nonsingular.1} ∧
    -WithZero.log (finiteAdic controlCurve control_negative_nonsingular
      (MillerNormValuationAudit.lineFunction controlCurve 2 2 (controlCurve.slope 2 2 2 2))) =
      (2 : ℤ) ∧
    -WithZero.log (finiteAdic controlCurve control_positive_nonsingular
      (MillerNormValuationAudit.lineFunction controlCurve 2 2 (controlCurve.slope 2 2 2 2))) =
      (1 : ℤ) := by
  refine ⟨?_, control_delta, control_two_x_not_uniformizer,
    control_negative_tangent_order, control_negative_other_zero⟩
  intro K _ _ W
  refine ⟨?_, ?_, ?_, ?_⟩
  · intro x y h
    exact ⟨pointLocalDVR W h, pointFunctionFractionRing W h.1, point_parameter_maximal W h⟩
  · intro x₁ y₁ x₂ y₂ h₁ h₂ hxy
    exact chord_principal_point_divisor W h₁ h₂ hxy
  · intro x y h
    exact vertical_principal_point_divisor W h
  · intro x y h h3
    exact three_tangent_principal_point_divisor W h h3

#print axioms miller_finite_point_certificate
end
end MillerFinitePointAudit
