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

theorem even_model_count : (3 : Nat)^12 = 531441 := by decide

theorem even_to_odd_branch_maps : (84 : Nat) * 2 = 168 := by decide

/- These are arithmetic inputs to the paper-level integral Tate argument. -/
def primeCheck (n : Nat) : Bool :=
  decide (n ≥ 2) && (List.range n).all (fun d => decide (d < 2 ∨ n % d ≠ 0))

theorem selected_prime_subgroups :
    primeCheck 19 = true ∧ 266 % 19 = 0 ∧ Nat.gcd 19 (15 * 488) = 1 ∧
    primeCheck 43 = true ∧ 258 % 43 = 0 ∧ Nat.gcd 43 (15 * 776) = 1 ∧
    primeCheck 127 = true ∧ 254 % 127 = 0 ∧ Nat.gcd 127 (15 * 872) = 1 ∧
    primeCheck 13 = true ∧ 234 % 13 = 0 ∧ Nat.gcd 13 (15 * 872) = 1 ∧
    primeCheck 23 = true ∧ 230 % 23 = 0 ∧ Nat.gcd 23 (15 * 776) = 1 ∧
    primeCheck 37 = true ∧ 222 % 37 = 0 ∧ Nat.gcd 37 (15 * 488) = 1 ∧
    primeCheck 109 = true ∧ 218 % 109 = 0 ∧ Nat.gcd 109 (15 * 296) = 1 := by decide

/- Howe's gamma_10^10 < 5669 gives the four integer degree ceilings. -/
theorem howe_degree_ceilings :
    (5669 : Nat) * 872^5 < 4^5 * 36^10 ∧
    (5669 : Nat) * 776^5 < 4^5 * 34^10 ∧
    (5669 : Nat) * 488^5 < 4^5 * 27^10 ∧
    (5669 : Nat) * 296^5 < 4^5 * 21^10 := by decide

theorem selected_cover_primes_exceed_ceilings :
    (43 : Nat) > 33 ∧ 127 > 35 ∧ 37 > 26 ∧ 109 > 20 := by decide

theorem degree_four_denominator_count :
    (5 : Nat) + 242 + 1 + 1 = 249 := by decide

theorem degree_four_assignment_count :
    (5 : Nat) * 3^4 + 244 * 3^5 = 59697 := by decide

theorem degree_five_denominator_count :
    (1 : Nat) + 5 + 242 + 10 + 1210 + 58801 = 60269 := by decide

theorem degree_five_assignment_count :
    (1 : Nat) * 3^5 + 5 * 3^4 + 242 * 3^5 +
    10 * 3^3 + 1210 * 3^4 + 58801 * 3^5 = 14446377 := by decide

theorem degree_six_denominator_count :
    (1 : Nat) + 1 + 5 + 242 + 1 + 5 + 242 + 10 + 1210 + 58801 = 60518 := by decide

theorem degree_six_assignment_count :
    (1 : Nat) * 3^5 + 1 * 3^5 + 5 * 3^4 + 242 * 3^5 +
    1 * 3^5 + 5 * 3^4 + 242 * 3^5 + 10 * 3^3 +
    1210 * 3^4 + 58801 * 3^5 = 14506074 := by decide

theorem degree_six_lambda_count : (10 : Nat) * 3^3 * (3^5 - 1) = 65340 := by decide

theorem trace26_ten_target_count : (1 : Nat) + 9 = 10 := by decide

theorem trace26_class_screen_sizes :
    (10 : Nat) * 59697 = 596970 ∧
    (10 : Nat) * 14446377 = 144463770 := by decide

theorem trace26_cover_degrees_coprime_to_109 :
    (List.range 21).all (fun d => decide (d < 7 ∨ Nat.gcd d 109 = 1)) = true := by decide

/- Arithmetic checks for the separately replayed s=11 primitive-j isogeny witness. -/
theorem primitive_s11_trace_step :
    (26 : Int) * 1534969625950 - 243 * 56701719386 = 26130692463902 := by decide

theorem primitive_s11_order :
    (3 : Nat)^55 + 1 - 26130692463902 =
      2 * 109 * 2399 * 333566377062870325133 := by decide

theorem primitive_s11_projective_divisor :
    (224990655913 : Nat) = 23 * 131 * 74673301 ∧
    224990655913 % 23 = 0 ∧
    Nat.gcd 23 333566377062870325133 = 1 := by decide

/- Counts for the two partial degree-seven denominator screens. -/
theorem degree_seven_sparse_assignment_counts :
    (3 : Nat)^5 = 243 ∧
    5 * 3^4 = 405 ∧
    10 * 3^3 = 270 ∧
    (3^5 - 1) * 270 = 65340 := by decide

theorem degree_seven_linear_square_assignment_counts :
    (3^5 - 1 : Nat) * 3^5 = 58806 ∧
    (3^5 - 1 : Nat) * 5 * 3^4 = 98010 := by decide

#print axioms ordinary_class_count
#print axioms selected_class_count
#print axioms all_eight_orders_two_mod_four
#print axioms example_trace_ten
#print axioms cubic_finite_candidate_count
#print axioms cubic_geometric_pole_pair_count
#print axioms even_model_count
#print axioms even_to_odd_branch_maps
#print axioms selected_prime_subgroups
#print axioms howe_degree_ceilings
#print axioms selected_cover_primes_exceed_ceilings
#print axioms degree_four_denominator_count
#print axioms degree_four_assignment_count
#print axioms degree_five_denominator_count
#print axioms degree_five_assignment_count
#print axioms degree_six_denominator_count
#print axioms degree_six_assignment_count
#print axioms degree_six_lambda_count
#print axioms trace26_ten_target_count
#print axioms trace26_class_screen_sizes
#print axioms trace26_cover_degrees_coprime_to_109
#print axioms primitive_s11_trace_step
#print axioms primitive_s11_order
#print axioms primitive_s11_projective_divisor
#print axioms degree_seven_sparse_assignment_counts
#print axioms degree_seven_linear_square_assignment_counts
end GHSOdd5Ledger
