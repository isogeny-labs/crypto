import Init

/-! Finite arithmetic for the W1 trace-minus-nine carrier. Geometry and Tate's
theorem are used in the written argument, not formalized here. -/
namespace W1Prime23Arithmetic

theorem carrier_twist_bit_identity : Nat.xor 0x802f 0xc07a = 0x4055 := by decide
theorem group_order : 1^14 + 9*1^7 + 128 = 138 := by decide
theorem subgroup_factor : 138 = 6*23 := by decide
theorem discriminant_factors_prime_to_subgroup :
    Nat.Coprime 23 (2^42 * 7^14 * 431^7) := by decide
theorem bad_discriminant_factor_remainders :
    2 % 23 = 2 ∧ 7 % 23 = 7 ∧ 431 % 23 = 17 := by decide

def tracePair : Nat → Int × Int
  | 0 => (2, -9)
  | n+1 => let (a,b) := tracePair n; (b, -9*b-128*a)

theorem trace11 : (tracePair 10).2 = -759947650713 := by decide
theorem extension11_order :
    2^77 + 1 - (tracePair 10).2 = 151115727452588594488986 := by decide
theorem extension11_factorization :
    151115727452588594488986 =
      2 * 3 * 23^2 * 8647 * 5506013662841737 := by decide
theorem extension11_selected_prime_coprime_to_discriminant_factors :
    Nat.Coprime 5506013662841737
      (2 * 7 * (4 * 2^77 - 759947650713^2)) := by decide

end W1Prime23Arithmetic
