import Mathlib.Algebra.Group.Hom.Defs
import Mathlib.Tactic

namespace XedniLocalPeriodicity

variable {A B : Type*} [AddGroup A] [AddGroup B]

/-- An annihilating multiple gives exact periodicity of coefficient tests. -/
theorem coefficient_period (P Q : A) (k n : ℕ) (hn : n • Q = 0) :
    (P + (k + n) • Q = 0) ↔ (P + k • Q = 0) := by
  simp [add_nsmul, hn]

/-- Order dividing four at the first auxiliary prime and order dividing six
at the second make both local relation tests periodic modulo twelve. -/
theorem two_local_periods (P₇ Q₇ : A) (P₁₁ Q₁₁ : B) (k : ℕ)
    (h₄ : 4 • Q₇ = 0) (h₆ : 6 • Q₁₁ = 0) :
    ((P₇ + (k + 12) • Q₇ = 0) ∧ (P₁₁ + (k + 12) • Q₁₁ = 0)) ↔
    ((P₇ + k • Q₇ = 0) ∧ (P₁₁ + k • Q₁₁ = 0)) := by
  have h₁₂₇ : 12 • Q₇ = 0 := by
    calc
      12 • Q₇ = (3 * 4 : ℕ) • Q₇ := by norm_num
      _ = 3 • (4 • Q₇) := (smul_smul 3 4 Q₇).symm
      _ = 0 := by simp [h₄]
  have h₁₂₁₁ : 12 • Q₁₁ = 0 := by
    calc
      12 • Q₁₁ = (2 * 6 : ℕ) • Q₁₁ := by norm_num
      _ = 2 • (6 • Q₁₁) := (smul_smul 2 6 Q₁₁).symm
      _ = 0 := by simp [h₆]
  exact and_congr (coefficient_period P₇ Q₇ k 12 h₁₂₇)
    (coefficient_period P₁₁ Q₁₁ k 12 h₁₂₁₁)

#print axioms coefficient_period
#print axioms two_local_periods

end XedniLocalPeriodicity
