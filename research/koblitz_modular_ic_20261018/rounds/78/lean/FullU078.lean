import Mathlib.Tactic.Ring

namespace FullU078

variable {R : Type*} [CommRing R]

theorem chart_overlap (u z v V : R) (huz : u * z = 1)
    (hV : V = z^2 * v) : u^2 * V = v := by
  rw [hV]
  calc
    u^2 * (z^2 * v) = (u*z)^2 * v := by ring
    _ = v := by rw [huz]; ring

private theorem square_sum (a b : R) (hchar : (2 : R) = 0) :
    (a+b)^2 = a^2+b^2 := by
  calc
    _ = a^2+b^2+2*a*b := by ring
    _ = a^2+b^2 := by rw [hchar]; ring

private theorem eighth_sum (a b : R) (hchar : (2 : R) = 0) :
    (a+b)^8 = a^8+b^8 := by
  calc
    _ = ((a+b)^2)^4 := by ring
    _ = (a^2+b^2)^4 := by rw [square_sum a b hchar]
    _ = ((a^2+b^2)^2)^2 := by ring
    _ = ((a^2)^2+(b^2)^2)^2 := by rw [square_sum (a^2) (b^2) hchar]
    _ = ((a^2)^2)^2+((b^2)^2)^2 := by rw [square_sum ((a^2)^2) ((b^2)^2) hchar]
    _ = a^8+b^8 := by ring

theorem repeated_quadratic_factor (x : R) (hchar : (2 : R) = 0) :
    (x^2+x+1)^8 = x^16+x^8+1 := by
  calc
    (x^2+x+1)^8 = (x^2+(x+1))^8 := by ring
    _ = (x^2)^8+(x+1)^8 := eighth_sum (x^2) (x+1) hchar
    _ = x^16+x^8+1 := by rw [eighth_sum x 1 hchar]; ring

theorem two_component_length : 16+16=32 := by decide

end FullU078
