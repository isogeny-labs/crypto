import Init

/-!
Checked finite arithmetic and the logical form of the Hess-isogeny closure
screen. The external Sage certificate supplies the counts. Hess's genus
theorem, finite-field point counting, Tate's theorem, and concrete maps are
not formalized in this file.
-/

set_option autoImplicit false

namespace GhsCubicHessClosureQ16

theorem normalized_models : 2 * (4096 - 1) = 8190 := by decide
theorem trace_zero_nonzero_count : 16^2 - 1 = 255 := by decide
theorem product_set_count :
    (16 - 1) * (16 + 1) * (16 + 2) / 2 = 2295 := by decide
theorem product_set_partition : 255 + 2040 = 2295 := by decide
theorem direct_hess_partition : 2550 + 5640 = 8190 := by decide
theorem closure_partition : 5315 + 2875 = 8190 := by decide
theorem closure_order_partition : 79 + 49 = 128 := by decide
theorem residual_catalog_partition : 2827 + 48 = 2875 := by decide
theorem residual_catalog_order_partition : 47 + 2 = 49 := by decide
theorem quartic_witness_models : 30 + 48 + 72 + 108 = 258 := by decide
theorem focus_4036 : 48 + 48 = 96 := by decide
theorem focus_4124 : 84 + 12 = 96 := by decide
theorem focus_4126 : 0 + 108 = 108 := by decide
theorem order_4126 : 2 * 2063 = 4126 := by decide
theorem derivative_4126 : (6 : Int) - 3 * (-29) = 93 := by decide
theorem simple_root_4126 : Nat.gcd 2063 93 = 1 := by decide

/-- The closure predicate used by the exhaustive order-class screen. -/
def InOrderClosure {E : Type} (order : E → Nat)
    (eligible : E → Prop) (x : E) : Prop :=
  ∃ y, order y = order x ∧ eligible y

/-- A negative closure label rules out every same-order eligible model. -/
theorem outside_closure_iff {E : Type} (order : E → Nat)
    (eligible : E → Prop) (x : E) :
    ¬ InOrderClosure order eligible x ↔
      ∀ y, order y = order x → ¬ eligible y := by
  constructor
  · intro h y hy he
    exact h ⟨y, hy, he⟩
  · intro h ⟨y, hy, he⟩
    exact h y hy he

end GhsCubicHessClosureQ16
