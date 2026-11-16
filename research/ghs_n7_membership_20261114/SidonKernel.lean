import Mathlib

/-!
The algebraic core of the degree-seven kernel-space intersection argument.
These statements use an arbitrary characteristic-two field and a field
endomorphism `σ`.  For `K = F_(q^7)` and `σ x = x^q`, the fixed field of `σ`
and `σ²` is `F_q`.  The finite-field fixed-field and dimension bridges are
not asserted here.
-/

namespace GHSN7

variable {K : Type*} [Field K] [CharP K 2]

private theorem two_is_zero : (2 : K) = 0 := CharTwo.two_eq_zero

private theorem two_equations (a b x y u v : K)
    (hx : a * x + b * u = 0) (hy : a * y + b * v = 0)
    (hdet : x * v - y * u ≠ 0) : a = 0 ∧ b = 0 := by
  have ha : a * (x * v - y * u) = 0 := by
    linear_combination v * hx - u * hy
  have hb : b * (x * v - y * u) = 0 := by
    linear_combination x * hy - y * hx
  exact ⟨(mul_eq_zero.mp ha).resolve_right hdet,
    (mul_eq_zero.mp hb).resolve_right hdet⟩

def KernelOne (σ : K →+* K) (x : K) : Prop :=
  σ (σ (σ x)) + σ x + x = 0

def KernelTwo (σ : K →+* K) (x : K) : Prop :=
  σ (σ (σ x)) + σ (σ x) + x = 0

theorem kernel_one_intersection (σ : K →+* K) (t x y : K)
    (hx : KernelOne σ x) (hy : KernelOne σ y)
    (htx : KernelOne σ (t * x)) (hty : KernelOne σ (t * y))
    (hdet : x * σ y - y * σ x ≠ 0) : σ t = t := by
  have ex : (σ (σ (σ t)) + t) * x +
      (σ (σ (σ t)) + σ t) * σ x = 0 := by
    dsimp [KernelOne] at hx htx
    simp only [map_mul] at htx
    linear_combination htx - σ (σ (σ t)) * hx +
      (σ (σ (σ t)) * x + σ (σ (σ t)) * σ x) * (two_is_zero (K := K))
  have ey : (σ (σ (σ t)) + t) * y +
      (σ (σ (σ t)) + σ t) * σ y = 0 := by
    dsimp [KernelOne] at hy hty
    simp only [map_mul] at hty
    linear_combination hty - σ (σ (σ t)) * hy +
      (σ (σ (σ t)) * y + σ (σ (σ t)) * σ y) * (two_is_zero (K := K))
  obtain ⟨ha, hb⟩ := two_equations
    (σ (σ (σ t)) + t) (σ (σ (σ t)) + σ t)
    x y (σ x) (σ y) ex ey hdet
  linear_combination hb - ha

theorem kernel_two_intersection (σ : K →+* K) (t x y : K)
    (hx : KernelTwo σ x) (hy : KernelTwo σ y)
    (htx : KernelTwo σ (t * x)) (hty : KernelTwo σ (t * y))
    (hdet : x * σ (σ y) - y * σ (σ x) ≠ 0) :
    σ (σ t) = t := by
  have ex : (σ (σ (σ t)) + t) * x +
      (σ (σ (σ t)) + σ (σ t)) * σ (σ x) = 0 := by
    dsimp [KernelTwo] at hx htx
    simp only [map_mul] at htx
    linear_combination htx - σ (σ (σ t)) * hx +
      (σ (σ (σ t)) * x + σ (σ (σ t)) * σ (σ x)) * (two_is_zero (K := K))
  have ey : (σ (σ (σ t)) + t) * y +
      (σ (σ (σ t)) + σ (σ t)) * σ (σ y) = 0 := by
    dsimp [KernelTwo] at hy hty
    simp only [map_mul] at hty
    linear_combination hty - σ (σ (σ t)) * hy +
      (σ (σ (σ t)) * y + σ (σ (σ t)) * σ (σ y)) * (two_is_zero (K := K))
  obtain ⟨ha, hb⟩ := two_equations
    (σ (σ (σ t)) + t) (σ (σ (σ t)) + σ (σ t))
    x y (σ (σ x)) (σ (σ y)) ex ey hdet
  linear_combination hb - ha

end GHSN7
