import Mathlib.Tactic.Ring

namespace Diamond85

variable {R : Type*} [CommRing R]

def equation (z V : R) : R := V^2 + z^2 * V + z + z^3
def h (z V : R) : R := z + V
def H (z V : R) : R := 1 + z + z^2 + z * V
def N (z V : R) : R := V + z * V + z^2

theorem first_open_product (z V : R) (hchar : (2 : R) = 0) :
    h z V ^ 2 + z * H z V = equation z V := by
  unfold h H equation
  calc
    (z + V)^2 + z * (1 + z + z^2 + z * V) =
        V^2 + z^2 * V + z + z^3 + 2 * z * (z + V) := by ring
    _ = V^2 + z^2 * V + z + z^3 := by rw [hchar]; ring

theorem second_open_numerator (z V : R) (hchar : (2 : R) = 0) :
    N z V + h z V + 1 = H z V := by
  unfold N h H
  calc
    V + z * V + z^2 + (z + V) + 1 =
        (1 + z + z^2 + z * V) + 2 * V := by ring
    _ = 1 + z + z^2 + z * V := by rw [hchar]; ring

theorem overlap_relation (z V : R) (hchar : (2 : R) = 0)
    (hE : equation z V = 0) : h z V ^ 2 + z * H z V = 0 := by
  rw [first_open_product z V hchar, hE]

end Diamond85
