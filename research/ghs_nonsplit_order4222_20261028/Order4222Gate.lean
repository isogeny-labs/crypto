/-!
Kernel-checked arithmetic gates for the exact order-4222 instance. The field
construction, smoothness, rational maps, and Jacobian divisor arithmetic are
checked by the retained Sage and Magma receipts, not by this Lean file.
-/

namespace GHSOrder4222

set_option maxRecDepth 8192
set_option maxHeartbeats 0

theorem source_order : 4222 = 2 * 2111 := by decide

theorem q16_nonsplit_count :
    (16^3 + 16 - 2)/2 = 2055 ∧
    (16^3 - 16^2 + 4*16 - 4)/4 = 975 ∧
    16^3 - 1 + 16^2*(16 + 1)/4 + 975 = 6158 ∧
    2*(16^3 - 1) - 6158 = 2032 := by decide

theorem target_scalar : 7 < 2111 ∧ 2 < 2111 ∧ 3 < 2111 ∧ 9 < 2111 := by decide

private def doubleMod (x : Nat) : Nat := (2*x) % 2111

theorem odd_subgroup_double_kernel :
    ∀ x : Fin 2111, doubleMod x.val = 0 → x.val = 0 := by decide

end GHSOrder4222
