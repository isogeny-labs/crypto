import PrimeTwo

namespace SmallPrimeTraceGeometryAudit
noncomputable section
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
abbrev F3 := ZMod 3

/-- Every general coefficient model over the actual prime field. -/
theorem three_closed_table : ∀ W : WeierstrassCurve.Affine F3,W.Δ≠0 →
    (countedSupersingular W ↔ W.b₂=0) ∧
    (countedSupersingular W → equationCount W=1 ∨ equationCount W=4 ∨ equationCount W=7) ∧
    (countedTrace W)^2≤4*(Fintype.card F3:ℤ) := by
  decide +kernel

theorem three_model_count : Nat.card (WeierstrassCurve.Affine F3)=243 := by
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem three_nonsingular_model_count : Nat.card {W : WeierstrassCurve.Affine F3 // W.Δ≠0}=162 := by
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem three_source_model_count :
    Nat.card {W : WeierstrassCurve.Affine F3 // W.Δ≠0 ∧ sourceSupersingular W}=54 := by
  rw [Nat.card_congr sourceModelEquiv,Nat.card_eq_fintype_card]
  decide +kernel

theorem three_source_iff_b2 (W : WeierstrassCurve.Affine F3) (hΔ : W.Δ≠0) :
    sourceSupersingular W ↔ W.b₂=0 :=
  (source_iff_counted W hΔ).trans (three_closed_table W hΔ).1

theorem three_point_cases (W : WeierstrassCurve.Affine F3) (hΔ : W.Δ≠0)
    (hs : sourceSupersingular W) : Nat.card W.Point=1 ∨ Nat.card W.Point=4 ∨ Nat.card W.Point=7 := by
  rw [actual_point_count W hΔ]
  exact (three_closed_table W hΔ).2.1 ((source_iff_counted W hΔ).mp hs)

theorem three_actual_hasse (W : WeierstrassCurve.Affine F3) (hΔ : W.Δ≠0) :
    (SupersingularTraceClassesAudit.fieldTrace W)^2≤4*(Fintype.card F3:ℤ) := by
  rw [actual_trace W hΔ]
  exact (three_closed_table W hΔ).2.2

theorem three_actual_trace_squares (W : WeierstrassCurve.Affine F3) (hΔ : W.Δ≠0)
    (hs : sourceSupersingular W) :
    SupersingularTraceClassesAudit.TraceSquares (Fintype.card F3)
      (SupersingularTraceClassesAudit.fieldTrace W) := by
  rcases three_point_cases W hΔ hs with h | h | h <;>
    norm_num [SupersingularTraceClassesAudit.TraceSquares,SupersingularTraceClassesAudit.fieldTrace,h]

abbrev Closure3 := AlgebraicClosure F3
private instance : DecidableEq Closure3 := Classical.decEq _

theorem three_source_geometric (W : WeierstrassCurve.Affine F3) (hΔ : W.Δ≠0) :
    sourceSupersingular W ↔
      Nat.card (ThreeTorsionRankAudit.General.ThreeTorsion (W.baseChange Closure3))=1 := by
  have : W.IsElliptic := SmallCharacteristicJAudit.ellipticOfNonzero W hΔ
  have hc := SmallCharacteristicJAudit.three_geometric_j (L:=Closure3) W
  simp only [W.j_eq_zero_iff_of_char_three] at hc
  rw [three_source_iff_b2 W hΔ,hc]
  by_cases h : W.b₂=0 <;> simp [h]

theorem three_actual_order_bound (W : WeierstrassCurve.Affine F3) (hΔ : W.Δ≠0)
    (hs : sourceSupersingular W) (r : ℕ) (hr : r∣Nat.card W.Point) :
    Nat.Coprime r 3 ∧ orderOf (3:ZMod r)∣6 ∧ orderOf (3:ZMod r)≤6 := by
  have hc := three_point_cases W hΔ hs
  have hcop : Nat.Coprime (Nat.card W.Point) 3 := by
    rcases hc with h | h | h <;> rw [h] <;> decide
  have hd : Nat.card W.Point∣3^6-1 := by
    rcases hc with h | h | h <;> rw [h] <;> decide
  have ho := modular_order_dvd 3 r 6 (by decide) (hr.trans hd)
  exact ⟨hcop.of_dvd_left hr,ho,Nat.le_of_dvd (by decide : 0<(6:ℕ)) ho⟩

#print axioms three_closed_table
#print axioms three_nonsingular_model_count
end
end SmallPrimeTraceGeometryAudit
