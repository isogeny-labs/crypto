import UniformOccupancy
import RecurrenceBounds
import Mathlib.Logic.Equiv.Prod
import Mathlib.Data.Fintype.BigOperators

namespace TheriaultIidAudit

variable {α β : Type*} [Fintype α] [Fintype β] [DecidableEq α]

omit [DecidableEq α] in
theorem projected_sum (s : ℕ) (f : (Fin s → α) → ℚ) :
    (∑ d : Fin s → α × β, f (fun i => (d i).1)) =
      Fintype.card (Fin s → β) * ∑ d : Fin s → α, f d := by
  let e := Equiv.arrowProdEquivProdArrow (Fin s) (fun _ => α) (fun _ => β)
  calc
    _ = ∑ d : (Fin s → α) × (Fin s → β), f d.1 :=
      e.sum_comp (fun d => f d.1)
    _ = _ := by
      rw [Fintype.sum_prod_type]
      simp [Finset.mul_sum, mul_comm]

def projectedExpectation (α β : Type*) [Fintype α] [Fintype β] [DecidableEq α]
    (s : ℕ) : ℚ :=
  (∑ d : Fin s → α × β, (repeats (fun i => (d i).1) : ℚ)) /
    Fintype.card (Fin s → α × β)

theorem projection_preserves_expectation [Nonempty α] [Nonempty β] (s : ℕ) :
    projectedExpectation α β s = expectedRepeats α s := by
  have hA : (Fintype.card (Fin s → α) : ℚ) ≠ 0 := by exact_mod_cast Fintype.card_ne_zero
  have hB : (Fintype.card (Fin s → β) : ℚ) ≠ 0 := by exact_mod_cast Fintype.card_ne_zero
  unfold projectedExpectation expectedRepeats
  rw [projected_sum s (fun d => (repeats d : ℚ))]
  rw [Fintype.card_congr (Equiv.arrowProdEquivProdArrow (Fin s) (fun _ => α) (fun _ => β))]
  simp only [Fintype.card_prod, Nat.cast_mul]
  field_simp

def pairedNeg (p : α × Bool) : α × Bool := (p.1, !p.2)

omit [Fintype α] [DecidableEq α] in
theorem pairedNeg_involutive : Function.Involutive (pairedNeg : α × Bool → α × Bool) := by
  intro ⟨a, b⟩
  cases b <;> rfl

omit [Fintype α] [DecidableEq α] in
theorem pairedNeg_free : ∀ p : α × Bool, pairedNeg p ≠ p := by
  intro ⟨a, b⟩
  cases b <;> simp [pairedNeg]

omit [Fintype α] [DecidableEq α] in
theorem paired_same_orbit (p q : α × Bool) :
    p.1 = q.1 ↔ p = q ∨ p = pairedNeg q := by
  rcases p with ⟨a, b⟩
  rcases q with ⟨c, d⟩
  cases b <;> cases d <;> simp [pairedNeg]

theorem actual_paired_formula [Nonempty α] (s : ℕ) :
    projectedExpectation α Bool s = (s : ℚ) - Fintype.card α +
      Fintype.card α * (1 - 1 / (Fintype.card α : ℚ))^s := by
  rw [projection_preserves_expectation, expected_repeats_formula]

theorem actual_paired_zero [Nonempty α] : projectedExpectation α Bool 0 = 0 := by
  rw [actual_paired_formula]
  simp

theorem actual_source_closed_form [Nonempty α] (s : ℕ) :
    projectedExpectation α Bool s =
      TheriaultCollisionAudit.sourceClosedForm (Fintype.card (α × Bool) : ℚ) s := by
  rw [actual_paired_formula]
  unfold TheriaultCollisionAudit.sourceClosedForm
  simp only [Fintype.card_prod, Fintype.card_bool, Nat.cast_mul, Nat.cast_ofNat]
  have hid : (1 : ℚ) - 2/((Fintype.card α : ℚ)*2) = 1-1/Fintype.card α := by ring
  rw [hid]
  ring

theorem actual_paired_recurrence [Nonempty α] (s : ℕ) :
    projectedExpectation α Bool (s+1) =
      (1 - 2 / (Fintype.card (α × Bool) : ℚ)) * projectedExpectation α Bool s +
      2 * s / (Fintype.card (α × Bool) : ℚ) := by
  rw [actual_paired_formula, actual_paired_formula]
  simp only [Fintype.card_prod, Fintype.card_bool, Nat.cast_mul, Nat.cast_ofNat,
    Nat.cast_add, Nat.cast_one, pow_succ]
  have hm : (Fintype.card α : ℚ) ≠ 0 := by exact_mod_cast Fintype.card_ne_zero
  field_simp
  ring

theorem actual_paired_repaired_bounds [Nonempty α] (s : ℕ) (hs : 3 ≤ s)
    (hsmall : (s : ℚ) ≤ (Fintype.card (α × Bool) : ℚ)/2) :
    4*(s : ℚ)^2/(9*(Fintype.card (α × Bool) : ℚ)) ≤ projectedExpectation α Bool s ∧
    projectedExpectation α Bool s ≤ (s : ℚ)^2/(Fintype.card (α × Bool) : ℚ) := by
  have hn : (2 : ℚ) ≤ Fintype.card (α × Bool) := by
    simp only [Fintype.card_prod, Fintype.card_bool, Nat.cast_mul, Nat.cast_ofNat]
    have hm : (1 : ℚ) ≤ Fintype.card α := by exact_mod_cast Fintype.card_pos
    linarith
  exact TheriaultCollisionAudit.repaired_quadratic_bounds _ hn
    (projectedExpectation α Bool) actual_paired_zero actual_paired_recurrence s hs hsmall

theorem eight_element_control : projectedExpectation (Fin 4) Bool 3 = 11/16 := by
  rw [actual_paired_formula]
  norm_num

theorem unpaired_negative_control : expectedRepeats (Fin 8) 3 = 23/64 := by
  rw [expected_repeats_formula]
  norm_num

theorem actual_three_lower_fails [Nonempty α] (hm : 3 < Fintype.card α) :
    ¬ (2*(3 : ℚ)^2/(3*(Fintype.card (α × Bool) : ℚ)) ≤ projectedExpectation α Bool 3) := by
  rw [actual_source_closed_form]
  apply not_le_of_gt
  apply TheriaultCollisionAudit.closed_form_lower_fails
  simp only [Fintype.card_prod, Fintype.card_bool, Nat.cast_mul, Nat.cast_ofNat]
  have hm' : (3 : ℚ) < Fintype.card α := by exact_mod_cast hm
  linarith

theorem actual_unbounded_counterfamily (t : ℕ) :
    (3 : ℚ) < (Fintype.card (Fin (t+4) × Bool) : ℚ)/2 ∧
    ¬ (2*(3 : ℚ)^2/(3*(Fintype.card (Fin (t+4) × Bool) : ℚ)) ≤
      projectedExpectation (Fin (t+4)) Bool 3) := by
  constructor
  · simp only [Fintype.card_prod, Fintype.card_fin, Fintype.card_bool, Nat.cast_mul,
      Nat.cast_add, Nat.cast_ofNat]
    have ht : (0 : ℚ) ≤ t := Nat.cast_nonneg t
    linarith
  · exact actual_three_lower_fails (by simp)

theorem pool_sizes_unbounded (b : ℕ) :
    ∃ t : ℕ, b < Fintype.card (Fin (t+4) × Bool) := by
  refine ⟨b, ?_⟩
  simp only [Fintype.card_prod, Fintype.card_fin, Fintype.card_bool]
  omega

end TheriaultIidAudit
