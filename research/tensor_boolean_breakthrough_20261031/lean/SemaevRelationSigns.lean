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

end

end TensorBoundaryBounds
