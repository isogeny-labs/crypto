import Mathlib

/-!
# An explicit finite packing bound for the JKSST height argument

The proof is the standard disjoint-balls volume argument, generalized from
`Besicovitch.card_le_of_separated`.  It supplies the finite inequality which
cannot be obtained by simply deleting the error term from a lattice-point
asymptotic.
-/

open Metric MeasureTheory

namespace JKSSTPacking

variable {E : Type*} [NormedAddCommGroup E] [NormedSpace ℝ E]
  [FiniteDimensional ℝ E]

/-- If the points of `s` lie in the radius-`R` ball and distinct points are
`δ`-separated, then disjoint radius-`δ/2` balls give this exact finite volume
inequality. -/
theorem card_mul_halfSeparation_pow_le
    (s : Finset E) {R δ : ℝ} (hR : 0 ≤ R) (hδ : 0 < δ)
    (inside : ∀ c ∈ s, ‖c‖ ≤ R)
    (separated : ∀ c ∈ s, ∀ d ∈ s, c ≠ d → δ ≤ ‖c - d‖) :
    (s.card : ℝ) * (δ / 2) ^ Module.finrank ℝ E ≤
      (R + δ / 2) ^ Module.finrank ℝ E := by
  borelize E
  let μ : Measure E := Measure.addHaar
  let ε : ℝ := δ / 2
  let ρ : ℝ := R + ε
  have εpos : 0 < ε := div_pos hδ (by norm_num)
  have ρpos : 0 < ρ := add_pos_of_nonneg_of_pos hR εpos
  set A := ⋃ c ∈ s, ball (c : E) ε with hA
  have disjointBalls : Set.Pairwise (s : Set E)
      (Function.onFun Disjoint fun c => ball (c : E) ε) := by
    rintro c hc d hd hcd
    apply ball_disjoint_ball
    rw [dist_eq_norm]
    have h := separated c hc d hd hcd
    simpa [ε] using h
  have A_subset : A ⊆ ball (0 : E) ρ := by
    intro y hy
    rw [hA] at hy
    simp only [Set.mem_iUnion] at hy
    obtain ⟨x, hx, hyx⟩ := hy
    rw [mem_ball] at hyx ⊢
    calc
      dist y 0 ≤ dist y x + dist x 0 := dist_triangle _ _ _
      _ < ε + R := add_lt_add_of_lt_of_le hyx (by
        simpa [dist_zero_right] using inside x hx)
      _ = ρ := by simp [ρ, add_comm]
  have measureIneq :
      (s.card : ENNReal) * ENNReal.ofReal (ε ^ Module.finrank ℝ E) * μ (ball 0 1) ≤
        ENNReal.ofReal (ρ ^ Module.finrank ℝ E) * μ (ball 0 1) :=
    calc
      (s.card : ENNReal) * ENNReal.ofReal (ε ^ Module.finrank ℝ E) * μ (ball 0 1) =
          μ A := by
            rw [hA, measure_biUnion_finset disjointBalls fun c _ => measurableSet_ball]
            simp only [μ.addHaar_ball_of_pos _ εpos]
            simp only [Finset.sum_const, nsmul_eq_mul, mul_assoc]
      _ ≤ μ (ball (0 : E) ρ) := measure_mono A_subset
      _ = ENNReal.ofReal (ρ ^ Module.finrank ℝ E) * μ (ball 0 1) := by
            simp only [μ.addHaar_ball_of_pos _ ρpos]
  have withoutUnitBall :
      (s.card : ENNReal) * ENNReal.ofReal (ε ^ Module.finrank ℝ E) ≤
        ENNReal.ofReal (ρ ^ Module.finrank ℝ E) :=
    (ENNReal.mul_le_mul_iff_left
      (measure_ball_pos (μ := μ) _ zero_lt_one).ne'
      (measure_ball_lt_top (μ := μ)).ne).1 measureIneq
  have realIneq :
      (s.card : ℝ) * ε ^ Module.finrank ℝ E ≤
        ρ ^ Module.finrank ℝ E := by
    have h := ENNReal.toReal_le_of_le_ofReal
      (pow_nonneg ρpos.le _) withoutUnitBall
    simpa [ENNReal.toReal_mul, εpos.le] using h
  simpa [ε, ρ] using realIneq

/-- Quotient form of the packing bound:

`#s ≤ ((R + δ/2)/(δ/2))^dim = (1 + 2R/δ)^dim`.
-/
theorem card_le_separated_ball_ratio_pow
    (s : Finset E) {R δ : ℝ} (hR : 0 ≤ R) (hδ : 0 < δ)
    (inside : ∀ c ∈ s, ‖c‖ ≤ R)
    (separated : ∀ c ∈ s, ∀ d ∈ s, c ≠ d → δ ≤ ‖c - d‖) :
    (s.card : ℝ) ≤
      ((R + δ / 2) / (δ / 2)) ^ Module.finrank ℝ E := by
  have h := card_mul_halfSeparation_pow_le s hR hδ inside separated
  have halfpos : 0 < δ / 2 := div_pos hδ (by norm_num)
  rw [div_pow]
  exact (le_div_iff₀ (pow_pos halfpos _)).2 h

