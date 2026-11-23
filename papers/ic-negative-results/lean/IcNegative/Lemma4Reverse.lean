import IcNegative.Lemma4Witness

/-!
# The reverse inequality `d'_F ≤ d_F` (field equations outside) fails

Companion to `research/disprove_easy_targets_20261102/` (target A) and the corpus node
`GAP-LEMMA4-REVERSE`. After Nagao's Lemma 4 (`d_F ≤ d'_F`) was refuted, the reverse
inequality with the field equations outside the generating set was recorded as a
candidate repair; the prior census of multilinear pairs had no counterexample.

Witness R: `f_1 = X`, `f_2 = X^2 + X` in `F_2[X, Y]`.
* True first fall degree (field equations outside) is 2: `X · f_1 + 1 · f_2 = X`, both
  products of degree 2, the sum of degree 1 and nonzero; no fall at 0 or 1.
* No fake fall at any degree: `f_2` reduces to 0 modulo the field equations, so the
  reduced system has the single effective generator `X`, and a single product can never
  have a sum of lower degree than itself.

So `d'_F = ∞ > 2 = d_F`. The census in `research/disprove_easy_targets_20261102/results/`
finds 7126 such pairs among all 521731 pairs of degree ≤ 2 in three variables (204 of
1891 in two variables), every one degenerate in this way (the reduced generators are
linearly dependent); none with linearly independent reduced generators. The surviving open form adds that hypothesis.

Encodings and their stated bridges are those of `IcNegative.Lemma4Witness`.
-/

namespace IcNegative.Lemma4Reverse

open IcNegative.Lemma4

/-- `X`. -/
def fR1 : Nat := 1 <<< 8
/-- `X^2 + X`, a field equation used as a generator. -/
def fR2 : Nat := (1 <<< 16) ^^^ (1 <<< 8)

/-- Generating set with the field equations outside. -/
def outsideR : List Nat := [fR1, fR2]
/-- The reduced system `[X, 0]`. -/
def fakeR : List Nat := [red fR1, red fR2]

theorem red_fR1 : red fR1 = 1 <<< 1 := by decide +kernel
theorem red_fR2 : red fR2 = 0 := by decide +kernel

/-- A true fall at degree 2: `g = (X, 1)`, sum `X`. -/
theorem outside_fall2 : IsFall outsideR [1 <<< 8, 1] 2 := by decide +kernel

/-- No true fall at degree 0 or 1. -/
theorem outside_noFall_below2 :
    ∀ d ∈ [0, 1], ∀ gs ∈ tuples outsideR d, ¬ IsFall outsideR gs d := by
  decide +kernel

/-- Every reduced product has degree at most 2. -/
theorem fake_qmaxDeg_le : ∀ gs ∈ qtuples fakeR, qmaxDeg (qproducts fakeR gs) ≤ 2 := by
  decide +kernel

/-- No fake fall at degree 0, 1 or 2. -/
theorem fake_noFall_le2 :
    ∀ d ∈ [0, 1, 2], ∀ gs ∈ qtuples fakeR, ¬ IsFakeFall fakeR gs d := by
  decide +kernel

/-- No fake fall at any degree. -/
theorem fake_noFall (d : Nat) : ∀ gs ∈ qtuples fakeR, ¬ IsFakeFall fakeR gs d := by
  intro gs hgs hfall
  by_cases hd : d ≤ 2
  · have h3 : d = 0 ∨ d = 1 ∨ d = 2 := by omega
    rcases h3 with rfl | rfl | rfl
    · exact fake_noFall_le2 0 (by decide) gs hgs hfall
    · exact fake_noFall_le2 1 (by decide) gs hgs hfall
    · exact fake_noFall_le2 2 (by decide) gs hgs hfall
  · have hle := fake_qmaxDeg_le gs hgs
    have heq := hfall.2.1
    omega

/-- Witness R refutes `d'_F ≤ d_F` with the field equations outside: `d_F = 2`, while
no fake fall exists at any degree. -/
theorem reverse_fails_witnessR :
    ((∃ gs, IsFall outsideR gs 2) ∧
      ∀ d ∈ [0, 1], ∀ gs ∈ tuples outsideR d, ¬ IsFall outsideR gs d) ∧
    ∀ d, ∀ gs ∈ qtuples fakeR, ¬ IsFakeFall fakeR gs d :=
  ⟨⟨⟨_, outside_fall2⟩, outside_noFall_below2⟩, fake_noFall⟩

end IcNegative.Lemma4Reverse
