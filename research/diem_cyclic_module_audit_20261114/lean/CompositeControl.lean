module
public import CyclicQuotient

@[expose] public section
namespace DiemCyclicModuleAudit
noncomputable section
open Polynomial
open scoped MonoidAlgebra

abbrev F2 := ZMod 2

theorem polynomial_two_zero : (2 : F2[X]) = 0 := CharP.cast_eq_zero F2[X] 2
theorem polynomial_three_one : (3 : F2[X]) = 1 := by linear_combination polynomial_two_zero
theorem polynomial_four_zero : (4 : F2[X]) = 0 := by linear_combination 2 * polynomial_two_zero
theorem polynomial_neg_one : (-1 : F2[X]) = 1 := by linear_combination -polynomial_two_zero

/-- Product of three distinct factors of degrees 1,2,3, not a field. -/
def compositePolynomial : F2[X] := X ^ 6 + X ^ 4 + X + 1

theorem composite_factorization : compositePolynomial =
    (X + 1) * (X ^ 2 + X + 1) * (X ^ 3 + X + 1) := by
  unfold compositePolynomial
  ring_nf
  simp only [polynomial_two_zero, polynomial_three_one, polynomial_four_zero,
    mul_zero, mul_one, add_zero]

theorem composite_monic : compositePolynomial.Monic := by unfold compositePolynomial; monicity!
theorem composite_degree : compositePolynomial.natDegree = 6 := by
  unfold compositePolynomial
  compute_degree!

abbrev V21 := Quotient compositePolynomial
def x21 : V21 := root compositePolynomial

theorem dimension21 : Module.finrank F2 V21 = 6 := by
  rw [quotient_dimension compositePolynomial composite_monic, composite_degree]

theorem cardinality21_module : Nat.card V21 = 64 := by
  let : Module.Finite F2 V21 := composite_monic.finite_adjoinRoot
  rw [Module.natCard_eq_pow_finrank (K := F2), dimension21]
  norm_num [F2, Nat.card_zmod]

theorem polynomial_divides_21 : compositePolynomial ∣ X ^ 21 - 1 := by
  refine ⟨X ^ 15 + X ^ 13 + X ^ 11 + X ^ 10 + X ^ 7 + X ^ 6 + X ^ 5 +
    X ^ 3 + X ^ 2 + X + 1, ?_⟩
  unfold compositePolynomial
  ring_nf
  simp only [polynomial_two_zero, polynomial_four_zero, polynomial_neg_one,
    mul_zero, add_zero]

theorem polynomial_not_divides_three : ¬ compositePolynomial ∣ X ^ 3 - 1 := by
  apply composite_monic.not_dvd_of_natDegree_lt
  · exact X_pow_sub_C_ne_zero (by decide : 0 < 3) (1 : F2)
  · rw [composite_degree]
    compute_degree!

theorem polynomial_not_divides_seven : ¬ compositePolynomial ∣ X ^ 7 - 1 := by
  intro hd
  have hh : compositePolynomial ∣ (X ^ 7 - 1) - X * compositePolynomial :=
    dvd_sub hd (dvd_mul_left compositePolynomial X)
  have hr : (X ^ 7 - 1) - X * compositePolynomial =
      (X ^ 5 + X ^ 2 + X + 1 : F2[X]) := by
    unfold compositePolynomial
    linear_combination -(1 + X + X ^ 2 + X ^ 5) * polynomial_two_zero
  rw [hr] at hh
  apply composite_monic.not_dvd_of_natDegree_lt _ _ hh
  · have he : (X ^ 5 + X ^ 2 + X + 1 : F2[X]).natDegree = 5 := by compute_degree!
    exact ne_zero_of_natDegree_gt (show 0 < (X ^ 5 + X ^ 2 + X + 1 : F2[X]).natDegree
      from by rw [he]; decide)
  · rw [composite_degree]
    compute_degree!

theorem x21_pow21 : x21 ^ 21 = 1 :=
  (root_power_eq_one_iff compositePolynomial 21).mpr polynomial_divides_21

theorem x21_pow_three_ne : x21 ^ 3 ≠ 1 :=
  fun he => polynomial_not_divides_three ((root_power_eq_one_iff compositePolynomial 3).mp he)

theorem x21_pow_seven_ne : x21 ^ 7 ≠ 1 :=
  fun he => polynomial_not_divides_seven ((root_power_eq_one_iff compositePolynomial 7).mp he)

