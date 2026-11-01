import JKSSTAudit
import JKSSTExplicitConstant
import JKSSTPacking

/-!
# A finite replacement for the JKSST height-ball argument

This theorem composes the exact pieces of the repaired argument:

1. a finite set of partial sums in the Mordell--Weil group;
2. projection to a finite, separated subset of a Euclidean space;
3. a uniform bound `T` on the projection fibers (the torsion factor);
4. the exact packing bound; and
5. finite pigeonhole with an explicit coefficient radius.

No lattice asymptotic or `O`-term occurs.  The arithmetic work needed in an
elliptic-curve application is visible in the projection, fiber, radius,
separation, and `base_le` hypotheses.
-/

open scoped BigOperators

namespace JKSSTFiniteHeightRepair

variable {G E ι : Type*} [AddCommGroup G] [Fintype ι]
  [NormedAddCommGroup E] [NormedSpace ℝ E] [FiniteDimensional ℝ E]

/-- An end-to-end finite geometry-of-numbers replacement for JKSST Lemma 4.1.

Write `k = finrank E` and `C = T*A^k`.  The hypothesis `base_le` says that
the exact packing base for the coefficient-`C` partial-sum ball is at most
the natural number `A*C`.  If the generated rank has codimension at least one,
then two coefficient vectors collide and give a nonzero relation bounded by
`C`.
-/
theorem bounded_relation_from_exact_packing
    (P : ι → G) (T A : ℕ)
    (rank_gap : Module.finrank ℝ E + 1 ≤ Fintype.card ι)
    (S : Finset G) (s : Finset E) (π : G →+ E)
    {R δ : ℝ} (hR : 0 ≤ R) (hδ : 0 < δ)
    (range_mem : ∀ c : ι → Fin (T * A ^ Module.finrank ℝ E + 1),
      (∑ i, (c i : ℤ) • P i) ∈ S)
    (mapsTo : ∀ x ∈ S, π x ∈ s)
    (fiberBound : ∀ y ∈ s, {x : G | x ∈ S ∧ π x = y}.ncard ≤ T)
    (inside : ∀ y ∈ s, ‖y‖ ≤ R)
    (separated : ∀ y ∈ s, ∀ z ∈ s, y ≠ z → δ ≤ ‖y - z‖)
    (base_le : 1 + 2 * R / δ ≤
      ((A * (T * A ^ Module.finrank ℝ E) : ℕ) : ℝ)) :
    ∃ m : ι → ℤ,
      m ≠ 0 ∧
      (∀ i, |m i| ≤ T * A ^ Module.finrank ℝ E) ∧
      ∑ i, m i • P i = 0 := by
  apply JKSSTAudit.bounded_relation_of_explicit_packing_count
    P T A (Module.finrank ℝ E) rank_gap S range_mem
  exact JKSSTPacking.card_le_fiberBound_mul_nat_pow
    S s π T (A * (T * A ^ Module.finrank ℝ E))
    hR hδ mapsTo fiberBound inside separated base_le

end JKSSTFiniteHeightRepair
