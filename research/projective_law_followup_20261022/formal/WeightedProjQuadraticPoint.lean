import WeightedProjNonbranchZeroOrder
import Mathlib.RingTheory.AdjoinRoot

/-!
# Extension-valued points on the actual weighted-Proj curve

An irreducible quadratic Mumford support has a degree-two residue field.  The
existing rational-point constructor cannot represent it.  This module builds
the evaluation, prime chart ideal, and projective point over a field extension,
then proves the chart ideal maximal for an irreducible support.  Its DVR and
divisor orders are separate gates.

The stalk transport uses the same open-immersion path as the rational-point
constructor.  See Stacks Project Tags 01HD (open-immersion stalks) and 00PD
(DVR criterion).
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjQuadraticPoint

open AlgebraicGeometry CategoryTheory MvPolynomial
  Common22SeparateChartMap Common22BranchStalk
  Common22WeightedProjKernel WeightedRingGraded

variable {K L : Type*} [Field K] [Field L] [Algebra K L]

def fAtExtension (a3 a2 a1 a0 : K) (x : L) : L :=
  x ^ 5 + algebraMap K L a3 * x ^ 3 +
    algebraMap K L a2 * x ^ 2 + algebraMap K L a1 * x +
      algebraMap K L a0

def preEval (x y : L) : Poly K →+* L :=
  eval₂Hom (algebraMap K L) (fun i : Fin 2 => if i = 0 then x else y)

theorem preEval_equation (a3 a2 a1 a0 : K) (x y : L) :
    preEval x y (affineEquation a3 a2 a1 a0) =
      y ^ 2 - fAtExtension a3 a2 a1 a0 x := by
  simp [preEval, affineEquation, fAtExtension]

def eval (a3 a2 a1 a0 : K) (x y : L)
    (hcurve : y ^ 2 = fAtExtension a3 a2 a1 a0 x) :
    AffineRing a3 a2 a1 a0 →+* L :=
  Ideal.Quotient.lift (affineIdeal a3 a2 a1 a0) (preEval x y) (by
    intro p hp
    have hle : affineIdeal a3 a2 a1 a0 ≤ RingHom.ker (preEval x y) := by
      apply Ideal.span_le.mpr
      intro t ht
      simp only [Set.mem_singleton_iff] at ht
      subst t
      simp [RingHom.mem_ker, preEval_equation, hcurve]
    exact hle hp)

def pointIdeal (a3 a2 a1 a0 : K) (x y : L)
    (hcurve : y ^ 2 = fAtExtension a3 a2 a1 a0 x) :
    Ideal (AffineRing a3 a2 a1 a0) :=
  RingHom.ker (eval a3 a2 a1 a0 x y hcurve)

instance pointIdeal_isPrime (a3 a2 a1 a0 : K) (x y : L)
    (hcurve : y ^ 2 = fAtExtension a3 a2 a1 a0 x) :
    (pointIdeal a3 a2 a1 a0 x y hcurve).IsPrime :=
  RingHom.ker_isPrime _

def pointSpec (a3 a2 a1 a0 : K) (x y : L)
    (hcurve : y ^ 2 = fAtExtension a3 a2 a1 a0 x) :
    PrimeSpectrum (AffineRing a3 a2 a1 a0) :=
  ⟨pointIdeal a3 a2 a1 a0 x y hcurve, inferInstance⟩

