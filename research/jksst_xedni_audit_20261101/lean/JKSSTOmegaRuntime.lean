import JKSSTExpectedTrials

/-!
# A literal asymptotic Omega lower bound for the JKSST waiting time

`JKSSTExpectedTrials.inverse_order_hazard_gives_linear_wait` is a
finite-horizon theorem. Here we choose the horizon `N(p) = p / (4*K)` for a
fixed positive numerator `K`. The finite theorem gives expected truncated
waiting time at least `N(p)/2`; elementary division estimates make this at
least `p/(16*K)` for every `p >= 4*K`.

Mathlib does not define a separate `IsBigOmega` predicate in this version, so
we state the standard eventual positive-linear-lower-bound definition
directly. All conditional-hazard hypotheses remain explicit and are imposed
only through the finite horizon, avoiding an all-time recurrence.
-/

open Filter
open scoped BigOperators

namespace JKSSTOmegaRuntime

open JKSSTExpectedTrials

variable {Omega : Type*} [Fintype Omega]

/-- The horizon used to turn a `K/p` hazard into a linear waiting-time bound. -/
def linearHorizon (K p : ℕ) : ℕ :=
  p / (4 * K)

/-- Weighted waiting time truncated at the certified horizon. This is an
actual expectation when `mu p` is a probability mass function. -/
noncomputable def truncatedRuntime
    (mu : ℕ → Omega → ℝ) (tau : ℕ → Omega → ℕ)
    (K p : ℕ) : ℝ :=
  finiteExpectation (mu p)
    (fun omega ↦ ((min (tau p omega) (linearHorizon K p) : ℕ) : ℝ))

/-- Explicit eventual-linear-lower-bound formulation of `f = Omega(p)`. -/
def IsOmegaLinear (f : ℕ → ℝ) : Prop :=
  ∃ c : ℝ, 0 < c ∧ ∀ᶠ p : ℕ in atTop, c * (p : ℝ) ≤ f p

/-- The floor horizon remains a fixed positive fraction of `p` once
`p >= 4*K`: `p/(16K) <= N(p)/2`. -/
theorem linearHorizon_half_lower_bound
    (K p : ℕ) (K_pos : 0 < K) (hp_large : 4 * K ≤ p) :
    (p : ℝ) / (16 * K) ≤ (linearHorizon K p : ℝ) / 2 := by
  let d := 4 * K
  let N := linearHorizon K p
  have d_pos : 0 < d := by
    dsimp [d]
    positivity
  have N_pos : 0 < N := by
    dsimp [N, linearHorizon, d]
    exact Nat.div_pos hp_large (by positivity)
  have d_le_dN : d ≤ d * N := by
    calc
      d = d * 1 := by simp
      _ ≤ d * N := Nat.mul_le_mul_left d N_pos
  have rem_lt : p % d < d := Nat.mod_lt p d_pos
  have rem_le_dN : p % d ≤ d * N :=
    (Nat.le_of_lt rem_lt).trans d_le_dN
  have decompose : d * N + p % d = p := by
    simpa [d, N, linearHorizon, Nat.mul_comm] using Nat.div_add_mod p d
  have p_le : p ≤ 2 * d * N := by
    calc
      p = d * N + p % d := decompose.symm
      _ ≤ d * N + d * N := Nat.add_le_add_left rem_le_dN _
      _ = 2 * d * N := by ring
  have p_le_expanded : p ≤ 8 * K * linearHorizon K p := by
    calc
      p ≤ 2 * d * N := p_le
      _ = 8 * K * linearHorizon K p := by
        change 2 * (4 * K) * linearHorizon K p =
          8 * K * linearHorizon K p
        ring
  have p_le_real : (p : ℝ) ≤
      (8 : ℝ) * K * linearHorizon K p := by
    exact_mod_cast p_le_expanded
  have K_real_pos : (0 : ℝ) < K := by exact_mod_cast K_pos
  calc
    (p : ℝ) / (16 * K) ≤
        ((8 : ℝ) * K * linearHorizon K p) / (16 * K) := by
      exact div_le_div_of_nonneg_right p_le_real (by positivity)
    _ = (linearHorizon K p : ℝ) / 2 := by
      field_simp
      ring

