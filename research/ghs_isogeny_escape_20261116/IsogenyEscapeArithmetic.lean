import Init

/-! Arithmetic for the trace-26 degree-seven isogeny escape obstruction.
The Frobenius representation and isogeny-factorization lemma are proved
in the accompanying mathematical note, not formalized in this file. -/
namespace IsogenyEscapeArithmetic

def u (t : Int) : Nat → Int
  | 0 => 0
  | 1 => 1
  | n + 2 => t * u t (n + 1) - 243 * u t n

theorem trace_negative_u7 : u (-26) 7 = -21156227 := by decide
theorem trace_positive_u7 : u 26 7 = -21156227 := by decide
theorem square_bound : 21156227 < 4600 * 4600 := by decide
theorem characteristic_excluded : 21156227 % 3 = 2 := by decide
theorem extension_degree_congruence : 21156227 % 7 = 1 := by decide

end IsogenyEscapeArithmetic
