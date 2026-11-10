import TorsionBoundaries
import SupersingularTraceClasses

/-! Actual prime-field curves with subgroup primes greater than two.
The finite-field supersingularity predicate is exactly the source's trace
criterion. The modular embedding degree is not silently identified with
rationality of the full torsion over a chosen extension. -/
namespace SmallCharacteristicPrimeAudit
noncomputable section
open TorsionBoundaryAudit SupersingularTraceClassesAudit
private instance primeTwo : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance primeSeven : Fact (Nat.Prime 7) := ⟨by decide⟩

def E2 : WeierstrassCurve.Affine (ZMod 2) := ⟨0,0,1,1,0⟩
def E3 : WeierstrassCurve.Affine (ZMod 3) := ⟨0,0,0,-1,1⟩

theorem E2_nonsingular : E2.Δ≠0 := by
  unfold E2 WeierstrassCurve.Δ WeierstrassCurve.b₂ WeierstrassCurve.b₄
    WeierstrassCurve.b₆ WeierstrassCurve.b₈
  decide +kernel

theorem E3_nonsingular : E3.Δ≠0 := by
  unfold E3 WeierstrassCurve.Δ WeierstrassCurve.b₂ WeierstrassCurve.b₄
    WeierstrassCurve.b₆ WeierstrassCurve.b₈
  decide +kernel