/-- Pointwise linear lower bound obtained from the finite-horizon hazard
theorem. -/
theorem runtime_ge_linear
    (mu : ℕ → Omega → ℝ) (tau : ℕ → Omega → ℕ)
    (K p : ℕ) (K_pos : 0 < K) (hp_large : 4 * K ≤ p)
    (h_initial : 1 ≤ survivalMass (mu p) (tau p) 0)
    (h_step : ∀ n, n < linearHorizon K p →
      (1 - (K : ℝ) / (p : ℝ)) * survivalMass (mu p) (tau p) n ≤
        survivalMass (mu p) (tau p) (n + 1)) :
    (p : ℝ) / (16 * K) ≤ truncatedRuntime mu tau K p := by
  have hp : 0 < p := lt_of_lt_of_le (by positivity : 0 < 4 * K) hp_large
  have hKp : K ≤ p := by omega
  have hbudget : 2 * K * linearHorizon K p ≤ p := by
    have hfull : 4 * K * linearHorizon K p ≤ p := by
      exact Nat.mul_div_le p (4 * K)
    calc
      2 * K * linearHorizon K p ≤ 4 * K * linearHorizon K p := by
        gcongr
        omega
      _ ≤ p := hfull
  calc
    (p : ℝ) / (16 * K) ≤ (linearHorizon K p : ℝ) / 2 :=
      linearHorizon_half_lower_bound K p K_pos hp_large
    _ ≤ truncatedRuntime mu tau K p := by
      exact inverse_order_hazard_gives_linear_wait
        (mu p) (tau p) p K (linearHorizon K p)
        hp hKp hbudget h_initial h_step

/-- The algebraic recurrence theorem. It intentionally does not interpret the
weights as probabilities; `inverse_order_hazard_isOmega` below supplies that
interface. -/
theorem inverse_order_recurrence_isOmega
    (mu : ℕ → Omega → ℝ) (tau : ℕ → Omega → ℕ)
    (K : ℕ) (K_pos : 0 < K)
    (h_initial : ∀ p, 4 * K ≤ p →
      1 ≤ survivalMass (mu p) (tau p) 0)
    (h_step : ∀ p, 4 * K ≤ p → ∀ n, n < linearHorizon K p →
      (1 - (K : ℝ) / (p : ℝ)) * survivalMass (mu p) (tau p) n ≤
        survivalMass (mu p) (tau p) (n + 1)) :
    IsOmegaLinear (truncatedRuntime mu tau K) := by
  refine ⟨1 / (16 * K), by positivity, ?_⟩
  filter_upwards [eventually_ge_atTop (4 * K)] with p hp
  simpa [div_eq_mul_inv, mul_comm, mul_left_comm, mul_assoc] using
    runtime_ge_linear mu tau K p K_pos hp (h_initial p hp) (h_step p hp)

/-- For fixed positive `K`, a uniform finite-horizon conditional-hazard bound
of `K/p` forces the truncated expected runtime to be `Omega(p)`. The weights
are explicitly nonnegative and normalized, and the waiting time is positive,
so `truncatedRuntime` has its claimed probabilistic interpretation. The
history-uniform recurrence remains a hypothesis. -/
theorem inverse_order_hazard_isOmega
    (mu : ℕ → Omega → ℝ) (tau : ℕ → Omega → ℕ)
    (K : ℕ) (K_pos : 0 < K)
    (h_nonneg : ∀ p omega, 0 ≤ mu p omega)
    (h_total : ∀ p, ∑ omega, mu p omega = 1)
    (h_positive : ∀ p omega, 0 < tau p omega)
    (h_step : ∀ p, 4 * K ≤ p → ∀ n, n < linearHorizon K p →
      (1 - (K : ℝ) / (p : ℝ)) * survivalMass (mu p) (tau p) n ≤
        survivalMass (mu p) (tau p) (n + 1)) :
    IsOmegaLinear (truncatedRuntime mu tau K) := by
  have _h_weights_nonnegative := h_nonneg
  apply inverse_order_recurrence_isOmega mu tau K K_pos
  · intro p _hp
    rw [survivalMass_zero_eq_one (mu p) (tau p) (h_total p) (h_positive p)]
  · exact h_step

end JKSSTOmegaRuntime
