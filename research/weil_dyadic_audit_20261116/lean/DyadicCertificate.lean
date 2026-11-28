import DyadicControls

namespace WeilDyadicAudit
noncomputable section
open WeilGeometricLawsAudit

def ActualDyadicNondegeneracy : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (_ : IsAlgClosed K) (_ : NeZero (2 : K)),
    ∀ (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℕ),
      (∀ P : torsionPoints W ((2 : ℤ)^n),
        (∀ Q,geometricPairing W hΔ ((2 : ℤ)^n) P Q=1)→P=0) ∧
      (∀ Q : torsionPoints W ((2 : ℤ)^n),
        (∀ P,geometricPairing W hΔ ((2 : ℤ)^n) P Q=1)→Q=0)

def ActualDyadicCharacteristicCollapse : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (_ : IsAlgClosed K) (_ : CharP K 2),
    ∀ (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℕ),
      ∀ P Q : torsionPoints W ((2 : ℤ)^n),geometricPairing W hΔ ((2 : ℤ)^n) P Q=1

def ActualBinaryDyadicRadical : Prop:=
  ∀ n : ℕ,WeilTwoNondegeneracyAudit.ordinaryBinaryCurve.Δ≠0 ∧ ordinaryDyadicPoint n≠0 ∧
      ∀ Q,geometricPairing WeilTwoNondegeneracyAudit.ordinaryBinaryCurve
        WeilTwoNondegeneracyAudit.ordinary_binary_delta ((2 : ℤ)^(n+1)) (ordinaryDyadicPoint n) Q=1

theorem actual_dyadic_pairing_certificate :
    ActualDyadicNondegeneracy ∧ ActualDyadicCharacteristicCollapse ∧
      ActualBinaryDyadicRadical ∧ ActualDyadicPositiveControls ∧ ¬UnguardedDyadicNondegeneracyReading := by
  refine ⟨?_,?_,actual_binary_dyadic_radical,actual_dyadic_positive_controls,
    unguarded_dyadic_nondegeneracy_false⟩
  · intro K _ _ _ _ W hΔ n
    exact ⟨geometric_dyadic_left_nondegenerate W hΔ n,geometric_dyadic_right_nondegenerate W hΔ n⟩
  · intro K _ _ _ _ W hΔ n P Q
    exact geometric_dyadic_characteristic_trivial W hΔ n P Q

#print axioms actual_dyadic_pairing_certificate
end
end WeilDyadicAudit
