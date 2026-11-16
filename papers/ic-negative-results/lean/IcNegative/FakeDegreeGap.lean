import IcNegative.Lemma4Witness

/-!
# The fake first fall degree does not bound the true one (field equations outside)

Companion to `research/lean_attacks_20261102/` (attack A1). After Nagao's Lemma 4
(`d_F ≤ d'_F`) was refuted, an estimate of the form "the fake degree is small, so the
true first fall degree is small" would still go through if some weaker repair held, such
as `d_F ≤ d'_F + c` or `d_F ≤ f(d'_F)` for some function `f`. None does, with the field
equations outside the generating set.

Family: `W_k = {X^k Y, XY + X}` over `F_2`, `k ≥ 2` (`W_2` is witness A).
* Reduced modulo the field equations, `X^k Y ↦ XY`, so the fake system is the same for
  every `k` and `d'_F(W_k) = 2` (`(XY) + (XY + X) = X`).
* Outside, `X^{k-1} (XY + X) + X^k Y = X^k` is a fall at `k + 1`. Below `k + 1` no
  nonzero multiple of `X^k Y` is admissible, and the multiples of the single remaining
  generator never fall (the sum is one product), so `d_F(W_k) = k + 1`.

So `d_F - d'_F = k - 1` is unbounded. This module kernel-checks `k = 3`
(`d'_F = 2`, `d_F = 4`), which already refutes `d_F ≤ d'_F + 1`; the native census
(`research/lean_attacks_20261102/results/family.txt`) checks `k = 2..7`. With the field
equations inside, `d_F(W_k) = 3` for every `k` computed: the gap is an artefact of
leaving the field equations out.

Encodings and their stated bridges are those of `IcNegative.Lemma4Witness`.
-/

namespace IcNegative.FakeDegreeGap

open IcNegative.Lemma4

/-- `X^3 Y`. -/
def fW1 : Nat := 1 <<< 25

/-- `W_3` with the field equations outside. -/
def outsideW3 : List Nat := [fW1, fA2]
/-- The reduced system `[XY, XY + X]`. -/
def fakeW3 : List Nat := [red fW1, red fA2]

theorem deg_fW1 : deg fW1 = 4 := by decide +kernel
theorem red_fW1 : red fW1 = 1 <<< 3 := by decide +kernel

/-- The reduced system is the same as witness A's. -/
theorem fakeW3_eq_fakeA : fakeW3 = fakeA := by decide +kernel

/-- A true fall at degree 4: `g = (1, X^2)`, sum `X^3`. -/
theorem outside_fall4 : IsFall outsideW3 [1, 1 <<< 16] 4 := by decide +kernel

/-- No true fall at degree 0, 1, 2 or 3. -/
theorem outside_noFall_below4 :
    ∀ d ∈ [0, 1, 2, 3], ∀ gs ∈ tuples outsideW3 d, ¬ IsFall outsideW3 gs d := by
  decide +kernel

/-- A fake fall at degree 2: `g = (1, 1)`, reduced sum `X`. -/
theorem fake_fall2 : IsFakeFall fakeW3 [1, 1] 2 := by decide +kernel

/-- No fake fall at degree 0 or 1. -/
theorem fake_noFall_below2 :
    ∀ d ∈ [0, 1], ∀ gs ∈ qtuples fakeW3, ¬ IsFakeFall fakeW3 gs d := by
  decide +kernel

/-- `W_3`: `d'_F = 2` and `d_F = 4` with the field equations outside, so
`d_F ≤ d'_F + 1` fails. -/
theorem gap_two_witnessW3 :
    ((∃ gs, IsFakeFall fakeW3 gs 2) ∧
      ∀ d ∈ [0, 1], ∀ gs ∈ qtuples fakeW3, ¬ IsFakeFall fakeW3 gs d) ∧
    ((∃ gs, IsFall outsideW3 gs 4) ∧
      ∀ d ∈ [0, 1, 2, 3], ∀ gs ∈ tuples outsideW3 d, ¬ IsFall outsideW3 gs d) :=
  ⟨⟨⟨_, fake_fall2⟩, fake_noFall_below2⟩, ⟨⟨_, outside_fall4⟩, outside_noFall_below4⟩⟩

end IcNegative.FakeDegreeGap
