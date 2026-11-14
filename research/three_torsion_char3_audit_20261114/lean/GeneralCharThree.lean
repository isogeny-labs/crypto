import NormalFormCharThree

namespace ThreeTorsionCharThreeAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General ThreeTorsionCharacterAudit
open BKThreeNormalizationAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

def killedEquiv {G H : Type*} [AddCommGroup G] [AddCommGroup H] (e : G ≃+ H) (n : ℕ) :
    {P : G // n • P=0} ≃ {P : H // n • P=0} where
  toFun P := ⟨e P.val,by rw [←map_nsmul,P.property,map_zero]⟩
  invFun P := ⟨e.symm P.val,by rw [←map_nsmul,P.property,map_zero]⟩
  left_inv P := by apply Subtype.ext; exact e.left_inv _
  right_inv P := by apply Subtype.ext; exact e.right_inv _

variable {K : Type*} [Field K] [DecidableEq K] [CharP K 3]

theorem normal_model_card (W : WeierstrassCurve.Affine K)
    (h2 : W.a₂=0) (h4 : W.a₄=0) (h6 : W.a₆=0) (hΔ : W.Δ≠0) :
    Nat.card (ThreeTorsion W)=3 := by
  rcases W with ⟨a,a2,b,a4,a6⟩
  change a2=0 at h2
  change a4=0 at h4
  change a6=0 at h6
  subst a2
  subst a4
  subst a6
  exact normal_three_card a b hΔ

theorem char_three_card_one_or_three (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (ThreeTorsion W)=1 ∨ Nat.card (ThreeTorsion W)=3 := by
  by_cases hex : ∃ P : W.Point, P≠0 ∧ (3:ℕ) • P=0
  · right
    obtain ⟨P,hP,ht⟩ := hex
    cases P with
    | zero => exact False.elim (hP rfl)
    | some x y hN =>
      obtain ⟨hd,h2,h4,h6,_⟩ := actual_normalization W hΔ x y hN ht
      have hc := normal_model_card (normalized W x y) h2 h4 h6 hd
      have he : Nat.card (ThreeTorsion (normalized W x y))=Nat.card (ThreeTorsion W) :=
        Nat.card_congr (killedEquiv (shiftEquiv W hΔ x (tangentSlope W x y) y) 3)
      exact he.symm.trans hc
  · left
    apply Nat.card_eq_one_iff_exists.mpr
    refine ⟨0,fun P => ?_⟩
    apply Subtype.ext
    by_contra hP
    exact hex ⟨P.val,hP,P.property⟩

theorem char_three_finite (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Finite (ThreeTorsion W) := by
  apply Nat.finite_of_card_ne_zero
  rcases char_three_card_one_or_three W hΔ with h | h <;> rw [h] <;> decide

theorem char_three_ne_nine (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (ThreeTorsion W)≠9 := by
  rcases char_three_card_one_or_three W hΔ with h | h <;> rw [h] <;> decide

theorem char_three_module_rank (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Module.finrank (ZMod 3) (ThreeTorsion W)=0 ∨ Module.finrank (ZMod 3) (ThreeTorsion W)=1 := by
  have : Finite (ThreeTorsion W) := char_three_finite W hΔ
  have hc := Module.natCard_eq_pow_finrank (K:=ZMod 3) (V:=ThreeTorsion W)
  have hn : Nat.card (ZMod 3)=3 := by simp
  rw [hn] at hc
  rcases char_three_card_one_or_three W hΔ with h | h
  · left
    rw [h] at hc
    have he : 3^Module.finrank (ZMod 3) (ThreeTorsion W)=3^0 := hc.symm
    exact Nat.pow_right_injective (by decide : 2≤3) he
  · right
    rw [h] at hc
    have he : 3^Module.finrank (ZMod 3) (ThreeTorsion W)=3^1 := hc.symm
    exact Nat.pow_right_injective (by decide : 2≤3) he

#print axioms char_three_card_one_or_three
#print axioms char_three_module_rank
end
end ThreeTorsionCharThreeAudit
