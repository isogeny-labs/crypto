import PrimeThree

namespace SmallPrimeTraceGeometryAudit
noncomputable section
open SupersingularTraceClassesAudit
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

def quarticControl : WeierstrassCurve.Affine F2 := ⟨0,0,1,1,0⟩
def sexticControl : WeierstrassCurve.Affine F3 := ⟨0,0,0,2,1⟩

theorem quartic_delta : quarticControl.Δ≠0 := by decide +kernel
theorem sextic_delta : sexticControl.Δ≠0 := by decide +kernel

theorem quartic_count : Nat.card quarticControl.Point=5 := by
  rw [actual_point_count quarticControl quartic_delta]
  decide +kernel

theorem sextic_count : Nat.card sexticControl.Point=7 := by
  rw [actual_point_count sexticControl sextic_delta]
  decide +kernel

theorem quartic_supersingular : sourceSupersingular quarticControl := by
  rw [two_source_iff_a1 quarticControl quartic_delta]
  rfl

theorem sextic_supersingular : sourceSupersingular sexticControl := by
  rw [three_source_iff_b2 sexticControl sextic_delta]
  rfl

def quarticPoint : quarticControl.Point := .some 0 0
  ((quarticControl.equation_iff_nonsingular_of_Δ_ne_zero quartic_delta).mp (by
    rw [WeierstrassCurve.Affine.equation_iff]
    decide +kernel))

def sexticPoint : sexticControl.Point := .some 0 1
  ((sexticControl.equation_iff_nonsingular_of_Δ_ne_zero sextic_delta).mp (by
    rw [WeierstrassCurve.Affine.equation_iff]
    decide +kernel))

private theorem prime_card_point_order {G : Type*} [AddCommGroup G]
    (r : ℕ) (hr : Nat.Prime r) (hc : Nat.card G=r) (P : G) (hP : P≠0) : addOrderOf P=r := by
  have hd : addOrderOf P∣r := hc ▸ addOrderOf_dvd_natCard P
  rcases hr.eq_one_or_self_of_dvd _ hd with h | h
  · exact False.elim (hP (AddMonoid.addOrderOf_eq_one_iff.mp h))
  · exact h

theorem quartic_point_order : addOrderOf quarticPoint=5 :=
  prime_card_point_order 5 (by decide) quartic_count quarticPoint (WeierstrassCurve.Affine.Point.some_ne_zero _)

theorem sextic_point_order : addOrderOf sexticPoint=7 :=
  prime_card_point_order 7 (by decide) sextic_count sexticPoint (WeierstrassCurve.Affine.Point.some_ne_zero _)

theorem quartic_exact_order : orderOf (2:ZMod 5)=4 := by
  apply (orderOf_eq_iff (by decide : 0<(4:ℕ))).mpr
  refine ⟨by decide +kernel,?_⟩
  intro m hm hp
  have h : ∀ n : Fin 4,0<n.val → (2:ZMod 5)^n.val≠1 := by decide +kernel
  exact h ⟨m,hm⟩ hp

theorem sextic_exact_order : orderOf (3:ZMod 7)=6 := by
  apply (orderOf_eq_iff (by decide : 0<(6:ℕ))).mpr
  refine ⟨by decide +kernel,?_⟩
  intro m hm hp
  have h : ∀ n : Fin 6,0<n.val → (3:ZMod 7)^n.val≠1 := by decide +kernel
  exact h ⟨m,hm⟩ hp

theorem quartic_actual_control :
    quarticControl.Δ≠0 ∧ sourceSupersingular quarticControl ∧
    Nat.card quarticControl.Point=5 ∧
    Nat.card (TwoTorsionRankAudit.TwoTorsion (quarticControl.baseChange Closure2))=1 ∧
    addOrderOf quarticPoint=5 ∧ orderOf (2:ZMod 5)=4 := by
  refine ⟨quartic_delta,quartic_supersingular,quartic_count,
    (two_source_geometric quarticControl quartic_delta).mp quartic_supersingular,?_,quartic_exact_order⟩
  exact quartic_point_order

theorem sextic_actual_control :
    sexticControl.Δ≠0 ∧ sourceSupersingular sexticControl ∧
    Nat.card sexticControl.Point=7 ∧
    Nat.card (ThreeTorsionRankAudit.General.ThreeTorsion (sexticControl.baseChange Closure3))=1 ∧
    addOrderOf sexticPoint=7 ∧ orderOf (3:ZMod 7)=6 := by
  refine ⟨sextic_delta,sextic_supersingular,sextic_count,
    (three_source_geometric sexticControl sextic_delta).mp sextic_supersingular,?_,sextic_exact_order⟩
  exact sextic_point_order

