import Mathlib.Tactic.LinearCombination
import Mathlib.Tactic.Ring

/-!+# Coefficient rigidity for the degree-four obstruction

This is the finite evaluation argument from the appendix of
`universal_five_symmetric_audit.txt`. It does not assume a closure/lower-bound
conclusion: its premises are explicit scalar evaluation equalities.
-/
namespace ThreePointRigidity

variable {K : Type*} [Field K]

/-- A quadratic vanishing at three distinct points has zero leading
coefficient. This form avoids any assumptions about polynomial functions. -/
theorem quadratic_leading_zero (a b c A B C : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c)
    (ha : A*a^2+B*a+C = 0) (hb : A*b^2+B*b+C = 0)
    (hc : A*c^2+B*c+C = 0) : A = 0 := by
  have h : A*((a-b)*(a-c)*(b-c)) = 0 := by
    linear_combination (b-c)*ha + (c-a)*hb + (a-b)*hc
  exact (mul_eq_zero.mp h).resolve_right
    (mul_ne_zero (mul_ne_zero (sub_ne_zero.mpr hab) (sub_ne_zero.mpr hac))
      (sub_ne_zero.mpr hbc))

/-- Affine interpolation of the two quartic u² coefficients on the guarded
three-point grid is impossible unless both quartic weights vanish.
The only characteristic restriction needed is 6 != 0. -/
theorem quartic_weights_zero (a b c r c₁ c₂ l₀ lₓ lᵧ lz lv : K)
    (hab : a ≠ b) (hac : a ≠ c) (hbc : b ≠ c) (h6 : (6 : K) ≠ 0)
    (h : ∀ x ∈ ({a,b,c} : Set K), ∀ y ∈ ({a,b,c} : Set K), x ≠ y →
      ∀ z ∈ ({a,b,c} : Set K),
      c₁*(x-y)^2+c₂*(z-r)^2 = l₀+lₓ*x+lᵧ*y+lz*z+lv*(x-y)⁻¹) :
    c₁ = 0 ∧ c₂ = 0 := by
  have ha := h a (by simp) b (by simp) hab a (by simp)
  have hb := h a (by simp) b (by simp) hab b (by simp)
  have hc := h a (by simp) b (by simp) hab c (by simp)
  have h₂ : c₂ = 0 := by
    apply quadratic_leading_zero a b c c₂ (-2*c₂*r-lz)
      (c₁*(a-b)^2+c₂*r^2-l₀-lₓ*a-lᵧ*b-lv*(a-b)⁻¹) hab hac hbc
    · linear_combination ha
    · linear_combination hb
    · linear_combination hc
  have hz : lz = 0 := by
    have hzprod : lz*(a-b) = 0 := by
      rw [h₂] at ha hb
      linear_combination -ha + hb
    exact (mul_eq_zero.mp hzprod).resolve_right (sub_ne_zero.mpr hab)
  have hs (x y : K) (hx : x ∈ ({a,b,c} : Set K))
      (hy : y ∈ ({a,b,c} : Set K)) (hxy : x ≠ y) :
      2*c₁*(x-y)^2 = 2*l₀+(lₓ+lᵧ)*(x+y) := by
    have hxy' := h x hx y hy hxy a (by simp)
    have hyx' := h y hy x hx hxy.symm a (by simp)
    rw [h₂, hz] at hxy' hyx'
    rw [show y-x = -(x-y) by ring, inv_neg] at hyx'
    linear_combination hxy' + hyx'
  have eab := hs a b (by simp) (by simp) hab
  have eac := hs a c (by simp) (by simp) hac
  have ebc := hs b c (by simp) (by simp) hbc
  have hlead : -6*c₁ = 0 := by
    apply quadratic_leading_zero a b c (-6*c₁)
      (4*(a+b+c)*c₁+(lₓ+lᵧ))
      (2*c₁*((a+b+c)^2-4*(a*b+a*c+b*c))-2*l₀-(lₓ+lᵧ)*(a+b+c))
      hab hac hbc
    · linear_combination ebc
    · linear_combination eac
    · linear_combination eab
  exact ⟨(mul_eq_zero.mp hlead).resolve_left (neg_ne_zero.mpr h6), h₂⟩

end ThreePointRigidity

#print axioms ThreePointRigidity.quadratic_leading_zero
#print axioms ThreePointRigidity.quartic_weights_zero
