import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

/-! An order-five auxiliary group turns the coefficient-six word into negation. -/

namespace XedniBridge6Aux5

theorem coefficient_six_forces_negation {G : Type*} [AddCommGroup G]
    (P Q : G) (h5 : (5 : ℕ) • Q = 0) (h6 : P + (6 : ℕ) • Q = 0) :
    P = -Q := by
  have h6' : P + ((5 : ℕ) + 1) • Q = 0 := by
    simpa only [show (5 : ℕ) + 1 = 6 by decide] using h6
  have h : P + Q = 0 := by
    simpa only [add_nsmul, one_nsmul, h5, zero_add] using h6'
  exact add_eq_zero_iff_eq_neg.mp h

theorem order_five_violates_F11_hasse_square :
    ¬ (((5 : ℤ) - 12) ^ 2 ≤ 4 * 11) := by
  norm_num

theorem no_order_five_under_F11_hasse_square (card : ℕ)
    (hhasse : ((card : ℤ) - 12) ^ 2 ≤ 4 * 11) : card ≠ 5 := by
  intro hfive
  subst card
  exact order_five_violates_F11_hasse_square hhasse

#print axioms coefficient_six_forces_negation
#print axioms order_five_violates_F11_hasse_square
#print axioms no_order_five_under_F11_hasse_square

end XedniBridge6Aux5
