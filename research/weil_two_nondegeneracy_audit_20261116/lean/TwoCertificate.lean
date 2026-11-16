import TwoControls

namespace WeilTwoNondegeneracyAudit
noncomputable section
open WeilGeometricLawsAudit

def ActualQualifiedTwoNondegeneracy : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (_ : IsAlgClosed K) (_ : NeZero (2 : K)),
    ∀ (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
      (∀ P : torsionPoints W 2,(∀ Q,geometricPairing W hΔ 2 P Q=1)→P=0) ∧
      (∀ Q : torsionPoints W 2,(∀ P,geometricPairing W hΔ 2 P Q=1)→Q=0)

def ActualBinaryTwoRadical : Prop:=
  ordinaryBinaryCurve.Δ≠0 ∧ ordinaryBinaryPoint≠0 ∧
    ∀ Q : torsionPoints ordinaryBinaryCurve 2,
      geometricPairing ordinaryBinaryCurve ordinary_binary_delta 2 ordinaryBinaryPoint Q=1

def ActualTwoSourceEvaluation : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (_ : IsAlgClosed K),
    ∀ (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (x y u v : K)
      (hP : W.Nonsingular x y) (hQ : W.Nonsingular u v)
      (P Q : torsionPoints W 2),
      P.val=WeierstrassCurve.Affine.Point.some x y hP→
      Q.val=WeierstrassCurve.Affine.Point.some u v hQ→x≠u→
      (geometricPairing W hΔ 2 P Q : K)= -1

theorem actual_two_nondegeneracy_certificate :
    ActualQualifiedTwoNondegeneracy ∧ ActualTwoSourceEvaluation ∧
      ActualBinaryTwoRadical ∧ ActualPositiveTwoControls ∧
      ¬UnguardedTwoNondegeneracyReading := by
  refine ⟨?_,?_,actual_binary_two_radical,actual_positive_two_controls,unguarded_two_nondegeneracy_false⟩
  · intro K _ _ _ _ W hΔ
    exact ⟨geometric_two_left_nondegenerate W hΔ,geometric_two_right_nondegenerate W hΔ⟩
  · intro K _ _ _ W hΔ
    exact geometric_two_distinct_x W hΔ

#print axioms actual_two_nondegeneracy_certificate
end
end WeilTwoNondegeneracyAudit
