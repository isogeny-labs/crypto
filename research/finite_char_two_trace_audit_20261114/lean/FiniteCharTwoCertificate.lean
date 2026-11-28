import ImperfectControl

namespace FiniteCharTwoTraceAudit
noncomputable section
open SmallPrimeTraceGeometryAudit SupersingularTraceClassesAudit TwoTorsionRankAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
private instance : DecidableEq ImperfectField := Classical.decEq _
private instance : DecidableEq ImperfectClosure := Classical.decEq _

/-- Credit the preceding actual F3 curve and its point/torsion counts. -/
theorem char_three_source_false : ¬sourceSupersingular SmallCharacteristicJAudit.finiteControl := by
  simp only [sourceSupersingular,fieldTrace,ZMod.card,
    SmallCharacteristicJAudit.finite_control_point_card]
  decide +kernel

theorem changing_characteristic_false :
    ¬∀ (K : Type) [Field K] [Fintype K] [DecidableEq K] [CharP K 3]
      (W : WeierstrassCurve.Affine K) [W.IsElliptic],
      sourceSupersingular W ↔ Nat.card (ThreeTorsionRankAudit.General.ThreeTorsion W)=1 := by
  intro h
  let : SmallCharacteristicJAudit.finiteControl.IsElliptic :=
    SmallCharacteristicJAudit.ellipticOfNonzero _ (by
      rw [SmallCharacteristicJAudit.finite_control_delta]
      exact one_ne_zero)
  exact char_three_source_false ((h (ZMod 3) SmallCharacteristicJAudit.finiteControl).mpr
    SmallCharacteristicJAudit.finite_control_torsion_card)

/-- All finite fields of characteristic two, with both finiteness and
characteristic negative controls. No Hasse or endomorphism-rank premise. -/
theorem finite_char_two_trace_certificate :
    (∀ (K : Type) [Field K] [Fintype K] [DecidableEq K] [CharP K 2]
      (W : WeierstrassCurve.Affine K) [W.IsElliptic],
      (sourceSupersingular W ↔ W.a₁=0) ∧
      (sourceSupersingular W ↔ W.j=0) ∧
      (sourceSupersingular W ↔ ¬2∣Nat.card W.Point) ∧
      (sourceSupersingular W ↔ Nat.card (TwoTorsion W)=1) ∧
      Nat.card (TwoTorsion W)=(if W.j=0 then 1 else 2) ∧
      Module.finrank (ZMod 2) (TwoTorsion W)=(if W.j=0 then 0 else 1)) ∧
    (∀ (K L : Type) [Field K] [Field L] [Fintype K] [DecidableEq K]
      [DecidableEq L] [CharP K 2] [Algebra K L] [IsAlgClosed L]
      (W : WeierstrassCurve.Affine K) [W.IsElliptic],
      (sourceSupersingular W ↔ Nat.card (TwoTorsion (W.baseChange L))=1) ∧
      Nat.card (TwoTorsion (W.baseChange L))=Nat.card (TwoTorsion W)) ∧
    (∀ (K L : Type) [Field K] [Field L] [Fintype K] [Fintype L]
      [DecidableEq K] [DecidableEq L] [CharP K 2] [Algebra K L]
      (W : WeierstrassCurve.Affine K) [W.IsElliptic],
      sourceSupersingular (W.baseChange L) ↔ sourceSupersingular W) ∧
    (imperfectControl.Δ≠0 ∧ imperfectControl.a₁=1 ∧ imperfectControl.j≠0 ∧
      Nat.card (TwoTorsion imperfectControl)=1 ∧
      Nat.card (TwoTorsion (imperfectControl.baseChange ImperfectClosure))=2) ∧
    (¬∀ (K : Type) [Field K] [DecidableEq K] [CharP K 2]
      (W : WeierstrassCurve.Affine K),W.Δ≠0 → W.a₁≠0 →
      ∃ P : W.Point,P≠0 ∧ 2 • P=0) ∧
    (¬sourceSupersingular SmallCharacteristicJAudit.finiteControl ∧
      Nat.card SmallCharacteristicJAudit.finiteControl.Point=5 ∧
      Nat.card (ThreeTorsionRankAudit.General.ThreeTorsion SmallCharacteristicJAudit.finiteControl)=1) ∧
    (¬∀ (K : Type) [Field K] [Fintype K] [DecidableEq K] [CharP K 3]
      (W : WeierstrassCurve.Affine K) [W.IsElliptic],
      sourceSupersingular W ↔ Nat.card (ThreeTorsionRankAudit.General.ThreeTorsion W)=1) := by
  refine ⟨?_,?_,?_,?_,deleting_finiteness_false,?_,changing_characteristic_false⟩
  · intro K _ _ _ _ W _
    have hΔ := SmallCharacteristicJAudit.delta_nonzero W
    exact ⟨source_iff_a1_zero W hΔ,source_iff_j_zero W,source_iff_odd_actual_count W,
      source_iff_rational_two_card_one W hΔ,finite_j_two_card W,finite_j_two_finrank W⟩
  · intro K L _ _ _ _ _ _ _ _ W _
    exact ⟨source_iff_geometric_two_card_one W,rational_geometric_two_card_equal W⟩
  · intro K L _ _ _ _ _ _ _ _ W _
    exact finite_extension_source_iff W
  · exact ⟨imperfect_nonsingular,rfl,imperfect_j_nonzero,imperfect_rational_two_card,
      imperfect_geometric_two_card⟩
  · exact ⟨char_three_source_false,SmallCharacteristicJAudit.finite_control_point_card,
      SmallCharacteristicJAudit.finite_control_torsion_card⟩

#print axioms changing_characteristic_false
#print axioms finite_char_two_trace_certificate
end
end FiniteCharTwoTraceAudit
