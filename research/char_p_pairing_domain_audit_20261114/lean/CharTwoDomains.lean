import ActualPQuotients
import FiniteTwoTorsion
import CharTwoModels

namespace CharPPairingDomainAudit
noncomputable section
open TwoTorsionRankAudit FiniteCharTwoTraceAudit
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
variable {K : Type*} [Field K] [DecidableEq K] [CharP K 2]

theorem two_domain_cards [Finite K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (actualLeft W 2)=(if W.a₁=0 then 1 else 2) ∧
      Nat.card (ActualRight W 2)=(if W.a₁=0 then 1 else 2) := by
  have hl : Nat.card (actualLeft W 2)=(if W.a₁=0 then 1 else 2) :=
    finite_two_card W hΔ
  exact ⟨hl,(actual_quotient_card W 2).trans hl⟩

theorem ordinary_two_no_unit_pairing [Finite K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (ha : W.a₁≠0) (L : Type*) [Field L] [CharP L 2] :
    ¬UnitPairingExists W 2 L := by
  obtain ⟨P,hP,hk⟩ := finite_ordinary_nonzero_killed W hΔ ha
  simpa only [pow_one] using no_unit_pairing_of_actual_point W 2 1 P hP
    (by simpa only [pow_one] using hk) L

theorem ordinary_two_no_tate_pairing [Finite K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (ha : W.a₁≠0) : ¬TatePairingExists W 2 := by
  obtain ⟨P,hP,hk⟩ := finite_ordinary_nonzero_killed W hΔ ha
  simpa only [pow_one] using no_tate_pairing_of_actual_point W 2 1 P hP
    (by simpa only [pow_one] using hk)

/-- This actual curve has a rational order-two point over every char2 field,
including infinite imperfect fields. No descent premise is needed. -/
theorem ordinary_model_no_unit_pairing (L : Type*) [Field L] [CharP L 2] :
    ¬UnitPairingExists (ordinaryModel K) 2 L := by
  simpa only [pow_one] using no_unit_pairing_of_actual_point (ordinaryModel K) 2 1
    (ordinaryPoint K) ordinaryPoint_nonzero
    (by simpa only [pow_one] using ordinaryPoint_killed (K:=K)) L

theorem ordinary_model_no_tate_pairing [Finite K] :
    ¬TatePairingExists (ordinaryModel K) 2 := by
  apply ordinary_two_no_tate_pairing
  · rw [ordinary_delta]
    exact one_ne_zero
  · exact one_ne_zero

theorem ordinary_model_domain_cards [Finite K] :
    Nat.card (actualLeft (ordinaryModel K) 2)=2 ∧
      Nat.card (ActualRight (ordinaryModel K) 2)=2 ∧
      Nat.card (PowerTarget K 2)=1 := by
  have hd : (ordinaryModel K).Δ≠0 := by rw [ordinary_delta]; exact one_ne_zero
  obtain ⟨hl,hr⟩ := two_domain_cards (ordinaryModel K) hd
  exact ⟨by simpa [ordinaryModel] using hl,by simpa [ordinaryModel] using hr,
    by simpa only [pow_one] using finite_power_target_card (K:=K) 2 1⟩

/-- The source coprimality condition cannot simply be deleted. -/
theorem deleting_coprimality_unit_false :
    ¬∀ (F : Type) [Field F] [Fintype F] [DecidableEq F]
      (W : WeierstrassCurve.Affine F) (r : ℕ),
      W.Δ≠0 → r.Prime → r∣Nat.card W.Point → UnitPairingExists W r F := by
  intro h
  let F := ZMod 2
  have hd : (ordinaryModel F).Δ≠0 := by rw [ordinary_delta]; exact one_ne_zero
  have hv : 2∣Nat.card (ordinaryModel F).Point :=
    (two_dvd_actual_count_iff (ordinaryModel F) hd).mpr one_ne_zero
  exact ordinary_model_no_unit_pairing (K:=F) F (h F (ordinaryModel F) 2 hd (by decide) hv)

theorem unqualified_tate_claim_false :
    ¬∀ (F : Type) [Field F] [Fintype F] [DecidableEq F]
      (W : WeierstrassCurve.Affine F) (r : ℕ),
      W.Δ≠0 → r.Prime → r∣Nat.card W.Point → TatePairingExists W r := by
  intro h
  let F := ZMod 2
  have hd : (ordinaryModel F).Δ≠0 := by rw [ordinary_delta]; exact one_ne_zero
  have hv : 2∣Nat.card (ordinaryModel F).Point :=
    (two_dvd_actual_count_iff (ordinaryModel F) hd).mpr one_ne_zero
  exact ordinary_model_no_tate_pairing (K:=F) (h F (ordinaryModel F) 2 hd (by decide) hv)

#print axioms two_domain_cards
#print axioms ordinary_two_no_unit_pairing
#print axioms ordinary_model_domain_cards
#print axioms deleting_coprimality_unit_false
#print axioms unqualified_tate_claim_false
end
end CharPPairingDomainAudit
