import Mathlib.Tactic.Ring

namespace Relative87

variable {R : Type*} [CommRing R]

theorem square_shift (a b : R) (hchar : (2 : R) = 0) :
    (a + b) ^ 2 = a ^ 2 + b ^ 2 := by
  calc
    (a + b) ^ 2 = a ^ 2 + b ^ 2 + 2 * a * b := by ring
    _ = a ^ 2 + b ^ 2 := by rw [hchar]; ring

theorem fourth_shift (a b : R) (hchar : (2 : R) = 0) :
    (a + b) ^ 4 = a ^ 4 + b ^ 4 := by
  calc
    (a + b) ^ 4 = ((a + b) ^ 2) ^ 2 := by ring
    _ = (a ^ 2 + b ^ 2) ^ 2 := by rw [square_shift a b hchar]
    _ = (a ^ 2) ^ 2 + (b ^ 2) ^ 2 := square_shift _ _ hchar
    _ = a ^ 4 + b ^ 4 := by ring

theorem eighth_shift (a b : R) (hchar : (2 : R) = 0) :
    (a + b) ^ 8 = a ^ 8 + b ^ 8 := by
  calc
    (a + b) ^ 8 = ((a + b) ^ 4) ^ 2 := by ring
    _ = (a ^ 4 + b ^ 4) ^ 2 := by rw [fourth_shift a b hchar]
    _ = (a ^ 4) ^ 2 + (b ^ 4) ^ 2 := square_shift _ _ hchar
    _ = a ^ 8 + b ^ 8 := by ring

theorem sixteenth_shift (a b : R) (hchar : (2 : R) = 0) :
    (a + b) ^ 16 = a ^ 16 + b ^ 16 := by
  calc
    (a + b) ^ 16 = ((a + b) ^ 8) ^ 2 := by ring
    _ = (a ^ 8 + b ^ 8) ^ 2 := by rw [eighth_shift a b hchar]
    _ = (a ^ 8) ^ 2 + (b ^ 8) ^ 2 := square_shift _ _ hchar
    _ = a ^ 16 + b ^ 16 := by ring

end Relative87
