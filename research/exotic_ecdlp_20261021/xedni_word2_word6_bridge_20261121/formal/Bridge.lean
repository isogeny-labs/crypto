import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

/-! Complete coefficient-two and coefficient-six words generate the nonzero scalar group. -/

namespace XedniWord2Word6Bridge

set_option maxRecDepth 20000
set_option maxHeartbeats 0

abbrev K := ZMod 43

theorem multiplier_two_order : (-2 : K)^7 = 1 := by decide

theorem multiplier_six_order : (-6 : K)^6 = 1 := by decide

theorem combined_multipliers_cover :
    ∀ x : K, x ≠ 0 →
      ∃ i : Fin 7, ∃ j : Fin 6,
        x = (-2 : K)^i.val * (-6 : K)^j.val := by decide

def wordConsistent (m : K) (f : K → K) : Prop :=
  ∀ a b : K, a + m*b = 0 → f a + m*f b = 0

theorem word_step (m : K) (f : K → K) (h : wordConsistent m f) (b : K) :
    f ((-m)*b) = (-m)*f b := by
  have hr : (-m)*b + m*b = 0 := by ring
  have hf := h ((-m)*b) b hr
  linear_combination hf

theorem word_power (m : K) (f : K → K) (h : wordConsistent m f)
    (n : ℕ) (b : K) :
    f ((-m)^n*b) = (-m)^n*f b := by
  induction n with
  | zero => simp
  | succ n ih =>
      calc
        f ((-m)^(n+1)*b) = f ((-m)*((-m)^n*b)) := by rw [pow_succ]; ring
        _ = (-m)*f ((-m)^n*b) := word_step m f h _
        _ = (-m)^(n+1)*f b := by rw [ih, pow_succ]; ring

theorem complete_words_determine_nonzero_logs
    (f : K → K)
    (h2 : wordConsistent 2 f)
    (h6 : wordConsistent 6 f)
    (hanchor : f 1 = 1) :
    ∀ x : K, x ≠ 0 → f x = x := by
  intro x hx
  obtain ⟨i, j, hrepr⟩ := combined_multipliers_cover x hx
  rw [hrepr]
  calc
    f ((-2 : K)^i.val * (-6 : K)^j.val) =
        (-2 : K)^i.val * f ((-6 : K)^j.val) :=
      word_power 2 f h2 i.val _
    _ = (-2 : K)^i.val * ((-6 : K)^j.val * f 1) := by
      rw [show f ((-6 : K)^j.val) = (-6 : K)^j.val * f 1 by
        simpa using word_power 6 f h6 j.val 1]
    _ = (-2 : K)^i.val * (-6 : K)^j.val := by rw [hanchor]; ring

#print axioms multiplier_two_order
#print axioms multiplier_six_order
#print axioms combined_multipliers_cover
#print axioms word_step
#print axioms word_power
#print axioms complete_words_determine_nonzero_logs

end XedniWord2Word6Bridge
