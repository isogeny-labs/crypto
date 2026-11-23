/-! Arithmetic part of the generalized Hess genus calculation for the N=15 panel.

The field-theoretic assertions that particular parameters have these
Frobenius-order degrees and satisfy the descent condition are checked by the
independent executable replay, not by this Lean module.
-/

set_option autoImplicit false

namespace GhsN15

def genus (t s₁ s₂ : Nat) : Int :=
  (2 : Int) ^ t + 1 - (2 : Int) ^ (t - s₁) - (2 : Int) ^ (t - s₂)

theorem genus_four_profile : genus 3 1 3 = 4 := by decide
theorem genus_five_profile : genus 3 2 2 = 5 := by decide
theorem genus_six_profile : genus 3 2 3 = 6 := by decide
theorem genus_seven_profile : genus 3 3 3 = 7 := by decide

/-- Within a relative-degree-three tower, genus five fixes both order
    polynomial degrees at two. -/
theorem genus_five_degree_pattern :
    ∀ t s₁ s₂ : Fin 4,
      1 ≤ t.val → 1 ≤ s₁.val → 1 ≤ s₂.val →
      s₁.val ≤ t.val → s₂.val ≤ t.val →
      genus t.val s₁.val s₂.val = 5 →
      t.val = 3 ∧ s₁.val = 2 ∧ s₂.val = 2 := by decide

section Trace

def traceThree (F : Nat → Nat) (x : Nat) : Nat := x ^^^ F x ^^^ F (F x)

/-- An order polynomial `1+T+T²` annihilating an element makes its
    degree-three relative trace vanish. -/
theorem trace_zero_of_order_seven (F : Nat → Nat) (x : Nat)
    (horder : F (F x) = x ^^^ F x) : traceThree F x = 0 := by
  simp [traceThree, horder]

theorem two_order_seven_profiles_fail_descent (F : Nat → Nat) (c d : Nat)
    (hc : F (F c) = c ^^^ F c)
    (hd : F (F d) = d ^^^ F d) :
    ¬ (traceThree F c ≠ 0 ∨ traceThree F d ≠ 0) := by
  have htc := trace_zero_of_order_seven F c hc
  have htd := trace_zero_of_order_seven F d hd
  simp [htc, htd]

end Trace

#print axioms genus_four_profile
#print axioms genus_five_profile
#print axioms genus_six_profile
#print axioms genus_seven_profile
#print axioms genus_five_degree_pattern
#print axioms trace_zero_of_order_seven
#print axioms two_order_seven_profiles_fail_descent

end GhsN15
