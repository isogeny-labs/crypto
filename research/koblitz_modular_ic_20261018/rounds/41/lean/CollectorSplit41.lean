import Mathlib.Tactic.Abel
import Mathlib.GroupTheory.OrderOfElement

/-! Conditional character splitting for an element killed by eleven.
No native arrays, source Picard group or modular target is imported. -/
namespace CollectorSplit41
variable {A : Type*} [AddCommGroup A]

theorem recover_components (x y : A) :
    (8 • x-y)+(y-7 • x)=x := by abel

theorem fifty_six_reduces (x : A) (hx : 11 • x=0) : 56 • x=x := by
  have hn : 56=1+11*5 := by decide
  rw [hn, add_nsmul, one_nsmul, mul_nsmul, hx, nsmul_zero, add_zero]

theorem seven_component_linear (x y z : A) (hx : 11 • x=0)
    (hy : 11 • y=0) (h : z+x=4 • y) :
    8 • y-z=7 • (8 • x-y) := by
  have hz : z=4 • y-x := eq_sub_iff_add_eq.mpr h
  have hs : 4 • y+7 • y=0 := by rw [← add_nsmul]; exact hy
  rw [nsmul_sub, ← mul_nsmul, show 8*7=56 by decide, fifty_six_reduces x hx]
  apply eq_sub_iff_add_eq.mpr
  rw [hz]
  calc
    (8 • y-(4 • y-x))+7 • y = x+(4 • y+7 • y) := by abel
    _ = x := by rw [hs, add_zero]

theorem eight_component_linear (x y z : A) (hx : 11 • x=0)
    (hy : 11 • y=0) (h : z+x=4 • y) :
    z-7 • y=8 • (y-7 • x) := by
  have hz : z=4 • y-x := eq_sub_iff_add_eq.mpr h
  have hy15 : 15 • y=4 • y := by
    rw [show 15=4+11 by decide, add_nsmul, hy, add_zero]
  have hd : 4 • y-7 • y=8 • y := by
    apply sub_eq_iff_eq_add.mpr
    rw [← add_nsmul]
    exact hy15.symm
  rw [nsmul_sub, ← mul_nsmul, show 7*8=56 by decide, fifty_six_reduces x hx, hz]
  calc
    (4 • y-x)-7 • y = (4 • y-7 • y)-x := by abel
    _ = 8 • y-x := by rw [hd]

theorem seven_component_equivariant (t : A →+ A) (x : A)
    (hx : 11 • x=0) (h : t (t x)+x=4 • t x) :
    t (8 • x-t x)=7 • (8 • x-t x) := by
  have hy : 11 • t x=0 := by
    rw [← map_nsmul, hx, map_zero]
  rw [map_sub, map_nsmul]
  exact seven_component_linear x (t x) (t (t x)) hx hy h

theorem eight_component_equivariant (t : A →+ A) (x : A)
    (hx : 11 • x=0) (h : t (t x)+x=4 • t x) :
    t (t x-7 • x)=8 • (t x-7 • x) := by
  have hy : 11 • t x=0 := by
    rw [← map_nsmul, hx, map_zero]
  rw [map_sub, map_nsmul]
  exact eight_component_linear x (t x) (t (t x)) hx hy h

end CollectorSplit41
