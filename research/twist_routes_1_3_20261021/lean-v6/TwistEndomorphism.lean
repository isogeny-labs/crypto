import Mathlib.Algebra.Group.Defs
import Mathlib.Tactic.LinearCombination
import Mathlib.Tactic.NormNum

namespace TwistEndomorphism

/- Conditional ring implications. The concrete curve/endomorphism bridge and
   commutative endomorphism-ring construction are not formalized here. -/
theorem norm_polynomial {R : Type*} [CommRing R] (a b : R)
    (hsum : a+b=1) (hnorm : b*a=17) : a*a-a+17=0 := by
  linear_combination a*hsum-hnorm

theorem frobenius_polynomial {R : Type*} [CommRing R] (a : R)
    (ha : a*a-a+17=0) : (7*a-1)^2-5*(7*a-1)+827=0 := by
  linear_combination 49*ha

theorem inverse_scalar {A : Type*} [AddCommGroup A]
    (ht : ∀ a : A, 823 • a = 0) (a : A) : 415 • (353 • a) = a := by
  calc
    415 • (353 • a) = (146495 : ℕ) • a := by rw [← mul_nsmul]
    _ = (1+178*823 : ℕ) • a := by norm_num
    _ = a+178 • (823 • a) := by rw [add_nsmul, one_nsmul, ← mul_nsmul]
    _ = a := by rw [ht]; simp

end TwistEndomorphism
