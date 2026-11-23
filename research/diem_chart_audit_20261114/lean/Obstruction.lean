import Mathlib.LinearAlgebra.Projectivization.Cardinality
import Mathlib.Data.ZMod.Basic
import Mathlib.SetTheory.Cardinal.Finite
import Mathlib.Tactic

/-! Actual projectivization, not an assumed three-element model. -/
namespace DiemChartAudit

abbrev Line (K : Type*) [Field K] := Projectivization K (K × K)

theorem line_card (K : Type*) [Field K] [Finite K] :
    Nat.card (Line K) = Nat.card K + 1 := by
  apply Projectivization.card_of_finrank_two
  simp

theorem binary_line_card : Nat.card (Line (ZMod 2)) = 3 := by
  rw [line_card]
  norm_num

theorem binary_torus_card : Nat.card (ZMod 2)ˣ = 1 := by
  rw [← Fintype.card_eq_nat_card, Fintype.card_units]
  norm_num

/-- Even arbitrary maps from two actual tori cannot cover the actual line. -/
theorem binary_two_maps_not_surjective (f : Fin 2 × (ZMod 2)ˣ → Line (ZMod 2)) :
    ¬ Function.Surjective f := by
  intro h
  have hc := Nat.card_le_card_of_surjective f h
  rw [binary_line_card, Nat.card_prod, binary_torus_card] at hc
  norm_num at hc

theorem binary_two_images_not_cover (f g : (ZMod 2)ˣ → Line (ZMod 2)) :
    ¬ (∀ p, (∃ u, f u = p) ∨ (∃ u, g u = p)) := by
  intro h
  apply binary_two_maps_not_surjective (fun z => if z.1 = 0 then f z.2 else g z.2)
  intro p
  rcases h p with ⟨u, hu⟩ | ⟨u, hu⟩
  · exact ⟨(0,u), by simpa using hu⟩
  · exact ⟨(1,u), by simpa using hu⟩

/-- Counts of genuine product points, for every positive number of factors. -/
theorem binary_product_obstruction (n : ℕ) (hn : 0 < n)
    (f : (Fin n → Fin 2 × (ZMod 2)ˣ) → (Fin n → Line (ZMod 2))) :
    ¬ Function.Surjective f := by
  intro h
  have hc := Nat.card_le_card_of_surjective f h
  simp only [Nat.card_fun, Nat.card_prod, Nat.card_fin, binary_line_card,
    binary_torus_card, mul_one] at hc
  have hp : 2^n < 3^n := Nat.pow_lt_pow_left (by omega) (by omega)
  omega

end DiemChartAudit
