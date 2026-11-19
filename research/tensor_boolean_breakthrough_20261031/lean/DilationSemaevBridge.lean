import TensorBoundaryBounds
import FiniteFieldSemaevVanishing
import DilationScaleExplicitDegree
import ECDLP.L49_Transport

/-!
# Canonical Semaev family at the dilation boundary

This module connects the already checked all-arity canonical resultant family
to the exact dilation-nondegeneracy interface imported by
`TensorBoundaryBounds`.  The first checkpoint is dependency compatibility;
the specialized simultaneous-scale polynomial and its laws are added here.
-/

namespace TensorBoundaryBounds

open RecursiveSemaevDegreeAudit RecursiveSemaevEvaluationAudit
open RecursiveSemaevGeometryAudit
open SemaevS3 SemaevPointListsAudit
open scoped BigOperators

/-- The imported canonical family has the proved generic last-input degree.
This binding ensures that the exact recursive construction is available in
the same environment as the upstream dilation certificate. -/
theorem canonicalSemaev_lastInputDegree
    {K : Type*} [Field K] (W : WeierstrassCurve.Affine K)
    (m : ℕ) (hm : 3 ≤ m) :
    (recursivePolynomial W (m - 3)).natDegree = 2 ^ (m - 2) :=
  all_arities_last_input_degree W m hm

/-- The canonical recursive stage whose arity is `2 * m`. -/
def evenArityStage (m : ℕ) : ℕ := 2 * m - 3

theorem evenArityStage_arity (m : ℕ) (hm : 2 ≤ m) :
    evenArityStage m + 3 = 2 * m := by
  unfold evenArityStage
  omega

/-- For `m ≥ 2`, the actual canonical `S_(2m)` family specialized along
the simultaneous scalar line through `t`. -/
noncomputable def canonicalEvenDilationPolynomial
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (hm : 2 ≤ m) (t : Fin (2 * m) → K) :
    Polynomial K :=
  DilationSemaevBridge.simultaneousScalePolynomial (ECDLP.L43.shortW A B)
    (evenArityStage m) (fun i => t (Fin.cast (evenArityStage_arity m hm) i))

/-- Exact evaluation law for the production even-arity dilation family. -/
theorem canonicalEvenDilationPolynomial_eval
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (hm : 2 ≤ m) (t : Fin (2 * m) → K) (s : K) :
    (canonicalEvenDilationPolynomial A B m hm t).eval s =
      evalCanonical (ECDLP.L43.shortW A B) (evenArityStage m)
        (fun i => s * t (Fin.cast (evenArityStage_arity m hm) i)) := by
  exact DilationSemaevBridge.simultaneousScalePolynomial_eval
    (ECDLP.L43.shortW A B) (evenArityStage m)
      (fun i => t (Fin.cast (evenArityStage_arity m hm) i)) s

/-- Total family used at the upstream interface.  A constant polynomial is
used in the arities below the recursive `S₃` base; those cases require their
own elementary bad-predicate exclusion, not a fictitious `S₀` or `S₂`. -/
noncomputable def canonicalDilationPolynomial
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (t : Fin (2 * m) → K) : Polynomial K :=
  if hm : 2 ≤ m then canonicalEvenDilationPolynomial A B m hm t else 1

theorem canonicalDilationPolynomial_of_two_le
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (hm : 2 ≤ m) (t : Fin (2 * m) → K) :
    canonicalDilationPolynomial A B m t =
      canonicalEvenDilationPolynomial A B m hm t := by
  simp [canonicalDilationPolynomial, hm]

theorem canonicalDilationPolynomial_nonzero_of_lt_two
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (hm : m < 2) (t : Fin (2 * m) → K) :
    canonicalDilationPolynomial A B m t ≠ 0 := by
  simp [canonicalDilationPolynomial, Nat.not_le.mpr hm]

