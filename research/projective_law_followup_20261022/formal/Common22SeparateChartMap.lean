import Common22ExplicitChart
import Mathlib.RingTheory.Localization.Away.Basic

/-! First explicit presentations for the two separately localized curve chart
coordinate rings. The intended map sends affine x to inverse infinity z and
affine y to infinity w times inverse z cubed. -/

set_option autoImplicit false

noncomputable section

namespace Common22SeparateChartMap

open MvPolynomial

variable {K : Type*} [Field K]

abbrev Poly (K : Type*) [CommRing K] := MvPolynomial (Fin 2) K

def affineEquation (a3 a2 a1 a0 : K) : Poly K :=
  X 1 ^ 2 - (X 0 ^ 5 + C a3 * X 0 ^ 3 + C a2 * X 0 ^ 2 + C a1 * X 0 + C a0)

def affineIdeal (a3 a2 a1 a0 : K) : Ideal (Poly K) :=
  Ideal.span {affineEquation a3 a2 a1 a0}

abbrev AffineRing (a3 a2 a1 a0 : K) :=
  Poly K ⧸ affineIdeal a3 a2 a1 a0

def xbar (a3 a2 a1 a0 : K) : AffineRing a3 a2 a1 a0 :=
  Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (X 0)

def ybar (a3 a2 a1 a0 : K) : AffineRing a3 a2 a1 a0 :=
  Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (X 1)

abbrev AffineOpen (a3 a2 a1 a0 : K) :=
  Localization.Away (xbar a3 a2 a1 a0)

abbrev InfinityOpen (a3 a2 a1 a0 : K) :=
  Localization.Away (Common22ExplicitChart.zbar a3 a2 a1 a0)

def infinityZ (a3 a2 a1 a0 : K) : InfinityOpen a3 a2 a1 a0 :=
  algebraMap (Common22ExplicitChart.ChartRing a3 a2 a1 a0)
    (InfinityOpen a3 a2 a1 a0) (Common22ExplicitChart.zbar a3 a2 a1 a0)

def infinityW (a3 a2 a1 a0 : K) : InfinityOpen a3 a2 a1 a0 :=
  algebraMap (Common22ExplicitChart.ChartRing a3 a2 a1 a0)
    (InfinityOpen a3 a2 a1 a0) (Common22ExplicitChart.wbar a3 a2 a1 a0)

def infinityInvZ (a3 a2 a1 a0 : K) : InfinityOpen a3 a2 a1 a0 :=
  IsLocalization.Away.invSelf (S := InfinityOpen a3 a2 a1 a0)
    (Common22ExplicitChart.zbar a3 a2 a1 a0)

theorem infinityInvZ_mul_z (a3 a2 a1 a0 : K) :
    infinityInvZ a3 a2 a1 a0 * infinityZ a3 a2 a1 a0 = 1 := by
  calc
    infinityInvZ a3 a2 a1 a0 * infinityZ a3 a2 a1 a0 =
      infinityZ a3 a2 a1 a0 * infinityInvZ a3 a2 a1 a0 := mul_comm _ _
    _ = 1 := IsLocalization.Away.mul_invSelf _

/-- The proposed substitution before imposing the affine equation. -/
def affinePolynomialToInfinityOpen (a3 a2 a1 a0 : K) :
    Poly K →ₐ[K] InfinityOpen a3 a2 a1 a0 :=
  MvPolynomial.aeval (fun i : Fin 2 =>
    if i = 0 then infinityInvZ a3 a2 a1 a0
    else infinityW a3 a2 a1 a0 * infinityInvZ a3 a2 a1 a0 ^ 3)

theorem affinePolynomialToInfinityOpen_X0 (a3 a2 a1 a0 : K) :
    affinePolynomialToInfinityOpen a3 a2 a1 a0 (X 0) =
      infinityInvZ a3 a2 a1 a0 := by
  simp [affinePolynomialToInfinityOpen]

theorem affinePolynomialToInfinityOpen_X1 (a3 a2 a1 a0 : K) :
    affinePolynomialToInfinityOpen a3 a2 a1 a0 (X 1) =
      infinityW a3 a2 a1 a0 * infinityInvZ a3 a2 a1 a0 ^ 3 := by
  simp [affinePolynomialToInfinityOpen]

theorem infinity_ubar_expansion (a3 a2 a1 a0 : K) :
    Common22ExplicitChart.ubar a3 a2 a1 a0 =
      1 + (algebraMap K (Common22ExplicitChart.ChartRing a3 a2 a1 a0)) a3 *
          Common22ExplicitChart.zbar a3 a2 a1 a0 ^ 2 +
        (algebraMap K (Common22ExplicitChart.ChartRing a3 a2 a1 a0)) a2 *
          Common22ExplicitChart.zbar a3 a2 a1 a0 ^ 3 +
        (algebraMap K (Common22ExplicitChart.ChartRing a3 a2 a1 a0)) a1 *
          Common22ExplicitChart.zbar a3 a2 a1 a0 ^ 4 +
        (algebraMap K (Common22ExplicitChart.ChartRing a3 a2 a1 a0)) a0 *
          Common22ExplicitChart.zbar a3 a2 a1 a0 ^ 5 := by
  have hC (a : K) :
      (Ideal.Quotient.mk (Common22ExplicitChart.relationIdeal a3 a2 a1 a0)) (C a) =
        (algebraMap K (Common22ExplicitChart.ChartRing a3 a2 a1 a0)) a := by
    change (Ideal.Quotient.mk (Common22ExplicitChart.relationIdeal a3 a2 a1 a0))
      (algebraMap K (Poly K) a) = _
    rfl
  simp [Common22ExplicitChart.ubar, Common22ExplicitChart.unitFactor,
    Common22ExplicitChart.zbar, hC]

theorem infinity_open_relation (a3 a2 a1 a0 : K) :
    infinityW a3 a2 a1 a0 ^ 2 = infinityZ a3 a2 a1 a0 *
      (1 + (algebraMap K (InfinityOpen a3 a2 a1 a0)) a3 *
          infinityZ a3 a2 a1 a0 ^ 2 +
        (algebraMap K (InfinityOpen a3 a2 a1 a0)) a2 *
          infinityZ a3 a2 a1 a0 ^ 3 +
        (algebraMap K (InfinityOpen a3 a2 a1 a0)) a1 *
          infinityZ a3 a2 a1 a0 ^ 4 +
        (algebraMap K (InfinityOpen a3 a2 a1 a0)) a0 *
          infinityZ a3 a2 a1 a0 ^ 5) := by
  have h := congrArg
    (algebraMap (Common22ExplicitChart.ChartRing a3 a2 a1 a0)
      (InfinityOpen a3 a2 a1 a0))
    (Common22ExplicitChart.quotient_chart_relation a3 a2 a1 a0)
  rw [infinity_ubar_expansion] at h
  simpa [infinityW, infinityZ, ← IsScalarTower.algebraMap_apply] using h

#print axioms infinityInvZ_mul_z
#print axioms affinePolynomialToInfinityOpen_X0
#print axioms affinePolynomialToInfinityOpen_X1
#print axioms infinity_ubar_expansion
#print axioms infinity_open_relation

end Common22SeparateChartMap
