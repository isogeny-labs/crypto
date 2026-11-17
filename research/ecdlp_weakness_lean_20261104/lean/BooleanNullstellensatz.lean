import Mathlib

/-!
# Boolean Nullstellensatz: a unique solution puts every `X i - z i` in the ideal

Corpus target: the bridge `k ≥ N - affdim(Z)` used by the `pq2_rank` census
(`GAP-PQ2-M2-RANK-CENSUS`), in its sharpest case `|Z| = 1`.

Over `F_2`, for a finite family `F` of polynomials with the field equations
`X i ^ 2 - X i` adjoined, if the common zero set of `F` in `F_2^σ` is exactly
`{z}`, then `X i - C (z i)` lies in the ideal for every variable `i`. So the
ideal contains `|σ|` linear polynomials with distinct leading variables, which
is the hypothesis `k = N` of `SolvingDegreeBound.solving_degree_ge_three` and
of `SubspaceCriterion.no_gb_in_rows_of_low_lt`.

Steps:
1. `X_pow_sub_X_mem`: `X i ^ e - X i` is in the field-equation ideal `J` for `e ≥ 1`.
2. `reduce_mem`: every polynomial is congruent mod `J` to one of degree `≤ 1` in
   each variable (`restrictDegree σ K 1`).
3. `mem_fieldIdeal_of_eval_zero`: a polynomial vanishing on all of `F_2^σ` lies in
   `J` (Mathlib's `eq_zero_of_eval_eq_zero` on the reduced form).
4. `indicator_mem`: for `w ≠ z` the indicator polynomial of `w` lies in the ideal.
5. `linear_mem_of_unique_zero`: the conclusion.
-/

open MvPolynomial
open scoped BigOperators

namespace BooleanNullstellensatz

variable {σ : Type}

local notation "K" => ZMod 2

/-- The field-equation ideal `⟨X i ^ 2 - X i⟩`. -/
noncomputable def fieldIdeal (σ : Type*) : Ideal (MvPolynomial σ K) :=
  Ideal.span (Set.range fun i : σ => X i ^ 2 - X i)

theorem X_sq_sub_X_mem (i : σ) : X i ^ 2 - X i ∈ fieldIdeal σ :=
  Ideal.subset_span ⟨i, rfl⟩

theorem X_pow_sub_X_mem (i : σ) : ∀ e : ℕ, 1 ≤ e → X i ^ e - X i ∈ fieldIdeal σ := by
  intro e he
  induction e with
  | zero => omega
  | succ e ih =>
    rcases Nat.eq_zero_or_pos e with h | h
    · subst h; simp
    · have h1 := ih h
      have h2 := X_sq_sub_X_mem i
      have : (X i : MvPolynomial σ K) ^ (e + 1) - X i = X i * (X i ^ e - X i) + (X i ^ 2 - X i) := by ring
      rw [this]
      exact Ideal.add_mem _ (Ideal.mul_mem_left _ _ h1) h2

theorem X_pow_sub_X_pow_min_mem (i : σ) (e : ℕ) :
    X i ^ e - X i ^ (min e 1) ∈ fieldIdeal σ := by
  rcases Nat.eq_zero_or_pos e with h | h
  · subst h; simp
  · have : min e 1 = 1 := by omega
    rw [this, pow_one]
    exact X_pow_sub_X_mem i e h

/-- Products of congruent factors are congruent. -/
theorem prod_sub_prod_mem (J : Ideal (MvPolynomial σ K)) (s : Finset σ)
    (a b : σ → MvPolynomial σ K) (h : ∀ i ∈ s, a i - b i ∈ J) :
    (∏ i ∈ s, a i) - ∏ i ∈ s, b i ∈ J := by
  classical
  induction s using Finset.induction_on with
  | empty => simp
  | insert j s hj ih =>
    rw [Finset.prod_insert hj, Finset.prod_insert hj]
    have ih' := ih (fun i hi => h i (Finset.mem_insert_of_mem hi))
    have hj' := h j (Finset.mem_insert_self j s)
    have : a j * ∏ i ∈ s, a i - b j * ∏ i ∈ s, b i
        = a j * ((∏ i ∈ s, a i) - ∏ i ∈ s, b i) + (a j - b j) * ∏ i ∈ s, b i := by ring
    rw [this]
    exact Ideal.add_mem _ (Ideal.mul_mem_left _ _ ih') (Ideal.mul_mem_right _ _ hj')

/-- Exponent reduction `e ↦ min e 1`. -/
noncomputable def red (m : σ →₀ ℕ) : σ →₀ ℕ := Finsupp.mapRange (fun e => min e 1) (by simp) m

theorem red_apply (m : σ →₀ ℕ) (i : σ) : red m i = min (m i) 1 := by
  simp [red]

theorem monomial_sub_monomial_red_mem (m : σ →₀ ℕ) (c : K) :
    monomial m c - monomial (red m) c ∈ fieldIdeal σ := by
  classical
  rw [monomial_eq, monomial_eq]
  have hsub : (red m).support ⊆ m.support := Finsupp.support_mapRange
  rw [Finsupp.prod_of_support_subset (red m) hsub (fun i e => X i ^ e) (fun _ _ => pow_zero _)]
  rw [Finsupp.prod]
  rw [← mul_sub]
  apply Ideal.mul_mem_left
  apply prod_sub_prod_mem
  intro i _
  rw [red_apply]
  exact X_pow_sub_X_pow_min_mem i (m i)

/-- Every polynomial is congruent mod the field equations to a multilinear one. -/
theorem reduce_mem (p : MvPolynomial σ K) :
    ∃ p' ∈ restrictDegree σ K 1, p - p' ∈ fieldIdeal σ := by
  classical
  refine ⟨∑ v ∈ p.support, monomial (red v) (p.coeff v), ?_, ?_⟩
  · apply Submodule.sum_mem
    intro v _
    rw [mem_restrictDegree]
    intro s hs i
    rw [support_monomial] at hs
    split_ifs at hs with h
    · simp at hs
    · rw [Finset.mem_singleton] at hs
      subst hs
      rw [red_apply]
      omega
  · have : p - ∑ v ∈ p.support, monomial (red v) (p.coeff v)
        = ∑ v ∈ p.support, (monomial v (p.coeff v) - monomial (red v) (p.coeff v)) := by
      rw [Finset.sum_sub_distrib, ← as_sum p]
    rw [this]
    apply Ideal.sum_mem
    intro v _
    exact monomial_sub_monomial_red_mem v (p.coeff v)

theorem eval_eq_zero_of_mem_fieldIdeal (v : σ → K) {p : MvPolynomial σ K}
    (hp : p ∈ fieldIdeal σ) : eval v p = 0 := by
  have hle : fieldIdeal σ ≤ RingHom.ker (eval v) := by
    rw [fieldIdeal, Ideal.span_le]
    rintro _ ⟨i, rfl⟩
    simp only [SetLike.mem_coe, RingHom.mem_ker, map_sub, map_pow, eval_X]
    rw [ZMod.pow_card (v i)]
    exact sub_self _
  exact hle hp

/-- Boolean Nullstellensatz, weak form: vanishing on all of `F_2^σ` means lying
in the field-equation ideal. -/
theorem mem_fieldIdeal_of_eval_zero [Fintype σ] (p : MvPolynomial σ K)
    (h : ∀ v : σ → K, eval v p = 0) : p ∈ fieldIdeal σ := by
  obtain ⟨p', hp', hdiff⟩ := reduce_mem p
  have hp'0 : p' = 0 := by
    apply MvPolynomial.eq_zero_of_eval_eq_zero _ _ p'
    · intro v
      have h1 : eval v (p - p') = 0 := eval_eq_zero_of_mem_fieldIdeal v hdiff
      rw [map_sub, h v] at h1
      linear_combination -h1
    · simpa [ZMod.card] using hp'
  rw [hp'0, sub_zero] at hdiff
  exact hdiff

/-- The generating ideal of the census: `F` together with the field equations. -/
noncomputable def sysIdeal (F : Finset (MvPolynomial σ K)) : Ideal (MvPolynomial σ K) :=
  Ideal.span ((F : Set (MvPolynomial σ K)) ∪ Set.range fun i : σ => X i ^ 2 - X i)

theorem fieldIdeal_le_sysIdeal (F : Finset (MvPolynomial σ K)) : fieldIdeal σ ≤ sysIdeal F :=
  Ideal.span_mono Set.subset_union_right

theorem mem_sysIdeal_of_mem (F : Finset (MvPolynomial σ K)) {f : MvPolynomial σ K} (hf : f ∈ F) :
    f ∈ sysIdeal F :=
  Ideal.subset_span (Set.mem_union_left _ hf)

/-- For a point `w` that is not a common zero, its indicator polynomial is in the ideal. -/
theorem indicator_mem [Fintype σ] [DecidableEq σ] (F : Finset (MvPolynomial σ K)) (w : σ → K)
    (hw : ∃ f ∈ F, eval w f ≠ 0) : indicator w ∈ sysIdeal F := by
  classical
  obtain ⟨f, hfF, hfw⟩ := hw
  set q := C (eval w f)⁻¹ * f * indicator w - indicator w with hq
  have hqJ : q ∈ fieldIdeal σ := by
    apply mem_fieldIdeal_of_eval_zero
    intro u
    simp only [hq, map_sub, map_mul, eval_C]
    by_cases hu : u = w
    · subst hu
      rw [eval_indicator_apply_eq_one, inv_mul_cancel₀ hfw]
      ring
    · rw [eval_indicator_apply_eq_zero u w hu]
      ring
  have h1 : C (eval w f)⁻¹ * f * indicator w ∈ sysIdeal F :=
    Ideal.mul_mem_right _ _ (Ideal.mul_mem_left _ _ (mem_sysIdeal_of_mem F hfF))
  have : indicator w = C (eval w f)⁻¹ * f * indicator w - q := by rw [hq]; ring
  rw [this]
  exact Ideal.sub_mem _ h1 (fieldIdeal_le_sysIdeal F hqJ)

/-- **Unique solution ⟹ linear consequences.** If `z` is the only common zero of
`F` in `F_2^σ`, then `X i - C (z i)` lies in the ideal generated by `F` and the
field equations, for every `i`. -/
theorem linear_mem_of_unique_zero [Fintype σ] [DecidableEq σ] (F : Finset (MvPolynomial σ K)) (z : σ → K)
    (huniq : ∀ w : σ → K, (∀ f ∈ F, eval w f = 0) → w = z) (i : σ) :
    X i - C (z i) ∈ sysIdeal F := by
  classical
  let S : Finset (σ → K) := Finset.univ.filter fun w => w ≠ z
  let s : MvPolynomial σ K := ∑ w ∈ S, C (w i - z i) * indicator w
  have hsI : s ∈ sysIdeal F := by
    apply Ideal.sum_mem
    intro w hw
    have hwz : w ≠ z := (Finset.mem_filter.mp hw).2
    have : ∃ f ∈ F, eval w f ≠ 0 := by
      by_contra hcon
      push Not at hcon
      exact hwz (huniq w hcon)
    exact Ideal.mul_mem_left _ _ (indicator_mem F w this)
  have hr : X i - C (z i) - s ∈ fieldIdeal σ := by
    apply mem_fieldIdeal_of_eval_zero
    intro u
    have hs : eval u s = if u = z then 0 else u i - z i := by
      simp only [s, map_sum, map_mul, eval_C]
      rw [Finset.sum_eq_single u]
      · by_cases hu : u = z
        · simp [hu]
        · simp [hu, eval_indicator_apply_eq_one]
      · intro w _ hwu
        rw [eval_indicator_apply_eq_zero u w (Ne.symm hwu)]
        ring
      · intro hu
        simp [S] at hu
        simp [hu]
    simp only [map_sub, eval_X, eval_C, hs]
    split_ifs with hu
    · subst hu; ring
    · ring
  have : X i - C (z i) = (X i - C (z i) - s) + s := by ring
  rw [this]
  exact Ideal.add_mem _ (fieldIdeal_le_sysIdeal F hr) hsI

end BooleanNullstellensatz
