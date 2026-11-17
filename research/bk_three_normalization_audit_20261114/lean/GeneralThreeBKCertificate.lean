import ShiftDescent
import ThreeNormalForm
import ThreeCountBridge
import ThreeCharacterControls

namespace BKThreeNormalizationAudit
noncomputable section
open WeierstrassCurve.Affine BKFrobeniusBridgeAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
private instance : Fact (Nat.Prime 7) := ⟨by decide⟩
local instance : DecidableEq TorsionBoundaryAudit.K := Classical.decEq _
local instance : DecidableEq ThreeTorsionCharacterAudit.OrdinaryClosure := Classical.decEq _

variable {F L : Type*} [Field F] [Fintype F] [Field L] [Algebra F L]

theorem three_nonzero_of_coprime (hc : Nat.Coprime 3 (Fintype.card F)) : (3:L)≠0 := by
  intro hz
  have hF : (3:F)=0 := (algebraMap F L).injective (by simpa only [map_ofNat,map_zero] using hz)
  have hO : addOrderOf (1:F)=3 := addOrderOf_eq_prime
    (by simpa only [nsmul_one,Nat.cast_ofNat] using hF) one_ne_zero
  have hd : 3 ∣ Fintype.card F := by
    rw [←Nat.card_eq_fintype_card,←hO]
    exact addOrderOf_dvd_natCard (1:F)
  have hbad : 3 ∣ 1 := hc.gcd_eq_one ▸ Nat.dvd_gcd (dvd_refl 3) hd
  norm_num at hbad

variable [DecidableEq F] [DecidableEq L] [IsAlgClosed L]

theorem actual_general_three_criterion {K : Type*} [Field K] [Fintype K]
    [Algebra F K] [Algebra K L] [IsScalarTower F K L] [DecidableEq K]
    (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0)
    (hc : Nat.Coprime 3 (Fintype.card F)) (hN : 3 ∣ Nat.card W.Point)
    (hq : ¬3 ∣ Fintype.card F-1) (k : ℕ) (hK : Fintype.card K=Fintype.card F^k) :
    (∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 →
      P∈Set.range (pointInclusion F K L W)) ↔ 3 ∣ Fintype.card F^k-1 := by
  have : NeZero (3:L) := ⟨three_nonzero_of_coprime hc⟩
  have hq' : (Fintype.card F : ZMod 3)≠1 := by
    intro he
    apply hq
    simpa only [pow_one] using
      (cyclotomic_cast_iff (Fintype.card F) 1 Fintype.card_pos).mp (by simpa using he)
  obtain ⟨P,hP,_,ht⟩ := BKLinearAudit.actual_rational_prime_torsion W 3 (by decide) hN
  cases P with
  | zero => exact False.elim (hP rfl)
  | some x y h =>
    obtain ⟨hd,h2,h4,h6,_⟩ := actual_normalization W hΔ x y h ht
    have hb := ThreeTorsionCharacterAudit.third_coefficient_ne_zero (normalized W x y) h2 h4 h6 hd
    have hn := ThreeTorsionCharacterAudit.actual_tate_extension_criterion
      (L:=L) (K:=K) (normalized W x y) h2 h4 h6 hb hd hq' k hK
    exact (shift_torsion_image_iff (A:=K) (B:=L) W hΔ x (tangentSlope W x y) y 3).symm.trans hn

theorem actual_general_three_nine_points {K : Type*} [Field K] [Fintype K]
    [Algebra F K] [Algebra K L] [IsScalarTower F K L] [DecidableEq K]
    (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0)
    (hc : Nat.Coprime 3 (Fintype.card F)) (hN : 3 ∣ Nat.card W.Point)
    (hq : ¬3 ∣ Fintype.card F-1) (k : ℕ) (hK : Fintype.card K=Fintype.card F^k) :
    Nat.card (ThreeTorsionRankAudit.General.ThreeTorsion (W.baseChange K))=9 ↔
      3 ∣ Fintype.card F^k-1 := by
  have : NeZero (3:L) := ⟨three_nonzero_of_coprime hc⟩
  exact (actual_nine_points_iff (K:=K) (L:=L) W hΔ).trans
    (actual_general_three_criterion W hΔ hc hN hq k hK)

