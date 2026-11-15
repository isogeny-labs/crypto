import Init

/-!
Exact arithmetic ledger for the q32 selected source. Field arithmetic,
Rabin irreducibility, Hess descent, Jacobians and the subgroup map are
checked separately and are not formalized by this file.
-/

namespace GHSQ32Ledger

theorem source_order :
    (6 : Nat) * 5726649013 = 34359894078 := by
  decide

theorem target_order_and_cofactor :
    (34359894078 : Nat) * 22 = 755917669716 ∧
    132 * 5726649013 = 755917669716 := by
  decide

theorem full_degree_coefficient_count :
    (2 : Nat) * (32767 - 7) = 65520 := by
  decide

#print axioms source_order
#print axioms target_order_and_cofactor
#print axioms full_degree_coefficient_count

end GHSQ32Ledger
