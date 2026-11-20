import DilationNonzeroBounds

/-!
# Sign normalization for Semaev point relations

The algebraically closed converse for the canonical Semaev family supplies
some point above each prescribed x-coordinate.  Any other chosen point above
the same x-coordinate differs by sign.  This module packages that observation
for a whole tuple, including the resulting signed zero-sum relation.
-/

namespace TensorBoundaryBounds

noncomputable section

open SemaevS3 SemaevPointListsAudit RecursiveSemaevEvaluationAudit
open RecursiveSemaevGeometryAudit
open scoped BigOperators

/-- The function-field radicand is exactly the short-curve right-hand side at
the scaled transcendental coordinate. -/
theorem shortCubicFunctionRadicand_eq_curve_rhs
    {K : Type*} [Field K] (A B scale : K) :
    shortCubicFunctionRadicand A B scale =
      (algebraMap K (FractionRing (Polynomial K)) scale *
          algebraMap (Polynomial K) (FractionRing (Polynomial K))
            Polynomial.X) ^ 3 +
        algebraMap K (FractionRing (Polynomial K)) A *
          (algebraMap K (FractionRing (Polynomial K)) scale *
            algebraMap (Polynomial K) (FractionRing (Polynomial K))
              Polynomial.X) +
        algebraMap K (FractionRing (Polynomial K)) B := by
  simp [shortCubicFunctionRadicand, shortCubicPolynomial,
    IsScalarTower.algebraMap_apply K (Polynomial K)
      (FractionRing (Polynomial K))]

/-- Rational function field used by the scaled short-cubic construction. -/
abbrev ShortCubicFunctionField (K : Type*) [Field K] :=
  FractionRing (Polynomial K)

/-- The scaled transcendental x-coordinate `scale * X`. -/
noncomputable def shortCubicScaledX
    {K : Type*} [Field K] (scale : K) : ShortCubicFunctionField K :=
  algebraMap K _ scale * algebraMap (Polynomial K) _ Polynomial.X

/-- The short curve after extension to its rational function field. -/
noncomputable def shortCubicFunctionCurve
    {K : Type*} [Field K] (A B : K) :
    WeierstrassCurve (ShortCubicFunctionField K) :=
  shortCharacterCurve 0 (algebraMap K _ A) (algebraMap K _ B)

/-- Ellipticity of a short curve is preserved by extension to its rational
function field. -/
noncomputable instance shortCubicFunctionCurve.instIsElliptic
    {K : Type*} [Field K] (A B : K)
    [(shortCharacterCurve 0 A B).IsElliptic] :
    (shortCubicFunctionCurve A B).IsElliptic := by
  have curveEq :
      (shortCharacterCurve 0 A B).map
          (algebraMap K (ShortCubicFunctionField K)) =
        shortCubicFunctionCurve A B := by
    ext <;>
      simp [shortCubicFunctionCurve, shortCharacterCurve]
  refine ⟨?_⟩
  rw [← curveEq, WeierstrassCurve.map_Δ]
  exact IsUnit.map (algebraMap K (ShortCubicFunctionField K))
    (shortCharacterCurve 0 A B).isUnit_Δ

/-- The indexed radicands whose canonical roots supply the point y-coordinates. -/
noncomputable def shortCubicRadicandFamily
    {K Index : Type*} [Field K] (A B : K) (scale : Index → K) :
    Index → ShortCubicFunctionField K :=
  fun index ↦ shortCubicFunctionRadicand A B (scale index)

/- A square root of a short-cubic function-field radicand in any extension is
an affine point on the corresponding base-changed short curve. -/
theorem shortCubicRoot_nonsingular
    {K L : Type*} [Field K] [Field L]
    [Algebra (ShortCubicFunctionField K) L]
    (A B scale : K)
    [(shortCubicFunctionCurve A B).IsElliptic]
    (root : L)
    (rootSquare : root ^ 2 =
      algebraMap (ShortCubicFunctionField K) L
        (shortCubicFunctionRadicand A B scale)) :
    ((shortCubicFunctionCurve A B).toAffine.baseChange L).Nonsingular
      (algebraMap (ShortCubicFunctionField K) L (shortCubicScaledX scale))
      root := by
  let F := ShortCubicFunctionField K
  have hDelta : (shortCubicFunctionCurve A B).Δ ≠ 0 :=
    (shortCubicFunctionCurve A B).isUnit_Δ.ne_zero
  have hDeltaL :
      ((shortCubicFunctionCurve A B).toAffine.baseChange L).Δ ≠ 0 := by
    change ((shortCubicFunctionCurve A B).map (algebraMap F L)).Δ ≠ 0
    rw [WeierstrassCurve.map_Δ]
    exact (map_ne_zero (algebraMap F L)).2 hDelta
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
    hDeltaL).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  rw [rootSquare]
  rw [shortCubicFunctionRadicand_eq_curve_rhs]
  simp [shortCubicFunctionCurve, shortCharacterCurve,
    WeierstrassCurve.baseChange, shortCubicScaledX, map_add, map_mul, map_pow]

/-- The point supplied by a square root of a short-cubic radicand is an
eigenvector for the natural Galois action, with eigenvalue equal to the
square-root sign character. -/
theorem shortCubicRoot_galois_eigenvector
    {K L : Type*} [Field K] [Field L]
    [Algebra (ShortCubicFunctionField K) L] [DecidableEq L]
    (A B scale : K)
    [(shortCubicFunctionCurve A B).IsElliptic]
    (root : L)
    (rootSquare : root ^ 2 =
      algebraMap (ShortCubicFunctionField K) L
        (shortCubicFunctionRadicand A B scale))
    (sigma : L ≃ₐ[ShortCubicFunctionField K] L) :
    affinePointGaloisAddEquiv
        (shortCubicFunctionCurve A B).toAffine sigma
        (WeierstrassCurve.Affine.Point.some
          (algebraMap (ShortCubicFunctionField K) L
            (shortCubicScaledX scale))
          root (shortCubicRoot_nonsingular A B scale root rootSquare)) =
      galoisSqrtSign root sigma •
        WeierstrassCurve.Affine.Point.some
          (algebraMap (ShortCubicFunctionField K) L
            (shortCubicScaledX scale))
          root (shortCubicRoot_nonsingular A B scale root rootSquare) := by
  apply affinePointGaloisAddEquiv_eigenvector
      (shortCubicFunctionCurve A B).toAffine
      (by simp [shortCubicFunctionCurve, shortCharacterCurve])
      (by simp [shortCubicFunctionCurve, shortCharacterCurve])
      rootSquare

/-- The canonical common splitting field for the indexed short-cubic
radicands. -/
abbrev ShortCubicSplittingField
    (K Index : Type*) [Field K] [Fintype Index]
    (A B : K) (scale : Index → K) :=
  (quadraticRadicandPolynomial
    (shortCubicRadicandFamily A B scale)).SplittingField

/-- The canonical point above `scale index * X`, obtained from the selected
root in the common quadratic splitting field. -/
noncomputable def canonicalShortCubicPoint
    {K Index : Type*} [Field K] [Fintype Index]
    (A B : K) (scale : Index → K)
    [(shortCubicFunctionCurve A B).IsElliptic]
    (index : Index) :
    ((shortCubicFunctionCurve A B).toAffine.baseChange
      (ShortCubicSplittingField K Index A B scale)).Point :=
  WeierstrassCurve.Affine.Point.some
    (algebraMap (ShortCubicFunctionField K)
      (ShortCubicSplittingField K Index A B scale)
      (shortCubicScaledX (scale index)))
    (quadraticRadicandRoot (shortCubicRadicandFamily A B scale) index)
    (shortCubicRoot_nonsingular A B (scale index)
      (quadraticRadicandRoot (shortCubicRadicandFamily A B scale) index)
      (by
        simpa [shortCubicRadicandFamily] using
          (quadraticRadicandRoot_sq
            (shortCubicRadicandFamily A B scale) index)))

/-- Every canonical short-cubic point is an eigenvector for the splitting
field Galois action.  Thus the transformed point relations required by the
sign projector are generated from one ordinary relation. -/
theorem canonicalShortCubicPoint_galois_eigenvector
    {K Index : Type*} [Field K] [Fintype Index]
    (A B : K) (scale : Index → K)
    [(shortCubicFunctionCurve A B).IsElliptic]
    [DecidableEq (ShortCubicSplittingField K Index A B scale)]
    (sigma : ShortCubicSplittingField K Index A B scale ≃ₐ[
      ShortCubicFunctionField K]
        ShortCubicSplittingField K Index A B scale)
    (index : Index) :
    affinePointGaloisAddEquiv
        (shortCubicFunctionCurve A B).toAffine sigma
        (canonicalShortCubicPoint A B scale index) =
      galoisSqrtSign
          (quadraticRadicandRoot
            (shortCubicRadicandFamily A B scale) index) sigma •
        canonicalShortCubicPoint A B scale index := by
  apply shortCubicRoot_galois_eigenvector
  simpa [shortCubicRadicandFamily] using
    quadraticRadicandRoot_sq (shortCubicRadicandFamily A B scale) index

/-- One zero-sum relation among the canonical points generates all of the
Galois sign-twisted relations used by the character projector. -/
theorem canonicalShortCubicPoint_transformedRelations
    {K Index : Type*} [Field K] [Fintype Index]
    (A B : K) (scale : Index → K)
    [(shortCubicFunctionCurve A B).IsElliptic]
    [DecidableEq (ShortCubicSplittingField K Index A B scale)]
    (coefficient : Index → ℤ)
    (relation : ∑ index,
      coefficient index • canonicalShortCubicPoint A B scale index = 0) :
    ∀ sigma : ShortCubicSplittingField K Index A B scale ≃ₐ[
        ShortCubicFunctionField K]
          ShortCubicSplittingField K Index A B scale,
      ∑ index,
        (galoisSqrtSign
            (quadraticRadicandRoot
              (shortCubicRadicandFamily A B scale) index) sigma *
          coefficient index) •
            canonicalShortCubicPoint A B scale index = 0 := by
  exact transformedRelations_of_addEquiv_eigenvectors
    (fun index sigma ↦ galoisSqrtSign
      (quadraticRadicandRoot
        (shortCubicRadicandFamily A B scale) index) sigma)
    coefficient (canonicalShortCubicPoint A B scale)
    (affinePointGaloisAddEquiv
      (shortCubicFunctionCurve A B).toAffine)
    (fun sigma index ↦
      canonicalShortCubicPoint_galois_eigenvector A B scale sigma index)
    relation

/-- Base change of the function-field short curve is the same short curve
with its coefficients mapped into the extension field. -/
theorem shortCubicFunctionCurve_baseChange_eq_shortCharacterCurve
    {K L : Type*} [Field K] [Field L]
    [Algebra K L] [Algebra (ShortCubicFunctionField K) L]
    [IsScalarTower K (ShortCubicFunctionField K) L]
    (A B : K) :
    (shortCubicFunctionCurve A B).toAffine.baseChange L =
      (shortCharacterCurve 0
        (algebraMap K L A) (algebraMap K L B)).toAffine := by
  ext <;>
    simp [shortCubicFunctionCurve, shortCharacterCurve,
      WeierstrassCurve.baseChange,
      IsScalarTower.algebraMap_apply K (ShortCubicFunctionField K) L]

/-- Tower-free form of the preceding base-change identity, with coefficient
maps written as their explicit composite through the function field. -/
theorem shortCubicFunctionCurve_baseChange_eq_shortCharacterCurve_map
    {K L : Type*} [Field K] [Field L]
    [Algebra (ShortCubicFunctionField K) L]
    (A B : K) :
    (shortCubicFunctionCurve A B).toAffine.baseChange L =
      (shortCharacterCurve 0
        (algebraMap (ShortCubicFunctionField K) L
          (algebraMap K (ShortCubicFunctionField K) A))
        (algebraMap (ShortCubicFunctionField K) L
          (algebraMap K (ShortCubicFunctionField K) B))).toAffine := by
  ext <;>
    simp [shortCubicFunctionCurve, shortCharacterCurve,
      WeierstrassCurve.baseChange]

/-- Weierstrass-curve form of the explicit coefficient-map identity. -/
theorem shortCubicFunctionCurve_map_eq_shortCharacterCurve_map
    {K L : Type*} [Field K] [Field L]
    [Algebra (ShortCubicFunctionField K) L]
    (A B : K) :
    (shortCubicFunctionCurve A B).map
        (algebraMap (ShortCubicFunctionField K) L) =
      shortCharacterCurve 0
        (algebraMap (ShortCubicFunctionField K) L
          (algebraMap K (ShortCubicFunctionField K) A))
        (algebraMap (ShortCubicFunctionField K) L
          (algebraMap K (ShortCubicFunctionField K) B)) := by
  ext <;>
    simp [shortCubicFunctionCurve, shortCharacterCurve]

/-- Transport a point across equality of affine curve models. -/
noncomputable def transportAffinePoint
    {K : Type*} [Field K]
    {W W' : WeierstrassCurve.Affine K} (curveEq : W = W') :
    W.Point → W'.Point := by
  subst W'
  exact id

/-- Point x-coordinates are unchanged by transport across equal curve
models. -/
theorem pointX_transportAffinePoint
    {K : Type*} [Field K]
    {W W' : WeierstrassCurve.Affine K} (curveEq : W = W')
    (point : W.Point) :
    pointX (transportAffinePoint curveEq point) = pointX point := by
  subst W'
  rfl