/-- Original qualified inclusion criterion at prime three; the type exposes
all finite fields, actual group cardinality and compatible chosen embeddings. -/
theorem actual_general_three_bk_certificate :
    (∀ (F L : Type) [Field F] [Fintype F] [Field L] [Algebra F L]
      [DecidableEq F] [DecidableEq L] [IsAlgClosed L]
      (W : WeierstrassCurve.Affine F), W.Δ≠0 →
      Nat.Coprime 3 (Fintype.card F) → 3 ∣ Nat.card W.Point →
      ¬3 ∣ Fintype.card F-1 →
      ∀ (K : Type) [Field K] [Fintype K] [Algebra F K] [Algebra K L]
        [IsScalarTower F K L] [DecidableEq K],
        ∀ k : ℕ, Fintype.card K=Fintype.card F^k →
        ((∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 →
          P∈Set.range (pointInclusion F K L W)) ↔ 3 ∣ Fintype.card F^k-1) ∧
        (Nat.card (ThreeTorsionRankAudit.General.ThreeTorsion (W.baseChange K))=9 ↔
          3 ∣ Fintype.card F^k-1)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K),
      W.Δ≠0 → ∀ x y : K, ∀ hN : W.Nonsingular x y,
      (3:ℕ) • Point.some x y hN=0 →
      (normalized W x y).Δ≠0 ∧ (normalized W x y).a₂=0 ∧
      (normalized W x y).a₄=0 ∧ (normalized W x y).a₆=0 ∧
      Nonempty ((normalized W x y).Point ≃+ W.Point)) ∧
    (Nat.Prime 3 ∧ Nat.Coprime 3 7 ∧ 3 ∣ 7-1 ∧
      3^2 ∣ Nat.card TorsionBoundaryAudit.bkCurve.Point ∧ TorsionBoundaryAudit.bkCurve.Δ≠0 ∧
      ∃ P : (TorsionBoundaryAudit.bkCurve.baseChange TorsionBoundaryAudit.K).Point,
        (3:ℕ) • P=0 ∧ FrobeniusPointDescentAudit.pointFrobenius TorsionBoundaryAudit.bkCurve P≠P) ∧
    (CharP ThreeTorsionCharacterAudit.OrdinaryClosure 3 ∧
      ThreeTorsionCharacterAudit.ordinaryBase.Δ≠0 ∧
      Nat.card (ThreeTorsionRankAudit.General.ThreeTorsion
        (ThreeTorsionCharacterAudit.curve (1:ThreeTorsionCharacterAudit.OrdinaryClosure) 1))=3 ∧
      ¬((∀ P : (ThreeTorsionCharacterAudit.ordinaryBase.baseChange
        ThreeTorsionCharacterAudit.OrdinaryClosure).Point, (3:ℕ) • P=0 →
        P∈Set.range (pointInclusion (ZMod 3) (ZMod 3)
          ThreeTorsionCharacterAudit.OrdinaryClosure ThreeTorsionCharacterAudit.ordinaryBase)) ↔
          3 ∣ 3^1-1)) := by
  refine ⟨?_,?_,FrobeniusPointDescentAudit.bk_nonfixed_control,?_⟩
  · intro F L _ _ _ _ _ _ _ W hΔ hc hN hq K _ _ _ _ _ _ k hK
    exact ⟨actual_general_three_criterion W hΔ hc hN hq k hK,
      actual_general_three_nine_points (L:=L) W hΔ hc hN hq k hK⟩
  · intro K _ _ W hΔ x y hN ht
    exact actual_normalization W hΔ x y hN ht
  · obtain ⟨hd,hcard,_⟩ := ThreeTorsionCharacterAudit.actual_char_three_rank_control
    exact ⟨inferInstance,hd,hcard,ThreeTorsionCharacterAudit.deleting_coprimality_inclusion_false⟩

#print axioms three_nonzero_of_coprime
#print axioms actual_general_three_criterion
#print axioms actual_general_three_bk_certificate
end
end BKThreeNormalizationAudit
