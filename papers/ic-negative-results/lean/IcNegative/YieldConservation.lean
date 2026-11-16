import Mathlib

/-!
# Mean decomposition yield is fixed by the size of the domain (statement S3)

Companion to `papers/ic-negative-results/`.  A *decomposition map* is any
`φ : τ → G` from a finite domain of candidate tuples `τ` (the `m`-multisets of a
factor base, Frobenius-rotated slots, partial sums, ...) to a finite group, or
just a finite type, `G` of targets.  The *yield* at a target `g` is the number
of tuples landing on `g`.

* `sum_yield`: summed over all targets the yield is the number of tuples,
  whatever `φ` is (a double count).  So the mean yield per target is
  `|τ| / |G|` for every map, i.e. for every factor-base geometry.
* `total_yield_eq`: two maps on domains of equal size have equal total yield.
  Instantiated with the `m`-multisets of a base of size `B` (a domain of size
  `Nat.choose (B + m - 1) m`, `Sym.card_sym_eq_choose` in Mathlib, not imported
  here), this is `crypto-autoresearcher:knowledge/findings/KN-FIND-007.md`.
* `coverage_le_card_dom`, `coverage_le_card_cod`: the number of targets that have
  at least one decomposition is at most `min (|τ|, |G|)`, so only the
  *distribution* of the fixed total is free.

Already kernel-checked in the list form in
`research/factor_base_designs_20261015/lean/FactorBaseDesigns/Yield.lean`
(`sum_count`, `patterns_neutral`); this file restates the identity for `Finset`
fibres over finite types and adds the coverage bound.
-/

namespace IcNegative

open Finset

variable {τ G : Type*} [Fintype τ] [Fintype G] [DecidableEq G]

/-- Yield of the decomposition map `φ` at the target `g`. -/
def yieldAt (φ : τ → G) (g : G) : ℕ :=
  (Finset.univ.filter (fun t => φ t = g)).card

/-- **Yield conservation.**  The total yield equals the number of tuples. -/
theorem sum_yield (φ : τ → G) : ∑ g, yieldAt φ g = Fintype.card τ := by
  first
  | (have h := Finset.card_eq_sum_card_fiberwise (f := φ) (s := (Finset.univ : Finset τ))
        (t := (Finset.univ : Finset G)) (fun x _ => Finset.mem_univ (φ x))
     simpa [yieldAt] using h.symm)
  | (have h := Finset.sum_fiberwise (Finset.univ : Finset τ) φ (fun _ => (1 : ℕ))
     simp only [yieldAt, Finset.card_eq_sum_ones]
     simpa using h)

/-- The mean yield per target, as a rational, is `|τ| / |G|`. -/
theorem sum_yield_cast (φ : τ → G) : (∑ g, (yieldAt φ g : ℚ)) = Fintype.card τ := by
  rw [← Nat.cast_sum, sum_yield]

/-- **Geometry invariance (K2/K5 of the register, in the mean).**  Two decomposition
maps whose domains have the same size have the same total yield. -/
theorem total_yield_eq {σ : Type*} [Fintype σ] (φ : τ → G) (ψ : σ → G)
    (h : Fintype.card τ = Fintype.card σ) :
    ∑ g, yieldAt φ g = ∑ g, yieldAt ψ g := by
  rw [sum_yield, sum_yield]
  exact h

/-- Number of targets with at least one decomposition. -/
def coverage (φ : τ → G) : ℕ := (Finset.univ.image φ).card

theorem coverage_le_card_dom (φ : τ → G) : coverage φ ≤ Fintype.card τ := by
  unfold coverage
  calc (Finset.univ.image φ).card ≤ (Finset.univ : Finset τ).card := Finset.card_image_le
    _ = Fintype.card τ := Finset.card_univ

theorem coverage_le_card_cod (φ : τ → G) : coverage φ ≤ Fintype.card G := by
  unfold coverage
  calc (Finset.univ.image φ).card ≤ (Finset.univ : Finset G).card :=
        Finset.card_le_card (Finset.subset_univ _)
    _ = Fintype.card G := Finset.card_univ

end IcNegative
