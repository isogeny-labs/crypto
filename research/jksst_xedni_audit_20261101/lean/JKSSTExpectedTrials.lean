import Mathlib

/-!
# A rigorous finite-horizon replacement for the JKSST runtime heuristic

The paper's phrase "at least `O(p)` repetitions" has the asymptotic direction
backwards and does not state the probabilistic assumptions.  This file works
with a finite weighted sample space and a natural-valued waiting time; the
normalization hypothesis is exposed separately.

For a waiting time `tau`, the expectation of `min tau N` is exactly the sum of
the first `N` survival probabilities.  A uniform conditional hazard bound
`epsilon` implies that those survival probabilities dominate the geometric
sequence `(1-epsilon)^n`; independence is not required.
-/

open scoped BigOperators

namespace JKSSTExpectedTrials

variable {Omega : Type*} [Fintype Omega]

/-- Finite expectation of a real-valued random variable.  Normalization and
nonnegativity of `mu` are deliberately kept as separate hypotheses. -/
noncomputable def finiteExpectation (mu : Omega → ℝ) (X : Omega → ℝ) : ℝ :=
  ∑ omega, mu omega * X omega

/-- The (possibly unnormalized) mass of the event `n < tau`. -/
noncomputable def survivalMass (mu : Omega → ℝ) (tau : Omega → ℕ)
    (n : ℕ) : ℝ :=
  ∑ omega, if n < tau omega then mu omega else 0

private theorem cast_min_eq_sum_indicator (t N : ℕ) :
    ((min t N : ℕ) : ℝ) =
      ∑ n ∈ Finset.range N, if n < t then (1 : ℝ) else 0 := by
  induction N with
  | zero => simp
  | succ N ih =>
      rw [Finset.sum_range_succ]
      by_cases h : N < t
      · have hmin_succ : min t (N + 1) = N + 1 := by omega
        have hmin : min t N = N := by omega
        have ihN : (N : ℝ) =
            ∑ n ∈ Finset.range N, if n < t then (1 : ℝ) else 0 := by
          simpa [hmin] using ih
        rw [hmin_succ, ← ihN]
        simp [h]
      · have hmin_succ : min t (N + 1) = min t N := by omega
        rw [hmin_succ, ih]
        simp [h]

/-- The finite tail-sum formula: `E[min(tau,N)]` is exactly the sum of the
survival probabilities at times `0,...,N-1`. -/
theorem truncated_expectation_eq_survival_sum
    (mu : Omega → ℝ) (tau : Omega → ℕ) (N : ℕ) :
    finiteExpectation mu (fun omega ↦ ((min (tau omega) N : ℕ) : ℝ)) =
      ∑ n ∈ Finset.range N, survivalMass mu tau n := by
  classical
  simp only [finiteExpectation, survivalMass, cast_min_eq_sum_indicator]
  calc
    ∑ omega, mu omega *
          (∑ n ∈ Finset.range N, if n < tau omega then (1 : ℝ) else 0) =
        ∑ omega, ∑ n ∈ Finset.range N,
          mu omega * (if n < tau omega then (1 : ℝ) else 0) := by
            apply Finset.sum_congr rfl
            intro omega homega
            rw [Finset.mul_sum]
    _ = ∑ n ∈ Finset.range N, ∑ omega,
          mu omega * (if n < tau omega then (1 : ℝ) else 0) := by
            rw [Finset.sum_comm]
    _ = ∑ n ∈ Finset.range N, ∑ omega,
          if n < tau omega then mu omega else 0 := by
            apply Finset.sum_congr rfl
            intro n hn
            apply Finset.sum_congr rfl
            intro omega homega
            by_cases h : n < tau omega <;> simp [h]

