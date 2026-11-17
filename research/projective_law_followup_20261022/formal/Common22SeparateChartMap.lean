import Common22ExplicitChart
import Common22PrincipalOpenEquation
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

theorem affine_relation_at_infinity (a3 a2 a1 a0 : K) :
    (infinityW a3 a2 a1 a0 * infinityInvZ a3 a2 a1 a0 ^ 3) ^ 2 =
      infinityInvZ a3 a2 a1 a0 ^ 5 +
      (algebraMap K (InfinityOpen a3 a2 a1 a0)) a3 *
        infinityInvZ a3 a2 a1 a0 ^ 3 +
      (algebraMap K (InfinityOpen a3 a2 a1 a0)) a2 *
        infinityInvZ a3 a2 a1 a0 ^ 2 +
      (algebraMap K (InfinityOpen a3 a2 a1 a0)) a1 *
        infinityInvZ a3 a2 a1 a0 +
      (algebraMap K (InfinityOpen a3 a2 a1 a0)) a0 := by
  have htz := infinityInvZ_mul_z a3 a2 a1 a0
  have hround :
      (infinityW a3 a2 a1 a0 * infinityInvZ a3 a2 a1 a0 ^ 3) *
          infinityZ a3 a2 a1 a0 ^ 3 = infinityW a3 a2 a1 a0 := by
    calc
      _ = infinityW a3 a2 a1 a0 *
          (infinityInvZ a3 a2 a1 a0 * infinityZ a3 a2 a1 a0) ^ 3 := by ring
      _ = infinityW a3 a2 a1 a0 := by rw [htz]; simp
  apply (Common22PrincipalOpenEquation.equation_iff_on_principal_open
    ((algebraMap K (InfinityOpen a3 a2 a1 a0)) a3)
    ((algebraMap K (InfinityOpen a3 a2 a1 a0)) a2)
    ((algebraMap K (InfinityOpen a3 a2 a1 a0)) a1)
    ((algebraMap K (InfinityOpen a3 a2 a1 a0)) a0)
    (infinityInvZ a3 a2 a1 a0)
    (infinityW a3 a2 a1 a0 * infinityInvZ a3 a2 a1 a0 ^ 3)
    (infinityZ a3 a2 a1 a0) htz).mpr
  simpa only [hround] using infinity_open_relation a3 a2 a1 a0

theorem affineEquation_maps_zero (a3 a2 a1 a0 : K) :
    affinePolynomialToInfinityOpen a3 a2 a1 a0
      (affineEquation a3 a2 a1 a0) = 0 := by
  have h := affine_relation_at_infinity a3 a2 a1 a0
  simpa [affineEquation, affinePolynomialToInfinityOpen, sub_eq_zero] using h

theorem affineIdeal_maps_zero (a3 a2 a1 a0 : K) :
    ∀ p : Poly K, p ∈ affineIdeal a3 a2 a1 a0 →
      affinePolynomialToInfinityOpen a3 a2 a1 a0 p = 0 := by
  have hgen : affineEquation a3 a2 a1 a0 ∈
      RingHom.ker (affinePolynomialToInfinityOpen a3 a2 a1 a0).toRingHom := by
    rw [RingHom.mem_ker]
    exact affineEquation_maps_zero a3 a2 a1 a0
  have hle : affineIdeal a3 a2 a1 a0 ≤
      RingHom.ker (affinePolynomialToInfinityOpen a3 a2 a1 a0).toRingHom := by
    apply Ideal.span_le.mpr
    intro p hp
    have hpeq : p = affineEquation a3 a2 a1 a0 := by simpa using hp
    subst p
    exact hgen
  intro p hp
  exact (RingHom.mem_ker).mp (hle hp)

/-- The candidate map has descended through the affine defining equation. -/
def affineRingToInfinityOpen (a3 a2 a1 a0 : K) :
    AffineRing a3 a2 a1 a0 →ₐ[K] InfinityOpen a3 a2 a1 a0 :=
  Ideal.Quotient.liftₐ (affineIdeal a3 a2 a1 a0)
    (affinePolynomialToInfinityOpen a3 a2 a1 a0)
    (affineIdeal_maps_zero a3 a2 a1 a0)

theorem affineRingToInfinityOpen_xbar (a3 a2 a1 a0 : K) :
    affineRingToInfinityOpen a3 a2 a1 a0 (xbar a3 a2 a1 a0) =
      infinityInvZ a3 a2 a1 a0 := by
  change affinePolynomialToInfinityOpen a3 a2 a1 a0 (X 0) = _
  exact affinePolynomialToInfinityOpen_X0 a3 a2 a1 a0

