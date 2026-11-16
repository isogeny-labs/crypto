import Mathlib

/-!
# The xedni waiting time needs per-trial marginals, not a hazard recurrence

Corpus target: `ASM-XEDNI-HAZARD-RECURRENCE` (ECDLP claim corpus,
WEAKNESS_TRIAGE W7) and `GAP-PA-ONE-SHOT-REPEAT`.

The JKSST audit (`research/jksst_xedni_audit_20261101`,
`JKSSTExpectedTrials.truncated_expectation_ge_half_horizon`) derives a linear
waiting time from a *conditional-hazard recurrence*
`(1 - ε) * s n ≤ s (n + 1)` on survival masses, and lists "derive the
recurrence for repeated, adaptive trials" as open. This file shows the
recurrence is not needed and is strictly stronger than what the conclusion uses:

1. `survival_ge_of_marginals`: if every trial `t ≤ N` succeeds with marginal
   mass at most `ε` (correlated or adaptive trials allowed), then the survival
   mass after `n ≤ N` trials is at least `1 - n ε` (union bound).
2. `truncated_expectation_ge_half_horizon_of_marginals`: hence, when `N ε ≤ 1/2`,
   `E[min τ N] ≥ N / 2`: the same conclusion as the JKSST lemma, from marginals.
3. `marginals_do_not_give_hazard`: a three-point model in which every trial's
   marginal success mass is `1/3`, yet the recurrence with `ε = 1/3` fails.

So the open obligation for xedni becomes a per-trial *marginal* bound for the
adaptive trial actually run (a one-shot statement about each trial's
distribution), not a history-uniform hazard bound.
-/

open scoped BigOperators

namespace HazardSeparation

variable {Omega : Type*} [Fintype Omega]

/-- Mass of `n < τ` (same definition as `JKSSTExpectedTrials.survivalMass`). -/
noncomputable def survivalMass (mu : Omega → ℝ) (tau : Omega → ℕ) (n : ℕ) : ℝ :=
  ∑ omega, if n < tau omega then mu omega else 0

/-- Mass of the event "trial `t` succeeds". -/
noncomputable def eventMass (mu : Omega → ℝ) (S : ℕ → Omega → Prop)
    [∀ t, DecidablePred (S t)] (t : ℕ) : ℝ :=
  ∑ omega, if S t omega then mu omega else 0

/-- Finite expectation, as in `JKSSTExpectedTrials.finiteExpectation`. -/
noncomputable def finiteExpectation (mu : Omega → ℝ) (X : Omega → ℝ) : ℝ :=
  ∑ omega, mu omega * X omega

private theorem cast_min_eq_sum_indicator (t N : ℕ) :
    ((min t N : ℕ) : ℝ) = ∑ n ∈ Finset.range N, if n < t then (1 : ℝ) else 0 := by
  induction N with
  | zero => simp
  | succ N ih =>
      rw [Finset.sum_range_succ]
      by_cases h : N < t
      · have h1 : min t (N + 1) = N + 1 := by omega
        have h2 : min t N = N := by omega
        rw [h1, Nat.cast_succ, ← show ((min t N : ℕ) : ℝ) = N by rw [h2], ih]
        simp [h]
      · have h1 : min t (N + 1) = min t N := by omega
        rw [h1, ih]
        simp [h]

/-- Tail-sum formula (re-proved here; identical to the JKSST statement). -/
theorem truncated_expectation_eq_survival_sum
    (mu : Omega → ℝ) (tau : Omega → ℕ) (N : ℕ) :
    finiteExpectation mu (fun omega ↦ ((min (tau omega) N : ℕ) : ℝ)) =
      ∑ n ∈ Finset.range N, survivalMass mu tau n := by
  classical
  simp only [finiteExpectation, survivalMass, cast_min_eq_sum_indicator, Finset.mul_sum]
  rw [Finset.sum_comm]
  apply Finset.sum_congr rfl
  intro n _
  apply Finset.sum_congr rfl
  intro omega _
  by_cases h : n < tau omega <;> simp [h]

