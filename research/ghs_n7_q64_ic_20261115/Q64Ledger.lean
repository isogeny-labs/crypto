import Init

/-! Exact q64 order and selector arithmetic only. Geometry is verified separately. -/
namespace GHSQ64Ledger
theorem source_order : (2 : Nat) * 2199023868557 = 4398047737114 := by decide
theorem target_order_and_cofactor :
    (4398047737114 : Nat) * 74 = 325455532546436 ∧
    148 * 2199023868557 = 325455532546436 := by decide
theorem full_degree_coefficient_count :
    (2 : Nat) * (2^18 - 2^6 - 2^9 + 2^3) = 523152 := by decide
#print axioms source_order
#print axioms target_order_and_cofactor
#print axioms full_degree_coefficient_count
end GHSQ64Ledger