theorem affineRingToInfinityOpen_ybar (a3 a2 a1 a0 : K) :
    affineRingToInfinityOpen a3 a2 a1 a0 (ybar a3 a2 a1 a0) =
      infinityW a3 a2 a1 a0 * infinityInvZ a3 a2 a1 a0 ^ 3 := by
  change affinePolynomialToInfinityOpen a3 a2 a1 a0 (X 1) = _
  exact affinePolynomialToInfinityOpen_X1 a3 a2 a1 a0

theorem affineRingToInfinityOpen_xbar_isUnit (a3 a2 a1 a0 : K) :
    IsUnit (affineRingToInfinityOpen a3 a2 a1 a0 (xbar a3 a2 a1 a0)) := by
  rw [affineRingToInfinityOpen_xbar]
  exact isUnit_iff_dvd_one.mpr
    ⟨infinityZ a3 a2 a1 a0, (infinityInvZ_mul_z a3 a2 a1 a0).symm⟩

/-- The checked candidate map from the localized affine coordinate ring. -/
def affineOpenToInfinityOpen (a3 a2 a1 a0 : K) :
    AffineOpen a3 a2 a1 a0 →ₐ[K] InfinityOpen a3 a2 a1 a0 :=
  IsLocalization.Away.liftAlgHom (S := AffineOpen a3 a2 a1 a0)
    (f := affineRingToInfinityOpen a3 a2 a1 a0)
    (xbar a3 a2 a1 a0)
    (affineRingToInfinityOpen_xbar_isUnit a3 a2 a1 a0)

def affineX (a3 a2 a1 a0 : K) : AffineOpen a3 a2 a1 a0 :=
  algebraMap (AffineRing a3 a2 a1 a0) (AffineOpen a3 a2 a1 a0)
    (xbar a3 a2 a1 a0)

def affineY (a3 a2 a1 a0 : K) : AffineOpen a3 a2 a1 a0 :=
  algebraMap (AffineRing a3 a2 a1 a0) (AffineOpen a3 a2 a1 a0)
    (ybar a3 a2 a1 a0)

def affineInvX (a3 a2 a1 a0 : K) : AffineOpen a3 a2 a1 a0 :=
  IsLocalization.Away.invSelf (S := AffineOpen a3 a2 a1 a0)
    (xbar a3 a2 a1 a0)

theorem affineOpenToInfinityOpen_affineX (a3 a2 a1 a0 : K) :
    affineOpenToInfinityOpen a3 a2 a1 a0 (affineX a3 a2 a1 a0) =
      infinityInvZ a3 a2 a1 a0 := by
  simpa [affineOpenToInfinityOpen, affineX] using
    affineRingToInfinityOpen_xbar a3 a2 a1 a0

theorem affineOpenToInfinityOpen_affineY (a3 a2 a1 a0 : K) :
    affineOpenToInfinityOpen a3 a2 a1 a0 (affineY a3 a2 a1 a0) =
      infinityW a3 a2 a1 a0 * infinityInvZ a3 a2 a1 a0 ^ 3 := by
  simpa [affineOpenToInfinityOpen, affineY] using
    affineRingToInfinityOpen_ybar a3 a2 a1 a0

theorem affineX_mul_invX (a3 a2 a1 a0 : K) :
    affineX a3 a2 a1 a0 * affineInvX a3 a2 a1 a0 = 1 :=
  IsLocalization.Away.mul_invSelf (xbar a3 a2 a1 a0)

theorem quotient_affine_relation (a3 a2 a1 a0 : K) :
    ybar a3 a2 a1 a0 ^ 2 =
      xbar a3 a2 a1 a0 ^ 5 +
        (algebraMap K (AffineRing a3 a2 a1 a0)) a3 * xbar a3 a2 a1 a0 ^ 3 +
        (algebraMap K (AffineRing a3 a2 a1 a0)) a2 * xbar a3 a2 a1 a0 ^ 2 +
        (algebraMap K (AffineRing a3 a2 a1 a0)) a1 * xbar a3 a2 a1 a0 +
        (algebraMap K (AffineRing a3 a2 a1 a0)) a0 := by
  have hmem : affineEquation a3 a2 a1 a0 ∈ affineIdeal a3 a2 a1 a0 :=
    Ideal.subset_span (by simp)
  have hzero := (Ideal.Quotient.eq_zero_iff_mem).mpr hmem
  simpa [affineEquation, xbar, ybar, affineIdeal] using sub_eq_zero.mp hzero

