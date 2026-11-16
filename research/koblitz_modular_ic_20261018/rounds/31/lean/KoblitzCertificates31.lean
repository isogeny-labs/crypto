import Mathlib.Data.ZMod.Basic
import Mathlib.Algebra.BigOperators.Group.Finset.Defs
import Mathlib.Tactic.FieldSimp
import Mathlib.Tactic.LinearCombination
import Mathlib.Tactic.Ring

/-! Structural lemmas used by CM31 and REVERSE31. These are not a formalization
of the native finite-field records, modularity, or a Picard-class evaluation. -/
namespace KoblitzCertificates31

theorem cyclic_translation_order (m s : ℕ) (hm : m ≠ 0) :
    addOrderOf (s : ZMod m) = m / Nat.gcd m s :=
  ZMod.addOrderOf_coe s hm

theorem cyclic_translation_period (m s k : ℕ) (hm : m ≠ 0) :
    k • (s : ZMod m) = 0 ↔ m / Nat.gcd m s ∣ k := by
  rw [← cyclic_translation_order m s hm]
  exact addOrderOf_dvd_iff_nsmul_eq_zero.symm

theorem relabelled_root_product {I J M : Type*} [Fintype I] [Fintype J]
    [CommMonoid M] (e : I ≃ J) (f : I → M) (g : J → M)
    (h : ∀ i, f i = g (e i)) :
    (∏ i, f i) = ∏ j, g j := Fintype.prod_equiv e f g h

section Tate
variable {K : Type*} [Field K]

theorem tate_normalization_a1 (r x₂ : K) (hr : r ≠ 0)
    (hv : r + x₂ ≠ 0) : (r / (r+x₂)) * (1+x₂/r) = 1 := by
  field_simp
  <;> ring

theorem tate_normalization_b2 (r v : K) (hr : r ≠ 0) (hv : v ≠ 0) :
    (v^3/r^2) * (r/v)^2 = v := by
  field_simp

theorem tate_normalization_b3 (r v : K) (hr : r ≠ 0) (hv : v ≠ 0) :
    (v^3/r^2) * (r/v)^3 = r := by
  field_simp

end Tate

section Twist
variable {K : Type*} [CommRing K] [CharP K 2]

theorem twist_quadratic_coefficient (a b l t : K) (ht : t^2+t=1) :
    b+(l+a*t)^2+a*(l+a*t) = b+l^2+a*l+a^2 := by
  have h2 : (2:K)=0 := CharP.cast_eq_zero K 2
  linear_combination a^2*ht + (l*a*t)*h2

theorem twist_linear_coefficient (a b l n t : K) :
    a*(n+b*t)+b*(l+a*t) = a*n+b*l := by
  have h2 : (2:K)=0 := CharP.cast_eq_zero K 2
  linear_combination (a*b*t)*h2

theorem twist_constant_coefficient (b n t : K) (ht : t^2+t=1) :
    (n+b*t)^2+b*(n+b*t) = n^2+b*n+b^2 := by
  have h2 : (2:K)=0 := CharP.cast_eq_zero K 2
  linear_combination b^2*ht + (n*b*t)*h2

theorem diamond_sign_involution (a b l n : K) :
    ((l+a)+a, (n+b)+b) = (l,n) := by
  have h2 : (2:K)=0 := CharP.cast_eq_zero K 2
  apply Prod.ext
  · linear_combination a*h2
  · linear_combination b*h2

end Twist
end KoblitzCertificates31
