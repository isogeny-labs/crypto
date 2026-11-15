import Mathlib

/-!
# Skeptical audit of the JKSST height/discriminant premise

JKSST Lemma 4.1 assumes, for the lifted curves produced by the algorithm,

`C₁ * maxᵢ hhat(Pᵢ) ≤ log |D|`

with one positive *absolute* constant `C₁`.  The results below distinguish
three logically different claims:

* such a bound cannot hold for every rational point of one fixed curve;
* for one fixed finite family, some instance-dependent positive constant is
  automatic and therefore says little;
* if an absolute `C₁` and a Lang constant `C₃` really are supplied, the ratio
  bound used by JKSST is valid, with the sharp constant `1 / (C₁*C₃)`.

Here `hP` represents the positive canonical height of a nontorsion point and
`logAbsDisc` represents the fixed number `log |D|`.  The only elliptic-curve
input to the counterexample is the standard quadratic identity
`hhat([n]P) = n² hhat(P)`, already expanded in the statement.
-/

namespace JKSSTHeightDiscriminant

/-- For every proposed positive constant and fixed discriminant, a multiple
of a positive-height point violates the proposed upper bound. -/
theorem violating_multiple_exists
    (hP logAbsDisc C₁ : ℝ) (hP_pos : 0 < hP) (C₁_pos : 0 < C₁) :
    ∃ n : ℕ, logAbsDisc < C₁ * ((n : ℝ) ^ 2 * hP) := by
  have product_pos : 0 < C₁ * hP := mul_pos C₁_pos hP_pos
  obtain ⟨n : ℕ, hn⟩ :=
    exists_nat_gt (max 1 (logAbsDisc / (C₁ * hP)))
  have one_lt_n : (1 : ℝ) < n :=
    lt_of_le_of_lt (le_max_left _ _) hn
  have ratio_lt_n : logAbsDisc / (C₁ * hP) < (n : ℝ) :=
    lt_of_le_of_lt (le_max_right _ _) hn
  have n_lt_sq : (n : ℝ) < (n : ℝ) ^ 2 := by
    nlinarith
  have ratio_lt_sq : logAbsDisc / (C₁ * hP) < (n : ℝ) ^ 2 :=
    ratio_lt_n.trans n_lt_sq
  refine ⟨n, ?_⟩
  have scaled := (div_lt_iff₀ product_pos).mp ratio_lt_sq
  calc
    logAbsDisc < (n : ℝ) ^ 2 * (C₁ * hP) := scaled
    _ = C₁ * ((n : ℝ) ^ 2 * hP) := by ring

/-- No positive constant bounds the canonical heights of all multiples by a
fixed discriminant.  This refutes only the universal reading, not JKSST's
restricted hypothesis about the algorithm's selected lifts. -/
theorem no_universal_fixed_discriminant_bound
    (hP logAbsDisc : ℝ) (hP_pos : 0 < hP) :
    ¬ ∃ C₁ : ℝ, 0 < C₁ ∧
      ∀ n : ℕ, C₁ * ((n : ℝ) ^ 2 * hP) ≤ logAbsDisc := by
  rintro ⟨C₁, C₁_pos, bound⟩
  obtain ⟨n, violates⟩ :=
    violating_multiple_exists hP logAbsDisc C₁ hP_pos C₁_pos
  exact (not_lt_of_ge (bound n)) violates

/-- For a single bounded instance, a positive comparison constant always
exists.  Thus the important JKSST assertion is uniformity across all produced
instances, not existence after an instance has been fixed. -/
theorem instance_dependent_constant_exists
    (M logAbsDisc : ℝ) (M_nonneg : 0 ≤ M) (logAbsDisc_pos : 0 < logAbsDisc) :
    ∃ C₁ : ℝ, 0 < C₁ ∧ C₁ * M ≤ logAbsDisc := by
  have denom_pos : 0 < M + 1 := by linarith
  refine ⟨logAbsDisc / (M + 1), div_pos logAbsDisc_pos denom_pos, ?_⟩
  calc
    logAbsDisc / (M + 1) * M ≤
        logAbsDisc / (M + 1) * (M + 1) := by
          apply mul_le_mul_of_nonneg_left
          · linarith
          · exact (div_nonneg logAbsDisc_pos.le denom_pos.le)
    _ = logAbsDisc := by field_simp

/-- The exact ratio implication used in JKSST Lemma 4.1.

`height_vs_disc` is their height/discriminant premise and `lang` is the
Lang-style lower bound.  No elliptic-curve claim is smuggled into this result:
both substantive arithmetic hypotheses occur explicitly as arguments.
-/
theorem height_ratio_bound
    (M m logAbsDisc C₁ C₃ : ℝ)
    (m_pos : 0 < m) (C₁_pos : 0 < C₁) (C₃_pos : 0 < C₃)
    (height_vs_disc : C₁ * M ≤ logAbsDisc)
    (lang : C₃ * logAbsDisc ≤ m) :
    M / m ≤ 1 / (C₁ * C₃) := by
  have product_pos : 0 < C₁ * C₃ := mul_pos C₁_pos C₃_pos
  have product_bound : (C₁ * C₃) * M ≤ m := by
    calc
      (C₁ * C₃) * M = C₃ * (C₁ * M) := by ring
      _ ≤ C₃ * logAbsDisc := mul_le_mul_of_nonneg_left height_vs_disc C₃_pos.le
      _ ≤ m := lang
  apply (div_le_iff₀ m_pos).2
  have product_bound' : M * (C₁ * C₃) ≤ m := by
    simpa [mul_comm] using product_bound
  have divided := (le_div_iff₀ product_pos).2 product_bound'
  calc
    M ≤ m / (C₁ * C₃) := divided
    _ = (1 / (C₁ * C₃)) * m := by ring

/-- The constant in `height_ratio_bound` is sharp using only its two displayed
inequalities: equality is attained when both hypotheses are equalities. -/
theorem height_ratio_bound_is_sharp
    (M C₁ C₃ : ℝ) (M_pos : 0 < M) (C₁_pos : 0 < C₁) (C₃_pos : 0 < C₃) :
    let logAbsDisc := C₁ * M
    let m := C₃ * logAbsDisc
    M / m = 1 / (C₁ * C₃) := by
  dsimp
  field_simp

/-! ## The complexity-relevant uniform quantifiers -/

/-- One pair of constants works over an entire family of produced instances.
This is the quantifier order needed for a uniform complexity conclusion;
allowing `C₁` and `C₃` to depend on the instance is strictly weaker. -/
def UniformInputs {I : Type*}
    (M m logAbsDisc : I → ℝ) (C₁ C₃ : ℝ) : Prop :=
  0 < C₁ ∧ 0 < C₃ ∧
    ∀ i, C₁ * M i ≤ logAbsDisc i ∧ C₃ * logAbsDisc i ≤ m i

/-- Uniform JKSST inputs give the same height-ratio bound at every instance.
No compactness or hidden choice of instance-dependent constants is used. -/
theorem uniform_height_ratio_bound
    {I : Type*} (M m logAbsDisc : I → ℝ) (C₁ C₃ : ℝ)
    (inputs : UniformInputs M m logAbsDisc C₁ C₃)
    (m_pos : ∀ i, 0 < m i) :
    ∀ i, M i / m i ≤ 1 / (C₁ * C₃) := by
  intro i
  exact height_ratio_bound (M i) (m i) (logAbsDisc i) C₁ C₃
    (m_pos i) inputs.1 inputs.2.1 (inputs.2.2 i).1 (inputs.2.2 i).2

end JKSSTHeightDiscriminant
