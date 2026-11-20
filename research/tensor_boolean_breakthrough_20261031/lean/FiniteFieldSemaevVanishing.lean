import RecursiveSemaevNaturality

/-!
# Field-independent forward vanishing

The converse geometric theorem genuinely needs algebraic closure.  Its
forward direction for an already supplied tuple of base-field points does
not: map the tuple to an algebraic closure, use the proved geometric theorem
there, and descend the zero through the injective coefficient map.
-/

namespace DilationSemaevBridge

noncomputable section

open SemaevS3 SemaevPointListsAudit RecursiveSemaevEvaluationAudit
open RecursiveSemaevGeometryAudit
open scoped BigOperators

universe u

variable {K : Type u} [Field K] [DecidableEq K]

/-- Actual base-field points summing to zero force the canonical Semaev
polynomial to vanish.  No algebraic-closure instance is assumed on `K`. -/
theorem evalCanonical_zero_of_manyPoints
    (W : WeierstrassCurve.Affine K) (hDelta : W.Δ ≠ 0)
    (n : ℕ) (xs : Fin (n + 3) → K) (hPoints : ManyPoints W xs) :
    evalCanonical W n xs = 0 := by
  classical
  let L := AlgebraicClosure K
  let phi : K →+* L := algebraMap K L
  let pointMap : W.Point →+ (W.baseChange L).Point :=
    WeierstrassCurve.Affine.Point.map (W' := W) (Algebra.ofId K L)
  obtain ⟨P, hX, hSum⟩ := hPoints
  have hDeltaL : (W.baseChange L).Δ ≠ 0 := by
    change (W.map (algebraMap K L)).Δ ≠ 0
    rw [WeierstrassCurve.map_Δ]
    intro hZero
    apply hDelta
    apply (algebraMap K L).injective
    rw [hZero, map_zero]
  have pointX_map (Q : W.Point) :
      pointX (pointMap Q) = (pointX Q).map phi := by
    cases Q <;> rfl
  have hXL : ∀ i, pointX (pointMap (P i)) = some (phi (xs i)) := by
    intro i
    rw [pointX_map, hX i]
    rfl
  have hSumL : ∑ i, pointMap (P i) = 0 := by
    rw [← map_sum, hSum, map_zero]
  have hZeroL :
      evalCanonical (W.baseChange L) n (fun i => phi (xs i)) = 0 := by
    apply (canonical_zero_iff (W.baseChange L) hDeltaL n _).2
    exact ⟨fun i => pointMap (P i), hXL, hSumL⟩
  have hMap := map_evalCanonical phi W n xs
  have hZeroMap :
      evalCanonical (W.map phi) n (fun i => phi (xs i)) = 0 := by
    simpa [WeierstrassCurve.baseChange, phi] using hZeroL
  rw [hZeroMap] at hMap
  exact phi.injective (by simpa using hMap)

end

end DilationSemaevBridge
