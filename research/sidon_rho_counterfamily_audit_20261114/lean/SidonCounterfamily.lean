import SlowCollision
import Mathlib.Data.Nat.Prime.Infinite

namespace SidonRhoCounterfamilyAudit
noncomputable section
open scoped BigOperators
open RhoFirstCollisionAudit RhoUniformCollisionAudit WalkWeightedGeneralAudit

theorem ratio_lower (p C : ℕ) [NeZero p] (hp : 904401 * C < p) :
    (C : ℝ) * p < (meanCollision p)^2 := by
  have hn := mean_nonnegative p
  have hlow := mean_lower p
  have hpos : (0 : ℝ) < p := by exact_mod_cast Nat.pos_of_ne_zero (NeZero.ne p)
  have hlarge : (904401 : ℝ) * C < p := by exact_mod_cast hp
  have hs := mul_nonneg (sub_nonneg.mpr hlow)
    (show 0 ≤ (951 : ℝ) * meanCollision p + p by positivity)
  have hc := mul_pos hpos (sub_pos.mpr hlarge)
  nlinarith

theorem arbitrarily_large_collision_ratio (C : ℕ) :
    ∃ (p : ℕ) (hp : Nat.Prime p), 1902 < p ∧
      (C : ℝ) * p < (@meanCollision p ⟨hp.ne_zero⟩)^2 := by
  obtain ⟨p, hpB, hp⟩ := Nat.exists_infinite_primes (904401 * C + 1903)
  let : NeZero p := ⟨hp.ne_zero⟩
  exact ⟨p, hp, by omega, ratio_lower p C (by omega)⟩

/-- Direct negation of a uniform square-root bound, even after retaining
the literal degree-two source guard. This is a fixed table, not a random table. -/
theorem universal_degree_two_collision_bound_false :
    ¬∃ C : ℝ, ∀ (p : ℕ) (hp : Nat.Prime p), 1902 < p →
      TwoStepCollisionAudit.DegreeTwoGeneric (increments p) →
        (@meanCollision p ⟨hp.ne_zero⟩)^2 ≤ C * p := by
  rintro ⟨C, hC⟩
  obtain ⟨N, hN⟩ := exists_nat_gt C
  obtain ⟨p, hp, hpB, hcounter⟩ := arbitrarily_large_collision_ratio N
  let : NeZero p := ⟨hp.ne_zero⟩
  have hh := hC p hp hpB (generic p hpB)
  have hc : C * (p : ℝ) ≤ (N : ℝ) * p :=
    mul_le_mul_of_nonneg_right (le_of_lt hN) (by positivity)
  exact (not_lt_of_ge (hh.trans hc)) hcounter

theorem concrete_prime : Nat.Prime 10000019 := by norm_num

theorem concrete_collision_ratio : (4 : ℝ) * 10000019 < (meanCollision 10000019)^2 :=
  ratio_lower 10000019 4 (by norm_num)

theorem sidon_rho_counterfamily_certificate :
    (∀ (p : ℕ) [NeZero p], 1902 < p →
      TwoStepCollisionAudit.DegreeTwoGeneric (increments p) ∧
      (∀ h : ZMod p → Fin 20,
        p ≤ 951 * firstCollision p (transition p h) 0 ∧
          firstCollision p (transition p h) 0 ≤ p) ∧
      (p : ℝ) ≤ 951 * meanCollision p ∧ meanCollision p ≤ p ∧
      meanCollision p = ∑ h, (uniformHashLaw p 20 (Nat.succ_pos 19) h).toReal *
        (firstCollision p (transition p h) 0 : ℝ) ∧
      (∀ target : ZMod p,
        expectation (uniformWeights 20) (fun h => inDegree (increments p) h target) = 1 ∧
        expectation (uniformWeights 20) (fun h => (inDegree (increments p) h target - 1)^2) =
          19 / 20) ∧
      (localLaw p).toOuterMeasure
          (GeneralTwoStepCollisionAudit.freshEvent (increments p) :
            Set (GeneralTwoStepCollisionAudit.JointSample (ZMod p) (Fin 20))) =
        ENNReal.ofReal ((7581 : ℝ) / (8000 * p))) ∧
    (¬∃ C : ℝ, ∀ (p : ℕ) (hp : Nat.Prime p), 1902 < p →
      TwoStepCollisionAudit.DegreeTwoGeneric (increments p) →
        (@meanCollision p ⟨hp.ne_zero⟩)^2 ≤ C * p) ∧
    Nat.Prime 10000019 ∧ (4 : ℝ) * 10000019 < (meanCollision 10000019)^2 := by
  refine ⟨?_, universal_degree_two_collision_bound_false, concrete_prime, concrete_collision_ratio⟩
  intro p _ hp
  exact ⟨generic p hp, firstCollision_bounds p, mean_lower p, mean_upper p,
    mean_eq_actual_pmf p, actual_moments p hp, actual_local_probability p hp⟩

#print axioms arbitrarily_large_collision_ratio
#print axioms universal_degree_two_collision_bound_false
#print axioms concrete_prime
#print axioms sidon_rho_counterfamily_certificate
end
end SidonRhoCounterfamilyAudit
