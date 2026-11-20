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

end

end TensorBoundaryBounds