/-- Complete actual-model small-field census and semantic couplings, with
positive controls. Every statement is bounded to F2/F3 or is the explicit
generic equation-count bridge; no general Hasse/classification is assumed. -/
theorem small_prime_trace_geometry_certificate :
    (Nat.card (WeierstrassCurve.Affine F2)=32 ∧
      Nat.card {W : WeierstrassCurve.Affine F2 // W.Δ≠0}=16 ∧
      Nat.card (WeierstrassCurve.Affine F3)=243 ∧
      Nat.card {W : WeierstrassCurve.Affine F3 // W.Δ≠0}=162 ∧
      Nat.card {W : WeierstrassCurve.Affine F2 // W.Δ≠0 ∧ sourceSupersingular W}=8 ∧
      Nat.card {W : WeierstrassCurve.Affine F3 // W.Δ≠0 ∧ sourceSupersingular W}=54) ∧
    (∀ (K : Type) [Field K] [Fintype K] [DecidableEq K]
      (W : WeierstrassCurve.Affine K),W.Δ≠0 →
      Nat.card W.Point=equationCount W ∧ fieldTrace W=countedTrace W) ∧
    (∀ W : WeierstrassCurve.Affine F2,W.Δ≠0 →
      (sourceSupersingular W ↔ W.a₁=0) ∧
      (sourceSupersingular W ↔
        Nat.card (TwoTorsionRankAudit.TwoTorsion (W.baseChange Closure2))=1) ∧
      (fieldTrace W)^2≤4*(Fintype.card F2:ℤ) ∧
      (sourceSupersingular W →
        (Nat.card W.Point=1 ∨ Nat.card W.Point=3 ∨ Nat.card W.Point=5) ∧
        TraceSquares (Fintype.card F2) (fieldTrace W) ∧
        ∀ r : ℕ,r∣Nat.card W.Point → Nat.Coprime r 2 ∧ orderOf (2:ZMod r)∣4 ∧
          orderOf (2:ZMod r)≤4)) ∧
    (∀ W : WeierstrassCurve.Affine F3,W.Δ≠0 →
      (sourceSupersingular W ↔ W.b₂=0) ∧
      (sourceSupersingular W ↔
        Nat.card (ThreeTorsionRankAudit.General.ThreeTorsion (W.baseChange Closure3))=1) ∧
      (fieldTrace W)^2≤4*(Fintype.card F3:ℤ) ∧
      (sourceSupersingular W →
        (Nat.card W.Point=1 ∨ Nat.card W.Point=4 ∨ Nat.card W.Point=7) ∧
        TraceSquares (Fintype.card F3) (fieldTrace W) ∧
        ∀ r : ℕ,r∣Nat.card W.Point → Nat.Coprime r 3 ∧ orderOf (3:ZMod r)∣6 ∧
          orderOf (3:ZMod r)≤6)) ∧
    (quarticControl.Δ≠0 ∧ sourceSupersingular quarticControl ∧
      Nat.card quarticControl.Point=5 ∧
      Nat.card (TwoTorsionRankAudit.TwoTorsion (quarticControl.baseChange Closure2))=1 ∧
      addOrderOf quarticPoint=5 ∧ orderOf (2:ZMod 5)=4) ∧
    (sexticControl.Δ≠0 ∧ sourceSupersingular sexticControl ∧
      Nat.card sexticControl.Point=7 ∧
      Nat.card (ThreeTorsionRankAudit.General.ThreeTorsion (sexticControl.baseChange Closure3))=1 ∧
      addOrderOf sexticPoint=7 ∧ orderOf (3:ZMod 7)=6) := by
  refine ⟨⟨two_model_count,two_nonsingular_model_count,three_model_count,three_nonsingular_model_count,
      two_source_model_count,three_source_model_count⟩,
    ?_,?_,?_,quartic_actual_control,sextic_actual_control⟩
  · intro K _ _ _ W hΔ
    exact ⟨actual_point_count W hΔ,actual_trace W hΔ⟩
  · intro W hΔ
    refine ⟨two_source_iff_a1 W hΔ,two_source_geometric W hΔ,two_actual_hasse W hΔ,?_⟩
    intro hs
    exact ⟨two_point_cases W hΔ hs,two_actual_trace_squares W hΔ hs,two_actual_order_bound W hΔ hs⟩
  · intro W hΔ
    refine ⟨three_source_iff_b2 W hΔ,three_source_geometric W hΔ,three_actual_hasse W hΔ,?_⟩
    intro hs
    exact ⟨three_point_cases W hΔ hs,three_actual_trace_squares W hΔ hs,three_actual_order_bound W hΔ hs⟩

#print axioms quartic_actual_control
#print axioms sextic_actual_control
#print axioms small_prime_trace_geometry_certificate
end
end SmallPrimeTraceGeometryAudit
