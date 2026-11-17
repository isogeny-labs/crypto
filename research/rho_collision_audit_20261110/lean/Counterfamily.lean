import UniformCollision
import Mathlib.Data.Nat.Prime.Infinite

namespace RhoCollisionCounterfamilyAudit
noncomputable section
open scoped BigOperators
open RhoFirstCollisionAudit RhoBoundedStepsAudit RhoUniformCollisionAudit
  WalkWeightedGeneralAudit

/-- Twenty distinct nonzero increments and independent uniform node labels.
The increment table is fixed, rather than sampled uniformly from the group. -/
theorem twenty_partition_certificate (p : ℕ) [NeZero p] (hp : 20<p) :
    Function.Injective (increments p 20) ∧
    (∀ i : Fin 20,increments p 20 i≠0) ∧
    (∀ h : ZMod p→Fin 20,
      p≤20*firstCollision p (transition p 20 h) 0 ∧
      firstCollision p (transition p 20 h) 0≤p) ∧
    (p:ℝ)≤20*meanCollision p 20 ∧ meanCollision p 20≤p ∧
    (∀ target : ZMod p,
      expectation (uniformWeights 20) (fun h=>inDegree (increments p 20) h target)=1 ∧
      expectation (uniformWeights 20) (fun h=>(inDegree (increments p 20) h target-1)^2)=19/20) ∧
    meanCollision p 20=
      ∑ h,(uniformHashLaw p 20 (Nat.succ_pos 19) h).toReal*
        (firstCollision p (transition p 20 h) 0:ℝ) := by
  refine ⟨increments_injective p 20 hp,increments_nonzero p 20 hp,
    firstCollision_bounds p 20,meanCollision_lower p 20 (by decide),
    meanCollision_upper p 20 (by decide),?_,expectation_eq_pmf p 20 (by decide) _⟩
  intro target
  convert mean_and_variance p 20 (by decide) hp target using 1; norm_num

theorem arbitrarily_large_collision_ratio (C : ℕ) :
    ∃ (p : ℕ) (hp : Nat.Prime p),20<p ∧
      (C:ℝ)*p<(@meanCollision p 20 ⟨hp.ne_zero⟩)^2 := by
  obtain ⟨p,hpB,hp⟩:=Nat.exists_infinite_primes (400*C+401)
  have hp20 : 20<p := by omega
  have hpC : 400*C<p := by omega
  let : NeZero p:=⟨hp.ne_zero⟩
  refine ⟨p,hp,hp20,?_⟩
  have hlow:=meanCollision_lower p 20 (by decide)
  have hn:=meanCollision_nonnegative p 20
  have hpos : (0:ℝ)<p := by exact_mod_cast hp.pos
  have hlarge : (400:ℝ)*C<p := by exact_mod_cast hpC
  have hs:=mul_nonneg (sub_nonneg.mpr hlow)
    (show 0≤(20:ℝ)*meanCollision p 20+p by positivity)
  have hc:=mul_pos hpos (sub_pos.mpr hlarge)
  norm_num at hlow hs
  nlinarith

/-- A variance-only square-root law cannot hold uniformly for all fixed
increment tables, even with twenty partitions and ideal independent labels. -/
theorem universal_variance_only_collision_bound_false :
    ¬∃ C : ℝ,∀ (p : ℕ) (hp : Nat.Prime p),20<p →
      (@meanCollision p 20 ⟨hp.ne_zero⟩)^2≤C*p := by
  rintro ⟨C,hC⟩
  obtain ⟨N,hN⟩:=exists_nat_gt C
  obtain ⟨p,hp,hp20,hcounter⟩:=arbitrarily_large_collision_ratio N
  let : NeZero p:=⟨hp.ne_zero⟩
  have hh:=hC p hp hp20
  have hc : C*(p:ℝ)≤(N:ℝ)*p :=
    mul_le_mul_of_nonneg_right (le_of_lt hN) (by positivity)
  exact (not_lt_of_ge (hh.trans hc)) hcounter

#print axioms twenty_partition_certificate
#print axioms arbitrarily_large_collision_ratio
#print axioms universal_variance_only_collision_bound_false
end
end RhoCollisionCounterfamilyAudit
