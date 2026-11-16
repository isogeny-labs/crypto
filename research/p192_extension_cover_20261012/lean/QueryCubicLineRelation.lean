import Mathlib.Algebra.Group.Hom.Basic
import Mathlib.Tactic.Abel

/-!
The additive-group consequence of two fully split query cubics and six
line cuts at one endpoint. The P-192 cubic-to-quotient bridge, its
cofactor coefficient, and the concrete cut equations are checked in the
finite-field receipts and independently replayed. They are explicit
hypotheses here rather than hidden axioms.
-/

namespace P192ExtensionCover

variable {G : Type*} [AddCommGroup G]

/-- If each cubic's three point images plus `k` times its known query
point have one common value, subtracting their three-cut line sums
cancels the common endpoint and section class. -/
theorem query_cubic_line_difference
    (S C U q₁ q₂ p₁₁ p₁₂ p₁₃ p₂₁ p₂₂ p₂₃
      x₁₁ x₁₂ x₁₃ x₂₁ x₂₂ x₂₃ : G) (k : ℕ)
    (hc₁ : p₁₁ + p₁₂ + p₁₃ + k • q₁ = C)
    (hc₂ : p₂₁ + p₂₂ + p₂₃ + k • q₂ = C)
    (h₁₁ : p₁₁ + U + x₁₁ = S)
    (h₁₂ : p₁₂ + U + x₁₂ = S)
    (h₁₃ : p₁₃ + U + x₁₃ = S)
    (h₂₁ : p₂₁ + U + x₂₁ = S)
    (h₂₂ : p₂₂ + U + x₂₂ = S)
    (h₂₃ : p₂₃ + U + x₂₃ = S) :
    (x₁₁ + x₁₂ + x₁₃) - (x₂₁ + x₂₂ + x₂₃) = k • (q₁ - q₂) := by
  have hs₁ :
      (p₁₁ + p₁₂ + p₁₃) + (U + U + U) + (x₁₁ + x₁₂ + x₁₃) =
        S + S + S := by
    calc
      _ = (p₁₁ + U + x₁₁) + (p₁₂ + U + x₁₂) +
            (p₁₃ + U + x₁₃) := by abel
      _ = S + S + S := by rw [h₁₁, h₁₂, h₁₃]
  have hs₂ :
      (p₂₁ + p₂₂ + p₂₃) + (U + U + U) + (x₂₁ + x₂₂ + x₂₃) =
        S + S + S := by
    calc
      _ = (p₂₁ + U + x₂₁) + (p₂₂ + U + x₂₂) +
            (p₂₃ + U + x₂₃) := by abel
      _ = S + S + S := by rw [h₂₁, h₂₂, h₂₃]
  have hp₁ : p₁₁ + p₁₂ + p₁₃ = C - k • q₁ := by
    calc
      _ = (p₁₁ + p₁₂ + p₁₃ + k • q₁) - k • q₁ := by abel
      _ = C - k • q₁ := by rw [hc₁]
  have hp₂ : p₂₁ + p₂₂ + p₂₃ = C - k • q₂ := by
    calc
      _ = (p₂₁ + p₂₂ + p₂₃ + k • q₂) - k • q₂ := by abel
      _ = C - k • q₂ := by rw [hc₂]
  have hx₁ : x₁₁ + x₁₂ + x₁₃ =
      S + S + S - (p₁₁ + p₁₂ + p₁₃) - (U + U + U) := by
    calc
      _ = ((p₁₁ + p₁₂ + p₁₃) + (U + U + U) +
            (x₁₁ + x₁₂ + x₁₃)) - (p₁₁ + p₁₂ + p₁₃) -
            (U + U + U) := by abel
      _ = S + S + S - (p₁₁ + p₁₂ + p₁₃) - (U + U + U) := by rw [hs₁]
  have hx₂ : x₂₁ + x₂₂ + x₂₃ =
      S + S + S - (p₂₁ + p₂₂ + p₂₃) - (U + U + U) := by
    calc
      _ = ((p₂₁ + p₂₂ + p₂₃) + (U + U + U) +
            (x₂₁ + x₂₂ + x₂₃)) - (p₂₁ + p₂₂ + p₂₃) -
            (U + U + U) := by abel
      _ = S + S + S - (p₂₁ + p₂₂ + p₂₃) - (U + U + U) := by rw [hs₂]
  rw [hx₁, hx₂, hp₁, hp₂, nsmul_sub]
  abel

#print axioms query_cubic_line_difference

end P192ExtensionCover
