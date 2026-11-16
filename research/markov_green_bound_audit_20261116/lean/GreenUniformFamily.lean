import GreenBound

namespace MarkovGreenBoundAudit
noncomputable section
open scoped BigOperators
open Filter Topology Asymptotics
open MarkovBirthdayRoundingAudit

variable (p : ℕ) [NeZero p]

theorem uniform_doubly_stochastic : transitionMatrix p ∈ doublyStochastic ℝ (ZMod p) := by
  exact mem_doublyStochastic_iff_sum.mpr
    ⟨fun x y => (transition_positive p x y).le,
      transition_row_sum p, transition_column_sum p⟩

theorem uniform_green (T : ℕ) (x y : ZMod p) :
    green (transitionMatrix p) T x y = (if x=y then 1 else 0)+(T : ℝ)/p := by
  induction T with
  | zero => simp [green, partialGreen, Matrix.one_apply]
  | succ T ih =>
    change partialGreen (transitionMatrix p) (T+1+1) x y = _
    rw [partialGreen, Finset.sum_range_succ]
    change green (transitionMatrix p) T x y + (transitionMatrix p^(T+1)) x y = _
    rw [ih, transition_power_succ, transition_entry]
    push_cast
    ring

theorem uniform_row_energy (T : ℕ) (x : ZMod p) :
    rowEnergy (transitionMatrix p) T x = 1+((T : ℝ)^2+2*T)/p := by
  have he (y : ZMod p) : green (transitionMatrix p) T x y^2 =
      (if y=x then 1+2*(T : ℝ)/p else 0)+((T : ℝ)/p)^2 := by
    rw [uniform_green]
    by_cases h : x=y
    · subst y
      simp only [ite_true]
      ring
    · simp only [ite_eq_right h, ite_eq_right (Ne.symm h), zero_add]
  unfold rowEnergy
  rw [Finset.sum_congr rfl (fun y _ => he y), Finset.sum_add_distrib]
  simp only [Finset.sum_ite_eq', Finset.mem_univ, ite_true, Finset.sum_const,
    Finset.card_univ, ZMod.card, nsmul_eq_mul]
  field_simp [ne_of_gt (modulus_positive p)]
  ring

theorem uniform_green_symmetric (T : ℕ) (x y : ZMod p) :
    green (transitionMatrix p) T x y = green (transitionMatrix p) T y x := by
  rw [uniform_green, uniform_green]
  simp only [eq_comm]

theorem uniform_column_energy (T : ℕ) (x : ZMod p) :
    columnEnergy (transitionMatrix p) T x = 1+((T : ℝ)^2+2*T)/p := by
  unfold columnEnergy
  simp_rw [uniform_green_symmetric p T _ x]
  exact uniform_row_energy p T x

