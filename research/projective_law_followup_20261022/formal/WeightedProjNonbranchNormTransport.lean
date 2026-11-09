import WeightedProjNonbranchDVR
import Common22CoordinateRing

/-!
# Polynomial common 2+2 norm in the actual rational-point stalk

This transports a polynomial identity `n²-u₁u₂U=b²f` to the actual
weighted-Proj rational-point stalk when `f` is exactly the chart quintic.
The identity is algebraic and does not require a DVR. The allocation of its
two conjugate factors to local orders still requires `2`, `b`, and the
point ordinate to be units. In characteristic two, the conjugates coincide.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjNonbranchNormTransport

open Polynomial Common22SeparateChartMap Common22BranchStalk
  WeightedProjNonbranchZeroOrder

variable {K : Type*} [Field K]

private abbrev Local (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :=
  Localization.AtPrime (pointIdeal a3 a2 a1 a0 a y hcurve)

private abbrev R (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :=
  ProjPointRing a3 a2 a1 a0 a y hcurve

def projectiveAffineHom (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    AffineRing a3 a2 a1 a0 →+* R a3 a2 a1 a0 a y hcurve :=
  (pointRingEquiv a3 a2 a1 a0 a y hcurve).symm.toRingHom.comp
    (algebraMap (AffineRing a3 a2 a1 a0) (Local a3 a2 a1 a0 a y hcurve))

def projectiveCoeffHom (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    K →+* R a3 a2 a1 a0 a y hcurve :=
  (projectiveAffineHom a3 a2 a1 a0 a y hcurve).comp
    (algebraMap K (AffineRing a3 a2 a1 a0))

def projectiveX (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    R a3 a2 a1 a0 a y hcurve :=
  projectiveAffineHom a3 a2 a1 a0 a y hcurve (xbar a3 a2 a1 a0)

def projectivePolyEval (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    K[X] →+* R a3 a2 a1 a0 a y hcurve :=
  Polynomial.eval₂RingHom (projectiveCoeffHom a3 a2 a1 a0 a y hcurve)
    (projectiveX a3 a2 a1 a0 a y hcurve)

private theorem projectivePolyEval_comp (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    projectivePolyEval a3 a2 a1 a0 a y hcurve =
      (projectiveAffineHom a3 a2 a1 a0 a y hcurve).comp
        (Polynomial.eval₂RingHom (algebraMap K (AffineRing a3 a2 a1 a0))
          (xbar a3 a2 a1 a0)) := by
  apply Polynomial.ringHom_ext
  · intro b
    simp [projectivePolyEval, projectiveCoeffHom]
  · simp [projectivePolyEval, projectiveX]

theorem projective_quintic_relation (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    projectivePolyEval a3 a2 a1 a0 a y hcurve
      (WeightedProjSmoothPoints.quintic a3 a2 a1 a0) =
      pointY a3 a2 a1 a0 a y hcurve ^ 2 := by
  have hq : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).eval₂
      (algebraMap K (AffineRing a3 a2 a1 a0))
      (xbar a3 a2 a1 a0) = ybar a3 a2 a1 a0 ^ 2 := by
    simpa [WeightedProjSmoothPoints.quintic] using
      (quotient_affine_relation a3 a2 a1 a0).symm
  have hm := congrArg (projectiveAffineHom a3 a2 a1 a0 a y hcurve) hq
  rw [projectivePolyEval_comp]
  simpa [projectiveAffineHom, pointY] using hm

theorem projective_norm_identity (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (u1 u2 n support : K[X]) (b : K)
    (hpoly : n ^ 2 - u1 * u2 * support =
      C (b ^ 2) * WeightedProjSmoothPoints.quintic a3 a2 a1 a0) :
    let φ := projectivePolyEval a3 a2 a1 a0 a y hcurve
    let B := projectiveCoeffHom a3 a2 a1 a0 a y hcurve b
    let Y := pointY a3 a2 a1 a0 a y hcurve
    (φ n - B * Y) * (φ n + B * Y) = φ u1 * φ u2 * φ support := by
  dsimp
  have hp := congrArg (projectivePolyEval a3 a2 a1 a0 a y hcurve) hpoly
  simp only [map_sub, map_pow, map_mul] at hp
  have hC : projectivePolyEval a3 a2 a1 a0 a y hcurve (C b) =
      projectiveCoeffHom a3 a2 a1 a0 a y hcurve b := by
    simp [projectivePolyEval]
  rw [hC] at hp
  rw [projective_quintic_relation a3 a2 a1 a0 a y hcurve] at hp
  calc
    _ = (projectivePolyEval a3 a2 a1 a0 a y hcurve n) ^ 2 -
      (projectiveCoeffHom a3 a2 a1 a0 a y hcurve b) ^ 2 *
        pointY a3 a2 a1 a0 a y hcurve ^ 2 := by ring
    _ = projectivePolyEval a3 a2 a1 a0 a y hcurve u1 *
        projectivePolyEval a3 a2 a1 a0 a y hcurve u2 *
          projectivePolyEval a3 a2 a1 a0 a y hcurve support := by
      linear_combination hp

/-- Instantiate the actual-stalk norm identity with the checked proof-side
source-style common 2+2 polynomial theorem. Valid Mumford inputs and a
nonzero interpolation determinant remain explicit. -/
theorem source_style_projective_norm_identity
    (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (q1 r1 s1 t1 q2 r2 s2 t2 : K)
    (hb : (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1 ≠ 0)
    (valid1 : Common22Coprime.firstSupport q1 r1 ∣
      WeightedProjSmoothPoints.quintic a3 a2 a1 a0 -
        (C s1 * X + C t1) ^ 2)
    (valid2 : Common22Coprime.secondSupport q2 r2 ∣
      WeightedProjSmoothPoints.quintic a3 a2 a1 a0 -
        (C s2 * X + C t2) ^ 2) :
    let (b, aa, c, q, r) :=
      Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
    let u1 : K[X] := X ^ 2 + C q1 * X + C r1
    let u2 : K[X] := X ^ 2 + C q2 * X + C r2
    let n : K[X] := C (-c) * X ^ 3 + C (-(c * q1 + aa)) * X ^ 2 +
      C (b * s1 - c * r1 - aa * q1) * X + C (b * t1 - aa * r1)
    let support : K[X] := C (c ^ 2) * X ^ 2 + C q * X + C r
    let φ := projectivePolyEval a3 a2 a1 a0 a y hcurve
    let B := projectiveCoeffHom a3 a2 a1 a0 a y hcurve b
    let Y := pointY a3 a2 a1 a0 a y hcurve
    (φ n - B * Y) * (φ n + B * Y) = φ u1 * φ u2 * φ support := by
  have hs := Common22SourceHead.source_curve_eq q1 r1 s1 t1 q2 r2 s2 t2
    (WeightedProjSmoothPoints.quintic a3 a2 a1 a0)
    hb valid1 valid2
    (WeightedProjQuinticHead.quintic_natDegree_le_five a3 a2 a1 a0)
    (WeightedProjQuinticHead.quintic_coeff_five a3 a2 a1 a0)
    (WeightedProjQuinticHead.quintic_coeff_four a3 a2 a1 a0)
  unfold Common22SourceHead.sourceE at hs
  simpa only using (projective_norm_identity a3 a2 a1 a0 a y hcurve
    (X ^ 2 + C q1 * X + C r1)
    (X ^ 2 + C q2 * X + C r2)
    (let (b, aa, c, _, _) :=
      Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
     C (-c) * X ^ 3 + C (-(c * q1 + aa)) * X ^ 2 +
       C (b * s1 - c * r1 - aa * q1) * X + C (b * t1 - aa * r1))
    (let (_, _, c, q, r) :=
      Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2
     C (c ^ 2) * X ^ 2 + C q * X + C r)
    (Common22SupportHead.coefficientData q1 r1 s1 t1 q2 r2 s2 t2).1
    (by simpa only [Common22Coprime.firstSupport,
      Common22Coprime.secondSupport] using hs))

/-- In the odd-characteristic nonbranch case, the transported polynomial
norm determines the local order of the chosen vanishing factor. The
vanishing/nonunit premise is explicit; the norm identity does not choose
between the two conjugate points by itself. -/
theorem projective_norm_order (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable)
    (h2y : 2 * y ≠ 0)
    (u1 u2 n support : K[X]) (b : K) (hb : b ≠ 0)
    (hpoly : n ^ 2 - u1 * u2 * support =
      C (b ^ 2) * WeightedProjSmoothPoints.quintic a3 a2 a1 a0)
    (hg : projectivePolyEval a3 a2 a1 a0 a y hcurve n -
      projectiveCoeffHom a3 a2 a1 a0 a y hcurve b *
        pointY a3 a2 a1 a0 a y hcurve ∈
          nonunits (R a3 a2 a1 a0 a y hcurve)) :
    let R0 := R a3 a2 a1 a0 a y hcurve
    let φ := projectivePolyEval a3 a2 a1 a0 a y hcurve
    let B := projectiveCoeffHom a3 a2 a1 a0 a y hcurve b
    let Y := pointY a3 a2 a1 a0 a y hcurve
    @IsDiscreteValuationRing.addVal R0 _
      (Common22SeparableAffineDomain.projective_point_domain_of_separable
        a3 a2 a1 a0 a y hcurve hsep)
      (WeightedProjNonbranchDVR.projective_point_dvr
        a3 a2 a1 a0 a y hcurve hsep h2y)
      (φ n - B * Y) =
      @IsDiscreteValuationRing.addVal R0 _
        (Common22SeparableAffineDomain.projective_point_domain_of_separable
          a3 a2 a1 a0 a y hcurve hsep)
        (WeightedProjNonbranchDVR.projective_point_dvr
          a3 a2 a1 a0 a y hcurve hsep h2y) (φ u1) +
      @IsDiscreteValuationRing.addVal R0 _
        (Common22SeparableAffineDomain.projective_point_domain_of_separable
          a3 a2 a1 a0 a y hcurve hsep)
        (WeightedProjNonbranchDVR.projective_point_dvr
          a3 a2 a1 a0 a y hcurve hsep h2y) (φ u2) +
      @IsDiscreteValuationRing.addVal R0 _
        (Common22SeparableAffineDomain.projective_point_domain_of_separable
          a3 a2 a1 a0 a y hcurve hsep)
        (WeightedProjNonbranchDVR.projective_point_dvr
          a3 a2 a1 a0 a y hcurve hsep h2y) (φ support) := by
  letI : IsDomain (R a3 a2 a1 a0 a y hcurve) :=
    Common22SeparableAffineDomain.projective_point_domain_of_separable
      a3 a2 a1 a0 a y hcurve hsep
  letI : IsDiscreteValuationRing (R a3 a2 a1 a0 a y hcurve) :=
    WeightedProjNonbranchDVR.projective_point_dvr
      a3 a2 a1 a0 a y hcurve hsep h2y
  have hy : y ≠ 0 := by
    intro h
    exact h2y (by simp [h])
  have h2K : (2 : K) ≠ 0 := by
    intro h
    exact h2y (by simp [h])
  have h2R : IsUnit (2 : R a3 a2 a1 a0 a y hcurve) := by
    have hu : IsUnit (2 : K) := isUnit_iff_ne_zero.mpr h2K
    have hmap : IsUnit ((projectiveCoeffHom a3 a2 a1 a0 a y hcurve) (2 : K)) :=
      hu.map (projectiveCoeffHom a3 a2 a1 a0 a y hcurve)
    simpa only [map_ofNat] using hmap
  have hB : IsUnit (projectiveCoeffHom a3 a2 a1 a0 a y hcurve b) :=
    (isUnit_iff_ne_zero.mpr hb).map
      (projectiveCoeffHom a3 a2 a1 a0 a y hcurve)
  have hnorm := projective_norm_identity a3 a2 a1 a0 a y hcurve
    u1 u2 n support b hpoly
  dsimp at hnorm
  dsimp
  exact WeightedProjNonbranchZeroOrder.point_norm_order
    a3 a2 a1 a0 a y hcurve hy
    (projectivePolyEval a3 a2 a1 a0 a y hcurve n)
    (projectiveCoeffHom a3 a2 a1 a0 a y hcurve b)
    (projectivePolyEval a3 a2 a1 a0 a y hcurve u1)
    (projectivePolyEval a3 a2 a1 a0 a y hcurve u2)
    (projectivePolyEval a3 a2 a1 a0 a y hcurve support)
    h2R hB hg hnorm

#print axioms projective_quintic_relation
#print axioms projective_norm_identity
#print axioms source_style_projective_norm_identity
#print axioms projective_norm_order

end WeightedProjNonbranchNormTransport
