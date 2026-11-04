import Mathlib

/-! Kernel mechanism only. Binding to a signed-column matrix and the general
rank/count conclusion are NOT asserted by defining these hypotheses. -/
namespace PairRank

theorem even_reflection_translation {G : Type*} [AddCommGroup G] {K : Type*}
    (f : G → K) (m : G) (he : ∀ x, f (-x) = f x)
    (hr : ∀ x, f (m-x) = f x) : ∀ x, f (x+m) = f x := by
  intro x
  calc
    f (x+m) = f (m-(-x)) := by congr 1; abel
    _ = f (-x) := hr (-x)
    _ = f x := he x

theorem periodic_multiples {r : ℕ} {K : Type*} (f : ZMod r → K)
    (m : ZMod r) (hp : ∀ x, f (x+m) = f x) (n : ℕ) :
    f ((n : ZMod r)*m) = f 0 := by
  induction n with
  | zero => simp
  | succ n ih =>
    rw [Nat.cast_add, Nat.cast_one, add_mul, one_mul, hp, ih]

theorem prime_even_reflection_zero {r : ℕ} [Fact r.Prime]
    {K : Type*} [Zero K] (f : ZMod r → K) (m : ZMod r) (hm : m ≠ 0)
    (hz : f 0 = 0) (he : ∀ x, f (-x) = f x)
    (hr : ∀ x, f (m-x) = f x) : ∀ x, f x = 0 := by
  intro x
  have hp := even_reflection_translation f m he hr
  have h := periodic_multiples f m hp (x / m).val
  rw [ZMod.natCast_zmod_val, div_mul_cancel₀ x hm, hz] at h
  exact h

/-- w is a symmetric weight on unordered endpoint pairs. Its signed boundary
is the displayed difference. The zero endpoint is excluded and only pairs
summing to m are supported. These are actual linear equations, not an assumed
independence/rank assertion. Diagonal coefficient conversion still needs 2≠0
when applying this kernel to ordinary unweighted coefficient rows. -/
theorem symmetric_pair_weight_kernel {r : ℕ} [Fact r.Prime]
    {K : Type*} [AddCommGroup K] (w : ZMod r → ZMod r → K)
    (m : ZMod r) (hm : m ≠ 0)
    (hs : ∀ x y, w x y = w y x)
    (hz : ∀ y, w 0 y = 0)
    (support : ∀ x y, x+y ≠ m → w x y = 0)
    (balance : ∀ x, w x (m-x) - w (-x) (m+x) = 0) :
    ∀ x y, w x y = 0 := by
  let f : ZMod r → K := fun x => w x (m-x)
  have he : ∀ x, f (-x) = f x := by
    intro x
    have h := sub_eq_zero.mp (balance x)
    simpa [f] using h.symm
  have hr : ∀ x, f (m-x) = f x := by
    intro x
    simpa [f] using hs (m-x) x
  have hf0 : f 0 = 0 := by simpa [f] using hz m
  have hf := prime_even_reflection_zero f m hm hf0 he hr
  intro x y
  by_cases h : x+y = m
  · have hy : y = m-x := by linear_combination h
    rw [hy]
    exact hf x
  · exact support x y h

theorem signed_pair_count_polynomial (b : ℤ) :
    (2*b)*(2*b+1)-2*b = 4*b*b := by ring

#print axioms even_reflection_translation
#print axioms periodic_multiples
#print axioms prime_even_reflection_zero
#print axioms symmetric_pair_weight_kernel
#print axioms signed_pair_count_polynomial
end PairRank
