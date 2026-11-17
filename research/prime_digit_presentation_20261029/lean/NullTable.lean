import Lean

/-! Arithmetic of the registered table in `PROTOCOL.md` section 4 (core Lean, no Mathlib).

This file checks integer arithmetic only.  It does NOT formalize, and does not prove,

* the closed form `d_ff(m, 2, s) = m 2^(m-1) + (s - 2^(m-1)) / 2 + 1`, which the ledger
  (H-PFDR-4148b8) derives from Wilson's rank theorem on inclusion matrices (H-WIL);
  Missing: the graded annihilator of `(sum a_i)^e` in the squarefree algebra, Wilson's
  theorem, and the tensor-product kernel formula.  None is in core Lean;
* the null bound `D_null = (n + delta) / 2 + 1`, which follows from a dimension count
  `binom n j > binom n (j + delta)` that needs the (unformalized here) fact that the
  support-matched null has full-rank multiplication;
* any measured value.

What is checked: the numbers the protocol registers. -/
namespace PrimeDigitPresentation

/-- the closed form for the first fall degree, `2^(m-1) <= s` -/
def law (m s : Nat) : Nat := m * 2 ^ (m - 1) + (s - 2 ^ (m - 1)) / 2 + 1
/-- the first fall forced by dimension counting for a support-matched null in `n` variables, top degree `delta` -/
def dNull (n delta : Nat) : Nat := (n + delta) / 2 + 1

theorem law_m2 : (List.range 7).map (fun i => law 2 (i + 2)) = [5, 5, 6, 6, 7, 7, 8] := by decide
theorem law_m3 : [law 3 4, law 3 5] = [13, 13] := by decide
/-- `delta = m 2^(m-1)`; for `m = 2`, `n = 2 s`: `dNull = s + 3` -/
theorem dnull_m2 : (List.range 7).map (fun i => dNull (2 * (i + 2)) 4) = [5, 6, 7, 8, 9, 10, 11] := by decide
/-- the registered strict gap: the null falls strictly later than the closed form for `s = 3 .. 8` -/
theorem null_strictly_later : ∀ s ∈ List.range 6, law 2 (s + 3) < dNull (2 * (s + 3)) 4 := by decide
/-- no strict gap at the boundary cell `s = 2` -/
theorem boundary_equal : law 2 2 = dNull 4 4 := by decide
/-- the plain presentation reads `p + 2` on the probes; the digit law is below it for `p > 2^s`, `s = 2 .. 8` -/
theorem digit_below_plain : ∀ s ∈ List.range 7, law 2 (s + 2) < 2 ^ (s + 2) + 2 := by decide

end PrimeDigitPresentation
