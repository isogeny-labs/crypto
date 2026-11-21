import GreenAsymptotic

namespace MarkovGreenUpperAudit
noncomputable section
open scoped BigOperators
open Filter Topology Asymptotics
open MarkovGreenBoundAudit MarkovBirthdayRoundingAudit

/-- Finite weighted algebra, without an infinite-series premise. -/
def ActualWeightedGeometric : Prop :=
  (∀ d : ℝ, ∀ n : ℕ,
    (1-d)^2*weightedGeometric d n = 1-((n : ℝ)+1)*d^n+(n : ℝ)*d^(n+1)) ∧
  (∀ d : ℝ, 0≤d → d<1 → ∀ n : ℕ, weightedGeometric d n ≤ 1/(1-d)^2) ∧
  (∀ c d : ℝ, ∀ n : ℕ,
    2*(∑ j ∈ Finset.range n, ((j : ℝ)+1)*(c+d^j)) =
      c*(n : ℝ)*((n : ℝ)+1)+2*weightedGeometric d n)

/-- Finite bound uses actual transition entries and actual Green maxima. -/
def ActualFiniteGreenUpper : Prop :=
  ∀ (S : Type) [Fintype S] [DecidableEq S] [Nonempty S], ∀ P : Matrix S S ℝ,
    P ∈ doublyStochastic ℝ S → ∀ T : ℕ, ∀ c d : ℝ, 0≤d → d<1 →
    (∀ j : ℕ, j≤T → ∀ x y : S, (P^j) x y ≤ c+d^j) →
    rowMaximum P T ≤ c*((T : ℝ)+1)*((T : ℝ)+2)+2/(1-d)^2 ∧
    columnMaximum P T ≤ c*((T : ℝ)+1)*((T : ℝ)+2)+2/(1-d)^2

/-- Varying actual state spaces, an actual little-o hypothesis and its exact upper conclusion. -/
def ActualAsymptoticGreenUpper : Prop :=
  (∀ e : ℕ → ℝ, e =o[atTop] (fun t : ℕ => 1/(t : ℝ)^2) →
    Tendsto (remainder e) atTop (𝓝 0)) ∧
  (∀ (S : ℕ → Type) [∀ t, Fintype (S t)] [∀ t, DecidableEq (S t)] [∀ t, Nonempty (S t)],
    ∀ P : ∀ t, Matrix (S t) (S t) ℝ,
    (∀ t, P t ∈ doublyStochastic ℝ (S t)) → ∀ e : ℕ → ℝ, ∀ d : ℝ, 0≤d → d<1 →
    e =o[atTop] (fun t : ℕ => 1/(t : ℝ)^2) →
    (∀ t j : ℕ, j≤t → ∀ x y : S t, (P t^j) x y ≤ e t+d^j) →
    Tendsto (remainder e) atTop (𝓝 0) ∧ ∀ t : ℕ,
      rowMaximum (P t) t ≤ 2/(1-d)^2+remainder e t ∧
      columnMaximum (P t) t ≤ 2/(1-d)^2+remainder e t) ∧
  (∀ (S : ℕ → Type) [∀ t, Fintype (S t)] [∀ t, DecidableEq (S t)] [∀ t, Nonempty (S t)],
    ∀ P : ∀ t, Matrix (S t) (S t) ℝ,
    (∀ t, P t ∈ doublyStochastic ℝ (S t)) → ∀ e : ℕ → ℝ, ∀ d : ℝ, 0≤d → d<1 →
    e =o[atTop] (fun t : ℕ => 1/(t : ℝ)^2) →
    (∀ t j : ℕ, j≤t → ∀ x y : S t, (P t^j) x y ≤ e t+d^j) →
    ∀ ε : ℝ, 0<ε → ∀ᶠ t in atTop,
      rowMaximum (P t) t ≤ 2/(1-d)^2+ε ∧ columnMaximum (P t) t ≤ 2/(1-d)^2+ε)

theorem strict_family_upper_control (t : ℕ) :
    familyRow t ≤ 8+remainder familyError t ∧
    familyColumn t ≤ 8+remainder familyError t := by
  have h := finite_green_envelope (transitionMatrix (stateCount t))
    (uniform_doubly_stochastic (stateCount t)) t |familyError t| (1/2) (by norm_num) (by norm_num)
    (by
      intro j _ x y
      exact (family_envelope t j x y).trans (add_le_add (le_abs_self (familyError t)) le_rfl))
  norm_num at h
  simpa only [familyRow, familyColumn, remainder, add_comm] using h

/-- A strict upper-bound control retains the actual earlier equality counter. -/
def ActualStrictUpperControl : Prop :=
  (∀ t : ℕ, familyRow t ≤ 8+remainder familyError t ∧
    familyColumn t ≤ 8+remainder familyError t) ∧
  Tendsto (remainder familyError) atTop (𝓝 0) ∧
  Tendsto familyRow atTop (𝓝 1) ∧ Tendsto familyColumn atTop (𝓝 1) ∧
  ¬ Tendsto familyRow atTop (𝓝 ((2 : ℝ)/(1-1/2)^2))

theorem actual_green_upper_envelope_certificate :
    ActualWeightedGeometric ∧ ActualFiniteGreenUpper ∧
      ActualAsymptoticGreenUpper ∧ ActualStrictUpperControl := by
  refine ⟨?_, ?_, ?_, ?_⟩
  · exact ⟨weighted_geometric_identity, weighted_geometric_bound, envelope_weight_sum⟩
  · exact fun _ _ _ _ => finite_green_envelope
  · exact ⟨remainder_limit, fun _ _ _ _ => asymptotic_green_upper,
      fun _ _ _ _ => eventual_epsilon_green_upper⟩
  · exact ⟨strict_family_upper_control, remainder_limit familyError family_error_littleO,
      family_row_limit, family_column_limit, literal_asymptotic_equality_false⟩

#print axioms strict_family_upper_control
#print axioms actual_green_upper_envelope_certificate
end
end MarkovGreenUpperAudit
