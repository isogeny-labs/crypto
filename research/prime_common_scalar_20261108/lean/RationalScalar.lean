module
public import CommonScalar
public import Mathlib.Algebra.Polynomial.BigOperators
public section
namespace RationalScalar
open CauchyNormalizer

/-- One common denominator, independent of entry index. -/
@[expose] def denominator (a : ℕ) (t : ℚ) : ℚ :=
  ∑ s ∈ Finset.range (2^(a+1)), baseKernel a s t

theorem denominator_node (a m : ℕ) (hm : m < 2^(a+1)) :
    denominator a ((m : ℚ)-(2^a : ℕ)) = cauchyRho a m := by
  unfold denominator
  simp_rw [baseKernel_node a _ m hm]
  rw [Finset.sum_eq_single m]
  · simp
  · intro b hb hbm
    simp [Ne.symm hbm]
  · intro h
    exact False.elim (h (Finset.mem_range.mpr hm))

def kernel (a k : ℕ) (t : ℚ) : ℚ :=
  baseKernel a k t * baseKernel a k t / denominator a t

theorem node_preserved (a k m : ℕ) (hk : k < 2^(a+1)) (hm : m < 2^(a+1)) :
    kernel a k ((m : ℚ)-(2^a : ℕ)) = baseKernel a k ((m : ℚ)-(2^a : ℕ)) := by
  unfold kernel
  rw [denominator_node a m hm, baseKernel_node a k m hm]
  by_cases h : m=k
  · subst m
    simp only [ite_true]
    field_simp [cauchyRho_ne_zero a k hk]
  · simp [h]

theorem denominator_origin (a : ℕ) : denominator a 0 = 1 := by
  have h := denominator_node a (2^a) (center_lt a)
  simpa only [sub_self, cauchyRho_center] using h

theorem denominator_node_unit (a m : ℕ) (hm : m < 2^(a+1)) :
    padicValRat 2 (denominator a ((m : ℚ)-(2^a : ℕ))) = 0 := by
  rw [denominator_node a m hm]
  exact cauchyRho_unit a m hm

/-- Pointwise conditional lemma; full chart-series integrality is stronger. -/
theorem kernel_valuation (a k : ℕ) (t : ℚ)
    (hb : baseKernel a k t ≠ 0) (hd : denominator a t ≠ 0)
    (hunit : padicValRat 2 (denominator a t) = 0) :
    padicValRat 2 (kernel a k t) = 2*padicValRat 2 (baseKernel a k t) := by
  unfold kernel
  rw [padicValRat.div (mul_ne_zero hb hb) hd, padicValRat.mul hb hb, hunit]
  ring

theorem kernel_origin (a k : ℕ) : kernel a k 0 = if k=2^a then 1 else 0 := by
  unfold kernel
  rw [denominator_origin, baseKernel_origin]
  split_ifs <;> norm_num

/-- Homogeneous denominator clearing, independent of contraction order. -/
theorem clear_product {ι : Type*} (s : Finset ι) (f : ι → ℚ) (d : ℚ) (hd : d ≠ 0) :
    (∏ j ∈ s, f j / d) * d^s.card = ∏ j ∈ s, f j := by
  rw [Finset.prod_div_distrib]
  simp only [Finset.prod_const]
  field_simp

theorem clear_homogeneous_sum {ι κ : Type*} (s : Finset ι) (slots : ι → Finset κ)
    (f : ι → κ → ℚ) (weight : ι → ℚ) (d : ℚ) (M : ℕ)
    (hd : d ≠ 0) (hM : ∀ j ∈ s, (slots j).card = M) :
    (∑ j ∈ s, weight j * ∏ k ∈ slots j, f j k / d) * d^M =
      ∑ j ∈ s, weight j * ∏ k ∈ slots j, f j k := by
  rw [Finset.sum_mul]
  apply Finset.sum_congr rfl
  intro j hj
  rw [mul_assoc, ← hM j hj, clear_product (slots j) (f j) d hd]

/-- Unit status remains a separate obligation. -/
theorem scalar_relation (a k : ℕ) (t : ℚ) (hd : denominator a t ≠ 0) :
    kernel a k t * (CommonScalar.normalizer a t * denominator a t) =
      CommonScalar.kernel a k t := by
  rw [CommonScalar.kernel_formula]
  unfold kernel
  field_simp

/-- Generic degree bound; actual puncture-polynomial bridge remains explicit. -/
theorem squared_product_degree {ι : Type*} (s : Finset ι)
    (f : ι → Polynomial ℚ) (d : ℕ) (hf : ∀ j ∈ s, (f j).natDegree ≤ d) :
    (∏ j ∈ s, (f j)^2).natDegree ≤ s.card * (2*d) := by
  calc
    _ ≤ ∑ j ∈ s, ((f j)^2).natDegree := Polynomial.natDegree_prod_le _ _
    _ ≤ ∑ j ∈ s, 2*d := Finset.sum_le_sum (fun j hj => Polynomial.natDegree_pow_le_of_le 2 (hf j hj))
    _ = _ := by simp

/-- Numeric multiplicity margin only, not a full recovery theorem. -/
theorem three_level_margin (h u M : ℚ) (hu : 3 ≤ u) (hh : 0 < h)
    (hbudget : 3*M ≤ h*u) :
    4*M < h*(u+(u-1)/2+(u-2)/4) := by
  have hp : 0 ≤ h*(u-3) := mul_nonneg (le_of_lt hh) (by linarith)
  nlinarith

end RationalScalar
#print axioms RationalScalar.denominator_node
#print axioms RationalScalar.node_preserved
#print axioms RationalScalar.denominator_origin
#print axioms RationalScalar.denominator_node_unit
#print axioms RationalScalar.kernel_valuation
#print axioms RationalScalar.kernel_origin
#print axioms RationalScalar.clear_product
#print axioms RationalScalar.clear_homogeneous_sum
#print axioms RationalScalar.scalar_relation
#print axioms RationalScalar.squared_product_degree
#print axioms RationalScalar.three_level_margin