/-- The same quotient written in the familiar `1 + 2R/δ` form. -/
theorem card_le_one_add_two_mul_div_pow
    (s : Finset E) {R δ : ℝ} (hR : 0 ≤ R) (hδ : 0 < δ)
    (inside : ∀ c ∈ s, ‖c‖ ≤ R)
    (separated : ∀ c ∈ s, ∀ d ∈ s, c ≠ d → δ ≤ ‖c - d‖) :
    (s.card : ℝ) ≤ (1 + 2 * R / δ) ^ Module.finrank ℝ E := by
  convert card_le_separated_ball_ratio_pow s hR hδ inside separated using 1
  field_simp
  ring

/-- A finite map whose fibers have cardinality at most `T` has source
cardinality at most `T` times the cardinality of any finite set containing its
image.  This is the abstract counting step behind a torsion multiplicity
factor. -/
theorem card_le_card_mul_of_fiber_card_le
    {α β : Type*} (S : Finset α) (s : Finset β) (π : α → β) (T : ℕ)
    (mapsTo : ∀ x ∈ S, π x ∈ s)
    (fiberBound : ∀ y ∈ s, {x : α | x ∈ S ∧ π x = y}.ncard ≤ T) :
    S.card ≤ s.card * T := by
  classical
  rw [Finset.card_eq_sum_card_fiberwise mapsTo]
  calc
    ∑ y ∈ s, (S.filter fun x => π x = y).card ≤ ∑ _y ∈ s, T := by
      exact Finset.sum_le_sum fun y hy => by
        simpa [← Set.ncard_coe_finset] using fiberBound y hy
    _ = s.card * T := by simp

/-- Packing with a finite multiplicity (in the JKSST application, a torsion
fiber).  The map `π` may land in a proper subset of `s`; requiring only that
`s` contain its image makes every use of the projected packing set explicit.
-/
theorem card_le_fiberBound_mul_one_add_two_mul_div_pow
    {α : Type*} (S : Finset α) (s : Finset E) (π : α → E) (T : ℕ)
    {R δ : ℝ} (hR : 0 ≤ R) (hδ : 0 < δ)
    (mapsTo : ∀ x ∈ S, π x ∈ s)
    (fiberBound : ∀ y ∈ s, {x : α | x ∈ S ∧ π x = y}.ncard ≤ T)
    (inside : ∀ y ∈ s, ‖y‖ ≤ R)
    (separated : ∀ y ∈ s, ∀ z ∈ s, y ≠ z → δ ≤ ‖y - z‖) :
    (S.card : ℝ) ≤
      (T : ℝ) * (1 + 2 * R / δ) ^ Module.finrank ℝ E := by
  have sourceCard :=
    card_le_card_mul_of_fiber_card_le S s π T mapsTo fiberBound
  have sourceCardReal :
      (S.card : ℝ) ≤ (s.card : ℝ) * (T : ℝ) := by
    exact_mod_cast sourceCard
  have projectedCard :=
    card_le_one_add_two_mul_div_pow s hR hδ inside separated
  calc
    (S.card : ℝ) ≤ (s.card : ℝ) * (T : ℝ) := sourceCardReal
    _ = (T : ℝ) * (s.card : ℝ) := mul_comm _ _
    _ ≤ (T : ℝ) * (1 + 2 * R / δ) ^ Module.finrank ℝ E :=
      mul_le_mul_of_nonneg_left projectedCard (Nat.cast_nonneg T)

/-- A fully `Nat`-valued interface to the multiplicity packing bound.  Any
explicit natural cutoff `A` above the real packing base gives the discrete
estimate `#S ≤ T * A^dim`. -/
theorem card_le_fiberBound_mul_nat_pow
    {α : Type*} (S : Finset α) (s : Finset E) (π : α → E) (T A : ℕ)
    {R δ : ℝ} (hR : 0 ≤ R) (hδ : 0 < δ)
    (mapsTo : ∀ x ∈ S, π x ∈ s)
    (fiberBound : ∀ y ∈ s, {x : α | x ∈ S ∧ π x = y}.ncard ≤ T)
    (inside : ∀ y ∈ s, ‖y‖ ≤ R)
    (separated : ∀ y ∈ s, ∀ z ∈ s, y ≠ z → δ ≤ ‖y - z‖)
    (base_le : 1 + 2 * R / δ ≤ (A : ℝ)) :
    S.card ≤ T * A ^ Module.finrank ℝ E := by
  have realPacking := card_le_fiberBound_mul_one_add_two_mul_div_pow
    S s π T hR hδ mapsTo fiberBound inside separated
  have base_nonneg : 0 ≤ 1 + 2 * R / δ := by
    have ratio_nonneg : 0 ≤ 2 * R / δ :=
      div_nonneg (mul_nonneg (by norm_num) hR) hδ.le
    linarith
  have power_le :
      (1 + 2 * R / δ) ^ Module.finrank ℝ E ≤
        (A : ℝ) ^ Module.finrank ℝ E :=
    pow_le_pow_left₀ base_nonneg base_le _
  have realDiscrete :
      (S.card : ℝ) ≤ (T : ℝ) * (A : ℝ) ^ Module.finrank ℝ E :=
    realPacking.trans (mul_le_mul_of_nonneg_left power_le (Nat.cast_nonneg T))
  exact_mod_cast realDiscrete

end JKSSTPacking
