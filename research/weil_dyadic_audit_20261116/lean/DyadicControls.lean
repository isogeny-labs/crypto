import DyadicCharacteristic

namespace WeilDyadicAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K] [NeZero (2 : K)]

theorem actual_dyadic_positive (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℕ) :
    ∃ P Q : torsionPoints W ((2 : ℤ)^(n+1)),
      geometricPairing W hΔ ((2 : ℤ)^(n+1)) P Q≠1 := by
  obtain ⟨P,Q,hpair⟩:=WeilTwoNondegeneracyAudit.qualified_two_positive W hΔ
  have hp : P≠0 := by
    intro h
    rw [h,geometric_pairing_zero_left] at hpair
    exact hpair rfl
  by_contra hh
  have hr : ∀ Q,geometricPairing W hΔ ((2 : ℤ)^(n+1)) (twoToDyadic W n P) Q=1 := by
    intro Q
    by_contra hq
    exact hh ⟨twoToDyadic W n P,Q,hq⟩
  exact two_to_dyadic_ne_zero W n P hp
    (geometric_dyadic_left_nondegenerate W hΔ (n+1) _ hr)

def ActualDyadicPositiveControls : Prop:=
  (∀ n : ℕ,∃ P Q : torsionPoints
      (WeilTwoNondegeneracyAudit.healthyTwoCurve WeilTwoNondegeneracyAudit.TernaryClosure) ((2 : ℤ)^(n+1)),
    geometricPairing _ WeilTwoNondegeneracyAudit.healthy_ternary_delta ((2 : ℤ)^(n+1)) P Q≠1) ∧
  (∀ n : ℕ,∃ P Q : torsionPoints
      (WeilTwoNondegeneracyAudit.healthyTwoCurve WeilTwoNondegeneracyAudit.RationalClosure) ((2 : ℤ)^(n+1)),
    geometricPairing _ WeilTwoNondegeneracyAudit.healthy_rational_delta ((2 : ℤ)^(n+1)) P Q≠1)

theorem actual_dyadic_positive_controls : ActualDyadicPositiveControls :=
  ⟨actual_dyadic_positive _ _,actual_dyadic_positive _ _⟩

#print axioms actual_dyadic_positive
#print axioms actual_dyadic_positive_controls
end
end WeilDyadicAudit
