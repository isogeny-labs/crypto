import Init.Grind.Ring.Basic
import Init.Grind.Module.Basic

/-! Cleared finite-algebra identities and conditional norm/recovery identities.
Actual F101 coefficients, curves, jets, Picard maps, polarization and Galois
descent remain the explicitly written/native bridges in the frozen protocol.
No finite-domain function interpolation is used as a polynomial proof. -/
set_option autoImplicit false
namespace T20DirectNorms
open Lean.Grind

theorem cubic_fibre_ordinate_remainder {K : Type} [CommRing K]
    (x a b c0 c1 c2 c3 : K) :
    c0+c1*x+c2*x^2+c3*x^3 -
      ((c0-b*c2+a*b*c3)+(c1-a*c2+(a^2-b)*c3)*x) =
      (c2+c3*(x-a))*(x^2+a*x+b) := by grind

theorem linear_image_sum {K : Type} [CommRing K]
    (z1 z2 a A B : K) (hs : z1+z2 = -a) :
    (A+B*z1)+(A+B*z2)=2*A-a*B := by grind

theorem linear_image_ordinate {K : Type} [CommRing K]
    (z A B C D r m : K) (hm : m*B=D) :
    m*(A+B*z-r)-(C+D*z)=m*(A-r)-C := by grind

theorem equal_x_opposite_ordinates {K : Type} [CommRing K]
    (z1 z2 a C D : K) (hs : z1+z2 = -a)
    (hc : 2*C-a*D=0) :
    (C+D*z1)+(C+D*z2)=0 := by grind

theorem two_torsion_translation_x {K : Type} [CommRing K]
    (x e A B : K) (he : e^3+A*e+B=0) :
    x^3+A*x+B-(x+e)*(x-e)^2 =
      (e*(x-e)+(3*e^2+A))*(x-e) := by grind

theorem two_torsion_translation_y {K : Type} [CommRing K]
    (x e r k y : K) (hr : (r-e)*(x-e)=k) :
    y*(e-r)*(x-e) = -k*y := by grind

theorem coherent_translation_cancels {M : Type} [IntModule M]
    (x y c : M) : (x-c)-(y-c)=x-y := by grind

theorem four_norm_compositions
    {C J A : Type} [IntModule C] [IntModule J] [IntModule A]
    (r : C → J) (rh : J → C) (n : C → A) (p : A → C)
    (pn : ∀ x, p (n x)=(2 : Int) • x)
    (rrh : ∀ x, r (rh x)=(2 : Int) • x)
    (rTwo : ∀ x, r ((2 : Int) • x)=(2 : Int) • r x)
    (np : ∀ x, n (p x)=(2 : Int) • x)
    (rhr : ∀ x, rh (r x)=(2 : Int) • x)
    (nTwo : ∀ x, n ((2 : Int) • x)=(2 : Int) • n x) :
    (∀ x, r (p (n (rh x)))=(4 : Int) • x) ∧
    (∀ x, n (rh (r (p x)))=(4 : Int) • x) := by
  constructor
  · intro x
    rw [pn, rTwo, rrh]
    grind
  · intro x
    rw [rhr, nTwo, np]
    grind

theorem rational_four_recovery {M : Type} [IntModule M]
    (x : M) (order : (12317 : Int) • x=0) :
    (9238 : Int) • ((4 : Int) • x)=x := by grind

theorem actual_scalar_and_normalization_constants :
    (4 : Nat)*9238%12317=1 ∧ (4 : Nat)*82%109=1 ∧
    (4 : Nat)*85%113=1 ∧ (196 : Nat)*104%109=1 ∧
    (50 : Nat)^2%101=76 ∧ (50 : Nat)*99%101=1 ∧
    (76 : Nat)*4%101=1 ∧
    ([25,3,69,63,52,62,76] : List Nat).map (fun c => c*4%101)=
      [100,12,74,50,6,46,1] := by decide

end T20DirectNorms
#print axioms T20DirectNorms.cubic_fibre_ordinate_remainder
#print axioms T20DirectNorms.linear_image_sum
#print axioms T20DirectNorms.linear_image_ordinate
#print axioms T20DirectNorms.equal_x_opposite_ordinates
#print axioms T20DirectNorms.two_torsion_translation_x
#print axioms T20DirectNorms.two_torsion_translation_y
#print axioms T20DirectNorms.coherent_translation_cancels
#print axioms T20DirectNorms.four_norm_compositions
#print axioms T20DirectNorms.rational_four_recovery
#print axioms T20DirectNorms.actual_scalar_and_normalization_constants
