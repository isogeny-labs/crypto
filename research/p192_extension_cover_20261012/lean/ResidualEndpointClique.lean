import Mathlib.Tactic.Abel
import Mathlib.Combinatorics.Enumerative.DoubleCounting

/-!
The complete bundle at a residual endpoint of a split same-cubic root-pair
line contains the other residual twice.  Its six column appearances are a
five-element support when those five points are distinct.  The group
equation remains the ordinary known-query bundle equation with multiplicity.
-/

namespace P192ExtensionCover

open Finset

variable {G : Type*} [AddCommGroup G]

/-- Two cubic roots `P,Q` and residuals `R,T` lie on one line section.
At endpoint `R`, the cuts from `P` and `Q` each return `T` and the other
root. A cut from the third root `W` returns `Y,Z`. The resulting bundle has
six appearances, including `T` twice, and satisfies the same known-query
equation as any complete three-cut bundle. -/
theorem same_cubic_line_duplicate_residual_bundle
    (P Q W R T Y Z S C q : G) (k : ℕ)
    (hline : P + Q + R + T = S)
    (hthird : W + R + Y + Z = S)
    (hcubic : P + Q + W + k • q = C) :
    (P + Q + T + T + Y + Z) + 3 • R + C = 3 • S + k • q := by
  have hpq : P + Q = S - R - T := by
    calc
      P + Q = (P + Q + R + T) - R - T := by abel
      _ = S - R - T := by rw [hline]
  have hw : W = S - R - Y - Z := by
    calc
      W = (W + R + Y + Z) - R - Y - Z := by abel
      _ = S - R - Y - Z := by rw [hthird]
  calc
    (P + Q + T + T + Y + Z) + 3 • R + C =
        (P + Q + T + T + Y + Z) + 3 • R +
          (P + Q + W + k • q) := by rw [hcubic]
    _ = 3 • S + k • q := by rw [hpq, hw]; abel

#print axioms same_cubic_line_duplicate_residual_bundle

variable {ι κ : Type*} [Fintype ι] [DecidableEq ι] [DecidableEq κ]

/-- Double-count bundle-column incidences when supports have different sizes.
This applies directly to the five- and six-column bundles in the residual
endpoint census; a repeated column within one bundle is counted once. -/
theorem mixed_query_bundle_incidence_bound
    (I : Finset ι) (support : ι → Finset κ) (d : ℕ)
    (hdegree : ∀ x ∈ I.biUnion support,
      #{i | i ∈ I ∧ x ∈ support i} ≤ d) :
    (∑ i ∈ I, (support i).card) ≤ d * (I.biUnion support).card := by
  classical
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
    hdouble.trans_le hright

/-- The frozen combined target geometry has 283 selected bundles with
1,690 distinct bundle-column incidences. Closing at its rank 219 would
require some projected column in at least eight different bundles. This is
a necessary bound at the frozen incidence and rank, not a construction. -/
theorem targeted_bundle_closure_requires_eightfold_reuse
    (I : Finset ι) (support : ι → Finset κ) (d : ℕ)
    (hincidences : (∑ i ∈ I, (support i).card) = 1690)
    (hdegree : ∀ x ∈ I.biUnion support,
      #{i | i ∈ I ∧ x ∈ support i} ≤ d)
    (hclosure : (I.biUnion support).card ≤ 219) :
    8 ≤ d := by
  have hbound := mixed_query_bundle_incidence_bound I support d hdegree
  rw [hincidences] at hbound
  by_contra h
  have hd : d ≤ 7 := by omega
  have hupper : d * (I.biUnion support).card ≤ 7 * 219 :=
    Nat.mul_le_mul hd hclosure
  omega

#print axioms mixed_query_bundle_incidence_bound
#print axioms targeted_bundle_closure_requires_eightfold_reuse

end P192ExtensionCover
