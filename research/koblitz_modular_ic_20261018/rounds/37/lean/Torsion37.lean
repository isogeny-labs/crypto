import Mathlib.GroupTheory.OrderOfElement
import Mathlib.Tactic.Abel

/-! Conditional group consequences; native classes and their geometric
interpretation are not imported or formalized here. -/
namespace Torsion37
variable {A : Type*}
section AnyGroup
variable [AddGroup A]

theorem exact_order_eleven (x : A) (h : 11 • x = 0) (hx : x ≠ 0) :
    addOrderOf x = 11 := by
  letI : Fact (Nat.Prime 11) := ⟨by decide⟩
  exact addOrderOf_eq_prime h hx

end AnyGroup
section CommGroup
variable [AddCommGroup A]
theorem coprime_five_eleven (x : A) (h5 : 5 • x = 0) (h11 : 11 • x = 0) :
    x = 0 := by
  calc
    x = 11 • x - 2 • (5 • x) := by abel
    _ = 0 := by rw [h5, h11]; simp

end CommGroup
end Torsion37
