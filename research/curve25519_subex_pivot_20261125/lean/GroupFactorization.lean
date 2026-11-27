module

public import Mathlib

@[expose] public section

/-!
# Finite mode-to-group factorization identities

This module kernel-checks three algebraic steps used by the producer-side
factorization argument: row-wise expansion, finite Fubini across disjoint
group assignments, and precision truncation of terms carrying a sufficiently
large power of two. It does not connect these identities to Rust objects or
prove the scalar mode-series identity.
-/

open scoped BigOperators

namespace Curve25519GroupFactorization

/-- Expanding a finite product of finite row sums enumerates each row-choice
function exactly once. -/
theorem row_choice_expansion
    {Row : Type*} [Fintype Row] [DecidableEq Row]
    {Choice : Row → Type*} [∀ row, Fintype (Choice row)]
    {R : Type*} [CommSemiring R]
    (factor : ∀ row, Choice row → R) :
    (∏ row, ∑ choice, factor row choice) =
      ∑ selection : ∀ row, Choice row, ∏ row, factor row (selection row) := by
  exact Fintype.prod_sum factor

/-- The packet row factor with `none` selecting the local affine term and
`some mode` selecting one crossing-mode contribution. -/
def packetRowFactor
    {Row Mode R : Type*} [Mul R]
    (localTerm : Row → R)
    (coefficient : Row → Mode → R)
    (sourceForm : Mode → R)
    (row : Row) : Option Mode → R
  | none => localTerm row
  | some mode => coefficient row mode * sourceForm mode

/-- The direct packet row product expands into the choice functions that are
later regrouped by mode exponent. -/
theorem packet_row_choice_expansion
    {Row Mode R : Type*}
    [Fintype Row] [DecidableEq Row] [Fintype Mode] [CommSemiring R]
    (localTerm : Row → R)
    (coefficient : Row → Mode → R)
    (sourceForm : Mode → R) :
    (∏ row, (localTerm row + ∑ mode, coefficient row mode * sourceForm mode)) =
      ∑ selection : Row → Option Mode,
        ∏ row, packetRowFactor localTerm coefficient sourceForm row (selection row) := by
  rw [← row_choice_expansion]
  congr with row
  simp [packetRowFactor]

