import CharThreeCoefficient

namespace SmallCharacteristicJAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General

theorem ellipticOfNonzero {K : Type*} [Field K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) : W.IsElliptic := ⟨isUnit_iff_ne_zero.mpr hΔ⟩

theorem delta_nonzero {K : Type*} [Field K] (W : WeierstrassCurve.Affine K)
    [W.IsElliptic] : W.Δ≠0 := W.isUnit_Δ.ne_zero

variable {K : Type*} [Field K] [DecidableEq K]

theorem two_killed (W : WeierstrassCurve.Affine K) (P : TwoTorsionRankAudit.TwoTorsion W) :
    (2:ℕ) • P=0 := by
  apply Subtype.ext
  exact P.property

instance twoModule (W : WeierstrassCurve.Affine K) :
    Module (ZMod 2) (TwoTorsionRankAudit.TwoTorsion W) := AddCommGroup.zmodModule (two_killed W)

theorem killed_card_one_iff {G : Type*} [AddCommGroup G] (n : ℕ) :
    Nat.card {P : G // n • P=0}=1 ↔ ∀ P : G,n • P=0 → P=0 := by
  constructor
  · intro h P ht
    obtain ⟨hs,_⟩ := Nat.card_eq_one_iff_unique.mp h
    exact congrArg Subtype.val (hs.elim (⟨P,ht⟩ : {P : G // n • P=0}) ⟨0,smul_zero _⟩)
  · intro h
    apply Nat.card_eq_one_iff_exists.mpr
    exact ⟨⟨0,smul_zero _⟩,fun P => Subtype.ext (h P.val P.property)⟩

theorem two_j_card [CharP K 2] [IsAlgClosed K] (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    Nat.card (TwoTorsionRankAudit.TwoTorsion W)=if W.j=0 then 1 else 2 := by
  simp only [W.j_eq_zero_iff_of_char_two]
  exact TwoTorsionRankAudit.char_two_geometric_classification W (delta_nonzero W)

theorem three_j_card [CharP K 3] [IsAlgClosed K] (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    Nat.card (ThreeTorsion W)=if W.j=0 then 1 else 3 := by
  simp only [W.j_eq_zero_iff_of_char_three]
  exact char_three_coefficient_classification W (delta_nonzero W)

theorem two_j_trivial [CharP K 2] [IsAlgClosed K] (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    W.j=0 ↔ ∀ P : W.Point,(2:ℕ) • P=0 → P=0 := by
  rw [←killed_card_one_iff]
  change W.j=0 ↔ Nat.card (TwoTorsionRankAudit.TwoTorsion W)=1
  rw [two_j_card]
  by_cases hj : W.j=0 <;> simp [hj]

theorem three_j_trivial [CharP K 3] [IsAlgClosed K] (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    W.j=0 ↔ ∀ P : W.Point,(3:ℕ) • P=0 → P=0 := by
  rw [←killed_card_one_iff]
  change W.j=0 ↔ Nat.card (ThreeTorsion W)=1
  rw [three_j_card]
  by_cases hj : W.j=0 <;> simp [hj]

theorem two_j_finrank [CharP K 2] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    Module.finrank (ZMod 2) (TwoTorsionRankAudit.TwoTorsion W)=if W.j=0 then 0 else 1 := by
  have : Finite (TwoTorsionRankAudit.TwoTorsion W) := BKSmallPrimeSourceAudit.char_two_finite W (delta_nonzero W)
  have hc := Module.natCard_eq_pow_finrank (K:=ZMod 2) (V:=TwoTorsionRankAudit.TwoTorsion W)
  have hn : Nat.card (ZMod 2)=2 := by simp
  rw [hn,two_j_card W] at hc
  by_cases hj : W.j=0
  · rw [ite_eq_left hj] at hc ⊢
    exact Nat.pow_right_injective (by decide : 2≤2) (show 2^Module.finrank (ZMod 2) (TwoTorsionRankAudit.TwoTorsion W)=2^0 from hc.symm)
  · rw [ite_eq_right hj] at hc ⊢
    exact Nat.pow_right_injective (by decide : 2≤2) (show 2^Module.finrank (ZMod 2) (TwoTorsionRankAudit.TwoTorsion W)=2^1 from hc.symm)

theorem three_j_finrank [CharP K 3] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    Module.finrank (ZMod 3) (ThreeTorsion W)=if W.j=0 then 0 else 1 := by
  have : Finite (ThreeTorsion W) := ThreeTorsionCharThreeAudit.char_three_finite W (delta_nonzero W)
  have hc := Module.natCard_eq_pow_finrank (K:=ZMod 3) (V:=ThreeTorsion W)
  have hn : Nat.card (ZMod 3)=3 := by simp
  rw [hn,three_j_card W] at hc
  by_cases hj : W.j=0
  · rw [ite_eq_left hj] at hc ⊢
    exact Nat.pow_right_injective (by decide : 2≤3) (show 3^Module.finrank (ZMod 3) (ThreeTorsion W)=3^0 from hc.symm)
  · rw [ite_eq_right hj] at hc ⊢
    exact Nat.pow_right_injective (by decide : 2≤3) (show 3^Module.finrank (ZMod 3) (ThreeTorsion W)=3^1 from hc.symm)

section ScalarExtension
variable {F L : Type*} [Field F] [Field L] [Algebra F L] [DecidableEq F] [DecidableEq L] [IsAlgClosed L]

theorem two_geometric_j [CharP F 2] (W : WeierstrassCurve.Affine F) [W.IsElliptic] :
    Nat.card (TwoTorsionRankAudit.TwoTorsion (W.baseChange L))=if W.j=0 then 1 else 2 := by
  have : CharP L 2 := charP_of_injective_algebraMap' F 2
  have : (W.baseChange L).IsElliptic := ellipticOfNonzero _
    (BKThreeNormalizationAudit.base_discriminant_nonzero W (delta_nonzero W))
  have hj : (W.baseChange L).j=0 ↔ W.j=0 := by
    change (W.map (algebraMap F L)).j=0 ↔ W.j=0
    rw [WeierstrassCurve.map_j]
    exact map_eq_zero (algebraMap F L)
  simpa only [hj] using two_j_card (W.baseChange L)

theorem three_geometric_j [CharP F 3] (W : WeierstrassCurve.Affine F) [W.IsElliptic] :
    Nat.card (ThreeTorsion (W.baseChange L))=if W.j=0 then 1 else 3 := by
  have : CharP L 3 := charP_of_injective_algebraMap' F 3
  have : (W.baseChange L).IsElliptic := ellipticOfNonzero _
    (BKThreeNormalizationAudit.base_discriminant_nonzero W (delta_nonzero W))
  have hj : (W.baseChange L).j=0 ↔ W.j=0 := by
    change (W.map (algebraMap F L)).j=0 ↔ W.j=0
    rw [WeierstrassCurve.map_j]
    exact map_eq_zero (algebraMap F L)
  simpa only [hj] using three_j_card (W.baseChange L)
end ScalarExtension

#print axioms two_j_finrank
#print axioms three_j_finrank
#print axioms two_geometric_j
#print axioms three_geometric_j
end
end SmallCharacteristicJAudit
