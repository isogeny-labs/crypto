import Common22ExplicitChart
import Mathlib.RingTheory.Spectrum.Prime.Topology

/-!
# One point at infinity in the explicit chart

This proves a concrete geometric property needed to compare the checked
two-chart gluing with a projective completion. It does not identify the glued
scheme with a Proj construction or establish properness or smoothness.
-/

set_option autoImplicit false
noncomputable section

namespace Common22CompletionBridge

open Common22ExplicitChart

variable {K : Type*} [Field K]

def infinityOrigin (a3 a2 a1 a0 : K) :
    PrimeSpectrum (ChartRing a3 a2 a1 a0) :=
  ⟨originIdeal a3 a2 a1 a0, originIdeal_prime a3 a2 a1 a0⟩

theorem prime_eq_origin_of_z_mem (a3 a2 a1 a0 : K)
    (p : PrimeSpectrum (ChartRing a3 a2 a1 a0))
    (hz : zbar a3 a2 a1 a0 ∈ p.asIdeal) :
    p = infinityOrigin a3 a2 a1 a0 := by
  have hw2 : wbar a3 a2 a1 a0 ^ 2 ∈ p.asIdeal := by
    rw [quotient_chart_relation]
    exact p.asIdeal.mul_mem_right _ hz
  have hw : wbar a3 a2 a1 a0 ∈ p.asIdeal :=
    p.isPrime.mem_of_pow_mem 2 hw2
  have hle : originIdeal a3 a2 a1 a0 ≤ p.asIdeal := by
    rw [originIdeal_eq_chartCoordinateIdeal]
    apply Ideal.span_le.mpr
    intro x hx
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hx
    rcases hx with rfl | rfl
    · exact hz
    · exact hw
  apply PrimeSpectrum.ext
  exact (origin_ideal_maximal a3 a2 a1 a0).eq_of_le p.isPrime.ne_top hle |>.symm

theorem infinity_chart_complement_unique (a3 a2 a1 a0 : K)
    (p : PrimeSpectrum (ChartRing a3 a2 a1 a0))
    (hp : p ∉ PrimeSpectrum.basicOpen (zbar a3 a2 a1 a0)) :
    p = infinityOrigin a3 a2 a1 a0 := by
  apply prime_eq_origin_of_z_mem a3 a2 a1 a0 p
  exact Classical.not_not.mp ((PrimeSpectrum.mem_basicOpen _ _).not.mp hp)

theorem infinity_origin_outside_overlap (a3 a2 a1 a0 : K) :
    infinityOrigin a3 a2 a1 a0 ∉
      PrimeSpectrum.basicOpen (zbar a3 a2 a1 a0) := by
  rw [PrimeSpectrum.mem_basicOpen]
  simp [infinityOrigin, originIdeal, RingHom.mem_ker,
    Common22ExplicitChart.zbar, Common22ExplicitChart.originHom,
    Common22ExplicitChart.originEval, Common22ExplicitChart.z]

/-- The closed complement of the principal overlap in the explicit infinity
chart is a singleton, with its point given by the actual origin ideal. -/
theorem unique_infinity_point (a3 a2 a1 a0 : K) :
    ∃! p : PrimeSpectrum (ChartRing a3 a2 a1 a0),
      p ∉ PrimeSpectrum.basicOpen (zbar a3 a2 a1 a0) := by
  refine ⟨infinityOrigin a3 a2 a1 a0,
    infinity_origin_outside_overlap a3 a2 a1 a0, ?_⟩
  intro p hp
  exact infinity_chart_complement_unique a3 a2 a1 a0 p hp

#print axioms prime_eq_origin_of_z_mem
#print axioms infinity_chart_complement_unique
#print axioms infinity_origin_outside_overlap
#print axioms unique_infinity_point

end Common22CompletionBridge
