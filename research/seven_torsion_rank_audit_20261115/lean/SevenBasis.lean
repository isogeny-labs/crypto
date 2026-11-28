import SevenCardinality

namespace SevenTorsionRankAudit
noncomputable section
open WeierstrassCurve.Affine Module
private instance : Fact (Nat.Prime 7) := ⟨by decide⟩
variable {K : Type*} [Field K] [DecidableEq K]

theorem seven_torsion_killed (W : WeierstrassCurve.Affine K) (P : SevenTorsion W) :
    (7:ℕ) • P=0 := by apply Subtype.ext; exact P.property

instance sevenModule (W : WeierstrassCurve.Affine K) : Module (ZMod 7) (SevenTorsion W) :=
  AddCommGroup.zmodModule (seven_torsion_killed W)

theorem geometric_seven_finrank [IsAlgClosed K] [NeZero (7:K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : finrank (ZMod 7) (SevenTorsion W)=2 := by
  let : Finite (SevenTorsion W) := seven_torsion_finite W hΔ
  have hc := Module.natCard_eq_pow_finrank (K:=ZMod 7) (V:=SevenTorsion W)
  rw [geometric_seven_card W hΔ] at hc
  have hF : Nat.card (ZMod 7)=7 := by simp
  rw [hF] at hc
  apply Nat.pow_right_injective (by decide : 2≤7)
  exact hc.symm

def geometricSevenBasis [IsAlgClosed K] [NeZero (7:K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : Basis (Fin 2) (ZMod 7) (SevenTorsion W) := by
  let : Finite (SevenTorsion W) := seven_torsion_finite W hΔ
  exact Module.finBasisOfFinrankEq (ZMod 7) (SevenTorsion W) (geometric_seven_finrank W hΔ)

def nonzeroSevenExactEquiv (W : WeierstrassCurve.Affine K) :
    {P : SevenTorsion W // P≠0} ≃ {P : W.Point // addOrderOf P=7} where
  toFun P := ⟨P.val.val,addOrderOf_eq_prime P.val.property (by
    intro hz
    apply P.property
    exact Subtype.ext hz)⟩
  invFun P := by
    have hk : (7:ℕ) • P.val=0 := by
      have h := addOrderOf_nsmul_eq_zero P.val
      rwa [P.property] at h
    have hn : P.val≠0 := by
      intro hz
      have h := P.property
      rw [hz,addOrderOf_zero] at h
      norm_num at h
    exact ⟨⟨P.val,hk⟩,fun hz => hn (congrArg Subtype.val hz)⟩
  left_inv P := by apply Subtype.ext; apply Subtype.ext; rfl
  right_inv P := by apply Subtype.ext; rfl

theorem geometric_seven_exact_card [IsAlgClosed K] [NeZero (7:K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : Nat.card {P : W.Point // addOrderOf P=7}=48 := by
  let : Finite (SevenTorsion W) := seven_torsion_finite W hΔ
  let := Fintype.ofFinite (SevenTorsion W)
  rw [←Nat.card_congr (nonzeroSevenExactEquiv W),Nat.card_eq_fintype_card]
  have hc : Fintype.card {P : SevenTorsion W // P≠0}=Fintype.card (SevenTorsion W)-1 := by simp
  rw [hc,←Nat.card_eq_fintype_card,geometric_seven_card W hΔ]

omit [DecidableEq K] in
theorem scalar_extension_seven_card [NeZero (7:K)] {L : Type*} [Field L] [DecidableEq L]
    [IsAlgClosed L] (f : K →+* L) (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (SevenTorsion (W.map f))=49 := by
  let := ThreeTorsionRankAudit.scalar_map_ne_zero f 7
  apply geometric_seven_card
  rw [WeierstrassCurve.map_Δ]
  simpa only [map_zero] using f.injective.ne hΔ

#print axioms geometric_seven_finrank
#print axioms geometricSevenBasis
#print axioms geometric_seven_exact_card
end
end SevenTorsionRankAudit
