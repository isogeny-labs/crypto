import GreenMoment
import MarkovCollision

namespace MarkovBirthdayRoundingAudit
noncomputable section

theorem source_bound_fortyseven : sourceBound 47 1 1 1 (1/44)=1 := by
  norm_num [sourceBound,row_maximum_one,column_maximum_one]

theorem threshold_fortyseven : (1 : ℝ)/47<1-Real.exp (-(1/44)) := by
  have he:=Real.add_one_le_exp (1/44 : ℝ)
  have hm:=mul_le_mul_of_nonneg_left he (le_of_lt (Real.exp_pos (-(1/44 : ℝ))))
  have hx : Real.exp (-(1/44 : ℝ))*Real.exp (1/44)=1 := by
    rw [←Real.exp_add]
    norm_num
  rw [hx] at hm
  nlinarith

/-- Even rounding the displayed time upward to the next step does not fix this witness. -/
def LiteralUniform47Claim : Prop := ∀ c : ℝ,0≤c →
  1-Real.exp (-c)≤collisionCDF 47 0 (Nat.ceil (sourceBound 47 1 1 1 c))

theorem literal_uniform47_false : ¬LiteralUniform47Claim := by
  intro h
  have hc : Nat.ceil (sourceBound 47 1 1 1 (1/44))=1 := by
    rw [source_bound_fortyseven]
    norm_num
  have hh:=h (1/44) (by norm_num)
  rw [hc,collision_cdf_one] at hh
  exact (not_le.mpr threshold_fortyseven) hh

/-- All actual source finite-chain conditions and the actual violating probability. -/
theorem kmpt_literal_finite_counter :
    Nat.Prime 47 ∧
    (∀ x : ZMod 47,(∑ y : ZMod 47,transitionMatrix 47 x y)=1) ∧
    (∀ y : ZMod 47,(∑ x : ZMod 47,transitionMatrix 47 x y)=1) ∧
    (∀ x y : ZMod 47,0<transitionMatrix 47 x y) ∧
    (∀ x y : ZMod 47,∃ n : ℕ,0<n ∧ 0<(transitionMatrix 47^n) x y) ∧
    (∀ x : ZMod 47,∀ d : ℕ,(∀ n : ℕ,0<n → 0<(transitionMatrix 47^n) x x → d∣n) → d=1) ∧
    (∀ x y : ZMod 47,(1 : ℝ)/47≤(transitionMatrix 47^1) x y ∧
      (transitionMatrix 47^1) x y≤(1 : ℝ)/47) ∧
    rowMaximum 47 1=50/47 ∧ columnMaximum 47 1=50/47 ∧
    sourceBound 47 1 1 1 (1/44)=1 ∧
    collisionCDF 47 0 1=1/47 ∧
    collisionCDF 47 0 1<1-Real.exp (-(1/44)) ∧ ¬LiteralUniform47Claim := by
  refine ⟨by decide,transition_row_sum 47,transition_column_sum 47,transition_positive 47,
    actual_irreducible 47,actual_aperiodic 47,uniform_mixing_one 47,?_,?_,
    source_bound_fortyseven,collision_cdf_one 47 0,?_,literal_uniform47_false⟩
  · norm_num [row_maximum_one]
  · norm_num [column_maximum_one]
  · rw [collision_cdf_one]
    exact threshold_fortyseven

#print axioms source_bound_fortyseven
#print axioms threshold_fortyseven
#print axioms literal_uniform47_false
#print axioms kmpt_literal_finite_counter
end
end MarkovBirthdayRoundingAudit
