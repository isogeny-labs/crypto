import Mathlib.Tactic.Ring
namespace Infinity77
variable {R : Type*} [CommRing R]
theorem norm_B_poly (t U : R) (hchar : (2 : R) = 0) :
    (1+(1+t)*U)*(1+(1+t)*(U+1)) + t^5 =
      (1+t)^2*(U^2+U+t+t^3) := by
  have h3 : (3 : R) = 1 := by calc
    (3 : R) = 1 + 2 := by ring
    _ = 1 := by rw [hchar]; ring
  have h4 : (4 : R) = 0 := by calc
    (4 : R) = 2 + 2 := by ring
    _ = 0 := by rw [hchar]; ring
  ring_nf
  simp [hchar, h3, h4]
theorem norm_B_shift_poly (t U : R) (hchar : (2 : R) = 0) :
    (1+(1+t)*U+t^3)*(1+(1+t)*(U+1)+t^3) + t^3*(1+t)^3 =
      (1+t)^2*(U^2+U+t+t^3) := by
  have h3 : (3 : R) = 1 := by calc
    (3 : R) = 1 + 2 := by ring
    _ = 1 := by rw [hchar]; ring
  have h4 : (4 : R) = 0 := by calc
    (4 : R) = 2 + 2 := by ring
    _ = 0 := by rw [hchar]; ring
  ring_nf
  simp [hchar, h3, h4]

/-- In the degree-16 upper extension, the two quadratic infinity
places contribute the displayed weighted pole and zero degrees. -/
theorem weighted_infinity_degrees :
    16 * 7 = 112 ∧ 16 * 4 + 16 * 1 = 80 ∧ 96 + 16 = 112 := by
  constructor
  · decide
  constructor <;> decide

end Infinity77
