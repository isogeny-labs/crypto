import Mathlib.GroupTheory.OrderOfElement
namespace Coordinate48
/-- Exhaustive coordinates 0 through 10 in a cyclic source line of exact
order 11 are unique. No native class or geometry is imported. -/
theorem coordinate_unique {A : Type*} [AddGroup A] (x : A)
    (horder : addOrderOf x=11) {a b : ℕ} (ha : a<11) (hb : b<11)
    (he : a • x=b • x) : a=b := by
  apply nsmul_injOn_Iio_addOrderOf (x := x)
  · simpa [horder] using ha
  · simpa [horder] using hb
  · exact he
end Coordinate48
