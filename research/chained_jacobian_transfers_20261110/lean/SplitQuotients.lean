import Init.Grind.Module.Basic
import Init.Grind.Ring.Basic

/-! Cleared reciprocal coordinates and conditional split-map identities.
No finite fields,divisors,Picard maps or group orders are constructed. -/
set_option autoImplicit false
namespace SplitQuotients
open Lean.Grind

theorem reciprocal_curve_cleared {K : Type} [CommRing K]
    (x t y z s b a₂ a₄ : K) (hs : s^2 = b) (ht : x*t = s)
    (hz : z*x^3 = b*y) (hy : y^2 = x^6+a₂*x^4+a₄*x^2+b) :
    x^6*(z^2-(t^6+a₄*t^4+a₂*b*t^2+b^2)) = 0 := by
  have h₂ : x^2*t^2 = b := by
    calc
      x^2*t^2 = (x*t)^2 := by clear hs ht hz hy; grind
      _ = s^2 := congrArg (fun q : K => q^2) ht
      _ = b := hs
  have h₄ : x^4*t^4 = b^2 := by
    calc
      x^4*t^4 = (x^2*t^2)^2 := by clear hs ht hz hy h₂; grind
      _ = b^2 := congrArg (fun q : K => q^2) h₂
  have h₆ : x^6*t^6 = b^3 := by
    calc
      x^6*t^6 = (x^2*t^2)^3 := by clear hs ht hz hy h₂ h₄; grind
      _ = b^3 := congrArg (fun q : K => q^3) h₂
  have hzx : z^2*x^6 = b^2*y^2 := by
    calc
      z^2*x^6 = (z*x^3)^2 := by clear hs ht hz hy h₂ h₄ h₆; grind
      _ = (b*y)^2 := congrArg (fun q : K => q^2) hz
      _ = b^2*y^2 := by clear hs ht hz hy h₂ h₄ h₆; grind
  have expand : x^6*(z^2-(t^6+a₄*t^4+a₂*b*t^2+b^2)) =
      z^2*x^6 - x^6*t^6 - a₄*x^2*(x^4*t^4) - a₂*b*x^4*(x^2*t^2) - b^2*x^6 := by
    clear hs ht hz hy h₂ h₄ h₆ hzx
    grind
  rw [expand, hzx, h₆, h₄, h₂, hy]
  clear hs ht hz hy h₂ h₄ h₆ hzx expand
  grind

theorem reciprocal_inverse_cleared {K : Type} [CommRing K]
    (x t y z s b : K) (hs : s^2 = b) (ht : x*t = s)
    (hz : z*x^3 = b*y) : b*(y*t^3-s*z) = 0 := by grind

theorem complementary_norms {A : Type} [IntModule A] (d s : A) :
    (d+s)+(d-s) = (2 : Int) • d := by grind

theorem odd_halving {A : Type} [IntModule A] (d : A) (n k : Int)
    (hn : n • d = 0) (hk : 2*k = n+1) : k • ((2 : Int) • d) = d := by
  have h : k*2 = n+1 := by grind
  rw [← IntModule.mul_zsmul, h, IntModule.add_zsmul, hn,
      IntModule.one_zsmul, AddCommMonoid.zero_add]

theorem split_reconstruction {A B : Type} [IntModule A] [IntModule B]
    (norm : A → B) (pull : B → A)
    (hp : ∀ (k : Int) b, pull (k • b) = k • pull b)
    (hd : ∀ d, pull (norm d) = (2 : Int) • d)
    (d : A) (n k : Int) (hn : n • d = 0) (hk : 2*k = n+1) :
    pull (k • norm d) = d := by
  rw [hp, hd]
  exact odd_halving d n k hn hk

theorem target_first_component {A B T : Type}
    [IntModule A] [IntModule B] [IntModule T]
    (pull : A → B) (norm : B → A) (r : B → T) (dual : T → B)
    (hd : ∀ d, dual (r d) = (2 : Int) • d)
    (hn : ∀ a, norm (pull a) = (2 : Int) • a)
    (hs : ∀ d, norm ((2 : Int) • d) = (2 : Int) • norm d) (a : A) :
    norm (dual (r (pull a))) = (4 : Int) • a := by
  rw [hd, hs, hn]
  grind

theorem target_other_component {A B C T : Type}
    [IntModule A] [IntModule B] [IntModule C] [IntModule T]
    (pull : A → B) (other : B → C) (r : B → T) (dual : T → B)
    (hd : ∀ d, dual (r d) = (2 : Int) • d)
    (hz : ∀ a, other (pull a) = 0)
    (hs : ∀ d, other ((2 : Int) • d) = (2 : Int) • other d) (a : A) :
    other (dual (r (pull a))) = 0 := by
  rw [hd, hs, hz]
  grind

end SplitQuotients
#print axioms SplitQuotients.reciprocal_curve_cleared
#print axioms SplitQuotients.reciprocal_inverse_cleared
#print axioms SplitQuotients.complementary_norms
#print axioms SplitQuotients.odd_halving
#print axioms SplitQuotients.split_reconstruction
#print axioms SplitQuotients.target_first_component
#print axioms SplitQuotients.target_other_component
