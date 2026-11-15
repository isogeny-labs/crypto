import Mathlib.Algebra.Polynomial.Roots
import Mathlib.Data.Fintype.BigOperators
import Mathlib.Algebra.MvPolynomial.NoZeroDivisors

/-!
# Counting lemmas for the restricted index-calculus model (RAIC)

Status: `[pcc:unverified]` research scaffolding. Statements are the purely combinatorial or
degree-theoretic pieces of the proof in `PROOF.md`; the cost model, the Wald step and the
interpolation characterisation are not formalised here.

* `k1_root_bound`, `base_abscissas_le` : K1, a base cut out by a univariate `f` has at most
  `natDegree f` abscissas.
* `fiber_card_le_two`, `points_card_le`, `k1_points` : at most two points per abscissa, so
  `|B| ≤ 2 |B_x| ≤ 2 deg f`.
* `sums_card_le` : at most `|B|^m` decomposable targets.
* `surplus_eq_one` : equation surplus of a complete-intersection presentation is exactly 1,
  whatever the number of auxiliary variables.
* `refutation_needs_degree`, `elimination_needs_degree` : base-blindness. A Macaulay certificate
  in which every multiple of a base generator has degree `< d = deg f` cannot refute, and cannot
  produce a nonzero polynomial free of a variable that `S` depends on.
* `amgm_core` : the inequality behind the `m = 2` bound, `(n/b + b)^2 ≥ 4n`.
* `palindromic_half` : a palindromic coefficient sequence has at least half its mass in the
  first half (the median step of Lemma 7: `H(⌊ρ/2⌋) ≥ |W|^m / 2`).
* `master_sq` : the master inequality, squared. With `≥ B` relations, `≥ max(1, n/B^m)` attempts
  per relation and `≥ B^e` per attempt, `e ≥ m/2 - 1` forces total `≥ √n` (model constants aside).
-/

open Polynomial Finset

namespace ICModel

/-- **K1.** A nonzero univariate polynomial vanishes on at most `natDegree f` points of any finset. -/
theorem k1_root_bound {F : Type*} [Field F] [DecidableEq F] {f : F[X]} (hf : f ≠ 0)
    (Z : Finset F) (hZ : ∀ z ∈ Z, f.eval z = 0) : Z.card ≤ f.natDegree := by
  apply Polynomial.card_le_degree_of_subset_roots
  intro z hz
  exact (Polynomial.mem_roots hf).2 (hZ z hz)

/-- K1 for a factor base `B_x = {x : f x = 0 ∧ x is an abscissa}`. -/
theorem base_abscissas_le {F : Type*} [Field F] [Fintype F] [DecidableEq F] {f : F[X]}
    (hf : f ≠ 0) (onCurve : F → Prop) [DecidablePred onCurve] :
    (univ.filter (fun x => f.eval x = 0 ∧ onCurve x)).card ≤ f.natDegree :=
  k1_root_bound hf _ (fun _ hz => (mem_filter.1 hz).2.1)

