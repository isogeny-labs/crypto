import ChoiceControls
import ChoiceZero

namespace WeilDivisorChoiceAudit
noncomputable section
open WeierstrassCurve.Affine
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open EllipticPicardBridgeAudit WeilAuxiliaryRatioAudit GeneralTranslationAudit
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
attribute [local instance] generalF2CommRing

/-- Actual source-oriented geometric divisor quotient, choice proof, existence and guard control. -/
theorem actual_weil_divisor_choice_certificate
    (K : Type*) [Field K] [DecidableEq K] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    (∀ D : W.Point →₀ ℤ,divisorDegree W D=0→∀ T : Finset W.Point,
      ∃ (S : W.Point) (h : W.FunctionFieldˣ),
        Disjoint (shiftedRepresentative W S (divisorPointSum W D)).support T ∧
        functionDivisor W h h.ne_zero=shiftedRepresentative W S (divisorPointSum W D)-D) ∧
    (∀ n : ℤ,∀ A : DivisorPair W n,pairValue W n A^n=1 ∧
      pairValue W n (swapPair W n A)=(pairValue W n A)⁻¹) ∧
    (∀ n : ℤ,∀ A B : DivisorPair W n,A.D=B.D→A.E=B.E→pairValue W n A=pairValue W n B) ∧
    (∀ n : ℤ,∀ A B : DivisorPair W n,
      divisorPointSum W A.D=divisorPointSum W B.D→
      divisorPointSum W A.E=divisorPointSum W B.E→pairValue W n A=pairValue W n B) ∧
    (∀ n : ℤ,∀ D : W.Point →₀ ℤ,divisorDegree W D=0→n • divisorPointSum W D=0→
      ∃ f : W.FunctionFieldˣ,functionDivisor W f f.ne_zero=n • D) ∧
    (∀ n : ℤ,∀ P Q : W.Point,n • P=0→n • Q=0→
      ∃ A : DivisorPair W n,divisorPointSum W A.D=P ∧ divisorPointSum W A.E=Q) ∧
    (∀ A : DivisorPair W 0,pairValue W 0 A=1) ∧
    (binaryCurve.Δ≠0 ∧ binaryPoint≠0 ∧ binaryPoint≠ -binaryPoint ∧
      (∀ R : binaryCurve.Point,R=0 ∨ R=binaryPoint ∨ R= -binaryPoint) ∧
      ¬∃ S : binaryCurve.Point,
        Disjoint (shiftedRepresentative binaryCurve S binaryPoint).support {0,binaryPoint}) := by
  refine ⟨actual_moving_representative W hΔ,?_,pair_value_function_choice W hΔ,
    pair_value_representative_choice W hΔ,actual_multiple_principal W hΔ,
    actual_pair_exists W hΔ,pair_value_zero W hΔ,?_⟩
  · intro n A
    exact ⟨pair_value_root W hΔ n A,pair_value_swap W n A⟩
  · exact ⟨binary_delta,binary_point_nonzero,binary_point_not_negative,
      binary_point_population,binary_no_rational_shift.2.2⟩

#print axioms actual_weil_divisor_choice_certificate
end
end WeilDivisorChoiceAudit