/-- An upper bound `epsilon` on conditional success probability is encoded by
the recurrence `s(n+1) >= (1-epsilon) s(n)` for survival probabilities.  This
lemma iterates that recurrence through a declared finite horizon. -/
theorem survival_geometric_lower_bound
    (epsilon : ℝ) (s : ℕ → ℝ) (N : ℕ)
    (h_nonneg : 0 ≤ 1 - epsilon)
    (h_initial : 1 ≤ s 0)
    (h_step : ∀ n, n < N → (1 - epsilon) * s n ≤ s (n + 1)) :
    ∀ n, n ≤ N → (1 - epsilon) ^ n ≤ s n := by
  intro n hnN
  induction n with
  | zero => simpa using h_initial
  | succ n ih =>
      calc
        (1 - epsilon) ^ (n + 1) = (1 - epsilon) * (1 - epsilon) ^ n := by
          rw [pow_succ]
          ring
        _ ≤ (1 - epsilon) * s n :=
          mul_le_mul_of_nonneg_left (ih (Nat.le_of_succ_le hnN)) h_nonneg
        _ ≤ s (n + 1) := h_step n (Nat.lt_of_succ_le hnN)

/-- Consequently the truncated expected wait is bounded below by an exact
finite geometric sum. -/
theorem truncated_expectation_ge_geometric_sum
    (mu : Omega → ℝ) (tau : Omega → ℕ) (N : ℕ) (epsilon : ℝ)
    (h_nonneg : 0 ≤ 1 - epsilon)
    (h_initial : 1 ≤ survivalMass mu tau 0)
    (h_step : ∀ n, n < N →
      (1 - epsilon) * survivalMass mu tau n ≤ survivalMass mu tau (n + 1)) :
    (∑ n ∈ Finset.range N, (1 - epsilon) ^ n) ≤
      finiteExpectation mu (fun omega ↦ ((min (tau omega) N : ℕ) : ℝ)) := by
  rw [truncated_expectation_eq_survival_sum]
  apply Finset.sum_le_sum
  intro n hn
  exact survival_geometric_lower_bound epsilon (survivalMass mu tau) N
    h_nonneg h_initial h_step n (Nat.le_of_lt (Finset.mem_range.mp hn))

/-- A normalized mass function and a waiting time which is at least one have
initial survival probability exactly one. -/
theorem survivalMass_zero_eq_one
    (mu : Omega → ℝ) (tau : Omega → ℕ)
    (h_total : ∑ omega, mu omega = 1)
    (h_positive : ∀ omega, 0 < tau omega) :
    survivalMass mu tau 0 = 1 := by
  classical
  simpa [survivalMass, h_positive] using h_total

/-- A convenient explicit `Omega(1/epsilon)` form.  If the horizon consumes at
most half of the hazard budget, the truncated expected wait is at least half
the horizon.  This uses no independence assumption. -/
theorem truncated_expectation_ge_half_horizon
    (mu : Omega → ℝ) (tau : Omega → ℕ) (N : ℕ) (epsilon : ℝ)
    (h_epsilon_nonneg : 0 ≤ epsilon)
    (h_one_sub_nonneg : 0 ≤ 1 - epsilon)
    (h_budget : (N : ℝ) * epsilon ≤ 1 / 2)
    (h_initial : 1 ≤ survivalMass mu tau 0)
    (h_step : ∀ n, n < N →
      (1 - epsilon) * survivalMass mu tau n ≤ survivalMass mu tau (n + 1)) :
    (N : ℝ) / 2 ≤
      finiteExpectation mu (fun omega ↦ ((min (tau omega) N : ℕ) : ℝ)) := by
  have hgeom := truncated_expectation_ge_geometric_sum
    mu tau N epsilon h_one_sub_nonneg h_initial h_step
  apply le_trans _ hgeom
  calc
    (N : ℝ) / 2 = ∑ n ∈ Finset.range N, (1 / 2 : ℝ) := by
      simp
      ring
    _ ≤ ∑ n ∈ Finset.range N, (1 - epsilon) ^ n := by
      apply Finset.sum_le_sum
      intro n hn
      have hnN_nat : n ≤ N := Nat.le_of_lt (Finset.mem_range.mp hn)
      have hnN : (n : ℝ) ≤ (N : ℝ) := by exact_mod_cast hnN_nat
      have hmul : (n : ℝ) * epsilon ≤ (N : ℝ) * epsilon :=
        mul_le_mul_of_nonneg_right hnN h_epsilon_nonneg
      have hlinear : (1 / 2 : ℝ) ≤ 1 - (n : ℝ) * epsilon := by
        linarith
      have hbern : 1 - (n : ℝ) * epsilon ≤ (1 - epsilon) ^ n := by
        have hminus : (-2 : ℝ) ≤ -epsilon := by linarith
        simpa [sub_eq_add_neg] using one_add_mul_le_pow hminus n
      exact hlinear.trans hbern

