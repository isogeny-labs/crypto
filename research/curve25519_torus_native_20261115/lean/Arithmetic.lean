/- Concrete arithmetic instance obligations only. The geometric theorem is open. -/
set_option autoImplicit false
namespace Curve25519TorusArithmetic
example (P : Prop) [Decidable P] (h : decide P = true) : P := of_decide_eq_true h

def p : Nat := 2^255-19
def M : Nat := 60824497*213156431
def L : Nat := 4*M
def W0 : Nat := 16*L*(L-1)*(L-2)*(L-3)
def K : Nat := 2^28

theorem divisor : (p+1)%M=0 := by decide
theorem admissible_positive : 28*M < p+1 := by decide
theorem correction_lt_one : K*W0 < p+1-28*M := by decide
theorem class_lower_bound : 2^52 < (M-1)/2 := by decide

#print axioms divisor
#print axioms admissible_positive
#print axioms correction_lt_one
#print axioms class_lower_bound
end Curve25519TorusArithmetic

/- Concrete block-size obligations only, not a probability theorem. -/
namespace Curve25519TorusArithmetic
def birthdaySamples : Nat := 268951465398242860562463204936310
theorem birthday_minimum : 2 ≤ birthdaySamples := by decide
theorem birthday_completeness : 1600*birthdaySamples^2 ≤ W0 := by decide
theorem birthday_soundness : W0 ≤ 2304*(birthdaySamples-1)^2 := by decide
#print axioms birthday_minimum
#print axioms birthday_completeness
#print axioms birthday_soundness
end Curve25519TorusArithmetic
