import Mathlib
import ChainSd.GroupAlgebra

/-!
# The constant lower bound at degree 2: `1 ∉ V_2`, so `LFD(k) ≥ 3` for `k ≥ 2`

At degree 2 no shift is admitted to an element of `span {g}` unless it is zero, because the
coefficient of `g` at the weight-2 character `x₁² x₂²` is `1`.  Hence `V_2 = span {g}`, and `1`
is not a multiple of `g` (compare the coefficients at `x₁² x₂²`).  This holds for every
`(A, B, t)` and every depth `k ≥ 2` (for `k ≥ 2` the exponents `0, 1, 2` are distinct mod `2^k`).
The sharper bound `LFD(k) ≥ 4` (Theorem 6 of the report) is a paper proof.
-/

namespace ChainSd

open AddMonoidAlgebra

variable {p k : ℕ} [Fact p.Prime]

theorem natCast_ne_of_two_le (hk : 2 ≤ k) {a b : ℕ} (ha : a < 4) (hb : b < 4) (hab : a ≠ b) :
    ((a : ℕ) : ZMod (2 ^ k)) ≠ ((b : ℕ) : ZMod (2 ^ k)) := by
  intro h
  have h4 : 4 ≤ 2 ^ k := by
    calc 4 = 2 ^ 2 := by norm_num
      _ ≤ 2 ^ k := Nat.pow_le_pow_right (by norm_num) hk
  rw [ZMod.natCast_eq_natCast_iff', Nat.mod_eq_of_lt (by omega), Nat.mod_eq_of_lt (by omega)] at h
  exact hab h

/-- The coefficient of `g` at `x₁² x₂²` is `1` (depth `k ≥ 2`). -/
theorem gen_coeff_22 (hk : 2 ≤ k) (A B t : ZMod p) :
    (gen p k A B t).coeff (((2 : ℕ) : ZMod (2 ^ k)), ((2 : ℕ) : ZMod (2 ^ k))) = 1 := by
  classical
  have ne : ∀ a b : ℕ, a < 3 → b < 3 → (a ≠ 2 ∨ b ≠ 2) →
      (((a : ℕ) : ZMod (2 ^ k)), ((b : ℕ) : ZMod (2 ^ k))) ≠
        (((2 : ℕ) : ZMod (2 ^ k)), ((2 : ℕ) : ZMod (2 ^ k))) := by
    intro a b ha hb hab h
    rw [Prod.mk.injEq] at h
    rcases hab with h' | h'
    · exact natCast_ne_of_two_le hk (by omega) (by omega) h' h.1
    · exact natCast_ne_of_two_le hk (by omega) (by omega) h' h.2
  simp only [gen, mono, coeff_add, coeff_single, Finsupp.add_apply, Finsupp.single_apply]
  rw [if_pos trivial]
  rw [if_neg (ne 2 1 (by omega) (by omega) (by omega)), if_neg (ne 1 2 (by omega) (by omega) (by omega)),
    if_neg (ne 2 0 (by omega) (by omega) (by omega)), if_neg (ne 0 2 (by omega) (by omega) (by omega)),
    if_neg (ne 1 1 (by omega) (by omega) (by omega)), if_neg (ne 1 0 (by omega) (by omega) (by omega)),
    if_neg (ne 0 1 (by omega) (by omega) (by omega)), if_neg (ne 0 0 (by omega) (by omega) (by omega))]
  ring

theorem wt_22 (hk : 2 ≤ k) : wt k (((2 : ℕ) : ZMod (2 ^ k)), ((2 : ℕ) : ZMod (2 ^ k))) = 2 := by
  have h4 : 2 < 2 ^ k := by
    calc 2 < 4 := by norm_num
      _ = 2 ^ 2 := by norm_num
      _ ≤ 2 ^ k := Nat.pow_le_pow_right (by norm_num) hk
  unfold wt bitWeight
  rw [ZMod.val_natCast, Nat.mod_eq_of_lt h4]
  have : popcount 2 = 1 := by
    rw [popcount_eq, popcount_eq 1, popcount_eq 0]; simp [popcount_zero]
  rw [this]

/-- `span {g}` is closed at degree 2. -/
theorem isClosed_span_gen (hk : 2 ≤ k) (A B t : ZMod p) :
    (chainSystem p k A B t).IsClosed 2 (Submodule.span (ZMod p) {gen p k A B t}) := by
  refine ⟨Submodule.subset_span rfl, ?_⟩
  intro i u hu _ hw
  obtain ⟨c, rfl⟩ := Submodule.mem_span_singleton.mp hu
  -- the weight-≤1 condition forces c = 0
  have hc : c = 0 := by
    change c • gen p k A B t ∈ weightSub p k 1 at hw
    rw [mem_weightSub] at hw
    by_contra hc0
    have hmem : (((2 : ℕ) : ZMod (2 ^ k)), ((2 : ℕ) : ZMod (2 ^ k))) ∈ (c • gen p k A B t).coeff.support := by
      rw [Finsupp.mem_support_iff, coeff_smul, Finsupp.smul_apply, gen_coeff_22 hk, smul_eq_mul, mul_one]
      exact hc0
    have := hw _ hmem
    rw [wt_22 hk] at this
    omega
  rw [hc, zero_smul, map_zero]
  exact Submodule.zero_mem _

/-- **`1 ∉ V_2`** for every depth `k ≥ 2` and every `(A, B, t)`; hence `LFD(k) ≥ 3`. -/
theorem one_not_mem_closure_two (hk : 2 ≤ k) (A B t : ZMod p) :
    (1 : ChainAlg p k) ∉ (chainSystem p k A B t).closure 2 := by
  intro h
  have h' := (chainSystem p k A B t).closure_le (isClosed_span_gen hk A B t) h
  obtain ⟨c, hc⟩ := Submodule.mem_span_singleton.mp h'
  have e := congrArg (fun u : ChainAlg p k => u.coeff (((2 : ℕ) : ZMod (2 ^ k)), ((2 : ℕ) : ZMod (2 ^ k)))) hc
  simp only [coeff_smul, Finsupp.smul_apply, gen_coeff_22 hk, smul_eq_mul, mul_one] at e
  -- coefficient of 1 at (2,2) is 0
  have h0 : (1 : ChainAlg p k).coeff (((2 : ℕ) : ZMod (2 ^ k)), ((2 : ℕ) : ZMod (2 ^ k))) = 0 := by
    classical
    rw [one_def, coeff_single, Finsupp.single_apply, if_neg]
    intro h00
    rw [Prod.ext_iff] at h00
    exact natCast_ne_of_two_le hk (a := 0) (b := 2) (by omega) (by omega) (by omega)
      (by simpa using h00.1)
  rw [h0] at e
  -- so c = 0 and 1 = 0
  rw [e, zero_smul] at hc
  exact one_ne_zero hc.symm

theorem two_lt_solvingDegree (hk : 2 ≤ k) (A B t : ZMod p)
    (hfin : ((chainSystem p k A B t).refutationDegrees 1).Nonempty) :
    2 < (chainSystem p k A B t).solvingDegree 1 :=
  (chainSystem p k A B t).lt_solvingDegree_of_not_mem hfin (one_not_mem_closure_two hk A B t)

end ChainSd
