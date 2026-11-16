import MixedControls

namespace WeilMixedLevelAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit

def ActualMixedNondegeneracy : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (_ : IsAlgClosed K)
    (_ : NeZero (2 : K)) (_ : NeZero (3 : K)),
    ∀ (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℕ),
      (∀ P : torsionPoints W (mixedLevel n),(∀ Q,geometricPairing W hΔ (mixedLevel n) P Q=1)→P=0) ∧
      (∀ Q : torsionPoints W (mixedLevel n),(∀ P,geometricPairing W hΔ (mixedLevel n) P Q=1)→Q=0)

def ActualCharacteristicRadical : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (_ : IsAlgClosed K)
    (p : ℕ) (_ : Fact p.Prime) (_ : CharP K p),
    ∀ (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (m : ℤ)
      (P : torsionPoints W m),(p : ℕ) • P=0 →
        ∀ Q,geometricPairing W hΔ m P Q=1

def ActualTernaryMixedRadical : Prop:=
  ∀ n : ℕ,(originCurve TernaryClosure).Δ≠0 ∧ ternaryMixedPoint n≠0 ∧
    ∀ Q,geometricPairing _ ordinary_ternary_delta (mixedLevel n) (ternaryMixedPoint n) Q=1

def ActualBinaryMixedRadical : Prop:=
  ∀ n : ℕ,WeilTwoNondegeneracyAudit.ordinaryBinaryCurve.Δ≠0 ∧ binaryMixedPoint n≠0 ∧
    ∀ Q,geometricPairing _ WeilTwoNondegeneracyAudit.ordinary_binary_delta
      (mixedLevel (n+1)) (binaryMixedPoint n) Q=1

theorem actual_mixed_pairing_certificate :
    ActualMixedNondegeneracy ∧ ActualCharacteristicRadical ∧ ActualTernaryMixedRadical ∧
      ActualBinaryMixedRadical ∧ ActualMixedPositiveControls ∧
      ¬DeletedTwoGuardReading ∧ ¬DeletedThreeGuardReading := by
  refine ⟨?_,?_,actual_ternary_mixed_radical,actual_binary_mixed_radical,
    actual_mixed_positive_controls,deleted_two_guard_false,deleted_three_guard_false⟩
  · intro K _ _ _ _ _ W hΔ n
    exact ⟨geometric_mixed_left_nondegenerate W hΔ n,geometric_mixed_right_nondegenerate W hΔ n⟩
  · intro K _ _ _ p _ _ W hΔ m P hp Q
    exact characteristic_torsion_radical p W hΔ m P hp Q

#print axioms actual_mixed_pairing_certificate
end
end WeilMixedLevelAudit
