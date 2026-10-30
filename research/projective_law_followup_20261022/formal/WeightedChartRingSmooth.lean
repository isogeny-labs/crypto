import WeightedInfinityJacobian
import Mathlib.RingTheory.Smooth.StandardSmooth
import Mathlib.RingTheory.Localization.Away.Basic
import Mathlib.RingTheory.RingHom.Smooth
import Mathlib.RingTheory.Smooth.StandardSmoothCotangent
import Mathlib.RingTheory.Ideal.Quotient.Operations

set_option autoImplicit false
noncomputable section
namespace WeightedChartRingSmooth
open MvPolynomial Common22ExplicitChart
variable {K : Type*} [Field K]

private def rel (a3 a2 a1 a0 : K) : Unit → ChartPoly K :=
  fun _ => equation a3 a2 a1 a0

private theorem rel_ideal_eq (a3 a2 a1 a0 : K) :
    Ideal.span (Set.range (rel a3 a2 a1 a0)) =
      relationIdeal a3 a2 a1 a0 := by
  unfold relationIdeal
  apply congrArg Ideal.span
  ext x
  simp [rel, eq_comm]

private def qequiv (a3 a2 a1 a0 : K) :
    (ChartPoly K ⧸ Ideal.span (Set.range (rel a3 a2 a1 a0))) ≃ₐ[K]
      ChartRing a3 a2 a1 a0 :=
  Ideal.quotientEquivAlgOfEq K (rel_ideal_eq a3 a2 a1 a0)

private def naivePres (a3 a2 a1 a0 : K) :
    Algebra.PreSubmersivePresentation K
      (ChartPoly K ⧸ Ideal.span (Set.range (rel a3 a2 a1 a0))) (Fin 2) Unit :=
  Algebra.PreSubmersivePresentation.naive
    (v := rel a3 a2 a1 a0) (fun _ : Unit => (0 : Fin 2))
    (by intro x y _; exact Subsingleton.elim x y)

private def pres (a3 a2 a1 a0 : K) :
    Algebra.PreSubmersivePresentation K (ChartRing a3 a2 a1 a0) (Fin 2) Unit :=
  (naivePres a3 a2 a1 a0).ofAlgEquiv (qequiv a3 a2 a1 a0)

private theorem pres_jacobian_eq (a3 a2 a1 a0 : K) :
    (pres a3 a2 a1 a0).jacobian =
      Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0)
        ((equation a3 a2 a1 a0).pderiv (0 : Fin 2)) := by
  unfold pres
  rw [Algebra.PreSubmersivePresentation.jacobian_ofAlgEquiv,
    Algebra.PreSubmersivePresentation.jacobian_eq_jacobiMatrix_det,
    Matrix.det_unique]
  rw [Algebra.PreSubmersivePresentation.jacobiMatrix_apply]
  change (qequiv a3 a2 a1 a0)
    (Ideal.Quotient.mk (Ideal.span (Set.range (rel a3 a2 a1 a0)))
      ((equation a3 a2 a1 a0).pderiv (0 : Fin 2))) = _
  exact Ideal.quotientEquivAlgOfEq_mk K (rel_ideal_eq a3 a2 a1 a0) _


private def localPres (a3 a2 a1 a0 : K) :
    Algebra.PreSubmersivePresentation K
      (Localization.Away (WeightedInfinityJacobian.dZ a3 a2 a1 a0))
      (Unit ⊕ Fin 2) (Unit ⊕ Unit) :=
  (Algebra.PreSubmersivePresentation.localizationAway
    (S := Localization.Away (WeightedInfinityJacobian.dZ a3 a2 a1 a0))
    (WeightedInfinityJacobian.dZ a3 a2 a1 a0)).comp
    (pres a3 a2 a1 a0)

private theorem localPres_jacobian_isUnit (a3 a2 a1 a0 : K) :
    IsUnit (localPres a3 a2 a1 a0).jacobian := by
  unfold localPres
  rw [Algebra.PreSubmersivePresentation.comp_jacobian_eq_jacobian_smul_jacobian,
    pres_jacobian_eq,
    Algebra.PreSubmersivePresentation.localizationAway_jacobian]
  change IsUnit (WeightedInfinityJacobian.dZ a3 a2 a1 a0 •
    (algebraMap (ChartRing a3 a2 a1 a0)
    (Localization.Away (WeightedInfinityJacobian.dZ a3 a2 a1 a0)))
      (WeightedInfinityJacobian.dZ a3 a2 a1 a0))
  convert (IsLocalization.Away.algebraMap_isUnit
      (WeightedInfinityJacobian.dZ a3 a2 a1 a0)).mul
    (IsLocalization.Away.algebraMap_isUnit
      (WeightedInfinityJacobian.dZ a3 a2 a1 a0)) using 1
  · exact Algebra.smul_def
      (R := ChartRing a3 a2 a1 a0)
      (A := Localization.Away (WeightedInfinityJacobian.dZ a3 a2 a1 a0)) _ _
  · infer_instance
  · infer_instance

private def localSub (a3 a2 a1 a0 : K) :
    Algebra.SubmersivePresentation K
      (Localization.Away (WeightedInfinityJacobian.dZ a3 a2 a1 a0))
      (Unit ⊕ Fin 2) (Unit ⊕ Unit) where
  toPreSubmersivePresentation := localPres a3 a2 a1 a0
  jacobian_isUnit := localPres_jacobian_isUnit a3 a2 a1 a0

theorem infinity_derivative_open_standard_smooth (a3 a2 a1 a0 : K) :
    Algebra.IsStandardSmooth K
      (Localization.Away (WeightedInfinityJacobian.dZ a3 a2 a1 a0)) :=
  (localSub a3 a2 a1 a0).isStandardSmooth

theorem infinity_derivative_open_ring_smooth (a3 a2 a1 a0 : K) :
    RingHom.Smooth
      (algebraMap K (Localization.Away (WeightedInfinityJacobian.dZ a3 a2 a1 a0))) :=
  (RingHom.smooth_algebraMap).mpr (by
    haveI : Algebra.IsStandardSmooth K
      (Localization.Away (WeightedInfinityJacobian.dZ a3 a2 a1 a0)) :=
      infinity_derivative_open_standard_smooth a3 a2 a1 a0
    infer_instance)

#print axioms WeightedChartRingSmooth.pres_jacobian_eq
#print axioms WeightedChartRingSmooth.localPres_jacobian_isUnit
#print axioms WeightedChartRingSmooth.infinity_derivative_open_ring_smooth
end WeightedChartRingSmooth
