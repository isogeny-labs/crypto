module
public import Mathlib.Tactic
public import Mathlib.Data.ZMod.Basic

@[expose] public section
namespace BKCharFiveGeometricAudit
open Polynomial

variable {R : Type*} [CommRing R]
def denominator (x : R) : R := 4 * (x ^ 3 + 3 * x + 2)
def numerator (x : R) : R := x ^ 4 - 6 * x ^ 2 - 16 * x + 9
def triple (x : R) : R := 3 * x ^ 4 + 18 * x ^ 2 + 24 * x - 9
def eliminant (x : R) : R :=
  numerator x ^ 4 - 6 * numerator x ^ 2 * denominator x ^ 2 -
    16 * numerator x * denominator x ^ 3 + 9 * denominator x ^ 4 -
    4 * x * (numerator x ^ 3 * denominator x +
      3 * numerator x * denominator x ^ 3 + 2 * denominator x ^ 4)

theorem eliminant_zero_of_double (x u : R)
    (h1 : numerator x = denominator x * u)
    (h2 : numerator u - denominator u * x = 0) : eliminant x = 0 := by
  unfold eliminant
  rw [h1]
  calc
    _ = denominator x ^ 4 * (numerator u - denominator u * x) := by
      unfold numerator denominator
      ring
    _ = 0 := by rw [h2, mul_zero]

local instance : Fact (Nat.Prime 5) := ⟨by decide⟩

/-- Concrete polynomial identity, independent of any torsion premise. -/
theorem eliminant_polynomial_factor :
    eliminant (X : (ZMod 5)[X]) =
      -triple (X : (ZMod 5)[X]) * (X - 1) ^ 5 * (X - 2) ^ 5 := by
  unfold eliminant numerator denominator triple
  ring_nf
  ext n
  by_cases hn : n ≤ 16
  · interval_cases n <;> norm_num [Polynomial.coeff_X_pow, Polynomial.coeff_X]
      <;> decide +kernel
  · have hn0 : n ≠ 0 := by omega
    have hn1 : 1 ≠ n := by omega
    have hn2 : n ≠ 2 := by omega
    have hn3 : n ≠ 3 := by omega
    have hn4 : n ≠ 4 := by omega
    have hn5 : n ≠ 5 := by omega
    have hn6 : n ≠ 6 := by omega
    have hn7 : n ≠ 7 := by omega
    have hn8 : n ≠ 8 := by omega
    have hn9 : n ≠ 9 := by omega
    have hn10 : n ≠ 10 := by omega
    have hn11 : n ≠ 11 := by omega
    have hn12 : n ≠ 12 := by omega
    have hn13 : n ≠ 13 := by omega
    have hn14 : n ≠ 14 := by omega
    have hn16 : n ≠ 16 := by omega
    rw [← Polynomial.C_ofNat 61407, ← Polynomial.C_ofNat 288]
    simp [Polynomial.coeff_X_pow, Polynomial.coeff_X, Polynomial.coeff_C,
      hn0, hn1, hn2, hn3, hn4, hn5, hn6, hn7, hn8,
      hn9, hn10, hn11, hn12, hn13, hn14, hn16]

variable {K : Type*} [Field K] [CharP K 5]

theorem eliminant_factor (x : K) :
    eliminant x = -triple x * (x - 1) ^ 5 * (x - 2) ^ 5 := by
  have h := congrArg (Polynomial.eval₂RingHom (ZMod.castHom (dvd_refl 5) K) x)
    eliminant_polynomial_factor
  simpa [eliminant, numerator, denominator, triple, Polynomial.eval₂_pow] using h

#print axioms eliminant_zero_of_double
#print axioms eliminant_polynomial_factor
#print axioms eliminant_factor
end BKCharFiveGeometricAudit
