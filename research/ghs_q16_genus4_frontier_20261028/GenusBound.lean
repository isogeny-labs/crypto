/-!
Checked arithmetic for the exact q16 genus-four frontier.

The Hasse-Weil bound for a genus-two Jacobian over F_16 is an external
mathematical input: its order is at most (sqrt(16)+1)^4 = 625. A nonzero
image of a prime-order subgroup makes that prime divide the target order.
The first two theorems prove the numerical contradiction under these
explicit hypotheses. The Magma genus sweep and subgroup-map coordinate
checks, and the Sage modular irreducibility certificates, are separate
executable inputs, not Lean theorems here.
-/

namespace GHSQ16GenusBound

theorem no_genus_two_target_1987 (N : Nat) (hpos : 0 < N)
    (hbound : N ≤ 625) : ¬ 1987 ∣ N := by
  intro hdvd
  have hle : 1987 ≤ N := Nat.le_of_dvd hpos hdvd
  omega

theorem no_genus_two_target_2089 (N : Nat) (hpos : 0 < N)
    (hbound : N ≤ 625) : ¬ 2089 ∣ N := by
  intro hdvd
  have hle : 2089 ≤ N := Nat.le_of_dvd hpos hdvd
  omega

theorem exact_parameter_accounting :
    12 + 30 + 24 = 66 ∧
    30 + 450 + 3585 + 30 = 4095 ∧
    66 * 4095 = 270270 ∧
    66 * 30 = 1980 ∧
    66 * 450 = 29700 ∧
    66 * 3585 = 236610 := by decide

theorem representative_jacobian_orders :
    3974 * 14 = 55636 ∧
    3978 * 18 = 71604 ∧
    4178 * 18 = 75204 ∧
    Nat.gcd 1987 14 = 1 ∧
    Nat.gcd 17 18 = 1 ∧
    Nat.gcd 2089 18 = 1 := by decide

end GHSQ16GenusBound