theorem uniform_row_maximum (T : ℕ) :
    rowMaximum (transitionMatrix p) T = 1+((T : ℝ)^2+2*T)/p := by
  simp only [rowMaximum, uniform_row_energy, Finset.sup'_const]

theorem uniform_column_maximum (T : ℕ) :
    columnMaximum (transitionMatrix p) T = 1+((T : ℝ)^2+2*T)/p := by
  simp only [columnMaximum, uniform_column_energy, Finset.sup'_const]

/-- Prime state count is selected from an actual existence theorem. -/
def stateCount (t : ℕ) : ℕ := (Nat.exists_infinite_primes (3^(t+1))).choose

theorem stateCount_prime (t : ℕ) : Nat.Prime (stateCount t) :=
  (Nat.exists_infinite_primes (3^(t+1))).choose_spec.2

instance stateCount_nonzero (t : ℕ) : NeZero (stateCount t) :=
  ⟨(stateCount_prime t).ne_zero⟩

theorem stateCount_bound (t : ℕ) : 3^t ≤ stateCount t := by
  have h := (Nat.exists_infinite_primes (3^(t+1))).choose_spec.1
  change 3^(t+1) ≤ stateCount t at h
  rw [pow_succ] at h
  omega

theorem stateCount_real_bound (t : ℕ) : (3 : ℝ)^t ≤ stateCount t := by
  exact_mod_cast stateCount_bound t

def familyError (t : ℕ) : ℝ := 1/(stateCount t : ℝ)
def familyRow (t : ℕ) : ℝ := rowMaximum (transitionMatrix (stateCount t)) t
def familyColumn (t : ℕ) : ℝ := columnMaximum (transitionMatrix (stateCount t)) t

theorem polynomial_ratio_limit (k : ℕ) :
    Tendsto (fun t : ℕ => (t : ℝ)^k/(stateCount t : ℝ)) atTop (𝓝 0) := by
  apply squeeze_zero (fun t => div_nonneg (pow_nonneg (Nat.cast_nonneg t) k)
    (modulus_positive (stateCount t)).le)
    (fun t => div_le_div_of_nonneg_left (pow_nonneg (Nat.cast_nonneg t) k)
      (pow_pos (by norm_num : (0 : ℝ)<3) t) (stateCount_real_bound t))
  exact tendsto_pow_const_div_const_pow_of_one_lt k (by norm_num)

theorem family_error_littleO : familyError =o[atTop] (fun t : ℕ => 1/(t : ℝ)^2) := by
  apply (isLittleO_iff_tendsto' _).mpr
  · have he : (fun t : ℕ => familyError t/(1/(t : ℝ)^2)) =
        (fun t : ℕ => (t : ℝ)^2/(stateCount t : ℝ)) := by
      funext t
      simp [familyError, div_eq_mul_inv, mul_comm]
    rw [he]
    exact polynomial_ratio_limit 2
  · filter_upwards [eventually_ge_atTop 1] with t ht
    have hn : (t : ℝ) ≠ 0 := by exact_mod_cast (by omega : t ≠ 0)
    simp [hn]

/-- Envelope holds for all powers, not merely j at most t. -/
theorem family_envelope (t j : ℕ) (x y : ZMod (stateCount t)) :
    (transitionMatrix (stateCount t)^j) x y ≤ familyError t+(1/2 : ℝ)^j := by
  cases j with
  | zero =>
    simp only [pow_zero, Matrix.one_apply]
    have h : 0 ≤ familyError t := (one_div_pos.mpr (modulus_positive (stateCount t))).le
    split_ifs <;> linarith
  | succ j =>
    rw [transition_power_succ, transition_entry]
    exact le_add_of_nonneg_right (pow_nonneg (by norm_num) _)

theorem family_row_limit : Tendsto familyRow atTop (𝓝 1) := by
  have h := (polynomial_ratio_limit 2).add ((polynomial_ratio_limit 1).const_mul 2)
  have he : familyRow = (fun t : ℕ => 1+
      ((t : ℝ)^2/(stateCount t : ℝ)+2*((t : ℝ)^1/(stateCount t : ℝ)))) := by
    funext t
    rw [familyRow, uniform_row_maximum]
    simp only [pow_one]
    ring
  rw [he]
  simpa using tendsto_const_nhds.add h

theorem family_column_limit : Tendsto familyColumn atTop (𝓝 1) := by
  have he : familyColumn = familyRow := by
    funext t
    simp only [familyColumn, familyRow, uniform_column_maximum, uniform_row_maximum]
  rw [he]
  exact family_row_limit

theorem literal_asymptotic_equality_false :
    ¬ Tendsto familyRow atTop (𝓝 ((2 : ℝ)/(1-1/2)^2)) := by
  intro h
  have he := tendsto_nhds_unique family_row_limit h
  norm_num at he

/-- The printed equality cannot hold even with an actual vanishing remainder. -/
theorem literal_remainder_equality_false :
    ¬ ∃ e : ℕ → ℝ, Tendsto e atTop (𝓝 0) ∧
      ∀ᶠ t in atTop, familyRow t = (2+e t)/(1-1/2 : ℝ)^2 := by
  rintro ⟨e, he, heq⟩
  apply literal_asymptotic_equality_false
  have hc : Tendsto (fun _ : ℕ => (2 : ℝ)) atTop (𝓝 2) := tendsto_const_nhds
  have hl := (hc.add he).div_const ((1-1/2 : ℝ)^2)
  have hh : Tendsto (fun t => (2+e t)/(1-1/2 : ℝ)^2)
      atTop (𝓝 ((2 : ℝ)/(1-1/2)^2)) := by simpa only [add_zero] using hl
  apply hh.congr'
  filter_upwards [heq] with t ht
  exact ht.symm

/-- Every member also satisfies the qualified source finite inequality. -/
theorem family_qualified_control (t T : ℕ) :
    rowMaximum (transitionMatrix (stateCount t)) T ≤ sourceRHS (transitionMatrix (stateCount t)) T ∧
    columnMaximum (transitionMatrix (stateCount t)) T ≤ sourceRHS (transitionMatrix (stateCount t)) T :=
  qualified_green_bound _ (uniform_doubly_stochastic _) T

theorem uniform_zero_time_control (p : ℕ) [NeZero p] :
    rowMaximum (transitionMatrix p) 0 = 1 ∧ columnMaximum (transitionMatrix p) 0 = 1 := by
  simp only [uniform_row_maximum, uniform_column_maximum, Nat.cast_zero, zero_pow (by decide : 2≠0),
    mul_zero, zero_div, add_zero, and_self]

theorem uniform_singleton_control (T : ℕ) :
    rowMaximum (transitionMatrix 1) T = ((T : ℝ)+1)^2 := by
  rw [uniform_row_maximum]
  norm_num
  ring

#print axioms uniform_row_maximum
#print axioms family_error_littleO
#print axioms family_envelope
#print axioms family_row_limit
#print axioms literal_asymptotic_equality_false
end
end MarkovGreenBoundAudit
