import Init

/-! Kernel-checked finite-ring certificates for the proposed nondual7-chain.
Elliptic cyclicity,Frobenius characteristic polynomial and HLP transport are
written bridges,not formalized elliptic curves or Jacobians in this file. -/
set_option autoImplicit false
namespace CyclicFortyNine

theorem first_unique_scalar :
    ∀ a : Fin 49, (a.val % 7 = 5 ∧ (a.val^2+7*a.val+101)%49=0) ↔
      a.val=33 := by decide

theorem second_unique_scalar :
    ∀ a : Fin 49, (a.val % 7 = 6 ∧ (a.val^2+11*a.val+101)%49=0) ↔
      a.val=27 := by decide

theorem first_exact_order :
    (33 : Nat)^42 % 49 = 1 ∧
      ∀ n : Fin 41, (33 : Nat)^(n.val+1) % 49 ≠ 1 := by decide

theorem second_exact_order :
    (27 : Nat)^14 % 49 = 1 ∧
      ∀ n : Fin 13, (27 : Nat)^(n.val+1) % 49 ≠ 1 := by decide

theorem sixth_powers :
    (33 : Nat)^6 % 49 = 8 ∧ (27 : Nat)^6 % 49 = 29 := by decide

theorem sixth_fixed_residues :
    ∀ n : Fin 49,
      ((7*n.val)%49=0 ↔ n.val%7=0) ∧
      ((28*n.val)%49=0 ↔ n.val%7=0) := by decide

end CyclicFortyNine
#print axioms CyclicFortyNine.first_unique_scalar
#print axioms CyclicFortyNine.second_unique_scalar
#print axioms CyclicFortyNine.first_exact_order
#print axioms CyclicFortyNine.second_exact_order
#print axioms CyclicFortyNine.sixth_powers
#print axioms CyclicFortyNine.sixth_fixed_residues
