import SidonTable
import UniformCollision

namespace SidonRhoCounterfamilyAudit
noncomputable section
open scoped BigOperators
open RhoFirstCollisionAudit RhoUniformCollisionAudit WalkWeightedGeneralAudit
variable (p : ℕ) [NeZero p]

def transition (h : ZMod p → Fin 20) (x : ZMod p) : ZMod p := x + increments p (h x)

/-- This embedding is pointwise. Its distribution is not asserted uniform on Fin951. -/
def boundedHash (h : ZMod p → Fin 20) : ZMod p → Fin 951 :=
  fun x => ⟨stepNat (h x) - 1, by have := step_bounds (h x); omega⟩

omit [NeZero p] in
theorem transition_eq (h : ZMod p → Fin 20) :
    transition p h = RhoBoundedStepsAudit.transition p 951 (boundedHash p h) := by
  funext x
  have hs : stepNat (h x) - 1 + 1 = stepNat (h x) := by
    have := step_bounds (h x)
    omega
  simp only [transition, increments, RhoBoundedStepsAudit.transition,
    RhoBoundedStepsAudit.increments, boundedHash, hs]

theorem firstCollision_bounds (h : ZMod p → Fin 20) :
    p ≤ 951 * firstCollision p (transition p h) 0 ∧
      firstCollision p (transition p h) 0 ≤ p := by
  rw [transition_eq]
  exact RhoBoundedStepsAudit.firstCollision_bounds p 951 (boundedHash p h)

def meanCollision : ℝ := expectation (uniformWeights 20)
  (fun h : Hash (ZMod p) (Fin 20) => (firstCollision p (transition p h) 0 : ℝ))

theorem mean_nonnegative : 0 ≤ meanCollision p := by
  unfold meanCollision expectation
  apply Finset.sum_nonneg
  intro h _
  exact mul_nonneg (weight_nonnegative _ (weights_nonnegative 20) h) (by positivity)

theorem mean_lower : (p : ℝ) ≤ 951 * meanCollision p := by
  have hh := uniform_expectation_mono p 20 (fun _ => (p : ℝ))
    (fun h => (951 : ℝ) * (firstCollision p (transition p h) 0 : ℝ)) (by
      intro h
      exact_mod_cast (firstCollision_bounds p h).1)
  rw [uniform_expectation_const p 20 (by decide), uniform_expectation_mul] at hh
  exact hh

theorem mean_upper : meanCollision p ≤ p := by
  have hh := uniform_expectation_mono p 20
    (fun h => (firstCollision p (transition p h) 0 : ℝ)) (fun _ => (p : ℝ)) (by
      intro h
      exact_mod_cast (firstCollision_bounds p h).2)
  rw [uniform_expectation_const p 20 (by decide)] at hh
  exact hh

theorem mean_eq_actual_pmf : meanCollision p =
    ∑ h, (uniformHashLaw p 20 (by decide) h).toReal *
      (firstCollision p (transition p h) 0 : ℝ) :=
  expectation_eq_pmf p 20 (by decide) _

theorem actual_moments (hp : 1902 < p) (target : ZMod p) :
    expectation (uniformWeights 20) (fun h => inDegree (increments p) h target) = 1 ∧
    expectation (uniformWeights 20) (fun h => (inDegree (increments p) h target - 1)^2) =
      19 / 20 := by
  refine ⟨inDegree_mean _ (weights_sum 20 (by decide)) _ target, ?_⟩
  rw [inDegree_variance _ (weights_sum 20 (by decide)) _ (generic p hp).1]
  norm_num [uniformWeights]

theorem uniform_local_coefficient :
    1 - TwoStepCollisionAudit.secondMoment (uniformWeights 20) -
      (TwoStepCollisionAudit.secondMoment (uniformWeights 20))^2 +
        TwoStepCollisionAudit.fourthMoment (uniformWeights 20) = (7581 : ℝ) / 8000 := by
  norm_num [TwoStepCollisionAudit.secondMoment, TwoStepCollisionAudit.fourthMoment, uniformWeights]

abbrev localLaw : PMF (GeneralTwoStepCollisionAudit.JointSample (ZMod p) (Fin 20)) :=
  GeneralTwoStepCollisionAudit.jointLaw (uniformWeights 20)
    (weights_nonnegative 20) (weights_sum 20 (Nat.succ_pos 19))

theorem actual_local_probability (hp : 1902 < p) :
    (GeneralTwoStepCollisionAudit.jointLaw (G := ZMod p) (uniformWeights 20)
      (weights_nonnegative 20) (weights_sum 20 (by decide))).toOuterMeasure
        (GeneralTwoStepCollisionAudit.freshEvent (increments p) :
          Set (GeneralTwoStepCollisionAudit.JointSample (ZMod p) (Fin 20))) =
      ENNReal.ofReal ((7581 : ℝ) / (8000 * p)) := by
  rw [GeneralTwoStepCollisionAudit.actual_fresh_collision_measure
    (uniformWeights 20) (weights_nonnegative 20) (weights_sum 20 (by decide))
    (increments p) (generic p hp), uniform_local_coefficient, ZMod.card]
  congr 1
  ring

#print axioms firstCollision_bounds
#print axioms mean_lower
#print axioms mean_eq_actual_pmf
#print axioms actual_moments
#print axioms actual_local_probability
end
end SidonRhoCounterfamilyAudit
