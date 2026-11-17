import EquationCount

namespace SmallPrimeTraceGeometryAudit
noncomputable section
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
abbrev F2 := ZMod 2

/-- Complete coefficient-model census, closed in the Lean kernel. -/
theorem two_closed_table : ∀ W : WeierstrassCurve.Affine F2,W.Δ≠0 →
    (countedSupersingular W ↔ W.a₁=0) ∧
    (countedSupersingular W → equationCount W=1 ∨ equationCount W=3 ∨ equationCount W=5) ∧
    (countedTrace W)^2≤4*(Fintype.card F2:ℤ) := by
  decide +kernel

theorem two_model_count : Nat.card (WeierstrassCurve.Affine F2)=32 := by
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem two_nonsingular_model_count : Nat.card {W : WeierstrassCurve.Affine F2 // W.Δ≠0}=16 := by
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem two_source_model_count :
    Nat.card {W : WeierstrassCurve.Affine F2 // W.Δ≠0 ∧ sourceSupersingular W}=8 := by
  rw [Nat.card_congr sourceModelEquiv,Nat.card_eq_fintype_card]
  decide +kernel

theorem two_source_iff_a1 (W : WeierstrassCurve.Affine F2) (hΔ : W.Δ≠0) :
    sourceSupersingular W ↔ W.a₁=0 :=
  (source_iff_counted W hΔ).trans (two_closed_table W hΔ).1

theorem two_point_cases (W : WeierstrassCurve.Affine F2) (hΔ : W.Δ≠0)
    (hs : sourceSupersingular W) : Nat.card W.Point=1 ∨ Nat.card W.Point=3 ∨ Nat.card W.Point=5 := by
  rw [actual_point_count W hΔ]
  exact (two_closed_table W hΔ).2.1 ((source_iff_counted W hΔ).mp hs)

theorem two_actual_hasse (W : WeierstrassCurve.Affine F2) (hΔ : W.Δ≠0) :
    (SupersingularTraceClassesAudit.fieldTrace W)^2≤4*(Fintype.card F2:ℤ) := by
  rw [actual_trace W hΔ]
  exact (two_closed_table W hΔ).2.2

theorem two_actual_trace_squares (W : WeierstrassCurve.Affine F2) (hΔ : W.Δ≠0)
    (hs : sourceSupersingular W) :
    SupersingularTraceClassesAudit.TraceSquares (Fintype.card F2)
      (SupersingularTraceClassesAudit.fieldTrace W) := by
  rcases two_point_cases W hΔ hs with h | h | h <;>
    norm_num [SupersingularTraceClassesAudit.TraceSquares,SupersingularTraceClassesAudit.fieldTrace,h]

abbrev Closure2 := AlgebraicClosure F2
private instance : DecidableEq Closure2 := Classical.decEq _

theorem two_source_geometric (W : WeierstrassCurve.Affine F2) (hΔ : W.Δ≠0) :
    sourceSupersingular W ↔
      Nat.card (TwoTorsionRankAudit.TwoTorsion (W.baseChange Closure2))=1 := by
  have : W.IsElliptic := SmallCharacteristicJAudit.ellipticOfNonzero W hΔ
  have hc := SmallCharacteristicJAudit.two_geometric_j (L:=Closure2) W
  simp only [W.j_eq_zero_iff_of_char_two] at hc
  rw [two_source_iff_a1 W hΔ,hc]
  by_cases h : W.a₁=0 <;> simp [h]

/-- Every divisor of the actual point count, including composite divisors;
this does not assert that every divisor is realized as a cyclic point order. -/
theorem two_actual_order_bound (W : WeierstrassCurve.Affine F2) (hΔ : W.Δ≠0)
    (hs : sourceSupersingular W) (r : ℕ) (hr : r∣Nat.card W.Point) :
    Nat.Coprime r 2 ∧ orderOf (2:ZMod r)∣4 ∧ orderOf (2:ZMod r)≤4 := by
  have hc := two_point_cases W hΔ hs
  have hcop : Nat.Coprime (Nat.card W.Point) 2 := by
    rcases hc with h | h | h <;> rw [h] <;> decide
  have hd : Nat.card W.Point∣2^4-1 := by
    rcases hc with h | h | h <;> rw [h] <;> decide
  have ho := modular_order_dvd 2 r 4 (by decide) (hr.trans hd)
  exact ⟨hcop.of_dvd_left hr,ho,Nat.le_of_dvd (by decide : 0<(4:ℕ)) ho⟩

#print axioms two_closed_table
#print axioms two_nonsingular_model_count
end
end SmallPrimeTraceGeometryAudit
