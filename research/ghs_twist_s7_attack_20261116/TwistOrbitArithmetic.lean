import Init
set_option maxRecDepth 10000

/-! Checked integer inputs to the Frobenius orbit lemma. The general
kernel-descent argument is given in S7_RESULT.md; Lean does not formalize
elliptic-curve torsion or isogeny descent here. -/
namespace GHSTwistOrbitArithmetic

theorem u2 : (-26 : Int) * 1 - 243 * 0 = -26 := by decide
theorem u3 : (-26 : Int) * (-26) - 243 * 1 = 433 := by decide
theorem u4 : (-26 : Int) * 433 - 243 * (-26) = -4940 := by decide
theorem u5 : (-26 : Int) * (-4940) - 243 * 433 = 23221 := by decide
theorem u6 : (-26 : Int) * 23221 - 243 * (-4940) = 596674 := by decide
theorem u7 : (-26 : Int) * 596674 - 243 * 23221 = -21156227 := by decide

theorem square_bound : (4599 : Nat)^2 < 21156227 ∧ 21156227 < 4600^2 := by decide

def noSmallDivisor : Bool :=
  (List.range 4601).all (fun d => decide (d < 2 ∨ 21156227 % d ≠ 0))

theorem no_small_divisor : noSmallDivisor = true := by decide

end GHSTwistOrbitArithmetic