/-- Union bound: per-trial marginal success masses bound the survival mass from
below. `h_cover` says the waiting time is the index of a successful trial. -/
theorem survival_ge_of_marginals
    (mu : Omega → ℝ) (tau : Omega → ℕ) (S : ℕ → Omega → Prop)
    [∀ t, DecidablePred (S t)] (N : ℕ) (epsilon : ℝ)
    (h_nonneg : ∀ omega, 0 ≤ mu omega)
    (h_total : ∑ omega, mu omega = 1)
    (h_cover : ∀ omega n, tau omega ≤ n → ∃ t ∈ Finset.Icc 1 n, S t omega)
    (h_marg : ∀ t ∈ Finset.Icc 1 N, eventMass mu S t ≤ epsilon) :
    ∀ n, n ≤ N → 1 - n * epsilon ≤ survivalMass mu tau n := by
  classical
  intro n hn
  -- survival = total - mass{τ ≤ n}
  have hsplit : survivalMass mu tau n
      = 1 - ∑ omega, if tau omega ≤ n then mu omega else 0 := by
    rw [← h_total, survivalMass, ← Finset.sum_sub_distrib]
    apply Finset.sum_congr rfl
    intro omega _
    by_cases h : n < tau omega
    · have : ¬ tau omega ≤ n := by omega
      simp [h, this]
    · have : tau omega ≤ n := by omega
      simp [h, this]
  -- mass{τ ≤ n} ≤ Σ_t mass(S t)
  have hpoint : ∀ omega, (if tau omega ≤ n then mu omega else 0)
      ≤ ∑ t ∈ Finset.Icc 1 n, (if S t omega then mu omega else 0) := by
    intro omega
    by_cases h : tau omega ≤ n
    · obtain ⟨t, ht, hS⟩ := h_cover omega n h
      simp only [h, ↓reduceIte]
      calc mu omega = (if S t omega then mu omega else 0) := by simp [hS]
        _ ≤ ∑ t ∈ Finset.Icc 1 n, (if S t omega then mu omega else 0) :=
          Finset.single_le_sum (f := fun t => if S t omega then mu omega else 0)
            (fun t _ => by split_ifs <;> simp [h_nonneg]) ht
    · simp only [h, ↓reduceIte]
      exact Finset.sum_nonneg (fun t _ => by split_ifs <;> simp [h_nonneg])
  have hunion : (∑ omega, if tau omega ≤ n then mu omega else 0) ≤ n * epsilon := by
    calc (∑ omega, if tau omega ≤ n then mu omega else 0)
        ≤ ∑ omega, ∑ t ∈ Finset.Icc 1 n, (if S t omega then mu omega else 0) :=
          Finset.sum_le_sum (fun omega _ => hpoint omega)
      _ = ∑ t ∈ Finset.Icc 1 n, eventMass mu S t := by
          rw [Finset.sum_comm]; rfl
      _ ≤ ∑ _t ∈ Finset.Icc 1 n, epsilon :=
          Finset.sum_le_sum (fun t ht => h_marg t (by
            simp only [Finset.mem_Icc] at ht ⊢; omega))
      _ = n * epsilon := by simp
  rw [hsplit]
  linarith

