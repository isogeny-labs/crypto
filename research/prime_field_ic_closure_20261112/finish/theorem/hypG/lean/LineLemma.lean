import Mathlib.Algebra.BigOperators.Intervals
import Mathlib.Algebra.Field.Basic

/-!
# Recurrence core of the tower line lemma (follow-up hyp-G)

Status: `[pcc:unverified]` research scaffolding.

`recurrence_kills`: if a sequence `C` satisfies the order-`r` linear recurrence
`∑_{i ≤ r} Q i * C (b - i) = 0` for `r ≤ b < N` with leading coefficient `Q r ≠ 0`, and `C`
vanishes on the top run `[N - L, N)` of length `L ≥ r`, then `C` vanishes on `[0, N)`.

In the line lemma, `C a` is the coefficient of `x^a` in the reduced representative of
`q = P / Q` on `W = g·μ_N`, `N = 2^k`; `Q · q ≡ P (mod x^N - c)` with `deg P < r = deg Q` gives the
recurrence; a weight-degree `≤ t` forces vanishing on the top run of length `2^(k-t-1)`, whose
indices all have their top `t+1` bits set.  The bit-counting step and the reduction modulo
`x^N - c` are not formalised here.
-/

open Finset

theorem recurrence_kills {K : Type*} [Field K] (C Q : ℕ → K) (r N L : ℕ)
    (hQ : Q r ≠ 0) (hrL : r ≤ L) (hLN : L ≤ N)
    (hrec : ∀ b, r ≤ b → b < N → ∑ i ∈ range (r + 1), Q i * C (b - i) = 0)
    (hrun : ∀ a, N - L ≤ a → a < N → C a = 0) :
    ∀ a, a < N → C a = 0 := by
  have key : ∀ j a, N - L - j ≤ a → a < N → C a = 0 := by
    intro j
    induction j with
    | zero => intro a h1 h2; exact hrun a (by simpa using h1) h2
    | succ j ih =>
      intro a h1 h2
      by_cases hc : N - L - j ≤ a
      · exact ih a hc h2
      · push Not at hc
        -- a + L < N
        have haL : a + L < N := by omega
        have hb1 : r ≤ a + r := Nat.le_add_left r a
        have hb2 : a + r < N := by omega
        have hs := hrec (a + r) hb1 hb2
        rw [sum_range_succ] at hs
        have hzero : ∑ i ∈ range r, Q i * C (a + r - i) = 0 := by
          apply sum_eq_zero
          intro i hi
          rw [mem_range] at hi
          have : C (a + r - i) = 0 := ih (a + r - i) (by omega) (by omega)
          rw [this, mul_zero]
        rw [hzero, zero_add, Nat.add_sub_cancel] at hs
        rcases mul_eq_zero.mp hs with h | h
        · exact absurd h hQ
        · exact h
  intro a ha
  exact key N a (by omega) ha

/-- The top run: every `a` with `2^k - 2^j ≤ a < 2^k` (`j ≤ k`) has bit `i` set for every
`j ≤ i < k`.  With `j = k - t - 1` these are `t + 1` set bits, so such an `a` has binary weight
`> t`: the run `[2^k - 2^(k-t-1), 2^k)` lies inside `{a : wt a > t}`. -/
theorem top_run_bits (k j a : ℕ) (hjk : j ≤ k) (h1 : 2 ^ k - 2 ^ j ≤ a) (h2 : a < 2 ^ k) :
    ∀ i, j ≤ i → i < k → a.testBit i = true := by
  intro i hi1 hi2
  have hpow : 2 ^ k = 2 ^ (k - j) * 2 ^ j := by
    rw [← pow_add]; congr 1; omega
  have hpos2 : 1 ≤ 2 ^ (k - j) := Nat.one_le_two_pow
  have hq : a / 2 ^ j = 2 ^ (k - j) - 1 := by
    apply Nat.div_eq_of_lt_le
    · rw [Nat.sub_mul, one_mul, ← hpow]; exact h1
    · rw [Nat.sub_add_cancel hpos2, ← hpow]; exact h2
  have := Nat.testBit_div_two_pow (n := j) a (i - j)
  rw [hq, Nat.testBit_two_pow_sub_one, Nat.sub_add_cancel hi1] at this
  rw [← this]
  simp only [decide_eq_true_eq]
  omega

#print axioms recurrence_kills
#print axioms top_run_bits