def pointProj (a3 a2 a1 a0 : K) (x y : L)
    (hcurve : y ^ 2 = fAtExtension a3 a2 a1 a0 x) :
    Proj (degreeSubmodule a3 a2 a1 a0) :=
  (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
    (zbar a3 a2 a1 a0)).ι
    ((WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv
      (pointSpec a3 a2 a1 a0 x y hcurve))

def pointProjStalkIso (a3 a2 a1 a0 : K) (x y : L)
    (hcurve : y ^ 2 = fAtExtension a3 a2 a1 a0 x) :
    (Proj (degreeSubmodule a3 a2 a1 a0)).presheaf.stalk
      (pointProj a3 a2 a1 a0 x y hcurve) ≅
    ↧(Localization.AtPrime (pointIdeal a3 a2 a1 a0 x y hcurve)) := by
  let f := (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (zbar a3 a2 a1 a0)).ι
  let g := (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv
  let q := pointSpec a3 a2 a1 a0 x y hcurve
  letI : IsOpenImmersion f := inferInstance
  letI : IsOpenImmersion g := inferInstance
  have hi : IsIso (f.stalkMap (g q)) :=
    (IsOpenImmersion.iff_isIso_stalkMap).mp inferInstance |>.2 _
  have hg : IsIso (g.stalkMap q) :=
    (IsOpenImmersion.iff_isIso_stalkMap).mp inferInstance |>.2 _
  exact (@asIso CommRingCat _ _ _ (f.stalkMap (g q)) hi) ≪≫
    (@asIso CommRingCat _ _ _ (g.stalkMap q) hg) ≪≫
      Spec.stalkIso (CommRingCat.of (AffineRing a3 a2 a1 a0)) q

theorem pointIdeal_maximal_of_surjective (a3 a2 a1 a0 : K) (x y : L)
    (hcurve : y ^ 2 = fAtExtension a3 a2 a1 a0 x)
    (hsurj : Function.Surjective (eval a3 a2 a1 a0 x y hcurve)) :
    (pointIdeal a3 a2 a1 a0 x y hcurve).IsMaximal :=
  RingHom.ker_isMaximal_of_surjective _ hsurj

/-- A Mumford support equation gives actual extension-valued coordinates.
The irreducibility hypothesis makes the adjoined-root quotient a field. -/
theorem root_curve_of_dvd (a3 a2 a1 a0 : K) (u v : Polynomial K)
    [Fact (Irreducible u)]
    (hdiv : u ∣ v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0) :
    (Polynomial.aeval (AdjoinRoot.root u) v) ^ 2 =
      fAtExtension a3 a2 a1 a0 (AdjoinRoot.root u) := by
  have hz : Polynomial.aeval (AdjoinRoot.root u)
      (v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0) = 0 := by
    rw [AdjoinRoot.aeval_eq]
    exact AdjoinRoot.mk_eq_zero.mpr hdiv
  have heq : (Polynomial.aeval (AdjoinRoot.root u) v) ^ 2 =
      Polynomial.aeval (AdjoinRoot.root u)
        (WeightedProjSmoothPoints.quintic a3 a2 a1 a0) := by
    simpa only [map_sub, map_pow, sub_eq_zero] using hz
  simpa [WeightedProjSmoothPoints.quintic, fAtExtension] using heq

def rootProjPoint (a3 a2 a1 a0 : K) (u v : Polynomial K)
    [Fact (Irreducible u)]
    (hdiv : u ∣ v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0) :
    Proj (degreeSubmodule a3 a2 a1 a0) :=
  pointProj a3 a2 a1 a0
    (AdjoinRoot.root u) (Polynomial.aeval (AdjoinRoot.root u) v)
    (root_curve_of_dvd a3 a2 a1 a0 u v hdiv)

def rootEvalAlgHom (a3 a2 a1 a0 : K) (u v : Polynomial K)
    [Fact (Irreducible u)]
    (hdiv : u ∣ v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0) :
    AffineRing a3 a2 a1 a0 →ₐ[K] AdjoinRoot u where
  toRingHom := eval a3 a2 a1 a0
    (AdjoinRoot.root u) (Polynomial.aeval (AdjoinRoot.root u) v)
    (root_curve_of_dvd a3 a2 a1 a0 u v hdiv)
  commutes' := by
    intro c
    change preEval (AdjoinRoot.root u)
      (Polynomial.aeval (AdjoinRoot.root u) v) (MvPolynomial.C c) =
        AdjoinRoot.of u c
    simp [preEval]

theorem rootEvalAlgHom_xbar (a3 a2 a1 a0 : K) (u v : Polynomial K)
    [Fact (Irreducible u)]
    (hdiv : u ∣ v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0) :
    rootEvalAlgHom a3 a2 a1 a0 u v hdiv (xbar a3 a2 a1 a0) =
      AdjoinRoot.root u := by
  simp [rootEvalAlgHom, eval, preEval, xbar]

theorem rootEvalAlgHom_surjective (a3 a2 a1 a0 : K) (u v : Polynomial K)
    [Fact (Irreducible u)]
    (hdiv : u ∣ v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0) :
    Function.Surjective (rootEvalAlgHom a3 a2 a1 a0 u v hdiv) := by
  let F := rootEvalAlgHom a3 a2 a1 a0 u v hdiv
  have hroot : AdjoinRoot.root u ∈ F.range := by
    exact ⟨xbar a3 a2 a1 a0, rootEvalAlgHom_xbar a3 a2 a1 a0 u v hdiv⟩
  have htop : Algebra.adjoin K ({AdjoinRoot.root u} : Set (AdjoinRoot u)) ≤
      F.range := Algebra.adjoin_le (by simpa using hroot)
  rw [AdjoinRoot.adjoinRoot_eq_top] at htop
  intro z
  exact htop (by simp)

theorem rootPointIdeal_maximal (a3 a2 a1 a0 : K) (u v : Polynomial K)
    [Fact (Irreducible u)]
    (hdiv : u ∣ v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0) :
    (pointIdeal a3 a2 a1 a0
      (AdjoinRoot.root u) (Polynomial.aeval (AdjoinRoot.root u) v)
      (root_curve_of_dvd a3 a2 a1 a0 u v hdiv)).IsMaximal := by
  exact pointIdeal_maximal_of_surjective a3 a2 a1 a0
    (AdjoinRoot.root u) (Polynomial.aeval (AdjoinRoot.root u) v)
    (root_curve_of_dvd a3 a2 a1 a0 u v hdiv)
    (rootEvalAlgHom_surjective a3 a2 a1 a0 u v hdiv)

def rootProjStalkIso (a3 a2 a1 a0 : K) (u v : Polynomial K)
    [Fact (Irreducible u)]
    (hdiv : u ∣ v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0) :
    (Proj (degreeSubmodule a3 a2 a1 a0)).presheaf.stalk
      (rootProjPoint a3 a2 a1 a0 u v hdiv) ≅
    ↧(Localization.AtPrime (pointIdeal a3 a2 a1 a0
      (AdjoinRoot.root u) (Polynomial.aeval (AdjoinRoot.root u) v)
      (root_curve_of_dvd a3 a2 a1 a0 u v hdiv))) :=
  pointProjStalkIso a3 a2 a1 a0
    (AdjoinRoot.root u) (Polynomial.aeval (AdjoinRoot.root u) v)
    (root_curve_of_dvd a3 a2 a1 a0 u v hdiv)

#print axioms preEval_equation
#print axioms pointIdeal_isPrime
#print axioms pointProjStalkIso
#print axioms pointIdeal_maximal_of_surjective
#print axioms root_curve_of_dvd
#print axioms rootProjPoint
#print axioms rootEvalAlgHom_surjective
#print axioms rootPointIdeal_maximal
#print axioms rootProjStalkIso

end WeightedProjQuadraticPoint
