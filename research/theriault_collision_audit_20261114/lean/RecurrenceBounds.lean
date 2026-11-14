import Mathlib.Tactic

namespace TheriaultCollisionAudit

/-- The source's explicit solution, without assuming it equals a particular sampling expectation. -/
def sourceClosedForm (n : ℚ) (s : ℕ) : ℚ := n / 2 * (1 - 2 / n)^s + s - n / 2

theorem closed_form_cleared_identity (n : ℚ) :
    (n-2)^3 + 6*n^2 - n^3 = 12*n - 8 := by ring

theorem closed_form_three (n : ℚ) (hn : n ≠ 0) :
    sourceClosedForm n 3 = 6 / n - 4 / n^2 := by
  unfold sourceClosedForm
  field_simp
  ring

theorem closed_form_lower_fails (n : ℚ) (hn : 6 < n) :
    sourceClosedForm n 3 < 2 * (3 : ℚ)^2 / (3 * n) := by
  rw [closed_form_three n (by linarith)]
  have hpos : 0 < 4 / n^2 := div_pos (by norm_num) (sq_pos_of_pos (by linarith))
  have heq : 2 * (3 : ℚ)^2 / (3 * n) = 6 / n := by ring
  rw [heq]
  linarith

/-- A repair conditional on the displayed recurrence. It does not assume an iid bridge. -/
theorem repaired_bounds (n : ℚ) (hn : 2 ≤ n) (e : ℕ → ℚ)
    (hzero : e 0 = 0)
    (hstep : ∀ s, e (s+1) = (1 - 2/n) * e s + 2*s/n) :
    ∀ s : ℕ, (s : ℚ) ≤ n/2 →
      2*(s : ℚ)*((s : ℚ)-1)/(3*n) ≤ e s ∧
      e s ≤ (s : ℚ)*((s : ℚ)-1)/n := by
  have hn0 : 0 < n := by linarith
  have hnne : n ≠ 0 := ne_of_gt hn0
  have ha : 0 ≤ 1 - 2/n := by
    apply sub_nonneg.mpr
    exact (div_le_one hn0).mpr hn
  intro s
  induction s with
  | zero => simp [hzero]
  | succ s ih =>
    intro hs
    have hs' : (s : ℚ) ≤ n/2 := by push_cast at hs; linarith
    obtain ⟨hl, hu⟩ := ih hs'
    rw [hstep]
    push_cast
    constructor
    · have hm := mul_le_mul_of_nonneg_left hl ha
      have hb : 0 ≤ 2*(s : ℚ)*(n-2*s+2)/(3*n^2) := by
        have h : 0 ≤ n-2*s+2 := by linarith
        positivity
      have hid : (1-2/n)*(2*(s : ℚ)*((s : ℚ)-1)/(3*n)) + 2*s/n -
          2*((s : ℚ)+1)*((s : ℚ)+1-1)/(3*n) =
          2*(s : ℚ)*(n-2*s+2)/(3*n^2) := by field_simp; ring
      linarith
    · have hm := mul_le_mul_of_nonneg_left hu ha
      have hb : 0 ≤ 2*(s : ℚ)*((s : ℚ)-1)/n^2 := by
        have hsnon : (s : ℚ)*((s : ℚ)-1) ≥ 0 := by
          cases s with
          | zero => norm_num
          | succ t => push_cast; nlinarith [Nat.cast_nonneg (α := ℚ) t]
        exact div_nonneg (by nlinarith) (sq_nonneg n)
      have hid : ((s : ℚ)+1)*((s : ℚ)+1-1)/n -
          ((1-2/n)*((s : ℚ)*((s : ℚ)-1)/n)+2*s/n) =
          2*(s : ℚ)*((s : ℚ)-1)/n^2 := by field_simp; ring
      linarith

/-- Corrected constant-factor quadratic bounds, conditional on the same recurrence. -/
theorem repaired_quadratic_bounds (n : ℚ) (hn : 2 ≤ n) (e : ℕ → ℚ)
    (hzero : e 0 = 0)
    (hstep : ∀ s, e (s+1) = (1 - 2/n) * e s + 2*s/n)
    (s : ℕ) (hs : 3 ≤ s) (hsmall : (s : ℚ) ≤ n/2) :
    4*(s : ℚ)^2/(9*n) ≤ e s ∧ e s ≤ (s : ℚ)^2/n := by
  obtain ⟨hl, hu⟩ := repaired_bounds n hn e hzero hstep s hsmall
  have hn0 : 0 < n := by linarith
  have hs3 : (3 : ℚ) ≤ s := by exact_mod_cast hs
  constructor
  · apply le_trans _ hl
    apply (div_le_div_iff₀ (by positivity : 0 < 9*n) (by positivity : 0 < 3*n)).mpr
    have h : 0 ≤ (s : ℚ)*((s : ℚ)-3)*n := by positivity
    nlinarith
  · apply le_trans hu
    apply (div_le_div_iff_of_pos_right hn0).mpr
    nlinarith

end TheriaultCollisionAudit
