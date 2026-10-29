import Common22ExplicitChart
import Mathlib.Algebra.MvPolynomial.PDeriv
import Mathlib.RingTheory.Ideal.Operations
import Mathlib.Tactic.Ring

set_option autoImplicit false
noncomputable section
namespace WeightedInfinityJacobian
open MvPolynomial Common22ExplicitChart
variable {K : Type*} [Field K]

private def t (a3 a2 a1 a0 : K) : ChartPoly K :=
  C a3 * z + C a2 * z ^ 2 + C a1 * z ^ 3 + C a0 * z ^ 4

private theorem u_eq (a3 a2 a1 a0 : K) :
    unitFactor a3 a2 a1 a0 = 1 + z * t a3 a2 a1 a0 := by
  unfold unitFactor t
  ring

private theorem deriv_eq (a3 a2 a1 a0 : K) :
    (equation a3 a2 a1 a0).pderiv (0 : Fin 2) =
      -1 - z * (t a3 a2 a1 a0 + (unitFactor a3 a2 a1 a0).pderiv 0) := by
  have hu := u_eq a3 a2 a1 a0
  simp only [equation, z, w, map_sub, pderiv_pow, pderiv_mul, pderiv_X_self,
    pderiv_X_of_ne (show (1 : Fin 2) ≠ 0 by decide)]
  rw [hu]
  simp only [z]
  ring

/-- The z-partial of the infinity-chart equation in its coordinate ring. -/
def dZ (a3 a2 a1 a0 : K) : ChartRing a3 a2 a1 a0 :=
  Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0)
    ((equation a3 a2 a1 a0).pderiv (0 : Fin 2))

theorem infinity_jacobian_ideal_top (a3 a2 a1 a0 : K) :
    Ideal.span ({zbar a3 a2 a1 a0, dZ a3 a2 a1 a0} :
      Set (ChartRing a3 a2 a1 a0)) = ⊤ := by
  let I : Ideal (ChartRing a3 a2 a1 a0) :=
    Ideal.span ({zbar a3 a2 a1 a0, dZ a3 a2 a1 a0} : Set _)
  have hz : zbar a3 a2 a1 a0 ∈ I := Ideal.subset_span (by simp)
  have hd : dZ a3 a2 a1 a0 ∈ I := Ideal.subset_span (by simp)
  have hcert : dZ a3 a2 a1 a0 =
      -1 - zbar a3 a2 a1 a0 *
      Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0)
        (t a3 a2 a1 a0 + (unitFactor a3 a2 a1 a0).pderiv 0) := by
    simp only [dZ, deriv_eq, map_sub, map_neg, map_one, map_mul, zbar]
  have hm : zbar a3 a2 a1 a0 *
      Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0)
        (t a3 a2 a1 a0 + (unitFactor a3 a2 a1 a0).pderiv 0) ∈ I :=
    I.mul_mem_right _ hz
  have hneg : (-1 : ChartRing a3 a2 a1 a0) ∈ I := by
    convert I.add_mem hd hm using 1
    · rw [hcert]
      ring
  have h1 : (1 : ChartRing a3 a2 a1 a0) ∈ I := by simpa using I.neg_mem hneg
  change I = ⊤
  exact (Ideal.eq_top_iff_one I).mpr h1

#print axioms WeightedInfinityJacobian.infinity_jacobian_ideal_top
end WeightedInfinityJacobian