/-- The exact upstream bad-point predicate supplies a root at `u⁻²` of
the canonical even-arity polynomial. -/
theorem canonicalEvenDilationPolynomial_vanishes
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (hDelta : (ECDLP.L43.shortW A B).Δ ≠ 0)
    (m : ℕ) (hm : 2 ≤ m) (t : Fin (2 * m) → K) (u : Kˣ)
    (P : Fin (2 * m) → (ECDLP.L49.Eu A B u).Point)
    (hP : ∀ i, P i ≠ 0 ∧ ECDLP.L43.xco _ (P i) = t i)
    (hSum : ∑ i, P i = 0) :
    (canonicalEvenDilationPolynomial A B m hm t).eval
        (((u : K)⁻¹) ^ 2) = 0 := by
  classical
  let castIndex : Fin (evenArityStage m + 3) → Fin (2 * m) :=
    finCongr (evenArityStage_arity m hm)
  let Q : Fin (evenArityStage m + 3) → (ECDLP.L43.shortW A B).Point :=
    fun i => ECDLP.L49.undilFun A B u (P (castIndex i))
  rw [canonicalEvenDilationPolynomial_eval]
  apply DilationSemaevBridge.evalCanonical_zero_of_manyPoints
    (ECDLP.L43.shortW A B) hDelta
  refine ⟨Q, ?_, ?_⟩
  · intro i
    have hDil : ECDLP.L49.dil A B u (Q i) = P (castIndex i) := by
      exact ECDLP.L49.dil_undil A B u _
    have hQne : Q i ≠ 0 := by
      intro hZero
      have : P (castIndex i) = 0 := by
        rw [← hDil, hZero, map_zero]
      exact (hP (castIndex i)).1 this
    have hCoord : ECDLP.L43.xco _ (Q i) =
        ((u : K)⁻¹) ^ 2 * t (castIndex i) := by
      have hScaled : (u : K) ^ 2 * ECDLP.L43.xco _ (Q i) =
          t (castIndex i) := by
        rw [← ECDLP.L49.xco_dil A B u, hDil, (hP (castIndex i)).2]
      calc
        ECDLP.L43.xco _ (Q i) =
            ((u : K)⁻¹) ^ 2 * ((u : K) ^ 2 * ECDLP.L43.xco _ (Q i)) := by
          field_simp
        _ = ((u : K)⁻¹) ^ 2 * t (castIndex i) := by rw [hScaled]
    cases hQi : Q i with
    | zero => exact (hQne hQi).elim
    | some x y hxy =>
        change some x = some (((u : K)⁻¹) ^ 2 * t (castIndex i))
        congr
        simpa [Q, ECDLP.L43.xco, hQi] using hCoord
  · apply ECDLP.L49.dil_injective A B u
    rw [map_zero, map_sum]
    simp_rw [Q, ECDLP.L49.dil_undil]
    change (∑ i, P (finCongr (evenArityStage_arity m hm) i)) = 0
    rw [(finCongr (evenArityStage_arity m hm)).sum_comp, hSum]

/-- The total canonical family satisfies the exact upstream vanishing
obligation in every recursive arity (`m ≥ 2`). -/
theorem canonicalDilationPolynomial_vanishing_of_two_le
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (hDelta : (ECDLP.L43.shortW A B).Δ ≠ 0)
    (m : ℕ) (hm : 2 ≤ m) :
    ∀ (t : Fin (2 * m) → K) (u : Kˣ),
      (∃ P : Fin (2 * m) → (ECDLP.L49.Eu A B u).Point,
        (∀ i, P i ≠ 0 ∧ ECDLP.L43.xco _ (P i) = t i) ∧
          ∑ i, P i = 0) →
      (canonicalDilationPolynomial A B m t).eval
          (((u : K)⁻¹) ^ 2) = 0 := by
  intro t u bad
  obtain ⟨P, hP, hSum⟩ := bad
  rw [canonicalDilationPolynomial_of_two_le A B m hm t]
  exact canonicalEvenDilationPolynomial_vanishes
    A B hDelta m hm t u P hP hSum

