import Common22SeparateChartMap
import Mathlib.AlgebraicGeometry.AffineScheme
import Mathlib.RingTheory.Ideal.Quotient.Basic

/-!
# A rational Weierstrass point in the actual affine chart

For a root `a` of the monic quintic, evaluation at `(a,0)` descends to the
affine coordinate ring.  The kernel defines a maximal ideal, hence an actual
point of its spectrum, and the standard spectrum-stalk equivalence identifies
its stalk with localization at that ideal.  The parameter, DVR, and divisor
orders are later obligations.
-/

set_option autoImplicit false
noncomputable section

namespace Common22BranchStalk

open MvPolynomial Common22SeparateChartMap AlgebraicGeometry CategoryTheory

variable {K : Type*} [Field K]

def fAt (a3 a2 a1 a0 a : K) : K :=
  a ^ 5 + a3 * a ^ 3 + a2 * a ^ 2 + a1 * a + a0

def preEval (a : K) : Poly K →+* K :=
  eval₂Hom (RingHom.id K) (fun i : Fin 2 => if i = 0 then a else 0)

theorem preEval_equation (a3 a2 a1 a0 a : K) :
    preEval a (affineEquation a3 a2 a1 a0) = -fAt a3 a2 a1 a0 a := by
  simp [preEval, affineEquation, fAt]

def branchEval (a3 a2 a1 a0 a : K) (hroot : fAt a3 a2 a1 a0 a = 0) :
    AffineRing a3 a2 a1 a0 →+* K :=
  Ideal.Quotient.lift (affineIdeal a3 a2 a1 a0) (preEval a) (by
    intro p hp
    have hle : affineIdeal a3 a2 a1 a0 ≤ RingHom.ker (preEval a) := by
      apply Ideal.span_le.mpr
      intro x hx
      simp only [Set.mem_singleton_iff] at hx
      subst x
      simp [RingHom.mem_ker, preEval_equation, hroot]
    exact hle hp)

theorem branchEval_surjective (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    Function.Surjective (branchEval a3 a2 a1 a0 a hroot) := by
  apply Ideal.Quotient.lift_surjective_of_surjective
  intro k
  use C k
  simp [preEval]

def branchIdeal (a3 a2 a1 a0 a : K) (hroot : fAt a3 a2 a1 a0 a = 0) :
    Ideal (AffineRing a3 a2 a1 a0) :=
  RingHom.ker (branchEval a3 a2 a1 a0 a hroot)

theorem branchEval_xbar (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    branchEval a3 a2 a1 a0 a hroot (xbar a3 a2 a1 a0) = a := by
  simp [branchEval, xbar, preEval]

theorem branchEval_ybar (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    branchEval a3 a2 a1 a0 a hroot (ybar a3 a2 a1 a0) = 0 := by
  simp [branchEval, ybar, preEval]

theorem ybar_mem_branchIdeal (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    ybar a3 a2 a1 a0 ∈ branchIdeal a3 a2 a1 a0 a hroot := by
  simpa [branchIdeal, RingHom.mem_ker] using
    branchEval_ybar a3 a2 a1 a0 a hroot

theorem xbar_sub_a_mem_branchIdeal (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    xbar a3 a2 a1 a0 -
      Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (C a) ∈
        branchIdeal a3 a2 a1 a0 a hroot := by
  simp [branchIdeal, RingHom.mem_ker, branchEval, xbar, preEval]

theorem branchIdeal_maximal (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    (branchIdeal a3 a2 a1 a0 a hroot).IsMaximal :=
  RingHom.ker_isMaximal_of_surjective _
    (branchEval_surjective a3 a2 a1 a0 a hroot)

instance branchIdeal_isPrime (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    (branchIdeal a3 a2 a1 a0 a hroot).IsPrime :=
  (branchIdeal_maximal a3 a2 a1 a0 a hroot).isPrime

def branchPoint (a3 a2 a1 a0 a : K) (hroot : fAt a3 a2 a1 a0 a = 0) :
    PrimeSpectrum (AffineRing a3 a2 a1 a0) :=
  ⟨branchIdeal a3 a2 a1 a0 a hroot, inferInstance⟩

def branchSpecStalkIso (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    (Spec (CommRingCat.of (AffineRing a3 a2 a1 a0))).presheaf.stalk
      (branchPoint a3 a2 a1 a0 a hroot) ≅
      ↧(Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
  Spec.stalkIso (CommRingCat.of (AffineRing a3 a2 a1 a0))
    (branchPoint a3 a2 a1 a0 a hroot)

#print axioms preEval_equation
#print axioms branchEval
#print axioms branchEval_surjective
#print axioms branchEval_xbar
#print axioms branchEval_ybar
#print axioms ybar_mem_branchIdeal
#print axioms xbar_sub_a_mem_branchIdeal
#print axioms branchIdeal_maximal
#print axioms branchIdeal_isPrime
#print axioms branchPoint
#print axioms branchSpecStalkIso

end Common22BranchStalk
