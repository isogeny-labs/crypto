import Mathlib.RingTheory.RootsOfUnity.PrimitiveRoots
import Mathlib.Tactic.NormNum

set_option autoImplicit false

namespace Curve25519IndexedLabels

example (K : Type*) [Field K] (q : ℕ) [NeZero q] (g r : K)
    (hg : IsPrimitiveRoot g q) (hr : r ^ q = 1) :
    ∃ i < q, g ^ i = r := hg.eq_pow_of_pow_eq_one hr

theorem actual_subgroup_primitive (K : Type*) [Field K] (h : K)
    (hh : IsPrimitiveRoot h (60824497 * 213156431)) :
    IsPrimitiveRoot (h ^ 60824497) 213156431 := by
  convert hh.pow_div_gcd 60824497 (by norm_num) using 1

theorem actual_coset_index_exists (K : Type*) [Field K] (h z : K) (j : ℕ)
    (hh : h ≠ 0) (hg : IsPrimitiveRoot (h ^ 60824497) 213156431)
    (hm : z ^ 213156431 = (h ^ j) ^ 213156431) :
    ∃ t < 213156431, h ^ j * (h ^ 60824497) ^ t = z := by
  have hr : (z / h ^ j) ^ 213156431 = 1 := by
    rw [div_pow, hm, div_self (pow_ne_zero _ (pow_ne_zero j hh))]
  obtain ⟨t, ht, he⟩ := hg.eq_pow_of_pow_eq_one hr
  refine ⟨t, ht, ?_⟩
  rw [he]
  exact mul_div_cancel₀ z (pow_ne_zero j hh)

theorem actual_coset_index_unique (K : Type*) [Field K] (h : K) (j i t : ℕ)
    (hh : h ≠ 0) (hg : IsPrimitiveRoot (h ^ 60824497) 213156431)
    (hi : i < 213156431) (ht : t < 213156431)
    (he : h ^ j * (h ^ 60824497) ^ i = h ^ j * (h ^ 60824497) ^ t) : i = t :=
  hg.pow_inj hi ht (mul_left_cancel₀ (pow_ne_zero j hh) he)

#print axioms actual_subgroup_primitive
#print axioms actual_coset_index_exists
#print axioms actual_coset_index_unique
end Curve25519IndexedLabels
