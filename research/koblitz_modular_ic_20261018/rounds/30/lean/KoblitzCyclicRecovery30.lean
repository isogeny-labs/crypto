import Mathlib.Tactic.Ring
import Mathlib.Tactic.LinearCombination
import Mathlib.Algebra.Ring.Parity

/-! Universal algebra for the R30 polynomial family. No geometric map existence
is assumed or asserted. All identities hold in a commutative ring. -/
namespace KoblitzCyclicRecovery30
variable {A : Type*} [CommRing A]

def alt (x : A) : Nat → A
| 0 => 0
| n+1 => 1 - x * alt x n

theorem alternating_identity (x : A) (n : Nat) :
    (1+x)*alt x n = 1-(-x)^n := by
  induction n with
  | zero => simp [alt]
  | succ n ih =>
    simp only [alt, pow_succ]
    linear_combination -x * ih

theorem odd_cycle_sum (r : A) (n : Nat) (ho : Odd n) (hr : r^n=1) :
    (1+r^3)*alt (r^3) n = 2 := by
  rw [alternating_identity, ho.neg_pow, ← pow_mul, Nat.mul_comm 3 n, pow_mul, hr]
  ring

theorem cm_ramification (w : A) (hw : w^2+w+2=0) :
    (w-3)^2=7*(1-w) := by
  linear_combination hw

theorem coefficient_decomposition (w r : A) :
    (2+w)+7*r+(6+2*w)*r^3 =
      5*(1+r^3)+(w-3)*(1+2*r^3)+7*(r+r^3) := by ring

theorem approximate_inverse (w r s : A) (hs : (1+r^3)*s=2) :
    (12*s)*((2+w)+7*r+(6+2*w)*r^3) =
      1+(w-3)*(12*s)*(1+2*r^3)+7*(17+(12*s)*(r+r^3)) := by
  linear_combination 60 * hs

theorem integral_lift (a m e b h c : A)
    (hm : a*m=1+e*a*b+7*h) (he : e^2=7*c) :
    (a-e*a^2*b)*m+7*(c*a^2*b^2-h*(1-e*a*b))=1 := by
  linear_combination (1-e*a*b)*hm - a^2*b^2*he

theorem odd_cycle_recovery (w r : A) (n : Nat)
    (hw : w^2+w+2=0) (ho : Odd n) (hr : r^n=1) :
    let s := alt (r^3) n
    let a := 12*s
    let e := w-3
    let b := 1+2*r^3
    let h := 17+a*(r+r^3)
    (a-e*a^2*b)*((2+w)+7*r+(6+2*w)*r^3) +
      7*((1-w)*a^2*b^2-h*(1-e*a*b))=1 := by
  exact integral_lift _ _ _ _ _ _
    (approximate_inverse w r _ (odd_cycle_sum r n ho hr)) (cm_ramification w hw)

end KoblitzCyclicRecovery30
