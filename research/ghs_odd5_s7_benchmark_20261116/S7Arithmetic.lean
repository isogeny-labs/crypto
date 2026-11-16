import Init

/- Arithmetic checks for the s=7 transfer receipt. Finite-field geometry,
   primality, point actions, and attack timings are external to this file. -/
namespace GHSOdd5S7

def trace : Nat → Int
  | 0 => 2
  | 1 => 26
  | n + 2 => 26 * trace (n + 1) - 243 * trace n

theorem seventh_trace : trace 7 = -260078338 := by decide

theorem group_order :
    (3 : Int)^35 + 1 - trace 7 = 50031545359078046 := by decide

theorem subgroup_factorization :
    (2 : Nat) * 29 * 109 * 7393 * 1070455751 = 50031545359078046 := by decide

theorem rho_eigenvalue_polynomial :
    ((1048494599 : Nat)^2 + 243) % 1070455751 =
      (26 * 1048494599) % 1070455751 := by decide

theorem rho_eigenvalue_order_seven :
    (1048494599 : Nat)^7 % 1070455751 = 1 ∧
      1048494599 % 1070455751 ≠ 1 := by decide

#print axioms seventh_trace
#print axioms group_order
#print axioms subgroup_factorization
#print axioms rho_eigenvalue_polynomial
#print axioms rho_eigenvalue_order_seven

end GHSOdd5S7
