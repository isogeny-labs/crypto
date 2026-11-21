import FiveCardinality

namespace FiveTorsionRankAudit
noncomputable section
open WeierstrassCurve.Affine Module
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
variable {K : Type*} [Field K] [DecidableEq K]

theorem five_torsion_killed (W : WeierstrassCurve.Affine K) (P : FiveTorsion W) :
    (5:ℕ) • P=0 := by
  apply Subtype.ext
  exact P.property

instance fiveModule (W : WeierstrassCurve.Affine K) : Module (ZMod 5) (FiveTorsion W) :=
  AddCommGroup.zmodModule (five_torsion_killed W)

theorem geometric_five_finrank [IsAlgClosed K] [NeZero (5:K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : finrank (ZMod 5) (FiveTorsion W)=2 := by
  let : Finite (FiveTorsion W) := five_torsion_finite W hΔ
  have hc := Module.natCard_eq_pow_finrank (K:=ZMod 5) (V:=FiveTorsion W)
  rw [geometric_five_card W hΔ] at hc
  have hF : Nat.card (ZMod 5)=5 := by simp
  rw [hF] at hc
  apply Nat.pow_right_injective (by decide : 2≤5)
  exact hc.symm

def geometricFiveBasis [IsAlgClosed K] [NeZero (5:K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : Basis (Fin 2) (ZMod 5) (FiveTorsion W) := by
  let : Finite (FiveTorsion W) := five_torsion_finite W hΔ
  exact Module.finBasisOfFinrankEq (ZMod 5) (FiveTorsion W) (geometric_five_finrank W hΔ)

def nonzeroFiveExactEquiv (W : WeierstrassCurve.Affine K) :
    {P : FiveTorsion W // P≠0} ≃ {P : W.Point // addOrderOf P=5} where
  toFun P := ⟨P.val.val,addOrderOf_eq_prime P.val.property (by
    intro hz
    apply P.property
    exact Subtype.ext hz)⟩
  invFun P := by
    have hk : (5:ℕ) • P.val=0 := by
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

theorem geometric_five_exact_card [IsAlgClosed K] [NeZero (5:K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : Nat.card {P : W.Point // addOrderOf P=5}=24 := by
  let : Finite (FiveTorsion W) := five_torsion_finite W hΔ
  let := Fintype.ofFinite (FiveTorsion W)
  rw [←Nat.card_congr (nonzeroFiveExactEquiv W),Nat.card_eq_fintype_card]
  have hc : Fintype.card {P : FiveTorsion W // P≠0}=Fintype.card (FiveTorsion W)-1 := by simp
  rw [hc,←Nat.card_eq_fintype_card,geometric_five_card W hΔ]

omit [DecidableEq K] in
theorem scalar_extension_five_card [NeZero (5:K)] {L : Type*} [Field L] [DecidableEq L]
    [IsAlgClosed L] (f : K →+* L) (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (FiveTorsion (W.map f))=25 := by
  let := ThreeTorsionRankAudit.scalar_map_ne_zero f 5
  apply geometric_five_card
  rw [WeierstrassCurve.map_Δ]
  simpa only [map_zero] using f.injective.ne hΔ

#print axioms geometric_five_finrank
#print axioms geometricFiveBasis
#print axioms geometric_five_exact_card
end
end FiveTorsionRankAudit
