module
public import PrimeControls

@[expose] public section
namespace DiemCyclicModuleAudit
noncomputable section
open Polynomial
open scoped MonoidAlgebra

/-- This refutes the composite-degree algebraic inference used in a reading of
Diem2003 Lemma22. It does not assert a geometric GHS function-field counterexample
or a refutation of the source's norm-conorm kernel theorem. -/
theorem diem_cyclic_module_certificate :
    ¬ Nat.Prime 21 ∧ Odd 21 ∧ Nat.Coprime 2 21 ∧
    orderOf (2 : ZMod 21) = 6 ∧
    IsCyclic C21 ∧ Nat.card C21 = 21 ∧
    Module.finrank F2 V21 = 6 ∧ Nat.card V21 = 64 ∧
    Function.Injective action21 ∧
    Submodule.span F2 (Set.range (fun g : C21 => action21 g 1)) = ⊤ ∧
    Submodule.span F2[C21] {action21.asModuleEquiv.symm 1} = ⊤ ∧
    groupSubmodule21 ≠ ⊥ ∧ groupSubmodule21 ≠ ⊤ ∧
    (¬ (∀ U : Submodule F2 V21,
      (∀ g : C21, ∀ a ∈ U, action21 g a ∈ U) → U = ⊥ ∨ U = ⊤)) ∧
    (¬ compositePolynomial ∣ cyclotomic 21 F2) ∧
    (∀ n : ℕ, ¬ 2 ∣ n → ∀ f : F2[X], f ∣ cyclotomic n F2 →
      f.natDegree = orderOf (2 : ZMod n) → ∀ U : Submodule F2 (Quotient f),
      (∀ a ∈ U, root f * a ∈ U) → U = ⊥ ∨ U = ⊤) ∧
    Irreducible prime5Polynomial ∧ Irreducible prime7Polynomial ∧
    Module.finrank F2 V5 = 4 ∧ orderOf (2 : ZMod 5) = 4 ∧
    Module.finrank F2 V7 = 3 ∧ orderOf (2 : ZMod 7) = 3 ∧
    (∀ U : Submodule F2 V5, (∀ a ∈ U, root prime5Polynomial * a ∈ U) →
      U = ⊥ ∨ U = ⊤) ∧
    (∀ U : Submodule F2 V7, (∀ a ∈ U, root prime7Polynomial * a ∈ U) →
      U = ⊥ ∨ U = ⊤) := by
  exact ⟨twenty_one_not_prime, by decide, by decide, two_mod21_order,
    group21_cyclic, group21_card, dimension21, cardinality21_module,
    action21_faithful, action21_orbit_spans, group_module21_cyclic,
    groupSubmodule21_proper.1, groupSubmodule21_proper.2,
    composite_invariant_criterion_false, composite_excluded_by_repair,
    cyclotomic_annihilator_repair, prime5_irreducible, prime7_irreducible,
    dimensions_prime_controls.1, order_two_mod5, dimensions_prime_controls.2,
    order_two_mod7, prime5_no_proper_invariant, prime7_no_proper_invariant⟩

#print axioms diem_cyclic_module_certificate
end
end DiemCyclicModuleAudit
