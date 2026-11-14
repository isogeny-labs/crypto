module

public import UniformEvents
public import Mathlib.Data.Int.Interval
public import Mathlib.Data.Finset.Max

public section

namespace InjectiveBoxAudit

theorem integer_set_card_bound (S : Finset ℤ) (L U : ℝ) (hLU : L ≤ U)
    (h : ∀ z ∈ S, L ≤ (z : ℝ) ∧ (z : ℝ) ≤ U) : (S.card : ℝ) ≤ U-L+1 := by
  by_cases hs : S.Nonempty
  · let a := S.min' hs
    let b := S.max' hs
    have hab : a ≤ b := S.min'_le_max' hs
    have hsub : S ⊆ Finset.Icc a b := by
      intro z hz
      exact Finset.mem_Icc.mpr ⟨S.min'_le z hz,S.le_max' z hz⟩
    have hc := Finset.card_le_card hsub
    have hc' : (S.card : ℤ) ≤ (Finset.Icc a b).card := by exact_mod_cast hc
    rw [Int.card_Icc_of_le (a := a) (b := b) (by omega)] at hc'
    have hR : (S.card : ℝ) ≤ (b : ℝ)+1-(a : ℝ) := by exact_mod_cast hc'
    have hL := (h a (S.min'_mem hs)).1
    have hU := (h b (S.max'_mem hs)).2
    linarith
  · have he : S=∅ := Finset.not_nonempty_iff_eq_empty.mp hs
    subst S
    simp only [Finset.card_empty,Nat.cast_zero]
    linarith

theorem integer_image_card_bound {X : Type*} (S : Finset X) (f : X → ℤ)
    (hf : Function.Injective f) (L U : ℝ) (hLU : L ≤ U)
    (h : ∀ x ∈ S, L ≤ (f x : ℝ) ∧ (f x : ℝ) ≤ U) : (S.card : ℝ) ≤ U-L+1 := by
  have hb := integer_set_card_bound (S.image f) L U hLU (by
    intro z hz
    obtain ⟨x,hx,rfl⟩ := Finset.mem_image.mp hz
    exact h x hx)
  simpa only [Finset.card_image_of_injective S hf] using hb

end InjectiveBoxAudit
