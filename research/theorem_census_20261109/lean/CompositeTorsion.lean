import SourceFSTBoundary

/-! Composite subgroup orders in the prime-field trace argument.
FST2010 p230 Definition2.1/Remark2.2 permits an integer subgroup order.
Hasse is an explicit premise, not an axiom or a claimed proved bridge. -/
namespace CompositeTorsionAudit
open TorsionBoundaryAudit

theorem trace_zero_integer_embedding_degree (q r : ℕ) (hr : 2<r)
    (hd : r ∣ q+1) : embeddingDegree q r=2 := by
  have hz : (q : ZMod r)+1=0 := by
    simpa only [Nat.cast_add,Nat.cast_one] using
      (ZMod.natCast_eq_zero_iff (q+1) r).mpr hd
  have hq : (q : ZMod r)= -1 := (eq_neg_iff_add_eq_zero).mpr hz
  unfold embeddingDegree
  apply orderOf_eq_prime
  · rw [hq]
    ring
  · rw [hq]
    intro hh
    have htwo : (2 : ZMod r)=0 := by linear_combination -hh
    have hdiv : r ∣ 2 := (ZMod.natCast_eq_zero_iff 2 r).mp htwo
    exact (not_le.mpr hr) (Nat.le_of_dvd (by decide) hdiv)

/-- Composite r is allowed. The actual curve cardinality/source trace
are used, but the general Hasse bound remains an explicit premise. -/
theorem corrected_integer_embedding_degree (p r : ℕ)
    (hp : Nat.Prime p) (hp5 : 5≤p) (hr : 2<r)
    (E : WeierstrassCurve.Affine (ZMod p)) (_hΔ : E.Δ≠0)
    (hss : sourceSupersingular p E)
    (hHasse : (finiteFieldTrace p E)^2≤4*(p:ℤ))
    (hrN : r ∣ Nat.card E.Point) : embeddingDegree p r=2 := by
  have ht := hasse_trace_zero p (finiteFieldTrace p E) hp5
    (prime_non_coprime_trace_divisible p _ hp hss) hHasse
  have hN : Nat.card E.Point=p+1 := by
    unfold finiteFieldTrace at ht
    omega
  exact trace_zero_integer_embedding_degree p r hr (hN ▸ hrN)

def eightPoint : FSTBoundaryAudit.curve.Point := .some 3 3 (by
  rw [WeierstrassCurve.Affine.nonsingular_iff]
  constructor
  · rw [WeierstrassCurve.Affine.equation_iff]; decide
  · decide)

theorem eightPoint_killed_eight : (8:ℕ) • eightPoint=0 := by decide +kernel
theorem eightPoint_not_killed_four : (4:ℕ) • eightPoint≠0 := by decide +kernel

theorem eightPoint_order : addOrderOf eightPoint=8 := by
  exact addOrderOf_eq_prime_pow (p:=2) (n:=2)
    eightPoint_not_killed_four eightPoint_killed_eight

theorem eightPoint_embedding_degree : embeddingDegree 7 8=2 := by
  exact trace_zero_integer_embedding_degree 7 8 (by decide) (by decide)

/-- A faithful actual composite-order control. It asserts the modular
embedding order, without silently formalizing the general cyclotomic-field
or full geometric torsion equivalence. -/
theorem actual_composite_order_control :
    FSTBoundaryAudit.curve.Δ≠0 ∧
      sourceSupersingular 7 FSTBoundaryAudit.curve ∧
      Nat.card FSTBoundaryAudit.curve.Point=8 ∧
      addOrderOf eightPoint=8 ∧ embeddingDegree 7 8=2 :=
  ⟨FSTBoundaryAudit.curve_nonsingular,FSTBoundaryAudit.curve_source_supersingular,
    FSTBoundaryAudit.curve_cardinality,eightPoint_order,eightPoint_embedding_degree⟩

/-- One explicit binding for both the universal conditional statement and
its unconditional actual composite-order positive control. -/
theorem composite_order_audit_certificate :
    (∀ (p r : ℕ) (E : WeierstrassCurve.Affine (ZMod p)),
      Nat.Prime p → 5≤p → 2<r → E.Δ≠0 → sourceSupersingular p E →
      (finiteFieldTrace p E)^2≤4*(p:ℤ) → r ∣ Nat.card E.Point →
      embeddingDegree p r=2) ∧
    (FSTBoundaryAudit.curve.Δ≠0 ∧
      sourceSupersingular 7 FSTBoundaryAudit.curve ∧
      Nat.card FSTBoundaryAudit.curve.Point=8 ∧
      addOrderOf eightPoint=8 ∧ embeddingDegree 7 8=2) := by
  refine ⟨?_,actual_composite_order_control⟩
  intro p r E hp hp5 hr hΔ hss hHasse hrN
  exact corrected_integer_embedding_degree p r hp hp5 hr E hΔ hss hHasse hrN

#print axioms corrected_integer_embedding_degree
#print axioms actual_composite_order_control
#print axioms composite_order_audit_certificate
end CompositeTorsionAudit
