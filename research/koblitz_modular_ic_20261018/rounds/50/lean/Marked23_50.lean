import Mathlib.GroupTheory.OrderOfElement

namespace Marked23_50

/-- The exact binary prefix expression used by the native order23 check.
This is an abstract additive-monoid identity, not a formalization of the
native Weierstrass coordinates or their implementation. -/
theorem binary_prefix23 {A : Type*} [AddMonoid A] (P : A) :
    2 • (2 • (2 • (2 • P) + P) + P) + P = 23 • P := by
  have h : ((2 * (2 * (2 * 2 + 1) + 1) + 1 : ℕ) • P) =
      2 • (2 • (2 • (2 • P) + P) + P) + P := by
    simp only [add_nsmul, mul_nsmul, one_nsmul]
  exact h.symm

/-- The native group-law and nonidentity checks instantiate these hypotheses;
the concrete function-field curve and coordinate arrays remain native data. -/
theorem exact_order23 {A : Type*} [AddMonoid A] (P : A)
    (h23 : 23 • P = 0) (hne : P ≠ 0) : addOrderOf P = 23 := by
  letI : Fact (Nat.Prime 23) := ⟨by decide⟩
  exact addOrderOf_eq_prime h23 hne

end Marked23_50