/-- A global Boolean incidence assignment is equivalent to one independent
assignment on every group fiber. -/
def assignmentByGroupEquiv
    {Incidence Group : Type*}
    (group : Incidence → Group) :
    (Incidence → Bool) ≃
      (∀ target : Group, {incidence // group incidence = target} → Bool) where
  toFun assignment _ incidence := assignment incidence.1
  invFun assignments incidence :=
    assignments (group incidence) ⟨incidence, rfl⟩
  left_inv _ := rfl
  right_inv assignments := by
    funext target incidence
    obtain ⟨incidence, hgroup⟩ := incidence
    subst target
    rfl

/-- Once every factor depends only on one group fiber, summing over all global
incidence assignments factors as the product of the group contractions. -/
theorem partitioned_assignment_factorization
    {Incidence Group R : Type*}
    [Fintype Incidence] [DecidableEq Incidence]
    [Fintype Group] [DecidableEq Group] [CommSemiring R]
    (group : Incidence → Group)
    (groupTerm : ∀ target : Group,
      ({incidence // group incidence = target} → Bool) → R) :
    (∑ assignment : Incidence → Bool,
        ∏ target, groupTerm target (fun incidence ↦ assignment incidence.1)) =
      ∏ target, ∑ assignment, groupTerm target assignment := by
  rw [Fintype.prod_sum]
  exact Fintype.sum_equiv (assignmentByGroupEquiv group) _ _ (fun _ ↦ rfl)

/-- Any finite sum can be regrouped by the fibers of a finite key map. Applied
to row choices, the key is the resulting mode-exponent vector. -/
theorem sum_regroup_by_fibers
    {Source Target R : Type*}
    [Fintype Source] [Fintype Target] [DecidableEq Target]
    [AddCommMonoid R]
    (key : Source → Target) (term : Source → R) :
    (∑ source, term source) =
      ∑ target, ∑ source, if key source = target then term source else 0 := by
  rw [Finset.sum_comm]
  apply Finset.sum_congr rfl
  intro source _
  simp

/-- Multiplying terms with known powers of two adds their weights exactly. -/
theorem product_pow_two_weight
    {Term : Type*} [Fintype Term]
    (weight coefficient : Term → ℕ) :
    (∏ term, 2 ^ weight term * coefficient term) =
      2 ^ (∑ term, weight term) * ∏ term, coefficient term := by
  rw [Finset.prod_mul_distrib, Finset.prod_pow_eq_pow_sum]

/-- A term carrying at least `2^p` is zero in the coefficient ring modulo
`2^p`. -/
theorem pow_two_multiple_vanishes
    (p weight coefficient : ℕ)
    (hweight : p ≤ weight) :
    ((2 ^ weight * coefficient : ℕ) : ZMod (2 ^ p)) = 0 := by
  obtain ⟨extra, rfl⟩ := Nat.exists_eq_add_of_le hweight
  rw [pow_add]
  simp [mul_assoc]

/-- A finite collection of terms all above the precision cutoff contributes
zero modulo `2^p`. -/
theorem high_weight_sum_vanishes
    {Term : Type*} [Fintype Term]
    (p : ℕ) (weight coefficient : Term → ℕ)
    (hweight : ∀ term, p ≤ weight term) :
    (∑ term, ((2 ^ weight term * coefficient term : ℕ) : ZMod (2 ^ p))) = 0 := by
  apply Finset.sum_eq_zero
  intro term _
  exact pow_two_multiple_vanishes p (weight term) (coefficient term) (hweight term)

/-- A powers-of-two shifted element becomes nilpotent at the coefficient
precision once its accumulated valuation reaches the modulus exponent. -/
theorem shifted_power_nilpotent
    (precision shift exponent coefficient : ℕ)
    (hvaluation : precision ≤ shift * exponent) :
    (((2 ^ shift * coefficient : ℕ) : ZMod (2 ^ precision)) ^ exponent) = 0 := by
  simpa [mul_pow, pow_mul] using
    pow_two_multiple_vanishes precision (shift * exponent)
      (coefficient ^ exponent) hvaluation

/-- The finite geometric series is an inverse of `denominator-step` whenever
the normalized step is nilpotent at the truncation length. -/
theorem nilpotent_geometric_inverse
    {R : Type*} [CommRing R]
    (denominator denominatorInv step : R) (terms : ℕ)
    (hinverse : denominator * denominatorInv = 1)
    (hnilpotent : (step * denominatorInv) ^ terms = 0) :
    (denominator - step) *
        (denominatorInv * ∑ index ∈ Finset.range terms,
          (step * denominatorInv) ^ index) = 1 := by
  let ratio := step * denominatorInv
  calc
    (denominator - step) *
          (denominatorInv * ∑ index ∈ Finset.range terms, ratio ^ index) =
        (1 - ratio) * ∑ index ∈ Finset.range terms, ratio ^ index := by
          dsimp [ratio]
          rw [sub_mul, ← mul_assoc denominator denominatorInv, hinverse, one_mul,
            ← mul_assoc step denominatorInv]
          ring
    _ = 1 - ratio ^ terms := mul_neg_geom_sum ratio terms
    _ = 1 := by rw [hnilpotent]; simp

/-- An actual inverse of the shifted denominator equals the truncated
geometric expression; inverses are unique in a commutative ring. -/
theorem inverse_eq_nilpotent_geometric
    {R : Type*} [CommRing R]
    (denominator denominatorInv step shiftedInv : R) (terms : ℕ)
    (hinverse : denominator * denominatorInv = 1)
    (hshiftedInverse : (denominator - step) * shiftedInv = 1)
    (hnilpotent : (step * denominatorInv) ^ terms = 0) :
    shiftedInv = denominatorInv *
      ∑ index ∈ Finset.range terms, (step * denominatorInv) ^ index := by
  let candidate := denominatorInv *
    ∑ index ∈ Finset.range terms, (step * denominatorInv) ^ index
  have hcandidate : (denominator - step) * candidate = 1 := by
    exact nilpotent_geometric_inverse denominator denominatorInv step terms
      hinverse hnilpotent
  calc
    shiftedInv = shiftedInv * 1 := by simp
    _ = shiftedInv * ((denominator - step) * candidate) := by rw [hcandidate]
    _ = candidate * ((denominator - step) * shiftedInv) := by ring
    _ = candidate := by rw [hshiftedInverse]; simp

end Curve25519GroupFactorization