theorem affine_open_relation (a3 a2 a1 a0 : K) :
    affineY a3 a2 a1 a0 ^ 2 =
      affineX a3 a2 a1 a0 ^ 5 +
        (algebraMap K (AffineOpen a3 a2 a1 a0)) a3 * affineX a3 a2 a1 a0 ^ 3 +
        (algebraMap K (AffineOpen a3 a2 a1 a0)) a2 * affineX a3 a2 a1 a0 ^ 2 +
        (algebraMap K (AffineOpen a3 a2 a1 a0)) a1 * affineX a3 a2 a1 a0 +
        (algebraMap K (AffineOpen a3 a2 a1 a0)) a0 := by
  have h := congrArg
    (algebraMap (AffineRing a3 a2 a1 a0) (AffineOpen a3 a2 a1 a0))
    (quotient_affine_relation a3 a2 a1 a0)
  simpa [affineX, affineY, ← IsScalarTower.algebraMap_apply] using h

theorem chart_relation_at_affine (a3 a2 a1 a0 : K) :
    (affineY a3 a2 a1 a0 * affineInvX a3 a2 a1 a0 ^ 3) ^ 2 =
      affineInvX a3 a2 a1 a0 *
        (1 + (algebraMap K (AffineOpen a3 a2 a1 a0)) a3 *
            affineInvX a3 a2 a1 a0 ^ 2 +
          (algebraMap K (AffineOpen a3 a2 a1 a0)) a2 *
            affineInvX a3 a2 a1 a0 ^ 3 +
          (algebraMap K (AffineOpen a3 a2 a1 a0)) a1 *
            affineInvX a3 a2 a1 a0 ^ 4 +
          (algebraMap K (AffineOpen a3 a2 a1 a0)) a0 *
            affineInvX a3 a2 a1 a0 ^ 5) := by
  exact (Common22PrincipalOpenEquation.equation_iff_on_principal_open
    ((algebraMap K (AffineOpen a3 a2 a1 a0)) a3)
    ((algebraMap K (AffineOpen a3 a2 a1 a0)) a2)
    ((algebraMap K (AffineOpen a3 a2 a1 a0)) a1)
    ((algebraMap K (AffineOpen a3 a2 a1 a0)) a0)
    (affineX a3 a2 a1 a0) (affineY a3 a2 a1 a0)
    (affineInvX a3 a2 a1 a0)
    (affineX_mul_invX a3 a2 a1 a0)).mp
    (affine_open_relation a3 a2 a1 a0)

def infinityPolynomialToAffineOpen (a3 a2 a1 a0 : K) :
    Common22ExplicitChart.ChartPoly K →ₐ[K] AffineOpen a3 a2 a1 a0 :=
  MvPolynomial.aeval (fun i : Fin 2 =>
    if i = 0 then affineInvX a3 a2 a1 a0
    else affineY a3 a2 a1 a0 * affineInvX a3 a2 a1 a0 ^ 3)

theorem infinityEquation_maps_zero (a3 a2 a1 a0 : K) :
    infinityPolynomialToAffineOpen a3 a2 a1 a0
      (Common22ExplicitChart.equation a3 a2 a1 a0) = 0 := by
  have h := chart_relation_at_affine a3 a2 a1 a0
  simpa [infinityPolynomialToAffineOpen, Common22ExplicitChart.equation,
    Common22ExplicitChart.unitFactor, Common22ExplicitChart.z,
    Common22ExplicitChart.w, sub_eq_zero] using h

theorem infinityIdeal_maps_zero (a3 a2 a1 a0 : K) :
    ∀ p : Common22ExplicitChart.ChartPoly K,
      p ∈ Common22ExplicitChart.relationIdeal a3 a2 a1 a0 →
      infinityPolynomialToAffineOpen a3 a2 a1 a0 p = 0 := by
  have hgen : Common22ExplicitChart.equation a3 a2 a1 a0 ∈
      RingHom.ker (infinityPolynomialToAffineOpen a3 a2 a1 a0).toRingHom := by
    rw [RingHom.mem_ker]
    exact infinityEquation_maps_zero a3 a2 a1 a0
  have hle : Common22ExplicitChart.relationIdeal a3 a2 a1 a0 ≤
      RingHom.ker (infinityPolynomialToAffineOpen a3 a2 a1 a0).toRingHom := by
    apply Ideal.span_le.mpr
    intro p hp
    have hpeq : p = Common22ExplicitChart.equation a3 a2 a1 a0 := by simpa using hp
    subst p
    exact hgen
  intro p hp
  exact (RingHom.mem_ker).mp (hle hp)

