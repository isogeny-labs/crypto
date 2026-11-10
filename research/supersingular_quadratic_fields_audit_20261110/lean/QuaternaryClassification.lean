import FibreBound
import SourceTrace

namespace QuaternaryClassificationAudit
open SmallCharacteristicCensusAudit QuadraticFieldModelsAudit WeierstrassFibreBoundAudit
  TorsionBoundaryAudit SupersingularTraceClassesAudit FiniteFieldSourceTraceAudit

theorem count_cases (E : WeierstrassCurve.Affine F4)
    (hs : Int.gcd ((5:ℤ)-(counted E:ℤ)) 4≠1) :
    counted E=1 ∨ counted E=3 ∨ counted E=5 ∨ counted E=7 ∨ counted E=9 := by
  have hb:=counted_le_two_card_add_one E
  rw [field_cardinalities.1] at hb
  have hp : 0<counted E := by
    unfold counted
    rw [Fintype.card_option]
    omega
  generalize hN : counted E=N at hs hb hp ⊢
  interval_cases N <;> norm_num at hs <;> norm_num

theorem actual_count_cases (E : WeierstrassCurve.Affine F4)
    (hΔ : E.Δ≠0) (hs : SourceSupersingular E) :
    Nat.card E.Point=1 ∨ Nat.card E.Point=3 ∨ Nat.card E.Point=5 ∨
      Nat.card E.Point=7 ∨ Nat.card E.Point=9 := by
  have hc:=actual_cardinality_eq_counted E hΔ
  unfold SourceSupersingular fieldTrace at hs
  rw [field_cardinalities.1,hc] at hs
  rw [hc]
  exact count_cases E (by simpa using hs)

theorem actual_trace_class (E : WeierstrassCurve.Affine F4)
    (hΔ : E.Δ≠0) (hs : SourceSupersingular E) :
    TraceSquares 4 ((4:ℤ)+1-(Nat.card E.Point:ℤ)) := by
  rcases actual_count_cases E hΔ hs with h|h|h|h|h <;>
    rw [h] <;> norm_num [TraceSquares]

theorem prime_divisor_bound (E : WeierstrassCurve.Affine F4)
    (hΔ : E.Δ≠0) (hs : SourceSupersingular E)
    (r : ℕ) (hr : Nat.Prime r) (hrN : r∣Nat.card E.Point) :
    embeddingDegree 4 r≤6 ∧ Nat.Coprime r 4 ∧ ∃ P : E.Point,addOrderOf P=r := by
  have hc:=actual_trace_class E hΔ hs
  have hh:=trace_count_order_cases 4 (Nat.card E.Point) r hr hrN hc
  let : Fact (Nat.Prime r) := ⟨hr⟩
  refine ⟨?_,trace_count_coprime _ _ r hr hrN hc,
    SmallCharacteristicPrimeAudit.actual_prime_order_point E r hrN⟩
  unfold embeddingDegree
  rcases hh with h|h|h|h|h <;> omega

#print axioms actual_count_cases
#print axioms prime_divisor_bound
end QuaternaryClassificationAudit
