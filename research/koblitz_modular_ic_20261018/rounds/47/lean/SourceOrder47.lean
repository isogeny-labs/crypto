import Mathlib.GroupTheory.OrderOfElement
namespace SourceOrder47
/-- Exact order from the two Picard witness outcomes, conditional on their
interpretation as an additive group. Native spaces/geometry are not imported. -/
theorem exact_order_eleven {A : Type*} [AddGroup A] (x : A)
    (hne : x≠0) (hkill : 11 • x=0) : addOrderOf x=11 := by
  letI : Fact (Nat.Prime 11) := ⟨by decide⟩
  exact addOrderOf_eq_prime hkill hne
end SourceOrder47