def infinityRingToAffineOpen (a3 a2 a1 a0 : K) :
    Common22ExplicitChart.ChartRing a3 a2 a1 a0 →ₐ[K] AffineOpen a3 a2 a1 a0 :=
  Ideal.Quotient.liftₐ (Common22ExplicitChart.relationIdeal a3 a2 a1 a0)
    (infinityPolynomialToAffineOpen a3 a2 a1 a0)
    (infinityIdeal_maps_zero a3 a2 a1 a0)

theorem infinityRingToAffineOpen_zbar (a3 a2 a1 a0 : K) :
    infinityRingToAffineOpen a3 a2 a1 a0
      (Common22ExplicitChart.zbar a3 a2 a1 a0) = affineInvX a3 a2 a1 a0 := by
  change infinityPolynomialToAffineOpen a3 a2 a1 a0 (X 0) = _
  simp [infinityPolynomialToAffineOpen]

theorem infinityRingToAffineOpen_wbar (a3 a2 a1 a0 : K) :
    infinityRingToAffineOpen a3 a2 a1 a0
      (Common22ExplicitChart.wbar a3 a2 a1 a0) =
        affineY a3 a2 a1 a0 * affineInvX a3 a2 a1 a0 ^ 3 := by
  change infinityPolynomialToAffineOpen a3 a2 a1 a0 (X 1) = _
  simp [infinityPolynomialToAffineOpen]

theorem infinityRingToAffineOpen_zbar_isUnit (a3 a2 a1 a0 : K) :
    IsUnit (infinityRingToAffineOpen a3 a2 a1 a0
      (Common22ExplicitChart.zbar a3 a2 a1 a0)) := by
  rw [infinityRingToAffineOpen_zbar]
  exact isUnit_iff_dvd_one.mpr
    ⟨affineX a3 a2 a1 a0, by rw [mul_comm]; exact (affineX_mul_invX a3 a2 a1 a0).symm⟩

def infinityOpenToAffineOpen (a3 a2 a1 a0 : K) :
    InfinityOpen a3 a2 a1 a0 →ₐ[K] AffineOpen a3 a2 a1 a0 :=
  IsLocalization.Away.liftAlgHom (S := InfinityOpen a3 a2 a1 a0)
    (f := infinityRingToAffineOpen a3 a2 a1 a0)
    (Common22ExplicitChart.zbar a3 a2 a1 a0)
    (infinityRingToAffineOpen_zbar_isUnit a3 a2 a1 a0)

theorem infinityOpenToAffineOpen_infinityZ (a3 a2 a1 a0 : K) :
    infinityOpenToAffineOpen a3 a2 a1 a0 (infinityZ a3 a2 a1 a0) =
      affineInvX a3 a2 a1 a0 := by
  simpa [infinityOpenToAffineOpen, infinityZ] using
    infinityRingToAffineOpen_zbar a3 a2 a1 a0

theorem infinityOpenToAffineOpen_infinityW (a3 a2 a1 a0 : K) :
    infinityOpenToAffineOpen a3 a2 a1 a0 (infinityW a3 a2 a1 a0) =
      affineY a3 a2 a1 a0 * affineInvX a3 a2 a1 a0 ^ 3 := by
  simpa [infinityOpenToAffineOpen, infinityW] using
    infinityRingToAffineOpen_wbar a3 a2 a1 a0

theorem right_inverse_unique {R : Type*} [CommMonoid R] {a b c : R}
    (hab : a * b = 1) (hac : a * c = 1) : b = c := by
  calc
    b = b * (a * c) := by rw [hac]; simp
    _ = (a * b) * c := by ac_rfl
    _ = c := by rw [hab]; simp

