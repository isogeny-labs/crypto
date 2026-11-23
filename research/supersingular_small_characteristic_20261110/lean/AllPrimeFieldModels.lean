import PrimeFieldControls

namespace SmallCharacteristicCensusAudit
open SmallCharacteristicPrimeAudit TorsionBoundaryAudit SupersingularTraceClassesAudit

abbrev equationTest {K : Type*} [Field K]
    (E : WeierstrassCurve.Affine K) (xy : K × K) : Prop :=
  xy.2^2+E.a₁*xy.1*xy.2+E.a₃*xy.2=
    xy.1^3+E.a₂*xy.1^2+E.a₄*xy.1+E.a₆

/-- This count is executable from field operations and finite coordinates.
The following theorem, not a producer assertion, binds it to actual points. -/
def counted {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (E : WeierstrassCurve.Affine K) : ℕ :=
  Fintype.card (Option {xy : K × K // equationTest E xy})

theorem actual_cardinality_eq_counted {K : Type*} [Field K] [Fintype K]
    [DecidableEq K] (E : WeierstrassCurve.Affine K) (hΔ : E.Δ≠0) :
    Nat.card E.Point=counted E := by
  rw [Nat.card_congr E.nonsingularPointEquiv]
  change Nat.card (Option {xy : K × K // E.Nonsingular xy.1 xy.2})=counted E
  let e : {xy : K × K // E.Nonsingular xy.1 xy.2} ≃
      {xy : K × K // equationTest E xy} := Equiv.subtypeEquivRight fun xy => by
    change E.Nonsingular xy.1 xy.2 ↔
      xy.2^2+E.a₁*xy.1*xy.2+E.a₃*xy.2=
        xy.1^3+E.a₂*xy.1^2+E.a₄*xy.1+E.a₆
    rw [←WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ,
      WeierstrassCurve.Affine.equation_iff]
  rw [Nat.card_congr (Equiv.optionCongr e)]
  exact Nat.card_eq_fintype_card

private instance primeTwo : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩

theorem coefficient_tuple_counts :
    Fintype.card (ZMod 2 × ZMod 2 × ZMod 2 × ZMod 2 × ZMod 2)=32 ∧
    Fintype.card (ZMod 3 × ZMod 3 × ZMod 3 × ZMod 3 × ZMod 3)=243 := by
  decide +kernel

theorem all_binary_models : ∀ a₁ a₂ a₃ a₄ a₆ : ZMod 2,
    let E : WeierstrassCurve.Affine (ZMod 2) := ⟨a₁,a₂,a₃,a₄,a₆⟩
    E.Δ≠0 → Int.gcd ((3:ℤ)-(counted E:ℤ)) 2≠1 →
    counted E=1 ∨ counted E=3 ∨ counted E=5 := by decide +kernel

theorem all_ternary_models : ∀ a₁ a₂ a₃ a₄ a₆ : ZMod 3,
    let E : WeierstrassCurve.Affine (ZMod 3) := ⟨a₁,a₂,a₃,a₄,a₆⟩
    E.Δ≠0 → Int.gcd ((4:ℤ)-(counted E:ℤ)) 3≠1 →
    counted E=1 ∨ counted E=4 ∨ counted E=7 := by decide +kernel

theorem actual_binary_count_classification (E : WeierstrassCurve.Affine (ZMod 2))
    (hΔ : E.Δ≠0) (hs : sourceSupersingular 2 E) :
    Nat.card E.Point=1 ∨ Nat.card E.Point=3 ∨ Nat.card E.Point=5 := by
  have hc := actual_cardinality_eq_counted E hΔ
  unfold sourceSupersingular finiteFieldTrace at hs
  rw [hc] at hs ⊢
  cases E with
  | mk a₁ a₂ a₃ a₄ a₆ => exact all_binary_models a₁ a₂ a₃ a₄ a₆ hΔ hs

theorem actual_ternary_count_classification (E : WeierstrassCurve.Affine (ZMod 3))
    (hΔ : E.Δ≠0) (hs : sourceSupersingular 3 E) :
    Nat.card E.Point=1 ∨ Nat.card E.Point=4 ∨ Nat.card E.Point=7 := by
  have hc := actual_cardinality_eq_counted E hΔ
  unfold sourceSupersingular finiteFieldTrace at hs
  rw [hc] at hs ⊢
  cases E with
  | mk a₁ a₂ a₃ a₄ a₆ => exact all_ternary_models a₁ a₂ a₃ a₄ a₆ hΔ hs

theorem binary_prime_divisor_bound (E : WeierstrassCurve.Affine (ZMod 2))
    (hΔ : E.Δ≠0) (hs : sourceSupersingular 2 E)
    (r : ℕ) (hr : Nat.Prime r) (hrN : r∣Nat.card E.Point) : embeddingDegree 2 r≤6 := by
  have hc : TraceSquares 2 ((2:ℤ)+1-(Nat.card E.Point:ℤ)) := by
    rcases actual_binary_count_classification E hΔ hs with h | h | h <;>
      rw [h] <;> norm_num [TraceSquares]
  have hh := trace_count_order_cases 2 (Nat.card E.Point) r hr hrN hc
  unfold embeddingDegree
  rcases hh with h | h | h | h | h <;> omega

theorem ternary_prime_divisor_bound (E : WeierstrassCurve.Affine (ZMod 3))
    (hΔ : E.Δ≠0) (hs : sourceSupersingular 3 E)
    (r : ℕ) (hr : Nat.Prime r) (hrN : r∣Nat.card E.Point) : embeddingDegree 3 r≤6 := by
  have hc : TraceSquares 3 ((3:ℤ)+1-(Nat.card E.Point:ℤ)) := by
    rcases actual_ternary_count_classification E hΔ hs with h | h | h <;>
      rw [h] <;> norm_num [TraceSquares]
  have hh := trace_count_order_cases 3 (Nat.card E.Point) r hr hrN hc
  unfold embeddingDegree
  rcases hh with h | h | h | h | h <;> omega

theorem all_prime_field_models_audit_certificate :
    (∀ E : WeierstrassCurve.Affine (ZMod 2), E.Δ≠0 → sourceSupersingular 2 E →
      (Nat.card E.Point=1 ∨ Nat.card E.Point=3 ∨ Nat.card E.Point=5) ∧
      ∀ r : ℕ, Nat.Prime r → r∣Nat.card E.Point → embeddingDegree 2 r≤6) ∧
    (∀ E : WeierstrassCurve.Affine (ZMod 3), E.Δ≠0 → sourceSupersingular 3 E →
      (Nat.card E.Point=1 ∨ Nat.card E.Point=4 ∨ Nat.card E.Point=7) ∧
      ∀ r : ℕ, Nat.Prime r → r∣Nat.card E.Point → embeddingDegree 3 r≤6) ∧
    (E2.Δ≠0 ∧ Nat.card E2.Point=5 ∧ sourceSupersingular 2 E2 ∧
      (∃ P : E2.Point,addOrderOf P=5) ∧ embeddingDegree 2 5=4) ∧
    (E3.Δ≠0 ∧ Nat.card E3.Point=7 ∧ sourceSupersingular 3 E3 ∧
      (∃ P : E3.Point,addOrderOf P=7) ∧ embeddingDegree 3 7=6) ∧
    ¬ (∀ (p r : ℕ) (E : WeierstrassCurve.Affine (ZMod p)),
      Nat.Prime p → 2<p → Nat.Prime r → 2<r → Nat.Coprime r p → E.Δ≠0 →
      sourceSupersingular p E → r∣Nat.card E.Point → embeddingDegree p r=2) := by
  refine ⟨?_,?_,⟨E2_nonsingular,E2_cardinality,E2_source_supersingular,
    E2_order_five_point,E2_embedding_degree⟩,
    ⟨E3_nonsingular,E3_cardinality,E3_source_supersingular,
      E3_order_seven_point,E3_embedding_degree⟩,
    prime_field_degree_two_with_p_greater_two_false⟩
  · intro E hΔ hs
    exact ⟨actual_binary_count_classification E hΔ hs,binary_prime_divisor_bound E hΔ hs⟩
  · intro E hΔ hs
    exact ⟨actual_ternary_count_classification E hΔ hs,ternary_prime_divisor_bound E hΔ hs⟩

#print axioms actual_cardinality_eq_counted
#print axioms all_binary_models
#print axioms all_ternary_models
#print axioms all_prime_field_models_audit_certificate
end SmallCharacteristicCensusAudit
