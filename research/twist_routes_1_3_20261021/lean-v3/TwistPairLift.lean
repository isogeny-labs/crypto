import Mathlib.Tactic.Ring
import Mathlib.Tactic.FieldSimp

/- Classical Semaev F3 identities and an explicit coefficient-field lift.
   No novelty or complexity claim; the elliptic-curve addition bridge is separate. -/
namespace TwistPairLift

def cubic {K : Type*} [CommRing K] (x a b : K) : K := x^3 + a*x + b

def s3 {K : Type*} [CommRing K] (x y z a b : K) : K :=
  (x-y)^2*z^2 - 2*((x+y)*(x*y+a)+2*b)*z + (x*y-a)^2 - 4*b*(x+y)

def numerator {K : Type*} [CommRing K] (x y z a b : K) : K :=
  2*(x-z)^2*y - 2*((x+z)*(x*z+a)+2*b)

theorem swap_first_two {K : Type*} [CommRing K] (x y z a b : K) :
    s3 x y z a b = s3 y x z a b := by
  unfold s3
  ring

theorem quadratic_discriminant {K : Type*} [CommRing K] (x z a b : K) :
    (2*((x+z)*(x*z+a)+2*b))^2 -
      4*(x-z)^2*((x*z-a)^2-4*b*(x+z)) =
      16*cubic x a b*cubic z a b := by
  unfold cubic
  ring

theorem numerator_square {K : Type*} [CommRing K] (x y z a b : K) :
    (numerator x y z a b)^2 =
      16*cubic x a b*cubic z a b + 4*(x-z)^2*s3 x y z a b := by
  unfold numerator cubic s3
  ring

theorem explicit_square_lift {K : Type*} [Field K] (x y z a b ry : K)
    (hden : 4*ry ≠ 0) (htarget : ry^2 = cubic z a b)
    (hrelation : s3 x y z a b = 0) :
    (numerator x y z a b / (4*ry))^2 = cubic x a b := by
  rw [div_pow]
  apply (div_eq_iff (pow_ne_zero 2 hden)).2
  calc
    (numerator x y z a b)^2 =
      16*cubic x a b*cubic z a b + 4*(x-z)^2*s3 x y z a b :=
      numerator_square x y z a b
    _ = cubic x a b*(4*ry)^2 := by
      rw [hrelation, ← htarget]
      ring

theorem both_inputs_lift_over_same_field {K : Type*} [Field K]
    (x y z a b ry : K) (hden : 4*ry ≠ 0)
    (htarget : ry^2 = cubic z a b) (hrelation : s3 x y z a b = 0) :
    ∃ yx yy : K, yx^2 = cubic x a b ∧ yy^2 = cubic y a b := by
  refine ⟨numerator x y z a b / (4*ry), numerator y x z a b / (4*ry),
    explicit_square_lift x y z a b ry hden htarget hrelation, ?_⟩
  apply explicit_square_lift y x z a b ry hden htarget
  rw [← swap_first_two x y z a b]
  exact hrelation

end TwistPairLift
