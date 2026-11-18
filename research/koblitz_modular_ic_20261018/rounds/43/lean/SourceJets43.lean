import Mathlib.Algebra.CharP.Lemmas
import Mathlib.Tactic.NormNum
import Mathlib.Tactic.Ring

namespace SourceJets43
variable {R : Type*} [CommRing R] [CharP R 2]

def asSum (x : R) : ℕ → R
  | 0 => 0
  | k+1 => asSum x k + x^(2^k)

theorem as_sum_identity (x : R) (k : ℕ) :
    (asSum x k)^2 + asSum x k = x + x^(2^k) := by
  have htwo : (2 : R)=0 := CharP.cast_eq_zero R 2
  have hcancel (a : R) : a+a=0 := by rw [← two_mul, htwo, zero_mul]
  induction k with
  | zero => simp [asSum, hcancel]
  | succ k ih =>
    have hp : (x^(2^k))^2=x^(2^(k+1)) := by rw [← pow_mul, pow_succ]
    calc
      (asSum x (k+1))^2+asSum x (k+1) =
          (asSum x k)^2+asSum x k+x^(2^k)+(x^(2^k))^2 := by
            rw [asSum, add_pow_char]; ring
      _ = x+(x^(2^k)+x^(2^k))+(x^(2^k))^2 := by rw [ih]; ring
      _ = x+x^(2^(k+1)) := by rw [hcancel, add_zero, hp]

theorem as_sum_precision615 (x : R) (hx : x^615=0) :
    (asSum x 10)^2+asSum x 10=x := by
  have hz : x^1024=0 := by
    have h : x^(615+409)=x^615*x^409 := pow_add x 615 409
    simpa only [hx, zero_mul] using h
  rw [as_sum_identity]
  norm_num only at *
  rw [hz, add_zero]

theorem coefficient_lift_precision615 (c z : R) (hc : c^32=c)
    (hz : z^615=0) : (c+z)^1024=c := by
  have hz1024 : z^1024=0 := by
    have h : z^(615+409)=z^615*z^409 := pow_add z 615 409
    simpa only [hz, zero_mul] using h
  have hc1024 : c^1024=c := by
    calc
      c^1024=(c^32)^32 := by rw [← pow_mul]
      _=c := by rw [hc, hc]
  have h := add_pow_char_pow c z 2 10
  norm_num only at h
  rw [h, hz1024, hc1024, add_zero]
end SourceJets43
