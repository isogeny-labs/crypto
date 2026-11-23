import GeneralCardinality
import TernaryBasis
import CharacteristicThree

namespace ThreeTorsionRankAudit.General
open Module
noncomputable section
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {K : Type*} [Field K] [DecidableEq K]

theorem torsion_three_killed (W : WeierstrassCurve.Affine K) (P : ThreeTorsion W) : (3 : ℕ) • P=0 := by
  apply Subtype.ext
  change (3 : ℕ) • P.val=0
  exact P.property

instance torsionModule (W : WeierstrassCurve.Affine K) : Module (ZMod 3) (ThreeTorsion W) :=
  AddCommGroup.zmodModule (torsion_three_killed W)

theorem geometric_three_finrank [IsAlgClosed K] [NeZero (3 : K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) : Module.finrank (ZMod 3) (ThreeTorsion W)=2 := by
  let : Finite (ThreeTorsion W) := torsion_finite W hΔ
  have hc := Module.natCard_eq_pow_finrank (K := ZMod 3) (V := ThreeTorsion W)
  rw [geometric_three_torsion_card W hΔ] at hc
  have hf : Nat.card (ZMod 3)=3 := by simp
  rw [hf] at hc
  apply Nat.pow_right_injective (by decide : 2 ≤ 3)
  exact hc.symm

def geometricThreeBasis [IsAlgClosed K] [NeZero (3 : K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) : Basis (Fin 2) (ZMod 3) (ThreeTorsion W) := by
  let : Finite (ThreeTorsion W) := torsion_finite W hΔ
  exact Module.finBasisOfFinrankEq (ZMod 3) (ThreeTorsion W) (geometric_three_finrank W hΔ)

theorem geometric_three_basis_exists [IsAlgClosed K] [NeZero (3 : K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) : Nonempty (Basis (Fin 2) (ZMod 3) (ThreeTorsion W)) :=
  ⟨geometricThreeBasis W hΔ⟩

def nonzeroExactEquiv (W : WeierstrassCurve.Affine K) :
    {P : ThreeTorsion W // P ≠ 0} ≃ {P : W.Point // addOrderOf P=3} where
  toFun P := ⟨P.val.val,addOrderOf_eq_prime P.val.property (by
    intro hz
    apply P.property
    apply Subtype.ext
    exact hz)⟩
  invFun P := by
    have hk : (3 : ℕ) • P.val=0 := by
      have h := addOrderOf_nsmul_eq_zero P.val
      rwa [P.property] at h
    have hn : P.val ≠ 0 := by
      intro hz
      have h := P.property
      rw [hz,addOrderOf_zero] at h
      norm_num at h
    exact ⟨⟨P.val,hk⟩,fun hz => hn (congrArg Subtype.val hz)⟩
  left_inv P := by apply Subtype.ext; apply Subtype.ext; rfl
  right_inv P := by apply Subtype.ext; rfl

theorem exact_order_three_card [IsAlgClosed K] [NeZero (3 : K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) : Nat.card {P : W.Point // addOrderOf P=3}=8 := by
  classical
  let : Finite (ThreeTorsion W) := torsion_finite W hΔ
  let := Fintype.ofFinite (ThreeTorsion W)
  rw [← Nat.card_congr (nonzeroExactEquiv W),Nat.card_eq_fintype_card]
  have hc : Fintype.card {P : ThreeTorsion W // P ≠ 0}=Fintype.card (ThreeTorsion W)-1 := by
    simp
  rw [hc,← Nat.card_eq_fintype_card,geometric_three_torsion_card W hΔ]

omit [DecidableEq K] in
theorem scalar_extension_card [NeZero (3 : K)] {L : Type*} [Field L] [DecidableEq L] [IsAlgClosed L]
    (f : K →+* L) (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    Nat.card (ThreeTorsion (W.map f))=9 := by
  let := ThreeTorsionRankAudit.scalar_map_ne_zero f 3
  apply geometric_three_torsion_card
  rw [WeierstrassCurve.map_Δ]
  simpa only [map_zero] using f.injective.ne hΔ

theorem char_three_control :
    (short (-1 : CharThreeClosure) 0).Δ ≠ 0 ∧
    Nat.card (ThreeTorsion (short (-1 : CharThreeClosure) 0))=1 ∧
    Nat.card (ThreeTorsion (short (-1 : CharThreeClosure) 0))≠9 := by
  have hc : Nat.card (ThreeTorsion (short (-1 : CharThreeClosure) 0))=1 := by
    rw [Nat.card_congr (shortTorsionEquiv (-1 : CharThreeClosure) 0)]
    exact ThreeTorsionRankAudit.char_three_torsion_card
  exact ⟨ThreeTorsionRankAudit.char_three_discriminant,hc,by rw [hc]; decide⟩

end

noncomputable section
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
abbrev BinaryClosure := AlgebraicClosure (ZMod 2)
private instance : DecidableEq BinaryClosure := Classical.decEq _
private instance : CharP BinaryClosure 2 := charP_of_injective_algebraMap' (ZMod 2) 2
private instance : NeZero (3 : BinaryClosure) := by
  constructor
  intro h
  have hd : 2 ∣ 3 := (CharP.cast_eq_zero_iff BinaryClosure 2 3).mp h
  norm_num at hd

def binaryCurve : WeierstrassCurve.Affine BinaryClosure := ⟨0,0,1,0,1⟩

theorem binary_discriminant : binaryCurve.Δ ≠ 0 := by
  have he : binaryCurve.Δ=-(675 : BinaryClosure) := by
    norm_num [binaryCurve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
      WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  rw [he,neg_ne_zero]
  intro h
  have hd : 2 ∣ 675 := (CharP.cast_eq_zero_iff BinaryClosure 2 675).mp h
  norm_num at hd

theorem actual_binary_geometric_control :
    binaryCurve.Δ ≠ 0 ∧ Nat.card (ThreeTorsion binaryCurve)=9 ∧
    Module.finrank (ZMod 3) (ThreeTorsion binaryCurve)=2 ∧
    Nonempty (Basis (Fin 2) (ZMod 3) (ThreeTorsion binaryCurve)) :=
  ⟨binary_discriminant,geometric_three_torsion_card binaryCurve binary_discriminant,
    geometric_three_finrank binaryCurve binary_discriminant,
    geometric_three_basis_exists binaryCurve binary_discriminant⟩

end
end ThreeTorsionRankAudit.General
