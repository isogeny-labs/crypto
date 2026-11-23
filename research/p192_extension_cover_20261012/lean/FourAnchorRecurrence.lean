import CrossCubicAnchorFan
import Mathlib.Combinatorics.Enumerative.DoubleCounting

/-!
Four distinct projected anchors with eight distinct complete-bundle keys
each give four distinct columns of bundle degree at least eight. The
coordinate replay supplies the concrete anchor, key, and membership data.
-/

namespace P192ExtensionCover

open Finset

/-- A four-anchor family has at least four high-degree columns when each
anchor occurs in eight distinct active bundles and the anchor columns are
pairwise distinct. The hypotheses keep concrete P-192 coordinate and
quotient-image obligations explicit. -/
theorem four_anchor_eightfold_column_count
    {ι κ : Type*} [DecidableEq ι] [DecidableEq κ]
    (I : Finset ι) (support : ι → Finset κ)
    (anchor : Fin 4 → κ) (keys : Fin 4 → Fin 8 → ι)
    (hanchor : Function.Injective anchor)
    (hkeys : ∀ a, Function.Injective (keys a))
    (hmember : ∀ a j, keys a j ∈ I ∧ anchor a ∈ support (keys a j)) :
    4 ≤ ((I.biUnion support).filter fun x =>
      8 ≤ (I.filter fun i => x ∈ support i).card).card := by
  let high := (I.biUnion support).filter fun x =>
    8 ≤ (I.filter fun i => x ∈ support i).card
  have hsubset : (Finset.univ.image anchor) ⊆ high := by
    intro x hx
    rcases Finset.mem_image.mp hx with ⟨a, _, rfl⟩
    apply Finset.mem_filter.mpr
    constructor
    · exact Finset.mem_biUnion.mpr
        ⟨keys a 0, (hmember a 0).1, (hmember a 0).2⟩
    · exact cross_cubic_anchor_eight_bundle_degree I support
        (anchor a) (keys a) (hkeys a) (hmember a)
  have hcard : (Finset.univ.image anchor).card = 4 := by
    rw [Finset.card_image_of_injective _ hanchor]
    simp
  simpa [high] using hcard ▸ Finset.card_le_card hsubset

#print axioms four_anchor_eightfold_column_count

end P192ExtensionCover
