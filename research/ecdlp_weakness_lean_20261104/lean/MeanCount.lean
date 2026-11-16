import Mathlib

/-!
# The mean number of factor-base representations is exactly |F|^m / |G|

Corpus target: `HEUR-DECOMPOSITION-PROBABILITY`. Its statement separates the
mean ("follows by counting") from the distribution shape (heuristic). This file
pins the counting half: over a finite additive group `G`, for a factor base
`F ⊆ G` and arity `m`, the ordered representation counts `N(R) = #{f : Fin m → F
| Σ f = R}` sum to `|F|^m` over all targets `R`, so their mean over the `|G|`
targets is `|F|^m / |G|`. No independence or distribution assumption appears.
-/

open scoped BigOperators

namespace MeanCount

variable {G : Type*} [AddCommGroup G] [Fintype G] [DecidableEq G]

/-- Ordered representations of `R` as a sum of `m` factor-base elements. -/
def reps (F : Finset G) (m : ℕ) (R : G) : Finset (Fin m → F) :=
  Finset.univ.filter fun f => (∑ i, (f i : G)) = R

theorem sum_card_reps (F : Finset G) (m : ℕ) :
    ∑ R : G, (reps F m R).card = F.card ^ m := by
  simp only [reps]
  rw [← Finset.card_eq_sum_card_fiberwise (s := (Finset.univ : Finset (Fin m → F)))
      (t := Finset.univ) (f := fun f => ∑ i, (f i : G)) (fun _ _ => Finset.mem_univ _)]
  simp [Finset.card_univ, Fintype.card_coe]

/-- The mean over all `|G|` targets, as a rational number. -/
theorem mean_reps (F : Finset G) (m : ℕ) :
    ((∑ R : G, (reps F m R).card : ℕ) : ℚ) / Fintype.card G =
      (F.card ^ m : ℚ) / Fintype.card G := by
  rw [sum_card_reps]
  push_cast
  rfl

end MeanCount
