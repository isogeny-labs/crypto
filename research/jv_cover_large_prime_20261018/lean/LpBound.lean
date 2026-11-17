/-!
Cost-model inequality of `COSTMODEL.md`, Proposition 3 (conditional form).

Hypothesis (open, see README "Open"): any scheme with a sub-base needs at least
as many decomposition solutions as the plain method, `x0 ≤ x` (the rank bound).
Given it, with per-solution cost `r` and a linear-algebra cost `a' ≥ 0`, the total
cost of the sub-base scheme is at least the plain method's solution cost, i.e. it
undercuts the plain total `x0 * r + a` by at most the plain linear-algebra cost `a`.
All quantities are natural numbers (counted multiplications).
-/

theorem lp_saving_le_la_share (x0 x r a a' : Nat) (h : x0 ≤ x) :
    x0 * r + a ≤ (x * r + a') + a := by
  have : x0 * r ≤ x * r := Nat.mul_le_mul_right r h
  omega

/-- Relative form: the saving `(x0*r + a) - (x*r + a')` never exceeds `a`. -/
theorem lp_saving_bound (x0 x r a a' : Nat) (h : x0 ≤ x) :
    (x0 * r + a) - (x * r + a') ≤ a := by
  have : x0 * r ≤ x * r := Nat.mul_le_mul_right r h
  omega
