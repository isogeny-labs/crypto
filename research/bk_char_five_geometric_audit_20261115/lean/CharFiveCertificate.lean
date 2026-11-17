module
public import GeometricDescent

@[expose] public section
namespace BKCharFiveGeometricAudit
noncomputable section
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing F := (inferInstance : Field F).toCommRing

theorem bk_char_five_geometric_certificate :
    Nat.Prime 5 ∧ 5 ∣ Nat.card (curve F).Point ∧ ¬ 5 ∣ (5 : ℕ) - 1 ∧
    (curve F).Δ ≠ 0 ∧ Nat.card (curve F).Point = 5 ∧
    (∀ i s : Bool, addOrderOf (basePoint i s) = 5) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [Algebra F K],
      (∀ P : (geometricCurve K).Point, (5 : ℕ) • P = 0 →
        ∃ Q : (curve F).Point, baseMap K Q = P) ∧
      Nat.card (GeometricFiveTorsion K) = 5) ∧
    allGeometricFiveRational ∧
    ¬ (allGeometricFiveRational ↔ 5 ∣ (5 : ℕ) - 1) ∧
    (∀ k : ℕ, 0 < k → ¬ 5 ∣ 5 ^ k - 1) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [Algebra F K] [Fintype K],
      Nat.card (GeometricFiveTorsion K) = 5 ^ 2 ↔ 5 ∣ Fintype.card K - 1) := by
  refine ⟨by decide, ?_, by decide, base_delta, base_count, base_point_order,
    ?_, geometric_five_rational_closed, inclusion_paraphrase_false,
    no_modular_embedding, ?_⟩
  · rw [base_count]
  · intro K _ _ _
    exact ⟨all_geometric_five_rational K, geometric_five_card K⟩
  · intro K _ _ _ _
    exact original_count_criterion K

#print axioms bk_char_five_geometric_certificate
end
end BKCharFiveGeometricAudit