/-- Tuple-independent structural degree bound obtained directly from the
nested canonical polynomial before any coefficient specialization. -/
noncomputable def canonicalEvenDilationDegree
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) : ℕ :=
  DilationSemaevBridge.canonicalScaleDegreeBound (ECDLP.L43.shortW A B)
    (evenArityStage m)

theorem canonicalEvenDilationPolynomial_degree_le
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (hm : 2 ≤ m) (t : Fin (2 * m) → K) :
    (canonicalEvenDilationPolynomial A B m hm t).natDegree ≤
      canonicalEvenDilationDegree A B m := by
  exact DilationSemaevBridge.simultaneousScalePolynomial_natDegree_le
    (ECDLP.L43.shortW A B) (evenArityStage m)
      (fun i => t (Fin.cast (evenArityStage_arity m hm) i))

/-- Closed arity-only degree bound.  This coarse bound is obtained from
all-input symmetry and the exact individual-variable degree of the canonical
Semaev polynomial; it is independent of the curve, field, and tuple. -/
theorem canonicalEvenDilationPolynomial_degree_le_explicit
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (hm : 2 ≤ m) (t : Fin (2 * m) → K) :
    (canonicalEvenDilationPolynomial A B m hm t).natDegree ≤
      (2 * m) * 2 ^ (2 * m - 2) := by
  calc
    (canonicalEvenDilationPolynomial A B m hm t).natDegree ≤
        (evenArityStage m + 3) * 2 ^ (evenArityStage m + 1) :=
      DilationSemaevBridge.simultaneousScalePolynomial_natDegree_le_explicit
        (ECDLP.L43.shortW A B) (evenArityStage m)
          (fun i => t (Fin.cast (evenArityStage_arity m hm) i))
    _ = (2 * m) * 2 ^ (2 * m - 2) := by
      congr 1
      · exact evenArityStage_arity m hm
      · congr 1
        unfold evenArityStage
        omega

theorem canonicalDilationPolynomial_degree_le_structural
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (hm : 2 ≤ m) (t : Fin (2 * m) → K) :
    (canonicalDilationPolynomial A B m t).natDegree ≤
      canonicalEvenDilationDegree A B m := by
  rw [canonicalDilationPolynomial_of_two_le A B m hm t]
  exact canonicalEvenDilationPolynomial_degree_le A B m hm t

theorem canonicalDilationPolynomial_degree_le_explicit
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (hm : 2 ≤ m) (t : Fin (2 * m) → K) :
    (canonicalDilationPolynomial A B m t).natDegree ≤
      (2 * m) * 2 ^ (2 * m - 2) := by
  rw [canonicalDilationPolynomial_of_two_le A B m hm t]
  exact canonicalEvenDilationPolynomial_degree_le_explicit A B m hm t

/-- The exact maximum degree of the canonical specialized family over the
finite tuple space.  This is a rigorous uniform bound, but deliberately makes
no asymptotic claim; an explicit arity-only upper bound is a separate
obligation. -/
noncomputable def canonicalDilationDegree
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) : ℕ :=
  Finset.univ.sup fun t : Fin (2 * m) → K =>
    (canonicalDilationPolynomial A B m t).natDegree

theorem canonicalDilationPolynomial_degree_le_max
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (t : Fin (2 * m) → K) :
    (canonicalDilationPolynomial A B m t).natDegree ≤
      canonicalDilationDegree A B m := by
  classical
  exact Finset.le_sup (s := Finset.univ)
    (f := fun t : Fin (2 * m) → K =>
      (canonicalDilationPolynomial A B m t).natDegree)
    (Finset.mem_univ t)

theorem canonicalDilationDegree_le_structural
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (m : ℕ) (hm : 2 ≤ m) :
    canonicalDilationDegree A B m ≤ canonicalEvenDilationDegree A B m := by
  classical
  unfold canonicalDilationDegree
  apply Finset.sup_le
  intro t ht
  exact canonicalDilationPolynomial_degree_le_structural A B m hm t

