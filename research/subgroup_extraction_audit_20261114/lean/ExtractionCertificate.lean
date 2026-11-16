import CofactorFamily
import ActualCurve

namespace SubgroupExtractionAudit

theorem actual_subgroup_extraction_certificate :
    (∀ (G : Type) [AddCommGroup G] (r : ℕ) [Fact (Nat.Prime r)] (P : G),
      addOrderOf P = r → ∀ a b x : ℕ, a • P + b • (x • P) = 0 →
      (b : ZMod r) ≠ 0 → (x : ZMod r) = -(a : ZMod r)/(b : ZMod r)) ∧
    (∀ b : ℕ, ∃ r : ℕ, Nat.Prime r ∧ b < r ∧ addOrderOf (familyP r) = r ∧
      (r : ZMod (2*r)) ≠ 0 ∧ ¬ IsUnit (r : ZMod (2*r)) ∧
      (∀ x : ℕ, r • (x • familyP r) = 0)) ∧
    (∀ r : ℕ, 1 < r → Nat.card (ZMod (2*r)) = 2*r ∧ familyP r ≠ 0) ∧
    curve.Δ ≠ 0 ∧ Nat.card curve.Point = 30 ∧ addOrderOf P = 5 ∧ P ≠ 0 ∧
    (5 : ZMod (Nat.card curve.Point)) ≠ 0 ∧
    ¬ IsUnit (5 : ZMod (Nat.card curve.Point)) ∧
    (0 : ℕ) • P + (5 : ℕ) • P = 0 ∧
    ((-(0 : ZMod 30)*(5 : ZMod 30)⁻¹).val : ℕ) • P ≠ P ∧
    (¬ (∀ a b x : ℕ, a • P + b • (x • P) = 0 → (b : ZMod 30) ≠ 0 →
      ((-(a : ZMod 30)*(b : ZMod 30)⁻¹).val : ℕ) • P = x • P)) ∧
    (3 : ℕ) • P + (2 : ℕ) • P = 0 ∧ ¬ IsUnit (2 : ZMod 30) ∧
    (2 : ZMod 5) ≠ 0 ∧ (1 : ZMod 5) = -(3 : ZMod 5)/(2 : ZMod 5) ∧
    IsUnit (1 : ZMod 30) ∧ (-(29 : ZMod 30)*(1 : ZMod 30)⁻¹).val = 1 := by
  refine ⟨?_, unbounded_prime_family, ?_, curve_discriminant, ambient_card,
    P_order, P_nonzero, ambient_bad_beta.1, ambient_bad_beta.2, bad_relation,
    naive_bad_recovery_fails, universal_ambient_extraction_false, useful_relation,
    useful_beta_nonunit_ambient, useful_beta_nonzero_subgroup, correct_subgroup_recovery,
    ambient_unit_control.1, ambient_unit_control.2⟩
  · intro G _ r _ P ho a b x hrel hb
    exact recover_mod_prime r P ho a b x hrel hb
  · intro r hr
    exact ⟨family_ambient_card r (by omega), family_point_nonzero r hr⟩

#print axioms actual_subgroup_extraction_certificate

end SubgroupExtractionAudit
