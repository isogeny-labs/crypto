module
public import CompositeControl
public import Mathlib.RingTheory.Polynomial.Cyclotomic.Factorization
public import Mathlib.Algebra.Polynomial.SpecificDegree

@[expose] public section
namespace DiemCyclicModuleAudit
noncomputable section
open Polynomial

private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance : Fact (Nat.Prime 7) := ⟨by decide⟩

/-- A sufficient algebraic repair, valid even for composite odd n. The extra
cyclotomic-factor hypothesis excludes the actual n21 counterexample. -/
theorem cyclotomic_annihilator_repair (n : ℕ) (hn : ¬ 2 ∣ n) (f : F2[X])
    (hf : f ∣ cyclotomic n F2) (hd : f.natDegree = orderOf (2 : ZMod n))
    (U : Submodule F2 (Quotient f))
    (hU : ∀ a ∈ U, root f * a ∈ U) : U = ⊥ ∨ U = ⊤ := by
  have hi : Irreducible f := by
    apply ZMod.irreducible_of_dvd_cyclotomic_of_natDegree hn hf
    rw [← orderOf_units, ZMod.coe_unitOfCoprime]
    exact hd
  let : Fact (Irreducible f) := ⟨hi⟩
  exact invariant_eq_bot_or_top f U hU

def prime5Polynomial : F2[X] := X ^ 4 + X ^ 3 + X ^ 2 + X + 1
def prime7Polynomial : F2[X] := X ^ 3 + X + 1

theorem prime5_monic : prime5Polynomial.Monic := by unfold prime5Polynomial; monicity!
theorem prime7_monic : prime7Polynomial.Monic := by unfold prime7Polynomial; monicity!
theorem prime5_degree : prime5Polynomial.natDegree = 4 := by unfold prime5Polynomial; compute_degree!
theorem prime7_degree : prime7Polynomial.natDegree = 3 := by unfold prime7Polynomial; compute_degree!

theorem order_two_mod5 : orderOf (2 : ZMod 5) = 4 := by
  apply (orderOf_eq_iff (by decide : 0 < 4)).mpr
  refine ⟨by decide +kernel, ?_⟩
  have hh : ∀ n : Fin 4, 0 < n.val → (2 : ZMod 5) ^ n.val ≠ 1 := by decide +kernel
  exact fun n hn hp => hh ⟨n, hn⟩ hp

theorem order_two_mod7 : orderOf (2 : ZMod 7) = 3 := by
  apply (orderOf_eq_iff (by decide : 0 < 3)).mpr
  refine ⟨by decide +kernel, ?_⟩
  have hh : ∀ n : Fin 3, 0 < n.val → (2 : ZMod 7) ^ n.val ≠ 1 := by decide +kernel
  exact fun n hn hp => hh ⟨n, hn⟩ hp

theorem prime5_cyclotomic : prime5Polynomial = cyclotomic 5 F2 := by
  rw [cyclotomic_prime]
  norm_num [Finset.sum_range_succ, prime5Polynomial]
  ring

theorem prime5_irreducible : Irreducible prime5Polynomial := by
  apply ZMod.irreducible_of_dvd_cyclotomic_of_natDegree (p := 2) (n := 5) (by decide)
  · rw [← prime5_cyclotomic]
  · rw [prime5_degree, ← orderOf_units, ZMod.coe_unitOfCoprime]
    change 4 = orderOf (2 : ZMod 5)
    exact order_two_mod5.symm

theorem prime7_irreducible : Irreducible prime7Polynomial := by
  apply Polynomial.irreducible_of_degree_le_three_of_not_isRoot
  · rw [prime7_degree]
    decide
  · intro a ha
    have hh : a ^ 3 + a + 1 = 0 := by simpa [Polynomial.IsRoot, prime7Polynomial] using ha
    exact (by decide +kernel : ∀ a : F2, a ^ 3 + a + 1 ≠ 0) a hh

theorem prime7_divides_cyclotomic : prime7Polynomial ∣ cyclotomic 7 F2 := by
  refine ⟨X ^ 3 + X ^ 2 + 1, ?_⟩
  rw [cyclotomic_prime]
  norm_num [Finset.sum_range_succ, prime7Polynomial]
  ring_nf
  simp only [polynomial_three_one, mul_one]

instance : Fact (Irreducible prime5Polynomial) := ⟨prime5_irreducible⟩
instance : Fact (Irreducible prime7Polynomial) := ⟨prime7_irreducible⟩

abbrev V5 := Quotient prime5Polynomial
abbrev V7 := Quotient prime7Polynomial

theorem dimensions_prime_controls :
    Module.finrank F2 V5 = 4 ∧ Module.finrank F2 V7 = 3 := by
  exact ⟨(quotient_dimension prime5Polynomial prime5_monic).trans prime5_degree,
    (quotient_dimension prime7Polynomial prime7_monic).trans prime7_degree⟩

theorem prime5_no_proper_invariant (U : Submodule F2 V5)
    (hU : ∀ a ∈ U, root prime5Polynomial * a ∈ U) : U = ⊥ ∨ U = ⊤ :=
  invariant_eq_bot_or_top prime5Polynomial U hU

theorem prime7_no_proper_invariant (U : Submodule F2 V7)
    (hU : ∀ a ∈ U, root prime7Polynomial * a ∈ U) : U = ⊥ ∨ U = ⊤ :=
  invariant_eq_bot_or_top prime7Polynomial U hU

theorem prime_controls_match_repair :
    prime5Polynomial ∣ cyclotomic 5 F2 ∧
    prime5Polynomial.natDegree = orderOf (2 : ZMod 5) ∧
    prime7Polynomial ∣ cyclotomic 7 F2 ∧
    prime7Polynomial.natDegree = orderOf (2 : ZMod 7) := by
  exact ⟨prime5_cyclotomic ▸ dvd_rfl, prime5_degree.trans order_two_mod5.symm,
    prime7_divides_cyclotomic, prime7_degree.trans order_two_mod7.symm⟩

theorem composite_excluded_by_repair : ¬ compositePolynomial ∣ cyclotomic 21 F2 := by
  intro hd
  have hr : IsRoot compositePolynomial (1 : F2) := by
    norm_num [Polynomial.IsRoot, compositePolynomial, F2]
    decide +kernel
  have hc : IsRoot (cyclotomic 21 F2) (1 : F2) := hr.dvd hd
  let : NeZero (21 : F2) := ⟨by decide +kernel⟩
  have hp := (isRoot_cyclotomic_iff (R := F2)).mp hc
  exact hp.ne_one (by decide) rfl

#print axioms prime5_irreducible
#print axioms prime7_irreducible
#print axioms prime5_no_proper_invariant
#print axioms prime7_no_proper_invariant
#print axioms cyclotomic_annihilator_repair
#print axioms composite_excluded_by_repair
end
end DiemCyclicModuleAudit
