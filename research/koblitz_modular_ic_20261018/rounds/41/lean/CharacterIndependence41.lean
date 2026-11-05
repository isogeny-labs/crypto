import Mathlib.Tactic.Abel
import Mathlib.GroupTheory.OrderOfElement

/-! Distinct integral eigencharacters force disjoint additive components.
The source/endormorphism bindings remain explicit hypotheses. -/
namespace CharacterIndependence41
variable {A : Type*} [AddCommGroup A]

theorem seven_eight_equal_forces_zero (x : A) (h : 7 • x=8 • x) : x=0 := by
  have hn : 8 • x-7 • x=x := by abel
  rw [← h, sub_self] at hn
  exact hn.symm

theorem eigen_sum_zero_components (t : A →+ A) (x y : A)
    (hx : t x=7 • x) (hy : t y=8 • y) (h : x+y=0) :
    x=0 ∧ y=0 := by
  have ht : 7 • x+8 • y=0 := by
    simpa only [map_add, hx, hy, map_zero] using congrArg t h
  have hn : x = -y := by
    calc
      x = (x+y)-y := by abel
      _ = -y := by rw [h, zero_sub]
  rw [hn] at ht
  have hs : 7 • (-y)+8 • y=y := by abel
  rw [hs] at ht
  constructor
  · rw [hn, ht, neg_zero]
  · exact ht

theorem eight_not_nat_multiple_of_seven (t : A →+ A) (x y : A)
    (hx : t x=7 • x) (hy : t y=8 • y) (hn : y≠0) (n : ℕ) :
    y≠n • x := by
  intro he
  have ht : t y=7 • y := by
    rw [he, map_nsmul, hx]
    calc
      n • (7 • x) = (7*n) • x := by rw [mul_nsmul]
      _ = (n*7) • x := by rw [Nat.mul_comm]
      _ = 7 • (n • x) := by rw [mul_nsmul]
  exact hn (seven_eight_equal_forces_zero y (ht.symm.trans hy))

end CharacterIndependence41
