import GreenCore

namespace MarkovGreenBoundAudit
noncomputable section
open scoped BigOperators
variable {S : Type*} [Fintype S] [DecidableEq S] [Nonempty S]

/-- Row normalization gives a sharper finite bound without stationarity. -/
theorem partial_energy_bound (P : Matrix S S ℝ) (hP : P ∈ Matrix.rowStochastic ℝ S)
    (n : ℕ) (x : S) :
    (∑ y, partialGreen P n x y ^ 2) ≤
      ∑ j ∈ Finset.range n, (2*(j : ℝ)+1)*maxEntry P j := by
  induction n with
  | zero => simp [partialGreen]
  | succ n ih =>
    have hc : (∑ y, partialGreen P n x y * (P^n) x y) ≤
        maxEntry P n * (n : ℝ) := by
      calc
        _ ≤ ∑ y, partialGreen P n x y * maxEntry P n :=
          Finset.sum_le_sum (fun y _ => mul_le_mul_of_nonneg_left
            (entry_le_max P n x y) (partial_nonnegative P hP n x y))
        _ = _ := by rw [← Finset.sum_mul, partial_row_sum P hP]; ring
    have hs : (∑ y, ((P^n) x y)^2) ≤ maxEntry P n := by
      calc
        _ ≤ ∑ y, (P^n) x y * maxEntry P n :=
          Finset.sum_le_sum (fun y _ => by
            rw [pow_two]
            exact mul_le_mul_of_nonneg_left (entry_le_max P n x y)
              (power_nonnegative P hP n x y))
        _ = _ := by rw [← Finset.sum_mul, power_row_sum P hP, one_mul]
    have he : (∑ y, partialGreen P (n+1) x y ^ 2) =
        (∑ y, partialGreen P n x y ^ 2) +
        2*(∑ y, partialGreen P n x y * (P^n) x y) + ∑ y, ((P^n) x y)^2 := by
      simp only [partialGreen, Finset.sum_range_succ]
      simp_rw [add_sq, mul_assoc]
      rw [Finset.sum_add_distrib, Finset.sum_add_distrib, ← Finset.mul_sum]
    rw [he, Finset.sum_range_succ]
    nlinarith

theorem row_energy_bound (P : Matrix S S ℝ) (hP : P ∈ Matrix.rowStochastic ℝ S)
    (T : ℕ) (x : S) : rowEnergy P T x ≤ sourceRHS P T := by
  apply (partial_energy_bound P hP (T+1) x).trans
  unfold sourceRHS
  rw [Finset.mul_sum]
  apply Finset.sum_le_sum
  intro j _
  have hn := max_nonnegative P hP j
  nlinarith

theorem row_maximum_bound (P : Matrix S S ℝ) (hP : P ∈ Matrix.rowStochastic ℝ S)
    (T : ℕ) : rowMaximum P T ≤ sourceRHS P T := by
  exact Finset.sup'_le _ _ (fun x _ => row_energy_bound P hP T x)

theorem column_energy_bound (P : Matrix S S ℝ) (hP : P ∈ Matrix.colStochastic ℝ S)
    (T : ℕ) (x : S) : columnEnergy P T x ≤ sourceRHS P T := by
  have ht : P.transpose ∈ Matrix.rowStochastic ℝ S :=
    Matrix.transpose_mem_rowStochastic_iff_mem_colStochastic.mpr hP
  have h := row_energy_bound P.transpose ht T x
  simpa only [rowEnergy, columnEnergy, green_transpose, sourceRHS_transpose] using h

theorem column_maximum_bound (P : Matrix S S ℝ) (hP : P ∈ Matrix.colStochastic ℝ S)
    (T : ℕ) : columnMaximum P T ≤ sourceRHS P T :=
  Finset.sup'_le _ _ (fun x _ => column_energy_bound P hP T x)

/-- Source Lemma3.3, with its uniform-stationarity normalization explicit. -/
theorem qualified_green_bound (P : Matrix S S ℝ) (hP : P ∈ doublyStochastic ℝ S)
    (T : ℕ) : rowMaximum P T ≤ sourceRHS P T ∧ columnMaximum P T ≤ sourceRHS P T := by
  obtain ⟨hr, hc⟩ := mem_doublyStochastic_iff_mem_rowStochastic_and_mem_colStochastic.mp hP
  exact ⟨row_maximum_bound P hr T, column_maximum_bound P hc T⟩

/-- The uniform law is an actual normalized probability weight. -/
def uniformMass : S → ℝ := fun _ => 1/(Fintype.card S : ℝ)

omit [DecidableEq S] in
theorem uniform_mass_sum : (∑ x : S, uniformMass x) = 1 := by
  have hc : (Fintype.card S : ℝ) ≠ 0 := by exact_mod_cast Fintype.card_ne_zero
  simp only [uniformMass, Finset.sum_const, Finset.card_univ, nsmul_eq_mul]
  exact mul_one_div_cancel hc

/-- Uniform invariance is equivalent to the column condition for an actual stochastic kernel. -/
theorem uniform_stationary_iff_column (P : Matrix S S ℝ)
    (hP : P ∈ Matrix.rowStochastic ℝ S) :
    (∀ y : S, (∑ x : S, uniformMass x * P x y) = uniformMass y) ↔
      P ∈ Matrix.colStochastic ℝ S := by
  constructor
  · intro h
    apply Matrix.mem_colStochastic_iff_sum.mpr
    refine ⟨fun x y => Matrix.nonneg_of_mem_rowStochastic hP, ?_⟩
    intro y
    have hy := h y
    unfold uniformMass at hy
    rw [← Finset.mul_sum] at hy
    have hc : (1/(Fintype.card S : ℝ)) ≠ 0 := by
      exact one_div_ne_zero (by exact_mod_cast Fintype.card_ne_zero)
    apply mul_left_cancel₀ hc
    simpa only [mul_one] using hy
  · intro h y
    unfold uniformMass
    rw [← Finset.mul_sum, Matrix.sum_col_of_mem_colStochastic h y, mul_one]

/-- Source hypotheses expressed by the actual uniform stationary law. -/
theorem uniform_stationary_green_bound (P : Matrix S S ℝ)
    (hP : P ∈ Matrix.rowStochastic ℝ S)
    (hs : ∀ y : S, (∑ x : S, uniformMass x * P x y) = uniformMass y)
    (T : ℕ) : rowMaximum P T ≤ sourceRHS P T ∧ columnMaximum P T ≤ sourceRHS P T :=
  ⟨row_maximum_bound P hP T,
    column_maximum_bound P ((uniform_stationary_iff_column P hP).mp hs) T⟩

#print axioms partial_energy_bound
#print axioms qualified_green_bound
end
end MarkovGreenBoundAudit
