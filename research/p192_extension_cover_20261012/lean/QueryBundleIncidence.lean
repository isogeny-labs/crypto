import Mathlib.Combinatorics.Enumerative.DoubleCounting

/-!
Incidence bounds for complete three-cut query bundles. The mathematical
objects here are finite families of six distinct projected columns. The
concrete P-192 coordinate certificate and the rank of its endpointwise
difference rows are replayed separately.
-/

namespace P192ExtensionCover

open Finset

variable {ι κ : Type*} [Fintype ι] [DecidableEq ι] [DecidableEq κ]

/-- Double-counting the incidences between bundles and their projected
columns. If every bundle has `m` columns and every column belongs to at
most `d` bundles, then `m * bundles ≤ d * columns`. -/
theorem query_bundle_incidence_bound (I : Finset ι) (support : ι → Finset κ)
    (m d : ℕ)
    (hsize : ∀ i ∈ I, (support i).card = m)
    (hdegree : ∀ x ∈ I.biUnion support,
      #{i | i ∈ I ∧ x ∈ support i} ≤ d) :
    m * I.card ≤ d * (I.biUnion support).card := by
  classical
  have hleft : (∑ i ∈ I, (support i).card) = m * I.card := by
    calc
      _ = ∑ _i ∈ I, m := by
        apply Finset.sum_congr rfl
        intro i hi
        exact hsize i hi
      _ = m * I.card := by simp [mul_comm]
  have hdouble :
      (∑ i ∈ I, (support i).card) =
        ∑ x ∈ I.biUnion support, #{i | i ∈ I ∧ x ∈ support i} :=
    Finset.sum_card_eq_sum_biUnion_card support I
  have hright :
      (∑ x ∈ I.biUnion support, #{i | i ∈ I ∧ x ∈ support i}) ≤
        ∑ _x ∈ I.biUnion support, d := by
    apply Finset.sum_le_sum
    intro x hx
    exact hdegree x hx
  simpa [Finset.sum_const, nsmul_eq_mul, mul_comm] using
    hleft.symm.trans_le (hdouble.trans_le hright)

/-- Pairwise recurrence cannot close the column set for nonempty
endpointwise difference-row families: six columns per bundle and degree
at most two force more columns than the rank upper bound. -/
theorem query_bundle_pair_reuse_rank_gap (I : Finset ι)
    (support : ι → Finset κ) (rank endpoints : ℕ)
    (hsize : ∀ i ∈ I, (support i).card = 6)
    (hdegree : ∀ x ∈ I.biUnion support,
      #{i | i ∈ I ∧ x ∈ support i} ≤ 2)
    (hrank : rank + endpoints ≤ I.card)
    (hendpoints : 0 < endpoints) :
    rank < (I.biUnion support).card := by
  have hincidence := query_bundle_incidence_bound I support 6 2 hsize hdegree
  omega

/-- For 250 six-column bundles, a column set of at most 181 elements
requires maximum bundle multiplicity at least nine. This is a necessary
condition for column closure at the frozen rank, not a sufficient one. -/
theorem query_250_bundle_closure_requires_ninefold_reuse
    (I : Finset ι) (support : ι → Finset κ) (d : ℕ)
    (hbundles : I.card = 250)
    (hsize : ∀ i ∈ I, (support i).card = 6)
    (hdegree : ∀ x ∈ I.biUnion support,
      #{i | i ∈ I ∧ x ∈ support i} ≤ d)
    (hclosure : (I.biUnion support).card ≤ 181) :
    9 ≤ d := by
  have hincidence := query_bundle_incidence_bound I support 6 d hsize hdegree
  by_contra h
  have hd : d ≤ 8 := by omega
  have hupper : d * (I.biUnion support).card ≤ 8 * 181 :=
    Nat.mul_le_mul hd hclosure
  omega

#print axioms query_bundle_incidence_bound
#print axioms query_bundle_pair_reuse_rank_gap
#print axioms query_250_bundle_closure_requires_ninefold_reuse

end P192ExtensionCover