theorem affineOpenToInfinityOpen_invX (a3 a2 a1 a0 : K) :
    affineOpenToInfinityOpen a3 a2 a1 a0 (affineInvX a3 a2 a1 a0) =
      infinityZ a3 a2 a1 a0 := by
  have h := congrArg (affineOpenToInfinityOpen a3 a2 a1 a0)
    (affineX_mul_invX a3 a2 a1 a0)
  have hfirst : infinityInvZ a3 a2 a1 a0 *
      affineOpenToInfinityOpen a3 a2 a1 a0 (affineInvX a3 a2 a1 a0) = 1 := by
    simpa [affineOpenToInfinityOpen_affineX] using h
  exact right_inverse_unique hfirst (infinityInvZ_mul_z a3 a2 a1 a0)

theorem infinityOpenToAffineOpen_invZ (a3 a2 a1 a0 : K) :
    infinityOpenToAffineOpen a3 a2 a1 a0 (infinityInvZ a3 a2 a1 a0) =
      affineX a3 a2 a1 a0 := by
  have h := congrArg (infinityOpenToAffineOpen a3 a2 a1 a0)
    (infinityInvZ_mul_z a3 a2 a1 a0)
  have hfirst : affineInvX a3 a2 a1 a0 *
      infinityOpenToAffineOpen a3 a2 a1 a0 (infinityInvZ a3 a2 a1 a0) = 1 := by
    simpa [mul_comm, infinityOpenToAffineOpen_infinityZ] using h
  exact right_inverse_unique hfirst
    (by simpa only [mul_comm] using affineX_mul_invX a3 a2 a1 a0)

theorem reverse_forward_affineX (a3 a2 a1 a0 : K) :
    infinityOpenToAffineOpen a3 a2 a1 a0
      (affineOpenToInfinityOpen a3 a2 a1 a0 (affineX a3 a2 a1 a0)) =
        affineX a3 a2 a1 a0 := by
  rw [affineOpenToInfinityOpen_affineX, infinityOpenToAffineOpen_invZ]

theorem reverse_forward_affineY (a3 a2 a1 a0 : K) :
    infinityOpenToAffineOpen a3 a2 a1 a0
      (affineOpenToInfinityOpen a3 a2 a1 a0 (affineY a3 a2 a1 a0)) =
        affineY a3 a2 a1 a0 := by
  rw [affineOpenToInfinityOpen_affineY, map_mul, map_pow,
    infinityOpenToAffineOpen_infinityW, infinityOpenToAffineOpen_invZ]
  have hpow : affineInvX a3 a2 a1 a0 ^ 3 * affineX a3 a2 a1 a0 ^ 3 = 1 := by
    rw [← mul_pow, mul_comm, affineX_mul_invX, one_pow]
  calc
    _ = affineY a3 a2 a1 a0 *
        (affineInvX a3 a2 a1 a0 ^ 3 * affineX a3 a2 a1 a0 ^ 3) := by ring
    _ = affineY a3 a2 a1 a0 := by rw [hpow]; ring

theorem forward_reverse_infinityZ (a3 a2 a1 a0 : K) :
    affineOpenToInfinityOpen a3 a2 a1 a0
      (infinityOpenToAffineOpen a3 a2 a1 a0 (infinityZ a3 a2 a1 a0)) =
        infinityZ a3 a2 a1 a0 := by
  rw [infinityOpenToAffineOpen_infinityZ, affineOpenToInfinityOpen_invX]

theorem forward_reverse_infinityW (a3 a2 a1 a0 : K) :
    affineOpenToInfinityOpen a3 a2 a1 a0
      (infinityOpenToAffineOpen a3 a2 a1 a0 (infinityW a3 a2 a1 a0)) =
        infinityW a3 a2 a1 a0 := by
  rw [infinityOpenToAffineOpen_infinityW, map_mul, map_pow,
    affineOpenToInfinityOpen_affineY, affineOpenToInfinityOpen_invX]
  have hpow : infinityInvZ a3 a2 a1 a0 ^ 3 * infinityZ a3 a2 a1 a0 ^ 3 = 1 := by
    rw [← mul_pow, infinityInvZ_mul_z, one_pow]
  calc
    _ = infinityW a3 a2 a1 a0 *
        (infinityInvZ a3 a2 a1 a0 ^ 3 * infinityZ a3 a2 a1 a0 ^ 3) := by ring
    _ = infinityW a3 a2 a1 a0 := by rw [hpow]; ring

