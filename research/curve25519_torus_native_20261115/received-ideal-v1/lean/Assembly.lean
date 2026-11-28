module
public import Mathlib.Algebra.Polynomial.Div
public import Mathlib.Tactic.LinearCombination

example {R : Type*} [CommRing R] (p q : Polynomial R) (hq : q.Monic) :
    p %ₘ q = 0 ↔ q ∣ p := Polynomial.modByMonic_eq_zero_iff_dvd hq

example {R : Type*} [CommRing R] (p q : Polynomial R) (x : R)
    (hq : q.eval x = 0) : (p * q).eval x = 0 := by
  rw [Polynomial.eval_mul, hq, mul_zero]

example {R : Type*} [CommRing R] [Nontrivial R] (A b : R) :
    (Polynomial.X^4 +
      (Polynomial.C (A*(1-b)*(A^2-3*b)) * Polynomial.X^2 +
        Polynomial.C (b^3*(1-b)^2))).Monic := by
  apply Polynomial.monic_X_pow_add
  apply lt_of_le_of_lt (Polynomial.degree_add_le _ _)
  apply max_lt_iff.mpr
  constructor
  · exact lt_of_le_of_lt (Polynomial.degree_C_mul_X_pow_le _ _) (by decide)
  · exact lt_of_le_of_lt Polynomial.degree_C_le (by decide)
