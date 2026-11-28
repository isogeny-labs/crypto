import Init.Grind.Module.Basic
import Init.Grind.Ring.Basic

/-! Algebra and functional descent used by the written degree7 Jacobian proof.
No abelian varieties,regular morphisms,finite fields or actual divisors are
constructed. The universal-property bridge for varieties remains written. -/
set_option autoImplicit false
namespace SevenJacobian
open Lean.Grind

theorem support_cleared_identity {K : Type} [CommRing K] (u v b w : K) :
    (u-b*w)*v = u*v-b+b*(1-v*w) := by grind

theorem matched_pullback_support {K : Type} [CommRing K] (u v b w : K)
    (huv : u*v = b) (hvw : v*w = 1) : u = b*w := by grind

theorem kernel_fiber_constancy {A B C : Type}
    [IntModule A] [IntModule B] [IntModule C]
    (q : A → B) (f : A → C)
    (hq : ∀ a b, q (a-b) = q a-q b)
    (hf : ∀ a b, f (a-b) = f a-f b)
    (hker : ∀ a, q a = 0 → f a = 0) :
    ∀ a b, q a = q b → f a = f b := by
  intro a b hab
  have hzq : q (a-b) = 0 := by rw [hq, hab]; grind
  have hzf := hker (a-b) hzq
  rw [hf] at hzf
  grind

theorem factor_through_surjection {A B C : Type}
    (q : A → B) (f : A → C) (hq : ∀ b, ∃ a, q a = b)
    (hf : ∀ a b, q a = q b → f a = f b) :
    ∃ g : B → C, (∀ a, g (q a) = f a) ∧
      (∀ h : B → C, (∀ a, h (q a) = f a) → h = g) := by
  classical
  let preimage : B → A := fun b => Classical.choose (hq b)
  have hs : ∀ b, q (preimage b) = b := fun b => Classical.choose_spec (hq b)
  refine ⟨(fun b => f (preimage b)), ?_, ?_⟩
  · intro a
    exact hf (preimage (q a)) a (hs (q a))
  · intro h hh
    funext b
    have hb := hh (preimage b)
    rw [hs b] at hb
    exact hb

theorem factor_respects_addition {A B C : Type}
    [IntModule A] [IntModule B] [IntModule C]
    (q : A → B) (f : A → C) (g : B → C)
    (hqs : ∀ b, ∃ a, q a = b)
    (hqa : ∀ a b, q (a+b) = q a+q b)
    (hfa : ∀ a b, f (a+b) = f a+f b)
    (hg : ∀ a, g (q a) = f a) :
    ∀ b c, g (b+c) = g b+g c := by
  intro b c
  obtain ⟨a, ha⟩ := hqs b
  obtain ⟨d, hd⟩ := hqs c
  rw [← ha, ← hd, ← hqa a d, hg (a+d), hfa a d, hg a, hg d]

/- H represents a Hom group,not the geometric point group (which HAS
nonzero2-torsion). Its torsion-freeness is an explicit written bridge. -/
theorem cancel_factor_norm {H : Type} [IntModule H]
    (a identity : H) (hc : ∀ h : H, (4 : Int) • h = 0 → h = 0)
    (hn : (4 : Int) • a = (28 : Int) • identity) :
    a = (7 : Int) • identity := by
  have hz : (4 : Int) • (a-(7 : Int) • identity) = 0 := by grind
  have hh := hc (a-(7 : Int) • identity) hz
  exact AddCommGroup.sub_eq_zero_iff.mp hh

theorem recover_norm14_order109 {A : Type} [IntModule A]
    (p : A) (hp : (109 : Int) • p = 0) :
    (39 : Int) • ((14 : Int) • p) = p := by grind

theorem recover_norm28_order109 {A : Type} [IntModule A]
    (p : A) (hp : (109 : Int) • p = 0) :
    (74 : Int) • ((28 : Int) • p) = p := by grind

end SevenJacobian
#print axioms SevenJacobian.support_cleared_identity
#print axioms SevenJacobian.matched_pullback_support
#print axioms SevenJacobian.kernel_fiber_constancy
#print axioms SevenJacobian.factor_through_surjection
#print axioms SevenJacobian.factor_respects_addition
#print axioms SevenJacobian.cancel_factor_norm
#print axioms SevenJacobian.recover_norm14_order109
#print axioms SevenJacobian.recover_norm28_order109
