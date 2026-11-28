import Mathlib.Tactic.FieldSimp
import Mathlib.Tactic.Ring
import Mathlib.LinearAlgebra.FiniteDimensional.Lemmas
namespace Rational50
/-- The quadratic norm relation exactly clears the rational denominator. -/
theorem quadratic_norm_zero_iff {K : Type*} [Field K]
    (a b e : K) (hb : b ≠ 0) :
    (a / b)^2 + a / b + e = 0 ↔ a^2 + a*b + e*b^2 = 0 := by
  have hid : (a / b)^2 + a / b + e =
      (a^2 + a*b + e*b^2) / b^2 := by
    field_simp [hb] <;> ring
  rw [hid, div_eq_iff (pow_ne_zero 2 hb), zero_mul]

/-- Recover the linear second coordinate with the same rational denominator. -/
theorem linear_coordinate_numerator {K : Type*} [Field K]
    (a b c d : K) (hb : b ≠ 0) :
    c + d * (a / b) = (c*b + d*a) / b := by
  field_simp [hb] <;> ring

/-- A zero denominator breaks the norm equivalence even for a=1,e=0. -/
theorem zero_denominator_counterexample (K : Type*) [Field K] :
    ((1 / (0 : K))^2 + 1 / (0 : K) + 0 = 0) ∧
    ((1 : K)^2 + 1*0 + 0*0^2 ≠ 0) := by
  simp
/-- A Bezout identity for a,b supplies an inverse of b modulo the
quadratic norm in characteristic two. -/
theorem bezout_norm_inverse {K : Type*} [CommRing K]
    (a b e x y : K) (h2 : (2 : K) = 0) (h : x*a + y*b = 1) :
    b * (x^2*a + (y^2 + e*x^2)*b) +
      x^2 * (a^2 + a*b + e*b^2) = 1 := by
  calc
    _ = (x*a + y*b)^2 + (2 : K) *
        (x^2*a*b + e*x^2*b^2 - x*y*a*b) := by ring
    _ = 1 := by rw [h2, zero_mul, add_zero, h]; simp
/-- A map from a finite field extension to a nontrivial algebra of equal
finite dimension is bijective. The target need not be assumed a field. -/
theorem field_map_bijective_of_equal_finrank
    (K L T : Type*) [Field K] [Field L] [CommRing T] [Nontrivial T]
    [Algebra K L] [Algebra K T]
    [FiniteDimensional K L] [FiniteDimensional K T]
    (f : L →ₐ[K] T) (h : Module.finrank K L = Module.finrank K T) :
    Function.Bijective f := by
  refine ⟨f.toRingHom.injective, ?_⟩
  exact (LinearMap.injective_iff_surjective_of_finrank_eq_finrank
    (f := f.toLinearMap) h).mp f.toRingHom.injective
/-- The nontrivial coordinate round trip on the two-chart overlap. -/
theorem overlap_coordinate_roundtrip {K : Type*} [CommRing K]
    (t u s : K) (h : u*s = 1) : (t*s^4)*u^4 = t := by
  calc
    _ = t*(u*s)^4 := by ring
    _ = t := by rw [h]; ring
end Rational50
