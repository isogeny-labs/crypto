import Init

/-! Arithmetic ledger for the finite odd-degree-five carrier stratum.
The finite-field point counts, polynomial irreducibility, Tate isogeny,
and all-s base-change theorem are checked separately. -/
namespace GHSOdd5Ledger

theorem ordinary_class_count : (2 : Nat) * (3^5 - 1) = 484 := by decide

theorem selected_class_count :
    (10 : Nat) + 10 + 20 + 10 + 10 + 20 + 10 + 10 = 100 := by decide

theorem all_eight_orders_two_mod_four :
    (270 : Nat) % 4 = 2 ∧ 266 % 4 = 2 ∧ 258 % 4 = 2 ∧
    254 % 4 = 2 ∧ 234 % 4 = 2 ∧ 230 % 4 = 2 ∧
    222 % 4 = 2 ∧ 218 % 4 = 2 := by decide

theorem example_trace_ten : (244 : Nat) - 234 = 10 := by decide

theorem cubic_finite_candidate_count : (84 : Nat) * 5 * 243 = 102060 := by decide

theorem cubic_geometric_pole_pair_count : (84 : Nat) * 55 = 4620 := by decide

#print axioms ordinary_class_count
#print axioms selected_class_count
#print axioms all_eight_orders_two_mod_four
#print axioms example_trace_ten
#print axioms cubic_finite_candidate_count
#print axioms cubic_geometric_pole_pair_count
end GHSOdd5Ledger
