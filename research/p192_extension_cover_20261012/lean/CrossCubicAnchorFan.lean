import Mathlib.Tactic.Abel
import Mathlib.Data.Finset.Card
import Mathlib.Data.Fintype.Card

/-!
A split line from one fixed anchor root to roots in other query cubics
places the anchor in each complete partner-cubic bundle. The line and
query-cubic equations preserve the known-scalar bundle identity.
-/

namespace P192ExtensionCover

variable {G : Type*} [AddCommGroup G]

/-- `B` is the root in a partner cubic and `A` is the fixed anchor root.
The first cut at endpoint `R` comes from `A+B+R+T=S`, hence its two
residuals are `A,T`. The remaining partner-cubic roots `U,V` have the
displayed cuts. The six projected appearances retain the usual scalar
equation and include `A` as a column. -/
theorem cross_cubic_anchor_bundle
    (A B U V R T Y Z H J S C q : G) (k : ℕ)
    (hline : A + B + R + T = S)
    (hcutU : U + R + Y + Z = S)
    (hcutV : V + R + H + J = S)
    (hcubic : B + U + V + k • q = C) :
    (A + T + Y + Z + H + J) + 3 • R + C = 3 • S + k • q := by
  have hb : B = S - R - A - T := by
    calc
      B = (A + B + R + T) - R - A - T := by abel
      _ = S - R - A - T := by rw [hline]
  have hu : U = S - R - Y - Z := by
    calc
      U = (U + R + Y + Z) - R - Y - Z := by abel
      _ = S - R - Y - Z := by rw [hcutU]
  have hv : V = S - R - H - J := by
    calc
      V = (V + R + H + J) - R - H - J := by abel
      _ = S - R - H - J := by rw [hcutV]
  calc
    (A + T + Y + Z + H + J) + 3 • R + C =
        (A + T + Y + Z + H + J) + 3 • R +
          (B + U + V + k • q) := by rw [hcubic]
    _ = 3 • S + k • q := by rw [hb, hu, hv]; abel

#print axioms cross_cubic_anchor_bundle

/-- Eight distinct active bundle keys containing one anchor column give
that column bundle degree at least eight. The concrete P-192 membership
and distinctness witnesses are supplied by the frozen coordinate replay. -/
theorem cross_cubic_anchor_eight_bundle_degree
    {ι κ : Type*} [DecidableEq ι] [DecidableEq κ]
    (I : Finset ι) (support : ι → Finset κ) (anchor : κ)
    (key : Fin 8 → ι) (hinj : Function.Injective key)
    (hmember : ∀ j, key j ∈ I ∧ anchor ∈ support (key j)) :
    8 ≤ (I.filter fun i => anchor ∈ support i).card := by
  let good := I.filter fun i => anchor ∈ support i
  have hsubset : (Finset.univ.image key) ⊆ good := by
    intro i hi
    rcases Finset.mem_image.mp hi with ⟨j, _, rfl⟩
    exact Finset.mem_filter.mpr (hmember j)
  have hcard : (Finset.univ.image key).card = 8 := by
    rw [Finset.card_image_of_injective _ hinj]
    simp
  simpa [good] using hcard ▸ Finset.card_le_card hsubset

#print axioms cross_cubic_anchor_eight_bundle_degree

end P192ExtensionCover
