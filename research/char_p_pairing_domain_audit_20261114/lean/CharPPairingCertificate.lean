import CharTwoDomains
import CharThreeDomains
import ImperfectTarget

namespace CharPPairingDomainAudit
noncomputable section
open QuadraticTwistCountAudit FiniteCharThreeTraceAudit FiniteCharTwoTraceAudit
open TwoTorsionRankAudit

/-- Actual group/field domains; these controls retain the qualified primary
source and make no canonical divisor-pairing or group-scheme identification. -/
theorem characteristic_pairing_domain_certificate :
    (∀ (K : Type) [Field K] (p : ℕ) [Fact p.Prime] [CharP K p] (k : ℕ),
      (∀ u : Kˣ,u^(p^k)=1 → u=1) ∧
      (∀ [Finite K],Function.Surjective (powMonoidHom (p^k) : Kˣ →* Kˣ) ∧
        Nat.card (PowerTarget K (p^k))=1)) ∧
    (∀ (K : Type) [Field K] [Fintype K] (r : ℕ) [Fact r.Prime]
      (u : Kˣ),u^r=1 → u≠1 → Nat.Coprime r (Fintype.card K)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [Finite K]
      (W : WeierstrassCurve.Affine K) (n : ℕ),
      Nat.card (ActualRight W n)=Nat.card (actualLeft W n)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [Finite K] [CharP K 2]
      (W : WeierstrassCurve.Affine K),W.Δ≠0 →
      (Nat.card (actualLeft W 2)=(if W.a₁=0 then 1 else 2) ∧
        Nat.card (ActualRight W 2)=(if W.a₁=0 then 1 else 2)) ∧
      (W.a₁≠0 → ¬TatePairingExists W 2 ∧
        ∀ (L : Type) [Field L] [CharP L 2],¬UnitPairingExists W 2 L)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [Finite K] [CharP K 2],
      (Nat.card (actualLeft (ordinaryModel K) 2)=2 ∧
        Nat.card (ActualRight (ordinaryModel K) 2)=2 ∧ Nat.card (PowerTarget K 2)=1) ∧
      ¬TatePairingExists (ordinaryModel K) 2 ∧
      ∀ (L : Type) [Field L] [CharP L 2],¬UnitPairingExists (ordinaryModel K) 2 L) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [Fintype K] [CharP K 3]
      (a b c d : K),(cubicModel a b c).Δ≠0 → a≠0 → ¬IsSquare d →
      ThreeObstructed (cubicModel a b c) ∨ ThreeObstructed (twistModel a b c d)) ∧
    (ThreeObstructed ordinaryTwist ∧ addOrderOf twistPoint=3) ∧
    (variableClass≠1 ∧ Nat.card (PowerTarget ImperfectField 2)≠1 ∧
      ¬Function.Surjective (powMonoidHom 2 : ImperfectFieldˣ →* ImperfectFieldˣ) ∧
      ∀ u : ImperfectFieldˣ,u^2=1 → u=1) ∧
    (¬∀ (F : Type) [Field F] [Fintype F] [DecidableEq F]
      (W : WeierstrassCurve.Affine F) (r : ℕ),
      W.Δ≠0 → r.Prime → r∣Nat.card W.Point → UnitPairingExists W r F) ∧
    (¬∀ (F : Type) [Field F] [Fintype F] [DecidableEq F]
      (W : WeierstrassCurve.Affine F) (r : ℕ),
      W.Δ≠0 → r.Prime → r∣Nat.card W.Point → TatePairingExists W r) ∧
    (¬∀ (F : Type) [Field F] (p : ℕ) [Fact p.Prime] [CharP F p],
      Nat.card (PowerTarget F p)=1) := by
  refine ⟨?_,?_,?_,?_,?_,?_,?_,?_,?_,?_,?_⟩
  · intro K _ p _ _ k
    exact ⟨unit_prime_power_root p k,fun [_] =>
      ⟨finite_unit_power_surjective p k,finite_power_target_card p k⟩⟩
  · intro K _ _ r _ u hu hn
    exact prime_nontrivial_root_coprime r u hu hn
  · intro K _ _ _ W n
    exact actual_quotient_card W n
  · intro K _ _ _ _ W hΔ
    exact ⟨two_domain_cards W hΔ,fun ha =>
      ⟨ordinary_two_no_tate_pairing W hΔ ha,ordinary_two_no_unit_pairing W hΔ ha⟩⟩
  · intro K _ _ _ _
    exact ⟨ordinary_model_domain_cards,ordinary_model_no_tate_pairing,
      ordinary_model_no_unit_pairing⟩
  · intro K _ _ _ _ a b c d hΔ ha hd
    exact ordinary_three_pair_obstruction a b c d hΔ ha hd
  · exact ⟨named_three_obstruction,twist_point_order⟩
  · exact ⟨variable_class_nontrivial,imperfect_target_card_not_one,
      imperfect_unit_square_not_surjective,imperfect_unit_root_still_trivial⟩
  · exact deleting_coprimality_unit_false
  · exact unqualified_tate_claim_false
  · exact deleting_finiteness_target_false

#print axioms characteristic_pairing_domain_certificate
end
end CharPPairingDomainAudit
