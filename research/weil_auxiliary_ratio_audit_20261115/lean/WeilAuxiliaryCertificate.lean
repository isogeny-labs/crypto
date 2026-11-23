import AuxiliaryControls

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit GeneralTranslationAudit
open ThreeTorsionRankAudit.General ThreeTorsionCharacterAudit ThreeTangentPairingAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

/-- Actual all-n auxiliary independence and the constructed divisor-formula pairing at three.
All source ordering, point/local-unit/divisor domains and finite controls are explicit. -/
theorem actual_weil_auxiliary_certificate :
    (∀ (K : Type*) [Field K] [DecidableEq K] [IsAlgClosed K]
      (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
      (∀ (P Q : W.Point) (n : ℕ) (_hP : n • P=0) (_hQ : n • Q=0),
        ∃ c : Kˣ,
          (∃ S : W.Point,AuxiliaryAdmissible W P Q (chosenFunctionUnit W P n) (chosenFunctionUnit W Q n) S) ∧
          ∀ (S : W.Point) (h : AuxiliaryAdmissible W P Q (chosenFunctionUnit W P n) (chosenFunctionUnit W Q n) S),
            sourceAuxiliaryValue W hΔ P Q S (chosenFunctionUnit W P n) (chosenFunctionUnit W Q n) h=c) ∧
      (∀ P Q : ThreeTorsion W,
        (divisorWeilThree W hΔ P Q : K)=(tangentPair W P.val Q.val)⁻¹ ∧
          (divisorWeilThree W hΔ P Q)^3=1) ∧
      (∀ P : ThreeTorsion W,divisorWeilThree W hΔ P P=1) ∧
      (∀ P Q : ThreeTorsion W,
        divisorDegree W (pointRepresentative W P.val)=0 ∧
        divisorDegree W (shiftedRepresentative W (sourcePairingPoint W hΔ P Q) Q.val)=0 ∧
        Disjoint (pointRepresentative W P.val).support
          (shiftedRepresentative W (sourcePairingPoint W hΔ P Q) Q.val).support ∧
        functionDivisor W (chosenFunctionUnit W P.val 3) (chosenFunctionUnit W P.val 3).ne_zero=
          (3 : ℤ) • pointRepresentative W P.val ∧
        functionDivisor W (generalTranslatedUnit W hΔ (-(sourcePairingPoint W hΔ P Q)) (chosenFunctionUnit W Q.val 3))
            (generalTranslatedUnit W hΔ (-(sourcePairingPoint W hΔ P Q)) (chosenFunctionUnit W Q.val 3)).ne_zero=
          (3 : ℤ) • shiftedRepresentative W (sourcePairingPoint W hΔ P Q) Q.val) ∧
      (∀ (P Q : ThreeTorsion W) (f g : W.FunctionFieldˣ)
        (_hf : functionDivisor W f f.ne_zero=torsionDivisor W P.val 3)
        (_hg : functionDivisor W g g.ne_zero=torsionDivisor W Q.val 3)
        (S : W.Point) (h : AuxiliaryAdmissible W P.val Q.val f g S),
        sourceAuxiliaryValue W hΔ P.val Q.val S f g h=divisorWeilThree W hΔ P Q) ∧
      ((3 : K)≠0 →
        (∀ P Q R : ThreeTorsion W,
          divisorWeilThree W hΔ (P+Q) R=divisorWeilThree W hΔ P R*divisorWeilThree W hΔ Q R) ∧
        (∀ P Q R : ThreeTorsion W,
          divisorWeilThree W hΔ P (Q+R)=divisorWeilThree W hΔ P Q*divisorWeilThree W hΔ P R) ∧
        (∀ P : ThreeTorsion W,(∀ Q : ThreeTorsion W,divisorWeilThree W hΔ P Q=1) → P=0) ∧
        (∀ Q : ThreeTorsion W,(∀ P : ThreeTorsion W,divisorWeilThree W hΔ P Q=1) → Q=0) ∧
        (∀ (P : ThreeTorsion W) (_hP : P≠0),
          ∃ Q : Submodule.span (ZMod 3) {P},Q≠0 ∧
            ∀ R : Submodule.span (ZMod 3) {P},divisorWeilThree W hΔ Q.val R.val=1))) ∧
    (CharP BinaryClosure 2 ∧ (3 : BinaryClosure)≠0 ∧ binaryCurve.Δ≠0 ∧
      ∃ P Q : ThreeTorsion binaryCurve,
        (divisorWeilThree binaryCurve binary_discriminant P Q)^3=1 ∧
        (divisorWeilThree binaryCurve binary_discriminant P Q : BinaryClosure)=
          (tangentPair binaryCurve P.val Q.val)⁻¹ ∧
        (divisorWeilThree binaryCurve binary_discriminant P Q : BinaryClosure)≠tangentPair binaryCurve P.val Q.val ∧
        divisorWeilThree binaryCurve binary_discriminant P Q≠1) ∧
    (CharP TernaryClosure 3 ∧
      ∃ hΔ : (curve (1 : TernaryClosure) 1).Δ≠0,
        ∃ P : ThreeTorsion (curve (1 : TernaryClosure) 1),P≠0 ∧
          ∀ Q : ThreeTorsion (curve (1 : TernaryClosure) 1),
            divisorWeilThree (curve (1 : TernaryClosure) 1) hΔ P Q=1) := by
  refine ⟨?_,binary_inverse_convention_counter,ternary_radical_counter⟩
  intro K _ _ _ W hΔ
  refine ⟨?_,?_,divisor_weil_three_alternating W hΔ,divisor_weil_three_representatives W hΔ,
    arbitrary_three_functions_source W hΔ,?_⟩
  · intro P Q n hP hQ
    obtain ⟨c,hc⟩ := chosen_auxiliary_independent W hΔ P Q n hP hQ
    refine ⟨c⁻¹,exists_auxiliary_admissible W hΔ P Q _ _,?_⟩
    intro S h
    rw [source_auxiliary_inverse]
    exact congrArg Inv.inv (hc S h)
  · intro P Q
    exact ⟨divisor_weil_three_value W hΔ P Q,divisor_weil_three_cube W hΔ P Q⟩
  · intro h3
    let : NeZero (3 : K) := ⟨h3⟩
    exact ⟨divisor_weil_three_add_left W hΔ,divisor_weil_three_add_right W hΔ,
      divisor_weil_three_left_nondegenerate W hΔ,divisor_weil_three_right_nondegenerate W hΔ,
      source_cyclic_line_degenerate W hΔ⟩

#print axioms actual_weil_auxiliary_certificate
end
end WeilAuxiliaryRatioAudit
