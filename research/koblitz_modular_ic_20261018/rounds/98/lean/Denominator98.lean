import Std.Tactic

namespace Denominator98

-- The exact exponent list emitted by the source-bound G51 scanner.
def exponents : List Nat :=
  [81, 81, 81, 81, 79, 79, 80,
   98, 98, 98, 98, 96, 96, 97,
   128, 128, 128, 128, 126, 126, 127,
   161, 161, 161, 161, 159, 159, 160]

theorem count : exponents.length = 28 := by decide

theorem bound : ∀ e ∈ exponents, e ≤ 161 := by decide

theorem maximum_occurs : 161 ∈ exponents := by decide

-- For each denominator x^e, the explicit complement clears it by x^161.
theorem clearing_exponent :
    ∀ e ∈ exponents, ∃ k : Nat, e + k = 161 := by
  intro e he
  have h := bound e he
  exact ⟨161 - e, by omega⟩

end Denominator98
