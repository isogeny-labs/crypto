import Mathlib
import ChainSd.GroupAlgebra
import ChainSd.LowerThree

/-!
# Theorem 6 in Lean: `1 ∉ V_3`, so `LFD(k) ≥ 4` for every depth `k ≥ 2`

Report §7.1 proves `LFD(k) ≥ 4` on paper.  This file kernel-checks it.

*Proof shape.*  At depth `2` (exponents in `ZMod 4 × ZMod 4`, four shift variables) the subspace
`W = span {g, σ_i g}` is `3`-closed: an element `a • g + Σ c_i • σ_i g` of weight `≤ 2` has zero
coefficients at the four weight-`3` characters `x₁³x₂, x₁³x₂², x₁x₂³, x₁²x₂³`, and these four
linear conditions force every `c_i = 0` once `2 ≠ 0` and `A + 3t² ≠ 0` (each coordinate gives a
`2 × 2` system with determinant `-2 (3t² + A)`).  So the only admitted shifts are those of
multiples of `g`, which stay in `W`.  Since `1` has weight `0`, `1 ∈ W` would make `1` a
multiple of `g`, which the coefficient at `x₁²x₂²` refutes (as in `LowerThree`).

The statement for every depth `k ≥ 2` then follows from **depth monotonicity**
(`one_mem_closure_of_le`): if `1` were in the degree-3 closure at depth `k`, it would be in the
degree-3 closure at depth `2`.

*Hypotheses.* `p ≠ 2` and `A + 3t² ≠ 0`; any `B`.  The report's extra hypothesis `t ≠ 0` is not
needed (for `t = 0` the determinant is `-2A`, nonzero exactly when `A ≠ -3t² = 0`).

This is an all-depth (uniform in `k`) constant lower bound.  It is not a finite certificate and
not the open linear lower bound (report §7.3, T9).
-/

namespace ChainSd

open AddMonoidAlgebra

variable {p : ℕ} [Fact p.Prime]

section DepthTwo

/-- The degree-3 candidate subspace at depth 2: `span {g, σ_i g}`. -/
noncomputable def W3 (A B t : ZMod p) : Submodule (ZMod p) (ChainAlg p 2) :=
  Submodule.span (ZMod p)
    (insert (gen p 2 A B t) (Set.range fun js : Fin 2 × Bool => shiftMap p 2 js (gen p 2 A B t)))

/-- An exponent of weight `3` carries a zero coefficient in any element of weight `≤ 2`. -/
theorem coeff_eq_zero_of_mem_weightSub_two {u : ChainAlg p 2} (hu : u ∈ weightSub p 2 2)
    {w : Expo 2} (hw : wt 2 w = 3) : u.coeff w = 0 := by
  rw [mem_weightSub] at hu
  by_contra h
  have := hu w (Finsupp.mem_support_iff.mpr h)
  omega

theorem popcount_one : popcount 1 = 1 := by
  rw [popcount_eq, popcount_eq 0]; simp [popcount_zero]
theorem popcount_two : popcount 2 = 1 := by
  rw [popcount_eq, popcount_eq 1, popcount_eq 0]; simp [popcount_zero]
theorem popcount_three : popcount 3 = 2 := by
  rw [popcount_eq, popcount_eq 1, popcount_eq 0]; simp [popcount_zero]

/-- Weights of the four weight-3 characters at depth 2. -/
theorem wt_31 : wt 2 ((3 : ZMod 4), (1 : ZMod 4)) = 3 := by
  show popcount (3 : ZMod 4).val + popcount (1 : ZMod 4).val = 3
  rw [show (3 : ZMod 4).val = 3 from rfl, show (1 : ZMod 4).val = 1 from rfl, popcount_three,
    popcount_one]
theorem wt_32 : wt 2 ((3 : ZMod 4), (2 : ZMod 4)) = 3 := by
  show popcount (3 : ZMod 4).val + popcount (2 : ZMod 4).val = 3
  rw [show (3 : ZMod 4).val = 3 from rfl, show (2 : ZMod 4).val = 2 from rfl, popcount_three,
    popcount_two]
theorem wt_13 : wt 2 ((1 : ZMod 4), (3 : ZMod 4)) = 3 := by
  show popcount (1 : ZMod 4).val + popcount (3 : ZMod 4).val = 3
  rw [show (3 : ZMod 4).val = 3 from rfl, show (1 : ZMod 4).val = 1 from rfl, popcount_three,
    popcount_one]
