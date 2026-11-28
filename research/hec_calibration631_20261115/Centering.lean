import Mathlib
namespace Calibration631
/-- Halving both primary parts gives an exact half of the whole class. -/
theorem mixed_center {A : Type*} [AddCommGroup A]
    (h odd two bo bt : A) (hh : h = odd + two)
    (ho : bo + bo = odd) (ht : bt + bt = two) :
    (bo + bt) + (bo + bt) = h := by
  calc
    (bo + bt) + (bo + bt) = (bo + bo) + (bt + bt) := by abel
    _ = odd + two := by rw [ho, ht]
    _ = h := hh.symm
/-- Fourfold clearing also removes a two-torsion translation. -/
theorem four_clears_translation {A : Type*} [AddCommGroup A]
    (x y d : A) (h : x = y + d) (hd : d + d = 0) :
    x + x + x + x = y + y + y + y := by
  rw [h]
  calc
    (y+d)+(y+d)+(y+d)+(y+d) = (y+y+y+y)+((d+d)+(d+d)) := by abel
    _ = y+y+y+y := by rw [hd]; simp
end Calibration631
#print axioms Calibration631.mixed_center
#print axioms Calibration631.four_clears_translation
