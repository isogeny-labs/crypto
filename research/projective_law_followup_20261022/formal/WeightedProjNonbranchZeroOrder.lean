import WeightedProjSchemeChart
import Common22BranchStalk
import Common22FiniteZeroOrder
import WeightedProjQuinticHead
import Mathlib.AlgebraicGeometry.AffineScheme
import Mathlib.AlgebraicGeometry.OpenImmersion

/-!
# A rational nonbranch point on the actual weighted-Proj curve

This constructs the actual projective stalk at `(a,y)` with `y²=f(a)` and
`y≠0`, and transports the affine coordinates into that stalk. Stacks Project
Tag 01HD identifies stalks under open immersion. The branch-point pattern is
`WeightedFiniteBranchStalk.lean`; the abstract local order lemma is
`Common22FiniteZeroOrder.lean`. Stacks Tag 00PD states the DVR criterion.

No global divisor or compiled Rust semantics follows from this local packet.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjNonbranchZeroOrder

open AlgebraicGeometry CategoryTheory MvPolynomial
  Common22SeparateChartMap Common22BranchStalk
  Common22WeightedProjKernel WeightedRingGraded

variable {K : Type*} [Field K]

def pointPreEval (a y : K) : Poly K →+* K :=
  eval₂Hom (RingHom.id K) (fun i : Fin 2 => if i = 0 then a else y)

theorem pointPreEval_equation (a3 a2 a1 a0 a y : K) :
    pointPreEval a y (affineEquation a3 a2 a1 a0) =
      y ^ 2 - fAt a3 a2 a1 a0 a := by
  simp [pointPreEval, affineEquation, fAt]

