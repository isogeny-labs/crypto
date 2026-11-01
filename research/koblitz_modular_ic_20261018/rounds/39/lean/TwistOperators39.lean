import Mathlib.Tactic.Ring
import Mathlib.GroupTheory.OrderOfElement

/-! Universal identities used by R39. These statements do not import the native
source arrays or identify a geometric CM component with a chosen Koblitz model. -/
namespace TwistOperators39
section RingIdentities
variable {A : Type*} [CommRing A]

theorem negative_cm_root (a u : A) :
    (-a*u)^2 + (-a*u)*u + 2*u^2 = (a^2-a+2)*u^2 := by ring

theorem conjugate_negative_cm_root (a u : A) :
    (-(1-a)*u)^2 + (-(1-a)*u)*u + 2*u^2 = (a^2-a+2)*u^2 := by ring

theorem annihilate_negative_cm_root (a u : A) (h : a^2-a+2=0) :
    (-a*u)^2 + (-a*u)*u + 2*u^2 = 0 := by
  rw [negative_cm_root, h, zero_mul]

theorem annihilate_conjugate_negative_cm_root (a u : A) (h : a^2-a+2=0) :
    (-(1-a)*u)^2 + (-(1-a)*u)*u + 2*u^2 = 0 := by
  rw [conjugate_negative_cm_root, h, zero_mul]

theorem twist_fixed_equivalence (d f : A) (h : d^5 = -1) :
    d^5*f=1 ↔ f = -1 := by
  rw [h, neg_one_mul]
  constructor
  · intro hf
    have hn := congrArg (fun z : A => -z) hf
    simpa using hn
  · intro hf
    rw [hf, neg_neg]

end RingIdentities

section FiniteRecovery
variable {G : Type*} [AddCommGroup G]

theorem recover_nine (x : G) (h : 11 • x = 0) : 5 • (9 • x) = x := by
  rw [← mul_nsmul]
  have hn : 9*5 = 1+11*4 := by decide
  rw [hn, add_nsmul, one_nsmul, mul_nsmul, h, nsmul_zero, add_zero]

theorem nine_nonzero (x : G) (h : 11 • x = 0) (hx : x ≠ 0) :
    9 • x ≠ 0 := by
  intro hz
  have hr := recover_nine x h
  rw [hz, nsmul_zero] at hr
  exact hx hr.symm

theorem recover_operator_value (x y : G) (h : 11 • x = 0)
    (hy : y = 9 • x) : 5 • y = x := by
  rw [hy]
  exact recover_nine x h

end FiniteRecovery
end TwistOperators39
