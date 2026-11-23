import Init.Grind.Module.Basic
import Init.Grind.Ring.Basic

/-! Feasible algebra/module consequences of credited CT38/39/40 controls.
Actual abelian varieties,degree,tangent spaces,Tate modules and Frobenius
characteristic-polynomial realization are separate written dependencies.
The older OddKernelProjectors module already supplies generic CRT algebra.
This file adds fixed factor annihilation and structural module consequences.
-/
set_option autoImplicit false
namespace SpectralControls
open Lean.Grind

theorem rational_tangent_shadow {A V W : Type}
    [IntModule A] [IntModule V] [IntModule W]
    (a : A) (v : V) (w : W)
    (ha : (12317 : Int) • a = 0)
    (hv : (101 : Int) • v = 0) (hw : (7 : Int) • w = 0) :
    (1244018 : Int) • a = a ∧ (4976069 : Int) • a = a ∧
    (1244018 : Int) • v = v ∧ (4976069 : Int) • v = v ∧
    (1244018 : Int) • w = -w ∧ (4976069 : Int) • w = 0 := by
  grind

theorem fixed_frobenius_projectors {K : Type} [CommRing K] (x u : K)
    (hp : (x^2+7*x+101)*(x^2+11*x+101) = 0)
    (hu : (404 : K)*u = 1) :
    let a := u*(-(x+7)*(x^2+11*x+101))
    let b := u*((x+11)*(x^2+7*x+101))
    a+b = 1 ∧ a*b = 0 ∧ a^2 = a ∧ b^2 = b ∧
    (x^2+7*x+101)*a = 0 ∧ (x^2+11*x+101)*b = 0 := by
  let A := -(x+7)*(x^2+11*x+101)
  let B := (x+11)*(x^2+7*x+101)
  have hs : A+B = 404 := by dsimp [A, B]; grind
  have hz : A*B = 0 := by
    calc
      A*B = -(x+7)*(x+11)*((x^2+7*x+101)*(x^2+11*x+101)) := by
        dsimp [A, B]
        grind
      _ = 0 := by rw [hp]; grind
  have ha : (x^2+7*x+101)*A = 0 := by
    calc
      _ = -(x+7)*((x^2+7*x+101)*(x^2+11*x+101)) := by dsimp [A]; grind
      _ = 0 := by rw [hp]; grind
  have hb : (x^2+11*x+101)*B = 0 := by
    calc
      _ = (x+11)*((x^2+7*x+101)*(x^2+11*x+101)) := by dsimp [B]; grind
      _ = 0 := by rw [hp]; grind
  change u*A+u*B = 1 ∧ (u*A)*(u*B) = 0 ∧ (u*A)^2 = u*A ∧
    (u*B)^2 = u*B ∧ (x^2+7*x+101)*(u*A) = 0 ∧ (x^2+11*x+101)*(u*B) = 0
  have hsum : u*A+u*B = 1 := by
    calc
      _ = u*(A+B) := by grind
      _ = u*404 := by rw [hs]
      _ = 1 := by grind
  have hzero : (u*A)*(u*B) = 0 := by
    calc
      _ = u^2*(A*B) := by grind
      _ = 0 := by rw [hz]; grind
  grind

theorem unique_projector_decomposition {M : Type} [IntModule M]
    (p q : M → M)
    (pl : ∀ a b, p (a+b) = p a+p b)
    (ql : ∀ a b, q (a+b) = q a+q b)
    (hs : ∀ a, p a+q a = a)
    (hpq : ∀ a, p (q a) = 0) (hqp : ∀ a, q (p a) = 0) (x : M) :
    p x+q x = x ∧ p (p x) = p x ∧ q (q x) = q x ∧
    ∀ a b, p a = a → q b = b → a+b = x → a = p x ∧ b = q x := by
  have hpp := hs (p x)
  have hqq := hs (q x)
  refine ⟨hs x, ?_, ?_, ?_⟩
  · grind
  · grind
  · intro a b ha hb hab
    have hpa := pl a b
    have hqa := ql a b
    have hpzero : p b = 0 := by rw [← hb]; exact hpq b
    have hqzero : q a = 0 := by rw [← ha]; exact hqp a
    grind

theorem polynomial_projector_naturality {M N : Type} [IntModule M] [IntModule N]
    (pi : M → M) (tau : N → N) (f : M → N)
    (fl : ∀ (n : Int) a, f (n • a) = n • f a)
    (fa : ∀ a b, f (a+b) = f a+f b)
    (fc : ∀ a, f (pi a) = tau (f a))
    (u a b c d : Int) (x : M) :
    f (u • (a • pi (pi (pi x))+b • pi (pi x)+c • pi x+d • x)) =
    u • (a • tau (tau (tau (f x)))+b • tau (tau (f x))+c • tau (f x)+d • f x) := by
  simp only [fl, fa, fc]

end SpectralControls
#print axioms SpectralControls.rational_tangent_shadow
#print axioms SpectralControls.fixed_frobenius_projectors
#print axioms SpectralControls.unique_projector_decomposition
#print axioms SpectralControls.polynomial_projector_naturality