/-- For `m ≥ 2`, vanishing is no longer an assumption in the exact upstream
bridge: only nonzeroness on nontrivial tuples and a uniform degree bound
remain. -/
theorem canonicalDilationPolynomial_bounds_imply_nondeg
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (hDelta : (ECDLP.L43.shortW A B).Δ ≠ 0)
    (m : ℕ) (hm : 2 ≤ m) (T : Finset K) (degree : ℕ)
    (nonzero : ∀ t : Fin (2 * m) → K, (∀ i, t i ∈ T) →
      ¬ ECDLP.L40.TrivL (List.ofFn t) →
        canonicalDilationPolynomial A B m t ≠ 0)
    (degreeBound : ∀ t : Fin (2 * m) → K,
      (canonicalDilationPolynomial A B m t).natDegree ≤ degree) :
    ECDLP.L49.DilationNondeg A B m T degree := by
  apply upstreamDilationPolynomialFamily_implies_nondeg
    A B m T degree (canonicalDilationPolynomial A B m)
  · exact nonzero
  · exact degreeBound
  · exact canonicalDilationPolynomial_vanishing_of_two_le A B hDelta m hm

/-- With the finite maximum supplying degree control, the only remaining
curve-specific hypothesis is nonzeroness on upstream nontrivial tuples. -/
theorem canonicalDilationPolynomial_nonzero_implies_nondeg
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (hDelta : (ECDLP.L43.shortW A B).Δ ≠ 0)
    (m : ℕ) (hm : 2 ≤ m) (T : Finset K)
    (nonzero : ∀ t : Fin (2 * m) → K, (∀ i, t i ∈ T) →
      ¬ ECDLP.L40.TrivL (List.ofFn t) →
        canonicalDilationPolynomial A B m t ≠ 0) :
    ECDLP.L49.DilationNondeg A B m T
      (canonicalDilationDegree A B m) := by
  exact canonicalDilationPolynomial_bounds_imply_nondeg
    A B hDelta m hm T (canonicalDilationDegree A B m) nonzero
      (canonicalDilationPolynomial_degree_le_max A B m)

/-- Structural-degree version of the reduction.  Unlike the finite maximum,
this degree is computed from the unspecialized canonical polynomial and is
therefore independent of the tuple space enumeration. -/
theorem canonicalDilationPolynomial_nonzero_implies_nondeg_structural
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (hDelta : (ECDLP.L43.shortW A B).Δ ≠ 0)
    (m : ℕ) (hm : 2 ≤ m) (T : Finset K)
    (nonzero : ∀ t : Fin (2 * m) → K, (∀ i, t i ∈ T) →
      ¬ ECDLP.L40.TrivL (List.ofFn t) →
        canonicalDilationPolynomial A B m t ≠ 0) :
    ECDLP.L49.DilationNondeg A B m T
      (canonicalEvenDilationDegree A B m) := by
  exact canonicalDilationPolynomial_bounds_imply_nondeg
    A B hDelta m hm T (canonicalEvenDilationDegree A B m) nonzero
      (canonicalDilationPolynomial_degree_le_structural A B m hm)

/-- Fully explicit-degree reduction: for every recursive arity, the sole
remaining family-specific obligation is nonzeroness on nontrivial tuples. -/
theorem canonicalDilationPolynomial_nonzero_implies_nondeg_explicit
    {K : Type*} [Field K] [Fintype K] [DecidableEq K]
    (A B : K) (hDelta : (ECDLP.L43.shortW A B).Δ ≠ 0)
    (m : ℕ) (hm : 2 ≤ m) (T : Finset K)
    (nonzero : ∀ t : Fin (2 * m) → K, (∀ i, t i ∈ T) →
      ¬ ECDLP.L40.TrivL (List.ofFn t) →
        canonicalDilationPolynomial A B m t ≠ 0) :
    ECDLP.L49.DilationNondeg A B m T
      ((2 * m) * 2 ^ (2 * m - 2)) := by
  exact canonicalDilationPolynomial_bounds_imply_nondeg
    A B hDelta m hm T ((2 * m) * 2 ^ (2 * m - 2)) nonzero
      (canonicalDilationPolynomial_degree_le_explicit A B m hm)

end TensorBoundaryBounds
