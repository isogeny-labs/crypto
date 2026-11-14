import CharTwoAllFields
import CharTwoModels

namespace BKSmallPrimeSourceAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
variable {F K : Type*} [Field F] [Fintype F] [Field K] [Fintype K]
  [Algebra F K] [DecidableEq F] [DecidableEq K]

/-- Construct the auxiliary closure and tower rather than requiring them
as source-theorem premises. The conclusion concerns actual points over K. -/
theorem direct_original_three (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0)
    (hN : 3 ∣ Nat.card W.Point) (hq : ¬3 ∣ Fintype.card F-1)
    (k : ℕ) (hK : Fintype.card K=Fintype.card F^k) :
    Nat.card (ThreeTorsion (W.baseChange K))=9 ↔ 3 ∣ Fintype.card F^k-1 := by
  classical
  exact ThreeTorsionCharThreeAudit.source_original_three_criterion
    (L:=AlgebraicClosure K) W hΔ hN hq k hK

omit [DecidableEq F] in
theorem finite_two_zero_of_source_condition (hq : ¬2∣Fintype.card F-1) : (2:F)=0 := by
  have hp : 1<Fintype.card F := Fintype.one_lt_card
  have hd : 2∣Fintype.card F := by omega
  by_contra hn
  have hd' : 2∣Nat.card F := by simpa only [Nat.card_eq_fintype_card] using hd
  obtain ⟨x,hx⟩ := exists_prime_addOrderOf_dvd_card' (G:=F) 2 hd'
  have hkill := addOrderOf_nsmul_eq_zero x
  rw [hx] at hkill
  have hm : (2:F)*x=0 := by simpa only [nsmul_eq_mul,Nat.cast_ofNat] using hkill
  have hz : x=0 := (mul_eq_zero.mp hm).resolve_left hn
  rw [hz,addOrderOf_zero] at hx
  norm_num at hx

omit [DecidableEq F] in
theorem direct_original_two (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0)
    (_hN : 2∣Nat.card W.Point) (hq : ¬2∣Fintype.card F-1)
    (k : ℕ) (hK : Fintype.card K=Fintype.card F^k) :
    Nat.card (TwoTorsionRankAudit.TwoTorsion (W.baseChange K))=4 ↔
      2∣Fintype.card F^k-1 := by
  have h3 : (2:F)=0 := finite_two_zero_of_source_condition hq
  have hK2 : (2:K)=0 := by
    simpa only [map_ofNat,map_zero] using congrArg (algebraMap F K) h3
  have : CharP K 2 := (CharP.charP_iff_prime_eq_zero (by decide : Nat.Prime 2)).mpr
    (by simpa only [Nat.cast_ofNat] using hK2)
  have hn := char_two_ne_four (W.baseChange K) (BKThreeNormalizationAudit.base_discriminant_nonzero W hΔ)
  have hk : 0<k := by
    have hp : 1<Fintype.card K := Fintype.one_lt_card
    by_contra h
    have hz : k=0 := by omega
    rw [hz,pow_zero] at hK
    omega
  have hd : 2∣Fintype.card F := by
    have hp : 1<Fintype.card F := Fintype.one_lt_card
    omega
  have hc : ¬2∣Fintype.card F^k-1 := by
    intro hs
    have hpow : 1≤Fintype.card F^k := pow_pos Fintype.card_pos k
    have hd' := dvd_pow hd (Nat.ne_of_gt hk)
    obtain ⟨u,hu⟩ := hd'
    obtain ⟨v,hv⟩ := hs
    omega
  exact iff_of_false hn hc

theorem direct_small_prime_source_certificate :
    (∀ (F K : Type) [Field F] [Fintype F] [Field K] [Fintype K]
      [Algebra F K] [DecidableEq F] [DecidableEq K] (W : WeierstrassCurve.Affine F),
      W.Δ≠0 → 3∣Nat.card W.Point → ¬3∣Fintype.card F-1 →
      ∀ k : ℕ,Fintype.card K=Fintype.card F^k →
      (Nat.card (ThreeTorsion (W.baseChange K))=9 ↔ 3∣Fintype.card F^k-1)) ∧
    (∀ (F K : Type) [Field F] [Fintype F] [Field K] [Fintype K]
      [Algebra F K] [DecidableEq F] [DecidableEq K] (W : WeierstrassCurve.Affine F),
      W.Δ≠0 → 2∣Nat.card W.Point → ¬2∣Fintype.card F-1 →
      ∀ k : ℕ,Fintype.card K=Fintype.card F^k →
      (Nat.card (TwoTorsionRankAudit.TwoTorsion (W.baseChange K))=4 ↔ 2∣Fintype.card F^k-1)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [CharP K 2] (W : WeierstrassCurve.Affine K),
      W.Δ≠0 → (Nat.card (TwoTorsionRankAudit.TwoTorsion W)=1 ∨
        Nat.card (TwoTorsionRankAudit.TwoTorsion W)=2) ∧
      Finite (TwoTorsionRankAudit.TwoTorsion W) ∧ Nat.card (TwoTorsionRankAudit.TwoTorsion W)≠4) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [CharP K 2] [Finite K],
      (TwoTorsionRankAudit.ordinaryModel K).Δ=1 ∧
      (TwoTorsionRankAudit.supersingularModel K).Δ=1 ∧
      Nat.card (TwoTorsionRankAudit.TwoTorsion (TwoTorsionRankAudit.ordinaryModel K))=2 ∧
      Nat.card (TwoTorsionRankAudit.TwoTorsion (TwoTorsionRankAudit.supersingularModel K))=1 ∧
      2∣Nat.card (TwoTorsionRankAudit.ordinaryModel K).Point) := by
  refine ⟨?_,?_,?_,?_⟩
  · intro F K _ _ _ _ _ _ _ W hΔ hN hq k hK
    exact direct_original_three W hΔ hN hq k hK
  · intro F K _ _ _ _ _ _ _ W hΔ hN hq k hK
    exact direct_original_two W hΔ hN hq k hK
  · intro K _ _ _ W hΔ
    exact ⟨char_two_card_one_or_two W hΔ,char_two_finite W hΔ,char_two_ne_four W hΔ⟩
  · intro K _ _ _ _
    exact ⟨TwoTorsionRankAudit.ordinary_delta,TwoTorsionRankAudit.supersingular_delta,
      TwoTorsionRankAudit.ordinary_torsion_card,TwoTorsionRankAudit.supersingular_torsion_card,
      TwoTorsionRankAudit.ordinary_two_divides_point_card⟩

#print axioms direct_original_three
#print axioms direct_original_two
#print axioms direct_small_prime_source_certificate
end
end BKSmallPrimeSourceAudit
