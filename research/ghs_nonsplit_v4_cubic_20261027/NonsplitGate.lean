/-!
Kernel-checked arithmetic consequences of the nonsplit quadratic-pole count.
The field norm-image, projective orbit, trace, genus, and Jacobian-map
arguments are in REPORT.md and executable Sage/Magma receipts, not Lean.
-/

namespace GHSNonsplit

set_option maxRecDepth 8192
set_option maxHeartbeats 0

theorem q2_counts :
    (2^3+2-2)/2 = 4 ∧
    (2^3-2^2-4)/4 = 0 ∧
    2^3-1+2^2*(2+1)/4 = 10 := by decide

theorem q4_counts :
    (4^3+4-2)/2 = 33 ∧
    (4^3-4^2+4*4-4)/4 = 15 ∧
    4^3-1+4^2*(4+1)/4+15 = 98 := by decide

theorem q8_counts :
    (8^3+8-2)/2 = 259 ∧
    (8^3-8^2-4)/4 = 111 ∧
    8^3-1+8^2*(8+1)/4+111 = 766 := by decide

theorem q16_counts :
    (16^3+16-2)/2 = 2055 ∧
    (16^3-16^2+4*16-4)/4 = 975 ∧
    16^3-1+16^2*(16+1)/4+975 = 6158 ∧
    2*(16^3-1)-6158 = 2032 := by decide

theorem q32_counts :
    (32^3+32-2)/2 = 16399 ∧
    (32^3-32^2-4)/4 = 7935 ∧
    32^3-1+32^2*(32+1)/4+7935 = 49150 := by decide

theorem source_order : 4054 = 2*2027 := by decide

private def doubleMod (r x : Nat) : Nat := (2*x) % r

theorem source_2027_double_kernel :
    ∀ x : Fin 2027, doubleMod 2027 x.val = 0 → x.val = 0 := by decide

end GHSNonsplit
