import Init

/-!
Finite arithmetic used in the q=2 prime-degree transfer screen.
This checks the seventh Frobenius-trace recurrence at the five possible
base-field traces. It does not formalize Diem's simplicity theorem,
Capelli's criterion, GHS geometry, or a subgroup transfer.
-/

namespace GHSN7TraceExceptions

def trace7 (u : Int) : Int :=
  u ^ 7 - 14 * u ^ 5 + 56 * u ^ 3 - 56 * u

theorem q2_base_trace_values :
    trace7 (-2) = -16 ∧
    trace7 (-1) = 13 ∧
    trace7 0 = 0 ∧
    trace7 1 = -13 ∧
    trace7 2 = 16 := by
  decide

#print axioms q2_base_trace_values

end GHSN7TraceExceptions
