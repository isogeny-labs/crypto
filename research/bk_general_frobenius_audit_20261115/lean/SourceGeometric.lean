import SourceNineteen

namespace BKGeneralFrobeniusAudit.SourceNineteen
noncomputable section
private instance : Fact (Nat.Prime 19) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
abbrev Closure := AlgebraicClosure (ZMod 19)
private instance : DecidableEq Closure := Classical.decEq _
private instance : CharP Closure 19 := charP_of_injective_algebraMap' (ZMod 19) 19
private instance : NeZero (3 : Closure) := by
  constructor
  intro h
  have hd := (CharP.cast_eq_zero_iff Closure 19 3).mp h
  norm_num at hd

theorem closure_nonsingular : (curve.baseChange Closure).Δ ≠ 0 := by
  change (curve.map (algebraMap (ZMod 19) Closure)).Δ ≠ 0
  rw [WeierstrassCurve.map_Δ]
  exact map_ne_zero (algebraMap (ZMod 19) Closure) |>.mpr curve_nonsingular

theorem geometric_three_card : Nat.card (Torsion (curve.baseChange Closure) 3) = 9 := by
  change Nat.card (ThreeTorsionRankAudit.General.ThreeTorsion (curve.baseChange Closure)) = 9
  exact ThreeTorsionRankAudit.General.geometric_three_torsion_card _ closure_nonsingular

theorem source_fixed_card : Nat.card (fixedPower (L := Closure) curve 3 1) = 3 := by
  have h := fixedPower_card (K := ZMod 19) (L := Closure) curve 3 1 (by simp)
  have he : curve.baseChange (ZMod 19) = curve := by
    change curve.map (RingHom.id (ZMod 19)) = curve
    exact WeierstrassCurve.map_id _
  rw [he] at h
  exact h.trans rational_three_torsion_card

theorem source_frobenius_not_identity : linearFrobenius (L := Closure) curve 3 ≠ 1 := by
  intro h
  have hk : fixedPower (L := Closure) curve 3 1 = ⊤ := by
    simp [fixedPower, h]
  have hc : Nat.card (fixedPower (L := Closure) curve 3 1) =
      Nat.card (Torsion (curve.baseChange Closure) 3) := by
    rw [hk]
    exact Nat.card_congr (Submodule.topEquiv :
      (⊤ : Submodule (ZMod 3) (Torsion (curve.baseChange Closure) 3)) ≃ₗ[ZMod 3]
        Torsion (curve.baseChange Closure) 3).toEquiv
  rw [source_fixed_card, geometric_three_card] at hc
  norm_num at hc

theorem source_full_rationality_false :
    ¬∀ P : (curve.baseChange Closure).Point, (3 : ℕ) • P = 0 →
      P ∈ Set.range (WeierstrassCurve.Affine.Point.map (W' := curve)
        (Algebra.ofId (ZMod 19) Closure)) := by
  intro h
  apply source_frobenius_not_identity
  have hh := (actual_extension_iff_power_identity (K := ZMod 19) (L := Closure)
    curve 3 1 (by simp)).mp h
  simpa only [pow_one] using hh

theorem source_arithmetic_control : 3 ∣ Nat.card curve.Point ∧
    3^2 ∣ Nat.card curve.Point ∧ (19 : ZMod 3)^1 = 1 := by
  rw [rational_cardinality]
  decide +kernel

#print axioms geometric_three_card
#print axioms source_frobenius_not_identity
#print axioms source_full_rationality_false
end
end BKGeneralFrobeniusAudit.SourceNineteen
