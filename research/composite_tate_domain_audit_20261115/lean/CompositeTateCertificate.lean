module
public import PairingObstruction

@[expose] public section
namespace CompositeTateDomainAudit
noncomputable section
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing F := (inferInstance : Field F).toCommRing

theorem known_composite_tate_control_certificate :
    LiteralGuard ∧ Nat.card G = 6 ∧ addOrderOf threePoint = 3 ∧
    negativeUnit ≠ 1 ∧ negativeUnit ^ 6 = 1 ∧ orderOf negativeUnit = 2 ∧
    (¬ ∃ u : Fˣ, IsPrimitiveRoot u 6) ∧
    Nat.card LeftDomain = 6 ∧ Nat.card RightDomain = 6 ∧ Nat.card Target = 2 ∧
    leftPoint ≠ 1 ∧ rightPoint ≠ 1 ∧
    (∀ B : BilinearMap,
      (∀ Q : RightDomain, B leftPoint Q = 1) ∧
      (∀ P : LeftDomain, B P rightPoint = 1) ∧
      ¬ LeftNondegenerate B ∧ ¬ RightNondegenerate B) ∧
    (¬ ∃ B : BilinearMap, LeftNondegenerate B ∧ RightNondegenerate B) ∧
    (¬ (LiteralGuard → ∃ B : BilinearMap, LeftNondegenerate B ∧ RightNondegenerate B)) ∧
    (∀ (M : Type) [CommMonoid M] (ell : ℕ), Nat.Prime ell →
      ∀ u : M, u ^ ell = 1 → u ≠ 1 → IsPrimitiveRoot u ell) := by
  refine ⟨literal_guard_holds, curve_count, three_point_order,
    negative_unit_nontrivial, negative_unit_sixth, negative_unit_order,
    no_primitive_six, left_domain_card, right_domain_card, target_card,
    left_point_nontrivial, right_point_nontrivial, ?_, bilinear_perfectness_impossible,
    literal_nontrivial_root_nondegeneracy_false, ?_⟩
  · intro B
    exact ⟨left_point_radical B, right_point_radical B,
      left_nondegeneracy_impossible B, right_nondegeneracy_impossible B⟩
  · intro M _ ell hell u hu hne
    exact prime_root_primitive ell hell u hu hne

#print axioms known_composite_tate_control_certificate
end
end CompositeTateDomainAudit
