import Std.Tactic

/-!
Algebraic dependencies for the even-m quartic exclusion. These theorems
alone do not establish the finite-field/cube-root/Frobenius bridge.
-/
namespace ICConstructor
open Lean.Grind

theorem quartic_difference_identity {F : Type} [CommRing F]
    (a u c₁ c₀ : F) :
    (a+u)^4 + c₁*(a+u) + c₀ - (a^4+c₁*a+c₀) =
      u^4+c₁*u+4*a^3*u+6*a^2*u^2+4*a*u^3 := by
  grind

theorem root_difference_linearized {F : Type} [CommRing F] [IsCharP F 2]
    (a u c₁ c₀ : F)
    (h₁ : a^4+c₁*a+c₀=0)
    (h₂ : (a+u)^4+c₁*(a+u)+c₀=0) : u^4+c₁*u=0 := by
  grind

theorem fixed_translation_two_cycle {F : Type} [CommRing F] [IsCharP F 2]
    (f : F → F) (a u : F)
    (hadd : ∀ x y, f (x+y)=f x+f y)
    (hmove : f a=a+u) (hfixed : f u=u) : f (f a)=a := by
  rw [hmove, hadd, hmove, hfixed]
  grind

theorem cubic_and_quartic_cycles_force_fixed {A : Type} (f : A → A) (u : A)
    (h₃ : f (f (f u))=u) (h₄ : f (f (f (f u)))=u) : f u=u := by
  have h := congrArg f h₃
  exact h.symm.trans h₄

theorem fixed_cube_multiplier_three_cycle {F : Type} [CommRing F]
    (f : F → F) (u k : F)
    (hmul : ∀ x y, f (x*y)=f x*f y)
    (hmove : f u=k*u) (hfixed : f k=k) (hcube : k^3=1) :
    f (f (f u))=u := by
  rw [hmove, hmul, hfixed, hmove, hmul, hfixed, hmul, hfixed, hmove]
  grind

theorem quartic_root_two_cycle {F : Type} [Field F] [IsCharP F 2]
    (f : F → F) (a c₁ c₀ : F)
    (hadd : ∀ x y, f (x+y)=f x+f y)
    (hmul : ∀ x y, f (x*y)=f x*f y)
    (hc₁ : f c₁=c₁)
    (hμ₃ : ∀ k : F, k^3=1 → f k=k)
    (hfour : ∀ x, f (f (f (f x)))=x)
    (hroot : a^4+c₁*a+c₀=0)
    (hrootf : (f a)^4+c₁*(f a)+c₀=0) : f (f a)=a := by
  let u := f a+a
  have hmove : f a=a+u := by dsimp [u]; grind
  have hrootu : (a+u)^4+c₁*(a+u)+c₀=0 := by rw [← hmove]; exact hrootf
  have hu₄ := root_difference_linearized a u c₁ c₀ hroot hrootu
  by_cases hz : u=0
  · have ha : f a=a := by rw [hz] at hmove; simpa [Semiring.add_zero] using hmove
    rw [ha, ha]
  · have hinv : u*u⁻¹=1 := Field.mul_inv_cancel hz
    have hu₃ : u^3=c₁ := by grind
    have hpow : f (u^3)=(f u)^3 := by
      simp only [Semiring.pow_succ, Semiring.pow_zero, Semiring.one_mul, hmul]
    have hfu₃ : (f u)^3=c₁ := by rw [← hpow, hu₃, hc₁]
    let k := f u*u⁻¹
    have hku : k*u=f u := by dsimp [k]; grind
    have hk₃ : k^3=1 := by dsimp [k]; grind
    have h₃ := fixed_cube_multiplier_three_cycle f u k hmul hku.symm (hμ₃ k hk₃) hk₃
    have hfixed := cubic_and_quartic_cycles_force_fixed f u h₃ (hfour u)
    exact fixed_translation_two_cycle f a u hadd hmove hfixed

#print axioms quartic_difference_identity
#print axioms root_difference_linearized
#print axioms fixed_translation_two_cycle
#print axioms cubic_and_quartic_cycles_force_fixed
#print axioms fixed_cube_multiplier_three_cycle
#print axioms quartic_root_two_cycle
end ICConstructor