def u21 : V21ˣ := ⟨x21, x21 ^ 20, by simpa [← pow_succ'] using x21_pow21,
  by simpa [← pow_succ] using x21_pow21⟩

theorem u21_order : orderOf u21 = 21 := by
  apply orderOf_eq_of_pow_and_pow_div_prime (by decide : 0 < 21)
  · apply Units.ext
    exact x21_pow21
  · intro p hp hpd he
    rcases hp.dvd_mul.mp (show p ∣ 3 * 7 from hpd) with hd | hd
    · have heq := (Nat.prime_dvd_prime_iff_eq hp (by decide : Nat.Prime 3)).mp hd
      subst p
      exact x21_pow_seven_ne (congrArg Units.val he)
    · have heq := (Nat.prime_dvd_prime_iff_eq hp (by decide : Nat.Prime 7)).mp hd
      subst p
      exact x21_pow_three_ne (congrArg Units.val he)

abbrev C21 := Subgroup.zpowers u21
def action21 : Representation F2 C21 V21 := cyclicRepresentation compositePolynomial u21

theorem group21_card : Nat.card C21 = 21 := by rw [Nat.card_zpowers, u21_order]
theorem group21_cyclic : IsCyclic C21 := inferInstance
theorem action21_faithful : Function.Injective action21 :=
  cyclicRepresentation_faithful compositePolynomial u21

theorem action21_orbit_spans :
    Submodule.span F2 (Set.range (fun g : C21 => action21 g 1)) = ⊤ :=
  orbit_spans compositePolynomial u21 rfl

/-- Actual evaluation at 1 descends to the quotient because g(1)=0. -/
def evalAtOne : V21 →ₐ[F2] F2 :=
  AdjoinRoot.liftAlgHom compositePolynomial (Algebra.ofId F2 F2) 1
    (by norm_num [compositePolynomial, F2]; decide +kernel)

def invariant21 : Submodule F2 V21 := LinearMap.ker evalAtOne.toLinearMap

theorem invariant21_ne_bot : invariant21 ≠ ⊥ := by
  intro he
  have hm : x21 + 1 ∈ invariant21 := by
    simp [invariant21, x21, root, evalAtOne, F2]
    decide +kernel
  have hn : x21 + 1 ≠ 0 := by
    have hh := AdjoinRoot.mk_ne_zero_of_natDegree_lt composite_monic
      (g := X + 1) (by
        have hx : (X + 1 : F2[X]).natDegree = 1 := by compute_degree!
        intro hz
        rw [hz] at hx
        simp at hx)
      (by rw [composite_degree]; compute_degree!)
    simpa [x21, root] using hh
  apply hn
  simpa only [he, Submodule.mem_bot] using hm

theorem invariant21_ne_top : invariant21 ≠ ⊤ := by
  intro he
  have hm : (1 : V21) ∈ invariant21 := he ▸ Submodule.mem_top
  simp [invariant21] at hm

theorem invariant21_stable (g : C21) {a : V21} (ha : a ∈ invariant21) :
    action21 g a ∈ invariant21 := by
  change evalAtOne _ = 0 at ha ⊢
  change evalAtOne ((g.val : V21) * a) = 0
  simp [map_mul, ha]

def invariantLattice21 : action21.invtSubmodule :=
  ⟨invariant21, action21.mem_invtSubmodule.mpr fun g =>
    (Module.End.mem_invtSubmodule _).mpr fun _ ha => invariant21_stable g ha⟩

def groupSubmodule21 : Submodule F2[C21] action21.asModule :=
  action21.mapSubmodule invariantLattice21

theorem groupSubmodule21_proper : groupSubmodule21 ≠ ⊥ ∧ groupSubmodule21 ≠ ⊤ := by
  constructor
  · intro h
    have hh : invariantLattice21 = ⊥ := action21.mapSubmodule.injective (by simpa [groupSubmodule21] using h)
    exact invariant21_ne_bot (congrArg Subtype.val hh)
  · intro h
    have hh : invariantLattice21 = ⊤ := action21.mapSubmodule.injective (by simpa [groupSubmodule21] using h)
    exact invariant21_ne_top (congrArg Subtype.val hh)

theorem group_module21_cyclic : Submodule.span F2[C21]
    {action21.asModuleEquiv.symm 1} = ⊤ :=
  group_algebra_cyclic compositePolynomial u21 rfl

theorem composite_invariant_criterion_false :
    ¬ (∀ U : Submodule F2 V21,
      (∀ g : C21, ∀ a ∈ U, action21 g a ∈ U) → U = ⊥ ∨ U = ⊤) := by
  intro h
  rcases h invariant21 (fun g _ ha => invariant21_stable g ha) with hb | ht
  · exact invariant21_ne_bot hb
  · exact invariant21_ne_top ht

theorem two_mod21_order : orderOf (2 : ZMod 21) = 6 := by
  apply (orderOf_eq_iff (by decide : 0 < 6)).mpr
  refine ⟨by decide +kernel, ?_⟩
  have hh : ∀ n : Fin 6, 0 < n.val → (2 : ZMod 21) ^ n.val ≠ 1 := by decide +kernel
  exact fun n hn hp => hh ⟨n, hn⟩ hp
theorem twenty_one_not_prime : ¬ Nat.Prime 21 := by decide +kernel

#print axioms u21_order
#print axioms action21_orbit_spans
#print axioms invariant21_ne_bot
#print axioms invariant21_stable
#print axioms groupSubmodule21_proper
#print axioms group_module21_cyclic
#print axioms composite_invariant_criterion_false
end
end DiemCyclicModuleAudit