/-- The `/q` (or, after a subgroup-size hypothesis, `/p`) success bound gives
a genuinely linear waiting-time lower bound.  If `epsilon = K/p` and
`2*K*N ≤ p`, the first `N` trials have truncated expected wait at least `N/2`.
For fixed `K`, choosing `N` proportional to `p` is the precise content of an
`Omega(p)` conclusion. -/
theorem inverse_order_hazard_gives_linear_wait
    (mu : Omega → ℝ) (tau : Omega → ℕ) (p K N : ℕ)
    (hp : 0 < p) (hKp : K ≤ p) (h_budget_nat : 2 * K * N ≤ p)
    (h_initial : 1 ≤ survivalMass mu tau 0)
    (h_step : ∀ n, n < N →
      (1 - (K : ℝ) / (p : ℝ)) * survivalMass mu tau n ≤
        survivalMass mu tau (n + 1)) :
    (N : ℝ) / 2 ≤
      finiteExpectation mu (fun omega ↦ ((min (tau omega) N : ℕ) : ℝ)) := by
  apply truncated_expectation_ge_half_horizon
    mu tau N ((K : ℝ) / (p : ℝ))
  · positivity
  · have hp_real : (0 : ℝ) < (p : ℝ) := by positivity
    have hKp_real : (K : ℝ) ≤ (p : ℝ) := by exact_mod_cast hKp
    exact sub_nonneg.mpr ((div_le_one hp_real).2 hKp_real)
  · have hp_real : (0 : ℝ) < (p : ℝ) := by positivity
    have hbudget_real : ((2 * K * N : ℕ) : ℝ) ≤ (p : ℝ) := by
      exact_mod_cast h_budget_nat
    push_cast at hbudget_real
    calc
      (N : ℝ) * ((K : ℝ) / (p : ℝ)) =
          ((2 : ℝ) * K * N) / ((2 : ℝ) * p) := by field_simp
      _ ≤ (p : ℝ) / ((2 : ℝ) * p) :=
        div_le_div_of_nonneg_right hbudget_real (by positivity)
      _ = 1 / 2 := by field_simp
  · exact h_initial
  · exact h_step

/-- Closed form for the finite geometric lower bound, stated without division
so it remains valid at `epsilon = 0`. -/
theorem epsilon_mul_geometric_sum (epsilon : ℝ) (N : ℕ) :
    epsilon * (∑ n ∈ Finset.range N, (1 - epsilon) ^ n) =
      1 - (1 - epsilon) ^ N := by
  rw [mul_comm epsilon, Finset.sum_mul]
  calc
    ∑ n ∈ Finset.range N, (1 - epsilon) ^ n * epsilon =
        ∑ n ∈ Finset.range N,
          ((1 - epsilon) ^ n - (1 - epsilon) ^ (n + 1)) := by
            apply Finset.sum_congr rfl
            intro n hn
            rw [pow_succ]
            ring
    _ = 1 - (1 - epsilon) ^ N := by
      induction N with
      | zero => simp
      | succ N ih =>
          rw [Finset.sum_range_succ, ih]
          ring

end JKSSTExpectedTrials
