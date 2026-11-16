import Init.Grind.Module.Basic
import Init.Grind.Ring.Basic

/-! Sign descent through an odd iterate. The intended module is a Hom group,
NOT a geometric point group. Its lack of 2-torsion, the Galois action and
actual finite-field/coefficient hypotheses remain explicit written bridges. -/
set_option autoImplicit false
namespace OddFieldDescent
open Lean.Grind

def iterate {A : Type} (f : A → A) : Nat → A → A
  | 0, a => a
  | n+1, a => f (iterate f n a)

theorem iterate_add {A : Type} (f : A → A) (m n : Nat) (a : A) :
    iterate f (m+n) a = iterate f m (iterate f n a) := by
  induction m with
  | zero => simp [iterate]
  | succ m ih => simp [Nat.succ_add,iterate,ih]

theorem iterate_even_fixed {A : Type} (f : A → A) (a : A)
    (htwo : iterate f 2 a=a) (n : Nat) : iterate f (2*n) a=a := by
  induction n with
  | zero => rfl
  | succ n ih => rw [Nat.mul_succ,iterate_add,htwo,ih]

theorem iterate_odd_negative {H : Type} [IntModule H]
    (f : H → H) (a : H) (hneg : ∀ x, f (-x) = -f x)
    (ha : f a = -a) (n : Nat) : iterate f (2*n+1) a = -a := by
  have htwo : iterate f 2 a=a := by
    change f (f a)=a
    rw [ha,hneg,ha]
    grind
  change f (iterate f (2*n) a) = -a
  rw [iterate_even_fixed f a htwo n,ha]

theorem odd_sign_descent {H : Type} [IntModule H]
    (f : H → H) (a : H) (hneg : ∀ x, f (-x) = -f x)
    (htorsion : ∀ x : H, (2 : Int) • x=0 → x=0)
    (hne : a ≠ 0) (choices : f a=a ∨ f a = -a)
    (n : Nat) (hfixed : iterate f (2*n+1) a=a) : f a=a := by
  rcases choices with same | negative
  · exact same
  · have hodd := iterate_odd_negative f a hneg negative n
    have heq : -a=a := hodd.symm.trans hfixed
    have hz : (2 : Int) • a=0 := by grind
    exact False.elim (hne (htorsion a hz))

theorem cubic_sign_descent {H : Type} [IntModule H]
    (f : H → H) (a : H) (hneg : ∀ x, f (-x) = -f x)
    (htorsion : ∀ x : H, (2 : Int) • x=0 → x=0)
    (hne : a ≠ 0) (choices : f a=a ∨ f a = -a)
    (hfixed : f (f (f a))=a) : f a=a := by
  exact odd_sign_descent f a hneg htorsion hne choices 1 hfixed

theorem quadratic_twist_curve {K : Type} [CommRing K]
    (a b s d x y : K) (hs : s^2=d) (hy : y^2=x^3+a*x+b) :
    (s^3*y)^2=(d*x)^3+(d^2*a)*(d*x)+d^3*b := by
  grind

end OddFieldDescent
#print axioms OddFieldDescent.iterate_add
#print axioms OddFieldDescent.iterate_even_fixed
#print axioms OddFieldDescent.iterate_odd_negative
#print axioms OddFieldDescent.odd_sign_descent
#print axioms OddFieldDescent.cubic_sign_descent
#print axioms OddFieldDescent.quadratic_twist_curve
