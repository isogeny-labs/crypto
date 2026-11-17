import Std

/- The geometric theorem's *numerical implication*. The hypotheses expose
the missing geometry/CFT bridge; they do not assume the genus conclusion.
This file does not formalize curves, Jacobians, or class field theory. -/
theorem cyclic_window_numeric
    (g b d R : Int)
    (hb : 0 ≤ b) (hd : 1 ≤ d) (hR : 0 ≤ R)
    (hRH : g = 131 * (b - 1) + 1 + 65 * R)
    (hSpectral : 130 * d ≤ g - b)
    (hDivisors : d = 1 ∨ d = 2 ∨ 5 ≤ d)
    (hBranch : b ≤ 3 → d ≤ 2 → 65 ≤ R) :
    394 ≤ g := by
  apply Classical.byContradiction
  intro h
  have hb3 : b ≤ 3 := by omega
  have hd2 : d ≤ 2 := by omega
  have hr65 : 65 ≤ R := hBranch hb3 hd2
  omega

theorem dimension_three_order_bound : (5 : Nat)^3 = 125 ∧ 125 < 131 := by
  decide

#print axioms cyclic_window_numeric
#print axioms dimension_three_order_bound