theorem wt_23 : wt 2 ((2 : ZMod 4), (3 : ZMod 4)) = 3 := by
  show popcount (2 : ZMod 4).val + popcount (3 : ZMod 4).val = 3
  rw [show (3 : ZMod 4).val = 3 from rfl, show (2 : ZMod 4).val = 2 from rfl, popcount_three,
    popcount_two]
theorem wt_00 : wt 2 (0 : Expo 2) = 0 := by
  show popcount (0 : ZMod 4).val + popcount (0 : ZMod 4).val = 0
  rw [show (0 : ZMod 4).val = 0 from rfl, popcount_zero]

/-- Coefficient of `a • g + Σ c_i • σ_i g` at a fixed character, expanded over the four shifts. -/
theorem coeff_combo (A B t a : ZMod p) (c : Fin 2 × Bool → ZMod p) (w : Expo 2) :
    (a • gen p 2 A B t + ∑ js, c js • shiftMap p 2 js (gen p 2 A B t)).coeff w =
      a * (gen p 2 A B t).coeff w +
        ∑ js, c js * (shiftMap p 2 js (gen p 2 A B t)).coeff w := by
  simp [coeff_add, coeff_smul, Finsupp.add_apply, Finsupp.smul_apply, Finset.sum_apply']

/-- If an element of `W3` has weight `≤ 2`, it is a multiple of `g`. -/
theorem mem_span_gen_of_mem_W3 (hp : p ≠ 2) (A B t : ZMod p) (hA : A + 3 * t ^ 2 ≠ 0)
    {u : ChainAlg p 2} (hu : u ∈ W3 A B t) (hw : u ∈ weightSub p 2 2) :
    u ∈ Submodule.span (ZMod p) {gen p 2 A B t} := by
  classical
  obtain ⟨a, z, hz, rfl⟩ := Submodule.mem_span_insert.mp hu
  obtain ⟨c, rfl⟩ := (Submodule.mem_span_range_iff_exists_fun (ZMod p)).mp hz
  have two_ne : (2 : ZMod p) ≠ 0 := by
    intro h
    have h2 : ((2 : ℕ) : ZMod p) = 0 := by exact_mod_cast h
    rw [ZMod.natCast_eq_zero_iff] at h2
    rcases (Nat.dvd_prime Nat.prime_two).mp h2 with h1 | h1
    · exact (Fact.out : p.Prime).one_lt.ne' h1
    · exact hp h1
  -- the four weight-3 coefficient equations
  have e31 := coeff_eq_zero_of_mem_weightSub_two hw wt_31
  have e32 := coeff_eq_zero_of_mem_weightSub_two hw wt_32
  have e13 := coeff_eq_zero_of_mem_weightSub_two hw wt_13
  have e23 := coeff_eq_zero_of_mem_weightSub_two hw wt_23
  rw [coeff_combo] at e31 e32 e13 e23
  simp only [Fintype.sum_prod_type, Fin.sum_univ_two, Fintype.sum_bool] at e31 e32 e13 e23
  simp +decide [gen, mono, shiftMap, shiftExpo, LinearMap.mulLeft_apply,
    coeff_add, coeff_single, Finsupp.add_apply, Finsupp.single_apply] at e31 e32 e13 e23
  -- solve the two 2 × 2 systems
  have key : ∀ x y : ZMod p, x * (-2 * t) + y * (-2 * t ^ 2 - 2 * A) = 0 → x + y * (-2 * t) = 0 →
      x = 0 ∧ y = 0 := by
    intro x y h1 h2
    have hx : x = 2 * t * y := by linear_combination h2
    subst hx
    have : y * (-2 * (A + 3 * t ^ 2)) = 0 := by linear_combination h1
    have hy : y = 0 := by
      rcases mul_eq_zero.mp this with h | h
      · exact h
      · exfalso
        rcases mul_eq_zero.mp h with h' | h'
        · exact two_ne (by linear_combination -h')
        · exact hA h'
    exact ⟨by rw [hy]; ring, hy⟩
  obtain ⟨c0f, c1f⟩ := key (c (0, false)) (c (1, false)) (by linear_combination e31)
    (by linear_combination e32)
  obtain ⟨c0t, c1t⟩ := key (c (0, true)) (c (1, true)) (by linear_combination e13)
    (by linear_combination e23)
  have hc : ∀ js, c js = 0 := by
    rintro ⟨j, s⟩
    fin_cases j <;> cases s <;> assumption
  simp only [hc, zero_smul, Finset.sum_const_zero, add_zero]
  exact Submodule.smul_mem _ a (Submodule.subset_span rfl)

/-- `W3` is closed at degree 3. -/
theorem isClosed_W3 (hp : p ≠ 2) (A B t : ZMod p) (hA : A + 3 * t ^ 2 ≠ 0) :
    (chainSystem p 2 A B t).IsClosed 3 (W3 A B t) := by
  refine ⟨Submodule.subset_span (Set.mem_insert _ _), ?_⟩
  intro i u hu _ hw
  obtain ⟨a, rfl⟩ := Submodule.mem_span_singleton.mp (mem_span_gen_of_mem_W3 hp A B t hA hu hw)
  change shiftMap p 2 i (a • gen p 2 A B t) ∈ W3 A B t
  rw [map_smul]
  exact Submodule.smul_mem _ a
    (Submodule.subset_span (Set.mem_insert_of_mem _ ⟨i, rfl⟩))

/-- **`1 ∉ V_3` at depth 2.** -/
theorem one_not_mem_closure_three_depth_two (hp : p ≠ 2) (A B t : ZMod p)
    (hA : A + 3 * t ^ 2 ≠ 0) : (1 : ChainAlg p 2) ∉ (chainSystem p 2 A B t).closure 3 := by
  intro h
  have hW := (chainSystem p 2 A B t).closure_le (isClosed_W3 hp A B t hA) h
  have hw : (1 : ChainAlg p 2) ∈ weightSub p 2 2 := by
    rw [mem_weightSub]
    intro v hv
    have : v = 0 := by
      have := Finsupp.support_single_subset (a := (0 : Expo 2)) (b := (1 : ZMod p))
        (by simpa [one_def] using hv)
      simpa using this
    subst this
    rw [wt_00]; omega
  -- `1` would be a multiple of `g`; `1 ∉ span {g}` is the degree-2 argument
  have hspan := mem_span_gen_of_mem_W3 hp A B t hA hW hw
  have hcl2 : (1 : ChainAlg p 2) ∈ (chainSystem p 2 A B t).closure 2 := by
    obtain ⟨a, ha⟩ := Submodule.mem_span_singleton.mp hspan
    rw [← ha]
    exact Submodule.smul_mem _ a ((chainSystem p 2 A B t).g_mem_closure 2)
  exact one_not_mem_closure_two (le_refl 2) A B t hcl2

end DepthTwo

/-- **Theorem 6 (report §7.1).** For every depth `k ≥ 2`, `p ≠ 2` and `A + 3t² ≠ 0`, the constant
`1` is not in the degree-3 closure.  Transferred from depth 2 by depth monotonicity. -/
theorem one_not_mem_closure_three {k : ℕ} (hk : 2 ≤ k) (hp : p ≠ 2) (A B t : ZMod p)
    (hA : A + 3 * t ^ 2 ≠ 0) : (1 : ChainAlg p k) ∉ (chainSystem p k A B t).closure 3 :=
  fun h => one_not_mem_closure_three_depth_two hp A B t hA (one_mem_closure_of_le p hk A B t 3 h)

/-- **`LFD(k) ≥ 4`** for every depth `k ≥ 2` (whenever the solving degree is finite, e.g. for the
admissible unit family of `Unit.lean`). -/
theorem three_lt_solvingDegree {k : ℕ} (hk : 2 ≤ k) (hp : p ≠ 2) (A B t : ZMod p)
    (hA : A + 3 * t ^ 2 ≠ 0)
    (hfin : ((chainSystem p k A B t).refutationDegrees 1).Nonempty) :
    3 < (chainSystem p k A B t).solvingDegree 1 :=
  (chainSystem p k A B t).lt_solvingDegree_of_not_mem hfin (one_not_mem_closure_three hk hp A B t hA)

/-- Non-vacuity on the report's admissible instances: `A + 3t² ≠ 0` holds for
F1 (`p = 12289`, `A = n²(n − 1) = 1210`, `t = n = 11`) and F2 (`p = 65537`, `A = 18`, `t = 3`).
(For the family, `A + 3t² = n²(n + 2)`.) -/
theorem hyp_F1 : (1210 : ZMod 12289) + 3 * (11 : ZMod 12289) ^ 2 ≠ 0 := by decide
theorem hyp_F2 : (18 : ZMod 65537) + 3 * (3 : ZMod 65537) ^ 2 ≠ 0 := by decide

end ChainSd
