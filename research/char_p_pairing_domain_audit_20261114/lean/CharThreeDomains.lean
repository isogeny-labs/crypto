import ActualPQuotients
import FiniteCharThreeCertificate

namespace CharPPairingDomainAudit
noncomputable section
open FiniteCharThreeTraceAudit QuadraticTwistCountAudit ThreeTorsionRankAudit.General
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {K : Type*} [Field K] [DecidableEq K] [CharP K 3]

theorem three_left_card_with_point (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P : W.Point) (hP : P≠0) (hk : (3:ℕ) • P=0) : Nat.card (actualLeft W 3)=3 := by
  rcases ThreeTorsionCharThreeAudit.char_three_card_one_or_three W hΔ with h | h
  · exact False.elim (hP ((SmallCharacteristicJAudit.killed_card_one_iff 3).mp h P hk))
  · exact h

/-- These are actual curve kernel/cokernel and actual field-unit quotient sizes. -/
def ThreeObstructed (W : WeierstrassCurve.Affine K) : Prop :=
  Nat.card (actualLeft W 3)=3 ∧ Nat.card (ActualRight W 3)=3 ∧
    Nat.card (PowerTarget K 3)=1 ∧ ¬TatePairingExists W 3 ∧
    ∀ (L : Type) [Field L] [CharP L 3],¬UnitPairingExists W 3 L

theorem three_domains_with_point [Finite K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P : W.Point) (hP : P≠0) (hk : (3:ℕ) • P=0) : ThreeObstructed W := by
  have hl := three_left_card_with_point W hΔ P hP hk
  refine ⟨hl,(actual_quotient_card W 3).trans hl,?_,?_,?_⟩
  · simpa only [pow_one] using finite_power_target_card (K:=K) 3 1
  · simpa only [pow_one] using no_tate_pairing_of_actual_point W 3 1 P hP
      (by simpa only [pow_one] using hk)
  · intro L _ _
    simpa only [pow_one] using no_unit_pairing_of_actual_point W 3 1 P hP
      (by simpa only [pow_one] using hk) L

theorem ordinary_three_pair_obstruction [Fintype K] (a b c d : K)
    (hΔ : (cubicModel a b c).Δ≠0) (ha : a≠0) (hd : ¬IsSquare d) :
    ThreeObstructed (cubicModel a b c) ∨ ThreeObstructed (twistModel a b c d) := by
  rcases ordinary_either_three_point a b c d hΔ ha hd with h | h
  · obtain ⟨P,hP,hk⟩ := h
    exact Or.inl (three_domains_with_point _ hΔ P hP hk)
  · obtain ⟨P,hP,hk⟩ := h
    have hd0 : d≠0 := by intro hz; subst d; exact hd ⟨0,by simp⟩
    have ht : (twistModel a b c d).Δ≠0 := by
      rw [twist_discriminant]
      exact mul_ne_zero (pow_ne_zero 6 hd0) hΔ
    exact Or.inr (three_domains_with_point _ ht P hP hk)

theorem named_three_obstruction : ThreeObstructed ordinaryTwist :=
  three_domains_with_point _ ordinary_twist_delta twistPoint
    (WeierstrassCurve.Affine.Point.some_ne_zero _) twist_point_killed

#print axioms three_left_card_with_point
#print axioms ordinary_three_pair_obstruction
#print axioms named_three_obstruction
end
end CharPPairingDomainAudit
