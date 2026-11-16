import Mathlib

/-!
# What the Lang-height input to JKSST does and does not prove

Jacobson--Koblitz--Silverman--Stein--Teske, *Analysis of the Xedni
Calculus Attack*, DCC 20 (2000), Lemma 4.1, invokes Lang's conjectural
uniform lower bound for the least positive canonical height in terms of the
discriminant.  The paper explicitly labels this input a conjecture and notes
that it was then known only for a large class of curves.

This file does **not** claim to settle that open arithmetic conjecture.
Instead it proves:

* quadraticity and positive-definiteness alone cannot imply a uniform Lang
  constant across a family (`no_uniform_lower_bound_from_quadraticity`);
* the least positive height in a scaled rank-one lattice is exactly its scale;
* the direct height-gap hypothesis actually needed by the JKSST lattice
  argument, and the algebraic implication from the two JKSST inequalities to
  that direct gap and to an explicit ratio bound.

Thus the conjectural arithmetic input is isolated behind a precise interface,
rather than silently treated as a proved theorem.
-/

namespace JKSSTLang

/-- A family of positive-definite quadratic heights on the rank-one lattice
`ℤ`.  Scaling preserves every formal quadratic-height identity. -/
def scaledQuadraticHeight (scale : ℝ) (z : ℤ) : ℝ :=
  scale * (z : ℝ) ^ 2

/-- The canonical-height quadratic law holds for every scale, including
arbitrarily small positive scales. -/
theorem scaledQuadraticHeight_mul
    (scale : ℝ) (n z : ℤ) :
    scaledQuadraticHeight scale (n * z) =
      (n : ℝ) ^ 2 * scaledQuadraticHeight scale z := by
  simp only [scaledQuadraticHeight, Int.cast_mul]
  ring

/-- At positive scale the model height is positive on every nonzero lattice
point. -/
theorem scaledQuadraticHeight_pos
    {scale : ℝ} (scale_pos : 0 < scale) {z : ℤ} (z_ne : z ≠ 0) :
    0 < scaledQuadraticHeight scale z := by
  unfold scaledQuadraticHeight
  positivity

/-- The least positive height of the scaled rank-one lattice is exactly its
scale. -/
theorem scaled_rank_one_lower_bound
    {scale : ℝ} (scale_pos : 0 < scale) {z : ℤ} (z_ne : z ≠ 0) :
    scale ≤ scaledQuadraticHeight scale z := by
  have int_abs_ge : (1 : ℤ) ≤ |z| := Int.one_le_abs z_ne
  have real_abs_ge : (1 : ℝ) ≤ |(z : ℝ)| := by
    exact_mod_cast int_abs_ge
  have square_ge : (1 : ℝ) ≤ (z : ℝ) ^ 2 := by
    nlinarith [sq_abs (z : ℝ)]
  unfold scaledQuadraticHeight
  nlinarith [scale_pos]

theorem scaled_rank_one_attains_lower_bound (scale : ℝ) :
    scaledQuadraticHeight scale 1 = scale := by
  simp [scaledQuadraticHeight]

/-- **Countermodel to an overbroad Lang claim.**

There is no positive constant bounding from below every nonzero point in every
positive scaling of a quadratic rank-one lattice.  Hence quadraticity,
positive-definiteness, and finite rank do not by themselves imply Lang's
*uniform-across-curves* constant.
-/
theorem no_uniform_lower_bound_from_quadraticity :
    ¬ ∃ C : ℝ, 0 < C ∧
      ∀ scale : ℝ, 0 < scale →
        ∀ z : ℤ, z ≠ 0 → C ≤ scaledQuadraticHeight scale z := by
  rintro ⟨C, C_pos, lower⟩
  have scale_pos : 0 < C / 2 := by linarith
  have contrad := lower (C / 2) scale_pos 1 (by norm_num)
  simp only [scaledQuadraticHeight, Int.cast_one, one_pow, mul_one] at contrad
  linarith

/-- Every individual scaled rank-one lattice has an instance-specific positive
Lang-style constant relative to any positive discriminant parameter.  The
constant is `scale / discLog`; it is not uniform in a family. -/
theorem rank_one_has_instance_specific_constant
    {scale discLog : ℝ} (scale_pos : 0 < scale) (discLog_pos : 0 < discLog) :
    ∃ C : ℝ, 0 < C ∧
      ∀ z : ℤ, z ≠ 0 →
        C * discLog ≤ scaledQuadraticHeight scale z := by
  refine ⟨scale / discLog, div_pos scale_pos discLog_pos, ?_⟩
  intro z z_ne
  have lower := scaled_rank_one_lower_bound scale_pos z_ne
  have cancel : (scale / discLog) * discLog = scale := by
    field_simp
  rwa [cancel]

/-- The direct quantitative substitute actually consumed by the bounded
relation argument: the minimum positive height `m` is at least `gap * M`,
where `M` bounds the input-point heights. -/
def DirectHeightGap (M m gap : ℝ) : Prop :=
  gap * M ≤ m

/-- A direct positive height gap gives the explicit ratio bound used in the
geometry-of-numbers step. -/
theorem ratio_bound_of_direct_height_gap
    {M m gap : ℝ}
    (m_pos : 0 < m) (gap_pos : 0 < gap)
    (height_gap : DirectHeightGap M m gap) :
    M / m ≤ 1 / gap := by
  have gap_reordered : M * gap ≤ m := by
    simpa [DirectHeightGap, mul_comm] using height_gap
  have M_le : M ≤ m / gap := (le_div_iff₀ gap_pos).2 gap_reordered
  apply (div_le_iff₀ m_pos).2
  calc
    M ≤ m / gap := M_le
    _ = (1 / gap) * m := by ring

/-- The two JKSST hypotheses imply the direct gap with the exact product
constant `C₁*C₃`:

* `C₁*M ≤ log|D|` is the height/discriminant premise;
* `C₃*log|D| ≤ m` is the Lang input.
-/
theorem direct_gap_of_jksst_inputs
    {M m logAbsDisc C₁ C₃ : ℝ}
    (C₃_nonneg : 0 ≤ C₃)
    (height_discriminant : C₁ * M ≤ logAbsDisc)
    (lang_lower : C₃ * logAbsDisc ≤ m) :
    DirectHeightGap M m (C₁ * C₃) := by
  unfold DirectHeightGap
  calc
    (C₁ * C₃) * M = C₃ * (C₁ * M) := by ring
    _ ≤ C₃ * logAbsDisc :=
      mul_le_mul_of_nonneg_left height_discriminant C₃_nonneg
    _ ≤ m := lang_lower

/-- The exact numerical ratio conclusion of the two JKSST inputs. -/
theorem ratio_bound_of_jksst_inputs
    {M m logAbsDisc C₁ C₃ : ℝ}
    (m_pos : 0 < m)
    (C₁_pos : 0 < C₁) (C₃_pos : 0 < C₃)
    (height_discriminant : C₁ * M ≤ logAbsDisc)
    (lang_lower : C₃ * logAbsDisc ≤ m) :
    M / m ≤ 1 / (C₁ * C₃) := by
  apply ratio_bound_of_direct_height_gap m_pos (mul_pos C₁_pos C₃_pos)
  exact direct_gap_of_jksst_inputs C₃_pos.le height_discriminant lang_lower

end JKSSTLang
