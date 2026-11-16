import GreenImports
import UniformChain

namespace MarkovGreenBoundAudit
noncomputable section
open scoped BigOperators
variable {S : Type*} [Fintype S] [DecidableEq S]

/-- Actual transition law derived from an officially stochastic matrix. -/
def kernel (P : Matrix S S ℝ) (hP : P ∈ Matrix.rowStochastic ℝ S) (x : S) : PMF S :=
  PMF.ofFintype (fun y => ENNReal.ofReal (P x y)) (by
    rw [← ENNReal.ofReal_sum_of_nonneg
      (fun y _ => Matrix.nonneg_of_mem_rowStochastic hP (i := x) (j := y)),
      Matrix.sum_row_of_mem_rowStochastic hP x]
    simp)

theorem kernel_entry (P : Matrix S S ℝ) (hP : P ∈ Matrix.rowStochastic ℝ S) (x y : S) :
    (kernel P hP x y).toReal = P x y :=
  ENNReal.toReal_ofReal (Matrix.nonneg_of_mem_rowStochastic hP (i := x) (j := y))

theorem power_row_stochastic (P : Matrix S S ℝ) (hP : P ∈ Matrix.rowStochastic ℝ S)
    (n : ℕ) : P^n ∈ Matrix.rowStochastic ℝ S :=
  (Matrix.rowStochastic ℝ S).pow_mem hP n

theorem power_nonnegative (P : Matrix S S ℝ) (hP : P ∈ Matrix.rowStochastic ℝ S)
    (n : ℕ) (x y : S) : 0 ≤ (P^n) x y :=
  Matrix.nonneg_of_mem_rowStochastic (power_row_stochastic P hP n) (i := x) (j := y)

theorem power_row_sum (P : Matrix S S ℝ) (hP : P ∈ Matrix.rowStochastic ℝ S)
    (n : ℕ) (x : S) : (∑ y, (P^n) x y) = 1 :=
  Matrix.sum_row_of_mem_rowStochastic (power_row_stochastic P hP n) x

#print axioms kernel_entry
#print axioms power_nonnegative
#print axioms power_row_sum
end
end MarkovGreenBoundAudit
