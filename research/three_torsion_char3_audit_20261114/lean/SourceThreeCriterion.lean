import GeneralCharThree

namespace ThreeTorsionCharThreeAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General BKThreeNormalizationAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {F : Type*} [Field F] [Fintype F]

theorem finite_coprime_of_three_nonzero (h3 : (3:F)≠0) : Nat.Coprime 3 (Fintype.card F) := by
  apply (Nat.prime_three.coprime_iff_not_dvd).mpr
  intro hd
  have hd' : 3 ∣ Nat.card F := by simpa only [Nat.card_eq_fintype_card] using hd
  obtain ⟨x,hx⟩ := exists_prime_addOrderOf_dvd_card' (G:=F) 3 hd'
  have hkill := addOrderOf_nsmul_eq_zero x
  rw [hx] at hkill
  have hm : (3:F)*x=0 := by simpa only [nsmul_eq_mul,Nat.cast_ofNat] using hkill
  have hz : x=0 := (mul_eq_zero.mp hm).resolve_left h3
  rw [hz,addOrderOf_zero] at hx
  norm_num at hx

theorem finite_three_divides_of_zero (h3 : (3:F)=0) : 3 ∣ Fintype.card F := by
  have ho : addOrderOf (1:F)=3 := addOrderOf_eq_prime
    (by simpa only [nsmul_one,Nat.cast_ofNat] using h3) one_ne_zero
  rw [←Nat.card_eq_fintype_card,←ho]
  exact addOrderOf_dvd_natCard _

theorem cyclotomic_false_when_three_divides (q k : ℕ) (hq : 0<q)
    (hd : 3∣q) (hk : 0<k) : ¬3∣q^k-1 := by
  intro hs
  have hp : 1≤q^k := pow_pos hq k
  have hd' : 3∣q^k := dvd_pow hd (Nat.ne_of_gt hk)
  obtain ⟨u,hu⟩ := hd'
  obtain ⟨v,hv⟩ := hs
  omega

variable {L : Type*} [Field L] [Algebra F L] [DecidableEq F] [DecidableEq L] [IsAlgClosed L]

theorem source_original_three_criterion {K : Type*} [Field K] [Fintype K]
    [Algebra F K] [Algebra K L] [IsScalarTower F K L] [DecidableEq K]
    (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0)
    (hN : 3 ∣ Nat.card W.Point) (hq : ¬3 ∣ Fintype.card F-1)
    (k : ℕ) (hK : Fintype.card K=Fintype.card F^k) :
    Nat.card (ThreeTorsion (W.baseChange K))=9 ↔ 3 ∣ Fintype.card F^k-1 := by
  by_cases h3 : (3:F)=0
  · have hk : 0<k := by
      by_contra hn
      have hk0 : k=0 := by omega
      have hc : Fintype.card K=1 := by simpa only [hk0,pow_zero] using hK
      have hp : 1<Fintype.card K := Fintype.one_lt_card
      omega
    have h3K : (3:K)=0 := by
      simpa only [map_ofNat,map_zero] using congrArg (algebraMap F K) h3
    have : CharP K 3 := (CharP.charP_iff_prime_eq_zero (by decide : Nat.Prime 3)).mpr
      (by simpa only [Nat.cast_ofNat] using h3K)
    have hn := char_three_ne_nine (W.baseChange K) (base_discriminant_nonzero W hΔ)
    have hc := cyclotomic_false_when_three_divides (Fintype.card F) k Fintype.card_pos
      (finite_three_divides_of_zero h3) hk
    exact iff_of_false hn hc
  · exact actual_general_three_nine_points (L:=L) W hΔ (finite_coprime_of_three_nonzero h3) hN hq k hK

/-- Original nine-point source form at prime three, without adding the corpus
inclusion paraphrase's coprimality premise; actual characteristic-three
classification makes both sides false in the exceptional characteristic. -/
theorem original_three_source_certificate :
    (∀ (F L : Type) [Field F] [Fintype F] [Field L] [Algebra F L]
      [DecidableEq F] [DecidableEq L] [IsAlgClosed L]
      (W : WeierstrassCurve.Affine F), W.Δ≠0 → 3 ∣ Nat.card W.Point →
      ¬3 ∣ Fintype.card F-1 →
      ∀ (K : Type) [Field K] [Fintype K] [Algebra F K] [Algebra K L]
        [IsScalarTower F K L] [DecidableEq K],
        ∀ k : ℕ, Fintype.card K=Fintype.card F^k →
        (Nat.card (ThreeTorsion (W.baseChange K))=9 ↔ 3 ∣ Fintype.card F^k-1)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [CharP K 3]
      (W : WeierstrassCurve.Affine K), W.Δ≠0 →
      (Nat.card (ThreeTorsion W)=1 ∨ Nat.card (ThreeTorsion W)=3) ∧
      Finite (ThreeTorsion W) ∧ Nat.card (ThreeTorsion W)≠9 ∧
      (Module.finrank (ZMod 3) (ThreeTorsion W)=0 ∨ Module.finrank (ZMod 3) (ThreeTorsion W)=1)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [CharP K 3] (a b : K),
      (ThreeTorsionCharacterAudit.curve a b).Δ≠0 →
      Nat.card (ThreeTorsion (ThreeTorsionCharacterAudit.curve a b))=3) ∧
    ((ThreeTorsionRankAudit.short (-1:ThreeTorsionRankAudit.CharThreeClosure) 0).Δ≠0 ∧
      Nat.card (ThreeTorsion (ThreeTorsionRankAudit.short
        (-1:ThreeTorsionRankAudit.CharThreeClosure) 0))=1 ∧
      Nat.card (ThreeTorsion (ThreeTorsionRankAudit.short
        (-1:ThreeTorsionRankAudit.CharThreeClosure) 0))≠9) ∧
    (ThreeTorsionCharacterAudit.ordinaryBase.Δ≠0 ∧
      Nat.card (ThreeTorsion (ThreeTorsionCharacterAudit.curve
        (1:ThreeTorsionCharacterAudit.OrdinaryClosure) 1))=3 ∧
      Module.finrank (ZMod 3) (ThreeTorsion (ThreeTorsionCharacterAudit.curve
        (1:ThreeTorsionCharacterAudit.OrdinaryClosure) 1))=1) := by
  refine ⟨?_,?_,?_,ThreeTorsionRankAudit.General.char_three_control,
    ThreeTorsionCharacterAudit.actual_char_three_rank_control⟩
  · intro F L _ _ _ _ _ _ _ W hΔ hN hq K _ _ _ _ _ _ k hK
    exact source_original_three_criterion (L:=L) W hΔ hN hq k hK
  · intro K _ _ _ W hΔ
    exact ⟨char_three_card_one_or_three W hΔ,char_three_finite W hΔ,
      char_three_ne_nine W hΔ,char_three_module_rank W hΔ⟩
  · intro K _ _ _ a b hd
    exact normal_three_card a b hd

#print axioms finite_coprime_of_three_nonzero
#print axioms source_original_three_criterion
#print axioms original_three_source_certificate
end
end ThreeTorsionCharThreeAudit
