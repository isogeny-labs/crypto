import GreenControls
import GreenUniformFamily

namespace MarkovGreenBoundAudit
noncomputable section
open scoped BigOperators
open Filter Topology Asymptotics
open MarkovBirthdayRoundingAudit

/-- Quantified actual finite-kernel laws, not assumed moment inequalities. -/
def ActualGeneralGreenBounds : Prop :=
  (∀ (S : Type) [Fintype S] [DecidableEq S], ∀ P : Matrix S S ℝ,
    ∀ hP : P ∈ Matrix.rowStochastic ℝ S, ∀ x y : S, (kernel P hP x y).toReal = P x y) ∧
  (∀ (S : Type) [Fintype S] [DecidableEq S] [Nonempty S], ∀ P : Matrix S S ℝ,
    P ∈ Matrix.rowStochastic ℝ S → ∀ T : ℕ, rowMaximum P T ≤ sourceRHS P T) ∧
  (∀ (S : Type) [Fintype S] [DecidableEq S] [Nonempty S], ∀ P : Matrix S S ℝ,
    P ∈ Matrix.colStochastic ℝ S → ∀ T : ℕ, columnMaximum P T ≤ sourceRHS P T) ∧
  (∀ (S : Type) [Fintype S] [DecidableEq S] [Nonempty S], ∀ P : Matrix S S ℝ,
    P ∈ doublyStochastic ℝ S → ∀ T : ℕ,
      rowMaximum P T ≤ sourceRHS P T ∧ columnMaximum P T ≤ sourceRHS P T) ∧
  (∀ (S : Type) [Fintype S] [DecidableEq S] [Nonempty S], ∀ P : Matrix S S ℝ,
    P ∈ Matrix.rowStochastic ℝ S →
    (∀ y : S, (∑ x : S, uniformMass x * P x y) = uniformMass y) → ∀ T : ℕ,
      rowMaximum P T ≤ sourceRHS P T ∧ columnMaximum P T ≤ sourceRHS P T)

/-- Positive finite chain refutes removal of uniform stationarity. -/
def ActualStationarityGuardCounter : Prop :=
  (∀ x y : ZMod 11, (biasedLaw x y).toReal = biasedMatrix x y) ∧
  biasedMatrix ∈ Matrix.rowStochastic ℝ (ZMod 11) ∧
  (∀ x y : ZMod 11, ∃ n : ℕ, 0<n ∧ 0<(biasedMatrix^n) x y) ∧
  (∀ x : ZMod 11, ∀ d : ℕ,
    (∀ n : ℕ, 0<n → 0<(biasedMatrix^n) x x → d∣n) → d=1) ∧
  (∀ y : ZMod 11, (∑ x : ZMod 11, biasedWeight x*biasedMatrix x y) = biasedWeight y) ∧
  (¬ (∀ y : ZMod 11, (∑ x : ZMod 11, (1/11 : ℝ)*biasedMatrix x y) = 1/11)) ∧
  columnEnergy biasedMatrix 1 0 = 19/4 ∧ sourceRHS biasedMatrix 1 = 4 ∧
  columnMaximum biasedMatrix 1 > sourceRHS biasedMatrix 1

/-- Full actual uniform Green formula, including the zero-time identity. -/
def ActualUniformGreenFormula : Prop :=
  ∀ p : ℕ, ∀ [NeZero p], ∀ T : ℕ,
    transitionMatrix p ∈ doublyStochastic ℝ (ZMod p) ∧
    rowMaximum (transitionMatrix p) T = 1+((T : ℝ)^2+2*T)/p ∧
    columnMaximum (transitionMatrix p) T = 1+((T : ℝ)^2+2*T)/p

/-- A prime-state actual uniform family satisfies the envelope but not printed equality. -/
def ActualAsymptoticEqualityCounter : Prop :=
  (∀ t : ℕ, Nat.Prime (stateCount t)) ∧
  (∀ t : ℕ, 3^t ≤ stateCount t) ∧
  (∀ t : ℕ, transitionMatrix (stateCount t) ∈ doublyStochastic ℝ (ZMod (stateCount t))) ∧
  (∀ t : ℕ, ∀ x y : ZMod (stateCount t),
    ∃ n : ℕ, 0<n ∧ 0<(transitionMatrix (stateCount t)^n) x y) ∧
  (∀ t : ℕ, ∀ x : ZMod (stateCount t), ∀ d : ℕ,
    (∀ n : ℕ, 0<n → 0<(transitionMatrix (stateCount t)^n) x x → d∣n) → d=1) ∧
  familyError =o[atTop] (fun t : ℕ => 1/(t : ℝ)^2) ∧
  (∀ t j : ℕ, ∀ x y : ZMod (stateCount t),
    (transitionMatrix (stateCount t)^j) x y ≤ familyError t+(1/2 : ℝ)^j) ∧
  Tendsto familyRow atTop (𝓝 1) ∧ Tendsto familyColumn atTop (𝓝 1) ∧
  (¬ Tendsto familyRow atTop (𝓝 ((2 : ℝ)/(1-1/2)^2))) ∧
  ¬ ∃ e : ℕ → ℝ, Tendsto e atTop (𝓝 0) ∧
    ∀ᶠ t in atTop, familyRow t = (2+e t)/(1-1/2 : ℝ)^2

theorem actual_green_bound_and_counter_certificate :
    ActualGeneralGreenBounds ∧ ActualStationarityGuardCounter ∧
      ActualUniformGreenFormula ∧ ActualAsymptoticEqualityCounter := by
  refine ⟨?_, ?_, ?_, ?_⟩
  · exact ⟨fun _ _ _ => kernel_entry, fun _ _ _ _ => row_maximum_bound,
      fun _ _ _ _ => column_maximum_bound, fun _ _ _ _ => qualified_green_bound,
      fun _ _ _ _ => uniform_stationary_green_bound⟩
  · refine ⟨fun _ _ => rfl, ?_, biased_irreducible, biased_aperiodic, biased_stationary,
      biased_uniform_not_stationary, biased_column_energy_one, biased_source_bound_one,
      uniform_stationarity_guard_counter⟩
    apply Matrix.mem_rowStochastic_iff_sum.mpr
    exact ⟨fun x y => by simpa only [biased_entry] using (biased_weight_positive y).le,
      fun _ => by simpa only [biased_entry] using biased_weight_sum⟩
  · intro p hp T
    exact ⟨uniform_doubly_stochastic p, uniform_row_maximum p T, uniform_column_maximum p T⟩
  · exact ⟨stateCount_prime, stateCount_bound, fun t => uniform_doubly_stochastic (stateCount t),
      fun t => actual_irreducible (stateCount t), fun t => actual_aperiodic (stateCount t),
      family_error_littleO, family_envelope, family_row_limit, family_column_limit,
      literal_asymptotic_equality_false, literal_remainder_equality_false⟩

#print axioms actual_green_bound_and_counter_certificate
end
end MarkovGreenBoundAudit
