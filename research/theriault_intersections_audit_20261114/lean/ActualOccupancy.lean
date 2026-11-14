module

public import Mathlib.Tactic

@[expose] public section
namespace TheriaultIntersectionsAudit
noncomputable section
open Finset

def occupied {M s : ℕ} (w : Fin s → Fin M) : Finset (Fin M) := univ.image w
def repeatCount {M s : ℕ} (w : Fin s → Fin M) : ℕ := s - (occupied w).card

def mean (M s : ℕ) : ℝ :=
  (∑ w : Fin s → Fin M, (repeatCount w : ℝ)) / (M : ℝ) ^ s

theorem notMem_occupied {M s : ℕ} (w : Fin s → Fin M) (a : Fin M) :
    a ∉ occupied w ↔ ∀ i, w i ≠ a := by simp [occupied]

theorem absent_product {M s : ℕ} (w : Fin s → Fin M) (a : Fin M) :
    (if a ∉ occupied w then (1 : ℝ) else 0) =
      ∏ i : Fin s, if w i = a then (0 : ℝ) else 1 := by
  simp only [notMem_occupied]
  by_cases h : ∀ i, w i ≠ a
  · simp [h]
  · rw [ite_eq_right h]
    push Not at h
    obtain ⟨i, hi⟩ := h
    symm
    exact Finset.prod_eq_zero (mem_univ i) (by simp [hi])

theorem absent_sum (M s : ℕ) (a : Fin M) :
    (∑ w : Fin s → Fin M, if a ∉ occupied w then (1 : ℝ) else 0) =
      ((M : ℝ) - 1) ^ s := by
  simp_rw [absent_product]
  have hsum : (∑ x : Fin M, if x = a then (0 : ℝ) else 1) = (M : ℝ) - 1 := by
    calc
      _ = ∑ x : Fin M, (1 - if x = a then (1 : ℝ) else 0) := by
        apply Finset.sum_congr rfl
        intro x hx
        split_ifs <;> ring
      _ = _ := by rw [sum_sub_distrib]; simp
  calc
    _ = (∑ x : Fin M, if x = a then (0 : ℝ) else 1) ^ s :=
      (Fintype.sum_pow (fun x : Fin M => if x = a then (0 : ℝ) else 1) s).symm
    _ = _ := by rw [hsum]

theorem repeat_cast {M s : ℕ} (w : Fin s → Fin M) :
    (repeatCount w : ℝ) = (s : ℝ) - M +
      ∑ a : Fin M, if a ∉ occupied w then (1 : ℝ) else 0 := by
  have hc : (occupied w).card ≤ s := by
    simpa only [occupied, card_univ, Fintype.card_fin] using card_image_le (s := univ) (f := w)
  have hcard : ((occupied w).card : ℝ) =
      ∑ a : Fin M, if a ∈ occupied w then (1 : ℝ) else 0 := by simp
  have hpart : (∑ a : Fin M, if a ∈ occupied w then (1 : ℝ) else 0) =
      (M : ℝ) - ∑ a : Fin M, if a ∉ occupied w then (1 : ℝ) else 0 := by
    calc
      _ = ∑ a : Fin M, (1 - if a ∉ occupied w then (1 : ℝ) else 0) := by
        apply Finset.sum_congr rfl
        intro a ha
        by_cases h : a ∈ occupied w <;> simp [h]
      _ = _ := by rw [sum_sub_distrib]; simp
  unfold repeatCount
  rw [Nat.cast_sub hc, hcard, hpart]
  ring

theorem repeat_sum (M s : ℕ) :
    (∑ w : Fin s → Fin M, (repeatCount w : ℝ)) =
      (M : ℝ) ^ s * ((s : ℝ) - M) + M * ((M : ℝ) - 1) ^ s := by
  simp_rw [repeat_cast]
  rw [sum_add_distrib, Finset.sum_comm]
  simp only [absent_sum, sum_const, card_univ, Fintype.card_fun, Fintype.card_fin,
    nsmul_eq_mul, Nat.cast_pow]

theorem mean_closed (M s : ℕ) (hM : 0 < M) :
    mean M s = (s : ℝ) - M + M * (1 - (M : ℝ)⁻¹) ^ s := by
  have hm : (M : ℝ) ≠ 0 := by exact_mod_cast Nat.ne_of_gt hM
  unfold mean
  rw [repeat_sum]
  have hr : 1 - (M : ℝ)⁻¹ = ((M : ℝ) - 1) / M := by field_simp
  rw [hr, div_pow]
  field_simp

theorem mean_recurrence (M s : ℕ) (hM : 0 < M) :
    mean M (s + 1) = (1 - (M : ℝ)⁻¹) * mean M s + (s : ℝ) / M := by
  have hm : (M : ℝ) ≠ 0 := by exact_mod_cast Nat.ne_of_gt hM
  rw [mean_closed M (s + 1) hM, mean_closed M s hM, pow_succ]
  push_cast
  field_simp
  ring

#print axioms repeat_sum
#print axioms mean_closed
#print axioms mean_recurrence
end
end TheriaultIntersectionsAudit