/-- Linear waiting time from marginals alone: with `N ε ≤ 1/2`,
`E[min τ N] ≥ N / 2`. Same conclusion as
`JKSSTExpectedTrials.truncated_expectation_ge_half_horizon`, without the
conditional-hazard recurrence. -/
theorem truncated_expectation_ge_half_horizon_of_marginals
    (mu : Omega → ℝ) (tau : Omega → ℕ) (S : ℕ → Omega → Prop)
    [∀ t, DecidablePred (S t)] (N : ℕ) (epsilon : ℝ)
    (h_eps : 0 ≤ epsilon)
    (h_budget : (N : ℝ) * epsilon ≤ 1 / 2)
    (h_nonneg : ∀ omega, 0 ≤ mu omega)
    (h_total : ∑ omega, mu omega = 1)
    (h_cover : ∀ omega n, tau omega ≤ n → ∃ t ∈ Finset.Icc 1 n, S t omega)
    (h_marg : ∀ t ∈ Finset.Icc 1 N, eventMass mu S t ≤ epsilon) :
    (N : ℝ) / 2 ≤ finiteExpectation mu (fun omega ↦ ((min (tau omega) N : ℕ) : ℝ)) := by
  rw [truncated_expectation_eq_survival_sum]
  have hterm : ∀ n ∈ Finset.range N, (1 / 2 : ℝ) ≤ survivalMass mu tau n := by
    intro n hn
    have hnN : n ≤ N := Nat.le_of_lt (Finset.mem_range.mp hn)
    have h1 := survival_ge_of_marginals mu tau S N epsilon h_nonneg h_total h_cover h_marg n hnN
    have h2 : (n : ℝ) * epsilon ≤ N * epsilon :=
      mul_le_mul_of_nonneg_right (by exact_mod_cast hnN) h_eps
    linarith
  calc (N : ℝ) / 2 = ∑ _n ∈ Finset.range N, (1 / 2 : ℝ) := by
        simp; ring
    _ ≤ ∑ n ∈ Finset.range N, survivalMass mu tau n := Finset.sum_le_sum hterm

/-! ## Separation: marginals do not give the hazard recurrence -/

/-- Three equally likely outcomes; outcome `k` succeeds exactly at trial `k+1`. -/
noncomputable def muU : Fin 3 → ℝ := fun _ => 1 / 3
def tauU : Fin 3 → ℕ := fun k => k.val + 1
def SU : ℕ → Fin 3 → Prop := fun t k => k.val + 1 = t
instance : ∀ t, DecidablePred (SU t) := fun t k => inferInstanceAs (Decidable (k.val + 1 = t))

theorem cover_U : ∀ omega n, tauU omega ≤ n → ∃ t ∈ Finset.Icc 1 n, SU t omega := by
  intro omega n h
  exact ⟨omega.val + 1, by simp only [Finset.mem_Icc]; constructor <;> simp_all [tauU], rfl⟩

/-- Every trial's marginal success mass is at most `1/3`. -/
theorem marginals_U : ∀ t, eventMass muU SU t ≤ 1 / 3 := by
  intro t
  simp only [eventMass, Fin.sum_univ_three, muU, SU]
  rcases Nat.lt_or_ge t 4 with h | h
  · interval_cases t <;> norm_num
  · have h1 : ¬ (0 + 1 = t) := by omega
    have h2 : ¬ (1 + 1 = t) := by omega
    have h3 : ¬ (2 + 1 = t) := by omega
    simp [h1, h2, h3]

/-- The JKSST-style recurrence `(1 - ε) s(n) ≤ s(n+1)` with `ε = 1/3` fails at
`n = 1` in the model whose marginals are all at most `1/3`. -/
theorem marginals_do_not_give_hazard :
    ¬ (∀ n, n < 3 →
      (1 - (1 / 3 : ℝ)) * survivalMass muU tauU n ≤ survivalMass muU tauU (n + 1)) := by
  intro h
  have h1 := h 1 (by norm_num)
  simp only [survivalMass, Fin.sum_univ_three, muU, tauU] at h1
  norm_num at h1

/-- In the same model the linear wait still holds: `E[min τ 3] = 2 ≥ 3/2`. -/
theorem wait_U : finiteExpectation muU (fun omega ↦ ((min (tauU omega) 3 : ℕ) : ℝ)) = 2 := by
  simp only [finiteExpectation, Fin.sum_univ_three, muU, tauU]
  norm_num

end HazardSeparation
