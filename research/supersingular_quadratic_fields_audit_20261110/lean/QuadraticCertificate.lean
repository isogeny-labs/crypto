import QuaternaryClassification
import NonaryClassification
import ExtensionControls

namespace QuadraticFieldCertificateAudit
open QuadraticFieldModelsAudit FiniteFieldSourceTraceAudit QuadraticExtensionControlsAudit
  TorsionBoundaryAudit

theorem quadratic_field_model_audit_certificate :
    Fintype.card F4=4 ∧ Fintype.card F9=9 ∧
    Fintype.card (F4×F4×F4×F4×F4)=1024 ∧
    Fintype.card (F9×F9×F9×F9×F9)=59049 ∧
    (∀ E : WeierstrassCurve.Affine F4,E.Δ≠0 → SourceSupersingular E →
      (Nat.card E.Point=1 ∨ Nat.card E.Point=3 ∨ Nat.card E.Point=5 ∨
        Nat.card E.Point=7 ∨ Nat.card E.Point=9) ∧
      ∀ r : ℕ,Nat.Prime r → r∣Nat.card E.Point →
        embeddingDegree 4 r≤6 ∧ Nat.Coprime r 4 ∧ ∃ P : E.Point,addOrderOf P=r) ∧
    (∀ E : WeierstrassCurve.Affine F9,E.Δ≠0 → SourceSupersingular E →
      (Nat.card E.Point=4 ∨ Nat.card E.Point=7 ∨ Nat.card E.Point=10 ∨
        Nat.card E.Point=13 ∨ Nat.card E.Point=16) ∧
      ∀ r : ℕ,Nat.Prime r → r∣Nat.card E.Point →
        embeddingDegree 9 r≤6 ∧ Nat.Coprime r 9 ∧ ∃ P : E.Point,addOrderOf P=r) ∧
    (E4.Δ≠0 ∧ Nat.card E4.Point=7 ∧ SourceSupersingular E4 ∧
      (∃ P : E4.Point,addOrderOf P=7) ∧ embeddingDegree 4 7=3) ∧
    (E9.Δ≠0 ∧ Nat.card E9.Point=13 ∧ SourceSupersingular E9 ∧
      (∃ P : E9.Point,addOrderOf P=13) ∧ embeddingDegree 9 13=3) := by
  refine ⟨field_cardinalities.1,field_cardinalities.2,coefficient_population_sizes.1,
    coefficient_population_sizes.2,?_,?_,?_,?_⟩
  · intro E hΔ hs
    exact ⟨QuaternaryClassificationAudit.actual_count_cases E hΔ hs,
      QuaternaryClassificationAudit.prime_divisor_bound E hΔ hs⟩
  · intro E hΔ hs
    exact ⟨NonaryClassificationAudit.actual_count_cases E hΔ hs,
      NonaryClassificationAudit.prime_divisor_bound E hΔ hs⟩
  · exact ⟨E4_nonsingular,E4_cardinality,E4_supersingular,E4_prime_order_point,E4_embedding_degree⟩
  · exact ⟨E9_nonsingular,E9_cardinality,E9_supersingular,E9_prime_order_point,E9_embedding_degree⟩

#print axioms quadratic_field_model_audit_certificate
end QuadraticFieldCertificateAudit