/-- At most two ordinates `y` with `y ^ 2 = c`. -/
theorem fiber_card_le_two {F : Type*} [Field F] [Fintype F] [DecidableEq F] (c : F) :
    (univ.filter (fun y : F => y ^ 2 = c)).card ≤ 2 := by
  have h0 : (X ^ 2 - C c : F[X]) ≠ 0 := X_pow_sub_C_ne_zero (by norm_num) c
  calc (univ.filter (fun y : F => y ^ 2 = c)).card
      ≤ (X ^ 2 - C c : F[X]).natDegree := by
        apply k1_root_bound h0
        intro y hy
        have hy' : y ^ 2 = c := (mem_filter.1 hy).2
        simp [hy']
    _ = 2 := natDegree_X_pow_sub_C

/-- **|B| ≤ 2 |B_x|.** Affine points `(x, y)` with `x ∈ B_x` and `y ^ 2 = rhs x`. -/
theorem points_card_le {F : Type*} [Field F] [Fintype F] [DecidableEq F] (rhs : F → F)
    (Bx : Finset F) :
    ((Bx ×ˢ (univ : Finset F)).filter (fun q => q.2 ^ 2 = rhs q.1)).card ≤ 2 * Bx.card := by
  rw [card_filter, sum_product]
  calc ∑ x ∈ Bx, ∑ y ∈ (univ : Finset F), (if y ^ 2 = rhs x then 1 else 0)
      = ∑ x ∈ Bx, (univ.filter (fun y : F => y ^ 2 = rhs x)).card := by
        refine sum_congr rfl (fun x _ => ?_)
        rw [card_filter]
    _ ≤ ∑ _x ∈ Bx, 2 := sum_le_sum (fun x _ => fiber_card_le_two (rhs x))
    _ = 2 * Bx.card := by rw [sum_const, smul_eq_mul, mul_comm]

/-- **K1 for points.** A base cut out by `f` has at most `2 deg f` affine points. -/
theorem k1_points {F : Type*} [Field F] [Fintype F] [DecidableEq F] {f : F[X]} (hf : f ≠ 0)
    (rhs : F → F) :
    (((univ.filter (fun x => f.eval x = 0)) ×ˢ (univ : Finset F)).filter
        (fun q => q.2 ^ 2 = rhs q.1)).card ≤ 2 * f.natDegree :=
  (points_card_le rhs _).trans
    (Nat.mul_le_mul_left 2 (k1_root_bound hf _ (fun _ hz => (mem_filter.1 hz).2)))

/-- **Decomposable targets.** The `m`-fold sums of base elements number at most `|B| ^ m`. -/
theorem sums_card_le {G : Type*} [AddCommMonoid G] [DecidableEq G] (B : Finset G) (m : ℕ) :
    ((Fintype.piFinset fun _ : Fin m => B).image (fun v => ∑ i, v i)).card ≤ B.card ^ m := by
  refine card_image_le.trans ?_
  rw [Fintype.card_piFinset_const]

/-- **Equation surplus.** `m (1 + s) + 1` equations (one summation polynomial plus `1 + s` base
equations per summand) in `m (1 + s)` unknowns: the surplus is exactly `1` for every number `s` of
auxiliary variables. -/
theorem surplus_eq_one (m s : ℕ) : (m * (1 + s) + 1) - m * (1 + s) = 1 := by omega

open MvPolynomial in
/-- **Base-blindness, refutation.** If `1 = u₀ S + ∑ uᵢ fᵢ` where each base generator `fᵢ` has
total degree `d` and each product `uᵢ fᵢ` has total degree `≤ D < d`, then all `uᵢ = 0` and `S`
is a constant. So a nonconstant summation polynomial cannot be refuted below degree `d`. -/
theorem refutation_needs_degree {σ k ι : Type*} [Field k] [Fintype ι]
    (S u₀ : MvPolynomial σ k) (u f : ι → MvPolynomial σ k) (d D : ℕ)
    (hf : ∀ i, (f i).totalDegree = d) (hf0 : ∀ i, f i ≠ 0)
    (hdeg : ∀ i, (u i * f i).totalDegree ≤ D) (hD : D < d)
    (hcert : (1 : MvPolynomial σ k) = u₀ * S + ∑ i, u i * f i) :
    S.totalDegree = 0 := by
  have hu : ∀ i, u i = 0 := by
    intro i
    by_contra h
    have h1 := totalDegree_mul_of_isDomain h (hf0 i)
    have h2 := hdeg i
    rw [hf i] at h1
    omega
  have h1 : (1 : MvPolynomial σ k) = u₀ * S := by simpa [hu] using hcert
  have hu0 : u₀ ≠ 0 := by
    rintro rfl
    simp at h1
  have hS : S ≠ 0 := by
    rintro rfl
    simp at h1
  have h2 := totalDegree_mul_of_isDomain hu0 hS
  rw [← h1, totalDegree_one] at h2
  omega

open MvPolynomial in
/-- **Base-blindness, elimination.** If a nonzero `g` free of the variable `j` (for instance a
univariate eliminant in `x₁`) is `u₀ S + ∑ uᵢ fᵢ` with every `uᵢ fᵢ` of total degree `≤ D < d`,
then `S` is free of `j` as well. Semaev polynomials involve every variable, so no eliminant
appears below degree `d`. -/
theorem elimination_needs_degree {σ k ι : Type*} [Field k] [Fintype ι]
    (S u₀ g : MvPolynomial σ k) (u f : ι → MvPolynomial σ k) (d D : ℕ) (j : σ)
    (hf : ∀ i, (f i).totalDegree = d) (hf0 : ∀ i, f i ≠ 0)
    (hdeg : ∀ i, (u i * f i).totalDegree ≤ D) (hD : D < d)
    (hg : g ≠ 0) (hgj : g.degreeOf j = 0)
    (hcert : g = u₀ * S + ∑ i, u i * f i) :
    S.degreeOf j = 0 := by
  have hu : ∀ i, u i = 0 := by
    intro i
    by_contra h
    have h1 := totalDegree_mul_of_isDomain h (hf0 i)
    have h2 := hdeg i
    rw [hf i] at h1
    omega
  have h1 : g = u₀ * S := by simpa [hu] using hcert
  have hu0 : u₀ ≠ 0 := by
    rintro rfl
    exact hg (by simpa using h1)
  have hS : S ≠ 0 := by
    rintro rfl
    exact hg (by simpa using h1)
  have h2 := degreeOf_mul_eq (n := j) hu0 hS
  rw [← h1, hgj] at h2
  omega

/-- **Core of the `m = 2` bound.** For `b > 0`, `(n / b + b) ^ 2 ≥ 4 n`, so the relation phase
(`≥ c n / |B|` attempts) plus the relations themselves (`≥ c |B|`) cost at least `2 c √n`. -/
theorem amgm_core (n b : ℚ) (hb : 0 < b) : 4 * n ≤ (n / b + b) ^ 2 := by
  have hnb : n / b * b = n := div_mul_cancel₀ n (ne_of_gt hb)
  have h : (n / b + b) ^ 2 = (n / b - b) ^ 2 + 4 * (n / b * b) := by ring
  rw [h, hnb]
  exact le_add_of_nonneg_left (sq_nonneg _)

/-- **Master inequality (squared form).** Per relation the model pays at least
`max(1, n / B^m)` attempts, needs at least `B` relations, and each attempt costs at least `B^e`
with `e = E2 / 2`.  If `E2 + 2 ≥ m` (that is `e ≥ m/2 - 1`) then
`(max B (n / B^(m-1)))^2 * B^E2 ≥ n`, i.e. total `≥ √n` up to the model constants. -/
theorem master_sq (B n : ℚ) (m E2 : ℕ) (hB : 1 ≤ B) (hn : 0 < n) (hm : 1 ≤ m)
    (hE : m ≤ E2 + 2) : n ≤ (max B (n / B ^ (m - 1))) ^ 2 * B ^ E2 := by
  have hB0 : 0 < B := lt_of_lt_of_le one_pos hB
  have hBE : 0 ≤ B ^ E2 := pow_nonneg hB0.le _
  rcases le_or_gt n (B ^ m) with h | h
  · calc n ≤ B ^ m := h
      _ ≤ B ^ (2 + E2) := pow_le_pow_right₀ hB (by omega)
      _ = B ^ 2 * B ^ E2 := pow_add B 2 E2
      _ ≤ (max B (n / B ^ (m - 1))) ^ 2 * B ^ E2 :=
          mul_le_mul_of_nonneg_right (pow_le_pow_left₀ hB0.le (le_max_left _ _) 2) hBE
  · have hP : 0 < B ^ (m - 1) := pow_pos hB0 _
    have key : (B ^ (m - 1)) ^ 2 ≤ n * B ^ E2 := by
      calc (B ^ (m - 1)) ^ 2 = B ^ (2 * (m - 1)) := by rw [← pow_mul, mul_comm]
        _ ≤ B ^ (m + E2) := pow_le_pow_right₀ hB (by omega)
        _ = B ^ m * B ^ E2 := pow_add B m E2
        _ ≤ n * B ^ E2 := mul_le_mul_of_nonneg_right h.le hBE
    have hY : n / B ^ (m - 1) ≤ max B (n / B ^ (m - 1)) := le_max_right _ _
    have hY0 : 0 ≤ n / B ^ (m - 1) := div_nonneg hn.le hP.le
    have hYP : n / B ^ (m - 1) * B ^ (m - 1) = n := div_mul_cancel₀ n hP.ne'
    have h1 : n * n ≤ (n / B ^ (m - 1)) ^ 2 * B ^ E2 * n := by
      calc n * n = (n / B ^ (m - 1) * B ^ (m - 1)) ^ 2 := by rw [hYP]; ring
        _ = (n / B ^ (m - 1)) ^ 2 * (B ^ (m - 1)) ^ 2 := by ring
        _ ≤ (n / B ^ (m - 1)) ^ 2 * (n * B ^ E2) := mul_le_mul_of_nonneg_left key (sq_nonneg _)
        _ = (n / B ^ (m - 1)) ^ 2 * B ^ E2 * n := by ring
    have h2 : n ≤ (n / B ^ (m - 1)) ^ 2 * B ^ E2 := le_of_mul_le_mul_right h1 hn
    exact h2.trans (mul_le_mul_of_nonneg_right (pow_le_pow_left₀ hY0 hY 2) hBE)

/-- **Median of a palindromic h-vector.** If `c j = c (R - j)` for `j ≤ R`, then the first half
(indices `≤ R/2`) carries at least half the total mass: `H(⌊ρ/2⌋) ≥ |W|^m / 2` in Lemma 7. -/
theorem palindromic_half (c : ℕ → ℕ) (R : ℕ) (hsym : ∀ j ≤ R, c j = c (R - j)) :
    ∑ j ∈ range (R + 1), c j ≤ 2 * ∑ j ∈ range (R / 2 + 1), c j := by
  have hsplit : ∑ j ∈ range (R / 2 + 1), c j + ∑ j ∈ Ico (R / 2 + 1) (R + 1), c j
      = ∑ j ∈ range (R + 1), c j := sum_range_add_sum_Ico c (by omega)
  have htail : ∑ j ∈ Ico (R / 2 + 1) (R + 1), c j ≤ ∑ j ∈ range (R / 2 + 1), c j := by
    calc ∑ j ∈ Ico (R / 2 + 1) (R + 1), c j
        = ∑ j ∈ Ico (R / 2 + 1) (R + 1), c (R - j) := by
          refine sum_congr rfl (fun j hj => ?_)
          have := (mem_Ico.1 hj).2
          exact hsym j (by omega)
      _ = ∑ i ∈ (Ico (R / 2 + 1) (R + 1)).image (fun j => R - j), c i := by
          rw [sum_image]
          intro x hx y hy hxy
          have hx' := (mem_Ico.1 hx).2
          have hy' := (mem_Ico.1 hy).2
          simp only at hxy
          omega
      _ ≤ ∑ i ∈ range (R / 2 + 1), c i := by
          apply sum_le_sum_of_subset
          intro i hi
          obtain ⟨j, hj, rfl⟩ := mem_image.1 hi
          have := (mem_Ico.1 hj).1
          rw [mem_range]
          omega
  omega

end ICModel

#print axioms ICModel.k1_root_bound
#print axioms ICModel.base_abscissas_le
#print axioms ICModel.fiber_card_le_two
#print axioms ICModel.points_card_le
#print axioms ICModel.k1_points
#print axioms ICModel.sums_card_le
#print axioms ICModel.surplus_eq_one
#print axioms ICModel.refutation_needs_degree
#print axioms ICModel.elimination_needs_degree
#print axioms ICModel.amgm_core
#print axioms ICModel.master_sq
#print axioms ICModel.palindromic_half
