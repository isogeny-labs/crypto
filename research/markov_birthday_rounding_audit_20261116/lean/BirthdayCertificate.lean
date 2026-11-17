import BirthdayBound
import KMPTCounter

namespace MarkovBirthdayRoundingAudit
noncomputable section
open StaticHashCollisionAudit

def ActualUniformChains : Prop :=
  ∀ (p : ℕ) [NeZero p],
    (∀ x : ZMod p,(∑ y : ZMod p,transitionMatrix p x y)=1) ∧
    (∀ y : ZMod p,(∑ x : ZMod p,transitionMatrix p x y)=1) ∧
    (∀ x y : ZMod p,0<transitionMatrix p x y) ∧
    (∀ y : ZMod p,(∑ x : ZMod p,uniformWeight p x*transitionMatrix p x y)=uniformWeight p y) ∧
    (∀ x y : ZMod p,∃ n : ℕ,0<n ∧ 0<(transitionMatrix p^n) x y) ∧
    (∀ x : ZMod p,∀ d : ℕ,(∀ n : ℕ,0<n → 0<(transitionMatrix p^n) x x → d∣n) → d=1) ∧
    (∀ (n : ℕ) (w : Fin n → ZMod p),(wordLaw p n w).toReal=(1/(p : ℝ))^n)

def ActualGreenAtOne : Prop :=
  ∀ (p : ℕ) [NeZero p],
    rowMaximum p 1=1+3/(p : ℝ) ∧ columnMaximum p 1=1+3/(p : ℝ) ∧
      ∀ x y : ZMod p,(1 : ℝ)/p≤(transitionMatrix p^1) x y ∧
        (transitionMatrix p^1) x y≤(1 : ℝ)/p

def ActualUniformBirthday : Prop :=
  ∀ (p : ℕ) [NeZero p] (x : ZMod p) (n : ℕ),
    iidSurvival (uniformWeight p) (resetUpdate p) x n=
      ((p-1).descFactorial n : ℝ)*(1/(p : ℝ))^n ∧
    collisionCDF p x n=1-((p-1).descFactorial n : ℝ)*(1/(p : ℝ))^n ∧
    iidSurvival (uniformWeight p) (resetUpdate p) x n ≤ Real.exp (-((n : ℝ)*(n+1)/(2*(p : ℝ)))) ∧
    (∀ c : ℝ,2*(p : ℝ)*c≤(n : ℝ)*(n+1) → 1-Real.exp (-c)≤collisionCDF p x n) ∧
    (∀ c : ℝ,0≤c → 1-Real.exp (-c)≤collisionCDF p x (Nat.ceil (Real.sqrt (2*(p : ℝ)*c))))

def ActualFiniteCounterAndControls : Prop :=
  Nat.Prime 47 ∧ rowMaximum 47 1=50/47 ∧ columnMaximum 47 1=50/47 ∧
    sourceBound 47 1 1 1 (1/44)=1 ∧ collisionCDF 47 0 1=1/47 ∧
    collisionCDF 47 0 1<1-Real.exp (-(1/44)) ∧ ¬LiteralUniform47Claim ∧
    (1-Real.exp (-(1/44))≤collisionCDF 47 0 2) ∧
    (∀ (p : ℕ) [NeZero p] (x : ZMod p),collisionCDF p x 0=1-Real.exp (-(0 : ℝ))) ∧
    (∀ x : ZMod 1,collisionCDF 1 x 1=1)

theorem actual_markov_birthday_rounding_certificate :
    ActualUniformChains ∧ ActualGreenAtOne ∧ ActualUniformBirthday ∧ ActualFiniteCounterAndControls := by
  refine ⟨?_,?_,?_,?_⟩
  · intro p _
    exact ⟨transition_row_sum p,transition_column_sum p,transition_positive p,uniform_stationary p,
      actual_irreducible p,actual_aperiodic p,word_real_mass p⟩
  · intro p _
    exact ⟨row_maximum_one p,column_maximum_one p,uniform_mixing_one p⟩
  · intro p _ x n
    exact ⟨uniform_birthday_survival p x n,uniform_birthday_cdf p x n,uniform_survival_exp p x n,
      uniform_integer_time_bound p x n,uniform_ceiling_time_bound p x⟩
  · refine ⟨by decide,?_,?_,source_bound_fortyseven,collision_cdf_one 47 0,?_,literal_uniform47_false,
      fortyseven_two_step_control,zero_confidence_control,singleton_one_step_control⟩
    · norm_num [row_maximum_one]
    · norm_num [column_maximum_one]
    · rw [collision_cdf_one]
      exact threshold_fortyseven

#print axioms actual_markov_birthday_rounding_certificate
end
end MarkovBirthdayRoundingAudit
