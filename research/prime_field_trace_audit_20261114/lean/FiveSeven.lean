import ShortNormalization

namespace PrimeFieldTraceAudit
noncomputable section
open SmallPrimeTraceGeometryAudit SupersingularTraceClassesAudit
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance : Fact (Nat.Prime 7) := ⟨by decide⟩
abbrev F5 := ZMod 5
abbrev F7 := ZMod 7

theorem five_closed_table : ∀ a b : F5,(shortModel a b).Δ≠0 →
    (countedTrace (shortModel a b))^2≤4*(Fintype.card F5:ℤ) ∧
    (countedSupersingular (shortModel a b) ↔ equationCount (shortModel a b)=Fintype.card F5+1) := by
  decide +kernel

theorem seven_closed_table : ∀ a b : F7,(shortModel a b).Δ≠0 →
    (countedTrace (shortModel a b))^2≤4*(Fintype.card F7:ℤ) ∧
    (countedSupersingular (shortModel a b) ↔ equationCount (shortModel a b)=Fintype.card F7+1) := by
  decide +kernel

theorem five_all_models (W : WeierstrassCurve.Affine F5) (hΔ : W.Δ≠0) :
    (fieldTrace W)^2≤20 ∧ (sourceSupersingular W ↔ Nat.card W.Point=6) := by
  have h := transfer_short_table (by decide : (2:F5)≠0) (by decide) five_closed_table W hΔ
  norm_num only [ZMod.card] at h
  exact h

theorem seven_all_models (W : WeierstrassCurve.Affine F7) (hΔ : W.Δ≠0) :
    (fieldTrace W)^2≤28 ∧ (sourceSupersingular W ↔ Nat.card W.Point=8) := by
  have h := transfer_short_table (by decide : (2:F7)≠0) (by decide) seven_closed_table W hΔ
  norm_num only [ZMod.card] at h
  exact h

theorem five_source_iff_trace_zero (W : WeierstrassCurve.Affine F5) (hΔ : W.Δ≠0) :
    sourceSupersingular W ↔ fieldTrace W=0 := by
  rw [(five_all_models W hΔ).2,fieldTrace]
  simp only [ZMod.card]
  omega

theorem seven_source_iff_trace_zero (W : WeierstrassCurve.Affine F7) (hΔ : W.Δ≠0) :
    sourceSupersingular W ↔ fieldTrace W=0 := by
  rw [(seven_all_models W hΔ).2,fieldTrace]
  simp only [ZMod.card]
  omega

theorem five_all_count_divisors (W : WeierstrassCurve.Affine F5) (hΔ : W.Δ≠0)
    (hs : sourceSupersingular W) (r : ℕ) (hr : r∣Nat.card W.Point) :
    Nat.Coprime r 5 ∧ orderOf (5:ZMod r)∣2 ∧
    (2<r → orderOf (5:ZMod r)=2) := by
  have hN := (five_all_models W hΔ).2.mp hs
  have hd : r∣6 := hN ▸ hr
  refine ⟨Nat.Coprime.of_dvd_left hd (by decide : Nat.Coprime 6 5),?_,?_⟩
  · exact modular_order_dvd 5 r 2 (by decide) (hd.trans (by decide : 6∣5^2-1))
  · intro h
    exact composite_trace_zero_order 5 r h hd

theorem seven_all_count_divisors (W : WeierstrassCurve.Affine F7) (hΔ : W.Δ≠0)
    (hs : sourceSupersingular W) (r : ℕ) (hr : r∣Nat.card W.Point) :
    Nat.Coprime r 7 ∧ orderOf (7:ZMod r)∣2 ∧
    (2<r → orderOf (7:ZMod r)=2) := by
  have hN := (seven_all_models W hΔ).2.mp hs
  have hd : r∣8 := hN ▸ hr
  refine ⟨Nat.Coprime.of_dvd_left hd (by decide : Nat.Coprime 8 7),?_,?_⟩
  · exact modular_order_dvd 7 r 2 (by decide) (hd.trans (by decide : 8∣7^2-1))
  · intro h
    exact composite_trace_zero_order 7 r h hd

#print axioms five_all_models
#print axioms seven_all_models
#print axioms five_all_count_divisors
#print axioms seven_all_count_divisors
end
end PrimeFieldTraceAudit
