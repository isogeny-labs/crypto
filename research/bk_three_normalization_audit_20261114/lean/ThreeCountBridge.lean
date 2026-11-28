import ShiftDescent

namespace BKThreeNormalizationAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General BKFrobeniusBridgeAudit
variable {F K L : Type*} [Field F] [Field K] [Field L]
  [Algebra F K] [Algebra F L] [Algebra K L] [IsScalarTower F K L]
  [DecidableEq K] [DecidableEq L]

def threeInclusion (W : WeierstrassCurve.Affine F) :
    ThreeTorsion (W.baseChange K) → ThreeTorsion (W.baseChange L) :=
  fun P => ⟨pointInclusion F K L W P.val,by
    change (3:ℕ) • pointInclusion F K L W P.val=0
    rw [←map_nsmul,P.property,map_zero]⟩

theorem threeInclusion_injective (W : WeierstrassCurve.Affine F) :
    Function.Injective (threeInclusion (K:=K) (L:=L) W) := by
  intro P Q h
  apply Subtype.ext
  apply Point.map_injective (f:=Algebra.ofId K L)
  exact congrArg Subtype.val h

theorem threeInclusion_surjective_iff (W : WeierstrassCurve.Affine F) :
    Function.Surjective (threeInclusion (K:=K) (L:=L) W) ↔
    ∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 →
      P∈Set.range (pointInclusion F K L W) := by
  constructor
  · intro h P ht
    obtain ⟨T,hT⟩ := h ⟨P,ht⟩
    exact ⟨T.val,congrArg Subtype.val hT⟩
  · intro h P
    obtain ⟨T,hT⟩ := h P.val P.property
    have ht : (3:ℕ) • T=0 := by
      apply Point.map_injective (f:=Algebra.ofId K L)
      change pointInclusion F K L W ((3:ℕ) • T)=pointInclusion F K L W 0
      rw [map_nsmul,hT,P.property,map_zero]
    exact ⟨⟨T,ht⟩,Subtype.ext hT⟩

theorem actual_nine_points_iff [IsAlgClosed L] [NeZero (3:L)]
    (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0) :
    Nat.card (ThreeTorsion (W.baseChange K))=9 ↔
    ∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 →
      P∈Set.range (pointInclusion F K L W) := by
  have hd : (W.baseChange L).Δ≠0 := base_discriminant_nonzero W hΔ
  have : Finite (ThreeTorsion (W.baseChange L)) := torsion_finite _ hd
  have hc := geometric_three_torsion_card (W.baseChange L) hd
  rw [←threeInclusion_surjective_iff]
  constructor
  · intro h
    exact ((Nat.bijective_iff_injective_and_card (threeInclusion W)).mpr
      ⟨threeInclusion_injective W,h.trans hc.symm⟩).surjective
  · intro h
    exact (Nat.card_eq_of_bijective _ ⟨threeInclusion_injective W,h⟩).trans hc

#print axioms actual_nine_points_iff
end
end BKThreeNormalizationAudit
