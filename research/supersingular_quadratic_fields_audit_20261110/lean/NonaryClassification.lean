import CompleteSquare
import SourceTrace
import NormalizedAll

namespace NonaryClassificationAudit
open SmallCharacteristicCensusAudit QuadraticFieldModelsAudit
  CharacteristicThreeNormalizationAudit NonaryNormalizedAllAudit
  TorsionBoundaryAudit SupersingularTraceClassesAudit FiniteFieldSourceTraceAudit

theorem count_cases (E : WeierstrassCurve.Affine F9)
    (hs : Int.gcd ((10:ℤ)-(counted E:ℤ)) 9≠1) :
    counted E=4 ∨ counted E=7 ∨ counted E=10 ∨ counted E=13 ∨ counted E=16 := by
  rw [counted_normalize E] at hs ⊢
  exact all_normalized_models (E.a₂+E.a₁^2) (E.a₄-E.a₁*E.a₃) (E.a₆+E.a₃^2) hs

theorem actual_count_cases (E : WeierstrassCurve.Affine F9)
    (hΔ : E.Δ≠0) (hs : SourceSupersingular E) :
    Nat.card E.Point=4 ∨ Nat.card E.Point=7 ∨ Nat.card E.Point=10 ∨
      Nat.card E.Point=13 ∨ Nat.card E.Point=16 := by
  have hc:=actual_cardinality_eq_counted E hΔ
  unfold SourceSupersingular fieldTrace at hs
  rw [field_cardinalities.2,hc] at hs
  rw [hc]
  exact count_cases E (by simpa using hs)

theorem actual_trace_class (E : WeierstrassCurve.Affine F9)
    (hΔ : E.Δ≠0) (hs : SourceSupersingular E) :
    TraceSquares 9 ((9:ℤ)+1-(Nat.card E.Point:ℤ)) := by
  rcases actual_count_cases E hΔ hs with h|h|h|h|h <;>
    rw [h] <;> norm_num [TraceSquares]

theorem prime_divisor_bound (E : WeierstrassCurve.Affine F9)
    (hΔ : E.Δ≠0) (hs : SourceSupersingular E)
    (r : ℕ) (hr : Nat.Prime r) (hrN : r∣Nat.card E.Point) :
    embeddingDegree 9 r≤6 ∧ Nat.Coprime r 9 ∧ ∃ P : E.Point,addOrderOf P=r := by
  have hc:=actual_trace_class E hΔ hs
  have hh:=trace_count_order_cases 9 (Nat.card E.Point) r hr hrN hc
  let : Fact (Nat.Prime r) := ⟨hr⟩
  refine ⟨?_,trace_count_coprime _ _ r hr hrN hc,
    SmallCharacteristicPrimeAudit.actual_prime_order_point E r hrN⟩
  unfold embeddingDegree
  rcases hh with h|h|h|h|h <;> omega

#print axioms actual_count_cases
#print axioms prime_divisor_bound
end NonaryClassificationAudit
