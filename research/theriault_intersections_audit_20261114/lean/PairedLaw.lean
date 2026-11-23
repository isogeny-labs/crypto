import ActualOccupancy
import WalkBoundary

namespace TheriaultIntersectionsAudit
noncomputable section
open Finset WalkBoundaryAudit

abbrev PairedPoint (M : ℕ) := Fin M × Bool
abbrev PairedWord (M s : ℕ) := Fin s → PairedPoint M

def negatePoint {M : ℕ} (a : PairedPoint M) : PairedPoint M := (a.1, !a.2)

theorem negatePoint_involutive (M : ℕ) : Function.Involutive (@negatePoint M) := by
  intro a
  simp [negatePoint]

theorem negatePoint_no_fixed {M : ℕ} (a : PairedPoint M) : negatePoint a ≠ a := by
  rcases a with ⟨a, b⟩
  cases b <;> simp [negatePoint]

theorem same_orbit {M : ℕ} (a b : PairedPoint M) :
    a.1 = b.1 ↔ b = a ∨ b = negatePoint a := by
  rcases a with ⟨a, x⟩
  rcases b with ⟨b, y⟩
  cases x <;> cases y <;> simp [negatePoint, eq_comm]

def pairedCount {M s : ℕ} (w : PairedWord M s) : ℕ := repeatCount (fun i => (w i).1)

def pairedWordEquiv (M s : ℕ) :
    PairedWord M s ≃ (Fin s → Fin M) × (Fin s → Bool) where
  toFun w := (fun i => (w i).1, fun i => (w i).2)
  invFun z i := (z.1 i, z.2 i)
  left_inv w := by ext i <;> rfl
  right_inv z := by cases z; rfl

def pairedMean (M s : ℕ) [NeZero M] : ℝ :=
  ∑ w : PairedWord M s, (uniformPMF (PairedWord M s) w).toReal * (pairedCount w : ℝ)

theorem word_mass_product (M s : ℕ) [NeZero M] (w : PairedWord M s) :
    uniformPMF (PairedWord M s) w =
      ∏ i : Fin s, uniformPMF (PairedPoint M) (w i) := by
  simp [uniformPMF_apply, PairedWord, PairedPoint, ENNReal.inv_pow]

theorem word_mass_real (M s : ℕ) [NeZero M] (w : PairedWord M s) :
    (uniformPMF (PairedWord M s) w).toReal = 1 / (2 * (M : ℝ)) ^ s := by
  rw [uniformPMF_apply, ENNReal.toReal_inv, ENNReal.toReal_natCast]
  simp only [Fintype.card_fun, Fintype.card_prod, Fintype.card_fin, Fintype.card_bool,
    Nat.cast_pow, Nat.cast_mul, Nat.cast_ofNat, one_div, mul_comm]

theorem paired_count_sum (M s : ℕ) :
    (∑ w : PairedWord M s, (pairedCount w : ℝ)) =
      (2 : ℝ) ^ s * ∑ w : Fin s → Fin M, (repeatCount w : ℝ) := by
  calc
    _ = ∑ z : (Fin s → Fin M) × (Fin s → Bool), (repeatCount z.1 : ℝ) :=
      (pairedWordEquiv M s).sum_comp (fun z => (repeatCount z.1 : ℝ))
    _ = _ := by
      rw [Fintype.sum_prod_type]
      simp only [sum_const, card_univ, Fintype.card_fun, Fintype.card_bool,
        Fintype.card_fin, nsmul_eq_mul, Nat.cast_pow, Nat.cast_ofNat]
      rw [← Finset.mul_sum]

theorem paired_mean_eq (M s : ℕ) [NeZero M] : pairedMean M s = mean M s := by
  have htwo : (2 : ℝ) ^ s ≠ 0 := pow_ne_zero _ (by norm_num)
  calc
    _ = (∑ w : PairedWord M s, (pairedCount w : ℝ)) / (2 * (M : ℝ)) ^ s := by
      simp only [pairedMean, word_mass_real, div_eq_mul_inv, one_mul]
      rw [Finset.sum_mul]
      apply Finset.sum_congr rfl
      intro w hw
      ring
    _ = _ := by rw [paired_count_sum, mul_pow, mean]; field_simp

theorem paired_mean_closed (M s : ℕ) [NeZero M] :
    pairedMean M s = (s : ℝ) - M + M * (1 - (M : ℝ)⁻¹) ^ s := by
  rw [paired_mean_eq, mean_closed M s (Nat.pos_of_ne_zero (NeZero.ne M))]

theorem paired_mean_recurrence (M s : ℕ) [NeZero M] :
    pairedMean M (s + 1) =
      (1 - (M : ℝ)⁻¹) * pairedMean M s + (s : ℝ) / M := by
  simp only [paired_mean_eq]
  exact mean_recurrence M s (Nat.pos_of_ne_zero (NeZero.ne M))

#print axioms word_mass_product
#print axioms paired_mean_eq
#print axioms paired_mean_closed
end
end TheriaultIntersectionsAudit
