import Init

/-!
Finite integer gates for the three GF(16) cubic transfer families.

The Tate-module and geometric-isogeny theorem in REPORT.md is a cited
mathematical theorem, not formalized here. These checked statements certify
only the exact subgroup-order products and simple-root arithmetic. The
primality checks are performed separately by Sage in `audit.py`.
-/

namespace GhsCubicQ16

theorem order_4036 : 4036 = 4 * 1009 := by decide
theorem order_4124 : 4124 = 4 * 1031 := by decide
theorem order_4126 : 4126 = 2 * 2063 := by decide

theorem trace_4036 : 4097 - 4036 = 61 := by decide
theorem trace_4124 : 4124 - 4097 = 27 := by decide
theorem trace_4126 : 4126 - 4097 = 29 := by decide

theorem derivative_value_4036 : (6 : Int) - 3 * 61 = -177 := by decide
theorem derivative_value_4124 : (6 : Int) - 3 * (-27) = 87 := by decide
theorem derivative_value_4126 : (6 : Int) - 3 * (-29) = 93 := by decide

theorem derivative_4036 : Nat.gcd 1009 177 = 1 := by decide
theorem derivative_4124 : Nat.gcd 1031 87 = 1 := by decide
theorem derivative_4126 : Nat.gcd 2063 93 = 1 := by decide

theorem q3_exception_factorization : 4095 = 3 * 3 * 5 * 7 * 13 := by decide
theorem q3_coprime_1009 : Nat.gcd 1009 4095 = 1 := by decide
theorem q3_coprime_1031 : Nat.gcd 1031 4095 = 1 := by decide
theorem q3_coprime_2063 : Nat.gcd 2063 4095 = 1 := by decide

end GhsCubicQ16
