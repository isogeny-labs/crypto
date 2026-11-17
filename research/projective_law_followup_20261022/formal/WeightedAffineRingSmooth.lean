import WeightedJacobianIdeal
import Mathlib.RingTheory.RingHom.Smooth
import Mathlib.RingTheory.Smooth.StandardSmoothCotangent
import Mathlib.RingTheory.Localization.Away.Basic
import Mathlib.RingTheory.Ideal.Quotient.Operations

set_option autoImplicit false
noncomputable section
namespace WeightedAffineRingSmooth
open MvPolynomial
variable {K : Type*} [Field K]

private abbrev ring (F : Common22SeparateChartMap.Poly K) := Common22SeparateChartMap.Poly K ⧸ Ideal.span ({F} : Set (Common22SeparateChartMap.Poly K))
private def rel (F : Common22SeparateChartMap.Poly K) : Unit → Common22SeparateChartMap.Poly K := fun _ => F
private theorem rel_ideal_eq (F : Common22SeparateChartMap.Poly K) :
    Ideal.span (Set.range (rel F)) = Ideal.span ({F} : Set (Common22SeparateChartMap.Poly K)) := by
  apply congrArg Ideal.span
  ext x
  simp [rel, eq_comm]
private def qequiv (F : Common22SeparateChartMap.Poly K) :
    (Common22SeparateChartMap.Poly K ⧸ Ideal.span (Set.range (rel F))) ≃ₐ[K] ring F :=
  Ideal.quotientEquivAlgOfEq K (rel_ideal_eq F)
private def naivePres (F : Common22SeparateChartMap.Poly K) (i : Fin 2) :
    Algebra.PreSubmersivePresentation K
      (Common22SeparateChartMap.Poly K ⧸ Ideal.span (Set.range (rel F))) (Fin 2) Unit :=
  Algebra.PreSubmersivePresentation.naive (v := rel F)
    (fun _ : Unit => i) (by intro x y _; exact Subsingleton.elim x y)
private def pres (F : Common22SeparateChartMap.Poly K) (i : Fin 2) :
    Algebra.PreSubmersivePresentation K (ring F) (Fin 2) Unit :=
  (naivePres F i).ofAlgEquiv (qequiv F)
private def deriv (F : Common22SeparateChartMap.Poly K) (i : Fin 2) : ring F :=
  Ideal.Quotient.mk (Ideal.span ({F} : Set (Common22SeparateChartMap.Poly K))) (F.pderiv i)
private theorem pres_jacobian_eq (F : Common22SeparateChartMap.Poly K) (i : Fin 2) :
    (pres F i).jacobian = deriv F i := by
  unfold pres
  rw [Algebra.PreSubmersivePresentation.jacobian_ofAlgEquiv,
    Algebra.PreSubmersivePresentation.jacobian_eq_jacobiMatrix_det,
    Matrix.det_unique, Algebra.PreSubmersivePresentation.jacobiMatrix_apply]
  change (qequiv F)
    (Ideal.Quotient.mk (Ideal.span (Set.range (rel F))) (F.pderiv i)) = _
  exact Ideal.quotientEquivAlgOfEq_mk K (rel_ideal_eq F) _
private def localPres (F : Common22SeparateChartMap.Poly K) (i : Fin 2) :
    Algebra.PreSubmersivePresentation K (Localization.Away (deriv F i))
      (Unit ⊕ Fin 2) (Unit ⊕ Unit) :=
  (Algebra.PreSubmersivePresentation.localizationAway
    (S := Localization.Away (deriv F i)) (deriv F i)).comp (pres F i)
private theorem localPres_jacobian_isUnit (F : Common22SeparateChartMap.Poly K) (i : Fin 2) :
    IsUnit (localPres F i).jacobian := by
  unfold localPres
  rw [Algebra.PreSubmersivePresentation.comp_jacobian_eq_jacobian_smul_jacobian,
    pres_jacobian_eq,
    Algebra.PreSubmersivePresentation.localizationAway_jacobian]
  convert (IsLocalization.Away.algebraMap_isUnit (deriv F i)).mul
    (IsLocalization.Away.algebraMap_isUnit (deriv F i)) using 1
  · exact Algebra.smul_def (R := ring F) (A := Localization.Away (deriv F i)) _ _
  · infer_instance
  · infer_instance
private def localSub (F : Common22SeparateChartMap.Poly K) (i : Fin 2) :
    Algebra.SubmersivePresentation K (Localization.Away (deriv F i))
      (Unit ⊕ Fin 2) (Unit ⊕ Unit) where
  toPreSubmersivePresentation := localPres F i
  jacobian_isUnit := localPres_jacobian_isUnit F i

theorem derivative_open_ring_smooth (F : Common22SeparateChartMap.Poly K) (i : Fin 2) :
    RingHom.Smooth (algebraMap K (Localization.Away (deriv F i))) := by
  haveI : Algebra.IsStandardSmooth K (Localization.Away (deriv F i)) :=
    (localSub F i).isStandardSmooth
  exact (RingHom.smooth_algebraMap).mpr inferInstance

theorem ring_smooth_of_jacobian_span_top (F : Common22SeparateChartMap.Poly K)
    (htop : Ideal.span (Set.range (deriv F)) = ⊤) :
    RingHom.Smooth (algebraMap K (ring F)) := by
  apply RingHom.Smooth.ofLocalizationSpanTarget
    (algebraMap K (ring F)) (Set.range (deriv F)) htop
  intro r
  obtain ⟨i, hi⟩ := r.property
  change RingHom.Smooth (algebraMap K (Localization.Away (r : ring F)))
  rw [← hi]
  exact derivative_open_ring_smooth F i

private theorem mk_mul_two (I : Ideal (Common22SeparateChartMap.Poly K))
    (x : Common22SeparateChartMap.Poly K) :
    Ideal.Quotient.mk I (x * 2) = (2 : Common22SeparateChartMap.Poly K ⧸ I) *
      Ideal.Quotient.mk I x := by
  rw [map_mul, map_ofNat]
  ring

private theorem deriv_one_eq (a3 a2 a1 a0 : K) :
    deriv (Common22SeparateChartMap.affineEquation a3 a2 a1 a0) 1 =
      (2 : Common22SeparateChartMap.AffineRing a3 a2 a1 a0) *
        Common22SeparateChartMap.ybar a3 a2 a1 a0 := by
  simp [deriv, Common22SeparateChartMap.affineEquation,
    Common22SeparateChartMap.ybar, Common22SeparateChartMap.affineIdeal, map_sub, map_add, map_mul, map_pow,
    MvPolynomial.pderiv_pow, MvPolynomial.pderiv_mul]
  rw [show (2 : Common22SeparateChartMap.Poly K) * X 1 = X 1 * 2 by ring]
  exact mk_mul_two (Common22SeparateChartMap.affineIdeal a3 a2 a1 a0) (X 1)

#print axioms WeightedAffineRingSmooth.derivative_open_ring_smooth
#print axioms WeightedAffineRingSmooth.ring_smooth_of_jacobian_span_top
end WeightedAffineRingSmooth
