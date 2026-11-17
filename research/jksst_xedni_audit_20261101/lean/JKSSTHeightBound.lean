import Mathlib

/-!
# The JKSST height premise is not a universal height bound

Jacobson--Koblitz--Silverman--Stein--Teske, *Analysis of the Xedni
Calculus Attack*, Lemma 4.1 (DOI: 10.1023/A:1008312401197), assumes

`log |D| >= C₁ * maxᵢ hhat(Pᵢ)`.

If this is read as a uniform bound for *all* rational points on a fixed elliptic
curve, the canonical-height identity

`hhat([n]P) = n² * hhat(P)`

immediately contradicts it for every non-torsion point `P`.  The theorem below
formalizes exactly that numerical contradiction.  `hP` stands for `hhat(P)`
and `logAbsDisc` for `log |D|`; no unformalized elliptic-curve facts are hidden
in the proof.

This refutes the universal reading of the height premise.  It does not refute
the paper's explicitly conditional Lemma 4.1, whose premise is restricted to
the finitely many lifted points produced by the xedni algorithm.
-/

namespace JKSSTHeightBound

/-- The proposed uniform upper bound along the multiples of one point. -/
def UniformAlongMultiples (hP logAbsDisc : ℝ) : Prop :=
  ∃ C₁ : ℝ, 0 < C₁ ∧
    ∀ n : ℕ, C₁ * ((n : ℝ) ^ 2 * hP) ≤ logAbsDisc

/-- A positive quadratic height is unbounded along multiples, even while the
discriminant is fixed.  Hence no positive absolute `C₁` can satisfy the JKSST
height inequality for every multiple of a non-torsion point. -/
theorem not_uniform_along_multiples
    (hP logAbsDisc : ℝ) (hP_pos : 0 < hP) :
    ¬ UniformAlongMultiples hP logAbsDisc := by
  rintro ⟨C₁, C₁_pos, bound⟩
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
  have contradiction : logAbsDisc < C₁ * ((n : ℝ) ^ 2 * hP) := by
    have := (div_lt_iff₀ product_pos).mp ratio_lt_sq
    calc
      logAbsDisc < (n : ℝ) ^ 2 * (C₁ * hP) := this
      _ = C₁ * ((n : ℝ) ^ 2 * hP) := by ring
  exact (not_lt_of_ge (bound n)) contradiction

end JKSSTHeightBound