theorem reverse_forward_eq_id (a3 a2 a1 a0 : K) :
    (infinityOpenToAffineOpen a3 a2 a1 a0).comp
      (affineOpenToInfinityOpen a3 a2 a1 a0) =
        AlgHom.id K (AffineOpen a3 a2 a1 a0) := by
  apply IsLocalization.algHom_ext (Submonoid.powers (xbar a3 a2 a1 a0))
  apply Ideal.Quotient.algHom_ext
  apply MvPolynomial.algHom_ext
  intro i
  fin_cases i
  · change infinityOpenToAffineOpen a3 a2 a1 a0
        (affineOpenToInfinityOpen a3 a2 a1 a0 (affineX a3 a2 a1 a0)) =
          affineX a3 a2 a1 a0
    exact reverse_forward_affineX a3 a2 a1 a0
  · change infinityOpenToAffineOpen a3 a2 a1 a0
        (affineOpenToInfinityOpen a3 a2 a1 a0 (affineY a3 a2 a1 a0)) =
          affineY a3 a2 a1 a0
    exact reverse_forward_affineY a3 a2 a1 a0

theorem forward_reverse_eq_id (a3 a2 a1 a0 : K) :
    (affineOpenToInfinityOpen a3 a2 a1 a0).comp
      (infinityOpenToAffineOpen a3 a2 a1 a0) =
        AlgHom.id K (InfinityOpen a3 a2 a1 a0) := by
  apply IsLocalization.algHom_ext
    (Submonoid.powers (Common22ExplicitChart.zbar a3 a2 a1 a0))
  apply Ideal.Quotient.algHom_ext
  apply MvPolynomial.algHom_ext
  intro i
  fin_cases i
  · change affineOpenToInfinityOpen a3 a2 a1 a0
        (infinityOpenToAffineOpen a3 a2 a1 a0 (infinityZ a3 a2 a1 a0)) =
          infinityZ a3 a2 a1 a0
    exact forward_reverse_infinityZ a3 a2 a1 a0
  · change affineOpenToInfinityOpen a3 a2 a1 a0
        (infinityOpenToAffineOpen a3 a2 a1 a0 (infinityW a3 a2 a1 a0)) =
          infinityW a3 a2 a1 a0
    exact forward_reverse_infinityW a3 a2 a1 a0

/-- The two separately presented coordinate rings agree on their principal-open overlap. -/
def separateChartOpenEquiv (a3 a2 a1 a0 : K) :
    AffineOpen a3 a2 a1 a0 ≃ₐ[K] InfinityOpen a3 a2 a1 a0 :=
  AlgEquiv.ofAlgHom
    (affineOpenToInfinityOpen a3 a2 a1 a0)
    (infinityOpenToAffineOpen a3 a2 a1 a0)
    (forward_reverse_eq_id a3 a2 a1 a0)
    (reverse_forward_eq_id a3 a2 a1 a0)

#print axioms infinityInvZ_mul_z
#print axioms affinePolynomialToInfinityOpen_X0
#print axioms affinePolynomialToInfinityOpen_X1
#print axioms infinity_ubar_expansion
#print axioms infinity_open_relation
#print axioms affine_relation_at_infinity
#print axioms affineEquation_maps_zero
#print axioms affineIdeal_maps_zero
#print axioms affineRingToInfinityOpen
#print axioms affineRingToInfinityOpen_xbar
#print axioms affineRingToInfinityOpen_ybar
#print axioms affineRingToInfinityOpen_xbar_isUnit
#print axioms affineOpenToInfinityOpen
#print axioms affineOpenToInfinityOpen_affineX
#print axioms affineOpenToInfinityOpen_affineY
#print axioms affineX_mul_invX
#print axioms quotient_affine_relation
#print axioms affine_open_relation
#print axioms chart_relation_at_affine
#print axioms infinityEquation_maps_zero
#print axioms infinityIdeal_maps_zero
#print axioms infinityRingToAffineOpen
#print axioms infinityRingToAffineOpen_zbar
#print axioms infinityRingToAffineOpen_wbar
#print axioms infinityRingToAffineOpen_zbar_isUnit
#print axioms infinityOpenToAffineOpen
#print axioms infinityOpenToAffineOpen_infinityZ
#print axioms infinityOpenToAffineOpen_infinityW
#print axioms affineOpenToInfinityOpen_invX
#print axioms infinityOpenToAffineOpen_invZ
#print axioms reverse_forward_affineX
#print axioms reverse_forward_affineY
#print axioms forward_reverse_infinityZ
#print axioms forward_reverse_infinityW
#print axioms reverse_forward_eq_id
#print axioms forward_reverse_eq_id
#print axioms separateChartOpenEquiv

end Common22SeparateChartMap
