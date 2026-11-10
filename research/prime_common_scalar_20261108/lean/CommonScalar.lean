module
public import KernelNodes
public section
namespace CommonScalar
open CauchyNormalizer

/-- One scalar polynomial in T, not an entry-dependent inverse normalizer. -/
def normalizer (a : ℕ) (t : ℚ) : ℚ :=
  ∑ s ∈ Finset.range (2^(a+1)), baseKernel a s t / (cauchyRho a s)^2

theorem normalizer_node (a m : ℕ) (hm : m < 2^(a+1)) :
    normalizer a ((m : ℚ)-(2^a : ℕ)) = (cauchyRho a m)⁻¹ := by
  unfold normalizer
  simp_rw [baseKernel_node a _ m hm]
  rw [Finset.sum_eq_single m]
  · simp only [ite_true]
    field_simp [cauchyRho_ne_zero a m hm]
  · intro b hb hbm
    simp [Ne.symm hbm]
  · intro h
    exact False.elim (h (Finset.mem_range.mpr hm))

@[expose] def kernel (a k : ℕ) (t : ℚ) : ℚ :=
  normalizer a t * baseKernel a k t * baseKernel a k t

theorem kernel_formula (a k : ℕ) (t : ℚ) :
    kernel a k t = normalizer a t * baseKernel a k t * baseKernel a k t := rfl

theorem node_preserved (a k m : ℕ)
    (hk : k < 2^(a+1)) (hm : m < 2^(a+1)) :
    kernel a k ((m : ℚ)-(2^a : ℕ)) = baseKernel a k ((m : ℚ)-(2^a : ℕ)) := by
  unfold kernel
  rw [normalizer_node a m hm, baseKernel_node a k m hm]
  by_cases h : m=k
  · subst m
    simp only [ite_true]
    field_simp [cauchyRho_ne_zero a k hk]
  · simp [h]

theorem normalizer_origin (a : ℕ) : normalizer a 0 = 1 := by
  have h := normalizer_node a (2^a) (center_lt a)
  simpa only [sub_self, cauchyRho_center, inv_one] using h

theorem kernel_origin (a k : ℕ) :
    kernel a k 0 = if k=2^a then 1 else 0 := by
  unfold kernel
  rw [normalizer_origin, baseKernel_origin]
  split_ifs <;> norm_num

/-- Actual matrix indexing; preserving the center node retains the identity. -/
theorem matrix_origin (a x y : ℕ) (hx : x < 2^a) (hy : y < 2^a) :
    kernel a (y+2^a-x) 0 = if x=y then 1 else 0 := by
  rw [kernel_origin]
  have h : y+2^a-x=2^a ↔ x=y := by omega
  simp only [h]

/-- General obstruction for the proposed value-preserving jet correction. -/
theorem jet_origin_obstruction (W C H : ℚ → ℚ) (hC : C 0 = 0) :
    W 0 + C 0 * H 0 = W 0 := by rw [hC]; ring

/-- Exact finite expansion underlying the squared Cauchy crossing modes. -/
theorem squared_geometric_sum (x : ℚ) (n : ℕ) :
    (1-x)^2 * (∑ b ∈ Finset.range n, (b+1 : ℚ)*x^b) =
      1 - (n+1 : ℚ)*x^n + (n : ℚ)*x^(n+1) := by
  induction n with
  | zero => simp
  | succ n ih =>
    rw [Finset.sum_range_succ, mul_add, ih]
    push_cast
    ring

theorem squared_geometric_remainder (x : ℚ) (n : ℕ) (hx : 1-x ≠ 0) :
    1/(1-x)^2 - (∑ b ∈ Finset.range n, (b+1 : ℚ)*x^b) =
      x^n*((n+1 : ℚ)-(n : ℚ)*x)/(1-x)^2 := by
  have h := squared_geometric_sum x n
  simp only [pow_succ] at h
  field_simp [hx]
  linear_combination -h

/-- Precise scope: feasible-set inclusion for the coarse weights and a fully
doubled precision budget. This is not a lower bound for all ECDLP algorithms. -/
theorem coarse_budget_inclusion {ι : Type*} (s : Finset ι)
    (index multiplicity : ι → ℕ) (q : ℕ)
    (h : ∑ j ∈ s, (index j+1)*multiplicity j < q) :
    ∑ j ∈ s, (index j+2)*multiplicity j < 2*q := by
  have hle : ∑ j ∈ s, (index j+2)*multiplicity j ≤
      2*(∑ j ∈ s, (index j+1)*multiplicity j) := by
    rw [Finset.mul_sum]
    apply Finset.sum_le_sum
    intro j hj
    nlinarith
  omega

end CommonScalar
#print axioms CommonScalar.normalizer_node
#print axioms CommonScalar.normalizer_origin
#print axioms CommonScalar.node_preserved
#print axioms CommonScalar.kernel_origin
#print axioms CommonScalar.matrix_origin
#print axioms CommonScalar.jet_origin_obstruction
#print axioms CommonScalar.squared_geometric_sum
#print axioms CommonScalar.squared_geometric_remainder
#print axioms CommonScalar.coarse_budget_inclusion
