import Init

/-!
Checked integer arithmetic for the exact q=4 n=7 genus-eight source.
The paper proof of Weil-restriction simplicity also uses Capelli's criterion
and finite-field abelian-variety theory; this file does not formalize them.
-/

namespace GHSN7Q4TraceLedger

/-- Seventh Frobenius trace at base-field cardinality four. -/
def trace7 (t : Int) : Int :=
  t ^ 7 - 28 * t ^ 5 + 224 * t ^ 3 - 448 * t

theorem base_field_seventh_traces :
    trace7 (-1) = 251 ∧ trace7 3 = 87 := by
  decide

theorem candidate_arithmetic :
    16634 = 2 * 8317 ∧
    (-249 : Int) ^ 2 - 4 * 4 ^ 7 = -3535 ∧
    (-249 : Int) ≠ trace7 (-1) ∧
    (-249 : Int) ≠ trace7 3 := by
  decide

#print axioms base_field_seventh_traces
#print axioms candidate_arithmetic

end GHSN7Q4TraceLedger