/-- Conjugate an additive point endomorphism across equality of affine curve
models. -/
noncomputable def transportAffinePointEndomorphism
    {K : Type*} [Field K] [DecidableEq K]
    {W W' : WeierstrassCurve.Affine K}
    [W.IsElliptic] [W'.IsElliptic]
    (curveEq : W = W') (endomorphism : W'.Point →+ W'.Point) :
    W.Point →+ W.Point := by
  subst W'
  exact endomorphism

/-- The x-coordinate of a transported endomorphism is computed in the equal
target model after transporting its input point. -/
theorem pointX_transportAffinePointEndomorphism
    {K : Type*} [Field K] [DecidableEq K]
    {W W' : WeierstrassCurve.Affine K}
    [W.IsElliptic] [W'.IsElliptic]
    (curveEq : W = W') (endomorphism : W'.Point →+ W'.Point)
    (point : W.Point) :
    pointX (transportAffinePointEndomorphism curveEq endomorphism point) =
      pointX (endomorphism (transportAffinePoint curveEq point)) := by
  subst W'
  rfl

/-- A pointwise quadratic endomorphism relation is preserved when the curve
model and endomorphism are transported across equality. -/
theorem transportAffinePointEndomorphism_orbit_sum_eq_zero
    {K : Type*} [Field K] [DecidableEq K]
    {W W' : WeierstrassCurve.Affine K}
    [W.IsElliptic] [W'.IsElliptic]
    (curveEq : W = W') (endomorphism : W'.Point →+ W'.Point)
    (point : W.Point)
    (orbitRelation :
      endomorphism (endomorphism (transportAffinePoint curveEq point)) +
        endomorphism (transportAffinePoint curveEq point) +
        transportAffinePoint curveEq point = 0) :
    transportAffinePointEndomorphism curveEq endomorphism
          (transportAffinePointEndomorphism curveEq endomorphism point) +
        transportAffinePointEndomorphism curveEq endomorphism point +
        point = 0 := by
  subst W'
  exact orbitRelation

/-- The standard short-curve CM endomorphism multiplies every nonzero
x-coordinate by `omega`. -/
theorem shortCMEndomorphism_pointX
    {K : Type*} [Field K] [DecidableEq K]
    (B omega : K) [(shortCharacterCurve 0 0 B).IsElliptic]
    (omegaCube : omega ^ 3 = 1)
    (point : (shortCharacterCurve 0 0 B).toAffine.Point) :
    pointX (shortCMEndomorphism B omega omegaCube point) =
      (pointX point).map (fun x ↦ omega * x) := by
  cases point <;> rfl

/-- Cubic-CM endomorphism on the base-changed function-field curve.  The
definition transports the standard short-curve endomorphism across the
explicit coefficient base-change identity. -/
noncomputable def shortCubicCMEndomorphism
    {K L : Type*} [Field K] [Field L]
    [Algebra (ShortCubicFunctionField K) L] [DecidableEq L]
    (B omega : K) [(shortCubicFunctionCurve 0 B).IsElliptic]
    (omegaCube : omega ^ 3 = 1) :
    ((shortCubicFunctionCurve 0 B).toAffine.baseChange L).Point →+
      ((shortCubicFunctionCurve 0 B).toAffine.baseChange L).Point := by
  let W := (shortCubicFunctionCurve 0 B).toAffine.baseChange L
  let B' := algebraMap (ShortCubicFunctionField K) L
    (algebraMap K (ShortCubicFunctionField K) B)
  let omega' := algebraMap (ShortCubicFunctionField K) L
    (algebraMap K (ShortCubicFunctionField K) omega)
  have curveEq : W = (shortCharacterCurve 0 0 B').toAffine := by
    simpa [W, B'] using
      (shortCubicFunctionCurve_baseChange_eq_shortCharacterCurve_map
        (L := L) (0 : K) B)
  have curveEqCurve :
      (shortCubicFunctionCurve 0 B).map
          (algebraMap (ShortCubicFunctionField K) L) =
        shortCharacterCurve 0 0 B' := by
    simpa [B'] using
      (shortCubicFunctionCurve_map_eq_shortCharacterCurve_map
        (L := L) (0 : K) B)
  letI : W.IsElliptic := by
    dsimp [W]
    refine ⟨?_⟩
    change IsUnit (((shortCubicFunctionCurve 0 B).map
      (algebraMap (ShortCubicFunctionField K) L)).Δ)
    rw [WeierstrassCurve.map_Δ]
    exact IsUnit.map (algebraMap (ShortCubicFunctionField K) L)
      (shortCubicFunctionCurve 0 B).isUnit_Δ
  letI : (shortCharacterCurve 0 0 B').IsElliptic := by
    rw [← curveEqCurve]
    infer_instance
  have omegaCube' : omega' ^ 3 = 1 := by
    dsimp [omega']
    rw [← map_pow, ← map_pow, omegaCube, map_one, map_one]
  exact transportAffinePointEndomorphism curveEq
    (shortCMEndomorphism B' omega' omegaCube')

/-- The transported function-field CM endomorphism multiplies point
x-coordinates by the mapped cubic root of unity. -/
theorem shortCubicCMEndomorphism_pointX
    {K L : Type*} [Field K] [Field L]
    [Algebra (ShortCubicFunctionField K) L] [DecidableEq L]
    (B omega : K) [(shortCubicFunctionCurve 0 B).IsElliptic]
    (omegaCube : omega ^ 3 = 1)
    (point : ((shortCubicFunctionCurve 0 B).toAffine.baseChange L).Point) :
    pointX (shortCubicCMEndomorphism B omega omegaCube point) =
      (pointX point).map (fun x ↦
        algebraMap (ShortCubicFunctionField K) L
          (algebraMap K (ShortCubicFunctionField K) omega) * x) := by
  simp [shortCubicCMEndomorphism,
    pointX_transportAffinePointEndomorphism,
    shortCMEndomorphism_pointX, pointX_transportAffinePoint]

/-- The transported cubic-CM endomorphism satisfies the pointwise
quadratic relation `ζ² + ζ + 1 = 0`. -/
theorem shortCubicCM_orbit_sum_eq_zero
    {K L : Type*} [Field K] [Field L]
    [Algebra (ShortCubicFunctionField K) L] [DecidableEq L]
    (B omega : K) [(shortCubicFunctionCurve 0 B).IsElliptic]
    (twoNonzero : (2 : L) ≠ 0)
    (omegaRelation : omega ^ 2 + omega + 1 = 0)
    (omegaNonidentity : omega ≠ 1)
    (point : ((shortCubicFunctionCurve 0 B).toAffine.baseChange L).Point) :
    shortCubicCMEndomorphism B omega
          (cube_eq_one_of_sq_add_self_add_one_eq_zero omegaRelation)
        (shortCubicCMEndomorphism B omega
          (cube_eq_one_of_sq_add_self_add_one_eq_zero omegaRelation) point) +
      shortCubicCMEndomorphism B omega
          (cube_eq_one_of_sq_add_self_add_one_eq_zero omegaRelation) point +
      point = 0 := by
  let B' := algebraMap (ShortCubicFunctionField K) L
    (algebraMap K (ShortCubicFunctionField K) B)
  have curveEqCurve :
      (shortCubicFunctionCurve 0 B).map
          (algebraMap (ShortCubicFunctionField K) L) =
        shortCharacterCurve 0 0 B' := by
    simpa [B'] using
      (shortCubicFunctionCurve_map_eq_shortCharacterCurve_map
        (L := L) (0 : K) B)
  letI : ((shortCubicFunctionCurve 0 B).toAffine.baseChange L).IsElliptic := by
    refine ⟨?_⟩
    change IsUnit (((shortCubicFunctionCurve 0 B).map
      (algebraMap (ShortCubicFunctionField K) L)).Δ)
    rw [WeierstrassCurve.map_Δ]
    exact IsUnit.map (algebraMap (ShortCubicFunctionField K) L)
      (shortCubicFunctionCurve 0 B).isUnit_Δ
  letI : (shortCharacterCurve 0 0 B').IsElliptic := by
    rw [← curveEqCurve]
    infer_instance
  simp only [shortCubicCMEndomorphism]
  apply transportAffinePointEndomorphism_orbit_sum_eq_zero
  apply shortCM_orbit_sum_eq_zero
  · exact twoNonzero
  · have mappedRelation := congrArg
      (fun value : K ↦ algebraMap (ShortCubicFunctionField K) L
        (algebraMap K (ShortCubicFunctionField K) value)) omegaRelation
    simpa only [map_add, map_pow, map_one, map_zero] using mappedRelation
  · intro mappedOne
    apply omegaNonidentity
    apply (algebraMap K (ShortCubicFunctionField K)).injective
    apply (algebraMap (ShortCubicFunctionField K) L).injective
    simpa using mappedOne

/-- A cubic-CM relation between two base-field scales becomes the expected
multiplication relation between their transcendental x-coordinates. -/
theorem shortCubicScaledX_eq_cm_of_scale_eq
    {K : Type*} [Field K] {left right omega : K}
    (scaleRelation : right = omega * left) :
    shortCubicScaledX right =
      algebraMap K (ShortCubicFunctionField K) omega *
        shortCubicScaledX left := by
  rw [scaleRelation]
  simp [shortCubicScaledX, map_mul]
  ring

/-- For canonical splitting-field lifts, a collision partner whose scale is
`omega` times the chosen scale has exactly the x-coordinate of the cubic-CM
image.  This is the coordinate identity consumed by the signed-partner
projector. -/
theorem canonicalShortCubicPoint_partner_pointX_eq_cm
    {K Index : Type*} [Field K] [Fintype Index]
    (B : K) (scale : Index → K)
    [(shortCubicFunctionCurve 0 B).IsElliptic]
    (chosen partner : Index) (omega : K)
    (scaleRelation : scale partner = omega * scale chosen) :
    pointX (canonicalShortCubicPoint 0 B scale partner) =
      some (
        algebraMap (ShortCubicFunctionField K)
            (ShortCubicSplittingField K Index 0 B scale)
            (algebraMap K (ShortCubicFunctionField K) omega) *
          algebraMap (ShortCubicFunctionField K)
            (ShortCubicSplittingField K Index 0 B scale)
            (shortCubicScaledX (scale chosen))) := by
  change some (algebraMap (ShortCubicFunctionField K)
      (ShortCubicSplittingField K Index 0 B scale)
      (shortCubicScaledX (scale partner))) = _
  rw [shortCubicScaledX_eq_cm_of_scale_eq scaleRelation]
  rw [map_mul]

/-- The coordinate collision identity is exactly equality with the image of
the transported cubic-CM endomorphism. -/
theorem canonicalShortCubicPoint_partner_sameX_shortCubicCM
    {K Index : Type*} [Field K] [Fintype Index]
    (B : K) (scale : Index → K)
    [(shortCubicFunctionCurve 0 B).IsElliptic]
    [DecidableEq (ShortCubicSplittingField K Index 0 B scale)]
    (chosen partner : Index) (omega : K)
    (omegaCube : omega ^ 3 = 1)
    (scaleRelation : scale partner = omega * scale chosen) :
    pointX (canonicalShortCubicPoint 0 B scale partner) =
      pointX (shortCubicCMEndomorphism B omega omegaCube
        (canonicalShortCubicPoint 0 B scale chosen)) := by
  rw [shortCubicCMEndomorphism_pointX]
  rw [canonicalShortCubicPoint_partner_pointX_eq_cm
    B scale chosen partner omega scaleRelation]
  rfl

/-- A square-product collision between two distinct `A = 0` radicands
constructs a nontrivial cubic root of unity and the exact canonical-point
x-coordinate identity for its CM partner. -/
theorem canonicalShortCubicPairCollision_implies_cm_pointX
    {K Index : Type*} [Field K] [Fintype Index]
    (hK : ringChar K ≠ 2) (B : K) (scale : Index → K)
    [(shortCubicFunctionCurve 0 B).IsElliptic]
    (cubicNonsingular : (27 : K) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (chosen partner : Index)
    (chosenNonzero : scale chosen ≠ 0)
    (partnerNonzero : scale partner ≠ 0)
    (scalesDistinct : scale chosen ≠ scale partner)
    (pairSquare : IsSquare
      (shortCubicFunctionRadicand 0 B (scale chosen) *
        shortCubicFunctionRadicand 0 B (scale partner))) :
    ∃ omega : K,
      omega ^ 2 + omega + 1 = 0 ∧ omega ≠ 1 ∧
      scale partner = omega * scale chosen ∧
      pointX (canonicalShortCubicPoint 0 B scale partner) =
        some (
          algebraMap (ShortCubicFunctionField K)
              (ShortCubicSplittingField K Index 0 B scale)
              (algebraMap K (ShortCubicFunctionField K) omega) *
            algebraMap (ShortCubicFunctionField K)
              (ShortCubicSplittingField K Index 0 B scale)
              (shortCubicScaledX (scale chosen))) := by
  have polynomialPairSquare : IsSquare
      (algebraMap (Polynomial K) (ShortCubicFunctionField K)
        ((shortCubicPolynomial 0 B).comp
            (Polynomial.C (scale chosen) * Polynomial.X) *
          (shortCubicPolynomial 0 B).comp
            (Polynomial.C (scale partner) * Polynomial.X))) := by
    simpa only [shortCubicFunctionRadicand, ← map_mul] using pairSquare
  obtain ⟨omega, omegaRelation, omegaNonidentity, scaleRelation⟩ :=
    shortCubicPolynomial_scaled_pair_CM_ratio_of_isSquare
      hK B (scale chosen) (scale partner) cubicNonsingular BNonzero
      chosenNonzero partnerNonzero scalesDistinct polynomialPairSquare
  exact ⟨omega, omegaRelation, omegaNonidentity, scaleRelation,
    canonicalShortCubicPoint_partner_pointX_eq_cm
      B scale chosen partner omega scaleRelation⟩

/-- End-to-end same-x form of the canonical `A = 0` collision: the square
product itself constructs the CM endomorphism whose image shares the
partner's x-coordinate. -/
theorem canonicalShortCubicPairCollision_implies_cm_sameX
    {K Index : Type*} [Field K] [Fintype Index]
    (hK : ringChar K ≠ 2) (B : K) (scale : Index → K)
    [(shortCubicFunctionCurve 0 B).IsElliptic]
    [DecidableEq (ShortCubicSplittingField K Index 0 B scale)]
    (cubicNonsingular : (27 : K) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (chosen partner : Index)
    (chosenNonzero : scale chosen ≠ 0)
    (partnerNonzero : scale partner ≠ 0)
    (scalesDistinct : scale chosen ≠ scale partner)
    (pairSquare : IsSquare
      (shortCubicFunctionRadicand 0 B (scale chosen) *
        shortCubicFunctionRadicand 0 B (scale partner))) :
    ∃ omega : K, ∃ omegaCube : omega ^ 3 = 1,
      omega ^ 2 + omega + 1 = 0 ∧ omega ≠ 1 ∧
      scale partner = omega * scale chosen ∧
      pointX (canonicalShortCubicPoint 0 B scale partner) =
        pointX (shortCubicCMEndomorphism B omega omegaCube
          (canonicalShortCubicPoint 0 B scale chosen)) := by
  obtain ⟨omega, omegaRelation, omegaNonidentity, scaleRelation, -⟩ :=
    canonicalShortCubicPairCollision_implies_cm_pointX
      hK B scale cubicNonsingular BNonzero chosen partner
      chosenNonzero partnerNonzero scalesDistinct pairSquare
  let omegaCube :=
    cube_eq_one_of_sq_add_self_add_one_eq_zero omegaRelation
  exact ⟨omega, omegaCube, omegaRelation, omegaNonidentity, scaleRelation,
    canonicalShortCubicPoint_partner_sameX_shortCubicCM
      B scale chosen partner omega omegaCube scaleRelation⟩

/-- A single equivariant point relation suffices for the cubic-CM collision
branch.  The Galois action generates every sign-twisted relation, the pair
projector isolates the collision, and a supplied signed CM-partner identity
turns the pair relation into Eisenstein norm torsion. -/
theorem galoisPairEquivariantRelation_implies_norm_torsion
    {K L Index Point : Type*}
    [Field K] [Field L] [Algebra K L]
    [FiniteDimensional K L] [IsGalois K L]
    [Fintype Index] [DecidableEq Index] [AddCommGroup Point]
    (radicand : Index → K) (root : Index → L)
    (coefficient : Index → ℤ) (point : Index → Point)
    (chosen partner : Index) (distinct : chosen ≠ partner)
    (rootNonzero : ∀ index, root index ≠ 0)
    (twoNonzero : (2 : L) ≠ 0)
    (rootSquare : ∀ index,
      root index ^ 2 = algebraMap K L (radicand index))
    (pairSquare : IsSquare (radicand chosen * radicand partner))
    (otherNonsquare : ∀ index,
      index ≠ chosen → index ≠ partner →
        ¬ IsSquare (radicand chosen * radicand index))
    (action : (L ≃ₐ[K] L) → Point ≃+ Point)
    (pointAction : ∀ sigma index,
      action sigma (point index) =
        galoisSqrtSign (root index) sigma • point index)
    (relation : ∑ index, coefficient index • point index = 0)
    (zeta : Point →+ Point) (cmSign : ℤ)
    (orbitRelation :
      zeta (zeta (point chosen)) + zeta (point chosen) + point chosen = 0)
    (partnerRelation : point partner = cmSign • zeta (point chosen)) :
    eisensteinNorm
      ((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient chosen)
      (((Fintype.card (L ≃ₐ[K] L) : ℤ) *
        coefficient partner) * cmSign) • point chosen = 0 := by
  have transformedRelation : ∀ sigma : L ≃ₐ[K] L,
      ∑ index,
        (galoisSqrtSign (root index) sigma * coefficient index) •
          point index = 0 :=
    transformedRelations_of_addEquiv_eigenvectors
      (fun index sigma ↦ galoisSqrtSign (root index) sigma)
      coefficient point action pointAction relation
  have pairRelation :=
    galoisTwoCollisionSignCharacterProjector_isolates_pair_of_isSquare_mul
      radicand root coefficient point chosen partner distinct rootNonzero
      twoNonzero rootSquare pairSquare otherNonsquare transformedRelation
  have linearRelation :
      ((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient chosen) •
          point chosen +
        (((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient partner) *
          cmSign) • zeta (point chosen) = 0 := by
    rw [partnerRelation] at pairRelation
    simpa only [smul_smul] using pairRelation
  simpa only [eisensteinNorm] using
    cubicCMRelation_implies_norm_torsion zeta (point chosen)
      ((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient chosen)
      (((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient partner) *
        cmSign)
      orbitRelation linearRelation

/-- Coordinate-level form of the equivariant cubic-CM projector.  Equality of
the partner x-coordinate with the CM image constructs the missing sign
internally. -/
theorem galoisPairEquivariantRelation_sameX_implies_norm_torsion
    {K L F Index : Type*}
    [Field K] [Field L] [Algebra K L]
    [FiniteDimensional K L] [IsGalois K L]
    [Field F] [DecidableEq F]
    [Fintype Index] [DecidableEq Index]
    (W : WeierstrassCurve.Affine F)
    (radicand : Index → K) (root : Index → L)
    (coefficient : Index → ℤ) (point : Index → W.Point)
    (chosen partner : Index) (distinct : chosen ≠ partner)
    (rootNonzero : ∀ index, root index ≠ 0)
    (twoNonzero : (2 : L) ≠ 0)
    (rootSquare : ∀ index,
      root index ^ 2 = algebraMap K L (radicand index))
    (pairSquare : IsSquare (radicand chosen * radicand partner))
    (otherNonsquare : ∀ index,
      index ≠ chosen → index ≠ partner →
        ¬ IsSquare (radicand chosen * radicand index))
    (action : (L ≃ₐ[K] L) → W.Point ≃+ W.Point)
    (pointAction : ∀ sigma index,
      action sigma (point index) =
        galoisSqrtSign (root index) sigma • point index)
    (relation : ∑ index, coefficient index • point index = 0)
    (zeta : W.Point →+ W.Point)
    (orbitRelation :
      zeta (zeta (point chosen)) + zeta (point chosen) + point chosen = 0)
    (partnerSameX :
      pointX (point partner) = pointX (zeta (point chosen))) :
    ∃ sign : ℤ, (sign = 1 ∨ sign = -1) ∧
      eisensteinNorm
        ((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient chosen)
        (((Fintype.card (L ≃ₐ[K] L) : ℤ) * coefficient partner) *
          sign) • point chosen = 0 := by
  have signedPartner : ∃ sign : ℤ,
      (sign = 1 ∨ sign = -1) ∧
      point partner = sign • zeta (point chosen) := by
    rcases GeneralS4Audit.same_pointX_eq_or_neg W partnerSameX with equal | negated
    · exact ⟨1, Or.inl rfl, by simpa using equal⟩
    · exact ⟨-1, Or.inr rfl, by simpa using negated⟩
  obtain ⟨sign, signUnit, partnerRelation⟩ := signedPartner
  refine ⟨sign, signUnit, ?_⟩
  exact galoisPairEquivariantRelation_implies_norm_torsion
    radicand root coefficient point chosen partner distinct rootNonzero
    twoNonzero rootSquare pairSquare otherNonsquare action pointAction relation
    zeta sign orbitRelation partnerRelation

/-- Record whether a splitting-field automorphism fixes each selected
quadratic root. -/
noncomputable def quadraticRootFixedBits
    {K Index : Type*} [Field K] [Fintype Index]
    (radicand : Index → K)
    (sigma : (quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
      (quadraticRadicandPolynomial radicand).SplittingField) :
    Index → Bool :=
  by
    classical
    exact fun index ↦ decide
      (sigma (quadraticRadicandRoot radicand index) =
        quadraticRadicandRoot radicand index)

/-- The selected-root fixed-bit vector determines every automorphism of the
quadratic splitting field, including when factors repeat. -/
theorem quadraticRootFixedBits_injective
    {K Index : Type*} [Field K] [Fintype Index]
    (radicand : Index → K) :
    Function.Injective (quadraticRootFixedBits radicand) := by
  classical
  let p := quadraticRadicandPolynomial radicand
  let L := p.SplittingField
  intro sigma tau bitsEqual
  have selectedRootAction : ∀ index,
      sigma (quadraticRadicandRoot radicand index) =
        tau (quadraticRadicandRoot radicand index) := by
    intro index
    let root := quadraticRadicandRoot radicand index
    have rootSquare : root ^ 2 = algebraMap K L (radicand index) := by
      exact quadraticRadicandRoot_sq radicand index
    have fixedIff : sigma root = root ↔ tau root = root := by
      constructor
      · intro sigmaFixed
        have bitTrue : quadraticRootFixedBits radicand sigma index = true := by
          simp [quadraticRootFixedBits, root, sigmaFixed]
        rw [bitsEqual] at bitTrue
        simpa [quadraticRootFixedBits, root] using bitTrue
      · intro tauFixed
        have bitTrue : quadraticRootFixedBits radicand tau index = true := by
          simp [quadraticRootFixedBits, root, tauFixed]
        rw [← bitsEqual] at bitTrue
        simpa [quadraticRootFixedBits, root] using bitTrue
    by_cases sigmaFixed : sigma root = root
    · rw [sigmaFixed, (fixedIff.mp sigmaFixed)]
    · have tauNotFixed : tau root ≠ root :=
        fun tauFixed ↦ sigmaFixed (fixedIff.mpr tauFixed)
      have sigmaNeg : sigma root = -root := by
        simpa [galoisSqrtSign, sigmaFixed] using
          galoisSqrtSign_action rootSquare sigma
      have tauNeg : tau root = -root := by
        simpa [galoisSqrtSign, tauNotFixed] using
          galoisSqrtSign_action rootSquare tau
      rw [sigmaNeg, tauNeg]
  have algHomEqual : sigma.toAlgHom = tau.toAlgHom := by
    apply AlgHom.ext_of_adjoin_eq_top
      (Polynomial.IsSplittingField.adjoin_rootSet L p)
    intro root rootMem
    have evaluation : Polynomial.aeval root p = 0 :=
      Polynomial.aeval_eq_zero_of_mem_rootSet rootMem
    have factorProduct : ∏ index,
        (root ^ 2 - algebraMap K L (radicand index)) = 0 := by
      simpa [p, quadraticRadicandPolynomial, map_prod] using evaluation
    obtain ⟨index, -, factorZero⟩ := Finset.prod_eq_zero_iff.mp factorProduct
    have rootSquare : root ^ 2 =
        (quadraticRadicandRoot radicand index) ^ 2 := by
      rw [quadraticRadicandRoot_sq radicand index]
      exact sub_eq_zero.mp factorZero
    rcases sq_eq_sq_iff_eq_or_eq_neg.mp rootSquare with equal | negated
    · simpa [equal] using selectedRootAction index
    · rw [negated, map_neg, map_neg]
      exact congrArg (fun value ↦ -value) (selectedRootAction index)
  apply AlgEquiv.ext
  intro value
  exact DFunLike.congr_fun algHomEqual value

/-- A splitting field generated by `m` quadratic roots has at most `2^m`
base-field automorphisms, even when some indexed radicands repeat. -/
theorem quadraticRadicandSplittingField_card_aut_le_two_pow
    {K Index : Type*} [Field K] [Fintype Index]
    (radicand : Index → K) :
    Fintype.card
        ((quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
          (quadraticRadicandPolynomial radicand).SplittingField) ≤
      2 ^ Fintype.card Index := by
  classical
  calc
    Fintype.card
        ((quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
          (quadraticRadicandPolynomial radicand).SplittingField) ≤
        Fintype.card (Index → Bool) :=
      Fintype.card_le_of_injective (quadraticRootFixedBits radicand)
        (quadraticRootFixedBits_injective radicand)
    _ = 2 ^ Fintype.card Index := by rw [Fintype.card_fun, Fintype.card_bool]

/-- The arity-only inequality with `2^m` implies the exact Galois-cardinality
threshold used by the projected Eisenstein norm bound. -/
theorem quadraticRadicand_norm_threshold_of_arity
    {K Index : Type*} [Field K] [Fintype Index]
    (radicand : Index → K) (modulus : ℕ)
    (arityBelow :
      3 * (2 ^ Fintype.card Index * Fintype.card Index) ^ 2 < modulus) :
    3 * (Fintype.card
      ((quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
        (quadraticRadicandPolynomial radicand).SplittingField) *
      Fintype.card Index) ^ 2 < modulus := by
  have productBound :
      Fintype.card
          ((quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
            (quadraticRadicandPolynomial radicand).SplittingField) *
          Fintype.card Index ≤
        2 ^ Fintype.card Index * Fintype.card Index :=
    Nat.mul_le_mul_right (Fintype.card Index)
      (quadraticRadicandSplittingField_card_aut_le_two_pow radicand)
  exact (Nat.mul_le_mul_left 3 (Nat.pow_le_pow_left productBound 2)).trans_lt
    arityBelow

/-- Concrete canonical-point composition for the `A = 0` collision branch.
One ordinary relation among the selected splitting-field points, one square
collision, and nonsquare separation of every other index yield an explicit
Eisenstein norm-torsion certificate.  Nonsingularity, Galois equivariance,
the CM partner, and the CM orbit relation are constructed internally. -/
theorem canonicalShortCubicPairCollision_implies_norm_torsion
    {K Index : Type*} [Field K] [Fintype Index] [DecidableEq Index]
    (hK : ringChar K ≠ 2) (B : K) (scale : Index → K)
    [(shortCubicFunctionCurve 0 B).IsElliptic]
    [DecidableEq (ShortCubicSplittingField K Index 0 B scale)]
    (twoNonzeroBase : (2 : K) ≠ 0)
    (twoNonzero :
      (2 : ShortCubicSplittingField K Index 0 B scale) ≠ 0)
    (cubicNonsingular : (27 : K) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (chosen partner : Index) (distinct : chosen ≠ partner)
    (scaleNonzero : ∀ index, scale index ≠ 0)
    (scaleInjective : Function.Injective scale)
    (radicandNonzero : ∀ index,
      shortCubicRadicandFamily 0 B scale index ≠ 0)
    (pairSquare : IsSquare
      (shortCubicRadicandFamily 0 B scale chosen *
        shortCubicRadicandFamily 0 B scale partner))
    (otherNonsquare : ∀ index,
      index ≠ chosen → index ≠ partner →
        ¬ IsSquare
          (shortCubicRadicandFamily 0 B scale chosen *
            shortCubicRadicandFamily 0 B scale index))
    (coefficient : Index → ℤ)
    (relation : ∑ index,
      coefficient index • canonicalShortCubicPoint 0 B scale index = 0) :
    ∃ omega : K, ∃ sign : ℤ,
      omega ^ 2 + omega + 1 = 0 ∧ omega ≠ 1 ∧
      scale partner = omega * scale chosen ∧
      (sign = 1 ∨ sign = -1) ∧
      eisensteinNorm
        ((Fintype.card
          (ShortCubicSplittingField K Index 0 B scale ≃ₐ[
            ShortCubicFunctionField K]
              ShortCubicSplittingField K Index 0 B scale) : ℤ) *
          coefficient chosen)
        (((Fintype.card
          (ShortCubicSplittingField K Index 0 B scale ≃ₐ[
            ShortCubicFunctionField K]
              ShortCubicSplittingField K Index 0 B scale) : ℤ) *
          coefficient partner) * sign) •
            canonicalShortCubicPoint 0 B scale chosen = 0 := by
  have twoNonzeroFunctionField :
      (2 : ShortCubicFunctionField K) ≠ 0 := by
    rw [← map_ofNat (algebraMap K (ShortCubicFunctionField K)) 2]
    exact (map_ne_zero (algebraMap K (ShortCubicFunctionField K))).2
      twoNonzeroBase
  letI : IsGalois (ShortCubicFunctionField K)
      (ShortCubicSplittingField K Index 0 B scale) :=
    quadraticRadicandSplittingField_isGalois_allow_repeats
      (shortCubicRadicandFamily 0 B scale) twoNonzeroFunctionField
      radicandNonzero
  obtain ⟨omega, omegaCube, omegaRelation, omegaNonidentity,
      scaleRelation, partnerSameX⟩ :=
    canonicalShortCubicPairCollision_implies_cm_sameX
      hK B scale cubicNonsingular BNonzero chosen partner
      (scaleNonzero chosen) (scaleNonzero partner)
      (fun scaleEqual ↦ distinct (scaleInjective scaleEqual)) pairSquare
  obtain ⟨sign, signUnit, torsion⟩ :=
    galoisPairEquivariantRelation_sameX_implies_norm_torsion
      ((shortCubicFunctionCurve 0 B).toAffine.baseChange
        (ShortCubicSplittingField K Index 0 B scale))
      (shortCubicRadicandFamily 0 B scale)
      (quadraticRadicandRoot (shortCubicRadicandFamily 0 B scale))
      coefficient (canonicalShortCubicPoint 0 B scale)
      chosen partner distinct
      (fun index ↦ quadraticRadicandRoot_ne_zero
        (shortCubicRadicandFamily 0 B scale) index
        (radicandNonzero index))
      twoNonzero
      (fun index ↦ quadraticRadicandRoot_sq
        (shortCubicRadicandFamily 0 B scale) index)
      pairSquare otherNonsquare
      (affinePointGaloisAddEquiv
        (shortCubicFunctionCurve 0 B).toAffine)
      (fun sigma index ↦
        canonicalShortCubicPoint_galois_eigenvector
          0 B scale sigma index)
      relation
      (shortCubicCMEndomorphism B omega omegaCube)
      (shortCubicCM_orbit_sum_eq_zero B omega twoNonzero
        omegaRelation omegaNonidentity
        (canonicalShortCubicPoint 0 B scale chosen))
      partnerSameX
  exact ⟨omega, sign, omegaRelation, omegaNonidentity, scaleRelation,
    signUnit, torsion⟩

/-- Concrete ordinary Kummer branch for the canonical short-cubic point
family. Pairwise nonsquare radicand products isolate any selected term; the
`2^m` automorphism bound turns the arity-only characteristic inequality into
a nonzero projected torsion index. -/
theorem zmodCanonicalShortCubicPairwiseNonsquare_implies_nonzero_index_torsion
    (modulus : ℕ) [Fact modulus.Prime] {Index : Type*}
    [Fintype Index] [DecidableEq Index]
    (B : ZMod modulus) (scale : Index → ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    [DecidableEq (ShortCubicSplittingField (ZMod modulus) Index 0 B scale)]
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (radicandNonzero : ∀ index,
      shortCubicRadicandFamily 0 B scale index ≠ 0)
    (pairwiseNonsquare : ∀ left right, left ≠ right →
      ¬ IsSquare
        (shortCubicRadicandFamily 0 B scale left *
          shortCubicRadicandFamily 0 B scale right))
    (coefficient : Index → ℤ)
    (coefficientUnit : ∀ index,
      coefficient index = 1 ∨ coefficient index = -1)
    (relation : ∑ index,
      coefficient index • canonicalShortCubicPoint 0 B scale index = 0)
    (chosen : Index)
    (arityBelow :
      2 ^ Fintype.card Index * Fintype.card Index < modulus) :
    ((((Fintype.card
      (ShortCubicSplittingField (ZMod modulus) Index 0 B scale ≃ₐ[
        ShortCubicFunctionField (ZMod modulus)]
        ShortCubicSplittingField (ZMod modulus) Index 0 B scale) : ℤ) *
      coefficient chosen : ℤ)) : ZMod modulus) ≠ 0 ∧
    ((Fintype.card
      (ShortCubicSplittingField (ZMod modulus) Index 0 B scale ≃ₐ[
        ShortCubicFunctionField (ZMod modulus)]
        ShortCubicSplittingField (ZMod modulus) Index 0 B scale) : ℤ) *
      coefficient chosen) •
        canonicalShortCubicPoint 0 B scale chosen = 0 := by
  have twoNonzeroFunctionField :
      (2 : ShortCubicFunctionField (ZMod modulus)) ≠ 0 := by
    rw [← map_ofNat
      (algebraMap (ZMod modulus)
        (ShortCubicFunctionField (ZMod modulus))) 2]
    exact (map_ne_zero
      (algebraMap (ZMod modulus)
        (ShortCubicFunctionField (ZMod modulus)))).2 twoNonzeroBase
  have torsion :
      ((Fintype.card
        (ShortCubicSplittingField (ZMod modulus) Index 0 B scale ≃ₐ[
          ShortCubicFunctionField (ZMod modulus)]
          ShortCubicSplittingField (ZMod modulus) Index 0 B scale) : ℤ) *
        coefficient chosen) •
          canonicalShortCubicPoint 0 B scale chosen = 0 := by
    apply quadraticSplittingFieldAffinePointProjector_isolates_term
      (shortCubicFunctionCurve 0 B).toAffine
      (by simp [shortCubicFunctionCurve, shortCharacterCurve])
      (by simp [shortCubicFunctionCurve, shortCharacterCurve])
      (shortCubicRadicandFamily 0 B scale)
      (fun index ↦ shortCubicScaledX (scale index))
      coefficient chosen twoNonzeroFunctionField radicandNonzero
      pairwiseNonsquare
      (fun index ↦ shortCubicRoot_nonsingular 0 B (scale index)
        (quadraticRadicandRoot (shortCubicRadicandFamily 0 B scale) index)
        (by simpa [shortCubicRadicandFamily] using
          (quadraticRadicandRoot_sq
            (shortCubicRadicandFamily 0 B scale) index)))
    exact relation
  have cardBound := quadraticRadicandSplittingField_card_aut_le_two_pow
    (shortCubicRadicandFamily 0 B scale)
  have projectedBelow :
      Fintype.card
          (ShortCubicSplittingField (ZMod modulus) Index 0 B scale ≃ₐ[
            ShortCubicFunctionField (ZMod modulus)]
            ShortCubicSplittingField (ZMod modulus) Index 0 B scale) *
          Fintype.card Index < modulus :=
    (Nat.mul_le_mul_right (Fintype.card Index) cardBound).trans_lt arityBelow
  have chosenNonzero : coefficient chosen ≠ 0 := by
    rcases coefficientUnit chosen with equal | equal <;> simp [equal]
  have chosenBound : (coefficient chosen).natAbs ≤ Fintype.card Index := by
    have cardPositive : 0 < Fintype.card Index := Fintype.card_pos_iff.mpr ⟨chosen⟩
    have oneLe : 1 ≤ Fintype.card Index := cardPositive
    rcases coefficientUnit chosen with equal | equal <;>
      simpa [equal] using oneLe
  exact projectedTorsion_index_ne_zero
    (K := ZMod modulus)
    (Gamma := ShortCubicSplittingField (ZMod modulus) Index 0 B scale ≃ₐ[
      ShortCubicFunctionField (ZMod modulus)]
      ShortCubicSplittingField (ZMod modulus) Index 0 B scale)
    modulus (Fintype.card Index) (coefficient chosen)
    (canonicalShortCubicPoint 0 B scale chosen) chosenNonzero chosenBound
    projectedBelow torsion

/-- Prime-field interval specialization of the canonical `A = 0` branch.
The interval hypotheses construct every nonzero/nonsquare premise and the
quadratic splitting field internally; the only geometric input left is one
ordinary relation among the canonical points. -/
theorem zmodCanonicalShortCubicIntervalPairCollision_implies_norm_torsion
    (modulus X : ℕ) [Fact modulus.Prime]
    {Index : Type*} [Fintype Index] [DecidableEq Index]
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (scale : Index → ℕ)
    [DecidableEq (ShortCubicSplittingField (ZMod modulus) Index 0 B
      (fun i ↦ (scale i : ZMod modulus)))]
    (scalePositive : ∀ index, 0 < scale index)
    (scaleBound : ∀ index, scale index ≤ X)
    (scaleInjective : Function.Injective scale)
    (intervalShort : 3 * X < modulus)
    (chosen partner : Index) (distinct : chosen ≠ partner)
    (pairSquare : IsSquare
      (shortCubicFunctionRadicand 0 B
          (scale chosen : ZMod modulus) *
        shortCubicFunctionRadicand 0 B
          (scale partner : ZMod modulus)))
    (coefficient : Index → ℤ)
    (relation : ∑ index, coefficient index •
      canonicalShortCubicPoint 0 B
        (fun i ↦ (scale i : ZMod modulus)) index = 0) :
    ∃ omega : ZMod modulus, ∃ sign : ℤ,
      omega ^ 2 + omega + 1 = 0 ∧ omega ≠ 1 ∧
      (scale partner : ZMod modulus) =
        omega * (scale chosen : ZMod modulus) ∧
      (sign = 1 ∨ sign = -1) ∧
      eisensteinNorm
        ((Fintype.card
          (ShortCubicSplittingField (ZMod modulus) Index 0 B
              (fun i ↦ (scale i : ZMod modulus)) ≃ₐ[
            ShortCubicFunctionField (ZMod modulus)]
            ShortCubicSplittingField (ZMod modulus) Index 0 B
              (fun i ↦ (scale i : ZMod modulus))) : ℤ) *
          coefficient chosen)
        (((Fintype.card
          (ShortCubicSplittingField (ZMod modulus) Index 0 B
              (fun i ↦ (scale i : ZMod modulus)) ≃ₐ[
            ShortCubicFunctionField (ZMod modulus)]
            ShortCubicSplittingField (ZMod modulus) Index 0 B
              (fun i ↦ (scale i : ZMod modulus))) : ℤ) *
          coefficient partner) * sign) •
        canonicalShortCubicPoint 0 B
          (fun i ↦ (scale i : ZMod modulus)) chosen = 0 := by
  classical
  let scaleZ : Index → ZMod modulus := fun index ↦ scale index
  have scaleBelow : ∀ index, scale index < modulus := by
    intro index
    have := scaleBound index
    omega
  have scaleNonzero : ∀ index, scaleZ index ≠ 0 := by
    intro index castZero
    have values := congrArg ZMod.val castZero
    rw [ZMod.val_natCast_of_lt (scaleBelow index), ZMod.val_zero] at values
    exact (Nat.ne_of_gt (scalePositive index)) values
  have scaleZInjective : Function.Injective scaleZ := by
    intro left right castEqual
    apply scaleInjective
    have values := congrArg ZMod.val castEqual
    simpa [scaleZ, ZMod.val_natCast_of_lt (scaleBelow left),
      ZMod.val_natCast_of_lt (scaleBelow right)] using values
  have twoNonzeroExtension :
      (2 : ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ) ≠ 0 := by
    have twoNonzeroFunctionField :
        (2 : ShortCubicFunctionField (ZMod modulus)) ≠ 0 := by
      rw [← map_ofNat
        (algebraMap (ZMod modulus)
          (ShortCubicFunctionField (ZMod modulus))) 2]
      exact (map_ne_zero
        (algebraMap (ZMod modulus)
          (ShortCubicFunctionField (ZMod modulus)))).2 twoNonzeroBase
    rw [← map_ofNat
      (algebraMap (ShortCubicFunctionField (ZMod modulus))
        (ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ)) 2]
    exact (map_ne_zero
      (algebraMap (ShortCubicFunctionField (ZMod modulus))
        (ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ))).2
      twoNonzeroFunctionField
  apply canonicalShortCubicPairCollision_implies_norm_torsion
    hChar B scaleZ twoNonzeroBase twoNonzeroExtension cubicNonsingular
      BNonzero chosen partner distinct scaleNonzero scaleZInjective
  · intro index
    exact shortCubicFunctionRadicand_ne_zero 0 B (scaleZ index)
      (scaleNonzero index)
  · exact pairSquare
  · exact zmodShortCubicFunctionRadicand_mul_not_isSquare_of_A_eq_zero_outside_partner
      modulus X hChar B cubicNonsingular BNonzero scale scalePositive
      scaleBound scaleInjective intervalShort chosen partner distinct pairSquare
  · exact relation

/-- A bounded relation among distinct interval-scale canonical points is
impossible once one coefficient survives.  If the chosen radicand is
separated from every other radicand, the chosen-term Kummer projector applies
even when unchosen radicands collide with each other.  Otherwise the existing
cubic-CM pair projector applies to a square-product partner. -/
theorem zmodCanonicalShortCubicInterval_relation_impossible_of_no_torsion
    (modulus X arity : ℕ) [Fact modulus.Prime]
    {Index : Type*} [Fintype Index] [DecidableEq Index]
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (scale : Index → ℕ)
    [DecidableEq (ShortCubicSplittingField (ZMod modulus) Index 0 B
      (fun i ↦ (scale i : ZMod modulus)))]
    (scalePositive : ∀ index, 0 < scale index)
    (scaleBound : ∀ index, scale index ≤ X)
    (scaleInjective : Function.Injective scale)
    (intervalShort : 3 * X < modulus)
    (coefficient : Index → ℤ)
    (relation : ∑ index, coefficient index •
      canonicalShortCubicPoint 0 B
        (fun i ↦ (scale i : ZMod modulus)) index = 0)
    (chosen : Index) (chosenNonzero : coefficient chosen ≠ 0)
    (coefficientBound : ∀ index, (coefficient index).natAbs ≤ arity)
    (noNonzeroTorsion : ∀ (pointIndex : Index) (index : ℤ),
      (index : ZMod modulus) ≠ 0 →
      index • canonicalShortCubicPoint 0 B
        (fun i ↦ (scale i : ZMod modulus)) pointIndex = 0 → False)
    (arityBelow :
      3 * (2 ^ Fintype.card Index * arity) ^ 2 < modulus) :
    False := by
  classical
  let scaleZ : Index → ZMod modulus := fun index ↦ scale index
  have scaleBelow : ∀ index, scale index < modulus := by
    intro index
    have := scaleBound index
    omega
  have scaleNonzero : ∀ index, scaleZ index ≠ 0 := by
    intro index castZero
    have values := congrArg ZMod.val castZero
    rw [ZMod.val_natCast_of_lt (scaleBelow index), ZMod.val_zero] at values
    exact (Nat.ne_of_gt (scalePositive index)) values
  have radicandNonzero : ∀ index,
      shortCubicRadicandFamily 0 B scaleZ index ≠ 0 := by
    intro index
    exact shortCubicFunctionRadicand_ne_zero 0 B (scaleZ index)
      (scaleNonzero index)
  have productPositive : 0 < 2 ^ Fintype.card Index * arity := by
    have arityPositive : 0 < arity := by
      have := coefficientBound chosen
      have : 0 < (coefficient chosen).natAbs :=
        Int.natAbs_pos.mpr chosenNonzero
      omega
    positivity
  have cardProductBound :
      Fintype.card
          (ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ ≃ₐ[
            ShortCubicFunctionField (ZMod modulus)]
            ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ) *
          arity ≤
        2 ^ Fintype.card Index * arity :=
    Nat.mul_le_mul_right arity
      (quadraticRadicandSplittingField_card_aut_le_two_pow
        (shortCubicRadicandFamily 0 B scaleZ))
  by_cases chosenSeparated : ∀ index, index ≠ chosen →
      ¬ IsSquare
        (shortCubicRadicandFamily 0 B scaleZ chosen *
          shortCubicRadicandFamily 0 B scaleZ index)
  · have twoNonzeroFunctionField :
        (2 : ShortCubicFunctionField (ZMod modulus)) ≠ 0 := by
      rw [← map_ofNat
        (algebraMap (ZMod modulus)
          (ShortCubicFunctionField (ZMod modulus))) 2]
      exact (map_ne_zero
        (algebraMap (ZMod modulus)
          (ShortCubicFunctionField (ZMod modulus)))).2 twoNonzeroBase
    have torsion :
        ((Fintype.card
          (ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ ≃ₐ[
            ShortCubicFunctionField (ZMod modulus)]
            ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ) : ℤ) *
          coefficient chosen) •
            canonicalShortCubicPoint 0 B scaleZ chosen = 0 := by
      apply quadraticSplittingFieldSignProjector_isolates_chosen
        (shortCubicRadicandFamily 0 B scaleZ) coefficient
        (canonicalShortCubicPoint 0 B scaleZ) chosen
        twoNonzeroFunctionField radicandNonzero chosenSeparated
        (affinePointGaloisAddEquiv
          (shortCubicFunctionCurve 0 B).toAffine)
      · intro sigma index
        exact canonicalShortCubicPoint_galois_eigenvector
          0 B scaleZ sigma index
      · exact relation
    have projectedBelow :
        Fintype.card
            (ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ ≃ₐ[
              ShortCubicFunctionField (ZMod modulus)]
              ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ) *
            arity < modulus := by
      calc
        _ ≤ 2 ^ Fintype.card Index * arity := cardProductBound
        _ ≤ 3 * (2 ^ Fintype.card Index * arity) ^ 2 := by
          nlinarith
        _ < modulus := arityBelow
    have indexedTorsion := projectedTorsion_index_ne_zero
      (K := ZMod modulus)
      (Gamma := ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ ≃ₐ[
        ShortCubicFunctionField (ZMod modulus)]
        ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ)
      modulus arity (coefficient chosen)
      (canonicalShortCubicPoint 0 B scaleZ chosen) chosenNonzero
      (coefficientBound chosen) projectedBelow torsion
    exact noNonzeroTorsion chosen
      ((Fintype.card
        (ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ ≃ₐ[
          ShortCubicFunctionField (ZMod modulus)]
          ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ) : ℤ) *
        coefficient chosen) indexedTorsion.1 indexedTorsion.2
  · push Not at chosenSeparated
    obtain ⟨partner, distinct, pairSquare⟩ := chosenSeparated
    obtain ⟨omega, sign, -, -, -, signUnit, torsion⟩ :=
      zmodCanonicalShortCubicIntervalPairCollision_implies_norm_torsion
        modulus X hChar twoNonzeroBase B cubicNonsingular BNonzero scale
        scalePositive scaleBound scaleInjective intervalShort chosen partner
        distinct.symm pairSquare coefficient relation
    have exactNormBelow :
        3 * (Fintype.card
          (ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ ≃ₐ[
            ShortCubicFunctionField (ZMod modulus)]
            ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ) *
          arity) ^ 2 < modulus := by
      exact (Nat.mul_le_mul_left 3
        (Nat.pow_le_pow_left cardProductBound 2)).trans_lt arityBelow
    have indexedTorsion := projectedEisensteinNorm_torsion_index_ne_zero
      (K := ZMod modulus)
      (Gamma := ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ ≃ₐ[
        ShortCubicFunctionField (ZMod modulus)]
        ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ)
      modulus arity (coefficient chosen) (coefficient partner) sign
      (canonicalShortCubicPoint 0 B scaleZ chosen) chosenNonzero
      (coefficientBound chosen) (coefficientBound partner) signUnit
      exactNormBelow torsion
    exact noNonzeroTorsion chosen
      (eisensteinNorm
        ((Fintype.card
          (ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ ≃ₐ[
            ShortCubicFunctionField (ZMod modulus)]
            ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ) : ℤ) *
          coefficient chosen)
        (((Fintype.card
          (ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ ≃ₐ[
            ShortCubicFunctionField (ZMod modulus)]
            ShortCubicSplittingField (ZMod modulus) Index 0 B scaleZ) : ℤ) *
          coefficient partner) * sign)) indexedTorsion.1 indexedTorsion.2

/-- A signed relation of points descends along an algebra embedding.  Point
base change is injective, so it is enough to prove the mapped relation in a
larger field such as an algebraic closure. -/
theorem signed_point_relation_of_mapped_relation
    {K L M : Type*} [Field K] [Field L] [Field M]
    [Algebra K L] [Algebra K M] [DecidableEq L] [DecidableEq M]
    (W : WeierstrassCurve.Affine K) {n : ℕ}
    (f : L →ₐ[K] M) (point : Fin n → (W.baseChange L).Point)
    (coefficient : Fin n → ℤ)
    (mappedRelation :
      ∑ i, coefficient i •
        WeierstrassCurve.Affine.Point.map (W' := W) f (point i) = 0) :
    ∑ i, coefficient i • point i = 0 := by
  classical
  apply WeierstrassCurve.Affine.Point.map_injective (W' := W) f
  rw [map_zero, map_sum]
  simpa only [map_zsmul] using mappedRelation

/-- Two affine elliptic-curve points in the same x-coordinate fibre differ by
one integer sign. -/
theorem exists_sign_smul_eq_of_pointX_eq
    {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (left right : W.Point)
    (sameX : pointX left = pointX right) :
    ∃ sign : ℤ, (sign = 1 ∨ sign = -1) ∧ left = sign • right := by
  rcases GeneralS4Audit.same_pointX_eq_or_neg W sameX with equal | negated
  · exact ⟨1, Or.inl rfl, by simpa using equal⟩
  · exact ⟨-1, Or.inr rfl, by simpa using negated⟩

/-- A two-term relation whose partner shares the x-coordinate of a CM image
produces a signed Eisenstein norm-torsion certificate. -/
theorem pairRelation_sameX_implies_norm_torsion
    {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K)
    (zeta : W.Point →+ W.Point) (point partner : W.Point) (a b : ℤ)
    (sameX : pointX partner = pointX (zeta point))
    (orbitRelation : zeta (zeta point) + zeta point + point = 0)
    (pairRelation : a • point + b • partner = 0) :
    ∃ sign : ℤ, (sign = 1 ∨ sign = -1) ∧
      eisensteinNorm a (b * sign) • point = 0 := by
  obtain ⟨sign, signUnit, partnerRelation⟩ :=
    exists_sign_smul_eq_of_pointX_eq W partner (zeta point) sameX
  refine ⟨sign, signUnit, ?_⟩
  apply cubicCMRelation_implies_norm_torsion zeta point a (b * sign)
    orbitRelation
  rw [partnerRelation] at pairRelation
  simpa only [smul_smul] using pairRelation

/-- A zero-sum point tuple can be rewritten using arbitrary points in the same
x-coordinate fibres, at the cost of one sign `±1` per occurrence. -/
theorem signed_relation_of_same_pointX
    {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) {n : ℕ}
    (original chosen : Fin n → W.Point)
    (sameX : ∀ i, pointX (original i) = pointX (chosen i))
    (relation : ∑ i, original i = 0) :
    ∃ sign : Fin n → ℤ,
      (∀ i, sign i = 1 ∨ sign i = -1) ∧
      ∑ i, sign i • chosen i = 0 := by
  classical
  have each : ∀ i, ∃ s : ℤ,
      (s = 1 ∨ s = -1) ∧ original i = s • chosen i := by
    intro i
    rcases GeneralS4Audit.same_pointX_eq_or_neg W (sameX i) with h | h
    · exact ⟨1, Or.inl rfl, by simpa using h⟩
    · exact ⟨-1, Or.inr rfl, by simpa using h⟩
  choose sign signUnit representation using each
  refine ⟨sign, signUnit, ?_⟩
  calc
    ∑ i, sign i • chosen i = ∑ i, original i := by
      apply Finset.sum_congr rfl
      intro i hi
      exact (representation i).symm
    _ = 0 := relation

/-- Converse Semaev vanishing, normalized to any chosen lifts of the supplied
x-coordinates.  The output is the exact signed relation needed by the Galois
projector; this theorem does not yet descend the chosen points from the
algebraic closure to a finite splitting field. -/
theorem evalCanonical_zero_implies_signed_relation_on_chosen
    {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hDelta : W.Δ ≠ 0)
    (n : ℕ) (xs : Fin (n + 3) → K)
    (chosen : Fin (n + 3) → W.Point)
    (chosenX : ∀ i, pointX (chosen i) = some (xs i))
    (vanishing : evalCanonical W n xs = 0) :
    ∃ sign : Fin (n + 3) → ℤ,
      (∀ i, sign i = 1 ∨ sign i = -1) ∧
      ∑ i, sign i • chosen i = 0 := by
  obtain ⟨original, originalX, relation⟩ :=
    (canonical_zero_iff W hDelta n xs).mp vanishing
  apply signed_relation_of_same_pointX W original chosen
  · intro i
    exact (originalX i).trans (chosenX i).symm
  · exact relation

/-- Field-independent converse on already supplied point lifts.  Canonical
Semaev vanishing is mapped to an algebraic closure, sign-normalized there,
and the resulting relation descends by injectivity of point base change. -/
theorem evalCanonical_zero_implies_signed_relation
    {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (hDelta : W.Δ ≠ 0)
    (n : ℕ) (xs : Fin (n + 3) → K)
    (chosen : Fin (n + 3) → W.Point)
    (chosenX : ∀ i, pointX (chosen i) = some (xs i))
    (vanishing : evalCanonical W n xs = 0) :
    ∃ sign : Fin (n + 3) → ℤ,
      (∀ i, sign i = 1 ∨ sign i = -1) ∧
      ∑ i, sign i • chosen i = 0 := by
  classical
  let L := AlgebraicClosure K
  let f : K →ₐ[K] L := Algebra.ofId K L
  let pointMap : W.Point →+ (W.baseChange L).Point :=
    WeierstrassCurve.Affine.Point.map (W' := W) f
  have hDeltaL : (W.baseChange L).Δ ≠ 0 := by
    change (W.map (algebraMap K L)).Δ ≠ 0
    rw [WeierstrassCurve.map_Δ]
    exact (map_ne_zero (algebraMap K L)).2 hDelta
  have pointX_map (Q : W.Point) :
      pointX (pointMap Q) = (pointX Q).map f := by
    cases Q <;> rfl
  have chosenXL : ∀ i,
      pointX (pointMap (chosen i)) = some (f (xs i)) := by
    intro i
    rw [pointX_map, chosenX i]
    rfl
  have vanishingL :
      evalCanonical (W.baseChange L) n (fun i ↦ f (xs i)) = 0 := by
    have mapped := DilationSemaevBridge.map_evalCanonical f.toRingHom W n xs
    rw [vanishing, map_zero] at mapped
    simpa [WeierstrassCurve.baseChange, f, L] using mapped.symm
  obtain ⟨sign, signUnit, mappedRelation⟩ :=
    evalCanonical_zero_implies_signed_relation_on_chosen
      (W.baseChange L) hDeltaL n (fun i ↦ f (xs i))
      (fun i ↦ pointMap (chosen i)) chosenXL vanishingL
  refine ⟨sign, signUnit, ?_⟩
  exact signed_point_relation_of_mapped_relation W f chosen sign mappedRelation

/-- Canonical Semaev vanishing for scaled short-cubic x-coordinates yields
one ordinary signed relation among the corresponding canonical points in the
common quadratic splitting field. -/
theorem evalCanonical_shortCubicScaledX_zero_implies_canonical_relation
    {K : Type*} [Field K] [DecidableEq K]
    (B : K) [(shortCharacterCurve 0 0 B).IsElliptic]
    (n : ℕ) (scale : Fin (n + 3) → K)
    [DecidableEq (ShortCubicSplittingField K (Fin (n + 3)) 0 B scale)]
    (vanishing :
      evalCanonical (shortCubicFunctionCurve 0 B).toAffine n
        (fun index ↦ shortCubicScaledX (scale index)) = 0) :
    ∃ sign : Fin (n + 3) → ℤ,
      (∀ index, sign index = 1 ∨ sign index = -1) ∧
      ∑ index, sign index •
        canonicalShortCubicPoint 0 B scale index = 0 := by
  classical
  let F := ShortCubicFunctionField K
  let L := ShortCubicSplittingField K (Fin (n + 3)) 0 B scale
  let W := (shortCubicFunctionCurve 0 B).toAffine
  have hDelta : W.Δ ≠ 0 := by
    exact (shortCubicFunctionCurve 0 B).isUnit_Δ.ne_zero
  have hDeltaL : (W.baseChange L).Δ ≠ 0 := by
    change ((shortCubicFunctionCurve 0 B).map (algebraMap F L)).Δ ≠ 0
    rw [WeierstrassCurve.map_Δ]
    exact (map_ne_zero (algebraMap F L)).2 hDelta
  have vanishingL :
      evalCanonical (W.baseChange L) n
        (fun index ↦ algebraMap F L
          (shortCubicScaledX (scale index))) = 0 := by
    have mapped := DilationSemaevBridge.map_evalCanonical
      (algebraMap F L) W n
      (fun index ↦ shortCubicScaledX (scale index))
    rw [vanishing, map_zero] at mapped
    simpa [W, WeierstrassCurve.baseChange] using mapped.symm
  have chosenX : ∀ index,
      pointX (canonicalShortCubicPoint 0 B scale index) =
        some (algebraMap F L (shortCubicScaledX (scale index))) := by
    intro index
    rfl
  exact evalCanonical_zero_implies_signed_relation
    (W.baseChange L) hDeltaL n
    (fun index ↦ algebraMap F L (shortCubicScaledX (scale index)))
    (canonicalShortCubicPoint 0 B scale) chosenX vanishingL

/-- A point has a nonzero integer torsion index modulo `modulus`. -/
def HasNonzeroZModTorsionIndex
    (modulus : ℕ) {Point : Type*} [AddCommGroup Point] (point : Point) : Prop :=
  ∃ index : ℤ, (index : ZMod modulus) ≠ 0 ∧ index • point = 0

/-- A canonical short-cubic point with nonzero scale has no torsion index
that remains nonzero in the prime field.  The proof extracts a root of the
full squared division polynomial from the affine torsion relation, rewrites
the function-field curve as the base change of the constant short curve, and
contradicts transcendence of the embedded polynomial variable. -/
theorem zmodCanonicalShortCubicPoint_no_nonzero_torsion
    (modulus : ℕ) [Fact modulus.Prime]
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    {Index : Type*} [Fintype Index]
    (scale : Index → ZMod modulus)
    [DecidableEq (ShortCubicSplittingField (ZMod modulus) Index 0 B scale)]
    (chosen : Index) (scaleNonzero : scale chosen ≠ 0)
    (index : ℤ) (indexNonzero : (index : ZMod modulus) ≠ 0)
    (torsion : index • canonicalShortCubicPoint 0 B scale chosen = 0) :
    False := by
  let F := ShortCubicFunctionField (ZMod modulus)
  let L := ShortCubicSplittingField (ZMod modulus) Index 0 B scale
  let s : L := algebraMap F L
    (algebraMap (Polynomial (ZMod modulus)) F Polynomial.X)
  have sTranscendental : Transcendental (ZMod modulus) s := by
    apply (transcendental_algebraMap_iff
      (algebraMap F L).injective).2
    apply (transcendental_algebraMap_iff
      (IsFractionRing.injective (Polynomial (ZMod modulus)) F)).2
    exact Polynomial.transcendental_X (ZMod modulus)
  have curveEq :
      (shortCubicFunctionCurve 0 B).toAffine.baseChange L =
        ((shortCharacterCurve 0 0 B).map
          (algebraMap (ZMod modulus) L)).toAffine := by
    calc
      _ = (shortCharacterCurve 0
          (algebraMap (ZMod modulus) L 0)
          (algebraMap (ZMod modulus) L B)).toAffine :=
        shortCubicFunctionCurve_baseChange_eq_shortCharacterCurve 0 B
      _ = _ := by
        ext <;> simp [shortCharacterCurve]
  have curveEqCurve :
      (shortCubicFunctionCurve 0 B).map (algebraMap F L) =
        (shortCharacterCurve 0 0 B).map
          (algebraMap (ZMod modulus) L) := by
    exact curveEq
  have xEq :
      algebraMap F L (shortCubicScaledX (scale chosen)) =
        algebraMap (ZMod modulus) L (scale chosen) * s := by
    simp [shortCubicScaledX, s, F,
      IsScalarTower.algebraMap_apply (ZMod modulus) F L]
  have canonicalNonsingular := shortCubicRoot_nonsingular
    0 B (scale chosen)
    (quadraticRadicandRoot (shortCubicRadicandFamily 0 B scale) chosen)
    (by
      simpa [shortCubicRadicandFamily] using
        (quadraticRadicandRoot_sq
          (shortCubicRadicandFamily 0 B scale) chosen))
  have torsionOld : index • WeierstrassCurve.Affine.Point.some
      (algebraMap F L (shortCubicScaledX (scale chosen)))
      (quadraticRadicandRoot (shortCubicRadicandFamily 0 B scale) chosen)
      canonicalNonsingular = 0 := by
    simpa [canonicalShortCubicPoint] using torsion
  have rootOld :=
    divisionPolynomialSq_eval_eq_zero_of_affine_zsmul_eq_zero
      ((shortCubicFunctionCurve 0 B).map (algebraMap F L))
      canonicalNonsingular index torsionOld
  have rootDirect :
      (((shortCharacterCurve 0 0 B).map
        (algebraMap (ZMod modulus) L)).ΨSq index).eval
          (algebraMap (ZMod modulus) L (scale chosen) * s) = 0 := by
    rw [← curveEqCurve, ← xEq]
    exact rootOld
  apply divisionPolynomialSq_ne_zero_at_const_mul_transcendental
    (shortCharacterCurve 0 0 B) index indexNonzero s sTranscendental
      (scale chosen) scaleNonzero
  simpa [Polynomial.aeval_def, Polynomial.eval_map] using rootDirect

/-- A bounded relation among distinct interval-scale canonical points is
impossible once one coefficient survives.  This specializes the abstract
relation projector to the division-polynomial proof that every nonzero-scale
canonical point is free of nonzero prime-field torsion. -/
theorem zmodCanonicalShortCubicInterval_relation_impossible
    (modulus X arity : ℕ) [Fact modulus.Prime]
    {Index : Type*} [Fintype Index] [DecidableEq Index]
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (scale : Index → ℕ)
    [DecidableEq (ShortCubicSplittingField (ZMod modulus) Index 0 B
      (fun i ↦ (scale i : ZMod modulus)))]
    (scalePositive : ∀ index, 0 < scale index)
    (scaleBound : ∀ index, scale index ≤ X)
    (scaleInjective : Function.Injective scale)
    (intervalShort : 3 * X < modulus)
    (coefficient : Index → ℤ)
    (relation : ∑ index, coefficient index •
      canonicalShortCubicPoint 0 B
        (fun i ↦ (scale i : ZMod modulus)) index = 0)
    (chosen : Index) (chosenNonzero : coefficient chosen ≠ 0)
    (coefficientBound : ∀ index, (coefficient index).natAbs ≤ arity)
    (arityBelow :
      3 * (2 ^ Fintype.card Index * arity) ^ 2 < modulus) :
    False := by
  apply zmodCanonicalShortCubicInterval_relation_impossible_of_no_torsion
    modulus X arity hChar twoNonzeroBase B cubicNonsingular BNonzero scale
    scalePositive scaleBound scaleInjective intervalShort coefficient relation
    chosen chosenNonzero coefficientBound
  · intro pointIndex index indexNonzero torsion
    exact zmodCanonicalShortCubicPoint_no_nonzero_torsion
      modulus B (fun i ↦ (scale i : ZMod modulus)) pointIndex
      (by
        intro castZero
        have scaleBelow : scale pointIndex < modulus := by
          have := scaleBound pointIndex
          omega
        have values := congrArg ZMod.val castZero
        rw [ZMod.val_natCast_of_lt scaleBelow, ZMod.val_zero] at values
        exact (Nat.ne_of_gt (scalePositive pointIndex)) values)
      index indexNonzero torsion
  · exact arityBelow

/-- Direct ordinary-branch Semaev endpoint for pairwise nonsquare canonical
radicands. -/
theorem zmodCanonicalSemaevPairwiseNonsquare_implies_nonzero_index_torsion
    (modulus n : ℕ) [Fact modulus.Prime]
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (scale : Fin (n + 3) → ZMod modulus)
    [DecidableEq (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3))
      0 B scale)]
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (radicandNonzero : ∀ index,
      shortCubicRadicandFamily 0 B scale index ≠ 0)
    (pairwiseNonsquare : ∀ left right, left ≠ right →
      ¬ IsSquare
        (shortCubicRadicandFamily 0 B scale left *
          shortCubicRadicandFamily 0 B scale right))
    (chosen : Fin (n + 3))
    (vanishing :
      evalCanonical (shortCubicFunctionCurve 0 B).toAffine n
        (fun index ↦ shortCubicScaledX (scale index)) = 0)
    (arityBelow : 2 ^ (n + 3) * (n + 3) < modulus) :
    ∃ coefficient : Fin (n + 3) → ℤ,
      (∀ index, coefficient index = 1 ∨ coefficient index = -1) ∧
      ((((Fintype.card
        (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B scale
            ≃ₐ[ShortCubicFunctionField (ZMod modulus)]
          ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B scale) :
          ℤ) * coefficient chosen : ℤ)) : ZMod modulus) ≠ 0 ∧
      ((Fintype.card
        (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B scale
            ≃ₐ[ShortCubicFunctionField (ZMod modulus)]
          ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B scale) :
          ℤ) * coefficient chosen) •
        canonicalShortCubicPoint 0 B scale chosen = 0 := by
  obtain ⟨coefficient, coefficientUnit, relation⟩ :=
    evalCanonical_shortCubicScaledX_zero_implies_canonical_relation
      B n scale vanishing
  have indexedTorsion :=
    zmodCanonicalShortCubicPairwiseNonsquare_implies_nonzero_index_torsion
      modulus B scale twoNonzeroBase radicandNonzero pairwiseNonsquare
      coefficient coefficientUnit relation chosen (by
        simpa using arityBelow)
  exact ⟨coefficient, coefficientUnit, indexedTorsion⟩

/-- Direct prime-field `A = 0` Semaev endpoint.  Canonical Semaev vanishing
constructs the signed point relation; an interval square collision then
constructs its cubic-CM partner and an explicit Eisenstein norm-torsion
certificate. -/
theorem zmodCanonicalSemaevIntervalPairCollision_implies_norm_torsion
    (modulus X n : ℕ) [Fact modulus.Prime]
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (scale : Fin (n + 3) → ℕ)
    [DecidableEq (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3))
      0 B (fun i ↦ (scale i : ZMod modulus)))]
    (scalePositive : ∀ index, 0 < scale index)
    (scaleBound : ∀ index, scale index ≤ X)
    (scaleInjective : Function.Injective scale)
    (intervalShort : 3 * X < modulus)
    (chosen partner : Fin (n + 3)) (distinct : chosen ≠ partner)
    (pairSquare : IsSquare
      (shortCubicFunctionRadicand 0 B
          (scale chosen : ZMod modulus) *
        shortCubicFunctionRadicand 0 B
          (scale partner : ZMod modulus)))
    (vanishing :
      evalCanonical (shortCubicFunctionCurve 0 B).toAffine n
        (fun index ↦
          shortCubicScaledX (scale index : ZMod modulus)) = 0) :
    ∃ coefficient : Fin (n + 3) → ℤ,
      (∀ index, coefficient index = 1 ∨ coefficient index = -1) ∧
      ∃ omega : ZMod modulus, ∃ sign : ℤ,
        omega ^ 2 + omega + 1 = 0 ∧ omega ≠ 1 ∧
        (scale partner : ZMod modulus) =
          omega * (scale chosen : ZMod modulus) ∧
        (sign = 1 ∨ sign = -1) ∧
        eisensteinNorm
          ((Fintype.card
            (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B
                (fun i ↦ (scale i : ZMod modulus)) ≃ₐ[
              ShortCubicFunctionField (ZMod modulus)]
              ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B
                (fun i ↦ (scale i : ZMod modulus))) : ℤ) *
            coefficient chosen)
          (((Fintype.card
            (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B
                (fun i ↦ (scale i : ZMod modulus)) ≃ₐ[
              ShortCubicFunctionField (ZMod modulus)]
              ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B
                (fun i ↦ (scale i : ZMod modulus))) : ℤ) *
            coefficient partner) * sign) •
          canonicalShortCubicPoint 0 B
            (fun i ↦ (scale i : ZMod modulus)) chosen = 0 := by
  let scaleZ : Fin (n + 3) → ZMod modulus :=
    fun index ↦ scale index
  obtain ⟨coefficient, coefficientUnit, relation⟩ :=
    evalCanonical_shortCubicScaledX_zero_implies_canonical_relation
      B n scaleZ vanishing
  obtain ⟨omega, sign, omegaRelation, omegaNonidentity,
      scaleRelation, signUnit, torsion⟩ :=
    zmodCanonicalShortCubicIntervalPairCollision_implies_norm_torsion
      modulus X hChar twoNonzeroBase B cubicNonsingular BNonzero scale
      scalePositive scaleBound scaleInjective intervalShort chosen partner
      distinct pairSquare coefficient relation
  exact ⟨coefficient, coefficientUnit, omega, sign, omegaRelation,
    omegaNonidentity, scaleRelation, signUnit, torsion⟩

/-- Quantitative form of the direct `A = 0` Semaev endpoint.  Under the
explicit characteristic bound, the constructed Eisenstein index is nonzero
modulo `p` while its integer action annihilates the selected canonical point. -/
theorem zmodCanonicalSemaevIntervalPairCollision_implies_nonzero_index_torsion
    (modulus X n : ℕ) [Fact modulus.Prime]
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (scale : Fin (n + 3) → ℕ)
    [DecidableEq (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3))
      0 B (fun i ↦ (scale i : ZMod modulus)))]
    (scalePositive : ∀ index, 0 < scale index)
    (scaleBound : ∀ index, scale index ≤ X)
    (scaleInjective : Function.Injective scale)
    (intervalShort : 3 * X < modulus)
    (chosen partner : Fin (n + 3)) (distinct : chosen ≠ partner)
    (pairSquare : IsSquare
      (shortCubicFunctionRadicand 0 B
          (scale chosen : ZMod modulus) *
        shortCubicFunctionRadicand 0 B
          (scale partner : ZMod modulus)))
    (vanishing :
      evalCanonical (shortCubicFunctionCurve 0 B).toAffine n
        (fun index ↦
          shortCubicScaledX (scale index : ZMod modulus)) = 0)
    (normBelow :
      3 * (Fintype.card
        (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B
            (fun i ↦ (scale i : ZMod modulus)) ≃ₐ[
          ShortCubicFunctionField (ZMod modulus)]
          ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B
            (fun i ↦ (scale i : ZMod modulus))) * (n + 3)) ^ 2 < modulus) :
    ∃ coefficient : Fin (n + 3) → ℤ,
      (∀ index, coefficient index = 1 ∨ coefficient index = -1) ∧
      ∃ omega : ZMod modulus, ∃ sign : ℤ,
        omega ^ 2 + omega + 1 = 0 ∧ omega ≠ 1 ∧
        (scale partner : ZMod modulus) =
          omega * (scale chosen : ZMod modulus) ∧
        (sign = 1 ∨ sign = -1) ∧
        let index := eisensteinNorm
          ((Fintype.card
            (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B
                (fun i ↦ (scale i : ZMod modulus)) ≃ₐ[
              ShortCubicFunctionField (ZMod modulus)]
              ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B
                (fun i ↦ (scale i : ZMod modulus))) : ℤ) *
            coefficient chosen)
          (((Fintype.card
            (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B
                (fun i ↦ (scale i : ZMod modulus)) ≃ₐ[
              ShortCubicFunctionField (ZMod modulus)]
              ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B
                (fun i ↦ (scale i : ZMod modulus))) : ℤ) *
            coefficient partner) * sign)
        (index : ZMod modulus) ≠ 0 ∧
          index • canonicalShortCubicPoint 0 B
            (fun i ↦ (scale i : ZMod modulus)) chosen = 0 := by
  obtain ⟨coefficient, coefficientUnit, omega, sign, omegaRelation,
      omegaNonidentity, scaleRelation, signUnit, torsion⟩ :=
    zmodCanonicalSemaevIntervalPairCollision_implies_norm_torsion
      modulus X n hChar twoNonzeroBase B cubicNonsingular BNonzero scale
      scalePositive scaleBound scaleInjective intervalShort chosen partner
      distinct pairSquare vanishing
  have coefficientNonzero : coefficient chosen ≠ 0 := by
    rcases coefficientUnit chosen with equal | equal <;> simp [equal]
  have coefficientBound : ∀ index,
      (coefficient index).natAbs ≤ n + 3 := by
    intro index
    rcases coefficientUnit index with equal | equal <;> simp [equal]
  have indexedTorsion := projectedEisensteinNorm_torsion_index_ne_zero
    (K := ZMod modulus)
    (Gamma := ShortCubicSplittingField (ZMod modulus) (Fin (n + 3))
      0 B (fun i ↦ (scale i : ZMod modulus)) ≃ₐ[
        ShortCubicFunctionField (ZMod modulus)]
      ShortCubicSplittingField (ZMod modulus) (Fin (n + 3))
        0 B (fun i ↦ (scale i : ZMod modulus)))
    modulus (n + 3) (coefficient chosen) (coefficient partner) sign
    (canonicalShortCubicPoint 0 B
      (fun i ↦ (scale i : ZMod modulus)) chosen)
    coefficientNonzero (coefficientBound chosen) (coefficientBound partner)
    signUnit normBelow torsion
  exact ⟨coefficient, coefficientUnit, omega, sign, omegaRelation,
    omegaNonidentity, scaleRelation, signUnit, indexedTorsion⟩

/-- Uniform prime-field `A = 0` nonzero-torsion endpoint.  If every distinct
radicand pair has nonsquare product, the ordinary Kummer projector applies.
Otherwise a square pair exists and the interval cubic-CM projector applies.
The stronger CM arity bound covers both branches. -/
theorem zmodCanonicalSemaevInterval_implies_exists_nonzero_torsion
    (modulus X n : ℕ) [Fact modulus.Prime]
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (scale : Fin (n + 3) → ℕ)
    [DecidableEq (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3))
      0 B (fun i ↦ (scale i : ZMod modulus)))]
    (scalePositive : ∀ index, 0 < scale index)
    (scaleBound : ∀ index, scale index ≤ X)
    (scaleInjective : Function.Injective scale)
    (intervalShort : 3 * X < modulus)
    (vanishing :
      evalCanonical (shortCubicFunctionCurve 0 B).toAffine n
        (fun index ↦
          shortCubicScaledX (scale index : ZMod modulus)) = 0)
    (arityBelow :
      3 * (2 ^ (n + 3) * (n + 3)) ^ 2 < modulus) :
    ∃ chosen : Fin (n + 3),
      HasNonzeroZModTorsionIndex modulus
        (canonicalShortCubicPoint 0 B
          (fun i ↦ (scale i : ZMod modulus)) chosen) := by
  classical
  let scaleZ : Fin (n + 3) → ZMod modulus :=
    fun index ↦ scale index
  have scaleBelow : ∀ index, scale index < modulus := by
    intro index
    have := scaleBound index
    omega
  have scaleNonzero : ∀ index, scaleZ index ≠ 0 := by
    intro index castZero
    have values := congrArg ZMod.val castZero
    rw [ZMod.val_natCast_of_lt (scaleBelow index), ZMod.val_zero] at values
    exact (Nat.ne_of_gt (scalePositive index)) values
  have radicandNonzero : ∀ index,
      shortCubicRadicandFamily 0 B scaleZ index ≠ 0 := by
    intro index
    exact shortCubicFunctionRadicand_ne_zero 0 B (scaleZ index)
      (scaleNonzero index)
  by_cases pairwiseNonsquare : ∀ left right, left ≠ right →
      ¬ IsSquare
        (shortCubicRadicandFamily 0 B scaleZ left *
          shortCubicRadicandFamily 0 B scaleZ right)
  · let chosen : Fin (n + 3) := ⟨0, by omega⟩
    have sizePositive : 0 < 2 ^ (n + 3) * (n + 3) := by positivity
    have ordinaryBelow : 2 ^ (n + 3) * (n + 3) < modulus := by
      calc
        2 ^ (n + 3) * (n + 3) ≤
            3 * (2 ^ (n + 3) * (n + 3)) ^ 2 := by
              nlinarith
        _ < modulus := arityBelow
    obtain ⟨coefficient, -, indexNonzero, torsion⟩ :=
      zmodCanonicalSemaevPairwiseNonsquare_implies_nonzero_index_torsion
        modulus n B scaleZ twoNonzeroBase radicandNonzero pairwiseNonsquare
        chosen vanishing ordinaryBelow
    refine ⟨chosen,
      (Fintype.card
        (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B scaleZ
            ≃ₐ[ShortCubicFunctionField (ZMod modulus)]
          ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B scaleZ) :
          ℤ) * coefficient chosen, indexNonzero, torsion⟩
  · push Not at pairwiseNonsquare
    obtain ⟨chosen, partner, distinct, pairSquare⟩ := pairwiseNonsquare
    have exactNormBelow :
        3 * (Fintype.card
          (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B scaleZ
              ≃ₐ[ShortCubicFunctionField (ZMod modulus)]
            ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B scaleZ) *
          (n + 3)) ^ 2 < modulus := by
      have bound := quadraticRadicand_norm_threshold_of_arity
        (shortCubicRadicandFamily 0 B scaleZ) modulus (by
          simpa using arityBelow)
      simpa only [Fintype.card_fin] using bound
    obtain ⟨coefficient, -, omega, sign, -, -, -, -, indexNonzero, torsion⟩ :=
      zmodCanonicalSemaevIntervalPairCollision_implies_nonzero_index_torsion
        modulus X n hChar twoNonzeroBase B cubicNonsingular BNonzero scale
        scalePositive scaleBound scaleInjective intervalShort chosen partner
        distinct (by simpa [shortCubicRadicandFamily, scaleZ] using pairSquare)
        vanishing exactNormBelow
    refine ⟨chosen, eisensteinNorm
      ((Fintype.card
        (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B scaleZ
            ≃ₐ[ShortCubicFunctionField (ZMod modulus)]
          ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B scaleZ) :
          ℤ) * coefficient chosen)
      (((Fintype.card
        (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B scaleZ
            ≃ₐ[ShortCubicFunctionField (ZMod modulus)]
          ShortCubicSplittingField (ZMod modulus) (Fin (n + 3)) 0 B scaleZ) :
          ℤ) * coefficient partner) * sign), indexNonzero, torsion⟩

/-- Uniform prime-field `A = 0` nonvanishing theorem for the canonical
scaled interval.  Under the explicit characteristic and arity bounds, a
Semaev vanishing would produce a scalar-torsion relation whose index is
nonzero modulo `modulus`; the transcendental division-polynomial endpoint
rules out that relation. -/
theorem zmodCanonicalSemaevInterval_ne_zero
    (modulus X n : ℕ) [Fact modulus.Prime]
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (scale : Fin (n + 3) → ℕ)
    [DecidableEq (ShortCubicSplittingField (ZMod modulus) (Fin (n + 3))
      0 B (fun i ↦ (scale i : ZMod modulus)))]
    (scalePositive : ∀ index, 0 < scale index)
    (scaleBound : ∀ index, scale index ≤ X)
    (scaleInjective : Function.Injective scale)
    (intervalShort : 3 * X < modulus)
    (arityBelow :
      3 * (2 ^ (n + 3) * (n + 3)) ^ 2 < modulus) :
    evalCanonical (shortCubicFunctionCurve 0 B).toAffine n
      (fun index ↦ shortCubicScaledX (scale index : ZMod modulus)) ≠ 0 := by
  intro vanishing
  let scaleZ : Fin (n + 3) → ZMod modulus :=
    fun index ↦ scale index
  obtain ⟨chosen, index, indexNonzero, torsion⟩ :=
    zmodCanonicalSemaevInterval_implies_exists_nonzero_torsion
      modulus X n hChar twoNonzeroBase B cubicNonsingular BNonzero scale
      scalePositive scaleBound scaleInjective intervalShort vanishing arityBelow
  have scaleBelow : ∀ index, scale index < modulus := by
    intro index
    have := scaleBound index
    omega
  have chosenScaleNonzero : scaleZ chosen ≠ 0 := by
    intro castZero
    have values := congrArg ZMod.val castZero
    rw [ZMod.val_natCast_of_lt (scaleBelow chosen), ZMod.val_zero] at values
    exact (Nat.ne_of_gt (scalePositive chosen)) values
  exact zmodCanonicalShortCubicPoint_no_nonzero_torsion
    modulus B scaleZ chosen chosenScaleNonzero index indexNonzero torsion

/-- Full finite-image relation certificate obtained from canonical Semaev
vanishing.  For a nontrivial x-tuple and one chosen point in every used
x-fibre, the signed relation regroups over the finite set of distinct values,
and at least one grouped coefficient is nonzero with absolute value at most
the tuple arity. -/
theorem evalCanonical_zero_implies_nontriv_image_certificate
    {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (hDelta : W.Δ ≠ 0)
    (n : ℕ) (xs : Fin (n + 3) → K)
    (chosen : K → W.Point)
    (chosenX : ∀ i, pointX (chosen (xs i)) = some (xs i))
    (vanishing : evalCanonical W n xs = 0)
    (nontriv : ¬ ECDLP.L40.TrivL (List.ofFn xs)) :
    ∃ sign : Fin (n + 3) → ℤ,
      (∀ i, sign i = 1 ∨ sign i = -1) ∧
      (∑ v : {v // v ∈ Finset.univ.image xs},
          fiberCoefficient xs sign v.1 • chosen v.1 = 0) ∧
      ∃ v, fiberCoefficient xs sign v ≠ 0 ∧
        (fiberCoefficient xs sign v).natAbs ≤ n + 3 := by
  obtain ⟨sign, signUnit, relation⟩ :=
    evalCanonical_zero_implies_signed_relation
      W hDelta n xs (fun i ↦ chosen (xs i)) chosenX vanishing
  obtain ⟨regrouped, surviving⟩ :=
    nontriv_signed_relation_image_certificate
      xs sign signUnit chosen relation nontriv
  exact ⟨sign, signUnit, regrouped, surviving⟩

/-- Projector-shaped finite-image certificate.  The chosen point family and
the surviving coefficient are both indexed by the subtype of distinct tuple
values, matching the finite index used to build the quadratic splitting
field. -/
theorem evalCanonical_zero_implies_nontriv_image_subtype_certificate
    {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (hDelta : W.Δ ≠ 0)
    (n : ℕ) (xs : Fin (n + 3) → K)
    (chosen : {v // v ∈ Finset.univ.image xs} → W.Point)
    (chosenX : ∀ v, pointX (chosen v) = some v.1)
    (vanishing : evalCanonical W n xs = 0)
    (nontriv : ¬ ECDLP.L40.TrivL (List.ofFn xs)) :
    ∃ sign : Fin (n + 3) → ℤ,
      (∀ i, sign i = 1 ∨ sign i = -1) ∧
      (∑ v ∈ (Finset.univ.image xs).attach,
          fiberCoefficient xs sign v.1 • chosen v = 0) ∧
      ∃ v : {v // v ∈ Finset.univ.image xs},
        fiberCoefficient xs sign v.1 ≠ 0 ∧
        (fiberCoefficient xs sign v.1).natAbs ≤ n + 3 := by
  let occurrence : Fin (n + 3) → {v // v ∈ Finset.univ.image xs} :=
    fun i ↦ ⟨xs i, Finset.mem_image.mpr ⟨i, Finset.mem_univ i, rfl⟩⟩
  obtain ⟨sign, signUnit, relation⟩ :=
    evalCanonical_zero_implies_signed_relation
      W hDelta n xs (fun i ↦ chosen (occurrence i))
      (fun i ↦ chosenX (occurrence i)) vanishing
  obtain ⟨regrouped, surviving⟩ :=
    nontriv_signed_relation_image_subtype_certificate
      xs sign signUnit chosen relation nontriv
  exact ⟨sign, signUnit, regrouped, surviving⟩

/-- Canonical splitting-field lift of the Semaev relation.  One square root is
chosen for each distinct tuple value by `quadraticRadicandRoot`; the same
finite-image subtype indexes the resulting relation and its surviving
coefficient. -/
theorem evalCanonical_zero_implies_canonicalSplittingField_certificate
    {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (hDelta : W.Δ ≠ 0)
    (n : ℕ) (xs : Fin (n + 3) → K)
    (radicand : {v // v ∈ Finset.univ.image xs} → K)
    [DecidableEq (quadraticRadicandPolynomial radicand).SplittingField]
    (nonsingular : ∀ v,
      (W.baseChange
        (quadraticRadicandPolynomial radicand).SplittingField).Nonsingular
          (algebraMap K _ v.1) (quadraticRadicandRoot radicand v))
    (vanishing : evalCanonical W n xs = 0)
    (nontriv : ¬ ECDLP.L40.TrivL (List.ofFn xs)) :
    ∃ sign : Fin (n + 3) → ℤ,
      (∀ i, sign i = 1 ∨ sign i = -1) ∧
      (∑ v ∈ (Finset.univ.image xs).attach,
        fiberCoefficient xs sign v.1 •
          WeierstrassCurve.Affine.Point.some
            (algebraMap K
              (quadraticRadicandPolynomial radicand).SplittingField v.1)
            (quadraticRadicandRoot radicand v) (nonsingular v) = 0) ∧
      ∃ v : {v // v ∈ Finset.univ.image xs},
        fiberCoefficient xs sign v.1 ≠ 0 ∧
        (fiberCoefficient xs sign v.1).natAbs ≤ n + 3 := by
  classical
  let L := (quadraticRadicandPolynomial radicand).SplittingField
  let point : {v // v ∈ Finset.univ.image xs} →
      (W.baseChange L).Point := fun v ↦
    WeierstrassCurve.Affine.Point.some
      (algebraMap K L v.1) (quadraticRadicandRoot radicand v) (nonsingular v)
  let occurrence : Fin (n + 3) → {v // v ∈ Finset.univ.image xs} :=
    fun i ↦ ⟨xs i, Finset.mem_image.mpr ⟨i, Finset.mem_univ i, rfl⟩⟩
  have hDeltaL : (W.baseChange L).Δ ≠ 0 := by
    change (W.map (algebraMap K L)).Δ ≠ 0
    rw [WeierstrassCurve.map_Δ]
    exact (map_ne_zero (algebraMap K L)).2 hDelta
  have vanishingL :
      evalCanonical (W.baseChange L) n
        (fun i ↦ algebraMap K L (xs i)) = 0 := by
    have mapped := DilationSemaevBridge.map_evalCanonical
      (algebraMap K L) W n xs
    rw [vanishing, map_zero] at mapped
    simpa [WeierstrassCurve.baseChange] using mapped.symm
  have pointXOccurrence : ∀ i,
      pointX (point (occurrence i)) =
        some (algebraMap K L (xs i)) := by
    intro i
    rfl
  obtain ⟨sign, signUnit, relation⟩ :=
    evalCanonical_zero_implies_signed_relation
      (W.baseChange L) hDeltaL n (fun i ↦ algebraMap K L (xs i))
      (fun i ↦ point (occurrence i)) pointXOccurrence vanishingL
  obtain ⟨regrouped, surviving⟩ :=
    nontriv_signed_relation_image_subtype_certificate
      xs sign signUnit point relation nontriv
  exact ⟨sign, signUnit, regrouped, surviving⟩

/-- Canonical Semaev relation composed with the concrete quadratic
splitting-field projector.  A nontrivial vanishing tuple yields a bounded,
nonzero integer coefficient whose Galois-projected multiple annihilates the
corresponding canonical point. -/
theorem evalCanonical_zero_implies_projected_torsion
    {K : Type*} [Field K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) [W.IsElliptic]
    (a₁Zero : W.a₁ = 0) (a₃Zero : W.a₃ = 0)
    (hDelta : W.Δ ≠ 0) (twoNonzero : (2 : K) ≠ 0)
    (n : ℕ) (xs : Fin (n + 3) → K)
    (radicand : {v // v ∈ Finset.univ.image xs} → K)
    [DecidableEq (quadraticRadicandPolynomial radicand).SplittingField]
    (radicandNonzero : ∀ v, radicand v ≠ 0)
    (pairwiseNonsquare : ∀ left right, left ≠ right →
      ¬ IsSquare (radicand left * radicand right))
    (nonsingular : ∀ v,
      (W.baseChange
        (quadraticRadicandPolynomial radicand).SplittingField).Nonsingular
          (algebraMap K _ v.1) (quadraticRadicandRoot radicand v))
    (vanishing : evalCanonical W n xs = 0)
    (nontriv : ¬ ECDLP.L40.TrivL (List.ofFn xs)) :
    ∃ sign : Fin (n + 3) → ℤ,
      (∀ i, sign i = 1 ∨ sign i = -1) ∧
      ∃ chosen : {v // v ∈ Finset.univ.image xs},
        fiberCoefficient xs sign chosen.1 ≠ 0 ∧
        (fiberCoefficient xs sign chosen.1).natAbs ≤ n + 3 ∧
        ((Fintype.card
          ((quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
            (quadraticRadicandPolynomial radicand).SplittingField) : ℤ) *
          fiberCoefficient xs sign chosen.1) •
            WeierstrassCurve.Affine.Point.some
              (algebraMap K
                (quadraticRadicandPolynomial radicand).SplittingField
                  chosen.1)
              (quadraticRadicandRoot radicand chosen)
              (nonsingular chosen) = 0 := by
  classical
  obtain ⟨sign, signUnit, relation, chosen, coefficientNonzero,
      coefficientBound⟩ :=
    evalCanonical_zero_implies_canonicalSplittingField_certificate
      W hDelta n xs radicand nonsingular vanishing nontriv
  letI : Fintype {v // v ∈ Finset.univ.image xs} :=
    (Finset.univ.image xs).fintypeCoeSort
  have relationFintype :
      ∑ v : {v // v ∈ Finset.univ.image xs},
        fiberCoefficient xs sign v.1 •
          WeierstrassCurve.Affine.Point.some
            (algebraMap K
              (quadraticRadicandPolynomial radicand).SplittingField v.1)
            (quadraticRadicandRoot radicand v) (nonsingular v) = 0 := by
    rw [Finset.univ_eq_attach]
    exact relation
  refine ⟨sign, signUnit, chosen, coefficientNonzero, coefficientBound, ?_⟩
  exact quadraticSplittingFieldAffinePointProjector_isolates_term
    W a₁Zero a₃Zero radicand (fun v ↦ v.1)
    (fun v ↦ fiberCoefficient xs sign v.1) chosen twoNonzero
    radicandNonzero pairwiseNonsquare nonsingular relationFintype

/-- The projected Semaev torsion relation with the scalar simultaneously
proved nonzero in the characteristic-`p` coefficient field. -/
theorem evalCanonical_zero_implies_projected_nonzero_index_torsion
    {K : Type*} (modulus n : ℕ)
    [Field K] [CharP K modulus] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) [W.IsElliptic]
    (a₁Zero : W.a₁ = 0) (a₃Zero : W.a₃ = 0)
    (hDelta : W.Δ ≠ 0) (twoNonzero : (2 : K) ≠ 0)
    (xs : Fin (n + 3) → K)
    (radicand : {v // v ∈ Finset.univ.image xs} → K)
    [DecidableEq (quadraticRadicandPolynomial radicand).SplittingField]
    (radicandNonzero : ∀ v, radicand v ≠ 0)
    (pairwiseNonsquare : ∀ left right, left ≠ right →
      ¬ IsSquare (radicand left * radicand right))
    (nonsingular : ∀ v,
      (W.baseChange
        (quadraticRadicandPolynomial radicand).SplittingField).Nonsingular
          (algebraMap K _ v.1) (quadraticRadicandRoot radicand v))
    (projectedBelow :
      Fintype.card
        ((quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
          (quadraticRadicandPolynomial radicand).SplittingField) *
          (n + 3) < modulus)
    (vanishing : evalCanonical W n xs = 0)
    (nontriv : ¬ ECDLP.L40.TrivL (List.ofFn xs)) :
    ∃ sign : Fin (n + 3) → ℤ,
      (∀ i, sign i = 1 ∨ sign i = -1) ∧
      ∃ chosen : {v // v ∈ Finset.univ.image xs},
        fiberCoefficient xs sign chosen.1 ≠ 0 ∧
        (fiberCoefficient xs sign chosen.1).natAbs ≤ n + 3 ∧
        let index : ℤ :=
          (Fintype.card
            ((quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
              (quadraticRadicandPolynomial radicand).SplittingField) : ℤ) *
            fiberCoefficient xs sign chosen.1
        let point := WeierstrassCurve.Affine.Point.some
          (algebraMap K
            (quadraticRadicandPolynomial radicand).SplittingField chosen.1)
          (quadraticRadicandRoot radicand chosen) (nonsingular chosen)
        (index : K) ≠ 0 ∧ index • point = 0 := by
  classical
  obtain ⟨sign, signUnit, chosen, coefficientNonzero, coefficientBound,
      torsion⟩ := evalCanonical_zero_implies_projected_torsion
    W a₁Zero a₃Zero hDelta twoNonzero n xs radicand radicandNonzero
      pairwiseNonsquare nonsingular vanishing nontriv
  refine ⟨sign, signUnit, chosen, coefficientNonzero, coefficientBound, ?_⟩
  exact projectedTorsion_index_ne_zero
    (K := K)
    (Gamma :=
      (quadraticRadicandPolynomial radicand).SplittingField ≃ₐ[K]
        (quadraticRadicandPolynomial radicand).SplittingField)
    modulus (n + 3) (fiberCoefficient xs sign chosen.1)
    (WeierstrassCurve.Affine.Point.some
      (algebraMap K
        (quadraticRadicandPolynomial radicand).SplittingField chosen.1)
      (quadraticRadicandRoot radicand chosen) (nonsingular chosen))
    coefficientNonzero coefficientBound projectedBelow torsion

set_option maxHeartbeats 800000
/-- Repeated tuple values do not create a canonical `A = 0` Semaev zero on
the short positive interval.  The proof indexes the splitting field by the
finite image of the tuple, regroups the signed point relation by equal
values, and applies the separated-or-cubic-CM relation obstruction to a
surviving fibre coefficient. -/
theorem zmodCanonicalSemaevInterval_nontriv_ne_zero
    (modulus X n : ℕ) [Fact modulus.Prime]
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (t : Fin (n + 3) → ZMod modulus)
    (scalePositive : ∀ index, 0 < ZMod.val (t index))
    (scaleBound : ∀ index, ZMod.val (t index) ≤ X)
    (intervalShort : 3 * X < modulus)
    (nontriv : ¬ ECDLP.L40.TrivL (List.ofFn t))
    (arityBelow : 3 * (2 ^ (n + 3) * (n + 3)) ^ 2 < modulus) :
    evalCanonical (shortCubicFunctionCurve 0 B).toAffine n
      (fun index ↦ shortCubicScaledX (t index)) ≠ 0 := by
  classical
  intro vanishing
  let Image := {v // v ∈ Finset.univ.image t}
  letI : Fintype Image := (Finset.univ.image t).fintypeCoeSort
  let scale : Image → ℕ := fun value ↦ ZMod.val value.1
  let scaleZ : Image → ZMod modulus := fun value ↦ scale value
  let occurrence : Fin (n + 3) → Image := fun index ↦
    ⟨t index, Finset.mem_image.mpr ⟨index, Finset.mem_univ index, rfl⟩⟩
  let F := ShortCubicFunctionField (ZMod modulus)
  let L := ShortCubicSplittingField (ZMod modulus) Image 0 B scaleZ
  let W := (shortCubicFunctionCurve 0 B).toAffine
  have hDelta : W.Δ ≠ 0 := by
    exact (shortCubicFunctionCurve 0 B).isUnit_Δ.ne_zero
  have hDeltaL : (W.baseChange L).Δ ≠ 0 := by
    change ((shortCubicFunctionCurve 0 B).map (algebraMap F L)).Δ ≠ 0
    rw [WeierstrassCurve.map_Δ]
    exact (map_ne_zero (algebraMap F L)).2 hDelta
  have vanishingL :
      evalCanonical (W.baseChange L) n
        (fun index ↦ algebraMap F L (shortCubicScaledX (t index))) = 0 := by
    have mapped := DilationSemaevBridge.map_evalCanonical
      (algebraMap F L) W n (fun index ↦ shortCubicScaledX (t index))
    rw [vanishing, map_zero] at mapped
    simpa [W, WeierstrassCurve.baseChange] using mapped.symm
  have pointXOccurrence : ∀ index,
      pointX (canonicalShortCubicPoint 0 B scaleZ (occurrence index)) =
        some (algebraMap F L (shortCubicScaledX (t index))) := by
    intro index
    have scaleAt : scaleZ (occurrence index) = t index := by
      change ((ZMod.val (t index) : ℕ) : ZMod modulus) = t index
      exact ZMod.natCast_zmod_val (t index)
    rw [show pointX (canonicalShortCubicPoint 0 B scaleZ (occurrence index)) =
        some (algebraMap F L
          (shortCubicScaledX (scaleZ (occurrence index)))) by rfl]
    rw [scaleAt]
  obtain ⟨sign, signUnit, signedRelation⟩ :=
    evalCanonical_zero_implies_signed_relation
      (W.baseChange L) hDeltaL n
      (fun index ↦ algebraMap F L (shortCubicScaledX (t index)))
      (fun index ↦ canonicalShortCubicPoint 0 B scaleZ (occurrence index))
      pointXOccurrence vanishingL
  obtain ⟨relationAttach, chosen, chosenNonzero, chosenBound⟩ :=
    nontriv_signed_relation_image_subtype_certificate
      t sign signUnit (canonicalShortCubicPoint 0 B scaleZ)
      signedRelation nontriv
  have relation : ∑ value : Image,
      fiberCoefficient t sign value.1 •
        canonicalShortCubicPoint 0 B scaleZ value = 0 := by
    rw [Finset.univ_eq_attach]
    exact relationAttach
  have scalePositiveImage : ∀ value, 0 < scale value := by
    intro value
    obtain ⟨index, -, valueEq⟩ := Finset.mem_image.mp value.2
    change 0 < ZMod.val value.1
    rw [← valueEq]
    exact scalePositive index
  have scaleBoundImage : ∀ value, scale value ≤ X := by
    intro value
    obtain ⟨index, -, valueEq⟩ := Finset.mem_image.mp value.2
    change ZMod.val value.1 ≤ X
    rw [← valueEq]
    exact scaleBound index
  have scaleInjective : Function.Injective scale := by
    intro left right values
    apply Subtype.ext
    exact ZMod.val_injective modulus values
  have coefficientBound : ∀ value : Image,
      (fiberCoefficient t sign value.1).natAbs ≤ n + 3 := by
    intro value
    exact (fiberCoefficient_natAbs_le_card t sign signUnit value.1).trans
      (by simpa using
        (Finset.card_le_univ
          (Finset.univ.filter fun index ↦ t index = value.1)))
  have cardImage : Fintype.card Image ≤ n + 3 := by
    rw [Fintype.card_coe]
    exact (Finset.card_image_le.trans (by simp))
  have productBound :
      2 ^ Fintype.card Image * (n + 3) ≤
        2 ^ (n + 3) * (n + 3) := by
    exact Nat.mul_le_mul_right (n + 3)
      (Nat.pow_le_pow_right (by omega) cardImage)
  have arityBelowImage :
      3 * (2 ^ Fintype.card Image * (n + 3)) ^ 2 < modulus := by
    exact (Nat.mul_le_mul_left 3
      (Nat.pow_le_pow_left productBound 2)).trans_lt arityBelow
  exact zmodCanonicalShortCubicInterval_relation_impossible
    modulus X (n + 3) hChar twoNonzeroBase B cubicNonsingular BNonzero
    scale scalePositiveImage scaleBoundImage scaleInjective intervalShort
    (fun value ↦ fiberCoefficient t sign value.1) relation chosen
    chosenNonzero coefficientBound arityBelowImage

/-- Function-field nonvanishing detects nonzeroness of the actual
simultaneous-scale polynomial over the prime field.  If the polynomial were
zero, evaluation after coefficient extension at the transcendental variable
would give the forbidden canonical function-field Semaev zero. -/
theorem zmodCanonicalSimultaneousScalePolynomial_ne_zero
    (modulus X n : ℕ) [Fact modulus.Prime]
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (t : Fin (n + 3) → ZMod modulus)
    (scalePositive : ∀ index, 0 < ZMod.val (t index))
    (scaleBound : ∀ index, ZMod.val (t index) ≤ X)
    (intervalShort : 3 * X < modulus)
    (nontriv : ¬ ECDLP.L40.TrivL (List.ofFn t))
    (arityBelow : 3 * (2 ^ (n + 3) * (n + 3)) ^ 2 < modulus) :
    DilationSemaevBridge.simultaneousScalePolynomial
      (ECDLP.L43.shortW 0 B) n t ≠ 0 := by
  intro polynomialZero
  let transcendentalX : ShortCubicFunctionField (ZMod modulus) :=
    algebraMap (Polynomial (ZMod modulus))
      (ShortCubicFunctionField (ZMod modulus)) Polynomial.X
  let evaluate : Polynomial (ZMod modulus) →+*
      ShortCubicFunctionField (ZMod modulus) :=
    Polynomial.eval₂RingHom
      (algebraMap (ZMod modulus)
        (ShortCubicFunctionField (ZMod modulus))) transcendentalX
  have zeroAfterExtension :
      evaluate (DilationSemaevBridge.simultaneousScalePolynomial
        (ECDLP.L43.shortW 0 B) n t) = 0 := by
    rw [polynomialZero, map_zero]
  change evaluate
    (DilationSemaevBridge.evalCanonicalTo Polynomial.C
      (ECDLP.L43.shortW 0 B) n
      (fun index ↦ Polynomial.C (t index) * Polynomial.X)) = 0 at zeroAfterExtension
  rw [DilationSemaevBridge.evalCanonicalTo_natural] at zeroAfterExtension
  have compCoefficients :
      evaluate.comp (Polynomial.C : ZMod modulus →+* Polynomial (ZMod modulus)) =
        algebraMap (ZMod modulus)
          (ShortCubicFunctionField (ZMod modulus)) := by
    exact Polynomial.eval₂RingHom_comp_C
      (algebraMap (ZMod modulus)
        (ShortCubicFunctionField (ZMod modulus))) transcendentalX
  rw [compCoefficients] at zeroAfterExtension
  have inputs : (fun index ↦
      evaluate (Polynomial.C (t index) * Polynomial.X)) =
      (fun index ↦ shortCubicScaledX (t index)) := by
    funext index
    simp [evaluate, transcendentalX, shortCubicScaledX]
  rw [inputs] at zeroAfterExtension
  have mappedVanishing :
      evalCanonical
        ((ECDLP.L43.shortW 0 B).map
          (algebraMap (ZMod modulus)
            (ShortCubicFunctionField (ZMod modulus)))) n
        (fun index ↦ shortCubicScaledX (t index)) = 0 := by
    rw [← DilationSemaevBridge.evalCanonical_baseChange]
    exact zeroAfterExtension
  have curveEq :
      (ECDLP.L43.shortW 0 B).map
          (algebraMap (ZMod modulus)
            (ShortCubicFunctionField (ZMod modulus))) =
        (shortCubicFunctionCurve 0 B).toAffine := by
    ext <;> simp [ECDLP.L43.shortW, shortCubicFunctionCurve,
      shortCharacterCurve]
  rw [curveEq] at mappedVanishing
  exact (zmodCanonicalSemaevInterval_nontriv_ne_zero
    modulus X n hChar twoNonzeroBase B cubicNonsingular BNonzero t
    scalePositive scaleBound intervalShort nontriv arityBelow)
      mappedVanishing

/-- The production even-arity canonical dilation polynomial is nonzero on
every nontrivial tuple in the bounded positive interval. -/
theorem zmodCanonicalDilationPolynomial_ne_zero
    (modulus X m : ℕ) [Fact modulus.Prime]
    (hm : 2 ≤ m)
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (t : Fin (2 * m) → ZMod modulus)
    (scalePositive : ∀ index, 0 < ZMod.val (t index))
    (scaleBound : ∀ index, ZMod.val (t index) ≤ X)
    (intervalShort : 3 * X < modulus)
    (nontriv : ¬ ECDLP.L40.TrivL (List.ofFn t))
    (arityBelow : 3 * (2 ^ (2 * m) * (2 * m)) ^ 2 < modulus) :
    canonicalDilationPolynomial 0 B m t ≠ 0 := by
  rw [canonicalDilationPolynomial_of_two_le 0 B m hm t]
  let reindex : Fin (evenArityStage m + 3) → Fin (2 * m) :=
    Fin.cast (evenArityStage_arity m hm)
  have reindexedNontriv :
      ¬ ECDLP.L40.TrivL (List.ofFn (fun index ↦ t (reindex index))) := by
    intro reindexedTrivial
    apply nontriv
    rw [List.ofFn_congr (evenArityStage_arity m hm).symm t]
    exact reindexedTrivial
  apply zmodCanonicalSimultaneousScalePolynomial_ne_zero
    modulus X (evenArityStage m) hChar twoNonzeroBase B cubicNonsingular
    BNonzero (fun index ↦ t (reindex index))
  · intro index
    exact scalePositive (reindex index)
  · intro index
    exact scaleBound (reindex index)
  · exact intervalShort
  · exact reindexedNontriv
  · simpa only [evenArityStage_arity m hm] using arityBelow

/-- Exact production `DilationNondeg` certificate for the canonical `A = 0`
family on a bounded positive interval, with the explicit structural degree
bound used by the upstream average-energy theorem. -/
theorem zmodCanonicalDilationNondeg
    (modulus X m : ℕ) [Fact modulus.Prime]
    (hm : 2 ≤ m)
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (hDelta : (ECDLP.L43.shortW (0 : ZMod modulus) B).Δ ≠ 0)
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (T : Finset (ZMod modulus))
    (factorBasePositive : ∀ value ∈ T, 0 < ZMod.val value)
    (factorBaseBound : ∀ value ∈ T, ZMod.val value ≤ X)
    (intervalShort : 3 * X < modulus)
    (arityBelow : 3 * (2 ^ (2 * m) * (2 * m)) ^ 2 < modulus) :
    ECDLP.L49.DilationNondeg 0 B m T
      ((2 * m) * 2 ^ (2 * m - 2)) := by
  apply canonicalDilationPolynomial_nonzero_implies_nondeg_explicit
    0 B hDelta m hm T
  intro t tupleInFactorBase nontriv
  apply zmodCanonicalDilationPolynomial_ne_zero
    modulus X m hm hChar twoNonzeroBase B cubicNonsingular BNonzero t
  · intro index
    exact factorBasePositive (t index) (tupleInFactorBase index)
  · intro index
    exact factorBaseBound (t index) (tupleInFactorBase index)
  · exact intervalShort
  · exact nontriv
  · exact arityBelow

/-- The closed canonical nondegeneracy certificate feeds the exact upstream
average-energy estimate.  This is the quantitative analytic consequence
needed before a separate relation-yield and restart analysis. -/
theorem zmodCanonical_avg_energy_dil
    (modulus X n : ℕ) [Fact modulus.Prime]
    (hn : 1 ≤ n)
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (hDelta : (ECDLP.L43.shortW (0 : ZMod modulus) B).Δ ≠ 0)
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (T : Finset (ZMod modulus))
    (factorBasePositive : ∀ value ∈ T, 0 < ZMod.val value)
    (factorBaseBound : ∀ value ∈ T, ZMod.val value ≤ X)
    (intervalShort : 3 * X < modulus)
    (arityBelow :
      3 * (2 ^ (2 * (n + 1)) * (2 * (n + 1))) ^ 2 < modulus) :
    ∑ u : (ZMod modulus)ˣ,
        (ECDLP.L43.Eng T n
          ((u : ZMod modulus) ^ 4 * 0)
          ((u : ZMod modulus) ^ 6 * B) : ℝ) ≤
      ECDLP.L49.Mdil (ZMod modulus) n T
          ((2 * (n + 1)) * 2 ^ (2 * (n + 1) - 2)) *
        (Fintype.card (ZMod modulus)ˣ : ℝ) := by
  have hm : 2 ≤ n + 1 := by omega
  have nonsingular : ECDLP.L43.disc (0 : ZMod modulus) B ≠ 0 := by
    simpa [ECDLP.L43.disc] using cubicNonsingular
  exact ECDLP.L49.avg_energy_dil hChar 0 B nonsingular T n
    ((2 * (n + 1)) * 2 ^ (2 * (n + 1) - 2))
    (zmodCanonicalDilationNondeg
      modulus X (n + 1) hm hChar twoNonzeroBase B hDelta
      cubicNonsingular BNonzero T factorBasePositive factorBaseBound
      intervalShort arityBelow)

/-- At least one twelfth of all nonzero dilations simultaneously have a
factor base of size at least `|T|/2` and energy below twelve times any natural
upper bound for `Mdil`.  This composes the checked character-sum first moment
with the canonical average-energy certificate, without assuming the stronger
upstream variance hypothesis `FactorBaseVar`. -/
theorem zmodCanonical_oneTwelfth_good_dilations
    (modulus X n meanBound : ℕ) [Fact modulus.Prime]
    (hn : 1 ≤ n)
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (hDelta : (ECDLP.L43.shortW (0 : ZMod modulus) B).Δ ≠ 0)
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (T : Finset (ZMod modulus))
    (factorBasePositive : ∀ value ∈ T, 0 < ZMod.val value)
    (factorBaseBound : ∀ value ∈ T, ZMod.val value ≤ X)
    (TPositive : 0 < T.card)
    (fieldLarge : 324 ≤ Fintype.card (ZMod modulus))
    (intervalShort : 3 * X < modulus)
    (arityBelow :
      3 * (2 ^ (2 * (n + 1)) * (2 * (n + 1))) ^ 2 < modulus)
    (meanPositive : 0 < meanBound)
    (meanBoundAbove :
      ECDLP.L49.Mdil (ZMod modulus) n T
        ((2 * (n + 1)) * 2 ^ (2 * (n + 1) - 2)) ≤ meanBound) :
    Fintype.card (ZMod modulus)ˣ ≤ 12 *
      ((Finset.univ.filter fun u : (ZMod modulus)ˣ =>
          T.card ≤ 2 *
            (ECDLP.L43.fb
              ((u : ZMod modulus) ^ 4 * 0)
              ((u : ZMod modulus) ^ 6 * B) T).card) \
        (Finset.univ.filter fun u : (ZMod modulus)ˣ =>
          12 * meanBound ≤
            ECDLP.L43.Eng T n
              ((u : ZMod modulus) ^ 4 * 0)
              ((u : ZMod modulus) ^ 6 * B))).card := by
  classical
  let degree := (2 * (n + 1)) * 2 ^ (2 * (n + 1) - 2)
  let value : (ZMod modulus)ˣ → ℕ := fun u ↦
    (ECDLP.L43.fb
      ((u : ZMod modulus) ^ 4 * 0)
      ((u : ZMod modulus) ^ 6 * B) T).card
  let energy : (ZMod modulus)ˣ → ℕ := fun u ↦
    ECDLP.L43.Eng T n
      ((u : ZMod modulus) ^ 4 * 0)
      ((u : ZMod modulus) ^ 6 * B)
  have nonsingular : ECDLP.L43.disc (0 : ZMod modulus) B ≠ 0 := by
    simpa [ECDLP.L43.disc] using cubicNonsingular
  have originalDelta :
      (shortCharacterCurve 0 (0 : ZMod modulus) B).Δ ≠ 0 := by
    simpa [shortCharacterCurve, ECDLP.L43.shortW] using hDelta
  have factorBaseNonzero : ∀ value ∈ T, value ≠ 0 := by
    intro element elementMem elementZero
    subst elementZero
    simpa using factorBasePositive 0 elementMem
  have valueUpper : ∀ u, value u ≤ 2 * T.card := by
    intro u
    change (ECDLP.L43.fb
      ((u : ZMod modulus) ^ 4 * 0)
      ((u : ZMod modulus) ^ 6 * B) T).card ≤ 2 * T.card
    rw [← dilationFactorBaseSize_eq_dilated_fb_card
      hChar 0 B T u nonsingular]
    exact dilationFactorBaseSize_le_twice hChar 0 B T
      (⟨(u : ZMod modulus), u.ne_zero⟩ : NonzeroField (ZMod modulus))
  have valueSum :
      (∑ u : (ZMod modulus)ˣ, value u) =
        ∑ u : NonzeroField (ZMod modulus),
          dilationFactorBaseSize 0 B T u := by
    calc
      (∑ u : (ZMod modulus)ˣ, value u) =
          ∑ u : (ZMod modulus)ˣ,
            dilationFactorBaseSize 0 B T
              (⟨(u : ZMod modulus), u.ne_zero⟩ :
                NonzeroField (ZMod modulus)) := by
        apply Fintype.sum_congr
        intro u
        exact (dilationFactorBaseSize_eq_dilated_fb_card
          hChar 0 B T u nonsingular).symm
      _ = _ := Equiv.sum_comp unitsEquivNeZero
        (fun u : NonzeroField (ZMod modulus) ↦
          dilationFactorBaseSize 0 B T u)
  have valueTotalLower :
      3 * T.card * Fintype.card (ZMod modulus)ˣ ≤
        4 * ∑ u : (ZMod modulus)ˣ, value u := by
    have firstMoment := dilationFactorBaseFirstMoment
      hChar 0 B T factorBaseNonzero originalDelta
      (2 * (Nat.sqrt (Fintype.card (ZMod modulus)) + 1))
      (twiceSuccSqrt_radiusSquare (Fintype.card (ZMod modulus)))
      (twiceSuccSqrt_density (Fintype.card (ZMod modulus)) fieldLarge)
    rw [← Fintype.card_congr unitsEquivNeZero, ← valueSum] at firstMoment
    exact firstMoment
  have energyAverage := zmodCanonical_avg_energy_dil
    modulus X n hn hChar twoNonzeroBase B hDelta cubicNonsingular BNonzero
    T factorBasePositive factorBaseBound intervalShort arityBelow
  have energyTotalReal :
      (∑ u : (ZMod modulus)ˣ, (energy u : ℝ)) ≤
        (meanBound : ℝ) * Fintype.card (ZMod modulus)ˣ := by
    calc
      _ ≤ ECDLP.L49.Mdil (ZMod modulus) n T degree *
          (Fintype.card (ZMod modulus)ˣ : ℝ) := by
        simpa [energy, degree] using energyAverage
      _ ≤ (meanBound : ℝ) * Fintype.card (ZMod modulus)ˣ := by
        gcongr
  have energyTotalUpper :
      (∑ u : (ZMod modulus)ˣ, energy u) ≤
        meanBound * Fintype.card (ZMod modulus)ˣ := by
    exact_mod_cast energyTotalReal
  simpa [value, energy] using
    (firstMomentAndEnergyGiveOneTwelfthGoodSeeds
      value energy T.card meanBound TPositive meanPositive valueUpper
      valueTotalLower energyTotalUpper)

/-- A good dilation supplied by the first-moment/energy argument covers an
explicit number of points on the original curve by generic decompositions.
The strict interval-size hypothesis supplies the one unit of slack needed by
the upstream generic-coverage estimate. -/
theorem zmodCanonical_good_dilation_coverage
    (modulus n meanBound : ℕ) [Fact modulus.Prime]
    (B : ZMod modulus)
    (T : Finset (ZMod modulus))
    (TArity : 12 * n < T.card)
    (meanPositive : 0 < meanBound)
    (u : (ZMod modulus)ˣ)
    (factorBaseLarge :
      T.card ≤ 2 *
        (ECDLP.L43.fb
          ((u : ZMod modulus) ^ 4 * 0)
          ((u : ZMod modulus) ^ 6 * B) T).card)
    (energySmall :
      ECDLP.L43.Eng T n
          ((u : ZMod modulus) ^ 4 * 0)
          ((u : ZMod modulus) ^ 6 * B) <
        12 * meanBound) :
    ((((ECDLP.L43.fb
          ((u : ZMod modulus) ^ 4 * 0)
          ((u : ZMod modulus) ^ 6 * B) T).card : ℝ) ^ (n + 1) / 2) ^ 2 /
        (12 * meanBound : ℝ)) ≤
      ((Finset.univ.filter fun R :
          (ECDLP.L43.shortW (0 : ZMod modulus) B).Point =>
        (ECDLP.L46.Dec
          (ECDLP.L43.shortW (0 : ZMod modulus) B)
          (fun x => (u : ZMod modulus) ^ 2 * x ∈ T) n R).Nonempty).card) := by
  let A' : ZMod modulus := (u : ZMod modulus) ^ 4 * 0
  let B' : ZMod modulus := (u : ZMod modulus) ^ 6 * B
  let f := (ECDLP.L43.fb A' B' T).card
  have factorBaseEnough : 6 * n < f := by
    dsimp [f, A', B']
    omega
  have genericCoverage := ECDLP.L40.generic_coverage
    (n := n) (ECDLP.L43.fb A' B' T)
    (ECDLP.L43.xc (ECDLP.L43.shortW A' B'))
    (fun a => ECDLP.L43.card_fibre_le_two _ _ a)
  have genericCoverageReal :
      (((f ^ (n + 1) - 3 * n * f ^ n : ℕ) : ℝ) ^ 2) ≤
        (ECDLP.L43.Gen T n A' B' : ℝ) * ECDLP.L43.Eng T n A' B' := by
    exact_mod_cast genericCoverage
  have mainCoverage :=
    (ECDLP.L43.half_le f n factorBaseEnough).trans genericCoverageReal
  have energyUpper :
      (ECDLP.L43.Eng T n A' B' : ℝ) ≤ 12 * (meanBound : ℝ) := by
    exact_mod_cast energySmall.le
  have denominatorPositive : (0 : ℝ) < 12 * meanBound := by positivity
  rw [div_le_iff₀ denominatorPositive]
  calc
    (((f : ℝ) ^ (n + 1) / 2) ^ 2) ≤
        (ECDLP.L43.Gen T n A' B' : ℝ) * ECDLP.L43.Eng T n A' B' :=
      mainCoverage
    _ ≤ (ECDLP.L43.Gen T n A' B' : ℝ) * (12 * meanBound) :=
      mul_le_mul_of_nonneg_left energyUpper (by positivity)
    _ = ((Finset.univ.filter fun R :
          (ECDLP.L43.shortW (0 : ZMod modulus) B).Point =>
        (ECDLP.L46.Dec
          (ECDLP.L43.shortW (0 : ZMod modulus) B)
          (fun x => (u : ZMod modulus) ^ 2 * x ∈ T) n R).Nonempty).card *
          (12 * meanBound)) := by
      rw [ECDLP.L49.Gen_eq_E]
      congr 2
      apply congrArg Finset.card
      ext R
      simp

/-- For the canonical prime-field family, at least one twelfth of all
nonzero dilations have both a factor base of size at least `|T|/2` and the
stated explicit lower bound on the number of decomposable original-curve
points. -/
theorem zmodCanonical_oneTwelfth_decomposition_coverage
    (modulus X n meanBound : ℕ) [Fact modulus.Prime]
    (hn : 1 ≤ n)
    (hChar : ringChar (ZMod modulus) ≠ 2)
    (twoNonzeroBase : (2 : ZMod modulus) ≠ 0)
    (B : ZMod modulus)
    [(shortCharacterCurve 0 0 B).IsElliptic]
    (hDelta : (ECDLP.L43.shortW (0 : ZMod modulus) B).Δ ≠ 0)
    (cubicNonsingular : (27 : ZMod modulus) * B ^ 2 ≠ 0)
    (BNonzero : B ≠ 0)
    (T : Finset (ZMod modulus))
    (factorBasePositive : ∀ value ∈ T, 0 < ZMod.val value)
    (factorBaseBound : ∀ value ∈ T, ZMod.val value ≤ X)
    (TArity : 12 * n < T.card)
    (fieldLarge : 324 ≤ Fintype.card (ZMod modulus))
    (intervalShort : 3 * X < modulus)
    (arityBelow :
      3 * (2 ^ (2 * (n + 1)) * (2 * (n + 1))) ^ 2 < modulus)
    (meanPositive : 0 < meanBound)
    (meanBoundAbove :
      ECDLP.L49.Mdil (ZMod modulus) n T
        ((2 * (n + 1)) * 2 ^ (2 * (n + 1) - 2)) ≤ meanBound) :
    ∃ Good : Finset (ZMod modulus)ˣ,
      Fintype.card (ZMod modulus)ˣ ≤ 12 * Good.card ∧
      ∀ u ∈ Good,
        T.card ≤ 2 *
          (ECDLP.L43.fb
            ((u : ZMod modulus) ^ 4 * 0)
            ((u : ZMod modulus) ^ 6 * B) T).card ∧
        ((((ECDLP.L43.fb
              ((u : ZMod modulus) ^ 4 * 0)
              ((u : ZMod modulus) ^ 6 * B) T).card : ℝ) ^ (n + 1) / 2) ^ 2 /
            (12 * meanBound : ℝ)) ≤
          ((Finset.univ.filter fun R :
              (ECDLP.L43.shortW (0 : ZMod modulus) B).Point =>
            (ECDLP.L46.Dec
              (ECDLP.L43.shortW (0 : ZMod modulus) B)
              (fun x => (u : ZMod modulus) ^ 2 * x ∈ T) n R).Nonempty).card) := by
  let large := Finset.univ.filter fun u : (ZMod modulus)ˣ =>
    T.card ≤ 2 *
      (ECDLP.L43.fb
        ((u : ZMod modulus) ^ 4 * 0)
        ((u : ZMod modulus) ^ 6 * B) T).card
  let high := Finset.univ.filter fun u : (ZMod modulus)ˣ =>
    12 * meanBound ≤
      ECDLP.L43.Eng T n
        ((u : ZMod modulus) ^ 4 * 0)
        ((u : ZMod modulus) ^ 6 * B)
  refine ⟨large \ high, ?_, ?_⟩
  · simpa [large, high] using zmodCanonical_oneTwelfth_good_dilations
      modulus X n meanBound hn hChar twoNonzeroBase B hDelta
      cubicNonsingular BNonzero T factorBasePositive factorBaseBound
      (by omega) fieldLarge intervalShort arityBelow meanPositive meanBoundAbove
  · intro u hu
    have membership := Finset.mem_sdiff.mp hu
    have largeMembership := (Finset.mem_filter.mp membership.1).2
    have energySmall :
        ECDLP.L43.Eng T n
            ((u : ZMod modulus) ^ 4 * 0)
            ((u : ZMod modulus) ^ 6 * B) <
          12 * meanBound := by
      simpa [high] using membership.2
    exact ⟨largeMembership,
      zmodCanonical_good_dilation_coverage modulus n meanBound B T
        TArity meanPositive u largeMembership energySmall⟩

set_option maxHeartbeats 200000

end

end TensorBoundaryBounds
