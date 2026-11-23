module
public import RecursionControls

@[expose] public section
namespace MillerRecursionAudit
noncomputable section
open WeierstrassCurve.Affine MillerFinitePointAudit MillerNormValuationAudit
open MillerPrincipalIdealAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing F := (inferInstance : Field F).toCommRing

/-- Actual all-n nonzero rational Miller functions, local-valuation principal point divisors,
source infinity normalization and every-branch addition invariant, with a vertical guard control.
No projective Scheme.localRing/Divisor API identification or canonical pairing is asserted. -/
theorem miller_recursion_certificate :
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ),
      millerFunction W P n ≠ 0 ∧
      principalPointDivisor W (millerFunction W P n)
        ((miller_has_point_divisor W P n).support_finite W) = millerDivisor W P n ∧
      functionValuation W
        (parameter W ^ (-pointOrder W (millerFunction W P n) 0) * millerFunction W P n - 1) < 1 ∧
      (n • P = 0 → principalPointDivisor W (millerFunction W P n)
        ((miller_has_point_divisor W P n).support_finite W) =
        (n : ℤ) • Finsupp.single P 1 - (n : ℤ) • Finsupp.single 0 1)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K) (P : W.Point)
      (i j : ℕ) (f g : W.FunctionField),
      IsMillerFunction W P i f → IsMillerFunction W P j g →
      IsMillerFunction W P (i + j) (f * g * stepFunction W (i • P) (j • P))) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K) (P : W.Point),
      millerFunction W P 1 = 1) ∧
    controlCurve.Δ ≠ 0 ∧ twoPoint ≠ 0 ∧ twoPoint + twoPoint = 0 ∧
    pointOrder controlCurve (stepFunction controlCurve twoPoint twoPoint) 0 = (-2 : ℤ) ∧
    ¬ HasPointDivisor controlCurve
      (MillerNormValuationAudit.lineFunction controlCurve 4 0 (controlCurve.slope 4 4 0 0))
      (chordDivisor controlCurve twoPoint twoPoint) := by
  refine ⟨?_, ?_, ?_, control_delta, two_point_nonzero, two_point_double,
    two_point_step_infinity_order, unsafe_two_point_chord_divisor⟩
  · intro K _ _ W P n
    exact ⟨miller_function_nonzero W P n, miller_principal_point_divisor W P n,
      miller_infinity_monic W P n, miller_torsion_divisor W P n⟩
  · intro K _ _ W P i j f g hf hg
    exact normalized_miller_addition W P i j hf hg
  · intro K _ _ W P
    exact miller_function_one W P

#print axioms miller_recursion_certificate
end
end MillerRecursionAudit
