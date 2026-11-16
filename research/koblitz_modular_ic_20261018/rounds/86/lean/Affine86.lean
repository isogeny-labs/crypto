import Mathlib.Tactic.Ring

namespace Affine86

variable {R : Type*} [CommRing R]

def sourceB (u c : R) : R := u * c ^ 2 + c
def diamondK (b c : R) : R := b + (1 + c) * c

theorem k_simplification (u c : R) (hchar : (2 : R) = 0) :
    diamondK (sourceB u c) c = (u + 1) * c ^ 2 := by
  unfold diamondK sourceB
  calc
    u * c ^ 2 + c + (1 + c) * c =
        (u + 1) * c ^ 2 + 2 * c := by ring
    _ = (u + 1) * c ^ 2 := by rw [hchar]; ring

theorem lambda_cross_product (u c l : R) (hchar : (2 : R) = 0) :
    c * ((l * c + diamondK (sourceB u c) c) *
        diamondK (sourceB u c) c) =
      c ^ 4 * ((u + 1) * (l + (u + 1) * c)) := by
  rw [k_simplification u c hchar]
  ring

end Affine86
