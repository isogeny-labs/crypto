import Mathlib.Data.Fintype.Pi
import Mathlib.Data.Fintype.Card
import Mathlib.Algebra.BigOperators.Ring.Finset
import Mathlib.Tactic

namespace TheriaultIidAudit

variable {α : Type*} [Fintype α] [DecidableEq α]

def support {s : ℕ} (d : Fin s → α) : Finset α := Finset.univ.image d

def repeats {s : ℕ} (d : Fin s → α) : ℕ := s - (support d).card

/-- The actual uniform average over all ordered draws with replacement. -/
def expectedRepeats (α : Type*) [Fintype α] [DecidableEq α] (s : ℕ) : ℚ :=
  (∑ d : Fin s → α, (repeats d : ℚ)) / Fintype.card (Fin s → α)

omit [Fintype α] in
theorem support_card_le {s : ℕ} (d : Fin s → α) : (support d).card ≤ s := by
  simpa [support] using (Finset.card_image_le (s := Finset.univ) (f := d))

omit [Fintype α] in
theorem repeats_cast {s : ℕ} (d : Fin s → α) :
    (repeats d : ℚ) = s - ((support d).card : ℚ) := by
  exact Nat.cast_sub (support_card_le d)

theorem avoiding_card (s : ℕ) (a : α) :
    Fintype.card {d : Fin s → α // ∀ i, d i ≠ a} = (Fintype.card α - 1)^s := by
  rw [Fintype.card_congr (Equiv.subtypePiEquivPi (p := fun _ x => x ≠ a))]
  simp [Fintype.card_subtype_compl]

theorem missing_sum (s : ℕ) (a : α) :
    (∑ d : Fin s → α, if ∀ i, d i ≠ a then (1 : ℚ) else 0) =
      ((Fintype.card α - 1)^s : ℕ) := by
  rw [Finset.sum_boole, ← Fintype.card_subtype]
  exact_mod_cast avoiding_card s a

theorem support_card_as_sum {s : ℕ} (d : Fin s → α) :
    ((support d).card : ℚ) =
      ∑ a : α, (1 - if ∀ i, d i ≠ a then (1 : ℚ) else 0) := by
  calc
    ((support d).card : ℚ) = ∑ a : α, if a ∈ support d then (1 : ℚ) else 0 := by simp
    _ = _ := by
      apply Finset.sum_congr rfl
      intro a _
      by_cases h : ∀ i, d i ≠ a
      · have ha : a ∉ support d := by simpa [support] using h
        simp [h, ha]
      · have ha : a ∈ support d := by simpa [support] using h
        simp [h, ha]

theorem total_support (s : ℕ) :
    (∑ d : Fin s → α, ((support d).card : ℚ)) =
      Fintype.card α * ((Fintype.card α : ℚ)^s - ((Fintype.card α - 1)^s : ℕ)) := by
  simp_rw [support_card_as_sum]
  rw [Finset.sum_comm]
  simp_rw [Finset.sum_sub_distrib, missing_sum]
  simp [mul_sub]

theorem total_repeats (s : ℕ) :
    (∑ d : Fin s → α, (repeats d : ℚ)) =
      (s : ℚ) * (Fintype.card α : ℚ)^s -
        Fintype.card α * ((Fintype.card α : ℚ)^s - ((Fintype.card α - 1)^s : ℕ)) := by
  simp_rw [repeats_cast]
  rw [Finset.sum_sub_distrib, total_support]
  simp [mul_comm]

theorem expected_repeats_formula [Nonempty α] (s : ℕ) :
    expectedRepeats α s = (s : ℚ) - Fintype.card α +
      Fintype.card α * (1 - 1 / (Fintype.card α : ℚ))^s := by
  have hm : 0 < Fintype.card α := Fintype.card_pos
  have hmr : (Fintype.card α : ℚ) ≠ 0 := by exact_mod_cast ne_of_gt hm
  have hm1 : (1 : ℕ) ≤ Fintype.card α := hm
  unfold expectedRepeats
  rw [total_repeats]
  simp only [Fintype.card_fun, Fintype.card_fin, Nat.cast_pow]
  rw [Nat.cast_sub hm1]
  push_cast
  have hid : 1 - 1 / (Fintype.card α : ℚ) =
      ((Fintype.card α : ℚ)-1) / Fintype.card α := by field_simp
  rw [hid, div_pow]
  field_simp
  ring

end TheriaultIidAudit
