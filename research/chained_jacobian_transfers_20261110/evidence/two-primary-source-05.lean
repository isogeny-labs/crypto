import Init.Grind.Module.Basic
import Init.Grind.Ring.Basic

/-! Exact algebra for full4-primary quotient evaluation. Actual geometric
surjectivity,Picard maps and the identified kernel graph remain inputs. -/
set_option autoImplicit false
namespace TwoPrimaryQuotient
open Lean.Grind

theorem halving_abscissa_identity {R : Type} [CommRing R] (x a b X : R) :
    (3*x^2+a)^2-4*(x^3+a*x+b)*(2*x+X) =
    x^4-4*X*x^3-2*a*x^2-(8*b+4*a*X)*x+a^2-4*b*X := by
  grind

theorem quotient_lift_independent {B A C : Type}
    [IntModule B] [IntModule A] [IntModule C]
    (k : B → A) (l : B → C)
    (ks : ∀ b c, k (b-c)=k b-k c)
    (ls : ∀ b c, l (b-c)=l b-l c)
    (included : ∀ b, k b=0 → l b=0)
    (b c : B) (same : k b=k c) : l b=l c := by
  have hk : k (b-c)=0 := by rw [ks,same]; grind
  have hl := included (b-c) hk
  rw [ls] at hl
  grind

theorem four_classes_have_sixteen_lifts {B A : Type}
    [IntModule B] [IntModule A]
    (k : B → A) (h : A → B)
    (surjective : ∀ a, ∃ b, k b=a)
    (hz : h 0=0) (linear : ∀ n : Int, ∀ a, h (n • a)=n • h a)
    (norm : ∀ b, h (k b)=(4 : Int) • b)
    (a : A) (ha : (4 : Int) • a=0) :
    ∃ b, k b=a ∧ (16 : Int) • b=0 := by
  obtain ⟨b,hb⟩ := surjective a
  refine ⟨b,hb,?_⟩
  have hzero : h ((4 : Int) • k b)=0 := by rw [hb,ha,hz]
  rw [linear,norm] at hzero
  grind

theorem frobenius_twelve_mod_sixteen {R : Type} [CommRing R]
    (x : R) (h16 : (16 : R)=0)
    (hx : x^2+7*x+101=0 ∨ x^2+11*x+101=0) : x^12=1 := by
  rcases hx with he | hf
  · have h3 : x^3=12*x+3 := by grind
    have h6 : x^6=8*x+9 := by grind
    grind
  · have h3 : x^3=4*x+7 := by grind
    have h6 : x^6=8*x+1 := by grind
    grind

/-- The reverse isogeny direction:restriction kernel degrees divide n.
The canonical sum degree ratio supplies 4*n=r*s. All geometric inputs
are explicit,and ordinary natural-number cancellation is justified by n>0. -/
theorem reverse_cross_degree_four (n r s u v : Nat)
    (hn : 0<n) (hr : n=r*u) (hs : n=s*v) (degree : 4*n=r*s) :
    n=4*(u*v) := by
  apply Nat.mul_left_cancel hn
  calc
    n*n = (r*u)*n := congrArg (fun z => z*n) hr
    _ = (r*u)*(s*v) := congrArg (fun z => (r*u)*z) hs
    _ = (r*s)*(u*v) := by grind
    _ = n*(4*(u*v)) := by rw [← degree]; grind

theorem integer_cleared_maps_vanish_on_four {A C : Type}
    [IntModule A] [IntModule C]
    (f l : A → C) (fz : f 0=0)
    (linear : ∀ n : Int, ∀ a, f (n • a)=n • f a)
    (cleared : ∀ a, l a=(4 : Int) • f a)
    (a : A) (ha : (4 : Int) • a=0) : ∀ n : Int, n • l a=0 := by
  have hf : f ((4 : Int) • a)=0 := by rw [ha,fz]
  have hl : l a=0 := by rw [cleared,← linear,hf]
  intro n
  rw [hl]
  exact IntModule.zsmul_zero n

end TwoPrimaryQuotient
#print axioms TwoPrimaryQuotient.halving_abscissa_identity
#print axioms TwoPrimaryQuotient.quotient_lift_independent
#print axioms TwoPrimaryQuotient.four_classes_have_sixteen_lifts
#print axioms TwoPrimaryQuotient.frobenius_twelve_mod_sixteen
#print axioms TwoPrimaryQuotient.reverse_cross_degree_four
#print axioms TwoPrimaryQuotient.integer_cleared_maps_vanish_on_four
