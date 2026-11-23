import BoundedSteps

namespace RhoUniformCollisionAudit
noncomputable section
open scoped BigOperators
open RhoFirstCollisionAudit RhoBoundedStepsAudit WalkWeightedGeneralAudit
variable (p r : ℕ) [NeZero p]

def uniformWeights (_i : Fin r) : ℝ := 1/(r:ℝ)

theorem weights_nonnegative (i : Fin r) : 0≤uniformWeights r i := by
  unfold uniformWeights
  positivity

theorem weights_sum (hr : 0<r) : (∑ i : Fin r,uniformWeights r i)=1 := by
  simp only [uniformWeights,Finset.sum_const,Finset.card_univ,Fintype.card_fin,nsmul_eq_mul]
  have hr0 : (r:ℝ)≠0 := by exact_mod_cast (Nat.ne_of_gt hr)
  field_simp

def uniformHashLaw (hr : 0<r) : PMF (Hash (ZMod p) (Fin r)) :=
  hashLaw (uniformWeights r) (weights_nonnegative r) (weights_sum r hr)

theorem expectation_eq_pmf (hr : 0<r) (F : Hash (ZMod p) (Fin r)→ℝ) :
    expectation (uniformWeights r) F=∑ h,(uniformHashLaw p r hr h).toReal*F h := by
  unfold expectation
  apply Finset.sum_congr rfl
  intro h _
  rw [uniformHashLaw,hashLaw_real_mass]

theorem uniform_expectation_mono (F H : Hash (ZMod p) (Fin r)→ℝ)
    (hFH : ∀ h,F h≤H h) :
    expectation (uniformWeights r) F≤expectation (uniformWeights r) H := by
  apply Finset.sum_le_sum
  intro h _
  exact mul_le_mul_of_nonneg_left (hFH h)
    (weight_nonnegative _ (weights_nonnegative r) h)

theorem uniform_expectation_const (hr : 0<r) (c : ℝ) :
    expectation (uniformWeights r) (fun _ : Hash (ZMod p) (Fin r)=>c)=c := by
  unfold expectation
  rw [←Finset.sum_mul,weight_sum _ (weights_sum r hr),one_mul]

theorem uniform_expectation_mul (c : ℝ) (F : Hash (ZMod p) (Fin r)→ℝ) :
    expectation (uniformWeights r) (fun h=>c*F h)=c*expectation (uniformWeights r) F := by
  unfold expectation
  rw [Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro h _
  ring

def meanCollision : ℝ := expectation (uniformWeights r)
  (fun h : Hash (ZMod p) (Fin r)=>(firstCollision p (transition p r h) 0:ℝ))

theorem meanCollision_nonnegative : 0≤meanCollision p r := by
  unfold meanCollision expectation
  apply Finset.sum_nonneg
  intro h _
  exact mul_nonneg (weight_nonnegative _ (weights_nonnegative r) h) (by positivity)

theorem meanCollision_lower (hr : 0<r) : (p:ℝ)≤(r:ℝ)*meanCollision p r := by
  have hh:=uniform_expectation_mono p r
    (fun _=>(p:ℝ))
    (fun h=>(r:ℝ)*(firstCollision p (transition p r h) 0:ℝ)) (by
      intro h
      exact_mod_cast (firstCollision_bounds p r h).1)
  rw [uniform_expectation_const p r hr,uniform_expectation_mul] at hh
  exact hh

theorem meanCollision_upper (hr : 0<r) : meanCollision p r≤p := by
  have hh:=uniform_expectation_mono p r
    (fun h=>(firstCollision p (transition p r h) 0:ℝ))
    (fun _=>(p:ℝ)) (by
      intro h
      exact_mod_cast (firstCollision_bounds p r h).2)
  rw [uniform_expectation_const p r hr] at hh
  exact hh

theorem mean_and_variance (hr : 0<r) (hrp : r<p) (target : ZMod p) :
    expectation (uniformWeights r) (fun h=>inDegree (increments p r) h target)=1 ∧
    expectation (uniformWeights r) (fun h=>(inDegree (increments p r) h target-1)^2)=1-1/(r:ℝ) := by
  refine ⟨inDegree_mean _ (weights_sum r hr) _ target,?_⟩
  rw [inDegree_variance _ (weights_sum r hr) _ (increments_injective p r hrp)]
  simp only [uniformWeights,Finset.sum_const,Finset.card_univ,Fintype.card_fin,nsmul_eq_mul]
  have hr0 : (r:ℝ)≠0 := by exact_mod_cast (Nat.ne_of_gt hr)
  field_simp

#print axioms expectation_eq_pmf
#print axioms meanCollision_lower
#print axioms mean_and_variance
end
end RhoUniformCollisionAudit
