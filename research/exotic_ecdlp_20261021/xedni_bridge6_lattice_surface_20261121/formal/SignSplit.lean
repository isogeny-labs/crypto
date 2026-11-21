import Mathlib.Tactic

/-! Equal x-coordinates on a nonsingular Weierstrass model have two y branches. -/

namespace XedniBridge6LatticeSurface

theorem same_x_weierstrass_sign_split {R : Type*} [CommRing R] [NoZeroDivisors R]
    (a1 a3 x yP yR rhs : R)
    (hP : yP ^ 2 + a1 * x * yP + a3 * yP = rhs)
    (hR : yR ^ 2 + a1 * x * yR + a3 * yR = rhs) :
    yR = yP ∨ yR = -(yP + a1 * x + a3) := by
  have hproduct : (yR - yP) * (yR + yP + a1 * x + a3) = 0 := by
    calc
      (yR - yP) * (yR + yP + a1 * x + a3) =
          (yR ^ 2 + a1 * x * yR + a3 * yR) -
            (yP ^ 2 + a1 * x * yP + a3 * yP) := by ring
      _ = 0 := by rw [hR, hP]; ring
  rcases mul_eq_zero.mp hproduct with hsame | hinverse
  · left
    exact sub_eq_zero.mp hsame
  · right
    have hsum : yR + (yP + a1 * x + a3) = 0 := by
      simpa only [add_assoc] using hinverse
    exact add_eq_zero_iff_eq_neg.mp hsum

#print axioms same_x_weierstrass_sign_split

end XedniBridge6LatticeSurface
