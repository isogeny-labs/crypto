import Mathlib.Analysis.PSeries

/-!
# Vanishing survival tails do not imply finite cumulative tail

The sequence `S n = 1 / (n + 1)` tends to zero, but its partial sums tend to
infinity.  This is the exact analytic obstruction to inferring a bounded
cumulative tail from pointwise convergence alone.

The first theorem below is deliberately **not** called an expectation
theorem.  Interpreting a sequence as survival probabilities still requires a
probability space and a waiting-time random variable.  The committed
finite-truncation predecessor supplies one tail-sum identity; an infinite or
nontermination bridge requires additional measure-theoretic work.

The sufficient upper-tail premise has the opposite hazard direction from the JKSST runtime lower
bound.  A uniform *lower* stopping hazard `delta` gives the upper recurrence
`S (n + 1) ≤ (1 - delta) * S n`, hence a finite geometric upper tail.  The
JKSST one-attempt *upper* success bound would instead give a survival lower
bound and a runtime lower bound, after the missing repeated-trial bridge.
-/

open Filter
open scoped BigOperators Topology

namespace SurvivalTailExpectation

/-- Pointwise survival can vanish while the cumulative survival tail diverges.

Both conjuncts are existing Mathlib theorems used as stated: the first is the
limit of reciprocal natural successors and the second is divergence of the
harmonic partial sums. -/
theorem harmonic_survival_tail :
    Tendsto (fun n : ℕ ↦ (1 / (n + 1) : ℝ)) atTop (𝓝 0) ∧
    Tendsto (fun N : ℕ ↦
      ∑ n ∈ Finset.range N, (1 / (n + 1) : ℝ)) atTop atTop :=
  ⟨tendsto_one_div_add_atTop_nhds_zero_nat,
    Real.tendsto_sum_range_one_div_nat_succ_atTop⟩

/-- Any normalized real sequence satisfying the displayed upper recurrence is
bounded pointwise by the corresponding geometric sequence. The survival
interpretation and nonnegativity assumptions enter in the wrapper below. -/
theorem recurrence_le_geometric
    (S : ℕ → ℝ) (delta : ℝ)
    (hS0 : S 0 = 1)
    (hdelta1 : delta ≤ 1)
    (hstep : ∀ n, S (n + 1) ≤ (1 - delta) * S n) :
    ∀ n, S n ≤ (1 - delta) ^ n := by
  intro n
  induction n with
  | zero => simp [hS0]
  | succ n ih =>
      calc
        S (n + 1) ≤ (1 - delta) * S n := hstep n
        _ ≤ (1 - delta) * (1 - delta) ^ n :=
          mul_le_mul_of_nonneg_left ih (sub_nonneg.mpr hdelta1)
        _ = (1 - delta) ^ (n + 1) := by
          rw [pow_succ]
          ring

/-- Division-free scaled partial-sum endpoint.

Nonnegativity of `S` records the intended survival interpretation.  The
algebraic upper inequality itself follows from normalization and the stated
recurrence. At `delta = 0` the scaled inequality is tautological; the positive
hazard corollary below supplies the unscaled uniform truncated-tail bound. -/
theorem survival_upper_bound_and_scaled_partial_sum
    (S : ℕ → ℝ) (delta : ℝ)
    (hS0 : S 0 = 1)
    (hdelta0 : 0 ≤ delta) (hdelta1 : delta ≤ 1)
    (_hS : ∀ n, 0 ≤ S n)
    (hstep : ∀ n, S (n + 1) ≤ (1 - delta) * S n) :
    (∀ n, S n ≤ (1 - delta) ^ n) ∧
      ∀ N, delta * (∑ n ∈ Finset.range N, S n) ≤
        1 - (1 - delta) ^ N := by
  have hpoint : ∀ n, S n ≤ (1 - delta) ^ n :=
    recurrence_le_geometric S delta hS0 hdelta1 hstep
  refine ⟨hpoint, ?_⟩
  intro N
  calc
    delta * (∑ n ∈ Finset.range N, S n) ≤
        delta * (∑ n ∈ Finset.range N, (1 - delta) ^ n) := by
      apply mul_le_mul_of_nonneg_left _ hdelta0
      exact Finset.sum_le_sum fun n _hn ↦ hpoint n
    _ = 1 - (1 - delta) ^ N := by
      simpa [mul_comm] using geom_sum_mul_neg (1 - delta) N

/-- With a strictly positive lower stopping hazard, every truncated survival
sum is at most `1 / delta`.  This is still an analytic finite-tail statement;
an expectation corollary needs the separate probability/measure bridge. -/
theorem truncated_survival_sum_le_inv
    (S : ℕ → ℝ) (delta : ℝ)
    (hS0 : S 0 = 1)
    (hdelta0 : 0 < delta) (hdelta1 : delta ≤ 1)
    (hS : ∀ n, 0 ≤ S n)
    (hstep : ∀ n, S (n + 1) ≤ (1 - delta) * S n) :
    ∀ N, (∑ n ∈ Finset.range N, S n) ≤ 1 / delta := by
  intro N
  have hfinite :=
    (survival_upper_bound_and_scaled_partial_sum S delta hS0 hdelta0.le
      hdelta1 hS hstep).2 N
  have hpower : 0 ≤ (1 - delta) ^ N :=
    pow_nonneg (sub_nonneg.mpr hdelta1) N
  have hmul : delta * (∑ n ∈ Finset.range N, S n) ≤ 1 :=
    hfinite.trans (sub_le_self 1 hpower)
  rw [le_div_iff₀ hdelta0]
  simpa [mul_comm] using hmul

end SurvivalTailExpectation
