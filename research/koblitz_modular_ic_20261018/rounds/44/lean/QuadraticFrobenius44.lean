import Mathlib.Algebra.CharP.Lemmas
import Mathlib.Tactic.Ring
namespace QuadraticFrobenius44
variable {R : Type*} [CommRing R] [CharP R 2]
def correction (C : R) : ℕ → R
  | 0 => 0
  | n+1 => (correction C n)^2+C

theorem quadratic_frobenius (y C : R) (h : y^2+y=C) (n : ℕ) :
    y^(2^n)=y+correction C n := by
  have hc : y+y=0 := by rw [← two_mul, show (2:R)=0 from CharP.cast_eq_zero R 2, zero_mul]
  have hs : y^2=y+C := by
    calc
      y^2=y^2+(y+y) := by rw [hc, add_zero]
      _=y+(y^2+y) := by ring
      _=y+C := by rw [h]
  induction n with
  | zero => simp [correction]
  | succ n ih =>
    calc
      y^(2^(n+1))=(y^(2^n))^2 := by rw [pow_succ, pow_mul]
      _=(y+correction C n)^2 := by rw [ih]
      _=y^2+(correction C n)^2 := add_pow_char _ _ 2
      _=y+correction C (n+1) := by rw [hs, correction]; ring
end QuadraticFrobenius44