theorem E2_cardinality : Nat.card E2.Point=5 := by
  rw [Nat.card_congr E2.nonsingularPointEquiv]
  change Nat.card (Option {xy : ZMod 2 × ZMod 2 // E2.Nonsingular xy.1 xy.2})=5
  let : DecidablePred (fun xy : ZMod 2 × ZMod 2 => E2.Nonsingular xy.1 xy.2) :=
    fun xy => decidable_of_iff (xy.2^2+xy.2=xy.1^3+xy.1) (by
      change xy.2^2+xy.2=xy.1^3+xy.1 ↔ E2.Nonsingular xy.1 xy.2
      erw [←WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
        E2_nonsingular,WeierstrassCurve.Affine.equation_iff]
      simp [E2])
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem E3_cardinality : Nat.card E3.Point=7 := by
  rw [Nat.card_congr E3.nonsingularPointEquiv]
  change Nat.card (Option {xy : ZMod 3 × ZMod 3 // E3.Nonsingular xy.1 xy.2})=7
  let : DecidablePred (fun xy : ZMod 3 × ZMod 3 => E3.Nonsingular xy.1 xy.2) :=
    fun xy => decidable_of_iff (xy.2^2=xy.1^3-xy.1+1) (by
      change xy.2^2=xy.1^3-xy.1+1 ↔ E3.Nonsingular xy.1 xy.2
      erw [←WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
        E3_nonsingular,WeierstrassCurve.Affine.equation_iff]
      simp [E3])
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem E2_trace : finiteFieldTrace 2 E2= -2 := by
  norm_num [finiteFieldTrace,E2_cardinality]

theorem E3_trace : finiteFieldTrace 3 E3= -3 := by
  norm_num [finiteFieldTrace,E3_cardinality]

theorem E2_source_supersingular : sourceSupersingular 2 E2 := by
  norm_num [sourceSupersingular,E2_trace]

theorem E3_source_supersingular : sourceSupersingular 3 E3 := by
  norm_num [sourceSupersingular,E3_trace]

/-- Keep the equality decision explicit so that the actual point group in
this Cauchy application is definitionally the group in the caller's goal. -/
theorem actual_prime_order_point {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (E : WeierstrassCurve.Affine K) (r : ℕ) [Fact (Nat.Prime r)]
    (hrN : r∣Nat.card E.Point) : ∃ P : E.Point,addOrderOf P=r := by
  let : Finite (WithZero {xy : K × K // E.Nonsingular xy.1 xy.2}) :=
    inferInstanceAs (Finite (Option {xy : K × K // E.Nonsingular xy.1 xy.2}))
  let : Finite E.Point := Finite.of_injective E.nonsingularPointEquiv
    E.nonsingularPointEquiv.injective
  exact exists_prime_addOrderOf_dvd_card' r hrN

theorem E2_order_five_point : ∃ P : E2.Point,addOrderOf P=5 := by
  apply actual_prime_order_point E2 5
  rw [E2_cardinality]

theorem E3_order_seven_point : ∃ P : E3.Point,addOrderOf P=7 := by
  apply actual_prime_order_point E3 7
  rw [E3_cardinality]

theorem E2_embedding_degree : embeddingDegree 2 5=4 := by
  exact orderOf_eq_prime_pow (p:=2) (n:=1) (by decide +kernel) (by decide +kernel)

theorem E3_embedding_degree : embeddingDegree 3 7=6 := by
  apply (orderOf_eq_iff (by decide : 0<6)).mpr
  constructor
  · decide +kernel
  · intro m hm hpos
    interval_cases m <;> decide +kernel

/-- The prime-field degree-two reading fails even with r>2 when the
small characteristics are not excluded. This uses an actual curve/group. -/
theorem prime_field_degree_two_without_characteristic_exclusion_false :
    ¬ ∀ (p r : ℕ) (E : WeierstrassCurve.Affine (ZMod p)),
      Nat.Prime p → Nat.Prime r → 2<r → Nat.Coprime r p → E.Δ≠0 →
      sourceSupersingular p E → r∣Nat.card E.Point → embeddingDegree p r=2 := by
  intro h
  have hk := h 2 5 E2 (by decide) (by decide) (by decide) (by decide)
    E2_nonsingular E2_source_supersingular (by rw [E2_cardinality])
  rw [E2_embedding_degree] at hk
  contradiction

/-- Excluding characteristic two alone still leaves the characteristic-three
case. The subgroup prime is seven, not a small r=2 exception. -/
theorem prime_field_degree_two_with_p_greater_two_false :
    ¬ ∀ (p r : ℕ) (E : WeierstrassCurve.Affine (ZMod p)),
      Nat.Prime p → 2<p → Nat.Prime r → 2<r → Nat.Coprime r p → E.Δ≠0 →
      sourceSupersingular p E → r∣Nat.card E.Point → embeddingDegree p r=2 := by
  intro h
  have hk := h 3 7 E3 (by decide) (by decide) (by decide) (by decide) (by decide)
    E3_nonsingular E3_source_supersingular (by rw [E3_cardinality])
  rw [E3_embedding_degree] at hk
  contradiction

theorem small_characteristic_audit_certificate :
    E2.Δ≠0 ∧ Nat.card E2.Point=5 ∧ sourceSupersingular 2 E2 ∧
    (∃ P : E2.Point,addOrderOf P=5) ∧ embeddingDegree 2 5=4 ∧
    E3.Δ≠0 ∧ Nat.card E3.Point=7 ∧ sourceSupersingular 3 E3 ∧
    (∃ P : E3.Point,addOrderOf P=7) ∧ embeddingDegree 3 7=6 ∧
    ¬ (∀ (p r : ℕ) (E : WeierstrassCurve.Affine (ZMod p)),
      Nat.Prime p → 2<p → Nat.Prime r → 2<r → Nat.Coprime r p → E.Δ≠0 →
      sourceSupersingular p E → r∣Nat.card E.Point → embeddingDegree p r=2) := by
  exact ⟨E2_nonsingular,E2_cardinality,E2_source_supersingular,E2_order_five_point,
    E2_embedding_degree,E3_nonsingular,E3_cardinality,E3_source_supersingular,
    E3_order_seven_point,E3_embedding_degree,
    prime_field_degree_two_with_p_greater_two_false⟩

#print axioms E2_cardinality
#print axioms E3_cardinality
#print axioms prime_field_degree_two_without_characteristic_exclusion_false
#print axioms prime_field_degree_two_with_p_greater_two_false
#print axioms small_characteristic_audit_certificate
end
end SmallCharacteristicPrimeAudit