def pointEval (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    AffineRing a3 a2 a1 a0 →+* K :=
  Ideal.Quotient.lift (affineIdeal a3 a2 a1 a0) (pointPreEval a y) (by
    intro p hp
    have hle : affineIdeal a3 a2 a1 a0 ≤ RingHom.ker (pointPreEval a y) := by
      apply Ideal.span_le.mpr
      intro x hx
      simp only [Set.mem_singleton_iff] at hx
      subst x
      simp [RingHom.mem_ker, pointPreEval_equation, hcurve]
    exact hle hp)

theorem pointEval_surjective (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    Function.Surjective (pointEval a3 a2 a1 a0 a y hcurve) := by
  apply Ideal.Quotient.lift_surjective_of_surjective
  intro k
  use C k
  simp [pointPreEval]

def pointIdeal (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    Ideal (AffineRing a3 a2 a1 a0) :=
  RingHom.ker (pointEval a3 a2 a1 a0 a y hcurve)

theorem pointIdeal_maximal (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    (pointIdeal a3 a2 a1 a0 a y hcurve).IsMaximal :=
  RingHom.ker_isMaximal_of_surjective _
    (pointEval_surjective a3 a2 a1 a0 a y hcurve)

instance pointIdeal_isPrime (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    (pointIdeal a3 a2 a1 a0 a y hcurve).IsPrime :=
  (pointIdeal_maximal a3 a2 a1 a0 a y hcurve).isPrime

def pointSpec (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    PrimeSpectrum (AffineRing a3 a2 a1 a0) :=
  ⟨pointIdeal a3 a2 a1 a0 a y hcurve, inferInstance⟩

def pointProj (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    Proj (degreeSubmodule a3 a2 a1 a0) :=
  (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
    (zbar a3 a2 a1 a0)).ι
    ((WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv
      (pointSpec a3 a2 a1 a0 a y hcurve))

def pointProjStalkIso (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    (Proj (degreeSubmodule a3 a2 a1 a0)).presheaf.stalk
      (pointProj a3 a2 a1 a0 a y hcurve) ≅
    ↧(Localization.AtPrime (pointIdeal a3 a2 a1 a0 a y hcurve)) := by
  let f := (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (zbar a3 a2 a1 a0)).ι
  let g := (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv
  let q := pointSpec a3 a2 a1 a0 a y hcurve
  letI : IsOpenImmersion f := inferInstance
  letI : IsOpenImmersion g := inferInstance
  have hi : IsIso (f.stalkMap (g q)) :=
    (IsOpenImmersion.iff_isIso_stalkMap).mp inferInstance |>.2 _
  have hg : IsIso (g.stalkMap q) :=
    (IsOpenImmersion.iff_isIso_stalkMap).mp inferInstance |>.2 _
  exact (@asIso CommRingCat _ _ _ (f.stalkMap (g q)) hi) ≪≫
    (@asIso CommRingCat _ _ _ (g.stalkMap q) hg) ≪≫
      Spec.stalkIso (CommRingCat.of (AffineRing a3 a2 a1 a0)) q

abbrev ProjPointRing (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) : CommRingCat :=
  (Proj (degreeSubmodule a3 a2 a1 a0)).presheaf.stalk
    (pointProj a3 a2 a1 a0 a y hcurve)

def pointRingEquiv (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    ProjPointRing a3 a2 a1 a0 a y hcurve ≃+*
      Localization.AtPrime (pointIdeal a3 a2 a1 a0 a y hcurve) :=
  (pointProjStalkIso a3 a2 a1 a0 a y hcurve).commRingCatIsoToRingEquiv

theorem pointEval_ybar (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    pointEval a3 a2 a1 a0 a y hcurve (ybar a3 a2 a1 a0) = y := by
  simp [pointEval, ybar, pointPreEval]

def pointY (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    ProjPointRing a3 a2 a1 a0 a y hcurve :=
  (pointRingEquiv a3 a2 a1 a0 a y hcurve).symm
    ((algebraMap (AffineRing a3 a2 a1 a0)
      (Localization.AtPrime (pointIdeal a3 a2 a1 a0 a y hcurve)))
      (ybar a3 a2 a1 a0))

theorem pointY_unit (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) (hy : y ≠ 0) :
    IsUnit (pointY a3 a2 a1 a0 a y hcurve) := by
  have hloc : IsUnit ((algebraMap (AffineRing a3 a2 a1 a0)
      (Localization.AtPrime (pointIdeal a3 a2 a1 a0 a y hcurve)))
      (ybar a3 a2 a1 a0)) := by
    apply (IsLocalization.AtPrime.isUnit_to_map_iff
      (Localization.AtPrime (pointIdeal a3 a2 a1 a0 a y hcurve))
      (pointIdeal a3 a2 a1 a0 a y hcurve) _).mpr
    simp only [Ideal.mem_primeCompl_iff]
    intro hmem
    exact hy ((pointEval_ybar a3 a2 a1 a0 a y hcurve).symm.trans
      ((RingHom.mem_ker).mp hmem))
  exact hloc.map (pointRingEquiv a3 a2 a1 a0 a y hcurve).symm.toRingHom

/-- Conditional local norm order on the *actual* weighted-Proj stalk.  The
domain and DVR assumptions are deliberately explicit: their derivation at an
arbitrary nonbranch point is a separate outstanding geometry theorem. -/
theorem point_norm_order (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) (hy : y ≠ 0)
    [IsDomain (ProjPointRing a3 a2 a1 a0 a y hcurve)]
    [IsDiscreteValuationRing (ProjPointRing a3 a2 a1 a0 a y hcurve)]
    (N B u₁ u₂ U : ProjPointRing a3 a2 a1 a0 a y hcurve)
    (h2 : IsUnit (2 : ProjPointRing a3 a2 a1 a0 a y hcurve))
    (hB : IsUnit B)
    (hg : N - B * pointY a3 a2 a1 a0 a y hcurve ∈
      nonunits (ProjPointRing a3 a2 a1 a0 a y hcurve))
    (hnorm : (N - B * pointY a3 a2 a1 a0 a y hcurve) *
      (N + B * pointY a3 a2 a1 a0 a y hcurve) = u₁ * u₂ * U) :
    IsDiscreteValuationRing.addVal _
      (N - B * pointY a3 a2 a1 a0 a y hcurve) =
      IsDiscreteValuationRing.addVal _ u₁ +
        IsDiscreteValuationRing.addVal _ u₂ +
          IsDiscreteValuationRing.addVal _ U :=
  Common22FiniteZeroOrder.nonbranch_norm_order N B
    (pointY a3 a2 a1 a0 a y hcurve) u₁ u₂ U
    h2 hB (pointY_unit a3 a2 a1 a0 a y hcurve hy) hg hnorm

#print axioms pointPreEval_equation
#print axioms pointEval_surjective
#print axioms pointIdeal_maximal
#print axioms pointProjStalkIso
#print axioms pointRingEquiv
#print axioms pointY_unit
#print axioms point_norm_order

end WeightedProjNonbranchZeroOrder
