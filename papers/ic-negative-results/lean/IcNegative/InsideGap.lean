import IcNegative.Lemma4Witness

/-!
# With the field equations inside, the true first fall degree is at most 4

Companion to `research/inside_gap_n4_20261102/` (question `GAP-FAKE-GAP-INSIDE-BOUNDED`).

For `n ≥ 2` variables and every generating set `F` over `F_2`, the system
`F ∪ {X_i^2 + X_i}` has a fall at degree 4 made of field equations alone:
`Y^2 (X^2 + X) + X^2 (Y^2 + Y) = X Y^2 + X^2 Y` (both products of degree 4, the sum of
degree 3 and nonzero; every generator of `F` gets the multiplier 0). Hence
`d_F(F ∪ S_fe) ≤ 4`, and since `d'_F ≥ 1`, the inside gap `d_F − d'_F` is at most 3. The
gap 3 is attained at `n = 4` (`{XYW + XY + W, XZW + XW + 1}`, native, two implementations).
With one variable there is no second field equation, and `{X^k, X^k + 1}` has
`d_F(inside) = k`, `d'_F = 1`: the gap is unbounded for `n = 1` only.

This is the "trivial degree fall" from field equations that Ding–Hodges and
Petit–Quisquater exclude in their definition of the first fall degree; the equality
reading of Nagao's Definition 5 used throughout `papers/ic-negative-results` does not
exclude it, so with the field equations inside it never exceeds 4.

Kernel-checked here: the field-equation fall at 4 (alone, and padded with witness A's
generators), and that the field equations alone have no fall below 4. The step "zero
multipliers on any further generators leave the fall intact" is argued, not formalised.
-/

namespace IcNegative.InsideGap

open IcNegative.Lemma4

/-- The two field equations of `F_2[X, Y]`. -/
def feXY : List Nat := [feX, feY]

/-- A fall at degree 4 from the field equations alone: multipliers `(Y^2, X^2)`,
sum `X Y^2 + X^2 Y`. -/
theorem fe_fall4 : IsFall feXY [1 <<< 2, 1 <<< 16] 4 := by decide +kernel

/-- The field equations alone have no fall below 4, so `d_F(S_fe) = 4`. -/
theorem fe_noFall_below4 :
    ∀ d ∈ [0, 1, 2, 3], ∀ gs ∈ tuples feXY d, ¬ IsFall feXY gs d := by
  decide +kernel

/-- The same fall inside witness A's system: `d_F(insideA) ≤ 4`. -/
theorem insideA_fall4 : IsFall insideA [0, 0, 1 <<< 2, 1 <<< 16] 4 := by decide +kernel

/-- `d_F(S_fe) = 4` in two variables. -/
theorem fe_first_fall_four :
    (∃ gs, IsFall feXY gs 4) ∧ ∀ d ∈ [0, 1, 2, 3], ∀ gs ∈ tuples feXY d, ¬ IsFall feXY gs d :=
  ⟨⟨_, fe_fall4⟩, fe_noFall_below4⟩

end IcNegative.InsideGap
