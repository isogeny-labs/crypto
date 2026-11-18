import UniformChain

namespace MarkovBirthdayRoundingAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit StaticHashCollisionAudit RhoFirstCollisionAudit
variable (p : ℕ) [NeZero p]

/-- The actual static random-function law, whose first-repeat tails equal the actual chain's. -/
def collisionCDF (x : ZMod p) (n : ℕ) : ℝ :=
  ∑ h : Hash (ZMod p) (ZMod p),
    (hashLaw (uniformWeight p) (uniform_weight_nonnegative p) (uniform_weight_sum p) h).toReal *
      (if firstCollision p (staticStep (resetUpdate p) h) x≤n then 1 else 0)

theorem collision_by_one_iff (h : ZMod p → ZMod p) (x : ZMod p) :
    firstCollision p (staticStep (resetUpdate p) h) x≤1 ↔ h x=x := by
  constructor
  · intro he
    have ht : firstCollision p (staticStep (resetUpdate p) h) x=1 :=
      Nat.le_antisymm he (firstCollision_positive p _ x)
    have hc : CollisionAt p (staticStep (resetUpdate p) h) x 1 := by
      simpa only [ht] using firstCollision_spec p (staticStep (resetUpdate p) h) x
    obtain ⟨i,hi⟩:=hc
    have hi0 : i=0 := Subsingleton.elim _ _
    subst i
    simpa only [Fin.val_zero,Function.iterate_zero_apply,Function.iterate_one,staticStep,resetUpdate]
      using hi.symm
  · intro he
    have hc : CollisionAt p (staticStep (resetUpdate p) h) x 1 :=
      ⟨0,by simpa only [Fin.val_zero,Function.iterate_zero_apply,Function.iterate_one,staticStep,resetUpdate]
        using he.symm⟩
    exact Nat.find_min' (collision_exists p (staticStep (resetUpdate p) h) x) hc

theorem collision_cdf_one (x : ZMod p) : collisionCDF p x 1=1/(p : ℝ) := by
  unfold collisionCDF
  simp only [hashLaw_real_mass,collision_by_one_iff]
  simpa only [expectation,delta,uniformWeight] using
    expectation_coordinate (uniformWeight p) (uniform_weight_sum p) x x

theorem collision_cdf_zero (x : ZMod p) : collisionCDF p x 0=0 := by
  have ht (h : ZMod p → ZMod p) :
      ¬firstCollision p (staticStep (resetUpdate p) h) x≤0 :=
    Nat.not_le.mpr (firstCollision_positive p _ x)
  simp only [collisionCDF,ite_eq_right (ht _),mul_zero,Finset.sum_const_zero]

theorem collision_cdf_complement (x : ZMod p) (n : ℕ) :
    collisionCDF p x n=1-iidSurvival (uniformWeight p) (resetUpdate p) x n := by
  have ht:=first_collision_tail_mass p (uniformWeight p) (uniform_weight_sum p) (resetUpdate p) x n
  unfold expectation at ht
  unfold collisionCDF
  simp only [hashLaw_real_mass]
  have he (h : Hash (ZMod p) (ZMod p)) :
      weight (uniformWeight p) h*(if firstCollision p (staticStep (resetUpdate p) h) x≤n then 1 else 0)=
      weight (uniformWeight p) h-
        weight (uniformWeight p) h*(if n<firstCollision p (staticStep (resetUpdate p) h) x then 1 else 0) := by
    by_cases hh : firstCollision p (staticStep (resetUpdate p) h) x≤n
    · simp only [ite_eq_left hh,ite_eq_right (Nat.not_lt.mpr hh),mul_one,mul_zero,sub_zero]
    · simp only [ite_eq_right hh,ite_eq_left (Nat.lt_of_not_ge hh),mul_one,mul_zero,sub_self]
  simp_rw [he]
  rw [Finset.sum_sub_distrib,weight_sum (uniformWeight p) (uniform_weight_sum p),ht]

#print axioms collision_by_one_iff
#print axioms collision_cdf_one
#print axioms collision_cdf_zero
#print axioms collision_cdf_complement
end
end MarkovBirthdayRoundingAudit
