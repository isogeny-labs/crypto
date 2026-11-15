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

#print axioms genus_four_profile
#print axioms genus_five_profile
#print axioms genus_six_profile
#print axioms genus_seven_profile

end GhsN15
